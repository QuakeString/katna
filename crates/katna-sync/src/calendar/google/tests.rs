// SPDX-License-Identifier: GPL-3.0-or-later

//! The Google Calendar client against a fake Google on the loopback.

use std::sync::Mutex;

use katna_core::{OAuthProvider, Paths};
use katna_store::Mode;

use super::*;
use crate::calendar::fake::{self, Answer, Seen};
use crate::oauth::Provider;

const ACCOUNT: AccountId = AccountId(1);

fn provider() -> Provider {
    Provider {
        kind: OAuthProvider::Google,
        auth_url: "https://accounts.test/auth".into(),
        token_url: "http://127.0.0.1:1/token".into(),
        client_id: "katna-test".into(),
        client_secret: "not-secret".into(),
        scope: format!("https://mail.test/ {GOOGLE_CALENDAR}"),
        consent: String::new(),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    }
}

fn client(api: &str, scope: &str) -> GoogleCalendar {
    let tokens = TokenSource::new(provider(), "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600))
        .with_scope(Some(scope.to_owned()));
    GoogleCalendar::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
}

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    (dir, store)
}

fn at(text: &str, zone: &str) -> i64 {
    text.parse::<jiff::civil::DateTime>()
        .unwrap()
        .to_zoned(TimeZone::get(zone).unwrap())
        .unwrap()
        .timestamp()
        .as_second()
}

const CALENDARS: &str = r##"{
  "items": [
    {"id": "me@test", "summary": "me@test", "summaryOverride": "Mine",
     "backgroundColor": "#9FE1E7", "accessRole": "owner", "primary": true,
     "timeZone": "Asia/Kolkata", "defaultReminders": [{"method": "popup", "minutes": 10}]},
    {"id": "en.indian#holiday@group.v.calendar.google.com", "summary": "Holidays in India",
     "backgroundColor": "#16a765", "accessRole": "reader", "timeZone": "Asia/Kolkata"}
  ]
}"##;

/// The first page of the full listing of `me@test`.
const PAGE_1: &str = r#"{
  "items": [
    {"id": "standup", "etag": "\"1\"", "status": "confirmed", "iCalUID": "standup@google.com",
     "summary": "Standup", "htmlLink": "https://calendar.test/standup",
     "start": {"dateTime": "2026-09-28T09:00:00+05:30", "timeZone": "Asia/Kolkata"},
     "end": {"dateTime": "2026-09-28T09:15:00+05:30", "timeZone": "Asia/Kolkata"},
     "recurrence": ["RRULE:FREQ=WEEKLY;BYDAY=MO;COUNT=4",
                    "EXDATE;TZID=Asia/Kolkata:20261005T090000"],
     "hangoutLink": "https://meet.google.com/abc-defg-hij",
     "attendees": [{"email": "me@test", "self": true, "responseStatus": "needsAction"},
                   {"email": "boss@test", "organizer": true, "responseStatus": "accepted",
                    "displayName": "Boss"}],
     "organizer": {"email": "boss@test", "displayName": "Boss"},
     "reminders": {"useDefault": true},
     "updated": "2026-09-01T10:00:00.000Z"},
    {"id": "standup_20261012T033000Z", "etag": "\"2\"", "status": "confirmed",
     "iCalUID": "standup@google.com", "recurringEventId": "standup",
     "originalStartTime": {"dateTime": "2026-10-12T09:00:00+05:30", "timeZone": "Asia/Kolkata"},
     "summary": "Standup (late)",
     "start": {"dateTime": "2026-10-12T11:00:00+05:30"},
     "end": {"dateTime": "2026-10-12T11:15:00+05:30"},
     "colorId": "7", "reminders": {"useDefault": false, "overrides": [{"method": "popup", "minutes": 5}]}}
  ],
  "nextPageToken": "p2"
}"#;

