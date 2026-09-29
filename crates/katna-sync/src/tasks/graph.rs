// SPDX-License-Identifier: GPL-3.0-or-later

//! Microsoft To Do through Microsoft Graph for Microsoft accounts, scope
//! [`MICROSOFT_TASKS`]; like OneDrive's, its tokens come separately.
//!
//! To Do keeps a task's title, notes, due day, whether it is done, a
//! reminder, repeat and importance (the star), so those come from To Do
//! ([`RemoteTask::extras`]). Steps are To Do's checklist items: a step's
//! ID here is `task|item`, and To Do keeps only its title and whether it
//! is ticked. A pull follows the list's delta link, so only changes come
//! after the first; each changed task's steps are read again with it.

use std::{sync::Arc, time::Duration};

use katna_store::tasks::{RemoteTask, RemoteTaskList, Task, TaskExtras};
use serde::Deserialize;
use serde_json::{Map, Value, json};

use super::{Pull, date_part, gone, segment, unix};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    net::Tls,
    oauth::{MICROSOFT_TASKS, TokenSource},
    onedrive::GRAPH_API,
};

const TIMEOUT: Duration = Duration::from_secs(60);

/// Graph writes times in UTC when asked this way.
const PREFER_UTC: &str = "outlook.timezone=\"UTC\"";

/// One Microsoft account's To Do lists.
#[derive(Clone)]
pub struct ToDo {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`GRAPH_API`], or a server under test.
    api: String,
}

#[derive(Deserialize)]
struct Page<T> {
    #[serde(default = "Vec::new")]
    value: Vec<T>,
    #[serde(rename = "@odata.nextLink")]
    next: Option<String>,
    #[serde(rename = "@odata.deltaLink")]
    delta: Option<String>,
}

#[derive(Deserialize)]
struct List {
    id: String,
    #[serde(rename = "displayName", default)]
    name: String,
    #[serde(rename = "wellknownListName", default)]
    wellknown: String,
}

#[derive(Deserialize, Default)]
struct DateTimeZone {
    #[serde(rename = "dateTime", default)]
    date_time: String,
    #[serde(rename = "timeZone", default)]
    time_zone: String,
}

#[derive(Deserialize, Default)]
struct Body {
    #[serde(default)]
    content: String,
}

#[derive(Deserialize, Default)]
struct Item {
    id: String,
    #[serde(rename = "@odata.etag", default)]
    etag: String,
    #[serde(rename = "@removed")]
    removed: Option<Value>,
    #[serde(default)]
    title: String,
    body: Option<Body>,
    #[serde(default)]
    status: String,
    #[serde(default)]
    importance: String,
    #[serde(rename = "dueDateTime")]
    due: Option<DateTimeZone>,
    #[serde(rename = "completedDateTime")]
    completed: Option<DateTimeZone>,
    #[serde(rename = "isReminderOn", default)]
    reminder_on: bool,
    #[serde(rename = "reminderDateTime")]
    reminder: Option<DateTimeZone>,
    recurrence: Option<Value>,
}

/// A task's checklist item: a step.
#[derive(Deserialize, Default)]
struct Check {
    id: String,
    #[serde(rename = "displayName", default)]
    name: String,
    #[serde(rename = "isChecked", default)]
    checked: bool,
    #[serde(rename = "checkedDateTime")]
    checked_at: Option<String>,
}

/// Joins a task's and a checklist item's IDs into a step's ID. Graph's
/// IDs are base64, so they never hold `|`.
fn step_id(task: &str, item: &str) -> String {
    format!("{task}|{item}")
}

/// The task and the checklist item of a step's ID; `None` for a task's.
fn split_step(id: &str) -> Option<(&str, &str)> {
    id.split_once('|')
}

/// Checklist item `check`, the `index`th of task `task`, as a step. A
/// step just sent takes `usize::MAX`: last, until the next pull places it.
fn step(task: &str, index: usize, check: Check) -> RemoteTask {
    let done_at = check
        .checked
        .then(|| check.checked_at.as_deref().and_then(unix).unwrap_or(0));
    RemoteTask {
        remote_id: step_id(task, &check.id),
        parent: Some(task.to_owned()),
        deleted: false,
        // To Do has no etag for a checklist item; what it keeps will do.
        etag: format!("{index}:{}:{}", check.checked, check.name),
        title: check.name,
        notes: String::new(),
        due: String::new(),
        done_at,
        position: format!("{index:08}"),
        extras: None,
    }
}

/// A step as a checklist item's request body.
fn check_body(task: &Task) -> Vec<u8> {
    json!({ "displayName": task.title, "isChecked": task.done_at.is_some() })
        .to_string()
        .into_bytes()
}

