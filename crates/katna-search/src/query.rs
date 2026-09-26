// SPDX-License-Identifier: GPL-3.0-or-later

//! The query language (`docs/ARCHITECTURE.md` §7.2), Gmail-compatible where
//! possible:
//!
//! ```text
//! budget  "exact phrase"  -exclude  a OR b  ( … )
//! from:  to:  cc:  bcc:  subject:  filename:  list:  org:
//! has:attachment  in:inbox  label:x  is:unread|read|starred|answered|draft
//! before:2001-05-01  after:  older_than:30d  newer_than:2w  larger:5M  smaller:
//! ```
//!
//! Words are ANDed. Text operators take a word, a `"phrase"` or a group:
//! `from:(alice OR bob)`. Parsing is lenient like Gmail's: an unknown
//! `word:value` is plain text and stray parentheses are ignored. Only values
//! that cannot mean anything (`before:yesterday`, `larger:big`) are errors.

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use katna_store::MessageFlags;

/// Parentheses nest at most this deep, so hostile input cannot overflow the stack.
pub const MAX_DEPTH: usize = 32;

/// A parsed query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    /// Every message (the empty query).
    All,
    And(Vec<Query>),
    Or(Vec<Query>),
    Not(Box<Query>),
    /// Words in `field`; more than one word must appear as a phrase.
    Text {
        field: TextField,
        text: String,
    },
    /// Like [`Text`](Query::Text), but the last word may be the start of a
    /// longer word: what the user is still typing.
    Prefix {
        field: TextField,
        text: String,
    },
    Filter(Filter),
}

/// Where a [`Query::Text`] looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextField {
    /// Subject, participants, attachment names, list and body.
    Any,
    From,
    /// To, Cc or Bcc (like Gmail).
    To,
    Cc,
    Bcc,
    Subject,
    Filename,
    List,
}

/// A condition that does not rank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    HasAttachment,
    /// A system flag is set (`true`) or not set.
    Flag(MessageFlags, bool),
    /// Folder path, one of its components, or its role.
    In(String),
    Label(String),
    /// A participant's domain (organizations come in Phase 2).
    Org(String),
    /// Date before this Unix time.
    Before(i64),
    /// Date at or after this Unix time.
    After(i64),
    /// Larger than this many bytes.
    Larger(u64),
    /// Smaller than this many bytes.
    Smaller(u64),
}

/// Why a query could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// An operator value that means nothing, such as `before:soon`.
    InvalidValue { operator: String, value: String },
    /// Parentheses nested deeper than [`MAX_DEPTH`].
    TooDeep,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue { operator, value } => {
                write!(f, "invalid value {value:?} for {operator}:")?;
                f.write_str(match operator.as_str() {
                    "before" | "after" => " (use YYYY-MM-DD)",
                    "older_than" | "newer_than" => " (use a number and d, w, m or y, like 30d)",
                    "larger" | "smaller" => " (use bytes or K, M, G, like 5M)",
                    "is" => " (use unread, read, starred, answered, draft or forwarded)",
                    "has" => " (use attachment)",
                    _ => "",
                })
            }
            Self::TooDeep => write!(f, "parentheses nested more than {MAX_DEPTH} deep"),
        }
    }
}

impl std::error::Error for ParseError {}

