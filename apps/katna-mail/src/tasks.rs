// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Tasks' data: the task lists read from the store, and the changes
//! the Tasks page sends to the daemon over `in.invenia.katna.Agenda1`
//! (`docs/ARCHITECTURE.md` §18.1). No GPUI here.

use std::collections::{HashMap, HashSet};

use futures_lite::{Stream, StreamExt};
use katna_core::{AccountId, OAuthProvider, Paths};
use katna_dbus::PimProxy;
use katna_dbus::agenda::{AgendaProxy, Item, edit, task};
use katna_dbus::zbus::Connection;
use katna_dbus::zbus::zvariant::{OwnedValue, Value};
use katna_store::tasks::{Task, TaskFile, TaskList};
use katna_store::{Mode, Store};

use crate::daemon::{AccountState, describe};

/// A task's ID on the wire (`t` and its row ID).
pub fn wire_id(id: i64) -> String {
    format!("t{id}")
}

/// A list and its tasks, as the page shows them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub list: TaskList,
    /// The account's address; empty for a list on this computer.
    pub account: String,
    /// A Microsoft To Do list: To Do makes a repeating task's next one
    /// itself, so Katna doesn't move it to its next day.
    pub to_do: bool,
    /// Each task followed by its steps.
    pub tasks: Vec<Task>,
}

/// Everything the Tasks page shows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Board {
    pub columns: Vec<Column>,
    /// The files on each task, by task.
    pub files: HashMap<i64, Vec<TaskFile>>,
    /// Every label on a task or a note, in order of name: the one set the
    /// label picker offers.
    pub labels: Vec<String>,
}

impl Board {
    pub fn task(&self, id: i64) -> Option<&Task> {
        self.columns
            .iter()
            .flat_map(|c| c.tasks.iter())
            .find(|t| t.id == id)
    }

    /// The files on task `id`.
    pub fn files_of(&self, id: i64) -> &[TaskFile] {
        self.files.get(&id).map_or(&[], Vec::as_slice)
    }

    /// The labels on tasks, each once, in order of name: the side list's
    /// Labels.
    pub fn task_labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = Vec::new();
        for task in self.columns.iter().flat_map(|c| c.tasks.iter()) {
            for label in &task.labels {
                if !labels.contains(label) {
                    labels.push(label.clone());
                }
            }
        }
        labels.sort_by_key(|l| l.to_lowercase());
        labels
    }

    /// The steps of task `id`.
    pub fn steps(&self, id: i64) -> Vec<Task> {
        self.columns
            .iter()
            .flat_map(|c| c.tasks.iter())
            .filter(|t| t.parent == Some(id))
            .cloned()
            .collect()
    }
}

/// Reads every list and task, but those of the `hidden` accounts. Opens
/// its own connection, for a background thread.
pub fn load(paths: &Paths, hidden: &HashSet<AccountId>) -> Result<Board, String> {
    let read = || -> katna_store::Result<Board> {
        let store = Store::open(paths, Mode::ReadOnly)?;
        let accounts = store.accounts()?;
        let to_do: HashSet<AccountId> = accounts
            .iter()
            .filter(|a| {
                store
                    .account_settings(a.id)
                    .ok()
                    .flatten()
                    .and_then(|s| s.oauth)
                    == Some(OAuthProvider::Microsoft)
            })
            .map(|a| a.id)
            .collect();
        let addresses: HashMap<AccountId, String> =
            accounts.into_iter().map(|a| (a.id, a.address)).collect();
        let mut columns = Vec::new();
        for list in store.task_lists()? {
            if list.account.is_some_and(|a| hidden.contains(&a)) {
                continue;
            }
            let tasks = store.tasks_in(list.id)?;
            let account = list
                .account
                .and_then(|a| addresses.get(&a).cloned())
                .unwrap_or_default();
            columns.push(Column {
                to_do: list.account.is_some_and(|a| to_do.contains(&a)),
                list,
                account,
                tasks,
            });
        }
        let mut files: HashMap<i64, Vec<TaskFile>> = HashMap::new();
        for file in store.task_files()? {
            files.entry(file.task).or_default().push(file);
        }
        let labels = store.labels_in_use()?;
        Ok(Board {
            columns,
            files,
            labels,
        })
    };
    read().map_err(|err| katna_i18n::tr!("store-tasks-failed", error = err.to_string()))
}

/// Where each account's task sync stands, by account.
pub async fn status(connection: &Connection) -> Result<HashMap<i64, AccountState>, String> {
    let pim = PimProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let accounts = pim.accounts().await.map_err(|err| describe(&err))?;
    let status = pim.tasks_status().await.map_err(|err| describe(&err))?;
    Ok(status
        .into_iter()
        .map(|(id, state, detail)| {
            let sign_in = accounts
                .iter()
                .find(|a| a.id == id)
                .and_then(|a| a.sign_in.parse().ok());
            let state = AccountState {
                state,
                detail,
                sign_in,
            };
            (id, state)
        })
        .collect())
}

