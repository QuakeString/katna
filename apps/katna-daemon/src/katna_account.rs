// SPDX-License-Identifier: GPL-3.0-or-later

//! This computer's Katna account on Katna Server (`server/katna-server`),
//! which every server feature needs, much like a Mailspring ID.
//!
//! The computer registers with the server once and gets a token; signing
//! in ties that token to an account. The token, with the account's
//! address, is kept in the Secret Service. The account's password is only
//! passed through to the server; mail passwords never go there.
//!
//! Server features ask [`Session::token`] for the token to send, and get
//! `sign_in` or `not_verified` errors from the server while the computer is
//! not signed in to an account with a confirmed address.

use std::time::Duration;

use katna_core::ids;
use katna_dbus::{KatnaAccount, KatnaDevice, katna_error};
use katna_sync::autoconfig::http;
use katna_sync::net::Tls;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::daemon::CommandError;
use crate::secrets::Secrets;

const TIMEOUT: Duration = Duration::from_secs(30);

/// The server: `KATNA_SERVER_URL` (for a server under test, such as
/// `http://127.0.0.1:8080`), else Katna's own.
pub fn server_url() -> String {
    std::env::var("KATNA_SERVER_URL")
        .ok()
        .filter(|url| !url.trim().is_empty())
        .unwrap_or_else(|| ids::TRACKING_SERVER_URL.to_owned())
        .trim_end_matches('/')
        .to_owned()
}

/// What is kept in the Secret Service.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Saved {
    token: String,
    /// The account signed in to; empty when signed out.
    #[serde(default)]
    email: String,
    #[serde(default)]
    verified: bool,
}

impl Saved {
    fn account(&self) -> KatnaAccount {
        KatnaAccount {
            signed_in: !self.email.is_empty(),
            email: self.email.clone(),
            verified: self.verified,
        }
    }
}

/// An answer from the server.
struct Answer {
    status: u16,
    body: Value,
}

impl Answer {
    fn ok(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// The server's error code (`sign_in`, `wrong_code`, …).
    fn code(&self) -> &str {
        self.body["code"].as_str().unwrap_or(katna_error::SERVER)
    }

    /// The error for a failed answer, with a [`katna_error`] name.
    fn error(&self) -> CommandError {
        let code = self.code();
        match code {
            katna_error::WRONG_PASSWORD => CommandError::AuthFailed(code.to_owned()),
            katna_error::EXISTS
            | katna_error::BAD_EMAIL
            | katna_error::SHORT_PASSWORD
            | katna_error::LONG_PASSWORD
            | katna_error::WRONG_CODE
            | katna_error::CODE_EXPIRED => CommandError::InvalidArgs(code.to_owned()),
            katna_error::TOO_MANY | katna_error::MAIL_FAILED | katna_error::SIGN_IN => {
                CommandError::Failed(code.to_owned())
            }
            "not_verified" => CommandError::Failed(katna_error::SIGN_IN.to_owned()),
            other => {
                tracing::warn!(status = self.status, code = other, "Katna Server refused");
                CommandError::Failed(katna_error::SERVER.to_owned())
            }
        }
    }

    /// The account in a sign-up, sign-in or reset answer.
    fn account(&self, token: String) -> Saved {
        Saved {
            token,
            email: self.body["email"].as_str().unwrap_or_default().to_owned(),
            verified: self.body["verified"].as_bool().unwrap_or(false),
        }
    }
}

/// Talks to Katna Server for this computer.
pub struct Session<'a> {
    secrets: &'a Secrets,
    base: String,
    tls: Tls,
}

impl<'a> Session<'a> {
    /// A session with the server in [`server_url`].
    pub fn new(secrets: &'a Secrets) -> Result<Self, CommandError> {
        let base = server_url();
        if base.is_empty() {
            return Err(CommandError::Failed(katna_error::OFFLINE.to_owned()));
        }
        let tls = Tls::system().map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        Ok(Self { secrets, base, tls })
    }

    async fn saved(&self) -> Result<Option<Saved>, CommandError> {
        Ok(self
            .secrets
            .server_token()
            .await?
            .and_then(|text| serde_json::from_str(&text).ok()))
    }

    async fn save(&self, saved: &Saved) -> Result<(), CommandError> {
        let text =
            serde_json::to_string(saved).map_err(|err| CommandError::Failed(err.to_string()))?;
        Ok(self.secrets.set_server_token(&text).await?)
    }

    /// The token to send for a server feature, if this computer has
    /// registered.
    pub async fn token(&self) -> Result<Option<String>, CommandError> {
        Ok(self.saved().await?.map(|saved| saved.token))
    }

