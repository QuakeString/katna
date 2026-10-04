// SPDX-License-Identifier: GPL-3.0-or-later

//! Task lists synced with each account's own task service
//! (`docs/ARCHITECTURE.md` §18.1): Google Tasks for Google accounts
//! ([`google`]), Microsoft To Do through Graph for Microsoft accounts
//! ([`graph`]), Zoho Mail's tasks for Zoho accounts ([`zoho`]), and
//! to-dos on the CalDAV server of an account with a password
//! ([`caldav`]). Only the few calls sync needs, over our own
//! HTTPS client.
//!
//! [`sync_account`] runs one round for an account: list changes made in
//! Katna go out, the service's lists come in, then for each list the task
//! changes made in Katna go out and the service's changes come in. A
//! change made in Katna and not yet sent wins over the service's. Files on
//! tasks follow their tasks ([`push_files`]): To Do keeps them as
//! attachments and CalDAV inside the to-do; Google Tasks and Zoho keep
//! none, so there they stay on this computer.

use std::collections::BTreeMap;
use std::sync::Mutex;

use katna_core::AccountId;
use katna_store::{
    Store,
    tasks::{PendingFile, PendingTask, Place, RemoteFile, RemoteTask, RemoteTaskList, Task},
};

use crate::{Error, Result, autoconfig::http::Reply};

pub mod caldav;
pub mod google;
pub mod graph;
pub mod zoho;

/// An account's task service.
pub enum TaskService {
    Google(google::GoogleTasks),
    Microsoft(graph::ToDo),
    CalDav(caldav::DavTasks),
    Zoho(zoho::ZohoTasks),
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
            Self::CalDav(service) => service.allowed().await,
            Self::Zoho(service) => service.allowed().await,
        }
    }

    async fn lists(&self) -> Result<Vec<RemoteTaskList>> {
        match self {
            Self::Google(service) => service.lists().await,
            Self::Microsoft(service) => service.lists().await,
            Self::CalDav(service) => service.lists().await,
            Self::Zoho(service) => service.lists().await,
        }
    }

    async fn add_list(&self, title: &str) -> Result<String> {
        match self {
            Self::Google(service) => service.add_list(title).await,
            Self::Microsoft(service) => service.add_list(title).await,
            Self::CalDav(service) => service.add_list(title).await,
            Self::Zoho(service) => service.add_list(title).await,
        }
    }

    async fn rename_list(&self, id: &str, title: &str) -> Result<()> {
        match self {
            Self::Google(service) => service.rename_list(id, title).await,
            Self::Microsoft(service) => service.rename_list(id, title).await,
            Self::CalDav(service) => service.rename_list(id, title).await,
            Self::Zoho(service) => service.rename_list(id, title).await,
        }
    }

    async fn delete_list(&self, id: &str) -> Result<()> {
        match self {
            Self::Google(service) => service.delete_list(id).await,
            Self::Microsoft(service) => service.delete_list(id).await,
            Self::CalDav(service) => service.delete_list(id).await,
            Self::Zoho(service) => service.delete_list(id).await,
        }
    }

    async fn pull(&self, list: &str, state: Option<&str>) -> Result<Pull> {
        match self {
            Self::Google(service) => service.pull(list, state).await,
            Self::Microsoft(service) => service.pull(list, state).await,
            Self::CalDav(service) => service.pull(list, state).await,
            Self::Zoho(service) => service.pull(list, state).await,
        }
    }

    /// Adds `task` to `list`, as a step of `parent` when it has one.
    async fn insert(&self, list: &str, task: &Task, parent: Option<&str>) -> Result<RemoteTask> {
        match self {
            Self::Google(service) => service.insert(list, task, parent).await,
            Self::Microsoft(service) => service.insert(list, task, parent).await,
            Self::CalDav(service) => service.insert(list, task, parent).await,
            Self::Zoho(service) => service.insert(list, task, parent).await,
        }
    }

    /// Changes task `id`; `Ok(None)` when the service no longer has it.
    async fn update(&self, list: &str, id: &str, task: &Task) -> Result<Option<RemoteTask>> {
        match self {
            Self::Google(service) => service.update(list, id, task).await,
            Self::Microsoft(service) => service.update(list, id, task).await,
            Self::CalDav(service) => service.update(list, id, task).await,
            Self::Zoho(service) => service.update(list, id, task).await,
        }
    }

    /// Moves task `id` right after task `after` of `list`, or first, where
    /// the service keeps the order (Google Tasks). `Ok(None)` when it
    /// keeps none, or no longer has the task.
    async fn place(&self, list: &str, id: &str, after: Option<&str>) -> Result<Option<RemoteTask>> {
        match self {
            Self::Google(service) => service.place(list, id, after).await,
            // To Do, CalDAV and Zoho keep no order Katna can set: it
            // stays here.
            Self::Microsoft(_) | Self::CalDav(_) | Self::Zoho(_) => Ok(None),
        }
    }

    /// The content of file `file` of task `task`, for a service that
    /// lists files without it (To Do); `None` when it is gone.
    async fn file_data(&self, list: &str, task: &str, file: &str) -> Result<Option<Vec<u8>>> {
        match self {
            Self::Microsoft(service) => service.file_data(list, task, file).await,
            Self::Google(_) | Self::CalDav(_) | Self::Zoho(_) => Ok(None),
        }
    }

    async fn delete(&self, list: &str, id: &str) -> Result<()> {
        match self {
            Self::Google(service) => service.delete(list, id).await,
            Self::Microsoft(service) => service.delete(list, id).await,
            Self::CalDav(service) => service.delete(list, id).await,
            Self::Zoho(service) => service.delete(list, id).await,
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
            // A list the service refuses (Zoho makes none of one's own)
            // stays here, logged, rather than stopping the round.
            (false, None) => match service.add_list(&list.title).await {
                Ok(remote) => store.lock().unwrap().task_list_pushed(list.id, &remote)?,
                Err(Error::Rejected(err)) => {
                    tracing::warn!(list = list.id, %err, "the task service refused a new list");
                }
                Err(err) => return Err(err),
            },
            (false, Some(remote)) => match service.rename_list(remote, &list.title).await {
                Ok(()) => store.lock().unwrap().task_list_pushed(list.id, remote)?,
                Err(Error::Rejected(err)) => {
                    tracing::warn!(list = list.id, %err, "the task service refused a new name");
                }
                Err(err) => return Err(err),
            },
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
        push_files(service, store, list, &remote).await?;
        let state = store.lock().unwrap().task_list_sync_state(list)?;
        let pull = service.pull(&remote, state.as_deref()).await?;
        {
            let mut store = store.lock().unwrap();
            changed |= store.drop_unlisted_steps(list, &pull.steps_of, &pull.tasks)?;
            changed |= store.sync_tasks(list, &pull.tasks, pull.all)?;
            store.set_task_list_sync_state(list, pull.state.as_deref())?;
        }
        for task in pull.tasks.iter().filter(|t| !t.deleted) {
            let Some(files) = &task.files else { continue };
            changed |= pull_files(service, store, list, &remote, &task.remote_id, files).await?;
        }
    }
    Ok(changed)
}

