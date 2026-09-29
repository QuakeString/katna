// SPDX-License-Identifier: GPL-3.0-or-later

//! Changes made in Katna: in the store, and sent to a fake Google,
//! Microsoft and CalDAV server on the loopback. Never a real one.

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU32, Ordering},
};
use std::time::Duration;

use katna_core::{AccountId, OAuthProvider, Paths};
use katna_store::{
    Mode,
    calendar::{CalendarAccess, EventKind, NewCalendar},
};
use serde_json::Value;

use super::*;
use crate::calendar::{
    Item, apply_full,
    fake::{self, Answer, Seen},
};
use crate::net::Tls;
use crate::oauth::{GOOGLE_CALENDAR, MICROSOFT_CALENDARS, Provider, TokenSource};

const WEEK: i64 = 7 * DAY;
const RULE: &str = "FREQ=WEEKLY;BYDAY=MO";

fn at(text: &str) -> i64 {
    text.parse::<jiff::civil::DateTime>()
        .unwrap()
        .to_zoned(TimeZone::get("Asia/Kolkata").unwrap())
        .unwrap()
        .timestamp()
        .as_second()
}

/// Monday 5 October 2026, 9:00 in Kolkata.
fn start() -> i64 {
    at("2026-10-05T09:00")
}

fn now() -> i64 {
    at("2026-10-01T12:00")
}

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    (dir, store)
}

fn calendar(
    store: &mut Store,
    source: CalendarSource,
    account: Option<i64>,
    remote_id: &str,
    access: CalendarAccess,
) -> i64 {
    store
        .upsert_calendar(
            account.map(AccountId),
            source,
            &NewCalendar {
                remote_id: remote_id.into(),
                name: remote_id.into(),
                color: String::new(),
                access,
                is_primary: false,
                time_zone: "Asia/Kolkata".into(),
            },
            0,
        )
        .unwrap()
}

fn edit(title: &str, start: i64, end: i64, rrule: &str) -> EventEdit {
    EventEdit {
        title: title.into(),
        start,
        end,
        time_zone: "Asia/Kolkata".into(),
        rrule: rrule.into(),
        busy: true,
        reminders: vec![10],
        ..EventEdit::default()
    }
}

fn standup() -> EventEdit {
    edit("Standup", start(), start() + 900, RULE)
}

fn add(store: &mut Store, calendar: i64, edit: EventEdit) -> Applied {
    apply(store, &EventChange::Add { calendar, edit }, now()).unwrap()
}

fn change(
    store: &mut Store,
    event: i64,
    scope: EditScope,
    occurrence: Option<i64>,
    edit: EventEdit,
) -> Applied {
    let change = EventChange::Change {
        event,
        scope,
        occurrence,
        edit,
        calendar: None,
    };
    apply(store, &change, now()).unwrap()
}

fn delete(store: &mut Store, event: i64, scope: EditScope, occurrence: Option<i64>) -> Applied {
    let change = EventChange::Delete {
        event,
        scope,
        occurrence,
    };
    apply(store, &change, now()).unwrap()
}

fn restore(store: &mut Store, event: i64, occurrence: i64) -> Applied {
    apply(store, &EventChange::Restore { event, occurrence }, now()).unwrap()
}

fn data(store: &Store, id: i64) -> EventData {
    store.event(id).unwrap().unwrap().data
}

#[test]
fn a_single_event_on_this_computer() {
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Local,
        None,
        "local",
        CalendarAccess::Owner,
    );
    let added = add(
        &mut store,
        cal,
        edit("Dentist", start(), start() + 3600, ""),
    );
    assert!(added.steps.is_empty() && added.rows.is_empty());
    let event = data(&store, added.id);
    assert!(event.uid.ends_with("@katna"), "{}", event.uid);
    assert_eq!(event.title, "Dentist");
    assert_eq!(event.range_end, Some(start() + 3600));
    assert_eq!(event.updated_at, now());
    assert_eq!(store.event_pending(added.id).unwrap(), Some(Pending::None));

    let moved = change(
        &mut store,
        added.id,
        EditScope::This,
        None,
        edit("Dentist", start() + 3600, start() + 7200, ""),
    );
    assert_eq!(moved.id, added.id);
    assert_eq!(data(&store, added.id).start, start() + 3600);

    assert_eq!(delete(&mut store, added.id, EditScope::This, None).id, 0);
    assert!(store.event(added.id).unwrap().is_none());
}

#[test]
fn one_occurrence_of_a_series() {
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Local,
        None,
        "local",
        CalendarAccess::Owner,
    );
    let series = add(&mut store, cal, standup()).id;
    let second = start() + WEEK;
    let late = change(
        &mut store,
        series,
        EditScope::This,
        Some(second),
        edit("Standup (late)", second + 3600, second + 4500, RULE),
    );
    assert_ne!(late.id, series);
    let exception = data(&store, late.id);
    assert_eq!(exception.recurrence_id, Some(second));
    assert_eq!(exception.uid, data(&store, series).uid);
    assert!(exception.rrule.is_empty());
    assert_eq!(exception.start, second + 3600);
    assert_eq!(exception.range_end, Some(second + 4500));

    // The same occurrence again, through the series or itself.
    let again = change(
        &mut store,
        series,
        EditScope::This,
        Some(second),
        edit("Standup (later)", second + 7200, second + 8100, RULE),
    );
    assert_eq!(again.id, late.id);
    let itself = change(
        &mut store,
        late.id,
        EditScope::This,
        Some(second + 7200),
        edit("Standup (latest)", second + 7200, second + 8100, RULE),
    );
    assert_eq!(itself.id, late.id);
    assert_eq!(data(&store, late.id).title, "Standup (latest)");

    // Deleting a changed occurrence leaves it, cancelled; Undo brings it back.
    delete(&mut store, late.id, EditScope::This, Some(second + 7200));
    assert_eq!(data(&store, late.id).status, EventStatus::Cancelled);
    assert_eq!(restore(&mut store, late.id, second).id, series);
    assert_eq!(data(&store, late.id).status, EventStatus::Confirmed);

    // Deleting another skips it.
    let third = start() + 2 * WEEK;
    delete(&mut store, series, EditScope::This, Some(third));
    assert_eq!(data(&store, series).exdates, [third]);
    restore(&mut store, series, third);
    assert!(data(&store, series).exdates.is_empty());
}

