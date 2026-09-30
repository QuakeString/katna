// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars made, renamed, recoloured and removed on fake services on
//! the loopback.

use std::{sync::Arc, time::Duration};

use katna_core::{AccountId, OAuthProvider};
use katna_store::calendar::{Calendar, CalendarAccess, CalendarSource};

use super::*;
use crate::calendar::{
    caldav::CalDav,
    fake::{self, Answer},
    google::GoogleCalendar,
    graph::GraphCalendar,
    zoho::ZohoCalendar,
};
use crate::net::Tls;
use crate::oauth::{MICROSOFT_CALENDARS, Provider, TokenSource};

fn tokens(kind: OAuthProvider) -> Arc<TokenSource> {
    let provider = Provider {
        kind,
        auth_url: "https://accounts.test/auth".into(),
        token_url: "http://127.0.0.1:1/token".into(),
        client_id: "katna-test".into(),
        client_secret: "not-secret".into(),
        scope: String::new(),
        consent: String::new(),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    };
    let hour = Duration::from_secs(3600);
    Arc::new(
        TokenSource::new(provider, "rt".into(), None)
            .with_access_token("at-1".into(), hour)
            .with_access_token_for(MICROSOFT_CALENDARS, "at-1".into(), hour),
    )
}

fn calendar(source: CalendarSource, remote_id: &str, access: CalendarAccess) -> Calendar {
    Calendar {
        id: 1,
        account: Some(AccountId(1)),
        source,
        remote_id: remote_id.into(),
        name: "Work".into(),
        color: "#039be5".into(),
        access,
        is_primary: false,
        hidden: false,
        time_zone: String::new(),
        sync_token: None,
        position: 0,
    }
}

