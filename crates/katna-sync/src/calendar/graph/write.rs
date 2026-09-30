// SPDX-License-Identifier: GPL-3.0-or-later

//! Sending changes made in Katna to Outlook through Microsoft Graph:
//! `POST /me/calendars/{id}/events`, `PATCH` and `DELETE /me/events/{id}`.
//! One occurrence of a series is found through the series' `instances`
//! by the start it had. A series' `RRULE` becomes Graph's `recurrence` for
//! the rules Katna's editor makes (daily, weekly on some days, monthly on
//! a day or the nth weekday, yearly); times go in the event's zone by its
//! Windows name, or in UTC. Graph can't bring back a deleted occurrence.

use jiff::Timestamp;
use jiff::civil::{Date, Weekday};
use jiff::tz::TimeZone;
use katna_dav::ical;
use katna_store::{
    Store,
    calendar::{Calendar, EventData, EventKind},
};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{
    CalendarError, GraphCalendar, GraphEvent, MAX_ANSWER, OnlineMeeting, PREFER, TIMEOUT, check,
    parse,
};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    calendar::edit::{NEW_PREFIX, Step},
    oauth::MICROSOFT_CALENDARS,
};

/// What Graph says about an event it took.
#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct Saved {
    id: String,
    #[serde(rename = "@odata.etag")]
    etag: Option<String>,
    online_meeting: Option<OnlineMeeting>,
}

/// What Graph says about a calendar, for a Teams call.
#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct Providers {
    allowed_online_meeting_providers: Vec<String>,
}

fn at(seconds: i64) -> Timestamp {
    Timestamp::from_second(seconds).unwrap_or(Timestamp::UNIX_EPOCH)
}

/// The zone `event`'s times go in: its own, when Graph knows it by a
/// Windows name, else UTC. Whole days are in UTC, as they are stored.
fn zone(event: &EventData) -> (TimeZone, &'static str) {
    if !event.all_day
        && let Some(windows) = ical::windows_zone(&event.time_zone)
        && let Ok(tz) = TimeZone::get(&event.time_zone)
    {
        return (tz, windows);
    }
    (TimeZone::UTC, "UTC")
}

fn time(seconds: i64, event: &EventData) -> Value {
    let (tz, name) = zone(event);
    json!({
        "dateTime": at(seconds).to_zoned(tz).strftime("%Y-%m-%dT%H:%M:%S").to_string(),
        "timeZone": name,
    })
}

fn weekday(code: &str) -> Option<&'static str> {
    Some(match code {
        "MO" => "monday",
        "TU" => "tuesday",
        "WE" => "wednesday",
        "TH" => "thursday",
        "FR" => "friday",
        "SA" => "saturday",
        "SU" => "sunday",
        _ => return None,
    })
}

fn index(n: i32) -> Option<&'static str> {
    Some(match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        -1 => "last",
        _ => return None,
    })
}

/// A `BYDAY` entry: its position (0 when none) and Graph's day.
fn by_day(entry: &str) -> Option<(i32, &'static str)> {
    let entry = entry.trim();
    let split = entry.len().checked_sub(2)?;
    let (n, day) = entry.split_at(split);
    let n = if n.is_empty() { 0 } else { n.parse().ok()? };
    Some((n, weekday(&day.to_ascii_uppercase())?))
}

