// SPDX-License-Identifier: GPL-3.0-or-later

//! Outlook's calendars through Microsoft Graph (`/me/calendars` and each
//! calendar's `/events`: single events and series masters, times in UTC).
//! Graph has no sync token for a calendar's stored events, so each sync
//! lists them all and writes only those whose etag changed. A series'
//! `recurrence` becomes an `RRULE`. Changed occurrences come with the
//! series where Graph expands `exceptionOccurrences`, and cancelled ones
//! from `cancelledOccurrences`; where Graph refuses that expansion, series
//! show without their changed occurrences. Scope [`MICROSOFT_CALENDARS`],
//! a token of its own ([`TokenSource::access_token_for`]).

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

use jiff::civil::{Date, DateTime};
use jiff::tz::TimeZone;
use katna_core::AccountId;
use katna_dav::ical;
use katna_store::{
    Store,
    calendar::{
        Attendee, CalendarAccess, CalendarSource, EventData, EventKind, EventStatus, NewCalendar,
    },
};
use serde::{Deserialize, Deserializer};

use super::{CalendarError, Item, MAX_ANSWER, SyncResult, apply_full, hex_color, rfc3339};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    net::Tls,
    oauth::{MICROSOFT_CALENDARS, TokenSource},
};

/// Microsoft Graph.
pub const GRAPH_API: &str = "https://graph.microsoft.com/v1.0";

const TIMEOUT: Duration = Duration::from_secs(2 * 60);

/// Times in UTC, bodies as text.
const PREFER: &str = r#"outlook.timezone="UTC", outlook.body-content-type="text""#;

/// One Microsoft account's calendars.
pub struct GraphCalendar {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`GRAPH_API`], or a server under test.
    api: String,
    /// The calendars for which Graph refused
    /// `$expand=exceptionOccurrences`.
    no_exceptions: Mutex<HashSet<String>>,
}

/// `null` read as the default.
fn nullable<'de, D, T>(d: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

