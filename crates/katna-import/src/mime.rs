// SPDX-License-Identifier: GPL-3.0-or-later

//! Which MIME parts are attachments, and their file names. Shared by the
//! parser of whole messages ([`crate::parse_message`]) and by IMAP sync,
//! which only sees a message's structure (`BODYSTRUCTURE`), so that a
//! message keeps its paperclip when its body is downloaded later.

use mail_parser::{Message, MessageParser, MimeHeaders, PartType};

/// One leaf part of a message, as far as the attachment rule needs it.
#[derive(Clone, Copy, Debug, Default)]
pub struct PartInfo<'a> {
    /// `type/subtype`, any case.
    pub mime: &'a str,
    /// `Content-Disposition` type (`attachment`, `inline`), if any.
    pub disposition: Option<&'a str>,
    /// The part has a `Content-ID`: the HTML body can show it (`cid:`).
    pub content_id: bool,
    /// The part has a file name.
    pub filename: bool,
    /// The part is one of the versions of a `multipart/alternative`.
    pub alternative: bool,
}

/// Text types that are a version of the message itself, not a file:
/// Gmail's AMP body, Apple Watch text and the old rich text formats.
pub const BODY_TEXT: [&str; 6] = [
    "text/plain",
    "text/html",
    "text/x-amp-html",
    "text/watch-html",
    "text/enriched",
    "text/richtext",
];

/// Whether the part is shown as an attachment: anything marked
/// `attachment`; otherwise anything but the text of the message and
/// pictures the HTML body shows inline.
pub fn is_attachment(part: &PartInfo<'_>) -> bool {
    // Signatures and the PGP/MIME version part belong to the message's
    // protection, which the reading view shows in its own banner.
    const PROTECTION_PARTS: [&str; 4] = [
        "application/pgp-signature",
        "application/pkcs7-signature",
        "application/x-pkcs7-signature",
        "application/pgp-encrypted",
    ];
    if PROTECTION_PARTS
        .iter()
        .any(|t| part.mime.eq_ignore_ascii_case(t))
    {
        return false;
    }
    if part
        .disposition
        .is_some_and(|d| d.eq_ignore_ascii_case("attachment"))
    {
        return true;
    }
    if part.filename {
        return !part.content_id;
    }
    // A version of the body, a picture the body shows, or the body text.
    if part.alternative || part.content_id {
        return false;
    }
    !BODY_TEXT.iter().any(|t| part.mime.eq_ignore_ascii_case(t))
}

/// One attachment of a whole message.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attachment {
    /// IMAP body section (`1`, `2.1`), as `BODYSTRUCTURE` numbers it.
    pub part: String,
    /// `type/subtype`, lower case.
    pub mime: String,
    pub filename: Option<String>,
    /// Decoded size in bytes.
    pub size: u64,
}

/// The attachments of a parsed message, with the body sections IMAP gives
/// them, so a list read from the body matches one read from its
/// `BODYSTRUCTURE`. Parts of an attached message stay inside it.
pub fn attachments(message: &Message<'_>) -> Vec<Attachment> {
    let mut out = Vec::new();
    collect(message, 0, "", false, &mut out);
    out
}

fn collect(
    message: &Message<'_>,
    index: usize,
    section: &str,
    alternative: bool,
    out: &mut Vec<Attachment>,
) {
    let Some(part) = message.parts.get(index) else {
        return;
    };
    let content_type = part.content_type();
    if let PartType::Multipart(children) = &part.body {
        let alternative = content_type
            .and_then(|t| t.subtype())
            .is_some_and(|s| s.eq_ignore_ascii_case("alternative"));
        for (i, &child) in children.iter().enumerate() {
            let section = match section {
                "" => (i + 1).to_string(),
                _ => format!("{section}.{}", i + 1),
            };
            collect(message, child as usize, &section, alternative, out);
        }
        return;
    }
    let mime = match (&part.body, content_type) {
        (PartType::Message(_), _) => "message/rfc822".to_owned(),
        (_, Some(t)) => match t.subtype() {
            Some(sub) => format!("{}/{sub}", t.ctype()),
            None => t.ctype().to_owned(),
        },
        (_, None) => "text/plain".to_owned(),
    }
    .to_ascii_lowercase();
    let filename = part
        .attachment_name()
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map(str::to_owned);
    let info = PartInfo {
        mime: &mime,
        disposition: part.content_disposition().map(|d| d.ctype()),
        content_id: part.content_id().is_some(),
        filename: filename.is_some(),
        alternative,
    };
    if !is_attachment(&info) {
        return;
    }
    out.push(Attachment {
        part: match section {
            "" => "1".to_owned(),
            _ => section.to_owned(),
        },
        mime,
        filename,
        size: part.len() as u64,
    });
}

