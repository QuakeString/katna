// SPDX-License-Identifier: GPL-3.0-or-later

//! The People API and Graph contact readers against fakes on the loopback.

use katna_core::OAuthProvider;

use super::*;
use crate::fake_http::serve;
use crate::oauth::Provider;

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

fn google(api: &str, scope: &str) -> GoogleContacts {
    let tokens = TokenSource::new(provider(OAuthProvider::Google), "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600))
        .with_scope(Some(format!("https://mail.test/ {scope}")));
    GoogleContacts::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
}

const GROUPS: &str = r#"{"contactGroups":[
  {"resourceName":"contactGroups/starred","name":"starred","groupType":"SYSTEM_CONTACT_GROUP"},
  {"resourceName":"contactGroups/abc","name":"Suppliers","formattedName":"Suppliers","groupType":"USER_CONTACT_GROUP"}]}"#;

const PAGE_1: &str = r#"{"connections":[{"resourceName":"people/c1","etag":"e1",
  "names":[{"displayName":"Arjun Mehta","givenName":"Arjun","familyName":"Mehta"}],
  "emailAddresses":[{"value":"arjun@acme.co","type":"work"}],
  "phoneNumbers":[{"value":"+91 99000 55120","type":"mobile"}],
  "addresses":[{"type":"work","streetAddress":"12 MG Road","city":"Pune","country":"India"}],
  "organizations":[{"name":"Acme Traders","title":"Buyer"}],
  "birthdays":[{"date":{"month":3,"day":14}}],
  "photos":[{"url":"https://lh3.test/letter","default":true}],
  "memberships":[{"contactGroupMembership":{"contactGroupResourceName":"contactGroups/starred"}},
                 {"contactGroupMembership":{"contactGroupResourceName":"contactGroups/abc"}}]}],
  "nextPageToken":"p2"}"#;

const PAGE_2: &str = r#"{"connections":[{"resourceName":"people/c2","etag":"e2",
  "names":[{"displayName":"Bo"}],
  "photos":[{"url":"https://lh3.test/bo"}]}],
  "nextSyncToken":"sync-1"}"#;

#[test]
fn google_reads_every_page_then_changes() {
    let (api, seen) = serve(|req, _| {
        let path = req.path.as_str();
        if path.starts_with("/v1/contactGroups") {
            return (200, vec![], GROUPS.into());
        }
        if path.contains("syncToken=sync-1") {
            return (
                200,
                vec![],
                r#"{"connections":[{"resourceName":"people/c2","metadata":{"deleted":true}}],"nextSyncToken":"sync-2"}"#.into(),
            );
        }
        if path.contains("syncToken=old") {
            return (
                410,
                vec![],
                r#"{"error":{"code":410,"status":"FAILED_PRECONDITION"}}"#.into(),
            );
        }
        if path.contains("pageToken=p2") {
            return (200, vec![], PAGE_2.into());
        }
        (200, vec![], PAGE_1.into())
    });
    let google = google(&api, GOOGLE_CONTACTS);
    assert!(smol::block_on(google.allowed()).unwrap());

    let full = smol::block_on(google.sync(None)).unwrap();
    assert!(full.full);
    assert_eq!(full.sync_token.as_deref(), Some("sync-1"));
    assert_eq!(
        full.groups.as_deref().unwrap(),
        [SyncedGroup {
            remote_id: "contactGroups/abc".into(),
            name: "Suppliers".into()
        }]
    );
    assert_eq!(full.contacts.len(), 2);
    let arjun = &full.contacts[0];
    assert_eq!(arjun.remote_id, "people/c1");
    assert_eq!(arjun.etag.as_deref(), Some("e1"));
    assert!(arjun.starred);
    assert_eq!(arjun.card.display_name(), "Arjun Mehta");
    assert_eq!(
        arjun.card.phones[0],
        Typed::new("+91 99000 55120", "mobile")
    );
    assert_eq!(arjun.card.job(), "Buyer, Acme Traders");
    assert_eq!(arjun.card.birthday, "--03-14");
    assert_eq!(arjun.card.photo_url, "", "Google's letter is not a picture");
    assert_eq!(full.contacts[1].card.photo_url, "https://lh3.test/bo");
    let requests = seen.lock().unwrap().clone();
    assert!(
        requests
            .iter()
            .all(|r| r.header("Authorization") == Some("Bearer at-1"))
    );

    let change = smol::block_on(google.sync(Some("sync-1"))).unwrap();
    assert!(!change.full);
    assert_eq!(change.deleted, ["people/c2"]);
    assert_eq!(change.sync_token.as_deref(), Some("sync-2"));

    // A token Google forgot means reading everything again.
    let again = smol::block_on(google.sync(Some("old"))).unwrap();
    assert!(again.full);
    assert_eq!(again.contacts.len(), 2);
}

