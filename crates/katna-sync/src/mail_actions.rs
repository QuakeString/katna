// SPDX-License-Identifier: GPL-3.0-or-later

//! What a new-mail notification can offer to do with one message
//! (`docs/ARCHITECTURE.md` §15.1.3): copy a one-time code, or open a
//! verify, confirm or activate link. Found only where the mail says what
//! it is (a code beside "code", "OTP" or "passcode"; a link whose text or
//! address says "verify", "confirm" or "activate"), so ordinary mail gets
//! no button.

use mail_parser::MessageParser;

/// A shortcut for one message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shortcut {
    /// A one-time code, without the spaces or dashes it was written with.
    Code(String),
    /// A link to verify, confirm or activate something.
    Link {
        kind: LinkKind,
        url: String,
        /// Where it goes, as people read it: the host without `www.`.
        domain: String,
    },
}

/// What a link says it does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkKind {
    Verify,
    Confirm,
    Activate,
}

/// Lines read for a code, after the subject.
const LINES: usize = 80;
/// Words that say a mail is about signing in, proving an address or
/// activating an account. The
/// first three are the starter "One-time codes" mail rule's.
const ABOUT_CODES: &[&str] = &[
    "otp",
    "verification code",
    "one-time password",
    "one-time",
    "one time",
    "verification",
    "verify",
    "passcode",
    "security code",
    "login code",
    "log in",
    "login",
    "sign-in",
    "sign in",
    "confirm",
    "activat",
    "authentication",
    "2fa",
    "two-factor",
    "two-step",
    "access code",
    "your code",
    "code is",
    "this code",
    "the code",
];
/// Words a code is written beside.
const CODE_WORDS: &[&str] = &["code", "otp", "passcode", "pin"];
/// Lines with these hold other kinds of codes.
const NOT_CODES: &[&str] = &[
    "promo", "coupon", "discount", "voucher", "referral", "zip", "postal", "gift", "offer",
];
/// Link texts that are never the shortcut.
const NOT_LINKS: &[&str] = &[
    "unsubscribe",
    "not you",
    "didn't",
    "did not",
    "report",
    "privacy",
];

/// The shortcut for the message `raw`, if it has one. A code wins over a
/// link: mail with both asks for the code.
pub fn shortcut(raw: &[u8]) -> Option<Shortcut> {
    let message = MessageParser::default().parse(raw)?;
    let subject = message.subject().unwrap_or_default().to_owned();
    let text = message
        .body_text(0)
        .map(|t| t.replace("\r\n", "\n"))
        .unwrap_or_default();
    let html = message.body_html(0).map(|h| h.into_owned());
    find(&subject, &text, html.as_deref())
}

/// The shortcut in a mail with `subject`, plain `text` and maybe `html`.
pub fn find(subject: &str, text: &str, html: Option<&str>) -> Option<Shortcut> {
    let about = |words: &str| {
        let words = words.to_lowercase();
        ABOUT_CODES.iter().any(|w| words.contains(w))
    };
    let start: String = text.chars().take(4000).collect();
    let code_subject = CODE_WORDS
        .iter()
        .any(|w| has_word(&subject.to_lowercase(), w));
    if !code_subject && !about(subject) && !about(&start) {
        return None;
    }
    code(subject, text)
        .map(Shortcut::Code)
        .or_else(|| html.and_then(html_link).or_else(|| text_link(text)))
}

/// The code beside a code word in the subject or the first lines of
/// `text`, or on one of the three short lines after it.
fn code(subject: &str, text: &str) -> Option<String> {
    let lines: Vec<&str> = std::iter::once(subject)
        .chain(text.lines())
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .take(LINES + 1)
        .collect();
    for (at, line) in lines.iter().enumerate() {
        let lower = line.to_lowercase();
        if !CODE_WORDS.iter().any(|w| has_word(&lower, w)) || has_any(&lower, NOT_CODES) {
            continue;
        }
        if let Some(code) = candidates(line).into_iter().next() {
            return Some(code);
        }
        // "Your code is:" with the code on a line of its own.
        let after = lines.iter().skip(at + 1).take(3);
        for next in after.filter(|l| l.chars().count() <= 24) {
            if let Some(code) = candidates(next).into_iter().next() {
                return Some(code);
            }
        }
    }
    None
}

