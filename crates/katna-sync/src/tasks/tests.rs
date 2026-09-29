// SPDX-License-Identifier: GPL-3.0-or-later

//! Task sync against fake Google Tasks and To Do servers on the loopback.

use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use katna_core::{AccountKind, OAuthProvider, Paths};
use katna_store::{
    Mode, Store,
    tasks::{TaskFields, TaskList},
};
use serde_json::{Value, json};

use super::*;
use crate::{
    net::Tls,
    oauth::{GOOGLE_TASKS, MICROSOFT_TASKS, Provider, TokenSource},
};

/// A request as the fake saw it.
#[derive(Debug, Clone)]
struct Seen {
    method: String,
    path: String,
    body: Value,
}

fn read_request(stream: &mut TcpStream) -> Option<Seen> {
    let mut data = Vec::new();
    let mut buf = [0; 16 * 1024];
    loop {
        let n = stream.read(&mut buf).ok()?;
        if n == 0 {
            return None;
        }
        data.extend_from_slice(&buf[..n]);
        let Some(end) = data.windows(4).position(|w| w == b"\r\n\r\n") else {
            continue;
        };
        let head = String::from_utf8_lossy(&data[..end]).into_owned();
        let mut lines = head.split("\r\n");
        let mut first = lines.next()?.split(' ');
        let method = first.next()?.to_owned();
        let path = first.next()?.to_owned();
        let length = lines
            .filter_map(|l| l.split_once(':'))
            .find(|(n, _)| n.trim().eq_ignore_ascii_case("content-length"))
            .map_or(0, |(_, v)| v.trim().parse::<usize>().unwrap());
        while data.len() < end + 4 + length {
            let n = stream.read(&mut buf).ok()?;
            if n == 0 {
                return None;
            }
            data.extend_from_slice(&buf[..n]);
        }
        let body = serde_json::from_slice(&data[end + 4..end + 4 + length]).unwrap_or(Value::Null);
        return Some(Seen { method, path, body });
    }
}

fn answer(stream: &mut TcpStream, status: u16, body: &Value) {
    let body = if body.is_null() {
        String::new()
    } else {
        body.to_string()
    };
    let text = format!(
        "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\
         Connection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(text.as_bytes());
}

/// A fake server's state and the requests it saw.
type Fake<S> = Arc<Mutex<(S, Vec<Seen>)>>;

/// A fake server: `handle` answers each request from the shared state.
fn serve<S: Send + 'static>(
    state: S,
    handle: fn(&mut S, &Seen, &str) -> (u16, Value),
) -> (String, Fake<S>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let shared = Arc::new(Mutex::new((state, Vec::new())));
    let inner = shared.clone();
    let url = base.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let Some(request) = read_request(&mut stream) else {
                continue;
            };
            let mut guard = inner.lock().unwrap();
            let (state, seen) = &mut *guard;
            seen.push(request.clone());
            let (status, body) = handle(state, &request, &url);
            drop(guard);
            answer(&mut stream, status, &body);
        }
    });
    (base, shared)
}

fn provider(kind: OAuthProvider) -> Provider {
    Provider {
        kind,
        auth_url: "https://accounts.test/auth".into(),
        token_url: "http://127.0.0.1:1/token".into(),
        client_id: "katna-test".into(),
        client_secret: String::new(),
        scope: "https://mail.test/".into(),
        consent: String::new(),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    }
}

fn store() -> (tempfile::TempDir, Mutex<Store>, AccountId) {
    let dir = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Me", "me@example.test")
        .unwrap()
        .id;
    (dir, Mutex::new(store), account)
}

// --- Google Tasks ------------------------------------------------------------

#[derive(Default)]
struct Google {
    /// List ID to title.
    lists: BTreeMap<String, String>,
    /// Task ID to (list, task JSON).
    tasks: BTreeMap<String, (String, Value)>,
    next: u32,
}

impl Google {
    fn id(&mut self, prefix: &str) -> String {
        self.next += 1;
        format!("{prefix}{}", self.next)
    }

    fn put(&mut self, list: &str, mut task: Value) -> Value {
        let id = task["id"].as_str().unwrap().to_owned();
        self.next += 1;
        task["etag"] = json!(format!("etag-{}", self.next));
        task["updated"] = json!("2026-09-29T03:00:00.000Z");
        self.tasks.insert(id, (list.to_owned(), task.clone()));
        task
    }
}

