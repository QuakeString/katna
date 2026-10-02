// SPDX-License-Identifier: GPL-3.0-or-later

//! Images the reading pane asks the daemon for, since the app never uses
//! the network itself (`docs/ARCHITECTURE.md` §12):
//!
//! - [`Pictures::image`]: a remote image in a message, once the user chose
//!   to show that message's images.
//! - [`Pictures::sender`]: the picture of a sender's organization: its BIMI
//!   logo (the `default._bimi` DNS record), or else its website's icon (the
//!   largest one its home page names, then the usual file names).
//!   Addresses at free-mail providers get none, since the provider's logo
//!   says nothing about the person. Answers, including "none", are kept in
//!   the cache directory for a week.
//!
//! A sender picture is looked up by the sender's organizational domain
//! ([`organizational_domain`]) only, and kept under it, so mail from
//! `x@<unique-id>.tracker.example` causes no lookup of its own: the
//! sender learns nothing per message or per recipient from it beyond
//! one lookup of the organization a week. Only URLs on that domain or its
//! subdomains are followed (a BIMI `l=` or an icon a home page names).
//! The daemon asks only for senders whose mail the user's provider
//! authenticated (`katna_sync::auth_results`).
//!
//! Only `https` on port 443 is used, to public addresses only
//! ([`crate::net::Reach::Public`]), bodies are capped, and anything that is
//! not an image by its first bytes is refused.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use katna_core::image::ImageKind;

use crate::autoconfig::{dns, http};
use crate::net::Tls;
use crate::{Error, Result};

mod company;
pub use company::Company;

/// Largest remote image fetched for a message.
pub const MAX_IMAGE: usize = 8 * 1024 * 1024;
/// Largest sender picture: BIMI allows 32 KB; icons are small.
const MAX_PICTURE: usize = 256 * 1024;
/// How much of a home page is read for the icons its head names.
const MAX_PAGE: usize = 512 * 1024;
/// How long a sender picture (or its absence) is kept.
const PICTURE_TTL: Duration = Duration::from_secs(7 * 24 * 3600);

/// Domains shared by many unrelated people: their logo is not the sender's.
const FREE_MAIL: &[&str] = &[
    "aol.com",
    "fastmail.com",
    "gmail.com",
    "gmx.com",
    "gmx.de",
    "gmx.net",
    "googlemail.com",
    "hey.com",
    "hotmail.com",
    "icloud.com",
    "live.com",
    "mac.com",
    "mail.com",
    "mail.ru",
    "me.com",
    "msn.com",
    "outlook.com",
    "pm.me",
    "proton.me",
    "protonmail.com",
    "qq.com",
    "tutanota.com",
    "web.de",
    "yahoo.com",
    "yandex.com",
    "yandex.ru",
    "zoho.com",
    "zohomail.com",
];

pub struct Pictures {
    tls: Tls,
    resolvers: Vec<SocketAddr>,
    timeout: Duration,
    /// Where sender pictures are kept.
    cache: PathBuf,
}

impl Pictures {
    /// The system's resolvers and trust store; sender pictures are kept in
    /// `cache_dir/pictures`.
    pub fn system(cache_dir: &Path) -> Result<Self> {
        Ok(Self {
            tls: Tls::system()?,
            resolvers: dns::system_resolvers(),
            timeout: Duration::from_secs(6),
            cache: Self::cache_dir(cache_dir),
        })
    }

    /// Where [`system`](Self::system) keeps sender pictures.
    pub fn cache_dir(cache_dir: &Path) -> PathBuf {
        cache_dir.join("pictures")
    }

    /// Fetches the image at `url` (`http` is upgraded to `https`).
    pub async fn image(&self, url: &str) -> Result<Vec<u8>> {
        let url = https(url).ok_or_else(|| Error::Protocol(format!("{url}: not a web URL")))?;
        let body = http::get_public(&url, &self.tls, self.timeout * 2, MAX_IMAGE)
            .await?
            .ok_or_else(|| Error::Protocol(format!("{url}: not found")))?;
        if ImageKind::sniff(&body).is_none() {
            return Err(Error::Protocol(format!("{url}: not an image")));
        }
        Ok(body)
    }

