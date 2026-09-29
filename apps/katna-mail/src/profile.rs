// SPDX-License-Identifier: GPL-3.0-or-later

//! What the contact panel shows about one person, read from the local
//! mail only: the mail exchanged, recent conversations and files, open
//! tasks made from mail with them, their phone, title and company from
//! their signatures, and their UTC offset from the `Date` headers of their
//! mail. Nothing is looked up online.

use katna_core::Paths;
use katna_store::{ContactConversation, ContactFile, ContactSummary, MessageId, Mode, Store};
use mail_parser::MessageParser;

/// Recent conversations the panel lists: the first few, and the rest
/// after More.
pub const CONVERSATIONS: usize = 10;
/// Files the panel lists.
pub const FILES: usize = 6;
/// Their newest messages read for a signature.
const SIGNED: usize = 8;
/// A signature is at most this many lines; longer text is a message or a
/// disclaimer.
const SIGNATURE_LINES: usize = 12;

/// One person, as the mail shows them.
#[derive(Debug, Clone, Default)]
pub struct Profile {
    pub summary: ContactSummary,
    pub conversations: Vec<ContactConversation>,
    pub files: Vec<ContactFile>,
    pub card: Card,
    /// Their offset from UTC in minutes, from the `Date` header of their
    /// newest stored message.
    pub offset: Option<i32>,
    /// The mails of open tasks (`Message-ID`s) they take part in.
    pub task_mails: Vec<String>,
}

/// What their signature says.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Card {
    pub phone: Option<String>,
    pub title: Option<String>,
    pub company: Option<String>,
}

impl Card {
    fn is_full(&self) -> bool {
        self.phone.is_some() && self.title.is_some() && self.company.is_some()
    }

    /// Fills what is missing from `other`.
    fn fill(&mut self, other: Card) {
        self.phone = self.phone.take().or(other.phone);
        self.title = self.title.take().or(other.title);
        self.company = self.company.take().or(other.company);
    }
}

/// Reads the profile of `email`, with which of `task_mails` (the mails of
/// open tasks) they take part in. Opens its own connection, for a
/// background thread.
pub fn read(paths: &Paths, email: &str, task_mails: &[String]) -> Result<Profile, String> {
    let email = email.trim().to_lowercase();
    let store = Store::open(paths, Mode::ReadOnly).map_err(|e| e.to_string())?;
    let summary = store.contact_summary(&email).map_err(|e| e.to_string())?;
    let conversations = store
        .contact_conversations(&email, CONVERSATIONS)
        .map_err(|e| e.to_string())?;
    let files = store
        .contact_files(&email, FILES)
        .map_err(|e| e.to_string())?;
    let task_mails = store
        .contact_on_mail(&email, task_mails)
        .map_err(|e| e.to_string())?;
    let mut card = Card::default();
    let mut offset = None;
    let signed = store
        .messages_from(&email, SIGNED)
        .map_err(|e| e.to_string())?;
    for raw in signed.into_iter().filter_map(|id| raw(&store, id)) {
        if offset.is_none() {
            offset = date_offset(&raw);
        }
        let view = katna_render::message_view(&raw);
        card.fill(signature_card(&view.body, summary.name.as_deref()));
        if card.is_full() && offset.is_some() {
            break;
        }
    }
    Ok(Profile {
        summary,
        conversations,
        files,
        card,
        offset,
        task_mails,
    })
}

fn raw(store: &Store, id: MessageId) -> Option<Vec<u8>> {
    let message = store.messages_by_id(&[id]).ok()?.pop()?;
    store.blobs().get(&message.blob_hash?).ok().flatten()
}

/// The UTC offset in minutes the `Date` header of `raw` was written in.
fn date_offset(raw: &[u8]) -> Option<i32> {
    let message = MessageParser::default().parse_headers(raw)?;
    let date = message.date()?;
    let minutes = i32::from(date.tz_hour) * 60 + i32::from(date.tz_minute);
    // Outside any real time zone: a broken header.
    if minutes > 14 * 60 {
        return None;
    }
    Some(if date.tz_before_gmt {
        -minutes
    } else {
        minutes
    })
}

