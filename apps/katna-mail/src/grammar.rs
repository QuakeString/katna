// SPDX-License-Identifier: GPL-3.0-or-later

//! Grammar checking while writing, with Harper
//! (<https://github.com/automattic/harper>). English only: paragraphs
//! that do not look English are left alone. Spelling stays with
//! `spell.rs`. No GPUI here.

use std::sync::Mutex;

use harper_core::linting::{LintGroup, Linter, Suggestion};
use harper_core::parsers::PlainEnglish;
use harper_core::spell::FstDictionary;
use harper_core::{Dialect, Document, language_detection};
use katna_ui::rich::{GrammarCheck, GrammarFix, GrammarIssue};

/// At most this many fixes in the menu.
const FIXES: usize = 4;

/// Harper's rules, set up for one kind of English.
pub struct Grammar {
    linter: Mutex<LintGroup>,
}

impl Grammar {
    /// Sets up the rules; takes most of a second, so not on the UI thread.
    /// `language` is the spelling dictionary's, as `en_GB`; British,
    /// Canadian, Australian and Indian English have their own spellings.
    pub fn load(language: &str) -> Self {
        let mut linter = LintGroup::new_curated(FstDictionary::curated(), dialect(language));
        // Spelling is underlined by the spell checker already.
        linter.config.set_rule_enabled("SpellCheck", false);
        Self {
            linter: Mutex::new(linter),
        }
    }
}

impl GrammarCheck for Grammar {
    fn check(&self, text: &str) -> Vec<GrammarIssue> {
        let dictionary = FstDictionary::curated();
        let document = Document::new(text, &PlainEnglish, &*dictionary);
        if !language_detection::is_doc_likely_english(&document, &*dictionary) {
            return Vec::new();
        }
        let Ok(mut linter) = self.linter.lock() else {
            return Vec::new();
        };
        let chars: Vec<char> = text.chars().collect();
        // Harper counts characters; the editor counts bytes.
        let mut bytes = Vec::with_capacity(chars.len() + 1);
        let mut at = 0;
        for c in &chars {
            bytes.push(at);
            at += c.len_utf8();
        }
        bytes.push(at);
        let mut issues: Vec<GrammarIssue> = linter
            .lint(&document)
            .into_iter()
            .filter(|lint| lint.span.end <= chars.len() && lint.span.start < lint.span.end)
            .map(|lint| {
                let flagged: String = chars[lint.span.start..lint.span.end].iter().collect();
                let fixes = lint
                    .suggestions
                    .iter()
                    .map(|s| fix(&flagged, s))
                    .filter(|f| f.replacement != flagged)
                    .fold(Vec::<GrammarFix>::new(), |mut all, f| {
                        // Harper can offer the same words twice, once with
                        // a space after.
                        if !all.iter().any(|a| a.label == f.label) {
                            all.push(f);
                        }
                        all
                    })
                    .into_iter()
                    .take(FIXES)
                    .collect();
                GrammarIssue {
                    range: bytes[lint.span.start]..bytes[lint.span.end],
                    message: plain_message(&lint.message),
                    fixes,
                }
            })
            .collect();
        issues.sort_by_key(|i| i.range.start);
        issues
    }
}

/// What a suggestion does to the flagged words.
fn fix(flagged: &str, suggestion: &Suggestion) -> GrammarFix {
    match suggestion {
        Suggestion::ReplaceWith(chars) => {
            let text: String = chars.iter().collect();
            GrammarFix {
                label: format!("\u{201c}{}\u{201d}", text.trim()),
                replacement: text,
            }
        }
        Suggestion::InsertAfter(chars) => {
            let text: String = chars.iter().collect();
            GrammarFix {
                label: format!("Add \u{201c}{}\u{201d}", text.trim()),
                replacement: format!("{flagged}{text}"),
            }
        }
        Suggestion::Remove => GrammarFix {
            label: format!("Remove \u{201c}{}\u{201d}", flagged.trim()),
            replacement: String::new(),
        },
    }
}

/// Harper writes messages in Markdown; the menu shows them as text, with
/// code spans in quotes.
fn plain_message(message: &str) -> String {
    let mut open = false;
    message
        .replace("**", "")
        .chars()
        .map(|c| match c {
            '`' => {
                open = !open;
                if open { '\u{201c}' } else { '\u{201d}' }
            }
            c => c,
        })
        .collect()
}

fn dialect(language: &str) -> Dialect {
    match language.get(..5).map(|l| l.replace('-', "_")).as_deref() {
        Some("en_GB") | Some("en_IE") | Some("en_NZ") | Some("en_ZA") => Dialect::British,
        Some("en_CA") => Dialect::Canadian,
        Some("en_AU") => Dialect::Australian,
        Some("en_IN") => Dialect::Indian,
        _ => Dialect::American,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_grammar_but_not_spelling() {
        let grammar = Grammar::load("en_US");
        let text = "Their going to the the stroe, it’s an plan.";
        let issues = grammar.check(text);
        let flagged: Vec<&str> = issues.iter().map(|i| &text[i.range.clone()]).collect();
        assert!(flagged.iter().any(|f| f.contains("the the")), "{flagged:?}");
        assert!(flagged.contains(&"an"), "{flagged:?}");
        // Misspelled words are the spell checker's.
        assert!(!flagged.contains(&"stroe"), "{flagged:?}");
        let article = issues
            .iter()
            .find(|i| &text[i.range.clone()] == "an")
            .unwrap();
        assert_eq!(article.fixes[0].replacement, "a");
    }

    #[test]
    fn leaves_other_languages_alone() {
        let grammar = Grammar::load("en_US");
        assert!(
            grammar
                .check("Wir treffen uns morgen um zehn Uhr im Büro.")
                .is_empty()
        );
    }

    #[test]
    fn messages_are_plain_text() {
        assert_eq!(
            plain_message("Did you mean `they're`?"),
            "Did you mean \u{201c}they're\u{201d}?"
        );
    }

    #[test]
    fn dialects() {
        assert!(dialect("en_GB") == Dialect::British);
        assert!(dialect("") == Dialect::American);
        assert!(dialect("de_DE") == Dialect::American);
    }
}
