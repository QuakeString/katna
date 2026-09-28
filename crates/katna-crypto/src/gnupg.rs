// SPDX-License-Identifier: GPL-3.0-or-later

//! Running `gpg` and `gpgsm`.

use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::status::{RawSignature, Status};
use crate::{Decryption, Failure, Signature, SignatureState, Standard};

/// Where GnuPG is and which keyring it uses.
#[derive(Debug, Clone)]
pub struct Gnupg {
    gpg: PathBuf,
    gpgsm: PathBuf,
    /// `GNUPGHOME`; `None` uses the user's own (`~/.gnupg`).
    home: Option<PathBuf>,
}

impl Default for Gnupg {
    fn default() -> Self {
        Self {
            gpg: "gpg".into(),
            gpgsm: "gpgsm".into(),
            home: None,
        }
    }
}

/// What a decrypt or verify run gave.
pub(crate) struct Outcome {
    /// The decrypted or verified content. Empty when there is none.
    pub content: Vec<u8>,
    pub decryption: Option<Decryption>,
    pub signatures: Vec<Signature>,
}

pub(crate) struct Run {
    pub stdout: Vec<u8>,
    pub status: Status,
    /// GnuPG exited with 0.
    pub success: bool,
}

impl Gnupg {
    /// `gpg` and `gpgsm` from `PATH`, with the user's own keyring.
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs these programs instead of `gpg` and `gpgsm` from `PATH`.
    pub fn with_programs(mut self, gpg: impl Into<PathBuf>, gpgsm: impl Into<PathBuf>) -> Self {
        self.gpg = gpg.into();
        self.gpgsm = gpgsm.into();
        self
    }

    /// Uses the keyring in `home` instead of the user's (`GNUPGHOME`).
    pub fn with_home(mut self, home: impl Into<PathBuf>) -> Self {
        self.home = Some(home.into());
        self
    }

    /// Decrypts `input`, and checks the signatures inside it. With
    /// `encrypted` false it also opens inline clear-signed text, for which
    /// `decryption` is `None`; with `encrypted` true, input GnuPG does not
    /// see as encrypted at all is a failure.
    pub(crate) fn decrypt(
        &self,
        standard: Standard,
        input: &[u8],
        encrypted: bool,
        sender: Option<&str>,
    ) -> Outcome {
        let mut args: Vec<&OsStr> = no_key_fetching(standard)
            .iter()
            .map(|a| OsStr::new(*a))
            .collect();
        args.push(OsStr::new("--decrypt"));
        match self.run(standard, &args, input) {
            Ok(run) => {
                // The status lines decide, not the exit status: gpg exits
                // with an error when a signature inside does not check out
                // (2 for a missing key), and when one of several keys the
                // message is encrypted to fails though another opened it.
                // A stream that is damaged or cut short never counts as
                // decrypted ([`Status::decryption`]).
                let decryption = match run.status.decryption() {
                    None if encrypted => Some(Decryption::Failed(run.status.failure())),
                    decryption => decryption,
                };
                let content = if decryption
                    .as_ref()
                    .is_none_or(|d| *d == Decryption::Decrypted)
                {
                    run.stdout
                } else {
                    Vec::new()
                };
                Outcome {
                    content,
                    decryption,
                    signatures: self.signatures(standard, &run.status, sender),
                }
            }
            Err(err) => Outcome {
                content: Vec::new(),
                decryption: Some(Decryption::Failed(failure(&err))),
                signatures: Vec::new(),
            },
        }
    }

    /// Checks a detached `signature` over `content`.
    pub(crate) fn verify_detached(
        &self,
        standard: Standard,
        signature: &[u8],
        content: &[u8],
        sender: Option<&str>,
    ) -> Vec<Signature> {
        let result = (|| {
            let mut file = temp_file()?;
            file.write_all(signature)?;
            file.flush()?;
            let mut args: Vec<&OsStr> = no_key_fetching(standard)
                .iter()
                .map(|a| OsStr::new(*a))
                .collect();
            args.extend([
                OsStr::new("--verify"),
                file.path().as_os_str(),
                OsStr::new("-"),
            ]);
            self.run(standard, &args, content)
        })();
        match result {
            Ok(run) => {
                let signatures = self.signatures(standard, &run.status, sender);
                if signatures.is_empty() {
                    vec![unchecked(SignatureState::Error)]
                } else {
                    signatures
                }
            }
            Err(err) => vec![unchecked(signature_state(&err))],
        }
    }

