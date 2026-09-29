// SPDX-License-Identifier: GPL-3.0-or-later

//! Task lists synced with each account's own task service
//! (`docs/ARCHITECTURE.md` §18.1): Google Tasks for Google accounts
//! ([`google`]), Microsoft To Do through Graph for Microsoft accounts
//! ([`graph`]). Only the few calls sync needs, over our own HTTPS client,
//! with the account's OAuth2 tokens.
//!
//! [`sync_account`] runs one round for an account: list changes made in
//! Katna go out, the service's lists come in, then for each list the task
//! changes made in Katna go out and the service's changes come in. A
//! change made in Katna and not yet sent wins over the service's.

use std::sync::Mutex;

use katna_core::AccountId;
use katna_store::{
    Store,
    tasks::{PendingTask, RemoteTask, RemoteTaskList, Task},
};

use crate::{Error, Result, autoconfig::http::Reply};

pub mod google;
pub mod graph;

/// An account's task service.
pub enum TaskService {
    Google(google::GoogleTasks),
    Microsoft(graph::ToDo),
}

/// What a pull of one list brought.
#[derive(Debug, Default)]
pub struct Pull {
    pub tasks: Vec<RemoteTask>,
    /// Every task the service has, rather than the changes since the
    /// last pull.
    pub all: bool,
    /// Where the next pull starts.
    pub state: Option<String>,
    /// Tasks whose steps [`Self::tasks`] holds every one of, when the
    /// service reads steps apart from their tasks (To Do); steps of these
    /// that it doesn't hold went on the service.
    pub steps_of: Vec<String>,
}

/// An answer that says the item is gone (deleted on the service).
fn gone(reply: &Reply) -> bool {
    matches!(reply.status, 404 | 410)
}

impl TaskService {
    /// Whether the account's sign-in allowed Katna into its tasks.
    /// Accounts signed in before Katna asked have to sign in again.
    pub async fn allowed(&self) -> Result<bool> {
        match self {
            Self::Google(service) => service.allowed().await,
            Self::Microsoft(service) => service.allowed().await,
        }
    }

    async fn lists(&self) -> Result<Vec<RemoteTaskList>> {
        match self {
            Self::Google(service) => service.lists().await,
            Self::Microsoft(service) => service.lists().await,
        }
    }

    async fn add_list(&self, title: &str) -> Result<String> {
        match self {
            Self::Google(service) => service.add_list(title).await,
            Self::Microsoft(service) => service.add_list(title).await,
        }
    }

    async fn rename_list(&self, id: &str, title: &str) -> Result<()> {
        match self {
            Self::Google(service) => service.rename_list(id, title).await,
            Self::Microsoft(service) => service.rename_list(id, title).await,
        }
    }

    async fn delete_list(&self, id: &str) -> Result<()> {
        match self {
            Self::Google(service) => service.delete_list(id).await,
            Self::Microsoft(service) => service.delete_list(id).await,
        }
    }

    async fn pull(&self, list: &str, state: Option<&str>) -> Result<Pull> {
        match self {
            Self::Google(service) => service.pull(list, state).await,
            Self::Microsoft(service) => service.pull(list, state).await,
        }
    }

    /// Adds `task` to `list`, as a step of `parent` when it has one.
    async fn insert(&self, list: &str, task: &Task, parent: Option<&str>) -> Result<RemoteTask> {
        match self {
            Self::Google(service) => service.insert(list, task, parent).await,
            Self::Microsoft(service) => service.insert(list, task, parent).await,
        }
    }

    /// Changes task `id`; `Ok(None)` when the service no longer has it.
    async fn update(&self, list: &str, id: &str, task: &Task) -> Result<Option<RemoteTask>> {
        match self {
            Self::Google(service) => service.update(list, id, task).await,
            Self::Microsoft(service) => service.update(list, id, task).await,
        }
    }

    async fn delete(&self, list: &str, id: &str) -> Result<()> {
        match self {
            Self::Google(service) => service.delete(list, id).await,
            Self::Microsoft(service) => service.delete(list, id).await,
        }
    }
}

