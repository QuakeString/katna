// SPDX-License-Identifier: GPL-3.0-or-later

//! One mail body split into what the sender wrote and what a chat-style
//! view folds away: the earlier mail it quotes, the signature, and a mail
//! forwarded in it.
//!
//! Mail carries no structure for any of this, so it is read from the
//! conventions clients write: `>` lines under an "On … wrote:" line, the
//! `-- ` signature line, Outlook's "-----Original Message-----" and
//! "From: / Sent: / Subject:" blocks, and the "Forwarded message" lines of
//! Gmail, Apple Mail and Thunderbird. Signatures without a `-- ` line
//! are read from sign-offs, rules over contact details, blocks of contact
//! details and unsubscribe or confidentiality footers. A quote or a
//! forward is only cut
//! when nothing the sender wrote comes after it, so replies written
//! between quoted lines stay whole. [`plain`] reads text; the HTML side is
//! [`crate::html::trimmed`].

/// One mail body in pieces.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Trimmed<T> {
    /// What the sender wrote in this mail.
    pub said: T,
    /// The earlier mail it quotes, with its "On … wrote:" line.
    pub quoted: Option<T>,
    /// The sender's signature (and a legal disclaimer after it).
    pub signature: Option<T>,
    /// A mail forwarded in this one.
    pub forwarded: Option<Forwarded<T>>,
}

/// A mail forwarded inside another.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Forwarded<T> {
    /// From the forwarded header block, as written ("Demo Air <fares@demo.example>").
    pub from: Option<String>,
    pub date: Option<String>,
    pub subject: Option<String>,
    /// The forwarded mail's body, header block left out.
    pub body: T,
}

/// Splits a `text/plain` body.
pub fn plain(body: &str) -> Trimmed<String> {
    let body = body.replace("\r\n", "\n");
    let lines: Vec<&str> = body.lines().collect();

    // A forward runs to the end, unless it sits inside an Outlook quote
    // (a reply to a forward).
    let outlook = outlook_quote(&lines);
    let forward = forward(&lines).filter(|f| outlook.is_none_or(|o| o > f.start));
    let head = &lines[..forward.as_ref().map_or(lines.len(), |f| f.start)];

    let quote = quote(head);
    let above = quote.as_ref().map_or(head.len(), |q| q.start);
    let said_end = signature_start(&head[..above]).unwrap_or(above);
    let mut signature = join(&head[said_end..above]);
    if let Some(q) = &quote {
        // Signature lines under the quote.
        let below = join(&head[q.end..]);
        if !below.is_empty() {
            if !signature.is_empty() {
                signature.push_str("\n\n");
            }
            signature.push_str(&below);
        }
    }

    Trimmed {
        said: head[..said_end].join("\n").trim_end().to_owned(),
        quoted: quote.map(|q| join(&head[q.start..q.end])),
        signature: (!signature.is_empty()).then_some(signature),
        forwarded: forward.map(|f| f.forwarded),
    }
}

/// Lines joined, blank lines at either end left out.
fn join<S: AsRef<str>>(lines: &[S]) -> String {
    let blank = |l: &S| l.as_ref().trim().is_empty();
    let start = lines.iter().position(|l| !blank(l)).unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|l| !blank(l))
        .map_or(start, |e| e + 1);
    lines[start..end]
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn blank(line: &str) -> bool {
    line.trim().is_empty()
}

fn quoted_line(line: &str) -> bool {
    line.trim_start().starts_with('>')
}

/// A quote: lines `start..end`, with only signature lines after it.
struct Quote {
    start: usize,
    end: usize,
}

/// The quote at the end of `lines`: an Outlook header block, or `>` lines
/// (under their "On … wrote:" line) with nothing but a signature after.
fn quote(lines: &[&str]) -> Option<Quote> {
    let outlook = outlook_quote(lines).map(|start| Quote {
        start,
        end: lines.len(),
    });
    let cited = (|| {
        let first = lines.iter().position(|l| quoted_line(l))?;
        let last = lines.iter().rposition(|l| quoted_line(l))?;
        // Text between quoted lines is a reply written inline.
        if !lines[first..=last]
            .iter()
            .all(|l| quoted_line(l) || blank(l))
            || !only_signature(&lines[last + 1..])
        {
            return None;
        }
        let mut start = first;
        if let Some(above) = lines[..first].iter().rposition(|l| !blank(l)) {
            if attribution(lines[above]) {
                start = above;
            } else if above > 0
                && !blank(lines[above - 1])
                && attribution(&format!(
                    "{} {}",
                    lines[above - 1].trim(),
                    lines[above].trim()
                ))
            {
                start = above - 1;
            }
        }
        Some(Quote {
            start,
            end: last + 1,
        })
    })();
    match (outlook, cited) {
        (Some(o), Some(c)) => Some(if o.start < c.start { o } else { c }),
        (o, c) => o.or(c),
    }
}

/// Whether `lines` hold nothing but blank lines and a signature (or a
/// footer).
fn only_signature(lines: &[&str]) -> bool {
    match lines.iter().position(|l| !blank(l)) {
        None => true,
        Some(first) => {
            delimiter(lines[first])
                || lines[first..]
                    .iter()
                    .all(|l| blank(l) || mobile_signature(l))
                || tail_at(lines, first)
                || contact_block(lines).is_some_and(|(start, _)| start == first)
        }
    }
}

