// SPDX-License-Identifier: GPL-3.0-or-later

//! Encrypted and signed mail, through the user's own GnuPG.
//! See `docs/ARCHITECTURE.md` §19.1.
//!
//! Like KMail (through GPGME), Katna runs `gpg` for OpenPGP and `gpgsm` for
//! S/MIME instead of keeping keys itself. The user's existing keyring,
//! trust settings, `gpg-agent` passphrase cache, pinentry and smartcards all
//! work unchanged, and no key material passes through Katna.
//!
//! Sending: [`protect`] signs and/or encrypts a message built for
//! sending; [`encryption_keys`] tells which recipients have a key.
//!
//! Reading: [`protection`] tells cheaply whether a raw message is encrypted
//! or signed; [`open`] decrypts and verifies it and returns a message the
//! renderer shows as usual, plus a [`Security`] report for the banner above
//! it. Plaintext is only ever kept in memory; nothing here writes it to
//! disk.
//!
//! Supported: PGP/MIME (RFC 3156), inline PGP, and S/MIME (RFC 8551):
//! enveloped, opaque-signed and detached-signed. Nested layers (signed, then
//! encrypted) are opened in turn.

mod armor;
pub mod autocrypt;
mod gnupg;
mod keys;
mod mime;
mod peers;
mod protect;
mod status;
pub mod wkd;

use mail_parser::MessageParser;

pub use armor::without_armor;
pub use gnupg::Gnupg;
pub use keys::{
    Imported, Key, KeyInfo, delete_key, encryption_keys, has_secret_key, import_keys, key_info,
    sending_standard, show_keys,
};
pub use peers::{KeySource, MAX_KEY, PeerKey, PeerKeys};
pub use protect::{Protect, ProtectError, Recipients, protect};

/// At most this many protection layers are opened, so a crafted message
/// cannot make us run GnuPG over and over.
const MAX_LAYERS: usize = 4;

/// Which standard protects a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standard {
    OpenPgp,
    Smime,
}

/// What [`protection`] found, before anything is run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protection {
    Encrypted(Standard),
    Signed(Standard),
}

/// Why a message could not be decrypted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
    /// None of the user's secret keys can open it.
    NoSecretKey,
    /// The user closed the passphrase prompt.
    Cancelled,
    /// The encrypted data is damaged or was changed in transit.
    Damaged,
    /// `gpg` or `gpgsm` is not installed.
    Unavailable,
    /// Anything else, with GnuPG's own words when there are any.
    Other(String),
}

/// The result of decrypting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decryption {
    Decrypted,
    Failed(Failure),
}

/// What a signature check said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureState {
    /// The signature matches the content.
    Good,
    /// The content was changed after it was signed, or the signature is
    /// forged.
    Bad,
    /// Good, but the signature itself has expired.
    Expired,
    /// Good, but made with a key that has expired since.
    KeyExpired,
    /// Good, but made with a key its owner has revoked.
    KeyRevoked,
    /// The signer's public key is not in the keyring.
    MissingKey,
    /// `gpg` or `gpgsm` is not installed.
    Unavailable,
    /// The check did not complete.
    Error,
}

/// How sure GnuPG is that the key belongs to the person it names: the
/// user's own trust settings (web of trust, TOFU or certificate chains).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Validity {
    /// Fully or ultimately trusted, or a valid S/MIME certificate chain.
    Full,
    Marginal,
    /// Not known: a good signature from a key nobody vouched for.
    #[default]
    Unknown,
    /// The user said never to trust this key.
    Never,
}

/// One signature and who made it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub state: SignatureState,
    /// The signer as the key names them, for example
    /// `Ada Lovelace <ada@example.org>`.
    pub signer: Option<String>,
    /// Every address on the signing key or certificate, lowercase.
    pub emails: Vec<String>,
    /// The key's fingerprint, or its key ID when the key is missing.
    pub key: Option<String>,
    /// When it was signed, in Unix seconds.
    pub created: Option<i64>,
    pub validity: Validity,
    /// The `From` address is one of [`Self::emails`]. A good signature from
    /// someone else is not a signature by the sender.
    pub from_sender: bool,
}

impl Signature {
    /// Good, made by the sender, with a key the user's GnuPG trusts: what
    /// earns the green "verified" mark.
    pub fn verified(&self) -> bool {
        self.state == SignatureState::Good && self.validity == Validity::Full && self.from_sender
    }
}

/// What protected a message, and whether it held.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Security {
    /// The standard of the outermost layer.
    pub standard: Standard,
    /// The protection covers the whole message. When `false` only a part
    /// is encrypted or signed (a mailing list often adds an unsigned
    /// footer), and the rest could have been written by anyone.
    pub whole: bool,
    /// `None` when the message was only signed.
    pub decryption: Option<Decryption>,
    /// The signatures, outermost layer first. Empty when unsigned.
    pub signatures: Vec<Signature>,
}

impl Security {
    pub fn encrypted(&self) -> bool {
        self.decryption.is_some()
    }

    pub fn decrypted(&self) -> bool {
        self.decryption == Some(Decryption::Decrypted)
    }
}

/// An opened message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opened {
    /// The message to show: the original headers with the decrypted or
    /// verified content in place of the protected part, and signature
    /// parts removed. When decryption failed, the original message.
    pub raw: Vec<u8>,
    pub security: Security,
}

/// Whether `raw` is encrypted or signed, without running GnuPG. Cheap
/// enough to call for every message shown.
pub fn protection(raw: &[u8]) -> Option<Protection> {
    let message = MessageParser::default().parse(raw)?;
    mime::find(&message).map(|found| found.protection())
}

/// Decrypts and verifies `raw`. Returns `None` when it is neither encrypted
/// nor signed.
///
/// Runs `gpg` or `gpgsm` and waits for them, which includes waiting for the
/// user to type a passphrase into pinentry: never call it on the UI
/// thread.
pub fn open(raw: &[u8], gnupg: &Gnupg) -> Option<Opened> {
    let message = MessageParser::default().parse(raw)?;
    let sender = message
        .from()
        .and_then(|from| from.first())
        .and_then(|addr| addr.address.as_deref())
        .map(|email| email.trim().to_lowercase());
    let first = mime::find(&message)?;

    let mut security = Security {
        standard: first.standard(),
        whole: true,
        decryption: None,
        signatures: Vec::new(),
    };
    let mut current = raw.to_vec();
    let mut found = Some(first);
    for _ in 0..MAX_LAYERS {
        let Some(layer) = found.take() else {
            break;
        };
        let step = mime::open_layer(&current, layer, gnupg, sender.as_deref());
        security.whole &= step.whole;
        if let Some(decryption) = step.decryption {
            let failed = decryption != Decryption::Decrypted;
            // The outermost encryption is the one that matters.
            security.decryption.get_or_insert(decryption);
            if failed {
                break;
            }
        }
        security.signatures.extend(step.signatures);
        let Some(next) = step.raw else {
            break;
        };
        current = next;
        found = MessageParser::default()
            .parse(&current)
            .and_then(|message| mime::find(&message));
    }
    Some(Opened {
        raw: current,
        security,
    })
}