/// The file name of a part from its `Content-Disposition` and
/// `Content-Type` parameters (`filename`, then `name`), decoded: RFC 2231
/// (`filename*=utf-8''…`, continuations) and RFC 2047 encoded words.
pub fn part_filename(
    disposition: &[(&str, &str)],
    content_type: &[(&str, &str)],
) -> Option<String> {
    let mut header = String::from("Content-Type: application/octet-stream");
    push_params(&mut header, content_type);
    header.push_str("\r\nContent-Disposition: attachment");
    push_params(&mut header, disposition);
    header.push_str("\r\n\r\n");
    let message = MessageParser::default().parse_headers(header.as_bytes())?;
    let name = message.attachment_name()?.trim();
    (!name.is_empty()).then(|| name.to_owned())
}

fn push_params(header: &mut String, params: &[(&str, &str)]) {
    for (name, value) in params {
        // A parameter name is a token; anything else would break the line.
        if name.is_empty() || !name.bytes().all(is_token) {
            continue;
        }
        header.push_str("; ");
        header.push_str(name);
        header.push('=');
        if !value.is_empty() && value.bytes().all(is_token) {
            header.push_str(value);
        } else {
            header.push('"');
            for c in value.chars().filter(|c| !matches!(c, '\r' | '\n')) {
                if matches!(c, '"' | '\\') {
                    header.push('\\');
                }
                header.push(c);
            }
            header.push('"');
        }
    }
}

