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
//!
//! [`sender_checks`] keeps every check, so the reading view can warn
//! about mail whose sender failed them (`docs/ARCHITECTURE.md` §12).

use mail_parser::MessageParser;

/// What the provider found of one sender check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Pass,
    Fail,
    /// `softfail`, `neutral`, `policy`, `temperror` or `permerror`: the
    /// check ran and said neither yes nor no.
    Unsure,
    /// `none`: nothing to check (no record, no signature).
    None,
}

impl Outcome {
    fn parse(word: &str) -> Self {
        match word.to_ascii_lowercase().as_str() {
            "pass" => Self::Pass,
            "fail" | "hardfail" => Self::Fail,
            "none" => Self::None,
            _ => Self::Unsure,
        }
    }
}

/// One check: what it found, and for which domain (DMARC's `header.from`,
/// DKIM's signer, SPF's `smtp.mailfrom` or `smtp.helo` domain).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    pub outcome: Outcome,
    pub domain: Option<String>,
}

/// The provider's word on a message's sender, in short.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// DMARC passed for the `From` domain, or a DKIM signature passed for
    /// a domain aligned with it.
    Passed,
    /// DMARC failed for the `From` domain, or SPF failed for an aligned
    /// envelope domain and no aligned signature passed: most likely
    /// someone else wrote that `From`.
    Failed,
    /// The provider checked, and nothing confirmed the sender.
    Unconfirmed,
}

/// What the user's provider found of a message's sender.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SenderChecks {
    /// The provider's server that checked (its `authserv-id`, lower case).
    pub server: String,
    /// The `From` domain, lower case.
    pub domain: String,
    pub verdict: Verdict,
    pub dmarc: Option<Check>,
    /// The aligned signature that passed, else any that passed, else the
    /// first.
    pub dkim: Option<Check>,
    pub spf: Option<Check>,
}

/// Whether the provider says the `From` domain of `raw` is authentic.
pub fn sender_authenticated(raw: &[u8]) -> bool {
    sender_checks(raw).is_some_and(|checks| checks.verdict == Verdict::Passed)
}

/// What the provider found of the sender of `raw`; `None` without its
/// `Authentication-Results`, or without one `From` address with a domain.
pub fn sender_checks(raw: &[u8]) -> Option<SenderChecks> {
    let message = MessageParser::default().parse_headers(raw)?;
    // One sender only: with more, which one was checked is unclear.
    let from = match message.from().map(|from| from.iter().collect::<Vec<_>>()) {
        Some(list) if list.len() == 1 => list[0].address().unwrap_or_default(),
        _ => return None,
    };
    let domain = from
        .rsplit_once('@')
        .map(|(_, domain)| domain.trim().trim_end_matches('.').to_ascii_lowercase())
        .filter(|domain| domain.contains('.'))?;
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
    checks(fields, domain)
}

/// Reads the fields (topmost first) naming the topmost one's server.
fn checks(fields: impl Iterator<Item = String>, domain: String) -> Option<SenderChecks> {
    let mut server: Option<String> = None;
    let mut dmarc: Vec<Check> = Vec::new();
    let mut dkim: Vec<Check> = Vec::new();
    let mut spf: Vec<Check> = Vec::new();
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
        for result in parts {
            let mut words = result.split_whitespace();
            let Some((method, outcome)) = words.next().and_then(|w| w.split_once('=')) else {
                continue;
            };
            let method = method.split('/').next().unwrap_or_default();
            let outcome = Outcome::parse(outcome);
            let property = |name: &str| property(result, name);
            if method.eq_ignore_ascii_case("dmarc") {
                dmarc.push(Check {
                    outcome,
                    domain: property("header.from"),
                });
            } else if method.eq_ignore_ascii_case("dkim") {
                dkim.push(Check {
                    outcome,
                    domain: property("header.d").or_else(|| property("header.i")),
                });
            } else if method.eq_ignore_ascii_case("spf") {
                spf.push(Check {
                    outcome,
                    domain: property("smtp.mailfrom").or_else(|| property("smtp.helo")),
                });
            }
        }
    }
    let server = server.filter(|server| !server.is_empty())?;
    let ours = |check: &Check| check.domain.as_deref().is_some_and(|d| aligned(d, &domain));
    let passed = dmarc
        .iter()
        .any(|c| c.outcome == Outcome::Pass && c.domain.as_deref() == Some(&*domain))
        || dkim.iter().any(|c| c.outcome == Outcome::Pass && ours(c));
    // A DMARC result without `header.from` is about the `From` domain.
    let failed = dmarc
        .iter()
        .any(|c| c.outcome == Outcome::Fail && c.domain.as_deref().is_none_or(|d| d == domain))
        || spf.iter().any(|c| c.outcome == Outcome::Fail && ours(c));
    let verdict = if passed {
        Verdict::Passed
    } else if failed {
        Verdict::Failed
    } else {
        Verdict::Unconfirmed
    };
    let dmarc = dmarc
        .iter()
        .find(|c| c.domain.as_deref().is_none_or(|d| d == domain))
        .or(dmarc.first())
        .cloned();
    let dkim = dkim
        .iter()
        .find(|c| c.outcome == Outcome::Pass && ours(c))
        .or_else(|| dkim.iter().find(|c| c.outcome == Outcome::Pass))
        .or(dkim.first())
        .cloned();
    let spf = spf.first().cloned();
    Some(SenderChecks {
        server,
        domain,
        verdict,
        dmarc,
        dkim,
        spf,
    })
}