fn google(state: &mut Google, request: &Seen, _base: &str) -> (u16, Value) {
    let path = request.path.strip_prefix("/tasks/v1/").unwrap();
    let (path, query) = path.split_once('?').unwrap_or((path, ""));
    let parts: Vec<&str> = path.split('/').collect();
    match (request.method.as_str(), parts.as_slice()) {
        ("GET", ["users", "@me", "lists"]) => {
            let items: Vec<Value> = state
                .lists
                .iter()
                .map(|(id, title)| json!({ "id": id, "title": title }))
                .collect();
            (200, json!({ "items": items }))
        }
        ("GET", ["users", "@me", "lists", "@default"]) => {
            let (id, title) = state.lists.iter().next().unwrap();
            (200, json!({ "id": id, "title": title }))
        }
        ("POST", ["users", "@me", "lists"]) => {
            let id = state.id("L");
            let title = request.body["title"].as_str().unwrap().to_owned();
            state.lists.insert(id.clone(), title.clone());
            (200, json!({ "id": id, "title": title }))
        }
        ("GET", ["lists", list, "tasks"]) => {
            assert!(query.contains("showHidden=true"), "{query}");
            let items: Vec<Value> = state
                .tasks
                .values()
                .filter(|(l, _)| l == list)
                .map(|(_, t)| t.clone())
                .collect();
            (200, json!({ "items": items }))
        }
        ("POST", ["lists", list, "tasks"]) => {
            let mut task = request.body.clone();
            task["id"] = json!(state.id("T"));
            task["position"] = json!(format!("{:020}", 1000 - state.next));
            if let Some(parent) = query.strip_prefix("parent=") {
                task["parent"] = json!(parent);
            }
            (200, state.put(list, task))
        }
        ("PATCH", ["lists", list, "tasks", id]) => {
            let Some((_, old)) = state.tasks.get(*id).cloned() else {
                return (
                    404,
                    json!({ "error": { "code": 404, "message": "Not Found" } }),
                );
            };
            let mut task = old;
            for (key, value) in request.body.as_object().unwrap() {
                task[key] = value.clone();
            }
            (200, state.put(list, task))
        }
        ("DELETE", ["lists", _, "tasks", id]) => {
            state.tasks.remove(*id);
            (204, Value::Null)
        }
        _ => (
            404,
            json!({ "error": { "code": 404, "message": "no such thing" } }),
        ),
    }
}

