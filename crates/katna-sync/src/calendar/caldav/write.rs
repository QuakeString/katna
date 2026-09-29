// SPDX-License-Identifier: GPL-3.0-or-later

//! Sending changes made in Katna to a CalDAV server: each change rewrites
//! the event's whole resource (a series and its changed occurrences, as
//! [`katna_dav::ical::write_calendar`] writes it) with a `PUT`, only if the
//! server still has the version Katna read (`If-Match`) or, for a new
//! one, has none (`If-None-Match: *`); a deleted event's resource goes
//! with a `DELETE`. The server's new `ETag` is kept.

use katna_dav::ical;
use katna_store::{
    Store,
    calendar::{Calendar, EventData, Pending},
};

use super::{CalDav, CalendarError, MAX_ANSWER, TIMEOUT, origin};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    calendar::edit::Step,
};

impl CalDav {
    /// The URL of resource `href` (a path) of `calendar`: on the host the
    /// calendar was found on.
    fn resource_url(&self, calendar: &Calendar, href: &str) -> String {
        let found = self.urls.lock().unwrap().get(&calendar.remote_id).cloned();
        let base = found.as_deref().map_or(self.origin.as_str(), origin);
        format!("{base}{href}")
    }

    /// Sends `method` to `url` with the account's password and `headers`,
    /// and a body of type `kind`.
    async fn send_body(
        &self,
        method: &str,
        url: &str,
        headers: &[(&str, &str)],
        body: Option<(&str, &[u8])>,
    ) -> Result<Reply> {
        if !self.trusted(url) {
            return Err(Error::Protocol(format!("CalDAV led elsewhere: {url}")));
        }
        let mut all = vec![("Authorization", self.authorization.as_str())];
        all.extend_from_slice(headers);
        http::exchange_limited(
            method, url, &all, body, None, &self.tls, TIMEOUT, MAX_ANSWER,
        )
        .await
    }

    /// What the server's answer to a write means.
    fn answered(reply: &Reply, doing: &str) -> std::result::Result<(), CalendarError> {
        match reply.status {
            200..=299 => Ok(()),
            401 | 403 => Err(CalendarError::Failed(Error::Rejected(
                "the CalDAV server refused the password".into(),
            ))),
            412 => Err(CalendarError::Failed(Error::Rejected(format!(
                "the event changed on the CalDAV server while {doing}"
            )))),
            status => Err(CalendarError::Failed(Error::Rejected(format!(
                "CalDAV server answered {status} while {doing}"
            )))),
        }
    }

    /// Deletes resource `href` of `calendar`, if it still is at `etag`.
    async fn delete_resource(
        &self,
        calendar: &Calendar,
        href: &str,
        etag: Option<&str>,
    ) -> std::result::Result<(), CalendarError> {
        let url = self.resource_url(calendar, href);
        let headers: Vec<(&str, &str)> = etag.map(|e| ("If-Match", e)).into_iter().collect();
        let reply = self.send_body("DELETE", &url, &headers, None).await?;
        if reply.status == 404 {
            return Ok(());
        }
        Self::answered(&reply, "deleting an event")
    }

    /// Writes resource `href` of `calendar` as the store has it now.
    async fn write_resource(
        &self,
        store: &mut Store,
        calendar: &Calendar,
        href: &str,
    ) -> std::result::Result<(), CalendarError> {
        let rows = store.resource_events(calendar.id, href)?;
        let etag = rows.iter().find_map(|(e, _)| e.data.etag.clone());
        let events: Vec<EventData> = rows
            .into_iter()
            .filter(|(_, pending)| *pending != Pending::Delete)
            .map(|(e, _)| e.data)
            .collect();
        if events.is_empty() {
            return self.delete_resource(calendar, href, etag.as_deref()).await;
        }
        let text = ical::write_calendar(&events, jiff::Timestamp::now().as_second());
        let url = self.resource_url(calendar, href);
        let condition = match &etag {
            Some(etag) => ("If-Match", etag.as_str()),
            None => ("If-None-Match", "*"),
        };
        let reply = self
            .send_body(
                "PUT",
                &url,
                &[condition],
                Some(("text/calendar; charset=utf-8", text.as_bytes())),
            )
            .await?;
        Self::answered(&reply, "saving an event")?;
        // Without an ETag the next sync downloads what the server made of it.
        store.set_resource_etag(calendar.id, href, reply.etag.as_deref())?;
        Ok(())
    }

    /// Sends `step` for `calendar`, one of this account's.
    pub(crate) async fn push(
        &self,
        store: &mut Store,
        calendar: &Calendar,
        step: &Step,
    ) -> std::result::Result<(), CalendarError> {
        let row = match step {
            Step::Write { row, .. } | Step::Delete { row, .. } | Step::Respond { row, .. } => *row,
            Step::Cancel { series, .. } | Step::Restore { series, .. } => *series,
            Step::DeleteRemote {
                remote_id, etag, ..
            } => {
                return self
                    .delete_resource(calendar, remote_id, etag.as_deref())
                    .await;
            }
            Step::GoogleMove { .. } => {
                return Err(CalendarError::Failed(Error::Protocol(
                    "a Google move sent to a CalDAV server".into(),
                )));
            }
        };
        let Some(event) = store.event(row)? else {
            return Ok(());
        };
        self.write_resource(store, calendar, &event.data.remote_id)
            .await
    }
}
