// SPDX-License-Identifier: GPL-3.0-or-later

//! Talking to `katna-daemon` over D-Bus: the changes the user makes (flags,
//! archive, delete, move), sending mail, and the signals that mail changed
//! or a message could not be sent. The app never writes the store itself.
//! No GPUI here.

use futures_lite::{Stream, StreamExt};
use katna_core::OAuthProvider;
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
    /// Snoozes messages until then (Unix seconds).
    Snooze(Vec<MessageId>, i64),
    /// Brings snoozed messages back now.
    Unsnooze(Vec<MessageId>),
    /// Takes back the queued message with this outbox ID.
    UndoSend(i64),
    /// Opens the message just discarded, or not saved, again. The app does
    /// this itself; the daemon never sees it.
    ReopenDraft,
    /// Puts back the quoted message just removed from a reply. The app does
    /// this itself; the daemon never sees it.
    RestoreQuote,
    /// Has the daemon read the settings file again.
    ReloadConfig,
    /// These, one after the other: an undo that moves mail back to
    /// several folders.
    Several(Vec<Command>),
    /// Saves a note (a new one for ID 0).
    SaveNote(Box<katna_dbus::NoteItem>),
    /// Moves notes to Trash, or back out of it.
    TrashNotes(Vec<i64>, bool),
    /// Deletes notes for good.
    DeleteNotes(Vec<i64>),
    /// A change on the Tasks page.
    Task(Box<crate::tasks::TaskCommand>),
}

/// Most messages one call to the daemon changes. A large selection ("all
/// 20,000 in Inbox") goes in several calls, so each is short and the
/// daemon keeps serving the app, sync and other calls in between.
const BATCH: usize = 500;

impl Command {
    /// Whether this changes notes, so the Notes page reads them again.
    pub fn touches_notes(&self) -> bool {
        match self {
            Self::SaveNote(_) | Self::TrashNotes(..) | Self::DeleteNotes(_) => true,
            Self::Several(commands) => commands.iter().any(Self::touches_notes),
            _ => false,
        }
    }

    /// `self` split into commands of at most `size` messages each, in order.
    pub fn batches(&self, size: usize) -> Vec<Command> {
        let split = |ids: &[MessageId], make: &dyn Fn(Vec<MessageId>) -> Command| {
            ids.chunks(size.max(1))
                .map(|chunk| make(chunk.to_vec()))
                .collect::<Vec<_>>()
        };
        match self {
            Self::MarkRead(ids, on) if ids.len() > size => {
                split(ids, &|ids| Self::MarkRead(ids, *on))
            }
            Self::Star(ids, on) if ids.len() > size => split(ids, &|ids| Self::Star(ids, *on)),
            Self::Important(ids, on) if ids.len() > size => {
                split(ids, &|ids| Self::Important(ids, *on))
            }
            Self::Pin(ids, on) if ids.len() > size => split(ids, &|ids| Self::Pin(ids, *on)),
            Self::Archive(ids) if ids.len() > size => split(ids, &Self::Archive),
            Self::Delete(ids) if ids.len() > size => split(ids, &Self::Delete),
            Self::Move(ids, to) if ids.len() > size => split(ids, &|ids| Self::Move(ids, *to)),
            Self::Snooze(ids, until) if ids.len() > size => {
                split(ids, &|ids| Self::Snooze(ids, *until))
            }
            Self::Unsnooze(ids) if ids.len() > size => split(ids, &Self::Unsnooze),
            _ => vec![self.clone()],
        }
    }

