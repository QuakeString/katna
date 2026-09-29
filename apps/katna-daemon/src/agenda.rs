// SPDX-License-Identifier: GPL-3.0-or-later

//! Events and tasks for the desktop's clock: `in.invenia.katna.Agenda1`
//! (wire format in [`katna_dbus::agenda`]).
//!
//! Tasks are those of every list in `pim.db`: the accounts' own (Google
//! Tasks, To Do), synced by the daemon, and the ones on this computer. A
//! task added here goes to the first account's default list once one has
//! synced. Events are those of the calendars the daemon syncs
//! (`daemon/calendar.rs`), repeating ones expanded in this computer's time
//! zone ([`katna_dav::occurrences`]).

use std::collections::HashMap;
use std::sync::Arc;

use jiff::tz::TimeZone;
use katna_core::{Paths, ids};
use katna_dbus::agenda::{Item, event, task};
use katna_store::tasks::Task;
use zbus::fdo;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{OwnedValue, Value};

use crate::daemon::{CommandError, Daemon};

/// The longest task title, in characters.
const MAX_TITLE: usize = 500;
/// Ticked-off tasks stay in the list this long, so a wrong tick can be
/// taken back.
const DONE_SHOWN_SECS: i64 = 24 * 60 * 60;
/// A task's ID on the wire: this letter and its row ID.
const TASK_ID: char = 't';
/// An event's ID on the wire: this letter, its row ID, `:` and the
/// occurrence's start.
const EVENT_ID: char = 'e';
/// The longest range `Events` takes, so a caller can't have a series
/// expanded for centuries.
const MAX_EVENTS_RANGE: i64 = 400 * 24 * 60 * 60;
/// Written once Katna's GNOME Shell extension was switched on, so it is
/// switched on only once: after that, turning it off is the user's choice.
const EXTENSION_ENABLED_FILE: &str = "gnome-clock-extension-enabled";

pub(crate) struct AgendaService {
    daemon: Arc<Daemon>,
}

/// Serves the interface on `connection`.
pub(crate) async fn serve(connection: &zbus::Connection, daemon: Arc<Daemon>) -> zbus::Result<()> {
    connection
        .object_server()
        .at(ids::AGENDA_OBJECT_PATH, AgendaService { daemon })
        .await?;
    Ok(())
}

