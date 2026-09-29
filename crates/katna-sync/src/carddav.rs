// SPDX-License-Identifier: GPL-3.0-or-later

//! CardDAV (RFC 6352) address books: found from the account's address
//! (RFC 6764 `.well-known`), read with `sync-collection` (RFC 6578) so a
//! sync fetches only what changed, and `addressbook-multiget` for the cards.
//! Servers without `sync-collection` are listed with `PROPFIND` and only
//! cards whose ETag changed are fetched.

use std::collections::HashMap;
use std::time::Duration;

use base64::Engine as _;
use katna_core::contact::Card;
use katna_dav::vcard;
use katna_store::{BookSync, SyncedContact, SyncedGroup};

use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    methods::{self, Dav},
    net::Tls,
};

/// How long one request may take.
const TIMEOUT: Duration = Duration::from_secs(60);

/// Largest answer read: a listing of thousands of cards, or a batch of
/// cards with pictures.
const MAX_ANSWER: usize = 32 * 1024 * 1024;

/// Cards fetched in one `addressbook-multiget`.
const BATCH: usize = 50;

/// Redirects followed from a `.well-known` address.
const REDIRECTS: usize = 5;

const DAV: &str = "DAV:";
const CARD: &str = "urn:ietf:params:xml:ns:carddav";

/// An address book on the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    /// Its whole URL, ending in `/`.
    pub url: String,
    pub name: String,
}

/// One account on a CardDAV server.
#[derive(Clone)]
pub struct CardDav {
    authorization: String,
    tls: Tls,
}

/// Where to look for an account's address books: a URL the user or a
/// test gave, then the provider's known server and the `.well-known`
/// addresses of the mail and IMAP domains
/// ([`crate::methods::dav_start_urls`]).
pub fn start_urls(configured: Option<&str>, address: &str, imap_host: Option<&str>) -> Vec<String> {
    let mut urls: Vec<String> = configured
        .map(str::trim)
        .filter(|u| !u.is_empty())
        .map(str::to_owned)
        .into_iter()
        .collect();
    for url in methods::dav_start_urls(Dav::Card, address, imap_host) {
        if !urls.contains(&url) {
            urls.push(url);
        }
    }
    urls
}

/// The parts of a URL: `scheme://authority` and the path.
fn split_url(url: &str) -> Option<(&str, &str)> {
    let after = url.find("://")? + 3;
    match url[after..].find('/') {
        Some(slash) => Some((&url[..after + slash], &url[after + slash..])),
        None => Some((url, "/")),
    }
}

/// `href` from a listing, made whole against `base`.
fn absolute(base: &str, href: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        return href.to_owned();
    }
    let (origin, path) = split_url(base).unwrap_or((base, "/"));
    if href.starts_with('/') {
        return format!("{origin}{href}");
    }
    let dir = &path[..path.rfind('/').map_or(0, |i| i + 1)];
    format!("{origin}{dir}{href}")
}

/// The path of a URL, which is how cards are named in listings.
fn path_of(url: &str) -> String {
    split_url(url).map_or(url, |(_, p)| p).to_owned()
}

/// One `<response>` of a multistatus answer.
#[derive(Debug, Default)]
struct Response {
    href: String,
    /// The response's own status (a removed card in `sync-collection`).
    gone: bool,
    etag: Option<String>,
    display_name: String,
    address_book: bool,
    principal: Option<String>,
    home: Option<String>,
    data: Option<String>,
}