/// Fields of a task to change; `None` leaves one as it is.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskEdit {
    pub title: Option<String>,
    pub notes: Option<String>,
    pub due: Option<String>,
    /// `Some(None)` takes the time away.
    pub due_time: Option<Option<u32>>,
    pub remind_at: Option<Option<i64>>,
    pub repeat: Option<String>,
    pub starred: Option<bool>,
    pub labels: Option<Vec<String>>,
}

impl TaskEdit {
    fn item(&self) -> Item {
        let mut item = Item::new();
        let mut put = |key: &str, value: Value<'_>| {
            if let Ok(value) = OwnedValue::try_from(value) {
                item.insert(key.to_owned(), value);
            }
        };
        if let Some(title) = &self.title {
            put(edit::TITLE, title.as_str().into());
        }
        if let Some(notes) = &self.notes {
            put(edit::NOTES, notes.as_str().into());
        }
        if let Some(due) = &self.due {
            put(edit::DUE, due.as_str().into());
        }
        if let Some(time) = self.due_time {
            let minutes = time.and_then(|m| i32::try_from(m).ok()).unwrap_or(-1);
            put(edit::DUE_TIME, minutes.into());
        }
        if let Some(at) = self.remind_at {
            put(edit::REMIND_AT, at.unwrap_or(0).into());
        }
        if let Some(repeat) = &self.repeat {
            put(edit::REPEAT, repeat.as_str().into());
        }
        if let Some(starred) = self.starred {
            put(edit::STARRED, starred.into());
        }
        if let Some(labels) = &self.labels {
            put(edit::LABELS, labels.clone().into());
        }
        item
    }

    /// Every field of `task`, to put it back as it was.
    pub fn all_of(task: &Task) -> Self {
        Self {
            title: Some(task.title.clone()),
            notes: Some(task.notes.clone()),
            due: Some(task.due.clone()),
            due_time: Some(task.due_time),
            remind_at: Some(task.remind_at),
            repeat: Some(task.repeat.clone()),
            starred: Some(task.starred),
            labels: Some(task.labels.clone()),
        }
    }
}

/// A file to put on a task: dropped on it, picked, or kept from a mail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFile {
    pub name: String,
    pub mime: String,
    pub data: Vec<u8>,
}

/// A change the Tasks page sends to the daemon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskCommand {
    /// Adds a task on top of list `list` (0: the default list), as a step
    /// of `parent`, made from the mail with Message-ID `mail` if not empty.
    Add {
        list: i64,
        parent: Option<i64>,
        title: String,
        /// `YYYY-MM-DD`, or empty.
        due: String,
        mail: String,
    },
    /// Takes back the newest open task made from each of these mails (Undo
    /// of Add to Tasks).
    RemoveFromMail(Vec<String>),
    SetDone(i64, bool),
    Edit(i64, TaskEdit),
    /// Deletes a task with its steps.
    Delete(i64),
    /// Puts a deleted task back in its list, with its steps and files.
    Restore {
        task: Task,
        steps: Vec<Task>,
        files: Vec<NewFile>,
    },
    Move(i64, i64),
    /// Puts a task in list `list` (its own or another) right after task
    /// `after`, or first: where it was dragged to.
    Place {
        id: i64,
        list: i64,
        after: Option<i64>,
    },
    /// Adds a task ([`TaskCommand::Add`]), then gives it these fields: what
    /// typed quick add found beyond a due day.
    AddThen(Box<TaskCommand>, TaskEdit),
    /// Adds a task ([`TaskCommand::Add`]), then puts these files on it:
    /// the attachments of the mail it is made from.
    AddWithFiles(Box<TaskCommand>, Vec<NewFile>),
    /// Puts files on a task.
    AddFiles(i64, Vec<NewFile>),
    /// Takes a file off its task.
    RemoveFile(i64),
    /// Adds a list to an account, or to this computer.
    AddList(Option<AccountId>, String),
    RenameList(i64, String),
    DeleteList(i64),
}

