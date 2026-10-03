// SPDX-License-Identifier: GPL-3.0-or-later

//! Short plain-text replies written outside Katna Mail: the reply typed
//! into a new-mail notification (`docs/ARCHITECTURE.md` §15.1.2). The
//! daemon reads the message being answered with [`original`] and builds
//! the reply with [`build`]; the outbox adds `Date` and `Message-ID`.

use base64::{Engine, engine::general_purpose::STANDARD};
use katna_import::{Role, parse_message};
use mail_parser::MessageParser;

/// An address with its display name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mailbox {
    pub name: Option<String>,
    pub email: String,
}

impl Mailbox {
    /// `Name <email>`, or the address alone.
    pub fn text(&self) -> String {
        match self
            .name
            .as_deref()
            .map(str::trim)
            .filter(|n| !n.is_empty())
        {
            Some(name) => format!("{name} <{}>", self.email),
            None => self.email.clone(),
        }
    }
}

/// What a reply needs of the message it answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Original {
    /// `Message-ID` without angle brackets.
    pub message_id: Option<String>,
    /// `References` without angle brackets, oldest first.
    pub references: Vec<String>,
    pub subject: String,
    /// Its sender.
    pub from: Option<Mailbox>,
    /// Where replies go: `Reply-To`, else the sender.
    pub reply_to: Option<Mailbox>,
    /// Unix seconds.
    pub date: Option<i64>,
    /// The body as plain text, without inline PGP armor.
    pub text: String,
}

/// Reads the message `raw` for a reply. `None` without headers.
pub fn original(raw: &[u8]) -> Option<Original> {
    let parsed = parse_message(raw)?;
    let mailbox = |role: Role| {
        parsed
            .participants
            .iter()
            .find(|p| p.role == role)
            .map(|p| Mailbox {
                name: p.display_name.clone(),
                email: p.email_norm.clone(),
            })
    };
    let from = mailbox(Role::From);
    let reply_to = mailbox(Role::ReplyTo).or_else(|| from.clone());
    let text = MessageParser::default()
        .parse(raw)
        .and_then(|message| message.body_text(0).map(|t| t.into_owned()))
        .map(|text| {
            katna_crypto::without_armor(&text)
                .replace("\r\n", "\n")
                .trim()
                .to_owned()
        })
        .unwrap_or_default();
    Some(Original {
        message_id: parsed.message_id,
        references: parsed.references,
        subject: parsed.subject.unwrap_or_default(),
        from,
        reply_to,
        date: parsed.date,
        text,
    })
}

/// A reply to `original` from `from` saying `text`, then `signature` (if
/// not empty) after a `-- ` line, then the original quoted under
/// `quote_intro` ("On …, … wrote:"). `None` when there is no one to reply
/// to.
pub fn build(
    original: &Original,
    from: &Mailbox,
    text: &str,
    signature: &str,
    quote_intro: &str,
) -> Option<Vec<u8>> {
    let to = original.reply_to.as_ref()?;
    let mut head = String::new();
    head.push_str(&format!("From: {}\r\n", address(from)));
    head.push_str(&format!("To: {}\r\n", address(to)));
    head.push_str(&format!(
        "Subject: {}\r\n",
        encode_words(&re(&original.subject))
    ));
    if let Some(id) = &original.message_id {
        head.push_str(&format!("In-Reply-To: <{id}>\r\n"));
        let references: Vec<String> = original
            .references
            .iter()
            .filter(|r| *r != id)
            .chain(std::iter::once(id))
            .map(|r| format!("<{r}>"))
            .collect();
        head.push_str(&format!("References: {}\r\n", references.join("\r\n ")));
    }
    head.push_str("MIME-Version: 1.0\r\n");
    head.push_str("Content-Type: text/plain; charset=utf-8\r\n");
    head.push_str("Content-Transfer-Encoding: quoted-printable\r\n\r\n");

    let mut body = text.trim_end().to_owned();
    let signature = signature.trim();
    if !signature.is_empty() {
        body.push_str("\n\n-- \n");
        body.push_str(signature);
    }
    if !original.text.is_empty() {
        body.push_str("\n\n");
        body.push_str(quote_intro);
        body.push('\n');
        for line in original.text.lines() {
            if line.is_empty() {
                body.push_str(">\n");
            } else {
                body.push_str("> ");
                body.push_str(line);
                body.push('\n');
            }
        }
    }
    let mut raw = head.into_bytes();
    raw.extend_from_slice(quoted_printable(&body).as_bytes());
    Some(raw)
}

/// `subject` with one `Re: ` in front.
fn re(subject: &str) -> String {
    let subject = subject.trim();
    let already = subject
        .get(..3)
        .is_some_and(|start| start.eq_ignore_ascii_case("re:"));
    if already {
        subject.to_owned()
    } else {
        format!("Re: {subject}")
    }
}

/// `mailbox` as an address header: the name quoted or encoded as needed.
pub(crate) fn address(mailbox: &Mailbox) -> String {
    let name = mailbox
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty());
    match name {
        None => mailbox.email.clone(),
        Some(name) if !name.is_ascii() => format!("{} <{}>", encode_words(name), mailbox.email),
        Some(name)
            if name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || " .-'".contains(c)) =>
        {
            format!("{name} <{}>", mailbox.email)
        }
        Some(name) => {
            let quoted = name.replace('\\', "\\\\").replace('"', "\\\"");
            format!("\"{quoted}\" <{}>", mailbox.email)
        }
    }
}

