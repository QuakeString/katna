// SPDX-License-Identifier: GPL-3.0-or-later

//! Changing events from Katna (`Pim1.EditEvent`, `docs/ARCHITECTURE.md`
//! §18). [`apply`] writes an [`EventChange`] to the store at once, marking
//! the rows the calendar's service doesn't have yet ([`Pending`]), and
//! says what to send it: [`Step`]s, which [`push`] then sends through the
//! service's own API (Google Calendar, Microsoft Graph, CalDAV). Calendars
//! on this computer have no service: their changes are only stored.
//!
//! How a change lands:
//! - A new event gets a UID of its own (`…@katna`) and, where the service
//!   lets Katna choose it, its remote ID (Google's event ID, the CalDAV
//!   resource's name); Graph gives its ID when it takes the event.
//! - One occurrence of a series becomes a changed occurrence (a row with
//!   `recurrence_id`); deleting one skips it (`EXDATE`, or a cancelled
//!   changed occurrence).
//! - "This and following" ends the series the day before the occurrence
//!   (`UNTIL`) and starts a new series there; "all" moves the series by as
//!   much as the occurrence moved.

use jiff::Timestamp;
use jiff::tz::TimeZone;
use katna_dav::{ical, recurrence::Rule};
use katna_store::{
    Store,
    calendar::{
        Calendar, CalendarSource, EditScope, EventChange, EventData, EventEdit, EventStatus,
        Pending, StoredEvent,
    },
};

use super::{CalendarError, caldav::CalDav, google::GoogleCalendar, graph::GraphCalendar};

const DAY: i64 = 86_400;

/// Why a change was not made.
#[derive(Debug, thiserror::Error)]
pub enum EditError {
    /// The change asks for something that can't be: no such event, a
    /// calendar that can't be changed…
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    Store(#[from] katna_store::Error),
}

/// One thing to send to a calendar's service after [`apply`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Event row `row` as it is now (a single event, a series or a changed
    /// occurrence): added when the service doesn't have it yet, else
    /// changed; with `add_call`, the service adds a video call to it.
    Write {
        calendar: i64,
        row: i64,
        add_call: bool,
    },
    /// Occurrence `occurrence` (its start in the series) of series row
    /// `series` was deleted.
    Cancel {
        calendar: i64,
        series: i64,
        occurrence: i64,
    },
    /// Occurrence `occurrence` of series row `series` is back.
    Restore {
        calendar: i64,
        series: i64,
        occurrence: i64,
    },
    /// Event row `row` (with its changed occurrences) was deleted.
    Delete { calendar: i64, row: i64 },
    /// The service's event `remote_id` (etag `etag`) moved to another
    /// calendar: it goes from this one.
    DeleteRemote {
        calendar: i64,
        remote_id: String,
        etag: Option<String>,
    },
    /// Event row `row` moved from Google calendar `calendar` to Google
    /// calendar `to` of the same account (Google's `move`).
    GoogleMove { calendar: i64, row: i64, to: i64 },
    /// The user answered the invitation to event row `row` (a single
    /// event, a series or one occurrence) with `status`: `accepted`,
    /// `tentative` or `declined`.
    Respond {
        calendar: i64,
        row: i64,
        status: String,
    },
}

impl Step {
    /// The calendar whose service takes this step.
    pub fn calendar(&self) -> i64 {
        match self {
            Self::Write { calendar, .. }
            | Self::Cancel { calendar, .. }
            | Self::Restore { calendar, .. }
            | Self::Delete { calendar, .. }
            | Self::DeleteRemote { calendar, .. }
            | Self::GoogleMove { calendar, .. }
            | Self::Respond { calendar, .. } => *calendar,
        }
    }
}

/// What [`apply`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Applied {
    /// The event row added or changed (for one occurrence of a series,
    /// its changed occurrence; for "this and following", the new series),
    /// or 0.
    pub id: i64,
    /// What to send, in order. Steps for calendars on this computer are
    /// left out.
    pub steps: Vec<Step>,
    /// The rows marked [`Pending`]: once the steps went through
    /// ([`Store::events_pushed`]) or failed
    /// ([`Store::forget_pending_events`]).
    pub rows: Vec<i64>,
}

