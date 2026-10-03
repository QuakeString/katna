// SPDX-License-Identifier: GPL-3.0-or-later

//! Google Tasks (API v1) for Google accounts, scope [`GOOGLE_TASKS`].
//!
//! Google keeps a task's title, notes, due day (never a time), whether
//! it is done, its place in the list and one level of subtasks. A due
//! time, reminders, repeat and the star stay in Katna
//! ([`RemoteTask::extras`] is `None`). A pull asks for what changed since
//! the last one (`updatedMin`), and for everything once a day.

use std::{sync::Arc, time::Duration};

use katna_store::tasks::{RemoteTask, RemoteTaskList, Task};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Pull, date_part, gone, rfc3339, segment, unix};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply, form_encode},
    net::Tls,
    oauth::{GOOGLE_TASKS, TokenSource},
};

/// Google Tasks' host.
pub const TASKS_API: &str = "https://tasks.googleapis.com";

const TIMEOUT: Duration = Duration::from_secs(60);

/// A pull asks for changes since this long before the last one began,
/// in case the clocks differ; what comes twice changes nothing.
const OVERLAP: i64 = 5 * 60;

/// After this long, a pull asks for everything again, so nothing missed
/// stays missed.
const FULL_EVERY: i64 = 24 * 60 * 60;

/// Most items Google gives in one page.
const PAGE: &str = "100";

/// One Google account's task lists.
#[derive(Clone)]
pub struct GoogleTasks {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`TASKS_API`], or a server under test.
    api: String,
}

#[derive(Deserialize)]
struct Lists {
    #[serde(default)]
    items: Vec<ListItem>,
    #[serde(rename = "nextPageToken")]
    next: Option<String>,
}

#[derive(Deserialize)]
struct ListItem {
    id: String,
    #[serde(default)]
    title: String,
}

#[derive(Deserialize)]
struct Tasks {
    #[serde(default)]
    items: Vec<Item>,
    #[serde(rename = "nextPageToken")]
    next: Option<String>,
}

#[derive(Deserialize, Default)]
struct Item {
    id: String,
    #[serde(default)]
    etag: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    notes: String,
    #[serde(default)]
    status: String,
    due: Option<String>,
    completed: Option<String>,
    parent: Option<String>,
    #[serde(default)]
    position: String,
    #[serde(default)]
    deleted: bool,
}

impl From<Item> for RemoteTask {
    fn from(item: Item) -> Self {
        let done_at = (item.status == "completed")
            .then(|| item.completed.as_deref().and_then(unix).unwrap_or(0));
        Self {
            remote_id: item.id,
            parent: item.parent,
            deleted: item.deleted,
            title: item.title,
            notes: item.notes,
            due: item.due.as_deref().map(date_part).unwrap_or_default(),
            done_at,
            position: item.position,
            etag: item.etag,
            extras: None,
            starred: None,
            labels: None,
            files: None,
        }
    }
}

/// The fields Google keeps, as a request body.
fn body(task: &Task) -> Vec<u8> {
    let due = if task.due.is_empty() {
        Value::Null
    } else {
        // Google drops the time; midnight UTC is what its own apps send.
        Value::String(format!("{}T00:00:00.000Z", task.due))
    };
    let (status, completed) = match task.done_at {
        Some(at) => ("completed", Value::String(rfc3339(at))),
        None => ("needsAction", Value::Null),
    };
    json!({
        "title": task.title,
        "notes": task.notes,
        "due": due,
        "status": status,
        "completed": completed,
    })
    .to_string()
    .into_bytes()
}

