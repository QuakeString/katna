// SPDX-License-Identifier: GPL-3.0-or-later

//! "Did you mean": the search text with each misspelled word replaced by
//! the nearest word that is in the mail.

use std::collections::HashMap;
use std::sync::OnceLock;

use levenshtein_automata::{DFA, Distance, LevenshteinAutomatonBuilder, SINK_STATE};
use tantivy::Searcher;
use tantivy::schema::Field;
use tantivy_fst::Automaton;

use crate::compile::max_typos;
use crate::error::Result;
use crate::schema::{Fields, tokens};

/// The text with each word of four or more letters that no field it
/// searches has replaced by the nearest word that one has: fewest typos,
/// then the most messages. `None` if every word is there, or no near word
/// is. With `as_you_type`, an unfinished last word counts as there if a
/// word starts with it, and is otherwise replaced by a word that starts
/// one typo from it (`haskin` → hasina). Operators, exclusions and phrases
/// are kept as typed.
pub(crate) fn suggest(
    searcher: &Searcher,
    fields: &Fields,
    input: &str,
    as_you_type: bool,
) -> Result<Option<String>> {
    let unfinished = as_you_type && !input.ends_with(char::is_whitespace);
    let pieces = pieces(input);
    let mut out = String::with_capacity(input.len());
    let mut copied = 0;
    let mut changed = false;
    for (index, &(start, piece)) in pieces.iter().enumerate() {
        let Some((offset, word, targets)) = word_of(fields, piece) else {
            continue;
        };
        let prefix = unfinished && index + 1 == pieces.len();
        if max_typos(&word) == 0 || targets.iter().any(|f| has(searcher, *f, &word, prefix)) {
            continue;
        }
        let Some(nearest) = nearest(searcher, &targets, &word, prefix)? else {
            continue;
        };
        let at = start + offset;
        out.push_str(&input[copied..at]);
        // Keep a capital first letter, as in a name.
        let mut chars = nearest.chars();
        match (input[at..].chars().next(), chars.next()) {
            (Some(typed), Some(first)) if typed.is_uppercase() => {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
            _ => out.push_str(&nearest),
        }
        copied = at + word.len();
        changed = true;
    }
    if !changed {
        return Ok(None);
    }
    out.push_str(&input[copied..]);
    Ok(Some(out))
}

/// Whitespace-separated pieces of `input` with their byte offsets. A quoted
/// phrase is one piece, so its words are left alone.
fn pieces(input: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut start = None;
    let mut quoted = false;
    for (at, c) in input.char_indices() {
        match (start, c) {
            (_, '"') => {
                quoted = !quoted;
                start.get_or_insert(at);
            }
            (Some(from), c) if c.is_whitespace() && !quoted => {
                out.push((from, &input[from..at]));
                start = None;
            }
            (None, c) if !c.is_whitespace() => start = Some(at),
            _ => {}
        }
    }
    if let Some(from) = start {
        out.push((from, &input[from..]));
    }
    out
}

/// The word of a piece that could be misspelled, its byte offset in the
/// piece and the fields it searches. `None` for operators other than
/// names and subject, exclusions, phrases, `OR` and anything that is not
/// one plain lower-case word as the index writes it.
fn word_of(fields: &Fields, piece: &str) -> Option<(usize, String, Vec<Field>)> {
    if piece.starts_with('-') || piece.contains('"') || piece == "OR" {
        return None;
    }
    let (offset, value, targets) = match piece.split_once(':') {
        Some((operator, value)) => {
            let targets = match operator.to_ascii_lowercase().as_str() {
                "from" => vec![fields.from],
                "to" => vec![fields.to, fields.cc, fields.bcc],
                "cc" => vec![fields.cc],
                "bcc" => vec![fields.bcc],
                "subject" => vec![fields.subject],
                _ => return None,
            };
            (operator.len() + 1, value, targets)
        }
        None => (
            0,
            piece,
            fields.free_text().iter().map(|(field, _)| *field).collect(),
        ),
    };
    let trimmed = value.trim_start_matches('(');
    let offset = offset + value.len() - trimmed.len();
    let value = trimmed.trim_end_matches(')');
    let [word] = tokens(value).try_into().ok()?;
    // Only replace what can be replaced in place.
    (value.to_lowercase() == word).then_some((offset, word, targets))
}

fn has(searcher: &Searcher, field: Field, word: &str, prefix: bool) -> bool {
    searcher.segment_readers().iter().any(|segment| {
        let Ok(index) = segment.inverted_index(field) else {
            return false;
        };
        let mut range = index.terms().range().ge(word.as_bytes());
        if prefix {
            let mut end = word.as_bytes().to_vec();
            if let Some(last) = end.last_mut() {
                *last = last.saturating_add(1);
            }
            range = range.lt(end);
        } else {
            range = range.le(word.as_bytes());
        }
        range.into_stream().is_ok_and(|mut stream| stream.advance())
    })
}

/// The word of `fields` fewest typos from `word`, then in the most
/// messages.
fn nearest(
    searcher: &Searcher,
    fields: &[Field],
    word: &str,
    prefix: bool,
) -> Result<Option<String>> {
    let dfa = if prefix {
        builder(1).build_prefix_dfa(word)
    } else {
        builder(max_typos(word)).build_dfa(word)
    };
    // Candidate → (typos, messages).
    let mut found: HashMap<String, (u8, u64)> = HashMap::new();
    for segment in searcher.segment_readers() {
        for &field in fields {
            let index = segment.inverted_index(field)?;
            let mut stream = index
                .terms()
                .search(Near(&dfa))
                .into_stream()
                .map_err(tantivy::TantivyError::from)?;
            while stream.advance() {
                let Ok(candidate) = std::str::from_utf8(stream.key()) else {
                    continue;
                };
                let Distance::Exact(typos) = dfa.eval(candidate) else {
                    continue;
                };
                let entry = found.entry(candidate.to_owned()).or_insert((typos, 0));
                entry.1 += u64::from(stream.value().doc_freq);
            }
        }
    }
    // Fewest typos, then the same first letter (people rarely get that
    // wrong: `kenet` → kenneth, not genex), then the most messages.
    let first = word.chars().next();
    let other_start = |candidate: &str| candidate.chars().next() != first;
    Ok(found
        .into_iter()
        .min_by(|(a, (a_typos, a_count)), (b, (b_typos, b_count))| {
            a_typos
                .cmp(b_typos)
                .then(other_start(a).cmp(&other_start(b)))
                .then(b_count.cmp(a_count))
                .then(a.cmp(b))
        })
        .map(|(word, _)| word))
}

/// Levenshtein automaton builders (a swap of neighbours is one typo); slow
/// to make, so made once.
pub(crate) fn builder(typos: u8) -> &'static LevenshteinAutomatonBuilder {
    static ONE: OnceLock<LevenshteinAutomatonBuilder> = OnceLock::new();
    static TWO: OnceLock<LevenshteinAutomatonBuilder> = OnceLock::new();
    if typos <= 1 {
        ONE.get_or_init(|| LevenshteinAutomatonBuilder::new(1, true))
    } else {
        TWO.get_or_init(|| LevenshteinAutomatonBuilder::new(2, true))
    }
}