/// Whether `word` is in `text` on its own, not inside another word
/// ("pin" is not in "shipping").
fn has_word(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + word.len()..].chars().next();
        !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric)
    })
}

fn has_any(text: &str, words: &[&str]) -> bool {
    words.iter().any(|w| text.contains(w))
}

/// The codes on `line`, in order: 4 to 8 digits (also as two or three
/// groups such as `123 456` or `123-456`), or 5 to 8 capitals and digits
/// with both. Not years, times, amounts, phone numbers or parts of links.
fn candidates(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut found = Vec::new();
    let mut at = 0;
    while at < chars.len() {
        if !chars[at].is_ascii_alphanumeric() {
            at += 1;
            continue;
        }
        // A run of letters and digits, with single spaces or dashes
        // between groups of digits.
        let start = at;
        let mut end = at;
        while end < chars.len() && chars[end].is_ascii_alphanumeric() {
            end += 1;
        }
        let mut groups = vec![(start, end)];
        while end + 1 < chars.len()
            && (chars[end] == ' ' || chars[end] == '-')
            && chars[end + 1].is_ascii_digit()
            && chars[start..end].iter().all(char::is_ascii_digit)
        {
            let next = end + 1;
            let mut stop = next;
            while stop < chars.len() && chars[stop].is_ascii_alphanumeric() {
                stop += 1;
            }
            if !chars[next..stop].iter().all(char::is_ascii_digit) {
                break;
            }
            groups.push((next, stop));
            end = stop;
        }
        at = end;
        let before = start.checked_sub(1).map(|i| chars[i]);
        let after = chars.get(end).copied();
        let after_next = chars.get(end + 1).copied();
        // Amounts, references, links, addresses, percentages, times.
        if before.is_some_and(|c| "$€£₹¥#/@+=&?.".contains(c))
            || before == Some(':') && start >= 2 && chars[start - 2].is_ascii_digit()
            || after.is_some_and(|c| "/@%".contains(c))
            || (after == Some(':') || after == Some('.') || after == Some(','))
                && after_next.is_some_and(|c| c.is_ascii_digit())
        {
            continue;
        }
        let code: String = groups
            .iter()
            .flat_map(|&(a, b)| chars[a..b].iter())
            .collect();
        let digits = code.chars().all(|c| c.is_ascii_digit());
        let fits = if groups.len() > 1 {
            // Grouped digits: 6 to 8 in groups of 2 to 4.
            groups.iter().all(|&(a, b)| (2..=4).contains(&(b - a))) && (6..=8).contains(&code.len())
        } else if digits {
            (4..=8).contains(&code.len()) && !is_year(&code)
        } else {
            (5..=8).contains(&code.len())
                && code
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                && code.chars().any(|c| c.is_ascii_digit())
                && code.chars().any(|c| c.is_ascii_uppercase())
        };
        if fits {
            found.push(code);
        }
    }
    found
}

fn is_year(digits: &str) -> bool {
    digits.len() == 4
        && digits
            .parse::<u32>()
            .is_ok_and(|y| (1900..=2100).contains(&y))
}

/// What `words` (a link's text or address) say the link does.
fn link_kind(words: &str) -> Option<LinkKind> {
    let words = words.to_lowercase();
    if has_any(&words, NOT_LINKS) {
        return None;
    }
    if words.contains("activat") {
        Some(LinkKind::Activate)
    } else if words.contains("verif") || words.contains("validat") {
        Some(LinkKind::Verify)
    } else if words.contains("confirm") {
        Some(LinkKind::Confirm)
    } else {
        None
    }
}