impl GoogleTasks {
    /// Google's, or the server under test in `KATNA_GOOGLE_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GOOGLE_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(TASKS_API))
    }

    /// Talks to `api` instead of Google, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna into its tasks.
    pub async fn allowed(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_TASKS).await
    }

    /// Sends a request with the account's access token, trying once more
    /// with a fresh token when Google refuses the one it had.
    async fn call(&self, method: &str, path: &str, body: Option<&[u8]>) -> Result<Reply> {
        let url = format!("{}/tasks/v1/{path}", self.api);
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let headers = [("Authorization", token.as_str())];
            let body = body.map(|b| ("application/json; charset=UTF-8", b));
            let reply =
                http::exchange(method, &url, &headers, body, None, &self.tls, TIMEOUT).await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    pub async fn lists(&self) -> Result<Vec<RemoteTaskList>> {
        let mut all = Vec::new();
        let mut page: Option<String> = None;
        loop {
            let mut query = vec![("maxResults", PAGE)];
            if let Some(page) = &page {
                query.push(("pageToken", page));
            }
            let reply = self
                .call(
                    "GET",
                    &format!("users/@me/lists?{}", form_encode(&query)),
                    None,
                )
                .await?;
            let lists: Lists = parse(&reply, "reading the task lists")?;
            all.extend(lists.items);
            match lists.next {
                Some(next) if !next.is_empty() => page = Some(next),
                _ => break,
            }
        }
        let reply = self.call("GET", "users/@me/lists/@default", None).await?;
        let default: ListItem = parse(&reply, "reading the default task list")?;
        Ok(all
            .into_iter()
            .map(|list| RemoteTaskList {
                is_default: list.id == default.id,
                remote_id: list.id,
                title: list.title,
            })
            .collect())
    }

    pub async fn add_list(&self, title: &str) -> Result<String> {
        let body = json!({ "title": title }).to_string();
        let reply = self
            .call("POST", "users/@me/lists", Some(body.as_bytes()))
            .await?;
        let list: ListItem = parse(&reply, "adding a task list")?;
        Ok(list.id)
    }

    pub async fn rename_list(&self, id: &str, title: &str) -> Result<()> {
        let body = json!({ "title": title }).to_string();
        let reply = self
            .call(
                "PATCH",
                &format!("users/@me/lists/{}", segment(id)),
                Some(body.as_bytes()),
            )
            .await?;
        check(&reply, "renaming a task list")
    }

    pub async fn delete_list(&self, id: &str) -> Result<()> {
        let reply = self
            .call("DELETE", &format!("users/@me/lists/{}", segment(id)), None)
            .await?;
        if gone(&reply) {
            return Ok(());
        }
        check(&reply, "deleting a task list")
    }

    /// The tasks of `list` that changed since `state` (from the last
    /// pull), or all of them.
    pub async fn pull(&self, list: &str, state: Option<&str>) -> Result<Pull> {
        let started = jiff::Timestamp::now().as_second();
        let since = state
            .and_then(|state| state.parse::<i64>().ok())
            .filter(|since| started - since < FULL_EVERY);
        let updated_min = since.map(|since| rfc3339(since - OVERLAP));
        let mut tasks = Vec::new();
        let mut page: Option<String> = None;
        loop {
            // Tasks done in Google's own apps are hidden at once, so hidden
            // ones are asked for too.
            let mut query = vec![
                ("maxResults", PAGE),
                ("showCompleted", "true"),
                ("showHidden", "true"),
            ];
            if let Some(min) = &updated_min {
                query.push(("showDeleted", "true"));
                query.push(("updatedMin", min));
            }
            if let Some(page) = &page {
                query.push(("pageToken", page));
            }
            let reply = self
                .call(
                    "GET",
                    &format!("lists/{}/tasks?{}", segment(list), form_encode(&query)),
                    None,
                )
                .await?;
            if gone(&reply) {
                // The list went meanwhile; the next round drops it.
                return Ok(Pull {
                    tasks: Vec::new(),
                    all: false,
                    state: state.map(str::to_owned),
                    steps_of: Vec::new(),
                });
            }
            let answer: Tasks = parse(&reply, "reading tasks")?;
            tasks.extend(answer.items.into_iter().map(RemoteTask::from));
            match answer.next {
                Some(next) if !next.is_empty() => page = Some(next),
                _ => break,
            }
        }
        Ok(Pull {
            tasks,
            all: updated_min.is_none(),
            state: Some(started.to_string()),
            steps_of: Vec::new(),
        })
    }

    /// Adds `task` on top of `list`, as a subtask of `parent` if given.
    pub async fn insert(
        &self,
        list: &str,
        task: &Task,
        parent: Option<&str>,
    ) -> Result<RemoteTask> {
        let mut path = format!("lists/{}/tasks", segment(list));
        if let Some(parent) = parent {
            path.push_str(&format!("?{}", form_encode(&[("parent", parent)])));
        }
        let reply = self.call("POST", &path, Some(&body(task))).await?;
        let item: Item = parse(&reply, "adding a task")?;
        Ok(item.into())
    }

    /// Changes task `id`; `None` when Google no longer has it.
    pub async fn update(&self, list: &str, id: &str, task: &Task) -> Result<Option<RemoteTask>> {
        let path = format!("lists/{}/tasks/{}", segment(list), segment(id));
        let reply = self.call("PATCH", &path, Some(&body(task))).await?;
        if gone(&reply) {
            return Ok(None);
        }
        let item: Item = parse(&reply, "changing a task")?;
        Ok(Some(item.into()))
    }

    /// Moves task `id` (a task, not a subtask) right after task `after`
    /// of `list`, or first (`tasks.move` with `previous`); `None` when
    /// Google no longer has it.
    pub async fn place(
        &self,
        list: &str,
        id: &str,
        after: Option<&str>,
    ) -> Result<Option<RemoteTask>> {
        let mut path = format!("lists/{}/tasks/{}/move", segment(list), segment(id));
        if let Some(after) = after {
            path.push_str(&format!("?{}", form_encode(&[("previous", after)])));
        }
        let reply = self.call("POST", &path, None).await?;
        if gone(&reply) {
            return Ok(None);
        }
        let item: Item = parse(&reply, "moving a task")?;
        Ok(Some(item.into()))
    }

    pub async fn delete(&self, list: &str, id: &str) -> Result<()> {
        let path = format!("lists/{}/tasks/{}", segment(list), segment(id));
        let reply = self.call("DELETE", &path, None).await?;
        if gone(&reply) {
            return Ok(());
        }
        check(&reply, "deleting a task")
    }
}

