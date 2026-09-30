// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars and their events (`calendar` and `event` in `pim.db`,
//! `docs/ARCHITECTURE.md` §18). The daemon writes what it syncs; apps read.
//! Repeating events are stored once, with their rule; `katna_dav` expands
//! them into occurrences when read.

use katna_core::AccountId;
use rusqlite::{OptionalExtension, Row, params};

use crate::Store;
use crate::error::Result;

/// Where a calendar's events come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CalendarSource {
    Google,
    Microsoft,
    CalDav,
    /// Zoho Calendar's REST API.
    Zoho,
    Local,
}

impl CalendarSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Microsoft => "microsoft",
            Self::CalDav => "caldav",
            Self::Zoho => "zoho",
            Self::Local => "local",
        }
    }

    fn parse(text: &str) -> Self {
        match text {
            "google" => Self::Google,
            "microsoft" => Self::Microsoft,
            "caldav" => Self::CalDav,
            "zoho" => Self::Zoho,
            _ => Self::Local,
        }
    }
}

/// What the user may do with a calendar's events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CalendarAccess {
    Owner,
    Writer,
    Reader,
    /// Only when its owner is busy.
    FreeBusy,
}

impl CalendarAccess {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Writer => "writer",
            Self::Reader => "reader",
            Self::FreeBusy => "freebusy",
        }
    }

    fn parse(text: &str) -> Self {
        match text {
            "owner" => Self::Owner,
            "writer" => Self::Writer,
            "freebusy" => Self::FreeBusy,
            _ => Self::Reader,
        }
    }

    /// Whether events can be added and changed.
    pub fn can_edit(self) -> bool {
        matches!(self, Self::Owner | Self::Writer)
    }
}

/// A calendar as the service describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewCalendar {
    pub remote_id: String,
    pub name: String,
    /// `#rrggbb`, or empty.
    pub color: String,
    pub access: CalendarAccess,
    pub is_primary: bool,
    /// IANA name, or empty.
    pub time_zone: String,
}

/// A stored calendar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calendar {
    pub id: i64,
    /// `None` for a calendar on this computer.
    pub account: Option<AccountId>,
    pub source: CalendarSource,
    pub remote_id: String,
    pub name: String,
    pub color: String,
    pub access: CalendarAccess,
    pub is_primary: bool,
    pub hidden: bool,
    pub time_zone: String,
    pub sync_token: Option<String>,
    pub position: i64,
}

/// An event's state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EventStatus {
    #[default]
    Confirmed,
    Tentative,
    Cancelled,
}

impl EventStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Tentative => "tentative",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(text: &str) -> Self {
        match text {
            "tentative" => Self::Tentative,
            "cancelled" => Self::Cancelled,
            _ => Self::Confirmed,
        }
    }
}

/// What kind of time an event is.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    #[default]
    Default,
    Focus,
    OutOfOffice,
    WorkingLocation,
    Birthday,
}

impl EventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Focus => "focus",
            Self::OutOfOffice => "out_of_office",
            Self::WorkingLocation => "working_location",
            Self::Birthday => "birthday",
        }
    }

    pub fn is_default(&self) -> bool {
        *self == Self::Default
    }

    pub fn parse(text: &str) -> Self {
        match text {
            "focus" => Self::Focus,
            "out_of_office" => Self::OutOfOffice,
            "working_location" => Self::WorkingLocation,
            "birthday" => Self::Birthday,
            _ => Self::Default,
        }
    }
}

/// Someone invited to an event.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Attendee {
    pub email: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// `accepted`, `tentative`, `declined` or `needs_action`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub optional: bool,
    /// The user.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_self: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub organizer: bool,
}

/// One event as synced: a single event, the first of a series, or a
/// changed occurrence of one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EventData {
    pub remote_id: String,
    pub uid: String,
    pub etag: Option<String>,
    /// A changed occurrence: the start it had in the series.
    pub recurrence_id: Option<i64>,
    pub status: EventStatus,
    pub title: String,
    pub location: String,
    pub description: String,
    /// Unix seconds. A whole-day event starts and ends at UTC midnights.
    pub start: i64,
    /// Unix seconds, after the event.
    pub end: i64,
    pub all_day: bool,
    /// IANA name of the start's zone, or empty.
    pub time_zone: String,
    /// The RRULE value, or empty.
    pub rrule: String,
    /// Starts the series skips.
    pub exdates: Vec<i64>,
    /// Starts the series adds.
    pub rdates: Vec<i64>,
    /// Where the series ends, `None` if never; for a single event, its end.
    pub range_end: Option<i64>,
    pub busy: bool,
    pub kind: EventKind,
    pub color: String,
    pub organizer: String,
    pub organizer_name: String,
    pub attendees: Vec<Attendee>,
    /// The user's answer: empty, `accepted`, `tentative`, `declined` or
    /// `needs_action`.
    pub self_status: String,
    pub join_url: String,
    /// Minutes before the start.
    pub reminders: Vec<i64>,
    pub web_link: String,
    pub updated_at: i64,
}

/// A stored event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredEvent {
    pub id: i64,
    pub calendar_id: i64,
    pub data: EventData,
}

