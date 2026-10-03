// SPDX-License-Identifier: GPL-3.0-or-later

//! Quick capture from the desktop's search (`docs/ARCHITECTURE.md` §15.3):
//! `task: Call the plumber fri 6pm #home` or `note: Ideas for the garden`
//! in KRunner gives one result, and Enter adds the task or note at once,
//! read as Katna Mail's quick capture card reads it (the day, time,
//! repeat and labels typed; the task in the default list, the note in
//! the first mail account's Notes). The result's button opens the card
//! with the text instead, to change it first; `task:` alone opens the
//! empty card.

use jiff::tz::TimeZone;
use katna_core::{AccountId, AccountKind, ids};
use katna_dav::quick_task::{self, TypedTask, default_list};
use katna_dbus::agenda::{Item, edit};
use katna_dbus::{NoteItem, app_action};
use katna_i18n::tr;
use katna_store::Store;
use katna_store::tasks::TaskFields;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{OwnedValue, Value};

use crate::daemon::{CommandError, Daemon, Notice};
use crate::desktop_search::Found;
use crate::mail_app;

/// The first letter of a capture result's ID: then `t` or `n`, and the
/// text typed.
pub(crate) const CAPTURE_ID: char = 'q';
const TASK: char = 't';
const NOTE: char = 'n';

/// What starts a task or a note, in any case.
const TASK_PREFIX: &str = "task:";
const NOTE_PREFIX: &str = "note:";

/// The capture result's button: opens the card with the text.
pub(crate) const EDIT: &str = "capture";

/// Above everything else: the words were meant for it.
const RELEVANCE: f64 = 1.0;
/// KRunner's `QueryMatch::CategoryRelevance::Highest`.
const HIGHEST: i32 = 100;

/// A task or note to add, from what was typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Capture {
    pub note: bool,
    pub text: String,
}

/// What follows `task:` or `note:` at the start of `text`.
pub(crate) fn typed(text: &str) -> Option<Capture> {
    let text = text.trim_start();
    for (prefix, note) in [(TASK_PREFIX, false), (NOTE_PREFIX, true)] {
        if let Some(head) = text.get(..prefix.len())
            && head.eq_ignore_ascii_case(prefix)
        {
            return Some(Capture {
                note,
                text: text[prefix.len()..].trim().to_owned(),
            });
        }
    }
    None
}

/// The capture a result ID stands for.
pub(crate) fn parse_id(id: &str) -> Option<Capture> {
    let mut chars = id.chars();
    if chars.next()? != CAPTURE_ID {
        return None;
    }
    let note = match chars.next()? {
        TASK => false,
        NOTE => true,
        _ => return None,
    };
    Some(Capture {
        note,
        text: chars.as_str().to_owned(),
    })
}

impl Capture {
    fn id(&self) -> String {
        let kind = if self.note { NOTE } else { TASK };
        format!("{CAPTURE_ID}{kind}{}", self.text)
    }

    /// The one result: what Enter adds, where to, and when it is due.
    pub(crate) fn found(&self, store: Option<&Store>) -> Found {
        let (text, subtext) = if self.note {
            let (body, _) = quick_task::take_labels(&self.text);
            let place = store
                .and_then(|s| note_account(s).ok().flatten())
                .and_then(|id| {
                    let accounts = store?.accounts().ok()?;
                    accounts.into_iter().find(|a| a.id == id)
                })
                .map(|a| a.display_name);
            (
                if body.is_empty() {
                    tr!("search-new-note")
                } else {
                    tr!("search-add-note", title = body)
                },
                match place {
                    Some(place) => tr!("search-add-note-to", place = place),
                    None => tr!("search-add-note-here"),
                },
            )
        } else {
            let typed = typed_task(&self.text);
            let list = store
                .and_then(|s| s.task_lists().ok())
                .and_then(|lists| default_list(&lists).map(|l| l.title.clone()))
                .unwrap_or_default();
            let when = typed.due.as_deref().and_then(when);
            (
                if self.text.is_empty() {
                    tr!("search-new-task")
                } else {
                    tr!("search-add-task", title = typed.title)
                },
                match when {
                    Some(when) => tr!("search-add-task-when", when = when, list = list),
                    None if list.is_empty() => String::new(),
                    None => tr!("search-add-task-to", list = list),
                },
            )
        };
        Found {
            id: self.id(),
            text,
            subtext,
            icon: if self.note {
                "view-pim-notes"
            } else {
                "task-new"
            },
            kind: HIGHEST,
            relevance: RELEVANCE,
        }
    }

    /// Enter on the result: adds the task or note, or opens the card when
    /// nothing was typed yet. `edit` (the button) opens the card with the
    /// text.
    pub(crate) async fn run(
        &self,
        connection: &zbus::Connection,
        daemon: Option<&Daemon>,
        edit: bool,
        token: Option<String>,
    ) {
        let open_card = edit || self.text.is_empty() || daemon.is_none();
        if let Some(daemon) = daemon.filter(|_| !open_card) {
            let added = if self.note {
                add_note(daemon, &self.text)
            } else {
                add_task(connection, daemon, &self.text).await
            };
            match added {
                Ok(()) => return,
                Err(err) => tracing::warn!(%err, note = self.note, "quick capture from search"),
            }
        }
        let kind = if self.note {
            app_action::CAPTURE_NOTE
        } else {
            app_action::CAPTURE_TASK
        };
        let param = vec![Value::from(app_action::capture(kind, &self.text))];
        mail_app::run(connection, Some(app_action::CAPTURE), param, token).await;
    }
}

