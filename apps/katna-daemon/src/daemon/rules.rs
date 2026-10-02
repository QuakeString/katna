// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail rules run by Katna (`docs/ARCHITECTURE.md` §9.4).
//!
//! After each sync of an account, and after bodies arrive, its rules run on
//! its new incoming mail ([`katna_sync::rules::Watch`]), before new-mail
//! notifications are looked for, so "don't notify" keeps one from showing
//! and mail a rule moves out of the inbox never rings. Their changes go
//! through `katna_sync::ops`, as the user's own do, so the workers send
//! them to the servers. A rule whose action fails is switched off with the
//! reason; `RulesChanged` tells the apps.
//!
//! Undo: the app's Undo history is kept per window for the user's own
//! changes; a rule's changes are not in it. They are logged here with the
//! rule's name.

use std::sync::Arc;
use std::time::Duration;

use katna_core::AccountId;
use katna_store::rules::Rule;
use katna_store::{MessageFlags, MessageId};
use katna_sync::rules::{self, Context, Outcome, Watch};

use super::{CommandError, Daemon, Notice, unix_now};

/// The most days "also apply to these" looks back.
const MAX_APPLY_DAYS: u32 = 3650;

impl Daemon {
    /// Starts watching `account`'s new mail for its rules, unless already
    /// watching.
    pub(super) fn watch_rules(&self, account: AccountId) {
        let watch = match Watch::new(&self.store(), account) {
            Ok(watch) => watch,
            Err(err) => {
                tracing::warn!(%err, %account, "mail rules don't run");
                return;
            }
        };
        self.rule_watches
            .lock()
            .unwrap()
            .entry(account)
            .or_insert(watch);
    }

    pub(super) fn forget_rules(&self, account: AccountId) {
        self.rule_watches.lock().unwrap().remove(&account);
    }

    /// Whether new mail of `account` waits for its body before its rules
    /// run.
    pub(super) fn rules_waiting(&self, account: AccountId) -> bool {
        self.rule_watches
            .lock()
            .unwrap()
            .get(&account)
            .is_some_and(Watch::waiting)
    }

    /// After a sync of `account` or its bodies: runs its rules, then looks
    /// for mail to notify about. Mail held for its body is looked at again
    /// once the wait is over, even if no body arrives.
    pub(super) async fn rules_then_notices(self: &Arc<Self>, account: AccountId) {
        let held = self.run_rules(account);
        if let Some(notices) = self.new_mail_notices() {
            notices.synced(&self.store, account).await;
        }
        if held > 0 {
            let daemon = Arc::downgrade(self);
            smol::spawn(async move {
                async_io::Timer::after(Duration::from_secs(rules::HOLD.unsigned_abs() + 1)).await;
                // The wait is over now: the rules run on what waited.
                if let Some(daemon) = daemon.upgrade()
                    && daemon.rules_waiting(account)
                {
                    daemon.run_rules(account);
                    if let Some(notices) = daemon.new_mail_notices() {
                        notices.synced(&daemon.store, account).await;
                    }
                }
            })
            .detach();
        }
    }

