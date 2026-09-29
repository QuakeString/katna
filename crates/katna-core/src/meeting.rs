// SPDX-License-Identifier: GPL-3.0-or-later

//! Video call links (`docs/ARCHITECTURE.md` §18.2): which service a link
//! in a mail joins, for its Join button, and new Jitsi Meet links for
//! accounts whose mail service has no meetings of its own.

/// Default for [`crate::config::Meetings::jitsi_server`].
pub const JITSI_DEFAULT: &str = "https://meet.jit.si";

/// At most this many call links show under one mail.
pub const MAX_LINKS: usize = 3;

/// The service a call link joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    GoogleMeet,
    Teams,
    Zoom,
    Webex,
    Jitsi,
    WhatsApp,
    Telegram,
}

/// The host of an `https` URL, lowercased, and the rest after it
/// (path and query); `None` for anything else.
fn split(url: &str) -> Option<(String, &str)> {
    let rest = url
        .get(..8)
        .filter(|s| s.eq_ignore_ascii_case("https://"))
        .map(|_| &url[8..])?;
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..end];
    // A name before the host only disguises it.
    if authority.contains('@') {
        return None;
    }
    let host = authority.split(':').next()?.trim_end_matches('.');
    (!host.is_empty()).then(|| (host.to_ascii_lowercase(), &rest[end..]))
}

fn under(host: &str, domain: &str) -> bool {
    host == domain
        || host
            .strip_suffix(domain)
            .is_some_and(|h| h.ends_with('.'))
}

/// The first segment of `path` (`/abc-defg-hij?x` gives `abc-defg-hij`).
fn first_segment(path: &str) -> &str {
    let path = path.strip_prefix('/').unwrap_or("");
    let end = path.find(['/', '?', '#']).unwrap_or(path.len());
    &path[..end]
}

/// A Google Meet code: `abc-defg-hij`.
fn meet_code(segment: &str) -> bool {
    let parts: Vec<&str> = segment.split('-').collect();
    parts.len() == 3
        && [3, 4, 3]
            .iter()
            .zip(&parts)
            .all(|(n, p)| p.len() == *n && p.bytes().all(|b| b.is_ascii_lowercase()))
}

/// The service `url` joins a call of, if any. `jitsi_host` is the user's
/// own Jitsi server's host, whose rooms count too.
pub fn service(url: &str, jitsi_host: Option<&str>) -> Option<Service> {
    let (host, path) = split(url)?;
    let lower = path.to_ascii_lowercase();
    let room = first_segment(path);
    if host == "meet.google.com" {
        return (meet_code(room) || room == "lookup").then_some(Service::GoogleMeet);
    }
    if under(&host, "teams.microsoft.com") || host == "teams.live.com" {
        return (lower.starts_with("/l/meetup-join/") || lower.starts_with("/meet/"))
            .then_some(Service::Teams);
    }
    if under(&host, "zoom.us") || under(&host, "zoomgov.com") {
        return ["/j/", "/w/", "/my/", "/s/"]
            .iter()
            .any(|p| lower.starts_with(p))
            .then_some(Service::Zoom);
    }
    if under(&host, "webex.com") {
        return ["/meet/", "/join/", "/j.php", "/wbxmjs/joinservice/"]
            .iter()
            .any(|p| lower.contains(p))
            .then_some(Service::Webex);
    }
    if host == "call.whatsapp.com" {
        return (lower.starts_with("/video/") || lower.starts_with("/voice/"))
            .then_some(Service::WhatsApp);
    }
    if host == "t.me" || host == "telegram.me" {
        let chat = ["videochat", "voicechat", "livestream"]
            .iter()
            .any(|k| lower.contains(&format!("?{k}")) || lower.contains(&format!("&{k}")));
        return (lower.starts_with("/call/") || chat).then_some(Service::Telegram);
    }
    let jitsi = host == "meet.jit.si"
        || host == "8x8.vc"
        || jitsi_host.is_some_and(|j| j.eq_ignore_ascii_case(&host));
    // A room, not the server's own pages.
    let page = matches!(room, "" | "static" | "libs" | "css" | "images" | "sounds")
        || room.contains('.');
    (jitsi && !page).then_some(Service::Jitsi)
}

