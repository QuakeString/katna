// SPDX-License-Identifier: GPL-3.0-or-later

//! Signing in to Google and Microsoft accounts with OAuth2
//! (`docs/ARCHITECTURE.md` §6.4): the browser shows the provider's page,
//! the daemon trades the answer for tokens and keeps the refresh token in
//! the Secret Service. Accounts that sign in this way log in with SASL
//! XOAUTH2 and fresh access tokens from a [`TokenSource`].

use std::{
    path::PathBuf,
    sync::{Arc, Weak, atomic::Ordering},
    time::Duration,
};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::{AccountId, AccountKind, AccountSettings, OAuthProvider, Paths, Server};
use katna_i18n::tr;
use katna_sync::{
    Credentials,
    autoconfig::http,
    net::Tls,
    oauth::{Grant, Pages, Provider, SignIn, TokenSource},
};

use super::{CommandError, Daemon, Notice, check_credentials};

/// How long the browser may take before the sign-in gives up.
const SIGN_IN_TIMEOUT: Duration = Duration::from_secs(10 * 60);

/// Largest account picture taken from a provider.
const MAX_PICTURE: usize = 2 * 1024 * 1024;

/// Where the picture of `account` from its provider is kept. Katna Mail
/// shows it when the user picked none (`account-pictures/<id>`).
pub(crate) fn provider_picture(paths: &Paths, account: AccountId) -> PathBuf {
    paths
        .account_pictures_dir()
        .join("provider")
        .join(account.to_string())
}

impl Daemon {
    /// How `account` logs in to `server`: its password, or access tokens
    /// from its OAuth2 sign-in.
    pub(super) async fn credentials(
        &self,
        account: AccountId,
        server: &Server,
        settings: &AccountSettings,
    ) -> Result<Credentials, String> {
        let Some(provider) = settings.oauth else {
            let password = self
                .secrets
                .password(account)
                .await
                .map_err(|err| err.to_string())?
                .ok_or("no password saved; set one with katnactl password")?;
            return Ok(Credentials::new(server.username.clone(), password));
        };
        let tokens = self.oauth_tokens(account, provider).await?;
        Ok(Credentials::oauth2(server.username.clone(), tokens))
    }

    /// The access tokens of `account`, which signs in to `provider`:
    /// shared by its connections and its Drive.
    pub(super) async fn oauth_tokens(
        &self,
        account: AccountId,
        provider: OAuthProvider,
    ) -> Result<Arc<TokenSource>, String> {
        if let Some(tokens) = self.tokens.lock().unwrap().get(&account).cloned() {
            return Ok(tokens);
        }
        let refresh = self
            .secrets
            .password(account)
            .await
            .map_err(|err| err.to_string())?
            .ok_or_else(|| format!("{} asks to sign in again", provider.name()))?;
        let tokens = Arc::new(self.token_source(account, provider, refresh, None)?);
        Ok(self
            .tokens
            .lock()
            .unwrap()
            .entry(account)
            .or_insert(tokens)
            .clone())
    }

    /// A token source for `account` that saves refresh tokens the
    /// provider replaces; with `grant`, it starts with its access token.
    pub(super) fn token_source(
        &self,
        account: AccountId,
        provider: OAuthProvider,
        refresh_token: String,
        grant: Option<&Grant>,
    ) -> Result<TokenSource, String> {
        let tls = Tls::system().map_err(|err| format!("TLS setup: {err}"))?;
        let config = Provider::new(provider, tls).ok_or_else(|| {
            format!(
                "this build of Katna cannot sign in to {} accounts",
                provider.name()
            )
        })?;
        let rotated = self.rotated.0.clone();
        let tokens = TokenSource::new(
            config,
            refresh_token,
            Some(Box::new(move |token| {
                let _ = rotated.try_send((account, token));
            })),
        );
        Ok(match grant {
            Some(grant) => tokens
                .with_access_token(grant.access_token.clone(), grant.expires_in)
                .with_scope(grant.scope.clone()),
            None => tokens,
        })
    }

