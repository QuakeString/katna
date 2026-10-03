// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail rules on the accounts' mail services (`docs/ARCHITECTURE.md`
//! §9.4): a rule runs where the account's service can run it, so it
//! works with this computer off and on the phone, else in Katna.
//!
//! - Gmail accounts signed in with Google: Gmail filters
//!   ([`katna_sync::gmail_filters`]).
//! - Other IMAP accounts whose server answers ManageSieve on port 4190 of
//!   the IMAP host: Katna's Sieve script ([`katna_sync::sieve`]). Whether
//!   it answers is kept per account and asked again after a day.
//!
//! A round runs shortly after the rules change, for every account, and
//! when an account first syncs, for it. A rule an account's service runs
//! doesn't run in Katna on that account's mail (`mail_rule_remote`); a
//! rule shows "runs on Gmail" or "runs on the server" when every account
//! it covers runs it there, else why it stays in Katna. A failed upload
//! leaves the rule in Katna, saying why, until the next round.

use std::{
    collections::{HashMap, HashSet},
    sync::Weak,
    time::Duration,
};

use async_channel::Receiver;
use katna_core::{AccountId, AccountKind, OAuthProvider};
use katna_store::rules::{RemoteRule, Rule, RuleServer, RunsNote, RunsOn};
use katna_sync::{
    Error, Security, folders,
    gmail_filters::{self, GmailSettings},
    net::Tls,
    rules_remote::{self, Verdict},
    sieve::{
        self,
        managesieve::{self, ManageSieve},
    },
};

use super::{Daemon, Notice, endpoint, unix_now};

/// How long after a change a round starts, so a burst of changes (a drag,
/// a save and its "also apply") goes out once.
const SETTLE: Duration = Duration::from_secs(3);

/// After this long, a server that had no ManageSieve is asked again.
const SIEVE_RECHECK: i64 = 24 * 60 * 60;

/// Where one account runs each of its rules: on its service, or in Katna
/// with or without a reason.
pub(super) type Placed = HashMap<i64, Result<RunsOn, Option<RunsNote>>>;

/// Runs rounds when woken: `None` for every account, else one.
pub(super) async fn run(daemon: Weak<Daemon>, wake: Receiver<Option<AccountId>>) {
    while let Ok(first) = wake.recv().await {
        smol::Timer::after(SETTLE).await;
        let mut all = first.is_none();
        let mut accounts: HashSet<AccountId> = first.into_iter().collect();
        while let Ok(more) = wake.try_recv() {
            match more {
                None => all = true,
                Some(account) => {
                    accounts.insert(account);
                }
            }
        }
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if let Err(err) = daemon.place_rules(all, &accounts).await {
            tracing::warn!(%err, "putting mail rules on the mail services");
        }
    }
}

/// `verdicts` as where each rule runs.
fn placed(verdicts: Vec<(i64, Verdict<()>)>, service: RunsOn) -> Placed {
    verdicts
        .into_iter()
        .map(|(id, v)| (id, v.map(|()| service).map_err(Some)))
        .collect()
}

/// Every rule of `account` in Katna, saying `note`.
fn all_in_katna(rules: &[Rule], account: AccountId, note: Option<RunsNote>) -> Placed {
    rules
        .iter()
        .filter(|r| r.enabled && r.covers(account))
        .map(|r| (r.id, Err(note.clone())))
        .collect()
}

impl Daemon {
    /// Has the rules of `account` (or of every account) go to the mail
    /// services soon.
    pub(super) fn place_rules_soon(&self, account: Option<AccountId>) {
        let _ = self.rules_wake.0.try_send(account);
    }

