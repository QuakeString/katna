// SPDX-License-Identifier: GPL-3.0-or-later

//! The plain-text drafts an assistant saves: written here, saved by the
//! daemon (`SaveDraft`), which adds `Date` and `Message-ID`.

use katna_core::mime::{Mailbox, address, encode_words, quoted_printable};
use mail_parser::{HeaderValue, MessageParser};

/// A draft to save.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Draft {
    pub from: Option<Mailbox>,
    pub to: Vec<Mailbox>,
    pub cc: Vec<Mailbox>,
    pub bcc: Vec<Mailbox>,
    pub subject: String,
    pub body: String,
    /// The message answered: its `Message-ID`, without angle brackets.
    pub in_reply_to: Option<String>,
    /// Its `References` and its `Message-ID`, oldest first.
    pub references: Vec<String>,
}

/// What a draft answering a message needs of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Original {
    pub message_id: Option<String>,
    pub references: Vec<String>,
    pub subject: String,
    /// `Reply-To`, else the sender.
    pub reply_to: Vec<Mailbox>,
}

/// Reads the message `raw` for a reply.
pub fn original(raw: &[u8]) -> Option<Original> {
    let message = MessageParser::default().parse_headers(raw)?;
    let mailboxes = |value: Option<&mail_parser::Address>| -> Vec<Mailbox> {
        value
            .map(|list| {
                list.iter()
                    .filter_map(|a| {
                        Some(Mailbox {
                            name: a.name().map(str::to_owned),
                            email: a.address()?.to_owned(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let mut reply_to = mailboxes(message.reply_to());
    if reply_to.is_empty() {
        reply_to = mailboxes(message.from());
    }
    let references = match message.references() {
        HeaderValue::Text(id) => vec![id.to_string()],
        HeaderValue::TextList(ids) => ids.iter().map(|id| id.to_string()).collect(),
        _ => Vec::new(),
    };
    Some(Original {
        message_id: message.message_id().map(str::to_owned),
        references,
        subject: message.subject().unwrap_or_default().to_owned(),
        reply_to,
    })
}

impl Draft {
    /// Makes this draft answer `original`: in its conversation, and to its
    /// sender and with its subject unless they were given.
    pub fn answer(&mut self, original: &Original) {
        if let Some(id) = &original.message_id {
            self.references = original
                .references
                .iter()
                .filter(|r| *r != id)
                .chain(std::iter::once(id))
                .cloned()
                .collect();
            self.in_reply_to = Some(id.clone());
        }
        if self.to.is_empty() {
            self.to = original.reply_to.clone();
        }
        if self.subject.trim().is_empty() {
            self.subject = re(&original.subject);
        }
    }

    /// The draft as a message.
    pub fn build(&self) -> Vec<u8> {
        let mut head = String::new();
        if let Some(from) = &self.from {
            head.push_str(&format!("From: {}\r\n", address(from)));
        }
        for (name, list) in [("To", &self.to), ("Cc", &self.cc), ("Bcc", &self.bcc)] {
            if !list.is_empty() {
                let list: Vec<String> = list.iter().map(address).collect();
                head.push_str(&format!("{name}: {}\r\n", list.join(",\r\n ")));
            }
        }
        head.push_str(&format!(
            "Subject: {}\r\n",
            encode_words(&one_line(&self.subject))
        ));
        if let Some(id) = &self.in_reply_to {
            head.push_str(&format!("In-Reply-To: <{id}>\r\n"));
            let references: Vec<String> =
                self.references.iter().map(|r| format!("<{r}>")).collect();
            head.push_str(&format!("References: {}\r\n", references.join("\r\n ")));
        }
        head.push_str("MIME-Version: 1.0\r\n");
        head.push_str("Content-Type: text/plain; charset=utf-8\r\n");
        head.push_str("Content-Transfer-Encoding: quoted-printable\r\n\r\n");
        let mut raw = head.into_bytes();
        let mut body = self.body.replace("\r\n", "\n");
        if !body.ends_with('\n') {
            body.push('\n');
        }
        raw.extend_from_slice(quoted_printable(&body).as_bytes());
        raw
    }
}

/// The addresses of `text`, separated by commas: `Name <address>` or the
/// address alone. An entry that is not an address is an error naming it.
pub fn addresses(text: &str) -> Result<Vec<Mailbox>, String> {
    let text = one_line(text);
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let header = format!("To: {text}\r\n\r\n");
    let parsed = MessageParser::default()
        .parse_headers(header.as_bytes())
        .ok_or_else(|| format!("{text:?} is not a list of addresses"))?;
    let mut out = Vec::new();
    if let Some(list) = parsed.to() {
        for entry in list.iter() {
            let email = entry.address().unwrap_or_default().trim();
            let valid = email.split_once('@').is_some_and(|(local, domain)| {
                !local.is_empty() && (domain.contains('.') || domain == "localhost")
            });
            if !valid {
                let shown = entry.name().unwrap_or(email);
                return Err(format!("{shown:?} is not an email address"));
            }
            out.push(Mailbox {
                name: entry
                    .name()
                    .map(str::trim)
                    .filter(|n| !n.is_empty())
                    .map(str::to_owned),
                email: email.to_owned(),
            });
        }
    }
    if out.is_empty() {
        return Err(format!("{text:?} has no email address"));
    }
    Ok(out)
}

/// `subject` with one `Re: ` in front.
fn re(subject: &str) -> String {
    let subject = subject.trim();
    if katna_core::subject::without_reply_prefixes(subject).len() < subject.len() {
        subject.to_owned()
    } else {
        format!("Re: {subject}")
    }
}

/// `text` without line breaks, which would end a header.
fn one_line(text: &str) -> String {
    text.split(['\r', '\n'])
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_are_read_and_checked() {
        let list = addresses("Alex Lee <alex@example.org>, kay@example.com").unwrap();
        assert_eq!(
            list,
            [
                Mailbox {
                    name: Some("Alex Lee".into()),
                    email: "alex@example.org".into()
                },
                Mailbox {
                    name: None,
                    email: "kay@example.com".into()
                },
            ]
        );
        assert_eq!(addresses("  ").unwrap(), []);
        assert!(addresses("not an address").is_err());
        assert!(addresses("a@example.org, Bo").is_err());
    }

    #[test]
    fn a_reply_joins_the_conversation() {
        let raw = b"Message-ID: <q3@example.org>\r\nReferences: <q1@example.org> <q2@example.org>\r\n\
From: Alex Lee <alex@example.org>\r\nReply-To: team@example.org\r\nSubject: Q3 planning\r\n\r\nHi\r\n";
        let original = original(raw).unwrap();
        let mut draft = Draft {
            from: Some(Mailbox {
                name: Some("Kay Mann".into()),
                email: "kay@example.com".into(),
            }),
            body: "Works for me.\nKay".into(),
            ..Draft::default()
        };
        draft.answer(&original);
        let text = String::from_utf8(draft.build()).unwrap();
        assert!(
            text.contains("From: Kay Mann <kay@example.com>\r\n"),
            "{text}"
        );
        assert!(text.contains("To: team@example.org\r\n"), "{text}");
        assert!(text.contains("Subject: Re: Q3 planning\r\n"));
        assert!(text.contains("In-Reply-To: <q3@example.org>\r\n"));
        assert!(text.contains(
            "References: <q1@example.org>\r\n <q2@example.org>\r\n <q3@example.org>\r\n"
        ));
        let parsed = MessageParser::default().parse(text.as_bytes()).unwrap();
        assert_eq!(parsed.body_text(0).unwrap(), "Works for me.\r\nKay\r\n");

        // A subject already answered keeps its prefix.
        assert_eq!(re("RE: Hi"), "RE: Hi");
    }

    #[test]
    fn headers_cannot_be_broken_into() {
        let draft = Draft {
            to: addresses("a@example.org\r\nBcc: evil@example.org").unwrap_or_default(),
            subject: "Hello\r\nBcc: evil@example.org".into(),
            body: "Hi".into(),
            ..Draft::default()
        };
        let text = String::from_utf8(draft.build()).unwrap();
        assert!(!text.contains("\r\nBcc:"), "{text}");
        let parsed = MessageParser::default().parse(text.as_bytes()).unwrap();
        assert_eq!(parsed.subject(), Some("Hello Bcc: evil@example.org"));
        assert!(parsed.bcc().is_none());
    }

    #[test]
    fn unicode_subjects_and_bodies_survive() {
        let draft = Draft {
            to: addresses("Zoë <z@example.org>").unwrap(),
            subject: "Café ☕".into(),
            body: "Ünïcödé\n".into(),
            ..Draft::default()
        };
        let raw = draft.build();
        let parsed = MessageParser::default().parse(&raw[..]).unwrap();
        assert_eq!(parsed.subject(), Some("Café ☕"));
        assert_eq!(parsed.body_text(0).unwrap(), "Ünïcödé\r\n");
        let to = parsed.to().unwrap().first().unwrap();
        assert_eq!(to.name(), Some("Zoë"));
    }
}
