// SPDX-License-Identifier: GPL-3.0-or-later

//! Autocrypt Level 1 (autocrypt.org/level1.html): mail carries its
//! sender's public key in an `Autocrypt` header, so replies can be
//! encrypted without anyone swapping keys first.
//!
//! Sending: [`header`] makes the header when the user's GnuPG has a secret
//! key for the sender. Receiving: [`in_message`] reads one. Katna does not
//! state a `prefer-encrypt` preference, and only keeps a received key when
//! the user's provider authenticated the mail's `From` (the daemon checks
//! that before [`crate::PeerKeys::remember`]).

use std::ffi::OsStr;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use mail_parser::MessageParser;

use crate::keys::parse_listing;
use crate::mime::{field_name, header_fields};
use crate::{Gnupg, Standard};

/// Largest key put in or taken from a header. A minimal key with one
/// address is under 1 KiB for Ed25519 and about 2 KiB for RSA 4096.
const MAX_KEYDATA: usize = 10 * 1024;

/// The `Autocrypt` header field (with its CRLF) for mail from `sender`:
/// the sender's newest usable key, reduced to the primary key, the user
/// ID of that address and the encryption subkey. `None` when GnuPG has no
/// secret key for the address.
pub fn header(gnupg: &Gnupg, sender: &str) -> Option<String> {
    let sender = sender.trim().to_lowercase();
    if sender.is_empty() || sender.contains(['\r', '\n', ';', '\'', '"']) {
        return None;
    }
    let pattern = format!("<{sender}>");
    let args = [
        OsStr::new("--with-colons"),
        OsStr::new("--list-secret-keys"),
        OsStr::new(&pattern),
    ];
    let run = gnupg.run(Standard::OpenPgp, &args, b"").ok()?;
    let listing = String::from_utf8_lossy(&run.stdout);
    let fingerprint = parse_listing(&listing, Standard::OpenPgp)
        .into_iter()
        .filter(|key| key.usable() && key.info.emails.contains(&sender))
        .max_by_key(|key| key.info.created)?
        .info
        .fingerprint;
    let keep_uid = format!("keep-uid=mbox={sender}");
    let args = [
        OsStr::new("--export-options"),
        OsStr::new("export-minimal"),
        OsStr::new("--export-filter"),
        OsStr::new(&keep_uid),
        OsStr::new("--export-filter"),
        OsStr::new("drop-subkey=usage!~e"),
        OsStr::new("--export"),
        OsStr::new(&fingerprint),
    ];
    let run = gnupg.run(Standard::OpenPgp, &args, b"").ok()?;
    if !run.success || run.stdout.is_empty() || run.stdout.len() > MAX_KEYDATA {
        return None;
    }
    Some(fold(&format!(
        "Autocrypt: addr={sender}; keydata={}",
        STANDARD.encode(&run.stdout)
    )))
}

/// `raw`, an outgoing message, with the sender's [`header`] when there is
/// one, and without any `Autocrypt` field it had before (a message
/// reopened from the outbox has one already).
pub fn with_header(raw: &[u8], gnupg: &Gnupg, sender: &str) -> Vec<u8> {
    let (fields, body_at) = header_fields(raw);
    if fields.is_empty() {
        return raw.to_vec();
    }
    let mut out = header(gnupg, sender).unwrap_or_default().into_bytes();
    for field in fields {
        if field_name(field) != "autocrypt" {
            out.extend_from_slice(field);
        }
    }
    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(&raw[body_at..]);
    out
}

/// The `Autocrypt` header of a received message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Received {
    /// The address it is for, lowercase: the message's `From`.
    pub address: String,
    /// The key, binary.
    pub key: Vec<u8>,
    /// The message's date, Unix seconds.
    pub date: i64,
}

/// Whether `raw`'s top-level header has an `Autocrypt` field: cheap, for
/// deciding whether to hand a message to [`in_message`].
pub fn has_header(raw: &[u8]) -> bool {
    header_fields(raw)
        .0
        .iter()
        .any(|field| field_name(field) == "autocrypt")
}

/// The Autocrypt key in `raw`'s top-level header: exactly one valid
/// `Autocrypt` field whose `addr` is the single `From` address. Says
/// nothing about whether the mail is really from that address.
pub fn in_message(raw: &[u8]) -> Option<Received> {
    let (fields, _) = header_fields(raw);
    let mut found = fields
        .iter()
        .filter(|field| field_name(field) == "autocrypt");
    let field = found.next()?;
    // Several are invalid together (Level 1, 5.1).
    if found.next().is_some() {
        return None;
    }
    let value = std::str::from_utf8(field).ok()?;
    let (address, key) = parse(value.split_once(':')?.1)?;
    let message = MessageParser::default().parse_headers(raw)?;
    let from = message.from()?;
    let mut senders = from.iter();
    let sender = senders.next()?.address()?.trim().to_lowercase();
    if senders.next().is_some() || sender != address {
        return None;
    }
    let date = message.date()?.to_timestamp();
    Some(Received { address, key, date })
}

