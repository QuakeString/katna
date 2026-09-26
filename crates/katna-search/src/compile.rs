// SPDX-License-Identifier: GPL-3.0-or-later

//! Turns a parsed [`Query`] into a tantivy query.

use std::ops::Bound;

use tantivy::query::{
    AllQuery, BooleanQuery, BoostQuery, ConstScoreQuery, EmptyQuery, FuzzyTermQuery, Occur,
    PhrasePrefixQuery, PhraseQuery, Query as TantivyQuery, RangeQuery, TermQuery,
};
use tantivy::schema::{Field, IndexRecordOption};
use tantivy::{Searcher, Term};

use crate::document::flag_term;
use crate::query::{Filter, Query, TextField};
use crate::schema::{Fields, stems, tokens};

type Boxed = Box<dyn TantivyQuery>;

/// How far words may be from what was typed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Fuzziness {
    /// Words match only as written (and their other forms).
    Exact,
    /// Names in From, To, Cc and Bcc also match with a typo or two, since
    /// names are what people misspell and misremember. The default.
    #[default]
    Names,
    /// Every free-text field matches with typos: the fallback when nothing
    /// matches otherwise.
    Everywhere,
}

/// A word being typed matches at most this many longer body words; fewer
/// for one or two letters, which match the most words and are replaced by
/// the next keystroke anyway. Other fields have far fewer words, so there
/// it matches all of them (`hasina b` finds Hasina Banu however many other
/// names start with b).
fn max_body_expansions(prefix: &str) -> u32 {
    match prefix.chars().count() {
        0 | 1 => 16,
        2 => 32,
        _ => 64,
    }
}

/// Typos allowed in a word of `word`'s length: none in short words, where
/// one typo is a different word, one in words of four letters and two in
/// longer ones (`kenet` finds `kenneth`). A swap of neighbouring letters
/// counts as one.
pub(crate) fn max_typos(word: &str) -> u8 {
    match word.chars().count() {
        0..=3 => 0,
        4 => 1,
        _ => 2,
    }
}

/// Weight of a near match relative to an exact one, so exact matches rank
/// first.
const FUZZY_BOOST: f32 = 0.3;

#[derive(Clone, Copy)]
struct Ctx<'a> {
    fields: &'a Fields,
    fuzziness: Fuzziness,
    searcher: Option<&'a Searcher>,
}

impl Ctx<'_> {
    /// Whether `field` has `word`, or with `prefix` a word starting with
    /// it. Without a searcher every word counts as missing.
    fn has(&self, field: Field, word: &str, prefix: bool) -> bool {
        let Some(searcher) = self.searcher else {
            return false;
        };
        searcher.segment_readers().iter().any(|segment| {
            let Ok(index) = segment.inverted_index(field) else {
                return false;
            };
            if !prefix {
                let term = Term::from_field_text(field, word);
                return index.get_term_info(&term).ok().flatten().is_some();
            }
            let terms = index.terms();
            let mut range = terms.range().ge(word.as_bytes());
            if let Some(end) = prefix_end(word) {
                range = range.lt(end);
            }
            range.into_stream().is_ok_and(|mut stream| stream.advance())
        })
    }
}

/// The first byte string after every string starting with `prefix`.
fn prefix_end(prefix: &str) -> Option<Vec<u8>> {
    let mut end = prefix.as_bytes().to_vec();
    while let Some(last) = end.pop() {
        if last < u8::MAX {
            end.push(last + 1);
            return Some(end);
        }
    }
    None
}

/// Compiles `query` for an index with `fields`, with [`Fuzziness::Names`].
pub fn compile(fields: &Fields, query: &Query) -> Boxed {
    compile_with(fields, query, Fuzziness::default(), None)
}

/// Compiles `query` for an index with `fields`. Excluded words (`-word`)
/// always match exactly. With `searcher`, a word only matches words a typo
/// away if none of the fields it searches has it as typed.
pub fn compile_with(
    fields: &Fields,
    query: &Query,
    fuzziness: Fuzziness,
    searcher: Option<&Searcher>,
) -> Boxed {
    compile_in(
        Ctx {
            fields,
            fuzziness,
            searcher,
        },
        query,
    )
}

