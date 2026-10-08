// SPDX-License-Identifier: GPL-3.0-or-later

//! Text direction: which way a paragraph reads, from its first strong
//! character (Unicode's bidirectional algorithm, UAX #9, rules P2 and P3),
//! and isolation marks for a name or address set inside a sentence.

use unicode_bidi::{BidiClass, bidi_class};

/// Which way text reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    pub fn is_rtl(self) -> bool {
        self == Self::Rtl
    }

    /// The other direction.
    pub fn flipped(self) -> Self {
        match self {
            Self::Ltr => Self::Rtl,
            Self::Rtl => Self::Ltr,
        }
    }

    /// The value of an HTML `dir` attribute.
    pub fn html(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }

    /// Reads an HTML `dir` attribute; `auto` and anything else is `None`.
    pub fn from_html(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "ltr" => Some(Self::Ltr),
            "rtl" => Some(Self::Rtl),
            _ => None,
        }
    }
}

/// First Strong Isolate (U+2068).
pub const FSI: char = '\u{2068}';
/// Pop Directional Isolate (U+2069).
pub const PDI: char = '\u{2069}';

/// The direction of the paragraph `text` starts with: that of its first
/// strong character, skipping what sits between an isolate and its
/// matching PDI (UAX #9 P2–P3). `None` when it has none (digits,
/// punctuation, nothing), so the paragraph takes its surroundings'.
pub fn first_strong(text: &str) -> Option<Direction> {
    let mut isolates = 0usize;
    for ch in text.chars() {
        match bidi_class(ch) {
            BidiClass::LRI | BidiClass::RLI | BidiClass::FSI => isolates += 1,
            BidiClass::PDI => isolates = isolates.saturating_sub(1),
            // A paragraph ends at its separator.
            BidiClass::B => return None,
            _ if isolates > 0 => {}
            BidiClass::L => return Some(Direction::Ltr),
            BidiClass::R | BidiClass::AL => return Some(Direction::Rtl),
            _ => {}
        }
    }
    None
}

/// Whether `text` has any right-to-left letter.
pub fn has_rtl(text: &str) -> bool {
    text.chars()
        .any(|ch| matches!(bidi_class(ch), BidiClass::R | BidiClass::AL))
}

/// `text` between FSI and PDI, so a name or address keeps its own order
/// in a sentence of the other direction (an Arabic "Ravi wrote:").
pub fn isolate(text: &str) -> String {
    format!("{FSI}{text}{PDI}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_strong_character_decides() {
        assert_eq!(first_strong("Hello"), Some(Direction::Ltr));
        assert_eq!(first_strong("مرحبا Ravi"), Some(Direction::Rtl));
        assert_eq!(first_strong("שלום"), Some(Direction::Rtl));
        assert_eq!(first_strong("123, (Ravi) سلام"), Some(Direction::Ltr));
        assert_eq!(first_strong("  ٤٬٢٥٠ درهم"), Some(Direction::Rtl));
        assert_eq!(first_strong("12 – 34!"), None);
        assert_eq!(first_strong(""), None);
    }

    #[test]
    fn isolated_text_is_skipped() {
        assert_eq!(
            first_strong(&format!("{} كتب", isolate("Ravi"))),
            Some(Direction::Rtl)
        );
        assert_eq!(first_strong(&isolate("Ravi")), None);
        // A paragraph separator ends the paragraph.
        assert_eq!(first_strong("1\u{2029}abc"), None);
    }

    #[test]
    fn html_values() {
        assert_eq!(Direction::from_html(" RTL "), Some(Direction::Rtl));
        assert_eq!(Direction::from_html("ltr"), Some(Direction::Ltr));
        assert_eq!(Direction::from_html("auto"), None);
        assert_eq!(Direction::Rtl.html(), "rtl");
        assert!(has_rtl("Invoice فاتورة"));
        assert!(!has_rtl("Invoice ٤"));
    }
}