/// Takes the service's files of task `task` (its ID there): those Katna
/// lacks come in, reading their content when the list came without it,
/// and those gone from the service go. Returns whether any changed.
async fn pull_files(
    service: &TaskService,
    store: &Mutex<Store>,
    list: i64,
    remote_list: &str,
    task: &str,
    files: &[RemoteFile],
) -> Result<bool> {
    let synced = store.lock().unwrap().sync_task_files(list, task, files)?;
    let mut changed = synced.changed;
    for file in synced.wanted {
        let Some(data) = service
            .file_data(remote_list, task, &file.remote_id)
            .await?
        else {
            continue;
        };
        let file = RemoteFile {
            data: Some(data),
            ..file
        };
        changed |= store
            .lock()
            .unwrap()
            .add_remote_task_file(list, task, &file)?;
    }
    Ok(changed)
}

/// Sends the files added to and removed from the tasks of list `list`
/// here. One the service can't keep (too large, refused, or a service
/// without files) stays on this computer.
async fn push_files(
    service: &TaskService,
    store: &Mutex<Store>,
    list: i64,
    remote_list: &str,
) -> Result<()> {
    let pending = store.lock().unwrap().pending_task_files(list)?;
    if pending.is_empty() {
        return Ok(());
    }
    match service {
        TaskService::Microsoft(todo) => {
            for change in pending {
                let PendingFile {
                    file,
                    task_remote,
                    deleted,
                } = change;
                if deleted {
                    if let Some(remote) = &file.remote_id {
                        todo.delete_file(remote_list, &task_remote, remote).await?;
                    }
                    store.lock().unwrap().forget_task_file(file.id)?;
                    continue;
                }
                let data = store.lock().unwrap().task_file_data(file.id)?;
                let Some(data) = data else { continue };
                let sent = todo
                    .add_file(remote_list, &task_remote, &file.name, &file.mime, &data)
                    .await;
                let remote = match sent {
                    Ok(remote) => remote,
                    Err(Error::Rejected(err)) => {
                        tracing::warn!(file = file.id, %err, "To Do refused a file");
                        None
                    }
                    Err(err) => return Err(err),
                };
                store
                    .lock()
                    .unwrap()
                    .task_file_pushed(file.id, remote.as_deref())?;
            }
        }
        TaskService::CalDav(dav) => {
            // The files go inside the to-do: all of a task's at once.
            let mut by_task: BTreeMap<String, Vec<PendingFile>> = BTreeMap::new();
            for change in pending {
                by_task
                    .entry(change.task_remote.clone())
                    .or_default()
                    .push(change);
            }
            for (task_remote, changes) in by_task {
                let task = changes[0].file.task;
                let mut inline = Vec::new();
                let mut sending = Vec::new();
                {
                    let mut store = store.lock().unwrap();
                    for file in store.files_of_task(task)? {
                        let new = file.remote_id.is_none();
                        if file.local_only || (new && file.size > caldav::MAX_FILE) {
                            if new && !file.local_only {
                                store.task_file_pushed(file.id, None)?;
                            }
                            continue;
                        }
                        let Some(data) = store.task_file_data(file.id)? else {
                            continue;
                        };
                        sending.push((file.id, new.then(|| caldav::inline_id(&data))));
                        inline.push(katna_dav::todo::TodoFile {
                            name: file.name,
                            mime: file.mime,
                            data,
                        });
                    }
                }
                let kept = dav.set_files(remote_list, &task_remote, &inline).await?;
                let mut store = store.lock().unwrap();
                for change in changes.iter().filter(|c| c.deleted) {
                    store.forget_task_file(change.file.id)?;
                }
                match kept {
                    Some(true) => {
                        for (id, remote) in sending {
                            if let Some(remote) = remote {
                                store.task_file_pushed(id, Some(&remote))?;
                            }
                        }
                    }
                    // Refused or dropped: all of the task's files stay
                    // here, also those the server had before.
                    Some(false) => {
                        for (id, _) in sending {
                            store.task_file_pushed(id, None)?;
                        }
                    }
                    // The to-do went meanwhile; the next pull says so.
                    None => {}
                }
            }
        }
        TaskService::Google(_) | TaskService::Zoho(_) => {
            let mut store = store.lock().unwrap();
            for change in pending {
                if change.deleted {
                    store.forget_task_file(change.file.id)?;
                } else {
                    store.task_file_pushed(change.file.id, None)?;
                }
            }
        }
    }
    Ok(())
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
        let insert = || service.insert(list, &pending.task, pending.parent_remote.as_deref());
        let mut remote = match &pending.remote_id {
            // Only its place changed: nothing else to send.
            Some(remote) if !pending.edited => RemoteTask {
                remote_id: remote.clone(),
                etag: pending.etag.clone().unwrap_or_default(),
                ..RemoteTask::default()
            },
            Some(remote) => match service.update(list, remote, &pending.task).await? {
                Some(done) => done,
                // Deleted on the service while changed here: the change
                // brings it back.
                None => insert().await?,
            },
            None => insert().await?,
        };
        // Dragged to a new place here: after the task before it there.
        let placed = match &pending.place {
            None => true,
            Some(Place::Waiting) => false,
            Some(place) => {
                let after = match place {
                    Place::After(after) => Some(after.as_str()),
                    _ => None,
                };
                match service.place(list, &remote.remote_id, after).await {
                    Ok(Some(moved)) => remote = moved,
                    Ok(None) => {}
                    // The task before it went meanwhile: the next pull
                    // shows where Google put it.
                    Err(Error::Rejected(err)) => {
                        tracing::warn!(id, %err, "the task service refused a new place");
                    }
                    Err(err) => return Err(err),
                }
                true
            }
        };
        store
            .lock()
            .unwrap()
            .task_pushed(id, pending.stamp, &remote, placed)?;
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
