// SPDX-License-Identifier: GPL-3.0-or-later

//! Talking to `katna-daemon` over D-Bus: the changes the user makes (flags,
//! archive, delete, move), sending mail, and the signals that mail changed
//! or a message could not be sent. The app never writes the store itself.
//! No GPUI here.

use futures_lite::{Stream, StreamExt};
use katna_dbus::zbus::Connection;
use katna_dbus::{NewImapAccount, OutboxItem, PimProxy, flag, send_state, state};
use katna_store::{FolderId, MessageId};

/// A change to send to the daemon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    MarkRead(Vec<MessageId>, bool),
    Star(Vec<MessageId>, bool),
    Important(Vec<MessageId>, bool),
    /// Pins a conversation's messages to the top of the list, or unpins.
    Pin(Vec<MessageId>, bool),
    Archive(Vec<MessageId>),
    Delete(Vec<MessageId>),
    Move(Vec<MessageId>, FolderId),
    SyncNow,
    /// Takes back the queued message with this outbox ID.
    UndoSend(i64),
}

impl Command {
    /// What the snackbar says once the change is sent, if anything.
    pub fn done_text(&self, what: &str) -> Option<String> {
        match self {
            Self::Archive(_) => Some(format!("{what} archived.")),
            Self::Delete(_) => Some(format!("{what} moved to Trash.")),
            Self::Move(..) => Some(format!("{what} moved.")),
            Self::Star(_, true) => Some(format!("{what} starred.")),
            Self::Star(_, false) => Some(format!("{what} unstarred.")),
            Self::Important(_, true) => Some(format!("{what} marked as important.")),
            Self::Important(_, false) => Some(format!("{what} marked as not important.")),
            Self::Pin(_, true) => Some(format!("{what} pinned to the top.")),
            Self::Pin(_, false) => Some(format!("{what} unpinned.")),
            Self::MarkRead(..) | Self::SyncNow | Self::UndoSend(_) => None,
        }
    }
}

/// What [`describe`] says when the daemon is not running.
pub const NOT_RUNNING: &str = "The Katna background service is not running.";

/// Why a change could not be sent.
pub fn describe(err: &katna_dbus::zbus::Error) -> String {
    match err {
        katna_dbus::zbus::Error::MethodError(name, detail, _) => {
            if name.as_str() == "org.freedesktop.DBus.Error.ServiceUnknown" {
                NOT_RUNNING.to_owned()
            } else {
                detail.clone().unwrap_or_else(|| name.to_string())
            }
        }
        err => format!("The Katna background service did not answer: {err}"),
    }
}

/// Connects to the session bus.
pub async fn connect() -> Result<Connection, String> {
    Connection::session()
        .await
        .map_err(|err| format!("No D-Bus session: {err}"))
}

/// Sends `command` and waits until the daemon has applied it to the store.
pub async fn send(connection: &Connection, command: &Command) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let ids = |ids: &[MessageId]| ids.iter().map(|id| id.0).collect::<Vec<i64>>();
    let result = match command {
        Command::MarkRead(messages, read) => {
            let (add, remove): (&[&str], &[&str]) = if *read {
                (&[flag::SEEN], &[])
            } else {
                (&[], &[flag::SEEN])
            };
            pim.set_flags(&ids(messages), add, remove).await
        }
        Command::Star(messages, on) => {
            let (add, remove): (&[&str], &[&str]) = if *on {
                (&[flag::FLAGGED], &[])
            } else {
                (&[], &[flag::FLAGGED])
            };
            pim.set_flags(&ids(messages), add, remove).await
        }
        Command::Important(messages, on) => {
            let (add, remove): (&[&str], &[&str]) = if *on {
                (&[flag::IMPORTANT], &[])
            } else {
                (&[], &[flag::IMPORTANT])
            };
            pim.set_flags(&ids(messages), add, remove).await
        }
        Command::Pin(messages, on) => pim.set_pinned(&ids(messages), *on).await,
        Command::Archive(messages) => pim.archive_messages(&ids(messages)).await,
        Command::Delete(messages) => pim.delete_messages(&ids(messages)).await,
        Command::Move(messages, folder) => pim.move_messages(&ids(messages), folder.0).await,
        Command::SyncNow => pim.sync_now(0).await,
        Command::UndoSend(id) => match pim.undo_send(*id).await {
            // The app opens the message again, so the outbox can forget it.
            Ok(true) => pim.discard_send(*id).await.map(|_| ()),
            Ok(false) => {
                return Err("Too late to undo: the message is already on its way.".to_owned());
            }
            Err(err) => Err(err),
        },
    };
    result.map_err(|err| describe(&err))
}

/// Queues an RFC 5322 message from `account` to go out in `delay` seconds.
/// Returns its outbox ID, for [`Command::UndoSend`].
pub async fn queue_send(
    connection: &Connection,
    account: i64,
    message: &[u8],
    delay: u32,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.queue_send(account, message, delay)
        .await
        .map_err(|err| describe(&err))
}

/// Finds the servers of `address`. Returns them and where they came from
/// (`built-in`, `provider`, `ispdb`, `dns-srv`, `mx` or `guess`).
pub async fn discover(
    connection: &Connection,
    address: &str,
) -> Result<(NewImapAccount, String), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.discover_account(address)
        .await
        .map_err(|err| describe(&err))
}

