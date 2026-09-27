// SPDX-License-Identifier: GPL-3.0-or-later

//! Recipient suggestions: the addresses in the mail, matched as the user
//! types a recipient and ranked like Gmail's: the people the user writes
//! to most, and most lately, come first.
//!
//! A [`ContactBook`] is built from [`katna_store::Correspondent`] rows
//! (slow to read, so apps keep the book and rebuild it now and then) and
//! answers each keystroke from memory. A typed word matches the start of
//! any word of a name or address (`sup` finds `api-support@…`), or the
//! start of the whole address; words of three or more letters may have a
//! typo, and of six or more two.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::Mutex;

use katna_store::Correspondent;
use levenshtein_automata::{DFA, Distance};
use serde::{Deserialize, Serialize};

use crate::suggest::builder;

/// How one account has written with an address.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exchange {
    pub account: i64,
    /// Messages the account sent to the address, and the newest (Unix
    /// seconds).
    pub sent: u32,
    pub last_sent: Option<i64>,
    /// Messages from the address.
    pub received: u32,
    pub last_received: Option<i64>,
    /// Messages to others the address was also on.
    pub copied: u32,
    pub last_copied: Option<i64>,
}

/// An address and how each account has written with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    /// Lower-cased address.
    pub email: String,
    pub name: Option<String>,
    pub exchanges: Vec<Exchange>,
}

/// One suggestion, with the parts of the name and address that matched
/// (byte ranges, for bold).
#[derive(Debug, Clone, PartialEq)]
pub struct Suggestion {
    pub email: String,
    pub name: Option<String>,
    pub name_marks: Vec<Range<usize>>,
    pub email_marks: Vec<Range<usize>>,
    pub score: f64,
}

/// The addresses to suggest, with their words ready for matching.
#[derive(Debug, Default)]
pub struct ContactBook {
    contacts: Vec<Contact>,
    /// Each contact's words, as a range of `words`.
    spans: Vec<Words>,
    words: Vec<Word>,
    /// The lower-cased words, names and addresses, one after another, so a
    /// search reads memory in order.
    text: String,
    /// The contacts with a word starting with each letter: every match
    /// starts a word, typos included, so only these are looked at.
    by_letter: HashMap<char, Vec<u32>>,
    /// Each contact's [`affinity`] for the last account and day asked.
    affinities: Mutex<Option<Affinities>>,
}

#[derive(Debug, Clone)]
struct Affinities {
    account: Option<i64>,
    day: i64,
    values: Vec<f64>,
}

impl Clone for ContactBook {
    fn clone(&self) -> Self {
        Self {
            contacts: self.contacts.clone(),
            spans: self.spans.clone(),
            words: self.words.clone(),
            text: self.text.clone(),
            by_letter: self.by_letter.clone(),
            affinities: Mutex::default(),
        }
    }
}

/// Where a contact's words are: `words[first..end]`, the name's first.
#[derive(Debug, Clone, Copy)]
struct Words {
    first: u32,
    names: u32,
    end: u32,
    /// The lower-cased address and name in `text`.
    email: (u32, u32),
    full_name: (u32, u32),
}

/// A lower-cased word in `text`, and where it starts in the shown name or
/// address (bytes).
#[derive(Debug, Clone, Copy)]
struct Word {
    text: (u32, u32),
    start: u32,
}

/// A contact's words, borrowed from the book.
struct View<'a> {
    text: &'a str,
    words: &'a [Word],
    names: usize,
    email: &'a str,
    full_name: &'a str,
}

impl<'a> View<'a> {
    fn word(&self, word: &Word) -> &'a str {
        &self.text[word.text.0 as usize..word.text.1 as usize]
    }
}

/// How well a query word matched a word: exact prefix or typos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Fit {
    Typos(u8),
    Prefix,
    /// The start of the whole name or address.
    Start,
}

impl Fit {
    fn weight(self) -> f64 {
        match self {
            Fit::Start => 1.0,
            Fit::Prefix => 0.8,
            Fit::Typos(1) => 0.4,
            Fit::Typos(_) => 0.2,
        }
    }
}

