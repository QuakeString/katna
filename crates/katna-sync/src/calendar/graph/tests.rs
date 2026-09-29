// SPDX-License-Identifier: GPL-3.0-or-later

//! The Graph calendar client against a fake Microsoft on the loopback.

use std::sync::Mutex;

use katna_core::{OAuthProvider, Paths};
use katna_store::Mode;

use super::*;
use crate::calendar::fake::{self, Answer};
use crate::oauth::Provider;

const ACCOUNT: AccountId = AccountId(2);
const ME: &str = "me@outlook.test";

fn client(base: &str) -> GraphCalendar {
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

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    (dir, store)
}

fn at(text: &str, zone: &str) -> i64 {
    text.parse::<DateTime>()
        .unwrap()
        .to_zoned(TimeZone::get(zone).unwrap())
        .unwrap()
        .timestamp()
        .as_second()
}

/// The second Tuesday of each month, 10:00 in Berlin (08:00 UTC in
/// October), six times; October's moved to 14:00, November's cancelled.
const SERIES: &str = r#"{
  "@odata.etag": "W/\"s1\"", "id": "series-1", "iCalUId": "040000008200E00074C5B7101A82E008",
  "subject": "Review", "type": "seriesMaster", "isAllDay": false, "isCancelled": false,
  "showAs": "busy", "originalStartTimeZone": "W. Europe Standard Time",
  "start": {"dateTime": "2026-09-08T08:00:00.0000000", "timeZone": "UTC"},
  "end": {"dateTime": "2026-09-08T09:00:00.0000000", "timeZone": "UTC"},
  "recurrence": {
    "pattern": {"type": "relativeMonthly", "interval": 1, "month": 0, "dayOfMonth": 0,
                "daysOfWeek": ["tuesday"], "firstDayOfWeek": "sunday", "index": "second"},
    "range": {"type": "numbered", "startDate": "2026-09-08", "endDate": "0001-01-01",
              "recurrenceTimeZone": "W. Europe Standard Time", "numberOfOccurrences": 6}
  },
  "location": {"displayName": "Room 4"},
  "body": {"contentType": "text", "content": "Agenda\r\n"},
  "organizer": {"emailAddress": {"name": "Me", "address": "me@outlook.test"}},
  "isOrganizer": true,
  "attendees": [{"type": "optional", "status": {"response": "tentativelyAccepted", "time": "x"},
                 "emailAddress": {"name": "Ana", "address": "ana@test"}}],
  "onlineMeeting": {"joinUrl": "https://teams.test/l/meetup-join/1"},
  "isReminderOn": true, "reminderMinutesBeforeStart": 15,
  "webLink": "https://outlook.test/owa/?itemid=series-1",
  "lastModifiedDateTime": "2026-09-01T10:00:00Z",
  "cancelledOccurrences": ["OID.series-1.2026-11-10"],
  "exceptionOccurrences": [{
    "@odata.etag": "W/\"x1\"", "id": "exception-1", "iCalUId": "different",
    "subject": "Review (afternoon)", "type": "exception", "isAllDay": false,
    "originalStart": "2026-10-13T08:00:00Z", "originalStartTimeZone": "W. Europe Standard Time",
    "start": {"dateTime": "2026-10-13T12:00:00.0000000", "timeZone": "UTC"},
    "end": {"dateTime": "2026-10-13T13:00:00.0000000", "timeZone": "UTC"},
    "showAs": "busy", "responseStatus": {"response": "organizer"}, "isOrganizer": true,
    "onlineMeeting": null, "recurrence": null
  }]
}"#;

const HOLIDAY: &str = r#"{
  "@odata.etag": "W/\"h1\"", "id": "holiday", "iCalUId": "holiday-uid", "subject": "Away",
  "type": "singleInstance", "isAllDay": true, "showAs": "oof",
  "start": {"dateTime": "2026-10-02T00:00:00.0000000", "timeZone": "UTC"},
  "end": {"dateTime": "2026-10-05T00:00:00.0000000", "timeZone": "UTC"},
  "originalStartTimeZone": "India Standard Time",
  "responseStatus": {"response": "none"}, "onlineMeeting": null, "recurrence": null,
  "attendees": null
}"#;

