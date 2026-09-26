// SPDX-License-Identifier: GPL-3.0-or-later

//! Extracts the fields the store keeps for each message (ARCHITECTURE.md §5.3)
//! from a raw RFC 5322 message.

use mail_parser::{Address, HeaderValue, MessageParser};

/// Maximum snippet length in characters.
pub const SNIPPET_CHARS: usize = 200;

/// Header fields of one message, ready to be written to the store.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedMessage {
    /// `Message-ID` without angle brackets.
    pub message_id: Option<String>,
    pub subject: Option<String>,
    /// `Date` as seconds since the Unix epoch (UTC).
    pub date: Option<i64>,
    /// Size of the raw message in bytes.
    pub size: u64,
    pub has_attachments: bool,
    /// `List-Id` without angle brackets.
    pub list_id: Option<String>,
    /// Start of the body text, whitespace collapsed.
    pub snippet: Option<String>,
    pub participants: Vec<Participant>,
    /// Message-IDs of the ancestors, oldest first: `References`, then the
    /// `In-Reply-To` IDs it lacks. The last one is the parent.
    pub references: Vec<String>,
}

/// Role of an address in a message (`participant.role`).
pub use katna_store::ParticipantRole as Role;

/// One address of a message (`participant` row).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    pub role: Role,
    /// Address, trimmed and lower-cased.
    pub email_norm: String,
    /// Part after the last `@`, empty when there is none.
    pub domain: String,
    pub display_name: Option<String>,
}

/// Parses a raw message. Returns `None` if it has no headers at all.
///
/// Parsing is lenient: broken headers become `None` rather than errors,
/// because real mailboxes (Enron included) are full of them.
pub fn parse_message(raw: &[u8]) -> Option<ParsedMessage> {
    let message = MessageParser::default().parse(raw)?;
    if message.headers().is_empty() {
        return None;
    }

    let mut participants = Vec::new();
    for (role, address) in [
        (Role::From, message.from()),
        (Role::Sender, message.sender()),
        (Role::ReplyTo, message.reply_to()),
        (Role::To, message.to()),
        (Role::Cc, message.cc()),
        (Role::Bcc, message.bcc()),
    ] {
        if let Some(address) = address {
            push_participants(&mut participants, role, address);
        }
    }

    Some(ParsedMessage {
        message_id: message.message_id().and_then(non_empty),
        subject: message.subject().and_then(non_empty),
        date: message.date().map(|date| date.to_timestamp()),
        size: raw.len() as u64,
        has_attachments: message.attachment_count() > 0,
        list_id: list_id(message.list_id()),
        snippet: message
            .body_preview(SNIPPET_CHARS)
            .and_then(|text| non_empty(&collapse_whitespace(&text))),
        participants,
        references: references(&message),
    })
}

/// `Message-ID` and ancestors (as in [`ParsedMessage::references`]) from
/// the header alone; cheaper than [`parse_message`] for threading.
pub fn thread_headers(raw: &[u8]) -> (Option<String>, Vec<String>) {
    let end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|at| at + 4)
        .or_else(|| raw.windows(2).position(|w| w == b"\n\n").map(|at| at + 2))
        .unwrap_or(raw.len());
    match MessageParser::default().parse_headers(&raw[..end]) {
        Some(message) => (
            message.message_id().and_then(non_empty),
            references(&message),
        ),
        None => (None, Vec::new()),
    }
}

fn references(message: &mail_parser::Message<'_>) -> Vec<String> {
    let mut ids = Vec::new();
    for value in [message.references(), message.in_reply_to()] {
        let texts: Vec<&str> = match value {
            HeaderValue::Text(text) => vec![text.as_ref()],
            HeaderValue::TextList(list) => list.iter().map(|text| text.as_ref()).collect(),
            _ => Vec::new(),
        };
        for text in texts {
            // Some mailers put several IDs in one list entry.
            for id in text.split_whitespace() {
                let id = id.trim_start_matches('<').trim_end_matches('>');
                if !id.is_empty() && !ids.iter().any(|known| known == id) {
                    ids.push(id.to_owned());
                }
            }
        }
    }
    ids
}

fn push_participants(out: &mut Vec<Participant>, role: Role, address: &Address<'_>) {
    for addr in address.iter() {
        let Some(email) = addr.address() else {
            continue;
        };
        let email_norm = email.trim().to_lowercase();
        if email_norm.is_empty() {
            continue;
        }
        let domain = email_norm
            .rsplit_once('@')
            .map(|(_, domain)| domain.to_owned())
            .unwrap_or_default();
        out.push(Participant {
            role,
            email_norm,
            domain,
            display_name: addr.name().and_then(non_empty),
        });
    }
}

