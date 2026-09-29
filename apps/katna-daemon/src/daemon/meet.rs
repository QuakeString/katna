// SPDX-License-Identifier: GPL-3.0-or-later

//! "Start a video call" (`docs/ARCHITECTURE.md` §18.2): a Google Meet
//! link from a Gmail account's own Meet. Katna Mail makes a Jitsi link
//! itself when there is none.

use katna_core::{AccountId, OAuthProvider};
use katna_sync::{Error, meet::Meet, net::Tls};

use super::{CommandError, Daemon};

impl Daemon {
    /// A new meeting link from the mail service of `account`, or an empty
    /// string when it has no meetings Katna may make (not Google, or
    /// signed in before Katna asked for Meet).
    pub async fn meeting_link(&self, account: AccountId) -> Result<String, CommandError> {
        let settings = self.store().account_settings(account)?.unwrap_or_default();
        if settings.oauth != Some(OAuthProvider::Google) {
            return Ok(String::new());
        }
        let tokens = self
            .oauth_tokens(account, OAuthProvider::Google)
            .await
            .map_err(CommandError::AuthFailed)?;
        let tls = Tls::system().map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        let meet = Meet::new(tokens, tls);
        match meet.allowed().await {
            Ok(true) => {}
            Ok(false) | Err(Error::Auth(_)) => return Ok(String::new()),
            Err(err) => return Err(CommandError::Failed(err.to_string())),
        }
        match meet.create().await {
            Ok(link) => Ok(link),
            Err(Error::Auth(message)) => {
                tracing::info!(%account, %message, "Google Meet refused; a Jitsi link instead");
                Ok(String::new())
            }
            Err(err) => Err(CommandError::Failed(err.to_string())),
        }
    }
}
