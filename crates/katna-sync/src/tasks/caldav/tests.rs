// SPDX-License-Identifier: GPL-3.0-or-later

//! CalDAV to-dos against a fake CalDAV server on the loopback.

use std::sync::{Arc, Mutex};

use katna_core::{AccountKind, Paths};
use katna_store::{Mode, Store, tasks::TaskFields};

use super::*;
use crate::{
    calendar::fake::{self, Answer, Seen},
    net::Tls,
    tasks::{TaskService, sync_account},
};

const LIST: &str = "/dav/calendars/me/tasks/";

#[derive(Default)]
struct Server {
    ctag: u32,
    /// href → (etag, iCalendar).
    todos: BTreeMap<String, (String, String)>,
}

impl Server {
    fn put(&mut self, href: &str, text: &str) {
        self.ctag += 1;
        let etag = format!("\"e{}\"", self.ctag);
        self.todos.insert(href.to_owned(), (etag, text.to_owned()));
    }
}

fn multistatus(responses: &str) -> Answer {
    Answer::xml(
        207,
        format!(
            r#"<?xml version="1.0"?><d:multistatus xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav" xmlns:cs="http://calendarserver.org/ns/">{responses}</d:multistatus>"#
        ),
    )
}

fn ok(href: &str, props: &str) -> String {
    format!(
        "<d:response><d:href>{href}</d:href><d:propstat><d:prop>{props}</d:prop>\
         <d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>"
    )
}

fn answer(server: &Mutex<Server>, request: &Seen) -> Answer {
    let mut server = server.lock().unwrap();
    let route = request.route().to_owned();
    match (request.method.as_str(), route.as_str()) {
        ("PROPFIND", "/.well-known/caldav") => Answer::redirect("/dav/"),
        ("PROPFIND", "/dav/") => multistatus(&ok(
            "/dav/",
            "<d:current-user-principal><d:href>/dav/principals/me/</d:href></d:current-user-principal>",
        )),
        ("PROPFIND", "/dav/principals/me/") => multistatus(&ok(
            "/dav/principals/me/",
            "<c:calendar-home-set><d:href>/dav/calendars/me/</d:href></c:calendar-home-set>",
        )),
        ("PROPFIND", "/dav/calendars/me/") => {
            let work = ok(
                "/dav/calendars/me/work/",
                "<d:resourcetype><d:collection/><c:calendar/></d:resourcetype>\
                 <d:displayname>Work</d:displayname>\
                 <c:supported-calendar-component-set><c:comp name=\"VEVENT\"/></c:supported-calendar-component-set>",
            );
            let tasks = ok(
                LIST,
                "<d:resourcetype><d:collection/><c:calendar/></d:resourcetype>\
                 <d:displayname>Reminders</d:displayname>\
                 <c:supported-calendar-component-set><c:comp name=\"VTODO\"/></c:supported-calendar-component-set>",
            );
            multistatus(&format!("{work}{tasks}"))
        }
        ("PROPFIND", LIST) => multistatus(&ok(
            LIST,
            &format!("<cs:getctag>c{}</cs:getctag>", server.ctag),
        )),
        ("REPORT", LIST) if request.text().contains("calendar-query") => {
            let mut all = ok(LIST, "<d:getetag>\"col\"</d:getetag>");
            for (href, (etag, _)) in &server.todos {
                all.push_str(&ok(href, &format!("<d:getetag>{etag}</d:getetag>")));
            }
            multistatus(&all)
        }
        ("REPORT", LIST) => {
            let body = request.text();
            let mut all = String::new();
            for (href, (etag, data)) in &server.todos {
                if body.contains(&format!("<d:href>{href}</d:href>")) {
                    let data = data.replace('&', "&amp;").replace('<', "&lt;");
                    all.push_str(&ok(
                        href,
                        &format!(
                            "<d:getetag>{etag}</d:getetag><c:calendar-data>{data}</c:calendar-data>"
                        ),
                    ));
                }
            }
            multistatus(&all)
        }
        ("PUT", href) if href.starts_with(LIST) => {
            let current = server.todos.get(href).map(|(etag, _)| etag.clone());
            let clash = match (request.header("If-None-Match"), request.header("If-Match")) {
                (Some("*"), _) => current.is_some(),
                (_, Some(wanted)) => current.as_deref() != Some(wanted),
                _ => false,
            };
            if clash {
                return Answer::xml(412, "");
            }
            let created = current.is_none();
            server.put(href, &request.text());
            Answer::xml(if created { 201 } else { 204 }, "")
        }
        ("DELETE", href) if server.todos.contains_key(href) => {
            server.todos.remove(href);
            server.ctag += 1;
            Answer::xml(204, "")
        }
        _ => Answer::xml(404, ""),
    }
}

fn todo(uid: &str, title: &str, extra: &str) -> String {
    format!(
        "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VTODO\r\nUID:{uid}\r\nSUMMARY:{title}\r\n\
         {extra}CATEGORIES:Kept\r\nEND:VTODO\r\nEND:VCALENDAR\r\n"
    )
}

