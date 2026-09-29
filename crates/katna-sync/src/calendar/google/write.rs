// SPDX-License-Identifier: GPL-3.0-or-later

//! Sending changes made in Katna to Google Calendar: `events.insert`
//! (with Katna's own event ID, so a retry can't add it twice),
//! `events.patch` of a series, a single event or one occurrence (its ID
//! is the series' and the occurrence's start, `id_20261012T033000Z`),
//! `events.delete`, and `events.move` between calendars of the account.

use jiff::Timestamp;
use jiff::tz::TimeZone;
use katna_dav::ical;
use katna_store::{
    Store,
    calendar::{Calendar, EventData, EventStatus},
};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{
    CalendarError, Conference, EVENT_COLORS, GoogleCalendar, MAX_ANSWER, TIMEOUT, check, parse,
};
use crate::{
    Result,
    autoconfig::http::{self, Reply},
    calendar::edit::{Step, google_instance_id, request_id},
};

/// What Google says about an event it took.
#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct Saved {
    id: String,
    etag: Option<String>,
    hangout_link: String,
    conference_data: Option<Conference>,
}

impl Saved {
    /// The link to the event's video call, if it has one.
    fn join_url(&self) -> Option<&str> {
        if self.hangout_link.starts_with("https://") {
            return Some(&self.hangout_link);
        }
        self.conference_data
            .as_ref()?
            .entry_points
            .iter()
            .find(|p| p.entry_point_type == "video" && p.uri.starts_with("https://"))
            .map(|p| p.uri.as_str())
    }
}

fn date_time(seconds: i64, zone: &str) -> Value {
    let at = Timestamp::from_second(seconds).unwrap_or(Timestamp::UNIX_EPOCH);
    match TimeZone::get(zone).ok().filter(|_| !zone.is_empty()) {
        Some(tz) => json!({
            "dateTime": at.to_zoned(tz).strftime("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            "timeZone": zone,
        }),
        None => json!({
            "dateTime": at.to_zoned(TimeZone::UTC).strftime("%Y-%m-%dT%H:%M:%SZ").to_string(),
            "timeZone": "UTC",
        }),
    }
}

/// A start or end as Google takes it.
fn time(seconds: i64, event: &EventData) -> Value {
    if event.all_day {
        let day = Timestamp::from_second(seconds)
            .unwrap_or(Timestamp::UNIX_EPOCH)
            .to_zoned(TimeZone::UTC)
            .date();
        json!({ "date": day.to_string() })
    } else {
        date_time(seconds, &event.time_zone)
    }
}

fn response(status: &str) -> &'static str {
    match status {
        "accepted" => "accepted",
        "tentative" => "tentative",
        "declined" => "declined",
        _ => "needsAction",
    }
}

/// `event` as Google's event resource: what Katna changes of it.
pub(crate) fn event_body(event: &EventData) -> Value {
    let mut body = json!({
        "summary": event.title,
        "description": event.description,
        "location": event.location,
        "start": time(event.start, event),
        "end": time(event.end, event),
        "transparency": if event.busy { "opaque" } else { "transparent" },
        "status": match event.status {
            EventStatus::Confirmed => "confirmed",
            EventStatus::Tentative => "tentative",
            EventStatus::Cancelled => "cancelled",
        },
        "reminders": {
            "useDefault": false,
            "overrides": event
                .reminders
                .iter()
                .map(|m| json!({ "method": "popup", "minutes": m }))
                .collect::<Vec<_>>(),
        },
    });
    if event.recurrence_id.is_none() {
        body["recurrence"] = json!(ical::recurrence_lines(event));
    }
    if event.color.is_empty() {
        body["colorId"] = Value::Null;
    } else if let Some(n) = EVENT_COLORS.iter().position(|c| *c == event.color) {
        body["colorId"] = json!((n + 1).to_string());
    }
    if !event.attendees.is_empty() {
        body["attendees"] = event
            .attendees
            .iter()
            .map(|a| {
                let mut person = json!({
                    "email": a.email,
                    "responseStatus": response(&a.status),
                });
                if !a.name.is_empty() {
                    person["displayName"] = json!(a.name);
                }
                if a.optional {
                    person["optional"] = json!(true);
                }
                person
            })
            .collect();
    }
    body
}

/// A 404 or 410: Google no longer has it, which is what a delete wants.
fn gone(reply: &Reply) -> bool {
    matches!(reply.status, 404 | 410)
}

