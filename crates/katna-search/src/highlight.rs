// SPDX-License-Identifier: GPL-3.0-or-later

//! Highlighted snippets that also mark other forms of the searched words.

use std::collections::{BTreeMap, HashSet};

use tantivy::schema::Field;
use tantivy::snippet::{Snippet, SnippetGenerator};

use crate::schema;

/// Stems share their first letters with the word (`contracted` →
/// `contract`), so only words starting like a searched stem are stemmed.
/// Snowball's few irregular words (`dying` → `die`) are not highlighted.
const PREFIX_CHARS: usize = 3;

/// Finds the words of a text whose stems were searched, then highlights
/// them with tantivy's snippet generator. Stemming only candidate words
/// keeps this nearly as fast as highlighting the words as written.
pub struct Highlighter {
    /// Searched stems and their weights (rarer words weigh more).
    stems: BTreeMap<String, f32>,
    /// The first letters of the stems (all of a short one).
    prefixes: Vec<String>,
    field: Field,
    max_chars: usize,
}

impl Highlighter {
    pub fn new(stems: BTreeMap<String, f32>, field: Field, max_chars: usize) -> Self {
        let mut prefixes: Vec<String> = stems.keys().map(|stem| prefix(stem).to_owned()).collect();
        prefixes.dedup();
        Self {
            stems,
            prefixes,
            field,
            max_chars,
        }
    }

    /// The best fragment of `text`, or `None` if no searched word is in it.
    pub fn snippet(&self, text: &str) -> Option<Snippet> {
        if self.stems.is_empty() {
            return None;
        }
        let words = self.matching_words(text);
        if words.is_empty() {
            return None;
        }
        let generator =
            SnippetGenerator::new(words, schema::analyzer(), self.field, self.max_chars);
        let snippet = generator.snippet(text);
        (!snippet.fragment().is_empty()).then_some(snippet)
    }

    /// The words of `text`, as written, whose stems were searched, with
    /// their stems' weights.
    fn matching_words(&self, text: &str) -> BTreeMap<String, f32> {
        let mut words = BTreeMap::new();
        let mut checked = HashSet::new();
        let mut stemmer = schema::stem_analyzer();
        let mut analyzer = schema::analyzer();
        let mut stream = analyzer.token_stream(text);
        while stream.advance() {
            let word = &stream.token().text;
            if !self.prefixes.iter().any(|p| word.starts_with(p.as_str()))
                || !checked.insert(word.clone())
            {
                continue;
            }
            let mut stems = stemmer.token_stream(word);
            if stems.advance()
                && let Some(weight) = self.stems.get(&stems.token().text)
            {
                words.insert(word.clone(), *weight);
            }
        }
        words
    }
}

fn prefix(word: &str) -> &str {
    match word.char_indices().nth(PREFIX_CHARS) {
        Some((end, _)) => &word[..end],
        None => word,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn marked(stems: &[&str], text: &str) -> Vec<String> {
        let stems = stems.iter().map(|s| (s.to_string(), 1.0)).collect();
        let field = schema::Fields::from_schema(&schema::build_schema())
            .unwrap()
            .body;
        let Some(snippet) = Highlighter::new(stems, field, 200).snippet(text) else {
            return Vec::new();
        };
        snippet
            .highlighted()
            .iter()
            .map(|range| snippet.fragment()[range.clone()].to_owned())
            .collect()
    }

    #[test]
    fn highlights_other_forms() {
        assert_eq!(
            marked(
                &["contract"],
                "Contracts were contracted by the contractor."
            ),
            ["Contracts", "contracted"]
        );
        assert_eq!(marked(&["go"], "go gone going"), ["go", "going"]);
        assert!(marked(&["price"], "nothing here").is_empty());
        assert!(marked(&[], "anything").is_empty());
    }
}