const PAGE_2: &str = r#"{
  "items": [
    {"id": "standup_20261019T033000Z", "etag": "\"3\"", "status": "cancelled",
     "recurringEventId": "standup",
     "originalStartTime": {"dateTime": "2026-10-19T09:00:00+05:30", "timeZone": "Asia/Kolkata"}},
    {"id": "trip", "etag": "\"4\"", "status": "confirmed", "iCalUID": "trip@google.com",
     "summary": "Goa", "transparency": "transparent", "eventType": "outOfOffice",
     "start": {"date": "2026-10-02"}, "end": {"date": "2026-10-05"}},
    {"id": "gone", "status": "cancelled"},
    {"id": "focus", "etag": "\"5\"", "status": "tentative", "iCalUID": "focus@google.com",
     "summary": "Deep work", "eventType": "focusTime",
     "start": {"dateTime": "2026-09-30T04:30:00Z"}, "end": {"dateTime": "2026-09-30T06:30:00Z"},
     "conferenceData": {"entryPoints": [{"entryPointType": "phone", "uri": "tel:+1"},
                                         {"entryPointType": "video", "uri": "https://zoom.test/j/1"}]}}
  ],
  "nextSyncToken": "s1"
}"#;

/// What changed after `s1`: the trip moved, the focus time went.
const CHANGES: &str = r#"{
  "items": [
    {"id": "trip", "etag": "\"6\"", "status": "confirmed", "iCalUID": "trip@google.com",
     "summary": "Goa (longer)", "start": {"date": "2026-10-02"}, "end": {"date": "2026-10-06"}},
    {"id": "focus", "status": "cancelled"}
  ],
  "nextSyncToken": "s2"
}"#;

#[derive(Default)]
struct Google {
    /// Answer 403 accessNotConfigured.
    disabled: bool,
}