/// The `https` links written out in `text`.
fn written_links(text: &str) -> impl Iterator<Item = &str> {
    let mut rest = text;
    std::iter::from_fn(move || {
        let start = rest.find("https://")?;
        let from = &rest[start..];
        let end = from
            .find(|c: char| c.is_whitespace() || matches!(c, '<' | '>' | '"' | '\'' | '`'))
            .unwrap_or(from.len());
        let link = from[..end].trim_end_matches(['.', ',', ';', ':', '!', '?', ')', ']', '}']);
        rest = &from[end..];
        Some(link)
    })
}

/// The call links among `links` and those written out in `texts`, each
/// once, in order, at most [`MAX_LINKS`].
pub fn find<'a>(
    links: impl IntoIterator<Item = &'a str>,
    texts: impl IntoIterator<Item = &'a str>,
    jitsi_host: Option<&str>,
) -> Vec<(Service, String)> {
    let mut out: Vec<(Service, String)> = Vec::new();
    let written = texts.into_iter().flat_map(written_links);
    for link in links.into_iter().chain(written) {
        if out.len() == MAX_LINKS {
            break;
        }
        let link = link.trim();
        if let Some(service) = service(link, jitsi_host)
            && !out.iter().any(|(_, l)| same_call(l, link))
        {
            out.push((service, link.to_owned()));
        }
    }
    out
}

/// Two links to one call: the same address, whatever the case of the
/// host or the query after it.
fn same_call(a: &str, b: &str) -> bool {
    let base = |l: &str| l.split(['?', '#']).next().unwrap_or(l).trim_end_matches('/').to_ascii_lowercase();
    base(a) == base(b)
}

/// The Jitsi server set in Settings as a base URL: `https://` added when
/// missing, no trailing `/`; [`JITSI_DEFAULT`] when empty or not a plain
/// `https` host.
pub fn jitsi_server(setting: &str) -> String {
    let setting = setting.trim().trim_end_matches('/');
    if setting.is_empty() {
        return JITSI_DEFAULT.to_owned();
    }
    let url = if setting.contains("://") {
        setting.to_owned()
    } else {
        format!("https://{setting}")
    };
    match split(&url) {
        Some((host, rest)) if !host.contains(char::is_whitespace) && host.contains('.') => {
            let rest = rest.trim_end_matches('/');
            if rest.contains(['?', '#']) {
                JITSI_DEFAULT.to_owned()
            } else {
                format!("https://{host}{rest}")
            }
        }
        _ => JITSI_DEFAULT.to_owned(),
    }
}

/// The host of the Jitsi server set in Settings, for recognising its
/// rooms in mail.
pub fn jitsi_host(setting: &str) -> Option<String> {
    split(&jitsi_server(setting)).map(|(host, _)| host)
}

