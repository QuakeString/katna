// SPDX-License-Identifier: GPL-3.0-or-later

//! Which MIME parts are attachments, and their file names. Shared by the
//! parser of whole messages ([`crate::parse_message`]) and by IMAP sync,
//! which only sees a message's structure (`BODYSTRUCTURE`), so that a
//! message keeps its paperclip when its body is downloaded later.

use mail_parser::{MessageParser, MimeHeaders};

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
}

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
    if part.content_id {
        return false;
    }
    let text = ["text/plain", "text/html"]
        .iter()
        .any(|t| part.mime.eq_ignore_ascii_case(t));
    !text || part.filename
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
