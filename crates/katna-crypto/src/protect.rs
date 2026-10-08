// SPDX-License-Identifier: GPL-3.0-or-later

//! Signing and encrypting outgoing mail: PGP/MIME (RFC 3156) and S/MIME
//! (RFC 8551).

use std::ffi::OsStr;

use crate::gnupg::Run;
use crate::keys::{Key, encryption_keys};
use crate::mime::{field_name, header_fields};
use crate::{Gnupg, Standard};

/// What to do to an outgoing message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Protect {
    pub standard: Standard,
    pub sign: bool,
    pub encrypt: bool,
}

/// Who a message goes to, for encryption.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Recipients {
    /// The sender: always encrypted to as well, so Sent stays readable,
    /// and the signing key.
    pub sender: String,
    /// To and Cc.
    pub visible: Vec<String>,
    /// Bcc: encrypted to without their key IDs in the message, so the
    /// other recipients cannot see who else got it.
    pub hidden: Vec<String>,
}

/// Why a message could not be signed or encrypted. Nothing was sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtectError {
    /// These addresses have no usable key in the keyring.
    MissingKeys(Vec<String>),
    /// The sender has no secret key to sign with.
    NoSigningKey,
    /// The user closed the passphrase prompt.
    Cancelled,
    /// `gpg` or `gpgsm` is not installed.
    Unavailable,
    Other(String),
}

impl std::fmt::Display for ProtectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingKeys(emails) => write!(
                f,
                "Can't encrypt: no key for {}. Import their key, or send without encryption.",
                emails.join(", ")
            ),
            Self::NoSigningKey => {
                f.write_str("Can't sign: you have no secret key for this address.")
            }
            Self::Cancelled => f.write_str("Signing was cancelled."),
            Self::Unavailable => f.write_str("Install GnuPG to sign or encrypt mail."),
            Self::Other(message) => write!(f, "Can't sign or encrypt: {message}"),
        }
    }
}

impl std::error::Error for ProtectError {}

/// The headers copied inside an encrypted message whose subject is
/// hidden (draft-autocrypt-lamps-protected-headers).
const PROTECTED_HEADERS: &[&str] = &[
    "subject",
    "from",
    "to",
    "cc",
    "reply-to",
    "date",
    "message-id",
    "references",
    "in-reply-to",
];

/// Signs and/or encrypts `raw`, an RFC 5322 message with CRLF line ends
/// whose body is 7-bit (quoted-printable or plain ASCII) so a signature
/// survives transport. The routing headers (From, To, Subject, …) stay
/// outside; the `Content-*` headers and the body are protected.
///
/// Runs GnuPG and may wait for a passphrase: never call it on the UI
/// thread.
pub fn protect(
    raw: &[u8],
    how: Protect,
    recipients: &Recipients,
    gnupg: &Gnupg,
) -> Result<Vec<u8>, ProtectError> {
    if !how.sign && !how.encrypt {
        return Ok(raw.to_vec());
    }
    let (fields, body_at) = header_fields(raw);
    // Encrypted OpenPGP mail hides its subject: the real one goes inside,
    // with copies of the other headers (protected headers, as Thunderbird
    // and KMail send and read them), and the outside says "...".
    let hide_subject = how.standard == Standard::OpenPgp && how.encrypt;
    let mut outer = Vec::new();
    let mut content = Vec::new();
    let mut inner = Vec::new();
    for field in &fields {
        let name = field_name(field);
        if name.starts_with("content-") {
            content.push(*field);
        } else if name != "mime-version" {
            if hide_subject && PROTECTED_HEADERS.contains(&name.as_str()) {
                inner.extend_from_slice(field);
            }
            if hide_subject && name == "subject" {
                outer.extend_from_slice(b"Subject: ...\r\n");
            } else {
                outer.extend_from_slice(field);
            }
        }
    }
    let mut entity = Vec::new();
    if content.is_empty() {
        content.push(b"Content-Type: text/plain; charset=utf-8\r\n");
    }
    for field in content {
        // A message opened and sealed again (the outbox's tracked copies)
        // may be marked already.
        let marked = String::from_utf8_lossy(field)
            .to_ascii_lowercase()
            .contains("protected-headers=");
        if hide_subject && !marked && field_name(field) == "content-type" {
            let end = field.len()
                - field
                    .iter()
                    .rev()
                    .take_while(|b| b.is_ascii_whitespace())
                    .count();
            entity.extend_from_slice(&field[..end]);
            entity.extend_from_slice(b"; protected-headers=\"v1\"\r\n");
        } else {
            entity.extend_from_slice(field);
        }
    }
    entity.extend_from_slice(&inner);
    entity.extend_from_slice(b"\r\n");
    entity.extend_from_slice(&crlf(&raw[body_at..]));

    let protected = match (how.standard, how.encrypt) {
        (Standard::OpenPgp, false) => pgp_signed(&entity, recipients, gnupg)?,
        (Standard::OpenPgp, true) => pgp_encrypted(&entity, how.sign, recipients, gnupg)?,
        (Standard::Smime, false) => smime_signed(&entity, recipients, gnupg)?,
        (Standard::Smime, true) => {
            let inner = if how.sign {
                smime_signed(&entity, recipients, gnupg)?
            } else {
                entity
            };
            smime_encrypted(&inner, recipients, gnupg)?
        }
    };
    let mut out = outer;
    out.extend_from_slice(b"MIME-Version: 1.0\r\n");
    out.extend_from_slice(&protected);
    Ok(out)
}

