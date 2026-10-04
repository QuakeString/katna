// SPDX-License-Identifier: GPL-3.0-or-later

//! Event reminders (`docs/ARCHITECTURE.md` §18, §15.1): each reminder of
//! an event in the user's shown calendars becomes a notification at its
//! time, with Join when the event has a video call and Snooze. One task
//! looks at the next week of events, sleeps until the next reminder (at
//! most a minute, so new and changed events count), and remembers in the
//! store up to when it looked, so a restart neither repeats reminders nor
//! brings back ones long past: those missed while the computer was off
//! show only when they fell due in the last few minutes.
//!
//! Tasks' reminders come the same way, with Mark as done and Snooze, and
//! notes' with Open and Snooze.

use std::{collections::HashSet, sync::Weak, time::Duration};

use jiff::tz::TimeZone;
use katna_dav::Occurrence;
use katna_i18n::tr;
use katna_store::{
    Store,
    calendar::{Calendar, EventStatus},
    tasks::Task,
};

use super::{Daemon, unix_now};

/// How far ahead events are read: the longest reminder Google offers is
/// four weeks, but a week covers what people set.
const AHEAD: i64 = 8 * 24 * 3600;
/// A reminder missed while the computer was off still shows when it fell
/// due this recently.
const MISSED: i64 = 10 * 60;
/// The longest sleep: new and changed events count within it.
const LOOK_EVERY: i64 = 60;
/// Where the time looked up to is kept (`meta`).
const META_KIND: &str = "calendar";
const META_KEY: &str = "alarms";
/// Snooze reminds again after this long.
pub(crate) const SNOOZE: i64 = 5 * 60;

/// A reminder to show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Alarm {
    /// The event's row and its occurrence's start: one reminder each.
    pub key: (i64, i64),
    pub title: String,
    pub lines: Vec<String>,
    pub join_url: String,
    /// When it is shown: the start less the reminder.
    pub at: i64,
    /// The task it is about, for a task's reminder; `key` is then the
    /// task's row and its reminder time.
    pub task: Option<i64>,
    /// The note it is about, for a note's reminder; `key` is then the
    /// note's row and its reminder time.
    pub note: Option<i64>,
}

/// The reminders due in `(from, to]` of `occurrences`, soonest first, and
/// the time of the next one after `to`. Events in hidden calendars,
/// cancelled ones and those the user declined have none; one event with
/// several reminders due at once shows once.
fn due(
    occurrences: &[Occurrence],
    calendars: &[Calendar],
    from: i64,
    to: i64,
) -> (Vec<Alarm>, Option<i64>) {
    let mut alarms = Vec::new();
    let mut next: Option<i64> = None;
    let mut seen = HashSet::new();
    for occurrence in occurrences {
        let data = &occurrence.event.data;
        let hidden = calendars
            .iter()
            .any(|c| c.id == occurrence.event.calendar_id && c.hidden);
        if hidden || data.status == EventStatus::Cancelled || data.self_status == "declined" {
            continue;
        }
        for minutes in &data.reminders {
            let at = occurrence.start - minutes * 60;
            if at > to {
                next = Some(next.map_or(at, |n| n.min(at)));
                continue;
            }
            let key = (occurrence.event.id, occurrence.start);
            if at <= from || !seen.insert(key) {
                continue;
            }
            alarms.push(Alarm {
                key,
                title: if data.title.is_empty() {
                    tr!("notify-no-subject")
                } else {
                    data.title.clone()
                },
                lines: lines(occurrence, to),
                join_url: join_url(data),
                at,
                task: None,
                note: None,
            });
        }
    }
    alarms.sort_by_key(|a| a.at);
    (alarms, next)
}

/// Where the event's call is: the calendar's own meeting, else the first
/// call link written in its place or notes (a Zoom, WhatsApp or Telegram
/// link pasted in), so Join works for those too.
fn join_url(data: &katna_store::calendar::EventData) -> String {
    if !data.join_url.is_empty() {
        return data.join_url.clone();
    }
    katna_core::meeting::find(
        [],
        [data.location.as_str(), data.description.as_str()],
        None,
    )
    .into_iter()
    .next()
    .map(|(_, link)| link)
    .unwrap_or_default()
}

/// When and where the event is: how soon it starts, as notifications
/// say it (the daemon formats no clock times), and its place.
fn lines(occurrence: &Occurrence, now: i64) -> Vec<String> {
    let data = &occurrence.event.data;
    let minutes = (occurrence.start - now + 30).div_euclid(60);
    let when = if data.all_day {
        tr!("notify-event-all-day")
    } else if minutes <= 0 {
        tr!("notify-event-now")
    } else if minutes < 60 {
        tr!("notify-event-in-minutes", count = minutes)
    } else if minutes < 24 * 60 {
        tr!("notify-event-in-hours", count = minutes / 60)
    } else {
        tr!("notify-event-in-days", count = minutes / (24 * 60))
    };
    let mut lines = vec![when];
    if !data.location.trim().is_empty() {
        lines.push(data.location.trim().to_owned());
    }
    lines
}