#[test]
fn google_without_the_scope_asks_to_allow() {
    let (api, _) = serve(|_, _| {
        (
            403,
            vec![],
            r#"{"error":{"code":403,"status":"PERMISSION_DENIED","details":[{"reason":"ACCESS_TOKEN_SCOPE_INSUFFICIENT"}]}}"#.into(),
        )
    });
    let google = google(&api, "https://www.googleapis.com/auth/drive.file");
    assert!(!smol::block_on(google.allowed()).unwrap());
    assert!(matches!(
        smol::block_on(google.sync(None)),
        Err(Error::Auth(_))
    ));
}

#[test]
fn microsoft_reads_pages_and_categories() {
    let (api, _) = serve(|req, base| {
        if req.path.contains("skip=1") {
            return (
                200,
                vec![],
                r#"{"value":[{"id":"m2","displayName":"Chen Wei","emailAddresses":[{"address":"chen@lotus.cn"}]}]}"#.into(),
            );
        }
        (
            200,
            vec![],
            format!(
                r#"{{"value":[{{"id":"m1","changeKey":"k1","displayName":"Bilal Ahmed",
                "givenName":"Bilal","surname":"Ahmed","emailAddresses":[{{"name":"Bilal","address":"bilal@northwind.com"}}],
                "mobilePhone":"+44 7700","businessPhones":["+44 20"],"companyName":"Northwind","jobTitle":"CTO",
                "businessAddress":{{"city":"London","countryOrRegion":"UK"}},"homeAddress":{{}},
                "birthday":"1980-01-02T11:59:00Z","categories":["Partners"]}}],
                "@odata.nextLink":"{base}/me/contacts?skip=1"}}"#
            ),
        )
    });
    let tokens = TokenSource::new(provider(OAuthProvider::Microsoft), "rt".into(), None)
        .with_access_token_for(MICROSOFT_CONTACTS, "gt-1".into(), Duration::from_secs(3600));
    let ms = MicrosoftContacts::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), &api);
    let sync = smol::block_on(ms.sync()).unwrap();
    assert!(sync.full);
    assert_eq!(sync.contacts.len(), 2);
    let bilal = &sync.contacts[0];
    assert_eq!(bilal.card.display_name(), "Bilal Ahmed");
    assert_eq!(bilal.card.phones.len(), 2);
    assert_eq!(bilal.card.phones[0].kind, "mobile");
    assert_eq!(
        bilal.card.addresses.len(),
        1,
        "the empty home address is left out"
    );
    assert_eq!(bilal.card.birthday, "1980-01-02");
    assert_eq!(bilal.groups, ["Partners"]);
    assert_eq!(sync.groups.unwrap()[0].name, "Partners");
}

