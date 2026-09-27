// SPDX-License-Identifier: GPL-3.0-or-later

//! Outgoing mail: the addresses typed in the compose window, and the
//! RFC 5322 message handed to `katna-daemon` to send: plain text in UTF-8,
//! with an HTML version, its pictures and attachments as MIME parts when
//! there are any. The daemon adds `Date` (unless a scheduled message sets
//! it) and `Message-ID`. No GPUI here.

use std::fmt::Write as _;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use katna_ui::rich::html::base64_encode;

/// A name and address, as in `Kay Mann <kay@example.org>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mailbox {
    pub name: Option<String>,
    pub email: String,
}

/// A message to send.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Outgoing {
    pub from: Option<Mailbox>,
    pub to: Vec<Mailbox>,
    pub cc: Vec<Mailbox>,
    pub bcc: Vec<Mailbox>,
    pub subject: String,
    pub body: String,
    /// Message-IDs, without angle brackets.
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
    /// The same text as HTML.
    pub html: Option<String>,
    /// Pictures the HTML shows, by `cid:`.
    pub inline: Vec<Part>,
    pub attachments: Vec<Part>,
    /// An RFC 5322 date for the `Date` header (scheduled mail carries the
    /// time it goes out).
    pub date: Option<String>,
}

/// A file in a message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub name: String,
    pub mime: String,
    pub data: Arc<Vec<u8>>,
    /// For pictures in the HTML, without angle brackets.
    pub content_id: Option<String>,
}

/// Parses a comma- or semicolon-separated address list such as
/// `Kay Mann <kay@example.org>, "Doe, Jo" <jo@example.org>, bob@example.org`.
/// The error names the entry that is not an address.
pub fn parse_addresses(text: &str) -> Result<Vec<Mailbox>, String> {
    let mut entries = Vec::new();
    let (mut current, mut quoted, mut angle) = (String::new(), false, false);
    for c in text.chars() {
        match c {
            '"' if !angle => quoted = !quoted,
            '<' if !quoted => angle = true,
            '>' if !quoted => angle = false,
            ',' | ';' if !quoted && !angle => {
                entries.push(std::mem::take(&mut current));
                continue;
            }
            _ => {}
        }
        current.push(c);
    }
    entries.push(current);
    entries
        .iter()
        .map(|e| e.trim())
        .filter(|e| !e.is_empty())
        .map(|entry| parse_mailbox(entry).ok_or_else(|| entry.to_owned()))
        .collect()
}

fn parse_mailbox(entry: &str) -> Option<Mailbox> {
    let (name, email) = match (entry.rfind('<'), entry.ends_with('>')) {
        (Some(open), true) => {
            let name = entry[..open].trim().trim_matches('"').trim();
            let name = (!name.is_empty()).then(|| name.replace("\\\"", "\""));
            (name, entry[open + 1..entry.len() - 1].trim())
        }
        _ => (None, entry),
    };
    valid_email(email).then(|| Mailbox {
        name,
        email: email.to_owned(),
    })
}

/// Whether `email` looks like `local@domain.tld`: something before the @,
/// and a domain of dot-separated names of letters, digits and hyphens.
pub fn valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.rsplit_once('@') else {
        return false;
    };
    let labels: Vec<&str> = domain.split('.').collect();
    !local.is_empty()
        && !local.starts_with('.')
        && !local.ends_with('.')
        && !local.contains("..")
        && labels.len() >= 2
        && labels.iter().all(|label| {
            !label.is_empty()
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label.chars().all(|c| c.is_alphanumeric() || c == '-')
        })
        && labels.last().is_some_and(|tld| tld.chars().count() >= 2)
        && !email
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || "<>(),;:\"[]\\".contains(c))
}