#[test]
fn the_whole_series_and_the_following_occurrences() {
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Local,
        None,
        "local",
        CalendarAccess::Owner,
    );
    let series = add(&mut store, cal, standup()).id;
    let second = start() + WEEK;
    let exception = change(
        &mut store,
        series,
        EditScope::This,
        Some(second),
        edit("Moved", second + 600, second + 1500, RULE),
    )
    .id;

    // An hour later, half an hour long, from the third occurrence.
    let third = start() + 2 * WEEK;
    let all = change(
        &mut store,
        series,
        EditScope::All,
        Some(third),
        edit("Standup", third + 3600, third + 5400, RULE),
    );
    assert_eq!(all.id, series);
    let moved = data(&store, series);
    assert_eq!(moved.start, start() + 3600);
    assert_eq!(moved.end, start() + 5400);
    assert!(store.event(exception).unwrap().is_none());

    // From the fourth on, a new series; the old one ends the day before.
    let fourth = start() + 3 * WEEK + 3600;
    let following = change(
        &mut store,
        series,
        EditScope::Following,
        Some(fourth),
        edit("New standup", fourth, fourth + 900, RULE),
    );
    assert_ne!(following.id, series);
    let old = data(&store, series);
    assert_eq!(old.rrule, "FREQ=WEEKLY;BYDAY=MO;UNTIL=20261025T182959Z");
    assert_eq!(old.range_end, Some(start() + 2 * WEEK + 5400));
    let new = data(&store, following.id);
    assert_ne!(new.uid, old.uid);
    assert_eq!((new.start, new.rrule.as_str()), (fourth, RULE));

    // "Following" from the first occurrence is the whole series.
    let first = change(
        &mut store,
        following.id,
        EditScope::Following,
        Some(fourth),
        edit("Renamed", fourth, fourth + 900, RULE),
    );
    assert_eq!(first.id, following.id);
    assert_eq!(data(&store, following.id).title, "Renamed");

    delete(
        &mut store,
        following.id,
        EditScope::Following,
        Some(fourth + WEEK),
    );
    assert!(
        data(&store, following.id)
            .rrule
            .ends_with(";UNTIL=20261101T182959Z")
    );
    delete(&mut store, following.id, EditScope::All, None);
    assert!(store.event(following.id).unwrap().is_none());
}

#[test]
fn read_only_calendars_and_bad_edits_are_refused() {
    let (_dir, mut store) = store();
    let holidays = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "holidays",
        CalendarAccess::Reader,
    );
    let err = apply(
        &mut store,
        &EventChange::Add {
            calendar: holidays,
            edit: standup(),
        },
        now(),
    )
    .unwrap_err();
    assert!(matches!(err, EditError::Invalid(_)));
    assert!(err.to_string().contains("read-only"), "{err}");
    store
        .replace_events(
            holidays,
            "diwali",
            &[EventData {
                remote_id: "diwali".into(),
                uid: "diwali".into(),
                start: start(),
                end: start() + DAY,
                ..EventData::default()
            }],
        )
        .unwrap();
    let id = store.event_rows_in_range(0, i64::MAX / 2).unwrap()[0].id;
    let refused = EventChange::Delete {
        event: id,
        scope: EditScope::This,
        occurrence: None,
    };
    assert!(matches!(
        apply(&mut store, &refused, now()),
        Err(EditError::Invalid(_))
    ));
    let mine = calendar(
        &mut store,
        CalendarSource::Local,
        None,
        "local",
        CalendarAccess::Owner,
    );
    let backwards = EventChange::Add {
        calendar: mine,
        edit: edit("Back", start(), start() - 1, ""),
    };
    assert!(apply(&mut store, &backwards, now()).is_err());
    let nonsense = EventChange::Add {
        calendar: mine,
        edit: edit("Odd", start(), start() + 1, "FREQ=SOMETIMES"),
    };
    assert!(apply(&mut store, &nonsense, now()).is_err());
    let missing = EventChange::Restore {
        event: 999,
        occurrence: 0,
    };
    assert!(apply(&mut store, &missing, now()).is_err());
}

/// A series as Google synced it.
fn synced_google(store: &mut Store, cal: i64) -> i64 {
    let mut series = EventData {
        remote_id: "standup".into(),
        uid: "standup@google.com".into(),
        etag: Some("\"1\"".into()),
        title: "Standup".into(),
        start: start(),
        end: start() + 900,
        time_zone: "Asia/Kolkata".into(),
        rrule: RULE.into(),
        busy: true,
        ..EventData::default()
    };
    series.range_end = ical::range_end(&series);
    store.replace_events(cal, "standup", &[series]).unwrap();
    store.resource_events(cal, "standup").unwrap()[0].0.id
}

