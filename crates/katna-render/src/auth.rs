// SPDX-License-Identifier: GPL-3.0-or-later

//! Whether the user's mail provider vouched for a message's `From`
//! address, from the `Authentication-Results` fields it adds (RFC 8601).
//!
//! Anyone can write any `From`, so a choice keyed on the sender (say,
//! "always show images from this sender") holds only for mail the
//! provider checked: DMARC passed for the `From` domain, or a DKIM
//! signature passed for a domain aligned with it.
//!
//! The provider adds its fields on top of the message, and removes any
//! the message came with that claim its name (RFC 8601 §5), so only the
//! topmost field and others naming the same server (`authserv-id`) are
//! read. A provider that adds none leaves the topmost field to the
//! sender; that is no worse than trusting `From` alone, which is what
//! Katna did before.

use mail_parser::MessageParser;

/// Whether the provider says the `From` domain of `raw` is authentic.
pub fn sender_authenticated(raw: &[u8]) -> bool {
    let Some(message) = MessageParser::default().parse_headers(raw) else {
        return false;
    };
    // One sender only: with more, which one was checked is unclear.
    let from = match message.from().map(|from| from.iter().collect::<Vec<_>>()) {
        Some(list) if list.len() == 1 => list[0].address().unwrap_or_default(),
        _ => return false,
    };
    let Some(domain) = from
        .rsplit_once('@')
        .map(|(_, domain)| domain.trim().trim_end_matches('.').to_ascii_lowercase())
        .filter(|domain| domain.contains('.'))
    else {
        return false;
    };
    let fields = message
        .headers()
        .iter()
        .filter(|h| {
            h.name
                .as_str()
                .eq_ignore_ascii_case("Authentication-Results")
        })
        .filter_map(|h| raw.get(h.offset_start as usize..h.offset_end as usize))
        .map(|value| String::from_utf8_lossy(value).into_owned());
    vouches(fields, &domain)
}

/// Whether the fields (topmost first) say `domain` passed.
fn vouches(fields: impl Iterator<Item = String>, domain: &str) -> bool {
    let mut server: Option<String> = None;
    for field in fields {
        let field = uncomment(&field);
        let mut parts = field.split(';');
        let id = parts
            .next()
            .and_then(|first| first.split_whitespace().next())
            .unwrap_or_default()
            .to_ascii_lowercase();
        match &server {
            None => server = Some(id),
            Some(server) if *server == id => {}
            // Another server's: the message may have come with it.
            Some(_) => continue,
        }
        if parts.any(|result| passes(result, domain)) {
            return true;
        }
    }
    false
}

/// Whether one `method=result property=value …` item is a DMARC pass for
/// `domain` or a DKIM pass for a domain aligned with it.
fn passes(result: &str, domain: &str) -> bool {
    let mut words = result.split_whitespace();
    let Some((method, outcome)) = words.next().and_then(|w| w.split_once('=')) else {
        return false;
    };
    let method = method.split('/').next().unwrap_or_default();
    if !outcome.eq_ignore_ascii_case("pass") {
        return false;
    }
    let property = |name: &str| {
        result.split_whitespace().skip(1).find_map(|word| {
            let (key, value) = word.split_once('=')?;
            key.eq_ignore_ascii_case(name).then(|| {
                value
                    .trim_matches('"')
                    .trim_start_matches('@')
                    .rsplit('@')
                    .next()
                    .unwrap_or_default()
                    .trim_end_matches('.')
                    .to_ascii_lowercase()
            })
        })
    };
    if method.eq_ignore_ascii_case("dmarc") {
        property("header.from").is_some_and(|from| from == domain)
    } else if method.eq_ignore_ascii_case("dkim") {
        property("header.d")
            .or_else(|| property("header.i"))
            .is_some_and(|signer| aligned(&signer, domain))
    } else {
        false
    }
}

/// Relaxed DMARC alignment, roughly: the same domain, or one inside the
/// other, and never a bare top-level name.
fn aligned(signer: &str, domain: &str) -> bool {
    if !signer.contains('.') {
        return false;
    }
    signer == domain
        || domain.ends_with(&format!(".{signer}"))
        || signer.ends_with(&format!(".{domain}"))
}