    /// The picture for mail from `address`, or empty when there is none.
    /// The caller checks that the sender is authenticated.
    pub async fn sender(&self, address: &str) -> Vec<u8> {
        let Some(domain) = address
            .rsplit_once('@')
            .map(|(_, d)| d.trim().trim_end_matches('.').to_ascii_lowercase())
            .filter(|d| valid_domain(d))
        else {
            return Vec::new();
        };
        let org = organizational_domain(&domain);
        if FREE_MAIL.contains(&org.as_str()) || !valid_domain(&org) {
            return Vec::new();
        }
        // Kept under the organizational domain. `.picture`: answers kept
        // under the full domain, and found by following URLs to any host
        // (`.pic`), are asked again.
        let cached = self.cache.join(format!("{org}.picture"));
        if let Some(bytes) = read_fresh(&cached) {
            return bytes;
        }
        let mut reached = false;
        let picture = self.find(&org, &mut reached).await.unwrap_or_default();
        // Offline is not "no picture": ask again next time.
        if !reached {
            return picture;
        }
        if let Err(err) =
            std::fs::create_dir_all(&self.cache).and_then(|()| std::fs::write(&cached, &picture))
        {
            tracing::debug!(%err, "cannot keep the sender picture");
        }
        picture
    }

    /// The picture of the organizational domain `org`. Sets `reached`
    /// when a web server answered.
    async fn find(&self, org: &str, reached: &mut bool) -> Option<Vec<u8>> {
        let records = dns::txt(
            &self.resolvers,
            &format!("default._bimi.{org}"),
            self.timeout,
        )
        .await;
        if let Some(url) = records
            .iter()
            .find_map(|r| bimi_logo(r))
            .filter(|url| within(url, org))
            && let Some(svg) = self.fetch(&url, MAX_PICTURE, reached).await
            && ImageKind::sniff(&svg) == Some(ImageKind::Svg)
        {
            tracing::debug!(org, url, "BIMI logo");
            return Some(svg);
        }
        let mut urls = Vec::new();
        for host in [org.to_owned(), format!("www.{org}")] {
            let Some(page) = self.fetch_head(&format!("https://{host}/"), reached).await else {
                continue;
            };
            urls = page_icons(&String::from_utf8_lossy(&page), &host);
            urls.retain(|url| within(url, org));
            tracing::debug!(host, icons = ?urls, "home page");
            break;
        }
        urls.extend([
            format!("https://{org}/apple-touch-icon.png"),
            format!("https://www.{org}/apple-touch-icon.png"),
            format!("https://{org}/favicon.ico"),
            format!("https://www.{org}/favicon.ico"),
        ]);
        urls.dedup();
        for url in urls {
            if let Some(icon) = self.fetch(&url, MAX_PICTURE, reached).await
                && ImageKind::sniff(&icon).is_some()
            {
                tracing::debug!(org, url, "site icon");
                return Some(icon);
            }
        }
        None
    }

    /// The start of the page at `url`, up to the end of its `<head>`.
    async fn fetch_head(&self, url: &str, reached: &mut bool) -> Option<Vec<u8>> {
        match http::get_head_public(url, &self.tls, self.timeout, MAX_PAGE).await {
            Ok(page) => {
                *reached = true;
                page
            }
            Err(err) => {
                // A site whose certificate is refused answered all the
                // same: not asked again until the answer is old.
                if matches!(err, Error::Tls(_)) {
                    *reached = true;
                }
                tracing::debug!(url, %err, "page not read");
                None
            }
        }
    }

    async fn fetch(&self, url: &str, max: usize, reached: &mut bool) -> Option<Vec<u8>> {
        match http::get_public(url, &self.tls, self.timeout, max).await {
            Ok(body) => {
                *reached = true;
                body
            }
            Err(err) => {
                // A site whose certificate is refused answered all the
                // same: not asked again until the answer is old.
                if matches!(err, Error::Tls(_)) {
                    *reached = true;
                }
                tracing::debug!(url, %err, "picture not fetched");
                None
            }
        }
    }
}