#[test]
fn changes_for_a_service_wait_for_it_through_syncs() {
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "me@test",
        CalendarAccess::Owner,
    );
    let series = synced_google(&mut store, cal);
    let synced = data(&store, series);
    let renamed = change(
        &mut store,
        series,
        EditScope::All,
        None,
        edit("Daily sync", start(), start() + 900, RULE),
    );
    assert_eq!(
        renamed.steps,
        [Step::Write {
            calendar: cal,
            row: series,
            add_call: false
        }]
    );
    assert_eq!(renamed.rows, [series]);
    let added = add(&mut store, cal, edit("Lunch", start(), start() + 3600, ""));
    let lunch = data(&store, added.id);
    assert_eq!(lunch.remote_id.len(), 32);
    assert!(lunch.remote_id.bytes().all(|b| b.is_ascii_hexdigit()));
    assert_eq!(lunch.etag, None);

    // Google doesn't have either yet: a full listing leaves both.
    let items: Vec<Item> = vec![("standup".into(), Some("\"2\"".into()), vec![synced])];
    apply_full(&mut store, cal, items.clone()).unwrap();
    assert_eq!(data(&store, series).title, "Daily sync");
    assert!(store.event(added.id).unwrap().is_some());

    // Google refused them: the next sync brings back what it has.
    let rows: Vec<i64> = renamed.rows.iter().chain(&added.rows).copied().collect();
    assert_eq!(store.forget_pending_events(Some(&rows)).unwrap(), [cal]);
    apply_full(&mut store, cal, items).unwrap();
    let rows = store.event_rows_in_range(0, i64::MAX / 2).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].data.title, "Standup");
}

#[test]
fn moving_to_another_calendar() {
    let (_dir, mut store) = store();
    let work = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "work",
        CalendarAccess::Owner,
    );
    let home = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "home",
        CalendarAccess::Writer,
    );
    let dav = calendar(
        &mut store,
        CalendarSource::CalDav,
        Some(2),
        "/cal/me/",
        CalendarAccess::Owner,
    );
    let series = synced_google(&mut store, work);
    let to = |calendar: i64, scope: EditScope, occurrence: Option<i64>| EventChange::Change {
        event: series,
        scope,
        occurrence,
        edit: standup(),
        calendar: Some(calendar),
    };
    let moved = apply(&mut store, &to(home, EditScope::All, None), now()).unwrap();
    assert_eq!(
        moved.steps,
        [Step::GoogleMove {
            calendar: work,
            row: series,
            to: home
        }]
    );
    assert_eq!(store.event(series).unwrap().unwrap().calendar_id, home);
    assert_eq!(data(&store, series).remote_id, "standup");

    let moved = apply(&mut store, &to(dav, EditScope::All, None), now()).unwrap();
    let href = data(&store, series).remote_id;
    assert!(
        href.starts_with("/cal/me/") && href.ends_with(".ics"),
        "{href}"
    );
    assert_eq!(
        moved.steps,
        [
            Step::DeleteRemote {
                calendar: home,
                remote_id: "standup".into(),
                etag: Some("\"1\"".into())
            },
            Step::Write {
                calendar: dav,
                row: series,
                add_call: false
            }
        ]
    );
    let one = to(work, EditScope::This, Some(start() + WEEK));
    assert!(matches!(
        apply(&mut store, &one, now()),
        Err(EditError::Invalid(_))
    ));
}

/// Sends every step of `applied` to `remote`, as the daemon does.
fn push_all(remote: &Remote<'_>, store: &mut Store, applied: &Applied) {
    for step in &applied.steps {
        let cal = store.calendar(step.calendar()).unwrap().unwrap();
        smol::block_on(push(remote, store, &cal, step)).unwrap();
    }
    store.events_pushed(&applied.rows).unwrap();
}

fn json(request: &Seen) -> Value {
    serde_json::from_slice(&request.body).unwrap()
}

/// Takes what the fake server saw since the last call.
fn taken(seen: &Arc<Mutex<Vec<Seen>>>) -> Vec<Seen> {
    std::mem::take(&mut *seen.lock().unwrap())
        .into_iter()
        .filter(|s| s.route() != "/token")
        .collect()
}