impl Applied {
    fn remote(cal: &Calendar) -> bool {
        cal.source != CalendarSource::Local
    }

    /// Sends `step` when `cal` has a service.
    fn step(&mut self, cal: &Calendar, step: Step) {
        if Self::remote(cal) {
            self.steps.push(step);
        }
    }

    /// Counts row `row` of `cal` as waiting for its service.
    fn pending(&mut self, cal: &Calendar, row: i64) {
        if Self::remote(cal) && !self.rows.contains(&row) {
            self.rows.push(row);
        }
    }
}

/// How a row of `cal` changed here is marked.
fn pending(cal: &Calendar) -> Pending {
    if cal.source == CalendarSource::Local {
        Pending::None
    } else {
        Pending::Write
    }
}

/// 32 random hex digits: a new event's UID before `@katna`, and its ID
/// where the service lets Katna choose it (Google takes `[a-v0-9]`).
fn new_key() -> String {
    use ring::rand::SecureRandom;
    let mut bytes = [0u8; 16];
    if ring::rand::SystemRandom::new().fill(&mut bytes).is_err() {
        // No randomness from the system: the time is unique enough here.
        let now = Timestamp::now().as_nanosecond();
        bytes = now.to_le_bytes();
    }
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A new random ID for a request, such as Google's request for a call.
pub(crate) fn request_id() -> String {
    new_key()
}

/// A new event's remote ID in `cal`: Google's event ID, the CalDAV
/// resource, or a stand-in until Graph gives one.
fn new_remote_id(cal: &Calendar, key: &str) -> String {
    match cal.source {
        CalendarSource::Google => key.to_owned(),
        CalendarSource::CalDav => {
            let dir = cal.remote_id.trim_end_matches('/');
            format!("{dir}/{key}.ics")
        }
        CalendarSource::Microsoft => format!("{NEW_PREFIX}{key}"),
        CalendarSource::Local => format!("local:{key}"),
    }
}

/// The remote ID of an event Graph doesn't have yet.
pub(crate) const NEW_PREFIX: &str = "new:";

fn utc_stamp(seconds: i64, all_day: bool) -> String {
    let at = Timestamp::from_second(seconds)
        .unwrap_or(Timestamp::UNIX_EPOCH)
        .to_zoned(TimeZone::UTC);
    if all_day {
        format!("{:04}{:02}{:02}", at.year(), at.month(), at.day())
    } else {
        format!(
            "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
            at.year(),
            at.month(),
            at.day(),
            at.hour(),
            at.minute(),
            at.second()
        )
    }
}

/// Google's ID of the occurrence of series `series` that starts (or
/// started, before it changed) at `original`.
pub(crate) fn google_instance_id(series: &str, original: i64, all_day: bool) -> String {
    format!("{series}_{}", utc_stamp(original, all_day))
}

/// `rule` ending the day before the occurrence at `original`: `UNTIL` the
/// last second of that day in the series' zone (its last day, for whole
/// days), without `COUNT`.
fn until_before(rule: &str, original: i64, all_day: bool, zone: &str) -> String {
    let mut parts: Vec<String> = rule
        .split(';')
        .filter(|p| !p.is_empty())
        .filter(|p| {
            let key = p.split('=').next().unwrap_or_default().trim();
            !key.eq_ignore_ascii_case("UNTIL") && !key.eq_ignore_ascii_case("COUNT")
        })
        .map(str::to_owned)
        .collect();
    let until = if all_day {
        utc_stamp(original - DAY, true)
    } else {
        let tz = TimeZone::get(zone).unwrap_or(TimeZone::UTC);
        let day = Timestamp::from_second(original)
            .unwrap_or(Timestamp::UNIX_EPOCH)
            .to_zoned(tz.clone())
            .date();
        let midnight = day
            .to_zoned(tz)
            .map_or(original, |z| z.timestamp().as_second());
        utc_stamp(midnight - 1, false)
    };
    parts.push(format!("UNTIL={until}"));
    parts.join(";")
}

/// Tidies what the editor sent and fills what follows from it.
fn finish(data: &mut EventData, now: i64) {
    let rule = data.rrule.trim();
    data.rrule = rule.strip_prefix("RRULE:").unwrap_or(rule).to_owned();
    if data.recurrence_id.is_some() {
        data.rrule.clear();
        data.exdates.clear();
        data.rdates.clear();
    }
    data.reminders.retain(|m| *m >= 0);
    data.reminders.sort_unstable();
    data.reminders.dedup();
    data.exdates.sort_unstable();
    data.exdates.dedup();
    data.range_end = ical::range_end(data);
    data.updated_at = now;
}

fn check_edit(edit: &EventEdit) -> Result<(), EditError> {
    if edit.end < edit.start {
        return Err(EditError::Invalid("the event ends before it starts".into()));
    }
    let rule = edit.rrule.trim();
    let rule = rule.strip_prefix("RRULE:").unwrap_or(rule);
    if !rule.is_empty() && Rule::parse(rule, &TimeZone::UTC).is_none() {
        return Err(EditError::Invalid(format!("can't repeat by {rule:?}")));
    }
    Ok(())
}

/// Calendar `id`, if its events can be changed.
fn editable(store: &Store, id: i64) -> Result<Calendar, EditError> {
    let cal = store
        .calendar(id)?
        .ok_or_else(|| EditError::Invalid(format!("no calendar {id}")))?;
    if !cal.access.can_edit() {
        let name = if cal.name.is_empty() {
            id.to_string()
        } else {
            format!("{id} ({})", cal.name)
        };
        return Err(EditError::Invalid(format!(
            "calendar {name} is read-only: its events can't be changed"
        )));
    }
    Ok(cal)
}

/// The event a change is for.
struct Target {
    row: StoredEvent,
    cal: Calendar,
    /// Its series, when it repeats (or is a changed occurrence).
    series: Option<StoredEvent>,
    /// Which occurrence: its start in the series.
    original: Option<i64>,
}

fn target(store: &Store, id: i64, occurrence: Option<i64>) -> Result<Target, EditError> {
    let row = store
        .event(id)?
        .ok_or_else(|| EditError::Invalid(format!("no event {id}")))?;
    if store.event_pending(id)? == Some(Pending::Delete) {
        return Err(EditError::Invalid(format!("event {id} was deleted")));
    }
    let cal = editable(store, row.calendar_id)?;
    let (series, original) = if let Some(original) = row.data.recurrence_id {
        let series = store
            .event_series(cal.id, &row.data.uid)?
            .filter(|s| !s.data.rrule.is_empty());
        (series, Some(original))
    } else if !row.data.rrule.is_empty() {
        (Some(row.clone()), occurrence)
    } else {
        (None, None)
    };
    Ok(Target {
        row,
        cal,
        series,
        original,
    })
}

/// Writes `change` to `store` at once and says what to send the
/// calendars' services; `now` is the time (Unix seconds).
pub fn apply(store: &mut Store, change: &EventChange, now: i64) -> Result<Applied, EditError> {
    let mut applied = Applied::default();
    match change {
        EventChange::Add { calendar, edit } => {
            check_edit(edit)?;
            let cal = editable(store, *calendar)?;
            applied.id = add_new(store, &mut applied, &cal, edit, now)?;
        }
        EventChange::Change {
            event,
            scope,
            occurrence,
            edit,
            calendar,
        } => {
            check_edit(edit)?;
            let t = target(store, *event, *occurrence)?;
            let dest = match calendar {
                Some(to) if *to != t.cal.id => Some(editable(store, *to)?),
                _ => None,
            };
            applied.id = match (&t.series, *scope, t.original) {
                (None, _, _) => {
                    change_whole(store, &mut applied, &t.row, &t.cal, dest, edit, None, now)?
                }
                (Some(series), EditScope::This, Some(original)) => {
                    if dest.is_some() {
                        return Err(EditError::Invalid(
                            "only a whole series moves to another calendar".into(),
                        ));
                    }
                    change_one(
                        store,
                        &mut applied,
                        &t.row,
                        series,
                        &t.cal,
                        original,
                        edit,
                        now,
                    )?
                }
                (Some(series), EditScope::Following, Some(original))
                    if original > series.data.start =>
                {
                    end_series(store, &mut applied, series, &t.cal, original, now)?;
                    let cal = dest.as_ref().unwrap_or(&t.cal);
                    add_new(store, &mut applied, cal, edit, now)?
                }
                (Some(series), _, _) => change_whole(
                    store,
                    &mut applied,
                    series,
                    &t.cal,
                    dest,
                    edit,
                    *occurrence,
                    now,
                )?,
            };
        }
        EventChange::Delete {
            event,
            scope,
            occurrence,
        } => {
            let t = target(store, *event, *occurrence)?;
            match (&t.series, *scope, t.original) {
                (Some(series), EditScope::This, Some(original)) => {
                    cancel_one(store, &mut applied, &t.row, series, &t.cal, original, now)?;
                }
                (Some(series), EditScope::Following, Some(original))
                    if original > series.data.start =>
                {
                    end_series(store, &mut applied, series, &t.cal, original, now)?;
                }
                (Some(series), _, _) => delete_whole(store, &mut applied, series, &t.cal)?,
                (None, _, _) => delete_whole(store, &mut applied, &t.row, &t.cal)?,
            }
        }
        EventChange::Restore { event, occurrence } => {
            let t = target(store, *event, Some(*occurrence))?;
            let series = t
                .series
                .ok_or_else(|| EditError::Invalid(format!("event {event} doesn't repeat")))?;
            applied.id = series.id;
            restore_one(store, &mut applied, &series, &t.cal, *occurrence, now)?;
        }
        EventChange::Respond {
            event,
            scope,
            occurrence,
            status,
        } => {
            if !matches!(status.as_str(), "accepted" | "tentative" | "declined") {
                return Err(EditError::Invalid(format!(
                    "can't answer {status:?} (accepted, tentative or declined)"
                )));
            }
            let t = target(store, *event, *occurrence)?;
            let answer = |data: &mut EventData| {
                data.self_status = status.clone();
                for attendee in data.attendees.iter_mut().filter(|a| a.is_self) {
                    attendee.status = status.clone();
                }
            };
            let id = match (&t.series, *scope, t.original) {
                (Some(series), EditScope::This, Some(original)) => {
                    occurrence_row(store, &t.row, series, &t.cal, original, now, answer)?
                }
                (series, _, _) => {
                    let row = series.as_ref().unwrap_or(&t.row);
                    let mut data = row.data.clone();
                    answer(&mut data);
                    data.updated_at = now;
                    store.update_event(row.id, &data, pending(&t.cal))?;
                    row.id
                }
            };
            applied.id = id;
            applied.pending(&t.cal, id);
            applied.step(
                &t.cal,
                Step::Respond {
                    calendar: t.cal.id,
                    row: id,
                    status: status.clone(),
                },
            );
        }
    }
    Ok(applied)
}

/// Makes the user the organizer of an event they invite others to, when
/// no one is: a CalDAV server sends the invitations for the organizer.
fn organize(store: &Store, cal: &Calendar, data: &mut EventData) -> Result<(), EditError> {
    if data.attendees.is_empty() || !data.organizer.is_empty() {
        return Ok(());
    }
    let Some(account) = cal.account else {
        return Ok(());
    };
    if let Some(account) = store.accounts()?.into_iter().find(|a| a.id == account) {
        data.organizer = account.address;
    }
    Ok(())
}

/// A new event (or series) in `cal` from `edit`. Returns its row.
fn add_new(
    store: &mut Store,
    applied: &mut Applied,
    cal: &Calendar,
    edit: &EventEdit,
    now: i64,
) -> Result<i64, EditError> {
    let key = new_key();
    let mut data = EventData {
        uid: format!("{key}@katna"),
        remote_id: new_remote_id(cal, &key),
        ..EventData::default()
    };
    edit.apply(&mut data);
    organize(store, cal, &mut data)?;
    finish(&mut data, now);
    let id = store.add_event(cal.id, &data, pending(cal))?;
    applied.pending(cal, id);
    applied.step(
        cal,
        Step::Write {
            calendar: cal.id,
            row: id,
            add_call: edit.add_call,
        },
    );
    Ok(id)
}

/// Changes a single event, or a whole series with `occurrence` the start
/// of the occurrence the edit's times are for (the series moves by as much
/// as it did), and moves it to `dest`. Returns its row.
#[allow(clippy::too_many_arguments)]
fn change_whole(
    store: &mut Store,
    applied: &mut Applied,
    row: &StoredEvent,
    cal: &Calendar,
    dest: Option<Calendar>,
    edit: &EventEdit,
    occurrence: Option<i64>,
    now: i64,
) -> Result<i64, EditError> {
    let mut data = row.data.clone();
    edit.apply(&mut data);
    organize(store, dest.as_ref().unwrap_or(cal), &mut data)?;
    let mut exceptions_go = data.rrule.is_empty();
    if let Some(occurrence) = occurrence.filter(|_| !row.data.rrule.is_empty()) {
        let delta = edit.start - occurrence;
        data.start = row.data.start + delta;
        data.end = data.start + (edit.end - edit.start);
        if delta != 0 {
            for time in data.exdates.iter_mut().chain(data.rdates.iter_mut()) {
                *time += delta;
            }
            // Changed occurrences no longer match any occurrence.
            exceptions_go = true;
        }
    }
    finish(&mut data, now);
    let add_call = edit.add_call && data.join_url.is_empty();
    let exceptions = if row.data.recurrence_id.is_none() {
        store.event_exceptions(cal.id, &row.data.uid)?
    } else {
        Vec::new()
    };
    if let Some(dest) = dest {
        // The changed occurrences stay behind on the service.
        for exception in &exceptions {
            store.delete_event(exception.id)?;
        }
        let same_google = cal.source == CalendarSource::Google
            && dest.source == CalendarSource::Google
            && cal.account == dest.account;
        let (old_remote, old_etag) = (row.data.remote_id.clone(), row.data.etag.clone());
        if !same_google {
            data.remote_id = new_remote_id(&dest, &new_key());
            data.etag = None;
        }
        store.update_event(row.id, &data, pending(&dest))?;
        store.set_event_calendar(row.id, dest.id)?;
        applied.pending(&dest, row.id);
        if same_google {
            applied.step(
                cal,
                Step::GoogleMove {
                    calendar: cal.id,
                    row: row.id,
                    to: dest.id,
                },
            );
        } else {
            applied.step(
                cal,
                Step::DeleteRemote {
                    calendar: cal.id,
                    remote_id: old_remote,
                    etag: old_etag,
                },
            );
            applied.step(
                &dest,
                Step::Write {
                    calendar: dest.id,
                    row: row.id,
                    add_call,
                },
            );
        }
        return Ok(row.id);
    }
    if exceptions_go {
        for exception in &exceptions {
            store.delete_event(exception.id)?;
        }
    }
    store.update_event(row.id, &data, pending(cal))?;
    applied.pending(cal, row.id);
    applied.step(
        cal,
        Step::Write {
            calendar: cal.id,
            row: row.id,
            add_call,
        },
    );
    Ok(row.id)
}

/// Changes the occurrence of `series` that starts at `original` in the
/// series (`row` is the series or that changed occurrence). Returns the
/// changed occurrence's row.
#[allow(clippy::too_many_arguments)]
fn change_one(
    store: &mut Store,
    applied: &mut Applied,
    row: &StoredEvent,
    series: &StoredEvent,
    cal: &Calendar,
    original: i64,
    edit: &EventEdit,
    now: i64,
) -> Result<i64, EditError> {
    let id = occurrence_row(store, row, series, cal, original, now, |data| {
        // The series' rule, which the editor sends along, is not the
        // occurrence's.
        edit.apply(data);
        data.status = EventStatus::Confirmed;
    })?;
    applied.pending(cal, id);
    applied.step(
        cal,
        Step::Write {
            calendar: cal.id,
            row: id,
            add_call: edit.add_call && store.event(id)?.is_some_and(|e| e.data.join_url.is_empty()),
        },
    );
    Ok(id)
}

/// Changes the occurrence of `series` that starts at `original` in the
/// series (`row` is the series or that changed occurrence) with `change`:
/// its changed occurrence, made from the series when there is none yet.
/// Returns that row.
fn occurrence_row(
    store: &mut Store,
    row: &StoredEvent,
    series: &StoredEvent,
    cal: &Calendar,
    original: i64,
    now: i64,
    change: impl Fn(&mut EventData),
) -> Result<i64, EditError> {
    let existing = if row.data.recurrence_id.is_some() {
        Some(row.clone())
    } else {
        store
            .event_exceptions(cal.id, &series.data.uid)?
            .into_iter()
            .find(|e| e.data.recurrence_id == Some(original))
    };
    Ok(match existing {
        Some(exception) => {
            let mut data = exception.data.clone();
            change(&mut data);
            finish(&mut data, now);
            store.update_event(exception.id, &data, pending(cal))?;
            exception.id
        }
        None => {
            let mut data = series.data.clone();
            data.recurrence_id = Some(original);
            let length = data.end - data.start;
            data.start = original;
            data.end = original + length;
            data.remote_id = match cal.source {
                CalendarSource::Google => {
                    google_instance_id(&series.data.remote_id, original, series.data.all_day)
                }
                // In the series' resource.
                CalendarSource::CalDav => series.data.remote_id.clone(),
                _ => new_remote_id(cal, &new_key()),
            };
            if cal.source != CalendarSource::CalDav {
                data.etag = None;
            }
            change(&mut data);
            finish(&mut data, now);
            store.add_event(cal.id, &data, pending(cal))?
        }
    })
}

/// Ends `series` the day before its occurrence at `original`, dropping
/// its changed occurrences from then on.
fn end_series(
    store: &mut Store,
    applied: &mut Applied,
    series: &StoredEvent,
    cal: &Calendar,
    original: i64,
    now: i64,
) -> Result<(), EditError> {
    let mut data = series.data.clone();
    data.rrule = until_before(&data.rrule, original, data.all_day, &data.time_zone);
    data.exdates.retain(|t| *t < original);
    data.rdates.retain(|t| *t < original);
    finish(&mut data, now);
    for exception in store.event_exceptions(cal.id, &series.data.uid)? {
        if exception.data.recurrence_id.is_some_and(|r| r >= original) {
            store.delete_event(exception.id)?;
        }
    }
    store.update_event(series.id, &data, pending(cal))?;
    applied.pending(cal, series.id);
    applied.step(
        cal,
        Step::Write {
            calendar: cal.id,
            row: series.id,
            add_call: false,
        },
    );
    Ok(())
}

/// Deletes event `row` (a single event or a series) with its changed
/// occurrences.
fn delete_whole(
    store: &mut Store,
    applied: &mut Applied,
    row: &StoredEvent,
    cal: &Calendar,
) -> Result<(), EditError> {
    let exceptions = if row.data.recurrence_id.is_none() {
        store.event_exceptions(cal.id, &row.data.uid)?
    } else {
        Vec::new()
    };
    let rows = std::iter::once(row.id).chain(exceptions.iter().map(|e| e.id));
    for id in rows {
        if Applied::remote(cal) {
            store.mark_event_deleted(id)?;
            applied.pending(cal, id);
        } else {
            store.delete_event(id)?;
        }
    }
    applied.step(
        cal,
        Step::Delete {
            calendar: cal.id,
            row: row.id,
        },
    );
    Ok(())
}

/// Deletes the occurrence of `series` that starts at `original` in it
/// (`row` is the series or that changed occurrence).
fn cancel_one(
    store: &mut Store,
    applied: &mut Applied,
    row: &StoredEvent,
    series: &StoredEvent,
    cal: &Calendar,
    original: i64,
    now: i64,
) -> Result<(), EditError> {
    let exception = if row.data.recurrence_id.is_some() {
        Some(row.clone())
    } else {
        store
            .event_exceptions(cal.id, &series.data.uid)?
            .into_iter()
            .find(|e| e.data.recurrence_id == Some(original))
    };
    let id = match exception {
        // A changed occurrence stays, cancelled, leaving a gap.
        Some(exception) => {
            let mut data = exception.data.clone();
            data.status = EventStatus::Cancelled;
            data.updated_at = now;
            store.update_event(exception.id, &data, pending(cal))?;
            exception.id
        }
        None => {
            let mut data = series.data.clone();
            data.exdates.push(original);
            finish(&mut data, now);
            store.update_event(series.id, &data, pending(cal))?;
            series.id
        }
    };
    applied.pending(cal, id);
    applied.step(
        cal,
        Step::Cancel {
            calendar: cal.id,
            series: series.id,
            occurrence: original,
        },
    );
    Ok(())
}

/// Brings back the occurrence of `series` at `original`, deleted alone.
fn restore_one(
    store: &mut Store,
    applied: &mut Applied,
    series: &StoredEvent,
    cal: &Calendar,
    original: i64,
    now: i64,
) -> Result<(), EditError> {
    let mut changed = false;
    if series.data.exdates.contains(&original) {
        let mut data = series.data.clone();
        data.exdates.retain(|t| *t != original);
        finish(&mut data, now);
        store.update_event(series.id, &data, pending(cal))?;
        applied.pending(cal, series.id);
        changed = true;
    }
    for exception in store.event_exceptions(cal.id, &series.data.uid)? {
        if exception.data.recurrence_id != Some(original)
            || exception.data.status != EventStatus::Cancelled
        {
            continue;
        }
        changed = true;
        if exception.data.title.is_empty() {
            // Only the mark of a deleted occurrence, as a service keeps it.
            if Applied::remote(cal) {
                store.mark_event_deleted(exception.id)?;
                applied.pending(cal, exception.id);
            } else {
                store.delete_event(exception.id)?;
            }
        } else {
            let mut data = exception.data.clone();
            data.status = EventStatus::Confirmed;
            data.updated_at = now;
            store.update_event(exception.id, &data, pending(cal))?;
            applied.pending(cal, exception.id);
        }
    }
    if changed {
        applied.step(
            cal,
            Step::Restore {
                calendar: cal.id,
                series: series.id,
                occurrence: original,
            },
        );
    }
    Ok(())
}

/// A calendar's service, to send [`Step`]s to.
pub enum Remote<'a> {
    Google(&'a GoogleCalendar),
    Microsoft(&'a GraphCalendar),
    CalDav(&'a CalDav),
}

/// Sends `step` to `remote`, the service of calendar `calendar`, and
/// keeps what it says (remote IDs, etags) in `store`.
pub async fn push(
    remote: &Remote<'_>,
    store: &mut Store,
    calendar: &Calendar,
    step: &Step,
) -> Result<(), CalendarError> {
    match remote {
        Remote::Google(google) => google.push(store, calendar, step).await,
        Remote::Microsoft(graph) => graph.push(store, calendar, step).await,
        Remote::CalDav(dav) => dav.push(store, calendar, step).await,
    }
}

#[cfg(test)]
mod tests;
