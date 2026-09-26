// SPDX-License-Identifier: GPL-3.0-or-later

//! Turns a stored message into an index document: parse → text → fields.

use katna_store::{MessageFlags, ParticipantRole, StoredMessage};
use mail_parser::{MessageParser, MimeHeaders};
use tantivy::TantivyDocument;

use crate::schema::Fields;

/// At most this much body text is indexed per message. Longer bodies are
/// nearly always logs, reports or pasted data; their start is enough to find
/// them, and it keeps one giant message from dominating index size.
pub const MAX_BODY_BYTES: usize = 256 * 1024;

/// The searchable text of a raw message.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MessageText {
    /// Text of all text parts (HTML converted to text), up to [`MAX_BODY_BYTES`].
    pub body: String,
    pub attachment_names: Vec<String>,
}

/// Extracts the searchable text of `raw`.
pub fn message_text(raw: &[u8]) -> MessageText {
    let Some(message) = MessageParser::default().parse(raw) else {
        return MessageText::default();
    };
    let mut body = String::new();
    for index in 0..message.text_body.len() {
        if body.len() >= MAX_BODY_BYTES {
            break;
        }
        if let Some(text) = message.body_text(index) {
            if !body.is_empty() {
                body.push('\n');
            }
            // Encrypted inline PGP is noise to search, and decrypted text
            // is never indexed (docs/ARCHITECTURE.md §19.1).
            body.push_str(&katna_crypto::without_armor(&text));
        }
    }
    truncate_at_char(&mut body, MAX_BODY_BYTES);
    let attachment_names = message
        .attachments()
        .filter_map(|part| part.attachment_name())
        .map(str::to_owned)
        .collect();
    MessageText {
        body,
        attachment_names,
    }
}

fn truncate_at_char(text: &mut String, max: usize) {
    if text.len() > max {
        let mut end = max;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
    }
}

const FLAG_NAMES: [(MessageFlags, &str); 7] = [
    (MessageFlags::SEEN, "seen"),
    (MessageFlags::ANSWERED, "answered"),
    (MessageFlags::FLAGGED, "flagged"),
    (MessageFlags::DRAFT, "draft"),
    (MessageFlags::DELETED, "deleted"),
    (MessageFlags::FORWARDED, "forwarded"),
    (MessageFlags::IMPORTANT, "important"),
];

/// Index term of a system flag in [`Fields::flag`].
pub fn flag_term(flag: MessageFlags) -> Option<&'static str> {
    FLAG_NAMES
        .iter()
        .find(|(f, _)| *f == flag)
        .map(|(_, name)| *name)
}

/// Builds the index document of `message`, whose text is `text` (`None` when
/// the raw message is not stored).
pub fn build(
    fields: &Fields,
    message: &StoredMessage,
    text: Option<&MessageText>,
) -> TantivyDocument {
    let mut doc = TantivyDocument::default();
    doc.add_u64(fields.msg_id, message.id.0 as u64);
    doc.add_u64(fields.account, message.account.0 as u64);
    if let Some(date) = message.date {
        doc.add_i64(fields.date, date);
    }
    doc.add_u64(fields.size, message.size);
    doc.add_u64(fields.flags, u64::from(message.flags.bits()));
    if !message.subject.is_empty() {
        doc.add_text(fields.subject, &message.subject);
        doc.add_text(fields.subject_stem, &message.subject);
    }

    let mut domains: Vec<&str> = Vec::new();
    for participant in &message.participants {
        let field = match participant.role {
            ParticipantRole::From | ParticipantRole::Sender => fields.from,
            ParticipantRole::To => fields.to,
            ParticipantRole::Cc => fields.cc,
            ParticipantRole::Bcc => fields.bcc,
            ParticipantRole::ReplyTo => continue,
        };
        // Name and address are separate values, so a phrase never spans them.
        if let Some(name) = &participant.display_name {
            doc.add_text(field, name);
        }
        doc.add_text(field, &participant.email_norm);
        // enron.com, and for mail.enron.com also enron.com.
        let mut domain = participant.domain.as_str();
        while domain.contains('.') {
            if !domains.contains(&domain) {
                domains.push(domain);
            }
            domain = domain.split_once('.').map_or("", |(_, rest)| rest);
        }
    }
    for domain in domains {
        doc.add_text(fields.domain, domain);
    }

    let mut folders: Vec<String> = Vec::new();
    for location in &message.locations {
        let path = location.path.to_lowercase();
        let mut names: Vec<String> = path.split('/').map(str::to_owned).collect();
        names.push(path.clone());
        if let Some(role) = &location.role {
            names.push(role.to_lowercase());
        }
        for name in names {
            if !name.is_empty() && !folders.contains(&name) {
                folders.push(name);
            }
        }
    }
    for folder in folders {
        doc.add_text(fields.folder, folder);
    }
    for keyword in &message.keywords {
        doc.add_text(fields.label, keyword.to_lowercase());
    }
    for (flag, name) in FLAG_NAMES {
        if message.flags.contains(flag) {
            doc.add_text(fields.flag, name);
        }
    }
    if let Some(list) = &message.list_id {
        doc.add_text(fields.list, list);
    }

    let has_attachment =
        message.has_attachments || text.is_some_and(|t| !t.attachment_names.is_empty());
    if has_attachment {
        doc.add_text(fields.has, "attachment");
    }
    if let Some(text) = text {
        for name in &text.attachment_names {
            doc.add_text(fields.attachment, name);
        }
        if !text.body.is_empty() {
            doc.add_text(fields.body, &text.body);
            doc.add_text(fields.body_stem, &text.body);
        }
    }
    doc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_text_and_attachment_names() {
        let raw = b"From: a@example.org\r
Subject: notes\r
Content-Type: multipart/mixed; boundary=\"b\"\r
\r
--b\r
Content-Type: text/html\r
\r
<p>Quarterly <b>budget</b></p>\r
--b\r
Content-Type: application/pdf\r
Content-Disposition: attachment; filename=\"Q3 budget.pdf\"\r
\r
%PDF\r
--b--\r
";
        let text = message_text(raw);
        assert!(text.body.contains("Quarterly budget"), "{:?}", text.body);
        assert!(!text.body.contains("<b>"));
        assert_eq!(text.attachment_names, ["Q3 budget.pdf"]);
    }

    #[test]
    fn caps_body_size() {
        let mut raw = b"Subject: log\r\n\r\n".to_vec();
        raw.extend("é".repeat(MAX_BODY_BYTES).as_bytes());
        let text = message_text(&raw);
        assert!(text.body.len() <= MAX_BODY_BYTES);
        assert!(text.body.len() > MAX_BODY_BYTES - 4);
    }

    #[test]
    fn no_text_for_garbage() {
        assert_eq!(message_text(b""), MessageText::default());
    }
}