    /// Checks an S/MIME opaque signature and returns the signed content.
    pub(crate) fn verify_opaque(&self, data: &[u8], sender: Option<&str>) -> Outcome {
        let args = [
            OsStr::new("--verify"),
            OsStr::new("--output"),
            OsStr::new("-"),
        ];
        match self.run(Standard::Smime, &args, data) {
            Ok(run) => {
                let mut signatures = self.signatures(Standard::Smime, &run.status, sender);
                if signatures.is_empty() {
                    signatures.push(unchecked(SignatureState::Error));
                }
                Outcome {
                    content: run.stdout,
                    decryption: None,
                    signatures,
                }
            }
            Err(err) => Outcome {
                content: Vec::new(),
                decryption: None,
                signatures: vec![unchecked(signature_state(&err))],
            },
        }
    }

    /// Runs the tool for `standard` with status lines on stderr, feeding
    /// it `input`.
    ///
    /// The status lines share stderr with GnuPG's log. `--no-verbose`
    /// keeps the log from printing what a message names (such as a file
    /// name), whatever `gpg.conf` says, so a message cannot print lines
    /// that look like status lines (CVE-2018-12020). A status pipe of its
    /// own would need a third file descriptor, which `std::process` cannot
    /// pass without `unsafe`.
    pub(crate) fn run(&self, standard: Standard, args: &[&OsStr], input: &[u8]) -> io::Result<Run> {
        let mut command = Command::new(match standard {
            Standard::OpenPgp => &self.gpg,
            Standard::Smime => &self.gpgsm,
        });
        // `--batch` stops GnuPG asking on a terminal; gpg-agent still
        // shows pinentry for a passphrase.
        command
            .args(["--batch", "--no-tty", "--no-verbose", "--status-fd", "2"])
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(home) = &self.home {
            command.env("GNUPGHOME", home);
        }
        let mut child = command.spawn()?;
        let mut stdin = child.stdin.take().expect("stdin is piped");
        let output = std::thread::scope(|scope| {
            // Written from another thread: GnuPG streams its output while
            // it reads, and both pipes filling up would deadlock.
            scope.spawn(move || {
                // GnuPG may stop reading early (bad data); its status says
                // why, so a broken pipe here is not an error of its own.
                let _ = stdin.write_all(input);
            });
            child.wait_with_output()
        })?;
        Ok(Run {
            stdout: output.stdout,
            status: Status::parse(&output.stderr),
            success: output.status.success(),
        })
    }

    /// The signatures of a run, with each key's addresses looked up.
    fn signatures(
        &self,
        standard: Standard,
        status: &Status,
        sender: Option<&str>,
    ) -> Vec<Signature> {
        status
            .signatures
            .iter()
            .filter(|raw| raw.state.is_some())
            .map(|raw| self.signature(standard, raw, sender))
            .collect()
    }

    fn signature(&self, standard: Standard, raw: &RawSignature, sender: Option<&str>) -> Signature {
        let state = raw.state.unwrap_or(SignatureState::Error);
        let key_uids = match (&raw.key, state) {
            (_, SignatureState::MissingKey | SignatureState::Error) => Vec::new(),
            (Some(key), _) => self.key_uids(standard, key),
            (None, _) => Vec::new(),
        };
        let mut emails: Vec<String> = key_uids
            .iter()
            .chain(raw.uid.as_ref())
            .filter_map(|uid| email_of(uid))
            .collect();
        emails.sort();
        emails.dedup();
        let signer = match standard {
            Standard::OpenPgp => raw.uid.clone(),
            Standard::Smime => {
                let dn = key_uids
                    .iter()
                    .find(|uid| !uid.starts_with('<'))
                    .or(raw.uid.as_ref());
                let name = dn.and_then(|dn| common_name(dn));
                match (name, emails.first()) {
                    (Some(name), Some(email)) => Some(format!("{name} <{email}>")),
                    (Some(name), None) => Some(name),
                    (None, Some(email)) => Some(email.clone()),
                    (None, None) => dn.cloned(),
                }
            }
        };
        Signature {
            state,
            signer,
            from_sender: sender.is_some_and(|sender| emails.iter().any(|e| e == sender)),
            emails,
            key: raw.key.clone(),
            created: raw.created,
            validity: raw.validity,
        }
    }