/// The reminders of open tasks due in `(from, to]`, soonest first, and
/// the time of the next one after `to`.
fn tasks_due(tasks: &[Task], from: i64, to: i64) -> (Vec<Alarm>, Option<i64>) {
    let mut alarms = Vec::new();
    let mut next: Option<i64> = None;
    for task in tasks.iter().filter(|t| t.done_at.is_none()) {
        let Some(at) = task.remind_at else { continue };
        if at > to {
            next = Some(next.map_or(at, |n| n.min(at)));
        } else if at > from {
            alarms.push(Alarm {
                key: (task.id, at),
                title: if task.title.trim().is_empty() {
                    tr!("notify-no-subject")
                } else {
                    task.title.trim().to_owned()
                },
                // The first line of its details, if it has any.
                lines: task
                    .notes
                    .lines()
                    .map(str::trim)
                    .find(|l| !l.is_empty())
                    .map(String::from)
                    .into_iter()
                    .collect(),
                join_url: String::new(),
                at,
                task: Some(task.id),
                note: None,
            });
        }
    }
    alarms.sort_by_key(|a| a.at);
    (alarms, next)
}

/// Notes' reminders due in `(from, to]`, from
/// [`Store::notes_reminding`]'s rows.
fn notes_due(rows: Vec<(i64, String, String, i64)>) -> Vec<Alarm> {
    rows.into_iter()
        .map(|(id, title, body, at)| {
            let mut lines = body
                .lines()
                .map(|l| l.trim().trim_start_matches(['☐', '☑']).trim())
                .filter(|l| !l.is_empty());
            let title = match title.trim() {
                "" => lines
                    .next()
                    .map_or_else(|| tr!("notify-no-subject"), str::to_owned),
                title => title.to_owned(),
            };
            Alarm {
                key: (id, at),
                title,
                lines: lines.next().map(str::to_owned).into_iter().collect(),
                join_url: String::new(),
                at,
                task: None,
                note: Some(id),
            }
        })
        .collect()
}

/// Reads the reminders due in `(from, to]` and the next one's time:
/// events', tasks', then notes'.
fn read(store: &Store, from: i64, to: i64, tz: &TimeZone) -> (Vec<Alarm>, Option<i64>) {
    let (mut alarms, next) = read_events(store, from, to, tz);
    let tasks = store.tasks(to).unwrap_or_else(|err| {
        tracing::warn!(%err, "reminders: cannot read tasks");
        Vec::new()
    });
    let (task_alarms, task_next) = tasks_due(&tasks, from, to);
    alarms.extend(task_alarms);
    let notes = store.notes_reminding(from, to).unwrap_or_else(|err| {
        tracing::warn!(%err, "reminders: cannot read notes");
        Vec::new()
    });
    alarms.extend(notes_due(notes));
    let note_next = store.next_note_reminder(to).ok().flatten();
    (
        alarms,
        next.into_iter().chain(task_next).chain(note_next).min(),
    )
}

fn read_events(store: &Store, from: i64, to: i64, tz: &TimeZone) -> (Vec<Alarm>, Option<i64>) {
    let calendars = store.calendars().unwrap_or_default();
    let rows = match store.event_rows_in_range(from, to + AHEAD) {
        Ok(rows) => rows,
        Err(err) => {
            tracing::warn!(%err, "reminders: cannot read events");
            return (Vec::new(), None);
        }
    };
    let occurrences = katna_dav::occurrences(rows, from, to + AHEAD, tz);
    due(&occurrences, &calendars, from, to)
}

/// Up to when reminders were looked at, if they were.
fn looked(store: &Store) -> Option<i64> {
    let row = store.meta(META_KIND, 0, META_KEY).ok().flatten()?;
    row.value_json.trim().parse().ok()
}