/// RFC 2045 token characters.
fn is_token(b: u8) -> bool {
    b.is_ascii_graphic() && !b"()<>@,;:\\\"/[]?=".contains(&b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(mime: &str) -> PartInfo<'_> {
        PartInfo {
            mime,
            ..PartInfo::default()
        }
    }

    #[test]
    fn text_is_the_message_and_the_rest_are_attachments() {
        assert!(!is_attachment(&part("text/plain")));
        assert!(!is_attachment(&part("TEXT/HTML")));
        assert!(is_attachment(&part("application/pdf")));
        assert!(is_attachment(&part("text/calendar")));
        assert!(is_attachment(&part("message/rfc822")));
        assert!(is_attachment(&PartInfo {
            filename: true,
            ..part("text/plain")
        }));
        assert!(is_attachment(&PartInfo {
            disposition: Some("Attachment"),
            ..part("text/plain")
        }));
    }

    #[test]
    fn versions_of_the_body_are_not_attachments() {
        // Gmail's AMP body and Apple Watch text, anywhere.
        assert!(!is_attachment(&part("text/x-amp-html")));
        assert!(!is_attachment(&part("text/watch-html")));
        // Any version of a multipart/alternative, unless sent as a file.
        let version = PartInfo {
            alternative: true,
            ..part("text/calendar")
        };
        assert!(!is_attachment(&version));
        assert!(is_attachment(&PartInfo {
            filename: true,
            ..version
        }));
        assert!(is_attachment(&PartInfo {
            disposition: Some("attachment"),
            ..version
        }));
    }

    #[test]
    fn lists_a_whole_message_as_imap_numbers_it() {
        let raw = concat!(
            "From: a@example.org\r\n",
            "Subject: Files\r\n",
            "Content-Type: multipart/mixed; boundary=\"m\"\r\n",
            "\r\n",
            "--m\r\n",
            "Content-Type: multipart/alternative; boundary=\"a\"\r\n",
            "\r\n",
            "--a\r\n",
            "Content-Type: text/plain\r\n\r\nHi\r\n",
            "--a\r\n",
            "Content-Type: multipart/related; boundary=\"r\"\r\n",
            "\r\n",
            "--r\r\n",
            "Content-Type: text/html\r\n\r\n<img src=\"cid:logo\">\r\n",
            "--r\r\n",
            "Content-Type: image/png; name=\"logo.png\"\r\n",
            "Content-ID: <logo>\r\n",
            "Content-Transfer-Encoding: base64\r\n\r\niVBORw0KGgo=\r\n",
            "--r--\r\n",
            "--a\r\n",
            "Content-Type: text/x-amp-html\r\n\r\n<html amp4email></html>\r\n",
            "--a--\r\n",
            "--m\r\n",
            "Content-Type: application/pdf; name=\"rates.pdf\"\r\n",
            "Content-Disposition: attachment; filename*=utf-8''%E2%82%AC%20rates.pdf\r\n",
            "Content-Transfer-Encoding: base64\r\n\r\nJVBERi0xLjQK\r\n",
            "--m\r\n",
            "Content-Type: message/rfc822\r\n\r\n",
            "Subject: Old\r\n\r\nForwarded\r\n",
            "--m--\r\n",
        );
        let message = MessageParser::default().parse(raw.as_bytes()).unwrap();
        let got: Vec<_> = attachments(&message)
            .into_iter()
            .map(|a| (a.part, a.mime, a.filename, a.size))
            .collect();
        assert_eq!(
            got,
            [
                (
                    "2".to_owned(),
                    "application/pdf".to_owned(),
                    Some("€ rates.pdf".to_owned()),
                    9
                ),
                ("3".to_owned(), "message/rfc822".to_owned(), None, 25),
            ]
        );

        let single = "Content-Type: application/pdf\r\n\r\n%PDF\r\n";
        let message = MessageParser::default().parse(single.as_bytes()).unwrap();
        let parts: Vec<_> = attachments(&message).into_iter().map(|a| a.part).collect();
        assert_eq!(parts, ["1"], "a message that is one file");
    }

    #[test]
    fn inline_pictures_are_not_attachments() {
        let logo = PartInfo {
            content_id: true,
            filename: true,
            disposition: Some("inline"),
            ..part("image/png")
        };
        assert!(!is_attachment(&logo));
        assert!(is_attachment(&PartInfo {
            disposition: Some("attachment"),
            ..logo
        }));
    }

    #[test]
    fn decodes_file_names() {
        assert_eq!(
            part_filename(&[("filename", "notes.pdf")], &[]).as_deref(),
            Some("notes.pdf")
        );
        assert_eq!(
            part_filename(&[], &[("name", "two words.txt")]).as_deref(),
            Some("two words.txt")
        );
        assert_eq!(
            part_filename(&[("filename*", "utf-8''%E2%82%AC%20rates.pdf")], &[]).as_deref(),
            Some("€ rates.pdf")
        );
        assert_eq!(
            part_filename(
                &[
                    ("filename*0*", "utf-8''Gr%C3%BC"),
                    ("filename*1", "sse.txt")
                ],
                &[]
            )
            .as_deref(),
            Some("Grüsse.txt")
        );
        assert_eq!(
            part_filename(&[], &[("name", "=?UTF-8?B?w6nDqS5wZGY=?=")]).as_deref(),
            Some("éé.pdf")
        );
        assert_eq!(
            part_filename(&[("filename", "say \"hi\".txt")], &[]).as_deref(),
            Some("say \"hi\".txt")
        );
        assert_eq!(part_filename(&[("filename", "  ")], &[]), None);
        assert_eq!(part_filename(&[], &[("charset", "utf-8")]), None);
    }
}