#[derive(Deserialize)]
struct Page<T> {
    #[serde(default = "Vec::new")]
    value: Vec<T>,
    #[serde(rename = "@odata.nextLink", default)]
    next: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct GraphCalendarEntry {
    #[serde(deserialize_with = "nullable")]
    id: String,
    #[serde(deserialize_with = "nullable")]
    name: String,
    #[serde(deserialize_with = "nullable")]
    color: String,
    #[serde(deserialize_with = "nullable")]
    hex_color: String,
    #[serde(deserialize_with = "nullable")]
    can_edit: bool,
    #[serde(deserialize_with = "nullable")]
    is_default_calendar: bool,
    owner: Option<Email>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Email {
    #[serde(deserialize_with = "nullable")]
    name: String,
    #[serde(deserialize_with = "nullable")]
    address: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Recipient {
    email_address: Option<Email>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct GraphAttendee {
    #[serde(rename = "type", deserialize_with = "nullable")]
    kind: String,
    status: Option<Response>,
    email_address: Option<Email>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Response {
    #[serde(deserialize_with = "nullable")]
    response: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct When {
    #[serde(deserialize_with = "nullable")]
    date_time: String,
    #[serde(deserialize_with = "nullable")]
    time_zone: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Pattern {
    #[serde(rename = "type", deserialize_with = "nullable")]
    kind: String,
    #[serde(deserialize_with = "nullable")]
    interval: u32,
    #[serde(deserialize_with = "nullable")]
    month: u32,
    #[serde(deserialize_with = "nullable")]
    day_of_month: u32,
    #[serde(deserialize_with = "nullable")]
    days_of_week: Vec<String>,
    #[serde(deserialize_with = "nullable")]
    first_day_of_week: String,
    #[serde(deserialize_with = "nullable")]
    index: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Range {
    #[serde(rename = "type", deserialize_with = "nullable")]
    kind: String,
    #[serde(deserialize_with = "nullable")]
    end_date: String,
    #[serde(deserialize_with = "nullable")]
    number_of_occurrences: u32,
    #[serde(deserialize_with = "nullable")]
    recurrence_time_zone: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Recurrence {
    pattern: Option<Pattern>,
    range: Option<Range>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Location {
    #[serde(deserialize_with = "nullable")]
    display_name: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct OnlineMeeting {
    #[serde(deserialize_with = "nullable")]
    join_url: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Body {
    #[serde(deserialize_with = "nullable")]
    content: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct GraphEvent {
    #[serde(deserialize_with = "nullable")]
    id: String,
    #[serde(rename = "@odata.etag")]
    etag: Option<String>,
    #[serde(rename = "iCalUId", deserialize_with = "nullable")]
    ical_uid: String,
    #[serde(deserialize_with = "nullable")]
    subject: String,
    body: Option<Body>,
    start: Option<When>,
    end: Option<When>,
    original_start: Option<String>,
    #[serde(deserialize_with = "nullable")]
    original_start_time_zone: String,
    #[serde(deserialize_with = "nullable")]
    is_all_day: bool,
    #[serde(deserialize_with = "nullable")]
    is_cancelled: bool,
    #[serde(deserialize_with = "nullable")]
    show_as: String,
    #[serde(rename = "type", deserialize_with = "nullable")]
    kind: String,
    recurrence: Option<Recurrence>,
    location: Option<Location>,
    organizer: Option<Recipient>,
    #[serde(deserialize_with = "nullable")]
    attendees: Vec<GraphAttendee>,
    response_status: Option<Response>,
    #[serde(deserialize_with = "nullable")]
    is_organizer: bool,
    online_meeting: Option<OnlineMeeting>,
    #[serde(deserialize_with = "nullable")]
    online_meeting_url: String,
    #[serde(deserialize_with = "nullable")]
    web_link: String,
    #[serde(deserialize_with = "nullable")]
    is_reminder_on: bool,
    #[serde(deserialize_with = "nullable")]
    reminder_minutes_before_start: i64,
    #[serde(deserialize_with = "nullable")]
    last_modified_date_time: String,
    #[serde(deserialize_with = "nullable")]
    exception_occurrences: Vec<GraphEvent>,
    #[serde(deserialize_with = "nullable")]
    cancelled_occurrences: Vec<String>,
}

/// Outlook's named calendar colours.
fn named_color(name: &str) -> &'static str {
    match name {
        "lightBlue" => "#a4c2f4",
        "lightGreen" => "#93c47d",
        "lightOrange" => "#f6b26b",
        "lightGray" => "#b7b7b7",
        "lightYellow" => "#ffd966",
        "lightTeal" => "#76d7c4",
        "lightPink" => "#f4a6c7",
        "lightBrown" => "#c9a17a",
        "lightRed" => "#e06666",
        _ => "",
    }
}

impl GraphCalendar {
    /// Microsoft's calendars, or the server under test in
    /// `KATNA_GRAPH_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GRAPH_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(GRAPH_API))
    }

    /// Talks to `api` instead of Microsoft, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
            no_exceptions: Mutex::default(),
        }
    }

    /// Whether the account's sign-in allowed Katna into its calendars.
    /// Accounts signed in before Katna asked have to sign in again.
    pub async fn allowed(&self) -> Result<bool> {
        match self.tokens.access_token_for(MICROSOFT_CALENDARS).await {
            Ok(_) => Ok(true),
            Err(Error::Auth(_)) => Ok(false),
            Err(err) => Err(err),
        }
    }

    /// A GET with Graph's access token, trying once more with a fresh
    /// token when Graph refuses the one it had.
    async fn get(&self, url: &str) -> Result<Reply> {
        // Links Graph gives lead back to Graph only.
        if !url.starts_with(&self.api) {
            return Err(Error::Protocol(format!("Graph linked elsewhere: {url}")));
        }
        loop {
            let token = format!(
                "Bearer {}",
                self.tokens.access_token_for(MICROSOFT_CALENDARS).await?
            );
            let headers = [("Authorization", token.as_str()), ("Prefer", PREFER)];
            let reply = http::exchange_limited(
                "GET", url, &headers, None, None, &self.tls, TIMEOUT, MAX_ANSWER,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token_for(MICROSOFT_CALENDARS) {
                continue;
            }
            return Ok(reply);
        }
    }

    /// Every page from `url` on.
    async fn all<T: for<'a> Deserialize<'a>>(
        &self,
        url: String,
        doing: &str,
    ) -> std::result::Result<Vec<T>, CalendarError> {
        let mut out = Vec::new();
        let mut next = Some(url);
        while let Some(url) = next.take() {
            let reply = self.get(&url).await?;
            check(&reply, doing)?;
            let page: Page<T> = parse(&reply.body)?;
            out.extend(page.value);
            next = page.next;
        }
        Ok(out)
    }

    /// The events of calendar `id`, series with their changed
    /// occurrences where Graph gives them.
    async fn events(&self, id: &str) -> std::result::Result<Vec<GraphEvent>, CalendarError> {
        let base = format!(
            "{}/me/calendars/{}/events?$top=250",
            self.api,
            http::escape(id)
        );
        let refused = self.no_exceptions.lock().unwrap().contains(id);
        if !refused {
            let expanded = format!("{base}&$expand=exceptionOccurrences");
            match self.all(expanded, "listing events").await {
                Err(CalendarError::Failed(Error::Rejected(message)))
                    if message.contains("status 400") || message.contains("expand") =>
                {
                    tracing::info!(%message, "Graph doesn't expand changed occurrences");
                    self.no_exceptions.lock().unwrap().insert(id.to_owned());
                }
                other => return other,
            }
        }
        self.all(base, "listing events").await
    }

    /// Brings `account`'s calendars and their events into `store`;
    /// `address` is the user's. Returns whether anything changed.
    pub async fn sync(&self, store: &mut Store, account: AccountId, address: &str) -> SyncResult {
        if !self.allowed().await? {
            return Err(CalendarError::NeedsSignIn(
                "sign in again to allow Katna into the Outlook calendar".into(),
            ));
        }
        let listed: Vec<GraphCalendarEntry> = self
            .all(
                format!("{}/me/calendars?$top=100", self.api),
                "listing the calendars",
            )
            .await?;
        let mut changed = false;
        let mut ids = Vec::new();
        let before = store.calendars()?;
        for (position, entry) in listed.iter().enumerate() {
            let own = entry
                .owner
                .as_ref()
                .is_some_and(|o| !address.is_empty() && o.address.eq_ignore_ascii_case(address));
            let color = match hex_color(&entry.hex_color) {
                hex if hex.is_empty() => named_color(&entry.color).to_owned(),
                hex => hex,
            };
            let calendar = NewCalendar {
                remote_id: entry.id.clone(),
                name: entry.name.clone(),
                color,
                access: match (entry.can_edit, own || entry.is_default_calendar) {
                    (true, true) => CalendarAccess::Owner,
                    (true, false) => CalendarAccess::Writer,
                    _ => CalendarAccess::Reader,
                },
                is_primary: entry.is_default_calendar,
                time_zone: String::new(),
            };
            let id = store.upsert_calendar(
                Some(account),
                CalendarSource::Microsoft,
                &calendar,
                position as i64,
            )?;
            let old = before.iter().find(|c| c.id == id);
            changed |= old.is_none_or(|old| {
                old.name != calendar.name
                    || old.color != calendar.color
                    || old.access != calendar.access
                    || old.position != position as i64
            });
            ids.push(id);
        }
        let keep: Vec<String> = listed.iter().map(|c| c.id.clone()).collect();
        changed |= store.remove_calendars_except(account, &keep)? > 0;

        for (entry, id) in listed.iter().zip(ids) {
            let events = match self.events(&entry.id).await {
                Ok(events) => events,
                Err(CalendarError::Failed(err)) => {
                    tracing::warn!(calendar = entry.name, %err, "calendar not synced");
                    continue;
                }
                Err(err) => return Err(err),
            };
            let mut items: Vec<Item> = Vec::new();
            for event in &events {
                items.extend(map_series(event, address));
            }
            changed |= apply_full(store, id, items)?;
        }
        Ok(changed)
    }
}

/// A Graph time: its zone's IANA name when it has one, else UTC (as the
/// `Prefer` header asks).
fn instant(when: &When) -> Option<i64> {
    let local: DateTime = when.date_time.trim_end_matches('Z').parse().ok()?;
    let zone = ical::zone(&when.time_zone).unwrap_or(TimeZone::UTC);
    Some(local.to_zoned(zone).ok()?.timestamp().as_second())
}

/// The day of a whole-day event's start or end, whatever zone it is in.
fn day(when: &When) -> Option<Date> {
    when.date_time.get(..10)?.parse().ok()
}

fn response(value: &str) -> String {
    match value {
        "accepted" | "organizer" => "accepted",
        "tentativelyAccepted" => "tentative",
        "declined" => "declined",
        "notResponded" => "needs_action",
        _ => "",
    }
    .to_owned()
}

fn weekday(name: &str) -> Option<&'static str> {
    Some(match name.to_ascii_lowercase().as_str() {
        "monday" => "MO",
        "tuesday" => "TU",
        "wednesday" => "WE",
        "thursday" => "TH",
        "friday" => "FR",
        "saturday" => "SA",
        "sunday" => "SU",
        _ => return None,
    })
}

/// A series' `recurrence` as an `RRULE` value; `None` for a pattern it
/// can't say.
fn rrule(recurrence: &Recurrence) -> Option<String> {
    let pattern = recurrence.pattern.as_ref()?;
    let days: Vec<&str> = pattern
        .days_of_week
        .iter()
        .filter_map(|d| weekday(d))
        .collect();
    let position = match pattern.index.as_str() {
        "second" => 2,
        "third" => 3,
        "fourth" => 4,
        "last" => -1,
        _ => 1,
    };
    // `2MO`, or `MO,TU` with a set position.
    let relative = |parts: &mut Vec<String>| -> Option<()> {
        match days.as_slice() {
            [] => return None,
            [one] => parts.push(format!("BYDAY={position}{one}")),
            many => {
                parts.push(format!("BYDAY={}", many.join(",")));
                parts.push(format!("BYSETPOS={position}"));
            }
        }
        Some(())
    };
    let mut parts = Vec::new();
    match pattern.kind.as_str() {
        "daily" => parts.push("FREQ=DAILY".to_owned()),
        "weekly" => {
            parts.push("FREQ=WEEKLY".to_owned());
            if !days.is_empty() {
                parts.push(format!("BYDAY={}", days.join(",")));
            }
        }
        "absoluteMonthly" => {
            parts.push("FREQ=MONTHLY".to_owned());
            parts.push(format!("BYMONTHDAY={}", pattern.day_of_month.max(1)));
        }
        "relativeMonthly" => {
            parts.push("FREQ=MONTHLY".to_owned());
            relative(&mut parts)?;
        }
        "absoluteYearly" => {
            parts.push("FREQ=YEARLY".to_owned());
            parts.push(format!("BYMONTH={}", pattern.month.clamp(1, 12)));
            parts.push(format!("BYMONTHDAY={}", pattern.day_of_month.max(1)));
        }
        "relativeYearly" => {
            parts.push("FREQ=YEARLY".to_owned());
            parts.push(format!("BYMONTH={}", pattern.month.clamp(1, 12)));
            relative(&mut parts)?;
        }
        _ => return None,
    }
    if pattern.interval > 1 {
        parts.push(format!("INTERVAL={}", pattern.interval));
    }
    if let Some(range) = &recurrence.range {
        match range.kind.as_str() {
            "endDate" => {
                let end: Date = range.end_date.parse().ok()?;
                parts.push(format!(
                    "UNTIL={:04}{:02}{:02}",
                    end.year(),
                    end.month(),
                    end.day()
                ));
            }
            "numbered" if range.number_of_occurrences > 0 => {
                parts.push(format!("COUNT={}", range.number_of_occurrences));
            }
            _ => {}
        }
    }
    if pattern.kind == "weekly"
        && let Some(start) = weekday(&pattern.first_day_of_week)
        && start != "MO"
    {
        parts.push(format!("WKST={start}"));
    }
    Some(parts.join(";"))
}

/// A series (or single event) as store items: itself, and its changed
/// and cancelled occurrences.
fn map_series(event: &GraphEvent, address: &str) -> Vec<Item> {
    let Some(master) = map_event(event, address) else {
        return Vec::new();
    };
    let mut items = Vec::new();
    let mut master = master;
    // Cancelled occurrences: `OID.<id>.<yyyy-mm-dd>`, the day of the start
    // they had.
    for oid in &event.cancelled_occurrences {
        let Some(day) = oid.rsplit('.').next().and_then(|d| d.parse::<Date>().ok()) else {
            continue;
        };
        if let Some(start) = occurrence_on(&master, day) {
            master.exdates.push(start);
        }
    }
    master.range_end = ical::range_end(&master);
    for exception in &event.exception_occurrences {
        let Some(mut data) = map_event(exception, address) else {
            continue;
        };
        let original = exception
            .original_start
            .as_deref()
            .and_then(|s| s.parse::<jiff::Timestamp>().ok());
        let Some(original) = original else {
            continue;
        };
        data.recurrence_id = Some(if master.all_day {
            let zone = TimeZone::get(&master.time_zone).unwrap_or(TimeZone::UTC);
            ical::utc_midnight(original.to_zoned(zone).date())
        } else {
            original.as_second()
        });
        data.uid = master.uid.clone();
        data.rrule.clear();
        data.range_end = Some(data.end);
        items.push((data.remote_id.clone(), data.etag.clone(), vec![data]));
    }
    items.insert(
        0,
        (master.remote_id.clone(), master.etag.clone(), vec![master]),
    );
    items
}

/// The start a series' occurrence on `day` has.
fn occurrence_on(master: &EventData, day: Date) -> Option<i64> {
    if master.all_day {
        return Some(ical::utc_midnight(day));
    }
    let zone = TimeZone::get(&master.time_zone).unwrap_or(TimeZone::UTC);
    let first = jiff::Timestamp::from_second(master.start)
        .ok()?
        .to_zoned(zone.clone());
    let at = DateTime::from_parts(day, first.time())
        .to_zoned(zone)
        .ok()?;
    Some(at.timestamp().as_second())
}

/// Graph's event as the store keeps it; `None` without a time.
fn map_event(event: &GraphEvent, address: &str) -> Option<EventData> {
    let (start, end) = (event.start.as_ref()?, event.end.as_ref()?);
    let zone = ical::zone_name(&event.original_start_time_zone)
        .or_else(|| {
            event
                .recurrence
                .as_ref()
                .and_then(|r| r.range.as_ref())
                .and_then(|r| ical::zone_name(&r.recurrence_time_zone))
        })
        .unwrap_or_default();
    let (start, end) = if event.is_all_day {
        (
            ical::utc_midnight(day(start)?),
            ical::utc_midnight(day(end)?),
        )
    } else {
        (instant(start)?, instant(end)?)
    };
    let end = end.max(start);
    let uid = if event.ical_uid.is_empty() {
        event.id.clone()
    } else {
        event.ical_uid.clone()
    };
    let mut data = EventData {
        remote_id: event.id.clone(),
        uid,
        etag: event.etag.clone(),
        status: if event.is_cancelled {
            EventStatus::Cancelled
        } else {
            EventStatus::Confirmed
        },
        title: event.subject.clone(),
        location: event
            .location
            .as_ref()
            .map(|l| l.display_name.clone())
            .unwrap_or_default(),
        description: event
            .body
            .as_ref()
            .map(|b| b.content.trim().to_owned())
            .unwrap_or_default(),
        start,
        end,
        all_day: event.is_all_day,
        time_zone: if event.is_all_day {
            String::new()
        } else {
            zone
        },
        busy: event.show_as != "free",
        kind: match event.show_as.as_str() {
            "oof" => EventKind::OutOfOffice,
            "workingElsewhere" => EventKind::WorkingLocation,
            _ => EventKind::Default,
        },
        web_link: event.web_link.clone(),
        updated_at: rfc3339(&event.last_modified_date_time),
        ..EventData::default()
    };
    if event.kind == "seriesMaster"
        && let Some(rule) = event.recurrence.as_ref().and_then(rrule)
    {
        data.rrule = rule;
    }
    if let Some(email) = event
        .organizer
        .as_ref()
        .and_then(|o| o.email_address.as_ref())
    {
        data.organizer = email.address.clone();
        data.organizer_name = email.name.clone();
    }
    for attendee in &event.attendees {
        let Some(email) = &attendee.email_address else {
            continue;
        };
        let is_self = !address.is_empty() && email.address.eq_ignore_ascii_case(address);
        data.attendees.push(Attendee {
            email: email.address.clone(),
            name: email.name.clone(),
            status: response(attendee.status.as_ref().map_or("", |s| s.response.as_str())),
            optional: attendee.kind == "optional",
            is_self,
            organizer: email.address.eq_ignore_ascii_case(&data.organizer),
        });
    }
    data.self_status = if event.is_organizer {
        "accepted".to_owned()
    } else {
        response(
            event
                .response_status
                .as_ref()
                .map_or("", |s| s.response.as_str()),
        )
    };
    let join = event
        .online_meeting
        .as_ref()
        .map(|m| m.join_url.clone())
        .filter(|u| u.starts_with("https://"))
        .or_else(|| Some(event.online_meeting_url.clone()).filter(|u| u.starts_with("https://")));
    data.join_url = join.unwrap_or_default();
    if event.is_reminder_on {
        data.reminders = vec![event.reminder_minutes_before_start.max(0)];
    }
    data.range_end = ical::range_end(&data);
    Some(data)
}

fn parse<T: for<'a> Deserialize<'a>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(body).map_err(|err| Error::Protocol(format!("Graph answer: {err}")))
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Failure {
    error: FailureBody,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct FailureBody {
    code: String,
    message: String,
}

/// A `2xx` answer is fine; a refused token or permission asks to sign in
/// again; anything else is Graph's own message.
fn check(reply: &Reply, doing: &str) -> std::result::Result<(), CalendarError> {
    if (200..300).contains(&reply.status) {
        return Ok(());
    }
    let failure: Failure = serde_json::from_slice(&reply.body).unwrap_or_default();
    let denied = matches!(
        failure.error.code.as_str(),
        "ErrorAccessDenied" | "accessDenied" | "InvalidAuthenticationToken"
    );
    if reply.status == 401 || (reply.status == 403 && denied) {
        return Err(CalendarError::NeedsSignIn(format!(
            "Outlook refused access to the calendar while {doing}"
        )));
    }
    let detail = if failure.error.message.is_empty() {
        format!("status {}", reply.status)
    } else {
        format!("status {}: {}", reply.status, failure.error.message)
    };
    Err(CalendarError::Failed(Error::Rejected(format!(
        "Outlook calendar, {doing}: {detail}"
    ))))
}

#[cfg(test)]
mod tests;
