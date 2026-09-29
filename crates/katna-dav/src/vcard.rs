// SPDX-License-Identifier: GPL-3.0-or-later

//! vCard 3.0 and 4.0 (RFC 2426, RFC 6350) read into a
//! [`katna_core::contact::Card`], and written back as vCard 3.0, which
//! every CardDAV server takes (iCloud takes no other).
//!
//! Properties Katna does not show are kept: [`write`] edits the card's own
//! text when it has one, so a server's extra fields survive a change.

use katna_core::contact::{Card, Name, PostalAddress, Typed};

/// One property line, unfolded.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    /// `item1.EMAIL` keeps its group apart: `EMAIL`.
    name: String,
    group: Option<String>,
    params: Vec<(String, String)>,
    value: String,
}

impl Line {
    fn param(&self, key: &str) -> impl Iterator<Item = &str> {
        self.params
            .iter()
            .filter(move |(k, _)| k.eq_ignore_ascii_case(key))
            .flat_map(|(_, v)| v.split(','))
    }

    /// The first TYPE that names a kind Katna shows (`home`, `work`, `cell`
    /// read as `mobile`, …), else empty; `pref` and `internet` are not kinds.
    fn kind(&self) -> String {
        let mut types: Vec<String> = self.param("TYPE").map(|t| t.to_lowercase()).collect();
        // vCard 2.1 style: `TEL;WORK;VOICE:…`.
        types.extend(
            self.params
                .iter()
                .filter(|(_, v)| v.is_empty())
                .map(|(k, _)| k.to_lowercase()),
        );
        for t in &types {
            match t.as_str() {
                "cell" | "mobile" | "iphone" => return "mobile".into(),
                "home" | "work" | "other" | "fax" | "main" | "pager" => return t.clone(),
                _ => {}
            }
        }
        String::new()
    }
}

/// Splits vCard text into its cards' lines.
fn cards(text: &str) -> Vec<Vec<Line>> {
    let mut out = Vec::new();
    let mut current: Option<Vec<Line>> = None;
    for text in unfold(text) {
        let Some(line) = parse_line(&text) else {
            continue;
        };
        match (line.name.as_str(), line.value.to_ascii_uppercase().as_str()) {
            ("BEGIN", "VCARD") => current = Some(Vec::new()),
            ("END", "VCARD") => out.extend(current.take()),
            _ => {
                if let Some(lines) = &mut current {
                    lines.push(line);
                }
            }
        }
    }
    out
}

/// The lines of `text`, unfolded: a line starting with a space or tab
/// continues the one before.
fn unfold(text: &str) -> Vec<String> {
    let mut unfolded: Vec<String> = Vec::new();
    for raw in text.split('\n') {
        let raw = raw.strip_suffix('\r').unwrap_or(raw);
        if let Some(rest) = raw.strip_prefix([' ', '\t'])
            && let Some(last) = unfolded.last_mut()
        {
            last.push_str(rest);
            continue;
        }
        unfolded.push(raw.to_owned());
    }
    unfolded
}

fn parse_line(text: &str) -> Option<Line> {
    // The name and parameters end at the first colon outside quotes.
    let mut quoted = false;
    let colon = text.char_indices().find_map(|(i, c)| match c {
        '"' => {
            quoted = !quoted;
            None
        }
        ':' if !quoted => Some(i),
        _ => None,
    })?;
    let (head, value) = (&text[..colon], &text[colon + 1..]);
    let mut parts = split_unquoted(head, ';').into_iter();
    let full = parts.next()?.trim().to_owned();
    if full.is_empty() {
        return None;
    }
    let (group, name) = match full.rsplit_once('.') {
        Some((g, n)) => (Some(g.to_owned()), n.to_ascii_uppercase()),
        None => (None, full.to_ascii_uppercase()),
    };
    let params = parts
        .map(|p| match p.split_once('=') {
            Some((k, v)) => (k.trim().to_owned(), v.trim().trim_matches('"').to_owned()),
            None => (p.trim().to_owned(), String::new()),
        })
        .collect();
    Some(Line {
        name,
        group,
        params,
        value: value.to_owned(),
    })
}