    /// The user IDs of `key`, from a colon listing.
    fn key_uids(&self, standard: Standard, key: &str) -> Vec<String> {
        // Only a hex fingerprint or key ID goes on the command line.
        if key.is_empty() || !key.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Vec::new();
        }
        let args = [
            OsStr::new("--with-colons"),
            OsStr::new("--list-keys"),
            OsStr::new(key),
        ];
        let Ok(run) = self.run(standard, &args, b"") else {
            return Vec::new();
        };
        String::from_utf8_lossy(&run.stdout)
            .lines()
            .filter(|line| line.starts_with("uid:"))
            .filter_map(|line| line.split(':').nth(9))
            .map(unescape_colons)
            .filter(|uid| !uid.is_empty())
            .collect()
    }
}

/// Options that stop `gpg` fetching a signer's key from the network while
/// it checks a message, even when the user's `gpg.conf` asks it to: the
/// fetch would tell the sender that the message was opened. `gpgsm`
/// fetches nothing unless told to.
fn no_key_fetching(standard: Standard) -> &'static [&'static str] {
    match standard {
        Standard::OpenPgp => &["--no-auto-key-retrieve", "--auto-key-locate", "local"],
        Standard::Smime => &[],
    }
}

/// A signature GnuPG could not look at.
fn unchecked(state: SignatureState) -> Signature {
    Signature {
        state,
        signer: None,
        emails: Vec::new(),
        key: None,
        created: None,
        validity: Default::default(),
        from_sender: false,
    }
}

fn failure(err: &io::Error) -> Failure {
    if err.kind() == io::ErrorKind::NotFound {
        Failure::Unavailable
    } else {
        Failure::Other(err.to_string())
    }
}

fn signature_state(err: &io::Error) -> SignatureState {
    if err.kind() == io::ErrorKind::NotFound {
        SignatureState::Unavailable
    } else {
        SignatureState::Error
    }
}

/// A file for a detached signature, in the private runtime directory when
/// there is one. Signatures are not secret, but nobody needs to see them.
fn temp_file() -> io::Result<tempfile::NamedTempFile> {
    let builder = {
        let mut builder = tempfile::Builder::new();
        builder.prefix("katna-signature-");
        builder
    };
    match std::env::var_os("XDG_RUNTIME_DIR") {
        Some(dir) if !dir.is_empty() => builder.tempfile_in(dir).or_else(|_| builder.tempfile()),
        _ => builder.tempfile(),
    }
}

/// The address in a user ID: `Name <a@b>`, `<a@b>` or a bare `a@b`.
fn email_of(uid: &str) -> Option<String> {
    let email = match (uid.rfind('<'), uid.rfind('>')) {
        (Some(open), Some(close)) if open < close => &uid[open + 1..close],
        _ => uid,
    };
    let email = email.trim();
    (email.contains('@') && !email.contains(char::is_whitespace)).then(|| email.to_lowercase())
}

/// The `CN` of a distinguished name, in RFC 4514 (`O=x,CN=y`) or gpgsm's
/// status form (`/CN=y/O=x`).
fn common_name(dn: &str) -> Option<String> {
    let separator = if dn.starts_with('/') { '/' } else { ',' };
    dn.split(separator)
        .filter_map(|rdn| rdn.trim().split_once('='))
        .find(|(kind, _)| kind.eq_ignore_ascii_case("CN"))
        .map(|(_, value)| value.replace('\\', ""))
        .filter(|name| !name.is_empty())
}

/// Undoes the `\xHH` escaping of colon listings.
fn unescape_colons(field: &str) -> String {
    let bytes = field.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\'
            && bytes.get(i + 1) == Some(&b'x')
            && let Some(byte) = bytes
                .get(i + 2..i + 4)
                .and_then(|hex| std::str::from_utf8(hex).ok())
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
        {
            out.push(byte);
            i += 4;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emails_and_names() {
        assert_eq!(
            email_of("Ada Lovelace <Ada@Example.org>").as_deref(),
            Some("ada@example.org")
        );
        assert_eq!(
            email_of("<bob@example.org>").as_deref(),
            Some("bob@example.org")
        );
        assert_eq!(
            email_of("bob@example.org").as_deref(),
            Some("bob@example.org")
        );
        assert_eq!(email_of("O=Katna Test,CN=Bob Tester"), None);
        assert_eq!(
            common_name("O=Katna Test,CN=Bob Tester").as_deref(),
            Some("Bob Tester")
        );
        assert_eq!(
            common_name("/CN=Bob Tester/O=Katna Test").as_deref(),
            Some("Bob Tester")
        );
        assert_eq!(unescape_colons("a\\x3ab"), "a:b");
    }
}
