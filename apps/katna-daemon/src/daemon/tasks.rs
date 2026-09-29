// SPDX-License-Identifier: GPL-3.0-or-later

//! Keeps task lists in sync with each account's own task service
//! (`docs/ARCHITECTURE.md` §18.1): Google Tasks for Google accounts,
//! Microsoft To Do for Microsoft accounts, and to-dos on the CalDAV
//! server of an account with a password ([`katna_sync::tasks`]).
//!
//! A round runs every few minutes, and shortly after a task changes in
//! Katna (the desktop clock, the Tasks page), so a tick shows on the
//! phone within seconds. Accounts signed in before Katna asked for their
//! tasks are skipped until they sign in again.

use std::{
    collections::{HashMap, HashSet},
    sync::Weak,
    time::Duration,
};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::{AccountId, AccountKind, OAuthProvider};
use katna_sync::{
    Error,
    calendar::caldav::CalDav,
    net::Tls,
    tasks::{TaskService, caldav::DavTasks, google::GoogleTasks, graph::ToDo, sync_account},
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

/// Each account's task service and what it depends on.
type Services = HashMap<AccountId, (String, TaskService)>;

/// Whether `service` lets Katna in.
async fn allowed(account: AccountId, service: &TaskService) -> bool {
    match service.allowed().await {
        Ok(allowed) => allowed,
        Err(err) => {
            tracing::debug!(account = account.0, %err, "could not ask about tasks");
            false
        }
    }
}

impl Daemon {
    /// Has task sync run a round soon: a task changed in Katna.
    pub(crate) fn wake_task_sync(&self) {
        let _ = self.task_sync_wake.0.try_send(());
    }

    /// The CalDAV to-dos of `account`, an account with a password, and
    /// what they depend on (a key that changes with the server or the
    /// password).
    async fn dav_tasks(&self, account: AccountId) -> Option<(String, TaskService)> {
        let settings = self.store().account_settings(account).ok()??;
        let server = settings.imap?;
        let password = self.secrets.password(account).await.ok()??;
        let (_, tls) = super::endpoint(&server).ok()?;
        let key = format!(
            "caldav {} {} {} {password}",
            server.host, server.username, server.accept_invalid_certs
        );
        let dav = CalDav::new(&server.host, &server.username, &password, tls);
        Some((key, TaskService::CalDav(DavTasks::new(dav))))
    }

    /// The task service of `account`, if it has one and its sign-in
    /// allowed Katna into it: the provider's own for a Google or Microsoft
    /// sign-in, else CalDAV on its server (kept in `known` between rounds,
    /// so the server is looked up once).
    async fn task_service<'a>(
        &self,
        account: AccountId,
        kind: AccountKind,
        known: &'a mut Services,
    ) -> Option<&'a TaskService> {
        let settings = self.store().account_settings(account).ok()??;
        let service = match settings.oauth {
            Some(provider) => {
                let tokens = self
                    .oauth_tokens(account, provider)
                    .await
                    .map_err(
                        |err| tracing::debug!(account = account.0, %err, "no tokens for tasks"),
                    )
                    .ok()?;
                let tls = Tls::system().ok()?;
                let service = match provider {
                    OAuthProvider::Google => TaskService::Google(GoogleTasks::new(tokens, tls)),
                    OAuthProvider::Microsoft => TaskService::Microsoft(ToDo::new(tokens, tls)),
                };
                (String::new(), service)
            }
            None if kind == AccountKind::Imap => {
                let (key, service) = self.dav_tasks(account).await?;
                let keep = known.get(&account).is_some_and(|(old, _)| *old == key);
                if keep {
                    let service = &known[&account].1;
                    return allowed(account, service).await.then_some(service);
                }
                (key, service)
            }
            None => return None,
        };
        known.insert(account, service);
        let service = &known[&account].1;
        allowed(account, service).await.then_some(service)
    }

    /// One round for every account. Returns whether anything changed.
    async fn sync_tasks(&self, told: &mut HashSet<AccountId>, known: &mut Services) -> bool {
        let Ok(accounts) = self.store().accounts() else {
            return false;
        };
        let mut changed = false;
        for account in accounts {
            if self.closing() {
                break;
            }
            let Some(service) = self.task_service(account.id, account.kind, known).await else {
                if told.insert(account.id) {
                    tracing::info!(
                        account = account.id.0,
                        "no task service, or its sign-in didn't allow tasks"
                    );
                }
                continue;
            };
            told.remove(&account.id);
            match sync_account(service, &self.store, account.id).await {
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
    let mut known = Services::new();
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
        if daemon.sync_tasks(&mut told, &mut known).await {
            let _ = daemon.notices().try_send(Notice::TasksChanged);
        }
    }
}