fn split_unquoted(text: &str, sep: char) -> Vec<&str> {
    let mut out = Vec::new();
    let mut quoted = false;
    let mut start = 0;
    for (i, c) in text.char_indices() {
        if c == '"' {
            quoted = !quoted;
        } else if c == sep && !quoted {
            out.push(&text[start..i]);
            start = i + c.len_utf8();
        }
    }
    out.push(&text[start..]);
    out
}

/// Splits a structured value at unescaped `sep`, unescaping each part.
fn components(value: &str, sep: char) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n' | 'N') => out.last_mut().unwrap().push('\n'),
                Some(other) => out.last_mut().unwrap().push(other),
                None => {}
            }
        } else if c == sep {
            out.push(String::new());
        } else {
            out.last_mut().unwrap().push(c);
        }
    }
    out
}

fn text_value(value: &str) -> String {
    components(value, '\u{0}').remove(0).trim().to_owned()
}

fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            ',' => out.push_str("\\,"),
            ';' => out.push_str("\\;"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            c => out.push(c),
        }
    }
    out
}

/// A card read from vCard text, with its UID and whether it is a group.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Parsed {
    pub uid: String,
    pub card: Card,
    /// The categories (labels) it lists.
    pub categories: Vec<String>,
    /// A group card (`KIND:group`, Apple's `X-ADDRESSBOOKSERVER-KIND`):
    /// its members' UIDs are in `members`.
    pub group: bool,
    pub members: Vec<String>,
    /// An inline picture, decoded.
    pub photo: Option<Vec<u8>>,
}

/// Reads every card in `text`.
pub fn parse(text: &str) -> Vec<Parsed> {
    cards(text).into_iter().map(|lines| read(&lines)).collect()
}

fn read(lines: &[Line]) -> Parsed {
    let mut out = Parsed::default();
    let card = &mut out.card;
    for line in lines {
        match line.name.as_str() {
            "UID" => out.uid = text_value(&line.value),
            "FN" => card.name.full = text_value(&line.value),
            "N" => {
                let parts = components(&line.value, ';');
                let part = |i: usize| {
                    parts
                        .get(i)
                        .map(|p| p.replace(',', " ").trim().to_owned())
                        .unwrap_or_default()
                };
                card.name = Name {
                    full: std::mem::take(&mut card.name.full),
                    family: part(0),
                    given: part(1),
                    middle: part(2),
                    prefix: part(3),
                    suffix: part(4),
                };
            }
            "NICKNAME" => card.nickname = text_value(&line.value),
            "EMAIL" => push(&mut card.emails, line),
            "TEL" => {
                let mut value = text_value(&line.value);
                if let Some(rest) = value.strip_prefix("tel:") {
                    value = rest.to_owned();
                }
                if !value.is_empty() {
                    card.phones.push(Typed::new(value, line.kind()));
                }
            }
            "URL" => push(&mut card.urls, line),
            "ADR" => {
                let parts = components(&line.value, ';');
                let part = |i: usize| {
                    parts
                        .get(i)
                        .map(|p| p.trim().to_owned())
                        .unwrap_or_default()
                };
                // PO box and extended address go before the street.
                let street = [part(0), part(1), part(2)]
                    .into_iter()
                    .filter(|p| !p.is_empty())
                    .collect::<Vec<_>>()
                    .join("\n");
                let address = PostalAddress {
                    kind: line.kind(),
                    street,
                    city: part(3),
                    region: part(4),
                    postcode: part(5),
                    country: part(6),
                };
                if !address.is_empty() {
                    card.addresses.push(address);
                }
            }
            "ORG" => {
                let parts = components(&line.value, ';');
                card.organization = parts
                    .first()
                    .map(|p| p.trim().to_owned())
                    .unwrap_or_default();
                card.department = parts
                    .get(1)
                    .map(|p| p.trim().to_owned())
                    .unwrap_or_default();
            }
            "TITLE" => card.title = text_value(&line.value),
            "BDAY" => card.birthday = birthday(&text_value(&line.value)),
            "NOTE" => card.note = text_value(&line.value),
            "CATEGORIES" => out.categories.extend(
                components(&line.value, ',')
                    .into_iter()
                    .map(|c| c.trim().to_owned())
                    .filter(|c| !c.is_empty()),
            ),
            "KIND" | "X-ADDRESSBOOKSERVER-KIND" => {
                out.group = line.value.trim().eq_ignore_ascii_case("group");
            }
            "MEMBER" | "X-ADDRESSBOOKSERVER-MEMBER" => {
                let value = line.value.trim();
                let uid = value.strip_prefix("urn:uuid:").unwrap_or(value);
                out.members.push(uid.to_owned());
            }
            "PHOTO" => out.photo = photo(line),
            _ => {}
        }
    }
    out
}

