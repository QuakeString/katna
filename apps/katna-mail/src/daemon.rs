// SPDX-License-Identifier: GPL-3.0-or-later

//! Talking to `katna-daemon` over D-Bus: the changes the user makes (flags,
//! archive, delete, move), sending mail, and the signals that mail changed
//! or a message could not be sent. The app never writes the store itself.
//! No GPUI here.

use std::collections::HashMap;

use futures_lite::{Stream, StreamExt};
use katna_core::OAuthProvider;
use katna_dbus::zbus::Connection;
use katna_dbus::{
    NewImapAccount, NewPop3Account, OutboxItem, PimProxy, ServerSpec, flag, send_state, state,
};
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
    /// Gmail: puts the labels (folders) of the first list on messages and
    /// takes those of the second off, leaving them where they are.
    Labels(Vec<MessageId>, Vec<FolderId>, Vec<FolderId>),
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
    /// Takes back the text just rephrased in the message. The app does
    /// this itself; the daemon never sees it.
    UndoRephrase,
    /// Puts back the subject the user typed before picking another
    /// wording. The app does this itself; the daemon never sees it.
    RestoreSubject(String),
    /// Brings back the saved contacts just deleted (by their first card).
    /// The Contacts page does this itself; the daemon never sees it.
    RestoreContacts(Vec<i64>),
    /// Brings back the color scheme just deleted: its id, its file's
    /// contents and whether it was in use. The app does this itself; the
    /// daemon never sees it.
    RestoreScheme(String, String, bool),
    /// Turns an app on again in Settings > Apps: the Undo after turning it
    /// off, or "Turn on" where it was asked for. The app does this itself;
    /// the daemon hears of it through `ReloadConfig`.
    TurnAppOn(katna_core::config::AppKind),
    /// Gives saved cards these labels, by name: an undo on the Contacts
    /// page.
    ContactLabels(Vec<(i64, Vec<String>)>),
    /// Renames a contact label; an empty new name takes it away.
    RenameContactLabel(String, String),
    /// Deletes saved cards: the Undo of Add to contacts.
    DeleteContacts(Vec<i64>),
    /// Writes saved cards, each over its card or as a new card in its
    /// book, with exactly these labels: a merge, and its Undo.
    WriteCards(Vec<WriteCard>),
    /// Has the daemon read the settings file again.
    ReloadConfig,
    /// Has the daemon delete the local copy of an app just turned off
    /// ("Remove the copy").
    ForgetApp(katna_core::config::AppKind),
    /// These, one after the other: an undo that moves mail back to
    /// several folders.
    Several(Vec<Command>),
    /// A change to the calendar (`Pim1.EditEvent`).
    Event(Box<katna_store::calendar::EventChange>),
    /// Saves a note (a new one for ID 0).
    SaveNote(Box<katna_dbus::NoteItem>),
    /// Moves notes to Trash, or back out of it.
    TrashNotes(Vec<i64>, bool),
    /// Deletes notes for good.
    DeleteNotes(Vec<i64>),
    /// Puts notes in this order on the board, the first on top.
    OrderNotes(Vec<i64>),
    /// Takes a label off notes and puts another on (see `RelabelNotes`).
    RelabelNotes(Vec<i64>, String, String),
    /// A change on the Tasks page.
    Task(Box<crate::tasks::TaskCommand>),
    /// A change to a calendar itself (`Pim1.RenameCalendar` and the like).
    Calendar(CalendarEdit),
    /// Mutes something until then (Unix seconds), or until unmuted (0).
    Mute(Muted, i64),
    Unmute(Muted),
    /// Pins something of a mail to the top of its chat, with the pin bar's
    /// label, in place of another pin (`Pim1.PinInChat`).
    PinInChat(MessageId, katna_store::Pinned, String, Option<i64>),
    /// Takes off a chat pin, by its ID.
    UnpinInChat(i64),
    /// Puts a chat's pins in this order.
    OrderChatPins(Vec<i64>),
    /// Moves items of an account's drive to its bin, or back out of it.
    CloudTrash(katna_core::AccountId, Vec<String>, bool),
    /// Renames an item of an account's drive: its id and new name.
    CloudRename(katna_core::AccountId, String, String),
    /// Sets whether a folder (or an inbox tab) notifies and counts.
    SetBell(
        FolderId,
        Option<katna_core::MailCategory>,
        katna_store::Bell,
    ),
}