/// Unix seconds of Graph's `dateTime` (no zone of its own) in UTC.
fn utc_unix(time: &DateTimeZone) -> Option<i64> {
    let text = time.date_time.split('.').next()?;
    unix(&format!("{text}Z"))
}

/// The due day of a `dueDateTime`. To Do's own apps write the local
/// midnight of the day, in UTC; midnight UTC means the day itself.
fn due_day(due: &DateTimeZone) -> String {
    let day = date_part(&due.date_time);
    let midnight = due.date_time.get(11..19) == Some("00:00:00");
    if midnight || !due.time_zone.eq_ignore_ascii_case("UTC") {
        return day;
    }
    utc_unix(due)
        .and_then(|at| jiff::Timestamp::from_second(at).ok())
        .map(|at| at.to_zoned(jiff::tz::TimeZone::system()).date().to_string())
        .unwrap_or(day)
}

/// `dateTimeTimeZone` of Unix seconds, in UTC.
fn graph_time(at: i64) -> Value {
    let text = jiff::Timestamp::from_second(at)
        .unwrap_or(jiff::Timestamp::UNIX_EPOCH)
        .strftime("%Y-%m-%dT%H:%M:%S")
        .to_string();
    json!({ "dateTime": text, "timeZone": "UTC" })
}

/// The local midnight of `day` (`YYYY-MM-DD`) as Graph's due time, the
/// way To Do's own apps write it.
fn graph_due(day: &str) -> Value {
    let midnight = day
        .parse::<jiff::civil::Date>()
        .ok()
        .and_then(|date| date.to_zoned(jiff::tz::TimeZone::system()).ok())
        .map(|zoned| zoned.timestamp().as_second());
    match midnight {
        Some(at) => graph_time(at),
        None => Value::Null,
    }
}

const DAYS: [(&str, &str); 7] = [
    ("sunday", "SU"),
    ("monday", "MO"),
    ("tuesday", "TU"),
    ("wednesday", "WE"),
    ("thursday", "TH"),
    ("friday", "FR"),
    ("saturday", "SA"),
];

const INDEXES: [(&str, &str); 5] = [
    ("first", "1"),
    ("second", "2"),
    ("third", "3"),
    ("fourth", "4"),
    ("last", "-1"),
];

fn lookup<'a>(table: &[(&'a str, &'a str)], key: &str, forward: bool) -> Option<&'a str> {
    table
        .iter()
        .find(|(a, b)| if forward { *a == key } else { *b == key })
        .map(|(a, b)| if forward { *b } else { *a })
}

/// Graph's `patternedRecurrence` as an RFC 5545 `RRULE` value; empty for
/// none.
pub fn rrule(recurrence: &Value) -> String {
    let pattern = &recurrence["pattern"];
    let interval = pattern["interval"].as_u64().unwrap_or(1).max(1);
    let days: Vec<&str> = pattern["daysOfWeek"]
        .as_array()
        .map(|days| {
            days.iter()
                .filter_map(|d| d.as_str().and_then(|d| lookup(&DAYS, d, true)))
                .collect()
        })
        .unwrap_or_default();
    let index = pattern["index"]
        .as_str()
        .and_then(|i| lookup(&INDEXES, i, true))
        .unwrap_or("1");
    let mut rule = match pattern["type"].as_str().unwrap_or_default() {
        "daily" => "FREQ=DAILY".to_owned(),
        "weekly" if days.is_empty() => "FREQ=WEEKLY".to_owned(),
        "weekly" => format!("FREQ=WEEKLY;BYDAY={}", days.join(",")),
        "absoluteMonthly" => "FREQ=MONTHLY".to_owned(),
        "relativeMonthly" => format!(
            "FREQ=MONTHLY;BYDAY={}",
            days.iter()
                .map(|d| format!("{index}{d}"))
                .collect::<Vec<_>>()
                .join(",")
        ),
        "absoluteYearly" => "FREQ=YEARLY".to_owned(),
        "relativeYearly" => format!(
            "FREQ=YEARLY;BYMONTH={};BYDAY={}",
            pattern["month"].as_u64().unwrap_or(1),
            days.iter()
                .map(|d| format!("{index}{d}"))
                .collect::<Vec<_>>()
                .join(",")
        ),
        _ => return String::new(),
    };
    if interval > 1 {
        rule.push_str(&format!(";INTERVAL={interval}"));
    }
    rule
}