/// "Today", "Tomorrow" or "In 3 days" for a due day (the service formats
/// no dates).
fn when(due: &str) -> Option<String> {
    let day: jiff::civil::Date = due.parse().ok()?;
    let today = jiff::Zoned::now().date();
    Some(match (day - today).get_days() {
        ..=0 => tr!("search-event-today"),
        1 => tr!("search-event-tomorrow"),
        count => tr!("search-event-in-days", count = count),
    })
}

fn typed_task(text: &str) -> TypedTask {
    let language = katna_i18n::current().language.tag.clone();
    quick_task::parse(text, jiff::Zoned::now().date(), &language)
}

/// Where a new note is kept: the first account whose server keeps
/// folders; `None` for this computer.
fn note_account(store: &Store) -> katna_store::Result<Option<AccountId>> {
    Ok(store
        .accounts()?
        .into_iter()
        .find(|a| a.kind == AccountKind::Imap)
        .map(|a| a.id))
}

/// Adds the task typed as `text` to the default list.
async fn add_task(
    connection: &zbus::Connection,
    daemon: &Daemon,
    text: &str,
) -> Result<(), CommandError> {
    let typed = typed_task(text);
    let title = crate::agenda::title(&typed.title)?;
    let fields = typed_fields(&typed);
    let id = {
        let mut store = daemon.store();
        let list = store.default_task_list()?;
        let id = store.add_task_to(
            list,
            None,
            &TaskFields {
                title,
                ..TaskFields::default()
            },
        )?;
        let task = store
            .task(id)?
            .ok_or_else(|| CommandError::Failed("the task went away".into()))?;
        let edited = crate::agenda::edited(task, &fields)?;
        store.edit_task(id, &edited)?;
        id
    };
    tracing::info!(id, "task added from the desktop's search");
    daemon.wake_task_sync();
    let emitter = SignalEmitter::new(connection, ids::AGENDA_OBJECT_PATH)
        .map_err(|err| CommandError::Failed(err.to_string()))?;
    if let Err(err) = crate::agenda::changed(&emitter).await {
        tracing::debug!(%err, "tasks changed");
    }
    Ok(())
}

/// What `typed` says beyond the title, as EditTask's fields: the day, the
/// time with a reminder at it, the repeat and the labels.
fn typed_fields(typed: &TypedTask) -> Item {
    let mut fields = Item::new();
    let mut put = |key: &str, value: Value<'_>| {
        if let Ok(value) = OwnedValue::try_from(value) {
            fields.insert(key.to_owned(), value);
        }
    };
    if let Some(due) = &typed.due {
        put(edit::DUE, due.as_str().into());
        if let Some(minutes) = typed.due_time {
            put(edit::DUE_TIME, i32::try_from(minutes).unwrap_or(-1).into());
            if let Some(at) = katna_dav::todo::due_at(due, Some(minutes), &TimeZone::system()) {
                put(edit::REMIND_AT, at.into());
            }
        }
    }
    if let Some(repeat) = &typed.repeat {
        put(edit::REPEAT, repeat.as_str().into());
    }
    if !typed.labels.is_empty() {
        put(edit::LABELS, typed.labels.clone().into());
    }
    fields
}

/// Adds the note typed as `text`: its words, with the labels typed after
/// `#` (each spelled as a label the notes already have, if one is).
fn add_note(daemon: &Daemon, text: &str) -> Result<(), CommandError> {
    let (body, typed_labels) = quick_task::take_labels(text);
    let (account, known) = {
        let store = daemon.store();
        let known: Vec<String> = store.notes()?.into_iter().flat_map(|n| n.labels).collect();
        (note_account(&store)?, known)
    };
    let labels = typed_labels
        .into_iter()
        .map(|label| {
            known
                .iter()
                .find(|k| k.to_lowercase() == label.to_lowercase())
                .cloned()
                .unwrap_or(label)
        })
        .collect();
    let account = account.map_or(0, |a| a.0);
    let body = if body.is_empty() {
        text.trim().to_owned()
    } else {
        body
    };
    let id = daemon.save_note(NoteItem {
        account,
        body,
        labels,
        ..NoteItem::default()
    })?;
    tracing::info!(id, "note added from the desktop's search");
    // Katna Mail reads its notes again.
    let _ = daemon
        .notices()
        .try_send(Notice::MailChanged(AccountId(account)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_are_found_in_any_case() {
        assert_eq!(
            typed("  Task: Call the plumber fri 6pm "),
            Some(Capture {
                note: false,
                text: "Call the plumber fri 6pm".into()
            })
        );
        assert_eq!(
            typed("note:"),
            Some(Capture {
                note: true,
                text: String::new()
            })
        );
        assert_eq!(typed("tasks for today"), None);
        assert_eq!(typed("notebook"), None);
        // Not a character boundary at the prefix's length.
        assert_eq!(typed("টাস্কঃ"), None);
    }

    #[test]
    fn ids_round_trip() {
        for capture in [
            Capture {
                note: false,
                text: "Buy milk: 2 litres".into(),
            },
            Capture {
                note: true,
                text: String::new(),
            },
        ] {
            assert_eq!(parse_id(&capture.id()), Some(capture));
        }
        assert_eq!(parse_id("t42"), None);
        assert_eq!(parse_id("qx"), None);
    }

    #[test]
    fn typed_fields_carry_a_reminder_at_the_time() {
        let typed = TypedTask {
            title: "Call".into(),
            due: Some("2026-10-09".into()),
            due_time: Some(18 * 60),
            repeat: None,
            labels: vec!["home".into()],
        };
        let fields = typed_fields(&typed);
        assert!(fields.contains_key(edit::DUE));
        assert!(fields.contains_key(edit::DUE_TIME));
        assert!(fields.contains_key(edit::REMIND_AT));
        assert!(fields.contains_key(edit::LABELS));
        assert!(!fields.contains_key(edit::REPEAT));
    }
}
