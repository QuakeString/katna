// SPDX-License-Identifier: GPL-3.0-or-later

//! Unsubscribing from mailing lists and newsletters
//! (`docs/ARCHITECTURE.md` §15.9). The sender's own way comes first: the
//! `List-Unsubscribe` header (RFC 2369), with `List-Unsubscribe-Post`
//! (RFC 8058) saying its web address takes a one-click POST. The daemon
//! posts there, or else sends the header's `mailto:` from the account the
//! mail came to; nothing goes through Katna's own servers. A link in the
//! body is only ever opened in the browser, never followed here.

use crate::mailto::Mailto;
use crate::mime::{Mailbox, address, encode_words, quoted_printable};

/// How a message can be unsubscribed from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ways {
    /// The `https` address that takes a one-click POST (RFC 8058).
    pub one_click: Option<String>,
    /// The `mailto:` of `List-Unsubscribe`.
    pub mail: Option<Mailto>,
    /// A web page to open in the browser: the header's `https` address
    /// when it takes no POST, else an unsubscribe link in the body.
    pub page: Option<String>,
}

impl Ways {
    /// Reads the `List-Unsubscribe` and `List-Unsubscribe-Post` header
    /// values (unfolded) and `body_link`, an unsubscribe link of the body.
    pub fn read(header: Option<&str>, post: Option<&str>, body_link: Option<&str>) -> Self {
        let mut ways = Self::default();
        let one_click = post.is_some_and(|p| {
            p.split(';')
                .any(|v| v.trim().eq_ignore_ascii_case("List-Unsubscribe=One-Click"))
        });
        for uri in header.map(addresses).unwrap_or_default() {
            let lower = uri.to_ascii_lowercase();
            if lower.starts_with("mailto:") {
                if ways.mail.is_none() {
                    ways.mail = Mailto::parse(&uri).filter(|m| !m.to.is_empty());
                }
            } else if lower.starts_with("https://") {
                if one_click && ways.one_click.is_none() {
                    ways.one_click = Some(uri.clone());
                }
                if ways.page.is_none() {
                    ways.page = Some(uri);
                }
            } else if lower.starts_with("http://") && ways.page.is_none() {
                // RFC 8058 posts over https only; the page still opens.
                ways.page = Some(uri);
            }
        }
        if ways.page.is_none() {
            ways.page = body_link
                .filter(|l| {
                    let l = l.to_ascii_lowercase();
                    l.starts_with("https://") || l.starts_with("http://")
                })
                .map(str::to_owned);
        }
        ways
    }

    /// It can be done in the background, without a web page.
    pub fn quiet(&self) -> bool {
        self.one_click.is_some() || self.mail.is_some()
    }

    /// There is any way at all.
    pub fn any(&self) -> bool {
        self.quiet() || self.page.is_some()
    }
}

/// The addresses of a `List-Unsubscribe` value: `<uri>, <uri>`, each in
/// angle brackets; folding whitespace inside one is dropped.
fn addresses(value: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = value;
    while let Some(start) = rest.find('<') {
        let Some(len) = rest[start + 1..].find('>') else {
            break;
        };
        let uri: String = rest[start + 1..start + 1 + len]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        if !uri.is_empty() {
            out.push(uri);
        }
        rest = &rest[start + 1 + len + 1..];
    }
    out
}

/// Whether a link's text offers to unsubscribe, in the languages
/// newsletters most often write it in.
pub fn offers_unsubscribe(text: &str) -> bool {
    const WORDS: &[&str] = &[
        "unsubscribe",
        "opt out",
        "opt-out",
        "abmelden",
        "abbestellen",
        "désabonner",
        "desabonner",
        "désinscri",
        "darse de baja",
        "cancelar suscripción",
        "annulla iscrizione",
        "cancelar inscrição",
        "uitschrijven",
        "afmelden",
        "отписаться",
        "退订",
        "取消订阅",
        "配信停止",
        "सदस्यता रद्द",
    ];
    let text = text.trim().to_lowercase();
    // A sentence that mentions it is not the link itself.
    text.chars().count() <= 60 && WORDS.iter().any(|w| text.contains(w))
}

/// The form a one-click POST sends (RFC 8058 §3.1).
pub const ONE_CLICK_BODY: &str = "List-Unsubscribe=One-Click";

/// The unfolded `List-Unsubscribe` and `List-Unsubscribe-Post` values of
/// the message `raw`.
pub fn list_headers(raw: &[u8]) -> (Option<String>, Option<String>) {
    let end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .or_else(|| raw.windows(2).position(|w| w == b"\n\n"))
        .unwrap_or(raw.len());
    let head = String::from_utf8_lossy(&raw[..end]);
    let mut fields: Vec<(String, String)> = Vec::new();
    for line in head.lines() {
        if line.starts_with([' ', '\t']) {
            if let Some((_, value)) = fields.last_mut() {
                value.push(' ');
                value.push_str(line.trim());
            }
        } else if let Some((name, value)) = line.split_once(':') {
            fields.push((name.trim().to_ascii_lowercase(), value.trim().to_owned()));
        }
    }
    let value = |name: &str| {
        fields
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.clone())
    };
    (value("list-unsubscribe"), value("list-unsubscribe-post"))
}