/// The first link in `html` whose text says it verifies, confirms or
/// activates; else the first whose address says so.
fn html_link(html: &str) -> Option<Shortcut> {
    let mut by_address = None;
    for (href, text) in anchors(html) {
        if let Some(found) = link_kind(&text).and_then(|kind| link(kind, &href)) {
            return Some(found);
        }
        if by_address.is_none() && !has_any(&text.to_lowercase(), NOT_LINKS) {
            by_address = link_kind(path_of(&href)).and_then(|kind| link(kind, &href));
        }
    }
    by_address
}

/// The first web address in plain `text` whose path says it verifies,
/// confirms or activates, or that follows a line saying so.
fn text_link(text: &str) -> Option<Shortcut> {
    let lines: Vec<&str> = text.lines().take(200).collect();
    for (at, line) in lines.iter().enumerate() {
        for url in line
            .split(|c: char| c.is_whitespace() || "<>\"'()[]".contains(c))
            .filter(|w| w.starts_with("https://") || w.starts_with("http://"))
        {
            let url = url.trim_end_matches(['.', ',', ';', '!']);
            let said = lines[at.saturating_sub(2)..=at].join(" ");
            let kind = link_kind(path_of(url)).or_else(|| link_kind(&said));
            if let Some(found) = kind.and_then(|kind| link(kind, url)) {
                return Some(found);
            }
        }
    }
    None
}

/// `url`'s path and query: what is after the host.
fn path_of(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    rest.find(['/', '?', '#']).map_or("", |at| &rest[at..])
}

/// A link shortcut to `url`, when it is a web address with a host.
fn link(kind: LinkKind, url: &str) -> Option<Shortcut> {
    let url = url.trim();
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let authority = &rest[..rest.find(['/', '?', '#']).unwrap_or(rest.len())];
    // `user@host`: the host is after the last `@`, whatever comes before.
    let host = authority.rsplit('@').next()?;
    let host = host.split(':').next()?.to_lowercase();
    let host = host.trim_end_matches('.');
    let valid = host.contains('.')
        && host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.');
    if !valid || url.chars().any(char::is_whitespace) {
        return None;
    }
    Some(Shortcut::Link {
        kind,
        url: url.to_owned(),
        domain: host.strip_prefix("www.").unwrap_or(host).to_owned(),
    })
}

/// Each `<a href>` in `html`: its address and its text.
fn anchors(html: &str) -> Vec<(String, String)> {
    let lower = html.to_ascii_lowercase();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(at) = lower[from..].find("<a").map(|i| i + from) {
        from = at + 2;
        if !lower[from..].starts_with(|c: char| c.is_whitespace()) {
            continue;
        }
        let Some(tag_end) = lower[from..].find('>').map(|i| i + from) else {
            break;
        };
        let tag = &html[from..tag_end];
        let close = lower[tag_end..]
            .find("</a")
            .map_or(lower.len(), |i| i + tag_end);
        let text = without_tags(&html[tag_end + 1..close]);
        from = close;
        if let Some(href) = attribute(tag, "href") {
            found.push((entities(&href), entities(&text)));
        }
    }
    found
}

/// The value of attribute `name` in the inside of a tag.
fn attribute(tag: &str, name: &str) -> Option<String> {
    let lower = tag.to_ascii_lowercase();
    let mut from = 0;
    while let Some(at) = lower[from..].find(name).map(|i| i + from) {
        from = at + name.len();
        let starts = at == 0 || lower.as_bytes()[at - 1].is_ascii_whitespace();
        let rest = lower[from..].trim_start();
        if !starts || !rest.starts_with('=') {
            continue;
        }
        let value = tag[tag.len() - rest.len() + 1..].trim_start();
        return Some(match value.chars().next()? {
            quote @ ('"' | '\'') => value[1..].split(quote).next()?.to_owned(),
            _ => value
                .split(|c: char| c.is_whitespace() || c == '>')
                .next()?
                .to_owned(),
        });
    }
    None
}

