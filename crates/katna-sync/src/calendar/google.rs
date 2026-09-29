// SPDX-License-Identifier: GPL-3.0-or-later

//! Google Calendar API v3: the calendar list, and each calendar's events
//! as stored (series with their rule, changed occurrences apart:
//! `singleEvents=false`), incrementally with the sync token Google gives
//! at the end of each listing. A token Google no longer takes (`410
//! Gone`) starts a full listing again. Scope [`GOOGLE_CALENDAR`], asked
//! at sign-in; accounts signed in before have to sign in again.

use std::{collections::HashMap, sync::Arc, time::Duration};

use jiff::tz::TimeZone;
use katna_core::AccountId;
use katna_dav::ical;
use katna_store::{
    Store,
    calendar::{
        Attendee, CalendarAccess, CalendarSource, EventData, EventKind, EventStatus, NewCalendar,
    },
};
use serde::Deserialize;

use super::{CalendarError, Item, MAX_ANSWER, SyncResult, apply_full, hex_color, rfc3339};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    net::Tls,
    oauth::{GOOGLE_CALENDAR, TokenSource},
};

/// Google's API host.
pub const GOOGLE_API: &str = "https://www.googleapis.com";

const TIMEOUT: Duration = Duration::from_secs(2 * 60);

/// Google's fixed event colours, by `colorId`.
const EVENT_COLORS: [&str; 11] = [
    "#7986cb", "#33b679", "#8e24aa", "#e67c73", "#f6bf26", "#f4511e", "#039be5", "#616161",
    "#3f51b5", "#0b8043", "#d50000",
];

