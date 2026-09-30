// SPDX-License-Identifier: GPL-3.0-or-later

//! Zoho Calendar's REST API (`calendar.zoho.<dc>/api/v1`), with the token
//! of "Sign in with Zoho" (scope [`ZOHO_CALENDAR`]). Zoho's CalDAV answers
//! discovery on port 443 only with a redirect to port 543, which many
//! networks never reach; the API stays on 443.
//!
//! Zoho lists events only by ranges of at most 31 days and gives no sync
//! token, so each calendar is read from [`PAST_DAYS`] ago to [`AHEAD_DAYS`]
//! ahead, each occurrence apart (`byinstance`), and only when its `ctag`
//! changed since the last read. Only events whose etag changed are
//! rewritten. Katna reads Zoho calendars; changing their events comes
//! later, so they are listed as read-only.

use std::{sync::Arc, time::Duration};

use jiff::{Timestamp, ToSpan, Zoned, civil::DateTime, tz::TimeZone};
use katna_core::AccountId;
use katna_dav::ical;
use katna_store::{
    Store,
    calendar::{Attendee, CalendarAccess, CalendarSource, EventData, EventStatus, NewCalendar},
};
use serde::Deserialize;
use serde_json::Value;

use super::{CalendarError, Item, MAX_ANSWER, SyncResult, apply_full, hex_color};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    net::Tls,
    oauth::TokenSource,
};

const TIMEOUT: Duration = Duration::from_secs(2 * 60);

/// How far back events are read.
pub const PAST_DAYS: i64 = 93;
/// How far ahead events are read.
pub const AHEAD_DAYS: i64 = 372;
/// The longest range Zoho lists at once.
const RANGE_DAYS: i64 = 31;

