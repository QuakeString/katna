// SPDX-License-Identifier: GPL-3.0-or-later

//! Keys Katna found for the people the user writes with: from Autocrypt
//! headers in mail their provider authenticated, and from their domain's
//! Web Key Directory. They are kept apart from the user's GnuPG keyring,
//! which stays the user's own, and only ever used to encrypt to the one
//! address they were found for.
//!
//! One address, two files in the directory: `<name>.key` (the binary key)
//! and `<name>.txt` (`address`, `fingerprint`, `source`, `seen` and
//! `expires` lines), named after the SHA-1 of the address. Only the daemon
//! writes them.

use std::io::Write;
use std::path::{Path, PathBuf};

use crate::keys::show_keys;
use crate::{Gnupg, KeyInfo};

/// Largest key kept. A minimal Autocrypt key is well under 4 KiB; a Web
/// Key Directory may serve a key with many signatures.
pub const MAX_KEY: usize = 256 * 1024;

/// Where a key was found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeySource {
    /// An `Autocrypt` header in mail from the address.
    Autocrypt,
    /// The address's Web Key Directory.
    Wkd,
}

impl KeySource {
    fn name(self) -> &'static str {
        match self {
            Self::Autocrypt => "autocrypt",
            Self::Wkd => "wkd",
        }
    }
}

/// A key Katna keeps for one address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerKey {
    /// Lowercase.
    pub address: String,
    pub fingerprint: String,
    pub source: KeySource,
    /// When it was found: the date of the mail that carried it, or of the
    /// lookup. Unix seconds.
    pub seen: i64,
    pub expires: Option<i64>,
}

/// The directory of keys Katna found.
#[derive(Debug, Clone)]
pub struct PeerKeys {
    dir: PathBuf,
}

impl PeerKeys {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// The key kept for `address`, with the file it is in, unless it has
    /// expired since.
    pub fn get(&self, address: &str) -> Option<(PeerKey, PathBuf)> {
        let address = address.trim().to_lowercase();
        let (meta, key) = self.files(&address);
        let peer = parse_meta(&std::fs::read_to_string(meta).ok()?)?;
        if peer.address != address || !key.is_file() {
            return None;
        }
        if peer.expires.is_some_and(|expires| expires <= now()) {
            return None;
        }
        Some((peer, key))
    }

    /// Keeps `data` as the key of `address` when it is one usable key for
    /// exactly that address. An Autocrypt key seen in older mail than the
    /// one kept does not replace it; a key from the Web Key Directory
    /// always does, being the domain's own word. Returns the key kept
    /// for the address afterwards, or why `data` was refused.
    pub fn remember(
        &self,
        gnupg: &Gnupg,
        address: &str,
        data: &[u8],
        source: KeySource,
        seen: i64,
    ) -> Result<PeerKey, String> {
        let address = address.trim().to_lowercase();
        if data.len() > MAX_KEY {
            return Err("the key is too large".into());
        }
        let keys = show_keys(gnupg, data);
        let info = usable_key(&keys, &address)?;
        if source == KeySource::Autocrypt
            && let Some((kept, _)) = self.get(&address)
            && kept.seen > seen
        {
            return Ok(kept);
        }
        let peer = PeerKey {
            address: address.clone(),
            fingerprint: info.fingerprint.clone(),
            source,
            seen,
            expires: info.expires,
        };
        private_dir(&self.dir).map_err(|err| err.to_string())?;
        let (meta, key) = self.files(&address);
        write_atomic(&key, data).map_err(|err| err.to_string())?;
        write_atomic(&meta, format_meta(&peer).as_bytes()).map_err(|err| err.to_string())?;
        Ok(peer)
    }

    /// Forgets the key of `address`.
    pub fn forget(&self, address: &str) {
        let (meta, key) = self.files(&address.trim().to_lowercase());
        let _ = std::fs::remove_file(meta);
        let _ = std::fs::remove_file(key);
    }

    fn files(&self, address: &str) -> (PathBuf, PathBuf) {
        let digest =
            ring::digest::digest(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY, address.as_bytes());
        let name: String = digest.as_ref().iter().map(|b| format!("{b:02x}")).collect();
        (
            self.dir.join(format!("{name}.txt")),
            self.dir.join(format!("{name}.key")),
        )
    }
}

/// The one key in `keys` that can encrypt to `address`, or why there is
/// none.
pub(crate) fn usable_key<'a>(keys: &'a [KeyInfo], address: &str) -> Result<&'a KeyInfo, String> {
    let [key] = keys else {
        return Err(if keys.is_empty() {
            "no key was found in it".into()
        } else {
            "it holds more than one key".into()
        });
    };
    if !key.emails.iter().any(|email| email == address) {
        return Err(format!("the key is not for {address}"));
    }
    if key.revoked {
        return Err("the key was revoked".into());
    }
    if key.expired {
        return Err("the key has expired".into());
    }
    if !key.can_encrypt {
        return Err("the key cannot encrypt".into());
    }
    Ok(key)
}

fn format_meta(peer: &PeerKey) -> String {
    let mut text = format!(
        "address={}\nfingerprint={}\nsource={}\nseen={}\n",
        peer.address,
        peer.fingerprint,
        peer.source.name(),
        peer.seen
    );
    if let Some(expires) = peer.expires {
        text.push_str(&format!("expires={expires}\n"));
    }
    text
}

fn parse_meta(text: &str) -> Option<PeerKey> {
    let field = |name: &str| {
        text.lines()
            .find_map(|line| line.strip_prefix(name)?.strip_prefix('='))
    };
    let fingerprint = field("fingerprint")?;
    if fingerprint.is_empty() || !fingerprint.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some(PeerKey {
        address: field("address")?.to_owned(),
        fingerprint: fingerprint.to_owned(),
        source: match field("source")? {
            "autocrypt" => KeySource::Autocrypt,
            "wkd" => KeySource::Wkd,
            _ => return None,
        },
        seen: field("seen")?.parse().ok()?,
        expires: field("expires").and_then(|expires| expires.parse().ok()),
    })
}

/// Creates the directory, private: the keys are public, but who the user
/// writes to is not.
fn private_dir(dir: &Path) -> std::io::Result<()> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
    builder.create(dir)
}

/// Writes `path` whole or not at all, so a reader never sees half a key.
fn write_atomic(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut file = tempfile::NamedTempFile::new_in(dir)?;
    file.write_all(data)?;
    file.persist(path).map_err(|err| err.error)?;
    Ok(())
}

fn now() -> i64 {
    jiff::Timestamp::now().as_second()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_round_trip() {
        let peer = PeerKey {
            address: "ada@example.org".into(),
            fingerprint: "80C06041F2DF004EA1DE3D0B044D0F0EBC2DE42E".into(),
            source: KeySource::Wkd,
            seen: 1790431381,
            expires: Some(1886048730),
        };
        assert_eq!(parse_meta(&format_meta(&peer)), Some(peer.clone()));
        let never = PeerKey {
            expires: None,
            source: KeySource::Autocrypt,
            ..peer
        };
        assert_eq!(parse_meta(&format_meta(&never)), Some(never));
        assert_eq!(
            parse_meta("address=a@b\nfingerprint=x y\nsource=wkd\nseen=1\n"),
            None
        );
    }
}
