// SPDX-License-Identifier: GPL-3.0-or-later

//! Keeps task lists in sync with each account's own task service
//! (`docs/ARCHITECTURE.md` §18.1): Google Tasks for Google accounts,
//! Microsoft To Do for Microsoft accounts ([`katna_sync::tasks`]).
//!
//! A round runs every few minutes, and shortly after a task changes in
//! Katna (the desktop clock, the Tasks page), so a tick shows on the
//! phone within seconds. Accounts signed in before Katna asked for their
//! tasks are skipped until they sign in again.

use std::{collections::HashSet, sync::Weak, time::Duration};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::{AccountId, OAuthProvider};
use katna_sync::{
    Error,
    net::Tls,
    tasks::{TaskService, google::GoogleTasks, graph::ToDo, sync_account},
};

use super::{Daemon, Notice};

/// How often the service's changes are fetched.
const EVERY: Duration = Duration::from_secs(5 * 60);
/// How long after the first start of the daemon the first round runs, out
/// of the way of the first mail sync.
const FIRST: Duration = Duration::from_secs(15);
/// How long after a change in Katna it goes out, so a few quick changes
/// go together.
const SETTLE: Duration = Duration::from_secs(2);

impl Daemon {
    /// Has task sync run a round soon: a task changed in Katna.
    pub(crate) fn wake_task_sync(&self) {
        let _ = self.task_sync_wake.0.try_send(());
    }

    /// The task service of `account`, if it has one and its sign-in
    /// allowed Katna into it.
    async fn task_service(&self, account: AccountId) -> Option<TaskService> {
        let settings = self.store().account_settings(account).ok()??;
        let provider = settings.oauth?;
        let tokens = self
            .oauth_tokens(account, provider)
            .await
            .map_err(|err| tracing::debug!(account = account.0, %err, "no tokens for tasks"))
            .ok()?;
        let tls = Tls::system().ok()?;
        let service = match provider {
            OAuthProvider::Google => TaskService::Google(GoogleTasks::new(tokens, tls)),
            OAuthProvider::Microsoft => TaskService::Microsoft(ToDo::new(tokens, tls)),
        };
        match service.allowed().await {
            Ok(true) => Some(service),
            Ok(false) => None,
            Err(err) => {
                tracing::debug!(account = account.0, %err, "could not ask about tasks");
                None
            }
        }
    }

    /// One round for every account. Returns whether anything changed.
    async fn sync_tasks(&self, told: &mut HashSet<AccountId>) -> bool {
        let Ok(accounts) = self.store().accounts() else {
            return false;
        };
        let mut changed = false;
        for account in accounts {
            if self.closing() {
                break;
            }
            let Some(service) = self.task_service(account.id).await else {
                if told.insert(account.id) {
                    tracing::info!(
                        account = account.id.0,
                        "no task service, or its sign-in didn't allow tasks"
                    );
                }
                continue;
            };
            told.remove(&account.id);
            match sync_account(&service, &self.store, account.id).await {
                Ok(true) => changed = true,
                Ok(false) => {}
                Err(Error::Auth(err)) => {
                    tracing::info!(account = account.id.0, %err, "tasks need a new sign-in");
                }
                Err(err) => tracing::warn!(account = account.id.0, %err, "task sync failed"),
            }
        }
        changed
    }
}

/// Runs task sync until the daemon goes.
pub(crate) async fn run(daemon: Weak<Daemon>, wakes: Receiver<()>) {
    let mut told = HashSet::new();
    let mut wait = FIRST;
    loop {
        let woken = async { wakes.recv().await.is_ok() }
            .or(async {
                smol::Timer::after(wait).await;
                false
            })
            .await;
        if woken {
            smol::Timer::after(SETTLE).await;
            while wakes.try_recv().is_ok() {}
        }
        wait = EVERY;
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.closing() {
            return;
        }
        if daemon.sync_tasks(&mut told).await {
            let _ = daemon.notices().try_send(Notice::TasksChanged);
        }
    }
}
