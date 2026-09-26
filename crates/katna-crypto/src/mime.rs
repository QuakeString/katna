// SPDX-License-Identifier: GPL-3.0-or-later

//! Finding the protected part of a message and putting the opened content
//! in its place.

use mail_parser::decoders::base64::base64_decode;
use mail_parser::decoders::charsets::map::charset_decoder;
use mail_parser::decoders::quoted_printable::quoted_printable_decode;
use mail_parser::{Encoding, Message, MessagePart, MimeHeaders, PartType};

use crate::gnupg::Gnupg;
use crate::{Decryption, Protection, Signature, Standard};

const PGP_MESSAGE_BEGIN: &[u8] = b"-----BEGIN PGP MESSAGE-----";
const PGP_MESSAGE_END: &[u8] = b"-----END PGP MESSAGE-----";
const PGP_SIGNED_BEGIN: &[u8] = b"-----BEGIN PGP SIGNED MESSAGE-----";
const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

/// Where a part sits in the raw message.
#[derive(Debug, Clone, Copy)]
struct Span {
    /// The part is the whole message.
    root: bool,
    header: usize,
    /// Where the root's body starts: its header block ends here.
    root_body: usize,
    end: usize,
}

/// The first protected part of a message, with what GnuPG needs.
pub(crate) struct Found {
    kind: Kind,
    span: Span,
}

enum Kind {
    /// `multipart/encrypted` (RFC 3156).
    PgpMime { ciphertext: Vec<u8> },
    /// `application/pkcs7-mime`, enveloped (RFC 8551).
    SmimeEnveloped { data: Vec<u8> },
    /// `application/pkcs7-mime; smime-type=signed-data`.
    SmimeOpaque { data: Vec<u8> },
    /// `multipart/signed`.
    Signed {
        standard: Standard,
        /// The signed part exactly as sent.
        content: Vec<u8>,
        signature: Vec<u8>,
    },
    /// A `text/plain` part with an armored PGP block in it.
    Inline {
        encrypted: bool,
        /// The part's text, transfer encoding undone but still in its own
        /// charset: signatures are made over these bytes.
        text: Vec<u8>,
        charset: Option<String>,
    },
}

impl Found {
    pub(crate) fn protection(&self) -> Protection {
        let standard = self.standard();
        match &self.kind {
            Kind::PgpMime { .. } | Kind::SmimeEnveloped { .. } => Protection::Encrypted(standard),
            Kind::Inline {
                encrypted: true, ..
            } => Protection::Encrypted(standard),
            _ => Protection::Signed(standard),
        }
    }

    pub(crate) fn standard(&self) -> Standard {
        match &self.kind {
            Kind::PgpMime { .. } | Kind::Inline { .. } => Standard::OpenPgp,
            Kind::SmimeEnveloped { .. } | Kind::SmimeOpaque { .. } => Standard::Smime,
            Kind::Signed { standard, .. } => *standard,
        }
    }
}

/// What opening one layer gave.
pub(crate) struct Step {
    /// The message with this layer opened; `None` when it stays as it was.
    pub raw: Option<Vec<u8>>,
    pub whole: bool,
    pub decryption: Option<Decryption>,
    pub signatures: Vec<Signature>,
}

