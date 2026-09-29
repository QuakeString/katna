// SPDX-License-Identifier: GPL-3.0-or-later

//! How Katna reaches an account's calendars, contacts and tasks
//! (`docs/ARCHITECTURE.md` §18). Each kind of account has a best way, tried
//! first, and others tried only when that one is not available:
//!
//! - Google sign-ins: Google's own APIs, then Google's CalDAV and CardDAV
//!   (the same sign-in; for when an API is switched off).
//! - Microsoft sign-ins: Microsoft Graph (Outlook.com has no CalDAV).
//! - Password accounts (Yahoo, Zoho, iCloud, Fastmail, your own server…):
//!   CalDAV and CardDAV, looked for on the provider's known server, the
//!   mail domain and the IMAP server ([`dav_start_urls`]).
//!
//! The way that worked is remembered per account and kind in `pim.db`'s
//! `meta` table, and tried first; after [`RECHECK`] the best way is tried
//! first again, so an API switched on later is used.

use katna_core::{AccountId, OAuthProvider};
use katna_store::Store;

use crate::autoconfig::http;

/// A remembered way is trusted this long before the best way is tried
/// first again (Unix seconds).
pub const RECHECK: i64 = 7 * 24 * 60 * 60;

/// What is synced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Data {
    Calendar,
    Contacts,
    Tasks,
}

impl Data {
    fn key(self) -> &'static str {
        match self {
            Self::Calendar => "sync-method:calendar",
            Self::Contacts => "sync-method:contacts",
            Self::Tasks => "sync-method:tasks",
        }
    }

    /// The DAV protocol that carries it.
    pub fn dav(self) -> Dav {
        match self {
            Self::Calendar | Self::Tasks => Dav::Cal,
            Self::Contacts => Dav::Card,
        }
    }
}

/// A way to reach one kind of data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    /// The provider's own API: Google Calendar, People and Tasks, or
    /// Microsoft Graph.
    Api,
    /// CalDAV or CardDAV.
    Dav,
}

impl Method {
    fn name(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Dav => "dav",
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        match name {
            "api" => Some(Self::Api),
            "dav" => Some(Self::Dav),
            _ => None,
        }
    }
}

/// CalDAV or CardDAV.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dav {
    Cal,
    Card,
}

impl Dav {
    fn well_known(self) -> &'static str {
        match self {
            Self::Cal => "/.well-known/caldav",
            Self::Card => "/.well-known/carddav",
        }
    }
}

/// The ways for an account signed in with `provider` (`None`: a password),
/// best first.
pub fn methods(provider: Option<OAuthProvider>) -> &'static [Method] {
    match provider {
        Some(OAuthProvider::Google) => &[Method::Api, Method::Dav],
        Some(OAuthProvider::Microsoft) => &[Method::Api],
        None => &[Method::Dav],
    }
}

/// The ways to try for `data` of `account`, in order: the remembered one
/// first while it is fresh, then the rest best first.
pub fn order(
    store: &Store,
    account: AccountId,
    data: Data,
    provider: Option<OAuthProvider>,
    now: i64,
) -> Vec<Method> {
    let all = methods(provider);
    let mut order = Vec::with_capacity(all.len());
    if let Some(method) = remembered(store, account, data, now).filter(|m| all.contains(m)) {
        order.push(method);
    }
    for method in all {
        if !order.contains(method) {
            order.push(*method);
        }
    }
    order
}

/// The way that last worked for `data` of `account`, while fresh.
pub fn remembered(store: &Store, account: AccountId, data: Data, now: i64) -> Option<Method> {
    let row = store
        .meta("account", account.0, data.key())
        .ok()
        .flatten()?;
    let value: serde_json::Value = serde_json::from_str(&row.value_json).ok()?;
    let at = value.get("at")?.as_i64()?;
    if now.saturating_sub(at) >= RECHECK {
        return None;
    }
    Method::from_name(value.get("method")?.as_str()?)
}

/// Remembers that `method` worked for `data` of `account`. Writes only
/// when the way changed or the note is getting old, not on every sync.
pub fn remember(store: &mut Store, account: AccountId, data: Data, method: Method, now: i64) {
    if remembered(store, account, data, now - RECHECK / 2) == Some(method) {
        return;
    }
    let value = serde_json::json!({ "method": method.name(), "at": now }).to_string();
    if let Err(err) = store.set_meta("account", account.0, data.key(), &value, None) {
        tracing::debug!(%err, "could not remember the sync method");
    }
}

/// Forgets the way remembered for `data` of `account` (a new sign-in).
pub fn forget(store: &mut Store, account: AccountId, data: Data) {
    let _ = store.remove_meta("account", account.0, data.key());
}