fn push(list: &mut Vec<Typed>, line: &Line) {
    let mut value = text_value(&line.value);
    if let Some(rest) = value.strip_prefix("mailto:") {
        value = rest.to_owned();
    }
    if !value.is_empty() {
        list.push(Typed::new(value, line.kind()));
    }
}

/// `19850314`, `1985-03-14`, `--0314`, `--03-14` or with a time: to
/// `YYYY-MM-DD` or `--MM-DD`; anything else is kept as it is.
fn birthday(value: &str) -> String {
    let date = value.split('T').next().unwrap_or(value);
    let digits: String = date.chars().filter(char::is_ascii_digit).collect();
    if date.starts_with("--") && digits.len() == 4 {
        return format!("--{}-{}", &digits[..2], &digits[2..]);
    }
    if digits.len() == 8 && !date.starts_with("--") {
        return format!("{}-{}-{}", &digits[..4], &digits[4..6], &digits[6..]);
    }
    value.to_owned()
}

fn photo(line: &Line) -> Option<Vec<u8>> {
    use base64::Engine as _;
    let value = line.value.trim();
    let data = if let Some(rest) = value.strip_prefix("data:") {
        rest.split_once(";base64,")?.1
    } else if line
        .param("ENCODING")
        .any(|e| e.eq_ignore_ascii_case("b") || e.eq_ignore_ascii_case("base64"))
    {
        value
    } else {
        return None;
    };
    let clean: String = data.chars().filter(|c| !c.is_whitespace()).collect();
    base64::engine::general_purpose::STANDARD.decode(clean).ok()
}

/// The properties [`write`] owns: everything else in the old text is kept.
const OWNED: [&str; 14] = [
    "VERSION",
    "UID",
    "FN",
    "N",
    "NICKNAME",
    "EMAIL",
    "TEL",
    "URL",
    "ADR",
    "ORG",
    "TITLE",
    "BDAY",
    "NOTE",
    "CATEGORIES",
];