/// Half-life of how much writing with someone counts: a year.
const HALF_LIFE_DAYS: f64 = 365.0;
/// Old mail still counts this much.
const OLDEST: f64 = 0.05;
/// Mail in accounts other than the one writing counts this much.
const OTHER_ACCOUNT: f64 = 0.5;

impl ContactBook {
    /// Groups the rows by address.
    pub fn new(rows: Vec<Correspondent>) -> Self {
        let mut by_email: HashMap<String, usize> = HashMap::new();
        let mut contacts: Vec<Contact> = Vec::new();
        for row in rows {
            let exchange = Exchange {
                account: row.account.0,
                sent: row.sent,
                last_sent: row.last_sent,
                received: row.received,
                last_received: row.last_received,
                copied: row.copied,
                last_copied: row.last_copied,
            };
            let name = row.name.as_deref().and_then(clean_name);
            match by_email.get(&row.email) {
                Some(&ix) => {
                    let contact = &mut contacts[ix];
                    contact.exchanges.push(exchange);
                    if contact.name.is_none() {
                        contact.name = name;
                    }
                }
                None => {
                    by_email.insert(row.email.clone(), contacts.len());
                    contacts.push(Contact {
                        email: row.email,
                        name,
                        exchanges: vec![exchange],
                    });
                }
            }
        }
        Self::from_contacts(contacts)
    }

    /// A book of contacts saved with [`ContactBook::contacts`].
    pub fn from_contacts(contacts: Vec<Contact>) -> Self {
        let mut book = Self::default();
        for contact in contacts {
            book.push(contact);
        }
        book
    }

    fn push(&mut self, contact: Contact) -> usize {
        let ix = self.contacts.len();
        let add = |text: &mut String, piece: &str| {
            let start = text.len() as u32;
            text.push_str(piece);
            (start, text.len() as u32)
        };
        let name = contact.name.as_deref().unwrap_or("");
        let email = add(&mut self.text, &contact.email);
        let full_name = add(&mut self.text, &name.to_lowercase());
        let first = self.words.len() as u32;
        let mut letters = Vec::new();
        for (shown, is_name) in [(name, true), (contact.email.as_str(), false)] {
            for (start, lower) in split(shown) {
                letters.extend(lower.chars().next());
                let text = add(&mut self.text, &lower);
                self.words.push(Word {
                    text,
                    start: start as u32,
                });
            }
            if is_name {
                let names = self.words.len() as u32 - first;
                self.spans.push(Words {
                    first,
                    names,
                    end: 0,
                    email,
                    full_name,
                });
            }
        }
        self.spans[ix].end = self.words.len() as u32;
        letters.sort_unstable();
        letters.dedup();
        for letter in letters {
            self.by_letter.entry(letter).or_default().push(ix as u32);
        }
        self.contacts.push(contact);
        ix
    }