/// `text` without `(comments)`, which may nest and hold `;`.
fn uncomment(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut depth = 0usize;
    let mut quoted = false;
    let mut escaped = false;
    for c in text.chars() {
        if escaped {
            escaped = false;
            if depth == 0 {
                out.push(c);
            }
            continue;
        }
        match c {
            '\\' => {
                escaped = true;
                if depth == 0 {
                    out.push(c);
                }
            }
            '"' if depth == 0 => {
                quoted = !quoted;
                out.push(c);
            }
            '(' if !quoted => depth += 1,
            ')' if !quoted && depth > 0 => depth -= 1,
            _ if depth == 0 => out.push(if c == '\r' || c == '\n' { ' ' } else { c }),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(from: &str, fields: &[&str]) -> Vec<u8> {
        let mut raw = String::new();
        for field in fields {
            raw.push_str(&format!("Authentication-Results: {field}\r\n"));
        }
        raw.push_str(&format!("From: {from}\r\nSubject: hi\r\n\r\nbody\r\n"));
        raw.into_bytes()
    }

    #[test]
    fn dmarc_pass_for_the_from_domain() {
        let gmail = "mx.google.com;\r\n       dkim=pass header.i=@news.shop.example header.s=s1 header.b=abc;\r\n       spf=pass (google.com: domain of bounce@shop.example designates 1.2.3.4 as permitted sender) smtp.mailfrom=bounce@shop.example;\r\n       dmarc=pass (p=REJECT sp=REJECT dis=NONE) header.from=shop.example";
        assert!(sender_authenticated(&message(
            "Shop <news@shop.example>",
            &[gmail]
        )));
        assert!(!sender_authenticated(&message(
            "news@other.example",
            &[gmail]
        )));
    }

    #[test]
    fn aligned_dkim_passes() {
        let field = "mx.example.net; dkim=pass header.d=mail.shop.example header.s=k1; dmarc=none";
        assert!(sender_authenticated(&message(
            "news@shop.example",
            &[field]
        )));
        let field = "mx.example.net; dkim=pass header.d=shop.example";
        assert!(sender_authenticated(&message(
            "a@news.shop.example",
            &[field]
        )));
        // Signed, but by someone else.
        let field = "mx.example.net; dkim=pass header.d=mailer.example; dmarc=fail header.from=shop.example";
        assert!(!sender_authenticated(&message(
            "news@shop.example",
            &[field]
        )));
        let field = "mx.example.net; dkim=pass header.d=example";
        assert!(!sender_authenticated(&message(
            "news@shop.example",
            &[field]
        )));
    }

    #[test]
    fn failures_and_missing_fields() {
        assert!(!sender_authenticated(&message("news@shop.example", &[])));
        let field =
            "mx.example.net; dkim=fail header.d=shop.example; dmarc=fail header.from=shop.example";
        assert!(!sender_authenticated(&message(
            "news@shop.example",
            &[field]
        )));
        let field = "mx.example.net; none";
        assert!(!sender_authenticated(&message(
            "news@shop.example",
            &[field]
        )));
    }

    /// A pass the message brought along under another server's name is
    /// not the provider's.
    #[test]
    fn only_the_providers_own_fields_count() {
        let provider = "mx.example.net; dmarc=fail header.from=shop.example";
        let forged = "evil.example; dmarc=pass header.from=shop.example";
        assert!(!sender_authenticated(&message(
            "news@shop.example",
            &[provider, forged]
        )));
        let second = "mx.example.net; dmarc=pass header.from=shop.example";
        assert!(sender_authenticated(&message(
            "news@shop.example",
            &[provider, second]
        )));
    }

    #[test]
    fn comments_hide_nothing() {
        let field = "mx.example.net; spf=fail (dmarc=pass header.from=shop.example; x) smtp.mailfrom=a@b.example";
        assert!(!sender_authenticated(&message(
            "news@shop.example",
            &[field]
        )));
        assert!(!sender_authenticated(&message(
            "a@shop.example, b@shop.example",
            &["mx.example.net; dmarc=pass header.from=shop.example"]
        )));
    }
}
