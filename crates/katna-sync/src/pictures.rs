// SPDX-License-Identifier: GPL-3.0-or-later

//! Images the reading pane asks the daemon for, since the app never uses
//! the network itself (`docs/ARCHITECTURE.md` §12):
//!
//! - [`Pictures::image`]: a remote image in a message, once the user chose
//!   to show that message's images.
//! - [`Pictures::sender`]: the picture of a sender's organization: its BIMI
//!   logo (the `default._bimi` DNS record), or else its website's icon.
//!   Addresses at free-mail providers get none, since the provider's logo
//!   says nothing about the person. Answers, including "none", are kept in
//!   the cache directory for a week.
//!
//! Only `https` is used, bodies are capped, and anything that is not an
//! image by its first bytes is refused.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use katna_core::image::ImageKind;

use crate::autoconfig::{dns, http};
use crate::net::Tls;
use crate::{Error, Result};

/// Largest remote image fetched for a message.
pub const MAX_IMAGE: usize = 8 * 1024 * 1024;
/// Largest sender picture: BIMI allows 32 KB; icons are small.
const MAX_PICTURE: usize = 256 * 1024;
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
            cache: cache_dir.join("pictures"),
        })
    }

    /// Fetches the image at `url` (`http` is upgraded to `https`).
    pub async fn image(&self, url: &str) -> Result<Vec<u8>> {
        let url = https(url).ok_or_else(|| Error::Protocol(format!("{url}: not a web URL")))?;
        let body = http::get_limited(&url, &self.tls, self.timeout * 2, MAX_IMAGE)
            .await?
            .ok_or_else(|| Error::Protocol(format!("{url}: not found")))?;
        if ImageKind::sniff(&body).is_none() {
            return Err(Error::Protocol(format!("{url}: not an image")));
        }
        Ok(body)
    }

    /// The picture for mail from `address`, or empty when there is none.
    pub async fn sender(&self, address: &str) -> Vec<u8> {
        let Some(domain) = address
            .rsplit_once('@')
            .map(|(_, d)| d.trim().trim_end_matches('.').to_ascii_lowercase())
            .filter(|d| valid_domain(d))
        else {
            return Vec::new();
        };
        let org = organizational_domain(&domain);
        if FREE_MAIL.contains(&org.as_str()) {
            return Vec::new();
        }
        let cached = self.cache.join(format!("{domain}.img"));
        if let Some(bytes) = read_fresh(&cached) {
            return bytes;
        }
        let mut reached = false;
        let picture = self
            .find(&domain, &org, &mut reached)
            .await
            .unwrap_or_default();
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

    /// Sets `reached` when a web server answered.
    async fn find(&self, domain: &str, org: &str, reached: &mut bool) -> Option<Vec<u8>> {
        let mut names = vec![domain.to_owned()];
        if org != domain {
            names.push(org.to_owned());
        }
        for name in &names {
            let records = dns::txt(
                &self.resolvers,
                &format!("default._bimi.{name}"),
                self.timeout,
            )
            .await;
            if let Some(url) = records.iter().find_map(|r| bimi_logo(r)) {
                match self.fetch(&url, MAX_PICTURE, reached).await {
                    Some(svg) if ImageKind::sniff(&svg) == Some(ImageKind::Svg) => {
                        tracing::debug!(domain, url, "BIMI logo");
                        return Some(svg);
                    }
                    _ => break,
                }
            }
        }
        for url in [
            format!("https://{org}/apple-touch-icon.png"),
            format!("https://www.{org}/apple-touch-icon.png"),
            format!("https://{org}/favicon.ico"),
        ] {
            if let Some(icon) = self.fetch(&url, MAX_PICTURE, reached).await
                && ImageKind::sniff(&icon).is_some()
            {
                tracing::debug!(domain, url, "site icon");
                return Some(icon);
            }
        }
        None
    }

    async fn fetch(&self, url: &str, max: usize, reached: &mut bool) -> Option<Vec<u8>> {
        match http::get_limited(url, &self.tls, self.timeout, max).await {
            Ok(body) => {
                *reached = true;
                body
            }
            Err(err) => {
                tracing::debug!(url, %err, "picture not fetched");
                None
            }
        }
    }
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

/// The registered domain: `marketing.example.co.uk` → `example.co.uk`.
/// A heuristic, not the Public Suffix List: a two-letter top-level domain
/// with a short second level (`co.uk`, `com.au`) keeps three labels.
fn organizational_domain(domain: &str) -> String {
    let labels: Vec<&str> = domain.split('.').collect();
    let keep = match labels.as_slice() {
        [.., second, top] if top.len() == 2 && second.len() <= 3 && labels.len() >= 3 => 3,
        _ => 2,
    };
    labels[labels.len().saturating_sub(keep)..].join(".")
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
        std::fs::write(cache.join("news.example.org.img"), b"\x89PNG\r\n\x1a\n").unwrap();
        let pictures = Pictures {
            tls: Tls::system().unwrap(),
            resolvers: Vec::new(),
            timeout: Duration::from_millis(1),
            cache,
        };
        let picture = futures_lite::future::block_on(pictures.sender("Info@News.Example.org"));
        assert_eq!(picture, b"\x89PNG\r\n\x1a\n");
    }
}
