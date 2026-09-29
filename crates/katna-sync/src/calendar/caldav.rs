// SPDX-License-Identifier: GPL-3.0-or-later

//! CalDAV (RFC 4791): for a password account, looked for on its provider's
//! known server, its mail domain and its IMAP server
//! ([`crate::methods::dav_start_urls`], RFC 6764 `.well-known`); for a
//! Google sign-in whose Calendar API is not available, Google's own CalDAV
//! with the sign-in's token. Then the user's principal and calendar home. Each sync lists the calendars
//! (a calendar whose `getctag` or `sync-token` didn't change is skipped),
//! asks each changed one for its events' etags, and downloads only the
//! events whose etag changed (`calendar-multiget`), read by
//! [`katna_dav::ical`]. The password is the IMAP one, sent only over TLS
//! and only to hosts of the domains the search started on.

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use jiff::tz::TimeZone;
use katna_core::AccountId;
use katna_dav::ical;
use katna_store::{
    Store,
    calendar::{CalendarAccess, CalendarSource, EventData, NewCalendar},
};

use super::{CalendarError, MAX_ANSWER, SyncResult, hex_color};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    methods::{self, Dav},
    net::Tls,
    oauth::TokenSource,
};

const TIMEOUT: Duration = Duration::from_secs(2 * 60);

/// A server that offered no calendars is asked again after this long.
const RETRY_DISCOVERY: Duration = Duration::from_secs(6 * 60 * 60);

/// Events downloaded per `calendar-multiget`.
const MULTIGET: usize = 50;

/// Redirects followed per request.
const MAX_REDIRECTS: usize = 5;

const PRINCIPAL: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<d:propfind xmlns:d="DAV:"><d:prop><d:current-user-principal/></d:prop></d:propfind>"#;

const HOME: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<d:propfind xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav"><d:prop><c:calendar-home-set/></d:prop></d:propfind>"#;

const CALENDARS: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<d:propfind xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav" xmlns:cs="http://calendarserver.org/ns/" xmlns:a="http://apple.com/ns/ical/">
<d:prop><d:resourcetype/><d:displayname/><a:calendar-color/><c:supported-calendar-component-set/><cs:getctag/><d:sync-token/><d:current-user-privilege-set/></d:prop>
</d:propfind>"#;

const ETAGS: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<c:calendar-query xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
<d:prop><d:getetag/></d:prop>
<c:filter><c:comp-filter name="VCALENDAR"><c:comp-filter name="VEVENT"/></c:comp-filter></c:filter>
</c:calendar-query>"#;

/// What discovery found.
enum Found {
    Unknown,
    /// The calendar home's URL.
    Home(String),
    /// No CalDAV, as of then.
    Missing(Instant),
}

/// How requests prove who the user is.
enum Auth {
    /// The account's password.
    Basic(String),
    /// A Google sign-in's access token.
    Bearer(Arc<TokenSource>),
}

/// One account's CalDAV server.
pub struct CalDav {
    /// `https://host[:port]` of the first place looked: where paths of
    /// calendars not found yet go.
    origin: String,
    /// Where discovery looks, in order.
    starts: Vec<String>,
    /// The domains whose hosts may get the credentials.
    domains: Vec<String>,
    auth: Auth,
    tls: Tls,
    found: Mutex<Found>,
    /// Each calendar's URL (by its remote ID) as the last sync found it,
    /// for sending changes.
    urls: Mutex<HashMap<String, String>>,
}

/// One property of a multistatus answer.
#[derive(Debug, Default, Clone)]
struct Prop {
    /// Local name.
    name: String,
    text: String,
    /// Local names of every element inside.
    inside: Vec<String>,
    /// Every `href` inside.
    hrefs: Vec<String>,
    /// The `name` of every `comp` inside.
    comps: Vec<String>,
}

/// One `response` of a multistatus answer, with its found properties.
#[derive(Debug, Default, Clone)]
struct DavResponse {
    href: String,
    props: Vec<Prop>,
}

impl DavResponse {
    fn prop(&self, name: &str) -> Option<&Prop> {
        self.props.iter().find(|p| p.name == name)
    }

    fn text(&self, name: &str) -> Option<&str> {
        self.prop(name)
            .map(|p| p.text.trim())
            .filter(|t| !t.is_empty())
    }
}