/// `html` without its tags, spaces collapsed.
fn without_tags(html: &str) -> String {
    let mut text = String::new();
    let mut inside = false;
    for c in html.chars() {
        match c {
            '<' => inside = true,
            '>' => {
                inside = false;
                text.push(' ');
            }
            _ if !inside => text.push(c),
            _ => {}
        }
    }
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The few character references links and button texts use.
fn entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&#38;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code_of(subject: &str, text: &str) -> Option<String> {
        match find(subject, text, None) {
            Some(Shortcut::Code(code)) => Some(code),
            _ => None,
        }
    }

    #[test]
    fn codes_beside_their_words() {
        assert_eq!(
            code_of(
                "Your verification code",
                "Your code is 482913.\nIt expires in 10 minutes."
            ),
            Some("482913".into())
        );
        assert_eq!(
            code_of("482913 is your Instagram code", ""),
            Some("482913".into())
        );
        assert_eq!(
            code_of(
                "Sign in to Acme",
                "Use this code to sign in:\n\n  831 204\n\nThanks"
            ),
            Some("831204".into())
        );
        assert_eq!(
            code_of("Your one-time password", "OTP: 4827-1930 (valid for 5 min)"),
            Some("48271930".into())
        );
        assert_eq!(
            code_of("Confirm your sign-in", "Enter the code K7Q2XP in the app."),
            Some("K7Q2XP".into())
        );
    }

    #[test]
    fn other_numbers_are_not_codes() {
        // Ordinary mail: no word about codes.
        assert_eq!(
            code_of("Order 123456 shipped", "Thanks for shopping."),
            None
        );
        // A promo code is not a sign-in code.
        assert_eq!(
            code_of("Verify and save", "Promo code SAVE2026 at checkout"),
            None
        );
        // Years, times, amounts and links.
        assert_eq!(
            code_of(
                "Your login code",
                "Code sent 2026 at 10:45, $1500 off, see https://x.io/123456"
            ),
            None
        );
        // "pin" inside another word.
        assert_eq!(code_of("Verify shipping", "Shipping 123456 today"), None);
    }

    #[test]
    fn verify_links_show_where_they_go() {
        let html = r#"<p>Welcome!</p>
            <a href="https://example.com/unsubscribe?u=1">Unsubscribe</a>
            <A class="btn" HREF="https://www.Accounts.example.com/verify?t=a&amp;b=2"><span>Verify email</span></A>"#;
        assert_eq!(
            find("Verify your email address", "", Some(html)),
            Some(Shortcut::Link {
                kind: LinkKind::Verify,
                url: "https://www.Accounts.example.com/verify?t=a&b=2".into(),
                domain: "accounts.example.com".into(),
            })
        );
        // A tricky address shows the host it really goes to.
        let html = r#"<a href='https://bank.com@evil.example/x'>Activate account</a>"#;
        assert!(matches!(
            find("Activate your account", "", Some(html)),
            Some(Shortcut::Link { kind: LinkKind::Activate, domain, .. }) if domain == "evil.example"
        ));
        // In plain text, the link after the line that says so.
        let text = "Please confirm your subscription:\nhttps://list.example.org/c/9f8e7d.\n";
        assert!(matches!(
            find("Confirm subscription", text, None),
            Some(Shortcut::Link { kind: LinkKind::Confirm, domain, .. }) if domain == "list.example.org"
        ));
        // A code wins over a link.
        assert_eq!(
            find("Verify", "Your code is 123456", Some(html)),
            Some(Shortcut::Code("123456".into()))
        );
        // Ordinary links are not shortcuts.
        assert_eq!(
            find(
                "Login alert",
                "",
                Some(r#"<a href="https://x.com/help">Help</a>"#)
            ),
            None
        );
    }

    #[test]
    fn a_whole_message() {
        let raw = b"From: Acme <no-reply@acme.example>\r\nSubject: Your Acme verification code\r\nContent-Type: text/plain\r\n\r\nHi,\r\n\r\nYour verification code is:\r\n\r\n920 114\r\n";
        assert_eq!(shortcut(raw), Some(Shortcut::Code("920114".into())));
    }
}