/// The first protected part of `message`, in document order.
///
/// Encrypted content is only opened when it is the whole message: showing
/// decrypted text next to parts an attacker added around it is how EFAIL
/// leaked plaintext. Signed parts are checked wherever they are, and the
/// report says when they are only a part.
pub(crate) fn find(message: &Message<'_>) -> Option<Found> {
    let raw = message.raw_message();
    let root_body = message.parts.first()?.offset_body as usize;
    for (ix, part) in message.parts.iter().enumerate() {
        let root = ix == 0;
        let span = Span {
            root,
            header: part.offset_header as usize,
            root_body,
            end: part.offset_end as usize,
        };
        let (ctype, subtype) = match part.content_type() {
            Some(ct) => (
                ct.ctype().to_ascii_lowercase(),
                ct.subtype().unwrap_or_default().to_ascii_lowercase(),
            ),
            None => ("text".to_owned(), "plain".to_owned()),
        };
        let attribute = |name: &str| {
            part.content_type()
                .and_then(|ct| ct.attribute(name))
                .map(|value| value.trim().to_ascii_lowercase())
        };
        let kind = match (ctype.as_str(), subtype.as_str()) {
            ("multipart", "encrypted") if root => {
                if attribute("protocol").as_deref() != Some("application/pgp-encrypted") {
                    continue;
                }
                let ciphertext = child(message, part, 1)?.contents().to_vec();
                Kind::PgpMime { ciphertext }
            }
            ("multipart", "signed") => {
                let standard = match attribute("protocol").as_deref() {
                    Some("application/pgp-signature") => Standard::OpenPgp,
                    Some("application/pkcs7-signature" | "application/x-pkcs7-signature") => {
                        Standard::Smime
                    }
                    _ => continue,
                };
                let (Some(content), Some(signature)) =
                    (child(message, part, 0), child(message, part, 1))
                else {
                    continue;
                };
                let content = raw
                    .get(content.offset_header as usize..content.offset_end as usize)?
                    .to_vec();
                Kind::Signed {
                    standard,
                    content,
                    signature: signature.contents().to_vec(),
                }
            }
            ("application", "pkcs7-mime" | "x-pkcs7-mime") => {
                let data = part.contents().to_vec();
                match attribute("smime-type").as_deref() {
                    Some("signed-data") => Kind::SmimeOpaque { data },
                    _ if root => Kind::SmimeEnveloped { data },
                    _ => continue,
                }
            }
            ("text", "plain") => {
                if !matches!(part.body, PartType::Text(_))
                    || part
                        .content_disposition()
                        .is_some_and(|d| d.is_attachment())
                {
                    continue;
                }
                let text = transfer_decoded(raw, part);
                let encrypted = if root && find_bytes(&text, PGP_MESSAGE_BEGIN).is_some() {
                    true
                } else if find_bytes(&text, PGP_SIGNED_BEGIN).is_some() {
                    false
                } else {
                    continue;
                };
                Kind::Inline {
                    encrypted,
                    text,
                    charset: attribute("charset"),
                }
            }
            _ => continue,
        };
        return Some(Found { kind, span });
    }
    None
}

/// Opens one protection layer of `raw`.
pub(crate) fn open_layer(raw: &[u8], found: Found, gnupg: &Gnupg, sender: Option<&str>) -> Step {
    let span = found.span;
    match found.kind {
        Kind::PgpMime { ciphertext } => decrypted(
            raw,
            span,
            gnupg.decrypt(Standard::OpenPgp, &ciphertext, true, sender),
        ),
        Kind::SmimeEnveloped { data } => decrypted(
            raw,
            span,
            gnupg.decrypt(Standard::Smime, &data, true, sender),
        ),
        Kind::SmimeOpaque { data } => {
            let outcome = gnupg.verify_opaque(&data, sender);
            Step {
                raw: (!outcome.content.is_empty()).then(|| splice(raw, span, &outcome.content)),
                whole: span.root,
                decryption: None,
                signatures: outcome.signatures,
            }
        }
        Kind::Signed {
            standard,
            content,
            signature,
        } => Step {
            signatures: gnupg.verify_detached(standard, &signature, &canonical(&content), sender),
            // The signature part goes; the signed part stays as it was.
            raw: Some(splice(raw, span, &content)),
            whole: span.root,
            decryption: None,
        },
        Kind::Inline {
            encrypted,
            text,
            charset,
        } => {
            let (begin, end_marker) = if encrypted {
                (PGP_MESSAGE_BEGIN, PGP_MESSAGE_END)
            } else {
                (PGP_SIGNED_BEGIN, PGP_SIGNATURE_END)
            };
            let Some((start, end)) = armor_block(&text, begin, end_marker) else {
                return Step {
                    raw: None,
                    whole: span.root,
                    decryption: None,
                    signatures: Vec::new(),
                };
            };
            let outcome = gnupg.decrypt(Standard::OpenPgp, &text[start..end], encrypted, sender);
            let (before, after) = (&text[..start], &text[end..]);
            let whole = span.root
                && before.iter().all(u8::is_ascii_whitespace)
                && after.iter().all(u8::is_ascii_whitespace);
            let opened = outcome
                .decryption
                .as_ref()
                .is_none_or(|d| *d == Decryption::Decrypted)
                && !outcome.content.is_empty();
            let raw = opened.then(|| {
                let decode = |bytes: &[u8]| decode_text(bytes, charset.as_deref());
                let mut body = decode(before);
                body.push_str(&decode(&outcome.content));
                body.push_str(&decode(after));
                let mut entity =
                    b"Content-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: 8bit\r\n\r\n"
                        .to_vec();
                entity.extend_from_slice(body.as_bytes());
                splice(raw, span, &entity)
            });
            Step {
                raw,
                whole,
                decryption: outcome.decryption.filter(|_| encrypted),
                signatures: outcome.signatures,
            }
        }
    }
}

