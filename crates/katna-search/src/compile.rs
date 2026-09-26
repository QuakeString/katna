// SPDX-License-Identifier: GPL-3.0-or-later

//! Turns a parsed [`Query`] into a tantivy query.

use std::ops::Bound;

use tantivy::Term;
use tantivy::query::{
    AllQuery, BooleanQuery, BoostQuery, ConstScoreQuery, EmptyQuery, Occur, PhraseQuery,
    Query as TantivyQuery, RangeQuery, TermQuery,
};
use tantivy::schema::{Field, IndexRecordOption};

use crate::document::flag_term;
use crate::query::{Filter, Query, TextField};
use crate::schema::{Fields, tokens};

type Boxed = Box<dyn TantivyQuery>;

/// Compiles `query` for an index with `fields`.
pub fn compile(fields: &Fields, query: &Query) -> Boxed {
    match query {
        Query::All => Box::new(AllQuery),
        Query::And(items) => and(fields, items),
        Query::Or(items) => Box::new(BooleanQuery::new(
            items
                .iter()
                .map(|item| (Occur::Should, compile(fields, item)))
                .collect(),
        )),
        Query::Not(inner) => not(fields, inner),
        Query::Text { field, text } => text_query(fields, *field, text),
        Query::Filter(filter) => Box::new(ConstScoreQuery::new(filter_query(fields, filter), 0.0)),
    }
}

/// All of `items`; exclusions become `MustNot`, and a query of only
/// exclusions matches everything else.
fn and(fields: &Fields, items: &[Query]) -> Boxed {
    let mut clauses: Vec<(Occur, Boxed)> = Vec::with_capacity(items.len() + 1);
    for item in items {
        match item {
            Query::Not(inner) => clauses.push((Occur::MustNot, compile(fields, inner))),
            item => clauses.push((Occur::Must, compile(fields, item))),
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

fn not(fields: &Fields, inner: &Query) -> Boxed {
    and(fields, &[Query::Not(Box::new(inner.clone()))])
}

fn text_query(fields: &Fields, field: TextField, text: &str) -> Boxed {
    let words = tokens(text);
    if words.is_empty() {
        return Box::new(AllQuery);
    }
    let targets: Vec<(Field, f32)> = match field {
        TextField::Any => fields.free_text().to_vec(),
        TextField::From => vec![(fields.from, 1.0)],
        TextField::To => vec![(fields.to, 1.0), (fields.cc, 1.0), (fields.bcc, 1.0)],
        TextField::Cc => vec![(fields.cc, 1.0)],
        TextField::Bcc => vec![(fields.bcc, 1.0)],
        TextField::Subject => vec![(fields.subject, 1.0)],
        TextField::Filename => vec![(fields.attachment, 1.0)],
        TextField::List => vec![(fields.list, 1.0)],
    };
    let mut alternatives: Vec<(Occur, Boxed)> = targets
        .into_iter()
        .map(|(field, boost)| {
            let query: Boxed = if let [word] = words.as_slice() {
                Box::new(TermQuery::new(
                    Term::from_field_text(field, word),
                    IndexRecordOption::WithFreqs,
                ))
            } else {
                Box::new(PhraseQuery::new(
                    words
                        .iter()
                        .map(|word| Term::from_field_text(field, word))
                        .collect(),
                ))
            };
            let query = if boost == 1.0 {
                query
            } else {
                Box::new(BoostQuery::new(query, boost))
            };
            (Occur::Should, query)
        })
        .collect();
    if alternatives.len() == 1 {
        return alternatives
            .pop()
            .map(|(_, q)| q)
            .unwrap_or_else(|| Box::new(EmptyQuery));
    }
    Box::new(BooleanQuery::new(alternatives))
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
