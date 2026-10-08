// SPDX-License-Identifier: GPL-3.0-or-later

//! Keys: finding the ones to encrypt to, describing one (for the details
//! of a signature, or before importing it), and importing them.

use std::ffi::OsStr;
use std::path::PathBuf;

use crate::gnupg::{common_name, email_of, unescape_colons};
use crate::status::timestamp;
use crate::{Gnupg, Standard, Validity};

/// A key that can encrypt to one address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    /// Fingerprint, hex.
    pub fingerprint: String,
    /// The user's GnuPG vouches that the key belongs to the address (full
    /// or ultimate validity, or a valid certificate chain).
    pub verified: bool,
    /// Not in the user's keyring: a key Katna found for the address
    /// (Autocrypt or the Web Key Directory, [`crate::PeerKeys`]), used
    /// from this file.
    pub file: Option<PathBuf>,
}

/// What a key is, as GnuPG lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyInfo {
    pub standard: Standard,
    /// Fingerprint, hex.
    pub fingerprint: String,
    /// The owner's name: the first user ID's, or the certificate's CN.
    pub name: Option<String>,
    /// Every address on the key, lowercase, in the key's order.
    pub emails: Vec<String>,
    /// Unix seconds.
    pub created: Option<i64>,
    /// Unix seconds; `None` when it never expires.
    pub expires: Option<i64>,
    /// Its algorithm and size, for example `Ed25519` or `RSA 3072`.
    pub algorithm: String,
    /// It, or one of its subkeys, can encrypt.
    pub can_encrypt: bool,
    pub revoked: bool,
    pub expired: bool,
    /// How sure the user's GnuPG is that it belongs to its owner.
    pub validity: Validity,
    /// Who issued an S/MIME certificate (its distinguished name).
    pub issuer: Option<String>,
}

