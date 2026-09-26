// SPDX-License-Identifier: GPL-3.0-or-later

//! The plain-text view of a raw message.

use mail_parser::{MessageParser, MimeHeaders, PartType};

/// At most this much body text is shown. Longer bodies are nearly always
/// logs or pasted data, and laying out megabytes of text would stall the UI.
pub const MAX_BODY_BYTES: usize = 256 * 1024;

/// One mailbox of an address header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub name: Option<String>,
    pub email: String,
}

impl Address {
    /// The display name, or the address when there is none.
    pub fn label(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.email)
    }
}

/// An attachment as listed under the message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    pub name: String,
    /// Decoded size in bytes.
    pub size: u64,
    /// The `Content-ID`, without angle brackets, for images an HTML body
    /// shows inline.
    pub content_id: Option<String>,
}

/// What the reading pane shows of a message.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MessageView {
    pub subject: String,
    pub from: Vec<Address>,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    /// Unix seconds, from the `Date` header.
    pub date: Option<i64>,
    /// The text parts, HTML converted to text, joined by blank lines.
    pub body: String,
    /// The body was cut at [`MAX_BODY_BYTES`].
    pub truncated: bool,
    /// The message has no plain-text part; `body` was converted from HTML.
    pub from_html: bool,
    pub attachments: Vec<Attachment>,
    /// The `Message-ID`, without angle brackets.
    pub message_id: Option<String>,
    /// The `References`, oldest first, without angle brackets.
    pub references: Vec<String>,
}

/// Parses `raw` into its plain-text view. A message that cannot be parsed
/// at all gives an empty view.
pub fn message_view(raw: &[u8]) -> MessageView {
    let Some(message) = MessageParser::default().parse(raw) else {
        return MessageView::default();
    };
    let addresses = |header: Option<&mail_parser::Address<'_>>| -> Vec<Address> {
        header
            .into_iter()
            .flat_map(|a| a.iter())
            .filter_map(|addr| {
                Some(Address {
                    name: addr
                        .name
                        .as_deref()
                        .map(str::trim)
                        .filter(|n| !n.is_empty())
                        .map(str::to_owned),
                    email: addr.address.as_deref()?.trim().to_owned(),
                })
            })
            .collect()
    };

    let mut body = String::new();
    let mut from_html = false;
    for index in 0..message.text_body_count() {
        if body.len() >= MAX_BODY_BYTES {
            break;
        }
        let Some(text) = message.body_text(index) else {
            continue;
        };
        let text = text.trim_end();
        if text.is_empty() {
            continue;
        }
        if message
            .text_part(index as u32)
            .is_some_and(|part| matches!(part.body, PartType::Html(_)))
        {
            from_html = true;
        }
        if !body.is_empty() {
            body.push_str("\n\n");
        }
        body.push_str(text);
    }
    let truncated = truncate_at_char(&mut body, MAX_BODY_BYTES);

    let attachments = message
        .attachments()
        .map(|part| Attachment {
            name: part
                .attachment_name()
                .map(str::to_owned)
                .or_else(|| {
                    part.message()
                        .and_then(|m| m.subject())
                        .map(|s| format!("{s}.eml"))
                })
                .unwrap_or_else(|| "Unnamed attachment".to_owned()),
            size: part.body.len() as u64,
            content_id: part
                .content_id()
                .map(|id| id.trim_matches(['<', '>', ' ']).to_owned()),
        })
        .collect();

    MessageView {
        subject: message.subject().unwrap_or_default().trim().to_owned(),
        from: addresses(message.from()),
        to: addresses(message.to()),
        cc: addresses(message.cc()),
        date: message.date().map(|d| d.to_timestamp()),
        body,
        truncated,
        from_html,
        attachments,
        message_id: message.message_id().map(str::to_owned),
        references: message
            .references()
            .as_text_list()
            .map(|ids| ids.iter().map(|id| id.to_string()).collect())
            .unwrap_or_default(),
    }
}

/// Cuts `text` to at most `max` bytes at a character boundary. Returns
/// whether anything was cut.
fn truncate_at_char(text: &mut String, max: usize) -> bool {
    if text.len() <= max {
        return false;
    }
    let mut end = max;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    text.truncate(end);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_message() {
        let raw = b"From: \"Ada Lovelace\" <ada@example.org>\r\n\
To: bob@example.net, Carol <carol@example.com>\r\n\
Cc: team: dan@example.com;\r\n\
Subject:  Engine notes \r\n\
Date: Tue, 1 Sep 2026 10:00:00 +0200\r\n\
\r\n\
Hello Bob,\r\n\r\nthe engine works.\r\n\r\n";
        let view = message_view(raw);
        assert_eq!(view.subject, "Engine notes");
        assert_eq!(
            view.from,
            [Address {
                name: Some("Ada Lovelace".into()),
                email: "ada@example.org".into()
            }]
        );
        assert_eq!(view.from[0].label(), "Ada Lovelace");
        let to: Vec<_> = view.to.iter().map(Address::label).collect();
        assert_eq!(to, ["bob@example.net", "Carol"]);
        assert_eq!(view.cc[0].email, "dan@example.com");
        assert_eq!(view.date, Some(1_788_249_600));
        assert_eq!(view.body, "Hello Bob,\r\n\r\nthe engine works.");
        assert!(!view.from_html && !view.truncated);
        assert!(view.attachments.is_empty());
    }

    #[test]
    fn html_only_message_becomes_text() {
        let raw = b"From: a@example.org\r\nSubject: News\r\n\
Content-Type: text/html; charset=utf-8\r\n\r\n\
<html><body><p>Big <b>news</b></p><p>Second</p></body></html>\r\n";
        let view = message_view(raw);
        assert!(view.from_html);
        assert!(view.body.contains("Big news"), "{:?}", view.body);
        assert!(view.body.contains("Second"));
        assert!(!view.body.contains('<'));
    }

    #[test]
    fn attachments_are_listed_not_shown() {
        let raw = b"From: a@example.org\r\nSubject: Report\r\n\
Content-Type: multipart/mixed; boundary=\"b\"\r\n\r\n\
--b\r\nContent-Type: text/plain\r\n\r\nSee attached.\r\n\
--b\r\nContent-Type: application/pdf\r\n\
Content-Disposition: attachment; filename=\"q3.pdf\"\r\n\
Content-Transfer-Encoding: base64\r\n\r\naGVsbG8=\r\n\
--b--\r\n";
        let view = message_view(raw);
        assert_eq!(view.body, "See attached.");
        assert_eq!(
            view.attachments,
            [Attachment {
                name: "q3.pdf".into(),
                size: 5,
                content_id: None,
            }]
        );
    }

    #[test]
    fn long_bodies_are_cut_at_a_character() {
        let mut raw = b"Subject: x\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n".to_vec();
        raw.extend("é".repeat(MAX_BODY_BYTES).as_bytes());
        let view = message_view(&raw);
        assert!(view.truncated);
        assert!(view.body.len() <= MAX_BODY_BYTES);
        assert!(view.body.chars().all(|c| c == 'é'));
    }

    #[test]
    fn garbage_gives_an_empty_view() {
        assert_eq!(message_view(b""), MessageView::default());
    }
}