#[test]
fn to_dos_sync_both_ways() {
    let server = Arc::new(Mutex::new(Server::default()));
    {
        let mut server = server.lock().unwrap();
        server.put(
            &format!("{LIST}a.ics"),
            &todo(
                "a",
                "Renew passport",
                "DUE;VALUE=DATE:20261101\r\nPRIORITY:1\r\n",
            ),
        );
        server.put(
            &format!("{LIST}b.ics"),
            &todo("b", "Find photos", "RELATED-TO;RELTYPE=PARENT:a\r\n"),
        );
    }
    let inner = server.clone();
    let (origin, seen) = fake::serve(move |request| answer(&inner, request));
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let service = TaskService::CalDav(DavTasks::new(dav));
    let dir = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Me", "me@example.test")
        .unwrap()
        .id;
    let store = Mutex::new(store);

    assert!(smol::block_on(service.allowed()).unwrap());
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let lists: Vec<_> = store
        .lock()
        .unwrap()
        .task_lists()
        .unwrap()
        .into_iter()
        .filter(|l| l.account.is_some())
        .collect();
    assert_eq!(lists.len(), 1, "the calendar of events is no task list");
    assert_eq!(lists[0].title, "Reminders");
    assert!(lists[0].is_default);
    let list = lists[0].id;
    let tasks = store.lock().unwrap().tasks_in(list).unwrap();
    let passport = tasks.iter().find(|t| t.title == "Renew passport").unwrap();
    let photos = tasks.iter().find(|t| t.title == "Find photos").unwrap();
    assert_eq!(passport.due, "2026-11-01");
    assert!(passport.starred);
    assert_eq!(photos.parent, Some(passport.id));
    let (passport, photos) = (passport.id, photos.id);

    // The same ctag: no listing.
    seen.lock().unwrap().clear();
    assert!(!smol::block_on(sync_account(&service, &store, account)).unwrap());
    assert!(seen.lock().unwrap().iter().all(|s| s.method != "REPORT"));

    // Ticked off, a new step and a new task here: all reach the server,
    // over the server's own text.
    {
        let mut store = store.lock().unwrap();
        store.set_task_done(photos, true).unwrap();
        let step = TaskFields {
            title: "Fill the form".into(),
            ..TaskFields::default()
        };
        store.add_task_to(list, Some(passport), &step).unwrap();
        let task = TaskFields {
            title: "Call the bank".into(),
            ..TaskFields::default()
        };
        store.add_task_to(list, None, &task).unwrap();
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let server = server.lock().unwrap();
        assert_eq!(server.todos.len(), 4);
        let b = &server.todos[&format!("{LIST}b.ics")].1;
        assert!(b.contains("STATUS:COMPLETED\r\n"));
        assert!(b.contains("CATEGORIES:Kept\r\n"));
        assert!(b.contains("RELATED-TO;RELTYPE=PARENT:a\r\n"));
        let form = server
            .todos
            .values()
            .find(|(_, t)| t.contains("SUMMARY:Fill the form"))
            .unwrap();
        assert!(form.1.contains("RELATED-TO;RELTYPE=PARENT:a\r\n"));
    }
    let tasks = store.lock().unwrap().tasks_in(list).unwrap();
    assert_eq!(tasks.len(), 4);
    assert!(tasks.iter().all(|t| !t.title.is_empty()));

    // Deleted on the server, and one deleted here.
    {
        let mut server = server.lock().unwrap();
        server.todos.remove(&format!("{LIST}a.ics"));
        server.ctag += 1;
    }
    let bank = tasks
        .iter()
        .find(|t| t.title == "Call the bank")
        .unwrap()
        .id;
    store.lock().unwrap().delete_task(bank).unwrap();
    assert!(smol::block_on(sync_account(&service, &store, account)).unwrap());
    let left: Vec<String> = store
        .lock()
        .unwrap()
        .tasks_in(list)
        .unwrap()
        .into_iter()
        .map(|t| t.title)
        .collect();
    assert!(!left.contains(&"Renew passport".to_owned()));
    assert!(!left.contains(&"Call the bank".to_owned()));
    let server = server.lock().unwrap();
    assert!(
        server
            .todos
            .values()
            .all(|(_, t)| !t.contains("Call the bank"))
    );
}

