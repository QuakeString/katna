// SPDX-License-Identifier: GPL-3.0-or-later

//! Spike S2: drive Pimalaya's sans-I/O `io-imap` and `io-smtp` coroutines
//! with our own rustls + `async-io` networking, behind Katna-owned traits.
//!
//! Throw-away code. The findings are in `docs/spikes/s2-pimalaya-io.md`.
//!
//! Nothing in this file mentions a Pimalaya type: the traits and data types
//! are what `katna-sync` would expose to the rest of Katna. Pimalaya types
//! stay inside [`imap`] and [`smtp`].

use std::{future::Future, time::Duration};

pub mod imap;
pub mod net;
pub mod smtp;

/// Errors from a mail backend. Protocol errors are flattened to text: the
/// engine only needs to know whether to retry, re-authenticate or give up.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network: {0}")]
    Io(#[from] std::io::Error),
    #[error("TLS: {0}")]
    Tls(String),
    #[error("authentication failed: {0}")]
    Auth(String),
    #[error("protocol: {0}")]
    Protocol(String),
    #[error("timed out after {0:?}")]
    Timeout(Duration),
}

pub type Result<T> = std::result::Result<T, Error>;

/// How to reach a server.
#[derive(Clone, Debug)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
    pub security: Security,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Security {
    /// Implicit TLS (IMAPS 993, SMTPS 465).
    Tls,
    /// Plain connection upgraded with STARTTLS.
    StartTls,
    /// No encryption. Only for local test servers.
    Plain,
}

/// Password credentials (app passwords included). OAuth2 comes later.
#[derive(Clone)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

/// A folder as the engine sees it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Folder {
    /// Full name, decoded from modified UTF-7.
    pub name: String,
    pub delimiter: Option<char>,
    pub role: Option<FolderRole>,
    pub selectable: bool,
}

/// RFC 6154 special-use roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FolderRole {
    Inbox,
    All,
    Archive,
    Drafts,
    Flagged,
    Junk,
    Sent,
    Trash,
}

/// State of a folder after selecting it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FolderStatus {
    pub exists: u32,
    pub uid_validity: Option<u32>,
    pub uid_next: Option<u32>,
    pub highest_modseq: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Address {
    pub name: Option<String>,
    pub email: String,
}

/// Message flags. Keywords keep their IMAP spelling.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    pub seen: bool,
    pub answered: bool,
    pub flagged: bool,
    pub deleted: bool,
    pub draft: bool,
    pub keywords: Vec<String>,
}

/// Header-level data for the message list. Header values are raw: RFC 2047
/// decoding belongs to our MIME layer (`mail-parser`), not to the protocol.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Envelope {
    pub uid: u32,
    pub size: u32,
    pub flags: Flags,
    pub date: Option<String>,
    pub subject: Option<String>,
    pub from: Vec<Address>,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub message_id: Option<String>,
    pub in_reply_to: Option<String>,
}

/// A change the server pushed while we waited on a folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FolderChange {
    /// The folder now holds this many messages.
    Exists(u32),
    /// The message at this sequence number was removed.
    Expunged(u32),
    /// Flags changed for the message at this sequence number.
    FlagsChanged { seq: u32, flags: Flags },
}

/// Mail access protocol (IMAP now; JMAP and POP3 later).
///
/// Futures are `Send` so the daemon can run accounts on any executor.
pub trait MailBackend: Send {
    fn list_folders(&mut self) -> impl Future<Output = Result<Vec<Folder>>> + Send;

    fn select(&mut self, folder: &str) -> impl Future<Output = Result<FolderStatus>> + Send;

    /// Envelopes for UIDs `first..=last` of the selected folder, in UID order.
    fn fetch_envelopes(
        &mut self,
        first: u32,
        last: Option<u32>,
    ) -> impl Future<Output = Result<Vec<Envelope>>> + Send;

    fn create_folder(&mut self, folder: &str) -> impl Future<Output = Result<()>> + Send;

    fn append(&mut self, folder: &str, message: Vec<u8>)
    -> impl Future<Output = Result<()>> + Send;

    /// Asks the server for pending changes on the selected folder (NOOP).
    fn poll_changes(&mut self) -> impl Future<Output = Result<Vec<FolderChange>>> + Send;

    /// Waits on the selected folder until the server reports a change or
    /// `max_wait` passes (then returns an empty list). Leaves the connection
    /// ready for the next command either way.
    fn wait_for_changes(
        &mut self,
        max_wait: Duration,
    ) -> impl Future<Output = Result<Vec<FolderChange>>> + Send;

    fn logout(self) -> impl Future<Output = Result<()>> + Send;
}

/// Mail submission protocol.
pub trait MailSender: Send {
    fn send(
        &mut self,
        from: &str,
        to: &[&str],
        message: Vec<u8>,
    ) -> impl Future<Output = Result<()>> + Send;

    fn quit(self) -> impl Future<Output = Result<()>> + Send;
}