/// On GNOME, switches on Katna's clock extension the first time GNOME
/// Shell knows it. A newly installed extension is found at the next login,
/// so until it is, this tries again at each start.
pub(crate) async fn enable_gnome_extension(connection: &zbus::Connection, paths: &Paths) {
    let marker = paths.state_dir().join(EXTENSION_ENABLED_FILE);
    if marker.exists() {
        return;
    }
    let Ok(dbus) = fdo::DBusProxy::new(connection).await else {
        return;
    };
    let shell = zbus::names::BusName::from_static_str("org.gnome.Shell").expect("valid name");
    if !dbus.name_has_owner(shell).await.unwrap_or(false) {
        return;
    }
    let enabled: zbus::Result<bool> = connection
        .call_method(
            Some("org.gnome.Shell.Extensions"),
            "/org/gnome/Shell/Extensions",
            Some("org.gnome.Shell.Extensions"),
            "EnableExtension",
            &(ids::CLOCK_EXTENSION_UUID,),
        )
        .await
        .and_then(|reply| reply.body().deserialize());
    match enabled {
        Ok(true) => {
            tracing::info!("GNOME clock extension switched on");
            if let Err(err) = std::fs::write(&marker, b"") {
                tracing::warn!(%err, "could not note the GNOME extension");
            }
        }
        Ok(false) => tracing::info!("GNOME Shell doesn't know the clock extension yet"),
        Err(err) => tracing::warn!(%err, "could not switch on the GNOME clock extension"),
    }
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

/// The row ID in a task's wire ID.
fn task_id(id: &str) -> Result<i64, CommandError> {
    id.strip_prefix(TASK_ID)
        .and_then(|n| n.parse().ok())
        .ok_or_else(|| CommandError::InvalidArgs(format!("not a task ID: {id:?}")))
}

/// A title, trimmed to one line; not empty and not too long.
fn title(text: &str) -> Result<String, CommandError> {
    let title = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if title.is_empty() {
        return Err(CommandError::InvalidArgs("a task needs a title".to_owned()));
    }
    if title.chars().count() > MAX_TITLE {
        return Err(CommandError::InvalidArgs(format!(
            "a task title is at most {MAX_TITLE} characters"
        )));
    }
    Ok(title)
}

/// `YYYY-MM-DD` of a real day, or empty.
fn due(text: &str) -> Result<&str, CommandError> {
    if text.is_empty() {
        return Ok(text);
    }
    let number = |part: &str| {
        part.bytes()
            .all(|b| b.is_ascii_digit())
            .then(|| part.parse::<u32>().ok())
            .flatten()
    };
    let parts: Vec<&str> = text.split('-').collect();
    let valid = match parts.as_slice() {
        [y, m, d] if y.len() == 4 && m.len() == 2 && d.len() == 2 => {
            match (number(y), number(m), number(d)) {
                (Some(year), Some(month @ 1..=12), Some(day)) => {
                    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
                    let days = match month {
                        2 if leap => 29,
                        2 => 28,
                        4 | 6 | 9 | 11 => 30,
                        _ => 31,
                    };
                    (1..=days).contains(&day)
                }
                _ => false,
            }
        }
        _ => false,
    };
    if valid {
        Ok(text)
    } else {
        Err(CommandError::InvalidArgs(format!(
            "a due day is YYYY-MM-DD, not {text:?}"
        )))
    }
}

fn value(value: Value<'_>) -> OwnedValue {
    // Plain strings and numbers always convert.
    OwnedValue::try_from(value).expect("no file descriptors")
}

fn wire(task: Task, lists: &HashMap<i64, String>) -> Item {
    let list = lists.get(&task.list).cloned().unwrap_or_default();
    Item::from([
        (
            task::ID.to_owned(),
            value(format!("{TASK_ID}{}", task.id).into()),
        ),
        (task::TITLE.to_owned(), value(task.title.into())),
        (task::NOTES.to_owned(), value(task.notes.into())),
        (task::DUE.to_owned(), value(task.due.into())),
        (task::DONE.to_owned(), value(task.done_at.is_some().into())),
        (task::LIST.to_owned(), value(list.into())),
    ])
}

/// Tells the clock to read again (calendars changed).
pub(crate) async fn changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()> {
    AgendaService::changed(emitter).await
}

impl AgendaService {
    /// The occurrences of events overlapping `from..to`.
    fn event_list(&self, from: i64, to: i64) -> Result<Vec<Item>, CommandError> {
        if to <= from {
            return Ok(Vec::new());
        }
        let to = to.min(from.saturating_add(MAX_EVENTS_RANGE));
        let (rows, calendars) = {
            let store = self.daemon.store();
            (store.event_rows_in_range(from, to)?, store.calendars()?)
        };
        let calendars: std::collections::HashMap<i64, _> =
            calendars.into_iter().map(|c| (c.id, c)).collect();
        let tz = TimeZone::system();
        Ok(katna_dav::occurrences(rows, from, to, &tz)
            .into_iter()
            .map(|occurrence| {
                let data = &occurrence.event.data;
                let calendar = calendars.get(&occurrence.event.calendar_id);
                let color = if data.color.is_empty() {
                    calendar.map(|c| c.color.clone()).unwrap_or_default()
                } else {
                    data.color.clone()
                };
                Item::from([
                    (
                        event::ID.to_owned(),
                        value(
                            format!("{EVENT_ID}{}:{}", occurrence.event.id, occurrence.start)
                                .into(),
                        ),
                    ),
                    (event::TITLE.to_owned(), value(data.title.clone().into())),
                    (event::START.to_owned(), value(occurrence.start.into())),
                    (event::END.to_owned(), value(occurrence.end.into())),
                    (event::ALL_DAY.to_owned(), value(data.all_day.into())),
                    (
                        event::LOCATION.to_owned(),
                        value(data.location.clone().into()),
                    ),
                    (event::COLOR.to_owned(), value(color.into())),
                    (
                        event::CALENDAR.to_owned(),
                        value(calendar.map(|c| c.name.clone()).unwrap_or_default().into()),
                    ),
                    (
                        event::JOIN_URL.to_owned(),
                        value(data.join_url.clone().into()),
                    ),
                ])
            })
            .collect())
    }

    fn task_list(&self) -> Result<Vec<Item>, CommandError> {
        let store = self.daemon.store();
        let lists: HashMap<i64, String> = store
            .task_lists()?
            .into_iter()
            .map(|list| (list.id, list.title))
            .collect();
        let tasks = store.tasks(now() - DONE_SHOWN_SECS)?;
        Ok(tasks.into_iter().map(|task| wire(task, &lists)).collect())
    }
}

macro_rules! agenda_interface {
    ($interface:tt, $bus_name:tt, $path:tt) => {
        #[zbus::interface(name = $interface)]
        impl AgendaService {
            async fn events(&self, from: i64, to: i64) -> fdo::Result<Vec<Item>> {
                Ok(self.event_list(from, to)?)
            }

            async fn tasks(&self) -> fdo::Result<Vec<Item>> {
                Ok(self.task_list()?)
            }

            async fn add_task(
                &self,
                #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
                title: String,
                due: String,
            ) -> fdo::Result<String> {
                let title = self::title(&title)?;
                let due = self::due(&due)?;
                let id = self
                    .daemon
                    .store()
                    .add_task(&title, due)
                    .map_err(CommandError::from)?;
                tracing::info!(id, "task added");
                self.daemon.wake_task_sync();
                Self::changed(&emitter).await?;
                Ok(format!("{TASK_ID}{id}"))
            }

            async fn set_task_done(
                &self,
                #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
                id: String,
                done: bool,
            ) -> fdo::Result<()> {
                let row = task_id(&id)?;
                let found = self
                    .daemon
                    .store()
                    .set_task_done(row, done)
                    .map_err(CommandError::from)?;
                if !found {
                    return Err(fdo::Error::UnknownObject(format!("no task {id}")));
                }
                self.daemon.wake_task_sync();
                Self::changed(&emitter).await?;
                Ok(())
            }

            async fn delete_task(
                &self,
                #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
                id: String,
            ) -> fdo::Result<()> {
                let row = task_id(&id)?;
                let deleted = self
                    .daemon
                    .store()
                    .delete_task(row)
                    .map_err(CommandError::from)?;
                if deleted.is_none() {
                    return Err(fdo::Error::UnknownObject(format!("no task {id}")));
                }
                tracing::info!(id = row, "task deleted");
                self.daemon.wake_task_sync();
                Self::changed(&emitter).await?;
                Ok(())
            }

            /// Katna has no calendar or tasks page yet to show one in
            /// from here.
            async fn open(&self, _id: String) -> bool {
                false
            }

            #[zbus(signal)]
            pub(crate) async fn changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
        }
    };
}

katna_core::with_agenda_names!(agenda_interface);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_ids_round_trip() {
        assert_eq!(task_id("t42").unwrap(), 42);
        assert!(task_id("42").is_err());
        assert!(task_id("tx").is_err());
        assert!(task_id("e42").is_err());
    }

    #[test]
    fn titles_are_one_trimmed_line() {
        assert_eq!(
            title("  Book\n train   tickets ").unwrap(),
            "Book train tickets"
        );
        assert!(title(" \n ").is_err());
        assert!(title(&"x".repeat(MAX_TITLE + 1)).is_err());
    }

    #[test]
    fn due_days_are_real_days() {
        assert_eq!(due("").unwrap(), "");
        assert_eq!(due("2026-09-29").unwrap(), "2026-09-29");
        assert!(due("2028-02-29").is_ok());
        for bad in [
            "2026-02-29",
            "2026-02-30",
            "2026-13-01",
            "2026-9-29",
            "29-09-2026",
            "tomorrow",
            "+026-09-29",
        ] {
            assert!(due(bad).is_err(), "{bad}");
        }
    }
}