    fn view(&self, ix: usize) -> View<'_> {
        let span = self.spans[ix];
        let slice = |(a, b): (u32, u32)| &self.text[a as usize..b as usize];
        View {
            text: &self.text,
            words: &self.words[span.first as usize..span.end as usize],
            names: span.names as usize,
            email: slice(span.email),
            full_name: slice(span.full_name),
        }
    }

    pub fn contacts(&self) -> &[Contact] {
        &self.contacts
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contacts.is_empty()
    }

    /// Counts a message `account` just sent to `emails`, so they come up
    /// before the book is next rebuilt.
    pub fn note_sent(&mut self, account: i64, emails: &[(String, Option<String>)], now: i64) {
        for (email, name) in emails {
            let email = email.trim().to_lowercase();
            let ix = match self.contacts.iter().position(|c| c.email == email) {
                Some(ix) => ix,
                None => self.push(Contact {
                    email,
                    name: name.as_deref().and_then(clean_name),
                    exchanges: Vec::new(),
                }),
            };
            let exchanges = &mut self.contacts[ix].exchanges;
            let exchange = match exchanges.iter().position(|e| e.account == account) {
                Some(at) => &mut exchanges[at],
                None => {
                    exchanges.push(Exchange {
                        account,
                        ..Exchange::default()
                    });
                    exchanges.last_mut().expect("just pushed")
                }
            };
            exchange.sent += 1;
            exchange.last_sent = Some(now);
        }
        *self.affinities.get_mut().unwrap_or_else(|e| e.into_inner()) = None;
    }

    /// The best `limit` contacts for `query`, typed in a recipient field of
    /// `account` (`None`: all accounts alike) at `now` (Unix seconds),
    /// leaving out `skip` (addresses already in the field).
    pub fn suggest(
        &self,
        query: &str,
        account: Option<i64>,
        now: i64,
        skip: &[String],
        limit: usize,
    ) -> Vec<Suggestion> {
        let query = query
            .trim()
            .trim_start_matches(['"', '\'', '<'])
            .to_lowercase();
        let terms: Vec<Term> = query.split_whitespace().map(Term::new).collect();
        if terms.is_empty() || limit == 0 {
            return Vec::new();
        }
        let Some(candidates) = terms[0]
            .text
            .chars()
            .next()
            .and_then(|letter| self.by_letter.get(&letter))
        else {
            return Vec::new();
        };
        let mut affinities = self.affinities.lock().unwrap_or_else(|e| e.into_inner());
        let day = now.div_euclid(86_400);
        if !affinities
            .as_ref()
            .is_some_and(|a| a.account == account && a.day == day)
        {
            *affinities = Some(Affinities {
                account,
                day,
                values: self
                    .contacts
                    .iter()
                    .map(|c| affinity(c, account, now))
                    .collect(),
            });
        }
        let affinity = &affinities.as_ref().expect("just set").values;
        let mut found: Vec<(f64, usize)> = Vec::new();
        for &ix in candidates {
            let ix = ix as usize;
            let view = self.view(ix);
            let Some(fit) = matches(&terms, &query, &view, None) else {
                continue;
            };
            if skip.iter().any(|s| s.eq_ignore_ascii_case(view.email)) {
                continue;
            }
            found.push((fit * (1.0 + affinity[ix]), ix));
        }
        let order = |a: &(f64, usize), b: &(f64, usize)| {
            let (x, y) = (&self.contacts[a.1].email, &self.contacts[b.1].email);
            b.0.total_cmp(&a.0)
                .then_with(|| x.len().cmp(&y.len()))
                .then_with(|| x.cmp(y))
        };
        if found.len() > limit {
            found.select_nth_unstable_by(limit, order);
            found.truncate(limit);
        }
        found.sort_by(order);
        found
            .into_iter()
            .map(|(score, ix)| {
                let contact = &self.contacts[ix];
                let mut marks = (Vec::new(), Vec::new());
                matches(&terms, &query, &self.view(ix), Some(&mut marks));
                let name = contact.name.as_deref().unwrap_or("");
                let place = |shown_in: &str, marks: Vec<Mark>| {
                    marks
                        .into_iter()
                        .map(|(start, chars)| shown(shown_in, start, chars))
                        .collect()
                };
                Suggestion {
                    email: contact.email.clone(),
                    name: contact.name.clone(),
                    name_marks: place(name, marks.0),
                    email_marks: place(&contact.email, marks.1),
                    score,
                }
            })
            .collect()
    }
}

/// A typed word and its typo automaton.
struct Term {
    text: String,
    chars: usize,
    dfa: Option<DFA>,
}

impl Term {
    fn new(text: &str) -> Self {
        let chars = text.chars().count();
        let typos = match chars {
            0..=2 => 0,
            3..=5 => 1,
            _ => 2,
        };
        let dfa = (typos > 0).then(|| builder(typos).build_prefix_dfa(text));
        Self {
            text: text.to_owned(),
            chars,
            dfa,
        }
    }

