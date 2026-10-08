// SPDX-License-Identifier: GPL-3.0-or-later

//! The OpenPGP Web Key Directory (draft-koch-openpgp-webkey-service): a
//! mail domain publishes its users' keys over HTTPS, at a URL made from
//! the address. Only the address's own domain is asked, so nobody else
//! learns who the user writes to. The daemon does the fetching.

/// The two places a key for `address` may be, in the order to try them:
/// the "advanced" method (a host of its own, `openpgpkey.<domain>`), then
/// the "direct" one. `None` for something that is not an address.
pub fn urls(address: &str) -> Option<[String; 2]> {
    let (local, domain) = address.trim().rsplit_once('@')?;
    let domain = domain.trim_end_matches('.').to_ascii_lowercase();
    let valid_domain = domain.contains('.')
        && domain.split('.').all(|label| {
            !label.is_empty()
                && label
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        });
    if local.is_empty() || !valid_domain {
        return None;
    }
    let hash = hash(local);
    let local = percent_encode(local);
    Some([
        format!("https://openpgpkey.{domain}/.well-known/openpgpkey/{domain}/hu/{hash}?l={local}"),
        format!("https://{domain}/.well-known/openpgpkey/hu/{hash}?l={local}"),
    ])
}

/// The z-base-32 SHA-1 of the lowercased local part.
fn hash(local: &str) -> String {
    let digest = ring::digest::digest(
        &ring::digest::SHA1_FOR_LEGACY_USE_ONLY,
        local.to_lowercase().as_bytes(),
    );
    zbase32(digest.as_ref())
}

fn zbase32(data: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ybndrfg8ejkmcpqxot1uwisza345h769";
    let mut out = String::with_capacity(data.len().div_ceil(5) * 8);
    let mut buffer: u32 = 0;
    let mut bits = 0;
    for &byte in data {
        buffer = (buffer << 8) | u32::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(char::from(ALPHABET[((buffer >> bits) & 31) as usize]));
        }
    }
    if bits > 0 {
        out.push(char::from(ALPHABET[((buffer << (5 - bits)) & 31) as usize]));
    }
    out
}

fn percent_encode(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                char::from(b).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_example() {
        // The example in draft-koch-openpgp-webkey-service, section 3.1.
        assert_eq!(hash("Joe.Doe"), "iy9q119eutrkn8s1mk4r39qejnbu3n5q");
        let [advanced, direct] = urls("Joe.Doe@Example.ORG").unwrap();
        assert_eq!(
            advanced,
            "https://openpgpkey.example.org/.well-known/openpgpkey/example.org/hu/iy9q119eutrkn8s1mk4r39qejnbu3n5q?l=Joe.Doe"
        );
        assert_eq!(
            direct,
            "https://example.org/.well-known/openpgpkey/hu/iy9q119eutrkn8s1mk4r39qejnbu3n5q?l=Joe.Doe"
        );
    }

    #[test]
    fn not_addresses() {
        assert!(urls("nobody").is_none());
        assert!(urls("@example.org").is_none());
        assert!(urls("a@localhost").is_none());
        assert!(urls("a@exa/mple.org").is_none());
        assert!(urls("a@exa mple.org").is_none());
        assert_eq!(
            urls("a+b c@example.org").unwrap()[1]
                .rsplit_once("?l=")
                .unwrap()
                .1,
            "a%2Bb%20c"
        );
    }
}