/// Asks the daemon for a remote image of a message.
pub async fn fetch_image(connection: &Connection, url: &str) -> Result<Vec<u8>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.fetch_image(url).await.map_err(|err| describe(&err))
}

/// Asks the daemon for the picture of the sender `address` (empty: none).
pub async fn sender_picture(connection: &Connection, address: &str) -> Result<Vec<u8>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.sender_picture(address)
        .await
        .map_err(|err| describe(&err))
}

/// Stops syncing `account` and deletes its mail and password from this
/// computer. Nothing changes on the server.
pub async fn remove_account(connection: &Connection, account: i64) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.remove_account(account)
        .await
        .map(|_| ())
        .map_err(|err| describe(&err))
}

/// Downloads message `id` from its server now; it is in the store when
/// this returns `Ok`.
pub async fn fetch_body(connection: &Connection, id: i64) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.fetch_body(id).await.map_err(|err| describe(&err))
}

/// Creates a folder (a label, on Gmail) on the account's server, inside
/// `parent` when given. Returns its ID.
pub async fn create_folder(
    connection: &Connection,
    account: i64,
    name: &str,
    parent: Option<i64>,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.create_folder(account, name, parent.unwrap_or(0))
        .await
        .map_err(|err| describe(&err))
}

/// Has the daemon delete everything Katna keeps on this computer. It exits
/// once done; the next call starts a new one.
pub async fn delete_all_data(connection: &Connection) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.delete_all_data().await.map_err(|err| describe(&err))
}

/// Why an account could not be added.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddError {
    /// The server refused the password.
    Password(String),
    Other(String),
}

/// Checks the login, then adds the account and starts syncing it.
pub async fn add_account(
    connection: &Connection,
    account: &NewImapAccount,
    password: &str,
) -> Result<i64, AddError> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| AddError::Other(describe(&err)))?;
    pim.add_imap_account(account, password)
        .await
        .map_err(|err| match &err {
            katna_dbus::zbus::Error::MethodError(name, _, _)
                if name.as_str() == "org.freedesktop.DBus.Error.AuthFailed" =>
            {
                AddError::Password(describe(&err))
            }
            _ => AddError::Other(describe(&err)),
        })
}

/// Yields the subject and reason of each message the server refused for
/// good.
pub async fn send_failures(
    connection: &Connection,
) -> Result<impl Stream<Item = (String, String)>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = pim
        .receive_outbox_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes
        .then(move |signal| {
            let pim = pim.clone();
            async move {
                let id = signal.args().ok()?.id;
                let items = pim.outbox().await.ok()?;
                items
                    .into_iter()
                    .find(|item| item.id == id && item.state == send_state::FAILED)
                    .map(|item| (item.subject, item.detail))
            }
        })
        .filter_map(|failure| failure))
}

/// Every message in the outbox: waiting, being sent, sent or failed.
pub async fn outbox(connection: &Connection) -> Result<Vec<OutboxItem>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.outbox().await.map_err(|err| describe(&err))
}

/// Yields whenever an outbox entry changes.
pub async fn outbox_changes(connection: &Connection) -> Result<impl Stream<Item = ()>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = pim
        .receive_outbox_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes.map(|_| ()))
}

/// Asks the daemon about the accounts, which also starts it if D-Bus can.
/// Returns whether an account still waits for its first sync.
pub async fn first_sync_pending(connection: &Connection) -> Result<bool, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let accounts = pim.accounts().await.map_err(|err| describe(&err))?;
    Ok(accounts
        .iter()
        .any(|a| a.last_sync == 0 && a.state != state::NOT_SYNCED && a.state != state::AUTH_FAILED))
}

/// Yields for every `MailChanged`, `AccountsChanged` and `SyncStatusChanged`
/// signal.
pub async fn mail_changes(connection: &Connection) -> Result<impl Stream<Item = ()>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = pim
        .receive_mail_changed()
        .await
        .map_err(|err| describe(&err))?;
    let accounts = pim
        .receive_accounts_changed()
        .await
        .map_err(|err| describe(&err))?;
    let status = pim
        .receive_sync_status_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes
        .map(|_| ())
        .or(accounts.map(|_| ()))
        .or(status.map(|_| ())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snackbar_texts() {
        let ids = vec![MessageId(1)];
        assert_eq!(
            Command::Archive(ids.clone())
                .done_text("Conversation")
                .as_deref(),
            Some("Conversation archived.")
        );
        assert_eq!(
            Command::Important(ids.clone(), false)
                .done_text("2 messages")
                .as_deref(),
            Some("2 messages marked as not important.")
        );
        assert_eq!(
            Command::Pin(ids.clone(), true)
                .done_text("Conversation")
                .as_deref(),
            Some("Conversation pinned to the top.")
        );
        assert_eq!(
            Command::Star(ids.clone(), true)
                .done_text("Message")
                .as_deref(),
            Some("Message starred.")
        );
        assert_eq!(Command::MarkRead(ids, true).done_text("x"), None);
    }
}