/// Runs one sync round for `account`. Returns whether the store changed.
pub async fn sync_account(
    service: &TaskService,
    store: &Mutex<Store>,
    account: AccountId,
) -> Result<bool> {
    let mut changed = false;

    let pending = store.lock().unwrap().pending_task_lists(account)?;
    for list in pending {
        match (list.deleted, list.remote_id.as_deref()) {
            (true, remote) => {
                if let Some(remote) = remote {
                    service.delete_list(remote).await?;
                }
                store.lock().unwrap().forget_task_list(list.id)?;
            }
            (false, None) => {
                let remote = service.add_list(&list.title).await?;
                store.lock().unwrap().task_list_pushed(list.id, &remote)?;
            }
            (false, Some(remote)) => {
                service.rename_list(remote, &list.title).await?;
                store.lock().unwrap().task_list_pushed(list.id, remote)?;
            }
        }
    }

    let lists = service.lists().await?;
    {
        let mut store = store.lock().unwrap();
        changed |= store.sync_task_lists(account, &lists)?;
        let moved = store.move_out_local_tasks()?;
        if moved > 0 {
            tracing::info!(
                moved,
                "tasks from this computer moved to the account's list"
            );
        }
    }

    let known = store.lock().unwrap().account_task_lists(account)?;
    for (list, remote) in known {
        let Some(remote) = remote else { continue };
        let pending = {
            let mut store = store.lock().unwrap();
            store.resend_unsent_steps(list)?;
            store.pending_tasks(list)?
        };
        for task in pending {
            push(service, store, &remote, task).await?;
        }
        let state = store.lock().unwrap().task_list_sync_state(list)?;
        let pull = service.pull(&remote, state.as_deref()).await?;
        let mut store = store.lock().unwrap();
        changed |= store.drop_unlisted_steps(list, &pull.steps_of, &pull.tasks)?;
        changed |= store.sync_tasks(list, &pull.tasks, pull.all)?;
        store.set_task_list_sync_state(list, pull.state.as_deref())?;
    }
    Ok(changed)
}

/// Sends one change to the service. A change the service refuses is
/// logged and left for the next round rather than stopping the others.
async fn push(
    service: &TaskService,
    store: &Mutex<Store>,
    list: &str,
    pending: PendingTask,
) -> Result<()> {
    let id = pending.task.id;
    let sent = async {
        if pending.deleted {
            if let Some(remote) = &pending.remote_id {
                service.delete(list, remote).await?;
            }
            store.lock().unwrap().forget_task(id)?;
            return Ok(());
        }
        if pending.task.parent.is_some() && pending.parent_remote.is_none() {
            // Its task is not on the service yet; next round.
            return Ok(());
        }
        let remote = match &pending.remote_id {
            Some(remote) => match service.update(list, remote, &pending.task).await? {
                Some(done) => done,
                // Deleted on the service while changed here: the change
                // brings it back.
                None => {
                    service
                        .insert(list, &pending.task, pending.parent_remote.as_deref())
                        .await?
                }
            },
            None => {
                service
                    .insert(list, &pending.task, pending.parent_remote.as_deref())
                    .await?
            }
        };
        store
            .lock()
            .unwrap()
            .task_pushed(id, pending.stamp, &remote)?;
        Ok(())
    };
    match sent.await {
        Err(Error::Rejected(err)) => {
            tracing::warn!(id, %err, "the task service refused a change");
            Ok(())
        }
        other => other,
    }
}

/// Percent-encodes an ID for a URL path.
fn segment(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for b in id.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// `YYYY-MM-DD` of an RFC 3339 time's own day (the first ten characters),
/// or empty.
fn date_part(time: &str) -> String {
    time.get(..10)
        .filter(|day| day.parse::<jiff::civil::Date>().is_ok())
        .map(str::to_owned)
        .unwrap_or_default()
}

/// Unix seconds of an RFC 3339 time.
fn unix(time: &str) -> Option<i64> {
    time.parse::<jiff::Timestamp>()
        .ok()
        .map(jiff::Timestamp::as_second)
}

/// `at` as an RFC 3339 time in UTC with milliseconds, as Google writes.
fn rfc3339(at: i64) -> String {
    jiff::Timestamp::from_second(at)
        .unwrap_or(jiff::Timestamp::UNIX_EPOCH)
        .strftime("%Y-%m-%dT%H:%M:%S.000Z")
        .to_string()
}

#[cfg(test)]
mod tests;