    /// What the snackbar says once the change is sent, if anything:
    /// `count` conversations, or messages when not `conversations`.
    pub fn done_text(&self, count: usize, conversations: bool) -> Option<String> {
        use katna_i18n::tr;
        let count = count as u64;
        let kind = if conversations {
            "conversation"
        } else {
            "message"
        };
        Some(match self {
            Self::Archive(_) => tr!("toast-archived", count = count, kind = kind),
            Self::Delete(_) => tr!("toast-trashed", count = count, kind = kind),
            Self::Move(..) => tr!("toast-moved", count = count, kind = kind),
            Self::Star(_, true) => tr!("toast-starred", count = count, kind = kind),
            Self::Star(_, false) => tr!("toast-unstarred", count = count, kind = kind),
            Self::Important(_, true) => tr!("toast-important", count = count, kind = kind),
            Self::Important(_, false) => tr!("toast-not-important", count = count, kind = kind),
            Self::Pin(_, true) => tr!("toast-pinned", count = count, kind = kind),
            Self::Pin(_, false) => tr!("toast-unpinned", count = count, kind = kind),
            Self::Unsnooze(_) => tr!("toast-unsnoozed", count = count, kind = kind),
            // The window says until when.
            Self::Snooze(..) => return None,
            Self::MarkRead(..)
            | Self::UndoSend(_)
            | Self::ReopenDraft
            | Self::RestoreQuote
            | Self::ReloadConfig
            | Self::SaveNote(_)
            | Self::TrashNotes(..)
            | Self::DeleteNotes(_)
            | Self::Several(_)
            | Self::Task(_) => {
                return None;
            }
        })
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
    katna_dbus::session()
        .await
        .map_err(|err| format!("No D-Bus session: {err}"))
}

/// Sends `command` and waits until the daemon has applied it to the store,
/// in batches of [`BATCH`] messages.
pub async fn send(connection: &Connection, command: &Command) -> Result<(), String> {
    for part in command.batches(BATCH) {
        send_one(connection, &part).await?;
    }
    Ok(())
}

async fn send_one(connection: &Connection, command: &Command) -> Result<(), String> {
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
        Command::Snooze(messages, until) => pim.snooze(&ids(messages), *until).await,
        Command::Unsnooze(messages) => pim.unsnooze(&ids(messages)).await,
        Command::ReloadConfig => pim.reload_config().await,
        Command::UndoSend(id) => match pim.undo_send(*id).await {
            // The app opens the message again, so the outbox can forget it.
            Ok(true) => pim.discard_send(*id).await.map(|_| ()),
            Ok(false) => return Err(katna_i18n::tr!("toast-too-late-to-undo-send")),
            Err(err) => Err(err),
        },
        Command::SaveNote(note) => pim.save_note(note).await.map(|_| ()),
        Command::TrashNotes(ids, trashed) => pim.trash_notes(ids, *trashed).await.map(|_| ()),
        Command::DeleteNotes(ids) => pim.delete_notes(ids).await.map(|_| ()),
        Command::ReopenDraft | Command::RestoreQuote => return Ok(()),
        Command::Task(task) => return crate::tasks::send(connection, task).await.map(|_| ()),
        Command::Several(commands) => {
            for command in commands {
                Box::pin(send(connection, command)).await?;
            }
            return Ok(());
        }
    };
    result.map_err(|err| describe(&err))
}

/// Saves a mail template (a new one when its ID is 0). Returns its ID.
pub async fn save_template(
    connection: &Connection,
    template: &katna_dbus::TemplateItem,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.save_template(template)
        .await
        .map_err(|err| describe(&err))
}

/// Saves a note (a new one when its ID is 0). Returns its ID.
pub async fn save_note(
    connection: &Connection,
    note: &katna_dbus::NoteItem,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.save_note(note).await.map_err(|err| describe(&err))
}

/// Deletes template `id`.
pub async fn delete_template(connection: &Connection, id: i64) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.delete_template(id)
        .await
        .map(|_| ())
        .map_err(|err| describe(&err))
}

/// Shows or hides calendar `id`'s events (`SetCalendarHidden`).
pub async fn set_calendar_hidden(
    connection: &Connection,
    id: i64,
    hidden: bool,
) -> Result<(), String> {
    connection
        .call_method(
            Some(katna_core::ids::DAEMON_BUS_NAME),
            katna_core::ids::PIM_OBJECT_PATH,
            Some(katna_core::ids::PIM_INTERFACE),
            "SetCalendarHidden",
            &(id, hidden),
        )
        .await
        .map(|_| ())
        .map_err(|err| describe(&err))
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

/// Reminds the user `after` seconds after outbox entry `id` goes out if
/// nobody replied by then.
pub async fn set_follow_up(connection: &Connection, id: i64, after: i64) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.set_follow_up(id, after)
        .await
        .map_err(|err| describe(&err))
}

/// Schedules an RFC 5322 message from `account` to go out at `at` (Unix
/// seconds); Undo works for `delay` seconds. Returns its outbox ID.
pub async fn schedule_send(
    connection: &Connection,
    account: i64,
    message: &[u8],
    delay: u32,
    at: i64,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.schedule_send(account, message, delay, at)
        .await
        .map_err(|err| describe(&err))
}