fn compile_in(ctx: Ctx<'_>, query: &Query) -> Boxed {
    let fields = ctx.fields;
    match query {
        Query::All => Box::new(AllQuery),
        Query::And(items) => and(ctx, items),
        Query::Or(items) => Box::new(BooleanQuery::new(
            items
                .iter()
                .map(|item| (Occur::Should, compile_in(ctx, item)))
                .collect(),
        )),
        Query::Not(inner) => and(ctx, &[Query::Not(inner.clone())]),
        Query::Text { field, text } => text_query(ctx, *field, text, false),
        Query::Prefix { field, text } => text_query(ctx, *field, text, true),
        Query::Filter(filter) => Box::new(ConstScoreQuery::new(filter_query(fields, filter), 0.0)),
    }
}

/// All of `items`; exclusions become `MustNot`, and a query of only
/// exclusions matches everything else.
fn and(ctx: Ctx<'_>, items: &[Query]) -> Boxed {
    let exact = Ctx {
        fuzziness: Fuzziness::Exact,
        ..ctx
    };
    let mut clauses: Vec<(Occur, Boxed)> = Vec::with_capacity(items.len() + 1);
    for item in items {
        match item {
            Query::Not(inner) => clauses.push((Occur::MustNot, compile_in(exact, inner))),
            item => clauses.push((Occur::Must, compile_in(ctx, item))),
        }
    }
    if !clauses.iter().any(|(occur, _)| *occur == Occur::Must) {
        clauses.push((
            Occur::Must,
            Box::new(ConstScoreQuery::new(Box::new(AllQuery), 0.0)),
        ));
    }
    Box::new(BooleanQuery::new(clauses))
}

/// Words of `text` in `field`: a term, or a phrase for several words. With
/// `prefix`, the last word also matches longer words. One whole word in the
/// subject or body also matches its other forms, through the stemmed fields;
/// phrases and unfinished words match as written. One word also matches
/// words a typo or two away, in the fields `ctx.fuzziness` allows.
fn text_query(ctx: Ctx<'_>, field: TextField, text: &str, prefix: bool) -> Boxed {
    let fields = ctx.fields;
    let words = tokens(text);
    if words.is_empty() {
        return Box::new(AllQuery);
    }
    let mut targets: Vec<(Field, f32)> = match field {
        TextField::Any => fields.free_text().to_vec(),
        TextField::From => vec![(fields.from, 1.0)],
        TextField::To => vec![(fields.to, 1.0), (fields.cc, 1.0), (fields.bcc, 1.0)],
        TextField::Cc => vec![(fields.cc, 1.0)],
        TextField::Bcc => vec![(fields.bcc, 1.0)],
        TextField::Subject => vec![(fields.subject, 1.0)],
        TextField::Filename => vec![(fields.attachment, 1.0)],
        TextField::List => vec![(fields.list, 1.0)],
    };
    let searched: Vec<Field> = targets.iter().map(|(field, _)| *field).collect();
    // Fields where one word may also match with typos.
    let fuzzy_targets: Vec<(Field, f32)> = match ctx.fuzziness {
        _ if words.len() != 1 => Vec::new(),
        Fuzziness::Exact => Vec::new(),
        Fuzziness::Names => {
            let names = [fields.from, fields.to, fields.cc, fields.bcc];
            targets
                .iter()
                .copied()
                .filter(|(field, _)| names.contains(field))
                .collect()
        }
        Fuzziness::Everywhere => targets.clone(),
    };
    // (field, boost, stem) of the stemmed fields a whole word also searches.
    let mut stemmed: Vec<(Field, f32, String)> = Vec::new();
    if let (false, [_], [stem]) = (prefix, words.as_slice(), stems(text).as_slice()) {
        match field {
            TextField::Any => {
                // The stemmed body replaces the body as written: its postings
                // include the word's, and the body is the costliest field.
                targets.retain(|(field, _)| *field != fields.body);
                stemmed.extend(
                    fields
                        .stemmed()
                        .into_iter()
                        .map(|(field, boost)| (field, boost, stem.clone())),
                );
            }
            TextField::Subject => stemmed.push((fields.subject_stem, 0.5, stem.clone())),
            _ => {}
        }
    }
    let last = words.last().map_or("", String::as_str);
    let mut alternatives: Vec<(Occur, Boxed)> = targets
        .into_iter()
        .map(|(field, boost)| {
            let query: Boxed = if prefix {
                let mut query = PhrasePrefixQuery::new(
                    words
                        .iter()
                        .map(|word| Term::from_field_text(field, word))
                        .collect(),
                );
                // A phrase narrows the matches itself; a lone prefix in a
                // small field may match every word it starts.
                if field == fields.body || words.len() > 1 {
                    query.set_max_expansions(max_body_expansions(last));
                } else {
                    query.set_max_expansions(u32::MAX);
                }
                Box::new(query)
            } else if let [word] = words.as_slice() {
                term_query(field, word)
            } else {
                Box::new(PhraseQuery::new(
                    words
                        .iter()
                        .map(|word| Term::from_field_text(field, word))
                        .collect(),
                ))
            };
            (Occur::Should, boosted(query, boost))
        })
        .collect();
    alternatives.extend(
        stemmed
            .into_iter()
            .map(|(field, boost, stem)| (Occur::Should, boosted(term_query(field, &stem), boost))),
    );
    // A word being typed is loose already: one typo at most.
    let typos = if prefix {
        max_typos(last).min(1)
    } else {
        max_typos(last)
    };
    // Only a word that no searched field has as typed: one that is there
    // is most likely spelled right, and searching near common words is
    // costly.
    if typos > 0
        && !fuzzy_targets.is_empty()
        && !searched.iter().any(|field| ctx.has(*field, last, prefix))
    {
        alternatives.extend(fuzzy_targets.into_iter().map(|(field, boost)| {
            let term = Term::from_field_text(field, last);
            let query = if prefix {
                FuzzyTermQuery::new_prefix(term, typos, true)
            } else {
                FuzzyTermQuery::new(term, typos, true)
            };
            (Occur::Should, boosted(Box::new(query), boost * FUZZY_BOOST))
        }));
    }
    if alternatives.len() == 1 {
        return alternatives
            .pop()
            .map(|(_, q)| q)
            .unwrap_or_else(|| Box::new(EmptyQuery));
    }
    Box::new(BooleanQuery::new(alternatives))
}

