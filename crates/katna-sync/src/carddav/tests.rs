// SPDX-License-Identifier: GPL-3.0-or-later

//! The CardDAV reader against a fake server on the loopback.

use std::sync::{Arc, Mutex};

use super::*;
use crate::fake_http::serve;

fn card(uid: &str, name: &str, email: &str) -> String {
    format!(
        "BEGIN:VCARD\r\nVERSION:3.0\r\nUID:{uid}\r\nFN:{name}\r\nEMAIL:{email}\r\nCATEGORIES:Friends\r\nEND:VCARD\r\n"
    )
}

fn ms(responses: &str, token: Option<&str>) -> (u16, Vec<(&'static str, String)>, String) {
    let token = token
        .map(|t| format!("<d:sync-token>{t}</d:sync-token>"))
        .unwrap_or_default();
    (
        207,
        vec![("Content-Type", "application/xml".into())],
        format!(
            r#"<?xml version="1.0"?><d:multistatus xmlns:d="DAV:" xmlns:card="urn:ietf:params:xml:ns:carddav">{responses}{token}</d:multistatus>"#
        ),
    )
}

fn ok(href: &str, props: &str) -> String {
    format!(
        "<d:response><d:href>{href}</d:href><d:propstat><d:prop>{props}</d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>"
    )
}

#[test]
fn discovers_through_well_known_and_syncs_changes() {
    let asked_for: Arc<Mutex<Vec<String>>> = Arc::default();
    let log = asked_for.clone();
    let (base, seen) = serve(move |req, base| {
        let body = req.text();
        match (req.method.as_str(), req.path.as_str()) {
            ("PROPFIND", "/.well-known/carddav") => (
                301,
                vec![("Location", format!("{base}/dav/"))],
                String::new(),
            ),
            ("PROPFIND", "/dav/") => ms(
                &ok(
                    "/dav/",
                    "<d:current-user-principal><d:href>/dav/principals/alice/</d:href></d:current-user-principal>",
                ),
                None,
            ),
            ("PROPFIND", "/dav/principals/alice/") => ms(
                &ok(
                    "/dav/principals/alice/",
                    "<card:addressbook-home-set><d:href>/dav/card/alice/</d:href></card:addressbook-home-set>",
                ),
                None,
            ),
            ("PROPFIND", "/dav/card/alice/") => ms(
                &(ok(
                    "/dav/card/alice/",
                    "<d:resourcetype><d:collection/></d:resourcetype>",
                ) + &ok(
                    "/dav/card/alice/default/",
                    "<d:resourcetype><d:collection/><card:addressbook/></d:resourcetype><d:displayname>Contacts</d:displayname>",
                ) + &ok(
                    "/dav/card/alice/cal/",
                    "<d:resourcetype><d:collection/></d:resourcetype>",
                )),
                None,
            ),
            ("REPORT", "/dav/card/alice/default/") if body.contains("sync-collection") => {
                if body.contains("<d:sync-token>t1</d:sync-token>") {
                    ms(
                        &(ok(
                            "/dav/card/alice/default/b.vcf",
                            "<d:getetag>\"b2\"</d:getetag>",
                        ) + "<d:response><d:href>/dav/card/alice/default/a.vcf</d:href><d:status>HTTP/1.1 404 Not Found</d:status></d:response>"),
                        Some("t2"),
                    )
                } else if body.contains("<d:sync-token>gone</d:sync-token>") {
                    (
                        403,
                        vec![],
                        "<d:error xmlns:d=\"DAV:\"><d:valid-sync-token/></d:error>".into(),
                    )
                } else {
                    ms(
                        &(ok("/dav/card/alice/default/", "")
                            + &ok(
                                "/dav/card/alice/default/a.vcf",
                                "<d:getetag>\"a1\"</d:getetag>",
                            )
                            + &ok(
                                "/dav/card/alice/default/b.vcf",
                                "<d:getetag>\"b1\"</d:getetag>",
                            )
                            + &ok(
                                "/dav/card/alice/default/g.vcf",
                                "<d:getetag>\"g1\"</d:getetag>",
                            )),
                        Some("t1"),
                    )
                }
            }
            ("REPORT", "/dav/card/alice/default/") => {
                let mut found = String::new();
                for (href, text) in [
                    ("/dav/card/alice/default/a.vcf", card("a", "Asha Rao", "asha@x.in")),
                    ("/dav/card/alice/default/b.vcf", card("b", "Bo &amp; Co", "bo@x.in")),
                    (
                        "/dav/card/alice/default/g.vcf",
                        "BEGIN:VCARD\r\nVERSION:3.0\r\nUID:g\r\nFN:Team\r\nX-ADDRESSBOOKSERVER-KIND:group\r\nX-ADDRESSBOOKSERVER-MEMBER:urn:uuid:a\r\nEND:VCARD\r\n".into(),
                    ),
                ] {
                    if body.contains(href) {
                        log.lock().unwrap().push(href.to_owned());
                        found += &ok(href, &format!("<d:getetag>\"x\"</d:getetag><card:address-data>{text}</card:address-data>"));
                    }
                }
                ms(&found, None)
            }
            _ => (404, vec![], String::new()),
        }
    });
    let dav = CardDav::new("alice", "pw", Tls::insecure_for_local_tests());
    let books = smol::block_on(dav.discover(&[format!("{base}/.well-known/carddav")])).unwrap();
    assert_eq!(
        books,
        [Collection {
            url: format!("{base}/dav/card/alice/default/"),
            name: "Contacts".into()
        }]
    );
    let url = &books[0].url;
    assert!(
        seen.lock()
            .unwrap()
            .iter()
            .all(|r| r.header("Authorization") == Some("Basic YWxpY2U6cHc="))
    );

    let full = smol::block_on(dav.sync(url, None, &HashMap::new())).unwrap();
    assert!(full.full);
    assert_eq!(full.sync_token.as_deref(), Some("t1"));
    assert_eq!(full.contacts.len(), 2, "the group card is a label");
    let asha = full
        .contacts
        .iter()
        .find(|c| c.card.name.full == "Asha Rao")
        .unwrap();
    assert_eq!(asha.remote_id, "/dav/card/alice/default/a.vcf");
    assert_eq!(asha.groups, ["Friends", "group:g"]);
    assert!(asha.raw.as_deref().unwrap().contains("UID:a"));
    assert_eq!(full.groups.as_ref().unwrap()[0].name, "Team");
    assert!(full.contacts.iter().any(|c| c.card.name.full == "Bo & Co"));

    asked_for.lock().unwrap().clear();
    let change = smol::block_on(dav.sync(url, Some("t1"), &HashMap::new())).unwrap();
    assert!(!change.full);
    assert_eq!(change.deleted, ["/dav/card/alice/default/a.vcf"]);
    assert_eq!(change.contacts.len(), 1);
    assert_eq!(
        *asked_for.lock().unwrap(),
        ["/dav/card/alice/default/b.vcf"]
    );

    // A forgotten token reads the list again; known ETags are not fetched.
    asked_for.lock().unwrap().clear();
    let known: HashMap<String, String> = [
        (
            "/dav/card/alice/default/a.vcf".to_owned(),
            "\"a1\"".to_owned(),
        ),
        (
            "/dav/card/alice/default/g.vcf".to_owned(),
            "\"g1\"".to_owned(),
        ),
    ]
    .into();
    let again = smol::block_on(dav.sync(url, Some("gone"), &known)).unwrap();
    assert!(again.full);
    assert_eq!(again.unchanged.len(), 2);
    assert_eq!(
        *asked_for.lock().unwrap(),
        ["/dav/card/alice/default/b.vcf"]
    );
}

#[test]
fn wrong_password_is_an_auth_error() {
    let (base, _) = serve(|_, _| (401, vec![], String::new()));
    let dav = CardDav::new("alice", "bad", Tls::insecure_for_local_tests());
    let found = smol::block_on(dav.discover(&[format!("{base}/.well-known/carddav")]));
    assert!(matches!(found, Err(Error::Auth(_))));
}

#[test]
fn start_urls_know_providers_and_domains() {
    let urls = start_urls(None, "me@fastmail.com", Some("imap.fastmail.com"));
    assert_eq!(urls[0], "https://carddav.fastmail.com/.well-known/carddav");
    assert!(urls.contains(&"https://fastmail.com/.well-known/carddav".to_owned()));
    let urls = start_urls(
        Some("https://dav.example.org/card/"),
        "me@example.org",
        None,
    );
    assert_eq!(urls[0], "https://dav.example.org/card/");
    assert_eq!(absolute("https://h/a/b/", "c.vcf"), "https://h/a/b/c.vcf");
    assert_eq!(absolute("https://h/a/b/", "/x/"), "https://h/x/");
}