#[test]
fn google_makes_renames_colours_and_removes() {
    let (api, seen) = fake::serve(|request| match (request.method.as_str(), request.route()) {
        ("POST", "/calendar/v3/calendars") => Answer::json(
            200,
            r#"{"id":"new@group.calendar.google.com","timeZone":"Asia/Kolkata"}"#,
        ),
        ("PATCH" | "DELETE", _) => Answer::json(200, "{}"),
        _ => Answer::json(404, "{}"),
    });
    let google = GoogleCalendar::with_api(
        tokens(OAuthProvider::Google),
        Tls::insecure_for_local_tests(),
        &api,
    );
    let remote = Remote::Google(&google);
    let made = smol::block_on(add(&remote, "Gym", "#d50000")).unwrap();
    assert_eq!(made.remote_id, "new@group.calendar.google.com");
    assert_eq!(
        (made.color.as_str(), made.access),
        ("#d50000", CalendarAccess::Owner)
    );

    let own = calendar(CalendarSource::Google, "own@group", CalendarAccess::Owner);
    let shared = calendar(
        CalendarSource::Google,
        "arijit@test",
        CalendarAccess::Reader,
    );
    smol::block_on(rename(&remote, &own, "Family")).unwrap();
    smol::block_on(rename(&remote, &shared, "Arijit")).unwrap();
    smol::block_on(recolor(&remote, &shared, "#33b679")).unwrap();
    smol::block_on(remove(&remote, &shared, false)).unwrap();
    smol::block_on(remove(&remote, &own, true)).unwrap();
    let mut main = own.clone();
    main.is_primary = true;
    assert!(smol::block_on(remove(&remote, &main, true)).is_err());

    let seen = seen.lock().unwrap();
    let calls: Vec<(String, String, String)> = seen
        .iter()
        .map(|s| (s.method.clone(), s.path.clone(), s.text()))
        .collect();
    let list = "/calendar/v3/users/me/calendarList";
    assert_eq!(calls[0].0, "POST");
    assert!(calls[0].2.contains(r#""summary":"Gym""#), "{:?}", calls[0]);
    assert_eq!(
        calls[1].1,
        format!("{list}/new%40group.calendar.google.com?colorRgbFormat=true")
    );
    assert!(calls[1].2.contains("#d50000"));
    assert_eq!(calls[2].1, "/calendar/v3/calendars/own%40group");
    assert!(calls[2].2.contains(r#""summary":"Family""#));
    assert_eq!(calls[3].1, format!("{list}/arijit%40test"));
    assert!(calls[3].2.contains(r#""summaryOverride":"Arijit""#));
    assert!(calls[4].2.contains("#33b679"));
    assert_eq!(
        (calls[5].0.as_str(), calls[5].1.as_str()),
        ("DELETE", format!("{list}/arijit%40test").as_str())
    );
    assert_eq!(
        (calls[6].0.as_str(), calls[6].1.as_str()),
        ("DELETE", "/calendar/v3/calendars/own%40group")
    );
    // The main calendar never went to Google.
    assert_eq!(calls.len(), 7);
}

#[test]
fn outlook_takes_its_nearest_colour() {
    let (api, seen) = fake::serve(|request| match (request.method.as_str(), request.route()) {
        ("POST", "/me/calendars") => Answer::json(
            201,
            r##"{"id":"AAMk1","name":"Gym","color":"lightRed","hexColor":"#e06666"}"##,
        ),
        ("PATCH" | "DELETE", "/me/calendars/AAMk1") => Answer::json(200, "{}"),
        _ => Answer::json(404, "{}"),
    });
    let graph = GraphCalendar::with_api(
        tokens(OAuthProvider::Microsoft),
        Tls::insecure_for_local_tests(),
        &api,
    );
    let remote = Remote::Microsoft(&graph);
    let made = smol::block_on(add(&remote, "Gym", "#d50000")).unwrap();
    assert_eq!(
        (made.remote_id.as_str(), made.color.as_str()),
        ("AAMk1", "#e06666")
    );
    let own = calendar(CalendarSource::Microsoft, "AAMk1", CalendarAccess::Owner);
    let kept = smol::block_on(recolor(&remote, &own, "#039be5")).unwrap();
    assert_eq!(kept, "#a4c2f4");
    smol::block_on(remove(&remote, &own, true)).unwrap();
    let seen = seen.lock().unwrap();
    assert!(seen[0].text().contains(r#""color":"lightRed""#));
    assert!(seen[1].text().contains(r#""color":"lightBlue""#));
    assert_eq!(seen[2].method, "DELETE");
}

#[test]
fn caldav_makes_a_collection_in_the_home() {
    let (origin, seen) = fake::serve(|request| {
        let ok = |href: &str, props: &str| {
            Answer::xml(
                207,
                format!(
                    r#"<?xml version="1.0"?><d:multistatus xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav"><d:response><d:href>{href}</d:href><d:propstat><d:prop>{props}</d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response></d:multistatus>"#
                ),
            )
        };
        match (request.method.as_str(), request.route()) {
            ("PROPFIND", "/.well-known/caldav") => Answer::redirect("/dav/"),
            ("PROPFIND", "/dav/") => ok(
                "/dav/",
                "<d:current-user-principal><d:href>/dav/principals/me/</d:href></d:current-user-principal>",
            ),
            ("PROPFIND", "/dav/principals/me/") => ok(
                "/dav/principals/me/",
                "<c:calendar-home-set><d:href>/dav/calendars/me/</d:href></c:calendar-home-set>",
            ),
            ("MKCALENDAR", route) if route.starts_with("/dav/calendars/me/") => {
                Answer::stored(201, None)
            }
            ("PROPPATCH", "/dav/calendars/me/work/") => {
                ok("/dav/calendars/me/work/", "<d:displayname/>")
            }
            ("DELETE", "/dav/calendars/me/work/") => Answer::stored(204, None),
            _ => Answer::xml(404, ""),
        }
    });
    let dav = CalDav::with_origin(&origin, "me", "secret", Tls::insecure_for_local_tests());
    let remote = Remote::CalDav(&dav);
    let made = smol::block_on(add(&remote, "Gym & more", "#d50000")).unwrap();
    assert!(
        made.remote_id.starts_with("/dav/calendars/me/"),
        "{}",
        made.remote_id
    );
    assert!(made.remote_id.ends_with('/'));
    let work = calendar(
        CalendarSource::CalDav,
        "/dav/calendars/me/work/",
        CalendarAccess::Owner,
    );
    smol::block_on(rename(&remote, &work, "Office")).unwrap();
    smol::block_on(recolor(&remote, &work, "#33b679")).unwrap();
    smol::block_on(remove(&remote, &work, true)).unwrap();
    let seen = seen.lock().unwrap();
    let make = seen.iter().find(|s| s.method == "MKCALENDAR").unwrap();
    assert!(
        make.text()
            .contains("<d:displayname>Gym &amp; more</d:displayname>")
    );
    assert!(make.text().contains("#d50000ff"));
    let patches: Vec<String> = seen
        .iter()
        .filter(|s| s.method == "PROPPATCH")
        .map(|s| s.text())
        .collect();
    assert!(patches[0].contains("<d:displayname>Office</d:displayname>"));
    assert!(patches[1].contains("#33b679ff"));
    assert!(seen.iter().any(|s| s.method == "DELETE"));
}

#[test]
fn zoho_changes_only_ones_own_calendars() {
    let (api, seen) = fake::serve(|request| match (request.method.as_str(), request.route()) {
        ("GET", "/calendars") => Answer::json(
            200,
            r##"{"calendars":[{"uid":"mine","name":"Work","privilege":"owner"},
                {"uid":"nitish","name":"Nitish","privilege":"read"}]}"##,
        ),
        ("POST", "/calendars") => Answer::json(
            200,
            r##"{"calendars":[{"uid":"new1","name":"Gym","color":"#D50000"}]}"##,
        ),
        ("PUT" | "DELETE", "/calendars/mine") => Answer::json(200, "{}"),
        _ => Answer::json(404, "{}"),
    });
    let zoho = ZohoCalendar::with_api(
        tokens(OAuthProvider::Zoho),
        Tls::insecure_for_local_tests(),
        &api,
    );
    let remote = Remote::Zoho(&zoho);
    let made = smol::block_on(add(&remote, "Gym", "#d50000")).unwrap();
    assert_eq!(
        (made.remote_id.as_str(), made.color.as_str()),
        ("new1", "#d50000")
    );
    let mine = calendar(CalendarSource::Zoho, "mine", CalendarAccess::Reader);
    let theirs = calendar(CalendarSource::Zoho, "nitish", CalendarAccess::Reader);
    smol::block_on(rename(&remote, &mine, "Office")).unwrap();
    smol::block_on(remove(&remote, &mine, true)).unwrap();
    let refused = smol::block_on(remove(&remote, &theirs, true)).unwrap_err();
    assert!(refused.to_string().contains("website"), "{refused}");
    let seen = seen.lock().unwrap();
    assert!(seen.iter().all(|s| !s.path.contains("nitish")));
    let put = seen.iter().find(|s| s.method == "PUT").unwrap();
    assert!(put.path.contains("calendarData="), "{}", put.path);
    assert!(
        seen.iter()
            .any(|s| s.method == "DELETE" && s.route() == "/calendars/mine")
    );
}