/// `card` as vCard 3.0 with `uid` and `categories`. When `old` is the
/// card's text as the server has it, the properties Katna does not show
/// (pictures, instant messaging, a server's own fields) are kept.
pub fn write(uid: &str, card: &Card, categories: &[String], old: Option<&str>) -> String {
    let mut out: Vec<String> = vec![
        "BEGIN:VCARD".into(),
        "VERSION:3.0".into(),
        format!("UID:{}", escape(uid)),
    ];
    let name = &card.name;
    out.push(format!(
        "N:{};{};{};{};{}",
        escape(&name.family),
        escape(&name.given),
        escape(&name.middle),
        escape(&name.prefix),
        escape(&name.suffix)
    ));
    out.push(format!("FN:{}", escape(&card.display_name())));
    if !card.nickname.is_empty() {
        out.push(format!("NICKNAME:{}", escape(&card.nickname)));
    }
    for email in &card.emails {
        out.push(format!(
            "EMAIL;TYPE=INTERNET{}:{}",
            type_param(&email.kind),
            escape(&email.value)
        ));
    }
    for phone in &card.phones {
        let kind = if phone.kind == "mobile" {
            "cell"
        } else {
            phone.kind.as_str()
        };
        out.push(format!("TEL{}:{}", type_param(kind), escape(&phone.value)));
    }
    for address in &card.addresses {
        out.push(format!(
            "ADR{}:;;{};{};{};{};{}",
            type_param(&address.kind),
            escape(&address.street),
            escape(&address.city),
            escape(&address.region),
            escape(&address.postcode),
            escape(&address.country)
        ));
    }
    if !card.organization.is_empty() || !card.department.is_empty() {
        let org = if card.department.is_empty() {
            escape(&card.organization)
        } else {
            format!(
                "{};{}",
                escape(&card.organization),
                escape(&card.department)
            )
        };
        out.push(format!("ORG:{org}"));
    }
    if !card.title.is_empty() {
        out.push(format!("TITLE:{}", escape(&card.title)));
    }
    if !card.birthday.is_empty() {
        out.push(format!("BDAY:{}", card.birthday));
    }
    for url in &card.urls {
        out.push(format!(
            "URL{}:{}",
            type_param(&url.kind),
            url.value.replace(['\r', '\n'], "")
        ));
    }
    if !card.note.is_empty() {
        out.push(format!("NOTE:{}", escape(&card.note)));
    }
    if !categories.is_empty() {
        out.push(format!(
            "CATEGORIES:{}",
            categories
                .iter()
                .map(|c| escape(c))
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    if let Some(old) = old {
        let kept = old_lines(old);
        out.extend(kept);
    }
    out.push("END:VCARD".into());
    let mut text = String::new();
    for line in out {
        fold(&line, &mut text);
    }
    text
}

fn type_param(kind: &str) -> String {
    let kind = kind.trim();
    if kind.is_empty() || !kind.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        String::new()
    } else {
        format!(";TYPE={}", kind.to_ascii_uppercase())
    }
}

/// The lines of the first card in `old` that [`write`] does not own,
/// unfolded, as they were.
fn old_lines(old: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for text in unfold(old) {
        let Some(line) = parse_line(&text) else {
            continue;
        };
        let value = line.value.trim().to_ascii_uppercase();
        match line.name.as_str() {
            "BEGIN" if value == "VCARD" => {
                if inside {
                    break;
                }
                inside = true;
            }
            "END" if value == "VCARD" => break,
            name if inside && !OWNED.contains(&name) => {
                // Apple labels (`item1.X-ABLabel`) belong to a line we
                // rewrote; they would dangle.
                if line.group.is_some() && name.starts_with("X-AB") {
                    continue;
                }
                out.push(text);
            }
            _ => {}
        }
    }
    out
}

/// Folds `line` at 75 octets (RFC 6350 §3.2), never inside a character.
fn fold(line: &str, out: &mut String) {
    let mut width = 0;
    for c in line.chars() {
        let len = c.len_utf8();
        if width + len > 75 {
            out.push_str("\r\n ");
            width = 1;
        }
        out.push(c);
        width += len;
    }
    out.push_str("\r\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const APPLE: &str = "BEGIN:VCARD\r\nVERSION:3.0\r\nPRODID:-//Apple Inc.//iOS 18//EN\r\n\
N:Mehta;Arjun;;;\r\nFN:Arjun Mehta\r\nORG:Acme Traders;Purchasing\r\nTITLE:Buyer\r\n\
item1.EMAIL;type=INTERNET;type=pref:arjun@acme.co\r\nitem1.X-ABLabel:_$!<Work>!$_\r\n\
EMAIL;type=INTERNET;type=HOME:arjun.m@gmail.com\r\nTEL;type=CELL;type=VOICE;type=pref:+91 99000 55120\r\n\
ADR;type=WORK:;;12 MG Road;Pune;MH;411001;India\r\nBDAY:1985-03-14\r\n\
NOTE:Prefers WhatsApp\\, not calls.\\nUrgent orders only.\r\n\
X-SOCIALPROFILE;type=twitter:https://x.com/arjun\r\nUID:abc-123\r\n\
PHOTO;ENCODING=b;TYPE=JPEG:aGVs\r\n bG8=\r\nEND:VCARD\r\n";

    #[test]
    fn reads_an_apple_card() {
        let cards = parse(APPLE);
        assert_eq!(cards.len(), 1);
        let p = &cards[0];
        assert_eq!(p.uid, "abc-123");
        let c = &p.card;
        assert_eq!(c.display_name(), "Arjun Mehta");
        assert_eq!(
            (c.name.given.as_str(), c.name.family.as_str()),
            ("Arjun", "Mehta")
        );
        assert_eq!(c.organization, "Acme Traders");
        assert_eq!(c.department, "Purchasing");
        assert_eq!(c.emails[0], Typed::new("arjun@acme.co", ""));
        assert_eq!(c.emails[1], Typed::new("arjun.m@gmail.com", "home"));
        assert_eq!(c.phones[0], Typed::new("+91 99000 55120", "mobile"));
        assert_eq!(
            c.addresses[0].lines(),
            ["12 MG Road", "Pune, MH, 411001", "India"]
        );
        assert_eq!(c.birthday, "1985-03-14");
        assert_eq!(c.note, "Prefers WhatsApp, not calls.\nUrgent orders only.");
        assert_eq!(p.photo.as_deref(), Some(&b"hello"[..]));
    }

    #[test]
    fn reads_vcard_4_groups_and_birthdays() {
        let text = "BEGIN:VCARD\nVERSION:4.0\nKIND:group\nFN:Family\nUID:urn:uuid:g1\n\
MEMBER:urn:uuid:p1\nMEMBER:urn:uuid:p2\nEND:VCARD\n\
BEGIN:VCARD\nVERSION:4.0\nFN:Bo\nEMAIL;TYPE=work,pref:mailto:bo@x.in\nTEL;VALUE=uri;TYPE=\"voice,cell\":tel:+1-555\n\
BDAY:--0314\nCATEGORIES:Family,Friends\nEND:VCARD\n";
        let cards = parse(text);
        assert_eq!(cards.len(), 2);
        assert!(cards[0].group);
        assert_eq!(cards[0].members, ["p1", "p2"]);
        let bo = &cards[1];
        assert!(!bo.group);
        assert_eq!(bo.card.emails[0], Typed::new("bo@x.in", "work"));
        assert_eq!(bo.card.phones[0], Typed::new("+1-555", "mobile"));
        assert_eq!(bo.card.birthday, "--03-14");
        assert_eq!(bo.categories, ["Family", "Friends"]);
    }

    #[test]
    fn writes_and_reads_back_keeping_unknown_lines() {
        let mut card = parse(APPLE).remove(0).card;
        card.title = "Head of buying".into();
        card.emails.remove(0);
        let text = write("abc-123", &card, &["Suppliers".into()], Some(APPLE));
        assert!(text.contains("X-SOCIALPROFILE;type=twitter:https://x.com/arjun"));
        assert!(text.contains("PRODID:"));
        assert!(!text.contains("X-ABLabel"), "labels of rewritten lines go");
        assert!(text.contains("PHOTO;ENCODING=b"));
        assert!(text.lines().all(|l| l.len() <= 76));
        let back = parse(&text).remove(0);
        assert_eq!(back.uid, "abc-123");
        assert_eq!(back.categories, ["Suppliers"]);
        assert_eq!(back.card.title, "Head of buying");
        assert_eq!(back.card.emails.len(), 1);
        assert_eq!(back.card.note, card.note);
        assert_eq!(back.card.addresses, card.addresses);
        assert_eq!(back.photo.as_deref(), Some(&b"hello"[..]));
    }

    #[test]
    fn folds_long_lines_on_character_boundaries() {
        let card = Card {
            note: "é".repeat(100),
            ..Card::default()
        };
        let text = write("u", &card, &[], None);
        assert!(text.lines().all(|l| l.len() <= 76));
        assert_eq!(parse(&text)[0].card.note, "é".repeat(100));
    }
}