const FREE: &str = r#"{
  "@odata.etag": "W/\"f1\"", "id": "free", "subject": "Lunch", "type": "singleInstance",
  "isAllDay": false, "showAs": "free",
  "start": {"dateTime": "2026-10-01T07:00:00.0000000", "timeZone": "UTC"},
  "end": {"dateTime": "2026-10-01T08:00:00.0000000", "timeZone": "UTC"},
  "originalStartTimeZone": "tzone://Microsoft/Utc",
  "responseStatus": {"response": "notResponded"}
}"#;

fn fake_microsoft() -> (String, Arc<Mutex<Vec<fake::Seen>>>) {
    let base = Arc::new(Mutex::new(String::new()));
    let shared = base.clone();
    let (url, seen) = fake::serve(move |request| {
        let base = shared.lock().unwrap().clone();
        if request.route() == "/token" {
            let form = request.text();
            assert!(form.contains("Calendars.ReadWrite"), "{form}");
            return Answer::json(
                200,
                r#"{"access_token":"graph-1","expires_in":3600,"refresh_token":"rt-2"}"#,
            );
        }
        assert_eq!(request.header("Authorization"), Some("Bearer graph-1"));
        assert!(
            request
                .header("Prefer")
                .is_some_and(|p| p.contains(r#"outlook.timezone="UTC""#))
        );
        let expand = request.query("$expand").is_some();
        match (request.route(), request.query("$skiptoken")) {
            ("/me/calendars", None) => Answer::json(
                200,
                format!(
                    r##"{{"value":[{{"id":"cal-1","name":"Calendar","color":"auto","hexColor":"#0078D4",
                    "isDefaultCalendar":true,"canEdit":true,"owner":{{"name":"Me","address":"{ME}"}}}}],
                    "@odata.nextLink":"{base}/me/calendars?$top=100&$skiptoken=2"}}"##
                ),
            ),
            ("/me/calendars", Some("2")) => Answer::json(
                200,
                r#"{"value":[{"id":"cal-2","name":"Birthdays","color":"lightGreen","hexColor":"",
                    "isDefaultCalendar":false,"canEdit":false,"owner":{"name":"Me","address":"me@outlook.test"}}]}"#,
            ),
            ("/me/calendars/cal-1/events", None) => {
                assert!(expand);
                Answer::json(
                    200,
                    format!(
                        r#"{{"value":[{SERIES},{HOLIDAY}],
                        "@odata.nextLink":"{base}/me/calendars/cal-1/events?$top=250&$expand=exceptionOccurrences&$skiptoken=2"}}"#
                    ),
                )
            }
            ("/me/calendars/cal-1/events", Some("2")) => {
                Answer::json(200, format!(r#"{{"value":[{FREE}]}}"#))
            }
            ("/me/calendars/cal-2/events", None) if expand => Answer::json(
                400,
                r#"{"error":{"code":"BadRequest","message":"Could not find a property named 'exceptionOccurrences'."}}"#,
            ),
            ("/me/calendars/cal-2/events", None) => Answer::json(200, r#"{"value":[]}"#),
            _ => Answer::json(404, "{}"),
        }
    });
    *base.lock().unwrap() = url.clone();
    (url, seen)
}

#[test]
fn relative_monthly_pattern_becomes_an_rrule() {
    let recurrence: Recurrence = serde_json::from_str(
        r#"{"pattern":{"type":"relativeMonthly","interval":2,"daysOfWeek":["monday","friday"],
            "index":"last"},"range":{"type":"endDate","endDate":"2027-06-30"}}"#,
    )
    .unwrap();
    assert_eq!(
        rrule(&recurrence).unwrap(),
        "FREQ=MONTHLY;BYDAY=MO,FR;BYSETPOS=-1;INTERVAL=2;UNTIL=20270630"
    );
    let yearly: Recurrence = serde_json::from_str(
        r#"{"pattern":{"type":"absoluteYearly","interval":1,"month":3,"dayOfMonth":14},
            "range":{"type":"noEnd"}}"#,
    )
    .unwrap();
    assert_eq!(
        rrule(&yearly).unwrap(),
        "FREQ=YEARLY;BYMONTH=3;BYMONTHDAY=14"
    );
    let weekly: Recurrence = serde_json::from_str(
        r#"{"pattern":{"type":"weekly","interval":1,"daysOfWeek":["tuesday","thursday"],
            "firstDayOfWeek":"sunday"},"range":{"type":"numbered","numberOfOccurrences":10}}"#,
    )
    .unwrap();
    assert_eq!(
        rrule(&weekly).unwrap(),
        "FREQ=WEEKLY;BYDAY=TU,TH;COUNT=10;WKST=SU"
    );
}