/// `event`'s `RRULE` as Graph's `recurrence`; an error for a rule Graph
/// can't hold.
pub(crate) fn recurrence(event: &EventData) -> std::result::Result<Value, CalendarError> {
    let refuse = || {
        CalendarError::Failed(Error::Rejected(format!(
            "Outlook can't repeat an event by {:?}",
            event.rrule
        )))
    };
    let (tz, zone_name) = zone(event);
    let first = at(event.start).to_zoned(tz.clone());
    let mut freq = "";
    let mut interval = 1u32;
    let mut days: Vec<(i32, &str)> = Vec::new();
    let mut month_day: Option<i32> = None;
    let mut month: Option<i32> = None;
    let mut set_pos: Option<i32> = None;
    let mut count: Option<u32> = None;
    let mut until: Option<Date> = None;
    let mut week_start = "monday";
    for part in event.rrule.split(';').filter(|p| !p.is_empty()) {
        let (key, value) = part.split_once('=').ok_or_else(refuse)?;
        let value = value.trim();
        match key.trim().to_ascii_uppercase().as_str() {
            "FREQ" => {
                freq = if value.eq_ignore_ascii_case("DAILY") {
                    "daily"
                } else if value.eq_ignore_ascii_case("WEEKLY") {
                    "weekly"
                } else if value.eq_ignore_ascii_case("MONTHLY") {
                    "monthly"
                } else if value.eq_ignore_ascii_case("YEARLY") {
                    "yearly"
                } else {
                    return Err(refuse());
                }
            }
            "INTERVAL" => interval = value.parse().map_err(|_| refuse())?,
            "BYDAY" => {
                for entry in value.split(',') {
                    days.push(by_day(entry).ok_or_else(refuse)?);
                }
            }
            "BYMONTHDAY" if !value.contains(',') => {
                month_day = Some(value.parse().map_err(|_| refuse())?);
            }
            "BYMONTH" if !value.contains(',') => {
                month = Some(value.parse().map_err(|_| refuse())?);
            }
            "BYSETPOS" if !value.contains(',') => {
                set_pos = Some(value.parse().map_err(|_| refuse())?);
            }
            "COUNT" => count = Some(value.parse().map_err(|_| refuse())?),
            "UNTIL" => {
                let (end, _) = ical::parse_date_time(value, &tz).ok_or_else(refuse)?;
                until = Some(at(end).to_zoned(tz.clone()).date());
            }
            "WKST" => week_start = weekday(&value.to_ascii_uppercase()).ok_or_else(refuse)?,
            _ => return Err(refuse()),
        }
    }
    if month_day.is_some_and(|d| d < 1) {
        return Err(refuse());
    }
    let plain_days = || -> std::result::Result<Vec<&str>, CalendarError> {
        if days.iter().any(|(n, _)| *n != 0) {
            return Err(refuse());
        }
        Ok(days.iter().map(|(_, d)| *d).collect())
    };
    // The nth weekday: `BYDAY=2MO`, or `BYDAY=MO,TU;BYSETPOS=2`.
    let relative = || -> Option<(&'static str, Vec<&str>)> {
        match (days.as_slice(), set_pos) {
            ([(n, day)], None) if *n != 0 => Some((index(*n)?, vec![*day])),
            (many, Some(pos)) if !many.is_empty() && many.iter().all(|(n, _)| *n == 0) => {
                Some((index(pos)?, many.iter().map(|(_, d)| *d).collect()))
            }
            _ => None,
        }
    };
    let start_day = Some(match first.weekday() {
        Weekday::Monday => "monday",
        Weekday::Tuesday => "tuesday",
        Weekday::Wednesday => "wednesday",
        Weekday::Thursday => "thursday",
        Weekday::Friday => "friday",
        Weekday::Saturday => "saturday",
        Weekday::Sunday => "sunday",
    });
    let pattern = match freq {
        "daily" if days.is_empty() => json!({ "type": "daily", "interval": interval }),
        // Every weekday.
        "daily" | "weekly" => {
            let mut list = plain_days()?;
            if list.is_empty() {
                list.extend(start_day);
            }
            json!({
                "type": "weekly",
                "interval": interval,
                "daysOfWeek": list,
                "firstDayOfWeek": week_start,
            })
        }
        "monthly" if !days.is_empty() => {
            let (index, list) = relative().ok_or_else(refuse)?;
            json!({
                "type": "relativeMonthly",
                "interval": interval,
                "daysOfWeek": list,
                "index": index,
            })
        }
        "monthly" => json!({
            "type": "absoluteMonthly",
            "interval": interval,
            "dayOfMonth": month_day.unwrap_or(i32::from(first.day())),
        }),
        "yearly" if !days.is_empty() => {
            let (index, list) = relative().ok_or_else(refuse)?;
            json!({
                "type": "relativeYearly",
                "interval": interval,
                "month": month.unwrap_or(i32::from(first.month())),
                "daysOfWeek": list,
                "index": index,
            })
        }
        "yearly" => json!({
            "type": "absoluteYearly",
            "interval": interval,
            "month": month.unwrap_or(i32::from(first.month())),
            "dayOfMonth": month_day.unwrap_or(i32::from(first.day())),
        }),
        _ => return Err(refuse()),
    };
    let mut range = json!({
        "type": "noEnd",
        "startDate": first.date().to_string(),
        "recurrenceTimeZone": zone_name,
    });
    if let Some(end) = until {
        range["type"] = json!("endDate");
        range["endDate"] = json!(end.to_string());
    } else if let Some(count) = count {
        range["type"] = json!("numbered");
        range["numberOfOccurrences"] = json!(count);
    }
    Ok(json!({ "pattern": pattern, "range": range }))
}