/// What the user sets on an event in the editor: the fields an app sends
/// the daemon to add or change an event ([`EventChange`]).
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct EventEdit {
    pub title: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub description: String,
    /// Unix seconds; a whole-day event's are UTC midnights.
    pub start: i64,
    pub end: i64,
    #[serde(default)]
    pub all_day: bool,
    /// IANA name of the zone the times were picked in.
    #[serde(default)]
    pub time_zone: String,
    /// The RRULE value, or empty.
    #[serde(default)]
    pub rrule: String,
    #[serde(default = "busy_default")]
    pub busy: bool,
    /// `#rrggbb`, or empty for the calendar's.
    #[serde(default)]
    pub color: String,
    /// Minutes before the start.
    #[serde(default)]
    pub reminders: Vec<i64>,
    #[serde(default)]
    pub attendees: Vec<Attendee>,
    /// Asks the service to add a video call (Google Meet, Teams).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub add_call: bool,
    /// Focus time, out of office or a working location; set when the
    /// event is made.
    #[serde(default, skip_serializing_if = "EventKind::is_default")]
    pub kind: EventKind,
}

fn busy_default() -> bool {
    true
}

impl EventEdit {
    /// What `event` has now, to edit.
    pub fn of(event: &EventData) -> Self {
        Self {
            title: event.title.clone(),
            location: event.location.clone(),
            description: event.description.clone(),
            start: event.start,
            end: event.end,
            all_day: event.all_day,
            time_zone: event.time_zone.clone(),
            rrule: event.rrule.clone(),
            busy: event.busy,
            color: event.color.clone(),
            reminders: event.reminders.clone(),
            attendees: event.attendees.clone(),
            add_call: false,
            kind: event.kind,
        }
    }

    /// Sets these fields on `event`.
    pub fn apply(&self, event: &mut EventData) {
        event.title = self.title.clone();
        event.location = self.location.clone();
        event.description = self.description.clone();
        event.start = self.start;
        event.end = self.end;
        event.all_day = self.all_day;
        event.time_zone = self.time_zone.clone();
        event.rrule = self.rrule.clone();
        event.busy = self.busy;
        event.color = self.color.clone();
        event.reminders = self.reminders.clone();
        event.attendees = self.attendees.clone();
        event.kind = self.kind;
    }
}

/// Which occurrences of a repeating event a change is for.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EditScope {
    /// This occurrence only, or the event when it does not repeat.
    #[default]
    This,
    /// This occurrence and those after it.
    Following,
    /// The whole series.
    All,
}

/// A change to the calendar that an app asks the daemon for
/// (`Pim1.EditEvent`, as JSON). The daemon writes it to `pim.db` at once
/// and then to the calendar's service.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum EventChange {
    /// A new event in calendar `calendar`.
    Add { calendar: i64, edit: EventEdit },
    /// Changes event `event`; for a repeating one, `occurrence` is the
    /// start the occurrence has in the series.
    Change {
        event: i64,
        #[serde(default)]
        scope: EditScope,
        #[serde(default)]
        occurrence: Option<i64>,
        edit: EventEdit,
        /// Moves the event to this calendar.
        #[serde(default)]
        calendar: Option<i64>,
    },
    /// Deletes event `event`, or occurrences of it.
    Delete {
        event: i64,
        #[serde(default)]
        scope: EditScope,
        #[serde(default)]
        occurrence: Option<i64>,
    },
    /// Brings back the occurrence starting at `occurrence` of the series
    /// `event`, deleted by itself.
    Restore { event: i64, occurrence: i64 },
    /// The user's answer to an invitation: `accepted`, `tentative` or
    /// `declined`, for one occurrence or the series.
    Respond {
        event: i64,
        #[serde(default)]
        scope: EditScope,
        #[serde(default)]
        occurrence: Option<i64>,
        status: String,
    },
}

