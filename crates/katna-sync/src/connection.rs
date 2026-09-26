// SPDX-License-Identifier: GPL-3.0-or-later

//! One task per connection.
//!
//! An IMAP command cannot be stopped halfway: dropping its future leaves
//! half a command on the wire or a reply unread (S2 problem 4). So a
//! [`MailBackend`] is moved into a single task, which runs every command to
//! the end, and the rest of the daemon talks to it through a cheap, cloneable
//! [`Connection`] handle. Dropping a handle's future only drops the answer;
//! the task still finishes the command.
//!
//! While the task waits for pushed changes (IMAP IDLE), a request from any
//! handle ends the wait cleanly with DONE, and the request runs next.
//!
//! ```no_run
//! # async fn demo(backend: katna_sync::imap::ImapBackend) -> katna_sync::Result<()> {
//! use std::time::Duration;
//!
//! let (conn, task) = katna_sync::connection::spawn(backend);
//! std::thread::spawn(|| futures_lite::future::block_on(task)); // or any executor
//! conn.select("INBOX").await?;
//! let changes = conn.wait_for_changes(Duration::from_secs(25 * 60)).await?;
//! # Ok(()) }
//! ```

use std::{future::Future, time::Duration};

use async_channel::{Receiver, Sender};

use futures_lite::FutureExt;

use crate::{
    Envelope, Error, FlagState, Folder, FolderChange, FolderStatus, MailBackend, MessageHeaders,
    Result, Wait,
};

type Reply<T> = Sender<Result<T>>;

enum Request {
    ListFolders(Reply<Vec<Folder>>),
    Select(String, Reply<FolderStatus>),
    FetchEnvelopes(u32, Option<u32>, Reply<Vec<Envelope>>),
    FetchHeaders(u32, Option<u32>, Reply<Vec<MessageHeaders>>),
    FetchFlags(u32, u32, Option<u64>, Reply<Vec<FlagState>>),
    Uids(Reply<Vec<u32>>),
    CreateFolder(String, Reply<()>),
    Append(String, Vec<u8>, Reply<()>),
    PollChanges(Reply<Vec<FolderChange>>),
    WaitForChanges(Duration, Reply<Vec<FolderChange>>),
    Logout(Reply<()>),
}

/// Handle to a connection owned by its own task. Clones share the
/// connection; requests run one at a time, in order of arrival.
///
/// When the last handle is dropped, the task logs out and ends.
#[derive(Clone)]
pub struct Connection {
    requests: Sender<Request>,
}

/// Moves `backend` into a new connection task. Run the returned future on
/// any executor; it ends after logout, when every handle is gone, or after
/// an error that breaks the connection.
pub fn spawn<B: MailBackend>(
    backend: B,
) -> (Connection, impl Future<Output = ()> + Send + 'static) {
    let (requests, inbox) = async_channel::unbounded();
    (Connection { requests }, run(backend, inbox))
}

impl Connection {
    pub async fn list_folders(&self) -> Result<Vec<Folder>> {
        self.call(Request::ListFolders).await
    }

    pub async fn select(&self, folder: &str) -> Result<FolderStatus> {
        let folder = folder.to_owned();
        self.call(|reply| Request::Select(folder, reply)).await
    }

    pub async fn fetch_envelopes(&self, first: u32, last: Option<u32>) -> Result<Vec<Envelope>> {
        self.call(|reply| Request::FetchEnvelopes(first, last, reply))
            .await
    }

    pub async fn fetch_headers(
        &self,
        first: u32,
        last: Option<u32>,
    ) -> Result<Vec<MessageHeaders>> {
        self.call(|reply| Request::FetchHeaders(first, last, reply))
            .await
    }

    pub async fn fetch_flags(
        &self,
        first: u32,
        last: u32,
        changed_since: Option<u64>,
    ) -> Result<Vec<FlagState>> {
        self.call(|reply| Request::FetchFlags(first, last, changed_since, reply))
            .await
    }

    pub async fn uids(&self) -> Result<Vec<u32>> {
        self.call(Request::Uids).await
    }

    pub async fn create_folder(&self, folder: &str) -> Result<()> {
        let folder = folder.to_owned();
        self.call(|reply| Request::CreateFolder(folder, reply))
            .await
    }

    pub async fn append(&self, folder: &str, message: Vec<u8>) -> Result<()> {
        let folder = folder.to_owned();
        self.call(|reply| Request::Append(folder, message, reply))
            .await
    }

    pub async fn poll_changes(&self) -> Result<Vec<FolderChange>> {
        self.call(Request::PollChanges).await
    }

    /// Waits for changes to the selected folder, for at most `max_wait`.
    ///
    /// Returns early, possibly with no changes, when another request needs
    /// the connection; call again to keep waiting.
    pub async fn wait_for_changes(&self, max_wait: Duration) -> Result<Vec<FolderChange>> {
        self.call(|reply| Request::WaitForChanges(max_wait, reply))
            .await
    }

    /// Logs out and ends the task. Other handles get [`Error::Closed`] from
    /// then on.
    pub async fn logout(&self) -> Result<()> {
        self.call(Request::Logout).await
    }

    /// Whether the task has ended.
    pub fn is_closed(&self) -> bool {
        self.requests.is_closed()
    }

    async fn call<T>(&self, request: impl FnOnce(Reply<T>) -> Request) -> Result<T> {
        let (reply, answer) = async_channel::bounded(1);
        self.requests
            .send(request(reply))
            .await
            .map_err(|_| closed())?;
        answer.recv().await.map_err(|_| closed())?
    }
}