impl Query {
    /// Parses `input`; relative dates (`newer_than:`) count from now.
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs() as i64);
        Self::parse_at(input, now)
    }

    /// Parses `input`; relative dates count from `now` (Unix seconds).
    pub fn parse_at(input: &str, now: i64) -> Result<Self, ParseError> {
        let mut parser = Parser {
            input,
            pos: 0,
            now,
            depth: 0,
        };
        let mut items = Vec::new();
        loop {
            let query = parser.and(None)?;
            if query != Query::All {
                items.push(query);
            }
            // A `)` without `(`: skip it and go on.
            parser.skip_space();
            if parser.eat(')') {
                continue;
            }
            break;
        }
        Ok(and(items))
    }

    /// Parses `input` while the user is still typing it: when it ends in a
    /// word, that word also matches longer words (`budg` finds `budget`).
    /// Relative dates count from `now` (Unix seconds).
    pub fn parse_as_you_type(input: &str, now: i64) -> Result<Self, ParseError> {
        let mut query = Self::parse_at(input, now)?;
        if input.chars().next_back().is_some_and(char::is_alphanumeric) {
            query.make_last_word_prefix();
        }
        Ok(query)
    }

    /// Turns the last text of the query into a [`Prefix`](Query::Prefix),
    /// unless it is excluded (`-word`) or an operator value like `in:`.
    fn make_last_word_prefix(&mut self) {
        match self {
            Query::And(items) | Query::Or(items) => {
                if let Some(last) = items.last_mut() {
                    last.make_last_word_prefix();
                }
            }
            Query::Text { field, text } => {
                *self = Query::Prefix {
                    field: *field,
                    text: std::mem::take(text),
                };
            }
            Query::All | Query::Not(_) | Query::Prefix { .. } | Query::Filter(_) => {}
        }
    }

    /// Whether the query has plain words or a `subject:`, which rank results
    /// by relevance. Queries of only operators like `from:` read as filters,
    /// so their results are best shown newest first.
    pub fn has_free_text(&self) -> bool {
        match self {
            Query::Text { field, .. } | Query::Prefix { field, .. } => {
                matches!(field, TextField::Any | TextField::Subject)
            }
            Query::And(items) | Query::Or(items) => items.iter().any(Query::has_free_text),
            Query::All | Query::Not(_) | Query::Filter(_) => false,
        }
    }
}

fn and(mut items: Vec<Query>) -> Query {
    items.retain(|q| *q != Query::All);
    match items.len() {
        0 => Query::All,
        1 => items.pop().unwrap_or(Query::All),
        _ => Query::And(items),
    }
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    now: i64,
    depth: usize,
}

impl<'a> Parser<'a> {
    fn rest(&self) -> &str {
        &self.input[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.pos += c.len_utf8();
            true
        } else {
            false
        }
    }

    fn skip_space(&mut self) {
        let trimmed = self.rest().trim_start();
        self.pos = self.input.len() - trimmed.len();
    }

    /// Whether the next token is the `OR` keyword.
    fn at_or(&self) -> bool {
        let rest = self.rest();
        rest.starts_with("OR") && rest[2..].chars().next().is_none_or(is_break)
    }

    /// `and := or*`, up to `)` or the end. As in Gmail, `OR` binds tighter
    /// than the implicit AND: `a OR b c` means `(a OR b) c`.
    fn and(&mut self, field: Option<TextField>) -> Result<Query, ParseError> {
        let mut items = Vec::new();
        loop {
            self.skip_space();
            match self.peek() {
                None | Some(')') => break,
                // An `OR` with nothing before it.
                _ if self.at_or() => self.pos += 2,
                _ => items.extend(self.or(field)?),
            }
        }
        Ok(and(items))
    }

    /// `or := unary ("OR" unary)*`
    fn or(&mut self, field: Option<TextField>) -> Result<Option<Query>, ParseError> {
        let mut alternatives: Vec<Query> = self.unary(field)?.into_iter().collect();
        loop {
            let before = self.pos;
            self.skip_space();
            if !self.at_or() {
                self.pos = before;
                break;
            }
            self.pos += 2;
            self.skip_space();
            match self.peek() {
                // `a OR` with nothing after it is just `a`.
                None | Some(')') => break,
                _ if self.at_or() => continue,
                _ => alternatives.extend(self.unary(field)?),
            }
        }
        alternatives.retain(|q| *q != Query::All);
        Ok(match alternatives.len() {
            0 => None,
            1 => alternatives.pop(),
            _ => Some(Query::Or(alternatives)),
        })
    }

    /// `unary := "-" unary | primary`
    fn unary(&mut self, field: Option<TextField>) -> Result<Option<Query>, ParseError> {
        if self.rest().starts_with('-')
            && self.rest()[1..]
                .chars()
                .next()
                .is_some_and(|c| !c.is_whitespace())
        {
            self.pos += 1;
            self.enter()?;
            let inner = self.unary(field);
            self.depth -= 1;
            return Ok(inner?.map(|q| match q {
                Query::All => Query::All,
                Query::Not(inner) => *inner,
                q => Query::Not(Box::new(q)),
            }));
        }
        self.primary(field)
    }