fn join(numbers: &[i64]) -> String {
    numbers
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

fn split(text: &str) -> Vec<i64> {
    text.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

const CALENDAR_COLUMNS: &str = "id, account_id, source, remote_id, name, color, access, \
     is_primary, hidden, time_zone, sync_token, position";

fn calendar_row(row: &Row<'_>) -> rusqlite::Result<Calendar> {
    Ok(Calendar {
        id: row.get(0)?,
        account: row.get::<_, Option<i64>>(1)?.map(AccountId),
        source: CalendarSource::parse(&row.get::<_, String>(2)?),
        remote_id: row.get(3)?,
        name: row.get(4)?,
        color: row.get(5)?,
        access: CalendarAccess::parse(&row.get::<_, String>(6)?),
        is_primary: row.get(7)?,
        hidden: row.get(8)?,
        time_zone: row.get(9)?,
        sync_token: row.get(10)?,
        position: row.get(11)?,
    })
}

const EVENT_COLUMNS: &str = "id, calendar_id, remote_id, uid, etag, recurrence_id, status, \
     title, location, description, start, end, all_day, time_zone, rrule, exdates, rdates, \
     range_end, busy, kind, color, organizer, organizer_name, attendees_json, self_status, \
     join_url, reminders, web_link, updated_at";

fn event_row(row: &Row<'_>) -> rusqlite::Result<StoredEvent> {
    let attendees: String = row.get(23)?;
    Ok(StoredEvent {
        id: row.get(0)?,
        calendar_id: row.get(1)?,
        data: EventData {
            remote_id: row.get(2)?,
            uid: row.get(3)?,
            etag: row.get(4)?,
            recurrence_id: row.get(5)?,
            status: EventStatus::parse(&row.get::<_, String>(6)?),
            title: row.get(7)?,
            location: row.get(8)?,
            description: row.get(9)?,
            start: row.get(10)?,
            end: row.get(11)?,
            all_day: row.get(12)?,
            time_zone: row.get(13)?,
            rrule: row.get(14)?,
            exdates: split(&row.get::<_, String>(15)?),
            rdates: split(&row.get::<_, String>(16)?),
            range_end: row.get(17)?,
            busy: row.get(18)?,
            kind: EventKind::parse(&row.get::<_, String>(19)?),
            color: row.get(20)?,
            organizer: row.get(21)?,
            organizer_name: row.get(22)?,
            attendees: serde_json::from_str(&attendees).unwrap_or_default(),
            self_status: row.get(24)?,
            join_url: row.get(25)?,
            reminders: split(&row.get::<_, String>(26)?),
            web_link: row.get(27)?,
            updated_at: row.get(28)?,
        },
    })
}

/// A day, in seconds: whole-day events are stored at UTC midnights, so a
/// range asked in local time reaches this far either side.
const DAY: i64 = 24 * 60 * 60;

/// Where an event row stands with its calendar's service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Pending {
    /// As the service has it (or in a calendar on this computer).
    #[default]
    None,
    /// Added or changed here; the service doesn't have it yet.
    Write,
    /// Deleted here: kept, cancelled, until the service deleted it too.
    Delete,
}

impl Pending {
    fn as_int(self) -> i64 {
        match self {
            Self::None => 0,
            Self::Write => 1,
            Self::Delete => 2,
        }
    }

    fn parse(value: i64) -> Self {
        match value {
            1 => Self::Write,
            2 => Self::Delete,
            _ => Self::None,
        }
    }
}

/// Inserts `event` into calendar `calendar` under `remote_id`. Returns
/// its ID.
fn insert_event(
    conn: &rusqlite::Connection,
    calendar: i64,
    remote_id: &str,
    event: &EventData,
    pending: Pending,
) -> Result<i64> {
    let attendees = serde_json::to_string(&event.attendees).unwrap_or_else(|_| "[]".to_owned());
    conn.prepare_cached(
        "INSERT INTO event (calendar_id, remote_id, uid, etag, recurrence_id, status,
             title, location, description, start, end, all_day, time_zone, rrule,
             exdates, rdates, range_end, busy, kind, color, organizer, organizer_name,
             attendees_json, self_status, join_url, reminders, web_link, updated_at, pending)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
             ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29)",
    )?
    .execute(params![
        calendar,
        remote_id,
        event.uid,
        event.etag,
        event.recurrence_id,
        event.status.as_str(),
        event.title,
        event.location,
        event.description,
        event.start,
        event.end,
        event.all_day,
        event.time_zone,
        event.rrule,
        join(&event.exdates),
        join(&event.rdates),
        event.range_end,
        event.busy,
        event.kind.as_str(),
        event.color,
        event.organizer,
        event.organizer_name,
        attendees,
        event.self_status,
        event.join_url,
        join(&event.reminders),
        event.web_link,
        event.updated_at,
        pending.as_int(),
    ])?;
    Ok(conn.last_insert_rowid())
}

/// Replaces the rows under `remote_id` in calendar `calendar` with
/// `events`, inside transaction `tx`. Rows changed here that the service
/// doesn't have yet stay as they are.
fn write_events(
    tx: &rusqlite::Transaction<'_>,
    calendar: i64,
    remote_id: &str,
    events: &[EventData],
) -> Result<()> {
    let pending: bool = tx
        .prepare_cached(
            "SELECT EXISTS (SELECT 1 FROM event
                 WHERE calendar_id = ?1 AND remote_id = ?2 AND pending != 0)",
        )?
        .query_row(params![calendar, remote_id], |row| row.get(0))?;
    if pending {
        tracing::debug!(
            calendar,
            remote_id,
            "event changed here; the sync leaves it"
        );
        return Ok(());
    }
    tx.execute(
        "DELETE FROM event WHERE calendar_id = ?1 AND remote_id = ?2",
        params![calendar, remote_id],
    )?;
    for event in events {
        insert_event(tx, calendar, remote_id, event, Pending::None)?;
    }
    Ok(())
}