/// An `RRULE` value as Graph's `patternedRecurrence`, starting on `due`;
/// `None` for a rule Graph can't hold.
pub fn recurrence(rule: &str, due: &str) -> Option<Value> {
    let date = due.parse::<jiff::civil::Date>().ok()?;
    let mut freq = "";
    let mut interval = 1;
    let mut by_day: Vec<&str> = Vec::new();
    let mut by_month = None;
    for part in rule.split(';') {
        let (key, value) = part.split_once('=')?;
        match key {
            "FREQ" => freq = value,
            "INTERVAL" => interval = value.parse::<u32>().ok()?,
            "BYDAY" => by_day = value.split(',').collect(),
            "BYMONTH" => by_month = Some(value.parse::<u32>().ok()?),
            _ => return None,
        }
    }
    // `2TU` is the second Tuesday: an index and a day.
    let split = |day: &str| -> Option<(Option<&'static str>, &'static str)> {
        let at = day.len().checked_sub(2)?;
        let (index, name) = day.split_at(at);
        let name = lookup(&DAYS, name, false)?;
        let index = if index.is_empty() {
            None
        } else {
            Some(lookup(&INDEXES, index, false)?)
        };
        Some((index, name))
    };
    let days: Vec<(Option<&str>, &str)> =
        by_day.iter().map(|day| split(day)).collect::<Option<_>>()?;
    let names: Vec<&str> = days.iter().map(|(_, name)| *name).collect();
    let index = days.first().and_then(|(index, _)| *index);
    let mut pattern = Map::new();
    pattern.insert("interval".into(), json!(interval));
    match (freq, index) {
        ("DAILY", None) if days.is_empty() => {
            pattern.insert("type".into(), json!("daily"));
        }
        ("WEEKLY", None) => {
            let names = if names.is_empty() {
                let weekday = date.weekday().to_sunday_zero_offset();
                vec![DAYS[usize::from(weekday.unsigned_abs())].0]
            } else {
                names
            };
            pattern.insert("type".into(), json!("weekly"));
            pattern.insert("daysOfWeek".into(), json!(names));
            pattern.insert("firstDayOfWeek".into(), json!("sunday"));
        }
        ("MONTHLY", None) if days.is_empty() => {
            pattern.insert("type".into(), json!("absoluteMonthly"));
            pattern.insert("dayOfMonth".into(), json!(date.day()));
        }
        ("MONTHLY", Some(index)) => {
            pattern.insert("type".into(), json!("relativeMonthly"));
            pattern.insert("daysOfWeek".into(), json!(names));
            pattern.insert("index".into(), json!(index));
        }
        ("YEARLY", None) if days.is_empty() => {
            pattern.insert("type".into(), json!("absoluteYearly"));
            pattern.insert("dayOfMonth".into(), json!(date.day()));
            pattern.insert("month".into(), json!(date.month()));
        }
        ("YEARLY", Some(index)) => {
            pattern.insert("type".into(), json!("relativeYearly"));
            pattern.insert("daysOfWeek".into(), json!(names));
            pattern.insert("index".into(), json!(index));
            pattern.insert(
                "month".into(),
                json!(by_month.unwrap_or(u32::from(date.month().unsigned_abs()))),
            );
        }
        _ => return None,
    }
    Some(json!({
        "pattern": pattern,
        "range": { "type": "noEnd", "startDate": due },
    }))
}

impl From<Item> for RemoteTask {
    fn from(item: Item) -> Self {
        let done_at = (item.status == "completed")
            .then(|| item.completed.as_ref().and_then(utc_unix).unwrap_or(0));
        let remind_at = item
            .reminder
            .as_ref()
            .filter(|_| item.reminder_on)
            .and_then(utc_unix);
        Self {
            deleted: item.removed.is_some(),
            title: item.title,
            notes: item.body.map(|b| b.content).unwrap_or_default(),
            due: item.due.as_ref().map(due_day).unwrap_or_default(),
            done_at,
            position: String::new(),
            etag: item.etag,
            extras: Some(TaskExtras {
                remind_at,
                repeat: item.recurrence.as_ref().map(rrule).unwrap_or_default(),
                starred: item.importance == "high",
            }),
            parent: None,
            remote_id: item.id,
        }
    }
}

/// A task as Graph's request body.
fn body(task: &Task) -> Vec<u8> {
    let due = if task.due.is_empty() {
        Value::Null
    } else {
        graph_due(&task.due)
    };
    let mut body = json!({
        "title": task.title,
        "body": { "content": task.notes, "contentType": "text" },
        "importance": if task.starred { "high" } else { "normal" },
        "status": if task.done_at.is_some() { "completed" } else { "notStarted" },
        "dueDateTime": due,
        "isReminderOn": task.remind_at.is_some(),
        "reminderDateTime": task.remind_at.map_or(Value::Null, graph_time),
        // A repeat needs a due day in To Do.
        "recurrence": recurrence(&task.repeat, &task.due).unwrap_or(Value::Null),
    });
    if let Some(at) = task.done_at {
        body["completedDateTime"] = graph_time(at);
    }
    body.to_string().into_bytes()
}

