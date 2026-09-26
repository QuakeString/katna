// SPDX-License-Identifier: GPL-3.0-or-later

//! Extracts the fields the store keeps for each message (ARCHITECTURE.md §5.3)
//! from a raw RFC 5322 message.

use katna_core::{MailCategory, MailFacts, classify};
use mail_parser::{Address, HeaderValue, Message, MessageParser, MimeHeaders};

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
    /// `In-Reply-To` without angle brackets (the first ID if there are
    /// several).
    pub in_reply_to: Option<String>,
    /// `References` without angle brackets, oldest first.
    pub references: Vec<String>,
    /// Inbox tab, from the header-based classifier.
    pub category: MailCategory,
}

/// The header facts threading and categories need, for messages stored
/// before either existed (see [`crate::backfill`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderLinks {
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
    pub category: MailCategory,
}

impl HeaderLinks {
    /// `references` as the store wants them.
    pub fn reference_strs(&self) -> Vec<&str> {
        self.references.iter().map(String::as_str).collect()
    }
}

impl ParsedMessage {
    /// `references` as the store wants them.
    pub fn reference_strs(&self) -> Vec<&str> {
        self.references.iter().map(String::as_str).collect()
    }
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

    let links = links(&message, &participants);
    Some(ParsedMessage {
        message_id: message.message_id().and_then(non_empty),
        subject: message.subject().and_then(non_empty),
        date: message.date().map(|date| date.to_timestamp()),
        size: raw.len() as u64,
        has_attachments: message.attachments().any(|part| {
            let mime = part.content_type().map(|t| match t.subtype() {
                Some(sub) => format!("{}/{sub}", t.ctype()),
                None => t.ctype().to_owned(),
            });
            crate::mime::is_attachment(&crate::mime::PartInfo {
                mime: mime.as_deref().unwrap_or("text/plain"),
                disposition: part.content_disposition().map(|d| d.ctype()),
                content_id: part.content_id().is_some(),
                filename: part.attachment_name().is_some(),
            })
        }),
        list_id: list_id(message.list_id()),
        snippet: message
            .body_preview(SNIPPET_CHARS)
            .and_then(|text| non_empty(&collapse_whitespace(&text))),
        participants,
        in_reply_to: links.in_reply_to,
        references: links.references,
        category: links.category,
    })
}

/// Reads only the header of `raw`, for threading and categories. Cheaper
/// than [`parse_message`], which also looks at the body. Returns `None` if
/// it has no headers.
pub fn parse_links(raw: &[u8]) -> Option<HeaderLinks> {
    let message = MessageParser::default().parse_headers(raw)?;
    if message.headers().is_empty() {
        return None;
    }
    let mut from = Vec::new();
    if let Some(address) = message.from() {
        push_participants(&mut from, Role::From, address);
    }
    Some(links(&message, &from))
}

fn links(message: &Message<'_>, participants: &[Participant]) -> HeaderLinks {
    let mut facts = MailFacts::from_headers(message.headers_raw());
    // Decoded values beat raw ones.
    if let Some(from) = participants.iter().find(|p| p.role == Role::From) {
        facts.from = Some(from.email_norm.clone());
    }
    if let Some(subject) = message.subject() {
        facts.subject = Some(subject.to_owned());
    }
    HeaderLinks {
        in_reply_to: message_ids(message.in_reply_to()).into_iter().next(),
        references: message_ids(message.references()),
        category: classify(&facts),
    }
}

/// Message IDs of an `In-Reply-To` or `References` value, without angle
/// brackets.
fn message_ids(value: &HeaderValue<'_>) -> Vec<String> {
    let texts: Vec<&str> = match value {
        HeaderValue::Text(text) => vec![text.as_ref()],
        HeaderValue::TextList(list) => list.iter().map(|t| t.as_ref()).collect(),
        _ => Vec::new(),
    };
    texts
        .into_iter()
        .flat_map(|text| text.split_whitespace())
        .map(|id| id.trim_matches(['<', '>', ',']))
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .collect()
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
    fn inline_pictures_are_not_attachments() {
        let raw = b"From: a@example.org\r
Subject: Newsletter\r
Content-Type: multipart/related; boundary=\"r\"\r
\r
--r\r
Content-Type: text/html\r
\r
<img src=\"cid:logo\">\r
--r\r
Content-Type: image/png; name=\"logo.png\"\r
Content-Disposition: inline; filename=\"logo.png\"\r
Content-ID: <logo>\r
\r
PNG\r
--r--\r
";
        assert!(!parse_message(raw).unwrap().has_attachments);
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
    fn reads_thread_links_and_category() {
        let raw = b"Message-ID: <3@example.org>\r
In-Reply-To: <2@example.org>\r
References: <1@example.org>\r
 <2@example.org>\r
From: Ada <ada@example.org>\r
Subject: Re: Plan\r
\r
Sounds good.\r
";
        let parsed = parse_message(raw).unwrap();
        assert_eq!(parsed.in_reply_to.as_deref(), Some("2@example.org"));
        assert_eq!(parsed.reference_strs(), ["1@example.org", "2@example.org"]);
        assert_eq!(parsed.category, MailCategory::Primary);
        let links = parse_links(raw).unwrap();
        assert_eq!(links.in_reply_to, parsed.in_reply_to);
        assert_eq!(links.references, parsed.references);

        let promo = b"From: =?utf-8?q?Caf=C3=A9?= <news@cafe.example>\r
Subject: =?utf-8?q?50=25_off_today?=\r
List-Unsubscribe: <https://cafe.example/u>\r
X-MC-User: abc\r
\r
Deals.\r
";
        assert_eq!(
            parse_message(promo).unwrap().category,
            MailCategory::Promotions
        );
        assert_eq!(
            parse_links(promo).unwrap().category,
            MailCategory::Promotions
        );
        let social = b"From: Facebook <notification@facebookmail.com>\r\nSubject: Hi\r\n\r\n";
        assert_eq!(parse_links(social).unwrap().category, MailCategory::Social);
        assert_eq!(parse_links(b""), None);
    }

    #[test]
    fn rejects_non_mail() {
        assert_eq!(parse_message(b""), None);
    }
}
