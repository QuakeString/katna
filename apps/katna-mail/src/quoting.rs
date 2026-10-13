// SPDX-License-Identifier: GPL-3.0-or-later

//! The lines a reply and a forward put over the earlier message ("On …,
//! … wrote:", "---------- Forwarded message ---------"), in the language
//! of the interface, and telling them apart from what the user wrote.
//!
//! A name or address in a sentence of the other direction is set between
//! isolation marks (FSI…PDI), so "Ravi Kumar <ravi@invenia.in>" keeps its
//! order in an Arabic line; Fluent does it for right-to-left languages,
//! [`isolated`] for an Arabic or Hebrew name in a left-to-right one.

use katna_core::bidi::{FSI, PDI, has_rtl, isolate};
use katna_i18n::tr;

/// Stands for a value while a message's fixed text is compared.
const HOLE: &str = "\u{e000}";

/// `text` isolated when it reads right to left in a left-to-right
/// interface (a right-to-left one isolates every value already).
pub fn isolated(text: &str) -> String {
    if !katna_i18n::rtl() && has_rtl(text) {
        isolate(text)
    } else {
        text.to_owned()
    }
}

/// "On {date}, {from} wrote:".
pub fn reply_header(date: &str, from: &str) -> String {
    tr!(
        "compose-quote-header",
        date = isolated(date),
        from = isolated(from)
    )
}

/// The first line of a forwarded copy.
pub fn forward_header() -> String {
    tr!("compose-forward-header")
}

/// Whether `line` is the line over a quoted message: as English and
/// most mail apps write it, or as [`reply_header`] does now.
pub fn is_reply_header(line: &str) -> bool {
    let line = line.trim();
    (line.starts_with("On ") && line.ends_with("wrote:"))
        || fits(line, &tr!("compose-quote-header", date = HOLE, from = HOLE))
}

/// Whether `line` starts a forwarded copy, in English or the
/// interface's language.
pub fn is_forward_header(line: &str) -> bool {
    let line = line.trim();
    line.starts_with("---------- Forwarded message") || fits(line, &forward_header())
}

/// Whether `line` is `template` with something in each of its holes,
/// isolation marks aside.
fn fits(line: &str, template: &str) -> bool {
    let clean = |s: &str| -> String { s.chars().filter(|&c| c != FSI && c != PDI).collect() };
    let (line, template) = (clean(line), clean(template));
    let line = line.trim();
    let parts: Vec<&str> = template.trim().split(HOLE).collect();
    let [first, middle @ .., last] = parts.as_slice() else {
        return line == template.trim();
    };
    let Some(mut rest) = line
        .strip_prefix(first)
        .and_then(|rest| rest.strip_suffix(last))
    else {
        return false;
    };
    for part in middle.iter().filter(|p| !p.is_empty()) {
        match rest.find(part) {
            // Each hole holds something.
            Some(at) if at > 0 => rest = &rest[at + part.len()..],
            _ => return false,
        }
    }
    !rest.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates_fit_their_lines() {
        let template = format!("في {HOLE}، كتب {HOLE}:");
        assert!(fits(
            &format!("في الأربعاء، كتب {FSI}Ravi <ravi@x.in>{PDI}:"),
            &template
        ));
        assert!(!fits("في الأربعاء، كتب :", &template));
        assert!(!fits("Thanks, Ravi", &template));
        assert!(fits("--- Fwd ---", "--- Fwd ---"));
        assert!(!fits("--- Fwd --- more", "--- Fwd ---"));
    }

    #[test]
    fn english_headers_are_known() {
        assert!(is_reply_header(
            "On Tue, 25 Jun 2002, Kay <kay@x.org> wrote:"
        ));
        assert!(!is_reply_header("Once more, she wrote"));
        assert!(is_forward_header("---------- Forwarded message ---------"));
    }
}
