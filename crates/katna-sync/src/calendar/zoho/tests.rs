// SPDX-License-Identifier: GPL-3.0-or-later

//! The Zoho Calendar client against a fake Zoho on the loopback.

use std::sync::Mutex;

use katna_core::{OAuthProvider, Paths};
use katna_store::Mode;

use super::*;
use crate::calendar::fake::{self, Answer, Seen};
use crate::oauth::Provider;

const ACCOUNT: AccountId = AccountId(1);

fn client(api: &str) -> ZohoCalendar {
    let provider = Provider {
        kind: OAuthProvider::Zoho,
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
    ZohoCalendar::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
}

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    (dir, store)
}

const CALENDARS: &str = r##"{"calendars": [
  {"uid": "c1", "name": "mozammel", "color": "#8CBF40", "privilege": "owner",
   "timezone": "Asia/Kolkata", "isdefault": true, "ctag": 1678958985948,
   "reminders": [{"minutes": "-15", "action": "popup"}]}
]}"##;

const EVENTS: &str = r##"{"events": [
  {"uid": "a@zoho.com", "title": "Standup", "location": "Office",
   "isallday": false, "etag": 1500358633876,
   "dateandtime": {"timezone": "India Standard Time", "start": "20261001T093000+0530",
                   "end": "20261001T100000+0530"},
   "organizer": "boss@invenia.in",
   "attendees": [{"email": "me@invenia.in", "status": "NEEDS-ACTION"},
                 {"email": "boss@invenia.in", "status": "ACCEPTED"}]},
  {"uid": "b@zoho.com", "title": "Holiday", "isallday": true, "etag": "7",
   "dateandtime": {"timezone": "Asia/Kolkata", "start": "20261002", "end": "20261002"}},
  {"uid": "r@zoho.com", "title": "Gym", "etag": 3, "rrule": "FREQ=DAILY;COUNT=2",
   "dateandtime": {"timezone": "Asia/Kolkata", "start": "20261003T070000+0530",
                   "end": "20261003T080000+0530"},
   "reminders": [{"minutes": "-10", "action": "popup"}]},
  {"uid": "r@zoho.com", "title": "Gym", "etag": 3, "rrule": "FREQ=DAILY;COUNT=2",
   "dateandtime": {"timezone": "Asia/Kolkata", "start": "20261004T070000+0530",
                   "end": "20261004T080000+0530"}}
]}"##;

fn fake_zoho(ctag: Arc<Mutex<u64>>) -> (String, Arc<Mutex<Vec<Seen>>>) {
    fake::serve(move |request| {
        if request.header("Authorization") != Some("Zoho-oauthtoken at-1") {
            return Answer::json(401, r#"{"message":"invalid token"}"#);
        }
        match request.route() {
            "/calendars" => Answer::json(
                200,
                CALENDARS.replace("1678958985948", &ctag.lock().unwrap().to_string()),
            ),
            "/calendars/c1/events" => Answer::json(200, EVENTS),
            _ => Answer::json(404, "{}"),
        }
    })
}

fn at(text: &str) -> i64 {
    text.parse::<jiff::civil::DateTime>()
        .unwrap()
        .to_zoned(TimeZone::get("Asia/Kolkata").unwrap())
        .unwrap()
        .timestamp()
        .as_second()
}

#[test]
fn calendars_and_occurrences_come_in_read_only() {
    let ctag = Arc::new(Mutex::new(1));
    let (api, seen) = fake_zoho(ctag.clone());
    let zoho = client(&api);
    let (_dir, mut store) = store();
    assert!(smol::block_on(zoho.sync(&mut store, ACCOUNT, "me@invenia.in")).unwrap());

    let calendars = store.calendars().unwrap();
    assert_eq!(calendars.len(), 1);
    let cal = &calendars[0];
    assert_eq!(cal.name, "mozammel");
    assert_eq!(cal.color, "#8cbf40");
    assert_eq!(cal.source, CalendarSource::Zoho);
    assert_eq!(cal.access, CalendarAccess::Reader);
    assert!(cal.is_primary);

    // Every range of at most 31 days is asked, with the token.
    let asked: Vec<Seen> = seen.lock().unwrap().clone();
    let ranges: Vec<&Seen> = asked
        .iter()
        .filter(|r| r.route().ends_with("/events"))
        .collect();
    let days = PAST_DAYS + AHEAD_DAYS;
    assert_eq!(ranges.len() as i64, (days + RANGE_DAYS - 1) / RANGE_DAYS);
    assert!(ranges.iter().all(|r| r.query("byinstance") == Some("true")));

    let mut events = store.event_etags(cal.id).unwrap();
    events.sort();
    let ids: Vec<&str> = events.iter().map(|(id, _)| id.as_str()).collect();
    let gym = at("2026-10-03T07:00:00");
    assert_eq!(
        ids,
        [
            "a@zoho.com".to_owned(),
            "b@zoho.com".to_owned(),
            format!("r@zoho.com#{gym}"),
            format!("r@zoho.com#{}", gym + 86_400),
        ]
    );

    // Nothing changed: no events asked again.
    let before = seen.lock().unwrap().len();
    assert!(!smol::block_on(zoho.sync(&mut store, ACCOUNT, "me@invenia.in")).unwrap());
    assert_eq!(seen.lock().unwrap().len(), before + 1);

    // A new ctag: asked again, and nothing to rewrite.
    *ctag.lock().unwrap() = 2;
    assert!(!smol::block_on(zoho.sync(&mut store, ACCOUNT, "me@invenia.in")).unwrap());
    assert!(seen.lock().unwrap().len() > before + 2);
}

#[test]
fn events_read_as_katna_keeps_them() {
    let events: Events = serde_json::from_str(EVENTS).unwrap();
    let zone = TimeZone::get("Asia/Kolkata").unwrap();
    let map = |ix: usize| map_event(&events.events[ix], &zone, &[15], "me@invenia.in").unwrap();

    let standup = map(0);
    assert_eq!(standup.start, at("2026-10-01T09:30:00"));
    assert_eq!(standup.end, at("2026-10-01T10:00:00"));
    assert_eq!(standup.time_zone, "Asia/Kolkata");
    assert_eq!(standup.location, "Office");
    assert_eq!(standup.etag.as_deref(), Some("1500358633876"));
    assert_eq!(standup.self_status, "needs_action");
    assert_eq!(standup.attendees[1].status, "accepted");
    assert!(standup.attendees[1].organizer);
    // The calendar's reminder when the event has none.
    assert_eq!(standup.reminders, [15]);

    let holiday = map(1);
    assert!(holiday.all_day);
    assert_eq!(holiday.end - holiday.start, 86_400);

    let gym = map(2);
    assert_eq!(gym.reminders, [10]);
    assert!(gym.rrule.is_empty());
}

#[test]
fn a_refused_token_asks_to_sign_in_again() {
    let (api, _seen) = fake::serve(|_| Answer::json(403, r#"{"message":"scope mismatch"}"#));
    let zoho = client(&api);
    let (_dir, mut store) = store();
    let err = smol::block_on(zoho.sync(&mut store, ACCOUNT, "me@invenia.in")).unwrap_err();
    assert!(matches!(err, CalendarError::NeedsSignIn(_)), "{err}");
}

#[test]
fn the_calendar_server_sits_beside_the_accounts_server() {
    assert_eq!(
        calendar_server("https://accounts.zoho.in"),
        "https://calendar.zoho.in"
    );
    assert_eq!(
        calendar_server("https://accounts.zohocloud.ca/"),
        "https://calendar.zohocloud.ca"
    );
}