/// How long the SMTP server of `account` holds scheduled mail, in seconds;
/// 0 when it cannot.
pub async fn server_hold_limit(connection: &Connection, account: i64) -> Result<u64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.server_hold_limit(account)
        .await
        .map_err(|err| describe(&err))
}

/// Whether the SMTP server of `account` sends delivery receipts.
pub async fn server_delivery_receipts(
    connection: &Connection,
    account: i64,
) -> Result<bool, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.server_delivery_receipts(account)
        .await
        .map_err(|err| describe(&err))
}

/// Like [`queue_send`], with open and click tracking: each recipient gets
/// a tracked copy of their own (mail that cannot be tracked goes out
/// untracked).
pub async fn queue_tracked_send(
    connection: &Connection,
    account: i64,
    message: &[u8],
    delay: u32,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.queue_tracked_send(account, message, delay)
        .await
        .map_err(|err| describe(&err))
}

/// Saves an RFC 5322 message as a draft of `account`, in place of the
/// copies saved before with its `Message-ID`. Returns the saved message.
pub async fn save_draft(
    connection: &Connection,
    account: i64,
    message: &[u8],
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.save_draft(account, message)
        .await
        .map_err(|err| describe(&err))
}

/// Translates `text`, the plain text of `message` in language `source`,
/// into `target`: the language it was in and the translation, or a
/// [`katna_dbus::translate_problem`].
pub async fn translate(
    connection: &Connection,
    message: i64,
    text: &str,
    source: &str,
    target: &str,
) -> Result<(String, String), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let (source, translated, problem) = pim
        .translate(message, text, source, target)
        .await
        .map_err(|_| katna_dbus::translate_problem::FAILED.to_owned())?;
    if problem.is_empty() {
        Ok((source, translated))
    } else {
        Err(problem)
    }
}

/// The languages the translation server can translate into `target`, or
/// a [`katna_dbus::translate_problem`].
pub async fn translation_sources(
    connection: &Connection,
    target: &str,
) -> Result<Vec<String>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let (sources, problem) = pim
        .translation_sources(target)
        .await
        .map_err(|err| describe(&err))?;
    if problem.is_empty() {
        Ok(sources)
    } else {
        Err(problem)
    }
}

/// Deletes every saved copy of the draft `message_id` of `account`.
pub async fn discard_draft(
    connection: &Connection,
    account: i64,
    message_id: &str,
) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.discard_draft(account, message_id)
        .await
        .map_err(|err| describe(&err))
}

/// What the daemon found for an address.
#[derive(Debug, Clone)]
pub struct Found {
    pub account: NewImapAccount,
    /// Where: `built-in`, `provider`, `ispdb`, `dns-srv`, `mx` or `guess`.
    pub source: String,
    /// The provider to sign in to in the browser, if the servers are
    /// Google's or Microsoft's.
    pub sign_in: Option<OAuthProvider>,
    /// A password works too.
    pub password: bool,
}

/// Finds the servers of `address`.
pub async fn discover(connection: &Connection, address: &str) -> Result<Found, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let (account, source, sign_in, password) = pim
        .discover_account(address)
        .await
        .map_err(|err| describe(&err))?;
    Ok(Found {
        account,
        source,
        sign_in: sign_in.parse().ok(),
        password,
    })
}

/// Signs in to `provider` in the browser and adds that account, or signs
/// `account` in again. Returns once the browser comes back, with the
/// account's ID.
pub async fn sign_in(
    connection: &Connection,
    provider: OAuthProvider,
    account: Option<i64>,
    address: &str,
) -> Result<i64, AddError> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| AddError::Other(describe(&err)))?;
    pim.sign_in(provider.as_str(), account.unwrap_or(0), address)
        .await
        .map_err(|err| add_error(&err))
}

/// Ends a sign-in that still waits for the browser.
pub async fn cancel_sign_in(connection: &Connection) {
    if let Ok(pim) = PimProxy::new(connection).await {
        let _ = pim.cancel_sign_in().await;
    }
}

/// The accounts that signed in with a provider which now asks to sign in
/// again: (ID, address, provider).
pub async fn signed_out(
    connection: &Connection,
) -> Result<Vec<(i64, String, OAuthProvider)>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let accounts = pim.accounts().await.map_err(|err| describe(&err))?;
    Ok(accounts
        .into_iter()
        .filter(|a| a.state == state::AUTH_FAILED)
        .filter_map(|a| Some((a.id, a.address, a.sign_in.parse().ok()?)))
        .collect())
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