    /// Runs the rules of `account` on its new mail, and tells the
    /// notifications which mail not to show (yet). Returns how many
    /// messages wait for their body.
    pub(super) fn run_rules(&self, account: AccountId) -> usize {
        let context = Context {
            now: unix_now(),
            live: true,
            can_send: self.check_smtp(account).is_ok(),
        };
        // Metered connections fetch no bodies: don't wait for them.
        let wait = !self.metered();
        let outcome = {
            let mut watches = self.rule_watches.lock().unwrap();
            let Some(watch) = watches.get_mut(&account) else {
                return 0;
            };
            let mut store = self.store();
            watch.run(&mut store, account, &context, wait)
        };
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(err) => {
                tracing::warn!(%err, %account, "running mail rules");
                return 0;
            }
        };
        if let Some(notices) = self.new_mail_notices() {
            notices.quiet(account, &outcome.quiet);
            notices.hold(account, &outcome.held);
        }
        if outcome.changed > 0 {
            tracing::info!(%account, messages = outcome.changed, "mail rules ran");
        }
        self.rules_done(&outcome);
        outcome.held.len()
    }

    /// Tells clients and workers what rules did.
    fn rules_done(&self, outcome: &Outcome) {
        if !outcome.failed.is_empty() {
            let _ = self.notices.try_send(Notice::RulesChanged);
        }
        // "Mark read after" may have set a time.
        if outcome.changed > 0 {
            self.wake_scheduler();
        }
        for &id in &outcome.outbox {
            self.queued(id);
        }
        let workers = self.workers();
        for account in &outcome.accounts {
            let _ = self.notices.try_send(Notice::MailChanged(*account));
            if let Some(running) = workers.get(account) {
                running.handle.send_changes();
            }
        }
    }

    /// Saves a rule from its JSON (`katna_store::rules::Rule`): a new one
    /// for ID 0. Returns its ID.
    pub fn save_rule(&self, json: &str) -> Result<i64, CommandError> {
        let rule: Rule = serde_json::from_str(json)
            .map_err(|err| CommandError::InvalidArgs(format!("not a rule: {err}")))?;
        rule.validate().map_err(CommandError::InvalidArgs)?;
        self.check_rule_places(&rule)?;
        let id = self.store().save_rule(&rule)?;
        tracing::info!(id, name = rule.name, "rule saved");
        let _ = self.notices.try_send(Notice::RulesChanged);
        Ok(id)
    }

    /// The accounts a rule names exist, and the folders it names are in
    /// them.
    fn check_rule_places(&self, rule: &Rule) -> Result<(), CommandError> {
        let store = self.store();
        let accounts = store.accounts()?;
        let mut folders = Vec::new();
        for &id in &rule.accounts {
            if !accounts.iter().any(|a| a.id.0 == id) {
                return Err(CommandError::UnknownAccount(id));
            }
            folders.extend(store.folders(AccountId(id))?.into_iter().map(|f| f.id.0));
        }
        for action in &rule.actions {
            if let Some(folder) = action.folder()
                && !folders.contains(&folder)
            {
                return Err(CommandError::UnknownFolder(folder));
            }
        }
        Ok(())
    }

    /// Deletes a rule.
    pub fn delete_rule(&self, id: i64) -> Result<(), CommandError> {
        if !self.store().delete_rule(id)? {
            return Err(CommandError::InvalidArgs(format!("no rule {id}")));
        }
        tracing::info!(id, "rule deleted");
        let _ = self.notices.try_send(Notice::RulesChanged);
        Ok(())
    }

    /// Puts rules `ids` first, in this order.
    pub fn reorder_rules(&self, ids: &[i64]) -> Result<(), CommandError> {
        self.store().reorder_rules(ids)?;
        let _ = self.notices.try_send(Notice::RulesChanged);
        Ok(())
    }

    /// Switches a rule on or off.
    pub fn set_rule_enabled(&self, id: i64, on: bool) -> Result<(), CommandError> {
        if !self.store().set_rule_enabled(id, on)? {
            return Err(CommandError::InvalidArgs(format!("no rule {id}")));
        }
        let _ = self.notices.try_send(Notice::RulesChanged);
        Ok(())
    }

    /// Runs rule `id` once over its accounts' inbox mail from the last
    /// `days` days. Returns how many messages it changed.
    pub fn apply_rule(&self, id: i64, days: u32) -> Result<u32, CommandError> {
        if days == 0 || days > MAX_APPLY_DAYS {
            return Err(CommandError::InvalidArgs(format!(
                "apply a rule to 1 to {MAX_APPLY_DAYS} days of mail"
            )));
        }
        let rule = self
            .store()
            .rule(id)?
            .ok_or_else(|| CommandError::InvalidArgs(format!("no rule {id}")))?;
        let outcome = rules::apply_recent(&mut self.store(), &rule, days, unix_now())?;
        self.rules_done(&outcome);
        tracing::info!(
            id,
            name = rule.name,
            days,
            changed = outcome.changed,
            "rule applied"
        );
        match outcome.failed.into_iter().next() {
            Some((_, reason)) => Err(CommandError::Failed(reason)),
            None => Ok(outcome.changed),
        }
    }

    /// "Mark read after N days" fell due for `message`.
    pub(super) fn read_after_due(&self, message: MessageId) {
        let _ = katna_meta::clear_read_after(&mut self.store(), message);
        match self.set_flags(&[message], MessageFlags::SEEN, MessageFlags::empty()) {
            Ok(()) | Err(CommandError::UnknownMessage(_)) => {}
            Err(err) => tracing::warn!(%err, %message, "marking read after a rule's wait"),
        }
    }
}
