// SPDX-License-Identifier: GPL-3.0-or-later

//! Keeps task lists in sync with each account's own task service
//! (`docs/ARCHITECTURE.md` §18.1): Google Tasks for Google accounts,
//! Microsoft To Do for Microsoft accounts, and to-dos on the CalDAV
//! server of an account with a password ([`katna_sync::tasks`]), each
//! account's best way first ([`katna_sync::methods`]).
//!
//! A round runs every few minutes, and shortly after a task changes in
//! Katna (the desktop clock, the Tasks page), so a tick shows on the
//! phone within seconds. Accounts signed in before Katna asked for their
//! tasks are skipped until they sign in again.

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Weak},
    time::Duration,
};

use async_channel::Receiver;
use futures_lite::FutureExt;
use jiff::tz::TimeZone;
use katna_core::{Account, AccountId, AccountKind, OAuthProvider};
use katna_store::tasks::TaskFields;
use katna_sync::{
    Error,
    calendar::caldav::CalDav,
    methods::{self, Data, Method},
    net::Tls,
    tasks::{TaskService, caldav::DavTasks, google::GoogleTasks, graph::ToDo, sync_account},
};

use super::{CommandError, Daemon, Notice};

/// How often the service's changes are fetched.
const EVERY: Duration = Duration::from_secs(5 * 60);
/// How long after the first start of the daemon the first round runs, out
/// of the way of the first mail sync.
const FIRST: Duration = Duration::from_secs(15);
/// How long after a change in Katna it goes out, so a few quick changes
/// go together.
const SETTLE: Duration = Duration::from_secs(2);

/// Each account's task service for each way, and what it depends on.
type Services = HashMap<(AccountId, Method), (String, TaskService)>;

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

    /// Ticks task `id` off (or back on when `done` is false). A task that
    /// repeats moves to its next day instead and stays open, its reminder
    /// with it, except in To Do, which makes the next one itself. Returns
    /// whether the task was found.
    pub(crate) fn set_task_done(&self, id: i64, done: bool) -> Result<bool, CommandError> {
        let mut store = self.store();
        let Some(task) = store.task(id)? else {
            return Ok(false);
        };
        if done && task.done_at.is_none() && task.parent.is_none() && !task.repeat.is_empty() {
            let account = store
                .task_lists()?
                .into_iter()
                .find(|l| l.id == task.list)
                .and_then(|l| l.account);
            let to_do = match account {
                Some(account) => {
                    store.account_settings(account)?.and_then(|s| s.oauth)
                        == Some(OAuthProvider::Microsoft)
                }
                None => false,
            };
            let zone = TimeZone::system();
            let today = jiff::Timestamp::now().to_zoned(zone.clone()).date();
            if !to_do
                && let Some((due, repeat)) =
                    katna_dav::todo::next_due(&task.due, &task.repeat, today)
            {
                let remind_at = katna_dav::todo::moved_reminder(
                    task.remind_at,
                    (&task.due, task.due_time),
                    (&due, task.due_time),
                    &zone,
                );
                tracing::info!(id, due, "repeating task moved to its next day");
                return Ok(store.edit_task(
                    id,
                    &TaskFields {
                        title: task.title,
                        notes: task.notes,
                        due,
                        due_time: task.due_time,
                        remind_at,
                        repeat,
                        starred: task.starred,
                        mail: task.mail,
                    },
                )?);
            }
        }
        Ok(store.set_task_done(id, done)?)
    }

    /// `account`'s task service the way `method`, and what it depends on
    /// (a key that changes with a new sign-in, the server or the
    /// password); `None` when that way has none.
    async fn new_task_service(
        &self,
        account: &Account,
        method: Method,
    ) -> Option<(String, TaskService)> {
        if account.kind != AccountKind::Imap {
            return None;
        }
        let settings = self.store().account_settings(account.id).ok()??;
        if let Some(provider) = settings.oauth {
            let tokens = self
                .oauth_tokens(account.id, provider)
                .await
                .map_err(|err| tracing::debug!(account = account.id.0, %err, "no tokens for tasks"))
                .ok()?;
            let key = format!("{provider:?} {:p}", Arc::as_ptr(&tokens));
            let tls = Tls::system().ok()?;
            // Google's CalDAV keeps no to-dos, so its tasks come only from
            // Google Tasks, as To Do's come only from Graph.
            let service = match (provider, method) {
                (OAuthProvider::Google, Method::Api) => {
                    TaskService::Google(GoogleTasks::new(tokens, tls))
                }
                (OAuthProvider::Microsoft, Method::Api) => {
                    TaskService::Microsoft(ToDo::new(tokens, tls))
                }
                (_, Method::Dav) => return None,
            };
            return Some((key, service));
        }
        if method != Method::Dav {
            return None;
        }
        let server = settings.imap?;
        let password = self.secrets.password(account.id).await.ok()??;
        let (_, tls) = super::endpoint(&server).ok()?;
        let key = format!(
            "caldav {} {} {} {} {password}",
            account.address, server.host, server.username, server.accept_invalid_certs
        );
        let dav = CalDav::for_account(
            &account.address,
            &server.host,
            &server.username,
            &password,
            tls,
        );
        Some((key, TaskService::CalDav(DavTasks::new(dav))))
    }

    /// `account`'s task service the way `method`, if it has one and its
    /// sign-in lets Katna into it (kept in `known` between rounds, so a
    /// CalDAV server is looked for once).
    async fn task_service<'a>(
        &self,
        account: &Account,
        method: Method,
        known: &'a mut Services,
    ) -> Option<&'a TaskService> {
        let (key, service) = self.new_task_service(account, method).await?;
        let at = (account.id, method);
        if known.get(&at).is_none_or(|(old, _)| *old != key) {
            known.insert(at, (key, service));
        }
        let service = &known[&at].1;
        allowed(account.id, service).await.then_some(service)
    }

    /// Syncs `account`'s tasks, the best way first and the others when it
    /// is not available. Returns whether any way let Katna in, and
    /// whether anything changed.
    async fn sync_account_tasks(&self, account: &Account, known: &mut Services) -> (bool, bool) {
        let Ok(settings) = self.store().account_settings(account.id) else {
            return (false, false);
        };
        let provider = settings.and_then(|s| s.oauth);
        let now = super::unix_now();
        let order = methods::order(&self.store(), account.id, Data::Tasks, provider, now);
        let mut reached = false;
        for method in order {
            let Some(service) = self.task_service(account, method, known).await else {
                continue;
            };
            reached = true;
            match sync_account(service, &self.store, account.id).await {
                Ok(changed) => {
                    methods::remember(&mut self.store(), account.id, Data::Tasks, method, now);
                    return (true, changed);
                }
                // A refused sign-in: another way may still let Katna in.
                Err(Error::Auth(err)) => {
                    tracing::info!(account = account.id.0, ?method, %err, "tasks need a new sign-in");
                }
                // The network or the server: not a reason to go another way.
                Err(err) => {
                    tracing::warn!(account = account.id.0, ?method, %err, "task sync failed");
                    return (true, false);
                }
            }
        }
        (reached, false)
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
            let (reached, account_changed) = self.sync_account_tasks(&account, known).await;
            changed |= account_changed;
            if reached {
                told.remove(&account.id);
            } else if told.insert(account.id) {
                tracing::info!(
                    account = account.id.0,
                    "no task service, or its sign-in didn't allow tasks"
                );
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
