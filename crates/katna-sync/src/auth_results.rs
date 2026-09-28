// SPDX-License-Identifier: GPL-3.0-or-later

//! The user's provider's verdict on who sent a message: its topmost
//! `Authentication-Results` header (RFC 8601). A receiving server adds its
//! own above the ones that came with the message, so only the first is
//! read. A provider that adds none leaves the sender's own header on top;
//! that is a known limit, and why the verdict only gates sender pictures,
//! never how a message is shown.

use crate::pictures::organizational_domain;

/// What the provider said about a message's `From` domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    /// The `dmarc=` result (`pass`, `fail`, `none`, …), lower case.
    pub dmarc: Option<String>,
    /// DMARC passed for the `From` domain, or DKIM passed for a domain
    /// aligned with it (the same organizational domain).
    pub aligned: bool,
}

impl Verdict {
    /// The verdict as `message.auth_results_json` keeps it.
    pub fn to_json(&self) -> String {
        serde_json::json!({ "dmarc": self.dmarc, "aligned": self.aligned }).to_string()
    }
}

/// The verdict of the topmost `Authentication-Results` field in `header`
/// (header fields, or a whole message) on mail `From` `from_domain`;
/// `None` when there is no such field or no `From` domain.
pub fn verdict(header: &[u8], from_domain: &str) -> Option<Verdict> {
    let from_domain = from_domain
        .trim()
        .trim_end_matches('.')
        .to_ascii_lowercase();
    if from_domain.is_empty() {
        return None;
    }
    let value = first_field(header, "authentication-results")?;
    let value = strip_comments(&value);
    let mut dmarc = None;
    let mut aligned = false;
    // The first part is the server's name (authserv-id).
    for result in split_outside_quotes(&value, ';').into_iter().skip(1) {
        let mut tokens = result.split_whitespace();
        let Some((method, outcome)) = tokens.next().and_then(|t| t.split_once('=')) else {
            continue;
        };
        let method = method.split('/').next().unwrap_or("").to_ascii_lowercase();
        let outcome = outcome.to_ascii_lowercase();
        let property = |name: &str| {
            result.split_whitespace().skip(1).find_map(|token| {
                let (key, value) = token.split_once('=')?;
                key.eq_ignore_ascii_case(name).then(|| {
                    value
                        .trim_matches('"')
                        .trim_end_matches('.')
                        .to_ascii_lowercase()
                })
            })
        };
        match method.as_str() {
            "dmarc" => {
                let for_from = property("header.from").is_none_or(|d| d == from_domain);
                if outcome == "pass" && for_from {
                    aligned = true;
                }
                dmarc.get_or_insert(outcome);
            }
            "dkim" if outcome == "pass" => {
                let signer = property("header.d").or_else(|| {
                    property("header.i").map(|i| i.rsplit('@').next().unwrap_or("").to_owned())
                });
                if signer.is_some_and(|d| {
                    !d.is_empty()
                        && organizational_domain(&d) == organizational_domain(&from_domain)
                }) {
                    aligned = true;
                }
            }
            _ => {}
        }
    }
    Some(Verdict { dmarc, aligned })
}

/// The unfolded value of the first header field called `name` (lower
/// case). Reading stops at the end of the header.
fn first_field(header: &[u8], name: &str) -> Option<String> {
    let text = String::from_utf8_lossy(header);
    let mut value: Option<String> = None;
    for line in text.split('\n') {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.is_empty() {
            break;
        }
        let folded = line.starts_with([' ', '\t']);
        match &mut value {
            Some(value) if folded => {
                value.push(' ');
                value.push_str(line.trim());
            }
            Some(_) => break,
            None if folded => {}
            None => {
                if let Some((field, rest)) = line.split_once(':')
                    && field.trim().eq_ignore_ascii_case(name)
                {
                    value = Some(rest.trim().to_owned());
                }
            }
        }
    }
    value
}