/// The best key for each of `emails`: usable for encryption (not revoked,
/// expired or disabled) and with a user ID of exactly that address.
/// `None` for addresses without one. A verified key wins over an
/// unverified one; among equals, the newest. An address with no key in the
/// keyring gets the one Katna found for it, when there is one
/// ([`Gnupg::with_peer_keys`], OpenPGP only).
///
/// Nothing is looked up on the network here, which would tell a key
/// server who the user writes to.
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
    let peers = (standard == Standard::OpenPgp)
        .then(|| gnupg.peer_keys())
        .flatten();
    wanted
        .iter()
        .map(|email| {
            keys.iter()
                .filter(|key| key.usable())
                .filter_map(|key| {
                    let uid = key.uids.iter().find(|uid| &uid.email == email)?;
                    Some((uid.verified, key.info.created, &key.info.fingerprint))
                })
                .max()
                .map(|(verified, _, fingerprint)| Key {
                    fingerprint: fingerprint.clone(),
                    verified,
                    file: None,
                })
                .or_else(|| {
                    let (peer, file) = peers.as_ref()?.get(email)?;
                    Some(Key {
                        fingerprint: peer.fingerprint,
                        verified: false,
                        file: Some(file),
                    })
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

/// The key or certificate `key` (a fingerprint or key ID) in the user's
/// keyring.
pub fn key_info(gnupg: &Gnupg, standard: Standard, key: &str) -> Option<KeyInfo> {
    // Only a hex fingerprint or key ID goes on the command line.
    if key.is_empty() || !key.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let args = [
        OsStr::new("--with-colons"),
        OsStr::new("--list-keys"),
        OsStr::new(key),
    ];
    let run = gnupg.run(standard, &args, b"").ok()?;
    let listing = String::from_utf8_lossy(&run.stdout);
    let key = key.to_ascii_uppercase();
    parse_listing(&listing, standard)
        .into_iter()
        .map(|listed| listed.info)
        .find(|info| info.fingerprint.ends_with(&key))
}

/// The OpenPGP keys in `data` (binary or armored), without importing them.
pub fn show_keys(gnupg: &Gnupg, data: &[u8]) -> Vec<KeyInfo> {
    let args = [OsStr::new("--with-colons"), OsStr::new("--show-keys")];
    let Ok(run) = gnupg.run(Standard::OpenPgp, &args, data) else {
        return Vec::new();
    };
    parse_listing(&String::from_utf8_lossy(&run.stdout), Standard::OpenPgp)
        .into_iter()
        .map(|listed| listed.info)
        .collect()
}

/// What [`import_keys`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Imported {
    /// Every key imported or updated, by fingerprint.
    pub fingerprints: Vec<String>,
    /// Those that were not in the keyring before: what an Undo removes.
    pub new: Vec<String>,
}

/// Imports the OpenPGP public keys in `data` into the user's keyring.
/// Secret keys in `data` are refused, so a mail cannot add one.
pub fn import_keys(gnupg: &Gnupg, data: &[u8]) -> Result<Imported, String> {
    // A secret key never comes from mail, armored or binary.
    let shown = gnupg
        .run(
            Standard::OpenPgp,
            &[OsStr::new("--with-colons"), OsStr::new("--show-keys")],
            data,
        )
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => "Install GnuPG to import keys.".to_owned(),
            _ => err.to_string(),
        })?;
    if String::from_utf8_lossy(&shown.stdout)
        .lines()
        .any(|line| line.starts_with("sec:"))
    {
        return Err("This is a secret key; Katna only imports public keys.".to_owned());
    }
    // `import-clean` drops signatures nobody can check and unusable parts.
    let args = [
        OsStr::new("--import-options"),
        OsStr::new("import-clean"),
        OsStr::new("--import"),
    ];
    let run = gnupg
        .run(Standard::OpenPgp, &args, data)
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => "Install GnuPG to import keys.".to_owned(),
            _ => err.to_string(),
        })?;
    if run.status.imported.is_empty() {
        return Err(run
            .status
            .message
            .clone()
            .unwrap_or_else(|| "No key was found in it.".to_owned()));
    }
    let mut imported = Imported::default();
    for (flags, fingerprint) in &run.status.imported {
        if flags & 16 != 0 {
            // A secret key: never kept.
            continue;
        }
        if !imported.fingerprints.contains(fingerprint) {
            imported.fingerprints.push(fingerprint.clone());
        }
        if flags & 1 != 0 && !imported.new.contains(fingerprint) {
            imported.new.push(fingerprint.clone());
        }
    }
    Ok(imported)
}

/// Removes the public key `fingerprint` from the user's keyring: an Undo
/// right after [`import_keys`] added it. Never removes a key the user has
/// the secret part of.
pub fn delete_key(gnupg: &Gnupg, fingerprint: &str) -> bool {
    if fingerprint.len() < 32 || !fingerprint.bytes().all(|b| b.is_ascii_hexdigit()) {
        return false;
    }
    let args = [
        OsStr::new("--yes"),
        OsStr::new("--delete-keys"),
        OsStr::new(fingerprint),
    ];
    gnupg
        .run(Standard::OpenPgp, &args, b"")
        .is_ok_and(|run| run.success)
}

pub(crate) struct ListedKey {
    pub(crate) info: KeyInfo,
    /// The primary key's (or certificate's) validity letter.
    validity: String,
    disabled: bool,
    uids: Vec<ListedUid>,
}

impl ListedKey {
    pub(crate) fn usable(&self) -> bool {
        self.info.can_encrypt && !self.disabled && !self.info.revoked && !self.info.expired
    }
}

struct ListedUid {
    email: String,
    verified: bool,
}

