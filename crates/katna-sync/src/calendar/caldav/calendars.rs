// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars themselves, as changed in Katna's side panel: `MKCALENDAR` in
//! the calendar home for a new one, `PROPPATCH` of its `displayname` or
//! Apple's `calendar-color` (which most servers keep), and `DELETE` of
//! the collection. On servers that share calendars (Nextcloud, iCloud,
//! SOGo), deleting one shared with you only takes it off your list.

use katna_store::calendar::{Calendar, CalendarAccess, NewCalendar};
use ring::rand::{SecureRandom, SystemRandom};

use super::{CalDav, CalendarError, path, xml_escape};
use crate::Error;

type Result<T> = std::result::Result<T, CalendarError>;

const XML: &str = "application/xml; charset=utf-8";

/// A new collection's name in the calendar home.
fn random_name() -> Result<String> {
    let mut bytes = [0u8; 16];
    SystemRandom::new()
        .fill(&mut bytes)
        .map_err(|_| CalendarError::Failed(Error::Protocol("no random bytes".into())))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// The colour property: Apple's, as `#rrggbbff`.
fn color_prop(color: &str) -> String {
    if color.is_empty() {
        return String::new();
    }
    format!(
        r#"<x:calendar-color xmlns:x="http://apple.com/ns/ical/">{}ff</x:calendar-color>"#,
        xml_escape(color)
    )
}

impl CalDav {
    /// The URL of `calendar`'s collection.
    fn calendar_url(&self, calendar: &Calendar) -> String {
        if let Some(url) = self.urls.lock().unwrap().get(&calendar.remote_id) {
            return url.clone();
        }
        format!("{}{}", self.origin, calendar.remote_id)
    }

    fn written(&self, status: u16, body: &[u8], doing: &str) -> Result<()> {
        match status {
            200..=299 => Ok(()),
            401 | 403 => Err(self.refused(status, body)),
            status => Err(CalendarError::Failed(Error::Rejected(format!(
                "CalDAV server answered {status} while {doing}"
            )))),
        }
    }

    /// Makes a calendar named `name` in `color` in the calendar home.
    pub(crate) async fn add_calendar(&self, name: &str, color: &str) -> Result<NewCalendar> {
        let Some(home) = self.home().await? else {
            return Err(CalendarError::NotOffered);
        };
        let url = format!("{}/{}/", home.trim_end_matches('/'), random_name()?);
        let body = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<c:mkcalendar xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav"><d:set><d:prop>
<d:displayname>{}</d:displayname>{}
<c:supported-calendar-component-set><c:comp name="VEVENT"/></c:supported-calendar-component-set>
</d:prop></d:set></c:mkcalendar>"#,
            xml_escape(name),
            color_prop(color)
        );
        let reply = self
            .request("MKCALENDAR", &url, &[], Some((XML, body.as_bytes())))
            .await?;
        self.written(reply.status, &reply.body, "adding a calendar")?;
        let remote_id = path(&url).to_owned();
        self.urls
            .lock()
            .unwrap()
            .insert(remote_id.clone(), url.clone());
        Ok(NewCalendar {
            remote_id,
            name: name.to_owned(),
            color: color.to_owned(),
            access: CalendarAccess::Owner,
            is_primary: false,
            time_zone: String::new(),
        })
    }

    async fn set_props(&self, calendar: &Calendar, props: &str, doing: &str) -> Result<()> {
        let body = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<d:propertyupdate xmlns:d="DAV:"><d:set><d:prop>{props}</d:prop></d:set></d:propertyupdate>"#
        );
        let url = self.calendar_url(calendar);
        let reply = self
            .request("PROPPATCH", &url, &[], Some((XML, body.as_bytes())))
            .await?;
        self.written(reply.status, &reply.body, doing)?;
        // A 207 can still refuse the property.
        let text = String::from_utf8_lossy(&reply.body);
        if reply.status == 207 && (text.contains(" 403 ") || text.contains(" 409 ")) {
            return Err(CalendarError::Failed(Error::Rejected(format!(
                "the CalDAV server refused {doing}"
            ))));
        }
        Ok(())
    }

    pub(crate) async fn rename_calendar(&self, calendar: &Calendar, name: &str) -> Result<()> {
        let prop = format!("<d:displayname>{}</d:displayname>", xml_escape(name));
        self.set_props(calendar, &prop, "renaming a calendar").await
    }

    /// Gives `calendar` colour `color`; returns it as stored.
    pub(crate) async fn recolor_calendar(
        &self,
        calendar: &Calendar,
        color: &str,
    ) -> Result<String> {
        self.set_props(calendar, &color_prop(color), "changing a calendar's colour")
            .await?;
        Ok(color.to_owned())
    }

    pub(crate) async fn remove_calendar(&self, calendar: &Calendar) -> Result<()> {
        let url = self.calendar_url(calendar);
        let reply = self.request("DELETE", &url, &[], None).await?;
        if matches!(reply.status, 404 | 410) {
            return Ok(());
        }
        self.written(reply.status, &reply.body, "removing a calendar")?;
        self.urls.lock().unwrap().remove(&calendar.remote_id);
        Ok(())
    }
}