/// One Google account's calendars.
#[derive(Clone)]
pub struct GoogleCalendar {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`GOOGLE_API`], or a server under test.
    api: String,
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct CalendarList {
    items: Vec<ListEntry>,
    next_page_token: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ListEntry {
    id: String,
    summary: String,
    summary_override: String,
    background_color: String,
    access_role: String,
    primary: bool,
    time_zone: String,
    default_reminders: Vec<Reminder>,
    deleted: bool,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Reminder {
    minutes: i64,
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct Events {
    items: Vec<Event>,
    next_page_token: Option<String>,
    next_sync_token: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Time {
    date: Option<String>,
    date_time: Option<String>,
    time_zone: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Person {
    email: String,
    display_name: String,
    response_status: String,
    optional: bool,
    #[serde(rename = "self")]
    is_self: bool,
    organizer: bool,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Reminders {
    use_default: bool,
    overrides: Vec<Reminder>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Conference {
    entry_points: Vec<EntryPoint>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct EntryPoint {
    entry_point_type: String,
    uri: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
struct Event {
    id: String,
    etag: Option<String>,
    status: String,
    html_link: String,
    summary: String,
    description: String,
    location: String,
    color_id: String,
    start: Option<Time>,
    end: Option<Time>,
    recurrence: Vec<String>,
    recurring_event_id: Option<String>,
    original_start_time: Option<Time>,
    transparency: String,
    #[serde(rename = "iCalUID")]
    ical_uid: Option<String>,
    attendees: Vec<Person>,
    organizer: Option<Person>,
    hangout_link: String,
    conference_data: Option<Conference>,
    event_type: String,
    reminders: Option<Reminders>,
    updated: String,
}

/// What Google says about a calendar, beside what the store keeps.
struct Listed {
    calendar: NewCalendar,
    default_reminders: Vec<i64>,
}

/// One listing of a calendar's events.
enum Listing {
    Done {
        events: Vec<Event>,
        sync_token: Option<String>,
    },
    /// The sync token ran out: list everything again.
    Gone,
}

impl GoogleCalendar {
    /// Google's calendars, or the server under test in
    /// `KATNA_GOOGLE_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GOOGLE_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(GOOGLE_API))
    }

    /// Talks to `api` instead of Google, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna into its calendars.
    /// Accounts signed in before Katna asked have to sign in again.
    pub async fn allowed(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_CALENDAR).await
    }

    /// A GET with the account's access token, trying once more with a
    /// fresh token when Google refuses the one it had.
    async fn get(&self, url: &str) -> Result<Reply> {
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let headers = [("Authorization", token.as_str())];
            let reply = http::exchange_limited(
                "GET", url, &headers, None, None, &self.tls, TIMEOUT, MAX_ANSWER,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    async fn calendars(&self) -> std::result::Result<Vec<Listed>, CalendarError> {
        let mut out = Vec::new();
        let mut page: Option<String> = None;
        loop {
            let mut url = format!(
                "{}/calendar/v3/users/me/calendarList?maxResults=250",
                self.api
            );
            if let Some(page) = &page {
                url.push_str(&format!("&pageToken={}", http::escape(page)));
            }
            let reply = self.get(&url).await?;
            check(&reply, "listing the calendars")?;
            let list: CalendarList = parse(&reply.body)?;
            for entry in list.items.into_iter().filter(|e| !e.deleted) {
                let name = if entry.summary_override.is_empty() {
                    entry.summary
                } else {
                    entry.summary_override
                };
                out.push(Listed {
                    calendar: NewCalendar {
                        remote_id: entry.id,
                        name,
                        color: hex_color(&entry.background_color),
                        access: match entry.access_role.as_str() {
                            "owner" => CalendarAccess::Owner,
                            "writer" => CalendarAccess::Writer,
                            "freeBusyReader" => CalendarAccess::FreeBusy,
                            _ => CalendarAccess::Reader,
                        },
                        is_primary: entry.primary,
                        time_zone: entry.time_zone,
                    },
                    default_reminders: entry.default_reminders.iter().map(|r| r.minutes).collect(),
                });
            }
            match list.next_page_token {
                Some(next) if !next.is_empty() => page = Some(next),
                _ => return Ok(out),
            }
        }
    }

    /// The events of calendar `id`: all of them, or with `sync_token`
    /// those changed since.
    async fn events(
        &self,
        id: &str,
        sync_token: Option<&str>,
    ) -> std::result::Result<Listing, CalendarError> {
        let mut events = Vec::new();
        let mut page: Option<String> = None;
        loop {
            let mut url = format!(
                "{}/calendar/v3/calendars/{}/events?singleEvents=false&showDeleted=true&maxResults=2500",
                self.api,
                http::escape(id)
            );
            if let Some(token) = sync_token {
                url.push_str(&format!("&syncToken={}", http::escape(token)));
            }
            if let Some(page) = &page {
                url.push_str(&format!("&pageToken={}", http::escape(page)));
            }
            let reply = self.get(&url).await?;
            if reply.status == 410 && sync_token.is_some() {
                return Ok(Listing::Gone);
            }
            check(&reply, "listing events")?;
            let listed: Events = parse(&reply.body)?;
            events.extend(listed.items);
            match listed.next_page_token {
                Some(next) if !next.is_empty() => page = Some(next),
                _ => {
                    return Ok(Listing::Done {
                        events,
                        sync_token: listed.next_sync_token,
                    });
                }
            }
        }
    }

    /// Brings `account`'s calendars and their events into `store`.
    /// Returns whether anything changed.
    pub async fn sync(&self, store: &mut Store, account: AccountId) -> SyncResult {
        if !self.allowed().await? {
            return Err(CalendarError::NeedsSignIn(
                "sign in again to allow Katna into Google Calendar".into(),
            ));
        }
        let listed = self.calendars().await?;
        let mut changed = false;
        let mut ids = Vec::new();
        {
            let before = store.calendars()?;
            for (position, entry) in listed.iter().enumerate() {
                let id = store.upsert_calendar(
                    Some(account),
                    CalendarSource::Google,
                    &entry.calendar,
                    position as i64,
                )?;
                let old = before.iter().find(|c| c.id == id);
                changed |= old.is_none_or(|old| {
                    old.name != entry.calendar.name
                        || old.color != entry.calendar.color
                        || old.access != entry.calendar.access
                        || old.position != position as i64
                });
                ids.push(id);
            }
            let keep: Vec<String> = listed
                .iter()
                .map(|l| l.calendar.remote_id.clone())
                .collect();
            changed |= store.remove_calendars_except(account, &keep)? > 0;
        }
        for (entry, id) in listed.iter().zip(ids) {
            match self.sync_calendar(store, id, entry).await {
                Ok(c) => changed |= c,
                // A calendar Google stopped sharing, for example.
                Err(CalendarError::Failed(err)) => {
                    tracing::warn!(calendar = entry.calendar.remote_id, %err, "calendar not synced");
                }
                Err(err) => return Err(err),
            }
        }
        Ok(changed)
    }

    async fn sync_calendar(&self, store: &mut Store, id: i64, entry: &Listed) -> SyncResult {
        let token = store.calendar(id)?.and_then(|c| c.sync_token);
        let zone = ical::zone(&entry.calendar.time_zone).unwrap_or_else(TimeZone::system);
        let map = |event: &Event, uids: &HashMap<String, String>| {
            map_event(event, &zone, &entry.default_reminders, uids)
        };
        if let Some(token) = token {
            match self.events(&entry.calendar.remote_id, Some(&token)).await? {
                Listing::Done { events, sync_token } => {
                    let changed = apply_changes(store, id, &events, map)?;
                    store.set_calendar_sync_token(id, sync_token.as_deref())?;
                    return Ok(changed);
                }
                Listing::Gone => {
                    tracing::info!(calendar = entry.calendar.remote_id, "sync token expired");
                    store.set_calendar_sync_token(id, None)?;
                }
            }
        }
        let Listing::Done { events, sync_token } =
            self.events(&entry.calendar.remote_id, None).await?
        else {
            return Err(CalendarError::Failed(Error::Protocol(
                "Google Calendar refused a full listing".into(),
            )));
        };
        // Changed occurrences that Google cancelled may lack their UID:
        // they take their series'.
        let uids: HashMap<String, String> = events
            .iter()
            .filter(|e| e.recurring_event_id.is_none())
            .filter_map(|e| Some((e.id.clone(), e.ical_uid.clone()?)))
            .collect();
        let items: Vec<Item> = events
            .iter()
            // A deleted event, or a deleted series.
            .filter(|e| !(e.status == "cancelled" && e.recurring_event_id.is_none()))
            .filter_map(|e| {
                let data = map(e, &uids)?;
                Some((e.id.clone(), e.etag.clone(), vec![data]))
            })
            .collect();
        let changed = apply_full(store, id, items)?;
        store.set_calendar_sync_token(id, sync_token.as_deref())?;
        Ok(changed)
    }
}

/// Writes the events of an incremental listing. Returns whether any came.
fn apply_changes(
    store: &mut Store,
    calendar: i64,
    events: &[Event],
    map: impl Fn(&Event, &HashMap<String, String>) -> Option<EventData>,
) -> std::result::Result<bool, katna_store::Error> {
    let mut uids: HashMap<String, String> = HashMap::new();
    for event in events {
        if event.recurring_event_id.is_none() {
            let uid = match &event.ical_uid {
                Some(uid) => Some(uid.clone()),
                None => store.event_uid(calendar, &event.id)?,
            };
            if let Some(uid) = uid {
                uids.insert(event.id.clone(), uid);
            }
        } else if let Some(master) = &event.recurring_event_id
            && !uids.contains_key(master)
            && let Some(uid) = store.event_uid(calendar, master)?
        {
            uids.insert(master.clone(), uid);
        }
    }
    let mut writes = Vec::new();
    for event in events {
        if event.status == "cancelled" && event.recurring_event_id.is_none() {
            // The event or the whole series went, with its changed
            // occurrences.
            if let Some(uid) = uids.get(&event.id) {
                store.remove_series(calendar, uid)?;
            }
            writes.push((event.id.clone(), Vec::new()));
            continue;
        }
        let rows = map(event, &uids).into_iter().collect();
        writes.push((event.id.clone(), rows));
    }
    if writes.is_empty() {
        return Ok(false);
    }
    store.replace_events_batch(calendar, &writes)?;
    Ok(true)
}

/// A start or end: Unix seconds, whether a whole day, and its zone.
fn time(time: &Time, fallback: &TimeZone) -> Option<(i64, bool, String)> {
    if let Some(date) = &time.date {
        let day: jiff::civil::Date = date.parse().ok()?;
        return Some((ical::utc_midnight(day), true, String::new()));
    }
    let at: jiff::Timestamp = time.date_time.as_deref()?.parse().ok()?;
    let zone = time
        .time_zone
        .as_deref()
        .and_then(ical::zone_name)
        .or_else(|| fallback.iana_name().map(str::to_owned))
        .unwrap_or_default();
    Some((at.as_second(), false, zone))
}

fn response(status: &str) -> String {
    match status {
        "accepted" => "accepted",
        "tentative" => "tentative",
        "declined" => "declined",
        "needsAction" => "needs_action",
        _ => "",
    }
    .to_owned()
}

/// Google's event as the store keeps it. `uids` has the UIDs of series by
/// their ID, for changed occurrences that don't say theirs. `None` when
/// it has no time.
fn map_event(
    event: &Event,
    calendar_zone: &TimeZone,
    default_reminders: &[i64],
    uids: &HashMap<String, String>,
) -> Option<EventData> {
    let original = event
        .original_start_time
        .as_ref()
        .and_then(|t| time(t, calendar_zone));
    let recurrence_id = event
        .recurring_event_id
        .as_ref()
        .and(original.as_ref())
        .map(|(s, _, _)| *s);
    let (start, all_day, zone) = match event.start.as_ref().and_then(|t| time(t, calendar_zone)) {
        Some(start) => start,
        // A cancelled occurrence says only which one it was.
        None => original.clone()?,
    };
    let end = event
        .end
        .as_ref()
        .and_then(|t| time(t, calendar_zone))
        .map_or(if all_day { start + 86_400 } else { start }, |(e, _, _)| e)
        .max(start);
    let uid = event
        .ical_uid
        .clone()
        .or_else(|| {
            event
                .recurring_event_id
                .as_ref()
                .and_then(|m| uids.get(m).cloned())
        })
        .or_else(|| event.recurring_event_id.clone())
        .unwrap_or_else(|| event.id.clone());
    let mut data = EventData {
        remote_id: event.id.clone(),
        uid,
        etag: event.etag.clone(),
        recurrence_id,
        status: match event.status.as_str() {
            "cancelled" => EventStatus::Cancelled,
            "tentative" => EventStatus::Tentative,
            _ => EventStatus::Confirmed,
        },
        title: event.summary.clone(),
        location: event.location.clone(),
        description: event.description.clone(),
        start,
        end,
        all_day,
        time_zone: zone.clone(),
        busy: event.transparency != "transparent",
        kind: match event.event_type.as_str() {
            "focusTime" => EventKind::Focus,
            "outOfOffice" => EventKind::OutOfOffice,
            "workingLocation" => EventKind::WorkingLocation,
            "birthday" => EventKind::Birthday,
            _ => EventKind::Default,
        },
        color: event
            .color_id
            .parse::<usize>()
            .ok()
            .and_then(|n| EVENT_COLORS.get(n.wrapping_sub(1)))
            .map(|c| (*c).to_owned())
            .unwrap_or_default(),
        web_link: event.html_link.clone(),
        updated_at: rfc3339(&event.updated),
        ..EventData::default()
    };
    let series_zone = if all_day {
        TimeZone::UTC
    } else {
        TimeZone::get(&zone).unwrap_or_else(|_| calendar_zone.clone())
    };
    for line in &event.recurrence {
        let Some(prop) = ical::property(line) else {
            continue;
        };
        match prop.name.as_str() {
            "RRULE" if data.rrule.is_empty() => data.rrule = prop.value.trim().to_owned(),
            "EXDATE" => data.exdates.extend(ical::date_list(&prop, &series_zone)),
            "RDATE" => data.rdates.extend(ical::date_list(&prop, &series_zone)),
            _ => {}
        }
    }
    if let Some(organizer) = &event.organizer {
        data.organizer = organizer.email.clone();
        data.organizer_name = organizer.display_name.clone();
    }
    for person in &event.attendees {
        let attendee = Attendee {
            email: person.email.clone(),
            name: person.display_name.clone(),
            status: response(&person.response_status),
            optional: person.optional,
            is_self: person.is_self,
            organizer: person.organizer,
        };
        if attendee.is_self {
            data.self_status = attendee.status.clone();
        }
        data.attendees.push(attendee);
    }
    data.join_url = if event.hangout_link.starts_with("https://") {
        event.hangout_link.clone()
    } else {
        event
            .conference_data
            .as_ref()
            .and_then(|c| {
                c.entry_points
                    .iter()
                    .find(|p| p.entry_point_type == "video" && p.uri.starts_with("https://"))
            })
            .map(|p| p.uri.clone())
            .unwrap_or_default()
    };
    data.reminders = match &event.reminders {
        Some(r) if !r.use_default => r.overrides.iter().map(|o| o.minutes).collect(),
        _ => default_reminders.to_vec(),
    };
    data.reminders.sort_unstable();
    data.reminders.dedup();
    data.range_end = ical::range_end(&data);
    Some(data)
}

fn parse<T: for<'a> Deserialize<'a>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(body)
        .map_err(|err| Error::Protocol(format!("Google Calendar answer: {err}")))
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Failure {
    error: FailureBody,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct FailureBody {
    message: String,
    errors: Vec<Reason>,
    details: Vec<Reason>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Reason {
    reason: String,
}

/// A `2xx` answer is fine; anything else is what Google says it is.
fn check(reply: &Reply, doing: &str) -> std::result::Result<(), CalendarError> {
    if (200..300).contains(&reply.status) {
        return Ok(());
    }
    let failure: Failure = serde_json::from_slice(&reply.body).unwrap_or_default();
    let reasons: Vec<&str> = failure
        .error
        .errors
        .iter()
        .chain(&failure.error.details)
        .map(|r| r.reason.as_str())
        .collect();
    let any = |names: &[&str]| reasons.iter().any(|r| names.contains(r));
    if reply.status == 403 && any(&["accessNotConfigured", "SERVICE_DISABLED"]) {
        return Err(CalendarError::NotEnabled(
            "the Google Calendar API is not enabled for Katna's Google Cloud project".into(),
        ));
    }
    if reply.status == 401
        || (reply.status == 403
            && any(&[
                "insufficientPermissions",
                "authError",
                "ACCESS_TOKEN_SCOPE_INSUFFICIENT",
            ]))
    {
        return Err(CalendarError::NeedsSignIn(format!(
            "Google Calendar refused access while {doing}"
        )));
    }
    let detail = if failure.error.message.is_empty() {
        format!("status {}", reply.status)
    } else {
        failure.error.message
    };
    Err(CalendarError::Failed(Error::Rejected(format!(
        "Google Calendar, {doing}: {detail}"
    ))))
}

mod write;

#[cfg(test)]
mod tests;