    /// Signs in to `provider` in the browser. Adds the account that signed
    /// in, or, when an account has its address (always `account`, when
    /// given), signs that one in again. `hint` fills in the address on the
    /// provider's page. Returns the account.
    pub async fn sign_in(
        self: &Arc<Self>,
        provider: OAuthProvider,
        account: Option<AccountId>,
        hint: &str,
    ) -> Result<AccountId, CommandError> {
        if self.closing.load(Ordering::SeqCst) {
            return Err(CommandError::Failed(tr!("daemon-deleting-data")));
        }
        let tls = Tls::system().map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        let config = Provider::new(provider, tls.clone()).ok_or_else(|| {
            CommandError::Failed(format!(
                "this build of Katna cannot sign in to {} accounts",
                provider.name()
            ))
        })?;
        let again = account.map(|id| self.account(id)).transpose()?;
        let hint = again
            .as_ref()
            .map_or(hint.trim(), |account| account.address.as_str())
            .to_owned();
        let flow = SignIn::start(&config, &hint).await.map_err(|err| {
            CommandError::Failed(format!("cannot wait for the browser's answer: {err}"))
        })?;

        // A new sign-in ends one still waiting.
        let (cancel, cancelled) = async_channel::bounded(1);
        if let Some(old) = self.signing_in.lock().unwrap().replace(cancel.clone()) {
            let _ = old.try_send(());
        }
        tracing::info!(%provider, "signing in in the browser");
        open_in_browser(flow.url()).await;
        let pages = Pages {
            signed_in: tr!("daemon-signed-in", provider = provider.name()),
            failed: tr!("daemon-sign-in-failed", provider = provider.name()),
        };
        let grant = flow
            .finish(&config, &pages)
            .or(async {
                let _ = cancelled.recv().await;
                Err(katna_sync::Error::Closed(
                    "the sign-in was cancelled".into(),
                ))
            })
            .or(async {
                async_io::Timer::after(SIGN_IN_TIMEOUT).await;
                Err(katna_sync::Error::Timeout(SIGN_IN_TIMEOUT))
            })
            .await;
        {
            let mut current = self.signing_in.lock().unwrap();
            if current.as_ref().is_some_and(|c| c.same_channel(&cancel)) {
                *current = None;
            }
        }
        let grant = grant.map_err(|err| match err {
            katna_sync::Error::Auth(message) => CommandError::AuthFailed(message),
            err => CommandError::Failed(err.to_string()),
        })?;
        let identity = grant.identity.clone().unwrap_or_default();
        let refresh = grant.refresh_token.clone().unwrap_or_default();

        let target = match again {
            Some(account) => {
                if !account.address.eq_ignore_ascii_case(&identity.email) {
                    return Err(CommandError::InvalidArgs(format!(
                        "you signed in as {}, not as {}",
                        identity.email, account.address
                    )));
                }
                Some(account)
            }
            None => self.store().accounts()?.into_iter().find(|account| {
                account.kind == AccountKind::Imap
                    && account.address.eq_ignore_ascii_case(&identity.email)
            }),
        };
        let mut settings = match &target {
            Some(account) => self
                .store()
                .account_settings(account.id)?
                .unwrap_or_default(),
            None => AccountSettings::default(),
        };
        let (imap, smtp) = Provider::servers(provider, &identity.email);
        let imap = settings.imap.clone().unwrap_or(imap);
        settings.smtp.get_or_insert(smtp);
        settings.imap = Some(imap.clone());
        settings.oauth = Some(provider);

        let check = TokenSource::new(config, refresh.clone(), None)
            .with_access_token(grant.access_token.clone(), grant.expires_in);
        let credentials = Credentials::oauth2(imap.username.clone(), Arc::new(check));
        check_credentials(AccountKind::Imap, &imap, credentials).await?;

        let id = match target {
            None => {
                self.save_account(
                    AccountKind::Imap,
                    "",
                    identity.email.clone(),
                    settings,
                    refresh,
                    Some(&grant),
                )
                .await?
            }
            Some(account) => {
                self.secrets
                    .set_password(account.id, &account.address, &refresh)
                    .await?;
                self.store().set_account_settings(account.id, &settings)?;
                let tokens = self
                    .token_source(account.id, provider, refresh, Some(&grant))
                    .map_err(CommandError::Failed)?;
                self.tokens
                    .lock()
                    .unwrap()
                    .insert(account.id, Arc::new(tokens));
                tracing::info!(account = %account.id, %provider, "signed in again");
                self.start_account(&account).await;
                self.wake_calendars();
                let _ = self.notices.try_send(Notice::AccountsChanged);
                account.id
            }
        };
        if !identity.picture.is_empty() {
            smol::spawn(save_picture(
                Arc::downgrade(self),
                id,
                identity.picture,
                tls,
            ))
            .detach();
        }
        Ok(id)
    }