/// Reads a `--with-colons` key listing (gpg `pub`, gpgsm `crt`, and
/// `sec` and `crs` for secret keys).
pub(crate) fn parse_listing(listing: &str, standard: Standard) -> Vec<ListedKey> {
    let primary = match standard {
        Standard::OpenPgp => ["pub", "sec"],
        Standard::Smime => ["crt", "crs"],
    };
    let mut keys: Vec<ListedKey> = Vec::new();
    // The fingerprint line after the primary key, not after a subkey.
    let mut want_fingerprint = false;
    for line in listing.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        let field = |n: usize| fields.get(n).copied().unwrap_or_default();
        match field(0) {
            kind if primary.contains(&kind) => {
                let validity = field(1);
                // Capital letters are what the whole key can do; `D` marks
                // a disabled key.
                let capabilities = field(11);
                keys.push(ListedKey {
                    info: KeyInfo {
                        standard,
                        fingerprint: String::new(),
                        name: None,
                        emails: Vec::new(),
                        created: timestamp(field(5)),
                        expires: timestamp(field(6)),
                        algorithm: algorithm(field(3), field(2), field(16)),
                        can_encrypt: capabilities.contains('E'),
                        revoked: validity == "r",
                        expired: validity == "e",
                        validity: validity_of(validity),
                        issuer: (standard == Standard::Smime)
                            .then(|| unescape_colons(field(9)))
                            .filter(|issuer| !issuer.is_empty()),
                    },
                    validity: validity.to_owned(),
                    disabled: capabilities.contains('D'),
                    uids: Vec::new(),
                });
                want_fingerprint = true;
            }
            "sub" | "ssb" => want_fingerprint = false,
            "fpr" if want_fingerprint => {
                if let Some(key) = keys.last_mut() {
                    key.info.fingerprint = field(9).to_owned();
                }
                want_fingerprint = false;
            }
            "uid" => {
                let Some(key) = keys.last_mut() else {
                    continue;
                };
                let uid = unescape_colons(field(9));
                // gpg rates each user ID; gpgsm rates the certificate.
                let validity = match standard {
                    Standard::OpenPgp => field(1).to_owned(),
                    Standard::Smime => key.validity.clone(),
                };
                if matches!(validity.as_str(), "r" | "e" | "i" | "d") {
                    continue;
                }
                let email = email_of(&uid);
                if key.info.name.is_none() {
                    key.info.name = match standard {
                        Standard::OpenPgp => name_of(&uid),
                        Standard::Smime => common_name(&uid),
                    };
                }
                let Some(email) = email else {
                    continue;
                };
                if !key.info.emails.contains(&email) {
                    key.info.emails.push(email.clone());
                }
                key.uids.push(ListedUid {
                    email,
                    verified: matches!(validity.as_str(), "f" | "u"),
                });
            }
            _ => {}
        }
    }
    keys.retain(|key| !key.info.fingerprint.is_empty());
    keys
}

/// The name part of `Name (comment) <a@b>`.
fn name_of(uid: &str) -> Option<String> {
    let name = uid.split('<').next().unwrap_or_default();
    let name = match name.find('(') {
        Some(open) => &name[..open],
        None => name,
    };
    let name = name.trim();
    (!name.is_empty() && !name.contains('@')).then(|| name.to_owned())
}

fn validity_of(letter: &str) -> Validity {
    match letter {
        "f" | "u" => Validity::Full,
        "m" => Validity::Marginal,
        "n" => Validity::Never,
        _ => Validity::Unknown,
    }
}