/// What [`Command::Mute`] acts on (`docs/ARCHITECTURE.md` §15.1.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Muted {
    Account(katna_core::AccountId),
    Folder(FolderId),
    /// The conversation of this message.
    Conversation(MessageId),
    /// Mail from this address.
    Sender(String),
}

impl Muted {
    /// `Pim1.Mute`'s kind, id and address.
    fn args(&self) -> (&'static str, i64, &str) {
        use katna_dbus::mute;
        match self {
            Self::Account(a) => (mute::ACCOUNT, a.0, ""),
            Self::Folder(f) => (mute::FOLDER, f.0, ""),
            Self::Conversation(m) => (mute::CONVERSATION, m.0, ""),
            Self::Sender(address) => (mute::SENDER, 0, address),
        }
    }
}

/// A saved card to write, for [`Command::WriteCards`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteCard {
    /// The card to write over, or 0 for a new card in `book`.
    pub id: i64,
    pub book: i64,
    pub card: katna_core::contact::Card,
    pub labels: Vec<String>,
}

/// Most messages one call to the daemon changes. A large selection ("all
/// 20,000 in Inbox") goes in several calls, so each is short and the
/// daemon keeps serving the app, sync and other calls in between.
const BATCH: usize = 500;

impl Command {
    /// Whether this changes notes, so the Notes page reads them again.
    pub fn touches_notes(&self) -> bool {
        match self {
            Self::SaveNote(_)
            | Self::TrashNotes(..)
            | Self::DeleteNotes(_)
            | Self::OrderNotes(_)
            | Self::RelabelNotes(..) => true,
            Self::Several(commands) => commands.iter().any(Self::touches_notes),
            _ => false,
        }
    }

