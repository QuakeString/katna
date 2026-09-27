// SPDX-License-Identifier: GPL-3.0-or-later

//! Who fetched a pixel or followed a link: a coarse label, decided while
//! the request is open. The address and user agent it looks at are never
//! stored.
//!
//! - Apple Mail Privacy Protection fetches every picture through Apple's
//!   proxy when mail is delivered, whether it is read or not, so an open
//!   from there only means "maybe opened".
//! - Security scanners (Microsoft Defender, Proofpoint, Mimecast and the
//!   like) fetch pictures and follow links within seconds of delivery.
//! - Gmail's and Outlook's picture proxies fetch when the person opens the
//!   mail, so they count as a person.

use std::net::IpAddr;
use std::time::Duration;

/// Coarse label of an event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// A person's mail program (possibly through Gmail's or Outlook's proxy).
    Person,
    /// Apple Mail Privacy Protection: maybe opened.
    AppleProxy,
    /// A security scanner or other automated fetch.
    Scanner,
}

impl Source {
    /// The label in the database and the event stream.
    pub fn as_str(self) -> &'static str {
        match self {
            Source::Person => "person",
            Source::AppleProxy => "apple_proxy",
            Source::Scanner => "scanner",
        }
    }

    /// Parses [`Source::as_str`].
    pub fn parse(label: &str) -> Option<Self> {
        match label {
            "person" => Some(Source::Person),
            "apple_proxy" => Some(Source::AppleProxy),
            "scanner" => Some(Source::Scanner),
            _ => None,
        }
    }
}

/// What was fetched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The open pixel.
    Open,
    /// A tracked link.
    Click,
}

impl Kind {
    /// The kind in the database and the event stream.
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Open => "open",
            Kind::Click => "click",
        }
    }

    /// Parses [`Kind::as_str`].
    pub fn parse(label: &str) -> Option<Self> {
        match label {
            "open" => Some(Kind::Open),
            "click" => Some(Kind::Click),
            _ => None,
        }
    }
}

/// What the request looked like.
#[derive(Clone, Copy, Debug)]
pub struct Request<'a> {
    /// Client address, when known.
    pub ip: Option<IpAddr>,
    /// `User-Agent` header, empty when missing.
    pub user_agent: &'a str,
    /// A `HEAD` request: scanners check links without fetching them.
    pub head: bool,
    /// Time since the tracking ID was created, which is when the mail was
    /// sent.
    pub since_sent: Duration,
}

/// Opens sooner than this after sending are taken for a scanner.
pub const OPEN_SCANNER_WINDOW: Duration = Duration::from_secs(5);

/// Clicks sooner than this after sending are taken for a scanner.
pub const CLICK_SCANNER_WINDOW: Duration = Duration::from_secs(30);

/// Labels one request.
pub fn classify(kind: Kind, request: Request<'_>) -> Source {
    let agent = request.user_agent.to_ascii_lowercase();
    if is_apple_proxy(request.ip, request.user_agent) {
        return Source::AppleProxy;
    }
    // Picture proxies that fetch when a person opens the mail.
    if agent.contains("googleimageproxy")
        || agent.contains("ggpht.com")
        || agent.contains("yahoomailproxy")
    {
        return Source::Person;
    }
    let window = match kind {
        Kind::Open => OPEN_SCANNER_WINDOW,
        Kind::Click => CLICK_SCANNER_WINDOW,
    };
    if request.head || agent.is_empty() || request.since_sent < window || is_automated(&agent) {
        return Source::Scanner;
    }
    Source::Person
}

/// Apple's proxy fetches from Apple's own network (17.0.0.0/8) and sends a
/// bare `Mozilla/5.0` user agent.
fn is_apple_proxy(ip: Option<IpAddr>, user_agent: &str) -> bool {
    let apple_network = matches!(ip, Some(IpAddr::V4(v4)) if v4.octets()[0] == 17);
    apple_network || user_agent.trim() == "Mozilla/5.0"
}

fn is_automated(agent: &str) -> bool {
    const WORDS: &[&str] = &[
        "bot",
        "crawler",
        "spider",
        "scanner",
        "preview",
        "curl/",
        "wget/",
        "python",
        "go-http-client",
        "java/",
        "okhttp",
        "libwww",
        "httpclient",
        "headless",
        "barracuda",
        "proofpoint",
        "mimecast",
        "symantec",
        "trendmicro",
        "fireeye",
        "safelinks",
    ];
    WORDS.iter().any(|word| agent.contains(word))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIREFOX: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0";

    fn request(user_agent: &str, secs: u64) -> Request<'_> {
        Request {
            ip: Some("203.0.113.9".parse().unwrap()),
            user_agent,
            head: false,
            since_sent: Duration::from_secs(secs),
        }
    }

    #[test]
    fn a_person() {
        assert_eq!(classify(Kind::Open, request(FIREFOX, 600)), Source::Person);
        assert_eq!(classify(Kind::Click, request(FIREFOX, 600)), Source::Person);
        let gmail = "Mozilla/5.0 (Windows NT 5.1; rv:11.0) Gecko Firefox/11.0 (via ggpht.com GoogleImageProxy)";
        assert_eq!(classify(Kind::Open, request(gmail, 2)), Source::Person);
    }

    #[test]
    fn apple_privacy_proxy() {
        assert_eq!(
            classify(Kind::Open, request("Mozilla/5.0", 600)),
            Source::AppleProxy
        );
        let mut from_apple = request(FIREFOX, 600);
        from_apple.ip = Some("17.58.10.1".parse().unwrap());
        assert_eq!(classify(Kind::Open, from_apple), Source::AppleProxy);
    }

    #[test]
    fn scanners() {
        assert_eq!(classify(Kind::Click, request(FIREFOX, 3)), Source::Scanner);
        assert_eq!(classify(Kind::Open, request("", 600)), Source::Scanner);
        assert_eq!(
            classify(Kind::Click, request("python-requests/2.32", 600)),
            Source::Scanner
        );
        let mut head = request(FIREFOX, 600);
        head.head = true;
        assert_eq!(classify(Kind::Click, head), Source::Scanner);
    }

    #[test]
    fn labels_round_trip() {
        for source in [Source::Person, Source::AppleProxy, Source::Scanner] {
            assert_eq!(Source::parse(source.as_str()), Some(source));
        }
        for kind in [Kind::Open, Kind::Click] {
            assert_eq!(Kind::parse(kind.as_str()), Some(kind));
        }
    }
}