/// Where the signature at the end of `lines` starts: the last `-- ` line,
/// or a "Sent from my iPhone" line at the very end; above either, a
/// sign-off, rule, contact block or footer ([`tail_start`]).
fn signature_start(lines: &[&str]) -> Option<usize> {
    let marked = lines.iter().rposition(|l| delimiter(l)).or_else(|| {
        let last = lines.iter().rposition(|l| !blank(l))?;
        mobile_signature(lines[last]).then_some(last)
    });
    tail_start(&lines[..marked.unwrap_or(lines.len())]).or(marked)
}

/// How far up from the end a signature or footer is looked for, in lines
/// with text.
const TAIL_LINES: usize = 60;
/// A signature under a sign-off or a rule has at most this many lines of
/// at most `SIGNATURE_WIDTH` characters, and more than `NAME_LINES` only
/// with contact details among them.
const SIGNATURE_LINES: usize = 25;
const SIGNATURE_WIDTH: usize = 90;
const NAME_LINES: usize = 6;

/// Where the signature or footer at the end of `lines` starts when no
/// `-- ` line marks it, read from what people write: a sign-off ("Best
/// regards,") over a name, a rule (`_____`, `-----`) over contact details
/// or a footer, a block of contact details, or a footer that offers to
/// unsubscribe or says the mail is confidential. Something the sender
/// wrote always stays above it.
fn tail_start(lines: &[&str]) -> Option<usize> {
    let text: Vec<usize> = (0..lines.len()).filter(|&i| !blank(lines[i])).collect();
    let from = text.len().saturating_sub(TAIL_LINES).max(1);
    text.get(from..)?
        .iter()
        .copied()
        .find(|&i| tail_at(lines, i))
        .or_else(|| {
            contact_block(lines)
                .map(|(_, block)| block)
                .filter(|&b| text[0] < b)
        })
}

/// Whether a signature or footer starts at line `i` and runs to the end.
fn tail_at(lines: &[&str], i: usize) -> bool {
    let line = lines[i];
    if sign_off(line) {
        let rest = &lines[i + 1..];
        rest.iter().any(|l| !blank(l)) && signed(rest)
    } else if rule(line) {
        // A rule between parts of a mail is no signature: contact
        // details, a sign-off or a footer must follow it.
        let rest = &lines[i + 1..];
        let first = rest.iter().position(|l| !blank(l) && !rule(l));
        first.is_some_and(|f| signed(&rest[f..]))
            && rest
                .iter()
                .any(|l| contact(l) != 0 || footer_line(l) || sign_off(l))
    } else {
        (i == 0 || blank(lines[i - 1])) && footer(&lines[i..])
    }
}

/// Whether `lines` are a signature, a footer or both: name, title and
/// contact lines, then perhaps a rule or a footer.
fn signed(lines: &[&str]) -> bool {
    let end = lines
        .iter()
        .position(|l| rule(l) || footer_line(l))
        .map_or(lines.len(), |b| paragraph_start(lines, b));
    let own: Vec<&str> = lines[..end]
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let short = own.len() <= SIGNATURE_LINES
        && own.iter().all(|l| {
            let lower = l.to_lowercase();
            visible_len(l) <= SIGNATURE_WIDTH
                && !l.ends_with('?')
                && !lower.starts_with("p.s")
                && !lower.starts_with("ps:")
        })
        && (own.iter().any(|l| contact(l) != 0)
            || own.len() <= NAME_LINES && own.first().is_none_or(|l| name_line(l)));
    if !short {
        return false;
    }
    let rest = &lines[end..];
    match rest.iter().position(|l| !blank(l)) {
        None => !own.is_empty(),
        Some(f) if rule(rest[f]) => {
            let after = rest[f..].iter().position(|l| !blank(l) && !rule(l));
            after.is_none_or(|a| signed(&rest[f + a..]))
        }
        Some(f) => footer(&rest[f..]),
    }
}

/// The first line of the paragraph that line `i` is in.
fn paragraph_start(lines: &[&str], i: usize) -> usize {
    lines[..i]
        .iter()
        .rposition(|l| blank(l))
        .map_or(0, |b| b + 1)
}

/// Whether `lines` are a footer: paragraphs of notices, links and
/// addresses, one of them a notice ("unsubscribe", "you have received
/// this", a confidentiality notice).
fn footer(lines: &[&str]) -> bool {
    let mut notice = false;
    for paragraph in lines.split(|l| blank(l)).filter(|p| !p.is_empty()) {
        if paragraph.iter().any(|l| footer_line(l)) {
            notice = true;
        } else if !(paragraph.iter().any(|l| contact(l) != 0)
            || (paragraph.len() <= 2 && paragraph.iter().all(|l| l.trim().chars().count() <= 40)))
        {
            return false;
        }
    }
    notice
}