/// The icons a home page on `host` names in its `<link rel=…icon…>` tags,
/// as `https` URLs, largest first.
fn page_icons(html: &str, host: &str) -> Vec<String> {
    let lower = html.to_ascii_lowercase();
    let mut icons: Vec<(u32, String)> = Vec::new();
    let mut at = 0;
    while let Some(start) = lower[at..].find("<link").map(|i| at + i) {
        let end = lower[start..].find('>').map_or(lower.len(), |i| start + i);
        at = end;
        let attrs = attributes(&html[start + 5..end]);
        let get = |name: &str| {
            attrs
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, v)| v.as_str())
        };
        let rel = get("rel").unwrap_or_default().to_ascii_lowercase();
        let Some(href) = get("href").map(|h| h.trim().replace("&amp;", "&")) else {
            continue;
        };
        let apple = rel.contains("apple-touch-icon");
        if !(apple || rel.split_whitespace().any(|r| r == "icon")) || href.is_empty() {
            continue;
        }
        let url = if let Some(rest) = href.strip_prefix("//") {
            format!("https://{rest}")
        } else if href.contains("://") {
            match https(&href) {
                Some(url) => url,
                None => continue,
            }
        } else if href
            .split('/')
            .next()
            .is_some_and(|first| first.contains(':'))
        {
            // `data:`, `javascript:` and other schemes.
            continue;
        } else {
            format!("https://{host}/{}", href.trim_start_matches('/'))
        };
        // The largest of `sizes` ("32x32 64x64", "any" for a drawing).
        let size = get("sizes")
            .unwrap_or_default()
            .split_whitespace()
            .filter_map(|s| {
                if s.eq_ignore_ascii_case("any") {
                    return Some(256);
                }
                s.split(['x', 'X']).next()?.parse::<u32>().ok()
            })
            .max()
            .unwrap_or(if apple {
                180
            } else if url.ends_with(".svg") {
                256
            } else {
                32
            });
        if !icons.iter().any(|(_, u)| *u == url) {
            icons.push((size, url));
        }
    }
    icons.sort_by_key(|icon| std::cmp::Reverse(icon.0));
    icons.into_iter().map(|(_, url)| url).collect()
}

/// The `name=value` pairs of a tag, names in lower case.
fn attributes(tag: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = tag.trim_start();
    while !rest.is_empty() {
        let name_end = rest
            .find(|c: char| c == '=' || c.is_whitespace() || c == '/')
            .unwrap_or(rest.len());
        let name = rest[..name_end].to_ascii_lowercase();
        rest = rest[name_end..].trim_start();
        let value = if let Some(after) = rest.strip_prefix('=') {
            let after = after.trim_start();
            let (value, next) = match after.chars().next() {
                Some(q @ ('"' | '\'')) => {
                    let body = &after[1..];
                    let close = body.find(q).unwrap_or(body.len());
                    (&body[..close], body.get(close + 1..).unwrap_or(""))
                }
                _ => {
                    let close = after.find(char::is_whitespace).unwrap_or(after.len());
                    (&after[..close], &after[close..])
                }
            };
            rest = next.trim_start();
            value.to_owned()
        } else {
            if name.is_empty() {
                // A stray `/` or other mark.
                rest = rest.get(1..).unwrap_or("").trim_start();
                continue;
            }
            String::new()
        };
        if !name.is_empty() {
            out.push((name, value));
        }
    }
    out
}

/// `url` as `https`, or `None` for anything but a web URL.
fn https(url: &str) -> Option<String> {
    let url = url.trim();
    let lower = url.to_ascii_lowercase();
    if lower.starts_with("https://") {
        Some(url.to_owned())
    } else if lower.starts_with("http://") {
        Some(format!("https://{}", &url[7..]))
    } else {
        None
    }
}

fn valid_domain(domain: &str) -> bool {
    domain.contains('.')
        && domain.len() <= 253
        && !domain.starts_with(['.', '-'])
        && domain
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
        && !domain.contains("..")
}

/// Second-level labels under which a two-letter country domain registers
/// names, as in `example.co.uk` or `shop.com.au`.
const SECOND_LEVELS: &[&str] = &[
    "ac", "co", "com", "edu", "gob", "gov", "govt", "ltd", "mil", "ne", "net", "nhs", "or", "org",
    "plc", "sch",
];