/// One Zoho account's calendars.
#[derive(Clone)]
pub struct ZohoCalendar {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// `https://calendar.zoho.<dc>/api/v1`, or a server under test.
    api: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Calendars {
    calendars: Vec<ListEntry>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct ListEntry {
    uid: String,
    name: String,
    color: String,
    privilege: String,
    timezone: String,
    isdefault: bool,
    /// Changes whenever an event does; a number or a string.
    ctag: Value,
    reminders: Vec<Reminder>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Reminder {
    /// Minutes, negative before the start; a number or a string.
    minutes: Value,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Events {
    events: Vec<Event>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct When {
    timezone: String,
    start: String,
    end: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Person {
    email: String,
    status: String,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
struct Event {
    uid: String,
    title: String,
    description: String,
    location: String,
    isallday: bool,
    dateandtime: When,
    etag: Value,
    rrule: String,
    organizer: String,
    attendees: Vec<Person>,
    reminders: Vec<Reminder>,
    /// `busy` or `free`, where Zoho says.
    transparency: String,
    lastmodifiedtime: String,
    #[serde(rename = "viewEventURL")]
    view_url: String,
}

/// What Zoho says about a calendar, beside what the store keeps.
struct Listed {
    calendar: NewCalendar,
    ctag: String,
    default_reminders: Vec<i64>,
}

impl ZohoCalendar {
    /// The calendars of the Zoho account whose sign-in went to
    /// `accounts_server` (`https://accounts.zoho.in`), or the server under
    /// test in `KATNA_ZOHO_CALENDAR_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls, accounts_server: &str) -> Self {
        let api = http::test_url("KATNA_ZOHO_CALENDAR_URL")
            .unwrap_or_else(|| format!("{}/api/v1", calendar_server(accounts_server)));
        Self::with_api(tokens, tls, &api)
    }

    /// Talks to `api` instead of Zoho, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// A GET with the account's access token, trying once more with a
    /// fresh token when Zoho refuses the one it had.
    async fn get(&self, url: &str) -> Result<Reply> {
        loop {
            let token = format!("Zoho-oauthtoken {}", self.tokens.access_token().await?);
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
        let reply = self.get(&format!("{}/calendars", self.api)).await?;
        check(&reply, "listing the calendars")?;
        let list: Calendars = parse(&reply.body)?;
        Ok(list
            .calendars
            .into_iter()
            .filter(|c| !c.uid.is_empty())
            .map(|entry| Listed {
                calendar: NewCalendar {
                    remote_id: entry.uid,
                    name: entry.name,
                    color: hex_color(&entry.color),
                    // Read-only until Katna sends changes to Zoho.
                    access: if entry.privilege.contains("freebusy") {
                        CalendarAccess::FreeBusy
                    } else {
                        CalendarAccess::Reader
                    },
                    is_primary: entry.isdefault,
                    time_zone: entry.timezone,
                },
                ctag: text(&entry.ctag),
                default_reminders: reminders(&entry.reminders),
            })
            .collect())
    }

    /// Calendar `uid`'s occurrences in the ranges Katna reads.
    async fn events(&self, uid: &str) -> std::result::Result<Vec<Event>, CalendarError> {
        let today = Zoned::now().with_time_zone(TimeZone::UTC).date();
        let mut from = today.checked_sub(PAST_DAYS.days()).unwrap_or(today);
        let last = today.checked_add(AHEAD_DAYS.days()).unwrap_or(today);
        let mut events = Vec::new();
        while from < last {
            let to = from
                .checked_add(RANGE_DAYS.days())
                .unwrap_or(last)
                .min(last);
            let range = format!(
                r#"{{"start":"{}T000000Z","end":"{}T000000Z"}}"#,
                from.strftime("%Y%m%d"),
                to.strftime("%Y%m%d")
            );
            let url = format!(
                "{}/calendars/{}/events?byinstance=true&range={}",
                self.api,
                http::escape(uid),
                http::escape(&range)
            );
            let reply = self.get(&url).await?;
            check(&reply, "listing events")?;
            let listed: Events = parse(&reply.body)?;
            events.extend(listed.events);
            from = to;
        }
        Ok(events)
    }

    /// Brings `account`'s calendars and their events into `store`.
    /// Returns whether anything changed.
    pub async fn sync(&self, store: &mut Store, account: AccountId, address: &str) -> SyncResult {
        let listed = self.calendars().await?;
        let mut changed = false;
        let mut ids = Vec::new();
        {
            let before = store.calendars()?;
            for (position, entry) in listed.iter().enumerate() {
                let id = store.upsert_calendar(
                    Some(account),
                    CalendarSource::Zoho,
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
            match self.sync_calendar(store, id, entry, address).await {
                Ok(c) => changed |= c,
                Err(CalendarError::Failed(err)) => {
                    tracing::warn!(calendar = entry.calendar.remote_id, %err, "calendar not synced");
                }
                Err(err) => return Err(err),
            }
        }
        Ok(changed)
    }

    async fn sync_calendar(
        &self,
        store: &mut Store,
        id: i64,
        entry: &Listed,
        address: &str,
    ) -> SyncResult {
        // The ctag and the day it was read: the ranges move each day.
        let marker = format!(
            "{}@{}",
            entry.ctag,
            Zoned::now().with_time_zone(TimeZone::UTC).date()
        );
        let stored = store.calendar(id)?.and_then(|c| c.sync_token);
        if !entry.ctag.is_empty() && stored.as_deref() == Some(marker.as_str()) {
            return Ok(false);
        }
        let zone = ical::zone(&entry.calendar.time_zone).unwrap_or_else(TimeZone::system);
        let items: Vec<Item> = self
            .events(&entry.calendar.remote_id)
            .await?
            .iter()
            .filter_map(|event| {
                let data = map_event(event, &zone, &entry.default_reminders, address)?;
                Some((data.remote_id.clone(), data.etag.clone(), vec![data]))
            })
            .collect();
        let changed = apply_full(store, id, items)?;
        store.set_calendar_sync_token(id, Some(&marker))?;
        Ok(changed)
    }
}

/// The calendar server beside Zoho's accounts server:
/// `https://accounts.zoho.in` → `https://calendar.zoho.in`.
pub fn calendar_server(accounts_server: &str) -> String {
    accounts_server
        .trim_end_matches('/')
        .replacen("://accounts.", "://calendar.", 1)
}

/// A JSON number or string as text.
fn text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Pop-up and e-mail reminders as minutes before the start.
fn reminders(list: &[Reminder]) -> Vec<i64> {
    let mut minutes: Vec<i64> = list
        .iter()
        .filter_map(|r| text(&r.minutes).trim().parse::<i64>().ok())
        .map(|m| -m)
        .filter(|m| *m >= 0)
        .collect();
    minutes.sort_unstable();
    minutes.dedup();
    minutes
}

/// A start or end as Zoho writes it: `20260930` for a whole day,
/// `20260930T093000+0530` or `20260930T040000Z` with a time.
fn moment(text: &str, zone: &TimeZone) -> Option<(i64, bool)> {
    let text = text.trim();
    if text.len() == 8 {
        let day = jiff::civil::Date::strptime("%Y%m%d", text).ok()?;
        return Some((ical::utc_midnight(day), true));
    }
    if let Some(at) = text.strip_suffix('Z') {
        let at = DateTime::strptime("%Y%m%dT%H%M%S", at).ok()?;
        return Some((
            at.to_zoned(TimeZone::UTC).ok()?.timestamp().as_second(),
            false,
        ));
    }
    if let Ok(at) = Timestamp::strptime("%Y%m%dT%H%M%S%z", text) {
        return Some((at.as_second(), false));
    }
    let at = DateTime::strptime("%Y%m%dT%H%M%S", text).ok()?;
    Some((
        at.to_zoned(zone.clone()).ok()?.timestamp().as_second(),
        false,
    ))
}

fn response(status: &str) -> String {
    match status.to_ascii_uppercase().as_str() {
        "ACCEPTED" => "accepted",
        "TENTATIVE" => "tentative",
        "DECLINED" => "declined",
        "NEEDS-ACTION" | "NEEDS_ACTION" | "NEEDSACTION" => "needs_action",
        _ => "",
    }
    .to_owned()
}

/// Zoho's occurrence as the store keeps it: each occurrence of a series is
/// an event of its own. `address` is the user's. `None` when it has no
/// time.
fn map_event(
    event: &Event,
    calendar_zone: &TimeZone,
    default_reminders: &[i64],
    address: &str,
) -> Option<EventData> {
    let zone_name = ical::zone_name(&event.dateandtime.timezone)
        .or_else(|| calendar_zone.iana_name().map(str::to_owned))
        .unwrap_or_default();
    let zone = TimeZone::get(&zone_name).unwrap_or_else(|_| calendar_zone.clone());
    let (start, day) = moment(&event.dateandtime.start, &zone)?;
    let all_day = event.isallday || day;
    let end = moment(&event.dateandtime.end, &zone)
        .map_or(if all_day { start + 86_400 } else { start }, |(e, _)| e)
        .max(start);
    // An all-day end Zoho gives as the last day, not the day after.
    let end = if all_day && end == start {
        start + 86_400
    } else {
        end
    };
    let base = if event.uid.is_empty() {
        return None;
    } else {
        event.uid.clone()
    };
    // Occurrences share the series' UID.
    let remote_id = if event.rrule.is_empty() {
        base
    } else {
        format!("{base}#{start}")
    };
    let mut data = EventData {
        uid: remote_id.clone(),
        remote_id,
        etag: Some(text(&event.etag)).filter(|e| !e.is_empty()),
        status: EventStatus::Confirmed,
        title: event.title.clone(),
        location: event.location.clone(),
        description: event.description.clone(),
        start,
        end,
        all_day,
        time_zone: if all_day { String::new() } else { zone_name },
        busy: event.transparency != "free",
        organizer: event.organizer.clone(),
        web_link: if event.view_url.starts_with("https://") {
            event.view_url.clone()
        } else {
            String::new()
        },
        updated_at: moment(&event.lastmodifiedtime, &TimeZone::UTC).map_or(0, |(t, _)| t),
        ..EventData::default()
    };
    for person in &event.attendees {
        let is_self = !address.is_empty() && person.email.eq_ignore_ascii_case(address);
        let attendee = Attendee {
            email: person.email.clone(),
            status: response(&person.status),
            is_self,
            organizer: person.email.eq_ignore_ascii_case(&event.organizer),
            ..Attendee::default()
        };
        if is_self {
            data.self_status = attendee.status.clone();
        }
        data.attendees.push(attendee);
    }
    data.reminders = if event.reminders.is_empty() {
        default_reminders.to_vec()
    } else {
        reminders(&event.reminders)
    };
    data.range_end = ical::range_end(&data);
    Some(data)
}

fn parse<T: for<'a> Deserialize<'a>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(body)
        .map_err(|err| Error::Protocol(format!("Zoho Calendar answer: {err}")))
}

/// A `2xx` answer is fine; anything else is what Zoho says it is.
fn check(reply: &Reply, doing: &str) -> std::result::Result<(), CalendarError> {
    if (200..300).contains(&reply.status) {
        return Ok(());
    }
    let body: Value = serde_json::from_slice(&reply.body).unwrap_or_default();
    let message = ["message", "description", "error"]
        .iter()
        .find_map(|key| body.get(key).map(text))
        .filter(|m| !m.is_empty())
        .unwrap_or_else(|| format!("status {}", reply.status));
    if matches!(reply.status, 401 | 403) {
        return Err(CalendarError::NeedsSignIn(format!(
            "Zoho Calendar refused access while {doing}: {message}"
        )));
    }
    Err(CalendarError::Failed(Error::Rejected(format!(
        "Zoho Calendar, {doing}: {message}"
    ))))
}

mod calendars;

#[cfg(test)]
mod tests;