    /// The account whose drive this changes, read again once it is done.
    pub fn drive(&self) -> Option<katna_core::AccountId> {
        match self {
            Self::CloudTrash(account, ..) | Self::CloudRename(account, ..) => Some(*account),
            _ => None,
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
            Self::Labels(ids, add, remove) if ids.len() > size => {
                split(ids, &|ids| Self::Labels(ids, add.clone(), remove.clone()))
            }
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
            | Self::UndoRephrase
            | Self::RestoreSubject(_)
            | Self::RestoreContacts(_)
            | Self::RestoreScheme(..)
            | Self::TurnAppOn(_)
            | Self::ContactLabels(_)
            | Self::RenameContactLabel(..)
            | Self::DeleteContacts(_)
            | Self::WriteCards(_)
            | Self::ReloadConfig
            | Self::ForgetApp(_)
            | Self::SaveNote(_)
            | Self::TrashNotes(..)
            | Self::DeleteNotes(_)
            | Self::OrderNotes(_)
            | Self::RelabelNotes(..)
            | Self::Event(_)
            | Self::Several(_)
            | Self::Task(_)
            | Self::Calendar(_)
            | Self::Mute(..)
            | Self::Unmute(_)
            | Self::PinInChat(..)
            | Self::UnpinInChat(_)
            | Self::OrderChatPins(_)
            | Self::CloudTrash(..)
            | Self::CloudRename(..)
            | Self::SetBell(..)
            // The window names the label.
            | Self::Labels(..) => {
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

/// Where one account's calendars, tasks or contacts stand, for the line
/// under it in a page's side list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountState {
    /// A [`katna_dbus::calendar_state`] (or [`katna_dbus::task_state`]).
    pub state: String,
    /// Why, for people; may be empty.
    pub detail: String,
    /// The provider it signs in with, when that is OAuth2.
    pub sign_in: Option<OAuthProvider>,
}

impl AccountState {
    /// The provider to sign in with to fix it: the account's own, or the
    /// one a password account has to sign in with (`USE_SIGN_IN`, whose
    /// detail is `provider` or `provider: why the other way failed`).
    pub fn provider(&self) -> Option<OAuthProvider> {
        self.sign_in.or_else(|| {
            (self.state == katna_dbus::calendar_state::USE_SIGN_IN)
                .then(|| self.use_sign_in().0.parse().ok())
                .flatten()
        })
    }

    /// Why the other way failed, when `USE_SIGN_IN` says (in English).
    pub fn use_sign_in_why(&self) -> &str {
        if self.state == katna_dbus::calendar_state::USE_SIGN_IN {
            self.use_sign_in().1
        } else {
            ""
        }
    }

    fn use_sign_in(&self) -> (&str, &str) {
        self.detail
            .split_once(':')
            .map_or((self.detail.as_str(), ""), |(provider, why)| {
                (provider.trim(), why.trim())
            })
    }
}

/// Connects to the session bus.
pub async fn connect() -> Result<Connection, String> {
    let connection = katna_dbus::session()
        .await
        .map_err(|err| format!("No D-Bus session: {err}"))?;
    katna_dbus::ensure_daemon(&connection).await;
    Ok(connection)
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
        Command::Labels(messages, add, remove) => {
            let folders = |list: &[FolderId]| list.iter().map(|f| f.0).collect::<Vec<i64>>();
            pim.set_labels(&ids(messages), &folders(add), &folders(remove))
                .await
        }
        Command::Snooze(messages, until) => pim.snooze(&ids(messages), *until).await,
        Command::Unsnooze(messages) => pim.unsnooze(&ids(messages)).await,
        Command::ReloadConfig => pim.reload_config().await,
        Command::ForgetApp(app) => pim.forget_app(app.key()).await,
        Command::UndoSend(id) => match pim.undo_send(*id).await {
            // The app opens the message again, so the outbox can forget it.
            Ok(true) => pim.discard_send(*id).await.map(|_| ()),
            Ok(false) => return Err(katna_i18n::tr!("toast-too-late-to-undo-send")),
            Err(err) => Err(err),
        },
        Command::SaveNote(note) => pim.save_note(note).await.map(|_| ()),
        Command::TrashNotes(ids, trashed) => pim.trash_notes(ids, *trashed).await.map(|_| ()),
        Command::DeleteNotes(ids) => pim.delete_notes(ids).await.map(|_| ()),
        Command::OrderNotes(ids) => pim.order_notes(ids).await.map(|_| ()),
        Command::PinInChat(message, what, label, replace) => {
            let (kind, file, text) = match what {
                katna_store::Pinned::Mail => ("mail", 0, ""),
                katna_store::Pinned::File(n) => ("file", *n as i64, ""),
                katna_store::Pinned::Text(text) => ("text", 0, text.as_str()),
            };
            match pim
                .pin_in_chat(message.0, kind, file, text, label, replace.unwrap_or(0))
                .await
            {
                Ok(0) => return Err(katna_i18n::tr!("chat-pins-full")),
                other => other.map(|_| ()),
            }
        }
        Command::UnpinInChat(id) => pim.unpin_in_chat(*id).await,
        Command::OrderChatPins(ids) => pim.order_chat_pins(ids).await,
        Command::ContactLabels(cards) => {
            for (card, labels) in cards {
                pim.set_contact_labels(*card, labels)
                    .await
                    .map_err(|err| describe(&err))?;
            }
            return Ok(());
        }
        Command::RenameContactLabel(old, new) => pim.rename_contact_label(old, new).await,
        Command::DeleteContacts(ids) => pim.delete_contacts(ids).await,
        Command::WriteCards(cards) => {
            for c in cards {
                let json = serde_json::to_string(&c.card).map_err(|err| err.to_string())?;
                let id = pim
                    .save_contact(c.id, c.book, &json)
                    .await
                    .map_err(|err| describe(&err))?;
                pim.set_contact_labels(id, &c.labels)
                    .await
                    .map_err(|err| describe(&err))?;
            }
            return Ok(());
        }
        Command::RelabelNotes(ids, old, new) => pim.relabel_notes(ids, old, new).await.map(|_| ()),
        Command::ReopenDraft
        | Command::RestoreQuote
        | Command::UndoRephrase
        | Command::RestoreSubject(_)
        | Command::RestoreContacts(_)
        | Command::RestoreScheme(..)
        | Command::TurnAppOn(_) => {
            return Ok(());
        }
        Command::Event(change) => return edit_event(connection, change).await.map(|_| ()),
        Command::Task(task) => return crate::tasks::send(connection, task).await.map(|_| ()),
        Command::Calendar(edit) => return edit_calendar(connection, edit).await.map(|_| ()),
        Command::Mute(what, until) => {
            let (kind, id, address) = what.args();
            pim.mute(kind, id, address, *until).await
        }
        Command::Unmute(what) => {
            let (kind, id, address) = what.args();
            pim.unmute(kind, id, address).await
        }
        Command::SetBell(folder, category, bell) => {
            let category = category.map_or(0, katna_core::MailCategory::to_storage);
            pim.set_bell(folder.0, category, bell.notify, bell.count)
                .await
        }
        Command::CloudTrash(account, ids, trashed) => {
            pim.cloud_trash(account.0, ids, *trashed).await.map(|_| ())
        }
        Command::CloudRename(account, id, name) => pim.cloud_rename(account.0, id, name).await,
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

/// Saves `card` over saved card `contact`, or as a new card in address
/// book `book` (0: this computer) when `contact` is 0. Returns its id.
pub async fn save_contact(
    connection: &Connection,
    contact: i64,
    book: i64,
    card: &katna_core::contact::Card,
) -> Result<i64, String> {
    let json = serde_json::to_string(card).map_err(|err| err.to_string())?;
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.save_contact(contact, book, &json)
        .await
        .map_err(|err| describe(&err))
}

/// Saves one of Google's other contacts; returns the new card, or 0 when
/// it comes with the next sync.
pub async fn save_other_contact(connection: &Connection, id: i64) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.save_other_contact(id)
        .await
        .map_err(|err| describe(&err))
}

/// Saves cards read from a file, with their labels, as new contacts in
/// address book `book`; returns their ids.
pub async fn import_contacts(
    connection: &Connection,
    book: i64,
    cards: &[(katna_core::contact::Card, Vec<String>)],
) -> Result<Vec<i64>, String> {
    let json = serde_json::Value::Array(
        cards
            .iter()
            .map(|(card, labels)| serde_json::json!({ "card": card, "labels": labels }))
            .collect(),
    )
    .to_string();
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.import_contacts(book, &json)
        .await
        .map_err(|err| describe(&err))
}

/// Deletes saved cards `ids`, from their accounts too.
pub async fn delete_contacts(connection: &Connection, ids: &[i64]) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.delete_contacts(ids).await.map_err(|err| describe(&err))
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

/// Mail rules (`docs/ARCHITECTURE.md` §9.4). The app reads them from the
/// store (`katna_store::rules`) and previews them there
/// (`Store::rule_preview`); these change them.
pub mod rules {
    use futures_lite::{Stream, StreamExt};
    use katna_dbus::PimProxy;
    use katna_dbus::zbus::Connection;
    use katna_store::rules::Rule;

    use super::describe;

    /// Saves `rule` (a new one when its ID is 0). Returns its ID.
    pub async fn save(connection: &Connection, rule: &Rule) -> Result<i64, String> {
        let json = serde_json::to_string(rule).map_err(|err| err.to_string())?;
        let pim = PimProxy::new(connection)
            .await
            .map_err(|err| describe(&err))?;
        pim.save_rule(&json).await.map_err(|err| describe(&err))
    }

    /// Deletes rule `id`.
    pub async fn delete(connection: &Connection, id: i64) -> Result<(), String> {
        let pim = PimProxy::new(connection)
            .await
            .map_err(|err| describe(&err))?;
        pim.delete_rule(id).await.map_err(|err| describe(&err))
    }

    /// Puts rules `ids` first, in this order.
    pub async fn reorder(connection: &Connection, ids: &[i64]) -> Result<(), String> {
        let pim = PimProxy::new(connection)
            .await
            .map_err(|err| describe(&err))?;
        pim.reorder_rules(ids).await.map_err(|err| describe(&err))
    }

    /// Switches rule `id` on or off.
    pub async fn set_enabled(connection: &Connection, id: i64, on: bool) -> Result<(), String> {
        let pim = PimProxy::new(connection)
            .await
            .map_err(|err| describe(&err))?;
        pim.set_rule_enabled(id, on)
            .await
            .map_err(|err| describe(&err))
    }

    /// Runs rule `id` over the inbox mail of the last `days` days.
    /// Returns how many messages it changed.
    pub async fn apply(connection: &Connection, id: i64, days: u32) -> Result<u32, String> {
        let pim = PimProxy::new(connection)
            .await
            .map_err(|err| describe(&err))?;
        pim.apply_rule(id, days).await.map_err(|err| describe(&err))
    }

    /// Fires when the rules changed.
    pub async fn changes(connection: &Connection) -> Result<impl Stream<Item = ()>, String> {
        let pim = PimProxy::new(connection)
            .await
            .map_err(|err| describe(&err))?;
        let changes = pim
            .receive_rules_changed()
            .await
            .map_err(|err| describe(&err))?;
        Ok(changes.map(|_| ()))
    }
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

/// A calendar change on a calendar itself, sent to its service first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarEdit {
    /// A new calendar in account (0: this computer), with a name and a
    /// colour.
    Add(i64, String, String),
    Rename(i64, String),
    Recolor(i64, String),
    /// Deleted for everyone (`true`) or taken off the person's list.
    Delete(i64, bool),
}

/// Sends `edit` (`AddCalendar`, `RenameCalendar`, `SetCalendarColor`,
/// `DeleteCalendar`). Returns the new calendar's ID, or 0.
pub async fn edit_calendar(connection: &Connection, edit: &CalendarEdit) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let done = match edit {
        CalendarEdit::Add(account, name, color) => pim.add_calendar(*account, name, color).await,
        CalendarEdit::Rename(id, name) => pim.rename_calendar(*id, name).await.map(|()| 0),
        CalendarEdit::Recolor(id, color) => pim.set_calendar_color(*id, color).await.map(|_| 0),
        CalendarEdit::Delete(id, delete) => pim.delete_calendar(*id, *delete).await.map(|()| 0),
    };
    done.map_err(|err| describe(&err))
}

/// Asks the daemon for a change to the calendar. Returns the ID of the
/// event added or changed, or 0.
pub async fn edit_event(
    connection: &Connection,
    change: &katna_store::calendar::EventChange,
) -> Result<i64, String> {
    let json = serde_json::to_string(change).map_err(|err| err.to_string())?;
    let reply = connection
        .call_method(
            Some(katna_core::ids::DAEMON_BUS_NAME),
            katna_core::ids::PIM_OBJECT_PATH,
            Some(katna_core::ids::PIM_INTERFACE),
            "EditEvent",
            &(json,),
        )
        .await
        .map_err(|err| describe(&err))?;
    reply
        .body()
        .deserialize::<i64>()
        .map_err(|err| err.to_string())
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

/// What came back from rephrasing: the text, and the plan it was done
/// under (`trial` with its days left, `paid`, or `own`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rephrased {
    pub text: String,
    pub plan: String,
    pub days_left: u32,
}

/// Rephrases `text` in `tone` with the AI service the settings name, or
/// gives a `katna_ai::wire::problem`.
pub async fn ai_rephrase(
    connection: &Connection,
    text: &str,
    tone: &str,
    instruction: &str,
) -> Result<Rephrased, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|_| katna_ai::wire::problem::FAILED.to_owned())?;
    let (text, plan, days_left, problem) = pim
        .ai_rephrase(text, tone, instruction)
        .await
        .map_err(|_| katna_ai::wire::problem::FAILED.to_owned())?;
    if problem.is_empty() {
        Ok(Rephrased {
            text,
            plan,
            days_left,
        })
    } else {
        Err(problem)
    }
}

