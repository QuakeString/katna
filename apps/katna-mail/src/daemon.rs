// SPDX-License-Identifier: GPL-3.0-or-later

//! Talking to `katna-daemon` over D-Bus: the changes the user makes (flags,
//! archive, delete, move) and the signal that mail changed. The app never
//! writes the store itself. No GPUI here.

use futures_lite::{Stream, StreamExt};
use katna_dbus::zbus::Connection;
use katna_dbus::{PimProxy, flag};
use katna_store::{FolderId, MessageId};

/// A change to send to the daemon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    MarkRead(Vec<MessageId>, bool),
    Star(Vec<MessageId>, bool),
    Archive(Vec<MessageId>),
    Delete(Vec<MessageId>),
    Move(Vec<MessageId>, FolderId),
    SyncNow,
}

impl Command {
    /// What the snackbar says once the change is sent, if anything.
    pub fn done_text(&self, what: &str) -> Option<String> {
        match self {
            Self::Archive(_) => Some(format!("{what} archived.")),
            Self::Delete(_) => Some(format!("{what} moved to Trash.")),
            Self::Move(..) => Some(format!("{what} moved.")),
            Self::MarkRead(..) | Self::Star(..) | Self::SyncNow => None,
        }
    }
}

/// Why a change could not be sent.
pub fn describe(err: &katna_dbus::zbus::Error) -> String {
    match err {
        katna_dbus::zbus::Error::MethodError(name, detail, _) => {
            if name.as_str() == "org.freedesktop.DBus.Error.ServiceUnknown" {
                "The Katna background service is not running.".to_owned()
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
        Command::Archive(messages) => pim.archive_messages(&ids(messages)).await,
        Command::Delete(messages) => pim.delete_messages(&ids(messages)).await,
        Command::Move(messages, folder) => pim.move_messages(&ids(messages), folder.0).await,
        Command::SyncNow => pim.sync_now(0).await,
    };
    result.map_err(|err| describe(&err))
}

/// Yields once for every burst of `MailChanged` signals.
pub async fn mail_changes(connection: &Connection) -> Result<impl Stream<Item = ()>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = pim
        .receive_mail_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes.map(|_| ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snackbar_texts() {
        let ids = vec![MessageId(1)];
        assert_eq!(
            Command::Archive(ids.clone()).done_text("Conversation").as_deref(),
            Some("Conversation archived.")
        );
        assert_eq!(Command::MarkRead(ids, true).done_text("x"), None);
    }
}