/// A block of contact details at the end of `lines`, with the name,
/// title and company over them: where its short lines start, and where
/// the block starts, below the first line with text. It needs two kinds
/// of detail (a phone number and an address, say), so a phone number
/// given in a mail is not taken for a signature.
fn contact_block(lines: &[&str]) -> Option<(usize, usize)> {
    // The short lines at the end, none of them a sentence.
    let mut start = lines.len();
    let mut kinds = 0;
    let mut first_contact = None;
    let mut count = 0;
    for i in (0..lines.len()).rev() {
        let line = lines[i].trim();
        if line.is_empty() {
            continue;
        }
        if line.chars().count() > SIGNATURE_WIDTH || sentence(line) || count == SIGNATURE_LINES {
            break;
        }
        let kind = contact(line);
        if kind != 0 {
            first_contact = Some(i);
        }
        kinds |= kind;
        count += 1;
        start = i;
    }
    if kinds.count_ones() < 2 {
        return None;
    }
    // From the paragraph of the first contact line, and the name, title
    // and company over it when each is a paragraph of its own (as HTML
    // mail reads in text).
    let first_text = lines.iter().position(|l| !blank(l))?;
    let mut block = paragraph_start(lines, first_contact?).max(start);
    for _ in 0..3 {
        let Some(above) = lines[..block].iter().rposition(|l| !blank(l)) else {
            break;
        };
        let own_paragraph = above > 0 && blank(lines[above - 1]);
        if above < start || above == first_text || !own_paragraph || !name_like(lines[above]) {
            break;
        }
        block = above;
    }
    Some((start, block))
}

/// A line that could be a name, a title or a company.
fn name_like(line: &str) -> bool {
    let line = line.trim();
    let words = line.split_whitespace().count();
    words <= 5 && !line.ends_with(['!', '?', ':', ',']) && !(line.ends_with('.') && words >= 3)
}

/// A line that could start a signature: a name ("Omar Haddad", "Dr. A.
/// Rao"), not the start of something more to say.
fn name_line(line: &str) -> bool {
    let line = line.trim();
    line.split_whitespace().count() <= 4 && !line.ends_with(['.', '!', '?', ':', ','])
}

/// A line that reads as a sentence rather than a name or an address.
fn sentence(line: &str) -> bool {
    line.ends_with(['.', '!', '?', ':']) && line.split_whitespace().count() >= 5
}

/// A line drawn across: `_____`, `-----`, `=====`, `*****`.
pub(crate) fn rule(line: &str) -> bool {
    let line = line.trim();
    line.chars().count() >= 5
        && line.chars().all(|c| {
            matches!(
                c,
                '_' | '-' | '=' | '*' | '~' | '\u{2014}' | '\u{2500}' | '\u{2013}'
            )
        })
}

/// A line that closes a mail, like "Best regards,".
pub(crate) fn sign_off(line: &str) -> bool {
    const SIGN_OFFS: &[&str] = &[
        "all the best",
        "best",
        "best regards",
        "best wishes",
        "br",
        "cheers",
        "cordially",
        "kind regards",
        "many thanks",
        "regards",
        "regards and thanks",
        "regards & thanks",
        "respectfully",
        "rgds",
        "sincerely",
        "sincerely yours",
        "take care",
        "thank you",
        "thanking you",
        "thanks",
        "thanks again",
        "thanks and regards",
        "thanks & regards",
        "thanks and best regards",
        "thanks & best regards",
        "thanks in advance",
        "thanks n regards",
        "thx",
        "warm regards",
        "warmest regards",
        "warm wishes",
        "warmly",
        "with best regards",
        "with best wishes",
        "with kind regards",
        "with regards",
        "with thanks",
        "with warm regards",
        "yours",
        "yours faithfully",
        "yours sincerely",
        "yours truly",
        // German, French, Spanish, Italian, Dutch.
        "beste grüße",
        "freundliche grüße",
        "liebe grüße",
        "mit freundlichen grüßen",
        "viele grüße",
        "bien cordialement",
        "cordialement",
        "bien à vous",
        "atentamente",
        "saludos",
        "un saludo",
        "cordiali saluti",
        "met vriendelijke groet",
    ];
    let line = line
        .trim()
        .trim_end_matches([',', '.', '!', ' ', '\u{a0}'])
        .to_lowercase();
    let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
    SIGN_OFFS.contains(&line.as_str())
}

/// The kinds of contact detail a line holds, as bits.
const PHONE: u8 = 1;
const EMAIL: u8 = 2;
const WEB: u8 = 4;
const PLACE: u8 = 8;

fn contact(line: &str) -> u8 {
    let lower = line.to_lowercase();
    let mut kinds = 0;
    if lower.split_whitespace().any(|w| {
        w.split_once('@')
            .is_some_and(|(user, host)| !user.is_empty() && host.contains('.'))
    }) {
        kinds |= EMAIL;
    }
    if lower.contains("www.") || lower.contains("http://") || lower.contains("https://") {
        kinds |= WEB;
    }
    if line.split(['/', '|', ',']).any(phone) {
        kinds |= PHONE;
    }
    let label = lower.trim_start_matches(['|', '\u{2022}', '\u{b7}', '-', '*', ' ']);
    if label.ends_with("office:")
        || [
            "address",
            "office",
            "regd",
            "registered office",
            "head office",
            "hq",
        ]
        .iter()
        .any(|w| label.starts_with(w) && label[w.len()..].trim_start().starts_with(':'))
        || postal(line) && line.contains(',')
    {
        kinds |= PLACE;
    }
    kinds
}