fn google_service(api: &str) -> TaskService {
    let tokens = TokenSource::new(provider(OAuthProvider::Google), "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600))
        .with_scope(Some(format!("https://mail.test/ {GOOGLE_TASKS}")));
    TaskService::Google(google::GoogleTasks::with_api(
        Arc::new(tokens),
        Tls::insecure_for_local_tests(),
        api,
    ))
}

fn account_lists(store: &Mutex<Store>) -> Vec<TaskList> {
    store
        .lock()
        .unwrap()
        .task_lists()
        .unwrap()
        .into_iter()
        .filter(|l| l.account.is_some())
        .collect()
}

#[test]
fn google_tasks_sync_both_ways() {
    let mut fake = Google::default();
    fake.lists.insert("L0".into(), "My Tasks".into());
    fake.put(
        "L0",
        json!({ "id": "P", "title": "Plan the trip", "status": "needsAction",
                "due": "2026-10-05T00:00:00.000Z", "position": "00000000000000000001" }),
    );
    fake.put(
        "L0",
        json!({ "id": "S", "title": "Book the train", "status": "completed",
                "completed": "2026-09-28T10:00:00.000Z", "parent": "P", "hidden": true,
                "position": "00000000000000000002" }),
    );
    let (api, fake) = serve(fake, google);
    let service = google_service(&api);
    let (_dir, store, account) = store();
    // Added in the desktop clock before any sync.
    store
        .lock()
        .unwrap()
        .add_task("From the clock", "")
        .unwrap();

    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let lists = account_lists(&store);
    assert_eq!(lists.len(), 1);
    assert!(lists[0].is_default);
    let tasks = store.lock().unwrap().tasks_in(lists[0].id).unwrap();
    let titles: Vec<_> = tasks.iter().map(|t| t.title.as_str()).collect();
    // The clock's task moved to Google and came back with its place.
    assert!(titles.contains(&"From the clock"), "{titles:?}");
    let plan = tasks.iter().find(|t| t.title == "Plan the trip").unwrap();
    assert_eq!(plan.due, "2026-10-05");
    let step = tasks.iter().find(|t| t.title == "Book the train").unwrap();
    assert_eq!(step.parent, Some(plan.id));
    assert!(step.done_at.is_some());
    {
        let fake = fake.lock().unwrap();
        let clock = fake
            .0
            .tasks
            .values()
            .find(|(_, t)| t["title"] == "From the clock")
            .expect("sent to Google");
        assert_eq!(clock.0, "L0");
    }

    // Changed here: a due day and a time; Google keeps the day only.
    let fields = TaskFields {
        title: "Plan the trip to Goa".into(),
        due: "2026-10-06".into(),
        due_time: Some(18 * 60),
        starred: true,
        ..TaskFields::default()
    };
    store.lock().unwrap().edit_task(plan.id, &fields).unwrap();
    store.lock().unwrap().set_task_done(step.id, false).unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let (_, plan) = &fake.0.tasks["P"];
        assert_eq!(plan["title"], "Plan the trip to Goa");
        assert_eq!(plan["due"], "2026-10-06T00:00:00.000Z");
        let (_, step) = &fake.0.tasks["S"];
        assert_eq!(step["status"], "needsAction");
        assert_eq!(step["completed"], Value::Null);
    }
    let plan = store.lock().unwrap().task(plan.id).unwrap().unwrap();
    assert_eq!(plan.due_time, Some(18 * 60), "the time stays in Katna");
    assert!(plan.starred);
    assert!(
        store
            .lock()
            .unwrap()
            .pending_tasks(lists[0].id)
            .unwrap()
            .is_empty()
    );

    // A step added here goes under its task on Google.
    let step = store
        .lock()
        .unwrap()
        .add_task_to(
            lists[0].id,
            Some(plan.id),
            &TaskFields {
                title: "Pack".into(),
                ..TaskFields::default()
            },
        )
        .unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let (_, pack) = fake
            .0
            .tasks
            .values()
            .find(|(_, t)| t["title"] == "Pack")
            .unwrap();
        assert_eq!(pack["parent"], "P");
    }

    // Deleted here: deleted on Google, and stays deleted.
    store.lock().unwrap().delete_task(step).unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert!(
        fake.lock()
            .unwrap()
            .0
            .tasks
            .values()
            .all(|(_, t)| t["title"] != "Pack")
    );
    let titles: Vec<_> = store
        .lock()
        .unwrap()
        .tasks_in(lists[0].id)
        .unwrap()
        .into_iter()
        .map(|t| t.title)
        .collect();
    assert!(!titles.contains(&"Pack".to_owned()));

    // Deleted on the phone: gone here at the next full pull.
    fake.lock().unwrap().0.tasks.remove("S");
    store
        .lock()
        .unwrap()
        .set_task_list_sync_state(lists[0].id, None)
        .unwrap();
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let titles: Vec<_> = store
        .lock()
        .unwrap()
        .tasks_in(lists[0].id)
        .unwrap()
        .into_iter()
        .map(|t| t.title)
        .collect();
    assert_eq!(titles.len(), 2, "{titles:?}");
    assert!(!titles.contains(&"Book the train".to_owned()));
}

#[test]
fn google_asks_only_for_changes_after_the_first_pull() {
    let mut fake = Google::default();
    fake.lists.insert("L0".into(), "My Tasks".into());
    let (api, fake) = serve(fake, google);
    let service = google_service(&api);
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let fake = fake.lock().unwrap();
    let pulls: Vec<&Seen> = fake
        .1
        .iter()
        .filter(|s| s.method == "GET" && s.path.contains("/tasks?"))
        .collect();
    assert_eq!(pulls.len(), 2);
    assert!(!pulls[0].path.contains("updatedMin"));
    assert!(pulls[1].path.contains("updatedMin"));
    assert!(pulls[1].path.contains("showDeleted=true"));
}

