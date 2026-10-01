// SPDX-License-Identifier: GPL-3.0-or-later

//! One mail body split into what the sender wrote and what a chat-style
//! view folds away: the earlier mail it quotes, the signature, and a mail
//! forwarded in it.
//!
//! Mail carries no structure for any of this, so it is read from the
//! conventions clients write: `>` lines under an "On … wrote:" line, the
//! `-- ` signature line, Outlook's "-----Original Message-----" and
//! "From: / Sent: / Subject:" blocks, and the "Forwarded message" lines of
//! Gmail, Apple Mail and Thunderbird. A quote or a forward is only cut
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

fn blank(line: &str) -> bool {
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

/// Whether `lines` hold nothing but blank lines and a signature.
fn only_signature(lines: &[&str]) -> bool {
    match lines.iter().position(|l| !blank(l)) {
        None => true,
        Some(first) => {
            delimiter(lines[first])
                || lines[first..]
                    .iter()
                    .all(|l| blank(l) || mobile_signature(l))
        }
    }
}

/// Where the signature at the end of `lines` starts: the last `-- ` line,
/// or a "Sent from my iPhone" line at the very end.
fn signature_start(lines: &[&str]) -> Option<usize> {
    lines.iter().rposition(|l| delimiter(l)).or_else(|| {
        let last = lines.iter().rposition(|l| !blank(l))?;
        mobile_signature(lines[last]).then_some(last)
    })
}

/// The signature delimiter, `-- ` (or `--` from clients that trim it).
fn delimiter(line: &str) -> bool {
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
        assert_eq!(t.said, "Approved.\n\nRegards\nOmar");
        let quoted = t.quoted.unwrap();
        assert!(quoted.starts_with("-----Original Message-----\nFrom: Priya"));
        assert!(quoted.ends_with("Please approve the budget."));
        assert_eq!(t.signature, None);
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
    fn only_a_quote() {
        let t = plain("> Are you there?\n");
        assert_eq!(t.said, "");
        assert_eq!(t.quoted.as_deref(), Some("> Are you there?"));
    }
}