/// Where Google's CalDAV or CardDAV starts for `address`, for a Google
/// sign-in when the API is not available.
pub fn google_dav_start(dav: Dav, address: &str) -> String {
    let address = address.trim().to_ascii_lowercase();
    match dav {
        Dav::Cal => format!("https://apidata.googleusercontent.com/caldav/v2/{address}/user"),
        Dav::Card => format!("https://www.googleapis.com/carddav/v1/principals/{address}/"),
    }
}

/// Servers of providers whose calendars and contacts are not on their
/// mail host: CalDAV and CardDAV starts.
fn known(
    domain: &str,
    imap_host: &str,
) -> Option<(&'static [&'static str], &'static [&'static str])> {
    let is = |names: &[&str]| {
        names
            .iter()
            .any(|n| domain == *n || imap_host == *n || imap_host.ends_with(&format!(".{n}")))
    };
    let yahoo =
        domain.starts_with("yahoo.") || is(&["ymail.com", "rocketmail.com", "mail.yahoo.com"]);
    Some(
        if is(&[
            "fastmail.com",
            "fastmail.fm",
            "fastmail.net",
            "sent.com",
            "messagingengine.com",
        ]) {
            (
                &["https://caldav.fastmail.com/.well-known/caldav"],
                &["https://carddav.fastmail.com/.well-known/carddav"],
            )
        } else if is(&["icloud.com", "me.com", "mac.com"]) {
            (
                &["https://caldav.icloud.com/"],
                &["https://contacts.icloud.com/"],
            )
        } else if yahoo {
            (
                &["https://caldav.calendar.yahoo.com/.well-known/caldav"],
                &["https://carddav.address.yahoo.com/.well-known/carddav"],
            )
        } else if is(&["zoho.in", "zohomail.in"]) {
            (
                &[
                    "https://calendar.zoho.in/.well-known/caldav",
                    "https://calendar.zoho.in/caldav/",
                ],
                &[
                    "https://contacts.zoho.in/.well-known/carddav",
                    "https://contacts.zoho.in/carddav/",
                ],
            )
        } else if is(&["zoho.eu", "zohomail.eu"]) {
            (
                &[
                    "https://calendar.zoho.eu/.well-known/caldav",
                    "https://calendar.zoho.eu/caldav/",
                ],
                &[
                    "https://contacts.zoho.eu/.well-known/carddav",
                    "https://contacts.zoho.eu/carddav/",
                ],
            )
        } else if is(&["zoho.com.au", "zohomail.com.au"]) {
            (
                &[
                    "https://calendar.zoho.com.au/.well-known/caldav",
                    "https://calendar.zoho.com.au/caldav/",
                ],
                &[
                    "https://contacts.zoho.com.au/.well-known/carddav",
                    "https://contacts.zoho.com.au/carddav/",
                ],
            )
        } else if is(&["zoho.com", "zohomail.com"]) {
            (
                &[
                    "https://calendar.zoho.com/.well-known/caldav",
                    "https://calendar.zoho.com/caldav/",
                ],
                &[
                    "https://contacts.zoho.com/.well-known/carddav",
                    "https://contacts.zoho.com/carddav/",
                ],
            )
        } else if is(&["mailbox.org"]) {
            (
                &["https://dav.mailbox.org/.well-known/caldav"],
                &["https://dav.mailbox.org/.well-known/carddav"],
            )
        } else if is(&["posteo.de", "posteo.net"]) {
            (
                &["https://posteo.de:8443/.well-known/caldav"],
                &["https://posteo.de:8843/.well-known/carddav"],
            )
        } else if is(&["gmx.net", "gmx.de", "gmx.at", "gmx.ch", "gmx.com"]) {
            (&["https://caldav.gmx.net/"], &["https://carddav.gmx.net/"])
        } else if is(&["aol.com"]) {
            (&["https://caldav.aol.com/"], &["https://carddav.aol.com/"])
        } else if is(&["web.de"]) {
            (&["https://caldav.web.de/"], &["https://carddav.web.de/"])
        } else {
            return None;
        },
    )
}

/// Whether `d` looks like a domain name.
fn valid(d: &str) -> bool {
    d.contains('.')
        && d.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}

