// SPDX-License-Identifier: GPL-3.0-or-later

//! Plain-text messages written outside Katna Mail's composer: address
//! headers, RFC 2047 encoded words and quoted-printable bodies. Used for
//! the reply typed into a notification (`katna-sync`'s `quick_reply`), the
//! mail rules' replies and the drafts an AI assistant saves through
//! `katnactl mcp`.

use base64::{Engine, engine::general_purpose::STANDARD};

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

/// `mailbox` as an address header: the name quoted or encoded as needed.
pub fn address(mailbox: &Mailbox) -> String {
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
pub fn encode_words(text: &str) -> String {
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
pub fn quoted_printable(text: &str) -> String {
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

    #[test]
    fn names_are_quoted_or_encoded() {
        let mailbox = |name: &str| Mailbox {
            name: Some(name.into()),
            email: "a@x.org".into(),
        };
        assert_eq!(address(&mailbox("Bo Li")), "Bo Li <a@x.org>");
        assert_eq!(address(&mailbox("Li, Bo")), "\"Li, Bo\" <a@x.org>");
        assert!(address(&mailbox("Zoë")).starts_with("=?utf-8?B?"));
    }
}