/// Shows reminders until the daemon goes.
pub(crate) async fn run(daemon: Weak<Daemon>) {
    let tz = TimeZone::system();
    let mut from = {
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        let now = unix_now();
        looked(&daemon.store()).map_or(now, |at| at.clamp(now - MISSED, now))
    };
    loop {
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.closing() {
            return;
        }
        let now = unix_now();
        let (alarms, next) = read(&daemon.store(), from, now, &tz);
        let notices = daemon.new_mail_notices();
        let snoozed = notices
            .as_ref()
            .map(|n| n.snoozed_due(now))
            .unwrap_or_default();
        for alarm in alarms.into_iter().chain(snoozed) {
            match (alarm.task, alarm.note) {
                (Some(task), _) => tracing::info!(task, "task reminder"),
                (_, Some(note)) => tracing::info!(note, "note reminder"),
                _ => tracing::info!(event = alarm.key.0, "event reminder"),
            }
            if let Some(notices) = &notices {
                notices.event_reminder(alarm).await;
            }
        }
        if now > from {
            let mut store = daemon.store();
            if let Err(err) = store.set_meta(META_KIND, 0, META_KEY, &now.to_string(), None) {
                tracing::debug!(%err, "reminders: cannot note the time");
            }
            from = now;
        }
        let snooze_next = notices.as_ref().and_then(|n| n.next_snoozed());
        drop(notices);
        drop(daemon);
        let wake = [next, snooze_next]
            .into_iter()
            .flatten()
            .min()
            .map_or(LOOK_EVERY, |at| (at - now).clamp(1, LOOK_EVERY));
        async_io::Timer::after(Duration::from_secs(wake as u64)).await;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use katna_store::calendar::{CalendarAccess, CalendarSource, EventData, StoredEvent};

    fn occurrence(id: i64, start: i64, reminders: Vec<i64>) -> Occurrence {
        Occurrence {
            event: Arc::new(StoredEvent {
                id,
                calendar_id: 1,
                data: EventData {
                    title: format!("Event {id}"),
                    start,
                    end: start + 3600,
                    reminders,
                    location: "Room 4B".into(),
                    ..EventData::default()
                },
            }),
            start,
            end: start + 3600,
            series_start: None,
        }
    }

    #[test]
    fn reminders_fall_due_once_between_looks() {
        let start = 1_800_000_000;
        let events = [
            occurrence(1, start, vec![10]),
            occurrence(2, start + 3600, vec![10, 60]),
        ];
        // 10 minutes before the first: its reminder, and the second's
        // hour-before one is next.
        let (alarms, next) = due(&events, &[], start - 700, start - 600);
        assert_eq!(alarms.len(), 1);
        assert_eq!(alarms[0].key, (1, start));
        assert_eq!(
            alarms[0].lines,
            ["In 10 minutes", "Room 4B"].map(String::from)
        );
        assert_eq!(next, Some(start));
        // Not again in the next look.
        let (alarms, _) = due(&events, &[], start - 600, start - 500);
        assert!(alarms.is_empty());
        // Two reminders of one event due in one look show once.
        let (alarms, next) = due(&events, &[], start - 1, start + 3000);
        assert_eq!(alarms.len(), 1);
        assert_eq!(alarms[0].key, (2, start + 3600));
        assert_eq!(next, None);
    }

    #[test]
    fn open_tasks_remind_once() {
        let at = 1_800_000_000;
        let task = |id: i64, remind_at: Option<i64>, done_at: Option<i64>| Task {
            id,
            list: 1,
            parent: None,
            title: format!("Task {id}"),
            notes: "\n  Call before noon\nsecond line".into(),
            due: String::new(),
            due_time: None,
            remind_at,
            repeat: String::new(),
            starred: false,
            done_at,
            position: String::new(),
            mail: String::new(),
            labels: Vec::new(),
        };
        let tasks = [
            task(1, Some(at), None),
            task(2, Some(at + 600), None),
            task(3, Some(at), Some(at - 60)),
            task(4, None, None),
        ];
        let (alarms, next) = tasks_due(&tasks, at - 60, at);
        assert_eq!(alarms.len(), 1);
        assert_eq!(alarms[0].task, Some(1));
        assert_eq!(alarms[0].title, "Task 1");
        assert_eq!(alarms[0].lines, ["Call before noon"]);
        assert_eq!(next, Some(at + 600));
        // Not again in the next look.
        let (alarms, _) = tasks_due(&tasks, at, at + 60);
        assert!(alarms.is_empty());
    }

    #[test]
    fn declined_cancelled_and_hidden_events_have_no_reminders() {
        let start = 1_800_000_000;
        let mut declined = occurrence(1, start, vec![10]);
        Arc::make_mut(&mut declined.event).data.self_status = "declined".into();
        let mut cancelled = occurrence(2, start, vec![10]);
        Arc::make_mut(&mut cancelled.event).data.status = EventStatus::Cancelled;
        let mut hidden = occurrence(3, start, vec![10]);
        Arc::make_mut(&mut hidden.event).calendar_id = 9;
        let calendars = [Calendar {
            id: 9,
            account: None,
            source: CalendarSource::Local,
            remote_id: "local".into(),
            name: "Hidden".into(),
            color: String::new(),
            access: CalendarAccess::Owner,
            is_primary: false,
            hidden: true,
            time_zone: String::new(),
            sync_token: None,
            position: 0,
        }];
        let (alarms, next) = due(
            &[declined, cancelled, hidden],
            &calendars,
            start - 700,
            start,
        );
        assert!(alarms.is_empty());
        assert_eq!(next, None);
    }

    #[test]
    fn join_takes_a_call_link_from_the_notes() {
        let mut data = EventData {
            description: "Dial in: https://us02web.zoom.us/j/81234567890?pwd=x thanks".into(),
            ..EventData::default()
        };
        assert_eq!(
            join_url(&data),
            "https://us02web.zoom.us/j/81234567890?pwd=x"
        );
        data.join_url = "https://meet.google.com/abc-defg-hij".into();
        assert_eq!(join_url(&data), "https://meet.google.com/abc-defg-hij");
        assert_eq!(join_url(&EventData::default()), "");
    }
}