/// Sums up a conversation: `request` is a
/// `katna_ai::summary::SummarizeRequest` and `newest` the newest of its
/// mails sent. Gives the summary (as JSON) with its plan, or a
/// `katna_ai::wire::problem`.
pub async fn ai_summarize(
    connection: &Connection,
    newest: MessageId,
    request: &katna_ai::summary::SummarizeRequest,
) -> Result<Rephrased, String> {
    let failed = || katna_ai::wire::problem::FAILED.to_owned();
    let request = serde_json::to_string(request).map_err(|_| failed())?;
    let pim = PimProxy::new(connection).await.map_err(|_| failed())?;
    let (text, plan, days_left, problem) = pim
        .ai_summarize(newest.0, &request)
        .await
        .map_err(|_| failed())?;
    if problem.is_empty() {
        Ok(Rephrased {
            text,
            plan,
            days_left,
        })
    } else {
        Err(problem)
    }
}

/// A first draft of a reply or a forward's note, or ideas for one (a
/// JSON array of strings), with its plan, or a `katna_ai::wire::problem`.
pub async fn ai_draft(
    connection: &Connection,
    request: &katna_ai::draft::DraftRequest,
) -> Result<Rephrased, String> {
    let failed = || katna_ai::wire::problem::FAILED.to_owned();
    let request = serde_json::to_string(request).map_err(|_| failed())?;
    let pim = PimProxy::new(connection).await.map_err(|_| failed())?;
    let (text, plan, days_left, problem) = pim.ai_draft(&request).await.map_err(|_| failed())?;
    if problem.is_empty() {
        Ok(Rephrased {
            text,
            plan,
            days_left,
        })
    } else {
        Err(problem)
    }
}