/// The text a message's author wrote: its lines before the quoted
/// message it answers or forwards.
fn own_lines(body: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    for line in body.lines() {
        let t = line.trim();
        let lower = t.to_lowercase();
        if t.starts_with('>')
            || (lower.starts_with("on ") && lower.ends_with("wrote:"))
            || lower.contains("original message")
            || lower.contains("forwarded message")
            || (t.len() >= 20 && t.chars().all(|c| c == '_'))
            || (lower.starts_with("from:") && !lines.is_empty())
        {
            break;
        }
        lines.push(t);
    }
    lines
}

/// Whether `line` closes a message, like "Best regards,".
fn is_sign_off(line: &str) -> bool {
    const SIGN_OFFS: &[&str] = &[
        "regards",
        "best regards",
        "kind regards",
        "warm regards",
        "warmest regards",
        "with regards",
        "many thanks",
        "thanks",
        "thank you",
        "thanks and regards",
        "thanks & regards",
        "thanks & best regards",
        "cheers",
        "best",
        "all the best",
        "best wishes",
        "sincerely",
        "yours sincerely",
        "yours truly",
        "yours",
        "br",
        "cordially",
        "respectfully",
    ];
    let line = line
        .trim()
        .trim_end_matches([',', '.', '!', ' '])
        .to_lowercase();
    SIGN_OFFS.contains(&line.as_str())
}

/// The lines of the signature in `lines`: after a "-- " line, else after
/// the last sign-off. Empty when there is neither.
fn signature(lines: &[&str]) -> Vec<String> {
    let start = lines
        .iter()
        .rposition(|l| *l == "--" || *l == "-- ")
        .or_else(|| lines.iter().rposition(|l| is_sign_off(l)));
    let Some(start) = start else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in &lines[start + 1..] {
        if line.chars().count() > 100 {
            break;
        }
        if !line.is_empty() {
            out.push((*line).to_owned());
        }
        if out.len() == SIGNATURE_LINES {
            break;
        }
    }
    out
}

/// The phone number in `text`, if it has one: 7 to 15 digits with
/// spaces, dashes, dots and brackets, not a date.
fn phone(text: &str) -> Option<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        let starts = c.is_ascii_digit() || ((c == '+' || c == '(') && at + 1 < chars.len());
        if !starts || (at > 0 && chars[at - 1].is_alphanumeric()) {
            at += 1;
            continue;
        }
        let mut end = at;
        while end < chars.len()
            && (chars[end].is_ascii_digit() || " +-.()/\u{a0}".contains(chars[end]))
        {
            end += 1;
        }
        let run: String = chars[at..end].iter().collect();
        let run = run.trim_end_matches([' ', '-', '.', '/', '(', '\u{a0}']);
        let digits = run.chars().filter(char::is_ascii_digit).count();
        let letter_after = chars.get(end).is_some_and(|c| c.is_alphabetic());
        let date_like = run.len() == 10
            && run
                .chars()
                .filter(|c| *c == '-' || *c == '/' || *c == '.')
                .count()
                == 2;
        if (7..=15).contains(&digits) && !letter_after && !date_like {
            let has_digit_first = run.chars().find(|c| !c.is_whitespace() && *c != '(');
            if has_digit_first.is_some_and(|c| c.is_ascii_digit() || c == '+') {
                return Some(run.trim().to_owned());
            }
        }
        at = end.max(at + 1);
    }
    None
}

