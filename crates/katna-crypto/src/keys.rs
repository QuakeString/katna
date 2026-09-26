// SPDX-License-Identifier: GPL-3.0-or-later

//! Finding the keys to encrypt to, in the user's keyring.

use std::ffi::OsStr;

use crate::{Gnupg, Standard};

/// A key that can encrypt to one address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    /// Fingerprint, hex.
    pub fingerprint: String,
    /// The user's GnuPG vouches that the key belongs to the address (full
    /// or ultimate validity, or a valid certificate chain).
    pub verified: bool,
}

/// The best key for each of `emails`: usable for encryption (not revoked,
/// expired or disabled) and with a user ID of exactly that address.
/// `None` for addresses without one. A verified key wins over an
/// unverified one; among equals, the newest.
///
/// Only the local keyring is searched: nothing is looked up on the
/// network, which would tell a key server who the user writes to.
pub fn encryption_keys(gnupg: &Gnupg, standard: Standard, emails: &[String]) -> Vec<Option<Key>> {
    let wanted: Vec<String> = emails.iter().map(|e| e.trim().to_lowercase()).collect();
    let mut args: Vec<String> = vec!["--with-colons".into(), "--list-keys".into()];
    // `<a@b>` matches the address exactly, not as a substring.
    args.extend(wanted.iter().map(|email| format!("<{email}>")));
    let args: Vec<&OsStr> = args.iter().map(OsStr::new).collect();
    let listing = match gnupg.run(standard, &args, b"") {
        Ok(run) => String::from_utf8_lossy(&run.stdout).into_owned(),
        Err(_) => String::new(),
    };
    let keys = parse_listing(&listing, standard);
    wanted
        .iter()
        .map(|email| {
            keys.iter()
                .filter(|key| key.usable)
                .filter_map(|key| {
                    let uid = key.uids.iter().find(|uid| &uid.email == email)?;
                    Some((uid.verified, key.created, &key.fingerprint))
                })
                .max()
                .map(|(verified, _, fingerprint)| Key {
                    fingerprint: fingerprint.clone(),
                    verified,
                })
        })
        .collect()
}

/// Whether the user has a secret key (or certificate with its key) for
/// `email`, to sign with.
pub fn has_secret_key(gnupg: &Gnupg, standard: Standard, email: &str) -> bool {
    let pattern = format!("<{}>", email.trim().to_lowercase());
    let args = [
        OsStr::new("--with-colons"),
        OsStr::new("--list-secret-keys"),
        OsStr::new(&pattern),
    ];
    let Ok(run) = gnupg.run(standard, &args, b"") else {
        return false;
    };
    let primary = match standard {
        Standard::OpenPgp => "sec:",
        Standard::Smime => "crs:",
    };
    String::from_utf8_lossy(&run.stdout)
        .lines()
        .any(|line| line.starts_with(primary))
}

/// The standard to sign and encrypt with for `sender`: `preferred`, unless
/// the sender only has a key for the other one.
pub fn sending_standard(gnupg: &Gnupg, sender: &str, preferred: Standard) -> Standard {
    let other = match preferred {
        Standard::OpenPgp => Standard::Smime,
        Standard::Smime => Standard::OpenPgp,
    };
    if !has_secret_key(gnupg, preferred, sender) && has_secret_key(gnupg, other, sender) {
        other
    } else {
        preferred
    }
}

struct ListedKey {
    fingerprint: String,
    usable: bool,
    /// The primary key's (or certificate's) validity letter.
    validity: String,
    created: i64,
    uids: Vec<ListedUid>,
}

struct ListedUid {
    email: String,
    verified: bool,
}