/// The rest of the sentence at the end of `before` (empty when unsure),
/// or a `katna_ai::wire::problem`.
pub async fn ai_complete(
    connection: &Connection,
    before: &str,
    answered: &str,
) -> Result<String, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|_| katna_ai::wire::problem::FAILED.to_owned())?;
    let (text, problem) = pim
        .ai_complete(before, answered)
        .await
        .map_err(|_| katna_ai::wire::problem::FAILED.to_owned())?;
    if problem.is_empty() {
        Ok(text)
    } else {
        Err(problem)
    }
}

/// Saves the key of the user's own AI service; empty deletes it.
pub async fn set_ai_key(connection: &Connection, key: &str) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.set_ai_key(key).await.map_err(|err| describe(&err))
}

/// Whether a key of the user's own AI service is saved.
pub async fn ai_key_saved(connection: &Connection) -> Result<bool, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.ai_key_saved().await.map_err(|err| describe(&err))
}

/// The models the user's own AI service `provider` offers to the saved
/// key, or a `katna_ai::wire::problem`.
pub async fn ai_models(
    connection: &Connection,
    provider: &str,
    address: &str,
) -> Result<Vec<String>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|_| katna_ai::wire::problem::FAILED.to_owned())?;
    let (models, problem) = pim
        .ai_models(provider, address)
        .await
        .map_err(|_| katna_ai::wire::problem::FAILED.to_owned())?;
    if problem.is_empty() {
        Ok(models)
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
    /// The IMAP and SMTP servers; an empty host was not found.
    pub account: NewImapAccount,
    /// The POP3 server; an empty host was not found. There is always an
    /// IMAP or a POP3 server.
    pub pop3: ServerSpec,
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
    let (account, pop3, source, sign_in, password) = pim
        .discover_account(address)
        .await
        .map_err(|err| describe(&err))?;
    Ok(Found {
        account,
        pop3,
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

/// Where account `id`'s mail sync stands, or `None` if there is no such
/// account.
pub async fn account_status(
    connection: &Connection,
    id: katna_core::AccountId,
) -> Result<Option<katna_dbus::AccountStatus>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let accounts = pim.accounts().await.map_err(|err| describe(&err))?;
    Ok(accounts.into_iter().find(|a| a.id == id.0))
}

/// Where each account's calendar sync stands, by account.
pub async fn calendar_status(
    connection: &Connection,
) -> Result<HashMap<i64, AccountState>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let accounts = pim.accounts().await.map_err(|err| describe(&err))?;
    let status = pim.calendar_status().await.map_err(|err| describe(&err))?;
    Ok(status
        .into_iter()
        .map(|(id, state, detail)| {
            let sign_in = accounts
                .iter()
                .find(|a| a.id == id)
                .and_then(|a| a.sign_in.parse().ok());
            (
                id,
                AccountState {
                    state,
                    detail,
                    sign_in,
                },
            )
        })
        .collect())
}

/// Where each account's contacts sync stands, by account: the state is a
/// [`katna_dbus::contacts_state`].
pub async fn contacts_status(
    connection: &Connection,
) -> Result<HashMap<i64, AccountState>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let accounts = pim.accounts().await.map_err(|err| describe(&err))?;
    let status = pim.contacts_status().await.map_err(|err| describe(&err))?;
    Ok(status
        .into_iter()
        .map(|(id, state, detail)| {
            let sign_in = accounts
                .iter()
                .find(|a| a.id == id)
                .and_then(|a| a.sign_in.parse().ok());
            (
                id,
                AccountState {
                    state,
                    detail,
                    sign_in,
                },
            )
        })
        .collect())
}

