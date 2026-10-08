// SPDX-License-Identifier: GPL-3.0-or-later

//! Delivery and read receipts that come back for sent mail: delivery
//! status notifications (DSN, RFC 3464) from the sender's mail server and
//! message disposition notifications (MDN, RFC 8098) from a recipient's
//! app. Both are `multipart/report` mail; the sync reads them as their
//! bodies arrive and the reading view shows them as ticks beside each
//! recipient (`docs/ARCHITECTURE.md` §16.1).

use mail_parser::{MessageParser, MimeHeaders};

/// Larger mail is not a receipt (a report is a few hundred bytes plus at
/// most the original's headers).
pub const MAX_SIZE: usize = 64 * 1024;

/// What a receipt says happened for one recipient.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Their mail server took the message.
    Delivered,
    /// It could not be delivered (a bounce).
    Failed,
    /// Their app showed it.
    Read,
}

/// A receipt for one sent message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// The `Message-ID` of the sent message, without angle brackets.
    pub original: String,
    /// What happened, per recipient (lower case).
    pub outcomes: Vec<(String, Outcome)>,
}

/// `raw` as a delivery or read receipt, if it is one that says something
/// happened (delays and deletions without reading give `None`).
pub fn parse(raw: &[u8]) -> Option<Report> {
    if raw.len() > MAX_SIZE {
        return None;
    }
    let message = MessageParser::default().parse(raw)?;
    let kind = message.content_type()?;
    if !(kind.ctype().eq_ignore_ascii_case("multipart")
        && kind
            .subtype()
            .is_some_and(|s| s.eq_ignore_ascii_case("report")))
    {
        return None;
    }
    let report_type = kind.attribute("report-type")?.to_ascii_lowercase();
    let read = match report_type.as_str() {
        "delivery-status" => false,
        "disposition-notification" => true,
        _ => return None,
    };
    let part = |types: &[&str]| {
        message.parts.iter().find(|part| {
            part.content_type().is_some_and(|t| {
                let full = format!(
                    "{}/{}",
                    t.ctype().to_ascii_lowercase(),
                    t.subtype().unwrap_or_default().to_ascii_lowercase()
                );
                types.contains(&full.as_str())
            })
        })
    };
    let status = part(&[
        "message/delivery-status",
        "message/global-delivery-status",
        "message/disposition-notification",
        "message/global-disposition-notification",
    ])?;
    let status = String::from_utf8_lossy(status.contents()).into_owned();
    // The sent message's headers come back after the report.
    let headers = part(&[
        "text/rfc822-headers",
        "message/rfc822",
        "message/global-headers",
        "message/global",
    ])
    .and_then(|p| match p.message() {
        Some(nested) => nested.message_id().map(str::to_owned),
        None => field(
            &groups(&String::from_utf8_lossy(p.contents())),
            "message-id",
        ),
    });
    let groups = groups(&status);
    let mut outcomes = Vec::new();
    let original = if !read {
        // The first group is about the message; each other is one
        // recipient.
        for group in groups.iter().skip(1) {
            let action = value(group, "action")
                .unwrap_or_default()
                .to_ascii_lowercase();
            let outcome = match action.as_str() {
                "delivered" | "relayed" | "expanded" => Outcome::Delivered,
                "failed" => Outcome::Failed,
                _ => continue,
            };
            if let Some(who) = recipient(group) {
                outcomes.push((who, outcome));
            }
        }
        headers
    } else {
        let group = groups.first()?;
        // "manual-action/MDN-sent-manually; displayed"
        let displayed = value(group, "disposition").is_some_and(|d| {
            d.rsplit_once(';')
                .is_some_and(|(_, kind)| kind.trim().eq_ignore_ascii_case("displayed"))
        });
        if !displayed {
            return None;
        }
        let who = recipient(group).or_else(|| {
            message
                .from()
                .and_then(|from| from.first())
                .and_then(|a| a.address())
                .map(str::to_ascii_lowercase)
        })?;
        outcomes.push((who, Outcome::Read));
        // Some apps name the message only in `In-Reply-To`.
        value(group, "original-message-id")
            .or(headers)
            .or_else(|| message.in_reply_to().as_text().map(str::to_owned))
    };
    let original = original?
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .to_owned();
    (!original.is_empty() && !outcomes.is_empty()).then_some(Report { original, outcomes })
}

/// Whether `header` (a message's header fields) is a read receipt's:
/// `multipart/report` with `report-type=disposition-notification`. Known
/// before the body arrives, so a receipt can be marked read before it
/// would notify.
pub fn is_read_receipt(header: &[u8]) -> bool {
    MessageParser::default()
        .parse_headers(header)
        .and_then(|message| {
            let kind = message.content_type()?;
            Some(
                kind.ctype().eq_ignore_ascii_case("multipart")
                    && kind
                        .subtype()
                        .is_some_and(|s| s.eq_ignore_ascii_case("report"))
                    && kind
                        .attribute("report-type")
                        .is_some_and(|t| t.eq_ignore_ascii_case("disposition-notification")),
            )
        })
        .unwrap_or(false)
}

/// The address a report's field group is about: `Original-Recipient`, the
/// address the message was sent to, else `Final-Recipient`.
fn recipient(group: &[(String, String)]) -> Option<String> {
    ["original-recipient", "final-recipient"]
        .iter()
        .filter_map(|name| value(group, name))
        // "rfc822; bea@example.org"
        .map(|v| {
            v.split_once(';')
                .map_or(v.as_str(), |(_, address)| address)
                .trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_ascii_lowercase()
        })
        .find(|address| address.contains('@'))
}