impl GoogleCalendar {
    /// Sends `method` to `url` with a JSON `body`, trying once more with
    /// a fresh token when Google refuses the one it had.
    async fn send(&self, method: &str, url: &str, body: Option<&Value>) -> Result<Reply> {
        let bytes = body.map(|b| b.to_string().into_bytes());
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let headers = [("Authorization", token.as_str())];
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
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    fn events_url(&self, calendar: &str) -> String {
        format!(
            "{}/calendar/v3/calendars/{}/events",
            self.api,
            http::escape(calendar)
        )
    }

    /// Adds or changes event row `row` in `calendar`, with a Google Meet
    /// call when `add_call`, and keeps Google's ID and etag of it (and the
    /// call's link).
    async fn write_row(
        &self,
        store: &mut Store,
        calendar: &str,
        row: i64,
        add_call: bool,
    ) -> std::result::Result<(), CalendarError> {
        let Some(event) = store.event(row)? else {
            return Ok(());
        };
        let data = &event.data;
        let mut query = Vec::new();
        if !data.attendees.is_empty() {
            // Google invites the attendees, or tells them what changed.
            query.push("sendUpdates=all");
        }
        let mut body = event_body(data);
        if add_call {
            query.push("conferenceDataVersion=1");
            body["conferenceData"] = json!({
                "createRequest": {
                    "requestId": request_id(),
                    "conferenceSolutionKey": { "type": "hangoutsMeet" },
                },
            });
        }
        let query = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let base = self.events_url(calendar);
        let url = format!("{base}/{}{query}", http::escape(&data.remote_id));
        let reply = if data.recurrence_id.is_none() && data.etag.is_none() {
            let mut new = body.clone();
            new["id"] = json!(data.remote_id);
            new["iCalUID"] = json!(data.uid);
            let reply = self
                .send("POST", &format!("{base}{query}"), Some(&new))
                .await?;
            if reply.status == 409 {
                // Google has it already: an earlier try went through.
                self.send("PATCH", &url, Some(&body)).await?
            } else {
                reply
            }
        } else {
            self.send("PATCH", &url, Some(&body)).await?
        };
        check(&reply, "saving an event")?;
        let saved: Saved = parse(&reply.body)?;
        let id = if saved.id.is_empty() {
            data.remote_id.clone()
        } else {
            saved.id.clone()
        };
        store.set_event_remote(row, &id, saved.etag.as_deref())?;
        if let Some(url) = saved.join_url() {
            store.set_event_join_url(row, url)?;
        }
        Ok(())
    }

    /// Deletes Google's event `id` from `calendar`.
    async fn delete_remote(
        &self,
        calendar: &str,
        id: &str,
    ) -> std::result::Result<(), CalendarError> {
        let url = format!("{}/{}", self.events_url(calendar), http::escape(id));
        let reply = self.send("DELETE", &url, None).await?;
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
        let cal = calendar.remote_id.as_str();
        match step {
            Step::Write { row, add_call, .. } => self.write_row(store, cal, *row, *add_call).await,
            Step::Cancel {
                series, occurrence, ..
            } => {
                let Some(series) = store.event(*series)? else {
                    return Ok(());
                };
                let id =
                    google_instance_id(&series.data.remote_id, *occurrence, series.data.all_day);
                self.delete_remote(cal, &id).await
            }
            Step::Restore {
                series, occurrence, ..
            } => {
                // The series without the start it skipped…
                self.write_row(store, cal, *series, false).await?;
                let Some(series) = store.event(*series)? else {
                    return Ok(());
                };
                // …and the occurrence, where Google keeps it cancelled.
                let id =
                    google_instance_id(&series.data.remote_id, *occurrence, series.data.all_day);
                let url = format!("{}/{}", self.events_url(cal), http::escape(&id));
                let reply = self
                    .send("PATCH", &url, Some(&json!({ "status": "confirmed" })))
                    .await?;
                if !gone(&reply) && reply.status != 400 {
                    check(&reply, "bringing back an occurrence")?;
                }
                Ok(())
            }
            Step::Delete { row, .. } => {
                let Some(event) = store.event(*row)? else {
                    return Ok(());
                };
                if event.data.etag.is_none() && event.data.recurrence_id.is_none() {
                    // Google never had it.
                    return Ok(());
                }
                self.delete_remote(cal, &event.data.remote_id).await
            }
            Step::DeleteRemote { remote_id, .. } => self.delete_remote(cal, remote_id).await,
            Step::GoogleMove { row, to, .. } => {
                let Some(event) = store.event(*row)? else {
                    return Ok(());
                };
                let Some(dest) = store.calendar(*to)? else {
                    return Ok(());
                };
                let url = format!(
                    "{}/{}/move?destination={}",
                    self.events_url(cal),
                    http::escape(&event.data.remote_id),
                    http::escape(&dest.remote_id)
                );
                let reply = self.send("POST", &url, None).await?;
                check(&reply, "moving an event")?;
                self.write_row(store, &dest.remote_id, *row, false).await
            }
            Step::Respond { row, .. } => {
                let Some(event) = store.event(*row)? else {
                    return Ok(());
                };
                let data = &event.data;
                if data.attendees.is_empty() {
                    return Ok(());
                }
                // The organizer hears the answer.
                let url = format!(
                    "{}/{}?sendUpdates=all",
                    self.events_url(cal),
                    http::escape(&data.remote_id)
                );
                let body = json!({ "attendees": event_body(data)["attendees"] });
                let reply = self.send("PATCH", &url, Some(&body)).await?;
                check(&reply, "answering an invitation")?;
                let saved: Saved = parse(&reply.body)?;
                if !saved.id.is_empty() {
                    store.set_event_remote(*row, &saved.id, saved.etag.as_deref())?;
                }
                Ok(())
            }
        }
    }
}