/// Wakes `account`'s sync (mail, calendars, contacts and tasks) without
/// waiting.
pub async fn sync_now(connection: &Connection, account: i64) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.sync_now(account).await.map_err(|err| describe(&err))
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

/// Asks the daemon for the company of the person at `address`, the one at
/// `website` first: JSON, empty for none.
pub async fn company_of(
    connection: &Connection,
    address: &str,
    website: &str,
) -> Result<String, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.company_of(address, website)
        .await
        .map_err(|err| describe(&err))
}

/// The signatures Gmail adds for `account`: (address, name, HTML).
/// `Err(None)` when its sign-in does not allow reading them.
pub async fn gmail_signatures(
    connection: &Connection,
    account: i64,
) -> Result<Vec<(String, String, String)>, Option<String>> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| Some(describe(&err)))?;
    pim.gmail_signatures(account)
        .await
        .map_err(|err| match &err {
            katna_dbus::zbus::Error::MethodError(name, _, _)
                if name.as_str() == "org.freedesktop.DBus.Error.AuthFailed" =>
            {
                None
            }
            err => Some(describe(err)),
        })
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

/// Renames folder `folder` (a label, on Gmail) on its account's server;
/// it stays inside the same parent.
pub async fn rename_folder(
    connection: &Connection,
    folder: i64,
    new_name: &str,
) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.rename_folder(folder, new_name)
        .await
        .map_err(|err| describe(&err))
}

