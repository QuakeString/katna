// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars themselves, as changed in Katna's side panel, with
//! `/me/calendars`: a `POST` for a new one, a `PATCH` of its name or
//! colour, and a `DELETE`, which deletes one's own calendar and takes
//! someone else's off one's list. Outlook takes only its named colours,
//! so a colour becomes the nearest of them.

use katna_store::calendar::{Calendar, CalendarAccess, NewCalendar};
use serde_json::json;

use super::{
    CalendarError, GraphCalendar, GraphCalendarEntry, check, hex_color, named_color, parse,
};
use crate::{Error, autoconfig::http};

type Result<T> = std::result::Result<T, CalendarError>;

/// Outlook's named colours, which [`named_color`] draws.
const NAMES: [&str; 9] = [
    "lightBlue",
    "lightGreen",
    "lightOrange",
    "lightGray",
    "lightYellow",
    "lightTeal",
    "lightPink",
    "lightBrown",
    "lightRed",
];

/// Hue in degrees and saturation (0 to 1) of `#rrggbb`.
fn hue(hex: &str) -> Option<(f32, f32)> {
    let hex = hex_color(hex);
    let n = u32::from_str_radix(hex.get(1..)?, 16).ok()?;
    let [r, g, b] = [n >> 16, n >> 8, n].map(|c| (c & 255) as f32 / 255.0);
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let delta = max - min;
    let saturation = if max == 0.0 { 0.0 } else { delta / max };
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    Some((hue, saturation))
}

/// The Outlook colour nearest to `color` by hue (grey for a greyish one),
/// `auto` for none.
pub(super) fn nearest_name(color: &str) -> &'static str {
    let Some((want, saturation)) = hue(color) else {
        return "auto";
    };
    if saturation < 0.15 {
        return "lightGray";
    }
    NAMES
        .iter()
        .filter(|name| **name != "lightGray")
        .min_by_key(|name| {
            let (have, _) = hue(named_color(name)).unwrap_or_default();
            let apart = (have - want).abs();
            apart.min(360.0 - apart) as i32
        })
        .copied()
        .unwrap_or("auto")
}

impl GraphCalendar {
    fn calendar_url(&self, id: &str) -> String {
        format!("{}/me/calendars/{}", self.api, http::escape(id))
    }

    /// Makes a calendar named `name` in the Outlook colour nearest to
    /// `color`.
    pub(crate) async fn add_calendar(&self, name: &str, color: &str) -> Result<NewCalendar> {
        let body = json!({ "name": name, "color": nearest_name(color) });
        let url = format!("{}/me/calendars", self.api);
        let reply = self.send("POST", &url, Some(&body)).await?;
        check(&reply, "adding a calendar")?;
        let made: GraphCalendarEntry = parse(&reply.body)?;
        if made.id.is_empty() {
            return Err(CalendarError::Failed(Error::Protocol(
                "Outlook made a calendar without an ID".into(),
            )));
        }
        let color = match hex_color(&made.hex_color) {
            hex if hex.is_empty() => named_color(&made.color).to_owned(),
            hex => hex,
        };
        Ok(NewCalendar {
            remote_id: made.id,
            name: name.to_owned(),
            color,
            access: CalendarAccess::Owner,
            is_primary: false,
            time_zone: String::new(),
        })
    }

    pub(crate) async fn rename_calendar(&self, calendar: &Calendar, name: &str) -> Result<()> {
        let url = self.calendar_url(&calendar.remote_id);
        let reply = self
            .send("PATCH", &url, Some(&json!({ "name": name })))
            .await?;
        check(&reply, "renaming a calendar")
    }

    /// Gives `calendar` the Outlook colour nearest to `color`; returns the
    /// colour as stored.
    pub(crate) async fn recolor_calendar(
        &self,
        calendar: &Calendar,
        color: &str,
    ) -> Result<String> {
        let name = nearest_name(color);
        let url = self.calendar_url(&calendar.remote_id);
        let reply = self
            .send("PATCH", &url, Some(&json!({ "color": name })))
            .await?;
        check(&reply, "changing a calendar's colour")?;
        Ok(named_color(name).to_owned())
    }

    pub(crate) async fn remove_calendar(&self, calendar: &Calendar) -> Result<()> {
        if calendar.is_primary {
            return Err(CalendarError::Failed(Error::Rejected(
                "Outlook keeps the account's main calendar".into(),
            )));
        }
        let reply = self
            .send("DELETE", &self.calendar_url(&calendar.remote_id), None)
            .await?;
        if matches!(reply.status, 404 | 410) {
            return Ok(());
        }
        check(&reply, "removing a calendar")
    }
}

#[cfg(test)]
mod tests {
    use super::nearest_name;

    #[test]
    fn nearest_outlook_colours() {
        assert_eq!(nearest_name("#d50000"), "lightRed");
        assert_eq!(nearest_name("#039be5"), "lightBlue");
        assert_eq!(nearest_name("#7cb342"), "lightGreen");
        assert_eq!(nearest_name("#8e24aa"), "lightPink");
        assert_eq!(nearest_name("#616161"), "lightGray");
        assert_eq!(nearest_name(""), "auto");
    }
}
