// SPDX-License-Identifier: GPL-3.0-or-later

//! Talking to Katna Server (`katna_core::ids::TRACKING_SERVER_URL`,
//! `server/katna-server`): this install registers once and keeps the
//! token it is given in the Secret Service; every request carries it.

use std::time::Duration;

use katna_core::ids;
use katna_sync::autoconfig::http;
use katna_sync::net::Tls;
use serde::Deserialize;

use crate::secrets::Secrets;
use crate::translate::{Server, TranslateError};

/// How long one request may take. Translating a long piece takes a while
/// on a small server.
const TIMEOUT: Duration = Duration::from_secs(60);
/// Largest answer read.
const MAX_ANSWER: usize = 1024 * 1024;

/// Katna Server with this install's token.
pub struct KatnaServer {
    base: String,
    token: String,
    tls: Tls,
}

#[derive(Deserialize)]
struct Registered {
    token: String,
}

impl KatnaServer {
    /// The server with the saved token, registering this install first
    /// when there is none. [`TranslateError::Off`] when the build has no
    /// server.
    pub async fn connect(secrets: &Secrets) -> Result<Self, TranslateError> {
        let base = ids::TRACKING_SERVER_URL.trim_end_matches('/');
        if base.is_empty() {
            return Err(TranslateError::Off);
        }
        let tls = Tls::system().map_err(|err| TranslateError::Server(format!("TLS: {err}")))?;
        let saved = secrets
            .server_token()
            .await
            .map_err(|err| TranslateError::Server(err.to_string()))?;
        let token = match saved {
            Some(token) => token,
            None => {
                let (status, body) = http::exchange(
                    "POST",
                    &format!("{base}/api/v1/installs"),
                    &[],
                    &[],
                    &tls,
                    TIMEOUT,
                    MAX_ANSWER,
                )
                .await
                .map_err(|err| TranslateError::Server(err.to_string()))?;
                match status {
                    200 | 201 => {}
                    429 => return Err(TranslateError::TooMany),
                    status => {
                        return Err(TranslateError::Server(format!(
                            "registering answered {status}"
                        )));
                    }
                }
                let registered: Registered = serde_json::from_slice(&body)
                    .map_err(|err| TranslateError::Server(format!("registering: {err}")))?;
                secrets
                    .set_server_token(Some(&registered.token))
                    .await
                    .map_err(|err| TranslateError::Server(err.to_string()))?;
                tracing::info!("registered with Katna Server");
                registered.token
            }
        };
        Ok(Self {
            base: base.to_owned(),
            token,
            tls,
        })
    }

    /// Forgets the saved token, after the server said it does not know
    /// it (an install unused for months is deleted there); the next
    /// [`Self::connect`] registers again.
    pub async fn forget(secrets: &Secrets) {
        if let Err(err) = secrets.set_server_token(None).await {
            tracing::warn!(%err, "forgetting the server token");
        }
    }
}

impl Server for KatnaServer {
    async fn call(&self, method: &str, path: &str, body: &[u8]) -> Result<(u16, Vec<u8>), String> {
        let authorization = format!("Bearer {}", self.token);
        http::exchange(
            method,
            &format!("{}{path}", self.base),
            &[
                ("Authorization", &authorization),
                ("Content-Type", "application/json"),
            ],
            body,
            &self.tls,
            TIMEOUT,
            MAX_ANSWER,
        )
        .await
        .map_err(|err| err.to_string())
    }
}
