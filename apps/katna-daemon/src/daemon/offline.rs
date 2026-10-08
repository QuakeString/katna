// SPDX-License-Identifier: GPL-3.0-or-later

//! Accounts the user takes offline (`[offline]` in the settings,
//! `docs/ARCHITECTURE.md` §6.1): the daemon stops their worker and doesn't
//! connect to their servers at all, for mail, calendars, contacts, tasks,
//! notes or rules, until each is brought back or its time ends. Changes
//! made meanwhile wait where they always do (the op queue, the outbox,
//! the store's unsent tasks and notes, held event changes) and go out
//! when the account is back. It stays offline across restarts.

use std::{
    collections::HashSet,
    sync::{Arc, Weak},
    time::Duration,
};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::{AccountId, config::OfflineAccounts};
use katna_dbus::state;
use katna_store::SendState;

use super::{Daemon, Notice, Status, stop, unix_now};

impl Daemon {
    /// Whether the user took `account` offline (as last applied).
    pub(crate) fn is_offline(&self, account: AccountId) -> bool {
        self.offline_now.lock().unwrap().contains(&account)
    }

    /// Takes in the `[offline]` settings and applies them soon.
    pub(super) fn set_offline(&self, offline: OfflineAccounts) {
        *self.offline.lock().unwrap() = offline;
        let _ = self.offline_wake.0.try_send(());
    }

    /// The accounts offline at `now`, by the settings.
    fn offline_at(&self, now: i64) -> HashSet<AccountId> {
        let offline = self.offline.lock().unwrap().clone();
        if offline.until.is_empty() {
            return HashSet::new();
        }
        let Ok(accounts) = self.store().accounts() else {
            return self.offline_now.lock().unwrap().clone();
        };
        accounts
            .into_iter()
            .filter(|a| offline.is_offline(&a.address, now))
            .map(|a| a.id)
            .collect()
    }

    /// Notes which accounts are offline before any worker starts, so none
    /// connects for a moment at login.
    pub(super) fn apply_offline_at_start(&self) {
        let now = self.offline_at(unix_now());
        for &id in &now {
            self.set_status(id, Status::new(state::PAUSED, ""));
        }
        if !now.is_empty() {
            tracing::info!(accounts = ?now, "offline accounts stay offline");
        }
        *self.offline_now.lock().unwrap() = now;
    }

    /// Stops the accounts newly taken offline and starts the ones back.
    async fn apply_offline(self: &Arc<Self>) {
        let now = self.offline_at(unix_now());
        let before = std::mem::replace(&mut *self.offline_now.lock().unwrap(), now.clone());
        for &id in now.difference(&before) {
            tracing::info!(account = %id, "taken offline");
            let running = self.workers().remove(&id);
            if let Some(running) = running {
                stop(id, running).await;
            }
            self.on_demand.close(id);
            self.set_status(id, Status::new(state::PAUSED, ""));
            let _ = self.notices.try_send(Notice::MailChanged(id));
        }
        let back: Vec<AccountId> = before.difference(&now).copied().collect();
        if back.is_empty() {
            return;
        }
        let Ok(accounts) = self.store().accounts() else {
            return;
        };
        for id in &back {
            tracing::info!(account = %id, "back online");
            let Some(account) = accounts.iter().find(|a| a.id == *id) else {
                continue;
            };
            self.set_status(*id, Status::new(state::CONNECTING, ""));
            // Its worker sends the waiting changes before it syncs.
            self.start_account(account).await;
            self.send_waiting_mail(*id);
            self.place_rules_soon(Some(*id));
        }
        self.resend_held_event_changes();
        self.wake_calendars();
        self.wake_contacts();
        self.wake_task_sync();
        for id in back {
            self.notes_changed(Some(id.0));
        }
    }

    /// Mail that waited for `account` to come back, or to be signed in
    /// again, goes out now, not at its next try.
    pub(super) fn send_waiting_mail(&self, account: AccountId) {
        let waited: Vec<i64> = self.send_errors.lock().unwrap().keys().copied().collect();
        let mut store = self.store();
        let Ok(entries) = store.outbox() else {
            return;
        };
        let now = unix_now();
        let due: Vec<i64> = entries
            .into_iter()
            .filter(|entry| {
                entry.account == account
                    && entry.state == SendState::Queued
                    && entry.send_at > now
                    && waited.contains(&entry.id)
            })
            .map(|entry| entry.id)
            .collect();
        if !due.is_empty() {
            let sent_now = store.mail_batch().and_then(|mut batch| {
                for &id in &due {
                    batch.set_send_state(id, SendState::Queued, Some(now), None)?;
                }
                batch.commit()
            });
            if let Err(err) = sent_now {
                tracing::warn!(%err, "could not send waiting mail now");
            }
        }
        drop(store);
        if let Some(sending) = self.outbox.lock().unwrap().as_ref() {
            sending.handle.wake();
        }
    }
}

/// Applies the `[offline]` settings when they change and when an
/// account's time ends, until the daemon goes away.
pub(super) async fn run(daemon: Weak<Daemon>, wake: Receiver<()>) {
    loop {
        let next = {
            let Some(daemon) = daemon.upgrade() else {
                return;
            };
            if daemon.closing() {
                return;
            }
            daemon.apply_offline().await;
            daemon.offline.lock().unwrap().next_end(unix_now())
        };
        // A second late, so the time has surely ended.
        let wait = next.map_or(Duration::from_secs(24 * 60 * 60), |at| {
            Duration::from_secs(u64::try_from(at - unix_now()).unwrap_or(0) + 1)
        });
        async {
            let _ = wake.recv().await;
        }
        .or(async {
            smol::Timer::after(wait).await;
        })
        .await;
    }
}
