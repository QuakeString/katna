// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendar sync (`docs/ARCHITECTURE.md` §18): each account's calendars
//! and events into the store, through its provider's own API.
//!
//! - [`google`]: Google Calendar API v3, incremental with sync tokens.
//! - [`graph`]: Microsoft Graph (Outlook.com, Microsoft 365), read in full
//!   each time and written only where an event's etag changed.
//! - [`caldav`]: CalDAV (RFC 4791) on the server of an IMAP account, when
//!   it offers it; only events whose etag changed are downloaded.
//!
//! Each client's `sync` writes through [`katna_store::Store`] and says
//! whether anything changed, so the daemon only signals real changes.
//! Changes made in Katna go the other way through [`edit`].

use std::collections::{HashMap, HashSet};

use katna_store::{Store, calendar::EventData};

use crate::Error;

pub mod caldav;
pub mod edit;
pub mod google;
pub mod graph;

#[cfg(test)]
pub(crate) mod fake;

/// Largest answer a calendar request reads: a page of 2500 Google events
/// or a CalDAV multiget.
const MAX_ANSWER: usize = 32 * 1024 * 1024;

/// Why an account's calendars could not sync.
#[derive(Debug, thiserror::Error)]
pub enum CalendarError {
    /// The account's sign-in did not allow Katna into its calendars (it
    /// was signed in before Katna asked), or the grant was revoked: the
    /// user has to sign in again.
    #[error("{0}")]
    NeedsSignIn(String),
    /// The provider has the calendar API switched off for Katna (Google's
    /// `accessNotConfigured`).
    #[error("{0}")]
    NotEnabled(String),
    /// The account's server has no calendars (no CalDAV).
    #[error("the server offers no calendars")]
    NotOffered,
    /// Anything else: the network, the server, the store.
    #[error("{0}")]
    Failed(Error),
}

impl From<Error> for CalendarError {
    fn from(err: Error) -> Self {
        match err {
            Error::Auth(message) => Self::NeedsSignIn(message),
            other => Self::Failed(other),
        }
    }
}

impl From<katna_store::Error> for CalendarError {
    fn from(err: katna_store::Error) -> Self {
        Self::Failed(Error::Store(err))
    }
}

/// Result of a calendar sync: whether anything changed in the store.
pub type SyncResult = std::result::Result<bool, CalendarError>;

/// One remote item as a full listing returns it: its ID, its etag and the
/// rows it stands for.
pub(crate) type Item = (String, Option<String>, Vec<EventData>);

/// Makes calendar `calendar` hold exactly `items`, rewriting only those
/// whose etag changed and deleting those no longer listed. Returns
/// whether anything changed.
pub(crate) fn apply_full(
    store: &mut Store,
    calendar: i64,
    items: Vec<Item>,
) -> Result<bool, katna_store::Error> {
    let have: HashMap<String, Option<String>> = store.event_etags(calendar)?.into_iter().collect();
    let mut seen = HashSet::new();
    let mut writes = Vec::new();
    for (remote_id, etag, events) in items {
        if !seen.insert(remote_id.clone()) {
            continue;
        }
        if etag.is_some() && have.get(&remote_id) == Some(&etag) {
            continue;
        }
        writes.push((remote_id, events));
    }
    for remote_id in have.keys() {
        if !seen.contains(remote_id) {
            writes.push((remote_id.clone(), Vec::new()));
        }
    }
    if writes.is_empty() {
        return Ok(false);
    }
    store.replace_events_batch(calendar, &writes)?;
    Ok(true)
}

/// Unix seconds of an RFC 3339 time, or 0.
pub(crate) fn rfc3339(text: &str) -> i64 {
    text.parse::<jiff::Timestamp>().map_or(0, |t| t.as_second())
}

/// `#rrggbb` in lower case, or empty.
pub(crate) fn hex_color(text: &str) -> String {
    let text = text.trim();
    let valid = text.len() >= 7
        && text.starts_with('#')
        && text[1..7].bytes().all(|b| b.is_ascii_hexdigit());
    if valid {
        text[..7].to_ascii_lowercase()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors() {
        assert_eq!(hex_color("#039BE5"), "#039be5");
        assert_eq!(hex_color("#FF2968FF"), "#ff2968");
        assert_eq!(hex_color("blue"), "");
        assert_eq!(hex_color("#12"), "");
    }

    #[test]
    fn full_listings_write_only_what_changed() {
        use katna_core::{AccountId, Paths};
        use katna_store::calendar::{CalendarAccess, CalendarSource, NewCalendar};

        let dir = tempfile::tempdir().unwrap();
        let mut store =
            Store::open(&Paths::with_root(dir.path()), katna_store::Mode::ReadWrite).unwrap();
        let calendar = store
            .upsert_calendar(
                Some(AccountId(1)),
                CalendarSource::Google,
                &NewCalendar {
                    remote_id: "c".into(),
                    name: "C".into(),
                    color: String::new(),
                    access: CalendarAccess::Owner,
                    is_primary: true,
                    time_zone: String::new(),
                },
                0,
            )
            .unwrap();
        let event = |id: &str, etag: &str| -> Item {
            (
                id.into(),
                Some(etag.into()),
                vec![EventData {
                    remote_id: id.into(),
                    uid: id.into(),
                    etag: Some(etag.into()),
                    start: 10,
                    end: 20,
                    range_end: Some(20),
                    ..EventData::default()
                }],
            )
        };
        assert!(apply_full(&mut store, calendar, vec![event("a", "1"), event("b", "1")]).unwrap());
        assert!(!apply_full(&mut store, calendar, vec![event("a", "1"), event("b", "1")]).unwrap());
        assert!(apply_full(&mut store, calendar, vec![event("a", "2")]).unwrap());
        let mut etags = store.event_etags(calendar).unwrap();
        etags.sort();
        assert_eq!(etags, [("a".to_owned(), Some("2".to_owned()))]);
    }
}