/// A handle is a backend too, so code written against [`MailBackend`] (the
/// sync engine) can share a connection with other callers.
impl MailBackend for Connection {
    async fn list_folders(&mut self) -> Result<Vec<Folder>> {
        Connection::list_folders(self).await
    }

    async fn select(&mut self, folder: &str) -> Result<FolderStatus> {
        Connection::select(self, folder).await
    }

    async fn fetch_envelopes(&mut self, first: u32, last: Option<u32>) -> Result<Vec<Envelope>> {
        Connection::fetch_envelopes(self, first, last).await
    }

    async fn fetch_headers(
        &mut self,
        first: u32,
        last: Option<u32>,
    ) -> Result<Vec<MessageHeaders>> {
        Connection::fetch_headers(self, first, last).await
    }

    async fn fetch_flags(
        &mut self,
        first: u32,
        last: u32,
        changed_since: Option<u64>,
    ) -> Result<Vec<FlagState>> {
        Connection::fetch_flags(self, first, last, changed_since).await
    }

    async fn uids(&mut self) -> Result<Vec<u32>> {
        Connection::uids(self).await
    }

    async fn create_folder(&mut self, folder: &str) -> Result<()> {
        Connection::create_folder(self, folder).await
    }

    async fn append(&mut self, folder: &str, message: Vec<u8>) -> Result<()> {
        Connection::append(self, folder, message).await
    }

    async fn poll_changes(&mut self) -> Result<Vec<FolderChange>> {
        Connection::poll_changes(self).await
    }

    /// If `interrupt` wins, the task still finishes its wait, and changes
    /// it reports after that are not delivered; the next sync finds them.
    async fn wait_for_changes<I>(
        &mut self,
        max_wait: Duration,
        interrupt: I,
    ) -> Result<Wait<I::Output>>
    where
        I: Future + Send,
        I::Output: Send,
    {
        let wait = async {
            Connection::wait_for_changes(self, max_wait)
                .await
                .map(|changes| Wait {
                    changes,
                    interrupted: None,
                })
        };
        let stop = async {
            Ok(Wait {
                changes: Vec::new(),
                interrupted: Some(interrupt.await),
            })
        };
        wait.or(stop).await
    }

    async fn logout(self) -> Result<()> {
        Connection::logout(&self).await
    }
}

fn closed() -> Error {
    Error::Closed("the connection has ended".into())
}

async fn run<B: MailBackend>(mut backend: B, inbox: Receiver<Request>) {
    let mut next = None;
    loop {
        let request = match next.take() {
            Some(request) => request,
            None => match inbox.recv().await {
                Ok(request) => request,
                Err(_) => break, // every handle is gone
            },
        };
        let error = match request {
            Request::ListFolders(reply) => answer(&reply, backend.list_folders().await),
            Request::Select(folder, reply) => answer(&reply, backend.select(&folder).await),
            Request::FetchEnvelopes(first, last, reply) => {
                answer(&reply, backend.fetch_envelopes(first, last).await)
            }
            Request::FetchHeaders(first, last, reply) => {
                answer(&reply, backend.fetch_headers(first, last).await)
            }
            Request::FetchFlags(first, last, changed_since, reply) => answer(
                &reply,
                backend.fetch_flags(first, last, changed_since).await,
            ),
            Request::Uids(reply) => answer(&reply, backend.uids().await),
            Request::CreateFolder(folder, reply) => {
                answer(&reply, backend.create_folder(&folder).await)
            }
            Request::Append(folder, message, reply) => {
                answer(&reply, backend.append(&folder, message).await)
            }
            Request::PollChanges(reply) => answer(&reply, backend.poll_changes().await),
            Request::WaitForChanges(max_wait, reply) => {
                // The next request, or the last handle going away, ends the
                // wait. `recv` is cancel-safe: nothing is lost if the server
                // reports a change first.
                let result = backend.wait_for_changes(max_wait, inbox.recv()).await;
                let result = result.map(
                    |Wait {
                         changes,
                         interrupted,
                     }| {
                        next = interrupted.and_then(|request| request.ok());
                        changes
                    },
                );
                let error = answer(&reply, result);
                if next.is_none() && inbox.is_closed() && inbox.is_empty() {
                    break;
                }
                error
            }
            Request::Logout(reply) => {
                inbox.close();
                let _ = reply.try_send(backend.logout().await);
                fail_pending(&inbox);
                return;
            }
        };
        if let Some(error) = error {
            tracing::info!(%error, "connection lost");
            inbox.close();
            fail_pending(&inbox);
            return;
        }
    }
    inbox.close();
    if let Err(error) = backend.logout().await {
        tracing::debug!(%error, "logout after the last handle was dropped");
    }
}

/// Sends the result back. Returns the error text if it broke the
/// connection. A caller that stopped waiting is not an error.
fn answer<T>(reply: &Reply<T>, result: Result<T>) -> Option<String> {
    let fatal = match &result {
        Err(error) if error.is_fatal() => Some(error.to_string()),
        _ => None,
    };
    let _ = reply.try_send(result);
    fatal
}

/// Drops the requests that were queued before the task ended; their
/// callers get [`Error::Closed`].
fn fail_pending(inbox: &Receiver<Request>) {
    while inbox.try_recv().is_ok() {}
}
