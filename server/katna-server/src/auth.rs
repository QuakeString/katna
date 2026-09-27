// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna accounts: who a request comes from, password hashes, emailed
//! codes and address checks.
//!
//! Every install gets a token when it registers (`POST /api/v1/installs`).
//! Signing in ties the install to a Katna account; it is then one of the
//! account's devices. A route for a server feature takes [`SignedIn`] as an
//! argument, which turns away installs that are not signed in to an account
//! with a confirmed address.

use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use std::sync::LazyLock;

use crate::db::{InstallAuth, now_ms};
use crate::ids;
use crate::routes::{ApiError, AppState};

/// Shortest password accepted.
pub const MIN_PASSWORD: usize = 8;

/// Longest password accepted (hashing is slow on purpose; keep it bounded).
pub const MAX_PASSWORD: usize = 256;

/// Longest address accepted.
pub const MAX_EMAIL: usize = 254;

/// Longest device name kept.
pub const MAX_DEVICE_NAME: usize = 64;

/// How long an emailed code works.
pub const CODE_LIFETIME_MS: i64 = 30 * 60 * 1000;

/// The install a request's token belongs to, signed in to an account with
/// a confirmed address. Server features take this as an argument.
pub struct SignedIn {
    /// The install (device).
    pub install: String,
    /// Its Katna account.
    pub account: String,
}

/// The install a request's token belongs to, signed in to an account
/// whose address may not be confirmed yet (account routes).
pub struct Member {
    /// The install (device).
    pub install: String,
    /// Its Katna account.
    pub account: String,
    /// Whether the account's address is confirmed.
    pub verified: bool,
}

/// The install a request's token belongs to, signed in or not.
pub(crate) async fn install_auth(parts: &Parts, state: &AppState) -> Result<InstallAuth, ApiError> {
    let token = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| ids::is_valid_token(token))
        .ok_or(ApiError::Unauthorized)?;
    state
        .db()
        .install_auth(&ids::token_hash(token), now_ms())
        .await?
        .ok_or(ApiError::Unauthorized)
}

impl FromRequestParts<AppState> for Member {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        let install = install_auth(parts, state).await?;
        let account = install.account.ok_or(ApiError::SignInNeeded)?;
        Ok(Member {
            install: install.id,
            account,
            verified: install.verified,
        })
    }
}

impl FromRequestParts<AppState> for SignedIn {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        let member = Member::from_request_parts(parts, state).await?;
        if !member.verified {
            return Err(ApiError::NotVerified);
        }
        Ok(SignedIn {
            install: member.install,
            account: member.account,
        })
    }
}

/// The address as stored: trimmed and lowercased, or `None` when it does
/// not look like an address.
pub fn normalize_email(email: &str) -> Option<String> {
    let email = email.trim().to_lowercase();
    let (local, domain) = email.rsplit_once('@')?;
    let ok = !local.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && email.len() <= MAX_EMAIL
        && !email
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || matches!(c, '<' | '>' | ',' | ';'));
    ok.then_some(email)
}

/// Checks a new password's length.
pub fn check_password(password: &str) -> Result<(), ApiError> {
    let chars = password.chars().count();
    if chars < MIN_PASSWORD {
        return Err(ApiError::Invalid(
            "short_password",
            "the password needs at least 8 characters",
        ));
    }
    if password.len() > MAX_PASSWORD {
        return Err(ApiError::Invalid(
            "long_password",
            "the password is too long",
        ));
    }
    Ok(())
}

/// A device name as stored: trimmed, without control characters, at most
/// [`MAX_DEVICE_NAME`] characters.
pub fn clean_device_name(name: &str) -> String {
    name.trim()
        .chars()
        .filter(|c| !c.is_control())
        .take(MAX_DEVICE_NAME)
        .collect()
}

/// Hashes a password with Argon2id (on a blocking thread: it is slow on
/// purpose).
pub async fn hash_password(password: String) -> Result<String, ApiError> {
    tokio::task::spawn_blocking(move || {
        let mut salt = [0u8; 16];
        getrandom::fill(&mut salt).expect("the operating system's random source failed");
        let salt = SaltString::encode_b64(&salt).map_err(|_| ApiError::Hash)?;
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| ApiError::Hash)
    })
    .await
    .map_err(|_| ApiError::Hash)?
}

/// Checks a password against a stored hash (on a blocking thread).
pub async fn verify_password(password: String, hash: String) -> bool {
    tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash)
            .map(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok()
            })
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false)
}

/// A hash to check passwords against when there is no account, so a
/// wrong address takes as long as a wrong password.
pub static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
    let salt = SaltString::encode_b64(&[7u8; 16]).expect("16 bytes make a salt");
    Argon2::default()
        .hash_password(ids::new_token().as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .unwrap_or_default()
});

/// A new six-digit code.
pub fn new_code() -> String {
    let mut bytes = [0u8; 4];
    getrandom::fill(&mut bytes).expect("the operating system's random source failed");
    format!("{:06}", u32::from_le_bytes(bytes) % 1_000_000)
}

/// The code as typed: digits only, so spaces and dashes don't matter.
pub fn clean_code(code: &str) -> String {
    code.chars().filter(char::is_ascii_digit).collect()
}

/// The hash stored in place of a code, tied to its account.
pub fn code_hash(account: &str, code: &str) -> Vec<u8> {
    ids::token_hash(&format!("{account}:{code}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_addresses() {
        assert_eq!(
            normalize_email("  Me@Example.COM ").as_deref(),
            Some("me@example.com")
        );
        for bad in [
            "",
            "me",
            "@example.com",
            "me@",
            "me@localhost",
            "a b@c.d",
            "me@.com",
            "<me@x.io>",
        ] {
            assert_eq!(normalize_email(bad), None, "{bad}");
        }
    }

    #[test]
    fn checks_passwords() {
        assert!(check_password("short").is_err());
        assert!(check_password("long enough").is_ok());
        assert!(check_password(&"x".repeat(MAX_PASSWORD + 1)).is_err());
    }

    #[test]
    fn codes_are_six_digits() {
        for _ in 0..50 {
            let code = new_code();
            assert_eq!(code.len(), 6);
            assert!(code.chars().all(|c| c.is_ascii_digit()));
        }
        assert_eq!(clean_code(" 123-456 "), "123456");
        assert_ne!(code_hash("a", "123456"), code_hash("b", "123456"));
    }

    #[test]
    fn cleans_device_names() {
        assert_eq!(clean_device_name("  mzarch\n"), "mzarch");
        assert_eq!(clean_device_name(&"x".repeat(100)).len(), MAX_DEVICE_NAME);
    }

    #[tokio::test]
    async fn hashes_and_verifies() {
        let hash = hash_password("correct horse".into()).await.unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(verify_password("correct horse".into(), hash.clone()).await);
        assert!(!verify_password("wrong horse".into(), hash).await);
        assert!(!verify_password("x".into(), DUMMY_HASH.clone()).await);
    }
}
