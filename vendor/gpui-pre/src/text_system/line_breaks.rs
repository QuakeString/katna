//! Katna: where a line of text may wrap.
//!
//! Line break opportunities follow UAX #14 (ICU4X's line segmenter, with
//! dictionaries for Thai, Lao, Khmer and Burmese, which are written without
//! spaces between words). When a single word is wider than the line, it is
//! split between grapheme clusters, never inside one, so an Indic conjunct,
//! an Arabic letter with its marks or an emoji sequence stays whole.

use icu_segmenter::{
    GraphemeClusterSegmenter, LineSegmenter,
    options::{LineBreakOptions, LineBreakStrictness},
};

const LINE_BREAK: u8 = 1;
const GRAPHEME: u8 = 2;

/// Break flags for each byte offset of a text, `0..=text.len()`.
pub(crate) struct Breaks(Vec<u8>);

impl Breaks {
    pub(crate) fn new(text: &str) -> Self {
        let mut flags = vec![0u8; text.len() + 1];
        let mut options = LineBreakOptions::default();
        // CSS `line-break: normal`, the web's default.
        options.strictness = Some(LineBreakStrictness::Normal);
        for ix in LineSegmenter::new_dictionary(options).segment_str(text) {
            flags[ix] |= LINE_BREAK;
        }
        for ix in GraphemeClusterSegmenter::new().segment_str(text) {
            flags[ix] |= GRAPHEME;
        }
        Self(flags)
    }

    /// Whether a line may wrap before the byte at `ix`.
    pub(crate) fn line_break_at(&self, ix: usize) -> bool {
        ix > 0 && self.0.get(ix).is_some_and(|flags| flags & LINE_BREAK != 0)
    }

    /// Whether a grapheme cluster starts at `ix`, so a word that does not
    /// fit can be split there.
    pub(crate) fn grapheme_at(&self, ix: usize) -> bool {
        self.0.get(ix).is_some_and(|flags| flags & GRAPHEME != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_breaks(text: &str) -> Vec<&str> {
        let breaks = Breaks::new(text);
        let mut parts = Vec::new();
        let mut start = 0;
        for ix in 1..=text.len() {
            if ix == text.len() || breaks.line_break_at(ix) {
                parts.push(&text[start..ix]);
                start = ix;
            }
        }
        parts
    }

    #[test]
    fn english_breaks_after_spaces_and_keeps_punctuation() {
        assert_eq!(
            line_breaks("Hello, world! (see this)"),
            ["Hello, ", "world! ", "(see ", "this)"]
        );
    }

    #[test]
    fn arabic_breaks_between_words_only() {
        assert_eq!(
            line_breaks("مرحبا بالعالم الجميل"),
            ["مرحبا ", "بالعالم ", "الجميل"]
        );
    }

    #[test]
    fn hindi_breaks_between_words_only() {
        assert_eq!(
            line_breaks("नमस्ते दुनिया क्षत्रिय"),
            ["नमस्ते ", "दुनिया ", "क्षत्रिय"]
        );
    }

    #[test]
    fn thai_breaks_between_dictionary_words() {
        // "Hello, how are you" without spaces, as Thai is written.
        let parts = line_breaks("สวัสดีครับคุณสบายดีไหม");
        assert!(parts.len() > 1, "{parts:?}");
        assert_eq!(parts[0], "สวัสดี");
    }

    #[test]
    fn japanese_breaks_between_characters_but_not_before_closing_marks() {
        let parts = line_breaks("日本語のテキスト。");
        assert!(parts.len() > 3, "{parts:?}");
        assert!(parts.last().unwrap().ends_with("ト。"), "{parts:?}");
    }

    #[test]
    fn graphemes_keep_marks_with_their_letter() {
        let text = "क्षि";
        let breaks = Breaks::new(text);
        let starts: Vec<usize> = (0..text.len())
            .filter(|ix| breaks.grapheme_at(*ix))
            .collect();
        // One cluster: क्ष (ka, virama, ssa) with the vowel sign ि.
        assert_eq!(starts, [0]);
    }
}
