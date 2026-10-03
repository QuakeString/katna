// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Mutex;

use katna_core::OAuthProvider;
use katna_store::FolderId;

use super::*;
use crate::{fake_http::serve, oauth::Provider};

fn folder(id: i64, path: &str, role: Option<FolderRole>) -> StoredFolder {
    StoredFolder {
        id: FolderId(id),
        path: path.to_owned(),
        role,
        uidvalidity: None,
        highestmodseq: None,
        sync_state: None,
    }
}

fn gmail() -> GmailAccount {
    GmailAccount {
        folders: vec![
            folder(1, "INBOX", Some(FolderRole::Inbox)),
            folder(2, "[Gmail]/All Mail", Some(FolderRole::All)),
            folder(3, "[Gmail]/Trash", Some(FolderRole::Trash)),
            folder(4, "[Gmail]/Spam", Some(FolderRole::Junk)),
            folder(5, "[Gmail]/Sent Mail", Some(FolderRole::Sent)),
            folder(6, "Receipts", None),
            folder(7, "Work/Clients", None),
            folder(8, "Gone", None),
        ],
        labels: [("Receipts", "Label_1"), ("Work/Clients", "Label_2")]
            .into_iter()
            .map(|(n, i)| (n.to_owned(), i.to_owned()))
            .collect(),
        forwarding: vec!["me@example.org".into()],
    }
}

fn condition(field: Field, comparator: Comparator, value: &str) -> Condition {
    Condition {
        field,
        comparator,
        value: value.to_owned(),
    }
}

fn rule(id: i64, conditions: Vec<Condition>, actions: Vec<Action>) -> Rule {
    Rule {
        id,
        name: format!("Rule {id}"),
        conditions,
        actions,
        accounts: vec![1],
        ..Rule::default()
    }
}

fn query(c: Condition) -> Result<String, RunsNote> {
    translate(&rule(1, vec![c], vec![Action::Star]), &gmail(), false)
        .map(|f| f[0].criteria.query.clone())
}

#[test]
fn criteria_are_what_gmail_matches_as_katna_does() {
    use Comparator::*;
    use Field::*;
    let ok = |c: Condition, want: &str| {
        assert_eq!(query(c.clone()).as_deref(), Ok(want), "{c:?}");
    };
    ok(
        condition(From, Contains, " bob@example.org "),
        "from:(bob@example.org)",
    );
    ok(
        condition(From, Contains, "example.org"),
        "from:(example.org)",
    );
    ok(condition(From, EndsWith, "@bank.test"), "from:(bank.test)");
    ok(condition(To, Equals, "me@katna.test"), "to:(me@katna.test)");
    ok(
        condition(Cc, NotContains, "list.example.org"),
        "-cc:(list.example.org)",
    );
    ok(
        condition(AnyRecipient, Contains, "team@example.org"),
        "{to:(team@example.org) cc:(team@example.org) bcc:(team@example.org)}",
    );
    ok(
        condition(Subject, Contains, "Your receipt"),
        "subject:\"Your receipt\"",
    );
    ok(
        condition(Subject, NotContains, "re-sent"),
        "-subject:\"re-sent\"",
    );
    ok(condition(HasAttachment, Contains, ""), "has:attachment");
    ok(condition(HasAttachment, Contains, "no"), "-has:attachment");

    for c in [
        // Gmail matches words, Katna text: "bob" would miss "bobby".
        condition(From, Contains, "bob"),
        condition(From, Contains, "Bob Builder"),
        condition(From, Contains, "bob@"),
        condition(From, Equals, "@example.org"),
        condition(From, EndsWith, "bob@example.org"),
        condition(From, BeginsWith, "bob@"),
        condition(From, Matches, "bob|carol"),
        condition(Subject, Equals, "Hello"),
        condition(Subject, BeginsWith, "Re"),
        condition(Subject, Contains, "50% off!"),
        condition(Subject, Contains, "a\"b"),
        condition(ReplyTo, Contains, "x@y.org"),
        condition(Body, Contains, "unsubscribe"),
        condition(AttachmentName, Contains, "pdf"),
    ] {
        assert_eq!(
            query(c.clone()),
            Err(RunsNote::Condition {
                service: RunsOn::Gmail,
                field: c.field,
                comparator: c.comparator,
            }),
            "{c:?}"
        );
    }
}

