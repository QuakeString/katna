// SPDX-License-Identifier: GPL-3.0-or-later

//! Zoho's tasks (the Tasks of Zoho Mail, also shown in Zoho ToDo) through
//! the Zoho Mail Tasks API, for accounts signed in with Zoho, scope
//! [`SCOPE`]. Zoho's CalDAV server keeps no to-dos.
//!
//! Lists: the personal tasks (`me`) are the default list, and each group
//! with tasks in the organisation is a list of its own (`group:<zgid>`).
//! Zoho makes no lists of one's own, so a list made in Katna stays on this
//! computer (the service refuses it); renaming or deleting a Zoho list in
//! Katna is not sent, and the next round brings it back as Zoho has it.
//!
//! Zoho keeps a task's title, description (notes), due day, whether it is
//! done, its priority (high is the star) and one level of subtasks
//! (steps). Its reminder and repeat are not mapped: a due time, reminders,
//! repeat, labels and files stay in Katna ([`RemoteTask::extras`] is
//! `None`), as with Google Tasks.
//! Zoho has no list of changes, so every pull reads the whole list; a
//! task's subtasks are read when Zoho says it has some.
//!
//! Each field has its own `PUT` in Zoho's API reference, so a change
//! reads the task first and sends only the fields that differ.

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use katna_store::tasks::{RemoteTask, RemoteTaskList, Task};
use serde::Deserialize;
use serde_json::{Map, Value, json};

use super::{Pull, gone, segment, unix};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply, form_encode},
    net::Tls,
    oauth::TokenSource,
};

/// Zoho Mail's scope for tasks, read and write.
const SCOPE: &str = crate::oauth::ZOHO_TASKS;

/// The remote ID of the personal tasks, the default list.
pub const PERSONAL: &str = "me";

/// The personal list's title: Zoho's own name for it.
const PERSONAL_TITLE: &str = "Personal Tasks";

/// Group lists' remote IDs start with this, then Zoho's group ID.
const GROUP: &str = "group:";

const TIMEOUT: Duration = Duration::from_secs(60);

/// Tasks asked for in one page (Zoho allows 1 to 499).
const PAGE: usize = 200;

/// Zoho's own apps write a due day this way.
const DUE_FORMAT: &str = "%d/%m/%Y";

/// One Zoho account's tasks.
pub struct ZohoTasks {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// `https://mail.zoho.<dc>/api`, or a server under test.
    api: String,
    /// Zoho answered a call with this sign-in: no need to ask again.
    let_in: AtomicBool,
}

/// Zoho's answer: `{"status": {...}, "data": ...}`.
#[derive(Deserialize, Default)]
struct Envelope {
    #[serde(default)]
    status: Status,
    #[serde(default)]
    data: Value,
}

#[derive(Deserialize, Default)]
struct Status {
    #[serde(default)]
    code: u16,
    #[serde(default)]
    description: String,
}

#[derive(Deserialize, Default)]
struct Item {
    #[serde(default)]
    id: Value,
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    status: String,
    #[serde(rename = "dueDate", default)]
    due: Value,
    #[serde(rename = "modifiedTime", default)]
    modified: Value,
    #[serde(rename = "numberOfSubtasks", default)]
    subtask_count: Value,
    #[serde(default)]
    subtasks: Vec<Value>,
    #[serde(rename = "parentTaskId", default)]
    parent: Value,
    #[serde(default)]
    priority: Value,
}

#[derive(Deserialize, Default)]
struct Group {
    #[serde(default)]
    id: Value,
    #[serde(default)]
    name: String,
}

/// A number or a string as text: Zoho writes IDs either way.
fn text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        _ => String::new(),
    }
}

/// `YYYY-MM-DD` of Zoho's due day: `DD/MM/YYYY` as documented, or an
/// ISO day or time; empty for none.
fn due_day(due: &Value) -> String {
    let due = text(due);
    let due = due.trim();
    if let Ok(date) = jiff::civil::Date::strptime(DUE_FORMAT, due) {
        return date.to_string();
    }
    due.get(..10)
        .filter(|day| day.parse::<jiff::civil::Date>().is_ok())
        .map(str::to_owned)
        .unwrap_or_default()
}