    /// One round: the rules of every mail account (with `all`) or of
    /// `accounts` go to their services, then each rule says where it runs.
    async fn place_rules(
        &self,
        all: bool,
        accounts: &HashSet<AccountId>,
    ) -> Result<(), katna_store::Error> {
        let mail: Vec<AccountId> = self
            .store()
            .accounts()?
            .into_iter()
            .filter(|a| a.kind.is_mail())
            .map(|a| a.id)
            .collect();
        let rules = self.store().rules()?;
        for &account in &mail {
            if !all && !accounts.contains(&account) {
                continue;
            }
            // Without the folders a rule names in its other accounts.
            let mine = {
                let store = self.store();
                rules
                    .iter()
                    .map(|rule| {
                        let here = katna_sync::rules::for_account(&store, rule, account)?;
                        Ok(if here.actions.is_empty() {
                            rule.clone()
                        } else {
                            here
                        })
                    })
                    .collect::<Result<Vec<_>, katna_store::Error>>()?
            };
            let placed = self.place_account(account, &mine).await?;
            self.rules_placed.lock().unwrap().insert(account, placed);
        }

        // Where each rule runs, from every account it covers.
        let known = self.rules_placed.lock().unwrap().clone();
        let mut changed = false;
        for rule in &rules {
            let mut per_account = Vec::new();
            for &account in &rule.accounts {
                let account = AccountId(account);
                let here = match known.get(&account).and_then(|p| p.get(&rule.id)) {
                    Some(placed) => placed.clone(),
                    // Not looked at since the daemon started: what was put.
                    None => match self
                        .store()
                        .remote_rules(account)?
                        .into_iter()
                        .find(|r| r.rule_id == rule.id)
                    {
                        Some(remote) => Ok(remote.runs_on),
                        None => Err(None),
                    },
                };
                per_account.push(here);
            }
            let (runs_on, note) = if rule.enabled {
                rules_remote::runs(&per_account)
            } else {
                (RunsOn::Katna, None)
            };
            if runs_on != rule.runs_on || note != rule.runs_note {
                self.store()
                    .set_rule_runs(rule.id, runs_on, note.as_ref())?;
                changed = true;
            }
        }
        if changed {
            let _ = self.notices.try_send(Notice::RulesChanged);
        }
        Ok(())
    }

    /// Puts the rules of `account` on its service, if it has one that
    /// runs rules.
    async fn place_account(
        &self,
        account: AccountId,
        rules: &[Rule],
    ) -> Result<Placed, katna_store::Error> {
        let found = self
            .store()
            .accounts()?
            .into_iter()
            .find(|a| a.id == account);
        let Some(found) = found else {
            return Ok(Placed::new());
        };
        let settings = self.store().account_settings(account)?.unwrap_or_default();
        let folders = self.store().folders(account)?;
        let imap = match (found.kind, settings.imap.clone()) {
            (AccountKind::Imap, Some(imap)) => imap,
            _ => {
                self.forget_remote(account, None)?;
                return Ok(all_in_katna(rules, account, None));
            }
        };
        let host = imap.host.to_ascii_lowercase();
        let gmail = settings.oauth == Some(OAuthProvider::Google)
            || folders::is_gmail(&folders)
            || ["gmail.com", "googlemail.com"]
                .iter()
                .any(|d| host.strip_suffix(d).is_some_and(|h| h.ends_with('.')));
        if gmail {
            return self.place_gmail(account, rules, &settings, folders).await;
        }
        self.place_sieve(account, rules, &imap, &settings, &folders)
            .await
    }

    async fn place_gmail(
        &self,
        account: AccountId,
        rules: &[Rule],
        settings: &katna_core::AccountSettings,
        folders: Vec<katna_store::remote::StoredFolder>,
    ) -> Result<Placed, katna_store::Error> {
        self.forget_remote(account, Some(RunsOn::Sieve))?;
        let existing = self.store().remote_rules(account)?;
        // On Gmail already, whatever happens now.
        let on_gmail = |placed: &mut Placed| {
            for row in existing.iter().filter(|r| r.runs_on == RunsOn::Gmail) {
                if let Some(entry) = placed.get_mut(&row.rule_id) {
                    *entry = Ok(RunsOn::Gmail);
                }
            }
        };
        if settings.oauth != Some(OAuthProvider::Google) {
            // A password: no Gmail API.
            return Ok(all_in_katna(rules, account, Some(RunsNote::SignIn)));
        }
        let tokens = match self.oauth_tokens(account, OAuthProvider::Google).await {
            Ok(tokens) => tokens,
            Err(err) => {
                tracing::info!(%account, %err, "no Gmail filters");
                let mut placed = all_in_katna(rules, account, Some(RunsNote::SignIn));
                on_gmail(&mut placed);
                return Ok(placed);
            }
        };
        let tls = match Tls::system() {
            Ok(tls) => tls,
            Err(err) => {
                let note = RunsNote::Failed {
                    service: RunsOn::Gmail,
                    error: err.to_string(),
                };
                let mut placed = all_in_katna(rules, account, Some(note));
                on_gmail(&mut placed);
                return Ok(placed);
            }
        };
        let client = GmailSettings::new(tokens, tls);
        match client.allowed().await {
            Ok(true) => {}
            Ok(false) | Err(Error::Auth(_)) => {
                let mut placed = all_in_katna(rules, account, Some(RunsNote::SignIn));
                on_gmail(&mut placed);
                return Ok(placed);
            }
            Err(err) => {
                let note = RunsNote::Failed {
                    service: RunsOn::Gmail,
                    error: err.to_string(),
                };
                let mut placed = all_in_katna(rules, account, Some(note));
                on_gmail(&mut placed);
                return Ok(placed);
            }
        }
        let push = gmail_filters::push(&client, rules, account, folders, existing.clone()).await;
        if let Some(err) = &push.error {
            tracing::warn!(%account, %err, "Gmail filters");
        }
        {
            let mut store = self.store();
            for row in existing.iter().filter(|r| r.runs_on == RunsOn::Gmail) {
                if !push.rows.iter().any(|r| r.rule_id == row.rule_id) {
                    store.drop_remote_rule(account, row.rule_id)?;
                }
            }
            for row in &push.rows {
                store.put_remote_rule(account, row)?;
            }
        }
        Ok(placed(push.verdicts, RunsOn::Gmail))
    }