impl Store {
    /// Every calendar, by account and then as the service lists them.
    pub fn calendars(&self) -> Result<Vec<Calendar>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {CALENDAR_COLUMNS} FROM calendar
             ORDER BY account_id IS NULL, account_id, is_primary DESC, position, name, id"
        ))?;
        let rows = stmt.query_map([], calendar_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Calendar `id`.
    pub fn calendar(&self, id: i64) -> Result<Option<Calendar>> {
        Ok(self
            .pim
            .prepare_cached(&format!(
                "SELECT {CALENDAR_COLUMNS} FROM calendar WHERE id = ?1"
            ))?
            .query_row([id], calendar_row)
            .optional()?)
    }

    /// Adds or updates `account`'s calendar as the service lists it at
    /// `position`, keeping whether it is shown and its sync token. Returns
    /// its ID.
    pub fn upsert_calendar(
        &mut self,
        account: Option<AccountId>,
        source: CalendarSource,
        calendar: &NewCalendar,
        position: i64,
    ) -> Result<i64> {
        let account = account.map(|a| a.0);
        let existing: Option<i64> = self
            .pim
            .prepare_cached("SELECT id FROM calendar WHERE account_id IS ?1 AND remote_id = ?2")?
            .query_row(params![account, calendar.remote_id], |row| row.get(0))
            .optional()?;
        if let Some(id) = existing {
            self.pim.execute(
                "UPDATE calendar SET source = ?2, name = ?3, color = ?4, access = ?5,
                     is_primary = ?6, time_zone = ?7, position = ?8
                 WHERE id = ?1",
                params![
                    id,
                    source.as_str(),
                    calendar.name,
                    calendar.color,
                    calendar.access.as_str(),
                    calendar.is_primary,
                    calendar.time_zone,
                    position
                ],
            )?;
            return Ok(id);
        }
        self.pim.execute(
            "INSERT INTO calendar (account_id, source, remote_id, name, color, access,
                 is_primary, time_zone, position)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                account,
                source.as_str(),
                calendar.remote_id,
                calendar.name,
                calendar.color,
                calendar.access.as_str(),
                calendar.is_primary,
                calendar.time_zone,
                position
            ],
        )?;
        Ok(self.pim.last_insert_rowid())
    }

    /// Deletes `account`'s calendars the service no longer lists, with
    /// their events. Returns how many went.
    pub fn remove_calendars_except(
        &mut self,
        account: AccountId,
        keep: &[String],
    ) -> Result<usize> {
        let ids: Vec<(i64, String)> = self
            .pim
            .prepare_cached("SELECT id, remote_id FROM calendar WHERE account_id = ?1")?
            .query_map([account.0], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut removed = 0;
        for (id, remote) in ids {
            if !keep.contains(&remote) {
                removed += self
                    .pim
                    .execute("DELETE FROM calendar WHERE id = ?1", [id])?;
            }
        }
        Ok(removed)
    }

    /// Deletes every calendar of `account` (the account went away).
    pub fn remove_account_calendars(&mut self, account: AccountId) -> Result<usize> {
        Ok(self
            .pim
            .execute("DELETE FROM calendar WHERE account_id = ?1", [account.0])?)
    }

    /// Shows or hides calendar `id`'s events. Returns whether it exists.
    pub fn set_calendar_hidden(&mut self, id: i64, hidden: bool) -> Result<bool> {
        Ok(self.pim.execute(
            "UPDATE calendar SET hidden = ?2 WHERE id = ?1",
            params![id, hidden],
        )? > 0)
    }

    /// Keeps the token for calendar `id`'s next incremental sync; `None`
    /// makes the next sync a full one.
    pub fn set_calendar_sync_token(&mut self, id: i64, token: Option<&str>) -> Result<()> {
        self.pim.execute(
            "UPDATE calendar SET sync_token = ?2 WHERE id = ?1",
            params![id, token],
        )?;
        Ok(())
    }

    /// Replaces the events stored under `remote_id` in calendar `calendar`
    /// with `events` (a CalDAV resource holds a series and its changed
    /// occurrences; a Google or Graph item one event). Empty `events`
    /// deletes them.
    pub fn replace_events(
        &mut self,
        calendar: i64,
        remote_id: &str,
        events: &[EventData],
    ) -> Result<()> {
        let tx = self.pim.transaction()?;
        write_events(&tx, calendar, remote_id, events)?;
        tx.commit()?;
        Ok(())
    }

    /// [`Self::replace_events`] for many remote IDs at once, in one
    /// transaction: what a sync brings.
    pub fn replace_events_batch(
        &mut self,
        calendar: i64,
        items: &[(String, Vec<EventData>)],
    ) -> Result<()> {
        let tx = self.pim.transaction()?;
        for (remote_id, events) in items {
            write_events(&tx, calendar, remote_id, events)?;
        }
        tx.commit()?;
        Ok(())
    }

    /// The UID of the events stored under `remote_id` in calendar
    /// `calendar`, if any.
    pub fn event_uid(&self, calendar: i64, remote_id: &str) -> Result<Option<String>> {
        Ok(self
            .pim
            .prepare_cached(
                "SELECT uid FROM event WHERE calendar_id = ?1 AND remote_id = ?2 LIMIT 1",
            )?
            .query_row(params![calendar, remote_id], |row| row.get(0))
            .optional()?)
    }

    /// Deletes the series `uid` of calendar `calendar` with its changed
    /// occurrences (the series was deleted). Returns how many rows went.
    /// Rows changed here that the service doesn't have yet stay.
    pub fn remove_series(&mut self, calendar: i64, uid: &str) -> Result<usize> {
        Ok(self.pim.execute(
            "DELETE FROM event WHERE calendar_id = ?1 AND uid = ?2 AND pending = 0",
            params![calendar, uid],
        )?)
    }

    /// Deletes every event of calendar `calendar`, before a full sync,
    /// but those changed here that the service doesn't have yet.
    pub fn clear_calendar_events(&mut self, calendar: i64) -> Result<usize> {
        Ok(self.pim.execute(
            "DELETE FROM event WHERE calendar_id = ?1 AND pending = 0",
            [calendar],
        )?)
    }

    /// The remote IDs and etags of calendar `calendar`'s events, for a
    /// CalDAV sync to compare with the server's.
    pub fn event_etags(&self, calendar: i64) -> Result<Vec<(String, Option<String>)>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT remote_id, MAX(etag) FROM event WHERE calendar_id = ?1 GROUP BY remote_id",
        )?;
        let rows = stmt.query_map([calendar], |row| Ok((row.get(0)?, row.get(1)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Event `id`.
    pub fn event(&self, id: i64) -> Result<Option<StoredEvent>> {
        Ok(self
            .pim
            .prepare_cached(&format!("SELECT {EVENT_COLUMNS} FROM event WHERE id = ?1"))?
            .query_row([id], event_row)
            .optional()?)
    }

    /// The stored events of shown calendars that may have an occurrence
    /// in `from..to` (Unix seconds): single events and changed occurrences
    /// near the range, and every series that started before its end and
    /// has not ended before its start. Cancelled occurrences come too, as
    /// they take occurrences out of their series. `katna_dav` turns these
    /// into the occurrences themselves.
    pub fn event_rows_in_range(&self, from: i64, to: i64) -> Result<Vec<StoredEvent>> {
        let (from, to) = (from.saturating_sub(DAY), to.saturating_add(DAY));
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {EVENT_COLUMNS} FROM event
             WHERE calendar_id IN (SELECT id FROM calendar WHERE hidden = 0)
               AND pending != 2
               AND start < ?2
               AND (
                     (rrule = '' AND end > ?1)
                  OR (rrule != '' AND (range_end IS NULL OR range_end > ?1))
                  OR (recurrence_id IS NOT NULL AND recurrence_id >= ?1)
               )
             UNION
             SELECT {EVENT_COLUMNS} FROM event
             WHERE calendar_id IN (SELECT id FROM calendar WHERE hidden = 0)
               AND pending != 2
               AND recurrence_id IS NOT NULL AND recurrence_id >= ?1 AND recurrence_id < ?2
             ORDER BY start, id"
        ))?;
        let rows = stmt.query_map(params![from, to], event_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The changed occurrences of the series `uid` in calendar `calendar`.
    pub fn event_exceptions(&self, calendar: i64, uid: &str) -> Result<Vec<StoredEvent>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {EVENT_COLUMNS} FROM event
             WHERE calendar_id = ?1 AND uid = ?2 AND recurrence_id IS NOT NULL
             ORDER BY recurrence_id"
        ))?;
        let rows = stmt.query_map(params![calendar, uid], event_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The series (or single event) `uid` of calendar `calendar`: its row
    /// that is not a changed occurrence.
    pub fn event_series(&self, calendar: i64, uid: &str) -> Result<Option<StoredEvent>> {
        Ok(self
            .pim
            .prepare_cached(&format!(
                "SELECT {EVENT_COLUMNS} FROM event
                 WHERE calendar_id = ?1 AND uid = ?2 AND recurrence_id IS NULL
                 ORDER BY pending = 2, id LIMIT 1"
            ))?
            .query_row(params![calendar, uid], event_row)
            .optional()?)
    }

    /// The rows stored under `remote_id` in calendar `calendar` (a CalDAV
    /// resource: a series and its changed occurrences), with where each
    /// stands with the service.
    pub fn resource_events(
        &self,
        calendar: i64,
        remote_id: &str,
    ) -> Result<Vec<(StoredEvent, Pending)>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {EVENT_COLUMNS}, pending FROM event
             WHERE calendar_id = ?1 AND remote_id = ?2
             ORDER BY recurrence_id IS NOT NULL, recurrence_id, id"
        ))?;
        let rows = stmt.query_map(params![calendar, remote_id], |row| {
            Ok((event_row(row)?, Pending::parse(row.get(29)?)))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Where event `id` stands with its service; `None` if there is no
    /// such event.
    pub fn event_pending(&self, id: i64) -> Result<Option<Pending>> {
        Ok(self
            .pim
            .prepare_cached("SELECT pending FROM event WHERE id = ?1")?
            .query_row([id], |row| row.get::<_, i64>(0))
            .optional()?
            .map(Pending::parse))
    }

    /// Adds `event` to calendar `calendar`, as the user made it here.
    /// Returns its ID.
    pub fn add_event(&mut self, calendar: i64, event: &EventData, pending: Pending) -> Result<i64> {
        insert_event(&self.pim, calendar, &event.remote_id, event, pending)
    }

    /// Replaces what event `id` holds with `event`, as the user changed it
    /// here. Returns whether it exists.
    pub fn update_event(&mut self, id: i64, event: &EventData, pending: Pending) -> Result<bool> {
        let attendees = serde_json::to_string(&event.attendees).unwrap_or_else(|_| "[]".to_owned());
        let changed = self
            .pim
            .prepare_cached(
                "UPDATE event SET remote_id = ?2, uid = ?3, etag = ?4, recurrence_id = ?5,
                     status = ?6, title = ?7, location = ?8, description = ?9, start = ?10,
                     end = ?11, all_day = ?12, time_zone = ?13, rrule = ?14, exdates = ?15,
                     rdates = ?16, range_end = ?17, busy = ?18, kind = ?19, color = ?20,
                     organizer = ?21, organizer_name = ?22, attendees_json = ?23,
                     self_status = ?24, join_url = ?25, reminders = ?26, web_link = ?27,
                     updated_at = ?28, pending = ?29
                 WHERE id = ?1",
            )?
            .execute(params![
                id,
                event.remote_id,
                event.uid,
                event.etag,
                event.recurrence_id,
                event.status.as_str(),
                event.title,
                event.location,
                event.description,
                event.start,
                event.end,
                event.all_day,
                event.time_zone,
                event.rrule,
                join(&event.exdates),
                join(&event.rdates),
                event.range_end,
                event.busy,
                event.kind.as_str(),
                event.color,
                event.organizer,
                event.organizer_name,
                attendees,
                event.self_status,
                event.join_url,
                join(&event.reminders),
                event.web_link,
                event.updated_at,
                pending.as_int(),
            ])?;
        Ok(changed > 0)
    }

    /// Moves event `id` to calendar `calendar`.
    pub fn set_event_calendar(&mut self, id: i64, calendar: i64) -> Result<()> {
        self.pim.execute(
            "UPDATE event SET calendar_id = ?2 WHERE id = ?1",
            params![id, calendar],
        )?;
        Ok(())
    }

    /// Marks event `id` deleted here: kept, cancelled, until its service
    /// deleted it too.
    pub fn mark_event_deleted(&mut self, id: i64) -> Result<()> {
        self.pim.execute(
            "UPDATE event SET status = 'cancelled', pending = 2 WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    /// Deletes event `id` at once. Returns whether it existed.
    pub fn delete_event(&mut self, id: i64) -> Result<bool> {
        Ok(self.pim.execute("DELETE FROM event WHERE id = ?1", [id])? > 0)
    }

    /// Keeps what the service calls event `id` and its etag, once the
    /// service took it.
    pub fn set_event_remote(&mut self, id: i64, remote_id: &str, etag: Option<&str>) -> Result<()> {
        self.pim.execute(
            "UPDATE event SET remote_id = ?2, etag = ?3 WHERE id = ?1",
            params![id, remote_id, etag],
        )?;
        Ok(())
    }

    /// Keeps the link to the video call the service added to event `id`.
    pub fn set_event_join_url(&mut self, id: i64, url: &str) -> Result<()> {
        self.pim.execute(
            "UPDATE event SET join_url = ?2 WHERE id = ?1",
            params![id, url],
        )?;
        Ok(())
    }

    /// Sets the etag of every row under `remote_id` in calendar
    /// `calendar` (a CalDAV resource the server just took).
    pub fn set_resource_etag(
        &mut self,
        calendar: i64,
        remote_id: &str,
        etag: Option<&str>,
    ) -> Result<()> {
        self.pim.execute(
            "UPDATE event SET etag = ?3 WHERE calendar_id = ?1 AND remote_id = ?2",
            params![calendar, remote_id, etag],
        )?;
        Ok(())
    }

    /// Events `ids` are now as their service has them: those deleted here
    /// go, the others are synced again as usual.
    pub fn events_pushed(&mut self, ids: &[i64]) -> Result<()> {
        let tx = self.pim.transaction()?;
        for id in ids {
            tx.execute("DELETE FROM event WHERE id = ?1 AND pending = 2", [id])?;
            tx.execute("UPDATE event SET pending = 0 WHERE id = ?1", [id])?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Gives up sending the changes made here to events `ids` (every
    /// change, with `None`): their rows, and the rows stored with them,
    /// lose their etag and their calendars their sync token, so the next
    /// sync writes them again as the service has them. Returns the
    /// calendars that need that sync.
    pub fn forget_pending_events(&mut self, ids: Option<&[i64]>) -> Result<Vec<i64>> {
        let tx = self.pim.transaction()?;
        let rows: Vec<(i64, i64, String)> = {
            let mut stmt = tx.prepare_cached(
                "SELECT id, calendar_id, remote_id FROM event WHERE pending != 0",
            )?;
            let found = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
            found.collect::<rusqlite::Result<_>>()?
        };
        let mut calendars = Vec::new();
        for (id, calendar, remote_id) in rows {
            if ids.is_some_and(|ids| !ids.contains(&id)) {
                continue;
            }
            tx.execute(
                "UPDATE event SET etag = NULL WHERE calendar_id = ?1 AND remote_id = ?2",
                params![calendar, remote_id],
            )?;
            tx.execute("UPDATE event SET pending = 0 WHERE id = ?1", [id])?;
            if !calendars.contains(&calendar) {
                tx.execute(
                    "UPDATE calendar SET sync_token = NULL WHERE id = ?1",
                    [calendar],
                )?;
                calendars.push(calendar);
            }
        }
        tx.commit()?;
        Ok(calendars)
    }

    /// Makes a calendar on this computer when there is no calendar at all,
    /// so events can be added from the start. It has no name: apps show
    /// their own for it. Returns its ID when it made one.
    pub fn add_local_calendar_if_none(&mut self) -> Result<Option<i64>> {
        let any: bool =
            self.pim
                .query_row("SELECT EXISTS (SELECT 1 FROM calendar)", [], |row| {
                    row.get(0)
                })?;
        if any {
            return Ok(None);
        }
        self.pim.execute(
            "INSERT INTO calendar (account_id, source, remote_id, name, access, is_primary)
             VALUES (NULL, 'local', 'local', '', 'owner', 1)",
            [],
        )?;
        Ok(Some(self.pim.last_insert_rowid()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;
    use katna_core::Paths;

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(dir.path());
        let store = Store::open(&paths, Mode::ReadWrite).unwrap();
        (dir, store)
    }

    #[test]
    fn event_changes_read_back_as_sent() {
        let change = EventChange::Change {
            event: 7,
            scope: EditScope::Following,
            occurrence: Some(1_800_000_000),
            edit: EventEdit {
                title: "Standup".into(),
                start: 1_800_000_000,
                end: 1_800_001_800,
                rrule: "FREQ=WEEKLY;BYDAY=MO".into(),
                busy: true,
                reminders: vec![10],
                ..EventEdit::default()
            },
            calendar: None,
        };
        let json = serde_json::to_string(&change).unwrap();
        assert!(json.contains("\"op\":\"change\""), "{json}");
        assert_eq!(serde_json::from_str::<EventChange>(&json).unwrap(), change);
        let delete: EventChange = serde_json::from_str(r#"{"op":"delete","event":3}"#).unwrap();
        assert_eq!(
            delete,
            EventChange::Delete {
                event: 3,
                scope: EditScope::This,
                occurrence: None
            }
        );
    }

    fn work() -> NewCalendar {
        NewCalendar {
            remote_id: "work@example.com".into(),
            name: "Work".into(),
            color: "#039be5".into(),
            access: CalendarAccess::Owner,
            is_primary: true,
            time_zone: "Asia/Kolkata".into(),
        }
    }

    fn event(remote: &str, start: i64, end: i64) -> EventData {
        EventData {
            remote_id: remote.into(),
            uid: format!("{remote}@test"),
            title: remote.into(),
            start,
            end,
            range_end: Some(end),
            busy: true,
            ..EventData::default()
        }
    }

    #[test]
    fn calendars_keep_their_shown_state_across_syncs() {
        let (_dir, mut store) = store();
        let account = Some(AccountId(3));
        let id = store
            .upsert_calendar(account, CalendarSource::Google, &work(), 0)
            .unwrap();
        assert!(store.set_calendar_hidden(id, true).unwrap());
        store.set_calendar_sync_token(id, Some("t1")).unwrap();
        let mut renamed = work();
        renamed.name = "Office".into();
        assert_eq!(
            store
                .upsert_calendar(account, CalendarSource::Google, &renamed, 0)
                .unwrap(),
            id
        );
        let calendar = store.calendar(id).unwrap().unwrap();
        assert_eq!(calendar.name, "Office");
        assert!(calendar.hidden);
        assert_eq!(calendar.sync_token.as_deref(), Some("t1"));
        assert_eq!(calendar.account, Some(AccountId(3)));
        assert_eq!(calendar.access, CalendarAccess::Owner);
    }

    #[test]
    fn calendars_the_service_dropped_go_with_their_events() {
        let (_dir, mut store) = store();
        let account = AccountId(1);
        let keep = store
            .upsert_calendar(Some(account), CalendarSource::Google, &work(), 0)
            .unwrap();
        let mut other = work();
        other.remote_id = "holidays".into();
        let gone = store
            .upsert_calendar(Some(account), CalendarSource::Google, &other, 1)
            .unwrap();
        store
            .replace_events(gone, "a", &[event("a", 100, 200)])
            .unwrap();
        assert_eq!(
            store
                .remove_calendars_except(account, &["work@example.com".into()])
                .unwrap(),
            1
        );
        assert_eq!(store.calendars().unwrap().len(), 1);
        assert_eq!(store.calendars().unwrap()[0].id, keep);
        assert!(store.event_rows_in_range(0, 1000).unwrap().is_empty());
    }

    #[test]
    fn range_reads_find_single_events_and_series() {
        let (_dir, mut store) = store();
        let cal = store
            .upsert_calendar(None, CalendarSource::Local, &work(), 0)
            .unwrap();
        let day = DAY;
        store
            .replace_events(cal, "past", &[event("past", 0, 3600)])
            .unwrap();
        store
            .replace_events(cal, "in", &[event("in", 10 * day, 10 * day + 3600)])
            .unwrap();
        let mut series = event("weekly", 0, 3600);
        series.rrule = "FREQ=WEEKLY".into();
        series.range_end = None;
        store.replace_events(cal, "weekly", &[series]).unwrap();
        let mut ended = event("ended", 0, 3600);
        ended.rrule = "FREQ=DAILY;COUNT=2".into();
        ended.range_end = Some(day + 3600);
        store.replace_events(cal, "ended", &[ended]).unwrap();

        let titles: Vec<String> = store
            .event_rows_in_range(9 * day, 11 * day)
            .unwrap()
            .into_iter()
            .map(|e| e.data.title)
            .collect();
        assert_eq!(titles, ["weekly", "in"]);

        let hidden = store.calendars().unwrap()[0].id;
        store.set_calendar_hidden(hidden, true).unwrap();
        assert!(
            store
                .event_rows_in_range(9 * day, 11 * day)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn replacing_a_resource_keeps_exceptions_together() {
        let (_dir, mut store) = store();
        let cal = store
            .upsert_calendar(None, CalendarSource::Local, &work(), 0)
            .unwrap();
        let mut master = event("r.ics", 0, 3600);
        master.rrule = "FREQ=DAILY".into();
        master.range_end = None;
        master.attendees = vec![Attendee {
            email: "anita@example.com".into(),
            status: "accepted".into(),
            ..Attendee::default()
        }];
        master.exdates = vec![DAY];
        master.reminders = vec![10, 30];
        let mut moved = event("r.ics", 2 * DAY + 600, 2 * DAY + 4200);
        moved.uid = master.uid.clone();
        moved.recurrence_id = Some(2 * DAY);
        store
            .replace_events(cal, "r.ics", &[master.clone(), moved])
            .unwrap();
        let rows = store.event_rows_in_range(0, 3 * DAY).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].data, master);
        assert_eq!(
            store.event_exceptions(cal, &master.uid).unwrap()[0]
                .data
                .recurrence_id,
            Some(2 * DAY)
        );
        store.replace_events(cal, "r.ics", &[]).unwrap();
        assert!(store.event_rows_in_range(0, 3 * DAY).unwrap().is_empty());
    }

    #[test]
    fn a_sync_leaves_changes_the_service_does_not_have_yet() {
        let (_dir, mut store) = store();
        let cal = store
            .upsert_calendar(Some(AccountId(1)), CalendarSource::Google, &work(), 0)
            .unwrap();
        let mut synced = event("a", 100, 200);
        synced.etag = Some("1".into());
        store.replace_events(cal, "a", &[synced.clone()]).unwrap();
        let id = store.event_rows_in_range(0, 1000).unwrap()[0].id;

        let mut mine = synced.clone();
        mine.title = "Mine".into();
        assert!(store.update_event(id, &mine, Pending::Write).unwrap());
        let added = store
            .add_event(cal, &event("new", 300, 400), Pending::Write)
            .unwrap();
        // The service still has the old one, and not the new one.
        store
            .replace_events_batch(
                cal,
                &[("a".into(), vec![synced.clone()]), ("new".into(), vec![])],
            )
            .unwrap();
        store.remove_series(cal, "new@test").unwrap();
        store.clear_calendar_events(cal).unwrap();
        assert_eq!(store.event(id).unwrap().unwrap().data.title, "Mine");
        assert!(store.event(added).unwrap().is_some());
        assert_eq!(store.event_pending(id).unwrap(), Some(Pending::Write));

        // Once sent, the next sync writes it as the service has it.
        store.set_event_remote(added, "new-id", Some("7")).unwrap();
        store.events_pushed(&[id, added]).unwrap();
        assert_eq!(store.event_pending(added).unwrap(), Some(Pending::None));
        assert_eq!(
            store.event(added).unwrap().unwrap().data.remote_id,
            "new-id"
        );
        store.replace_events(cal, "a", &[synced]).unwrap();
        let titles: Vec<String> = store
            .event_rows_in_range(0, 1000)
            .unwrap()
            .into_iter()
            .map(|e| e.data.title)
            .collect();
        assert_eq!(titles, ["a", "new"]);
    }

    #[test]
    fn deleted_here_hides_at_once_and_goes_once_sent() {
        let (_dir, mut store) = store();
        let cal = store
            .upsert_calendar(Some(AccountId(1)), CalendarSource::Google, &work(), 0)
            .unwrap();
        store
            .replace_events(cal, "a", &[event("a", 100, 200)])
            .unwrap();
        let id = store.event_rows_in_range(0, 1000).unwrap()[0].id;
        store.mark_event_deleted(id).unwrap();
        assert!(store.event_rows_in_range(0, 1000).unwrap().is_empty());
        store
            .replace_events(cal, "a", &[event("a", 100, 200)])
            .unwrap();
        assert!(store.event_rows_in_range(0, 1000).unwrap().is_empty());
        store.events_pushed(&[id]).unwrap();
        assert!(store.event(id).unwrap().is_none());
    }

    #[test]
    fn a_change_the_service_refused_is_synced_over() {
        let (_dir, mut store) = store();
        let cal = store
            .upsert_calendar(Some(AccountId(1)), CalendarSource::CalDav, &work(), 0)
            .unwrap();
        store.set_calendar_sync_token(cal, Some("ctag")).unwrap();
        let mut master = event("r.ics", 0, 3600);
        master.etag = Some("e1".into());
        let mut moved = master.clone();
        moved.recurrence_id = Some(DAY);
        store
            .replace_events(cal, "r.ics", &[master.clone(), moved])
            .unwrap();
        let rows = store.resource_events(cal, "r.ics").unwrap();
        assert_eq!(rows.len(), 2);
        let id = rows[0].0.id;
        master.title = "Changed".into();
        store.update_event(id, &master, Pending::Write).unwrap();
        assert_eq!(store.forget_pending_events(Some(&[id])).unwrap(), [cal]);
        assert_eq!(store.event_pending(id).unwrap(), Some(Pending::None));
        assert_eq!(store.event_etags(cal).unwrap(), [("r.ics".into(), None)]);
        assert_eq!(store.calendar(cal).unwrap().unwrap().sync_token, None);
        assert!(store.forget_pending_events(None).unwrap().is_empty());
    }

    #[test]
    fn a_local_calendar_only_when_there_is_none() {
        let (_dir, mut store) = store();
        let id = store.add_local_calendar_if_none().unwrap().unwrap();
        let calendar = store.calendar(id).unwrap().unwrap();
        assert_eq!(calendar.source, CalendarSource::Local);
        assert_eq!(calendar.account, None);
        assert!(calendar.name.is_empty());
        assert!(calendar.access.can_edit());
        assert_eq!(store.add_local_calendar_if_none().unwrap(), None);
    }

    #[test]
    fn events_are_added_changed_and_moved_here() {
        let (_dir, mut store) = store();
        let one = store
            .upsert_calendar(None, CalendarSource::Local, &work(), 0)
            .unwrap();
        let mut other = work();
        other.remote_id = "other".into();
        let two = store
            .upsert_calendar(None, CalendarSource::Local, &other, 1)
            .unwrap();
        let mut series = event("s", 0, 3600);
        series.rrule = "FREQ=DAILY".into();
        series.range_end = None;
        let id = store.add_event(one, &series, Pending::None).unwrap();
        let mut moved = series.clone();
        moved.rrule.clear();
        moved.recurrence_id = Some(DAY);
        moved.start = DAY + 600;
        moved.end = DAY + 4200;
        moved.range_end = Some(moved.end);
        let exception = store.add_event(one, &moved, Pending::None).unwrap();
        assert_eq!(store.event_series(one, "s@test").unwrap().unwrap().id, id);
        assert_eq!(
            store.event_exceptions(one, "s@test").unwrap()[0].id,
            exception
        );
        series.title = "Renamed".into();
        store.update_event(id, &series, Pending::None).unwrap();
        store.set_event_calendar(id, two).unwrap();
        let back = store.event(id).unwrap().unwrap();
        assert_eq!(back.calendar_id, two);
        assert_eq!(back.data, series);
        assert!(store.delete_event(exception).unwrap());
        assert!(!store.delete_event(exception).unwrap());
    }
}
