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