    fn enter(&mut self) -> Result<(), ParseError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(ParseError::TooDeep);
        }
        Ok(())
    }

    /// `primary := "(" or ")" | phrase | operator:value | word`
    fn primary(&mut self, field: Option<TextField>) -> Result<Option<Query>, ParseError> {
        if self.eat('(') {
            self.enter()?;
            let inner = self.and(field);
            self.depth -= 1;
            let inner = inner?;
            self.skip_space();
            self.eat(')');
            return Ok(Some(inner));
        }
        if self.peek() == Some('"') {
            let phrase = self.phrase();
            return Ok(Some(text(field.unwrap_or(TextField::Any), phrase)));
        }
        let word = self.word();
        if word.is_empty() {
            // A lone `-` or other character that starts no token.
            let skip = self.peek().map_or(0, char::len_utf8);
            self.pos += skip;
            return Ok(None);
        }
        if field.is_none()
            && let Some((operator, value)) = word.split_once(':')
            && let Some(op) = Operator::parse(operator)
        {
            return self.operator(op, operator, value).map(Some);
        }
        Ok(Some(text(field.unwrap_or(TextField::Any), word.to_owned())))
    }

    /// A `"…"` phrase; the closing quote is optional at the end of input.
    fn phrase(&mut self) -> String {
        self.pos += 1;
        let rest = self.rest();
        let (phrase, advance) = match rest.find('"') {
            Some(end) => (&rest[..end], end + 1),
            None => (rest, rest.len()),
        };
        let phrase = phrase.to_owned();
        self.pos += advance;
        phrase
    }

    /// A run up to whitespace, a parenthesis or a quote. A quote right after
    /// `operator:` is part of the value (`from:"Kenneth Lay"`), handled by
    /// [`operator`](Self::operator).
    fn word(&mut self) -> &'a str {
        let input = self.input;
        let rest = &input[self.pos..];
        let end = rest.find(is_break).unwrap_or(rest.len());
        let start = self.pos;
        self.pos += end;
        &input[start..self.pos]
    }

    fn operator(&mut self, op: Operator, name: &str, value: &str) -> Result<Query, ParseError> {
        let name = name.to_ascii_lowercase();
        if let Operator::Text(field) = op {
            if value.is_empty() {
                if self.peek() == Some('"') {
                    return Ok(text(field, self.phrase()));
                }
                if self.eat('(') {
                    self.enter()?;
                    let inner = self.and(Some(field));
                    self.depth -= 1;
                    self.skip_space();
                    self.eat(')');
                    return inner;
                }
            }
            return Ok(text(field, value.to_owned()));
        }
        let value = if value.is_empty() && self.peek() == Some('"') {
            self.phrase()
        } else {
            value.to_owned()
        };
        let invalid = || ParseError::InvalidValue {
            operator: name.clone(),
            value: value.clone(),
        };
        let lower = value.to_lowercase();
        let filter = match op {
            Operator::Text(_) => unreachable!("handled above"),
            Operator::Has => match lower.as_str() {
                "attachment" | "attachments" => Filter::HasAttachment,
                _ => return Err(invalid()),
            },
            Operator::Is => match lower.as_str() {
                "unread" => Filter::Flag(MessageFlags::SEEN, false),
                "read" => Filter::Flag(MessageFlags::SEEN, true),
                "starred" | "flagged" => Filter::Flag(MessageFlags::FLAGGED, true),
                "answered" | "replied" => Filter::Flag(MessageFlags::ANSWERED, true),
                "draft" => Filter::Flag(MessageFlags::DRAFT, true),
                "forwarded" => Filter::Flag(MessageFlags::FORWARDED, true),
                _ => return Err(invalid()),
            },
            Operator::In if lower.is_empty() => return Err(invalid()),
            Operator::In => Filter::In(lower),
            Operator::Label if lower.is_empty() => return Err(invalid()),
            Operator::Label => Filter::Label(lower),
            Operator::Org if lower.is_empty() => return Err(invalid()),
            Operator::Org => Filter::Org(lower),
            Operator::Before => Filter::Before(parse_date(&value).ok_or_else(invalid)?),
            Operator::After => Filter::After(parse_date(&value).ok_or_else(invalid)?),
            Operator::OlderThan => Filter::Before(
                self.now
                    .saturating_sub(parse_age(&lower).ok_or_else(invalid)?),
            ),
            Operator::NewerThan => Filter::After(
                self.now
                    .saturating_sub(parse_age(&lower).ok_or_else(invalid)?),
            ),
            Operator::Larger => Filter::Larger(parse_size(&lower).ok_or_else(invalid)?),
            Operator::Smaller => Filter::Smaller(parse_size(&lower).ok_or_else(invalid)?),
        };
        Ok(Query::Filter(filter))
    }
}