fn google(api: &str) -> GoogleCalendar {
    let provider = Provider {
        kind: OAuthProvider::Google,
        auth_url: "https://accounts.test/auth".into(),
        token_url: "http://127.0.0.1:1/token".into(),
        client_id: "katna-test".into(),
        client_secret: "not-secret".into(),
        scope: format!("https://mail.test/ {GOOGLE_CALENDAR}"),
        consent: String::new(),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    };
    let tokens = TokenSource::new(provider, "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600));
    GoogleCalendar::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
}

/// A fake Google Calendar that takes every change.
fn fake_google() -> (String, Arc<Mutex<Vec<Seen>>>) {
    fake::serve(|request| {
        assert_eq!(request.header("Authorization"), Some("Bearer at-1"));
        let last = request.route().rsplit('/').next().unwrap_or_default();
        match request.method.as_str() {
            "POST" if last == "events" => {
                let id = json(request)["id"].as_str().unwrap().to_owned();
                Answer::json(200, format!(r#"{{"id":"{id}","etag":"\"new\""}}"#))
            }
            "PATCH" => Answer::json(200, format!(r#"{{"id":"{last}","etag":"\"patched\""}}"#)),
            "DELETE" => Answer::json(204, ""),
            _ => Answer::json(404, r#"{"error":{"message":"no"}}"#),
        }
    })
}

#[test]
fn google_focus_time_falls_back_to_a_plain_event_when_refused() {
    let (api, seen) = fake::serve(|request| {
        let body = json(request);
        if body["eventType"] == "focusTime" {
            return Answer::json(
                400,
                r#"{"error":{"code":400,"message":"Focus time events cannot be created on this calendar."}}"#,
            );
        }
        let id = body["id"].as_str().unwrap().to_owned();
        Answer::json(200, format!(r#"{{"id":"{id}","etag":"\"new\""}}"#))
    });
    let client = google(&api);
    let remote = Remote::Google(&client);
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "me@test",
        CalendarAccess::Owner,
    );
    let mut focus = edit("Focus time", start(), start() + 7200, "");
    focus.kind = EventKind::Focus;
    let added = add(&mut store, cal, focus);
    push_all(&remote, &mut store, &added);
    let sent = taken(&seen);
    assert_eq!(sent.len(), 2);
    let typed = json(&sent[0]);
    assert_eq!(typed["eventType"], "focusTime");
    assert_eq!(typed["focusTimeProperties"]["chatStatus"], "doNotDisturb");
    let plain = json(&sent[1]);
    assert!(plain.get("eventType").is_none());
    assert_eq!(plain["summary"], "Focus time");
    let saved = data(&store, added.id);
    assert_eq!(saved.etag.as_deref(), Some("\"new\""));
    assert_eq!(saved.kind, EventKind::Focus);
}

#[test]
fn google_gets_each_change() {
    let (api, seen) = fake_google();
    let client = google(&api);
    let remote = Remote::Google(&client);
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "me@test",
        CalendarAccess::Owner,
    );
    let events = "/calendar/v3/calendars/me%40test/events";

    let mut new = standup();
    new.busy = false;
    let added = add(&mut store, cal, new);
    push_all(&remote, &mut store, &added);
    let sent = taken(&seen);
    assert_eq!(sent.len(), 1);
    assert_eq!((sent[0].method.as_str(), sent[0].route()), ("POST", events));
    let body = json(&sent[0]);
    let series = data(&store, added.id);
    assert_eq!(body["id"], series.remote_id.as_str());
    assert_eq!(body["iCalUID"], series.uid.as_str());
    assert_eq!(body["summary"], "Standup");
    assert_eq!(
        body["recurrence"],
        serde_json::json!(["RRULE:FREQ=WEEKLY;BYDAY=MO"])
    );
    assert_eq!(
        body["start"],
        serde_json::json!({"dateTime": "2026-10-05T09:00:00+05:30", "timeZone": "Asia/Kolkata"})
    );
    assert_eq!(body["transparency"], "transparent");
    assert_eq!(
        body["reminders"],
        serde_json::json!({"useDefault": false, "overrides": [{"method": "popup", "minutes": 10}]})
    );
    assert_eq!(series.etag.as_deref(), Some("\"new\""));
    assert_eq!(store.event_pending(added.id).unwrap(), Some(Pending::None));
    let id = series.remote_id.clone();

    // One occurrence: Google's instance of it.
    let second = start() + WEEK;
    let one = change(
        &mut store,
        added.id,
        EditScope::This,
        Some(second),
        edit("Late", second + 3600, second + 4500, RULE),
    );
    push_all(&remote, &mut store, &one);
    let sent = taken(&seen);
    let instance = format!("{events}/{id}_20261012T033000Z");
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("PATCH", instance.as_str())
    );
    assert!(json(&sent[0]).get("recurrence").is_none());
    assert_eq!(
        data(&store, one.id).remote_id,
        format!("{id}_20261012T033000Z")
    );

    // The whole series.
    let all = change(
        &mut store,
        added.id,
        EditScope::All,
        Some(start()),
        edit("Standup", start(), start() + 1800, RULE),
    );
    push_all(&remote, &mut store, &all);
    let sent = taken(&seen);
    let series_url = format!("{events}/{id}");
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("PATCH", series_url.as_str())
    );
    assert_eq!(
        json(&sent[0])["end"]["dateTime"],
        "2026-10-05T09:30:00+05:30"
    );

    // Deleting one occurrence cancels Google's instance; Undo brings it
    // back.
    let third = start() + 2 * WEEK;
    let gone = delete(&mut store, added.id, EditScope::This, Some(third));
    push_all(&remote, &mut store, &gone);
    let sent = taken(&seen);
    let third_url = format!("{events}/{id}_20261019T033000Z");
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("DELETE", third_url.as_str())
    );
    let back = restore(&mut store, added.id, third);
    push_all(&remote, &mut store, &back);
    let sent = taken(&seen);
    assert_eq!(sent.len(), 2);
    assert_eq!(sent[0].route(), series_url);
    assert_eq!(json(&sent[0])["recurrence"].as_array().unwrap().len(), 1);
    assert_eq!(
        (sent[1].method.as_str(), sent[1].route()),
        ("PATCH", third_url.as_str())
    );
    assert_eq!(json(&sent[1]), serde_json::json!({"status": "confirmed"}));

    // From the fourth on: the series ends, a new one starts.
    let fourth = start() + 3 * WEEK;
    let following = change(
        &mut store,
        added.id,
        EditScope::Following,
        Some(fourth),
        edit("New", fourth, fourth + 900, RULE),
    );
    push_all(&remote, &mut store, &following);
    let sent = taken(&seen);
    assert_eq!(sent.len(), 2);
    assert_eq!(
        json(&sent[0])["recurrence"],
        serde_json::json!(["RRULE:FREQ=WEEKLY;BYDAY=MO;UNTIL=20261025T182959Z"])
    );
    assert_eq!((sent[1].method.as_str(), sent[1].route()), ("POST", events));
    let new_id = data(&store, following.id).remote_id;

    // Deleting a series.
    let gone = delete(&mut store, following.id, EditScope::All, None);
    push_all(&remote, &mut store, &gone);
    let sent = taken(&seen);
    let new_url = format!("{events}/{new_id}");
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("DELETE", new_url.as_str())
    );
    assert!(store.event(following.id).unwrap().is_none());
}

#[test]
fn google_refusing_a_change_is_an_error() {
    let (api, _seen) = fake::serve(|_| {
        Answer::json(
            403,
            r#"{"error":{"message":"Forbidden","errors":[{"reason":"forbidden"}]}}"#,
        )
    });
    let client = google(&api);
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "me@test",
        CalendarAccess::Owner,
    );
    let added = add(&mut store, cal, standup());
    let calendar = store.calendar(cal).unwrap().unwrap();
    let err = smol::block_on(push(
        &Remote::Google(&client),
        &mut store,
        &calendar,
        &added.steps[0],
    ))
    .unwrap_err();
    assert!(err.to_string().contains("Forbidden"), "{err}");
}

fn graph(base: &str) -> GraphCalendar {
    let provider = Provider {
        kind: OAuthProvider::Microsoft,
        auth_url: "https://login.test/authorize".into(),
        token_url: format!("{base}/token"),
        client_id: "katna-test".into(),
        client_secret: String::new(),
        scope: "https://outlook.test/IMAP.AccessAsUser.All offline_access".into(),
        consent: MICROSOFT_CALENDARS.into(),
        redirect_host: "localhost",
        tls: Tls::insecure_for_local_tests(),
    };
    let tokens = TokenSource::new(provider, "rt-1".into(), None);
    GraphCalendar::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), base)
}