    /// How `word` starts like this term, and how many of its characters
    /// match.
    fn fit(&self, word: &str) -> Option<(Fit, usize)> {
        if word.starts_with(&self.text) {
            return Some((Fit::Prefix, self.chars));
        }
        let dfa = self.dfa.as_ref()?;
        // A typo in the first letter is rare; skipping those keeps the
        // suggestions to the likely ones (and the search quick).
        if word.chars().next() != self.text.chars().next() {
            return None;
        }
        match dfa.eval(word) {
            Distance::Exact(typos) => Some((Fit::Typos(typos), self.chars)),
            Distance::AtLeast(_) => None,
        }
    }
}

/// A mark: where it starts in the shown text (bytes), and how many
/// lower-cased characters it covers.
type Mark = (usize, usize);

/// How well all of `terms` match the contact; with `marks`, also what
/// matched in the name and in the address.
fn matches(
    terms: &[Term],
    query: &str,
    view: &View<'_>,
    marks: Option<&mut (Vec<Mark>, Vec<Mark>)>,
) -> Option<f64> {
    let want = marks.is_some();
    let (mut name_marks, mut email_marks) = (Vec::new(), Vec::new());
    let done = |fit: Fit, name_marks, email_marks| {
        if let Some(marks) = marks {
            *marks = (name_marks, email_marks);
        }
        Some(fit.weight())
    };
    let chars = || query.chars().count();
    let name_start = || {
        view.words
            .first()
            .filter(|_| view.names > 0)
            .map(|w| w.start as usize)
    };
    // The query as the name or address starts: the best fit.
    if view.email.starts_with(query) {
        if want
            && view.full_name.starts_with(query)
            && let Some(start) = name_start()
        {
            name_marks.push((start, chars()));
        }
        return done(Fit::Start, name_marks, vec![(0, chars())]);
    }
    if terms.len() > 1 && view.full_name.starts_with(query) {
        name_marks.push((name_start()?, chars()));
        return done(Fit::Start, name_marks, email_marks);
    }
    // Else every term must start a word; the worst fit counts.
    let mut worst = Fit::Start;
    for term in terms {
        let mut best: Option<(Fit, usize, usize)> = None;
        for (at, word) in view.words.iter().enumerate() {
            let Some((mut fit, len)) = term.fit(view.word(word)) else {
                continue;
            };
            // A single word that starts the name or address.
            if fit == Fit::Prefix && (at == 0 || at == view.names) && terms.len() == 1 {
                fit = Fit::Start;
            }
            if best.is_none_or(|(b, ..)| fit > b) {
                best = Some((fit, at, len));
            }
        }
        let (fit, at, len) = best?;
        worst = worst.min(fit);
        if !want {
            continue;
        }
        let start = view.words[at].start as usize;
        if at < view.names {
            name_marks.push((start, len));
        } else {
            email_marks.push((start, len));
        }
    }
    // Mark the same letters in the address when a name word matched as its
    // start (Gmail marks both).
    if want
        && email_marks.is_empty()
        && let Some(term) = terms.first()
        && view.email.starts_with(&term.text)
    {
        email_marks.push((0, term.chars));
    }
    done(worst, name_marks, email_marks)
}

/// The byte range in `shown` of `chars` lower-cased characters from
/// `start`.
fn shown(shown: &str, start: usize, chars: usize) -> Range<usize> {
    let mut count = 0;
    let rest = shown.get(start..).unwrap_or("");
    for (ix, c) in rest.char_indices() {
        if count >= chars {
            return start..start + ix;
        }
        count += c.to_lowercase().count();
    }
    start..shown.len()
}