    /// Ends the browser sign-in under way. Returns whether there was one.
    pub fn cancel_sign_in(&self) -> bool {
        self.signing_in
            .lock()
            .unwrap()
            .take()
            .is_some_and(|cancel| cancel.try_send(()).is_ok())
    }
}

/// Saves refresh tokens the providers replaced.
pub(super) async fn save_rotated(daemon: Weak<Daemon>, tokens: Receiver<(AccountId, String)>) {
    while let Ok((id, token)) = tokens.recv().await {
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        let Some(account) = daemon.account(id).ok() else {
            continue;
        };
        match daemon
            .secrets
            .set_password(id, &account.address, &token)
            .await
        {
            Ok(()) => tracing::debug!(account = %id, "saved the new refresh token"),
            Err(err) => tracing::warn!(account = %id, %err, "could not save the new refresh token"),
        }
    }
}

/// Downloads the account's picture from its provider, for when the user
/// picked none.
async fn save_picture(daemon: Weak<Daemon>, account: AccountId, url: String, tls: Tls) {
    // Google's picture URLs end in a size: ask for one that stays sharp.
    let url = match url.rsplit_once("=s96-c") {
        Some((base, "")) => format!("{base}=s256-c"),
        _ => url,
    };
    let picture = match http::get_limited(&url, &tls, Duration::from_secs(20), MAX_PICTURE).await {
        Ok(Some(picture)) if !picture.is_empty() => picture,
        Ok(_) => return,
        Err(err) => {
            tracing::info!(%err, "no account picture from the provider");
            return;
        }
    };
    let Some(daemon) = daemon.upgrade() else {
        return;
    };
    let path = provider_picture(&daemon.paths, account);
    // Written beside it and renamed, so Katna Mail sees the folder change.
    let part = path.with_extension("part");
    let saved = path
        .parent()
        .map_or(Ok(()), std::fs::create_dir_all)
        .and_then(|()| std::fs::write(&part, picture))
        .and_then(|()| std::fs::rename(&part, &path));
    match saved {
        Ok(()) => {
            let _ = daemon.notices.try_send(Notice::AccountsChanged);
        }
        Err(err) => tracing::warn!(path = %path.display(), %err, "cannot save the account picture"),
    }
}

/// Opens `url` in the default browser: through the desktop portal, else
/// `xdg-open`.
#[cfg(unix)]
async fn open_in_browser(url: &str) {
    let portal = async {
        let connection = zbus::Connection::session().await?;
        connection
            .call_method(
                Some("org.freedesktop.portal.Desktop"),
                "/org/freedesktop/portal/desktop",
                Some("org.freedesktop.portal.OpenURI"),
                "OpenURI",
                &(
                    "",
                    url,
                    std::collections::HashMap::<&str, zbus::zvariant::Value<'_>>::new(),
                ),
            )
            .await?;
        Ok::<_, zbus::Error>(())
    };
    let Err(err) = portal.await else {
        return;
    };
    tracing::info!(%err, "no OpenURI portal; trying xdg-open");
    spawn_opener(std::process::Command::new("xdg-open").arg(url));
}

/// Opens `url` in the default browser. `rundll32` takes the URL as one
/// argument, where `cmd /c start` would split it at every `&`.
#[cfg(windows)]
async fn open_in_browser(url: &str) {
    spawn_opener(
        std::process::Command::new("rundll32")
            .arg("url.dll,FileProtocolHandler")
            .arg(url),
    );
}

fn spawn_opener(command: &mut std::process::Command) {
    match command.spawn() {
        Ok(mut child) => {
            smol::unblock(move || child.wait()).detach();
        }
        Err(err) => tracing::warn!(%err, "cannot open the browser"),
    }
}