/// `addr` and the decoded `keydata` of a header value.
fn parse(value: &str) -> Option<(String, Vec<u8>)> {
    let mut address = None;
    let mut keydata = None;
    for attribute in value.split(';') {
        let attribute = attribute.trim();
        if attribute.is_empty() {
            continue;
        }
        let (name, value) = attribute.split_once('=')?;
        match name.trim() {
            "addr" => address = Some(value.trim().to_lowercase()),
            "keydata" => keydata = Some(value),
            "prefer-encrypt" => {}
            // Optional attributes start with an underscore; any other
            // unknown one makes the header invalid (Level 1, 5.1).
            name if name.starts_with('_') => {}
            _ => return None,
        }
    }
    let address = address.filter(|address| address.contains('@'))?;
    let keydata: String = keydata?
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    if keydata.len() > MAX_KEYDATA * 4 / 3 + 4 {
        return None;
    }
    let key = STANDARD.decode(keydata).ok()?;
    (!key.is_empty()).then_some((address, key))
}

/// Folds a header field into lines of at most 78 characters, breaking
/// only inside the base64 key data and after `;`, and adds the CRLF.
fn fold(field: &str) -> String {
    let (head, data) = field.split_once("keydata=").unwrap_or((field, ""));
    let mut out = format!("{head}keydata=");
    let mut line = out.len();
    for chunk in data.as_bytes().chunks(76) {
        if line + chunk.len() > 78 {
            out.push_str("\r\n ");
            line = 1;
        }
        out.push_str(std::str::from_utf8(chunk).expect("base64 is ASCII"));
        line += chunk.len();
    }
    out.push_str("\r\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "xjMEZvU+3BYJKwYBBAHaRw8BAQdA";

    fn message(autocrypt: &str) -> Vec<u8> {
        format!(
            "From: Ada <Ada@Example.org>\r\nTo: bob@example.net\r\n\
Date: Thu, 08 Oct 2026 06:00:00 +0000\r\n{autocrypt}Subject: Hi\r\n\r\nHello\r\n"
        )
        .into_bytes()
    }

    #[test]
    fn reads_a_folded_header() {
        let raw = message(&format!(
            "Autocrypt: addr=ada@example.org; prefer-encrypt=mutual;\r\n _note=x; keydata=\r\n {}\r\n {}\r\n",
            &KEY[..10],
            &KEY[10..]
        ));
        assert!(has_header(&raw));
        let found = in_message(&raw).unwrap();
        assert_eq!(found.address, "ada@example.org");
        assert_eq!(found.key, STANDARD.decode(KEY).unwrap());
        assert_eq!(found.date, 1791439200);
    }

    #[test]
    fn refuses_bad_headers() {
        // Another address than From.
        assert!(
            in_message(&message(&format!(
                "Autocrypt: addr=eve@example.org; keydata={KEY}\r\n"
            )))
            .is_none()
        );
        // An unknown critical attribute.
        assert!(
            in_message(&message(&format!(
                "Autocrypt: addr=ada@example.org; type=2; keydata={KEY}\r\n"
            )))
            .is_none()
        );
        // Two headers.
        let two = format!("Autocrypt: addr=ada@example.org; keydata={KEY}\r\n");
        assert!(in_message(&message(&format!("{two}{two}"))).is_none());
        // No key.
        assert!(in_message(&message("Autocrypt: addr=ada@example.org\r\n")).is_none());
        assert!(in_message(&message("")).is_none());
        assert!(!has_header(&message("")));
    }

    #[test]
    fn replaces_old_headers() {
        let raw = message(&format!(
            "Autocrypt: addr=ada@example.org; keydata=\r\n {KEY}\r\n"
        ));
        // No GnuPG here: the old header goes, and no new one comes.
        let gnupg = Gnupg::new().with_programs("/nonexistent/gpg", "/nonexistent/gpgsm");
        let out = with_header(&raw, &gnupg, "ada@example.org");
        let text = String::from_utf8(out).unwrap();
        assert!(!text.contains("Autocrypt") && !text.contains(KEY), "{text}");
        assert!(text.contains("Subject: Hi\r\n\r\nHello\r\n"));
    }

    #[test]
    fn folds_long_keys() {
        let data = "A".repeat(500);
        let field = fold(&format!("Autocrypt: addr=a@b.example; keydata={data}"));
        assert!(field.ends_with("\r\n"));
        assert!(field.split("\r\n").all(|line| line.len() <= 78));
        let unfolded: String = field.replace("\r\n ", "");
        assert_eq!(
            unfolded.trim_end(),
            format!("Autocrypt: addr=a@b.example; keydata={data}")
        );
    }
}