/// Renames `account`; an empty name goes back to the name its own mail
/// is sent under.
pub async fn rename_account(
    connection: &Connection,
    account: i64,
    name: &str,
) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.rename_account(account, name)
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

/// Has the daemon delete the mail it downloaded, the search index and
/// sender pictures, and download recent mail again. Returns how many
/// messages lost their body and the bytes deleted.
pub async fn reset_cache(connection: &Connection) -> Result<(u64, u64), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.reset_cache().await.map_err(|err| describe(&err))
}

/// Why an account could not be added.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddError {
    /// The server refused the password, or the user did not allow access
    /// in the browser.
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
        .map_err(|err| add_error(&err))
}

fn add_error(err: &katna_dbus::zbus::Error) -> AddError {
    match err {
        katna_dbus::zbus::Error::MethodError(name, _, _)
            if name.as_str() == "org.freedesktop.DBus.Error.AuthFailed" =>
        {
            AddError::Password(describe(err))
        }
        _ => AddError::Other(describe(err)),
    }
}

/// What became of a message handed to the outbox.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SendOutcome {
    /// Outbox entry `0` went out.
    Sent(i64),
    /// The server refused it for good: its subject and why.
    Failed {
        id: i64,
        subject: String,
        detail: String,
    },
}

/// Yields each message that went out (or left the outbox) or that the
/// server refused for good.
pub async fn send_outcomes(
    connection: &Connection,
) -> Result<impl Stream<Item = SendOutcome>, String> {
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
                // A sent message the mail server files in Sent itself
                // leaves the outbox at once.
                let Some(item) = items.into_iter().find(|item| item.id == id) else {
                    return Some(SendOutcome::Sent(id));
                };
                match item.state.as_str() {
                    send_state::SENT => Some(SendOutcome::Sent(id)),
                    send_state::FAILED => Some(SendOutcome::Failed {
                        id,
                        subject: item.subject,
                        detail: item.detail,
                    }),
                    _ => None,
                }
            }
        })
        .filter_map(|outcome| outcome))
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

/// Starts putting the file at `path` in the Google Drive or OneDrive of
/// `account`.
/// Returns the upload's ID.
pub async fn drive_upload(
    connection: &Connection,
    account: i64,
    path: &str,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.drive_upload(account, path)
        .await
        .map_err(|err| describe(&err))
}

/// Where Drive upload `id` stands.
pub async fn drive_upload_status(
    connection: &Connection,
    id: i64,
) -> Result<katna_dbus::DriveUpload, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.drive_upload_status(id)
        .await
        .map_err(|err| describe(&err))
}

/// Stops Drive upload `id` and moves its file to the bin.
pub async fn drive_cancel(connection: &Connection, id: i64) -> Result<bool, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.drive_cancel(id).await.map_err(|err| describe(&err))
}

/// Shares the files of `uploads` with `addresses`; returns those Drive
/// would not share with.
pub async fn drive_share(
    connection: &Connection,
    uploads: &[i64],
    addresses: &[String],
) -> Result<Vec<String>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let addresses: Vec<&str> = addresses.iter().map(String::as_str).collect();
    pim.drive_share(uploads, &addresses)
        .await
        .map_err(|err| describe(&err))
}

/// Lets anyone with the link view the files of `uploads`; returns their
/// links, in order.
pub async fn drive_share_with_link(
    connection: &Connection,
    uploads: &[i64],
) -> Result<Vec<String>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.drive_share_with_link(uploads)
        .await
        .map_err(|err| describe(&err))
}

/// Yields the ID of each Drive upload that moves on.
pub async fn drive_changes(connection: &Connection) -> Result<impl Stream<Item = i64>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = pim
        .receive_drive_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes.filter_map(|signal| signal.args().ok().map(|args| args.id)))
}

/// Where an update of Katna stands.
pub async fn update_status(connection: &Connection) -> Result<katna_dbus::UpdateStatus, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.update_status().await.map_err(|err| describe(&err))
}

/// What the version on offer brings, once a check found one.
pub async fn update_details(
    connection: &Connection,
) -> Result<Option<katna_core::update::Manifest>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let json = pim.update_details().await.map_err(|err| describe(&err))?;
    Ok(katna_core::update::Manifest::parse(json.as_bytes()))
}