/// `multipart/signed` with an OpenPGP signature over `entity`.
fn pgp_signed(entity: &[u8], to: &Recipients, gnupg: &Gnupg) -> Result<Vec<u8>, ProtectError> {
    let args = ["--armor", "--detach-sign", "--local-user", &to.sender];
    let run = checked(gnupg.run(Standard::OpenPgp, &os(&args), entity), false)?;
    let micalg = hash_name(run.status.signed_with, "pgp-");
    let boundary = boundary(entity);
    let mut out = format!(
        "Content-Type: multipart/signed; micalg={micalg};\r\n protocol=\"application/pgp-signature\"; boundary=\"{boundary}\"\r\n\r\n\
This is an OpenPGP/MIME signed message (RFC 4880 and 3156)\r\n--{boundary}\r\n"
    )
    .into_bytes();
    out.extend_from_slice(entity);
    out.extend_from_slice(
        format!(
            "\r\n--{boundary}\r\nContent-Type: application/pgp-signature; name=\"signature.asc\"\r\n\
Content-Description: OpenPGP digital signature\r\n\
Content-Disposition: attachment; filename=\"signature.asc\"\r\n\r\n"
        )
        .as_bytes(),
    );
    out.extend_from_slice(&crlf(&run.stdout));
    out.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    Ok(out)
}