/// How long `line` reads, without the links and pictures text versions
/// of HTML mail spell out (`<https://…>`, `[image: logo]`).
fn visible_len(line: &str) -> usize {
    let mut len = 0;
    let mut hidden: Option<char> = None;
    let mut rest = line;
    while let Some(c) = rest.chars().next() {
        match hidden {
            Some(close) if c == close => hidden = None,
            Some(_) => {}
            None if c == '<' && rest[1..].trim_start().starts_with("http") => hidden = Some('>'),
            None if c == '[' && rest[1..].to_lowercase().starts_with("image:") => {
                hidden = Some(']');
            }
            None => len += 1,
        }
        rest = &rest[c.len_utf8()..];
    }
    len
}

/// Whether `text` holds a phone number: 7 to 15 digits with spaces,
/// dashes, dots and brackets, not a date.
pub(crate) fn phone(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if !(c.is_ascii_digit() || c == '+' || c == '(')
            || (at > 0 && chars[at - 1].is_alphanumeric())
        {
            at += 1;
            continue;
        }
        let mut end = at + 1;
        while end < chars.len()
            && (chars[end].is_ascii_digit() || " +-.()\u{a0}".contains(chars[end]))
        {
            end += 1;
        }
        let run: String = chars[at..end].iter().collect();
        let digits = run.chars().filter(char::is_ascii_digit).count();
        let letter_after = chars.get(end).is_some_and(|c| c.is_alphabetic());
        let run = run.trim();
        let date =
            run.len() == 10 && run.chars().filter(|c| matches!(c, '-' | '/' | '.')).count() == 2;
        if (7..=15).contains(&digits) && !letter_after && !date {
            return true;
        }
        at = end;
    }
    false
}

/// Whether `line` holds a postal code: five or six digits on their own.
pub(crate) fn postal(line: &str) -> bool {
    line.split(|c: char| !c.is_ascii_alphanumeric())
        .any(|w| (5..=6).contains(&w.len()) && w.chars().all(|c| c.is_ascii_digit()))
}

/// A line of a footer: unsubscribing, why the mail came, a
/// confidentiality notice, copyright.
pub(crate) fn footer_line(line: &str) -> bool {
    const NOTICES: &[&str] = &[
        "unsubscribe",
        "you have received this",
        "you received this",
        "you are receiving this",
        "you're receiving this",
        "you\u{2019}re receiving this",
        "this email was sent to",
        "this e-mail was sent to",
        "this message was sent to",
        "to stop receiving",
        "no longer wish to receive",
        "manage your preferences",
        "update your preferences",
        "email preferences",
        "notification settings",
        "opt out",
        "opt-out",
        "view this email in your browser",
        "view in browser",
        "all rights reserved",
        "privacy policy",
        "please do not reply",
        "do not reply to this",
        "this is an automated",
        "this is a system generated",
        "this is a system-generated",
        "this email and any",
        "this e-mail and any",
        "this message and any",
        "this message is intended",
        "this email is intended",
        "this e-mail is intended",
        "intended recipient",
        "intended solely",
        "disclaimer",
        "before printing",
        "think before you print",
        "print only when necessary",
    ];
    let lower = line.to_lowercase();
    NOTICES.iter().any(|n| lower.contains(n))
        || lower.contains('\u{a9}')
        || (lower.contains("confidential")
            && ["privileged", "recipient", "notify", "intended"]
                .iter()
                .any(|w| lower.contains(w)))
}

/// Where the lines `text` ends with, the same as at the end of `other`
/// (another mail from the same person), start in `text`: a signature no
/// sign-off, rule or `-- ` line gives away. At least two lines, and both
/// mails keep something of their own above them.
pub fn shared_tail(text: &str, other: &str) -> Option<usize> {
    let lines = |s: &str| -> Vec<(usize, String)> {
        let mut at = 0;
        let mut out = Vec::new();
        for line in s.split_inclusive('\n') {
            let words = line.split_whitespace().collect::<Vec<_>>().join(" ");
            if !words.is_empty() {
                out.push((at, words));
            }
            at += line.len();
        }
        out
    };
    let ours = lines(text);
    let theirs = lines(other);
    let same = ours
        .iter()
        .rev()
        .zip(theirs.iter().rev())
        .take_while(|(a, b)| a.1 == b.1)
        .count();
    let own = ours.len() - same;
    let chars: usize = ours[own..].iter().map(|(_, l)| l.chars().count()).sum();
    (same >= 2 && chars >= 16 && own > 0 && theirs.len() > same).then(|| ours[own].0)
}

/// The signature delimiter, `-- ` (or `--` from clients that trim it).
pub(crate) fn delimiter(line: &str) -> bool {
    line.trim_end() == "--"
}

/// The line phones and webmail add under a message.
pub(crate) fn mobile_signature(line: &str) -> bool {
    let line = line.trim();
    let lower = line.to_lowercase();
    line.chars().count() < 80
        && [
            "sent from my ",
            "sent from outlook",
            "sent from mail for ",
            "sent from yahoo mail",
            "sent from samsung",
            "sent from proton mail",
            "sent with proton mail",
            "get outlook for ",
            "envoyé de mon ",
            "von meinem ",
            "enviado desde mi ",
        ]
        .iter()
        .any(|p| lower.starts_with(p))
}