fn multistatus(body: &[u8]) -> Result<(Vec<Response>, Option<String>)> {
    let text = String::from_utf8_lossy(body);
    let doc = roxmltree::Document::parse(&text)
        .map_err(|e| Error::Protocol(format!("CardDAV: bad XML: {e}")))?;
    let mut out = Vec::new();
    let mut token = None;
    let root = doc.root_element();
    for node in root.children().filter(|n| n.is_element()) {
        let name = node.tag_name();
        if name.namespace() == Some(DAV) && name.name() == "sync-token" {
            token = node.text().map(|t| t.trim().to_owned());
        }
        if name.namespace() != Some(DAV) || name.name() != "response" {
            continue;
        }
        let mut response = Response::default();
        for child in node.children().filter(|n| n.is_element()) {
            match (child.tag_name().namespace(), child.tag_name().name()) {
                (Some(DAV), "href") => {
                    response.href = child.text().unwrap_or_default().trim().to_owned();
                }
                (Some(DAV), "status") => {
                    response.gone = child.text().unwrap_or_default().contains(" 404");
                }
                (Some(DAV), "propstat") => {
                    let ok = child
                        .children()
                        .find(|n| n.has_tag_name((DAV, "status")))
                        .and_then(|n| n.text())
                        .is_none_or(|s| s.contains(" 200"));
                    if !ok {
                        continue;
                    }
                    let Some(prop) = child.children().find(|n| n.has_tag_name((DAV, "prop")))
                    else {
                        continue;
                    };
                    for p in prop.children().filter(|n| n.is_element()) {
                        let href = || {
                            p.children()
                                .find(|n| n.has_tag_name((DAV, "href")))
                                .and_then(|n| n.text())
                                .map(|t| t.trim().to_owned())
                        };
                        match (p.tag_name().namespace(), p.tag_name().name()) {
                            (Some(DAV), "getetag") => {
                                response.etag = p.text().map(|t| t.trim().to_owned());
                            }
                            (Some(DAV), "displayname") => {
                                response.display_name =
                                    p.text().unwrap_or_default().trim().to_owned();
                            }
                            (Some(DAV), "resourcetype") => {
                                response.address_book =
                                    p.children().any(|n| n.has_tag_name((CARD, "addressbook")));
                            }
                            (Some(DAV), "current-user-principal") => response.principal = href(),
                            (Some(CARD), "addressbook-home-set") => response.home = href(),
                            (Some(CARD), "address-data") => {
                                response.data = p.text().map(str::to_owned);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        out.push(response);
    }
    Ok((out, token))
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

impl CardDav {
    pub fn new(user: &str, password: &str, tls: Tls) -> Self {
        let credentials =
            base64::engine::general_purpose::STANDARD.encode(format!("{user}:{password}"));
        Self {
            authorization: format!("Basic {credentials}"),
            tls,
        }
    }

    /// Signs in with an OAuth access token, as Google's CardDAV server asks.
    pub fn bearer(token: &str, tls: Tls) -> Self {
        Self {
            authorization: format!("Bearer {token}"),
            tls,
        }
    }

    async fn send(&self, method: &str, url: &str, depth: &str, body: &str) -> Result<Reply> {
        http::exchange_limited(
            method,
            url,
            &[("Authorization", &self.authorization), ("Depth", depth)],
            Some(("application/xml; charset=utf-8", body.as_bytes())),
            None,
            &self.tls,
            TIMEOUT,
            MAX_ANSWER,
        )
        .await
    }

    /// Sends a `PROPFIND`, following redirects; returns the listing and
    /// the URL that answered.
    async fn propfind(
        &self,
        url: &str,
        depth: &str,
        props: &str,
    ) -> Result<(Vec<Response>, String)> {
        let body = format!(
            r#"<?xml version="1.0" encoding="utf-8"?><d:propfind xmlns:d="DAV:" xmlns:card="{CARD}"><d:prop>{props}</d:prop></d:propfind>"#
        );
        let mut url = url.to_owned();
        for _ in 0..=REDIRECTS {
            let reply = self.send("PROPFIND", &url, depth, &body).await?;
            match reply.status {
                207 => return Ok((multistatus(&reply.body)?.0, url)),
                301 | 302 | 303 | 307 | 308 => {
                    let Some(to) = reply.location else { break };
                    let next = absolute(&url, &to);
                    // Never from https down to http, except on the loopback.
                    if url.starts_with("https://") && !next.starts_with("https://") {
                        break;
                    }
                    url = next;
                }
                401 => {
                    return Err(Error::Auth(format!(
                        "CardDAV refused the password at {url}"
                    )));
                }
                status => return Err(Error::Rejected(format!("CardDAV {url}: status {status}"))),
            }
        }
        Err(Error::Rejected(format!(
            "CardDAV {url}: too many redirects"
        )))
    }

    /// The address books reachable from the first of `starts` that leads
    /// to any. A wrong password is [`Error::Auth`].
    pub async fn discover(&self, starts: &[String]) -> Result<Vec<Collection>> {
        let mut last = None;
        for start in starts {
            match self.discover_from(start).await {
                Ok(found) if !found.is_empty() => return Ok(found),
                Ok(_) => {}
                Err(e @ Error::Auth(_)) => return Err(e),
                Err(e) => {
                    tracing::debug!(%start, error = %e, "contacts: no CardDAV here");
                    last = Some(e);
                }
            }
        }
        match last {
            Some(e) if starts.len() == 1 => Err(e),
            _ => Ok(Vec::new()),
        }
    }

    async fn discover_from(&self, start: &str) -> Result<Vec<Collection>> {
        let (found, at) = self
            .propfind(start, "0", "<d:current-user-principal/><d:resourcetype/>")
            .await?;
        let first = found.into_iter().next().unwrap_or_default();
        let mut homes = Vec::new();
        if let Some(principal) = first.principal {
            let principal = absolute(&at, &principal);
            let (found, at) = self
                .propfind(&principal, "0", "<card:addressbook-home-set/>")
                .await?;
            homes.extend(
                found
                    .into_iter()
                    .filter_map(|r| r.home)
                    .map(|home| absolute(&at, &home)),
            );
        }
        if homes.is_empty() {
            // A server that answers with the book or home itself.
            homes.push(at);
        }
        let mut books = Vec::new();
        for home in homes {
            let (found, at) = self
                .propfind(&home, "1", "<d:resourcetype/><d:displayname/>")
                .await?;
            for r in found.into_iter().filter(|r| r.address_book) {
                let mut url = absolute(&at, &r.href);
                if !url.ends_with('/') {
                    url.push('/');
                }
                if books.iter().any(|b: &Collection| b.url == url) {
                    continue;
                }
                let name = if r.display_name.is_empty() {
                    url.trim_end_matches('/')
                        .rsplit('/')
                        .next()
                        .unwrap_or_default()
                        .to_owned()
                } else {
                    r.display_name
                };
                books.push(Collection { url, name });
            }
        }
        Ok(books)
    }

    /// Reads what changed in the book at `url` since `token`, or all of
    /// it without one. `known` holds the ETags of the cards already kept,
    /// by href, so unchanged cards are not fetched again.
    pub async fn sync(
        &self,
        url: &str,
        token: Option<&str>,
        known: &HashMap<String, String>,
    ) -> Result<BookSync> {
        match self.sync_collection(url, token).await? {
            Some(Ok(listing)) => self.fetch(url, listing, known).await,
            // The server forgot the token: everything again.
            Some(Err(())) if token.is_some() => match self.sync_collection(url, None).await? {
                Some(Ok(listing)) => self.fetch(url, listing, known).await,
                _ => self.listing(url, known).await,
            },
            _ => self.listing(url, known).await,
        }
    }

    /// `Ok(None)` when the server has no `sync-collection`; `Err(())`
    /// inside when it no longer knows `token`.
    async fn sync_collection(
        &self,
        url: &str,
        token: Option<&str>,
    ) -> Result<Option<std::result::Result<Listing, ()>>> {
        let body = format!(
            r#"<?xml version="1.0" encoding="utf-8"?><d:sync-collection xmlns:d="DAV:"><d:sync-token>{}</d:sync-token><d:sync-level>1</d:sync-level><d:prop><d:getetag/></d:prop></d:sync-collection>"#,
            escape_xml(token.unwrap_or_default())
        );
        let reply = self.send("REPORT", url, "0", &body).await?;
        match reply.status {
            207 => {
                let (responses, sync_token) = multistatus(&reply.body)?;
                let own = path_of(url);
                let mut listing = Listing {
                    full: token.is_none(),
                    sync_token,
                    ..Listing::default()
                };
                for r in responses {
                    let href = path_of(&absolute(url, &r.href));
                    if href.trim_end_matches('/') == own.trim_end_matches('/') {
                        continue;
                    }
                    if r.gone {
                        listing.gone.push(href);
                    } else {
                        listing.cards.push((href, r.etag));
                    }
                }
                Ok(Some(Ok(listing)))
            }
            401 => Err(Error::Auth(format!(
                "CardDAV refused the password at {url}"
            ))),
            403 | 409 if String::from_utf8_lossy(&reply.body).contains("valid-sync-token") => {
                Ok(Some(Err(())))
            }
            _ => Ok(None),
        }
    }

    /// Every card of the book by `PROPFIND`, for servers without
    /// `sync-collection`.
    async fn listing(&self, url: &str, known: &HashMap<String, String>) -> Result<BookSync> {
        let (responses, at) = self
            .propfind(url, "1", "<d:getetag/><d:resourcetype/>")
            .await?;
        let own = path_of(&at);
        let cards = responses
            .into_iter()
            .filter(|r| !r.address_book)
            .map(|r| (path_of(&absolute(&at, &r.href)), r.etag))
            .filter(|(href, _)| href.trim_end_matches('/') != own.trim_end_matches('/'))
            .collect();
        let listing = Listing {
            full: true,
            cards,
            ..Listing::default()
        };
        self.fetch(url, listing, known).await
    }

    /// Fetches the listed cards whose ETag is new, in batches.
    async fn fetch(
        &self,
        url: &str,
        listing: Listing,
        known: &HashMap<String, String>,
    ) -> Result<BookSync> {
        let mut out = BookSync {
            full: listing.full,
            deleted: listing.gone,
            sync_token: listing.sync_token,
            ..BookSync::default()
        };
        let mut wanted = Vec::new();
        for (href, etag) in listing.cards {
            match (&etag, known.get(&href)) {
                (Some(new), Some(old)) if new == old => out.unchanged.push(href),
                _ => wanted.push(href),
            }
        }
        let mut parsed: Vec<(String, Option<String>, String, vcard::Parsed)> = Vec::new();
        for batch in wanted.chunks(BATCH) {
            let hrefs: String = batch
                .iter()
                .map(|h| format!("<d:href>{}</d:href>", escape_xml(h)))
                .collect();
            let body = format!(
                r#"<?xml version="1.0" encoding="utf-8"?><card:addressbook-multiget xmlns:d="DAV:" xmlns:card="{CARD}"><d:prop><d:getetag/><card:address-data/></d:prop>{hrefs}</card:addressbook-multiget>"#
            );
            let reply = self.send("REPORT", url, "1", &body).await?;
            match reply.status {
                207 => {}
                401 => {
                    return Err(Error::Auth(format!(
                        "CardDAV refused the password at {url}"
                    )));
                }
                status => {
                    return Err(Error::Rejected(format!(
                        "CardDAV {url}: reading cards: status {status}"
                    )));
                }
            }
            for r in multistatus(&reply.body)?.0 {
                let href = path_of(&absolute(url, &r.href));
                if r.gone {
                    // Removed between the listing and now.
                    out.deleted.push(href);
                    continue;
                }
                let Some(text) = r.data else { continue };
                if let Some(card) = vcard::parse(&text).into_iter().next() {
                    parsed.push((href, r.etag, text, card));
                }
            }
        }

        // Group cards (Apple's) are labels on their members.
        let mut member_of: HashMap<String, Vec<String>> = HashMap::new();
        let mut groups: Vec<SyncedGroup> = Vec::new();
        for (_, _, _, p) in parsed.iter().filter(|(.., p)| p.group) {
            let id = format!("group:{}", p.uid);
            groups.push(SyncedGroup {
                remote_id: id.clone(),
                name: p.card.display_name(),
            });
            for member in &p.members {
                member_of
                    .entry(member.clone())
                    .or_default()
                    .push(id.clone());
            }
        }
        for (href, etag, text, p) in parsed.into_iter().filter(|(.., p)| !p.group) {
            let mut labels = p.categories.clone();
            if let Some(extra) = member_of.get(&p.uid) {
                labels.extend(extra.iter().cloned());
            }
            out.contacts.push(SyncedContact {
                remote_id: href,
                etag,
                card: p.card,
                raw: Some(text),
                starred: false,
                groups: labels,
                photo: p.photo,
            });
        }
        if !groups.is_empty() {
            out.groups = Some(groups);
        }
        Ok(out)
    }
}

impl CardDav {
    /// Saves `card` in collection `url`: a new card when `href` is `None`,
    /// else over `href`, only if it is still at `etag` (a change made
    /// elsewhere since is not overwritten). `old` is the card's vCard as
    /// read, so properties Katna does not show are kept. Returns the card
    /// as the server keeps it.
    pub async fn save(
        &self,
        url: &str,
        href: Option<&str>,
        etag: Option<&str>,
        old: Option<&str>,
        categories: &[String],
        card: &Card,
    ) -> Result<SyncedContact> {
        let (target, uid) = match href {
            Some(href) => {
                let uid = old
                    .and_then(|old| vcard::parse(old).into_iter().next())
                    .map(|p| p.uid)
                    .filter(|uid| !uid.is_empty())
                    .unwrap_or_else(new_uid);
                (absolute(url, href), uid)
            }
            None => {
                let uid = new_uid();
                (format!("{}/{uid}.vcf", url.trim_end_matches('/')), uid)
            }
        };
        let text = vcard::write(&uid, card, categories, old);
        let mut headers = vec![("Authorization", self.authorization.as_str())];
        match (href, etag) {
            (Some(_), Some(etag)) => headers.push(("If-Match", etag)),
            (None, _) => headers.push(("If-None-Match", "*")),
            (Some(_), None) => {}
        }
        let reply = http::exchange_limited(
            "PUT",
            &target,
            &headers,
            Some(("text/vcard; charset=utf-8", text.as_bytes())),
            None,
            &self.tls,
            TIMEOUT,
            MAX_ANSWER,
        )
        .await?;
        write_status(&reply, &target, "saving a card")?;
        // The server may change the card (a new ETag at least): read it
        // back as it keeps it.
        let path = path_of(&target);
        let listing = Listing {
            full: false,
            cards: vec![(path.clone(), None)],
            ..Listing::default()
        };
        let read = self.fetch(url, listing, &HashMap::new()).await?;
        read.contacts
            .into_iter()
            .find(|c| c.remote_id == path)
            .ok_or_else(|| Error::Protocol(format!("CardDAV {target}: saved card not found")))
    }

    /// Deletes card `href` of collection `url`, only if it is still at
    /// `etag`. One already gone is fine.
    pub async fn delete(&self, url: &str, href: &str, etag: Option<&str>) -> Result<()> {
        let target = absolute(url, href);
        let mut headers = vec![("Authorization", self.authorization.as_str())];
        if let Some(etag) = etag {
            headers.push(("If-Match", etag));
        }
        let reply = http::exchange_limited(
            "DELETE", &target, &headers, None, None, &self.tls, TIMEOUT, MAX_ANSWER,
        )
        .await?;
        if reply.status == 404 {
            return Ok(());
        }
        write_status(&reply, &target, "deleting a card")
    }
}

/// A `2xx` answer to a write is fine; a changed card is said so.
fn write_status(reply: &Reply, url: &str, doing: &str) -> Result<()> {
    match reply.status {
        200..=299 => Ok(()),
        401 => Err(Error::Auth(format!(
            "CardDAV refused the password at {url}"
        ))),
        412 => Err(Error::Rejected(format!(
            "CardDAV {url}: the card was changed elsewhere; try again after the next sync"
        ))),
        status => Err(Error::Rejected(format!(
            "CardDAV {url}: {doing}: status {status}"
        ))),
    }
}

/// A new card's UID, random like a UUID.
pub fn new_uid() -> String {
    use ring::rand::SecureRandom;
    let mut bytes = [0u8; 16];
    if ring::rand::SystemRandom::new().fill(&mut bytes).is_err() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        bytes = nanos.to_le_bytes();
    }
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    )
}

/// What a listing found before the cards are fetched.
#[derive(Debug, Default)]
struct Listing {
    full: bool,
    /// Hrefs (paths) with their ETags.
    cards: Vec<(String, Option<String>)>,
    gone: Vec<String>,
    sync_token: Option<String>,
}

#[cfg(test)]
mod tests;