/// `List-Id: Some List <list.example.org>` → `list.example.org`.
fn list_id(value: &HeaderValue<'_>) -> Option<String> {
    let text = match value {
        HeaderValue::Address(address) => address.first().and_then(|addr| addr.address()),
        HeaderValue::Text(text) => Some(text.as_ref()),
        _ => value.as_text(),
    }?;
    let text = match (text.rfind('<'), text.rfind('>')) {
        (Some(start), Some(end)) if start < end => &text[start + 1..end],
        _ => text,
    };
    non_empty(text)
}

fn non_empty(text: &str) -> Option<String> {
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Shape of an Enron message (maildir/allen-p/_sent_mail/1.).
    const ENRON: &[u8] = b"Message-ID: <18782981.1075855378110.JavaMail.evans@thyme>\r
Date: Mon, 14 May 2001 16:39:00 -0700 (PDT)\r
From: phillip.allen@enron.com\r
To: tim.belden@enron.com\r
Subject: \r
Mime-Version: 1.0\r
Content-Type: text/plain; charset=us-ascii\r
Content-Transfer-Encoding: 7bit\r
X-From: Phillip K Allen\r
X-To: Tim Belden <Tim Belden/Enron@EnronXGate>\r
X-Folder: \\Phillip_Allen_Jan2002_1\\Allen, Phillip K.\\'Sent Mail\r
\r
Here is our forecast\r
\r
 ";

    #[test]
    fn parses_enron_message() {
        let parsed = parse_message(ENRON).unwrap();
        assert_eq!(
            parsed.message_id.as_deref(),
            Some("18782981.1075855378110.JavaMail.evans@thyme")
        );
        assert_eq!(parsed.subject, None);
        assert_eq!(parsed.date, Some(989883540));
        assert_eq!(parsed.size, ENRON.len() as u64);
        assert!(!parsed.has_attachments);
        assert_eq!(parsed.snippet.as_deref(), Some("Here is our forecast"));
        assert_eq!(
            parsed.participants,
            [
                Participant {
                    role: Role::From,
                    email_norm: "phillip.allen@enron.com".into(),
                    domain: "enron.com".into(),
                    display_name: None,
                },
                Participant {
                    role: Role::To,
                    email_norm: "tim.belden@enron.com".into(),
                    domain: "enron.com".into(),
                    display_name: None,
                },
            ]
        );
    }

    #[test]
    fn reads_references_oldest_first() {
        let raw = b"Message-ID: <c@x>\r\nReferences: <a@x>\r\n <b@x>\r\n\
                    In-Reply-To: <b@x> (Ada's message)\r\nSubject: Re: hi\r\n\r\nbody\r\n";
        assert_eq!(parse_message(raw).unwrap().references, ["a@x", "b@x"]);
        let (id, refs) = thread_headers(raw);
        assert_eq!(id.as_deref(), Some("c@x"));
        assert_eq!(refs, ["a@x", "b@x"]);
        // In-Reply-To alone, and an ID only In-Reply-To knows.
        let raw = b"In-Reply-To: <p@x>\r\nReferences: <o@x>\r\n\r\n";
        assert_eq!(thread_headers(raw).1, ["o@x", "p@x"]);
        assert_eq!(thread_headers(b"Subject: x\r\n\r\n"), (None, Vec::new()));
    }

    #[test]
    fn parses_names_lists_and_attachments() {
        let raw = b"From: \"Ada Lovelace\" <Ada@Example.ORG>\r
To: a@example.org, Group: b@example.org, c@example.org;\r
Cc: \"Charles\" <charles@example.net>\r
List-Id: Analytical Engines <engines.lists.example.org>\r
Subject: =?utf-8?q?Caf=C3=A9?=\r
Content-Type: multipart/mixed; boundary=\"b\"\r
\r
--b\r
Content-Type: text/plain\r
\r
Notes   attached.\r
--b\r
Content-Type: application/pdf\r
Content-Disposition: attachment; filename=\"notes.pdf\"\r
\r
%PDF\r
--b--\r
";
        let parsed = parse_message(raw).unwrap();
        assert_eq!(parsed.subject.as_deref(), Some("Café"));
        assert_eq!(parsed.list_id.as_deref(), Some("engines.lists.example.org"));
        assert!(parsed.has_attachments);
        assert_eq!(parsed.snippet.as_deref(), Some("Notes attached."));
        let summary: Vec<_> = parsed
            .participants
            .iter()
            .map(|p| {
                (
                    p.role.as_str(),
                    p.email_norm.as_str(),
                    p.display_name.as_deref(),
                )
            })
            .collect();
        assert_eq!(
            summary,
            [
                ("from", "ada@example.org", Some("Ada Lovelace")),
                ("to", "a@example.org", None),
                ("to", "b@example.org", None),
                ("to", "c@example.org", None),
                ("cc", "charles@example.net", Some("Charles")),
            ]
        );
    }

    #[test]
    fn rejects_non_mail() {
        assert_eq!(parse_message(b""), None);
    }
}