#[test]
fn any_of_becomes_a_filter_per_condition() {
    let both = vec![
        condition(Field::From, Comparator::Contains, "a@x.org"),
        condition(Field::Subject, Comparator::Contains, "news"),
    ];
    let all = translate(
        &rule(1, both.clone(), vec![Action::Archive]),
        &gmail(),
        false,
    )
    .unwrap();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].criteria.query, "from:(a@x.org) subject:\"news\"");
    let any = translate(
        &Rule {
            match_mode: MatchMode::Any,
            ..rule(1, both, vec![Action::Archive])
        },
        &gmail(),
        false,
    )
    .unwrap();
    let queries: Vec<&str> = any.iter().map(|f| f.criteria.query.as_str()).collect();
    assert_eq!(queries, ["from:(a@x.org)", "subject:\"news\""]);
    assert!(any.iter().all(|f| f.action.remove_label_ids == ["INBOX"]));
}

#[test]
fn actions_are_labels_on_and_off() {
    let action = |actions: Vec<Action>| {
        translate(
            &rule(
                1,
                vec![condition(Field::From, Comparator::Contains, "a@x.org")],
                actions,
            ),
            &gmail(),
            false,
        )
        .map(|f| f[0].action.clone())
    };
    let ids = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let got = action(vec![
        Action::Move { folder: 7 },
        Action::MarkRead,
        Action::Star,
        Action::MarkImportant,
        Action::AddLabel { folder: 6 },
        Action::Forward {
            to: "Me@Example.org".into(),
        },
    ])
    .unwrap();
    assert_eq!(
        got.add_label_ids,
        ids(&["Label_2", "STARRED", "IMPORTANT", "Label_1"])
    );
    assert_eq!(got.remove_label_ids, ids(&["INBOX", "UNREAD"]));
    assert_eq!(got.forward.as_deref(), Some("Me@Example.org"));
    assert_eq!(
        serde_json::to_value(Filter {
            criteria: Criteria { query: "x".into() },
            action: got
        })
        .unwrap(),
        serde_json::json!({
            "criteria": {"query": "x"},
            "action": {
                "addLabelIds": ["Label_2", "STARRED", "IMPORTANT", "Label_1"],
                "removeLabelIds": ["INBOX", "UNREAD"],
                "forward": "Me@Example.org"
            }
        })
    );

    assert_eq!(
        action(vec![Action::Archive]).unwrap().remove_label_ids,
        ["INBOX"]
    );
    assert_eq!(
        action(vec![Action::Move { folder: 2 }])
            .unwrap()
            .remove_label_ids,
        ["INBOX"]
    );
    assert_eq!(
        action(vec![Action::Trash]).unwrap().add_label_ids,
        ["TRASH"]
    );
    assert_eq!(
        action(vec![Action::Move { folder: 3 }])
            .unwrap()
            .add_label_ids,
        ["TRASH"]
    );
    assert_eq!(
        action(vec![Action::Move { folder: 4 }])
            .unwrap()
            .add_label_ids,
        ["SPAM"]
    );

    let gone = Err(RunsNote::Folder {
        service: RunsOn::Gmail,
    });
    assert_eq!(action(vec![Action::Move { folder: 8 }]), gone);
    assert_eq!(action(vec![Action::AddLabel { folder: 99 }]), gone);
    for cant in [
        Action::Move { folder: 5 },
        Action::DontNotify,
        Action::MarkReadAfter { days: 2 },
    ] {
        assert_eq!(
            action(vec![cant.clone()]),
            Err(RunsNote::Action {
                service: RunsOn::Gmail,
                action: cant
            })
        );
    }
    assert_eq!(
        action(vec![Action::Forward {
            to: "boss@example.org".into()
        }]),
        Err(RunsNote::ForwardAddress {
            to: "boss@example.org".into()
        })
    );

    // Gmail runs every filter that matches: "stop" only fits the last.
    let stop = Rule {
        stop: true,
        ..rule(
            1,
            vec![condition(Field::From, Comparator::Contains, "a@x.org")],
            vec![Action::Star],
        )
    };
    assert!(translate(&stop, &gmail(), false).is_ok());
    assert_eq!(
        translate(&stop, &gmail(), true),
        Err(RunsNote::Stop {
            service: RunsOn::Gmail
        })
    );
}

