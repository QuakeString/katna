// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars themselves, as changed in Katna's side panel, with Zoho
//! Calendar's `calendars` resource: a `POST` for a new one, a `PUT` of its
//! name or colour, and a `DELETE`. Zoho lists every calendar read-only in
//! Katna for now, so before changing one Katna asks Zoho whether it is
//! the person's own; one shared with them is left to Zoho's website.

use katna_store::calendar::{Calendar, CalendarAccess, NewCalendar};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{CalendarError, Calendars, TIMEOUT, ZohoCalendar, check, hex_color, parse};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
};

type Done<T> = std::result::Result<T, CalendarError>;

#[derive(Deserialize, Default)]
#[serde(default)]
struct Made {
    uid: String,
    color: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Answer {
    calendars: Vec<Made>,
}

impl ZohoCalendar {
    /// Sends `method` to `url`, trying once more with a fresh token when
    /// Zoho refuses the one it had.
    async fn send(&self, method: &str, url: &str) -> Result<Reply> {
        loop {
            let token = format!("Zoho-oauthtoken {}", self.tokens.access_token().await?);
            let headers = [("Authorization", token.as_str())];
            let reply = http::exchange_limited(
                method,
                url,
                &headers,
                None,
                None,
                &self.tls,
                TIMEOUT,
                super::MAX_ANSWER,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    fn calendar_url(&self, uid: &str, data: Option<&Value>) -> String {
        let mut url = format!("{}/calendars/{}", self.api, http::escape(uid));
        if let Some(data) = data {
            url.push_str(&format!(
                "?calendarData={}",
                http::escape(&data.to_string())
            ));
        }
        url
    }

    /// Refuses a calendar that isn't the person's own.
    async fn own(&self, calendar: &Calendar, doing: &str) -> Done<()> {
        let reply = self.get(&format!("{}/calendars", self.api)).await?;
        check(&reply, "listing the calendars")?;
        let list: Calendars = parse(&reply.body)?;
        let own = list
            .calendars
            .iter()
            .any(|c| c.uid == calendar.remote_id && c.privilege == "owner");
        if own {
            return Ok(());
        }
        Err(CalendarError::Failed(Error::Rejected(format!(
            "Zoho allows {doing} a calendar shared with you only on its website"
        ))))
    }

    /// Makes a calendar named `name` in `color`.
    pub(crate) async fn add_calendar(&self, name: &str, color: &str) -> Done<NewCalendar> {
        let mut data = json!({ "name": name });
        if !color.is_empty() {
            data["color"] = json!(color);
        }
        let url = format!(
            "{}/calendars?calendarData={}",
            self.api,
            http::escape(&data.to_string())
        );
        let reply = self.send("POST", &url).await?;
        check(&reply, "adding a calendar")?;
        let answer: Answer = parse(&reply.body)?;
        let made = answer.calendars.into_iter().next().unwrap_or_default();
        if made.uid.is_empty() {
            return Err(CalendarError::Failed(Error::Protocol(
                "Zoho made a calendar without an ID".into(),
            )));
        }
        let color = match hex_color(&made.color) {
            hex if hex.is_empty() => color.to_owned(),
            hex => hex,
        };
        Ok(NewCalendar {
            remote_id: made.uid,
            name: name.to_owned(),
            color,
            // Read-only here until Katna sends event changes to Zoho.
            access: CalendarAccess::Reader,
            is_primary: false,
            time_zone: String::new(),
        })
    }

    async fn update(&self, calendar: &Calendar, data: Value, doing: &str) -> Done<()> {
        self.own(calendar, doing).await?;
        let url = self.calendar_url(&calendar.remote_id, Some(&data));
        let reply = self.send("PUT", &url).await?;
        check(&reply, doing)
    }

    pub(crate) async fn rename_calendar(&self, calendar: &Calendar, name: &str) -> Done<()> {
        let data = json!({ "name": name, "color": calendar.color });
        self.update(calendar, data, "renaming").await
    }

    /// Gives `calendar` colour `color`; returns it as stored.
    pub(crate) async fn recolor_calendar(&self, calendar: &Calendar, color: &str) -> Done<String> {
        let data = json!({ "name": calendar.name, "color": color });
        self.update(calendar, data, "recolouring").await?;
        Ok(color.to_owned())
    }

    pub(crate) async fn remove_calendar(&self, calendar: &Calendar) -> Done<()> {
        if calendar.is_primary {
            return Err(CalendarError::Failed(Error::Rejected(
                "Zoho keeps the account's main calendar".into(),
            )));
        }
        self.own(calendar, "removing").await?;
        let reply = self
            .send("DELETE", &self.calendar_url(&calendar.remote_id, None))
            .await?;
        if matches!(reply.status, 404 | 410) {
            return Ok(());
        }
        check(&reply, "removing a calendar")
    }
}