#[test]
fn outlook_gets_each_change() {
    let (base, seen) = fake::serve(|request| {
        if request.route() == "/token" {
            return Answer::json(
                200,
                r#"{"access_token":"graph-1","expires_in":3600,"refresh_token":"rt-2"}"#,
            );
        }
        assert_eq!(request.header("Authorization"), Some("Bearer graph-1"));
        let route = request.route();
        match request.method.as_str() {
            "POST" => Answer::json(201, r#"{"id":"G1","@odata.etag":"W/\"1\""}"#),
            "GET" if route == "/me/events/G1/instances" => Answer::json(
                200,
                r#"{"value":[
                    {"id":"G1-2","originalStart":"2026-10-12T03:30:00Z"},
                    {"id":"G1-3","originalStart":"2026-10-19T03:30:00Z"}]}"#,
            ),
            "PATCH" => {
                let id = route.rsplit('/').next().unwrap();
                Answer::json(200, format!(r#"{{"id":"{id}","@odata.etag":"W/\"2\""}}"#))
            }
            "DELETE" => Answer::json(204, ""),
            _ => Answer::json(404, r#"{"error":{"code":"x","message":"no"}}"#),
        }
    });
    let client = graph(&base);
    let remote = Remote::Microsoft(&client);
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Microsoft,
        Some(2),
        "AQMk",
        CalendarAccess::Owner,
    );

    let mut new = standup();
    new.reminders = vec![15];
    let added = add(&mut store, cal, new);
    assert!(data(&store, added.id).remote_id.starts_with(NEW_PREFIX));
    push_all(&remote, &mut store, &added);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("POST", "/me/calendars/AQMk/events")
    );
    let body = json(&sent[0]);
    assert_eq!(body["subject"], "Standup");
    assert_eq!(
        body["start"],
        serde_json::json!({"dateTime": "2026-10-05T09:00:00", "timeZone": "India Standard Time"})
    );
    assert_eq!(body["showAs"], "busy");
    assert_eq!(body["isReminderOn"], true);
    assert_eq!(body["reminderMinutesBeforeStart"], 15);
    assert_eq!(
        body["recurrence"],
        serde_json::json!({
            "pattern": {"type": "weekly", "interval": 1, "daysOfWeek": ["monday"],
                        "firstDayOfWeek": "monday"},
            "range": {"type": "noEnd", "startDate": "2026-10-05",
                      "recurrenceTimeZone": "India Standard Time"}
        })
    );
    let series = data(&store, added.id);
    assert_eq!(series.remote_id, "G1");
    assert_eq!(series.etag.as_deref(), Some("W/\"1\""));

    // One occurrence, found among the series' instances.
    let second = start() + WEEK;
    let one = change(
        &mut store,
        added.id,
        EditScope::This,
        Some(second),
        edit("Late", second + 3600, second + 4500, RULE),
    );
    push_all(&remote, &mut store, &one);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("GET", "/me/events/G1/instances")
    );
    assert_eq!(
        (sent[1].method.as_str(), sent[1].route()),
        ("PATCH", "/me/events/G1-2")
    );
    assert!(json(&sent[1]).get("recurrence").is_none());
    assert_eq!(data(&store, one.id).remote_id, "G1-2");

    // The whole series, then one occurrence deleted.
    let all = change(
        &mut store,
        added.id,
        EditScope::All,
        None,
        edit("Standup", start(), start() + 900, "FREQ=MONTHLY;BYDAY=-1MO"),
    );
    push_all(&remote, &mut store, &all);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("PATCH", "/me/events/G1")
    );
    assert_eq!(
        json(&sent[0])["recurrence"]["pattern"],
        serde_json::json!({"type": "relativeMonthly", "interval": 1,
                           "daysOfWeek": ["monday"], "index": "last"})
    );
    let third = start() + 2 * WEEK;
    let gone = delete(&mut store, added.id, EditScope::This, Some(third));
    push_all(&remote, &mut store, &gone);
    let sent = taken(&seen);
    assert_eq!(
        (sent[1].method.as_str(), sent[1].route()),
        ("DELETE", "/me/events/G1-3")
    );

    // Outlook can't bring an occurrence back.
    let back = restore(&mut store, added.id, third);
    let calendar = store.calendar(cal).unwrap().unwrap();
    assert!(smol::block_on(push(&remote, &mut store, &calendar, &back.steps[0])).is_err());
    store.forget_pending_events(Some(&back.rows)).unwrap();

    let gone = delete(&mut store, added.id, EditScope::All, None);
    push_all(&remote, &mut store, &gone);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("DELETE", "/me/events/G1")
    );
    assert!(store.event(added.id).unwrap().is_none());
}

