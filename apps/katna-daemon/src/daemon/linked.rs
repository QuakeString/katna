// SPDX-License-Identifier: GPL-3.0-or-later

//! Sign-ins linked to an account (`AccountSettings::linked`,
//! `docs/ARCHITECTURE.md` §6.4): a second OAuth2 sign-in for services the
//! account's mail login does not reach. Zoho's tasks and calendars are
//! one: Zoho Mail logs in with a password, since Zoho lets only its "self
//! client" apps use OAuth2 for IMAP. The refresh token sits in the Secret
//! Service apart from the password.

use std::sync::Arc;

use katna_core::{AccountId, LinkedSignIn};
use katna_sync::{
    methods::{self, Data},
    net::Tls,
    oauth::{self, Provider, TokenSource},
};

use super::{CommandError, Daemon, Notice};

impl Daemon {
    /// The access tokens of the sign-in linked to `account`, and where it
    /// signed in; `None` when there is none. An error means the user has
    /// to sign in again.
    pub async fn linked_tokens(
        &self,
        account: AccountId,
    ) -> Result<Option<(Arc<TokenSource>, LinkedSignIn)>, String> {
        let settings = self
            .store()
            .account_settings(account)
            .map_err(|err| err.to_string())?
            .unwrap_or_default();
        let Some(linked) = settings.linked else {
            return Ok(None);
        };
        if let Some(tokens) = self.linked.lock().unwrap().get(&account).cloned() {
            return Ok(Some((tokens, linked)));
        }
        let name = linked.provider.name();
        let refresh = self
            .secrets
            .linked_token(account)
            .await
            .map_err(|err| err.to_string())?
            .ok_or_else(|| format!("{name} asks to sign in again"))?;
        let tls = Tls::system().map_err(|err| format!("TLS setup: {err}"))?;
        let config = Provider::new(linked.provider, tls)
            .ok_or_else(|| format!("this build of Katna cannot sign in to {name}"))?
            .at_accounts_server(&linked.accounts_server);
        let tokens = Arc::new(TokenSource::new(config, refresh, None));
        let tokens = self
            .linked
            .lock()
            .unwrap()
            .entry(account)
            .or_insert(tokens)
            .clone();
        Ok(Some((tokens, linked)))
    }

    /// Signs `account` in to `config`'s provider in the browser and keeps
    /// the sign-in beside the account's own. Returns the account.
    pub(super) async fn link(
        self: &Arc<Self>,
        config: Provider,
        account: AccountId,
    ) -> Result<AccountId, CommandError> {
        let account = self.account(account)?;
        let settings = self
            .store()
            .account_settings(account.id)?
            .unwrap_or_default();
        // Start at the data centre that keeps the mail; Zoho sends
        // everyone else on to theirs.
        let place = settings
            .imap
            .as_ref()
            .and_then(|imap| oauth::zoho_accounts_server(&imap.host))
            .or_else(|| oauth::zoho_accounts_server(&account.address));
        let config = match place {
            Some(server) => config.at_accounts_server(server),
            None => config,
        };
        let grant = self.browser_grant(&config, &account.address).await?;
        let refresh = grant.refresh_token.clone().unwrap_or_default();
        let linked = LinkedSignIn {
            provider: config.kind,
            accounts_server: grant
                .accounts_server
                .clone()
                .unwrap_or_else(|| config.accounts_server().to_owned()),
            api_domain: grant.api_domain.clone().unwrap_or_default(),
        };
        let label = format!("{} for {}", config.kind.name(), account.address);
        self.secrets
            .set_linked_token(account.id, &label, &refresh)
            .await?;
        // Read again: the account may have changed while the browser was
        // open.
        let mut settings = self
            .store()
            .account_settings(account.id)?
            .unwrap_or_default();
        settings.linked = Some(linked.clone());
        self.store().set_account_settings(account.id, &settings)?;
        let tokens = TokenSource::new(
            config.at_accounts_server(&linked.accounts_server),
            refresh,
            None,
        )
        .with_access_token(grant.access_token.clone(), grant.expires_in)
        .with_scope(grant.scope.clone());
        self.linked
            .lock()
            .unwrap()
            .insert(account.id, Arc::new(tokens));
        tracing::info!(
            account = %account.id,
            provider = %linked.provider,
            server = %linked.accounts_server,
            "linked sign-in saved"
        );
        // The new sign-in may reach what the account could not.
        for data in [Data::Calendar, Data::Tasks] {
            methods::forget(&mut self.store(), account.id, data);
        }
        self.wake_calendars();
        self.wake_task_sync();
        let _ = self.notices.try_send(Notice::AccountsChanged);
        Ok(account.id)
    }

    /// Forgets the sign-in linked to `account`, as the account goes.
    pub(super) async fn forget_linked(&self, account: AccountId) {
        self.linked.lock().unwrap().remove(&account);
        if let Err(err) = self.secrets.delete_linked_token(account).await {
            tracing::warn!(%account, %err, "could not delete the linked sign-in");
        }
    }
}
