// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars themselves, as people change them in Katna's side panel: a
//! new calendar, a new name or colour, and deleting one or taking one
//! shared with them off their list. Unlike event changes these go to the
//! service at once and reach the store only once it took them, so a
//! refusal is shown where it was asked, not undone later.

use katna_store::calendar::{Calendar, NewCalendar};

use super::{CalendarError, edit::Remote};

/// Makes a calendar named `name` in `color` (`#rrggbb`, or empty) on
/// `remote`; returns it as the store keeps it.
pub async fn add(
    remote: &Remote<'_>,
    name: &str,
    color: &str,
) -> Result<NewCalendar, CalendarError> {
    match remote {
        Remote::Google(google) => google.add_calendar(name, color).await,
        Remote::Microsoft(graph) => graph.add_calendar(name, color).await,
        Remote::CalDav(dav) => dav.add_calendar(name, color).await,
        Remote::Zoho(zoho) => zoho.add_calendar(name, color).await,
    }
}

/// Renames `calendar` on `remote`, its service.
pub async fn rename(
    remote: &Remote<'_>,
    calendar: &Calendar,
    name: &str,
) -> Result<(), CalendarError> {
    match remote {
        Remote::Google(google) => google.rename_calendar(calendar, name).await,
        Remote::Microsoft(graph) => graph.rename_calendar(calendar, name).await,
        Remote::CalDav(dav) => dav.rename_calendar(calendar, name).await,
        Remote::Zoho(zoho) => zoho.rename_calendar(calendar, name).await,
    }
}

/// Gives `calendar` colour `color` on `remote`; returns the colour the
/// service keeps (Outlook's nearest named one).
pub async fn recolor(
    remote: &Remote<'_>,
    calendar: &Calendar,
    color: &str,
) -> Result<String, CalendarError> {
    match remote {
        Remote::Google(google) => google.recolor_calendar(calendar, color).await,
        Remote::Microsoft(graph) => graph.recolor_calendar(calendar, color).await,
        Remote::CalDav(dav) => dav.recolor_calendar(calendar, color).await,
        Remote::Zoho(zoho) => zoho.recolor_calendar(calendar, color).await,
    }
}

/// Deletes `calendar` on `remote` (`delete`), or takes it off the
/// person's list. Only Google tells the two apart; elsewhere the service
/// deletes one's own calendar and unsubscribes from someone else's.
pub async fn remove(
    remote: &Remote<'_>,
    calendar: &Calendar,
    delete: bool,
) -> Result<(), CalendarError> {
    match remote {
        Remote::Google(google) => google.remove_calendar(calendar, delete).await,
        Remote::Microsoft(graph) => graph.remove_calendar(calendar).await,
        Remote::CalDav(dav) => dav.remove_calendar(calendar).await,
        Remote::Zoho(zoho) => zoho.remove_calendar(calendar).await,
    }
}

#[cfg(test)]
mod tests;
