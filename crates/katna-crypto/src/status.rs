// SPDX-License-Identifier: GPL-3.0-or-later

//! GnuPG's machine-readable status lines (`--status-fd`, described in
//! GnuPG's `doc/DETAILS`). Only the `[GNUPG:] ` lines are read; the
//! human-readable messages around them are translated and not stable.

use crate::{Decryption, Failure, SignatureState, Validity};

const PREFIX: &str = "[GNUPG:] ";

/// `GPG_ERR_CANCELED` and `GPG_ERR_FULLY_CANCELED`: the user closed
/// pinentry.
const ERR_CANCELED: u32 = 99;
const ERR_FULLY_CANCELED: u32 = 198;
/// `GPG_ERR_NO_SECKEY`.
const ERR_NO_SECKEY: u32 = 17;
/// `ERRSIG` return code for a missing public key.
const ERRSIG_NO_PUBKEY: &str = "9";

/// One signature as the status lines describe it, before the key's
/// addresses are looked up.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RawSignature {
    pub state: Option<SignatureState>,
    /// The user ID from `GOODSIG` and friends: the primary user ID for
    /// OpenPGP, the subject DN for S/MIME.
    pub uid: Option<String>,
    /// Primary key fingerprint when known, else the key ID.
    pub key: Option<String>,
    pub created: Option<i64>,
    pub validity: Validity,
}

/// Everything one GnuPG run reported.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Status {
    /// The input was encrypted (GnuPG started decrypting).
    encrypted: bool,
    okay: bool,
    /// `BEGIN_DECRYPTION` and `END_DECRYPTION` (gpg sends both, gpgsm
    /// neither): GnuPG got to the end of the encrypted data.
    began: bool,
    ended: bool,
    failed: bool,
    recipients: usize,
    missing_secret_keys: usize,
    pub cancelled: bool,
    damaged: bool,
    no_secret_key_error: bool,
    /// The last human-readable error line, for [`Failure::Other`].
    pub message: Option<String>,
    pub signatures: Vec<RawSignature>,
    /// The hash algorithm (libgcrypt number) of a signature just made.
    pub signed_with: Option<u32>,
    /// `INV_SGNR`: the signing key was not usable.
    pub invalid_signer: bool,
    /// `INV_RECP`: recipients whose key was not usable.
    pub invalid_recipients: Vec<String>,
}

impl Status {
    pub(crate) fn parse(stderr: &[u8]) -> Self {
        let mut status = Status::default();
        for line in String::from_utf8_lossy(stderr).lines() {
            let line = line.trim_end_matches('\r');
            match line.strip_prefix(PREFIX) {
                Some(rest) => status.status_line(rest),
                None if line.starts_with("gpg: ") || line.starts_with("gpgsm: ") => {
                    status.message = Some(line.to_owned());
                }
                None => {}
            }
        }
        status
    }

