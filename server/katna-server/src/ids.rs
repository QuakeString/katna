// SPDX-License-Identifier: GPL-3.0-or-later

//! Random identifiers and install tokens.

use sha2::{Digest, Sha256};

/// Length in hex characters of a tracking or install ID (128 bits).
pub const ID_LEN: usize = 32;

/// Length in hex characters of an install token (256 bits).
pub const TOKEN_LEN: usize = 64;

/// A new random 128-bit ID as 32 lowercase hex characters. It carries no
/// data, so it cannot be decoded or forged into someone else's.
pub fn new_id() -> String {
    random_hex::<16>()
}

/// A new random 256-bit install token as 64 lowercase hex characters.
pub fn new_token() -> String {
    random_hex::<32>()
}

/// Returns whether `id` looks like an ID from [`new_id`], so malformed
/// requests are turned away before the database is asked.
pub fn is_valid_id(id: &str) -> bool {
    id.len() == ID_LEN && id.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Returns whether `token` looks like a token from [`new_token`].
pub fn is_valid_token(token: &str) -> bool {
    token.len() == TOKEN_LEN
        && token
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// The hash the server stores in place of an install token.
pub fn token_hash(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}

fn random_hex<const N: usize>() -> String {
    let mut bytes = [0u8; N];
    getrandom::fill(&mut bytes).expect("the operating system's random source failed");
    let mut out = String::with_capacity(N * 2);
    for byte in bytes {
        out.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
        out.push(char::from_digit(u32::from(byte & 0xf), 16).unwrap_or('0'));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_random_hex() {
        let a = new_id();
        let b = new_id();
        assert!(is_valid_id(&a), "{a}");
        assert_ne!(a, b);
        assert!(is_valid_token(&new_token()));
    }

    #[test]
    fn rejects_malformed_ids() {
        assert!(!is_valid_id(""));
        assert!(!is_valid_id("../../etc/passwd"));
        assert!(!is_valid_id(&"A".repeat(ID_LEN)));
        assert!(!is_valid_id(&"a".repeat(ID_LEN + 1)));
        assert!(!is_valid_token(&new_id()));
    }

    #[test]
    fn token_hash_is_stable() {
        let token = new_token();
        assert_eq!(token_hash(&token), token_hash(&token));
        assert_eq!(token_hash(&token).len(), 32);
    }
}
