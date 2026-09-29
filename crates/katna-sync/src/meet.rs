// SPDX-License-Identifier: GPL-3.0-or-later

//! Google Meet, for "Start a video call" from a Gmail account
//! (`docs/ARCHITECTURE.md` §18.2): one call of the Meet REST API makes a
//! meeting space, and its link goes in the mail, as Gmail's own "Start an
//! instant meeting" does. Scope [`GOOGLE_MEET`]: only the spaces Katna
//! made. Accounts without it get a Jitsi link from Katna Mail instead.

use std::{sync::Arc, time::Duration};

use serde::Deserialize;

use crate::{
    Error, Result,
    autoconfig::http,
    net::Tls,
    oauth::{GOOGLE_MEET, TokenSource},
};

/// The Meet REST API's host.
pub const MEET_API: &str = "https://meet.googleapis.com";

/// How long making a space may take.
const TIMEOUT: Duration = Duration::from_secs(30);

/// One Google account's Meet.
#[derive(Clone)]
pub struct Meet {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`MEET_API`], or a server under test.
    api: String,
}

#[derive(Deserialize)]
struct Space {
    #[serde(rename = "meetingUri", default)]
    meeting_uri: String,
}

impl Meet {
    /// Google's Meet, or the server under test in `KATNA_GOOGLE_MEET_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GOOGLE_MEET_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(MEET_API))
    }

    /// Talks to `api` instead of Google, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna to make meetings.
    /// Accounts signed in before Katna asked for it have to sign in again.
    pub async fn allowed(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_MEET).await
    }

    /// Makes a meeting space open to anyone with its link; returns the
    /// link (`https://meet.google.com/abc-mnop-xyz`).
    pub async fn create(&self) -> Result<String> {
        let url = format!("{}/v2/spaces", self.api);
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let reply = http::exchange(
                "POST",
                &url,
                &[("Authorization", token.as_str())],
                Some(("application/json; charset=UTF-8", b"{}")),
                None,
                &self.tls,
                TIMEOUT,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            if reply.status == 401 || reply.status == 403 {
                return Err(Error::Auth("Google Meet refused access".into()));
            }
            if !(200..300).contains(&reply.status) {
                return Err(Error::Rejected(format!(
                    "Google Meet, making a meeting: status {}",
                    reply.status
                )));
            }
            let space: Space = serde_json::from_slice(&reply.body)
                .map_err(|err| Error::Protocol(format!("Google Meet's answer: {err}")))?;
            if !space.meeting_uri.starts_with("https://") {
                return Err(Error::Protocol("Google Meet gave no meeting link".into()));
            }
            return Ok(space.meeting_uri);
        }
    }
}

#[cfg(test)]
mod tests {
    use katna_core::OAuthProvider;

    use super::*;
    use crate::{fake_http::serve, oauth::Provider};

    fn meet(api: &str, scope: &str) -> Meet {
        let provider = Provider {
            kind: OAuthProvider::Google,
            auth_url: "https://accounts.test/auth".into(),
            token_url: "http://127.0.0.1:1/token".into(),
            client_id: "katna-test".into(),
            client_secret: "not-secret".into(),
            scope: format!("https://mail.test/ {GOOGLE_MEET}"),
            consent: String::new(),
            redirect_host: "127.0.0.1",
            tls: Tls::insecure_for_local_tests(),
        };
        let tokens = TokenSource::new(provider, "rt".into(), None)
            .with_access_token("at-1".into(), Duration::from_secs(3600))
            .with_scope(Some(scope.to_owned()));
        Meet::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
    }

    #[test]
    fn makes_a_space_and_returns_its_link() {
        let (api, seen) = serve(|_, _| {
            (
                200,
                Vec::new(),
                r#"{"name":"spaces/abc","meetingUri":"https://meet.google.com/abc-mnop-xyz","meetingCode":"abc-mnop-xyz"}"#
                    .into(),
            )
        });
        let meet = meet(&api, GOOGLE_MEET);
        assert!(smol::block_on(meet.allowed()).unwrap());
        let link = smol::block_on(meet.create()).unwrap();
        assert_eq!(link, "https://meet.google.com/abc-mnop-xyz");
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].method, "POST");
        assert_eq!(seen[0].path, "/v2/spaces");
        assert_eq!(seen[0].header("Authorization"), Some("Bearer at-1"));
    }

    #[test]
    fn a_sign_in_without_meet_is_not_allowed() {
        let meet = meet("http://127.0.0.1:1", "https://mail.test/");
        assert!(!smol::block_on(meet.allowed()).unwrap());
    }

    #[test]
    fn a_refusal_asks_to_sign_in_again() {
        let (api, _) = serve(|_, _| (403, Vec::new(), "{}".into()));
        let meet = meet(&api, GOOGLE_MEET);
        assert!(matches!(smol::block_on(meet.create()), Err(Error::Auth(_))));
    }
}