fn fake_google(google: Google) -> (String, Arc<Mutex<Vec<Seen>>>) {
    fake::serve(move |request| {
        assert_eq!(request.header("Authorization"), Some("Bearer at-1"));
        if google.disabled {
            return Answer::json(
                403,
                r#"{"error":{"code":403,"message":"Google Calendar API has not been used in project 1 before or it is disabled.","errors":[{"reason":"accessNotConfigured"}],"status":"PERMISSION_DENIED"}}"#,
            );
        }
        match request.route() {
            "/calendar/v3/users/me/calendarList" => Answer::json(200, CALENDARS),
            "/calendar/v3/calendars/me%40test/events" => {
                assert_eq!(request.query("singleEvents"), Some("false"));
                assert_eq!(request.query("showDeleted"), Some("true"));
                match (request.query("syncToken"), request.query("pageToken")) {
                    (None, None) => Answer::json(200, PAGE_1),
                    (None, Some("p2")) => Answer::json(200, PAGE_2),
                    (Some("s1"), None) => Answer::json(200, CHANGES),
                    (Some("s2"), None) => Answer::json(200, r#"{"items":[],"nextSyncToken":"s2"}"#),
                    (Some(_), _) => Answer::json(
                        410,
                        r#"{"error":{"code":410,"message":"Sync token is no longer valid, a full sync is required.","errors":[{"reason":"fullSyncRequired"}]}}"#,
                    ),
                    _ => Answer::json(404, "{}"),
                }
            }
            "/calendar/v3/calendars/en.indian%23holiday%40group.v.calendar.google.com/events"
                if request.query("syncToken") == Some("h1") =>
            {
                Answer::json(200, r#"{"items":[],"nextSyncToken":"h1"}"#)
            }
            "/calendar/v3/calendars/en.indian%23holiday%40group.v.calendar.google.com/events" => {
                Answer::json(
                    200,
                    r#"{"items":[{"id":"diwali","etag":"\"h1\"","status":"confirmed","summary":"Diwali","start":{"date":"2026-11-08"},"end":{"date":"2026-11-09"}}],"nextSyncToken":"h1"}"#,
                )
            }
            _ => Answer::json(404, "{}"),
        }
    })
}

fn rows(store: &Store, calendar: i64) -> Vec<EventData> {
    let mut rows: Vec<EventData> = store
        .event_rows_in_range(0, i64::MAX / 2)
        .unwrap()
        .into_iter()
        .filter(|r| r.calendar_id == calendar)
        .map(|r| r.data)
        .collect();
    rows.sort_by(|a, b| a.remote_id.cmp(&b.remote_id));
    rows
}

fn mine(store: &Store) -> i64 {
    store
        .calendars()
        .unwrap()
        .into_iter()
        .find(|c| c.remote_id == "me@test")
        .unwrap()
        .id
}

#[test]
fn a_full_sync_brings_calendars_series_exceptions_and_whole_days() {
    let (api, _) = fake_google(Google::default());
    let (_dir, mut store) = store();
    let google = client(&api, &format!("https://mail.test/ {GOOGLE_CALENDAR}"));
    assert!(smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap());

    let calendars = store.calendars().unwrap();
    assert_eq!(calendars.len(), 2);
    let primary = &calendars[0];
    assert_eq!(primary.name, "Mine");
    assert_eq!(primary.color, "#9fe1e7");
    assert!(primary.is_primary);
    assert_eq!(primary.access, CalendarAccess::Owner);
    assert_eq!(primary.sync_token.as_deref(), Some("s1"));
    assert_eq!(calendars[1].access, CalendarAccess::Reader);

    let events = rows(&store, primary.id);
    let ids: Vec<&str> = events.iter().map(|e| e.remote_id.as_str()).collect();
    assert_eq!(
        ids,
        [
            "focus",
            "standup",
            "standup_20261012T033000Z",
            "standup_20261019T033000Z",
            "trip"
        ],
        "a deleted event is not kept"
    );
    let standup = &events[1];
    let first = at("2026-09-28T09:00", "Asia/Kolkata");
    assert_eq!(standup.start, first);
    assert_eq!(standup.end, first + 900);
    assert_eq!(standup.uid, "standup@google.com");
    assert_eq!(standup.rrule, "FREQ=WEEKLY;BYDAY=MO;COUNT=4");
    assert_eq!(standup.exdates, [first + 7 * 86_400]);
    assert_eq!(standup.time_zone, "Asia/Kolkata");
    assert_eq!(standup.range_end, Some(first + 21 * 86_400 + 900));
    assert_eq!(standup.join_url, "https://meet.google.com/abc-defg-hij");
    assert_eq!(standup.self_status, "needs_action");
    assert_eq!(standup.organizer_name, "Boss");
    assert_eq!(standup.reminders, [10], "the calendar's default");
    assert_eq!(standup.web_link, "https://calendar.test/standup");
    assert_eq!(standup.etag.as_deref(), Some("\"1\""));

    let late = &events[2];
    assert_eq!(late.recurrence_id, Some(first + 14 * 86_400));
    assert_eq!(late.uid, "standup@google.com");
    assert_eq!(late.color, "#039be5");
    assert_eq!(late.reminders, [5]);

    let cancelled = &events[3];
    assert_eq!(cancelled.status, EventStatus::Cancelled);
    assert_eq!(cancelled.uid, "standup@google.com", "its series' UID");
    assert_eq!(cancelled.recurrence_id, Some(first + 21 * 86_400));

    let trip = &events[4];
    assert!(trip.all_day);
    assert_eq!(trip.start, at("2026-10-02T00:00", "UTC"));
    assert_eq!(trip.end, at("2026-10-05T00:00", "UTC"));
    assert!(!trip.busy);
    assert_eq!(trip.kind, EventKind::OutOfOffice);

    let focus = &events[0];
    assert_eq!(focus.kind, EventKind::Focus);
    assert_eq!(focus.status, EventStatus::Tentative);
    assert_eq!(focus.join_url, "https://zoom.test/j/1");

    // The series: 28 Sept, (5 Oct skipped), 12 Oct moved, 19 Oct cancelled.
    let kolkata = TimeZone::get("Asia/Kolkata").unwrap();
    let from = at("2026-09-27T00:00", "Asia/Kolkata");
    let to = at("2026-10-31T00:00", "Asia/Kolkata");
    let shown: Vec<(String, i64)> = katna_dav::occurrences(
        store.event_rows_in_range(from, to).unwrap(),
        from,
        to,
        &kolkata,
    )
    .into_iter()
    .filter(|o| o.event.data.uid == "standup@google.com")
    .map(|o| (o.event.data.title.clone(), o.start))
    .collect();
    assert_eq!(
        shown,
        [
            ("Standup".to_owned(), first),
            (
                "Standup (late)".to_owned(),
                at("2026-10-12T11:00", "Asia/Kolkata")
            ),
        ]
    );
}

#[test]
fn an_incremental_sync_applies_changes_and_deletions() {
    let (api, seen) = fake_google(Google::default());
    let (_dir, mut store) = store();
    let google = client(&api, &format!("https://mail.test/ {GOOGLE_CALENDAR}"));
    smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap();
    seen.lock().unwrap().clear();

    assert!(smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap());
    let seen = seen.lock().unwrap();
    assert!(
        seen.iter()
            .any(|s| s.query("syncToken") == Some("s1") && s.route().contains("me%40test"))
    );
    let id = mine(&store);
    assert_eq!(
        store.calendar(id).unwrap().unwrap().sync_token.as_deref(),
        Some("s2")
    );
    let events = rows(&store, id);
    assert!(events.iter().all(|e| e.remote_id != "focus"));
    let trip = events.iter().find(|e| e.remote_id == "trip").unwrap();
    assert_eq!(trip.title, "Goa (longer)");
    assert_eq!(trip.end, at("2026-10-06T00:00", "UTC"));
    // The series is untouched.
    assert_eq!(events.len(), 4);
    drop(seen);

    // Nothing new: nothing changes.
    assert!(!smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap());
}

#[test]
fn an_expired_sync_token_starts_over() {
    let (api, _) = fake_google(Google::default());
    let (_dir, mut store) = store();
    let google = client(&api, &format!("https://mail.test/ {GOOGLE_CALENDAR}"));
    smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap();
    let id = mine(&store);
    store.set_calendar_sync_token(id, Some("expired")).unwrap();
    // A row Google no longer has goes with the full listing.
    store
        .replace_events(
            id,
            "stale",
            &[EventData {
                remote_id: "stale".into(),
                uid: "stale".into(),
                etag: Some("x".into()),
                start: 1,
                end: 2,
                range_end: Some(2),
                ..EventData::default()
            }],
        )
        .unwrap();
    assert!(smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap());
    assert_eq!(
        store.calendar(id).unwrap().unwrap().sync_token.as_deref(),
        Some("s1")
    );
    let events = rows(&store, id);
    assert!(events.iter().all(|e| e.remote_id != "stale"));
    assert_eq!(events.len(), 5);
}

#[test]
fn a_sign_in_without_calendar_asks_to_sign_in_again() {
    let (api, seen) = fake_google(Google::default());
    let (_dir, mut store) = store();
    let google = client(&api, "https://mail.test/ openid");
    let err = smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap_err();
    assert!(matches!(err, CalendarError::NeedsSignIn(_)), "{err}");
    assert!(seen.lock().unwrap().is_empty(), "Google is not asked");
}

#[test]
fn a_switched_off_api_says_so() {
    let (api, _) = fake_google(Google { disabled: true });
    let (_dir, mut store) = store();
    let google = client(&api, &format!("https://mail.test/ {GOOGLE_CALENDAR}"));
    let err = smol::block_on(google.sync(&mut store, ACCOUNT)).unwrap_err();
    assert!(matches!(err, CalendarError::NotEnabled(_)), "{err}");
}
