// SPDX-License-Identifier: GPL-3.0-or-later

//! Read receipts (message disposition notifications, RFC 8098): the
//! reports a recipient's app sends back when a message asked for one with
//! `Disposition-Notification-To`. The conversation shows them as opens of
//! the message they answer.

use mail_parser::{MessageParser, MimeHeaders};

/// Larger mail is not a read receipt (a report is a few hundred bytes plus
/// at most the original's headers).
pub const MAX_SIZE: u64 = 64 * 1024;

/// What a read receipt says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    /// Who read it: the report's `Final-Recipient` address, else its sender.
    pub by: String,
    /// The sender's name, when the sender is the one who read it.
    pub name: Option<String>,
    /// The `Message-ID` it answers, without angle brackets.
    pub original: Option<String>,
    /// The message was shown ("displayed"); otherwise it was deleted or
    /// the like without being read.
    pub displayed: bool,
}

impl Receipt {
    /// Who read it, by name when known.
    pub fn who(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.by.clone())
    }
}

/// `raw` as a read receipt, if it is one.
pub fn parse(raw: &[u8]) -> Option<Receipt> {
    let message = MessageParser::default().parse(raw)?;
    let kind = message.content_type()?;
    let is_report = kind.ctype().eq_ignore_ascii_case("multipart")
        && kind
            .subtype()
            .is_some_and(|s| s.eq_ignore_ascii_case("report"))
        && kind
            .attribute("report-type")
            .is_some_and(|t| t.eq_ignore_ascii_case("disposition-notification"));
    if !is_report {
        return None;
    }
    let report = message.parts.iter().find(|part| {
        part.content_type().is_some_and(|t| {
            t.ctype().eq_ignore_ascii_case("message")
                && t.subtype()
                    .is_some_and(|s| s.eq_ignore_ascii_case("disposition-notification"))
        })
    })?;
    let text = String::from_utf8_lossy(report.contents());
    let mut by = None;
    let mut original = None;
    let mut displayed = false;
    for (name, value) in fields(&text) {
        match name.to_ascii_lowercase().as_str() {
            // "rfc822; bea@example.org"
            "final-recipient" => {
                by = value
                    .split_once(';')
                    .map_or(value.as_str(), |(_, address)| address)
                    .trim()
                    .to_owned()
                    .into();
            }
            "original-message-id" => {
                original = Some(
                    value
                        .trim()
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .to_owned(),
                );
            }
            // "manual-action/MDN-sent-manually; displayed"
            "disposition" => {
                displayed = value
                    .rsplit_once(';')
                    .is_some_and(|(_, kind)| kind.trim().eq_ignore_ascii_case("displayed"));
            }
            _ => {}
        }
    }
    let from = message.from().and_then(|from| from.first());
    let by = by
        .filter(|b| !b.is_empty())
        .or_else(|| from.and_then(|a| a.address()).map(str::to_owned))?;
    let name = from
        .filter(|a| a.address().is_some_and(|a| a.eq_ignore_ascii_case(&by)))
        .and_then(|a| a.name())
        .map(|n| n.trim().to_owned())
        .filter(|n| !n.is_empty());
    Some(Receipt {
        by,
        name,
        original: original.filter(|o| !o.is_empty()),
        displayed,
    })
}

/// The `Name: value` fields of a report, folded lines joined.
fn fields(text: &str) -> Vec<(String, String)> {
    let mut fields: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        if line.starts_with([' ', '\t']) {
            if let Some((_, value)) = fields.last_mut() {
                value.push(' ');
                value.push_str(line.trim());
            }
        } else if let Some((name, value)) = line.split_once(':') {
            fields.push((name.trim().to_owned(), value.trim().to_owned()));
        }
    }
    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    const THUNDERBIRD: &str = "From: Bea <bea@example.org>\r\n\
To: a@example.com\r\n\
Subject: Return Receipt (displayed) - Proposal v2\r\n\
MIME-Version: 1.0\r\n\
Content-Type: multipart/report; report-type=disposition-notification;\r\n\
\x20boundary=\"b1\"\r\n\
\r\n\
--b1\r\n\
Content-Type: text/plain; charset=UTF-8\r\n\
\r\n\
This is a Return Receipt for the mail that you sent.\r\n\
--b1\r\n\
Content-Type: message/disposition-notification; name=\"MDNPart2.txt\"\r\n\
Content-Disposition: inline\r\n\
\r\n\
Reporting-UA: example.org; Thunderbird 140.0\r\n\
Final-Recipient: rfc822;bea@example.org\r\n\
Original-Message-ID: <m1@example.com>\r\n\
Disposition: manual-action/MDN-sent-manually;\r\n\
\x20displayed\r\n\
--b1--\r\n";

    #[test]
    fn reads_a_receipt() {
        // (`\x20`: a line continued in a Rust string loses its spaces.)
        assert_eq!(
            parse(THUNDERBIRD.as_bytes()),
            Some(Receipt {
                by: "bea@example.org".into(),
                name: Some("Bea".into()),
                original: Some("m1@example.com".into()),
                displayed: true,
            })
        );
        let deleted = THUNDERBIRD.replace(" displayed\r\n", " deleted\r\n");
        assert!(!parse(deleted.as_bytes()).unwrap().displayed);
    }

    #[test]
    fn ordinary_mail_is_not_a_receipt() {
        let raw = b"From: a@example.org\r\nSubject: Read: hi\r\n\r\nRead it.\r\n";
        assert_eq!(parse(raw), None);
        let delivery = THUNDERBIRD.replace("disposition-notification;\r\n", "delivery-status;\r\n");
        assert_eq!(parse(delivery.as_bytes()), None);
    }
}