/// A `YYYY-MM-DD` day as Zoho's `DD/MM/YYYY`; empty for none.
fn zoho_due(day: &str) -> String {
    day.parse::<jiff::civil::Date>()
        .map(|date| date.strftime(DUE_FORMAT).to_string())
        .unwrap_or_default()
}

/// Whether Zoho's priority (`High`, `high`) is the star.
fn is_high(priority: &Value) -> bool {
    text(priority).eq_ignore_ascii_case("high")
}

/// Zoho's priority for the star: high, or none. Sent only when the star
/// changed, so an unstarred task keeps Zoho's other priorities.
fn priority_of(starred: bool) -> &'static str {
    if starred { "high" } else { "none" }
}

/// Whether Zoho's status (`Completed`, `completed`) means done.
fn is_done(status: &str) -> bool {
    status.eq_ignore_ascii_case("completed")
}

impl Item {
    fn has_subtasks(&self) -> bool {
        !self.subtasks.is_empty()
            || match &self.subtask_count {
                Value::Number(n) => n.as_u64().unwrap_or(0) > 0,
                Value::String(n) => n.parse::<u64>().unwrap_or(0) > 0,
                _ => false,
            }
    }

    /// As a [`RemoteTask`], a step of `parent` when given.
    fn into_remote(self, parent: Option<&str>) -> RemoteTask {
        let modified = text(&self.modified);
        let due = due_day(&self.due);
        let done = is_done(&self.status);
        let starred = is_high(&self.priority);
        // Zoho says when a task last changed; without that, what it keeps
        // stands in for an etag.
        let etag = if modified.is_empty() {
            format!(
                "{}|{}|{}|{}|{}",
                self.title, self.description, due, done, starred
            )
        } else {
            modified.clone()
        };
        let parent = parent.map(str::to_owned).or_else(|| {
            let own = text(&self.parent);
            (!own.is_empty() && own != "0").then_some(own)
        });
        RemoteTask {
            remote_id: text(&self.id),
            parent,
            deleted: false,
            title: self.title,
            notes: self.description,
            due,
            // Zoho says not when it was ticked; its last change will do.
            done_at: done.then(|| unix(&modified).unwrap_or(0)),
            position: String::new(),
            etag,
            extras: None,
            starred: Some(starred),
            labels: None,
            files: None,
        }
    }
}

