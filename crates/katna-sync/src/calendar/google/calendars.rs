// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars themselves, as changed in Katna's side panel: `calendars.insert`
//! for a new one, `calendars.patch` to rename one's own calendar (a
//! `summaryOverride` in the calendar list for someone else's), the
//! calendar list's `backgroundColor` for its colour (each person's own),
//! and `calendars.delete` or, for a calendar only subscribed to,
//! `calendarList.delete`.

use katna_store::calendar::{Calendar, CalendarAccess, NewCalendar};
use serde::Deserialize;
use serde_json::json;

use super::{CalendarError, GoogleCalendar, check, parse};
use crate::{Error, autoconfig::http};

type Result<T> = std::result::Result<T, CalendarError>;

/// What Google says about a calendar it made.
#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct Made {
    id: String,
    time_zone: String,
}

impl GoogleCalendar {
    fn calendar_url(&self, id: &str) -> String {
        format!("{}/calendar/v3/calendars/{}", self.api, http::escape(id))
    }

    fn list_entry_url(&self, id: &str) -> String {
        format!(
            "{}/calendar/v3/users/me/calendarList/{}",
            self.api,
            http::escape(id)
        )
    }

    /// Makes a calendar named `name` in `color` (`#rrggbb`, or empty for
    /// Google's pick).
    pub(crate) async fn add_calendar(&self, name: &str, color: &str) -> Result<NewCalendar> {
        let url = format!("{}/calendar/v3/calendars", self.api);
        let reply = self
            .send("POST", &url, Some(&json!({ "summary": name })))
            .await?;
        check(&reply, "adding a calendar")?;
        let made: Made = parse(&reply.body)?;
        if made.id.is_empty() {
            return Err(CalendarError::Failed(Error::Protocol(
                "Google Calendar made a calendar without an ID".into(),
            )));
        }
        let mut color = color.to_owned();
        if !color.is_empty()
            && let Err(err) = self.color_entry(&made.id, &color).await
        {
            // The calendar is there; the sync brings Google's colour.
            tracing::warn!(%err, "new calendar kept Google's colour");
            color.clear();
        }
        Ok(NewCalendar {
            remote_id: made.id,
            name: name.to_owned(),
            color,
            access: CalendarAccess::Owner,
            is_primary: false,
            time_zone: made.time_zone,
        })
    }

    /// Renames `calendar`: for everyone when it is one's own, else only
    /// in one's own list.
    pub(crate) async fn rename_calendar(&self, calendar: &Calendar, name: &str) -> Result<()> {
        let reply = if calendar.access == CalendarAccess::Owner {
            let url = self.calendar_url(&calendar.remote_id);
            self.send("PATCH", &url, Some(&json!({ "summary": name })))
                .await?
        } else {
            let url = self.list_entry_url(&calendar.remote_id);
            self.send("PATCH", &url, Some(&json!({ "summaryOverride": name })))
                .await?
        };
        check(&reply, "renaming a calendar")
    }

    /// Gives `calendar` colour `color` (`#rrggbb`); returns it as stored.
    pub(crate) async fn recolor_calendar(
        &self,
        calendar: &Calendar,
        color: &str,
    ) -> Result<String> {
        self.color_entry(&calendar.remote_id, color).await?;
        Ok(color.to_owned())
    }

    async fn color_entry(&self, id: &str, color: &str) -> Result<()> {
        let url = format!("{}?colorRgbFormat=true", self.list_entry_url(id));
        let body = json!({ "backgroundColor": color, "foregroundColor": "#ffffff" });
        let reply = self.send("PATCH", &url, Some(&body)).await?;
        check(&reply, "changing a calendar's colour")
    }

    /// Deletes `calendar` for everyone (`delete`, one's own only) or takes
    /// it off one's list.
    pub(crate) async fn remove_calendar(&self, calendar: &Calendar, delete: bool) -> Result<()> {
        if calendar.is_primary {
            return Err(CalendarError::Failed(Error::Rejected(
                "Google keeps the account's main calendar".into(),
            )));
        }
        let (url, doing) = if delete && calendar.access == CalendarAccess::Owner {
            (
                self.calendar_url(&calendar.remote_id),
                "deleting a calendar",
            )
        } else {
            (
                self.list_entry_url(&calendar.remote_id),
                "removing a calendar from the list",
            )
        };
        let reply = self.send("DELETE", &url, None).await?;
        if matches!(reply.status, 404 | 410) {
            return Ok(());
        }
        check(&reply, doing)
    }
}
