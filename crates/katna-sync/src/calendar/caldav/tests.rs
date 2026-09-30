// SPDX-License-Identifier: GPL-3.0-or-later

//! The CalDAV client against a fake CalDAV server on the loopback.

use std::sync::{Arc, Mutex};

use katna_core::Paths;
use katna_store::Mode;

use super::*;
use crate::calendar::fake::{self, Answer, Seen};

const ACCOUNT: AccountId = AccountId(3);

const A: &str = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:a@test\r\n\
DTSTART;TZID=Europe/Berlin:20261001T090000\r\nDTEND;TZID=Europe/Berlin:20261001T100000\r\n\
SUMMARY:Dentist & more\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";

struct Server {
    ctag: String,
    /// href → (etag, iCalendar).
    events: Vec<(String, String, String)>,
}

fn b(title: &str) -> String {
    format!(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nUID:b@test\r\nDTSTART;VALUE=DATE:20261010\r\n\
         RRULE:FREQ=WEEKLY;COUNT=3\r\nSUMMARY:{title}\r\nEND:VEVENT\r\n\
         BEGIN:VEVENT\r\nUID:b@test\r\nRECURRENCE-ID;VALUE=DATE:20261017\r\n\
         DTSTART;VALUE=DATE:20261018\r\nSUMMARY:{title} (moved)\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n"
    )
}

fn multistatus(responses: &str) -> Answer {
    Answer::xml(
        207,
        format!(
            r#"<?xml version="1.0"?><d:multistatus xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav" xmlns:cs="http://calendarserver.org/ns/" xmlns:a="http://apple.com/ns/ical/">{responses}</d:multistatus>"#
        ),
    )
}

fn ok(href: &str, props: &str) -> String {
    format!(
        "<d:response><d:href>{href}</d:href><d:propstat><d:prop>{props}</d:prop>\
         <d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>"
    )
}

fn fake_dav(server: Arc<Mutex<Server>>) -> (String, Arc<Mutex<Vec<Seen>>>) {
    fake::serve(move |request| {
        assert_eq!(
            request.header("Authorization"),
            Some(format!("Basic {}", STANDARD.encode("me:secret")).as_str())
        );
        let server = server.lock().unwrap();
        match (request.method.as_str(), request.route()) {
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
                assert_eq!(request.header("Depth"), Some("1"));
                let home = ok(
                    "/dav/calendars/me/",
                    "<d:resourcetype><d:collection/></d:resourcetype>",
                );
                let work = ok(
                    "/dav/calendars/me/work/",
                    &format!(
                        "<d:resourcetype><d:collection/><c:calendar/></d:resourcetype>\
                         <d:displayname>Work</d:displayname>\
                         <a:calendar-color>#FF2968FF</a:calendar-color>\
                         <c:supported-calendar-component-set><c:comp name=\"VEVENT\"/></c:supported-calendar-component-set>\
                         <cs:getctag>{}</cs:getctag>\
                         <d:current-user-privilege-set><d:privilege><d:read/></d:privilege><d:privilege><d:write/></d:privilege></d:current-user-privilege-set>",
                        server.ctag
                    ),
                );
                let tasks = ok(
                    "/dav/calendars/me/tasks/",
                    "<d:resourcetype><d:collection/><c:calendar/></d:resourcetype>\
                     <d:displayname>Tasks</d:displayname>\
                     <c:supported-calendar-component-set><c:comp name=\"VTODO\"/></c:supported-calendar-component-set>",
                );
                multistatus(&format!("{home}{work}{tasks}"))
            }
            ("REPORT", "/dav/calendars/me/work/") if request.text().contains("calendar-query") => {
                let mut all = ok("/dav/calendars/me/work/", "<d:getetag>\"col\"</d:getetag>");
                for (href, etag, _) in &server.events {
                    all.push_str(&ok(href, &format!("<d:getetag>{etag}</d:getetag>")));
                }
                multistatus(&all)
            }
            ("REPORT", "/dav/calendars/me/work/") => {
                let body = request.text();
                let mut all = String::new();
                for (href, etag, data) in &server.events {
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
            _ => Answer::xml(404, ""),
        }
    })
}

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    (dir, store)
}

fn rows(store: &Store) -> Vec<EventData> {
    let mut rows: Vec<EventData> = store
        .event_rows_in_range(0, i64::MAX / 2)
        .unwrap()
        .into_iter()
        .map(|r| r.data)
        .collect();
    rows.sort_by_key(|r| (r.remote_id.clone(), r.recurrence_id));
    rows
}