/// Sends `command`. Returns the ID of a task it added, if any.
pub async fn send(connection: &Connection, command: &TaskCommand) -> Result<Option<i64>, String> {
    let agenda = AgendaProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let row = |id: String| id.strip_prefix('t').and_then(|n| n.parse::<i64>().ok());
    let add_files = async |id: &str, files: &[NewFile]| {
        for file in files {
            agenda
                .add_task_file(id, &file.name, &file.mime, &file.data)
                .await?;
        }
        Ok::<_, katna_dbus::zbus::Error>(())
    };
    let result = match command {
        TaskCommand::AddWithFiles(add, files) => {
            let id = Box::pin(send(connection, add)).await?;
            if let Some(id) = id {
                add_files(&wire_id(id), files)
                    .await
                    .map_err(|err| describe(&err))?;
            }
            return Ok(id);
        }
        TaskCommand::AddFiles(id, files) => add_files(&wire_id(*id), files).await,
        TaskCommand::RemoveFile(file) => agenda.remove_task_file(*file).await,
        TaskCommand::AddThen(add, fields) => {
            let id = Box::pin(send(connection, add)).await?;
            if let Some(id) = id
                && *fields != TaskEdit::default()
            {
                agenda
                    .edit_task(&wire_id(id), fields.item())
                    .await
                    .map_err(|err| describe(&err))?;
            }
            return Ok(id);
        }
        TaskCommand::Add {
            list,
            parent,
            title,
            due,
            mail,
        } => {
            let parent = parent.map(wire_id).unwrap_or_default();
            let added = async {
                let id = agenda.add_task_to(*list, &parent, title).await?;
                let mut fields = Item::new();
                for (key, value) in [(edit::MAIL, mail), (edit::DUE, due)] {
                    if !value.is_empty() {
                        fields.insert(
                            key.to_owned(),
                            OwnedValue::try_from(Value::from(value.as_str()))
                                .map_err(katna_dbus::zbus::Error::Variant)?,
                        );
                    }
                }
                if !fields.is_empty() {
                    agenda.edit_task(&id, fields).await?;
                }
                Ok::<_, katna_dbus::zbus::Error>(id)
            }
            .await;
            return added.map(row).map_err(|err| describe(&err));
        }
        TaskCommand::RemoveFromMail(mails) => {
            let tasks = agenda.tasks().await.map_err(|err| describe(&err))?;
            let text = |item: &Item, key: &str| {
                item.get(key)
                    .and_then(|v| String::try_from(v.try_clone().ok()?).ok())
                    .unwrap_or_default()
            };
            let open = |item: &Item| {
                !item
                    .get(task::DONE)
                    .is_some_and(|v| bool::try_from(v).unwrap_or(false))
            };
            for mail in mails {
                let newest = tasks
                    .iter()
                    .filter(|item| open(item) && text(item, task::MAIL) == *mail)
                    .filter_map(|item| row(text(item, task::ID)))
                    .max();
                if let Some(id) = newest {
                    agenda
                        .delete_task(&wire_id(id))
                        .await
                        .map_err(|err| describe(&err))?;
                }
            }
            Ok(())
        }
        TaskCommand::SetDone(id, done) => agenda.set_task_done(&wire_id(*id), *done).await,
        TaskCommand::Edit(id, fields) => agenda.edit_task(&wire_id(*id), fields.item()).await,
        TaskCommand::Delete(id) => agenda.delete_task(&wire_id(*id)).await,
        TaskCommand::Restore { task, steps, files } => {
            let restore = async |task: &Task, parent: Option<&str>| {
                let id = agenda
                    .add_task_to(task.list, parent.unwrap_or_default(), &task.title)
                    .await?;
                agenda.edit_task(&id, TaskEdit::all_of(task).item()).await?;
                if task.done_at.is_some() {
                    agenda.set_task_done(&id, true).await?;
                }
                Ok::<_, katna_dbus::zbus::Error>(id)
            };
            let result = async {
                let id = restore(task, None).await?;
                for step in steps {
                    restore(step, Some(&id)).await?;
                }
                add_files(&id, files).await?;
                Ok(id)
            }
            .await;
            return result.map(row).map_err(|err| describe(&err));
        }
        TaskCommand::Move(id, list) => agenda.move_task(&wire_id(*id), *list).await,
        TaskCommand::Place { id, list, after } => {
            let after = after.map(wire_id).unwrap_or_default();
            agenda.place_task(&wire_id(*id), *list, &after).await
        }
        TaskCommand::AddList(account, title) => {
            let account = account.map_or(0, |a| a.0);
            return agenda
                .add_task_list(account, title)
                .await
                .map(|_| None)
                .map_err(|err| describe(&err));
        }
        TaskCommand::RenameList(list, title) => agenda.rename_task_list(*list, title).await,
        TaskCommand::DeleteList(list) => agenda.delete_task_list(*list).await,
    };
    result.map(|()| None).map_err(|err| describe(&err))
}

/// Yields each time the daemon's tasks change.
pub async fn changes(connection: &Connection) -> Result<impl Stream<Item = ()>, String> {
    let agenda = AgendaProxy::new(connection)
        .await
        .map_err(|err| describe(&err))?;
    let changes = agenda
        .receive_changed()
        .await
        .map_err(|err| describe(&err))?;
    Ok(changes.map(|_| ()))
}