#[test]
fn outlook_repeats_what_the_editor_makes() {
    let series = |rule: &str| {
        let mut event = EventData {
            start: start(),
            end: start() + 900,
            time_zone: "Asia/Kolkata".into(),
            rrule: rule.into(),
            ..EventData::default()
        };
        event.range_end = ical::range_end(&event);
        crate::calendar::graph::write::recurrence(&event)
    };
    let pattern = |rule: &str| series(rule).unwrap()["pattern"].clone();
    assert_eq!(
        pattern("FREQ=DAILY;INTERVAL=2"),
        serde_json::json!({"type": "daily", "interval": 2})
    );
    assert_eq!(
        pattern("FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR")["daysOfWeek"],
        serde_json::json!(["monday", "tuesday", "wednesday", "thursday", "friday"])
    );
    assert_eq!(
        pattern("FREQ=MONTHLY;BYDAY=1MO"),
        serde_json::json!({"type": "relativeMonthly", "interval": 1,
                           "daysOfWeek": ["monday"], "index": "first"})
    );
    assert_eq!(
        pattern("FREQ=MONTHLY"),
        serde_json::json!({"type": "absoluteMonthly", "interval": 1, "dayOfMonth": 5})
    );
    assert_eq!(
        pattern("FREQ=YEARLY"),
        serde_json::json!({"type": "absoluteYearly", "interval": 1, "month": 10,
                           "dayOfMonth": 5})
    );
    let until = series("FREQ=WEEKLY;BYDAY=MO;UNTIL=20261025T182959Z").unwrap();
    assert_eq!(until["range"]["type"], "endDate");
    assert_eq!(until["range"]["endDate"], "2026-10-25");
    let count = series("FREQ=DAILY;COUNT=3").unwrap();
    assert_eq!(count["range"]["numberOfOccurrences"], 3);
    assert!(series("FREQ=HOURLY").is_err());
    assert!(series("FREQ=MONTHLY;BYMONTHDAY=-1").is_err());
    assert!(series("FREQ=MONTHLY;BYDAY=5MO").is_err());
}

#[test]
fn a_caldav_server_gets_each_change() {
    let count = Arc::new(AtomicU32::new(0));
    let (origin, seen) = fake::serve(move |request| match request.method.as_str() {
        "PUT" => {
            let n = count.fetch_add(1, Ordering::SeqCst) + 1;
            let etag = format!("\"e{n}\"");
            let status = if n == 1 { 201 } else { 204 };
            Answer::stored(status, Some(&etag))
        }
        "DELETE" => Answer::stored(204, None),
        _ => Answer::stored(405, None),
    });
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let remote = Remote::CalDav(&dav);
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::CalDav,
        Some(3),
        "/cal/me/",
        CalendarAccess::Owner,
    );

    let added = add(&mut store, cal, standup());
    let href = data(&store, added.id).remote_id;
    assert!(
        href.starts_with("/cal/me/") && href.ends_with(".ics"),
        "{href}"
    );
    push_all(&remote, &mut store, &added);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("PUT", href.as_str())
    );
    assert_eq!(sent[0].header("If-None-Match"), Some("*"));
    assert!(
        sent[0]
            .header("Authorization")
            .unwrap()
            .starts_with("Basic ")
    );
    assert!(
        sent[0]
            .header("Content-Type")
            .unwrap()
            .starts_with("text/calendar")
    );
    let text = sent[0].text();
    assert!(text.contains("RRULE:FREQ=WEEKLY;BYDAY=MO\r\n"), "{text}");
    assert!(text.contains("BEGIN:VTIMEZONE\r\nTZID:Asia/Kolkata\r\n"));
    let back = ical::parse_events(&text, &TimeZone::UTC, "");
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].start, start());
    assert_eq!(back[0].uid, data(&store, added.id).uid);
    assert_eq!(data(&store, added.id).etag.as_deref(), Some("\"e1\""));

    // One occurrence goes into the series' resource.
    let second = start() + WEEK;
    let one = change(
        &mut store,
        added.id,
        EditScope::This,
        Some(second),
        edit("Late", second + 3600, second + 4500, RULE),
    );
    assert_eq!(data(&store, one.id).remote_id, href);
    push_all(&remote, &mut store, &one);
    let sent = taken(&seen);
    assert_eq!(sent[0].header("If-Match"), Some("\"e1\""));
    let text = sent[0].text();
    assert!(text.contains("RECURRENCE-ID;TZID=Asia/Kolkata:20261012T090000\r\n"));
    assert_eq!(ical::parse_events(&text, &TimeZone::UTC, "").len(), 2);
    assert_eq!(data(&store, one.id).etag.as_deref(), Some("\"e2\""));
    assert_eq!(data(&store, added.id).etag.as_deref(), Some("\"e2\""));

    // A deleted occurrence is skipped, and comes back.
    let third = start() + 2 * WEEK;
    let gone = delete(&mut store, added.id, EditScope::This, Some(third));
    push_all(&remote, &mut store, &gone);
    let sent = taken(&seen);
    assert!(
        sent[0]
            .text()
            .contains("EXDATE;TZID=Asia/Kolkata:20261019T090000\r\n")
    );
    let again = restore(&mut store, added.id, third);
    push_all(&remote, &mut store, &again);
    let sent = taken(&seen);
    assert!(!sent[0].text().contains("EXDATE"));

    // "Following" ends this resource's series and adds another.
    let fourth = start() + 3 * WEEK;
    let following = change(
        &mut store,
        added.id,
        EditScope::Following,
        Some(fourth),
        edit("New", fourth, fourth + 900, RULE),
    );
    push_all(&remote, &mut store, &following);
    let sent = taken(&seen);
    assert_eq!(sent.len(), 2);
    assert!(sent[0].text().contains(";UNTIL=20261025T182959Z"));
    assert_ne!(sent[1].route(), href);
    assert_eq!(sent[1].header("If-None-Match"), Some("*"));

    // Deleting the series deletes its resource.
    let etag = data(&store, added.id).etag;
    let gone = delete(&mut store, added.id, EditScope::All, None);
    push_all(&remote, &mut store, &gone);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("DELETE", href.as_str())
    );
    assert_eq!(sent[0].header("If-Match"), etag.as_deref());
    assert!(store.resource_events(cal, &href).unwrap().is_empty());
}