/// `text` without its `(comments)`, which may nest; quoted strings keep
/// their parentheses.
fn strip_comments(text: &str) -> String {
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
            ')' if !quoted && depth > 0 => {
                depth -= 1;
                out.push(' ');
            }
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

/// `text` split at `separator`, except inside quoted strings.
fn split_outside_quotes(text: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut quoted = false;
    let mut start = 0;
    for (at, c) in text.char_indices() {
        match c {
            '"' => quoted = !quoted,
            c if c == separator && !quoted => {
                parts.push(&text[start..at]);
                start = at + c.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(&text[start..]);
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gmail_dmarc_pass() {
        let header = b"From: News <news@mail.shop.example>\r\n\
            Authentication-Results: mx.google.com;\r\n\
            \x20      dkim=pass header.i=@shop.example header.s=s1 header.b=abc;\r\n\
            \x20      spf=pass (google.com: domain of bounce@mail.shop.example designates 1.2.3.4 as permitted sender) smtp.mailfrom=bounce@mail.shop.example;\r\n\
            \x20      dmarc=pass (p=REJECT sp=REJECT dis=NONE) header.from=mail.shop.example\r\n\
            Subject: Hi\r\n";
        let found = verdict(header, "mail.shop.example").unwrap();
        assert_eq!(found.dmarc.as_deref(), Some("pass"));
        assert!(found.aligned);
        let json: serde_json::Value = serde_json::from_str(&found.to_json()).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "dmarc": "pass", "aligned": true })
        );
    }

    #[test]
    fn microsoft_without_authserv_id_and_aligned_dkim() {
        // Microsoft starts with the first result; DKIM of the parent
        // domain is aligned with a subdomain sender.
        let header = b"Authentication-Results: spf=none (sender IP is 1.2.3.4)\r\n\
            \x20smtp.mailfrom=x.example; dkim=pass (signature was verified)\r\n\
            \x20header.d=example.co.uk;dmarc=bestguesspass action=none\r\n\
            \x20header.from=news.example.co.uk;compauth=pass reason=109\r\n";
        let found = verdict(header, "news.example.co.uk").unwrap();
        assert_eq!(found.dmarc.as_deref(), Some("bestguesspass"));
        assert!(found.aligned);
    }

    #[test]
    fn forged_or_unaligned_senders_are_not_vouched_for() {
        // DKIM passed, but for another organization.
        let header =
            b"Authentication-Results: mx.example.net; dkim=pass header.d=bulk.example;\r\n\
            \x20dmarc=fail header.from=bank.example\r\n";
        let found = verdict(header, "bank.example").unwrap();
        assert_eq!(found.dmarc.as_deref(), Some("fail"));
        assert!(!found.aligned);

        // DMARC passed for a different From domain.
        let header =
            b"Authentication-Results: mx.example.net; dmarc=pass header.from=other.example\r\n";
        assert!(!verdict(header, "bank.example").unwrap().aligned);

        // Only the topmost header counts: the sender's own added below
        // the provider's is ignored.
        let header =
            b"Authentication-Results: mx.example.net; dmarc=none header.from=bank.example\r\n\
            Authentication-Results: mx.example.net; dmarc=pass header.from=bank.example\r\n";
        assert!(!verdict(header, "bank.example").unwrap().aligned);

        // A quoted ";" or "(" does not start a result.
        let header =
            b"Authentication-Results: mx.example.net; dkim=fail reason=\"a; dmarc=pass (x\"\r\n";
        assert!(!verdict(header, "bank.example").unwrap().aligned);

        // No header, no From domain, or the header after the body starts.
        assert_eq!(verdict(b"Subject: x\r\n", "bank.example"), None);
        assert_eq!(
            verdict(b"Authentication-Results: a; dmarc=pass\r\n", ""),
            None
        );
        assert_eq!(
            verdict(
                b"Subject: x\r\n\r\nAuthentication-Results: a; dmarc=pass\r\n",
                "bank.example"
            ),
            None
        );
    }
}
