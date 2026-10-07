// SPDX-License-Identifier: GPL-3.0-or-later

//! Task sync against fake Google Tasks, To Do and Zoho servers on the
//! loopback.

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
    /// Its `Authorization` header.
    auth: String,
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
        let headers: Vec<(&str, &str)> = lines
            .filter_map(|l| l.split_once(':'))
            .map(|(n, v)| (n.trim(), v.trim()))
            .collect();
        let header = |name: &str| {
            headers
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(name))
                .map(|(_, v)| *v)
        };
        let length = header("content-length").map_or(0, |v| v.parse::<usize>().unwrap());
        let auth = header("authorization").unwrap_or_default().to_owned();
        while data.len() < end + 4 + length {
            let n = stream.read(&mut buf).ok()?;
            if n == 0 {
                return None;
            }
            data.extend_from_slice(&buf[..n]);
        }
        let body = serde_json::from_slice(&data[end + 4..end + 4 + length]).unwrap_or(Value::Null);
        return Some(Seen {
            method,
            path,
            auth,
            body,
        });
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
    /// Moves that fail as if the connection dropped.
    failing_moves: u32,
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
        ("POST", ["lists", _, "tasks", _, "move"]) if state.failing_moves > 0 => {
            state.failing_moves -= 1;
            (
                503,
                json!({ "error": { "code": 503, "message": "Unavailable" } }),
            )
        }
        ("POST", ["lists", list, "tasks", id, "move"]) => {
            let Some((_, mut task)) = state.tasks.get(*id).cloned() else {
                return (
                    404,
                    json!({ "error": { "code": 404, "message": "Not Found" } }),
                );
            };
            // Right after `previous`, or before every other.
            let position = match query.strip_prefix("previous=") {
                Some(previous) => {
                    let (_, before) = &state.tasks[previous];
                    format!("{}5", before["position"].as_str().unwrap())
                }
                None => "0".to_owned(),
            };
            task["position"] = json!(position);
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

/// The titles of the fake Google's top-level tasks of `list`, in order.
fn google_order(fake: &Fake<Google>, list: &str) -> Vec<String> {
    let fake = fake.lock().unwrap();
    let mut tasks: Vec<&Value> = fake
        .0
        .tasks
        .values()
        .filter(|(l, t)| l == list && t["parent"].is_null())
        .map(|(_, t)| t)
        .collect();
    tasks.sort_by_key(|t| t["position"].as_str().unwrap_or_default().to_owned());
    tasks
        .into_iter()
        .map(|t| t["title"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn a_task_dragged_here_moves_on_google() {
    let mut fake = Google::default();
    fake.lists.insert("L0".into(), "My Tasks".into());
    fake.lists.insert("L9".into(), "Work".into());
    for (id, title, position) in [("A", "a", "1"), ("B", "b", "2"), ("C", "c", "3")] {
        fake.put(
            "L0",
            json!({ "id": id, "title": title, "status": "needsAction", "position": position }),
        );
    }
    fake.put(
        "L9",
        json!({ "id": "W", "title": "w", "status": "needsAction", "position": "1" }),
    );
    let (api, fake) = serve(fake, google);
    let service = google_service(&api);
    let (_dir, store, account) = store();
    store
        .lock()
        .unwrap()
        .set_account_settings(
            account,
            &katna_core::AccountSettings {
                oauth: Some(OAuthProvider::Google),
                ..katna_core::AccountSettings::default()
            },
        )
        .unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let lists = account_lists(&store);
    let list = |title: &str| lists.iter().find(|l| l.title == title).unwrap().id;
    let (mine, work) = (list("My Tasks"), list("Work"));
    let id = |list: i64, title: &str| {
        store
            .lock()
            .unwrap()
            .tasks_in(list)
            .unwrap()
            .into_iter()
            .find(|t| t.title == title)
            .unwrap()
            .id
    };
    let (a, c) = (id(mine, "a"), id(mine, "c"));

    // Within the list: only a move goes, no change of its fields.
    assert!(store.lock().unwrap().place_task(c, mine, Some(a)).unwrap());
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert_eq!(google_order(&fake, "L0"), ["a", "c", "b"]);
    {
        let fake = fake.lock().unwrap();
        let sent: Vec<&Seen> = fake.1.iter().filter(|s| s.method != "GET").collect();
        assert_eq!(sent.len(), 1, "{sent:?}");
        assert_eq!(sent[0].path, "/tasks/v1/lists/L0/tasks/C/move?previous=A");
    }
    let store_order = |list: i64| -> Vec<String> {
        store
            .lock()
            .unwrap()
            .tasks_in(list)
            .unwrap()
            .into_iter()
            .map(|t| t.title)
            .collect()
    };
    assert_eq!(store_order(mine), ["a", "c", "b"]);
    assert!(
        store
            .lock()
            .unwrap()
            .pending_tasks(mine)
            .unwrap()
            .is_empty()
    );

    // First in another list: out of this one, in at the top of that one.
    let w = id(work, "w");
    assert!(store.lock().unwrap().place_task(a, work, None).unwrap());
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert_eq!(google_order(&fake, "L0"), ["c", "b"]);
    assert_eq!(google_order(&fake, "L9"), ["a", "w"]);
    assert_eq!(store_order(work), ["a", "w"]);

    // After a task there.
    let b = id(mine, "b");
    assert!(store.lock().unwrap().place_task(b, work, Some(w)).unwrap());
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert_eq!(google_order(&fake, "L9"), ["a", "w", "b"]);
    assert_eq!(store_order(work), ["a", "w", "b"]);
    assert!(
        store
            .lock()
            .unwrap()
            .pending_tasks(work)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn a_new_task_is_sent_once_when_its_move_fails() {
    let mut fake = Google::default();
    fake.lists.insert("L0".into(), "My Tasks".into());
    fake.put(
        "L0",
        json!({ "id": "A", "title": "a", "status": "needsAction", "position": "1" }),
    );
    fake.failing_moves = 1;
    let (api, fake) = serve(fake, google);
    let service = google_service(&api);
    let (_dir, store, account) = store();
    store
        .lock()
        .unwrap()
        .set_account_settings(
            account,
            &katna_core::AccountSettings {
                oauth: Some(OAuthProvider::Google),
                ..katna_core::AccountSettings::default()
            },
        )
        .unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let mine = account_lists(&store)[0].id;
    let a = store.lock().unwrap().tasks_in(mine).unwrap()[0].id;
    // Added and dragged under `a` before it was ever sent.
    let new = store
        .lock()
        .unwrap()
        .add_task_to(
            mine,
            None,
            &TaskFields {
                title: "walk the dog".into(),
                ..TaskFields::default()
            },
        )
        .unwrap();
    assert!(
        store
            .lock()
            .unwrap()
            .place_task(new, mine, Some(a))
            .unwrap()
    );

    // Made on Google, then the move fails: the round stops there.
    assert!(smol::block_on(sync_account(&service, &store, account)).is_err());
    smol::block_on(sync_account(&service, &store, account)).unwrap();

    let titles = |list: &str| -> Vec<String> {
        let fake = fake.lock().unwrap();
        fake.0
            .tasks
            .values()
            .filter(|(l, _)| l == list)
            .map(|(_, t)| t["title"].as_str().unwrap().to_owned())
            .collect()
    };
    assert_eq!(
        titles("L0").iter().filter(|t| *t == "walk the dog").count(),
        1
    );
    assert_eq!(google_order(&fake, "L0"), ["a", "walk the dog"]);
    let here: Vec<String> = store
        .lock()
        .unwrap()
        .tasks_in(mine)
        .unwrap()
        .into_iter()
        .map(|t| t.title)
        .collect();
    assert_eq!(here, ["a", "walk the dog"]);
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

#[test]
fn a_switched_off_tasks_api_is_not_a_sign_in() {
    let (api, _) = serve((), |_, _, _| {
        (
            403,
            json!({ "error": { "code": 403, "status": "PERMISSION_DENIED",
                               "message": "Google Tasks API has not been used in project 1.",
                               "errors": [{ "reason": "accessNotConfigured" }],
                               "details": [{ "reason": "SERVICE_DISABLED" }] } }),
        )
    });
    let (_dir, store, account) = store();
    let err = smol::block_on(sync_account(&google_service(&api), &store, account)).unwrap_err();
    assert!(matches!(err, Error::NotEnabled(_)), "{err}");
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
    /// Task ID to its checklist items.
    checks: BTreeMap<String, Vec<Value>>,
    /// Task ID to its attachments, with their content.
    files: BTreeMap<String, Vec<Value>>,
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
        ("GET", ["lists", _, "tasks", task, "checklistItems"]) => {
            if !state.tasks.contains_key(*task) {
                return (404, json!({ "error": { "code": "ErrorItemNotFound" } }));
            }
            let value = state.checks.get(*task).cloned().unwrap_or_default();
            (200, json!({ "value": value }))
        }
        ("POST", ["lists", _, "tasks", task, "checklistItems"]) => {
            state.next += 1;
            let mut check = request.body.clone();
            check["id"] = json!(format!("C{}", state.next));
            state
                .checks
                .entry((*task).to_owned())
                .or_default()
                .push(check.clone());
            (201, check)
        }
        ("PATCH", ["lists", _, "tasks", task, "checklistItems", id]) => {
            let checks = state.checks.entry((*task).to_owned()).or_default();
            let Some(check) = checks.iter_mut().find(|c| c["id"] == *id) else {
                return (404, json!({ "error": { "code": "ErrorItemNotFound" } }));
            };
            for (key, value) in request.body.as_object().unwrap() {
                check[key] = value.clone();
            }
            (200, check.clone())
        }
        ("DELETE", ["lists", _, "tasks", task, "checklistItems", id]) => {
            let checks = state.checks.entry((*task).to_owned()).or_default();
            checks.retain(|c| c["id"] != *id);
            (204, Value::Null)
        }
        ("GET", ["lists", _, "tasks", task, "attachments"]) => {
            let value: Vec<Value> = state
                .files
                .get(*task)
                .into_iter()
                .flatten()
                .map(|f| {
                    let mut f = f.clone();
                    f.as_object_mut().unwrap().remove("contentBytes");
                    f
                })
                .collect();
            (200, json!({ "value": value }))
        }
        ("GET", ["lists", _, "tasks", task, "attachments", id]) => {
            let file = state
                .files
                .get(*task)
                .and_then(|files| files.iter().find(|f| f["id"] == *id));
            match file {
                Some(file) => (200, file.clone()),
                None => (404, json!({ "error": { "code": "ErrorItemNotFound" } })),
            }
        }
        ("POST", ["lists", _, "tasks", task, "attachments"]) => {
            state.next += 1;
            let mut file = request.body.clone();
            file["id"] = json!(format!("A{}", state.next));
            let size = file["contentBytes"].as_str().unwrap().len() * 3 / 4;
            file["size"] = json!(size);
            state
                .files
                .entry((*task).to_owned())
                .or_default()
                .push(file.clone());
            if let Some((_, t)) = state.tasks.get_mut(*task) {
                t["hasAttachments"] = json!(true);
            }
            (201, file)
        }
        ("DELETE", ["lists", _, "tasks", task, "attachments", id]) => {
            let files = state.files.entry((*task).to_owned()).or_default();
            files.retain(|f| f["id"] != *id);
            let any = !files.is_empty();
            if let Some((_, t)) = state.tasks.get_mut(*task) {
                t["hasAttachments"] = json!(any);
            }
            (204, Value::Null)
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
fn to_do_steps_are_checklist_items() {
    let mut fake = Graph::default();
    fake.lists
        .insert("D".into(), ("Tasks".into(), "defaultList".into()));
    fake.tasks.insert(
        "X".into(),
        (
            "D".into(),
            json!({ "id": "X", "@odata.etag": "W/\"1\"", "title": "Repot the fern",
                    "status": "notStarted" }),
        ),
    );
    fake.checks.insert(
        "X".into(),
        vec![
            json!({ "id": "A", "displayName": "Buy soil", "isChecked": false }),
            json!({ "id": "B", "displayName": "Find a pot", "isChecked": true,
                    "checkedDateTime": "2026-09-28T10:00:00.0000000Z" }),
        ],
    );
    let (api, fake) = serve(fake, graph);
    let service = graph_service(&api);
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();

    let list = account_lists(&store)[0].id;
    let tasks = store.lock().unwrap().tasks_in(list).unwrap();
    let parent = tasks.iter().find(|t| t.parent.is_none()).unwrap().id;
    let steps: Vec<_> = tasks.iter().filter(|t| t.parent == Some(parent)).collect();
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].title, "Buy soil");
    assert_eq!(steps[1].title, "Find a pot");
    assert_eq!(steps[1].done_at, Some(1_790_589_600));
    let soil = steps[0].id;

    // A step added and one ticked off here reach To Do.
    {
        let mut store = store.lock().unwrap();
        let fields = TaskFields {
            title: "Water it".into(),
            ..TaskFields::default()
        };
        store.add_task_to(list, Some(parent), &fields).unwrap();
        store.set_task_done(soil, true).unwrap();
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let checks = &fake.0.checks["X"];
        assert_eq!(checks.len(), 3);
        assert_eq!(checks[0]["isChecked"], true);
        assert_eq!(checks[2]["displayName"], "Water it");
    }

    // Removed in To Do: the task comes in the delta, the step goes here.
    {
        let mut fake = fake.lock().unwrap();
        fake.0
            .checks
            .get_mut("X")
            .unwrap()
            .retain(|c| c["id"] != "B");
        fake.0.changed.push("X".into());
    }
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let titles: Vec<String> = store
        .lock()
        .unwrap()
        .tasks_in(list)
        .unwrap()
        .into_iter()
        .filter(|t| t.parent.is_some())
        .map(|t| t.title)
        .collect();
    assert_eq!(titles, ["Buy soil", "Water it"]);

    // Deleted here: To Do deletes the checklist item.
    let water = store
        .lock()
        .unwrap()
        .tasks_in(list)
        .unwrap()
        .into_iter()
        .find(|t| t.title == "Water it")
        .unwrap()
        .id;
    store.lock().unwrap().delete_task(water).unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert_eq!(fake.lock().unwrap().0.checks["X"].len(), 1);
}

#[test]
fn to_do_keeps_the_star_labels_and_files() {
    use base64::Engine;
    let b64 = |data: &[u8]| base64::engine::general_purpose::STANDARD.encode(data);
    let mut fake = Graph::default();
    fake.lists
        .insert("D".into(), ("Tasks".into(), "defaultList".into()));
    fake.tasks.insert(
        "X".into(),
        (
            "D".into(),
            json!({ "id": "X", "@odata.etag": "W/\"1\"", "title": "Pay electricity bill",
                    "status": "notStarted", "importance": "normal",
                    "categories": ["Bills"], "hasAttachments": true }),
        ),
    );
    fake.files.insert(
        "X".into(),
        vec![
            json!({ "id": "F1", "name": "bill.pdf", "contentType": "application/pdf",
                     "size": 4, "contentBytes": b64(b"%PDF") }),
        ],
    );
    let (api, fake) = serve(fake, graph);
    let service = graph_service(&api);
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();

    // To Do's categories are labels, and its attachment came with its
    // content, read apart.
    let list = account_lists(&store)[0].id;
    let task = store.lock().unwrap().tasks_in(list).unwrap().remove(0);
    assert!(!task.starred);
    assert_eq!(task.labels, ["Bills"]);
    let files = store.lock().unwrap().files_of_task(task.id).unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "bill.pdf");
    assert_eq!(files[0].remote_id.as_deref(), Some("F1"));
    let bill = files[0].id;
    assert_eq!(
        store.lock().unwrap().task_file_data(bill).unwrap().unwrap(),
        b"%PDF"
    );

    // Starred, labelled and given files here: the star is high
    // importance, labels are categories, a small file is an attachment
    // and one over 3 MB stays here.
    let (note, big) = {
        let mut store = store.lock().unwrap();
        let fields = TaskFields {
            starred: true,
            labels: vec!["Bills".into(), "Home".into()],
            ..TaskFields::of(&task)
        };
        store.edit_task(task.id, &fields).unwrap();
        let note = store
            .add_task_file(task.id, "meter.txt", "text/plain", b"4521 units")
            .unwrap()
            .unwrap();
        let huge = vec![7u8; (graph::MAX_FILE + 1) as usize];
        let big = store
            .add_task_file(task.id, "scan.tiff", "image/tiff", &huge)
            .unwrap()
            .unwrap();
        (note, big)
    };
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let (_, x) = &fake.0.tasks["X"];
        assert_eq!(x["importance"], "high");
        assert_eq!(x["categories"], json!(["Bills", "Home"]));
        let files = &fake.0.files["X"];
        assert_eq!(files.len(), 2);
        assert_eq!(files[1]["name"], "meter.txt");
        assert_eq!(files[1]["contentBytes"], b64(b"4521 units"));
        assert_eq!(
            files[1]["@odata.type"],
            "#microsoft.graph.taskFileAttachment"
        );
    }
    {
        let store = store.lock().unwrap();
        assert!(store.task_file(big).unwrap().unwrap().local_only);
        assert!(store.task_file(note).unwrap().unwrap().remote_id.is_some());
        assert!(store.pending_task_files(list).unwrap().is_empty());
    }

    // Removed here: removed on To Do.
    store.lock().unwrap().remove_task_file(bill).unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert_eq!(fake.lock().unwrap().0.files["X"].len(), 1);

    // Unstarred and relabelled in To Do, a file removed there: here too.
    {
        let mut fake = fake.lock().unwrap();
        let (_, x) = fake.0.tasks.get_mut("X").unwrap();
        x["importance"] = json!("normal");
        x["categories"] = json!(["Work"]);
        x["@odata.etag"] = json!("W/\"99\"");
        fake.0.files.get_mut("X").unwrap().clear();
        fake.0.tasks.get_mut("X").unwrap().1["hasAttachments"] = json!(false);
        fake.0.changed.push("X".into());
    }
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let task = store.lock().unwrap().task(task.id).unwrap().unwrap();
    assert!(!task.starred);
    assert_eq!(task.labels, ["Work"]);
    let left: Vec<String> = store
        .lock()
        .unwrap()
        .files_of_task(task.id)
        .unwrap()
        .into_iter()
        .map(|f| f.name)
        .collect();
    assert_eq!(left, ["scan.tiff"], "the one kept here stays");
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

// --- Zoho Tasks -------------------------------------------------------------------

#[derive(Default)]
struct Zoho {
    /// Group ID to name.
    groups: BTreeMap<String, String>,
    /// Task ID to (list: `me` or a group ID, parent, task JSON).
    tasks: BTreeMap<String, (String, Option<String>, Value)>,
    next: u32,
    /// Every API call answered with this, as for a refused sign-in.
    refuse: Option<(u16, Value)>,
}

impl Zoho {
    fn put(&mut self, list: &str, parent: Option<&str>, mut task: Value) {
        self.next += 1;
        task["modifiedTime"] = json!(format!("2026-09-29T10:{:02}:00+05:30", self.next % 60));
        let id = task["id"].as_str().unwrap().to_owned();
        self.tasks
            .insert(id, (list.to_owned(), parent.map(str::to_owned), task));
    }

    /// Task `id` as Zoho writes it, with its count of subtasks.
    fn shown(&self, id: &str) -> Value {
        let (_, _, task) = &self.tasks[id];
        let mut task = task.clone();
        let count = self
            .tasks
            .values()
            .filter(|(_, p, _)| p.as_deref() == Some(id))
            .count();
        task["numberOfSubtasks"] = json!(count);
        task["subtasks"] = json!([]);
        task
    }
}

fn zoho_ok(data: Value) -> (u16, Value) {
    (
        200,
        json!({ "status": { "code": 200, "description": "success" }, "data": data }),
    )
}

fn zoho_missing() -> (u16, Value) {
    (
        404,
        json!({ "status": { "code": 404, "description": "Invalid Input" },
                "data": { "errorCode": "INVALID_INPUT" } }),
    )
}

fn zoho(state: &mut Zoho, request: &Seen, _base: &str) -> (u16, Value) {
    if request.path == "/token" {
        return (200, json!({ "access_token": "at-2", "expires_in": 3600 }));
    }
    if let Some(refused) = &state.refuse {
        return refused.clone();
    }
    let path = request.path.strip_prefix("/api/tasks/").unwrap();
    let (path, query) = path.split_once('?').unwrap_or((path, ""));
    let parts: Vec<&str> = path.split('/').collect();
    let (list, rest): (String, &[&str]) = match parts.as_slice() {
        ["groups"] => {
            let groups: Vec<Value> = state
                .groups
                .iter()
                .map(|(id, name)| json!({ "id": id.parse::<u64>().unwrap(), "name": name }))
                .collect();
            return zoho_ok(json!({ "groups": groups }));
        }
        ["me", rest @ ..] => ("me".into(), rest),
        ["groups", group, rest @ ..] => ((*group).to_owned(), rest),
        _ => return zoho_missing(),
    };
    match (request.method.as_str(), rest) {
        ("GET", []) => {
            let query: BTreeMap<&str, usize> = query
                .split('&')
                .filter_map(|p| p.split_once('='))
                .map(|(k, v)| (k, v.parse().unwrap()))
                .collect();
            let ids: Vec<String> = state
                .tasks
                .iter()
                .filter(|(_, (l, p, _))| *l == list && p.is_none())
                .map(|(id, _)| id.clone())
                .collect();
            let (from, limit) = (query["from"], query["limit"]);
            let page: Vec<Value> = ids
                .iter()
                .skip(from)
                .take(limit)
                .map(|id| state.shown(id))
                .collect();
            let mut data = json!({ "tasks": page });
            if from + limit < ids.len() {
                data["paging"] =
                    json!({ "nextPage": format!("/api/tasks/me?from={}", from + limit) });
            }
            zoho_ok(data)
        }
        ("POST", []) => {
            state.next += 1;
            let id = format!("Z{}", state.next);
            let body = &request.body;
            let status = if body["status"] == "completed" {
                "Completed"
            } else {
                "In Progress"
            };
            let task = json!({
                "id": id,
                "title": body["title"],
                "description": body["description"].as_str().unwrap_or_default(),
                "dueDate": body["dueDate"].as_str().unwrap_or_default(),
                "status": status,
                "priority": "Normal",
            });
            let parent = body["parentTaskId"].as_str();
            state.put(&list, parent, task);
            zoho_ok(state.shown(&id))
        }
        ("GET", [id]) => {
            if !state.tasks.contains_key(*id) {
                return zoho_missing();
            }
            zoho_ok(json!({ "tasks": [state.shown(id)] }))
        }
        ("PUT", [id]) => {
            let Some((l, parent, mut task)) = state.tasks.get(*id).cloned() else {
                return zoho_missing();
            };
            // Each of Zoho's documented changes carries one field.
            let body = request.body.as_object().unwrap();
            assert_eq!(body.len(), 1, "{body:?}");
            for (key, value) in body {
                task[key] = match (key.as_str(), value.as_str()) {
                    ("status", Some("completed")) => json!("Completed"),
                    ("status", Some("inprogress")) => json!("In Progress"),
                    _ => value.clone(),
                };
            }
            state.put(&l, parent.as_deref(), task);
            zoho_ok(Value::Null)
        }
        ("DELETE", [id]) => {
            let id = (*id).to_owned();
            state
                .tasks
                .retain(|task, (_, p, _)| *task != id && p.as_deref() != Some(id.as_str()));
            zoho_ok(Value::Null)
        }
        ("GET", [id, "subtasks"]) => {
            if !state.tasks.contains_key(*id) {
                return zoho_missing();
            }
            let subtasks: Vec<Value> = state
                .tasks
                .values()
                .filter(|(_, p, _)| p.as_deref() == Some(*id))
                .map(|(_, _, t)| t.clone())
                .collect();
            zoho_ok(json!({ "tasks": subtasks }))
        }
        _ => zoho_missing(),
    }
}

/// A Zoho service at the fake `base`, whose tokens also come from it. Any
/// provider kind will do: the client doesn't look at it.
fn zoho_service(base: &str, scope: Option<&str>) -> TaskService {
    let mut provider = provider(OAuthProvider::Microsoft);
    provider.token_url = format!("{base}/token");
    let tokens = TokenSource::new(provider, "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600))
        .with_scope(scope.map(str::to_owned));
    TaskService::Zoho(zoho::ZohoTasks::with_api(
        Arc::new(tokens),
        Tls::insecure_for_local_tests(),
        &format!("{base}/api"),
    ))
}

fn titles_in(store: &Mutex<Store>, list: i64) -> Vec<String> {
    store
        .lock()
        .unwrap()
        .tasks_in(list)
        .unwrap()
        .into_iter()
        .map(|t| t.title)
        .collect()
}

#[test]
fn zoho_tasks_sync_both_ways() {
    let mut fake = Zoho::default();
    fake.groups.insert("53658048".into(), "Marketing".into());
    fake.put(
        "me",
        None,
        json!({ "id": "P", "title": "Plan the trip", "description": "",
                "status": "In Progress", "dueDate": "05/10/2026", "priority": "Normal" }),
    );
    fake.put(
        "me",
        Some("P"),
        json!({ "id": "S", "title": "Book the train", "status": "Completed" }),
    );
    fake.put(
        "53658048",
        None,
        json!({ "id": "B", "title": "Blog post", "status": "In Progress" }),
    );
    let (base, fake) = serve(fake, zoho);
    let service = zoho_service(&base, Some("ZohoMail.accounts.READ ZohoMail.tasks.ALL"));
    let (_dir, store, account) = store();
    assert!(smol::block_on(service.allowed()).unwrap());

    // Zoho's tasks come in: personal ones in the default list, each
    // group's in a list of its own.
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let lists = account_lists(&store);
    assert_eq!(lists.len(), 2, "{lists:?}");
    let mine = lists.iter().find(|l| l.is_default).unwrap();
    assert_eq!(mine.title, "Personal Tasks");
    let group = lists.iter().find(|l| l.title == "Marketing").unwrap().id;
    let mine = mine.id;
    assert_eq!(titles_in(&store, group), ["Blog post"]);
    let tasks = store.lock().unwrap().tasks_in(mine).unwrap();
    assert_eq!(tasks.len(), 2, "{tasks:?}");
    let plan = tasks.iter().find(|t| t.title == "Plan the trip").unwrap();
    assert_eq!(plan.due, "2026-10-05");
    assert_eq!(plan.done_at, None);
    let step = tasks.iter().find(|t| t.title == "Book the train").unwrap();
    assert_eq!(step.parent, Some(plan.id));
    assert!(step.done_at.is_some());
    let (plan, step) = (plan.id, step.id);
    {
        let fake = fake.lock().unwrap();
        let requests = &fake.1;
        assert!(
            requests
                .iter()
                .any(|s| s.path == "/api/tasks/me/P/subtasks")
        );
    }

    // Added and changed here.
    {
        let mut store = store.lock().unwrap();
        store
            .add_task_to(
                mine,
                None,
                &TaskFields {
                    title: "Call mum".into(),
                    due: "2026-10-02".into(),
                    ..TaskFields::default()
                },
            )
            .unwrap();
        let fields = TaskFields {
            title: "Plan the trip to Goa".into(),
            notes: "Window seats".into(),
            due: "2026-10-06".into(),
            ..TaskFields::default()
        };
        store.edit_task(plan, &fields).unwrap();
        store.set_task_done(step, false).unwrap();
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let (_, _, p) = &fake.0.tasks["P"];
        assert_eq!(p["title"], "Plan the trip to Goa");
        assert_eq!(p["description"], "Window seats");
        assert_eq!(p["dueDate"], "06/10/2026");
        let (_, _, s) = &fake.0.tasks["S"];
        assert_eq!(s["status"], "In Progress");
        let (list, parent, call) = fake
            .0
            .tasks
            .values()
            .find(|(_, _, t)| t["title"] == "Call mum")
            .expect("sent to Zoho");
        assert_eq!((list.as_str(), parent), ("me", &None));
        assert_eq!(call["dueDate"], "02/10/2026");
        // Only what changed went, a field at a time.
        let puts: Vec<&Seen> = fake
            .1
            .iter()
            .filter(|s| s.method == "PUT" && s.path == "/api/tasks/me/P")
            .collect();
        assert_eq!(puts.len(), 3, "{puts:?}");
    }
    assert!(
        store
            .lock()
            .unwrap()
            .pending_tasks(mine)
            .unwrap()
            .is_empty()
    );

    // A step added here goes under its task on Zoho.
    store
        .lock()
        .unwrap()
        .add_task_to(
            mine,
            Some(plan),
            &TaskFields {
                title: "Pack".into(),
                ..TaskFields::default()
            },
        )
        .unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let (_, parent, _) = fake
            .0
            .tasks
            .values()
            .find(|(_, _, t)| t["title"] == "Pack")
            .unwrap();
        assert_eq!(parent.as_deref(), Some("P"));
    }

    // Ticked off and deleted here.
    let call = store
        .lock()
        .unwrap()
        .tasks_in(mine)
        .unwrap()
        .into_iter()
        .find(|t| t.title == "Call mum")
        .unwrap()
        .id;
    let blog = store.lock().unwrap().tasks_in(group).unwrap()[0].id;
    store.lock().unwrap().set_task_done(call, true).unwrap();
    store.lock().unwrap().delete_task(blog).unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        let (_, _, call) = fake
            .0
            .tasks
            .values()
            .find(|(_, _, t)| t["title"] == "Call mum")
            .unwrap();
        assert_eq!(call["status"], "Completed");
        assert!(!fake.0.tasks.contains_key("B"));
        assert!(
            fake.1
                .iter()
                .any(|s| s.method == "DELETE" && s.path == "/api/tasks/groups/53658048/B")
        );
    }
    assert!(titles_in(&store, group).is_empty());
    let done = store.lock().unwrap().task(call).unwrap().unwrap();
    assert!(done.done_at.is_some());

    // Changed on Zoho: renamed, one added, a subtask deleted.
    {
        let mut fake = fake.lock().unwrap();
        let (_, _, mut p) = fake.0.tasks["P"].clone();
        p["title"] = json!("Plan the trip to Kerala");
        fake.0.put("me", None, p);
        fake.0.put(
            "me",
            None,
            json!({ "id": "N", "title": "Pay rent", "status": "In Progress",
                    "dueDate": "01/10/2026" }),
        );
        fake.0.tasks.remove("S");
    }
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let mut titles = titles_in(&store, mine);
    titles.sort();
    assert_eq!(
        titles,
        ["Call mum", "Pack", "Pay rent", "Plan the trip to Kerala"]
    );
    let rent = store
        .lock()
        .unwrap()
        .tasks_in(mine)
        .unwrap()
        .into_iter()
        .find(|t| t.title == "Pay rent")
        .unwrap();
    assert_eq!(rent.due, "2026-10-01");
    // Every call carried Zoho's own kind of token.
    let fake = fake.lock().unwrap();
    assert!(fake.1.iter().all(|s| s.auth == "Zoho-oauthtoken at-1"));
}

#[test]
fn the_star_is_zohos_high_priority_and_labels_stay_here() {
    let mut fake = Zoho::default();
    fake.put(
        "me",
        None,
        json!({ "id": "H", "title": "Renew passport", "status": "In Progress",
                "priority": "High" }),
    );
    fake.put(
        "me",
        None,
        json!({ "id": "L", "title": "Sort old photos", "status": "In Progress",
                "priority": "Low" }),
    );
    let (base, fake) = serve(fake, zoho);
    let service = zoho_service(&base, Some("ZohoMail.tasks.ALL"));
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let list = account_lists(&store)[0].id;
    let tasks = store.lock().unwrap().tasks_in(list).unwrap();
    let high = tasks.iter().find(|t| t.title == "Renew passport").unwrap();
    let low = tasks.iter().find(|t| t.title == "Sort old photos").unwrap();
    assert!(high.starred);
    assert!(!low.starred);

    // Unstarred here: no priority. A label stays on this computer and
    // sends nothing; the low priority of the other is left alone.
    {
        let mut store = store.lock().unwrap();
        let fields = TaskFields {
            starred: false,
            ..TaskFields::of(high)
        };
        store.edit_task(high.id, &fields).unwrap();
        let fields = TaskFields {
            labels: vec!["Home".into()],
            ..TaskFields::of(low)
        };
        store.edit_task(low.id, &fields).unwrap();
        // A file stays here.
        store
            .add_task_file(low.id, "album.txt", "text/plain", b"2009")
            .unwrap();
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let fake = fake.lock().unwrap();
        assert_eq!(fake.0.tasks["H"].2["priority"], "none");
        assert_eq!(fake.0.tasks["L"].2["priority"], "Low");
        assert!(
            !fake
                .1
                .iter()
                .any(|s| s.method == "PUT" && s.path == "/api/tasks/me/L")
        );
    }
    let low = store.lock().unwrap().task(low.id).unwrap().unwrap();
    assert_eq!(low.labels, ["Home"]);
    let file = store
        .lock()
        .unwrap()
        .files_of_task(low.id)
        .unwrap()
        .remove(0);
    assert!(file.local_only);

    // Starred here: high on Zoho; starred on Zoho: here.
    {
        let mut store = store.lock().unwrap();
        let fields = TaskFields {
            starred: true,
            ..TaskFields::of(&low)
        };
        store.edit_task(low.id, &fields).unwrap();
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert_eq!(fake.lock().unwrap().0.tasks["L"].2["priority"], "high");
    {
        let mut fake = fake.lock().unwrap();
        let (_, _, mut h) = fake.0.tasks["H"].clone();
        h["priority"] = json!("High");
        fake.0.put("me", None, h);
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let tasks = store.lock().unwrap().tasks_in(list).unwrap();
    assert!(tasks.iter().all(|t| t.starred), "{tasks:?}");
    assert_eq!(
        tasks
            .iter()
            .find(|t| t.title == "Sort old photos")
            .unwrap()
            .labels,
        ["Home"]
    );
}

#[test]
fn zoho_lists_stay_as_zoho_has_them() {
    let mut fake = Zoho::default();
    fake.put(
        "me",
        None,
        json!({ "id": "P", "title": "Water plants", "status": "In Progress" }),
    );
    let (base, fake) = serve(fake, zoho);
    let service = zoho_service(&base, None);
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let mine = account_lists(&store)[0].id;

    // Zoho makes no lists: one made here stays here, and the round goes
    // on; a new name for Zoho's list is not sent and Zoho's comes back.
    let home = store
        .lock()
        .unwrap()
        .add_task_list(Some(account), "Home")
        .unwrap();
    store
        .lock()
        .unwrap()
        .rename_task_list(mine, "Mine")
        .unwrap();
    fake.lock().unwrap().0.put(
        "me",
        None,
        json!({ "id": "N", "title": "Pay rent", "status": "In Progress" }),
    );
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let lists = account_lists(&store);
    assert_eq!(lists.len(), 2, "{lists:?}");
    assert!(lists.iter().any(|l| l.id == home));
    let mine = lists.iter().find(|l| l.id == mine).unwrap();
    assert_eq!(mine.title, "Personal Tasks");
    assert_eq!(titles_in(&store, mine.id).len(), 2);
    assert!(fake.lock().unwrap().1.iter().all(|s| s.method == "GET"));
}

#[test]
fn a_zoho_sign_in_without_tasks_is_not_allowed() {
    // The grant names its scopes, without the tasks'.
    let service = zoho_service("http://127.0.0.1:1", Some("ZohoMail.messages.ALL"));
    assert!(!smol::block_on(service.allowed()).unwrap());

    // Zoho refuses the token, also a fresh one.
    let fake = Zoho {
        refuse: Some((
            401,
            json!({ "status": { "code": 401, "description": "Unauthorized" },
                    "data": { "errorCode": "INVALID_OAUTHTOKEN" } }),
        )),
        ..Zoho::default()
    };
    let (base, seen) = serve(fake, zoho);
    assert!(!smol::block_on(zoho_service(&base, None).allowed()).unwrap());
    assert!(seen.lock().unwrap().1.iter().any(|s| s.path == "/token"));

    // A token without the tasks scope: Zoho says so with a 404.
    let fake = Zoho {
        refuse: Some((
            404,
            json!({ "status": { "code": 404, "description": "Invalid Input" },
                    "data": { "errorCode": "INVALID_OAUTHSCOPE" } }),
        )),
        ..Zoho::default()
    };
    let (base, _) = serve(fake, zoho);
    let service = zoho_service(&base, None);
    assert!(!smol::block_on(service.allowed()).unwrap());
    let (_dir, store, account) = store();
    let err = smol::block_on(sync_account(&service, &store, account)).unwrap_err();
    assert!(matches!(err, Error::Auth(_)), "{err}");
}

#[test]
fn zoho_pages_through_many_tasks() {
    let mut fake = Zoho::default();
    for n in 0..450 {
        fake.put(
            "me",
            None,
            json!({ "id": format!("T{n:03}"), "title": format!("Task {n}"),
                    "status": "In Progress" }),
        );
    }
    let (base, fake) = serve(fake, zoho);
    let service = zoho_service(&base, None);
    let (_dir, store, account) = store();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let mine = account_lists(&store)[0].id;
    assert_eq!(titles_in(&store, mine).len(), 450);
    let pages = fake
        .lock()
        .unwrap()
        .1
        .iter()
        .filter(|s| s.path.starts_with("/api/tasks/me?"))
        .count();
    assert_eq!(pages, 3);
}