#[test]
fn syncs_series_whole_days_and_pages() {
    let (base, seen) = fake_microsoft();
    let (_dir, mut store) = store();
    let graph = client(&base);
    assert!(smol::block_on(graph.sync(&mut store, ACCOUNT, ME)).unwrap());

    let calendars = store.calendars().unwrap();
    assert_eq!(calendars.len(), 2, "both pages of calendars");
    let main = &calendars[0];
    assert_eq!(main.name, "Calendar");
    assert_eq!(main.color, "#0078d4");
    assert_eq!(main.access, CalendarAccess::Owner);
    assert_eq!(calendars[1].color, "#93c47d");
    assert_eq!(calendars[1].access, CalendarAccess::Reader);

    let mut rows: Vec<EventData> = store
        .event_rows_in_range(0, i64::MAX / 2)
        .unwrap()
        .into_iter()
        .map(|r| r.data)
        .collect();
    rows.sort_by(|a, b| a.remote_id.cmp(&b.remote_id));
    let ids: Vec<&str> = rows.iter().map(|r| r.remote_id.as_str()).collect();
    assert_eq!(ids, ["exception-1", "free", "holiday", "series-1"]);

    let series = &rows[3];
    assert_eq!(series.rrule, "FREQ=MONTHLY;BYDAY=2TU;COUNT=6");
    assert_eq!(series.time_zone, "Europe/Berlin");
    assert_eq!(series.start, at("2026-09-08T10:00", "Europe/Berlin"));
    assert_eq!(series.exdates, [at("2026-11-10T10:00", "Europe/Berlin")]);
    // Six: September to February, the last on 9 February at 10:00-11:00.
    assert_eq!(
        series.range_end,
        Some(at("2027-02-09T11:00", "Europe/Berlin"))
    );
    assert_eq!(series.location, "Room 4");
    assert_eq!(series.description, "Agenda");
    assert_eq!(series.self_status, "accepted");
    assert_eq!(series.join_url, "https://teams.test/l/meetup-join/1");
    assert_eq!(series.reminders, [15]);
    assert!(series.attendees[0].optional);
    assert_eq!(series.attendees[0].status, "tentative");

    let exception = &rows[0];
    assert_eq!(exception.uid, series.uid, "the series' UID");
    assert_eq!(
        exception.recurrence_id,
        Some(at("2026-10-13T10:00", "Europe/Berlin"))
    );
    assert_eq!(exception.start, at("2026-10-13T14:00", "Europe/Berlin"));

    let holiday = &rows[2];
    assert!(holiday.all_day);
    assert_eq!(holiday.start, at("2026-10-02T00:00", "UTC"));
    assert_eq!(holiday.end, at("2026-10-05T00:00", "UTC"));
    assert_eq!(holiday.kind, EventKind::OutOfOffice);

    let free = &rows[1];
    assert!(!free.busy);
    assert_eq!(free.self_status, "needs_action");
    assert_eq!(free.time_zone, "UTC");

    // The series as the clock shows it: October's moved, November's gone.
    let berlin = TimeZone::get("Europe/Berlin").unwrap();
    let from = at("2026-09-01T00:00", "Europe/Berlin");
    let to = at("2026-12-31T00:00", "Europe/Berlin");
    let starts: Vec<i64> = katna_dav::occurrences(
        store.event_rows_in_range(from, to).unwrap(),
        from,
        to,
        &berlin,
    )
    .into_iter()
    .filter(|o| o.event.data.uid == series.uid)
    .map(|o| o.start)
    .collect();
    assert_eq!(
        starts,
        [
            at("2026-09-08T10:00", "Europe/Berlin"),
            at("2026-10-13T14:00", "Europe/Berlin"),
            at("2026-12-08T10:00", "Europe/Berlin"),
        ]
    );

    // The same again changes nothing, and asks no more expansions of the
    // calendar that refused them.
    seen.lock().unwrap().clear();
    assert!(!smol::block_on(graph.sync(&mut store, ACCOUNT, ME)).unwrap());
    assert!(
        seen.lock()
            .unwrap()
            .iter()
            .filter(|s| s.route() == "/me/calendars/cal-2/events")
            .all(|s| s.query("$expand").is_none())
    );
}