/// The organizational (registrable) domain: `marketing.example.co.uk` →
/// `example.co.uk`. A heuristic, not the Public Suffix List (which is not
/// in the tree): the last two labels, or the last three when the top-level
/// domain has two letters and the second-level label is one of
/// [`SECOND_LEVELS`]. It errs towards a shorter domain for other public
/// suffixes (`github.io` gives `github.io`), which only makes more senders
/// share one lookup.
pub fn organizational_domain(domain: &str) -> String {
    let domain = domain.trim_end_matches('.').to_ascii_lowercase();
    let labels: Vec<&str> = domain.split('.').collect();
    let n = labels.len();
    let keep = if n >= 3 && labels[n - 1].len() == 2 && SECOND_LEVELS.contains(&labels[n - 2]) {
        3
    } else {
        2
    };
    labels[n.saturating_sub(keep)..].join(".")
}

/// Whether the `https` URL `url` names `domain` or a host under it.
fn within(url: &str, domain: &str) -> bool {
    let Ok(parts) = http::parse_url(url) else {
        return false;
    };
    let host = parts.host.trim_end_matches('.').to_ascii_lowercase();
    host == domain
        || host
            .strip_suffix(domain)
            .is_some_and(|rest| rest.ends_with('.'))
}

/// The logo URL of a BIMI record (`v=BIMI1; l=https://…; a=…`).
fn bimi_logo(record: &str) -> Option<String> {
    let mut tags = record.split(';').map(str::trim);
    if !tags.next()?.eq_ignore_ascii_case("v=BIMI1") {
        return None;
    }
    tags.find_map(|tag| {
        let (name, value) = tag.split_once('=')?;
        (name.trim().eq_ignore_ascii_case("l") && value.trim().starts_with("https://"))
            .then(|| value.trim().to_owned())
    })
}