    fn status_line(&mut self, line: &str) {
        let (keyword, args) = line.split_once(' ').unwrap_or((line, ""));
        let fields: Vec<&str> = args.split(' ').collect();
        match keyword {
            "ENC_TO" => {
                self.encrypted = true;
                self.recipients += 1;
            }
            "BEGIN_DECRYPTION" => {
                self.encrypted = true;
                self.began = true;
            }
            "DECRYPTION_OKAY" => self.okay = true,
            "END_DECRYPTION" => self.ended = true,
            "DECRYPTION_FAILED" => {
                self.encrypted = true;
                self.failed = true;
            }
            "NO_SECKEY" => self.missing_secret_keys += 1,
            "SIG_CREATED" => self.signed_with = fields.get(2).and_then(|a| a.parse().ok()),
            "INV_SGNR" => self.invalid_signer = true,
            "INV_RECP" => {
                if let Some(spec) = fields.get(1) {
                    self.invalid_recipients.push((*spec).to_owned());
                }
            }
            "BADMDC" | "NODATA" => self.damaged = true,
            "ERROR" | "FAILURE" => {
                let code = fields
                    .get(1)
                    .and_then(|code| code.parse::<u32>().ok())
                    .map(|code| code & 0xFFFF);
                match code {
                    Some(ERR_CANCELED | ERR_FULLY_CANCELED) => self.cancelled = true,
                    Some(ERR_NO_SECKEY) => self.no_secret_key_error = true,
                    _ => {}
                }
            }
            "NEWSIG" => self.signatures.push(RawSignature::default()),
            "GOODSIG" | "EXPSIG" | "EXPKEYSIG" | "REVKEYSIG" | "BADSIG" => {
                let state = match keyword {
                    "GOODSIG" => SignatureState::Good,
                    "EXPSIG" => SignatureState::Expired,
                    "EXPKEYSIG" => SignatureState::KeyExpired,
                    "REVKEYSIG" => SignatureState::KeyRevoked,
                    _ => SignatureState::Bad,
                };
                let (key, uid) = args.split_once(' ').unwrap_or((args, ""));
                let signature = self.signature();
                signature.state = Some(state);
                signature.key.get_or_insert_with(|| key.to_owned());
                signature.uid = Some(unescape_percent(uid)).filter(|uid| !uid.is_empty());
            }
            "ERRSIG" => {
                let signature = self.signature();
                signature.state = Some(if fields.get(5) == Some(&ERRSIG_NO_PUBKEY) {
                    SignatureState::MissingKey
                } else {
                    SignatureState::Error
                });
                // The fingerprint (newer GnuPG) is better than the key ID.
                signature.key = fields
                    .get(6)
                    .filter(|fpr| !fpr.is_empty() && **fpr != "-")
                    .or(fields.first())
                    .map(|key| (*key).to_owned());
                signature.created = fields.get(4).and_then(|time| timestamp(time));
            }
            "VALIDSIG" => {
                let signature = self.last_signature();
                // The primary key's fingerprint, when a subkey signed.
                if let Some(fpr) = fields.get(9).or(fields.first()).filter(|f| !f.is_empty()) {
                    signature.key = Some((*fpr).to_owned());
                }
                if let Some(created) = fields.get(2).and_then(|time| timestamp(time)) {
                    signature.created = Some(created);
                }
            }
            "TRUST_UNDEFINED" | "TRUST_NEVER" | "TRUST_MARGINAL" | "TRUST_FULLY"
            | "TRUST_ULTIMATE" => {
                self.last_signature().validity = match keyword {
                    "TRUST_FULLY" | "TRUST_ULTIMATE" => Validity::Full,
                    "TRUST_MARGINAL" => Validity::Marginal,
                    "TRUST_NEVER" => Validity::Never,
                    _ => Validity::Unknown,
                };
            }
            _ => {}
        }
    }

    /// The signature a `GOODSIG`-like line belongs to: the one `NEWSIG`
    /// opened, or a new one when GnuPG sent no `NEWSIG`.
    fn signature(&mut self) -> &mut RawSignature {
        if self.signatures.last().is_none_or(|s| s.state.is_some()) {
            self.signatures.push(RawSignature::default());
        }
        self.signatures.last_mut().expect("just pushed")
    }

    fn last_signature(&mut self) -> &mut RawSignature {
        if self.signatures.is_empty() {
            self.signatures.push(RawSignature::default());
        }
        self.signatures.last_mut().expect("just pushed")
    }

    /// `None` when the input was not encrypted (only signed). Decrypted
    /// only when GnuPG said so, got to the end (when it reports that), and
    /// saw nothing wrong on the way: a stream that is damaged or failed
    /// its integrity check (`BADMDC`) has failed, whatever else GnuPG
    /// printed.
    pub(crate) fn decryption(&self) -> Option<Decryption> {
        if self.okay && (self.ended || !self.began) && !self.failed && !self.damaged {
            return Some(Decryption::Decrypted);
        }
        if !self.encrypted && !self.cancelled && !self.no_secret_key_error {
            return None;
        }
        let failure = if self.cancelled {
            Failure::Cancelled
        } else if self.no_secret_key_error
            || (self.missing_secret_keys > 0 && self.missing_secret_keys >= self.recipients)
        {
            Failure::NoSecretKey
        } else if self.damaged || self.okay {
            // An integrity failure, or a stream cut short.
            Failure::Damaged
        } else {
            Failure::Other(
                self.message
                    .clone()
                    .unwrap_or_else(|| "GnuPG could not decrypt this message.".to_owned()),
            )
        };
        Some(Decryption::Failed(failure))
    }

