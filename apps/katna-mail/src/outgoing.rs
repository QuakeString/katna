// SPDX-License-Identifier: GPL-3.0-or-later

//! Outgoing mail: the addresses typed in the compose window, and the
//! RFC 5322 message handed to `katna-daemon` to send. Plain text in UTF-8;
//! the daemon adds `Date` and `Message-ID`. No GPUI here.

use std::fmt::Write as _;

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

/// Whether `email` looks like `local@domain`.
pub fn valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.rsplit_once('@') else {
        return false;
    };
    !local.is_empty()
        && !domain.is_empty()
        && !domain.starts_with('.')
        && !domain.ends_with('.')
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
    out.push_str("MIME-Version: 1.0\r\n");
    out.push_str("Content-Type: text/plain; charset=utf-8\r\n");
    let body = message.body.replace("\r\n", "\n");
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
    out.into_bytes()
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