fn reports(seen: &Mutex<Vec<Seen>>) -> Vec<String> {
    seen.lock()
        .unwrap()
        .iter()
        .filter(|s| s.method == "REPORT")
        .map(Seen::text)
        .collect()
}

#[test]
fn discovers_and_downloads_only_changed_events() {
    let server = Arc::new(Mutex::new(Server {
        ctag: "c1".into(),
        events: vec![
            (
                "/dav/calendars/me/work/a.ics".into(),
                "\"1\"".into(),
                A.into(),
            ),
            (
                "/dav/calendars/me/work/b.ics".into(),
                "\"2\"".into(),
                b("Swim"),
            ),
        ],
    }));
    let (origin, seen) = fake_dav(server.clone());
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let (_dir, mut store) = store();

    assert!(smol::block_on(dav.sync(&mut store, ACCOUNT, "me@test")).unwrap());
    let calendars = store.calendars().unwrap();
    assert_eq!(calendars.len(), 1, "the task list is not a calendar");
    assert_eq!(calendars[0].name, "Work");
    assert_eq!(calendars[0].color, "#ff2968");
    assert_eq!(calendars[0].remote_id, "/dav/calendars/me/work/");
    assert_eq!(calendars[0].access, CalendarAccess::Owner);
    assert_eq!(calendars[0].sync_token.as_deref(), Some("c1"));
    let events = rows(&store);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].title, "Dentist & more");
    assert_eq!(events[0].etag.as_deref(), Some("\"1\""));
    assert_eq!(events[0].time_zone, "Europe/Berlin");
    assert_eq!(events[1].remote_id, "/dav/calendars/me/work/b.ics");
    assert_eq!(events[1].rrule, "FREQ=WEEKLY;COUNT=3");
    assert!(events[1].all_day);
    assert_eq!(events[2].title, "Swim (moved)");
    assert_eq!(events[2].recurrence_id, Some(events[1].start + 7 * 86_400));

    // The same ctag: nothing is listed again.
    seen.lock().unwrap().clear();
    assert!(!smol::block_on(dav.sync(&mut store, ACCOUNT, "me@test")).unwrap());
    assert!(reports(&seen).is_empty());
    assert!(
        seen.lock()
            .unwrap()
            .iter()
            .all(|s| s.route() != "/.well-known/caldav"),
        "the calendar home is remembered"
    );

    // a.ics went, b.ics changed: only b.ics is downloaded.
    {
        let mut server = server.lock().unwrap();
        server.ctag = "c2".into();
        server.events.remove(0);
        server.events[0].1 = "\"3\"".into();
        server.events[0].2 = b("Swim longer");
    }
    assert!(smol::block_on(dav.sync(&mut store, ACCOUNT, "me@test")).unwrap());
    let multiget = reports(&seen)
        .into_iter()
        .find(|r| r.contains("calendar-multiget"))
        .unwrap();
    assert!(multiget.contains("b.ics") && !multiget.contains("a.ics"));
    let events = rows(&store);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].title, "Swim longer");
    assert_eq!(
        store.calendars().unwrap()[0].sync_token.as_deref(),
        Some("c2")
    );
}

#[test]
fn a_server_without_caldav_offers_no_calendars() {
    let (origin, seen) = fake::serve(|_| Answer::xml(404, ""));
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let (_dir, mut store) = store();
    let err = smol::block_on(dav.sync(&mut store, ACCOUNT, "me@test")).unwrap_err();
    assert!(matches!(err, CalendarError::NotOffered), "{err}");
    // It says what the server answered.
    assert!(
        dav.missing_why().ends_with("answered 404"),
        "{}",
        dav.missing_why()
    );
    let asked = seen.lock().unwrap().len();
    // Not asked again for a while.
    let err = smol::block_on(dav.sync(&mut store, ACCOUNT, "me@test")).unwrap_err();
    assert!(matches!(err, CalendarError::NotOffered), "{err}");
    assert_eq!(seen.lock().unwrap().len(), asked);
}