    /// Why a run that should have decrypted something did not, when the
    /// status lines do not say it was encrypted at all (garbage input).
    pub(crate) fn failure(&self) -> Failure {
        match self.decryption() {
            Some(Decryption::Failed(failure)) => failure,
            _ if self.damaged => Failure::Damaged,
            _ => Failure::Other(
                self.message
                    .clone()
                    .unwrap_or_else(|| "GnuPG could not read this message.".to_owned()),
            ),
        }
    }
}

/// A status-line time: Unix seconds (gpg) or `20260926T134134` (gpgsm).
fn timestamp(text: &str) -> Option<i64> {
    if text.is_empty() || text == "0" {
        return None;
    }
    if text.bytes().all(|b| b.is_ascii_digit()) {
        return text.parse().ok();
    }
    let civil = jiff::civil::DateTime::strptime("%Y%m%dT%H%M%S", text).ok()?;
    Some(
        civil
            .to_zoned(jiff::tz::TimeZone::UTC)
            .ok()?
            .timestamp()
            .as_second(),
    )
}

/// Undoes the `%XX` escaping of user IDs in status lines.
fn unescape_percent(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let Some(byte) = bytes
                .get(i + 1..i + 3)
                .and_then(|hex| std::str::from_utf8(hex).ok())
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
        {
            out.push(byte);
            i += 3;
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

    fn parse(text: &str) -> Status {
        Status::parse(text.as_bytes())
    }

    #[test]
    fn signed_and_encrypted() {
        let status = parse(
            "[GNUPG:] ENC_TO B69348D074BCE18B 18 0\n\
gpg: encrypted with cv25519 key, ID B69348D074BCE18B\n\
[GNUPG:] DECRYPTION_KEY 6048 846A u\n\
[GNUPG:] BEGIN_DECRYPTION\n\
[GNUPG:] PLAINTEXT 62 1790429929 \n\
[GNUPG:] NEWSIG ada@example.org\n\
[GNUPG:] GOODSIG 658CA70CA20C0FE0 Ada L%C3%B6velace <ada@example.org>\n\
[GNUPG:] VALIDSIG 846A21AE29D952F4 2026-09-26 1790429929 0 4 0 22 10 00 846A21AE29D952F43D0E033A658CA70CA20C0FE0\n\
[GNUPG:] TRUST_ULTIMATE 0 pgp ada@example.org\n\
[GNUPG:] DECRYPTION_OKAY\n\
[GNUPG:] GOODMDC\n\
[GNUPG:] END_DECRYPTION\n",
        );
        assert_eq!(status.decryption(), Some(Decryption::Decrypted));
        assert_eq!(
            status.signatures,
            [RawSignature {
                state: Some(SignatureState::Good),
                uid: Some("Ada Lövelace <ada@example.org>".into()),
                key: Some("846A21AE29D952F43D0E033A658CA70CA20C0FE0".into()),
                created: Some(1_790_429_929),
                validity: Validity::Full,
            }]
        );
    }

    /// Status lines that say both "okay" and "damaged" are a failure: the
    /// plaintext cannot be trusted.
    #[test]
    fn damaged_after_okay_is_a_failure() {
        let damaged = |line: &str| {
            parse(&format!(
                "[GNUPG:] ENC_TO B69348D074BCE18B 18 0\n\
[GNUPG:] BEGIN_DECRYPTION\n\
[GNUPG:] DECRYPTION_OKAY\n\
{line}\
[GNUPG:] END_DECRYPTION\n"
            ))
        };
        for line in [
            "[GNUPG:] BADMDC\n",
            "[GNUPG:] NODATA 3\n",
            "[GNUPG:] DECRYPTION_FAILED\n",
        ] {
            assert!(
                matches!(damaged(line).decryption(), Some(Decryption::Failed(_))),
                "{line}"
            );
        }
        assert_eq!(
            damaged("[GNUPG:] BADMDC\n").decryption(),
            Some(Decryption::Failed(Failure::Damaged))
        );
        assert_eq!(damaged("").decryption(), Some(Decryption::Decrypted));
        // Cut short: no END_DECRYPTION.
        let cut = parse(
            "[GNUPG:] ENC_TO B69348D074BCE18B 18 0\n\
[GNUPG:] BEGIN_DECRYPTION\n\
[GNUPG:] DECRYPTION_OKAY\n",
        );
        assert_eq!(cut.decryption(), Some(Decryption::Failed(Failure::Damaged)));
    }

    #[test]
    fn no_secret_key() {
        let status = parse(
            "[GNUPG:] ENC_TO 1111111111111111 18 0\n\
[GNUPG:] NO_SECKEY 1111111111111111\n\
[GNUPG:] BEGIN_DECRYPTION\n\
[GNUPG:] DECRYPTION_FAILED\n\
[GNUPG:] END_DECRYPTION\n",
        );
        assert_eq!(
            status.decryption(),
            Some(Decryption::Failed(Failure::NoSecretKey))
        );
    }

    #[test]
    fn cancelled_passphrase() {
        let status = parse(
            "[GNUPG:] ENC_TO 1111111111111111 18 0\n\
[GNUPG:] PINENTRY_LAUNCHED 1234 qt 1.2.1 - - - - 0/0 0\n\
[GNUPG:] ERROR pkdecrypt_failed 83886179\n\
[GNUPG:] BEGIN_DECRYPTION\n\
[GNUPG:] DECRYPTION_FAILED\n",
        );
        assert_eq!(
            status.decryption(),
            Some(Decryption::Failed(Failure::Cancelled))
        );
    }

    #[test]
    fn missing_public_key_and_bad_signature() {
        let status = parse(
            "[GNUPG:] NEWSIG\n\
[GNUPG:] ERRSIG 658CA70CA20C0FE0 22 10 00 1790429929 9 846A21AE29D952F43D0E033A658CA70CA20C0FE0\n\
[GNUPG:] NO_PUBKEY 658CA70CA20C0FE0\n\
[GNUPG:] NEWSIG\n\
[GNUPG:] BADSIG 658CA70CA20C0FE0 Ada <ada@example.org>\n",
        );
        assert_eq!(status.decryption(), None);
        assert_eq!(status.signatures.len(), 2);
        assert_eq!(status.signatures[0].state, Some(SignatureState::MissingKey));
        assert_eq!(
            status.signatures[0].key.as_deref(),
            Some("846A21AE29D952F43D0E033A658CA70CA20C0FE0")
        );
        assert_eq!(status.signatures[0].created, Some(1_790_429_929));
        assert_eq!(status.signatures[1].state, Some(SignatureState::Bad));
    }

    #[test]
    fn gpgsm_signature() {
        let status = parse(
            "[GNUPG:] NEWSIG\n\
gpgsm: Signature made 2026-09-26 13:41:34 UTC\n\
[GNUPG:] GOODSIG C32F1A750941DE846C109D32714CC0E0D157F65E /CN=Bob Tester/O=Katna Test\n\
[GNUPG:] VALIDSIG C32F1A750941DE846C109D32714CC0E0D157F65E 2026-09-26 20260926T134134 20360923T134117 0 0 1 8 00\n\
[GNUPG:] TRUST_FULLY 0 shell\n",
        );
        let signature = &status.signatures[0];
        assert_eq!(signature.state, Some(SignatureState::Good));
        assert_eq!(
            signature.uid.as_deref(),
            Some("/CN=Bob Tester/O=Katna Test")
        );
        assert_eq!(signature.created, Some(1_790_430_094));
        assert_eq!(signature.validity, Validity::Full);
    }
}