/// `event` as Graph's event resource: what Katna changes of it. A series
/// has its `recurrence`; `clear` sets none on an event that no longer
/// repeats.
pub(crate) fn event_body(
    event: &EventData,
    clear: bool,
) -> std::result::Result<Value, CalendarError> {
    let mut body = json!({
        "subject": event.title,
        "body": { "contentType": "text", "content": event.description },
        "location": { "displayName": event.location },
        "start": time(event.start, event),
        "end": time(event.end, event),
        "isAllDay": event.all_day,
        "showAs": if event.kind == EventKind::OutOfOffice {
            "oof"
        } else if event.kind == EventKind::WorkingLocation {
            "workingElsewhere"
        } else if event.busy {
            "busy"
        } else {
            "free"
        },
        "isReminderOn": !event.reminders.is_empty(),
    });
    if let Some(first) = event.reminders.first() {
        body["reminderMinutesBeforeStart"] = json!(first);
    }
    if event.recurrence_id.is_none() {
        if !event.rrule.is_empty() {
            body["recurrence"] = recurrence(event)?;
        } else if clear {
            body["recurrence"] = Value::Null;
        }
    }
    if !event.attendees.is_empty() {
        body["attendees"] = event
            .attendees
            .iter()
            .map(|a| {
                json!({
                    "emailAddress": { "address": a.email, "name": a.name },
                    "type": if a.optional { "optional" } else { "required" },
                })
            })
            .collect();
    }
    Ok(body)
}

fn gone(reply: &Reply) -> bool {
    matches!(reply.status, 404 | 410)
}