/// How much the user writes with the contact from `account`: sent mail
/// counts most, then mail received, then mail both were on; newer counts
/// more.
fn affinity(contact: &Contact, account: Option<i64>, now: i64) -> f64 {
    let recency = |last: Option<i64>| {
        let Some(last) = last else { return 0.0 };
        let days = (now - last).max(0) as f64 / 86_400.0;
        0.5f64.powf(days / HALF_LIFE_DAYS).max(OLDEST)
    };
    let part = |count: u32, last: Option<i64>| f64::from(count).ln_1p() * recency(last);
    contact
        .exchanges
        .iter()
        .map(|e| {
            let weight = match account {
                Some(account) if account != e.account => OTHER_ACCOUNT,
                _ => 1.0,
            };
            weight
                * (3.0 * part(e.sent, e.last_sent)
                    + part(e.received, e.last_received)
                    + 0.3 * part(e.copied, e.last_copied))
        })
        .sum()
}

/// A display name worth showing: without quotes, and not an address.
fn clean_name(name: &str) -> Option<String> {
    let name = name.trim().trim_matches(['"', '\'']).trim();
    (!name.is_empty() && !name.contains('@')).then(|| name.to_owned())
}

/// The words of a name or address, runs of letters and digits: where
/// each starts, and it lower-cased.
fn split(text: &str) -> Vec<(usize, String)> {
    let mut words = Vec::new();
    let mut current: Option<(usize, String)> = None;
    for (ix, c) in text.char_indices() {
        if c.is_alphanumeric() {
            current
                .get_or_insert_with(|| (ix, String::new()))
                .1
                .extend(c.to_lowercase());
        } else if let Some(word) = current.take() {
            words.push(word);
        }
    }
    words.extend(current);
    words
}

#[cfg(test)]
// Marks are lists of ranges, often of one.
#[allow(clippy::single_range_in_vec_init)]
mod tests {
    use katna_core::AccountId;

    use super::*;

    const DAY: i64 = 86_400;
    const NOW: i64 = 1_800_000_000;

    fn row(email: &str, name: Option<&str>, sent: u32, received: u32) -> Correspondent {
        Correspondent {
            account: AccountId(1),
            email: email.to_owned(),
            name: name.map(str::to_owned),
            sent,
            last_sent: (sent > 0).then_some(NOW - DAY),
            received,
            last_received: (received > 0).then_some(NOW - DAY),
            copied: 0,
            last_copied: None,
        }
    }

    fn book() -> ContactBook {
        ContactBook::new(vec![
            row("support@shop.example", None, 0, 3),
            row("api-support@compliance.example", None, 0, 1),
            row("info@mudra.example", Some("Support Ticket"), 0, 1),
            row("supayan.deb@siemens.example", Some("Supayan Deb"), 2, 5),
            row("kay.mann@enron.com", Some("\"Kay Mann\""), 40, 60),
            row("kate.symes@enron.com", Some("Kate Symes"), 1, 2),
            row("hasina.banu@example.org", Some("Hasina Banu"), 5, 5),
            row("hsana@example.org", None, 0, 0),
        ])
    }

    fn emails(found: &[Suggestion]) -> Vec<&str> {
        found.iter().map(|s| s.email.as_str()).collect()
    }

    #[test]
    fn matches_the_start_of_any_word() {
        let found = book().suggest("sup", Some(1), NOW, &[], 8);
        let got = emails(&found);
        // Written with most, first; every word that starts with "sup".
        assert_eq!(got[0], "supayan.deb@siemens.example");
        for email in [
            "support@shop.example",
            "api-support@compliance.example",
            "info@mudra.example",
        ] {
            assert!(got.contains(&email), "{email} in {got:?}");
        }
        assert!(!got.contains(&"kay.mann@enron.com"));
    }

    #[test]
    fn the_people_written_to_most_come_first() {
        let found = book().suggest("ka", Some(1), NOW, &[], 8);
        assert_eq!(
            emails(&found),
            ["kay.mann@enron.com", "kate.symes@enron.com"]
        );
        assert_eq!(found[0].name.as_deref(), Some("Kay Mann"));
    }