/// Reads a `207 Multi-Status` body.
fn multistatus(body: &[u8]) -> Result<Vec<DavResponse>> {
    let text = std::str::from_utf8(body)
        .map_err(|_| Error::Protocol("CalDAV answer is not UTF-8".into()))?;
    let doc = roxmltree::Document::parse(text)
        .map_err(|err| Error::Protocol(format!("CalDAV answer: {err}")))?;
    let local = |node: &roxmltree::Node<'_, '_>| node.tag_name().name().to_owned();
    let mut out = Vec::new();
    for response in doc
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "response")
    {
        let mut found = DavResponse::default();
        for child in response.children().filter(roxmltree::Node::is_element) {
            match child.tag_name().name() {
                "href" if found.href.is_empty() => {
                    found.href = child.text().unwrap_or_default().trim().to_owned();
                }
                "propstat" => {
                    let ok = child
                        .children()
                        .find(|n| n.tag_name().name() == "status")
                        .and_then(|n| n.text())
                        .is_none_or(|s| s.contains(" 200"));
                    if !ok {
                        continue;
                    }
                    for prop in child
                        .children()
                        .filter(|n| n.tag_name().name() == "prop")
                        .flat_map(|p| p.children().filter(roxmltree::Node::is_element))
                    {
                        let elements = prop.descendants().filter(|n| n.is_element()).skip(1);
                        found.props.push(Prop {
                            name: local(&prop),
                            text: prop
                                .descendants()
                                .filter(roxmltree::Node::is_text)
                                .filter_map(|n| n.text())
                                .collect(),
                            inside: elements.clone().map(|n| local(&n)).collect(),
                            hrefs: elements
                                .clone()
                                .filter(|n| n.tag_name().name() == "href")
                                .filter_map(|n| n.text())
                                .map(|t| t.trim().to_owned())
                                .collect(),
                            comps: elements
                                .filter(|n| n.tag_name().name() == "comp")
                                .filter_map(|n| n.attribute("name"))
                                .map(str::to_ascii_uppercase)
                                .collect(),
                        });
                    }
                }
                _ => {}
            }
        }
        out.push(found);
    }
    Ok(out)
}

/// Basic authentication with `user` and `password`.
fn basic(user: &str, password: &str) -> Auth {
    Auth::Basic(format!(
        "Basic {}",
        STANDARD.encode(format!("{user}:{password}"))
    ))
}

/// `scheme://authority` of `url`.
fn origin(url: &str) -> &str {
    let after = url.find("://").map_or(0, |i| i + 3);
    let end = url[after..].find('/').map_or(url.len(), |i| after + i);
    &url[..end]
}