/// Where to look for an account's CalDAV or CardDAV server, in order: a
/// test server, the provider's known server, then the `.well-known`
/// address of the mail domain, of the IMAP server's domain and of the IMAP
/// server itself, and the IMAP server's root.
pub fn dav_start_urls(dav: Dav, address: &str, imap_host: Option<&str>) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    let mut add = |url: String| {
        if !urls.contains(&url) {
            urls.push(url);
        }
    };
    let var = match dav {
        Dav::Cal => "KATNA_CALDAV_URL",
        Dav::Card => "KATNA_CARDDAV_URL",
    };
    if let Some(url) = http::test_url(var) {
        add(url);
    }
    let domain = address
        .rsplit_once('@')
        .map(|(_, d)| d.trim().to_ascii_lowercase())
        .unwrap_or_default();
    let host = imap_host
        .map(|h| h.trim().to_ascii_lowercase())
        .unwrap_or_default();
    if let Some((cal, card)) = known(&domain, &host) {
        let starts = match dav {
            Dav::Cal => cal,
            Dav::Card => card,
        };
        for url in starts {
            add((*url).to_owned());
        }
    }
    let well_known = dav.well_known();
    if valid(&domain) {
        add(format!("https://{domain}{well_known}"));
    }
    if valid(&host) {
        // imap.example.com → example.com, and the host itself.
        if let Some((_, parent)) = host.split_once('.')
            && valid(parent)
        {
            add(format!("https://{parent}{well_known}"));
        }
        add(format!("https://{host}{well_known}"));
        add(format!("https://{host}/"));
    }
    urls
}

/// The part of `host` its provider owns: `caldav.calendar.yahoo.com` →
/// `yahoo.com`, `calendar.zoho.com.au` → `zoho.com.au`. Credentials may go
/// to any host under it.
pub fn owner_domain(host: &str) -> String {
    let host = host.to_ascii_lowercase();
    let labels: Vec<&str> = host.split('.').collect();
    let keep = match labels.as_slice() {
        [.., second, last] if labels.len() >= 3 && last.len() == 2 && second.len() <= 3 => 3,
        _ => 2,
    };
    labels[labels.len().saturating_sub(keep)..].join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_accounts_look_on_the_provider_then_the_domain() {
        let urls = dav_start_urls(Dav::Cal, "me@yahoo.com", Some("imap.mail.yahoo.com"));
        assert_eq!(
            urls[0],
            "https://caldav.calendar.yahoo.com/.well-known/caldav"
        );
        assert!(urls.contains(&"https://yahoo.com/.well-known/caldav".to_owned()));
        assert_eq!(urls.last().unwrap(), "https://imap.mail.yahoo.com/");

        let urls = dav_start_urls(Dav::Card, "me@example.org", Some("imap.zoho.in"));
        assert_eq!(urls[0], "https://contacts.zoho.in/.well-known/carddav");
        assert!(urls.contains(&"https://example.org/.well-known/carddav".to_owned()));

        let urls = dav_start_urls(Dav::Cal, "me@example.org", Some("mail.example.org"));
        assert_eq!(
            urls,
            [
                "https://example.org/.well-known/caldav",
                "https://mail.example.org/.well-known/caldav",
                "https://mail.example.org/",
            ]
        );
    }

    #[test]
    fn each_account_kind_has_its_best_way_first() {
        assert_eq!(
            methods(Some(OAuthProvider::Google)),
            [Method::Api, Method::Dav]
        );
        assert_eq!(methods(Some(OAuthProvider::Microsoft)), [Method::Api]);
        assert_eq!(methods(None), [Method::Dav]);
        assert_eq!(
            google_dav_start(Dav::Cal, "Me@Gmail.com"),
            "https://apidata.googleusercontent.com/caldav/v2/me@gmail.com/user"
        );
    }

    #[test]
    fn owners_of_hosts() {
        assert_eq!(owner_domain("caldav.calendar.yahoo.com"), "yahoo.com");
        assert_eq!(owner_domain("calendar.zoho.com.au"), "zoho.com.au");
        assert_eq!(owner_domain("posteo.de"), "posteo.de");
        assert_eq!(owner_domain("dav.example.co.uk"), "example.co.uk");
        assert_eq!(owner_domain("localhost"), "localhost");
    }

    #[test]
    fn the_way_that_worked_is_tried_first_until_it_is_old() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(
            &katna_core::Paths::with_root(dir.path()),
            katna_store::Mode::ReadWrite,
        )
        .unwrap();
        let account = AccountId(1);
        let google = Some(OAuthProvider::Google);
        assert_eq!(
            order(&store, account, Data::Calendar, google, 1000),
            [Method::Api, Method::Dav]
        );
        remember(&mut store, account, Data::Calendar, Method::Dav, 1000);
        assert_eq!(
            order(&store, account, Data::Calendar, google, 2000),
            [Method::Dav, Method::Api]
        );
        // Other kinds of data keep their own.
        assert_eq!(
            order(&store, account, Data::Contacts, google, 2000),
            [Method::Api, Method::Dav]
        );
        // A week later the best way is tried first again.
        assert_eq!(
            order(&store, account, Data::Calendar, google, 1000 + RECHECK),
            [Method::Api, Method::Dav]
        );
        forget(&mut store, account, Data::Calendar);
        assert_eq!(remembered(&store, account, Data::Calendar, 2000), None);
    }
}