fn is_break(c: char) -> bool {
    c.is_whitespace() || matches!(c, '(' | ')' | '"')
}

fn text(field: TextField, text: String) -> Query {
    // The index keeps only letters and digits, so `-` or `!!!` finds nothing.
    if !text.chars().any(char::is_alphanumeric) {
        Query::All
    } else {
        Query::Text { field, text }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Text(TextField),
    Has,
    Is,
    In,
    Label,
    Org,
    Before,
    After,
    OlderThan,
    NewerThan,
    Larger,
    Smaller,
}

impl Operator {
    fn parse(name: &str) -> Option<Self> {
        Some(match name.to_ascii_lowercase().as_str() {
            "from" => Self::Text(TextField::From),
            "to" => Self::Text(TextField::To),
            "cc" => Self::Text(TextField::Cc),
            "bcc" => Self::Text(TextField::Bcc),
            "subject" => Self::Text(TextField::Subject),
            "filename" => Self::Text(TextField::Filename),
            "list" => Self::Text(TextField::List),
            "has" => Self::Has,
            "is" => Self::Is,
            "in" => Self::In,
            "label" => Self::Label,
            "org" => Self::Org,
            "before" => Self::Before,
            "after" => Self::After,
            "older_than" => Self::OlderThan,
            "newer_than" => Self::NewerThan,
            "larger" => Self::Larger,
            "smaller" => Self::Smaller,
            _ => return None,
        })
    }
}

/// `2001-05-14` or `2001/05/14` → Unix time of that day's start, UTC.
/// A time and a UTC offset may follow, as the app's date picker writes
/// them for local time: `2001-05-14T09:30+06:00`, `2001-05-14T09:30Z`
/// (UTC without an offset).
fn parse_date(value: &str) -> Option<i64> {
    let (day, time) = match value.split_once(['T', 't']) {
        Some((day, time)) => (day, Some(time)),
        None => (value, None),
    };
    let day = parse_day(day)?;
    let Some(time) = time else {
        return Some(day);
    };
    let (clock, offset) = match time.find(['+', '-', 'Z', 'z']) {
        Some(at) => time.split_at(at),
        None => (time, ""),
    };
    let offset = match offset {
        "" | "Z" | "z" => 0,
        _ => {
            let sign = if offset.starts_with('-') { -1 } else { 1 };
            let (hours, minutes) = offset[1..].split_once(':')?;
            let (hours, minutes) = (two_digits(hours, 23)?, two_digits(minutes, 59)?);
            sign * (hours * 3_600 + minutes * 60)
        }
    };
    let mut parts = clock.split(':');
    let hours = two_digits(parts.next()?, 23)?;
    let minutes = two_digits(parts.next()?, 59)?;
    let seconds = parts.next().map_or(Some(0), |s| two_digits(s, 59))?;
    if parts.next().is_some() {
        return None;
    }
    Some(day + hours * 3_600 + minutes * 60 + seconds - offset)
}

/// Exactly two ASCII digits, at most `max`.
fn two_digits(value: &str, max: i64) -> Option<i64> {
    if value.len() != 2 || !value.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    value.parse().ok().filter(|v| *v <= max)
}

/// The day part of [`parse_date`].
fn parse_day(value: &str) -> Option<i64> {
    let mut parts = value.split(['-', '/']);
    let year: i64 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=9999).contains(&year) || !(1..=12).contains(&month) {
        return None;
    }
    if day == 0 || day > days_in_month(year, month) {
        return None;
    }
    Some(days_from_civil(year, month, day) * 86_400)
}

fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Days since 1970-01-01 (Howard Hinnant's algorithm).
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let yoe = year - era * 400;
    let month = i64::from(month);
    let doy = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// `30d`, `2w`, `6m`, `1y` → seconds.
fn parse_age(value: &str) -> Option<i64> {
    let split = value.find(|c: char| !c.is_ascii_digit())?;
    let (number, unit) = value.split_at(split);
    let number: i64 = number.parse().ok()?;
    let day = 86_400;
    let unit = match unit {
        "d" => day,
        "w" => 7 * day,
        "m" => 30 * day,
        "y" => 365 * day,
        _ => return None,
    };
    number.checked_mul(unit)
}

/// `5000`, `100k`, `5m`, `1g` → bytes.
fn parse_size(value: &str) -> Option<u64> {
    let split = value
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(value.len());
    let (number, unit) = value.split_at(split);
    let number: u64 = number.parse().ok()?;
    let unit = match unit {
        "" | "b" => 1,
        "k" | "kb" => 1 << 10,
        "m" | "mb" => 1 << 20,
        "g" | "gb" => 1 << 30,
        _ => return None,
    };
    number.checked_mul(unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_000_000_000;

    fn parse(input: &str) -> Query {
        Query::parse_at(input, NOW).unwrap()
    }

    fn word(field: TextField, text: &str) -> Query {
        Query::Text {
            field,
            text: text.to_owned(),
        }
    }

    fn any(text: &str) -> Query {
        word(TextField::Any, text)
    }

    #[test]
    fn words_and_operators() {
        assert_eq!(parse(""), Query::All);
        assert_eq!(parse("  "), Query::All);
        assert_eq!(parse("budget"), any("budget"));
        assert_eq!(
            parse("from:kenneth.lay has:attachment budget"),
            Query::And(vec![
                word(TextField::From, "kenneth.lay"),
                Query::Filter(Filter::HasAttachment),
                any("budget"),
            ])
        );
        assert_eq!(parse("FROM:ada"), word(TextField::From, "ada"));
        assert_eq!(
            parse("subject:\"Q3 plan\""),
            word(TextField::Subject, "Q3 plan")
        );
        assert_eq!(parse("\"natural gas"), any("natural gas"));
        assert_eq!(parse("http://x.org"), any("http://x.org"));
        assert_eq!(parse("e-mail"), any("e-mail"));
    }

    #[test]
    fn boolean_structure() {
        assert_eq!(
            parse("gas OR power -california"),
            Query::And(vec![
                Query::Or(vec![any("gas"), any("power")]),
                Query::Not(Box::new(any("california"))),
            ])
        );
        assert_eq!(
            parse("(gas OR power) price"),
            Query::And(vec![
                Query::Or(vec![any("gas"), any("power")]),
                any("price")
            ])
        );
        assert_eq!(
            parse("from:(alice OR bob) -subject:re"),
            Query::And(vec![
                Query::Or(vec![
                    word(TextField::From, "alice"),
                    word(TextField::From, "bob")
                ]),
                Query::Not(Box::new(word(TextField::Subject, "re"))),
            ])
        );
        assert_eq!(parse("--x"), any("x"));
        assert_eq!(
            parse("or ORacle"),
            Query::And(vec![any("or"), any("ORacle")])
        );
        assert_eq!(parse("a OR"), any("a"));
        assert_eq!(parse("OR a OR OR b"), Query::Or(vec![any("a"), any("b")]));
    }

    #[test]
    fn lenient_with_broken_input() {
        assert_eq!(parse("a) b"), Query::And(vec![any("a"), any("b")]));
        assert_eq!(parse("(a b"), Query::And(vec![any("a"), any("b")]));
        assert_eq!(parse("- a"), any("a"));
        assert_eq!(parse("-!!! a"), any("a"));
        assert_eq!(parse("()"), Query::All);
        assert_eq!(parse("foo:bar"), any("foo:bar"));
        assert_eq!(parse("from:"), Query::All);
        let deep = "(".repeat(MAX_DEPTH + 1);
        assert_eq!(Query::parse_at(&deep, NOW), Err(ParseError::TooDeep));
        let fine = "(".repeat(MAX_DEPTH);
        assert_eq!(parse(&fine), Query::All);
        assert_eq!(
            Query::parse_at(&"-".repeat(10_000), NOW),
            Err(ParseError::TooDeep)
        );
    }

    #[test]
    fn filters() {
        let f = |input: &str| match parse(input) {
            Query::Filter(filter) => filter,
            other => panic!("{input}: {other:?}"),
        };
        assert_eq!(f("is:unread"), Filter::Flag(MessageFlags::SEEN, false));
        assert_eq!(f("is:Starred"), Filter::Flag(MessageFlags::FLAGGED, true));
        assert_eq!(f("in:Inbox"), Filter::In("inbox".into()));
        assert_eq!(f("label:Work"), Filter::Label("work".into()));
        assert_eq!(f("org:enron.com"), Filter::Org("enron.com".into()));
        assert_eq!(f("before:2001-05-14"), Filter::Before(989_798_400));
        assert_eq!(f("after:2001/5/14"), Filter::After(989_798_400));
        assert_eq!(f("after:1970-01-01"), Filter::After(0));
        assert_eq!(f("before:2000-02-29"), Filter::Before(951_782_400));
        // With a time, and a UTC offset as the app's date picker writes.
        assert_eq!(
            f("after:2001-05-14T09:30Z"),
            Filter::After(989_798_400 + 34_200)
        );
        assert_eq!(
            f("after:2001-05-14T09:30"),
            Filter::After(989_798_400 + 34_200)
        );
        assert_eq!(
            f("before:2001-05-14T00:00+06:00"),
            Filter::Before(989_798_400 - 21_600)
        );
        assert_eq!(
            f("after:2001-05-14T23:15:30-04:30"),
            Filter::After(989_798_400 + 83_730 + 16_200)
        );
        for bad in [
            "after:2001-05-14T",
            "after:2001-05-14T9:30",
            "after:2001-05-14T24:00",
            "after:2001-05-14T09:30+6",
            "after:2001-05-14T09:30+06:00:00",
            "after:2001-05-14T09:30:00:00",
        ] {
            assert!(Query::parse_at(bad, NOW).is_err(), "{bad}");
        }
        assert_eq!(f("newer_than:2d"), Filter::After(NOW - 2 * 86_400));
        assert_eq!(f("older_than:1y"), Filter::Before(NOW - 365 * 86_400));
        assert_eq!(f("larger:5M"), Filter::Larger(5 << 20));
        assert_eq!(f("smaller:100kb"), Filter::Smaller(100 << 10));
        assert_eq!(f("larger:12"), Filter::Larger(12));
        assert_eq!(f("in:\"Sent Items\""), Filter::In("sent items".into()));
    }

    #[test]
    fn invalid_values() {
        for input in [
            "before:soon",
            "before:2001-02-29",
            "after:2001-13-01",
            "larger:big",
            "larger:99999999999999999999",
            "newer_than:3x",
            "is:important",
            "has:drive",
            "in:",
        ] {
            let err = Query::parse_at(input, NOW).unwrap_err();
            assert!(matches!(err, ParseError::InvalidValue { .. }), "{input}");
            assert!(!err.to_string().is_empty());
        }
    }

    #[test]
    fn has_free_text() {
        assert!(parse("budget has:attachment").has_free_text());
        assert!(parse("subject:budget").has_free_text());
        assert!(!parse("from:ada").has_free_text());
        assert!(!parse("has:attachment -budget").has_free_text());
        assert!(!parse("").has_free_text());
    }

    #[test]
    fn as_you_type() {
        let typing = |input: &str| Query::parse_as_you_type(input, NOW).unwrap();
        let prefix = |field, text: &str| Query::Prefix {
            field,
            text: text.to_owned(),
        };
        assert_eq!(typing("budg"), prefix(TextField::Any, "budg"));
        assert_eq!(typing("budget "), any("budget"));
        assert_eq!(
            typing("from:kenneth.lay bud"),
            Query::And(vec![
                word(TextField::From, "kenneth.lay"),
                prefix(TextField::Any, "bud")
            ])
        );
        assert_eq!(typing("from:ken"), prefix(TextField::From, "ken"));
        assert_eq!(
            typing("gas OR pow"),
            Query::Or(vec![any("gas"), prefix(TextField::Any, "pow")])
        );
        assert_eq!(typing("\"natural gas\""), any("natural gas"));
        assert_eq!(typing("-budg"), Query::Not(Box::new(any("budg"))));
        assert_eq!(
            typing("in:inbox"),
            Query::Filter(Filter::In("inbox".into()))
        );
        assert!(typing("budg").has_free_text());
        assert_eq!(typing(""), Query::All);
    }

    /// Random input never panics and never nests past the limit (fuzzing
    /// without a fuzzer; a `cargo fuzz` target can reuse this generator).
    #[test]
    fn random_input_never_panics() {
        const PIECES: &[&str] = &[
            "(",
            ")",
            "\"",
            "-",
            " ",
            "OR",
            "from:",
            "to:",
            "is:",
            "has:",
            "before:",
            "larger:",
            "in:",
            "a",
            "é",
            "💥",
            ":",
            "2001-01-01",
            "5M",
            "unread",
            "\t",
        ];
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..20_000 {
            let len = next() % 24;
            let input: String = (0..len)
                .map(|_| PIECES[(next() % PIECES.len() as u64) as usize])
                .collect();
            let _ = Query::parse_at(&input, NOW);
        }
    }
}