/// Words that make a signature line a job title.
const ROLE_WORDS: &[&str] = &[
    "manager",
    "director",
    "engineer",
    "ceo",
    "cto",
    "cfo",
    "coo",
    "cmo",
    "founder",
    "co-founder",
    "cofounder",
    "president",
    "head",
    "lead",
    "officer",
    "consultant",
    "developer",
    "designer",
    "analyst",
    "specialist",
    "coordinator",
    "partner",
    "associate",
    "vp",
    "chief",
    "professor",
    "architect",
    "administrator",
    "assistant",
    "executive",
    "owner",
    "editor",
    "recruiter",
    "advisor",
    "adviser",
    "intern",
    "scientist",
    "researcher",
    "accountant",
    "attorney",
    "lawyer",
    "counsel",
    "representative",
    "supervisor",
    "principal",
    "producer",
    "officer",
    "strategist",
    "technician",
    "teacher",
    "lecturer",
    "secretary",
    "chairman",
    "treasurer",
    "agent",
    "programmer",
    "sde",
    "sre",
    "devops",
];

/// Endings that make a signature line a company name.
const COMPANY_WORDS: &[&str] = &[
    "inc",
    "inc.",
    "llc",
    "ltd",
    "ltd.",
    "limited",
    "gmbh",
    "corp",
    "corp.",
    "corporation",
    "co.",
    "company",
    "pvt",
    "pvt.",
    "plc",
    "ag",
    "s.a.",
    "sa",
    "bv",
    "b.v.",
    "llp",
    "group",
    "technologies",
    "systems",
    "solutions",
    "labs",
    "studio",
    "studios",
    "university",
    "institute",
    "foundation",
    "services",
    "software",
    "consulting",
    "partners",
    "holdings",
    "enterprises",
    "industries",
    "agency",
    "bank",
];

fn words(line: &str) -> impl Iterator<Item = String> + '_ {
    line.split(|c: char| c.is_whitespace() || c == ',' || c == '|' || c == '/')
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
}

fn is_role(line: &str) -> bool {
    words(line).any(|w| ROLE_WORDS.contains(&w.trim_matches(['(', ')', '.', ':'])))
        || words(line).any(|w| {
            w.trim_matches(['(', ')', ':'])
                .split('-')
                .any(|part| ROLE_WORDS.contains(&part))
        })
}

fn is_company(line: &str) -> bool {
    words(line).any(|w| COMPANY_WORDS.contains(&w.as_str()))
}

/// A line that can hold a title or a company: short, no digits, no
/// addresses or links.
fn is_wordy(line: &str) -> bool {
    let lower = line.to_lowercase();
    let chars = line.chars().count();
    (2..=60).contains(&chars)
        && !line.chars().any(|c| c.is_ascii_digit())
        && !lower.contains('@')
        && !lower.contains("http")
        && !lower.contains("www.")
        && !lower.contains(':')
}

/// Whether `line` is the person's name: their name as the mail shows it,
/// or its first word, or (with no name known) a short line of 2 to 4
/// capitalised words.
fn is_name(line: &str, name: Option<&str>) -> bool {
    let line = line.trim().trim_end_matches(',');
    if let Some(name) = name.map(str::trim).filter(|n| !n.is_empty()) {
        let first = name.split_whitespace().next().unwrap_or(name);
        return line.eq_ignore_ascii_case(name)
            || line.eq_ignore_ascii_case(first)
            || (line.split_whitespace().count() <= 4
                && line
                    .split_whitespace()
                    .next()
                    .is_some_and(|w| w.eq_ignore_ascii_case(first)));
    }
    let count = line.split_whitespace().count();
    (1..=4).contains(&count)
        && is_wordy(line)
        && line
            .split_whitespace()
            .all(|w| w.chars().next().is_some_and(char::is_uppercase))
}

/// Splits "Title at Company", "Title, Company" or "Title | Company".
fn split_title(line: &str) -> (String, Option<String>) {
    for sep in [
        " at ",
        " @ ",
        " | ",
        " \u{2013} ",
        " \u{2014} ",
        " - ",
        ", ",
    ] {
        if let Some((title, company)) = line.split_once(sep) {
            let (title, company) = (title.trim(), company.trim());
            if !title.is_empty() && !company.is_empty() && is_role(title) {
                return (title.to_owned(), Some(company.to_owned()));
            }
        }
    }
    (line.trim().to_owned(), None)
}