/// The message as RFC 5322 bytes with CRLF line ends. `Bcc` is kept; the
/// daemon takes it out of the copy the recipients get.
pub fn build(message: &Outgoing) -> Vec<u8> {
    let mut out = String::new();
    if let Some(from) = &message.from {
        header(&mut out, "From", &mailboxes(std::slice::from_ref(from)));
    }
    for (name, list) in [
        ("To", &message.to),
        ("Cc", &message.cc),
        ("Bcc", &message.bcc),
    ] {
        if !list.is_empty() {
            header(&mut out, name, &mailboxes(list));
        }
    }
    header(&mut out, "Subject", &encode_words(message.subject.trim()));
    if let Some(id) = &message.in_reply_to {
        header(&mut out, "In-Reply-To", &format!("<{id}>"));
    }
    if !message.references.is_empty() {
        let ids: Vec<String> = message
            .references
            .iter()
            .map(|id| format!("<{id}>"))
            .collect();
        header(&mut out, "References", &ids.join(" "));
    }
    if let Some(date) = &message.date {
        header(&mut out, "Date", date);
    }
    out.push_str("MIME-Version: 1.0\r\n");
    let seed = boundary_seed(message);
    let text = text_part(&message.body);
    let body = match &message.html {
        None => text,
        Some(html) => {
            let html = html_part(html);
            let html = if message.inline.is_empty() {
                html
            } else {
                let parts: Vec<String> = std::iter::once(html)
                    .chain(message.inline.iter().map(file_part))
                    .collect();
                multipart("related; type=\"text/html\"", &parts, seed + 2)
            };
            multipart("alternative", &[text, html], seed + 1)
        }
    };
    let body = if message.attachments.is_empty() {
        body
    } else {
        let parts: Vec<String> = std::iter::once(body)
            .chain(message.attachments.iter().map(file_part))
            .collect();
        multipart("mixed", &parts, seed)
    };
    out.push_str(&body);
    out.into_bytes()
}

/// A MIME entity: its headers, a blank line and its body.
fn text_part(body: &str) -> String {
    let body = body.replace("\r\n", "\n");
    let mut out = String::from("Content-Type: text/plain; charset=utf-8\r\n");
    let plain = body.is_ascii() && body.lines().all(|l| l.len() <= 78 && !l.ends_with(' '));
    if plain {
        out.push_str("Content-Transfer-Encoding: 7bit\r\n\r\n");
        for line in body.lines() {
            out.push_str(line);
            out.push_str("\r\n");
        }
    } else {
        out.push_str("Content-Transfer-Encoding: quoted-printable\r\n\r\n");
        out.push_str(&quoted_printable(&body));
    }
    out
}

fn html_part(html: &str) -> String {
    // Short lines keep quoted-printable readable.
    let html = html.replace("</div>", "</div>\n").replace("<br>", "<br>\n");
    format!(
        "Content-Type: text/html; charset=utf-8\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n{}",
        quoted_printable(&html)
    )
}

fn file_part(part: &Part) -> String {
    let mut out = String::new();
    let name = file_name_params(&part.name);
    let _ = write!(out, "Content-Type: {};\r\n {}\r\n", part.mime, name.0);
    match &part.content_id {
        Some(id) => {
            let _ = write!(out, "Content-Disposition: inline;\r\n {}\r\n", name.1);
            let _ = write!(out, "Content-ID: <{id}>\r\n");
        }
        None => {
            let _ = write!(out, "Content-Disposition: attachment;\r\n {}\r\n", name.1);
        }
    }
    out.push_str("Content-Transfer-Encoding: base64\r\n\r\n");
    let encoded = base64_encode(&part.data);
    for chunk in encoded.as_bytes().chunks(76) {
        out.push_str(std::str::from_utf8(chunk).unwrap_or_default());
        out.push_str("\r\n");
    }
    out
}