/// Reads a `--with-colons` key listing (gpg `pub`, gpgsm `crt`).
fn parse_listing(listing: &str, standard: Standard) -> Vec<ListedKey> {
    let primary = match standard {
        Standard::OpenPgp => "pub",
        Standard::Smime => "crt",
    };
    let mut keys: Vec<ListedKey> = Vec::new();
    // The fingerprint line after the primary key, not after a subkey.
    let mut want_fingerprint = false;
    for line in listing.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        let field = |n: usize| fields.get(n).copied().unwrap_or_default();
        match field(0) {
            kind if kind == primary => {
                let validity = field(1);
                // Capital letters are what the whole key can do; `D` marks
                // a disabled key.
                let capabilities = field(11);
                keys.push(ListedKey {
                    fingerprint: String::new(),
                    usable: capabilities.contains('E')
                        && !capabilities.contains('D')
                        && !matches!(validity, "r" | "e" | "i" | "d" | "n"),
                    validity: validity.to_owned(),
                    created: field(5).parse().unwrap_or(0),
                    uids: Vec::new(),
                });
                want_fingerprint = true;
            }
            "sub" | "ssb" => want_fingerprint = false,
            "fpr" if want_fingerprint => {
                if let Some(key) = keys.last_mut() {
                    key.fingerprint = field(9).to_owned();
                }
                want_fingerprint = false;
            }
            "uid" => {
                let Some(key) = keys.last_mut() else {
                    continue;
                };
                let uid = field(9);
                let email = match (uid.rfind('<'), uid.rfind('>')) {
                    (Some(open), Some(close)) if open < close => &uid[open + 1..close],
                    _ if uid.contains('@') && !uid.contains('=') => uid,
                    _ => continue,
                };
                // gpg rates each user ID; gpgsm rates the certificate.
                let validity = match standard {
                    Standard::OpenPgp => field(1).to_owned(),
                    Standard::Smime => key.validity.clone(),
                };
                let validity = validity.as_str();
                if matches!(validity, "r" | "e" | "i" | "d") {
                    continue;
                }
                key.uids.push(ListedUid {
                    email: email.trim().to_lowercase(),
                    verified: matches!(validity, "f" | "u"),
                });
            }
            _ => {}
        }
    }
    keys.retain(|key| !key.fingerprint.is_empty());
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpg_listing() {
        let listing = "tru::1:1790431381:0:3:1:5
pub:u:255:22:044D0F0EBC2DE42E:1790431381:::u:::scESC:::::ed25519:::0:
fpr:::::::::80C06041F2DF004EA1DE3D0B044D0F0EBC2DE42E:
uid:u::::1790431381::58D5::Ada Lovelace <Ada@example.org>::::::::::0:
sub:u:255:18:CFF72A8D83EA30D1:1790431381::::::e:::::cv25519::
fpr:::::::::6E21FD03A80970388BF306BACFF72A8D83EA30D1:
pub:-:255:22:B57C30D77AC191DF:1790431381:::-:::scESC:::::ed25519:::0:
fpr:::::::::437313BFFA99975FBBE6E233B57C30D77AC191DF:
uid:-::::1790431381::9A8B::Carol <carol@example.com>::::::::::0:
sub:-:255:18:F69AAAE4D9B64A33:1790431381::::::e:::::cv25519::
fpr:::::::::55766893EC64C80D4DC2D092F69AAAE4D9B64A33:
pub:r:255:22:1111111111111111:1690431381:::-:::sc:::::ed25519:::0:
fpr:::::::::1111111111111111111111111111111111111111:
uid:r::::1690431381::9A8B::Old <old@example.com>::::::::::0:
";
        let keys = parse_listing(listing, Standard::OpenPgp);
        assert_eq!(keys.len(), 3);
        assert_eq!(
            keys[0].fingerprint,
            "80C06041F2DF004EA1DE3D0B044D0F0EBC2DE42E"
        );
        assert!(keys[0].usable && keys[0].uids[0].verified);
        assert_eq!(keys[0].uids[0].email, "ada@example.org");
        assert!(keys[1].usable && !keys[1].uids[0].verified);
        assert!(!keys[2].usable && keys[2].uids.is_empty());
    }
}
