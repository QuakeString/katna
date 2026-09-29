// SPDX-License-Identifier: GPL-3.0-or-later

//! Subject normalization for threading (`docs/ARCHITECTURE.md` §6.5).
//!
//! `Re: [list] AW: Budget` and `budget` are the same conversation. The
//! normalized form drops reply and forward prefixes in many languages, list
//! tags in brackets, case and extra whitespace.

/// Reply and forward prefixes, lower case, without the colon.
const PREFIXES: &[&str] = &[
    "re",
    "fw",
    "fwd",
    "aw",
    "sv",
    "vs",
    "wg",
    "antw",
    "antwort",
    "rif",
    "r",
    "i",
    "tr",
    "enc",
    "res",
    "rv",
    "odp",
    "pd",
    "ynt",
    "vl",
    "vb",
    "fs",
    "doorst",
    "továbbítás",
    "vá",
    "atb",
    "ang",
    "回复",
    "回覆",
    "答复",
    "转发",
    "轉寄",
    "转",
];

/// A normalized subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedSubject {
    /// Lower case, prefixes and list tags removed, whitespace collapsed.
    pub text: String,
    /// It had at least one reply or forward prefix (`Re:`, `Fwd:`, …).
    pub is_reply: bool,
}

/// Normalizes `subject` for threading.
pub fn normalize_subject(subject: &str) -> NormalizedSubject {
    let mut rest = subject.trim();
    let mut is_reply = false;
    loop {
        if let Some(after) = strip_list_tag(rest) {
            rest = after.trim_start();
            continue;
        }
        if let Some(after) = strip_prefix(rest) {
            rest = after.trim_start();
            is_reply = true;
            continue;
        }
        break;
    }
    let text = rest
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    NormalizedSubject { text, is_reply }
}

/// `subject` without its reply and forward prefixes, case kept:
/// `Re: Fwd: Budget review` → `Budget review`.
pub fn without_reply_prefixes(subject: &str) -> &str {
    let mut rest = subject.trim();
    while let Some(after) = strip_prefix(rest) {
        rest = after.trim_start();
    }
    rest
}

/// `[list-name] rest` → `rest`. Tags longer than 64 bytes are subject text.
fn strip_list_tag(text: &str) -> Option<&str> {
    let inner = text.strip_prefix('[')?;
    let end = inner.find(']')?;
    (end <= 64 && !inner[..end].contains('[')).then(|| &inner[end + 1..])
}

/// `Re: rest`, `RE[2]: rest`, `Re(3): rest`, `Fwd：rest` → `rest`.
fn strip_prefix(text: &str) -> Option<&str> {
    let colon = text.find([':', '：'])?;
    let (head, tail) = text.split_at(colon);
    let tail = &tail[tail.chars().next()?.len_utf8()..];
    let head = head.trim_end();
    // An optional reply count: `[2]`, `(2)` or `*2`.
    let word = match head.find(['[', '(', '*']) {
        Some(at) => {
            let count = head[at..].trim_matches(['[', ']', '(', ')', '*']);
            if count.is_empty() || !count.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            &head[..at]
        }
        None => head,
    };
    let word = word.trim_end().to_lowercase();
    PREFIXES.contains(&word.as_str()).then_some(tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn norm(subject: &str) -> (String, bool) {
        let n = normalize_subject(subject);
        (n.text, n.is_reply)
    }

    #[test]
    fn strips_reply_and_forward_prefixes() {
        assert_eq!(norm("Budget  forecast"), ("budget forecast".into(), false));
        assert_eq!(norm("Re: Budget"), ("budget".into(), true));
        assert_eq!(norm("RE: re: Fwd: Budget"), ("budget".into(), true));
        assert_eq!(norm("Re[2]: Budget"), ("budget".into(), true));
        assert_eq!(norm("Re (3): Budget"), ("budget".into(), true));
        assert_eq!(norm("AW: WG: Budget"), ("budget".into(), true));
        assert_eq!(norm("SV: Budget"), ("budget".into(), true));
        assert_eq!(norm("Fw: Budget"), ("budget".into(), true));
        assert_eq!(norm("回复：预算"), ("预算".into(), true));
    }

    #[test]
    fn drops_prefixes_but_keeps_case() {
        assert_eq!(
            without_reply_prefixes("Re: Fwd: Budget Review"),
            "Budget Review"
        );
        assert_eq!(without_reply_prefixes("  Budget: Q3 "), "Budget: Q3");
        assert_eq!(without_reply_prefixes("AW: [team] Plan"), "[team] Plan");
    }

    #[test]
    fn strips_list_tags_anywhere_in_the_prefix() {
        assert_eq!(
            norm("[rust-users] Re: Borrow checker"),
            ("borrow checker".into(), true)
        );
        assert_eq!(
            norm("Re: [rust-users] Borrow checker"),
            ("borrow checker".into(), true)
        );
        assert_eq!(
            norm("[announce] Release 1.0"),
            ("release 1.0".into(), false)
        );
    }

    #[test]
    fn keeps_ordinary_colons() {
        assert_eq!(
            norm("Meeting: 3pm today"),
            ("meeting: 3pm today".into(), false)
        );
        assert_eq!(norm("Re: Note: this"), ("note: this".into(), true));
        assert_eq!(norm("Rex: hello"), ("rex: hello".into(), false));
        assert_eq!(norm("Re[x]: hello"), ("re[x]: hello".into(), false));
        assert_eq!(norm(""), (String::new(), false));
        assert_eq!(norm("Re:"), (String::new(), true));
    }
}
