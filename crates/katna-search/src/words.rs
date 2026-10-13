// SPDX-License-Identifier: GPL-3.0-or-later

//! The tokenizer of the text fields: words of letters and digits, as
//! tantivy's `SimpleTokenizer` splits them, except that runs of Thai, Lao,
//! Khmer and Burmese, which put no spaces between words, are split into
//! words with `icu_segmenter`'s dictionaries (`docs/ARCHITECTURE.md` §13.10).
//!
//! Only those four scripts' dictionaries are built in; Chinese and Japanese
//! keep the `SimpleTokenizer`'s runs (their dictionaries are optional
//! downloads, §17.3).

use std::collections::VecDeque;
use std::ops::Range;
use std::sync::LazyLock;

use icu_provider::prelude::*;
use icu_segmenter::provider::{
    Baked, SegmenterBreakGraphemeClusterV1, SegmenterBreakWordOverrideV1, SegmenterBreakWordV1,
    SegmenterDictionaryAutoV1, SegmenterDictionaryExtendedV1,
};
use icu_segmenter::{WordSegmenter, options::WordBreakOptions};
use tantivy::tokenizer::{Token, TokenStream, Tokenizer};

/// Splits text into words; see the module documentation.
#[derive(Clone, Default)]
pub struct WordTokenizer {
    token: Token,
}

/// The token stream of [`WordTokenizer`].
pub struct WordTokenStream<'a> {
    text: &'a str,
    /// Byte offset of the next character to look at.
    next: usize,
    /// Words of the last unspaced run not given out yet.
    pending: VecDeque<Range<usize>>,
    token: &'a mut Token,
}

impl Tokenizer for WordTokenizer {
    type TokenStream<'a> = WordTokenStream<'a>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> WordTokenStream<'a> {
        self.token.reset();
        WordTokenStream {
            text,
            next: 0,
            pending: VecDeque::new(),
            token: &mut self.token,
        }
    }
}

impl WordTokenStream<'_> {
    /// The next word's byte range.
    fn next_word(&mut self) -> Option<Range<usize>> {
        loop {
            if let Some(word) = self.pending.pop_front() {
                return Some(word);
            }
            let rest = &self.text[self.next..];
            let (start, first) = rest
                .char_indices()
                .find(|&(_, c)| is_unspaced(c) || c.is_alphanumeric())?;
            let start = self.next + start;
            let unspaced = is_unspaced(first);
            let len = self.text[start..]
                .char_indices()
                .find(|&(_, c)| {
                    if unspaced {
                        !is_unspaced(c)
                    } else {
                        is_unspaced(c) || !c.is_alphanumeric()
                    }
                })
                .map_or(self.text.len() - start, |(len, _)| len);
            let end = start + len;
            self.next = end;
            if !unspaced {
                return Some(start..end);
            }
            let segmenter = SEGMENTER.as_borrowed();
            let mut breaks = segmenter.segment_str(&self.text[start..end]);
            let mut from = 0;
            while let Some(to) = breaks.next() {
                if to > from && breaks.is_word_like() {
                    self.pending.push_back(start + from..start + to);
                }
                from = to;
            }
        }
    }
}

impl TokenStream for WordTokenStream<'_> {
    fn advance(&mut self) -> bool {
        self.token.text.clear();
        self.token.position = self.token.position.wrapping_add(1);
        let Some(word) = self.next_word() else {
            return false;
        };
        self.token.offset_from = word.start;
        self.token.offset_to = word.end;
        self.token.text.push_str(&self.text[word]);
        true
    }

    fn token(&self) -> &Token {
        self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}

/// Whether `c` belongs to Thai, Lao, Myanmar (Burmese) or Khmer, the scripts
/// written without spaces between words that the dictionaries cover. Marks
/// and signs count too: they are part of the words.
fn is_unspaced(c: char) -> bool {
    matches!(
        c,
        '\u{0E00}'..='\u{0EFF}' // Thai, Lao
            | '\u{1000}'..='\u{109F}' // Myanmar
            | '\u{A9E0}'..='\u{A9FF}' // Myanmar Extended-B
            | '\u{AA60}'..='\u{AA7F}' // Myanmar Extended-A
            | '\u{1780}'..='\u{17FF}' // Khmer
            | '\u{19E0}'..='\u{19FF}' // Khmer Symbols
    )
}