    async fn place_sieve(
        &self,
        account: AccountId,
        rules: &[Rule],
        imap: &katna_core::Server,
        settings: &katna_core::AccountSettings,
        folders: &[katna_store::remote::StoredFolder],
    ) -> Result<Placed, katna_store::Error> {
        self.forget_remote(account, Some(RunsOn::Gmail))?;
        let known = self.store().rule_server(account)?;
        let now = unix_now();
        if let Some(server) = &known
            && !server.sieve
            && now - server.checked_at < SIEVE_RECHECK
        {
            self.forget_remote(account, Some(RunsOn::Sieve))?;
            return Ok(all_in_katna(rules, account, None));
        }
        let failed = |err: &Error| RunsNote::Failed {
            service: RunsOn::Sieve,
            error: err.to_string(),
        };
        let credentials = match self.credentials(account, imap, settings).await {
            Ok(credentials) => credentials,
            Err(err) => {
                let err = Error::Auth(err);
                self.forget_remote(account, Some(RunsOn::Sieve))?;
                let note = known.filter(|s| s.sieve).map(|_| failed(&err));
                return Ok(all_in_katna(rules, account, note));
            }
        };
        let tls = match endpoint(imap) {
            Ok((_, tls)) => tls,
            Err(err) => {
                tracing::warn!(%account, %err, "no TLS for ManageSieve");
                return Ok(all_in_katna(rules, account, None));
            }
        };
        let security = match imap.security {
            katna_core::Security::Plain => Security::Plain,
            _ => Security::StartTls,
        };
        let session =
            ManageSieve::connect(&imap.host, managesieve::PORT, security, &credentials, tls).await;
        let mut session = match session {
            Ok(session) => session,
            Err(err) => {
                let had_sieve = known.as_ref().is_some_and(|s| s.sieve);
                self.forget_remote(account, Some(RunsOn::Sieve))?;
                if had_sieve {
                    tracing::warn!(%account, %err, "ManageSieve");
                    return Ok(all_in_katna(rules, account, Some(failed(&err))));
                }
                // No ManageSieve there: rules run in Katna, as before.
                tracing::debug!(%account, %err, "no ManageSieve");
                self.store().set_rule_server(
                    account,
                    &RuleServer {
                        sieve: false,
                        extensions: Vec::new(),
                        checked_at: now,
                    },
                )?;
                return Ok(all_in_katna(rules, account, None));
            }
        };
        self.store().set_rule_server(
            account,
            &RuleServer {
                sieve: true,
                extensions: session.capabilities().sieve.clone(),
                checked_at: now,
            },
        )?;
        let pushed = sieve::push(&mut session, rules, account, folders).await;
        let _ = session.logout().await;
        let verdicts = match pushed {
            Ok(verdicts) => verdicts,
            Err(err) => {
                tracing::warn!(%account, %err, "Sieve script");
                self.forget_remote(account, Some(RunsOn::Sieve))?;
                return Ok(all_in_katna(rules, account, Some(failed(&err))));
            }
        };
        {
            let mut store = self.store();
            let uploaded: HashSet<i64> = verdicts
                .iter()
                .filter(|(_, v)| v.is_ok())
                .map(|(id, _)| *id)
                .collect();
            for row in store.remote_rules(account)? {
                if !uploaded.contains(&row.rule_id) {
                    store.drop_remote_rule(account, row.rule_id)?;
                }
            }
            for &rule_id in &uploaded {
                store.put_remote_rule(
                    account,
                    &RemoteRule {
                        rule_id,
                        runs_on: RunsOn::Sieve,
                        remote_ids: Vec::new(),
                        spec: String::new(),
                    },
                )?;
            }
        }
        Ok(placed(verdicts, RunsOn::Sieve))
    }

    /// The rules of `account` run in Katna again: forgets what was put on
    /// `service` (every service with `None`).
    fn forget_remote(
        &self,
        account: AccountId,
        service: Option<RunsOn>,
    ) -> Result<(), katna_store::Error> {
        let mut store = self.store();
        for row in store.remote_rules(account)? {
            if service.is_none_or(|s| s == row.runs_on) {
                store.drop_remote_rule(account, row.rule_id)?;
            }
        }
        Ok(())
    }
}