#[test]
fn labels_are_categories_and_files_go_inside_the_to_do() {
    let server = Arc::new(Mutex::new(Server::default()));
    server.lock().unwrap().put(
        &format!("{LIST}a.ics"),
        &todo(
            "a",
            "Pay electricity bill",
            "PRIORITY:5\r\nATTACH:https://example.test/tariff.pdf\r\n",
        ),
    );
    let inner = server.clone();
    let (origin, _seen) = fake::serve(move |request| answer(&inner, request));
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let service = TaskService::CalDav(DavTasks::new(dav));
    let dir = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Me", "me@example.test")
        .unwrap()
        .id;
    let store = Mutex::new(store);
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let list = store.lock().unwrap().task_lists().unwrap()[0].id;
    let task = store.lock().unwrap().tasks_in(list).unwrap().remove(0);
    // The server's categories are its labels; priority 5 is no star.
    assert_eq!(task.labels, ["Kept"]);
    assert!(!task.starred);

    // Starred, a label and a file here: PRIORITY:1, CATEGORIES and an
    // inline ATTACH; the server's link to a file stays.
    {
        let mut store = store.lock().unwrap();
        let fields = TaskFields {
            starred: true,
            labels: vec!["Kept".into(), "Home".into()],
            ..TaskFields::of(&task)
        };
        store.edit_task(task.id, &fields).unwrap();
        store
            .add_task_file(task.id, "meter.txt", "text/plain", b"4521 units")
            .unwrap();
        let huge = vec![1u8; (MAX_FILE + 1) as usize];
        store
            .add_task_file(task.id, "scan.tiff", "image/tiff", &huge)
            .unwrap();
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let server = server.lock().unwrap();
        let text = &server.todos[&format!("{LIST}a.ics")].1;
        assert!(text.contains("PRIORITY:1\r\n"), "{text}");
        assert!(!text.contains("PRIORITY:5"));
        assert!(text.contains("CATEGORIES:Kept,Home\r\n"), "{text}");
        assert!(text.contains("ATTACH:https://example.test/tariff.pdf\r\n"));
        let unfolded = text.replace("\r\n ", "");
        assert!(
            unfolded.contains("FILENAME=meter.txt:NDUyMSB1bml0cw==\r\n"),
            "{text}"
        );
        assert!(!unfolded.contains("scan.tiff"));
    }
    let files = store.lock().unwrap().files_of_task(task.id).unwrap();
    let meter = files.iter().find(|f| f.name == "meter.txt").unwrap();
    assert!(meter.remote_id.as_deref().unwrap().starts_with("inline:"));
    assert!(
        files
            .iter()
            .find(|f| f.name == "scan.tiff")
            .unwrap()
            .local_only
    );

    // The next pull reads the same file back: nothing doubles.
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert_eq!(
        store.lock().unwrap().files_of_task(task.id).unwrap().len(),
        2
    );

    // Unstarred, and the file removed, here: PRIORITY goes, so does the
    // ATTACH.
    {
        let mut store = store.lock().unwrap();
        let task = store.task(task.id).unwrap().unwrap();
        let fields = TaskFields {
            starred: false,
            ..TaskFields::of(&task)
        };
        store.edit_task(task.id, &fields).unwrap();
        store.remove_task_file(meter.id).unwrap();
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    {
        let server = server.lock().unwrap();
        let text = &server.todos[&format!("{LIST}a.ics")].1;
        assert!(!text.contains("PRIORITY"), "{text}");
        assert!(!text.contains("meter.txt"), "{text}");
        assert!(text.contains("tariff.pdf"));
    }

    // Labels changed on the server come here.
    {
        let mut server = server.lock().unwrap();
        let href = format!("{LIST}a.ics");
        let text = server.todos[&href]
            .1
            .replace("CATEGORIES:Kept,Home", "CATEGORIES:Work");
        server.put(&href, &text);
    }
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let task = store.lock().unwrap().task(task.id).unwrap().unwrap();
    assert_eq!(task.labels, ["Work"]);
}

#[test]
fn files_a_server_refuses_stay_here() {
    let server = Arc::new(Mutex::new(Server::default()));
    server
        .lock()
        .unwrap()
        .put(&format!("{LIST}a.ics"), &todo("a", "Insurance", ""));
    let inner = server.clone();
    // This server takes no attachments.
    let (origin, _seen) = fake::serve(move |request| {
        if request.method == "PUT" && request.text().contains("ATTACH") {
            return Answer::xml(413, "");
        }
        answer(&inner, request)
    });
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let service = TaskService::CalDav(DavTasks::new(dav));
    let dir = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Me", "me@example.test")
        .unwrap()
        .id;
    let store = Mutex::new(store);
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    let list = store.lock().unwrap().task_lists().unwrap()[0].id;
    let task = store.lock().unwrap().tasks_in(list).unwrap()[0].id;
    let file = store
        .lock()
        .unwrap()
        .add_task_file(task, "policy.pdf", "application/pdf", b"%PDF")
        .unwrap()
        .unwrap();
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert!(
        store
            .lock()
            .unwrap()
            .task_file(file)
            .unwrap()
            .unwrap()
            .local_only
    );
    assert!(
        store
            .lock()
            .unwrap()
            .pending_task_files(list)
            .unwrap()
            .is_empty()
    );
    assert!(
        !server.lock().unwrap().todos[&format!("{LIST}a.ics")]
            .1
            .contains("ATTACH")
    );
    // Still there after the next pull.
    smol::block_on(sync_account(&service, &store, account)).unwrap();
    assert!(store.lock().unwrap().task_file(file).unwrap().is_some());
}