impl ToDo {
    /// Microsoft's, or the server under test in `KATNA_GRAPH_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GRAPH_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(GRAPH_API))
    }

    /// Talks to `api` instead of Microsoft, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna into To Do.
    pub async fn allowed(&self) -> Result<bool> {
        match self.tokens.access_token_for(MICROSOFT_TASKS).await {
            Ok(_) => Ok(true),
            Err(Error::Auth(_)) => Ok(false),
            Err(err) => Err(err),
        }
    }

    /// Sends a request to `url` (under [`Self::api`]) with the account's
    /// access token, trying once more with a fresh token when Graph
    /// refuses the one it had.
    async fn call(&self, method: &str, url: &str, body: Option<&[u8]>) -> Result<Reply> {
        if !url.starts_with(&format!("{}/", self.api)) {
            return Err(Error::Protocol(format!("Graph sent Katna to {url}")));
        }
        loop {
            let token = format!(
                "Bearer {}",
                self.tokens.access_token_for(MICROSOFT_TASKS).await?
            );
            let headers = [("Authorization", token.as_str()), ("Prefer", PREFER_UTC)];
            let body = body.map(|b| ("application/json", b));
            let reply =
                http::exchange(method, url, &headers, body, None, &self.tls, TIMEOUT).await?;
            if reply.status == 401 && self.tokens.forget_access_token_for(MICROSOFT_TASKS) {
                continue;
            }
            return Ok(reply);
        }
    }

    fn lists_url(&self) -> String {
        format!("{}/me/todo/lists", self.api)
    }

    fn tasks_url(&self, list: &str) -> String {
        format!("{}/{}/tasks", self.lists_url(), segment(list))
    }

    pub async fn lists(&self) -> Result<Vec<RemoteTaskList>> {
        let mut all = Vec::new();
        let mut url = self.lists_url();
        loop {
            let reply = self.call("GET", &url, None).await?;
            let page: Page<List> = parse(&reply, "reading the task lists")?;
            all.extend(page.value);
            match page.next {
                Some(next) => url = next,
                None => break,
            }
        }
        Ok(all
            .into_iter()
            .map(|list| RemoteTaskList {
                is_default: list.wellknown == "defaultList",
                remote_id: list.id,
                title: list.name,
            })
            .collect())
    }

    pub async fn add_list(&self, title: &str) -> Result<String> {
        let body = json!({ "displayName": title }).to_string();
        let reply = self
            .call("POST", &self.lists_url(), Some(body.as_bytes()))
            .await?;
        let list: List = parse(&reply, "adding a task list")?;
        Ok(list.id)
    }

    pub async fn rename_list(&self, id: &str, title: &str) -> Result<()> {
        let body = json!({ "displayName": title }).to_string();
        let url = format!("{}/{}", self.lists_url(), segment(id));
        let reply = self.call("PATCH", &url, Some(body.as_bytes())).await?;
        check(&reply, "renaming a task list")
    }

    pub async fn delete_list(&self, id: &str) -> Result<()> {
        let url = format!("{}/{}", self.lists_url(), segment(id));
        let reply = self.call("DELETE", &url, None).await?;
        if gone(&reply) {
            return Ok(());
        }
        check(&reply, "deleting a task list")
    }

    /// The changes of `list` since `state` (the last pull's delta link),
    /// or all of its tasks.
    pub async fn pull(&self, list: &str, state: Option<&str>) -> Result<Pull> {
        let first = format!("{}/delta", self.tasks_url(list));
        let mut all = state.is_none();
        let mut url = state.map_or_else(|| first.clone(), str::to_owned);
        let mut tasks = Vec::new();
        loop {
            let reply = self.call("GET", &url, None).await?;
            if reply.status == 410 && !all {
                // Graph forgot where the last pull ended: start over.
                all = true;
                url = first.clone();
                tasks.clear();
                continue;
            }
            if gone(&reply) {
                // The list went meanwhile; the next round drops it.
                return Ok(Pull {
                    tasks: Vec::new(),
                    all: false,
                    state: state.map(str::to_owned),
                    steps_of: Vec::new(),
                });
            }
            let page: Page<Item> = parse(&reply, "reading tasks")?;
            tasks.extend(page.value.into_iter().map(RemoteTask::from));
            match (page.next, page.delta) {
                (Some(next), _) => url = next,
                (None, delta) => {
                    // The steps of each task that came, all of them: the
                    // delta holds none.
                    let mut steps_of = Vec::new();
                    let mut steps = Vec::new();
                    for task in &tasks {
                        if !task.deleted {
                            match self.steps(list, &task.remote_id).await? {
                                Some(found) => steps.extend(found),
                                // Gone meanwhile: the next pull says so.
                                None => continue,
                            }
                        }
                        steps_of.push(task.remote_id.clone());
                    }
                    tasks.extend(steps);
                    return Ok(Pull {
                        tasks,
                        all,
                        state: delta,
                        steps_of,
                    });
                }
            }
        }
    }

    fn checks_url(&self, list: &str, task: &str) -> String {
        format!("{}/{}/checklistItems", self.tasks_url(list), segment(task))
    }

    /// Task `task`'s steps; `None` when To Do no longer has the task.
    async fn steps(&self, list: &str, task: &str) -> Result<Option<Vec<RemoteTask>>> {
        let mut url = self.checks_url(list, task);
        let mut checks = Vec::new();
        loop {
            let reply = self.call("GET", &url, None).await?;
            if gone(&reply) {
                return Ok(None);
            }
            let page: Page<Check> = parse(&reply, "reading steps")?;
            checks.extend(page.value);
            match page.next {
                Some(next) => url = next,
                None => break,
            }
        }
        Ok(Some(
            checks
                .into_iter()
                .enumerate()
                .map(|(index, check)| step(task, index, check))
                .collect(),
        ))
    }

    /// Adds `task` to `list`, as a step of `parent` when it has one.
    pub async fn insert(
        &self,
        list: &str,
        task: &Task,
        parent: Option<&str>,
    ) -> Result<RemoteTask> {
        if let Some(parent) = parent {
            let url = self.checks_url(list, parent);
            let reply = self.call("POST", &url, Some(&check_body(task))).await?;
            let check: Check = parse(&reply, "adding a step")?;
            return Ok(step(parent, usize::MAX, check));
        }
        let reply = self
            .call("POST", &self.tasks_url(list), Some(&body(task)))
            .await?;
        let item: Item = parse(&reply, "adding a task")?;
        Ok(item.into())
    }

    /// Changes task or step `id`; `None` when To Do no longer has it.
    pub async fn update(&self, list: &str, id: &str, task: &Task) -> Result<Option<RemoteTask>> {
        if let Some((parent, item)) = split_step(id) {
            let url = format!("{}/{}", self.checks_url(list, parent), segment(item));
            let reply = self.call("PATCH", &url, Some(&check_body(task))).await?;
            if gone(&reply) {
                return Ok(None);
            }
            let check: Check = parse(&reply, "changing a step")?;
            return Ok(Some(step(parent, usize::MAX, check)));
        }
        let url = format!("{}/{}", self.tasks_url(list), segment(id));
        let reply = self.call("PATCH", &url, Some(&body(task))).await?;
        if gone(&reply) {
            return Ok(None);
        }
        let item: Item = parse(&reply, "changing a task")?;
        Ok(Some(item.into()))
    }

    pub async fn delete(&self, list: &str, id: &str) -> Result<()> {
        let url = match split_step(id) {
            Some((parent, item)) => {
                format!("{}/{}", self.checks_url(list, parent), segment(item))
            }
            None => format!("{}/{}", self.tasks_url(list), segment(id)),
        };
        let reply = self.call("DELETE", &url, None).await?;
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
    code: String,
    #[serde(default)]
    message: String,
}

/// What a failed answer means: a refused grant asks to sign in again
/// ([`Error::Auth`]); anything else is Graph's own message.
fn failure(reply: &Reply, doing: &str) -> Error {
    let failure: Failure = serde_json::from_slice(&reply.body).unwrap_or_default();
    let denied = matches!(
        failure.error.code.as_str(),
        "accessDenied" | "unauthenticated" | "ErrorAccessDenied"
    );
    if reply.status == 401 || (reply.status == 403 && denied) {
        return Error::Auth(format!("Microsoft To Do refused access while {doing}"));
    }
    let detail = if failure.error.message.is_empty() {
        format!("status {}", reply.status)
    } else {
        failure.error.message
    };
    if reply.status == 429 || reply.status >= 500 {
        return Error::Closed(format!("Microsoft To Do, {doing}: {detail}"));
    }
    Error::Rejected(format!("Microsoft To Do, {doing}: {detail}"))
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
        .map_err(|err| Error::Protocol(format!("Microsoft To Do, {doing}: {err}")))
}