/// `Ed25519`, `RSA 3072`, … from a listing's algorithm number, key length
/// and curve name.
fn algorithm(number: &str, bits: &str, curve: &str) -> String {
    let curve = match curve {
        "ed25519" | "Ed25519" => "Ed25519",
        "cv25519" | "Curve25519" => "Curve25519",
        "ed448" | "Ed448" => "Ed448",
        "cv448" | "X448" => "X448",
        "nistp256" | "NIST P-256" => "NIST P-256",
        "nistp384" | "NIST P-384" => "NIST P-384",
        "nistp521" | "NIST P-521" => "NIST P-521",
        other => other,
    };
    if !curve.is_empty() {
        return curve.to_owned();
    }
    let name = match number {
        "1" | "2" | "3" => "RSA",
        "16" | "20" => "ElGamal",
        "17" => "DSA",
        "18" => "ECDH",
        "19" => "ECDSA",
        "22" => "EdDSA",
        _ => return bits.to_owned(),
    };
    match bits {
        "" | "0" => name.to_owned(),
        bits => format!("{name} {bits}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTING: &str = "tru::1:1790431381:0:3:1:5
pub:u:255:22:044D0F0EBC2DE42E:1790431381:1886048730::u:::scESC:::::ed25519:::0:
fpr:::::::::80C06041F2DF004EA1DE3D0B044D0F0EBC2DE42E:
uid:u::::1790431381::58D5::Ada Lovelace (work) <Ada@example.org>::::::::::0:
sub:u:255:18:CFF72A8D83EA30D1:1790431381::::::e:::::cv25519::
fpr:::::::::6E21FD03A80970388BF306BACFF72A8D83EA30D1:
pub:-:3072:1:B57C30D77AC191DF:1790431381:::-:::scESC::::::::0:
fpr:::::::::437313BFFA99975FBBE6E233B57C30D77AC191DF:
uid:-::::1790431381::9A8B::Carol <carol@example.com>::::::::::0:
sub:-:255:18:F69AAAE4D9B64A33:1790431381::::::e:::::cv25519::
fpr:::::::::55766893EC64C80D4DC2D092F69AAAE4D9B64A33:
pub:r:255:22:1111111111111111:1690431381:::-:::sc:::::ed25519:::0:
fpr:::::::::1111111111111111111111111111111111111111:
uid:r::::1690431381::9A8B::Old <old@example.com>::::::::::0:
";

    #[test]
    fn gpg_listing() {
        let keys = parse_listing(LISTING, Standard::OpenPgp);
        assert_eq!(keys.len(), 3);
        let ada = &keys[0];
        assert_eq!(
            ada.info.fingerprint,
            "80C06041F2DF004EA1DE3D0B044D0F0EBC2DE42E"
        );
        assert!(ada.usable() && ada.uids[0].verified);
        assert_eq!(ada.uids[0].email, "ada@example.org");
        assert_eq!(ada.info.name.as_deref(), Some("Ada Lovelace"));
        assert_eq!(ada.info.algorithm, "Ed25519");
        assert_eq!(ada.info.created, Some(1790431381));
        assert_eq!(ada.info.expires, Some(1886048730));
        assert_eq!(ada.info.validity, Validity::Full);
        let carol = &keys[1];
        assert!(carol.usable() && !carol.uids[0].verified);
        assert_eq!(carol.info.algorithm, "RSA 3072");
        assert_eq!(carol.info.expires, None);
        assert_eq!(carol.info.validity, Validity::Unknown);
        assert!(!keys[2].usable() && keys[2].uids.is_empty() && keys[2].info.revoked);
    }

    #[test]
    fn gpgsm_listing() {
        let listing = "crt:n:2048:1:714CC0E0D157F65E:20260926T134117:20360923T134117:2B02F0::O=Katna Test,CN=Bob Tester::esES::::::23:
fpr:::::::::C32F1A750941DE846C109D32714CC0E0D157F65E:::C32F1A750941DE846C109D32714CC0E0D157F65E:
uid:n::::::::O=Katna Test,CN=Bob Tester::
uid:n::::::::<bob@example.org>::
";
        let keys = parse_listing(listing, Standard::Smime);
        let bob = &keys[0].info;
        assert_eq!(bob.name.as_deref(), Some("Bob Tester"));
        assert_eq!(bob.emails, ["bob@example.org"]);
        assert_eq!(bob.issuer.as_deref(), Some("O=Katna Test,CN=Bob Tester"));
        assert_eq!(bob.algorithm, "RSA 2048");
        assert_eq!(bob.validity, Validity::Never);
        assert!(bob.created.is_some() && bob.expires.is_some());
    }
}