fn value(group: &[(String, String)], name: &str) -> Option<String> {
    group
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.clone())
}

fn field(groups: &[Vec<(String, String)>], name: &str) -> Option<String> {
    groups.first().and_then(|group| value(group, name))
}

/// The `Name: value` fields of a report, in groups separated by blank
/// lines, folded lines joined.
fn groups(text: &str) -> Vec<Vec<(String, String)>> {
    let mut groups: Vec<Vec<(String, String)>> = vec![Vec::new()];
    for line in text.lines() {
        let Some(group) = groups.last_mut() else {
            break;
        };
        if line.trim().is_empty() {
            if !group.is_empty() {
                groups.push(Vec::new());
            }
        } else if line.starts_with([' ', '\t']) {
            if let Some((_, value)) = group.last_mut() {
                value.push(' ');
                value.push_str(line.trim());
            }
        } else if let Some((name, value)) = line.split_once(':') {
            group.push((name.trim().to_owned(), value.trim().to_owned()));
        }
    }
    groups.retain(|g| !g.is_empty());
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    // (`\x20`: a line continued in a Rust string loses its spaces.)
    const MDN: &str = "From: Bea <bea@example.org>\r\n\
To: ada@example.com\r\n\
Subject: Read: Lunch\r\n\
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
Final-Recipient: rfc822;Bea@example.org\r\n\
Original-Message-ID: <m1@example.com>\r\n\
Disposition: manual-action/MDN-sent-manually;\r\n\
\x20displayed\r\n\
--b1--\r\n";

    const DSN: &str = "From: Mail Delivery System <MAILER-DAEMON@mx.example.com>\r\n\
To: ada@example.com\r\n\
Subject: Successful Mail Delivery Report\r\n\
Auto-Submitted: auto-replied\r\n\
MIME-Version: 1.0\r\n\
Content-Type: multipart/report; report-type=delivery-status;\r\n\
\x20boundary=\"b2\"\r\n\
\r\n\
--b2\r\n\
Content-Type: text/plain\r\n\
\r\n\
Your message was successfully delivered.\r\n\
--b2\r\n\
Content-Type: message/delivery-status\r\n\
\r\n\
Reporting-MTA: dns; mx.example.com\r\n\
Arrival-Date: Mon, 28 Sep 2026 01:00:00 +0000\r\n\
\r\n\
Original-Recipient: rfc822;bea@example.org\r\n\
Final-Recipient: rfc822;bea.smith@example.org\r\n\
Action: delivered\r\n\
Status: 2.0.0\r\n\
\r\n\
Final-Recipient: rfc822;carl@example.org\r\n\
Action: failed\r\n\
Status: 5.1.1\r\n\
\r\n\
Final-Recipient: rfc822;dan@example.org\r\n\
Action: relayed\r\n\
Status: 2.0.0\r\n\
--b2\r\n\
Content-Type: text/rfc822-headers\r\n\
\r\n\
From: ada@example.com\r\n\
To: bea@example.org, carl@example.org, dan@example.org\r\n\
Message-ID: <m2@example.com>\r\n\
Subject: Lunch\r\n\
--b2--\r\n";

    #[test]
    fn reads_a_read_receipt() {
        assert_eq!(
            parse(MDN.as_bytes()),
            Some(Report {
                original: "m1@example.com".into(),
                outcomes: vec![("bea@example.org".into(), Outcome::Read)],
            })
        );
        let deleted = MDN.replace(" displayed\r\n", " deleted\r\n");
        assert_eq!(parse(deleted.as_bytes()), None);
    }

    #[test]
    fn a_read_receipt_without_the_original_id_answers_in_reply_to() {
        // As Outlook sends them: the message named in `In-Reply-To` only.
        let outlook = MDN
            .replace("Original-Message-ID: <m1@example.com>\r\n", "")
            .replace(
                "Subject: Read: Lunch\r\n",
                "Subject: Read: Lunch\r\nIn-Reply-To: <m9@example.com>\r\n",
            );
        assert_eq!(
            parse(outlook.as_bytes()).map(|r| r.original),
            Some("m9@example.com".into())
        );
    }

    #[test]
    fn a_read_receipt_is_known_by_its_header() {
        let header = |raw: &str| raw.split("\r\n\r\n").next().unwrap().to_owned() + "\r\n\r\n";
        assert!(is_read_receipt(header(MDN).as_bytes()));
        assert!(!is_read_receipt(header(DSN).as_bytes()));
        assert!(!is_read_receipt(b"Subject: Read: Lunch\r\n\r\n"));
    }

    #[test]
    fn reads_whom_a_delivery_report_reached_and_who_bounced() {
        assert_eq!(
            parse(DSN.as_bytes()),
            Some(Report {
                original: "m2@example.com".into(),
                outcomes: vec![
                    ("bea@example.org".into(), Outcome::Delivered),
                    ("carl@example.org".into(), Outcome::Failed),
                    ("dan@example.org".into(), Outcome::Delivered),
                ],
            })
        );
        let delayed = DSN
            .replace("Action: delivered", "Action: delayed")
            .replace("Action: failed", "Action: delayed")
            .replace("Action: relayed", "Action: delayed");
        assert_eq!(parse(delayed.as_bytes()), None);
    }

    #[test]
    fn ordinary_mail_is_not_a_receipt() {
        let raw = b"From: a@example.org\r\nSubject: Read: hi\r\n\r\nRead it.\r\n";
        assert_eq!(parse(raw), None);
    }
}