/// An "On Tue, 30 Sep 2026 at 18:02, Priya Nair <p@x> wrote:" line, or its
/// German, French or Spanish form.
pub(crate) fn attribution(line: &str) -> bool {
    let line = line.trim();
    let lower = line.to_lowercase();
    if line.chars().count() > 300 {
        return false;
    }
    let ends = [
        "wrote:",
        "schrieb:",
        "a écrit :",
        "a écrit:",
        "escribió:",
    ]
    .iter()
    .any(|e| lower.ends_with(e))
        // "Am 30.09.2026 um 18:02 schrieb Priya Nair <p@x>:"
        || (lower.starts_with("am ") && lower.contains(" schrieb ") && lower.ends_with(':'));
    let starts = ["on ", "am ", "le ", "el "]
        .iter()
        .any(|s| lower.starts_with(s));
    ends && (starts || line.contains('@') || line.chars().any(|c| c.is_ascii_digit()))
}

/// The line that opens a forwarded mail.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Marker {
    /// "Forwarded message", "Begin forwarded message:".
    Forward,
    /// Thunderbird's "-------- Original Message --------": a forward when
    /// headers follow it, else the start of a quote.
    Original,
    /// Outlook's "-----Original Message-----": a quote.
    OutlookOriginal,
}

pub(crate) fn marker(line: &str) -> Option<Marker> {
    let line = line.trim();
    let lower = line.to_lowercase();
    if [
        "begin forwarded message:",
        "anfang der weitergeleiteten nachricht:",
        "début du message réexpédié :",
        "début du message réexpédié:",
    ]
    .contains(&lower.as_str())
    {
        return Some(Marker::Forward);
    }
    if !line.starts_with("---") {
        return None;
    }
    let inner = lower.trim_matches('-');
    match inner.trim() {
        "forwarded message"
        | "weitergeleitete nachricht"
        | "message transféré"
        | "mensaje reenviado" => Some(Marker::Forward),
        "original message" if inner.starts_with(' ') => Some(Marker::Original),
        "original message" => Some(Marker::OutlookOriginal),
        _ => None,
    }
}

/// A header field as clients write it above a quoted or forwarded mail.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Field {
    From,
    /// `Date:` or Outlook's `Sent:`.
    Date,
    Subject,
    /// `To:`, `Cc:`, `Reply-To:`.
    Other,
}

/// `From: Demo Air <fares@demo.example>` as its field and value. Outlook's
/// `*From:*` bold markers and a few languages are understood.
pub(crate) fn header_line(line: &str) -> Option<(Field, &str)> {
    let (key, value) = line.trim().split_once(':')?;
    let key = key.trim().trim_matches('*').trim().to_lowercase();
    let field = match key.as_str() {
        "from" | "von" | "de" => Field::From,
        "date" | "sent" | "datum" | "gesendet" | "envoyé" | "enviado" | "fecha" => Field::Date,
        "subject" | "betreff" | "objet" | "asunto" => Field::Subject,
        "to" | "cc" | "reply-to" | "an" | "à" | "para" => Field::Other,
        _ => return None,
    };
    Some((field, value.trim().trim_start_matches('*').trim()))
}

/// The From, Date and Subject of a header block.
#[derive(Default)]
pub(crate) struct Headers {
    pub from: Option<String>,
    pub date: Option<String>,
    pub subject: Option<String>,
    pub lines: usize,
}

impl Headers {
    /// Reads the header lines at the start of `lines`.
    pub fn read<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut headers = Headers::default();
        for line in lines {
            let Some((field, value)) = header_line(line.as_ref()) else {
                break;
            };
            let value = Some(value.to_owned()).filter(|v| !v.is_empty());
            match field {
                Field::From => headers.from = headers.from.take().or(value),
                Field::Date => headers.date = headers.date.take().or(value),
                Field::Subject => headers.subject = headers.subject.take().or(value),
                Field::Other => {}
            }
            headers.lines += 1;
        }
        headers
    }
}

/// Where an Outlook quote starts: an "-----Original Message-----" line, or
/// a From / Sent / Subject block (with the `____` rule above it).
fn outlook_quote(lines: &[&str]) -> Option<usize> {
    for (ix, line) in lines.iter().enumerate() {
        match marker(line) {
            Some(Marker::OutlookOriginal) => return Some(ix),
            Some(Marker::Original) if Headers::read(&lines[ix + 1..]).from.is_none() => {
                return Some(ix);
            }
            _ => {}
        }
        if header_line(line).is_some_and(|(f, _)| f == Field::From) {
            let h = Headers::read(&lines[ix..]);
            if h.date.is_some() && h.subject.is_some() {
                let rule = lines[..ix]
                    .iter()
                    .rposition(|l| !blank(l))
                    .filter(|&r| lines[r].trim().starts_with("_____"));
                return Some(rule.unwrap_or(ix));
            }
        }
    }
    None
}

struct Forward {
    start: usize,
    forwarded: Forwarded<String>,
}