impl ZohoTasks {
    /// Zoho's Mail API at `api` (`https://mail.zoho.<dc>/api`, the account's
    /// data centre's), or the server under test in `KATNA_ZOHO_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        let test = http::test_url("KATNA_ZOHO_API_URL");
        Self::with_api(tokens, tls, test.as_deref().unwrap_or(api))
    }

    /// Talks to `api` (the `…/api` root) instead of Zoho, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
            let_in: AtomicBool::new(false),
        }
    }

    /// Whether the account's sign-in allowed Katna into its tasks: not
    /// when the grant names its scopes without [`SCOPE`], or when Zoho
    /// refuses the token or its scope. Zoho's token answers may name no
    /// scopes, so the first time it is asked with one small read.
    pub async fn allowed(&self) -> Result<bool> {
        if self.tokens.granted(SCOPE) == Some(false) {
            return Ok(false);
        }
        if self.let_in.load(Ordering::Relaxed) {
            return Ok(true);
        }
        let url = format!(
            "{}?{}",
            self.list_url(PERSONAL),
            form_encode(&[("from", "0"), ("limit", "1")])
        );
        let reply = match self.call("GET", &url, None).await {
            Err(Error::Auth(_)) => return Ok(false),
            other => other?,
        };
        match check(&reply, "reading tasks") {
            Ok(()) => {
                self.let_in.store(true, Ordering::Relaxed);
                Ok(true)
            }
            Err(Error::Auth(_)) => Ok(false),
            Err(err) => Err(err),
        }
    }

    /// Sends a request with the account's access token, trying once more
    /// with a fresh token when Zoho refuses the one it had (once: a
    /// fresh token refused too is the answer).
    async fn call(&self, method: &str, url: &str, body: Option<&[u8]>) -> Result<Reply> {
        let mut retried = false;
        loop {
            let token = format!("Zoho-oauthtoken {}", self.tokens.access_token().await?);
            let headers = [
                ("Authorization", token.as_str()),
                ("Accept", "application/json"),
            ];
            let body = body.map(|b| ("application/json", b));
            let reply =
                http::exchange(method, url, &headers, body, None, &self.tls, TIMEOUT).await?;
            if reply.status == 401 && !retried && self.tokens.forget_access_token() {
                retried = true;
                continue;
            }
            return Ok(reply);
        }
    }

    /// The URL of list `list`'s tasks: `…/tasks/me` or
    /// `…/tasks/groups/<zgid>`.
    fn list_url(&self, list: &str) -> String {
        match list.strip_prefix(GROUP) {
            Some(group) => format!("{}/tasks/groups/{}", self.api, segment(group)),
            None => format!("{}/tasks/me", self.api),
        }
    }

    fn task_url(&self, list: &str, id: &str) -> String {
        format!("{}/{}", self.list_url(list), segment(id))
    }

    /// The personal list, then each group that has tasks.
    pub async fn lists(&self) -> Result<Vec<RemoteTaskList>> {
        let mut lists = vec![RemoteTaskList {
            remote_id: PERSONAL.to_owned(),
            title: PERSONAL_TITLE.to_owned(),
            is_default: true,
        }];
        let reply = self
            .call("GET", &format!("{}/tasks/groups", self.api), None)
            .await?;
        // An account outside an organisation may have no groups at all.
        if gone(&reply) && !auth_failure(&reply) {
            return Ok(lists);
        }
        let data = match parse(&reply, "reading the task groups") {
            Ok(data) => data,
            Err(Error::Rejected(err)) => {
                tracing::warn!(%err, "no Zoho task groups");
                return Ok(lists);
            }
            Err(err) => return Err(err),
        };
        let groups: Vec<Group> = serde_json::from_value(data["groups"].clone())
            .map_err(|err| protocol("reading the task groups", err))?;
        lists.extend(groups.into_iter().filter_map(|group| {
            let id = text(&group.id);
            (!id.is_empty()).then(|| RemoteTaskList {
                remote_id: format!("{GROUP}{id}"),
                title: group.name,
                is_default: false,
            })
        }));
        Ok(lists)
    }

    /// Zoho makes no lists of one's own: refused, so the list stays on
    /// this computer.
    pub async fn add_list(&self, title: &str) -> Result<String> {
        Err(Error::Rejected(format!(
            "Zoho keeps no task lists of your own, so “{title}” stays on this computer"
        )))
    }

    /// Not sent: Zoho's lists are its personal tasks and its groups,
    /// whose names Katna doesn't change. The next pull brings Zoho's
    /// name back.
    pub async fn rename_list(&self, id: &str, _title: &str) -> Result<()> {
        tracing::info!(id, "Zoho task lists can't be renamed from Katna");
        Ok(())
    }

    /// Not sent, for the same reason; the next round brings the list and
    /// its tasks back.
    pub async fn delete_list(&self, id: &str) -> Result<()> {
        tracing::info!(id, "Zoho task lists can't be deleted from Katna");
        Ok(())
    }

    /// Every task of `list`, with their subtasks: Zoho says nothing of
    /// what changed, so `state` is not used.
    pub async fn pull(&self, list: &str, state: Option<&str>) -> Result<Pull> {
        let mut items = Vec::new();
        let mut from = 0;
        loop {
            let (from_text, limit) = (from.to_string(), PAGE.to_string());
            let url = format!(
                "{}?{}",
                self.list_url(list),
                form_encode(&[("from", &from_text), ("limit", &limit)])
            );
            let reply = self.call("GET", &url, None).await?;
            if gone(&reply) && !auth_failure(&reply) {
                // The group went meanwhile; the next round drops it.
                return Ok(Pull {
                    tasks: Vec::new(),
                    all: false,
                    state: state.map(str::to_owned),
                    steps_of: Vec::new(),
                });
            }
            let data = parse(&reply, "reading tasks")?;
            let page: Vec<Item> = serde_json::from_value(data["tasks"].clone())
                .map_err(|err| protocol("reading tasks", err))?;
            let got = page.len();
            items.extend(page);
            let more = !data["paging"]["nextPage"].is_null()
                && !text(&data["paging"]["nextPage"]).is_empty();
            if got == 0 || (!more && got < PAGE) {
                break;
            }
            from += got;
        }
        let mut tasks: Vec<RemoteTask> = Vec::new();
        let mut steps: Vec<RemoteTask> = Vec::new();
        for item in items {
            let id = text(&item.id);
            if id.is_empty() {
                continue;
            }
            if item.has_subtasks() {
                match self.subtasks(list, &id).await? {
                    Some(found) => steps.extend(found),
                    // Gone meanwhile: the next pull says so.
                    None => continue,
                }
            }
            tasks.push(item.into_remote(None));
        }
        // A subtask listed among the tasks too comes once, as a step.
        tasks.retain(|task| !steps.iter().any(|s| s.remote_id == task.remote_id));
        tasks.extend(steps);
        Ok(Pull {
            tasks,
            all: true,
            state: None,
            steps_of: Vec::new(),
        })
    }

    /// Task `task`'s subtasks; `None` when Zoho no longer has the task.
    async fn subtasks(&self, list: &str, task: &str) -> Result<Option<Vec<RemoteTask>>> {
        let url = format!("{}/subtasks", self.task_url(list, task));
        let reply = self.call("GET", &url, None).await?;
        if gone(&reply) && !auth_failure(&reply) {
            return Ok(None);
        }
        let data = parse(&reply, "reading subtasks")?;
        let items: Vec<Item> = serde_json::from_value(data["tasks"].clone())
            .map_err(|err| protocol("reading subtasks", err))?;
        Ok(Some(
            items
                .into_iter()
                .filter(|item| !text(&item.id).is_empty())
                .map(|item| item.into_remote(Some(task)))
                .collect(),
        ))
    }

    /// Task `id` as Zoho has it; `None` when it has it no more.
    async fn read(&self, list: &str, id: &str) -> Result<Option<Item>> {
        let reply = self.call("GET", &self.task_url(list, id), None).await?;
        if gone(&reply) && !auth_failure(&reply) {
            return Ok(None);
        }
        let data = parse(&reply, "reading a task")?;
        // Documented as `{"tasks": [task]}`; a bare task will do too.
        let item = match data.get("tasks").and_then(Value::as_array) {
            Some(tasks) => tasks.first().cloned().unwrap_or(Value::Null),
            None => data,
        };
        if item.is_null() {
            return Ok(None);
        }
        serde_json::from_value(item)
            .map(Some)
            .map_err(|err| protocol("reading a task", err))
    }

    /// Adds `task` to `list`, as a subtask of `parent` when it has one.
    pub async fn insert(
        &self,
        list: &str,
        task: &Task,
        parent: Option<&str>,
    ) -> Result<RemoteTask> {
        let mut body = Map::new();
        body.insert("title".into(), json!(task.title));
        if !task.notes.is_empty() {
            body.insert("description".into(), json!(task.notes));
        }
        let due = zoho_due(&task.due);
        if !due.is_empty() {
            body.insert("dueDate".into(), json!(due));
        }
        if task.done_at.is_some() {
            body.insert("status".into(), json!("completed"));
        }
        if task.starred {
            body.insert("priority".into(), json!(priority_of(true)));
        }
        if let Some(parent) = parent {
            body.insert("parentTaskId".into(), json!(parent));
        }
        let body = Value::Object(body).to_string();
        let reply = self
            .call("POST", &self.list_url(list), Some(body.as_bytes()))
            .await?;
        let data = parse(&reply, "adding a task")?;
        let item: Item =
            serde_json::from_value(data).map_err(|err| protocol("adding a task", err))?;
        if text(&item.id).is_empty() {
            return Err(Error::Protocol(
                "Zoho Tasks, adding a task: no ID came back".into(),
            ));
        }
        Ok(item.into_remote(parent))
    }

    /// Changes task `id`, sending each field that differs from Zoho's;
    /// `None` when Zoho no longer has it.
    pub async fn update(&self, list: &str, id: &str, task: &Task) -> Result<Option<RemoteTask>> {
        let Some(old) = self.read(list, id).await? else {
            return Ok(None);
        };
        let parent = {
            let own = text(&old.parent);
            (!own.is_empty() && own != "0").then_some(own)
        };
        let mut changes = Vec::new();
        if old.title != task.title {
            changes.push(json!({ "title": task.title }));
        }
        if old.description != task.notes {
            changes.push(json!({ "description": task.notes }));
        }
        if due_day(&old.due) != task.due {
            // An empty day asks Zoho to drop the due day.
            changes.push(json!({ "dueDate": zoho_due(&task.due) }));
        }
        if is_done(&old.status) != task.done_at.is_some() {
            let status = if task.done_at.is_some() {
                "completed"
            } else {
                "inprogress"
            };
            changes.push(json!({ "status": status }));
        }
        if is_high(&old.priority) != task.starred {
            changes.push(json!({ "priority": priority_of(task.starred) }));
        }
        if changes.is_empty() {
            return Ok(Some(old.into_remote(parent.as_deref())));
        }
        let url = self.task_url(list, id);
        for change in changes {
            let body = change.to_string();
            let reply = self.call("PUT", &url, Some(body.as_bytes())).await?;
            if gone(&reply) && !auth_failure(&reply) {
                return Ok(None);
            }
            check(&reply, "changing a task")?;
        }
        Ok(self
            .read(list, id)
            .await?
            .map(|item| item.into_remote(parent.as_deref())))
    }

    pub async fn delete(&self, list: &str, id: &str) -> Result<()> {
        let reply = self.call("DELETE", &self.task_url(list, id), None).await?;
        if gone(&reply) && !auth_failure(&reply) {
            return Ok(());
        }
        check(&reply, "deleting a task")
    }
}