/// A new room on `server` (from [`jitsi_server`]) named from `random`:
/// `https://meet.jit.si/katna-k3xm-p9qa-7hzr-w2cd`, hard to guess, as
/// Jitsi asks, since anyone with the name can join.
pub fn jitsi_link(server: &str, random: &[u8; 16]) -> String {
    const LETTERS: &[u8] = b"abcdefghijkmnpqrstuvwxyz23456789";
    let mut room = String::from("katna");
    for (i, byte) in random.iter().enumerate() {
        if i % 4 == 0 {
            room.push('-');
        }
        room.push(LETTERS[usize::from(byte % 32)] as char);
    }
    format!("{server}/{room}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_each_service() {
        let cases = [
            ("https://meet.google.com/abc-defg-hij", Service::GoogleMeet),
            ("https://meet.google.com/abc-defg-hij?authuser=1", Service::GoogleMeet),
            (
                "https://teams.microsoft.com/l/meetup-join/19%3ameeting_x%40thread.v2/0",
                Service::Teams,
            ),
            ("https://teams.live.com/meet/9876543210", Service::Teams),
            ("https://us02web.zoom.us/j/81234567890?pwd=abc", Service::Zoom),
            ("https://acme.webex.com/meet/pat", Service::Webex),
            ("https://acme.webex.com/acme/j.php?MTID=m1", Service::Webex),
            ("https://meet.jit.si/SomeRoom", Service::Jitsi),
            ("https://8x8.vc/vpaas-magic-cookie-1/room", Service::Jitsi),
            ("https://call.whatsapp.com/video/zl6IuTVabc", Service::WhatsApp),
            ("https://call.whatsapp.com/voice/abc", Service::WhatsApp),
            ("https://t.me/call/AbCdEf", Service::Telegram),
            ("https://t.me/somegroup?videochat", Service::Telegram),
            ("https://t.me/somegroup?voicechat=abc", Service::Telegram),
        ];
        for (url, want) in cases {
            assert_eq!(service(url, None), Some(want), "{url}");
        }
    }

    #[test]
    fn leaves_other_links_alone() {
        for url in [
            "https://meet.google.com/",
            "https://meet.google.com/new",
            "https://teams.microsoft.com/",
            "https://zoom.us/pricing",
            "https://www.webex.com/",
            "https://meet.jit.si/",
            "https://meet.jit.si/static/close.html",
            "https://www.whatsapp.com/",
            "https://call.whatsapp.com/",
            "https://t.me/somechannel",
            "https://evil.example/meet.google.com/abc-defg-hij",
            "https://meet.google.com@evil.example/abc-defg-hij",
            "http://meet.google.com/abc-defg-hij",
            "https://notzoom.us/j/123",
            "https://jitsi.example.org/room",
        ] {
            assert_eq!(service(url, None), None, "{url}");
        }
    }

    #[test]
    fn rooms_on_the_users_own_jitsi_server_count() {
        let host = jitsi_host("jitsi.example.org/");
        assert_eq!(host.as_deref(), Some("jitsi.example.org"));
        assert_eq!(
            service("https://jitsi.example.org/room", host.as_deref()),
            Some(Service::Jitsi)
        );
    }

    #[test]
    fn finds_links_in_text_and_html_each_once() {
        let text = "Join here: https://meet.google.com/abc-defg-hij.\n\
                    Or (https://meet.google.com/abc-defg-hij?authuser=0) \
                    and https://example.com/x, then <https://call.whatsapp.com/video/Tok>";
        let found = find(
            ["https://us02web.zoom.us/j/1"],
            [text],
            None,
        );
        assert_eq!(
            found,
            vec![
                (Service::Zoom, "https://us02web.zoom.us/j/1".to_owned()),
                (
                    Service::GoogleMeet,
                    "https://meet.google.com/abc-defg-hij".to_owned()
                ),
                (
                    Service::WhatsApp,
                    "https://call.whatsapp.com/video/Tok".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn at_most_a_few_links() {
        let text = (0..10)
            .map(|i| format!("https://meet.jit.si/room{i} "))
            .collect::<String>();
        assert_eq!(find([], [text.as_str()], None).len(), MAX_LINKS);
    }

    #[test]
    fn jitsi_server_setting_is_tidied() {
        assert_eq!(jitsi_server(""), JITSI_DEFAULT);
        assert_eq!(jitsi_server("  "), JITSI_DEFAULT);
        assert_eq!(jitsi_server("jitsi.example.org/"), "https://jitsi.example.org");
        assert_eq!(
            jitsi_server("https://Example.org/jitsi/"),
            "https://example.org/jitsi"
        );
        assert_eq!(jitsi_server("http://jitsi.example.org"), JITSI_DEFAULT);
        assert_eq!(jitsi_server("not a server"), JITSI_DEFAULT);
    }

    #[test]
    fn new_jitsi_rooms_are_hard_to_guess() {
        let link = jitsi_link(JITSI_DEFAULT, &[0, 1, 2, 3, 31, 32, 33, 255, 7, 8, 9, 10, 11, 12, 13, 14]);
        assert_eq!(link, "https://meet.jit.si/katna-abcd-9ab9-hijk-mnpq");
        assert_eq!(service(&link, None), Some(Service::Jitsi));
    }
}