/// `multipart/encrypted`, signed inside when `sign`.
fn pgp_encrypted(
    entity: &[u8],
    sign: bool,
    to: &Recipients,
    gnupg: &Gnupg,
) -> Result<Vec<u8>, ProtectError> {
    let keys = resolve(Standard::OpenPgp, to, gnupg)?;
    // The keys were chosen by exact address above; `always` lets a key
    // the user has not certified be used (the compose window says so).
    let mut args: Vec<String> = ["--armor", "--trust-model", "always", "--encrypt"]
        .map(str::to_owned)
        .to_vec();
    for (key, hidden) in keys {
        let (option, value) = match key.file {
            // A key Katna found for the address, outside the keyring.
            Some(file) => ("-file", file.to_string_lossy().into_owned()),
            None => ("", key.fingerprint),
        };
        let kind = if hidden {
            "--hidden-recipient"
        } else {
            "--recipient"
        };
        args.push(format!("{kind}{option}"));
        args.push(value);
    }
    if sign {
        args.extend([
            "--sign".to_owned(),
            "--local-user".to_owned(),
            to.sender.clone(),
        ]);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let run = checked(gnupg.run(Standard::OpenPgp, &os(&args), entity), true)?;
    let boundary = boundary(&run.stdout);
    let mut out = format!(
        "Content-Type: multipart/encrypted;\r\n protocol=\"application/pgp-encrypted\"; boundary=\"{boundary}\"\r\n\r\n\
This is an OpenPGP/MIME encrypted message (RFC 4880 and 3156)\r\n\
--{boundary}\r\nContent-Type: application/pgp-encrypted\r\n\
Content-Description: PGP/MIME version identification\r\n\r\nVersion: 1\r\n\r\n\
--{boundary}\r\nContent-Type: application/octet-stream; name=\"encrypted.asc\"\r\n\
Content-Description: OpenPGP encrypted message\r\n\
Content-Disposition: inline; filename=\"encrypted.asc\"\r\n\r\n"
    )
    .into_bytes();
    out.extend_from_slice(&crlf(&run.stdout));
    out.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    Ok(out)
}

/// `multipart/signed` with an S/MIME (CMS) signature over `entity`.
fn smime_signed(entity: &[u8], to: &Recipients, gnupg: &Gnupg) -> Result<Vec<u8>, ProtectError> {
    let args = ["--detach-sign", "--local-user", &to.sender];
    let run = checked(gnupg.run(Standard::Smime, &os(&args), entity), false)?;
    let micalg = hash_name(run.status.signed_with, "");
    let boundary = boundary(entity);
    let mut out = format!(
        "Content-Type: multipart/signed; micalg={micalg};\r\n protocol=\"application/pkcs7-signature\"; boundary=\"{boundary}\"\r\n\r\n\
This is an S/MIME signed message\r\n--{boundary}\r\n"
    )
    .into_bytes();
    out.extend_from_slice(entity);
    out.extend_from_slice(
        format!(
            "\r\n--{boundary}\r\nContent-Type: application/pkcs7-signature; name=\"smime.p7s\"\r\n\
Content-Transfer-Encoding: base64\r\n\
Content-Disposition: attachment; filename=\"smime.p7s\"\r\n\r\n{}\r\n--{boundary}--\r\n",
            base64_lines(&run.stdout)
        )
        .as_bytes(),
    );
    Ok(out)
}

/// `application/pkcs7-mime; smime-type=enveloped-data`.
fn smime_encrypted(entity: &[u8], to: &Recipients, gnupg: &Gnupg) -> Result<Vec<u8>, ProtectError> {
    let keys = resolve(Standard::Smime, to, gnupg)?;
    // CMS has no hidden recipients: Bcc recipients are listed like the
    // others.
    let mut args: Vec<String> = vec!["--encrypt".to_owned()];
    for (key, _) in keys {
        args.push("--recipient".to_owned());
        args.push(key.fingerprint);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let run = checked(gnupg.run(Standard::Smime, &os(&args), entity), true)?;
    Ok(format!(
        "Content-Type: application/pkcs7-mime; smime-type=enveloped-data; name=\"smime.p7m\"\r\n\
Content-Transfer-Encoding: base64\r\n\
Content-Disposition: attachment; filename=\"smime.p7m\"\r\n\r\n{}\r\n",
        base64_lines(&run.stdout)
    )
    .into_bytes())
}

/// The key for the sender and every recipient, and whether it is a hidden
/// (Bcc) one. Fails with the addresses that have no key.
fn resolve(
    standard: Standard,
    to: &Recipients,
    gnupg: &Gnupg,
) -> Result<Vec<(Key, bool)>, ProtectError> {
    let mut emails: Vec<(String, bool)> = std::iter::once((to.sender.clone(), false))
        .chain(to.visible.iter().map(|e| (e.clone(), false)))
        .chain(to.hidden.iter().map(|e| (e.clone(), true)))
        .map(|(e, hidden)| (e.trim().to_lowercase(), hidden))
        .collect();
    let mut seen = std::collections::HashSet::new();
    emails.retain(|(email, _)| seen.insert(email.clone()));
    let addresses: Vec<String> = emails.iter().map(|(e, _)| e.clone()).collect();
    let keys = encryption_keys(gnupg, standard, &addresses);
    let missing: Vec<String> = addresses
        .iter()
        .zip(&keys)
        .filter(|(_, key)| key.is_none())
        .map(|(email, _)| email.clone())
        .collect();
    if !missing.is_empty() {
        return Err(ProtectError::MissingKeys(missing));
    }
    Ok(keys
        .into_iter()
        .zip(emails)
        .filter_map(|(key, (_, hidden))| Some((key?, hidden)))
        .collect())
}

/// Maps a GnuPG run to its output or the reason it failed.
fn checked(run: std::io::Result<Run>, encrypting: bool) -> Result<Run, ProtectError> {
    let run = match run {
        Ok(run) => run,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(ProtectError::Unavailable);
        }
        Err(err) => return Err(ProtectError::Other(err.to_string())),
    };
    if run.success && !run.stdout.is_empty() {
        return Ok(run);
    }
    let status = &run.status;
    Err(if status.cancelled {
        ProtectError::Cancelled
    } else if status.invalid_signer {
        ProtectError::NoSigningKey
    } else if encrypting && !status.invalid_recipients.is_empty() {
        ProtectError::MissingKeys(status.invalid_recipients.clone())
    } else {
        ProtectError::Other(
            status
                .message
                .clone()
                .unwrap_or_else(|| "GnuPG failed.".to_owned()),
        )
    })
}

fn os<'a>(args: &'a [&'a str]) -> Vec<&'a OsStr> {
    args.iter().map(OsStr::new).collect()
}

/// `micalg` for a libgcrypt hash algorithm number (RFC 3156 §5, RFC 8551
/// §3.4.3.2).
fn hash_name(algorithm: Option<u32>, prefix: &str) -> String {
    let name = match algorithm {
        Some(2) => "sha1",
        Some(8) => "sha256",
        Some(9) => "sha384",
        Some(11) => "sha224",
        _ => "sha512",
    };
    if prefix.is_empty() {
        // S/MIME spells them with a dash: sha-256.
        name.replacen("sha", "sha-", 1)
    } else {
        format!("{prefix}{name}")
    }
}

/// A boundary that does not occur in `content`.
fn boundary(content: &[u8]) -> String {
    let seed = fnv(content);
    (0..)
        .map(|n| format!("katna-{seed:016x}-{n}"))
        .find(|b| !content.windows(b.len()).any(|w| w == b.as_bytes()))
        .expect("some boundary is free")
}

/// A cheap, stable hash (FNV-1a) for boundaries.
fn fnv(content: &[u8]) -> u64 {
    content.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Every line ending as CRLF.
fn crlf(content: &[u8]) -> Vec<u8> {
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

/// Base64 in 76-character lines (RFC 2045).
fn base64_lines(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len() * 4 / 3 + data.len() / 38 + 4);
    for (i, chunk) in data.chunks(3).enumerate() {
        if i > 0 && i % 19 == 0 {
            out.push_str("\r\n");
        }
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for j in 0..4 {
            if j <= chunk.len() {
                out.push(ALPHABET[((n >> (18 - 6 * j)) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micalg_names() {
        assert_eq!(hash_name(Some(10), "pgp-"), "pgp-sha512");
        assert_eq!(hash_name(Some(8), "pgp-"), "pgp-sha256");
        assert_eq!(hash_name(Some(8), ""), "sha-256");
    }

    #[test]
    fn base64() {
        assert_eq!(base64_lines(b"hello"), "aGVsbG8=");
        assert_eq!(base64_lines(b"hi"), "aGk=");
        assert_eq!(base64_lines(&[0u8; 57]).len(), 76);
        assert!(base64_lines(&[0u8; 58]).contains("\r\n"));
        let decoded =
            mail_parser::decoders::base64::base64_decode(base64_lines(&[7u8; 300]).as_bytes());
        assert_eq!(decoded.as_deref(), Some(&[7u8; 300][..]));
    }

    #[test]
    fn boundaries_avoid_the_content() {
        let b = boundary(b"abc");
        assert!(b.starts_with("katna-"));
        let clash = format!("x{b}y");
        assert_ne!(boundary(clash.as_bytes()), b);
    }
}