/// The path (and query) of `href`, which may be a whole URL.
fn path(href: &str) -> &str {
    if href.starts_with("http://") || href.starts_with("https://") {
        let rest = &href[origin(href).len()..];
        if rest.is_empty() { "/" } else { rest }
    } else {
        href
    }
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The host of `url`, without port.
fn host(url: &str) -> &str {
    let authority = &origin(url)[origin(url).find("://").map_or(0, |i| i + 3)..];
    let authority = authority.rsplit('@').next().unwrap_or(authority);
    authority.split(':').next().unwrap_or(authority)
}

impl CalDav {
    /// The CalDAV server on `host` (the account's IMAP host), logging in
    /// as `user` with `password`.
    pub fn new(host: &str, user: &str, password: &str, tls: Tls) -> Self {
        Self::for_account("", host, user, password, tls)
    }

    /// The CalDAV server of a password account with `address` on IMAP
    /// server `host`, looked for where [`methods::dav_start_urls`] says.
    pub fn for_account(address: &str, host: &str, user: &str, password: &str, tls: Tls) -> Self {
        let starts = methods::dav_start_urls(Dav::Cal, address, Some(host));
        Self::with_starts(starts, basic(user, password), tls)
    }

    /// Google's CalDAV for the Google sign-in of `address`.
    pub fn google(tokens: Arc<TokenSource>, address: &str, tls: Tls) -> Self {
        let start = methods::google_dav_start(Dav::Cal, address);
        Self::with_starts(vec![start], Auth::Bearer(tokens), tls)
    }

    /// Talks to `origin` (`scheme://host[:port]`), for tests.
    pub fn with_origin(origin: &str, user: &str, password: &str, tls: Tls) -> Self {
        let origin = origin.trim_end_matches('/');
        let starts = vec![format!("{origin}/.well-known/caldav"), format!("{origin}/")];
        Self::with_starts(starts, basic(user, password), tls)
    }

    /// Google's CalDAV at `origin`, for tests.
    #[cfg(test)]
    pub(crate) fn google_at(origin: &str, tokens: Arc<TokenSource>, tls: Tls) -> Self {
        Self::with_starts(
            vec![format!("{origin}/caldav/v2/me/user")],
            Auth::Bearer(tokens),
            tls,
        )
    }

    fn with_starts(starts: Vec<String>, auth: Auth, tls: Tls) -> Self {
        let mut domains: Vec<String> = Vec::new();
        for start in &starts {
            let domain = methods::owner_domain(host(start));
            if !domains.contains(&domain) {
                domains.push(domain);
            }
        }
        Self {
            origin: starts
                .first()
                .map(|s| origin(s).to_owned())
                .unwrap_or_default(),
            starts,
            domains,
            auth,
            tls,
            found: Mutex::new(Found::Unknown),
            urls: Mutex::default(),
        }
    }

    /// The `Authorization` header.
    async fn authorization(&self) -> Result<String> {
        Ok(match &self.auth {
            Auth::Basic(header) => header.clone(),
            Auth::Bearer(tokens) => format!("Bearer {}", tokens.access_token().await?),
        })
    }

    /// Whether a refused request may pass with a fresh token.
    fn retry_refused(&self) -> bool {
        matches!(&self.auth, Auth::Bearer(tokens) if tokens.forget_access_token())
    }

    /// What the server refusing a request (401 or 403) means.
    fn refused(&self, status: u16, body: &[u8]) -> CalendarError {
        match (&self.auth, status) {
            (Auth::Basic(_), _) => CalendarError::Failed(Error::Rejected(
                "the CalDAV server refused the password".into(),
            )),
            (Auth::Bearer(_), 401) => {
                CalendarError::NeedsSignIn("Google refused the sign-in for CalDAV".into())
            }
            (Auth::Bearer(_), _) => CalendarError::NotEnabled(format!(
                "Google CalDAV: {}",
                String::from_utf8_lossy(&body[..body.len().min(300)])
            )),
        }
    }

    /// Whether the password may go to `url`: the same origin, or HTTPS to
    /// a host of the account's domain.
    fn trusted(&self, url: &str) -> bool {
        if self.starts.iter().any(|start| origin(start) == origin(url)) {
            return true;
        }
        let host = host(url).to_ascii_lowercase();
        url.starts_with("https://")
            && self
                .domains
                .iter()
                .any(|d| host == *d || host.ends_with(&format!(".{d}")))
    }

    /// `href` from an answer to `base` as a whole URL; `None` if the
    /// password may not go there.
    fn absolute(&self, base: &str, href: &str) -> Option<String> {
        let url = if href.starts_with("http://") || href.starts_with("https://") {
            href.to_owned()
        } else if href.starts_with('/') {
            format!("{}{href}", origin(base))
        } else {
            let dir = &base[..base.rfind('/').map_or(base.len(), |i| i + 1)];
            format!("{dir}{href}")
        };
        self.trusted(&url).then_some(url)
    }

    /// Sends `method` with an XML `body`, following redirects; returns
    /// where it ended and the answer.
    async fn send(
        &self,
        method: &str,
        url: &str,
        depth: &str,
        body: &str,
    ) -> Result<(String, Reply)> {
        let mut url = url.to_owned();
        let mut retried = false;
        for _ in 0..=MAX_REDIRECTS + 1 {
            if !self.trusted(&url) {
                return Err(Error::Protocol(format!("CalDAV led elsewhere: {url}")));
            }
            let authorization = self.authorization().await?;
            let headers = [("Authorization", authorization.as_str()), ("Depth", depth)];
            let reply = http::exchange_limited(
                method,
                &url,
                &headers,
                Some(("application/xml; charset=utf-8", body.as_bytes())),
                None,
                &self.tls,
                TIMEOUT,
                MAX_ANSWER,
            )
            .await?;
            if matches!(reply.status, 301 | 302 | 303 | 307 | 308)
                && let Some(to) = &reply.location
            {
                url = self
                    .absolute(&url, to)
                    .ok_or_else(|| Error::Protocol(format!("CalDAV led elsewhere: {to}")))?;
                continue;
            }
            if reply.status == 401 && !retried && self.retry_refused() {
                retried = true;
                continue;
            }
            return Ok((url, reply));
        }
        Err(Error::Protocol("CalDAV: too many redirects".into()))
    }

    /// A PROPFIND or REPORT: the multistatus answer, `None` when the
    /// server has no such thing (404, 405, 501…).
    async fn dav(
        &self,
        method: &str,
        url: &str,
        depth: &str,
        body: &str,
    ) -> std::result::Result<Option<(String, Vec<DavResponse>)>, CalendarError> {
        let (url, reply) = self.send(method, url, depth, body).await?;
        match reply.status {
            207 => Ok(Some((url, multistatus(&reply.body)?))),
            401 | 403 => Err(self.refused(reply.status, &reply.body)),
            status if status >= 500 && status != 501 => Err(CalendarError::Failed(
                Error::Rejected(format!("CalDAV server answered {status}")),
            )),
            _ => Ok(None),
        }
    }

    /// The calendar home, found now or before; `None` when the server has
    /// no CalDAV.
    async fn home(&self) -> std::result::Result<Option<String>, CalendarError> {
        match &*self.found.lock().unwrap() {
            Found::Home(home) => return Ok(Some(home.clone())),
            Found::Missing(when) if when.elapsed() < RETRY_DISCOVERY => return Ok(None),
            _ => {}
        }
        let found = match self.discover().await {
            Ok(found) => found,
            // No web server, or not one that speaks DAV.
            Err(CalendarError::Failed(err)) if !matches!(err, Error::Rejected(_)) => {
                tracing::debug!(%err, "no CalDAV");
                None
            }
            Err(err) => return Err(err),
        };
        *self.found.lock().unwrap() = match &found {
            Some(home) => Found::Home(home.clone()),
            None => Found::Missing(Instant::now()),
        };
        Ok(found)
    }

    async fn discover(&self) -> std::result::Result<Option<String>, CalendarError> {
        let mut principal = None;
        for url in &self.starts {
            let (at, responses) = match self.dav("PROPFIND", url, "0", PRINCIPAL).await {
                Ok(Some(found)) => found,
                Ok(None) => continue,
                // Nothing there, or not a DAV server: the next place.
                Err(CalendarError::Failed(err)) if !matches!(err, Error::Rejected(_)) => {
                    tracing::debug!(%url, %err, "no CalDAV here");
                    continue;
                }
                Err(err) => return Err(err),
            };
            let href = responses
                .iter()
                .find_map(|r| r.prop("current-user-principal")?.hrefs.first().cloned());
            if let Some(href) = href {
                principal = self.absolute(&at, &href);
                break;
            }
        }
        let Some(principal) = principal else {
            return Ok(None);
        };
        let Some((at, responses)) = self.dav("PROPFIND", &principal, "0", HOME).await? else {
            return Ok(None);
        };
        Ok(responses
            .iter()
            .find_map(|r| r.prop("calendar-home-set")?.hrefs.first().cloned())
            .and_then(|href| self.absolute(&at, &href)))
    }

    /// Brings `account`'s calendars and their events into `store`;
    /// `address` is the user's. Returns whether anything changed.
    pub async fn sync(&self, store: &mut Store, account: AccountId, address: &str) -> SyncResult {
        let Some(home) = self.home().await? else {
            return Err(CalendarError::NotOffered);
        };
        let Some((at, responses)) = self.dav("PROPFIND", &home, "1", CALENDARS).await? else {
            // The home moved: find it again next time.
            *self.found.lock().unwrap() = Found::Unknown;
            return Err(CalendarError::Failed(Error::Protocol(
                "the CalDAV calendar home is gone".into(),
            )));
        };
        let calendars: Vec<(NewCalendar, String, Option<String>)> = responses
            .iter()
            .filter(|r| {
                r.prop("resourcetype")
                    .is_some_and(|p| p.inside.iter().any(|n| n == "calendar"))
            })
            .filter(|r| {
                r.prop("supported-calendar-component-set")
                    .is_none_or(|p| p.comps.is_empty() || p.comps.iter().any(|c| c == "VEVENT"))
            })
            .filter_map(|r| {
                let url = self.absolute(&at, &r.href)?;
                let remote_id = path(&r.href).to_owned();
                let name = r.text("displayname").map_or_else(
                    || {
                        remote_id
                            .trim_end_matches('/')
                            .rsplit('/')
                            .next()
                            .unwrap_or_default()
                            .to_owned()
                    },
                    str::to_owned,
                );
                let write = r.prop("current-user-privilege-set").is_none_or(|p| {
                    p.inside
                        .iter()
                        .any(|n| matches!(n.as_str(), "write" | "write-content" | "all"))
                });
                let marker = r
                    .text("getctag")
                    .or_else(|| r.text("sync-token"))
                    .map(str::to_owned);
                Some((
                    NewCalendar {
                        remote_id,
                        name,
                        color: hex_color(r.text("calendar-color").unwrap_or_default()),
                        access: if write {
                            CalendarAccess::Owner
                        } else {
                            CalendarAccess::Reader
                        },
                        is_primary: false,
                        time_zone: String::new(),
                    },
                    url,
                    marker,
                ))
            })
            .collect();

        *self.urls.lock().unwrap() = calendars
            .iter()
            .map(|(calendar, url, _)| (calendar.remote_id.clone(), url.clone()))
            .collect();
        let mut changed = false;
        let before = store.calendars()?;
        let mut ids = Vec::new();
        for (position, (calendar, _, _)) in calendars.iter().enumerate() {
            let id = store.upsert_calendar(
                Some(account),
                CalendarSource::CalDav,
                calendar,
                position as i64,
            )?;
            changed |= before.iter().find(|c| c.id == id).is_none_or(|old| {
                old.name != calendar.name
                    || old.color != calendar.color
                    || old.access != calendar.access
                    || old.position != position as i64
            });
            ids.push(id);
        }
        let keep: Vec<String> = calendars.iter().map(|c| c.0.remote_id.clone()).collect();
        changed |= store.remove_calendars_except(account, &keep)? > 0;

        for ((calendar, url, marker), id) in calendars.iter().zip(ids) {
            let stored = store.calendar(id)?.and_then(|c| c.sync_token);
            if marker.is_some() && stored == *marker {
                continue;
            }
            match self.sync_calendar(store, id, url, address).await {
                Ok(c) => {
                    changed |= c;
                    store.set_calendar_sync_token(id, marker.as_deref())?;
                }
                Err(CalendarError::Failed(err)) => {
                    tracing::warn!(calendar = calendar.name, %err, "calendar not synced");
                }
                Err(err) => return Err(err),
            }
        }
        Ok(changed)
    }

    /// Downloads the events of the calendar at `url` whose etag changed,
    /// and forgets those it no longer has.
    async fn sync_calendar(
        &self,
        store: &mut Store,
        id: i64,
        url: &str,
        address: &str,
    ) -> SyncResult {
        let Some((at, listed)) = self.dav("REPORT", url, "1", ETAGS).await? else {
            return Err(CalendarError::Failed(Error::Rejected(
                "the CalDAV server can't list events".into(),
            )));
        };
        let own = path(url).trim_end_matches('/').to_owned();
        let remote: HashMap<String, String> = listed
            .iter()
            .filter(|r| path(&r.href).trim_end_matches('/') != own)
            .filter_map(|r| Some((path(&r.href).to_owned(), r.text("getetag")?.to_owned())))
            .collect();
        let have: HashMap<String, Option<String>> = store.event_etags(id)?.into_iter().collect();
        let wanted: Vec<&String> = remote
            .iter()
            .filter(|(href, etag)| have.get(*href).and_then(Option::as_ref) != Some(*etag))
            .map(|(href, _)| href)
            .collect();
        let mut writes: Vec<(String, Vec<EventData>)> = have
            .keys()
            .filter(|href| !remote.contains_key(*href))
            .map(|href| (href.clone(), Vec::new()))
            .collect();
        let zone = TimeZone::system();
        let mut got = HashSet::new();
        for batch in wanted.chunks(MULTIGET) {
            let hrefs: String = batch
                .iter()
                .map(|h| format!("<d:href>{}</d:href>", xml_escape(h)))
                .collect();
            let body = format!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<c:calendar-multiget xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
<d:prop><d:getetag/><c:calendar-data/></d:prop>{hrefs}</c:calendar-multiget>"#
            );
            let Some((_, responses)) = self.dav("REPORT", &at, "1", &body).await? else {
                return Err(CalendarError::Failed(Error::Rejected(
                    "the CalDAV server can't send events".into(),
                )));
            };
            for response in responses {
                let href = path(&response.href).to_owned();
                let Some(data) = response.text("calendar-data") else {
                    continue;
                };
                let etag = response
                    .text("getetag")
                    .map(str::to_owned)
                    .or_else(|| remote.get(&href).cloned());
                let events = ical::parse_events(data, &zone, address)
                    .into_iter()
                    .map(|mut event| {
                        event.remote_id = href.clone();
                        event.etag = etag.clone();
                        if event.uid.is_empty() {
                            event.uid = href.clone();
                        }
                        event
                    })
                    .collect();
                got.insert(href.clone());
                writes.push((href, events));
            }
        }
        let missing = wanted.iter().filter(|h| !got.contains(**h)).count();
        if missing > 0 {
            tracing::debug!(missing, "CalDAV sent fewer events than asked");
        }
        if writes.is_empty() {
            return Ok(false);
        }
        store.replace_events_batch(id, &writes)?;
        Ok(true)
    }
}

mod write;

#[cfg(test)]
mod tests;