fn decrypted(raw: &[u8], span: Span, outcome: crate::gnupg::Outcome) -> Step {
    let ok = outcome.decryption == Some(Decryption::Decrypted);
    Step {
        raw: ok.then(|| splice(raw, span, &outcome.content)),
        whole: span.root,
        decryption: outcome.decryption,
        signatures: outcome.signatures,
    }
}

/// The `n`th child of multipart `part`.
fn child<'a>(
    message: &'a Message<'_>,
    part: &MessagePart<'_>,
    n: usize,
) -> Option<&'a MessagePart<'a>> {
    let id = *part.sub_parts()?.get(n)?;
    message.part(id)
}

/// The body of `part` with its transfer encoding undone, in its own
/// charset.
fn transfer_decoded(raw: &[u8], part: &MessagePart<'_>) -> Vec<u8> {
    let body = raw
        .get(part.offset_body as usize..part.offset_end as usize)
        .unwrap_or_default();
    match part.encoding {
        Encoding::None => Some(body.to_vec()),
        Encoding::QuotedPrintable => quoted_printable_decode(body),
        Encoding::Base64 => base64_decode(body),
    }
    .unwrap_or_else(|| body.to_vec())
}

/// Text in `charset`, as a string.
fn decode_text(bytes: &[u8], charset: Option<&str>) -> String {
    match charset.and_then(|cs| charset_decoder(cs.as_bytes())) {
        Some(decode) => decode(bytes),
        None => String::from_utf8_lossy(bytes).into_owned(),
    }
}

/// MIME canonical form: every line ends in CRLF. Signatures over MIME
/// parts are made over this form (RFC 3156 §5, RFC 8551 §3.1.1), whatever
/// line endings the stored copy has.
fn canonical(content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len() + content.len() / 32);
    let mut previous = 0;
    for &byte in content {
        if byte == b'\n' && previous != b'\r' {
            out.push(b'\r');
        }
        out.push(byte);
        previous = byte;
    }
    out
}