/// The file's bytes if it was written within [`PICTURE_TTL`].
fn read_fresh(path: &Path) -> Option<Vec<u8>> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    let age = SystemTime::now()
        .duration_since(modified)
        .unwrap_or_default();
    (age < PICTURE_TTL).then(|| std::fs::read(path).ok())?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn organizational_domains() {
        assert_eq!(
            organizational_domain("marketing.beekeeperstudio.io"),
            "beekeeperstudio.io"
        );
        assert_eq!(organizational_domain("linkedin.com"), "linkedin.com");
        assert_eq!(organizational_domain("mail.bbc.co.uk"), "bbc.co.uk");
        assert_eq!(organizational_domain("e.shop.com.au"), "shop.com.au");
        assert_eq!(organizational_domain("news.example.de"), "example.de");
        // Unique subdomains share their organization's lookup.
        assert_eq!(
            organizational_domain("u-8f3a91.tracker.example"),
            "tracker.example"
        );
        assert_eq!(organizational_domain("news.abc.de"), "abc.de");
        assert_eq!(organizational_domain("a.b.example.io"), "example.io");
        assert_eq!(organizational_domain("x.y.gov.in."), "y.gov.in");
        assert_eq!(organizational_domain("co.uk"), "co.uk");
    }

    #[test]
    fn only_urls_on_the_domain_are_followed() {
        assert!(within("https://e.test/logo.svg", "e.test"));
        assert!(within("https://CDN.e.test./a.png", "e.test"));
        assert!(within("https://cdn.e.test:443/a.png", "e.test"));
        assert!(!within("https://tracker.example/e.test/logo.svg", "e.test"));
        assert!(!within("https://evile.test/logo.svg", "e.test"));
        assert!(!within("https://e.test.evil.example/", "e.test"));
        assert!(!within("http://e.test/", "e.test"));
    }

    #[test]
    fn images_only_come_from_public_addresses_on_port_443() {
        let dir = tempfile::tempdir().unwrap();
        let pictures = Pictures {
            tls: Tls::system().unwrap(),
            resolvers: Vec::new(),
            timeout: Duration::from_secs(2),
            cache: dir.path().join("pictures"),
        };
        for url in [
            "https://127.0.0.1/a.png",
            "http://localhost/a.png",
            "https://10.1.2.3/a.png",
            "https://169.254.169.254/latest/meta-data/",
            "https://100.64.0.1/a.png",
        ] {
            let err = futures_lite::future::block_on(pictures.image(url)).unwrap_err();
            assert!(err.to_string().contains("local network"), "{url}: {err}");
        }
        // IPv6 literals are not fetched at all.
        for url in ["https://[::1]/a.png", "https://[fd00::1]:443/a.png"] {
            assert!(futures_lite::future::block_on(pictures.image(url)).is_err());
        }
        let err = futures_lite::future::block_on(pictures.image("https://e.test:8443/a.png"))
            .unwrap_err();
        assert!(err.to_string().contains("port 443"), "{err}");
    }

    #[test]
    fn bimi_records() {
        assert_eq!(
            bimi_logo("v=BIMI1; l=https://e.test/logo.svg; a=https://e.test/vmc.pem"),
            Some("https://e.test/logo.svg".to_owned())
        );
        assert_eq!(bimi_logo("v=BIMI1; l=; a=;"), None);
        assert_eq!(bimi_logo("v=BIMI1; l=http://e.test/logo.svg"), None);
        assert_eq!(bimi_logo("v=spf1 -all"), None);
    }

    #[test]
    fn icons_named_by_a_home_page() {
        let page = r#"<html><head>
            <LINK rel="icon" href="/favicon-32.png" sizes="32x32">
            <link rel='shortcut icon' href=favicon.ico>
            <link rel="apple-touch-icon" href="https://cdn.e.test/touch.png?v=1&amp;x=2" />
            <link rel="icon" type="image/svg+xml" href="//cdn.e.test/logo.svg" sizes="any">
            <link rel="mask-icon" href="/mask.svg">
            <link rel="stylesheet" href="/site.css">
            <link rel="icon" sizes="192x192" href="http://e.test/big.png">
        </head></html>"#;
        assert_eq!(
            page_icons(page, "e.test"),
            [
                "https://cdn.e.test/logo.svg",
                "https://e.test/big.png",
                "https://cdn.e.test/touch.png?v=1&x=2",
                "https://e.test/favicon-32.png",
                "https://e.test/favicon.ico",
            ]
        );
        assert!(page_icons("<link rel=icon href='javascript:x'>", "e.test").is_empty());
    }

    #[test]
    fn domains_and_urls() {
        assert!(valid_domain("mail.example.org"));
        assert!(!valid_domain("../etc"));
        assert!(!valid_domain("localhost"));
        assert!(!valid_domain("a..b"));
        assert_eq!(
            https("http://e.test/a.png").as_deref(),
            Some("https://e.test/a.png")
        );
        assert_eq!(https("file:///etc/passwd"), None);
    }

    #[test]
    fn free_mail_gets_no_picture() {
        let dir = tempfile::tempdir().unwrap();
        let pictures = Pictures {
            tls: Tls::system().unwrap(),
            resolvers: Vec::new(),
            timeout: Duration::from_millis(1),
            cache: dir.path().join("pictures"),
        };
        let picture = futures_lite::future::block_on(pictures.sender("someone@gmail.com"));
        assert!(picture.is_empty());
        assert!(!dir.path().join("pictures").exists());
    }

    #[test]
    fn cached_answers_are_used() {
        let dir = tempfile::tempdir().unwrap();
        let cache = dir.path().join("pictures");
        std::fs::create_dir_all(&cache).unwrap();
        std::fs::write(cache.join("example.org.picture"), b"\x89PNG\r\n\x1a\n").unwrap();
        let pictures = Pictures {
            tls: Tls::system().unwrap(),
            resolvers: Vec::new(),
            timeout: Duration::from_millis(1),
            cache,
        };
        // Kept under the organizational domain, whatever the subdomain.
        for address in ["Info@News.Example.org", "x@u-123.example.org"] {
            let picture = futures_lite::future::block_on(pictures.sender(address));
            assert_eq!(picture, b"\x89PNG\r\n\x1a\n");
        }
    }
}