    #[test]
    fn forgives_typos() {
        // One wrong letter, a missing one, two swapped.
        for typed in ["hasuna", "hsina", "hasnia"] {
            let found = book().suggest(typed, Some(1), NOW, &[], 8);
            assert_eq!(
                emails(&found).first(),
                Some(&"hasina.banu@example.org"),
                "{typed}"
            );
        }
        // Two short letters must be right.
        assert!(book().suggest("hz", Some(1), NOW, &[], 8).is_empty());
    }

    #[test]
    fn several_words_and_addresses() {
        let found = book().suggest("kay m", Some(1), NOW, &[], 8);
        assert_eq!(emails(&found), ["kay.mann@enron.com"]);
        assert_eq!(found[0].name_marks, [0..5]);
        let found = book().suggest("mann kay", Some(1), NOW, &[], 8);
        assert_eq!(emails(&found), ["kay.mann@enron.com"]);
        let found = book().suggest("kay.mann@en", Some(1), NOW, &[], 8);
        assert_eq!(emails(&found), ["kay.mann@enron.com"]);
        assert_eq!(found[0].email_marks, [0..11]);
        // A domain.
        let found = book().suggest("siemens", Some(1), NOW, &[], 8);
        assert_eq!(emails(&found), ["supayan.deb@siemens.example"]);
    }

    #[test]
    fn marks_what_matched() {
        let found = book().suggest("sup", Some(1), NOW, &[], 8);
        let find = |email: &str| found.iter().find(|s| s.email == email).unwrap();
        assert_eq!(find("api-support@compliance.example").email_marks, [4..7]);
        let ticket = find("info@mudra.example");
        assert_eq!(ticket.name_marks, [0..3]);
        assert!(ticket.email_marks.is_empty());
        let supayan = find("supayan.deb@siemens.example");
        assert_eq!(supayan.name_marks, [0..3]);
        assert_eq!(supayan.email_marks, [0..3]);
    }

    #[test]
    fn prefers_the_account_writing_and_skips_added() {
        let mut rows = vec![
            row("a1@x.example", None, 1, 0),
            row("a2@x.example", None, 1, 0),
        ];
        rows[1].account = AccountId(2);
        let book = ContactBook::new(rows);
        let found = book.suggest("a", Some(2), NOW, &[], 8);
        assert_eq!(emails(&found), ["a2@x.example", "a1@x.example"]);
        let found = book.suggest("a", Some(2), NOW, &["A2@x.example".to_owned()], 8);
        assert_eq!(emails(&found), ["a1@x.example"]);
    }

    #[test]
    fn newer_mail_counts_more() {
        let mut old = row("old@x.example", None, 5, 0);
        old.last_sent = Some(NOW - 3 * 365 * DAY);
        let new = row("new@x.example", None, 5, 0);
        let book = ContactBook::new(vec![old, new]);
        assert_eq!(
            emails(&book.suggest("x", None, NOW, &[], 8)),
            ["new@x.example", "old@x.example"]
        );
    }

    #[test]
    fn sent_mail_is_counted_at_once() {
        let mut book = book();
        book.note_sent(
            1,
            &[(
                "New@Person.example".to_owned(),
                Some("New Person".to_owned()),
            )],
            NOW,
        );
        let found = book.suggest("new p", Some(1), NOW, &[], 8);
        assert_eq!(emails(&found), ["new@person.example"]);
        book.note_sent(1, &[("hsana@example.org".to_owned(), None)], NOW);
        book.note_sent(1, &[("hsana@example.org".to_owned(), None)], NOW);
        let hsana = &book
            .contacts()
            .iter()
            .find(|c| c.email == "hsana@example.org")
            .unwrap();
        assert_eq!(hsana.exchanges[0].sent, 2);
    }

    #[test]
    fn marks_non_ascii_names() {
        let book = ContactBook::new(vec![row("jo@x.example", Some("Jörg Ødegård"), 1, 0)]);
        let found = book.suggest("øde", None, NOW, &[], 8);
        assert_eq!(found[0].name_marks, [6..10]);
        assert_eq!(&"Jörg Ødegård"[found[0].name_marks[0].clone()], "Øde");
    }
}