fn term_query(field: Field, word: &str) -> Boxed {
    Box::new(TermQuery::new(
        Term::from_field_text(field, word),
        IndexRecordOption::WithFreqs,
    ))
}

fn boosted(query: Boxed, boost: f32) -> Boxed {
    if boost == 1.0 {
        query
    } else {
        Box::new(BoostQuery::new(query, boost))
    }
}

fn filter_query(fields: &Fields, filter: &Filter) -> Boxed {
    let term = |field: Field, value: &str| -> Boxed {
        Box::new(TermQuery::new(
            Term::from_field_text(field, value),
            IndexRecordOption::Basic,
        ))
    };
    match filter {
        Filter::HasAttachment => term(fields.has, "attachment"),
        Filter::Flag(flag, set) => {
            let Some(name) = flag_term(*flag) else {
                return Box::new(EmptyQuery);
            };
            if *set {
                term(fields.flag, name)
            } else {
                Box::new(BooleanQuery::new(vec![
                    (Occur::Must, Box::new(AllQuery)),
                    (Occur::MustNot, term(fields.flag, name)),
                ]))
            }
        }
        Filter::In(folder) => term(fields.folder, folder),
        Filter::Label(label) => term(fields.label, label),
        Filter::Org(domain) => term(fields.domain, domain),
        Filter::Before(time) => Box::new(RangeQuery::new(
            Bound::Unbounded,
            Bound::Excluded(Term::from_field_i64(fields.date, *time)),
        )),
        Filter::After(time) => Box::new(RangeQuery::new(
            Bound::Included(Term::from_field_i64(fields.date, *time)),
            Bound::Unbounded,
        )),
        Filter::Larger(bytes) => Box::new(RangeQuery::new(
            Bound::Excluded(Term::from_field_u64(fields.size, *bytes)),
            Bound::Unbounded,
        )),
        Filter::Smaller(bytes) => Box::new(RangeQuery::new(
            Bound::Unbounded,
            Bound::Excluded(Term::from_field_u64(fields.size, *bytes)),
        )),
    }
}