#[test]
fn a_caldav_server_that_changed_meanwhile_refuses() {
    let (origin, _seen) = fake::serve(|_| Answer::stored(412, None));
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::CalDav,
        Some(3),
        "/cal/me/",
        CalendarAccess::Owner,
    );
    let added = add(&mut store, cal, standup());
    let calendar = store.calendar(cal).unwrap().unwrap();
    let err = smol::block_on(push(
        &Remote::CalDav(&dav),
        &mut store,
        &calendar,
        &added.steps[0],
    ))
    .unwrap_err();
    assert!(err.to_string().contains("changed"), "{err}");
}

/// An invitation from `boss@test` to the user, `me@test`.
fn invitation(store: &mut Store, cal: i64, remote_id: &str, rrule: &str) -> i64 {
    use katna_store::calendar::Attendee;
    let mut event = EventData {
        remote_id: remote_id.into(),
        uid: format!("{remote_id}@example.com"),
        etag: Some("\"1\"".into()),
        title: "Review".into(),
        start: start(),
        end: start() + 3600,
        time_zone: "Asia/Kolkata".into(),
        rrule: rrule.into(),
        busy: true,
        organizer: "boss@test".into(),
        attendees: vec![
            Attendee {
                email: "boss@test".into(),
                status: "accepted".into(),
                organizer: true,
                ..Attendee::default()
            },
            Attendee {
                email: "me@test".into(),
                status: "needs_action".into(),
                is_self: true,
                ..Attendee::default()
            },
        ],
        self_status: "needs_action".into(),
        ..EventData::default()
    };
    event.range_end = ical::range_end(&event);
    store.replace_events(cal, remote_id, &[event]).unwrap();
    store.resource_events(cal, remote_id).unwrap()[0].0.id
}

fn respond(
    store: &mut Store,
    event: i64,
    scope: EditScope,
    occurrence: Option<i64>,
    status: &str,
) -> Applied {
    let change = EventChange::Respond {
        event,
        scope,
        occurrence,
        status: status.into(),
    };
    apply(store, &change, now()).unwrap()
}

#[test]
fn answering_an_invitation_here() {
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "me@test",
        CalendarAccess::Owner,
    );
    let single = invitation(&mut store, cal, "review", "");
    let answered = respond(&mut store, single, EditScope::This, None, "accepted");
    assert_eq!(answered.id, single);
    assert_eq!(
        answered.steps,
        [Step::Respond {
            calendar: cal,
            row: single,
            status: "accepted".into()
        }]
    );
    let event = data(&store, single);
    assert_eq!(event.self_status, "accepted");
    assert_eq!(event.attendees[1].status, "accepted");
    assert_eq!(event.attendees[0].status, "accepted");

    // One occurrence of a series: its own changed occurrence.
    let series = invitation(&mut store, cal, "weekly", RULE);
    let second = start() + WEEK;
    let one = respond(
        &mut store,
        series,
        EditScope::This,
        Some(second),
        "declined",
    );
    assert_ne!(one.id, series);
    let exception = data(&store, one.id);
    assert_eq!(exception.recurrence_id, Some(second));
    assert_eq!((exception.start, exception.end), (second, second + 3600));
    assert_eq!(exception.self_status, "declined");
    assert_eq!(exception.remote_id, "weekly_20261012T033000Z");
    assert_eq!(data(&store, series).self_status, "needs_action");
    let all = respond(&mut store, series, EditScope::All, None, "tentative");
    assert_eq!(all.id, series);
    assert_eq!(data(&store, series).self_status, "tentative");

    let odd = EventChange::Respond {
        event: single,
        scope: EditScope::This,
        occurrence: None,
        status: "maybe".into(),
    };
    assert!(apply(&mut store, &odd, now()).is_err());
}