/// Deletes folder `folder` (a label, on Gmail) and the folders inside it
/// on its account's server. Returns how many messages went to the Trash.
pub async fn delete_folder(connection: &Connection, folder: i64) -> Result<u32, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.delete_folder(folder)
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

/// Checks the password with the POP3 server and adds the account.
/// Returns its ID.
pub async fn add_pop3_account(
    connection: &Connection,
    account: &NewPop3Account,
    password: &str,
) -> Result<i64, AddError> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| AddError::Other(describe(&err)))?;
    pim.add_pop3_account(account, password)
        .await
        .map_err(|err| add_error(&err))
}

/// Sets what a POP3 account does with mail on the server (see
/// `NewPop3Account`).
pub async fn set_pop3_keep(
    connection: &Connection,
    account: i64,
    keep: katna_core::Pop3Keep,
) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.set_pop3_keep(
        account,
        keep.leave_on_server,
        keep.days.unwrap_or(0),
        keep.delete_with_local,
    )
    .await
    .map_err(|err| describe(&err))
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

/// Whether the sign-in of `account` lets Katna upload into its drive.
pub async fn cloud_writable(connection: &Connection, account: i64) -> Result<bool, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_writable(account)
        .await
        .map_err(|err| describe(&err))
}

/// Starts uploading file or folder `path` into folder `folder` of the
/// drive of `account`; returns the upload's id.
pub async fn cloud_upload(
    connection: &Connection,
    account: i64,
    folder: &str,
    path: &str,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_upload(account, folder, path)
        .await
        .map_err(|err| describe(&err))
}

/// Who may open item `id` of the drive of `account`.
pub async fn cloud_access(
    connection: &Connection,
    account: i64,
    id: &str,
) -> Result<Vec<katna_dbus::CloudAccess>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_access(account, id)
        .await
        .map_err(|err| describe(&err))
}