/// Zoho's error codes that ask to sign in again: a token refused, or one
/// without the tasks scope.
const AUTH_CODES: [&str; 5] = [
    "INVALID_OAUTHTOKEN",
    "INVALID_OAUTHSCOPE",
    "OAUTH_SCOPE_MISMATCH",
    "INVALID_TICKET",
    "UNAUTHORIZED",
];

/// The error code in Zoho's answer (`data.errorCode`), if any.
fn error_code(reply: &Reply) -> String {
    let envelope: Envelope = serde_json::from_slice(&reply.body).unwrap_or_default();
    text(&envelope.data["errorCode"])
}

/// Whether the answer refuses the sign-in rather than an item: Zoho
/// answers a missing scope with 404 and `INVALID_OAUTHSCOPE`.
fn auth_failure(reply: &Reply) -> bool {
    let code = error_code(reply);
    reply.status == 401 || AUTH_CODES.iter().any(|c| code.eq_ignore_ascii_case(c))
}

/// What a failed answer means: a refused token or scope asks to sign in
/// again ([`Error::Auth`]); busy or down waits for the next round
/// ([`Error::Closed`]); anything else is Zoho's own words.
fn failure(reply: &Reply, status: u16, doing: &str) -> Error {
    if auth_failure(reply) || status == 401 {
        return Error::Auth(format!("Zoho Tasks refused access while {doing}"));
    }
    let envelope: Envelope = serde_json::from_slice(&reply.body).unwrap_or_default();
    let code = error_code(reply);
    let detail = match (envelope.status.description.is_empty(), code.is_empty()) {
        (true, true) => format!("status {status}"),
        (false, true) => envelope.status.description,
        (true, false) => code,
        (false, false) => format!("{} ({code})", envelope.status.description),
    };
    if status == 429 || status >= 500 {
        return Error::Closed(format!("Zoho Tasks, {doing}: {detail}"));
    }
    Error::Rejected(format!("Zoho Tasks, {doing}: {detail}"))
}

/// Checks an answer, also the status Zoho writes inside a 200 one.
fn check(reply: &Reply, doing: &str) -> Result<()> {
    if !(200..300).contains(&reply.status) {
        return Err(failure(reply, reply.status, doing));
    }
    let envelope: Envelope = serde_json::from_slice(&reply.body).unwrap_or_default();
    if envelope.status.code >= 400 {
        return Err(failure(reply, envelope.status.code, doing));
    }
    Ok(())
}

/// The `data` of a good answer.
fn parse(reply: &Reply, doing: &str) -> Result<Value> {
    check(reply, doing)?;
    let envelope: Envelope =
        serde_json::from_slice(&reply.body).map_err(|err| protocol(doing, err))?;
    Ok(envelope.data)
}

fn protocol(doing: &str, err: serde_json::Error) -> Error {
    Error::Protocol(format!("Zoho Tasks, {doing}: {err}"))
}