/// `text` as a header value: as it is when plain ASCII, else RFC 2047
/// encoded words of at most 75 characters, folded onto lines of their own.
pub(crate) fn encode_words(text: &str) -> String {
    if text.chars().all(|c| c.is_ascii() && !c.is_ascii_control()) && !text.contains("=?") {
        return text.to_owned();
    }
    // 45 bytes are 60 in base64: with `=?utf-8?B?` and `?=`, 72.
    let mut words = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for (at, c) in text.char_indices() {
        if at + c.len_utf8() - start > 45 {
            words.push(&text[start..end]);
            start = end;
        }
        end = at + c.len_utf8();
    }
    words.push(&text[start..end]);
    words
        .iter()
        .map(|w| format!("=?utf-8?B?{}?=", STANDARD.encode(w.as_bytes())))
        .collect::<Vec<_>>()
        .join("\r\n ")
}

/// `text` (lines ending in `\n`) as quoted-printable with CRLF line ends
/// and lines of at most 76 characters.
fn quoted_printable(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + text.len() / 8);
    for line in text.split('\n') {
        let line = line.strip_suffix('\r').unwrap_or(line);
        let bytes = line.as_bytes();
        let mut width = 0;
        for (i, &b) in bytes.iter().enumerate() {
            let last = i + 1 == bytes.len();
            let plain =
                (b == b' ' || b == b'\t') && !last || (b'!'..=b'~').contains(&b) && b != b'=';
            let piece = if plain {
                (b as char).to_string()
            } else {
                format!("={b:02X}")
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
    // The split above adds one line end too many after the last `\n`.
    if text.ends_with('\n') {
        out.truncate(out.len() - 2);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAIL: &[u8] = b"Message-ID: <q3@example.org>\r
References: <q1@example.org>\r
In-Reply-To: <q1@example.org>\r
Date: Thu, 01 Oct 2026 09:00:00 +0000\r
From: Alex Lee <alex@example.org>\r
To: kay@example.com\r
Subject: Q3 planning\r
Content-Type: text/plain; charset=utf-8\r
\r
Can we meet at 3?\r
\r
Alex\r
";

    #[test]
    fn a_reply_answers_in_the_conversation() {
        let original = original(MAIL).unwrap();
        assert_eq!(original.text, "Can we meet at 3?\n\nAlex");
        let me = Mailbox {
            name: Some("Kay Mann".into()),
            email: "kay@example.com".into(),
        };
        let raw = build(
            &original,
            &me,
            "Works for me – see you.",
            "Kay",
            "On Thu, Alex Lee <alex@example.org> wrote:",
        )
        .unwrap();
        let text = String::from_utf8(raw.clone()).unwrap();
        assert!(text.contains("From: Kay Mann <kay@example.com>\r\n"));
        assert!(text.contains("To: Alex Lee <alex@example.org>\r\n"));
        assert!(text.contains("Subject: Re: Q3 planning\r\n"));
        assert!(text.contains("In-Reply-To: <q3@example.org>\r\n"));
        assert!(text.contains("References: <q1@example.org>\r\n <q3@example.org>\r\n"));

        let parsed = MessageParser::default().parse(&raw[..]).unwrap();
        assert_eq!(
            parsed.body_text(0).unwrap(),
            "Works for me – see you.\n\n-- \nKay\n\n\
             On Thu, Alex Lee <alex@example.org> wrote:\n\
             > Can we meet at 3?\n>\n> Alex\n"
                .replace('\n', "\r\n")
        );
    }

    #[test]
    fn reply_to_wins_and_names_are_encoded() {
        let mail = b"Message-ID: <a@x>\r\nFrom: list@x.org\r\nReply-To: Bo <bo@x.org>\r\nSubject: RE: Hi\r\n\r\nHi\r\n";
        let original = original(mail).unwrap();
        let me = Mailbox {
            name: Some("Zoë, K".into()),
            email: "z@x.org".into(),
        };
        let raw = String::from_utf8(build(&original, &me, "Yes", "", "On …").unwrap()).unwrap();
        assert!(raw.contains("To: Bo <bo@x.org>\r\n"), "{raw}");
        assert!(raw.contains("Subject: RE: Hi\r\n"));
        assert!(raw.contains("From: =?utf-8?B?"));
        let parsed = MessageParser::default().parse(raw.as_bytes()).unwrap();
        let from = parsed.from().unwrap().first().unwrap();
        assert_eq!(from.name(), Some("Zoë, K"));
    }

    #[test]
    fn long_lines_and_long_subjects_fold() {
        let long = "ä".repeat(100);
        let encoded = encode_words(&long);
        assert!(
            encoded.lines().all(|l| l.trim_end().len() <= 76),
            "{encoded}"
        );
        let qp = quoted_printable(&format!("{long} = end \n"));
        assert!(qp.lines().all(|l| l.len() <= 76), "{qp}");
        assert!(qp.contains("=3D"));
        assert!(qp.ends_with("end=20\r\n"), "{qp}");
    }
}