fn client(api: &str, scope: &str) -> GmailSettings {
    let provider = Provider {
        kind: OAuthProvider::Google,
        auth_url: "https://accounts.test/auth".into(),
        token_url: "http://127.0.0.1:1/token".into(),
        client_id: "katna-test".into(),
        client_secret: "not-secret".into(),
        scope: format!("https://mail.test/ {GOOGLE_GMAIL_SETTINGS}"),
        consent: String::new(),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    };
    let tokens = TokenSource::new(provider, "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600))
        .with_scope(Some(scope.to_owned()));
    GmailSettings::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
}

/// The fake's URL, the requests it saw and the filters it holds.
type Fake = (
    String,
    Arc<Mutex<Vec<crate::fake_http::Seen>>>,
    Arc<Mutex<Vec<String>>>,
);

/// A fake Gmail that keeps filters, refusing one whose query says
/// "refuse".
fn fake_gmail() -> Fake {
    let filters = Arc::new(Mutex::new(vec!["theirs".to_owned()]));
    let kept = filters.clone();
    let next = Arc::new(Mutex::new(0));
    let (api, seen) = serve(move |request, _| {
        let path = request.path.as_str();
        match (request.method.as_str(), path) {
            ("GET", "/gmail/v1/users/me/labels") => (
                200,
                Vec::new(),
                r#"{"labels":[{"id":"INBOX","name":"INBOX","type":"system"},
                              {"id":"Label_1","name":"Receipts","type":"user"}]}"#
                    .into(),
            ),
            ("GET", "/gmail/v1/users/me/settings/forwardingAddresses") => (
                200,
                Vec::new(),
                r#"{"forwardingAddresses":[{"forwardingEmail":"Me@Example.org","verificationStatus":"accepted"},
                                           {"forwardingEmail":"x@y.org","verificationStatus":"pending"}]}"#
                    .into(),
            ),
            ("POST", "/gmail/v1/users/me/settings/filters") => {
                if request.text().contains("refuse") {
                    return (
                        400,
                        Vec::new(),
                        r#"{"error":{"code":400,"message":"Filter doesn't have any criteria"}}"#.into(),
                    );
                }
                let mut n = next.lock().unwrap();
                *n += 1;
                let id = format!("f{n}");
                kept.lock().unwrap().push(id.clone());
                (200, Vec::new(), format!(r#"{{"id":"{id}"}}"#))
            }
            ("DELETE", path) => {
                let id = path.rsplit('/').next().unwrap_or_default().to_owned();
                let mut all = kept.lock().unwrap();
                let before = all.len();
                all.retain(|f| *f != id);
                if all.len() == before {
                    (404, Vec::new(), "{}".into())
                } else {
                    (204, Vec::new(), String::new())
                }
            }
            _ => (404, Vec::new(), "{}".into()),
        }
    });
    (api, seen, filters)
}