/// The first forwarded mail in `lines`, to the end.
fn forward(lines: &[&str]) -> Option<Forward> {
    lines.iter().enumerate().find_map(|(start, line)| {
        let kind = marker(line)?;
        if kind == Marker::OutlookOriginal {
            return None;
        }
        let rest = &lines[start + 1..];
        let first = rest.iter().position(|l| !blank(l)).unwrap_or(rest.len());
        // Apple Mail quotes the forwarded mail in plain text.
        let rest: Vec<&str> = if rest.get(first).is_some_and(|l| quoted_line(l)) {
            rest[first..]
                .iter()
                .map(|l| {
                    let l = l.trim_start();
                    l.strip_prefix("> ")
                        .or_else(|| l.strip_prefix('>'))
                        .unwrap_or(l)
                })
                .collect()
        } else {
            rest[first..].to_vec()
        };
        let headers = Headers::read(&rest);
        if kind == Marker::Original && headers.from.is_none() {
            return None;
        }
        Some(Forward {
            start,
            forwarded: Forwarded {
                from: headers.from,
                date: headers.date,
                subject: headers.subject,
                body: join(&rest[headers.lines..]),
            },
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reply_with_quote_and_signature() {
        let t = plain(
            "Friday works for me.\r\n\r\nSee you then,\r\nRajat\r\n\r\n-- \r\n\
Rajat Roy\r\nDemo Labs\r\n\r\n\
On Tue, 30 Sep 2026 at 18:02, Priya Nair <priya@demo.example>\r\n\
wrote:\r\n\r\n> Could we meet on Friday?\r\n>\r\n> Priya\r\n",
        );
        assert_eq!(t.said, "Friday works for me.\n\nSee you then,\nRajat");
        assert_eq!(t.signature.as_deref(), Some("-- \nRajat Roy\nDemo Labs"));
        assert_eq!(
            t.quoted.as_deref(),
            Some(
                "On Tue, 30 Sep 2026 at 18:02, Priya Nair <priya@demo.example>\nwrote:\n\n\
> Could we meet on Friday?\n>\n> Priya"
            )
        );
        assert_eq!(t.forwarded, None);
    }

    #[test]
    fn signature_under_the_quote() {
        let t = plain(
            "Yes.\n\nAm 30.09.2026 um 18:02 schrieb Priya Nair:\n> Kommst du?\n\n-- \nRajat\n",
        );
        assert_eq!(t.said, "Yes.");
        assert_eq!(
            t.quoted.as_deref(),
            Some("Am 30.09.2026 um 18:02 schrieb Priya Nair:\n> Kommst du?")
        );
        assert_eq!(t.signature.as_deref(), Some("-- \nRajat"));
    }

    #[test]
    fn outlook_original_message() {
        let t = plain(
            "Approved.\n\nRegards\nOmar\n\n-----Original Message-----\n\
From: Priya Nair <priya@demo.example>\nSent: Tuesday, September 30, 2026 6:02 PM\n\
To: Omar Haddad <omar@demo.example>\nSubject: Budget\n\nPlease approve the budget.\n",
        );
        assert_eq!(t.said, "Approved.");
        let quoted = t.quoted.unwrap();
        assert!(quoted.starts_with("-----Original Message-----\nFrom: Priya"));
        assert!(quoted.ends_with("Please approve the budget."));
        assert_eq!(t.signature.as_deref(), Some("Regards\nOmar"));
    }

    #[test]
    fn outlook_header_block() {
        let t = plain(
            "Done.\n\n________________________________\nFrom: Priya Nair <priya@demo.example>\n\
Sent: 30 September 2026 18:02\nTo: Omar Haddad\nSubject: Budget\n\nPlease?\n",
        );
        assert_eq!(t.said, "Done.");
        assert!(t.quoted.unwrap().starts_with("_____"));
    }

    #[test]
    fn gmail_forward() {
        let t = plain(
            "FYI, our tickets.\n\n---------- Forwarded message ---------\n\
From: Demo Air <fares@demo.example>\nDate: Tue, 30 Sep 2026 at 18:02\n\
Subject: Your booking\nTo: <priya@demo.example>\n\n\nYour flight is booked.\n\n\
On Mon, 29 Sep 2026, Priya Nair <priya@demo.example> wrote:\n> Please book.\n",
        );
        assert_eq!(t.said, "FYI, our tickets.");
        assert_eq!(t.quoted, None);
        assert_eq!(
            t.forwarded,
            Some(Forwarded {
                from: Some("Demo Air <fares@demo.example>".into()),
                date: Some("Tue, 30 Sep 2026 at 18:02".into()),
                subject: Some("Your booking".into()),
                body: "Your flight is booked.\n\n\
On Mon, 29 Sep 2026, Priya Nair <priya@demo.example> wrote:\n> Please book."
                    .into(),
            })
        );
    }

    #[test]
    fn apple_and_thunderbird_forwards() {
        let t = plain(
            "Begin forwarded message:\n\n> From: Demo Air <fares@demo.example>\n\
> Subject: Your booking\n> Date: 30 September 2026 at 18:02:11 BST\n\
> To: Priya Nair <priya@demo.example>\n> \n> Your flight is booked.\n",
        );
        assert_eq!(t.said, "");
        let f = t.forwarded.unwrap();
        assert_eq!(f.from.as_deref(), Some("Demo Air <fares@demo.example>"));
        assert_eq!(f.subject.as_deref(), Some("Your booking"));
        assert_eq!(f.body, "Your flight is booked.");

        let t = plain(
            "See below.\n\n-------- Forwarded Message --------\nSubject: \tYour booking\n\
Date: \tTue, 30 Sep 2026 18:02:11 +0100\nFrom: \tDemo Air <fares@demo.example>\n\n\
Your flight is booked.\n",
        );
        assert_eq!(t.said, "See below.");
        let f = t.forwarded.unwrap();
        assert_eq!(f.date.as_deref(), Some("Tue, 30 Sep 2026 18:02:11 +0100"));
        assert_eq!(f.body, "Your flight is booked.");
    }

    #[test]
    fn sent_from_my_iphone() {
        let t = plain("On my way!\n\nSent from my iPhone\n");
        assert_eq!(t.said, "On my way!");
        assert_eq!(t.signature.as_deref(), Some("Sent from my iPhone"));
        assert_eq!(t.quoted, None);

        let t =
            plain("Sure\n\nGet Outlook for iOS\n\nOn 30 Sep 2026, Priya Nair wrote:\n> Lunch?\n");
        assert_eq!(t.said, "Sure");
        assert_eq!(t.signature.as_deref(), Some("Get Outlook for iOS"));
        assert!(t.quoted.is_some());
    }

    #[test]
    fn inline_replies_are_kept() {
        let body = "On Tue, 30 Sep 2026 at 18:02, Priya Nair <priya@demo.example> wrote:\n\
> Can you do Friday?\n\nYes, after lunch.\n\n> And bring the slides?\n\nWill do.";
        let t = plain(body);
        assert_eq!(t.said, body);
        assert_eq!(t.quoted, None);
        assert_eq!(t.signature, None);
    }

    #[test]
    fn nothing_to_cut() {
        let body = "Hi Priya,\n\nthe report is attached -- the numbers\nare in section 2.\n\nRajat";
        assert_eq!(
            plain(body),
            Trimmed {
                said: body.to_owned(),
                ..Trimmed::default()
            }
        );
    }

    #[test]
    fn sign_off_and_name() {
        let t = plain("The slides are attached.\n\nBest regards,\nOmar Haddad\nDemo Travel Co\n");
        assert_eq!(t.said, "The slides are attached.");
        assert_eq!(
            t.signature.as_deref(),
            Some("Best regards,\nOmar Haddad\nDemo Travel Co")
        );

        // Thanks that start a sentence, or close a mail of nothing else,
        // stay.
        let body = "Thanks for the slides.\n\nSee you Friday.\nOmar";
        assert_eq!(plain(body).said, body);
        assert_eq!(plain("Thanks!\nOmar").said, "Thanks!\nOmar");
        let body = "Lunch is booked.\n\nThanks,\nCan you bring the slides?";
        assert_eq!(plain(body).said, body);
        let body = "Lunch is booked.\n\nThanks!\nWill send the menu later.";
        assert_eq!(plain(body).said, body);
        // A rule between parts of a mail.
        let body = "The agenda:\n\n-----\nBudget\nHiring";
        assert_eq!(plain(body).said, body);
    }

    #[test]
    fn company_signature_under_rules() {
        let t = plain(
            "Dear Sir/Madam,\n\nPlease send your quotation for the maintenance contract.\n\n\
For the scope of work, please see the attached document.\n\n\n\n\n\
______________________________________________\n\
______________________________________________\n______________\n\
DEMO NAME.MANAGER\nPROCUREMENT Demo Carbide & Chemicals Ltd\nDemo Complex Building\n\
Post Box No.103, Demo Town\n(ISO 9001: 2015 Certified Company)\n\
Contact Number: +975 0000 1111/ 0000 2222\nEmail ID: buyer@demo.example\n",
        );
        assert_eq!(
            t.said,
            "Dear Sir/Madam,\n\nPlease send your quotation for the maintenance contract.\n\n\
For the scope of work, please see the attached document."
        );
        let signature = t.signature.unwrap();
        assert!(signature.starts_with("_____"));
        assert!(signature.ends_with("Email ID: buyer@demo.example"));
    }

    #[test]
    fn notification_footer() {
        let t = plain(
            "Hello Admin,\n\nYour payment of 1,200.00 is due on 5 October 2026. Pay it from \
the billing page to keep your services running.\n\nSincerely,\nDemo Cloud Billing\n\n\
-------------------------------------------------------------------\n\n\
Help\nCentre<https://help.demo.example/billing\n/understand-your-bill>\n\n\
Contact us<https://admin.demo.example/support>\n\n\
Demo Cloud customer ID:  demo.example\nPayments profile ID:  1111-2222-3333\n\n\
Demo Cloud LLC 1 Example Way, Springfield, CA 90000\n\n\
To stop receiving emails about this payments profile, you can\n\
unsubscribe<https://demo.example/u/AAuDWvkxpsoZogOwBhri52usKAJlUPpQUuhka8d3gzdYLZHav2y0sNs2HOjMzmxBjX6mbVuEu>.\n\n\n\
You have received this mandatory service announcement to update you about\n\
important changes to Demo Cloud or your account.\n\nDemo Cloud\n",
        );
        assert_eq!(
            t.said,
            "Hello Admin,\n\nYour payment of 1,200.00 is due on 5 October 2026. Pay it from \
the billing page to keep your services running."
        );
        let signature = t.signature.unwrap();
        assert!(signature.starts_with("Sincerely,\nDemo Cloud Billing\n\n-----"));
        assert!(signature.ends_with("\n\nDemo Cloud"));
    }

    #[test]
    fn contact_block_as_html_reads() {
        // HTML mail in text: every line a paragraph.
        let t = plain(
            "Dear Sir,\n\nPlease find the revised quotation attached.\n\nThanks & Regards\n\n\
Demo Basu\n\nK. G. Demo Services\n\n\n\nCorporate Office:\n\n\
Demo IT Park, Phase - I, Module no. 201, New Town\n\nAction Area 1, Kolkata 700000, India\n\n\
Registered Office:\n\n1 Demo Road, Kolkata 700001, India\n\n\
| M    :  + 91 90000 00000 / 90000 00001\n\n| E     : sales@demo.example\n\n\
| W    : www.demo.example <http://www.demo.example/>\n",
        );
        assert_eq!(
            t.said,
            "Dear Sir,\n\nPlease find the revised quotation attached."
        );
        assert!(
            t.signature
                .unwrap()
                .starts_with("Thanks & Regards\n\nDemo Basu")
        );

        // No sign-off: the details and the name over them.
        let t = plain(
            "Dear Sir,\n\nPlease find the revised quotation attached.\n\nDemo Basu\n\n\
K. G. Demo Services\n\nCorporate Office:\n\nAction Area 1, Kolkata 700000, India\n\n\
| M    :  + 91 90000 00000\n\n| E     : sales@demo.example\n",
        );
        assert_eq!(
            t.said,
            "Dear Sir,\n\nPlease find the revised quotation attached."
        );
        assert!(t.signature.unwrap().starts_with("Demo Basu"));

        // A phone number someone asks to be called on stays.
        let body = "Can you call me?\n\nMy numbers:\n+91 90000 00000\n+91 90000 00001";
        assert_eq!(plain(body).said, body);
    }

    #[test]
    fn disclaimer_under_a_reply() {
        let t = plain(
            "Approved, go ahead.\n\nOn Tue, 30 Sep 2026 at 18:02, Priya Nair <priya@demo.example> \
wrote:\n> Can I order the parts?\n\n\
DISCLAIMER: This email and any files sent with it are confidential and intended solely \
for the use of the addressee.\n",
        );
        assert_eq!(t.said, "Approved, go ahead.");
        assert!(t.quoted.is_some());
        assert!(t.signature.unwrap().starts_with("DISCLAIMER:"));
    }

    #[test]
    fn a_signature_two_mails_share() {
        let first = "Can we move the call to 4?\n\nArjun Mehta\nDemo Travel Co\n";
        let second = "Done, invite sent.\n\nArjun Mehta\nDemo Travel Co";
        let at = shared_tail(second, first).unwrap();
        assert_eq!(&second[..at], "Done, invite sent.\n\n");
        // One line in common is a coincidence; the same mail twice is no
        // signature.
        assert_eq!(shared_tail("Done.\nSee you", "Sure.\nSee you"), None);
        assert_eq!(shared_tail(first, first), None);
    }

    #[test]
    fn html_company_mail_in_text() {
        // HTML mail reaches the chat as text.
        let raw = "From: Demo Supplies <sales@demo.example>\r\nTo: buyer@demo.example\r\n\
Subject: Offer\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\
<html><body><p>Hello,</p><p>Our offer for the spare parts is attached; prices hold until \
31 October.</p><p>Warm Regards,</p><p><b>Demo Sharma</b><br>Sales Head</p>\
<p>Demo Supplies Pvt. Ltd.<br>12 Example Street, Pune 411000, India<br>\
Tel: +91 20 0000 0000 | www.demo.example</p>\
<p style=\"font-size:10px\">This e-mail and any attachments are confidential and intended \
only for the addressee. If you are not the intended recipient, please notify the sender.</p>\
<p><a href=\"https://demo.example/u\">Unsubscribe</a> | &copy; 2026 Demo Supplies</p>\
</body></html>\r\n";
        let view = crate::plain::message_view(raw.as_bytes());
        let t = plain(&view.body);
        assert_eq!(
            t.said.split_whitespace().collect::<Vec<_>>().join(" "),
            "Hello, Our offer for the spare parts is attached; prices hold until 31 October."
        );
        let signature = t.signature.unwrap();
        assert!(signature.starts_with("Warm Regards,"), "{signature}");
        assert!(signature.contains("Unsubscribe"), "{signature}");
    }

    #[test]
    fn signature_with_links_and_banners() {
        let body = "Could we talk next week?\n\nBest regards,\nDemo Rao | Head of Growth\n\
                    Demo Labs\n+1 555 010 7788\n[image: Demo Labs] <https://demolabs.example>\n\
                    [image: Best Workplace 2026 banner]\nFollow us: Facebook \
                    <https://facebook.com/demolabs> | Twitter <https://twitter.com/demolabs> | \
                    LinkedIn <https://www.linkedin.com/company/demolabs>\n\n\
                    Please consider the environment before printing this e-mail.\n\n\
                    CONFIDENTIALITY: This e-mail and any attachments are confidential.";
        let t = plain(body);
        assert_eq!(t.said, "Could we talk next week?");
        assert!(t.signature.unwrap().starts_with("Best regards,"));
    }

    #[test]
    fn only_a_quote() {
        let t = plain("> Are you there?\n");
        assert_eq!(t.said, "");
        assert_eq!(t.quoted.as_deref(), Some("> Are you there?"));
    }
}