/// The `name=` and `filename=` parameters for a file name, RFC 2231
/// encoded when it is not plain ASCII.
fn file_name_params(name: &str) -> (String, String) {
    let simple = name.is_ascii()
        && !name
            .chars()
            .any(|c| c.is_control() || c == '"' || c == '\\');
    if simple {
        return (format!("name=\"{name}\""), format!("filename=\"{name}\""));
    }
    let mut encoded = String::new();
    for byte in name.bytes() {
        if byte.is_ascii_alphanumeric() || b"!#$&+-.^_`|~".contains(&byte) {
            encoded.push(byte as char);
        } else {
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    let words = encode_words(name);
    (
        format!("name=\"{}\"", words.replace('"', "")),
        format!("filename*=utf-8''{encoded}"),
    )
}

fn multipart(kind: &str, parts: &[String], seed: u64) -> String {
    let boundary = format!("katna-{seed:016x}");
    let mut out = format!("Content-Type: multipart/{kind};\r\n boundary=\"{boundary}\"\r\n\r\n");
    for part in parts {
        let _ = write!(out, "--{boundary}\r\n{part}");
        if !part.ends_with("\r\n") {
            out.push_str("\r\n");
        }
    }
    let _ = write!(out, "--{boundary}--\r\n");
    out
}

/// A number the boundaries are made from: from the content, so they are
/// unlikely to appear in it and the same message builds the same bytes.
fn boundary_seed(message: &Outgoing) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    message.body.hash(&mut hasher);
    message.html.hash(&mut hasher);
    message.subject.hash(&mut hasher);
    for part in message.inline.iter().chain(&message.attachments) {
        part.name.hash(&mut hasher);
        part.data.len().hash(&mut hasher);
    }
    hasher.finish() & 0xffff_ffff_ffff_fff0
}

/// `Name: value`, folded at the spaces between items to keep lines short.
fn header(out: &mut String, name: &str, value: &str) {
    let mut line = name.len() + 2;
    out.push_str(name);
    out.push(':');
    for (ix, word) in value.split(' ').enumerate() {
        if ix > 0 && line + 1 + word.len() > 78 {
            out.push_str("\r\n");
            line = 0;
        }
        out.push(' ');
        out.push_str(word);
        line += 1 + word.len();
    }
    out.push_str("\r\n");
}

fn mailboxes(list: &[Mailbox]) -> String {
    list.iter()
        .map(|m| match &m.name {
            Some(name) => format!("{} <{}>", display_name(name), m.email),
            None => m.email.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn display_name(name: &str) -> String {
    if !name.is_ascii() {
        encode_words(name)
    } else if name.chars().any(|c| "()<>[]:;@\\,.\"".contains(c)) {
        format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        name.to_owned()
    }
}

/// RFC 2047 encoded words (Q encoding) for text that is not ASCII.
fn encode_words(text: &str) -> String {
    if text.is_ascii() {
        return text.to_owned();
    }
    const MAX: usize = 75 - "=?utf-8?q??=".len();
    let mut words = Vec::new();
    let mut word = String::new();
    for c in text.chars() {
        let mut encoded = String::new();
        if c == ' ' {
            encoded.push('_');
        } else if c.is_ascii_alphanumeric() || "!*+-/".contains(c) {
            encoded.push(c);
        } else {
            let mut buf = [0; 4];
            for byte in c.encode_utf8(&mut buf).bytes() {
                let _ = write!(encoded, "={byte:02X}");
            }
        }
        if word.len() + encoded.len() > MAX {
            words.push(std::mem::take(&mut word));
        }
        word.push_str(&encoded);
    }
    words.push(word);
    words
        .iter()
        .map(|w| format!("=?utf-8?q?{w}?="))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Quoted-printable (RFC 2045) with CRLF line ends.
fn quoted_printable(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let bytes = line.as_bytes();
        let mut width = 0;
        for (ix, &byte) in bytes.iter().enumerate() {
            let last = ix + 1 == bytes.len();
            let literal = (byte == b' ' || byte == b'\t') && !last
                || (33..=126).contains(&byte) && byte != b'=';
            let piece = if literal {
                (byte as char).to_string()
            } else {
                format!("={byte:02X}")
            };
            if width + piece.len() > 75 {
                out.push_str("=\r\n");
                width = 0;
            }
            out.push_str(&piece);
            width += piece.len();
        }
        out.push_str("\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mailbox(name: Option<&str>, email: &str) -> Mailbox {
        Mailbox {
            name: name.map(str::to_owned),
            email: email.to_owned(),
        }
    }

    #[test]
    fn parses_address_lists() {
        assert_eq!(
            parse_addresses(r#"Kay Mann <kay@enron.com>, "Doe, Jo" <jo@x.org>; bob@x.org,"#),
            Ok(vec![
                mailbox(Some("Kay Mann"), "kay@enron.com"),
                mailbox(Some("Doe, Jo"), "jo@x.org"),
                mailbox(None, "bob@x.org"),
            ])
        );
        assert_eq!(parse_addresses("  "), Ok(vec![]));
        assert_eq!(parse_addresses("kay@enron.com, bob"), Err("bob".to_owned()));
        assert_eq!(parse_addresses("a b@x.org"), Err("a b@x.org".to_owned()));
    }

    #[test]
    fn checks_the_address_format() {
        for good in ["kay@enron.com", "k.m+news@mail.x-y.org", "jo@münchen.de"] {
            assert!(valid_email(good), "{good}");
        }
        for bad in [
            "xyz",
            "kay@",
            "@x.org",
            "kay@x",
            "kay@x.",
            "kay@.x.org",
            "kay@x..org",
            "kay@x.o",
            "kay@-x.org",
            "kay@x_y.org",
            ".kay@x.org",
            "k..m@x.org",
            "kay mann@x.org",
        ] {
            assert!(!valid_email(bad), "{bad}");
        }
    }

    #[test]
    fn builds_a_plain_message() {
        let raw = build(&Outgoing {
            from: Some(mailbox(Some("Kay Mann"), "kay@enron.com")),
            to: vec![mailbox(None, "bob@x.org")],
            bcc: vec![mailbox(Some("Doe, Jo"), "jo@x.org")],
            subject: "Gas".to_owned(),
            body: "Hi Bob,\n\nsee you.\n".to_owned(),
            in_reply_to: Some("1@x.org".to_owned()),
            references: vec!["0@x.org".to_owned(), "1@x.org".to_owned()],
            ..Outgoing::default()
        });
        assert_eq!(
            String::from_utf8(raw).unwrap(),
            "From: Kay Mann <kay@enron.com>\r\nTo: bob@x.org\r\n\
             Bcc: \"Doe, Jo\" <jo@x.org>\r\nSubject: Gas\r\nIn-Reply-To: <1@x.org>\r\n\
             References: <0@x.org> <1@x.org>\r\nMIME-Version: 1.0\r\n\
             Content-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: 7bit\r\n\r\n\
             Hi Bob,\r\n\r\nsee you.\r\n"
        );
    }

    #[test]
    fn encodes_non_ascii() {
        let raw = String::from_utf8(build(&Outgoing {
            to: vec![mailbox(Some("Zoë"), "zoe@x.org")],
            subject: "Café prices".to_owned(),
            body: "Naïve = true\n".to_owned(),
            ..Outgoing::default()
        }))
        .unwrap();
        assert!(raw.contains("To: =?utf-8?q?Zo=C3=AB?= <zoe@x.org>\r\n"));
        assert!(raw.contains("Subject: =?utf-8?q?Caf=C3=A9_prices?=\r\n"));
        assert!(raw.ends_with("quoted-printable\r\n\r\nNa=C3=AFve =3D true\r\n"));
        // What goes out parses back to what was written.
        let view = katna_render::message_view(raw.as_bytes());
        assert_eq!(view.subject, "Café prices");
        assert_eq!(view.body.trim_end(), "Naïve = true");
    }

    #[test]
    fn builds_html_with_pictures_and_attachments() {
        let raw = String::from_utf8(build(&Outgoing {
            to: vec![mailbox(None, "bob@x.org")],
            subject: "Report".to_owned(),
            body: "Hi *Bob*\n".to_owned(),
            html: Some("<div dir=\"ltr\"><div><b>Hi</b> Bob</div><div><img src=\"cid:logo@katna\"></div></div>".to_owned()),
            inline: vec![Part {
                name: "logo.png".to_owned(),
                mime: "image/png".to_owned(),
                data: Arc::new(vec![0x89, b'P', b'N', b'G']),
                content_id: Some("logo@katna".to_owned()),
            }],
            attachments: vec![Part {
                name: "Qüarterly report.pdf".to_owned(),
                mime: "application/pdf".to_owned(),
                data: Arc::new(b"%PDF-1.4 hello".to_vec()),
                content_id: None,
            }],
            date: Some("Mon, 28 Sep 2026 08:00:00 +0000".to_owned()),
            ..Outgoing::default()
        }))
        .unwrap();
        assert!(raw.contains("Date: Mon, 28 Sep 2026 08:00:00 +0000\r\n"));
        assert!(raw.contains("Content-Type: multipart/mixed;\r\n boundary="));
        assert!(raw.contains("Content-Type: multipart/alternative;\r\n boundary="));
        assert!(raw.contains("Content-Type: multipart/related; type=\"text/html\";\r\n boundary="));
        assert!(raw.contains("Content-ID: <logo@katna>\r\n"));
        assert!(raw.contains("filename*=utf-8''Q%C3%BCarterly%20report.pdf"));
        assert!(raw.lines().all(|l| l.len() <= 78), "{raw}");
        // It parses back: the text, the HTML and the attachment are there.
        let parsed = mail_parser::MessageParser::default()
            .parse(raw.as_bytes())
            .unwrap();
        assert_eq!(parsed.body_text(0).unwrap().trim_end(), "Hi *Bob*");
        assert!(parsed.body_html(0).unwrap().contains("<b>Hi</b> Bob"));
        use mail_parser::MimeHeaders;
        assert!(
            parsed
                .attachments()
                .any(|a| a.attachment_name() == Some("Qüarterly report.pdf")
                    && a.contents() == b"%PDF-1.4 hello")
        );
    }

    #[test]
    fn folds_long_lines() {
        let to: Vec<Mailbox> = (0..12)
            .map(|i| mailbox(None, &format!("person{i}@example.org")))
            .collect();
        let raw = String::from_utf8(build(&Outgoing {
            to,
            body: format!("{}\n", "word ".repeat(40).trim_end()),
            ..Outgoing::default()
        }))
        .unwrap();
        assert!(raw.lines().all(|l| l.len() <= 78), "{raw}");
        let view = katna_render::message_view(raw.as_bytes());
        assert_eq!(view.to.len(), 12);
        assert_eq!(view.body.trim_end(), "word ".repeat(40).trim_end());
    }
}