/// The armored block from `begin` to the end of the line holding `end`.
fn armor_block(text: &[u8], begin: &[u8], end: &[u8]) -> Option<(usize, usize)> {
    let start = find_bytes(text, begin)?;
    let end_at = start + find_bytes(&text[start..], end)? + end.len();
    let line_end = text[end_at..]
        .iter()
        .position(|&b| b == b'\n')
        .map_or(text.len(), |p| end_at + p + 1);
    Some((start, line_end))
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// `raw` with the part at `span` replaced by the MIME entity `entity`.
///
/// For the root, the message keeps its own headers (From, Subject, …) and
/// takes only the entity's `Content-*` headers. An entity that marks
/// protected headers (`protected-headers="v1"`) also brings its Subject,
/// so an encrypted subject shows instead of the "..." sent outside.
fn splice(raw: &[u8], span: Span, entity: &[u8]) -> Vec<u8> {
    if !span.root {
        let mut out = Vec::with_capacity(raw.len() + entity.len());
        out.extend_from_slice(&raw[..span.header.min(raw.len())]);
        out.extend_from_slice(entity);
        out.extend_from_slice(raw.get(span.end..).unwrap_or_default());
        return out;
    }

    let (outer, _) = header_fields(&raw[..span.root_body.min(raw.len())]);
    let (inner, body_at) = header_fields(entity);
    let (inner, body) = if inner.is_empty() {
        (Vec::new(), entity)
    } else {
        (inner, &entity[body_at..])
    };
    let protected = inner.iter().any(|field| {
        field_name(field) == "content-type"
            && String::from_utf8_lossy(field)
                .to_ascii_lowercase()
                .contains("protected-headers")
    });
    let inner_subject = protected && inner.iter().any(|f| field_name(f) == "subject");

    let mut out = Vec::with_capacity(raw.len() + entity.len());
    for field in &outer {
        let name = field_name(field);
        if name.starts_with("content-")
            || name == "mime-version"
            || (inner_subject && name == "subject")
        {
            continue;
        }
        push_field(&mut out, field);
    }
    out.extend_from_slice(b"MIME-Version: 1.0\r\n");
    let mut typed = false;
    for field in &inner {
        let name = field_name(field);
        if name.starts_with("content-") || (inner_subject && name == "subject") {
            typed |= name == "content-type";
            push_field(&mut out, field);
        }
    }
    if !typed {
        out.extend_from_slice(b"Content-Type: text/plain; charset=utf-8\r\n");
    }
    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(body);
    out
}

fn push_field(out: &mut Vec<u8>, field: &[u8]) {
    out.extend_from_slice(field);
    if !field.ends_with(b"\n") {
        out.extend_from_slice(b"\r\n");
    }
}

/// The header fields at the start of `block` (each with its folded lines
/// and line ending) and where the body after the blank line starts. No
/// fields when `block` does not start with a header.
fn header_fields(block: &[u8]) -> (Vec<&[u8]>, usize) {
    let mut fields: Vec<(usize, usize)> = Vec::new();
    let mut at = 0;
    while at < block.len() {
        let line_end = block[at..]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(block.len(), |p| at + p + 1);
        let line = &block[at..line_end];
        if line == b"\r\n" || line == b"\n" {
            at = line_end;
            break;
        }
        if matches!(line.first(), Some(b' ' | b'\t')) && !fields.is_empty() {
            fields.last_mut().expect("not empty").1 = line_end;
        } else if is_field_start(line) {
            fields.push((at, line_end));
        } else if fields.is_empty() {
            return (Vec::new(), 0);
        } else {
            // Not a header line: the header ended without a blank line.
            break;
        }
        at = line_end;
    }
    (fields.into_iter().map(|(s, e)| &block[s..e]).collect(), at)
}

fn is_field_start(line: &[u8]) -> bool {
    match line.iter().position(|&b| b == b':') {
        Some(colon) if colon > 0 => line[..colon]
            .iter()
            .all(|&b| b.is_ascii_graphic() && b != b':'),
        _ => false,
    }
}

fn field_name(field: &[u8]) -> String {
    let colon = field.iter().position(|&b| b == b':').unwrap_or(0);
    String::from_utf8_lossy(&field[..colon])
        .trim()
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use mail_parser::MessageParser;

    use super::*;

    fn found(raw: &[u8]) -> Option<Found> {
        find(&MessageParser::default().parse(raw).expect("parses"))
    }

    const SIGNED: &[u8] = b"From: Ada <ada@example.org>\r\n\
Subject: Signed\r\n\
MIME-Version: 1.0\r\n\
Content-Type: multipart/signed; micalg=pgp-sha256;\r\n protocol=\"application/pgp-signature\"; boundary=\"b1\"\r\n\
\r\n\
This is an OpenPGP/MIME signed message.\r\n\
--b1\r\n\
Content-Type: text/plain; charset=utf-8\r\n\
\r\n\
Hello Bob.\r\n\
--b1\r\n\
Content-Type: application/pgp-signature; name=\"signature.asc\"\r\n\
\r\n\
-----BEGIN PGP SIGNATURE-----\r\nAAAA\r\n-----END PGP SIGNATURE-----\r\n\
--b1--\r\n";

    #[test]
    fn signed_content_is_the_exact_part() {
        let found = found(SIGNED).expect("found");
        assert_eq!(found.protection(), Protection::Signed(Standard::OpenPgp));
        let Kind::Signed {
            content, signature, ..
        } = &found.kind
        else {
            panic!("not signed");
        };
        assert_eq!(
            content.as_slice(),
            b"Content-Type: text/plain; charset=utf-8\r\n\r\nHello Bob."
        );
        assert!(signature.starts_with(b"-----BEGIN PGP SIGNATURE-----"));
        // The signed part replaces the whole multipart/signed.
        let spliced = splice(SIGNED, found.span, content);
        let text = String::from_utf8(spliced).unwrap();
        assert!(text.starts_with("From: Ada <ada@example.org>\r\nSubject: Signed\r\n"));
        assert!(text.ends_with(
            "MIME-Version: 1.0\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello Bob."
        ));
    }

    #[test]
    fn nested_part_is_spliced_in_place() {
        let raw = b"From: a@example.org\r\n\
Content-Type: multipart/mixed; boundary=\"m\"\r\n\
\r\n\
--m\r\n\
Content-Type: multipart/signed; protocol=\"application/pgp-signature\"; boundary=\"s\"\r\n\
\r\n\
--s\r\n\
Content-Type: text/plain\r\n\
\r\n\
Signed text\r\n\
--s\r\n\
Content-Type: application/pgp-signature\r\n\
\r\n\
SIG\r\n\
--s--\r\n\
\r\n\
--m\r\n\
Content-Type: text/plain\r\n\
\r\n\
List footer\r\n\
--m--\r\n";
        let found = found(raw).expect("found");
        assert!(!found.span.root);
        let Kind::Signed { content, .. } = &found.kind else {
            panic!("not signed");
        };
        let spliced = splice(raw, found.span, content);
        let message = MessageParser::default().parse(&spliced).unwrap();
        let texts: Vec<_> = (0..message.text_body_count())
            .filter_map(|i| message.body_text(i))
            .map(|t| t.trim().to_owned())
            .collect();
        assert_eq!(texts, ["Signed text", "List footer"]);
        assert_eq!(message.attachment_count(), 0);
    }

    #[test]
    fn encrypted_parts_only_count_at_the_root() {
        let root = b"From: a@example.org\r\n\
Content-Type: multipart/encrypted; protocol=\"application/pgp-encrypted\"; boundary=\"e\"\r\n\
\r\n\
--e\r\n\
Content-Type: application/pgp-encrypted\r\n\
\r\n\
Version: 1\r\n\
--e\r\n\
Content-Type: application/octet-stream\r\n\
\r\n\
-----BEGIN PGP MESSAGE-----\r\nxx\r\n-----END PGP MESSAGE-----\r\n\
--e--\r\n";
        let found_root = found(root).expect("found");
        assert_eq!(
            found_root.protection(),
            Protection::Encrypted(Standard::OpenPgp)
        );
        let Kind::PgpMime { ciphertext } = &found_root.kind else {
            panic!("not PGP/MIME");
        };
        assert!(ciphertext.starts_with(PGP_MESSAGE_BEGIN));

        let mut wrapped = b"From: a@example.org\r\n\
Content-Type: multipart/mixed; boundary=\"w\"\r\n\r\n--w\r\n"
            .to_vec();
        wrapped.extend_from_slice(&root[b"From: a@example.org\r\n".len()..]);
        wrapped.extend_from_slice(b"\r\n--w--\r\n");
        assert!(found(&wrapped).is_none());

        let inline_part = b"From: a@example.org\r\n\
Content-Type: multipart/mixed; boundary=\"w\"\r\n\r\n--w\r\n\
Content-Type: text/plain\r\n\r\n\
-----BEGIN PGP MESSAGE-----\r\nxx\r\n-----END PGP MESSAGE-----\r\n--w--\r\n";
        assert!(found(inline_part).is_none());
    }

    #[test]
    fn inline_blocks() {
        let raw = b"From: a@example.org\r\n\
Content-Type: text/plain; charset=iso-8859-1\r\n\
Content-Transfer-Encoding: quoted-printable\r\n\
\r\n\
-----BEGIN PGP SIGNED MESSAGE-----\r\n\
Hash: SHA256\r\n\
\r\n\
Gr=FC=DFe\r\n\
-----BEGIN PGP SIGNATURE-----\r\n\
xx\r\n\
-----END PGP SIGNATURE-----\r\n";
        let found = found(raw).expect("found");
        assert_eq!(found.protection(), Protection::Signed(Standard::OpenPgp));
        let Kind::Inline { text, charset, .. } = &found.kind else {
            panic!("not inline");
        };
        // Still Latin-1: the signature is over these bytes.
        assert!(find_bytes(text, b"Gr\xfc\xdfe\r\n").is_some());
        assert_eq!(charset.as_deref(), Some("iso-8859-1"));
        assert_eq!(decode_text(b"Gr\xfc\xdfe", charset.as_deref()), "Grüße");
        let (start, end) = armor_block(text, PGP_SIGNED_BEGIN, PGP_SIGNATURE_END).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, text.len());
    }

    #[test]
    fn protected_subject_replaces_the_outer_one() {
        let raw = b"From: a@example.org\r\nSubject: ...\r\n\
Content-Type: multipart/encrypted; protocol=\"application/pgp-encrypted\"; boundary=\"e\"\r\n\r\nbody";
        let span = Span {
            root: true,
            header: 0,
            root_body: raw.len() - 4,
            end: raw.len(),
        };
        let entity = b"Content-Type: text/plain; charset=utf-8; protected-headers=\"v1\"\r\n\
Subject: The real subject\r\n\
From: mallory@example.org\r\n\
\r\n\
Secret\r\n";
        let text = String::from_utf8(splice(raw, span, entity)).unwrap();
        assert_eq!(
            text,
            "From: a@example.org\r\nMIME-Version: 1.0\r\n\
Content-Type: text/plain; charset=utf-8; protected-headers=\"v1\"\r\n\
Subject: The real subject\r\n\r\nSecret\r\n"
        );

        // Without the marker the inner Subject is ignored.
        let entity = b"Content-Type: text/plain\r\nSubject: Other\r\n\r\nSecret\r\n";
        let text = String::from_utf8(splice(raw, span, entity)).unwrap();
        assert!(text.contains("Subject: ...\r\n"));
        assert!(!text.contains("Other"));
    }

    #[test]
    fn canonical_line_endings() {
        assert_eq!(canonical(b"a\nb\r\nc\n"), b"a\r\nb\r\nc\r\n");
    }
}