#[derive(Deserialize, Default)]
struct Failure {
    #[serde(default)]
    error: FailureError,
}

#[derive(Deserialize, Default)]
struct FailureError {
    #[serde(default)]
    message: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    errors: Vec<Reason>,
}

#[derive(Deserialize)]
struct Reason {
    #[serde(default)]
    reason: String,
}

/// What a failed answer means: the Tasks API switched off for Katna's
/// Google Cloud project ([`Error::NotEnabled`]); a refused grant or scope
/// asks to sign in again ([`Error::Auth`]); anything else is Google's own
/// message.
fn failure(reply: &Reply, doing: &str) -> Error {
    if let Some(off) = crate::google_api::switched_off(reply.status, &reply.body) {
        return off;
    }
    let failure: Failure = serde_json::from_slice(&reply.body).unwrap_or_default();
    let scope = failure.error.status == "PERMISSION_DENIED"
        || failure.error.errors.iter().any(|r| {
            matches!(
                r.reason.as_str(),
                "insufficientPermissions" | "authError" | "ACCESS_TOKEN_SCOPE_INSUFFICIENT"
            )
        });
    if reply.status == 401 || (reply.status == 403 && scope) {
        return Error::Auth(format!("Google Tasks refused access while {doing}"));
    }
    let detail = if failure.error.message.is_empty() {
        format!("status {}", reply.status)
    } else {
        failure.error.message
    };
    if reply.status == 429 || reply.status >= 500 {
        // Busy or down: try again later, like a network failure.
        return Error::Closed(format!("Google Tasks, {doing}: {detail}"));
    }
    Error::Rejected(format!("Google Tasks, {doing}: {detail}"))
}

fn check(reply: &Reply, doing: &str) -> Result<()> {
    if (200..300).contains(&reply.status) {
        Ok(())
    } else {
        Err(failure(reply, doing))
    }
}

fn parse<T: for<'de> Deserialize<'de>>(reply: &Reply, doing: &str) -> Result<T> {
    check(reply, doing)?;
    serde_json::from_slice(&reply.body)
        .map_err(|err| Error::Protocol(format!("Google Tasks, {doing}: {err}")))
}