/// Words within the automaton's typos, for the term dictionary.
struct Near<'a>(&'a DFA);

impl Automaton for Near<'_> {
    type State = u32;

    fn start(&self) -> u32 {
        self.0.initial_state()
    }

    fn is_match(&self, state: &u32) -> bool {
        matches!(self.0.distance(*state), Distance::Exact(_))
    }

    fn can_match(&self, state: &u32) -> bool {
        *state != SINK_STATE
    }

    fn accept(&self, state: &u32, byte: u8) -> u32 {
        self.0.transition(*state, byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_pieces() {
        assert_eq!(
            pieces(" from:ada  \"natural gas\" -x"),
            [(1, "from:ada"), (11, "\"natural gas\""), (25, "-x")]
        );
    }

    #[test]
    fn picks_words() {
        let fields = Fields::from_schema(&crate::schema::build_schema()).unwrap();
        let (offset, word, targets) = word_of(&fields, "from:Hasnia").unwrap();
        assert_eq!((offset, word.as_str()), (5, "hasnia"));
        assert_eq!(targets, [fields.from]);
        assert_eq!(word_of(&fields, "(haskina").unwrap().0, 1);
        assert!(word_of(&fields, "-haskina").is_none());
        assert!(word_of(&fields, "in:inbox").is_none());
        assert!(word_of(&fields, "kenneth.lay").is_none());
    }
}