#[test]
fn pushes_filters_and_keeps_them_in_line_with_the_rules() {
    let (api, seen, filters) = fake_gmail();
    let client = client(&api, &format!("https://mail.test/ {GOOGLE_GMAIL_SETTINGS}"));
    assert!(smol::block_on(client.allowed()).unwrap());
    let folders = gmail().folders;
    let account = AccountId(1);
    let receipts = rule(
        1,
        vec![condition(Field::Subject, Comparator::Contains, "receipt")],
        vec![Action::Move { folder: 6 }],
    );
    let news = Rule {
        match_mode: MatchMode::Any,
        ..rule(
            2,
            vec![
                condition(Field::From, Comparator::Contains, "news@x.org"),
                condition(Field::From, Comparator::Contains, "list@x.org"),
            ],
            vec![
                Action::Archive,
                Action::Forward {
                    to: "me@example.org".into(),
                },
            ],
        )
    };
    let quiet = rule(
        3,
        vec![condition(Field::From, Comparator::Contains, "a@x.org")],
        vec![Action::DontNotify],
    );
    let rules = vec![receipts.clone(), news.clone(), quiet];

    let push1 = smol::block_on(push(&client, &rules, account, folders.clone(), Vec::new()));
    assert!(push1.error.is_none(), "{:?}", push1.error);
    assert_eq!(
        push1.verdicts,
        [
            (1, Ok(())),
            (2, Ok(())),
            (
                3,
                Err(RunsNote::Action {
                    service: RunsOn::Gmail,
                    action: Action::DontNotify
                })
            ),
        ]
    );
    let ids: Vec<Vec<String>> = push1.rows.iter().map(|r| r.remote_ids.clone()).collect();
    assert_eq!(
        ids,
        [
            vec!["f1".to_owned()],
            vec!["f2".to_owned(), "f3".to_owned()]
        ]
    );
    assert_eq!(*filters.lock().unwrap(), ["theirs", "f1", "f2", "f3"]);
    {
        let seen = seen.lock().unwrap();
        let made: Vec<serde_json::Value> = seen
            .iter()
            .filter(|s| s.method == "POST")
            .map(|s| serde_json::from_str(&s.text()).unwrap())
            .collect();
        assert_eq!(
            made[0],
            serde_json::json!({"criteria": {"query": "subject:\"receipt\""},
                               "action": {"addLabelIds": ["Label_1"], "removeLabelIds": ["INBOX"]}})
        );
        assert_eq!(made[2]["criteria"]["query"], "from:(list@x.org)");
        assert_eq!(made[2]["action"]["forward"], "me@example.org");
        assert!(
            seen.iter()
                .all(|s| s.header("Authorization") == Some("Bearer at-1"))
        );
    }

    // Nothing changed: nothing sent.
    let before = seen.lock().unwrap().len();
    let push2 = smol::block_on(push(
        &client,
        &rules,
        account,
        folders.clone(),
        push1.rows.clone(),
    ));
    assert_eq!(push2.rows, push1.rows);
    assert_eq!(
        seen.lock().unwrap().len(),
        before + 2,
        "labels and addresses only"
    );

    // A changed rule's filters go and new ones come; a deleted rule's go.
    let changed = Rule {
        conditions: vec![condition(Field::Subject, Comparator::Contains, "invoice")],
        ..receipts.clone()
    };
    let push3 = smol::block_on(push(
        &client,
        &[changed],
        account,
        folders.clone(),
        push2.rows,
    ));
    assert!(push3.error.is_none());
    assert_eq!(push3.rows.len(), 1);
    assert_eq!(push3.rows[0].remote_ids, ["f4"]);
    assert_eq!(*filters.lock().unwrap(), ["theirs", "f4"]);

    // Gmail refuses one: that rule stays in Katna, saying why.
    let refused = rule(
        5,
        vec![condition(Field::Subject, Comparator::Contains, "refuse")],
        vec![Action::Star],
    );
    let push4 = smol::block_on(push(
        &client,
        &[receipts.clone(), refused],
        account,
        folders.clone(),
        push3.rows,
    ));
    assert!(matches!(push4.error, Some(Error::Rejected(_))));
    assert_eq!(push4.verdicts[0], (1, Ok(())));
    match &push4.verdicts[1] {
        (5, Err(RunsNote::Failed { service, error })) => {
            assert_eq!(*service, RunsOn::Gmail);
            assert!(
                error.contains("Filter doesn't have any criteria"),
                "{error}"
            );
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        push4.rows.iter().map(|r| r.rule_id).collect::<Vec<_>>(),
        [1]
    );
    assert_eq!(*filters.lock().unwrap(), ["theirs", "f5"]);
}

#[test]
fn without_the_scope_rules_wait_for_a_new_sign_in() {
    let old = client("http://127.0.0.1:1", "https://mail.test/");
    assert!(!smol::block_on(old.allowed()).unwrap());
    // Gmail refusing the token: sign in again.
    let (api, _) = serve(|_, _| (403, Vec::new(), "{}".into()));
    let client = client(&api, &format!("https://mail.test/ {GOOGLE_GMAIL_SETTINGS}"));
    let rules = [rule(
        1,
        vec![condition(Field::From, Comparator::Contains, "a@x.org")],
        vec![Action::Star],
    )];
    let push = smol::block_on(push(
        &client,
        &rules,
        AccountId(1),
        gmail().folders,
        Vec::new(),
    ));
    assert!(matches!(push.error, Some(Error::Auth(_))));
    assert_eq!(push.verdicts, [(1, Err(RunsNote::SignIn))]);
}

#[test]
fn reads_the_signatures_gmail_adds() {
    let (api, _) = serve(|request, _| match request.path.as_str() {
        "/gmail/v1/users/me/settings/sendAs" => (
            200,
            Vec::new(),
            r#"{"sendAs":[{"sendAsEmail":"alias@x.org","displayName":"Ada","signature":""},
                          {"sendAsEmail":"ada@x.org","displayName":"Ada L","isDefault":true,
                           "signature":"<div dir=\"ltr\"><b>Ada</b></div>"}]}"#
                .into(),
        ),
        _ => (404, Vec::new(), "{}".into()),
    });
    let gmail = client(&api, "https://mail.test/");
    let all = smol::block_on(gmail.send_as()).unwrap();
    assert_eq!(all[0].send_as_email, "ada@x.org");
    assert_eq!(all[0].signature, "<div dir=\"ltr\"><b>Ada</b></div>");
    assert!(all[1].signature.is_empty());
    // A token without a scope that allows it: sign in again.
    let (api, _) = serve(|_, _| {
        (
            403,
            Vec::new(),
            r#"{"error":{"code":403,"message":"Request had insufficient authentication scopes."}}"#
                .into(),
        )
    });
    let gmail = client(&api, "https://mail.test/");
    assert!(matches!(
        smol::block_on(gmail.send_as()),
        Err(Error::Auth(_))
    ));
}