    async fn call(
        &self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Option<Value>,
    ) -> Result<Answer, CommandError> {
        let url = format!("{}{path}", self.base);
        let authorization = token.map(|token| format!("Bearer {token}"));
        let headers: Vec<(&str, &str)> = authorization
            .as_deref()
            .map(|value| ("Authorization", value))
            .into_iter()
            .collect();
        let body = body.map(|body| body.to_string().into_bytes());
        let (status, bytes) =
            http::request(method, &url, &headers, body.as_deref(), &self.tls, TIMEOUT)
                .await
                .map_err(|err| {
                    tracing::info!(%err, "could not reach Katna Server");
                    CommandError::Failed(katna_error::OFFLINE.to_owned())
                })?;
        Ok(Answer {
            status,
            body: serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        })
    }

    /// The saved token, registering this computer first when there is none.
    async fn registered(&self) -> Result<Saved, CommandError> {
        if let Some(saved) = self.saved().await? {
            return Ok(saved);
        }
        self.register().await
    }

    async fn register(&self) -> Result<Saved, CommandError> {
        let answer = self.call("POST", "/api/v1/installs", None, None).await?;
        if !answer.ok() {
            return Err(answer.error());
        }
        let token = answer.body["token"]
            .as_str()
            .ok_or_else(|| CommandError::Failed(katna_error::SERVER.to_owned()))?
            .to_owned();
        let saved = Saved {
            token,
            ..Saved::default()
        };
        self.save(&saved).await?;
        Ok(saved)
    }

    /// Sends a request that signs in (sign-up, sign-in, reset), registering
    /// again if the server forgot this computer.
    async fn signing_in(&self, path: &str, mut body: Value) -> Result<KatnaAccount, CommandError> {
        body["device"] = Value::from(device_name());
        let mut saved = self.registered().await?;
        let mut answer = self
            .call("POST", path, Some(&saved.token), Some(body.clone()))
            .await?;
        if answer.code() == "unknown_install" {
            saved = self.register().await?;
            answer = self
                .call("POST", path, Some(&saved.token), Some(body))
                .await?;
        }
        if !answer.ok() {
            return Err(answer.error());
        }
        let saved = answer.account(saved.token);
        self.save(&saved).await?;
        Ok(saved.account())
    }

    /// Sends a request for a signed-in computer. When the server says it is
    /// signed out (from another computer, or the account is gone), that is
    /// saved.
    async fn member(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
    ) -> Result<(Answer, Saved), CommandError> {
        let Some(saved) = self.saved().await?.filter(|saved| !saved.email.is_empty()) else {
            return Err(CommandError::Failed(katna_error::SIGN_IN.to_owned()));
        };
        let answer = self.call(method, path, Some(&saved.token), body).await?;
        match answer.code() {
            katna_error::SIGN_IN => {
                self.save(&Saved {
                    token: saved.token,
                    ..Saved::default()
                })
                .await?;
                Err(answer.error())
            }
            "unknown_install" => {
                self.secrets.set_server_token("").await?;
                Err(CommandError::Failed(katna_error::SIGN_IN.to_owned()))
            }
            _ if !answer.ok() => Err(answer.error()),
            _ => Ok((answer, saved)),
        }
    }