impl GraphCalendar {
    /// Sends `method` to `url` with a JSON `body`, trying once more with
    /// a fresh token when Graph refuses the one it had.
    pub(super) async fn send(
        &self,
        method: &str,
        url: &str,
        body: Option<&Value>,
    ) -> Result<Reply> {
        let bytes = body.map(|b| b.to_string().into_bytes());
        loop {
            let token = format!(
                "Bearer {}",
                self.tokens.access_token_for(MICROSOFT_CALENDARS).await?
            );
            let headers = [("Authorization", token.as_str()), ("Prefer", PREFER)];
            let reply = http::exchange_limited(
                method,
                url,
                &headers,
                bytes.as_deref().map(|b| ("application/json", b)),
                None,
                &self.tls,
                TIMEOUT,
                MAX_ANSWER,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token_for(MICROSOFT_CALENDARS) {
                continue;
            }
            return Ok(reply);
        }
    }

    fn event_url(&self, id: &str) -> String {
        format!("{}/me/events/{}", self.api, http::escape(id))
    }

    /// Graph's ID of the occurrence of series `series` (Graph's ID) that
    /// starts, or started before it changed, at `original`.
    async fn instance(
        &self,
        series: &str,
        original: i64,
        all_day: bool,
    ) -> std::result::Result<String, CalendarError> {
        let stamp = |s: i64| {
            at(s)
                .to_zoned(TimeZone::UTC)
                .strftime("%Y-%m-%dT%H:%M:%SZ")
                .to_string()
        };
        let url = format!(
            "{}/instances?startDateTime={}&endDateTime={}&$select=id,originalStart,start,isAllDay",
            self.event_url(series),
            http::escape(&stamp(original - 86_400)),
            http::escape(&stamp(original + 2 * 86_400)),
        );
        let found: Vec<GraphEvent> = self.all(url, "finding an occurrence").await?;
        let day = at(original).to_zoned(TimeZone::UTC).date().to_string();
        let matches = |event: &&GraphEvent| {
            let Some(started) = &event.original_start else {
                return false;
            };
            if all_day {
                started.get(..10) == Some(day.as_str())
            } else {
                started.parse::<Timestamp>().ok().map(|t| t.as_second()) == Some(original)
            }
        };
        found
            .iter()
            .find(matches)
            .map(|e| e.id.clone())
            .ok_or_else(|| {
                CalendarError::Failed(Error::Rejected(
                    "Outlook has no such occurrence of the series".into(),
                ))
            })
    }

    /// Graph's ID of changed occurrence `data` of a series in `calendar`:
    /// its own, or found among the series' instances.
    async fn occurrence_id(
        &self,
        store: &mut Store,
        calendar: &Calendar,
        data: &EventData,
    ) -> std::result::Result<String, CalendarError> {
        let Some(original) = data.recurrence_id else {
            return Ok(data.remote_id.clone());
        };
        if !data.remote_id.starts_with(NEW_PREFIX) {
            return Ok(data.remote_id.clone());
        }
        let series = store
            .event_series(calendar.id, &data.uid)?
            .filter(|s| !s.data.remote_id.starts_with(NEW_PREFIX))
            .ok_or_else(|| {
                CalendarError::Failed(Error::Rejected("the series is not in Outlook yet".into()))
            })?;
        self.instance(&series.data.remote_id, original, series.data.all_day)
            .await
    }

    /// Whether calendar `calendar` (Graph's ID) takes Teams calls.
    async fn takes_teams(&self, calendar: &str) -> std::result::Result<bool, CalendarError> {
        let url = format!(
            "{}/me/calendars/{}?$select=allowedOnlineMeetingProviders",
            self.api,
            http::escape(calendar)
        );
        let reply = self.send("GET", &url, None).await?;
        check(&reply, "reading the calendar")?;
        let providers: Providers = parse(&reply.body)?;
        Ok(providers
            .allowed_online_meeting_providers
            .iter()
            .any(|p| p == "teamsForBusiness"))
    }

    /// Adds or changes event row `row` in calendar `calendar`, with an
    /// online meeting when `add_call`, and keeps Graph's ID and etag of it
    /// (and the meeting's link).
    async fn write_row(
        &self,
        store: &mut Store,
        calendar: &Calendar,
        row: i64,
        add_call: bool,
    ) -> std::result::Result<(), CalendarError> {
        let Some(event) = store.event(row)? else {
            return Ok(());
        };
        let data = &event.data;
        let new = data.remote_id.starts_with(NEW_PREFIX);
        let meeting = |body: &mut Value, teams: bool| {
            if add_call {
                body["isOnlineMeeting"] = json!(true);
                if teams {
                    body["onlineMeetingProvider"] = json!("teamsForBusiness");
                }
            }
        };
        let teams = add_call && self.takes_teams(&calendar.remote_id).await?;
        let reply = match data.recurrence_id {
            Some(_) => {
                let id = self.occurrence_id(store, calendar, data).await?;
                let mut body = event_body(data, false)?;
                meeting(&mut body, teams);
                self.send("PATCH", &self.event_url(&id), Some(&body))
                    .await?
            }
            None if new => {
                let url = format!(
                    "{}/me/calendars/{}/events",
                    self.api,
                    http::escape(&calendar.remote_id)
                );
                let mut body = event_body(data, false)?;
                meeting(&mut body, teams);
                self.send("POST", &url, Some(&body)).await?
            }
            None => {
                let mut body = event_body(data, true)?;
                meeting(&mut body, teams);
                self.send("PATCH", &self.event_url(&data.remote_id), Some(&body))
                    .await?
            }
        };
        check(&reply, "saving an event")?;
        let saved: Saved = parse(&reply.body)?;
        if saved.id.is_empty() {
            return Err(CalendarError::Failed(Error::Protocol(
                "Outlook saved the event without an ID".into(),
            )));
        }
        store.set_event_remote(row, &saved.id, saved.etag.as_deref())?;
        if let Some(url) = saved
            .online_meeting
            .as_ref()
            .map(|m| m.join_url.as_str())
            .filter(|u| u.starts_with("https://"))
        {
            store.set_event_join_url(row, url)?;
        }
        Ok(())
    }

    async fn delete_remote(&self, id: &str) -> std::result::Result<(), CalendarError> {
        if id.starts_with(NEW_PREFIX) {
            // Outlook never had it.
            return Ok(());
        }
        let reply = self.send("DELETE", &self.event_url(id), None).await?;
        if !gone(&reply) {
            check(&reply, "deleting an event")?;
        }
        Ok(())
    }

    /// Sends `step` for `calendar`, one of this account's.
    pub(crate) async fn push(
        &self,
        store: &mut Store,
        calendar: &Calendar,
        step: &Step,
    ) -> std::result::Result<(), CalendarError> {
        match step {
            Step::Write { row, add_call, .. } => {
                self.write_row(store, calendar, *row, *add_call).await
            }
            Step::Cancel {
                series, occurrence, ..
            } => {
                let Some(series) = store.event(*series)? else {
                    return Ok(());
                };
                let id = self
                    .instance(&series.data.remote_id, *occurrence, series.data.all_day)
                    .await?;
                self.delete_remote(&id).await
            }
            Step::Restore { .. } => Err(CalendarError::Failed(Error::Rejected(
                "Outlook can't bring back a deleted occurrence".into(),
            ))),
            Step::Delete { row, .. } => {
                let Some(event) = store.event(*row)? else {
                    return Ok(());
                };
                self.delete_remote(&event.data.remote_id).await
            }
            Step::DeleteRemote { remote_id, .. } => self.delete_remote(remote_id).await,
            Step::GoogleMove { .. } => Err(CalendarError::Failed(Error::Protocol(
                "a Google move sent to Outlook".into(),
            ))),
            Step::Respond { row, status, .. } => {
                let Some(event) = store.event(*row)? else {
                    return Ok(());
                };
                let id = self.occurrence_id(store, calendar, &event.data).await?;
                if id.starts_with(NEW_PREFIX) {
                    return Err(CalendarError::Failed(Error::Rejected(
                        "the event is not in Outlook yet".into(),
                    )));
                }
                let action = match status.as_str() {
                    "accepted" => "accept",
                    "tentative" => "tentativelyAccept",
                    _ => "decline",
                };
                // Outlook tells the organizer.
                let url = format!("{}/{action}", self.event_url(&id));
                let body = json!({ "sendResponse": true });
                let reply = self.send("POST", &url, Some(&body)).await?;
                check(&reply, "answering an invitation")?;
                if id != event.data.remote_id {
                    store.set_event_remote(*row, &id, None)?;
                }
                Ok(())
            }
        }
    }
}