/// Has the daemon look for a newer version now.
pub async fn check_for_update(connection: &Connection) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.check_for_update().await.map_err(|err| describe(&err))
}

/// Has the daemon download the newer version it found.
pub async fn download_update(connection: &Connection) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.download_update().await.map_err(|err| describe(&err))
}

/// Yields whenever where an update stands changes.
pub async fn update_changes(connection: &Connection) -> Result<impl Stream<Item = ()>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = pim
        .receive_update_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes.map(|_| ()))
}

/// Fires when the saved contacts changed.
pub async fn contacts_changes(connection: &Connection) -> Result<impl Stream<Item = ()>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = pim
        .receive_contacts_changed()
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

/// Has the daemon check `account` for new mail now, every folder of it, or
/// every account for `None`. Returns once each has finished that sync or
/// failed to connect; an account the daemon does not sync is not waited for.
pub async fn check_mail(
    connection: &Connection,
    account: Option<katna_core::AccountId>,
) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    // Listening before asking, so the end of a quick sync is not missed.
    let mut changes = pim
        .receive_sync_status_changed()
        .await
        .map_err(|err| describe(&err))?;
    let mut waiting: Vec<i64> = pim
        .accounts()
        .await
        .map_err(|err| describe(&err))?
        .into_iter()
        .filter(|a| account.is_none_or(|id| id.0 == a.id) && a.state != state::NOT_SYNCED)
        .map(|a| a.id)
        .collect();
    pim.sync_now(account.map_or(0, |id| id.0))
        .await
        .map_err(|err| describe(&err))?;
    // A woken account says so when its sync ends: in sync, offline or
    // with its password refused. "Connecting" comes first when it had no
    // connection.
    while !waiting.is_empty() {
        let Some(change) = changes.next().await else {
            break;
        };
        let Ok(args) = change.args() else {
            continue;
        };
        let id = args.account;
        if !waiting.contains(&id) {
            continue;
        }
        let accounts = pim.accounts().await.map_err(|err| describe(&err))?;
        let connecting = accounts
            .iter()
            .any(|a| a.id == id && a.state == state::CONNECTING);
        if !connecting {
            waiting.retain(|w| *w != id);
        }
    }
    Ok(())
}

/// Yields for every `MailChanged`, `AccountsChanged`, `SyncStatusChanged`,
/// `TrackingChanged` and `CalendarChanged` signal.
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
    let tracking = pim
        .receive_tracking_changed()
        .await
        .map_err(|err| describe(&err))?;
    let calendar = pim
        .receive_calendar_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes
        .map(|_| ())
        .or(accounts.map(|_| ()))
        .or(status.map(|_| ()))
        .or(tracking.map(|_| ()))
        .or(calendar.map(|_| ())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_changes_go_in_batches() {
        let ids: Vec<MessageId> = (1..=1201).map(MessageId).collect();
        let parts = Command::Move(ids.clone(), FolderId(7)).batches(500);
        let sizes: Vec<usize> = parts
            .iter()
            .map(|p| match p {
                Command::Move(ids, FolderId(7)) => ids.len(),
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(sizes, [500, 500, 201]);
        let again: Vec<MessageId> = parts
            .into_iter()
            .flat_map(|p| match p {
                Command::Move(ids, _) => ids,
                _ => Vec::new(),
            })
            .collect();
        assert_eq!(again, ids);
        let small = Command::MarkRead(ids[..3].to_vec(), true);
        assert_eq!(small.batches(500), std::slice::from_ref(&small));
    }

    #[test]
    fn snackbar_texts() {
        let ids = vec![MessageId(1)];
        assert_eq!(
            Command::Archive(ids.clone()).done_text(1, true).as_deref(),
            Some("Conversation archived.")
        );
        assert_eq!(
            Command::Important(ids.clone(), false)
                .done_text(2, false)
                .as_deref(),
            Some("2 messages marked as not important.")
        );
        assert_eq!(
            Command::Pin(ids.clone(), true)
                .done_text(1, true)
                .as_deref(),
            Some("Conversation pinned to the top.")
        );
        assert_eq!(
            Command::Star(ids.clone(), true)
                .done_text(1, false)
                .as_deref(),
            Some("Message starred.")
        );
        assert_eq!(Command::MarkRead(ids, true).done_text(1, false), None);
    }
}