/// A word segmenter with the Thai, Lao, Khmer and Burmese dictionaries.
static SEGMENTER: LazyLock<WordSegmenter> = LazyLock::new(|| {
    WordSegmenter::try_new_dictionary_unstable(&SoutheastAsian, WordBreakOptions::default())
        .unwrap_or_else(|error| {
            // Cannot happen with compiled data; words then stay whole runs.
            tracing::error!(%error, "no word dictionaries");
            WordSegmenter::new_for_non_complex_scripts(Default::default()).static_to_owned()
        })
});

/// ICU4X's compiled data without the Chinese and Japanese dictionary, so
/// that the linker leaves it out.
struct SoutheastAsian;

macro_rules! from_baked {
    ($($marker:ty),*) => {$(
        impl DataProvider<$marker> for SoutheastAsian {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                Baked.load(req)
            }
        }
    )*};
}

from_baked!(
    SegmenterBreakWordV1,
    SegmenterBreakWordOverrideV1,
    SegmenterDictionaryExtendedV1,
    SegmenterBreakGraphemeClusterV1
);

impl DataProvider<SegmenterDictionaryAutoV1> for SoutheastAsian {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterDictionaryAutoV1>, DataError> {
        Err(DataErrorKind::IdentifierNotFound.with_req(SegmenterDictionaryAutoV1::INFO, req))
    }
}

#[cfg(test)]
mod tests {
    use tantivy::tokenizer::TextAnalyzer;

    use super::*;

    fn words(text: &str) -> Vec<(String, usize)> {
        let mut analyzer = TextAnalyzer::from(WordTokenizer::default());
        let mut stream = analyzer.token_stream(text);
        let mut out = Vec::new();
        while stream.advance() {
            let token = stream.token();
            assert_eq!(&text[token.offset_from..token.offset_to], token.text);
            out.push((token.text.clone(), token.position));
        }
        out
    }

    fn texts(text: &str) -> Vec<String> {
        words(text).into_iter().map(|(word, _)| word).collect()
    }

    #[test]
    fn splits_like_simple_tokenizer_elsewhere() {
        use tantivy::tokenizer::SimpleTokenizer;
        for text in [
            "Hello, happy tax payer!",
            "Kenneth.Lay@Enron.com 2024-01-02",
            "Café crème, naïve ÆSIR",
            "東京都に住んでいます 北京欢迎你 서울특별시",
            "Привет, мир! مرحبا بالعالم שלום עולם",
            "हिन्दी भाषा বাংলা",
            "",
            "  ...  ",
        ] {
            let mut simple = TextAnalyzer::from(SimpleTokenizer::default());
            let mut stream = simple.token_stream(text);
            let mut expected = Vec::new();
            while stream.advance() {
                expected.push((stream.token().text.clone(), stream.token().position));
            }
            assert_eq!(words(text), expected, "{text}");
        }
    }

    #[test]
    fn splits_unspaced_scripts_into_words() {
        // "I like to eat Thai food".
        let thai = texts("ฉันชอบกินอาหารไทย");
        assert!(thai.contains(&"อาหาร".to_owned()), "{thai:?}");
        assert!(thai.contains(&"ชอบ".to_owned()), "{thai:?}");
        assert_eq!(thai.concat(), "ฉันชอบกินอาหารไทย");
        // "I am going to the market".
        let lao = texts("ຂ້ອຍຈະໄປຕະຫຼາດ");
        assert!(lao.contains(&"ຕະຫຼາດ".to_owned()), "{lao:?}");
        assert!(lao.len() > 1, "{lao:?}");
        // "I like to eat rice".
        let khmer = texts("ខ្ញុំចូលចិត្តញ៉ាំបាយ។");
        assert!(khmer.contains(&"ចូលចិត្ត".to_owned()), "{khmer:?}");
        assert!(!khmer.iter().any(|word| word.contains('។')), "{khmer:?}");
        // "I go to school".
        let burmese = texts("ကျွန်တော်ကျောင်းသွားသည်");
        assert!(burmese.contains(&"ကျောင်း".to_owned()), "{burmese:?}");
        assert!(burmese.len() > 1, "{burmese:?}");
    }

    #[test]
    fn mixes_scripts_with_consecutive_positions() {
        let words = words("Katna ภาษาไทย 2026");
        let texts: Vec<&str> = words.iter().map(|(word, _)| word.as_str()).collect();
        assert_eq!(texts.first(), Some(&"Katna"));
        assert_eq!(texts.last(), Some(&"2026"));
        assert!(texts.contains(&"ไทย"), "{texts:?}");
        let positions: Vec<usize> = words.iter().map(|(_, position)| *position).collect();
        assert_eq!(positions, (0..words.len()).collect::<Vec<_>>());
    }
}