/// Shares item `id` with `addresses` as `role`; returns those the drive
/// refused.
pub async fn cloud_grant(
    connection: &Connection,
    account: i64,
    id: &str,
    addresses: &[String],
    role: &str,
    notify: bool,
) -> Result<Vec<String>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_grant(account, id, addresses, role, notify)
        .await
        .map_err(|err| describe(&err))
}

/// Changes grant `permission` of item `id` to `role`; empty takes it away.
pub async fn cloud_set_access(
    connection: &Connection,
    account: i64,
    id: &str,
    permission: &str,
    role: &str,
) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_set_access(account, id, permission, role)
        .await
        .map_err(|err| describe(&err))
}

/// Opens item `id` to anyone with the link as `role`; empty closes it.
pub async fn cloud_set_link(
    connection: &Connection,
    account: i64,
    id: &str,
    role: &str,
) -> Result<(), String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_set_link(account, id, role)
        .await
        .map_err(|err| describe(&err))
}

/// Links a file already in the drive of `account` to a message; returns
/// an upload id that is shared at Send like an uploaded file's.
pub async fn cloud_link(
    connection: &Connection,
    account: i64,
    entry: &katna_dbus::CloudEntry,
) -> Result<i64, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_link(account, entry)
        .await
        .map_err(|err| describe(&err))
}

/// One page of the drive of `account`: `place` is one of
/// `katna_dbus::cloud_place`, `what` the folder id or the words.
pub async fn cloud_list(
    connection: &Connection,
    account: i64,
    place: &str,
    what: &str,
    page: &str,
) -> Result<katna_dbus::CloudListing, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_list(account, place, what, page)
        .await
        .map_err(|err| describe(&err))
}

/// Downloads a drive file into Katna's cache; returns its path.
pub async fn cloud_fetch(
    connection: &Connection,
    account: i64,
    entry: &katna_dbus::CloudEntry,
) -> Result<String, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_fetch(account, entry)
        .await
        .map_err(|err| describe(&err))
}

/// The picture of a drive file, about `width` pixels wide.
pub async fn cloud_thumbnail(
    connection: &Connection,
    account: i64,
    link: &str,
    width: u32,
) -> Result<Vec<u8>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.cloud_thumbnail(account, link, width)
        .await
        .map_err(|err| describe(&err))
}

/// A new video call link from the mail service of `account`; empty when
/// it has none Katna may make.
pub async fn meeting_link(connection: &Connection, account: i64) -> Result<String, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    pim.meeting_link(account)
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
    Ok(accounts.iter().any(|a| {
        a.last_sync == 0
            && ![state::NOT_SYNCED, state::AUTH_FAILED, state::PAUSED].contains(&a.state.as_str())
    }))
}

/// Has the daemon check `account` for new mail now, every folder of it, or
/// every account for `None`. Returns once each has finished that sync or
/// failed to connect; an account the daemon does not sync is not waited for.
pub async fn check_mail(
    connection: &Connection,
    account: Option<katna_core::AccountId>,
) -> Result<(), String> {
    check(connection, account, &[]).await
}

/// Like [`check_mail`], for only `folders` (each with its account).
pub async fn check_folders(
    connection: &Connection,
    folders: &[(katna_core::AccountId, katna_store::FolderId)],
) -> Result<(), String> {
    check(connection, None, folders).await
}

async fn check(
    connection: &Connection,
    account: Option<katna_core::AccountId>,
    folders: &[(katna_core::AccountId, katna_store::FolderId)],
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
        .filter(|a| {
            if folders.is_empty() {
                account.is_none_or(|id| id.0 == a.id)
            } else {
                folders.iter().any(|(id, _)| id.0 == a.id)
            }
        })
        .filter(|a| a.state != state::NOT_SYNCED && a.state != state::PAUSED)
        .map(|a| a.id)
        .collect();
    if folders.is_empty() {
        pim.sync_now(account.map_or(0, |id| id.0))
            .await
            .map_err(|err| describe(&err))?;
    }
    for (_, folder) in folders {
        pim.sync_folder(folder.0)
            .await
            .map_err(|err| describe(&err))?;
    }
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