/// The domain in `property=value` of one result item: the part after any
/// `@`, lower case, without quotes or a final dot.
fn property(result: &str, name: &str) -> Option<String> {
    result
        .split_whitespace()
        .skip(1)
        .find_map(|word| {
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
        .filter(|domain| !domain.is_empty())
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

    fn verdict(from: &str, field: &str) -> Option<Verdict> {
        sender_checks(&message(from, &[field])).map(|checks| checks.verdict)
    }

    #[test]
    fn verdicts() {
        let from = "Bank <alerts@bank.example>";
        // Gmail on a forged sender.
        let forged = "mx.google.com; spf=fail (google.com: domain of alerts@bank.example does not designate 203.0.113.7 as permitted sender) smtp.mailfrom=alerts@bank.example; dmarc=fail (p=REJECT sp=REJECT dis=QUARANTINE) header.from=bank.example";
        assert_eq!(verdict(from, forged), Some(Verdict::Failed));
        let checks = sender_checks(&message(from, &[forged])).unwrap();
        assert_eq!(checks.server, "mx.google.com");
        assert_eq!(checks.domain, "bank.example");
        assert_eq!(
            checks.spf,
            Some(Check {
                outcome: Outcome::Fail,
                domain: Some("bank.example".to_owned())
            })
        );
        assert_eq!(checks.dkim, None);
        // SPF failed for someone else's envelope: says nothing of `From`.
        let field = "mx.example.net; spf=fail smtp.mailfrom=bounce@mailer.example; dmarc=none";
        assert_eq!(verdict(from, field), Some(Verdict::Unconfirmed));
        // Soft failures are not failures.
        let field = "mx.example.net; spf=softfail smtp.mailfrom=a@bank.example; dkim=none";
        assert_eq!(verdict(from, field), Some(Verdict::Unconfirmed));
        // An aligned signature outweighs a failed SPF (forwarded mail).
        let field = "mx.example.net; spf=fail smtp.mailfrom=a@bank.example; dkim=pass header.d=bank.example";
        assert_eq!(verdict(from, field), Some(Verdict::Passed));
        // DMARC without header.from is about the `From` domain.
        assert_eq!(
            verdict(from, "mx.example.net; dmarc=fail"),
            Some(Verdict::Failed)
        );
        // DMARC failing for another domain says nothing of this one.
        assert_eq!(
            verdict(from, "mx.example.net; dmarc=fail header.from=other.example"),
            Some(Verdict::Unconfirmed)
        );
        assert_eq!(
            verdict(from, "mx.example.net; none"),
            Some(Verdict::Unconfirmed)
        );
        assert_eq!(sender_checks(&message(from, &[])), None);
    }

    #[test]
    fn the_aligned_signature_is_the_one_kept() {
        let field = "mx.example.net; dkim=fail header.d=bank.example; dkim=pass header.d=esp.example; dkim=pass header.i=@mail.bank.example";
        let checks = sender_checks(&message("a@bank.example", &[field])).unwrap();
        assert_eq!(checks.verdict, Verdict::Passed);
        assert_eq!(
            checks.dkim.unwrap().domain.as_deref(),
            Some("mail.bank.example")
        );
        let field =
            "mx.example.net; dkim=fail header.d=bank.example; dkim=pass header.d=esp.example";
        let checks = sender_checks(&message("a@bank.example", &[field])).unwrap();
        assert_eq!(checks.verdict, Verdict::Unconfirmed);
        assert_eq!(checks.dkim.unwrap().domain.as_deref(), Some("esp.example"));
    }
}
