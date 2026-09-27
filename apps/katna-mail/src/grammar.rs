// SPDX-License-Identifier: GPL-3.0-or-later

//! Grammar checking while writing, with Harper
//! (<https://github.com/automattic/harper>). English only: paragraphs
//! that do not look English are left alone. Spelling stays with
//! `spell.rs`. No GPUI here.
//!
//! Harper's dictionary takes about 135 MB and, once loaded, stays for
//! the life of the process. So it runs in a helper process, this binary
//! started with [`HELPER_FLAG`], which lives while a message is being
//! written: [`Helper`] starts it and stops it when dropped.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitCode, Stdio};
use std::sync::Mutex;

use katna_i18n::tr;
use serde_json::{Value, json};

use harper_core::linting::{LintGroup, Linter, Suggestion};
use harper_core::parsers::PlainEnglish;
use harper_core::spell::FstDictionary;
use harper_core::{Dialect, Document, language_detection};
use katna_ui::rich::{GrammarCheck, GrammarFix, GrammarIssue};

/// At most this many fixes in the menu.
const FIXES: usize = 4;

/// Starts the grammar helper, with the spelling language and the
/// interface's language setting after it.
pub const HELPER_FLAG: &str = "--grammar-helper";

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
                label: tr!("grammar-replace", words = text.trim().to_owned()),
                replacement: text,
            }
        }
        Suggestion::InsertAfter(chars) => {
            let text: String = chars.iter().collect();
            GrammarFix {
                label: tr!("grammar-add", words = text.trim().to_owned()),
                replacement: format!("{flagged}{text}"),
            }
        }
        Suggestion::Remove => GrammarFix {
            label: tr!("grammar-remove", words = flagged.trim().to_owned()),
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

/// Harper in a helper process; stops it when dropped.
pub struct Helper {
    io: Mutex<Option<HelperIo>>,
}

struct HelperIo {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
}

impl Helper {
    /// Starts the helper for the spelling `language`, naming fixes in the
    /// interface language `interface` (the setting; empty follows the
    /// desktop). It loads the rules while the first paragraph waits.
    pub fn start(language: &str, interface: &str) -> std::io::Result<Self> {
        // After an update replaced the binary, the new one.
        let exe = std::env::current_exe()?;
        let exe = exe
            .to_str()
            .and_then(|p| p.strip_suffix(" (deleted)"))
            .map_or(exe.clone(), std::path::PathBuf::from);
        let mut child = Command::new(exe)
            .arg(HELPER_FLAG)
            .arg(language)
            .arg(interface)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let (Some(input), Some(output)) = (child.stdin.take(), child.stdout.take()) else {
            let _ = child.kill();
            return Err(std::io::Error::other("no pipes to the grammar helper"));
        };
        Ok(Self {
            io: Mutex::new(Some(HelperIo {
                child,
                input,
                output: BufReader::new(output),
            })),
        })
    }
}

impl GrammarCheck for Helper {
    fn check(&self, text: &str) -> Vec<GrammarIssue> {
        let Ok(mut io) = self.io.lock() else {
            return Vec::new();
        };
        let Some(pipes) = io.as_mut() else {
            return Vec::new();
        };
        let mut line = String::new();
        let asked = writeln!(pipes.input, "{}", Value::from(text))
            .and_then(|()| pipes.input.flush())
            .and_then(|()| pipes.output.read_line(&mut line));
        match asked {
            Ok(n) if n > 0 => from_wire(&line),
            _ => {
                // It is gone; the message goes on without grammar.
                tracing::warn!("grammar helper stopped");
                *io = None;
                Vec::new()
            }
        }
    }
}

impl Drop for Helper {
    fn drop(&mut self) {
        if let Ok(mut io) = self.io.lock()
            && let Some(mut pipes) = io.take()
        {
            let _ = pipes.child.kill();
            let _ = pipes.child.wait();
        }
    }
}

/// The helper's life: a paragraph in, as a JSON string on a line, its
/// mistakes out, as a JSON array on a line, until the app closes the pipe.
pub fn run_helper(language: &str, interface: &str) -> ExitCode {
    katna_i18n::apply(interface);
    let grammar = Grammar::load(language);
    let mut out = std::io::stdout().lock();
    for line in std::io::stdin().lock().lines() {
        let Ok(line) = line else {
            break;
        };
        let text = serde_json::from_str::<String>(&line).unwrap_or_default();
        let answer = to_wire(&grammar.check(&text));
        if writeln!(out, "{answer}")
            .and_then(|()| out.flush())
            .is_err()
        {
            break;
        }
    }
    ExitCode::SUCCESS
}

fn to_wire(issues: &[GrammarIssue]) -> Value {
    issues
        .iter()
        .map(|i| {
            json!({
                "start": i.range.start,
                "end": i.range.end,
                "message": i.message,
                "fixes": i.fixes.iter().map(|f| json!([f.label, f.replacement])).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn from_wire(line: &str) -> Vec<GrammarIssue> {
    let Ok(Value::Array(issues)) = serde_json::from_str(line) else {
        return Vec::new();
    };
    let number = |v: &Value| v.as_u64().map(|n| n as usize);
    let text = |v: &Value| v.as_str().map(str::to_owned);
    issues
        .iter()
        .filter_map(|i| {
            let fixes = i["fixes"]
                .as_array()?
                .iter()
                .filter_map(|f| {
                    Some(GrammarFix {
                        label: text(&f[0])?,
                        replacement: text(&f[1])?,
                    })
                })
                .collect();
            Some(GrammarIssue {
                range: number(&i["start"])?..number(&i["end"])?,
                message: text(&i["message"])?,
                fixes,
            })
        })
        .collect()
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
    fn mistakes_cross_the_pipe_unchanged() {
        let issues = vec![GrammarIssue {
            range: 5..11,
            message: "Use \u{201c}have\u{201d} here.".into(),
            fixes: vec![GrammarFix {
                label: "\u{201c}have\u{201d}".into(),
                replacement: "have".into(),
            }],
        }];
        assert_eq!(from_wire(&to_wire(&issues).to_string()), issues);
        assert_eq!(from_wire("not json"), Vec::new());
        // One line each way, whatever the text holds.
        assert!(!Value::from("a\nb").to_string().contains('\n'));
    }

    #[test]
    fn dialects() {
        assert!(dialect("en_GB") == Dialect::British);
        assert!(dialect("") == Dialect::American);
        assert!(dialect("de_DE") == Dialect::American);
    }
}