#[test]
fn a_list_made_here_is_made_on_google() {
    let mut fake = Google::default();
    fake.lists.insert("L0".into(), "My Tasks".into());
    let (api, fake) = serve(fake, google);
    let service = google_service(&api);
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let home = store
        .lock()
        .unwrap()
        .add_task_list(Some(account), "Home")
        .unwrap();
    store
        .lock()
        .unwrap()
        .add_task_to(
            home,
            None,
            &TaskFields {
                title: "Fix the tap".into(),
                ..TaskFields::default()
            },
        )
        .unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let fake = fake.lock().unwrap();
    let (id, _) = fake.0.lists.iter().find(|(_, t)| *t == "Home").unwrap();
    assert!(
        fake.0
            .tasks
            .values()
            .any(|(l, t)| l == id && t["title"] == "Fix the tap")
    );
    assert_eq!(account_lists(&store).len(), 2);
}

#[test]
fn a_missing_tasks_scope_asks_to_sign_in_again() {
    let tokens = TokenSource::new(provider(OAuthProvider::Google), "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600))
        .with_scope(Some("https://mail.test/ openid".into()));
    let service = TaskService::Google(google::GoogleTasks::with_api(
        Arc::new(tokens),
        Tls::insecure_for_local_tests(),
        "http://127.0.0.1:1",
    ));
    assert!(!smol::block_on(service.allowed()).unwrap());

    let (api, _) = serve((), |_, _, _| {
        (
            403,
            json!({ "error": { "code": 403, "status": "PERMISSION_DENIED",
                               "message": "Request had insufficient authentication scopes." } }),
        )
    });
    let (_dir, store, account) = store();
    let err = smol::block_on(sync_account(&google_service(&api), &store, account)).unwrap_err();
    assert!(matches!(err, Error::Auth(_)), "{err}");
}

// --- Microsoft To Do --------------------------------------------------------------

#[derive(Default)]
struct Graph {
    /// List ID to (name, well-known name).
    lists: BTreeMap<String, (String, String)>,
    /// Task ID to (list, task JSON).
    tasks: BTreeMap<String, (String, Value)>,
    /// Task IDs removed since the last delta.
    removed: Vec<(String, String)>,
    /// Whether a delta link was handed out (changes since then only).
    delta_given: bool,
    changed: Vec<String>,
    next: u32,
}

fn graph(state: &mut Graph, request: &Seen, base: &str) -> (u16, Value) {
    let path = request.path.strip_prefix("/me/todo/").unwrap();
    let (path, query) = path.split_once('?').unwrap_or((path, ""));
    let parts: Vec<&str> = path.split('/').collect();
    match (request.method.as_str(), parts.as_slice()) {
        ("GET", ["lists"]) => {
            let value: Vec<Value> = state
                .lists
                .iter()
                .map(|(id, (name, known))| {
                    json!({ "id": id, "displayName": name, "wellknownListName": known })
                })
                .collect();
            (200, json!({ "value": value }))
        }
        ("GET", ["lists", list, "tasks", "delta"]) => {
            let since = query.contains("deltatoken");
            let mut value: Vec<Value> = state
                .tasks
                .iter()
                .filter(|(id, (l, _))| l == list && (!since || state.changed.contains(id)))
                .map(|(_, (_, t))| t.clone())
                .collect();
            if since {
                value.extend(
                    state
                        .removed
                        .iter()
                        .filter(|(l, _)| l == list)
                        .map(|(_, id)| json!({ "id": id, "@removed": { "reason": "deleted" } })),
                );
            }
            state.changed.clear();
            state.removed.clear();
            state.delta_given = true;
            (
                200,
                json!({ "value": value,
                        "@odata.deltaLink": format!("{base}/me/todo/lists/{list}/tasks/delta?$deltatoken=d1") }),
            )
        }
        ("POST", ["lists", list, "tasks"]) => {
            state.next += 1;
            let id = format!("M{}", state.next);
            let mut task = request.body.clone();
            task["id"] = json!(id);
            task["@odata.etag"] = json!(format!("W/\"{}\"", state.next));
            state.tasks.insert(id, ((*list).to_owned(), task.clone()));
            (201, task)
        }
        ("PATCH", ["lists", _, "tasks", id]) => {
            let Some((list, mut task)) = state.tasks.get(*id).cloned() else {
                return (404, json!({ "error": { "code": "ErrorItemNotFound" } }));
            };
            for (key, value) in request.body.as_object().unwrap() {
                task[key] = value.clone();
            }
            state.next += 1;
            task["@odata.etag"] = json!(format!("W/\"{}\"", state.next));
            state.tasks.insert((*id).to_owned(), (list, task.clone()));
            (200, task)
        }
        _ => (404, json!({ "error": { "code": "NotFound" } })),
    }
}

fn graph_service(api: &str) -> TaskService {
    let tokens = TokenSource::new(provider(OAuthProvider::Microsoft), "rt".into(), None);
    // Graph's own token, as if the sign-in allowed To Do.
    let tokens = Arc::new(tokens);
    tokens.set_access_token_for_tests(MICROSOFT_TASKS, "gt-1");
    TaskService::Microsoft(graph::ToDo::with_api(
        tokens,
        Tls::insecure_for_local_tests(),
        api,
    ))
}

#[test]
fn to_do_syncs_with_reminders_repeat_and_star() {
    let mut fake = Graph::default();
    fake.lists
        .insert("D".into(), ("Tasks".into(), "defaultList".into()));
    fake.lists
        .insert("F".into(), ("Flagged email".into(), "flaggedEmails".into()));
    fake.tasks.insert(
        "X".into(),
        (
            "D".into(),
            json!({
                "id": "X", "@odata.etag": "W/\"1\"", "title": "Water plants",
                "status": "notStarted", "importance": "high",
                "body": { "content": "the balcony ones", "contentType": "text" },
                "dueDateTime": { "dateTime": "2026-10-01T00:00:00.0000000", "timeZone": "UTC" },
                "isReminderOn": true,
                "reminderDateTime": { "dateTime": "2026-10-01T03:30:00.0000000", "timeZone": "UTC" },
                "recurrence": { "pattern": { "type": "weekly", "interval": 1,
                                             "daysOfWeek": ["thursday"] },
                                "range": { "type": "noEnd", "startDate": "2026-10-01" } }
            }),
        ),
    );
    let (api, fake) = serve(fake, graph);
    let service = graph_service(&api);
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();

    let lists = account_lists(&store);
    assert_eq!(lists.len(), 2);
    let default = lists.iter().find(|l| l.is_default).unwrap();
    assert_eq!(default.title, "Tasks");
    let task = store
        .lock()
        .unwrap()
        .tasks_in(default.id)
        .unwrap()
        .remove(0);
    assert_eq!(task.title, "Water plants");
    assert_eq!(task.notes, "the balcony ones");
    assert_eq!(task.due, "2026-10-01");
    assert!(task.starred);
    assert_eq!(task.repeat, "FREQ=WEEKLY;BYDAY=TH");
    assert_eq!(task.remind_at, Some(1_790_825_400));

    // Ticked off here; To Do hears it.
    store.lock().unwrap().set_task_done(task.id, true).unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let (_, x) = &fake.0.tasks["X"];
        assert_eq!(x["status"], "completed");
        assert_eq!(x["importance"], "high");
        assert_eq!(
            x["recurrence"]["pattern"]["daysOfWeek"],
            json!(["thursday"])
        );
        assert!(fake.1.iter().any(|s| s.path.contains("deltatoken")));
    }

    // Deleted in To Do: the delta says so.
    {
        let mut fake = fake.lock().unwrap();
        fake.0.tasks.remove("X");
        fake.0.removed.push(("D".into(), "X".into()));
    }
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    assert!(
        store
            .lock()
            .unwrap()
            .tasks_in(default.id)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn rrules_and_graph_recurrence_round_trip() {
    for rule in [
        "FREQ=DAILY",
        "FREQ=DAILY;INTERVAL=2",
        "FREQ=WEEKLY;BYDAY=MO,WE",
        "FREQ=MONTHLY",
        "FREQ=MONTHLY;BYDAY=2TU",
        "FREQ=MONTHLY;BYDAY=-1FR",
        "FREQ=YEARLY",
        "FREQ=YEARLY;BYMONTH=11;BYDAY=4TH",
    ] {
        let graph = graph::recurrence(rule, "2026-10-01").unwrap();
        assert_eq!(graph::rrule(&graph), rule, "{graph}");
    }
    // A weekly rule without days repeats on the due day's weekday.
    let graph = graph::recurrence("FREQ=WEEKLY", "2026-10-01").unwrap();
    assert_eq!(graph["pattern"]["daysOfWeek"], json!(["thursday"]));
    assert_eq!(graph::recurrence("FREQ=HOURLY", "2026-10-01"), None);
    assert_eq!(graph::recurrence("FREQ=DAILY", ""), None);
    assert_eq!(graph::recurrence("", "2026-10-01"), None);
}