#[test]
fn google_creates_updates_and_deletes() {
    let (api, seen) = serve(|req, _| {
        match (req.method.as_str(), req.path.as_str()) {
        ("POST", p) if p.starts_with("/v1/people:createContact") => (
            200,
            vec![],
            r#"{"resourceName":"people/c9","etag":"n1","names":[{"displayName":"Asha Rao"}],
               "emailAddresses":[{"value":"asha@rao.in","type":"home"}]}"#
                .into(),
        ),
        ("PATCH", p) if p.starts_with("/v1/people/c9:updateContact") => (
            200,
            vec![],
            r#"{"resourceName":"people/c9","etag":"n2","names":[{"displayName":"Asha R. Rao"}],
               "memberships":[{"contactGroupMembership":{"contactGroupResourceName":"contactGroups/starred"}}]}"#
                .into(),
        ),
        ("DELETE", "/v1/people/c9:deleteContact") => (200, vec![], "{}".into()),
        ("DELETE", _) => (404, vec![], "{}".into()),
        _ => (400, vec![], "{}".into()),
    }
    });
    let google = google(&api, GOOGLE_CONTACTS);
    let mut card = Card {
        name: Name {
            given: "Asha".into(),
            family: "Rao".into(),
            ..Name::default()
        },
        emails: vec![Typed::new("asha@rao.in", "home")],
        birthday: "--03-14".into(),
        ..Card::default()
    };
    let made = smol::block_on(google.save(None, None, &card)).unwrap();
    assert_eq!(made.remote_id, "people/c9");
    assert_eq!(made.etag.as_deref(), Some("n1"));
    card.name.middle = "R.".into();
    let changed = smol::block_on(google.save(Some("people/c9"), Some("n1"), &card)).unwrap();
    assert_eq!(changed.etag.as_deref(), Some("n2"));
    assert!(changed.starred, "the star Google keeps comes back");
    smol::block_on(google.delete("people/c9")).unwrap();
    smol::block_on(google.delete("people/gone")).unwrap();

    let requests = seen.lock().unwrap().clone();
    let create: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(create["names"][0]["givenName"], "Asha");
    assert_eq!(create["emailAddresses"][0]["type"], "home");
    assert_eq!(create["birthdays"][0]["date"]["month"], 3);
    assert!(create["birthdays"][0]["date"].get("year").is_none());
    let update: serde_json::Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(update["etag"], "n1");
    assert!(requests[1].path.contains("updatePersonFields=names,"));
    let fields = requests[1]
        .path
        .split("updatePersonFields=")
        .nth(1)
        .unwrap();
    let fields = fields.split('&').next().unwrap();
    assert!(
        !fields.contains("memberships"),
        "labels stay as Google has them"
    );
}

#[test]
fn microsoft_creates_updates_and_deletes() {
    let (api, seen) = serve(|req, _| match req.method.as_str() {
        "POST" | "PATCH" => (
            201,
            vec![],
            r#"{"id":"m9","changeKey":"k9","displayName":"Asha Rao","categories":["Friends"]}"#
                .into(),
        ),
        _ => (204, vec![], String::new()),
    });
    let tokens = TokenSource::new(provider(OAuthProvider::Microsoft), "rt".into(), None)
        .with_access_token_for(MICROSOFT_CONTACTS, "gt-1".into(), Duration::from_secs(3600));
    let ms = MicrosoftContacts::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), &api);
    let card = Card {
        name: Name {
            given: "Asha".into(),
            family: "Rao".into(),
            ..Name::default()
        },
        phones: vec![
            Typed::new("+91 1", "mobile"),
            Typed::new("+91 2", "work"),
            Typed::new("+91 3", "home"),
        ],
        birthday: "1990-03-14".into(),
        ..Card::default()
    };
    let made = smol::block_on(ms.save(None, &card)).unwrap();
    assert_eq!(made.remote_id, "m9");
    assert_eq!(made.groups, ["Friends"]);
    smol::block_on(ms.save(Some("m9"), &card)).unwrap();
    smol::block_on(ms.delete("m9")).unwrap();
    let requests = seen.lock().unwrap().clone();
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].path, "/me/contacts");
    assert_eq!(requests[1].method, "PATCH");
    assert_eq!(requests[1].path, "/me/contacts/m9");
    assert_eq!(requests[2].method, "DELETE");
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["mobilePhone"], "+91 1");
    assert_eq!(body["businessPhones"][0], "+91 2");
    assert_eq!(body["homePhones"][0], "+91 3");
    assert_eq!(body["birthday"], "1990-03-14T11:59:00Z");
    assert!(body.get("categories").is_none());
}