#[test]
fn answers_and_invitations_reach_each_service() {
    // Google: the attendees, with the organizer told.
    let (api, seen) = fake_google();
    let client = google(&api);
    let remote = Remote::Google(&client);
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "me@test",
        CalendarAccess::Owner,
    );
    let series = invitation(&mut store, cal, "weekly", RULE);
    let one = respond(
        &mut store,
        series,
        EditScope::This,
        Some(start() + WEEK),
        "declined",
    );
    push_all(&remote, &mut store, &one);
    let sent = taken(&seen);
    assert_eq!(sent[0].method, "PATCH");
    assert_eq!(
        sent[0].path,
        "/calendar/v3/calendars/me%40test/events/weekly_20261012T033000Z?sendUpdates=all"
    );
    let body = json(&sent[0]);
    assert_eq!(body.as_object().unwrap().len(), 1, "{body}");
    assert_eq!(body["attendees"][1]["email"], "me@test");
    assert_eq!(body["attendees"][1]["responseStatus"], "declined");
    let mut meeting = edit("Planning", start(), start() + 1800, "");
    meeting.attendees = data(&store, series).attendees;
    let invited = add(&mut store, cal, meeting);
    push_all(&remote, &mut store, &invited);
    let sent = taken(&seen);
    assert_eq!(sent[0].method, "POST");
    assert!(
        sent[0].path.ends_with("/events?sendUpdates=all"),
        "{}",
        sent[0].path
    );

    // Outlook: its own accept, which answers the organizer.
    let (base, seen) = fake::serve(|request| {
        if request.route() == "/token" {
            return Answer::json(
                200,
                r#"{"access_token":"graph-1","expires_in":3600,"refresh_token":"rt-2"}"#,
            );
        }
        match request.method.as_str() {
            "GET" => Answer::json(
                200,
                r#"{"value":[{"id":"W-2","originalStart":"2026-10-12T03:30:00Z"}]}"#,
            ),
            _ => Answer::json(202, ""),
        }
    });
    let client = graph(&base);
    let remote = Remote::Microsoft(&client);
    let outlook = calendar(
        &mut store,
        CalendarSource::Microsoft,
        Some(2),
        "AQMk",
        CalendarAccess::Owner,
    );
    let series = invitation(&mut store, outlook, "W", RULE);
    let all = respond(&mut store, series, EditScope::All, None, "accepted");
    push_all(&remote, &mut store, &all);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("POST", "/me/events/W/accept")
    );
    assert_eq!(json(&sent[0]), serde_json::json!({"sendResponse": true}));
    let one = respond(
        &mut store,
        series,
        EditScope::This,
        Some(start() + WEEK),
        "tentative",
    );
    push_all(&remote, &mut store, &one);
    let sent = taken(&seen);
    assert_eq!(
        (sent[1].method.as_str(), sent[1].route()),
        ("POST", "/me/events/W-2/tentativelyAccept")
    );
    assert_eq!(data(&store, one.id).remote_id, "W-2");

    // CalDAV: the user's PARTSTAT, for the server to tell the organizer;
    // an invitation made here has the user as organizer.
    let (origin, seen) = fake::serve(|_| Answer::stored(204, Some("\"e9\"")));
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let remote = Remote::CalDav(&dav);
    let account = store
        .add_account(katna_core::AccountKind::Imap, "Me", "me@dav.test")
        .unwrap();
    let dav_cal = calendar(
        &mut store,
        CalendarSource::CalDav,
        Some(account.id.0),
        "/cal/me/",
        CalendarAccess::Owner,
    );
    let single = invitation(&mut store, dav_cal, "/cal/me/review.ics", "");
    let answered = respond(&mut store, single, EditScope::This, None, "declined");
    push_all(&remote, &mut store, &answered);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("PUT", "/cal/me/review.ics")
    );
    assert!(
        sent[0]
            .text()
            .contains("ATTENDEE;PARTSTAT=DECLINED:mailto:me@test\r\n"),
        "{}",
        sent[0].text()
    );
    let mut meeting = edit("Planning", start(), start() + 1800, "");
    meeting.attendees = vec![katna_store::calendar::Attendee {
        email: "anita@example.com".into(),
        ..katna_store::calendar::Attendee::default()
    }];
    let invited = add(&mut store, dav_cal, meeting);
    assert_eq!(data(&store, invited.id).organizer, "me@dav.test");
    push_all(&remote, &mut store, &invited);
    let sent = taken(&seen);
    let text = sent[0].text();
    assert!(text.contains("ORGANIZER:mailto:me@dav.test\r\n"), "{text}");
    assert!(
        text.contains("ATTENDEE:mailto:anita@example.com\r\n"),
        "{text}"
    );
}

#[test]
fn a_video_call_when_asked() {
    // Google Meet.
    let (api, seen) = fake::serve(|request| {
        let id = json(request)["id"].as_str().unwrap_or("x").to_owned();
        Answer::json(
            200,
            format!(
                r#"{{"id":"{id}","etag":"\"m\"","hangoutLink":"https://meet.google.com/abc-defg-hij"}}"#
            ),
        )
    });
    let client = google(&api);
    let (_dir, mut store) = store();
    let cal = calendar(
        &mut store,
        CalendarSource::Google,
        Some(1),
        "me@test",
        CalendarAccess::Owner,
    );
    let mut meeting = edit("Planning", start(), start() + 1800, "");
    meeting.add_call = true;
    let added = add(&mut store, cal, meeting.clone());
    assert!(matches!(added.steps[0], Step::Write { add_call: true, .. }));
    push_all(&Remote::Google(&client), &mut store, &added);
    let sent = taken(&seen);
    assert!(
        sent[0].path.ends_with("/events?conferenceDataVersion=1"),
        "{}",
        sent[0].path
    );
    let body = json(&sent[0]);
    let request = &body["conferenceData"]["createRequest"];
    assert_eq!(request["conferenceSolutionKey"]["type"], "hangoutsMeet");
    assert_eq!(request["requestId"].as_str().unwrap().len(), 32);
    assert_eq!(
        data(&store, added.id).join_url,
        "https://meet.google.com/abc-defg-hij"
    );
    // It has one now: changing the event asks for none.
    let changed = change(&mut store, added.id, EditScope::This, None, meeting.clone());
    assert!(matches!(
        changed.steps[0],
        Step::Write {
            add_call: false,
            ..
        }
    ));

    // Teams, where the calendar takes it.
    let (base, seen) = fake::serve(|request| {
        if request.route() == "/token" {
            return Answer::json(
                200,
                r#"{"access_token":"graph-1","expires_in":3600,"refresh_token":"rt-2"}"#,
            );
        }
        match request.method.as_str() {
            "GET" => Answer::json(
                200,
                r#"{"allowedOnlineMeetingProviders":["teamsForBusiness"]}"#,
            ),
            _ => Answer::json(
                201,
                r#"{"id":"T1","@odata.etag":"W/\"1\"",
                    "onlineMeeting":{"joinUrl":"https://teams.test/l/meetup-join/1"}}"#,
            ),
        }
    });
    let client = graph(&base);
    let outlook = calendar(
        &mut store,
        CalendarSource::Microsoft,
        Some(2),
        "AQMk",
        CalendarAccess::Owner,
    );
    let added = add(&mut store, outlook, meeting);
    push_all(&Remote::Microsoft(&client), &mut store, &added);
    let sent = taken(&seen);
    assert_eq!(
        (sent[0].method.as_str(), sent[0].route()),
        ("GET", "/me/calendars/AQMk")
    );
    let body = json(&sent[1]);
    assert_eq!(body["isOnlineMeeting"], true);
    assert_eq!(body["onlineMeetingProvider"], "teamsForBusiness");
    assert_eq!(
        data(&store, added.id).join_url,
        "https://teams.test/l/meetup-join/1"
    );
}