/// The message that unsubscribes through `mail`, from `from`; the outbox
/// adds `Date` and `Message-ID`.
pub fn message(mail: &Mailto, from: &Mailbox) -> Vec<u8> {
    let subject = match mail.subject.trim() {
        "" => "unsubscribe",
        subject => subject,
    };
    let body = match mail.body.trim() {
        "" => "unsubscribe",
        body => body,
    };
    let mut head = format!(
        "From: {}\r\nTo: {}\r\nSubject: {}\r\n",
        address(from),
        mail.to.join(", "),
        encode_words(subject)
    );
    if !mail.cc.is_empty() {
        head.push_str(&format!("Cc: {}\r\n", mail.cc.join(", ")));
    }
    head.push_str(
        "MIME-Version: 1.0\r\nContent-Type: text/plain; charset=utf-8\r\n\
         Content-Transfer-Encoding: quoted-printable\r\n\r\n",
    );
    let mut raw = head.into_bytes();
    raw.extend_from_slice(quoted_printable(body).as_bytes());
    raw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_click_needs_the_post_header_and_https() {
        let header =
            "<mailto:leave@list.example?subject=unsubscribe>,\r\n <https://list.example/u?id=1>";
        let ways = Ways::read(Some(header), Some("List-Unsubscribe=One-Click"), None);
        assert_eq!(
            ways.one_click.as_deref(),
            Some("https://list.example/u?id=1")
        );
        let mail = ways.mail.as_ref().expect("mailto");
        assert_eq!(mail.to, ["leave@list.example"]);
        assert_eq!(mail.subject, "unsubscribe");
        assert!(ways.quiet());

        let ways = Ways::read(Some(header), None, None);
        assert_eq!(ways.one_click, None);
        assert_eq!(ways.page.as_deref(), Some("https://list.example/u?id=1"));
        assert!(ways.quiet(), "the mailto still works");

        let ways = Ways::read(
            Some("<http://list.example/u>"),
            Some("List-Unsubscribe=One-Click"),
            None,
        );
        assert_eq!(ways.one_click, None);
        assert!(!ways.quiet());
        assert_eq!(ways.page.as_deref(), Some("http://list.example/u"));
    }

    #[test]
    fn a_body_link_is_only_a_page() {
        let ways = Ways::read(None, None, Some("https://shop.example/unsub"));
        assert!(!ways.quiet());
        assert!(ways.any());
        assert_eq!(ways.page.as_deref(), Some("https://shop.example/unsub"));
        assert!(!Ways::read(None, None, Some("javascript:x")).any());
        assert!(!Ways::read(Some("junk"), None, None).any());
    }

    #[test]
    fn folded_addresses_are_joined() {
        assert_eq!(
            addresses("<https://e.test/u?a=1\r\n &b=2>, <mailto:x@e.test>"),
            ["https://e.test/u?a=1&b=2", "mailto:x@e.test"]
        );
    }

    #[test]
    fn headers_are_read_unfolded() {
        let raw = b"From: a@e.test\r\nlist-unsubscribe: <https://e.test/u?a=1>,\r\n\t<mailto:u@e.test>\r\n\
List-Unsubscribe-Post: List-Unsubscribe=One-Click\r\n\r\nList-Unsubscribe: <https://body.test>\r\n";
        let (header, post) = list_headers(raw);
        assert_eq!(
            header.as_deref(),
            Some("<https://e.test/u?a=1>, <mailto:u@e.test>")
        );
        assert_eq!(post.as_deref(), Some("List-Unsubscribe=One-Click"));
        assert_eq!(list_headers(b"Subject: x\n\nbody"), (None, None));
    }

    #[test]
    fn the_unsubscribe_mail() {
        let mail = Mailto::parse("mailto:leave@list.example?subject=unsubscribe%20me").unwrap();
        let from = Mailbox {
            name: Some("Kay".into()),
            email: "kay@example.com".into(),
        };
        let raw = String::from_utf8(message(&mail, &from)).unwrap();
        assert!(raw.starts_with(
            "From: Kay <kay@example.com>\r\nTo: leave@list.example\r\nSubject: unsubscribe me\r\n"
        ));
        assert!(raw.trim_end().ends_with("\r\n\r\nunsubscribe"), "{raw}");
    }

    #[test]
    fn link_texts_that_offer_it() {
        assert!(offers_unsubscribe("Unsubscribe"));
        assert!(offers_unsubscribe(" Click here to unsubscribe "));
        assert!(offers_unsubscribe("Newsletter abbestellen"));
        assert!(!offers_unsubscribe("Read more"));
        assert!(!offers_unsubscribe(
            "You are receiving this because you signed up; you can unsubscribe at any time from your account settings page."
        ));
    }
}