    /// The account, asked of the server; what was last known when it cannot
    /// be reached.
    pub async fn account(&self) -> Result<KatnaAccount, CommandError> {
        let Some(saved) = self.saved().await?.filter(|saved| !saved.email.is_empty()) else {
            return Ok(KatnaAccount::default());
        };
        match self.member("GET", "/api/v1/account", None).await {
            Ok((answer, saved)) => {
                let fresh = answer.account(saved.token);
                self.save(&fresh).await?;
                Ok(fresh.account())
            }
            Err(CommandError::Failed(code)) if code == katna_error::SIGN_IN => {
                Ok(KatnaAccount::default())
            }
            Err(_) => Ok(saved.account()),
        }
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> Result<KatnaAccount, CommandError> {
        self.signing_in(
            "/api/v1/account",
            json!({ "email": email, "password": password }),
        )
        .await
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<KatnaAccount, CommandError> {
        self.signing_in(
            "/api/v1/account/sign-in",
            json!({ "email": email, "password": password }),
        )
        .await
    }

    pub async fn verify(&self, code: &str) -> Result<KatnaAccount, CommandError> {
        self.member(
            "POST",
            "/api/v1/account/verify",
            Some(json!({ "code": code })),
        )
        .await?;
        self.account().await
    }

    pub async fn resend_code(&self) -> Result<(), CommandError> {
        self.member("POST", "/api/v1/account/verify/resend", None)
            .await?;
        Ok(())
    }

    pub async fn sign_out(&self) -> Result<(), CommandError> {
        let Some(saved) = self.saved().await? else {
            return Ok(());
        };
        // Signed out here even when the server cannot be reached; the
        // device list still shows it until the next sign-out there.
        if let Err(err) = self.member("POST", "/api/v1/account/sign-out", None).await {
            tracing::info!(%err, "Katna Server did not take the sign-out");
        }
        self.save(&Saved {
            token: saved.token,
            ..Saved::default()
        })
        .await
    }

    pub async fn devices(&self) -> Result<Vec<KatnaDevice>, CommandError> {
        let (answer, _) = self.member("GET", "/api/v1/account/devices", None).await?;
        Ok(answer
            .body
            .as_array()
            .map(|devices| devices.iter().filter_map(device).collect())
            .unwrap_or_default())
    }

    pub async fn sign_out_device(&self, id: &str) -> Result<(), CommandError> {
        if !id.bytes().all(|b| b.is_ascii_hexdigit()) || id.is_empty() {
            return Err(CommandError::InvalidArgs("not a device".into()));
        }
        self.member("DELETE", &format!("/api/v1/account/devices/{id}"), None)
            .await?;
        Ok(())
    }

    pub async fn change_password(&self, current: &str, new: &str) -> Result<(), CommandError> {
        self.member(
            "POST",
            "/api/v1/account/password",
            Some(json!({ "current": current, "new": new })),
        )
        .await?;
        Ok(())
    }

    pub async fn reset_password(&self, email: &str) -> Result<(), CommandError> {
        let saved = self.registered().await?;
        let body = json!({ "email": email });
        let mut answer = self
            .call(
                "POST",
                "/api/v1/account/reset",
                Some(&saved.token),
                Some(body.clone()),
            )
            .await?;
        if answer.code() == "unknown_install" {
            let saved = self.register().await?;
            answer = self
                .call(
                    "POST",
                    "/api/v1/account/reset",
                    Some(&saved.token),
                    Some(body),
                )
                .await?;
        }
        if answer.ok() {
            Ok(())
        } else {
            Err(answer.error())
        }
    }

    pub async fn confirm_reset(
        &self,
        email: &str,
        code: &str,
        password: &str,
    ) -> Result<KatnaAccount, CommandError> {
        self.signing_in(
            "/api/v1/account/reset/confirm",
            json!({ "email": email, "code": code, "password": password }),
        )
        .await
    }

    pub async fn delete_account(&self, password: &str) -> Result<(), CommandError> {
        self.member(
            "POST",
            "/api/v1/account/delete",
            Some(json!({ "password": password })),
        )
        .await?;
        // The server deleted this computer's registration with the account.
        self.secrets.set_server_token("").await?;
        Ok(())
    }
}

fn device(value: &Value) -> Option<KatnaDevice> {
    Some(KatnaDevice {
        id: value["id"].as_str()?.to_owned(),
        name: value["name"].as_str().unwrap_or_default().to_owned(),
        signed_in_at: value["signed_in_at"].as_i64().unwrap_or(0) / 1000,
        last_seen: value["last_seen"].as_i64().unwrap_or(0) / 1000,
        this: value["this"].as_bool().unwrap_or(false),
    })
}

/// This computer's name for the device list.
fn device_name() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .or_else(|_| std::fs::read_to_string("/etc/hostname"))
        .map(|name| name.trim().to_owned())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_carry_the_server_code() {
        let answer = |status, code: &str| Answer {
            status,
            body: json!({ "code": code, "error": "text" }),
        };
        assert!(matches!(
            answer(401, "wrong_password").error(),
            CommandError::AuthFailed(code) if code == katna_error::WRONG_PASSWORD
        ));
        assert!(matches!(
            answer(400, "wrong_code").error(),
            CommandError::InvalidArgs(code) if code == katna_error::WRONG_CODE
        ));
        assert!(matches!(
            answer(403, "not_verified").error(),
            CommandError::Failed(code) if code == katna_error::SIGN_IN
        ));
        assert!(matches!(
            answer(500, "whatever").error(),
            CommandError::Failed(code) if code == katna_error::SERVER
        ));
    }

    #[test]
    fn reads_devices() {
        let row = json!({ "id": "ab12", "name": "mzarch", "signed_in_at": 1_700_000_000_123i64,
                          "last_seen": 1_700_000_100_000i64, "this": true });
        let device = device(&row).unwrap();
        assert_eq!(device.signed_in_at, 1_700_000_000);
        assert!(device.this);
        assert!(super::device(&json!({ "name": "x" })).is_none());
    }

    #[test]
    fn saved_state_round_trips() {
        let saved = Saved {
            token: "t".into(),
            email: "me@example.com".into(),
            verified: true,
        };
        let text = serde_json::to_string(&saved).unwrap();
        let back: Saved = serde_json::from_str(&text).unwrap();
        assert!(back.account().signed_in);
        assert!(!Saved::default().account().signed_in);
    }
}