/// The phone, title and company in the signature of `body`, a message
/// from someone called `name`.
pub fn signature_card(body: &str, name: Option<&str>) -> Card {
    let lines = own_lines(body);
    let signature = signature(&lines);
    if signature.is_empty() {
        return Card::default();
    }
    let mut card = Card {
        phone: signature
            .iter()
            .find_map(|l| l.split(['|', '\u{2022}', '\u{b7}']).find_map(phone)),
        ..Card::default()
    };
    // The lines under their name; without a name line, the first lines.
    let after = signature
        .iter()
        .position(|l| is_name(l, name))
        .map_or(0, |ix| ix + 1);
    let wordy: Vec<&String> = signature[after..]
        .iter()
        .take(3)
        .take_while(|l| is_wordy(l))
        .collect();
    for line in wordy {
        if card.title.is_none() && is_role(line) {
            let (title, company) = split_title(line);
            card.title = Some(title);
            if company.is_some() {
                card.company = company;
            }
        } else if card.company.is_none() && (is_company(line) || card.title.is_some()) {
            card.company = Some(line.trim().to_owned());
        }
    }
    card
}

/// A UTC offset as people write it: "UTC+5:30", "UTC−8", "UTC".
pub fn offset_label(minutes: i32) -> String {
    if minutes == 0 {
        return "UTC".to_owned();
    }
    let sign = if minutes < 0 { '\u{2212}' } else { '+' };
    let (h, m) = (minutes.abs() / 60, minutes.abs() % 60);
    if m == 0 {
        format!("UTC{sign}{h}")
    } else {
        format!("UTC{sign}{h}:{m:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dash_dash_signature() {
        let body = "Hi,\n\nSee the file.\n\n-- \nAda Lovelace\nSenior Software Engineer\nAnalytical Engines Ltd\nM: +44 20 7946 0958\n";
        assert_eq!(
            signature_card(body, Some("Ada Lovelace")),
            Card {
                phone: Some("+44 20 7946 0958".to_owned()),
                title: Some("Senior Software Engineer".to_owned()),
                company: Some("Analytical Engines Ltd".to_owned()),
            }
        );
    }

    #[test]
    fn sign_off_and_title_at_company() {
        let body = "Thanks for the call.\n\nBest regards,\nRavi\nProduct Manager at Invenia Systems\nPhone: (080) 4567-8901 | ravi@invenia.in\n\nOn Mon, Sep 1, 2026 at 10:00 AM Me <me@x.org> wrote:\n> Regards,\n> Me\n> Director\n";
        assert_eq!(
            signature_card(body, Some("Ravi Kumar")),
            Card {
                phone: Some("(080) 4567-8901".to_owned()),
                title: Some("Product Manager".to_owned()),
                company: Some("Invenia Systems".to_owned()),
            }
        );
    }

    #[test]
    fn no_signature_no_guess() {
        let body = "Meeting on 2026-09-27 at 10.\nCall 12 people.\n";
        assert_eq!(signature_card(body, Some("Bob")), Card::default());
        // A date is not a phone number.
        assert_eq!(phone("Sent 2026-09-27"), None);
        assert_eq!(
            phone("Tel. +1 (555) 010-9999."),
            Some("+1 (555) 010-9999".to_owned())
        );
        assert_eq!(phone("Order 12345"), None);
    }

    #[test]
    fn quoted_signatures_are_not_theirs() {
        let body = "ok\n\n> Regards,\n> Carol\n> CEO, Acme Inc\n";
        assert_eq!(signature_card(body, Some("Dan")), Card::default());
    }

    #[test]
    fn offsets() {
        assert_eq!(offset_label(330), "UTC+5:30");
        assert_eq!(offset_label(-480), "UTC\u{2212}8");
        assert_eq!(offset_label(0), "UTC");
        let raw = b"Date: Sat, 27 Sep 2026 22:22:55 +0530\r\nFrom: a@b.c\r\n\r\nhi";
        assert_eq!(date_offset(raw), Some(330));
        let raw = b"Date: Sat, 27 Sep 2026 09:00:00 -0700\r\n\r\nhi";
        assert_eq!(date_offset(raw), Some(-420));
    }
}