#[test]
fn a_stalled_server_is_asked_again_and_named() {
    // Takes connections and never answers.
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let taken = Arc::new(Mutex::new(Vec::new()));
    let held = taken.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            held.lock().unwrap().push(stream.unwrap());
        }
    });
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let (_dir, mut store) = store();
    let err = smol::block_on(dav.sync(&mut store, ACCOUNT, "me@test")).unwrap_err();
    // Not "no calendars": the server was not reached, and says so.
    let CalendarError::Failed(Error::Unreachable(why)) = err else {
        panic!("{err}");
    };
    assert_eq!(why, "127.0.0.1 did not answer within 2s");
    assert_eq!(dav.missing_why(), "");
    // Asked again on the next round, not hours later.
    let asked = taken.lock().unwrap().len();
    let _ = smol::block_on(dav.sync(&mut store, ACCOUNT, "me@test"));
    assert!(taken.lock().unwrap().len() > asked);
}

#[test]
fn the_password_goes_only_to_the_accounts_domain() {
    let dav = CalDav::new(
        "imap.example.com",
        "me",
        "secret",
        Tls::insecure_for_local_tests(),
    );
    assert!(dav.trusted("https://imap.example.com/dav/"));
    assert!(dav.trusted("https://caldav.example.com/dav/"));
    assert!(dav.trusted("https://example.com/"));
    assert!(!dav.trusted("https://example.com.evil.test/"));
    assert!(!dav.trusted("https://evil.test/example.com"));
    assert!(!dav.trusted("http://caldav.example.com/"));
    assert_eq!(
        dav.absolute("https://imap.example.com/dav/", "/x/y.ics")
            .as_deref(),
        Some("https://imap.example.com/x/y.ics")
    );
    assert_eq!(
        dav.absolute("https://imap.example.com/dav/", "https://evil.test/"),
        None
    );
    assert_eq!(path("https://imap.example.com/a/b.ics"), "/a/b.ics");
}

#[test]
fn a_yahoo_password_goes_to_yahoos_calendar_server_only() {
    let dav = CalDav::for_account(
        "me@yahoo.com",
        "imap.mail.yahoo.com",
        "me",
        "secret",
        Tls::insecure_for_local_tests(),
    );
    assert_eq!(
        dav.starts[0],
        "https://caldav.calendar.yahoo.com/.well-known/caldav"
    );
    assert!(dav.trusted("https://caldav.calendar.yahoo.com/dav/me/"));
    assert!(dav.trusted("https://imap.mail.yahoo.com/"));
    assert!(!dav.trusted("https://yahoo.com.evil.test/"));
    assert!(!dav.trusted("http://caldav.calendar.yahoo.com/"));
}

#[test]
fn google_caldav_uses_the_sign_in_and_says_when_it_is_off() {
    let (origin, seen) = fake::serve(|request| {
        assert_eq!(request.header("Authorization"), Some("Bearer at-1"));
        match request.route() {
            "/caldav/v2/me/user" => Answer::xml(
                403,
                "CalDAV API has not been used in project 1 before or it is disabled.",
            ),
            _ => Answer::xml(404, ""),
        }
    });
    let provider = crate::oauth::Provider {
        kind: katna_core::OAuthProvider::Google,
        auth_url: "https://accounts.test/auth".into(),
        token_url: "http://127.0.0.1:1/token".into(),
        client_id: "katna-test".into(),
        client_secret: "not-secret".into(),
        scope: String::new(),
        consent: String::new(),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    };
    let tokens = TokenSource::new(provider, "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600));
    let dav = CalDav::google_at(&origin, Arc::new(tokens), Tls::insecure_for_local_tests());
    let (_dir, mut store) = store();
    let err = smol::block_on(dav.sync(&mut store, ACCOUNT, "me@gmail.com")).unwrap_err();
    assert!(matches!(err, CalendarError::NotEnabled(_)), "{err}");
    assert_eq!(seen.lock().unwrap().len(), 1);
}

#[test]
fn a_server_that_names_no_principal_is_asked_for_the_home_itself() {
    let (origin, _seen) = fake::serve(|request| match (request.method.as_str(), request.route()) {
        ("PROPFIND", "/.well-known/caldav") => Answer::redirect("/caldav/"),
        ("PROPFIND", "/caldav/")
            if String::from_utf8_lossy(&request.body).contains("calendar-home-set") =>
        {
            multistatus(&ok(
                "/caldav/",
                "<c:calendar-home-set><d:href>/caldav/me/</d:href></c:calendar-home-set>",
            ))
        }
        ("PROPFIND", "/caldav/") => {
            multistatus(&ok("/caldav/", "<d:displayname>Zoho</d:displayname>"))
        }
        _ => Answer::xml(404, ""),
    });
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let home = smol::block_on(dav.home()).unwrap();
    assert_eq!(home, Some(format!("{origin}/caldav/me/")));
}
