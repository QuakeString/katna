// SPDX-License-Identifier: GPL-3.0-or-later

use katna_store::FolderId;

use super::*;

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

fn folders() -> Vec<StoredFolder> {
    vec![
        folder(1, "INBOX", Some(FolderRole::Inbox)),
        folder(2, "Archive", Some(FolderRole::Archive)),
        folder(3, "Trash", Some(FolderRole::Trash)),
        folder(4, "Bills \"2026\"", None),
    ]
}

/// Dovecot Pigeonhole's usual extensions.
fn dovecot() -> Extensions {
    Extensions::new([
        "fileinto",
        "reject",
        "envelope",
        "encoded-character",
        "vacation",
        "subaddress",
        "comparator-i;ascii-numeric",
        "relational",
        "regex",
        "imap4flags",
        "copy",
        "include",
        "variables",
        "body",
        "enotify",
        "environment",
        "mailbox",
        "date",
        "index",
        "ihave",
        "duplicate",
        "mime",
        "foreverypart",
        "extracttext",
    ])
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

fn sieve(c: Condition) -> Result<String, RunsNote> {
    translate(
        &rule(1, vec![c], vec![Action::MarkRead]),
        &folders(),
        &dovecot(),
    )
    .map(|r| {
        r.text
            .lines()
            .nth(1)
            .unwrap()
            .trim_start_matches("if ")
            .trim_end_matches(" {")
            .to_owned()
    })
}

fn cant(c: &Condition) -> Result<String, RunsNote> {
    Err(RunsNote::Condition {
        service: RunsOn::Sieve,
        field: c.field,
        comparator: c.comparator,
    })
}

#[test]
fn tests_say_what_katna_matches() {
    use Comparator::*;
    use Field::*;
    let ok = |field, comparator, value: &str, want: &str| {
        assert_eq!(
            sieve(condition(field, comparator, value)).as_deref(),
            Ok(want),
            "{field:?} {comparator:?} {value:?}"
        );
    };
    ok(From, Contains, " bob ", r#"header :contains "from" "bob""#);
    ok(
        AnyRecipient,
        NotContains,
        "list@",
        r#"not header :contains ["to", "cc", "bcc"] "list@""#,
    );
    ok(
        ReplyTo,
        Contains,
        "a\"b\\c",
        r#"header :contains "reply-to" "a\"b\\c""#,
    );
    ok(Subject, Equals, "Hello", r#"header :is "subject" "Hello""#);
    ok(
        Subject,
        BeginsWith,
        "50% *off*",
        r#"header :matches "subject" "50% \\*off\\**""#,
    );
    ok(
        Subject,
        EndsWith,
        "?",
        r#"header :matches "subject" "*\\?""#,
    );
    ok(
        From,
        Equals,
        "bob@example.org",
        r#"address :all :is "from" "bob@example.org""#,
    );
    ok(
        From,
        BeginsWith,
        "bob@",
        r#"address :all :matches "from" "bob@*""#,
    );
    ok(
        From,
        EndsWith,
        "@bank.test",
        r#"address :all :matches "from" "*@bank.test""#,
    );
    ok(
        Cc,
        EndsWith,
        "example.org",
        r#"address :all :matches "cc" "*example.org""#,
    );
    ok(
        Body,
        Contains,
        "unsubscribe",
        r#"body :text :contains "unsubscribe""#,
    );
    ok(Body, NotContains, "x", r#"not body :text :contains "x""#);
    ok(
        Subject,
        Matches,
        "^invoice [0-9]+$",
        r#"header :comparator "i;ascii-casemap" :regex "subject" "^invoice [0-9]+$""#,
    );
    ok(
        From,
        Matches,
        "bob|carol",
        r#"header :comparator "i;ascii-casemap" :regex "from" "bob|carol""#,
    );

    // What Sieve can't say as Katna means it.
    for c in [
        condition(From, Equals, "Bob Builder"),
        condition(From, BeginsWith, "Bob"),
        condition(From, EndsWith, "Builder"),
        condition(From, Matches, "^bob@"),
        condition(Subject, Matches, r"\d+"),
        condition(Subject, Matches, "(?i)x"),
        condition(Subject, Matches, "a.*?b"),
        condition(Subject, Matches, "café"),
        condition(Subject, Contains, "Rechnung für"),
        condition(Subject, Contains, "two\nlines"),
        condition(Body, Equals, "x"),
        condition(AttachmentName, Contains, "pdf"),
        condition(HasAttachment, Contains, "yes"),
    ] {
        assert_eq!(sieve(c.clone()), cant(&c), "{c:?}");
    }

    // Some servers fold any letter's case.
    let mut unicode = dovecot();
    unicode.0.insert("comparator-i;unicode-casemap".into());
    let r = translate(
        &rule(
            1,
            vec![condition(Subject, Contains, "Rechnung für")],
            vec![Action::MarkRead],
        ),
        &folders(),
        &unicode,
    )
    .unwrap();
    assert!(
        r.text.contains(
            r#"header :comparator "i;unicode-casemap" :contains "subject" "Rechnung für""#
        )
    );
    assert!(r.requires.contains("comparator-i;unicode-casemap"));

    // Without the extensions, they stay in Katna.
    let plain = Extensions::new(["fileinto"]);
    for c in [
        condition(Body, Contains, "x"),
        condition(Subject, Matches, "x+"),
    ] {
        let r = translate(
            &rule(1, vec![c.clone()], vec![Action::Archive]),
            &folders(),
            &plain,
        );
        assert_eq!(r.map(|r| r.text), cant(&c));
    }
}

#[test]
fn actions_flag_send_then_file() {
    let r = translate(
        &Rule {
            stop: true,
            match_mode: MatchMode::Any,
            ..rule(
                7,
                vec![
                    condition(Field::From, Comparator::Contains, "bank"),
                    condition(Field::Subject, Comparator::Contains, "statement"),
                ],
                vec![
                    Action::Move { folder: 4 },
                    Action::Forward {
                        to: " me@example.org ".into(),
                    },
                    Action::Star,
                    Action::MarkRead,
                ],
            )
        },
        &folders(),
        &dovecot(),
    )
    .unwrap();
    assert_eq!(
        r.text,
        "# Rule 7\n\
         if anyof (header :contains \"from\" \"bank\", header :contains \"subject\" \"statement\") {\n\
         \x20   addflag \"\\\\Flagged\";\n\
         \x20   addflag \"\\\\Seen\";\n\
         \x20   redirect :copy \"me@example.org\";\n\
         \x20   fileinto \"Bills \\\"2026\\\"\";\n\
         \x20   stop;\n\
         }\n"
    );
    assert_eq!(
        r.requires.into_iter().collect::<Vec<_>>(),
        ["copy", "fileinto", "imap4flags"]
    );

    let all = |actions| {
        translate(
            &rule(
                1,
                vec![
                    condition(Field::From, Comparator::Contains, "a"),
                    condition(Field::To, Comparator::Contains, "b"),
                ],
                actions,
            ),
            &folders(),
            &dovecot(),
        )
    };
    let archived = all(vec![Action::Archive]).unwrap().text;
    assert!(
        archived.contains(
            "if allof (header :contains \"from\" \"a\", header :contains \"to\" \"b\") {"
        )
    );
    assert!(archived.contains("    fileinto \"Archive\";\n"));
    assert!(
        all(vec![Action::Trash])
            .unwrap()
            .text
            .contains("fileinto \"Trash\";")
    );
    assert!(
        all(vec![Action::Move { folder: 1 }])
            .unwrap()
            .text
            .contains("    keep;\n")
    );

    for action in [
        Action::MarkImportant,
        Action::AddLabel { folder: 4 },
        Action::DontNotify,
        Action::MarkReadAfter { days: 3 },
    ] {
        assert_eq!(
            all(vec![Action::MarkRead, action.clone()]),
            Err(RunsNote::Action {
                service: RunsOn::Sieve,
                action
            })
        );
    }
    assert_eq!(
        all(vec![Action::Move { folder: 99 }]),
        Err(RunsNote::Folder {
            service: RunsOn::Sieve
        })
    );
    // No archive folder.
    let none = translate(
        &rule(
            1,
            vec![condition(Field::From, Comparator::Contains, "a")],
            vec![Action::Archive],
        ),
        &folders()[..1],
        &dovecot(),
    );
    assert_eq!(
        none,
        Err(RunsNote::Folder {
            service: RunsOn::Sieve
        })
    );
    // A server without imap4flags or copy.
    let bare = Extensions::new(["fileinto"]);
    for action in [Action::Star, Action::Forward { to: "x@y.z".into() }] {
        let r = translate(
            &rule(
                1,
                vec![condition(Field::From, Comparator::Contains, "a")],
                vec![action.clone()],
            ),
            &folders(),
            &bare,
        );
        assert_eq!(
            r,
            Err(RunsNote::Action {
                service: RunsOn::Sieve,
                action
            })
        );
    }
}

#[test]
fn the_script_holds_the_rules_that_run_on_the_server() {
    let rules = [
        rule(
            1,
            vec![condition(Field::From, Comparator::Contains, "news@")],
            vec![Action::Move { folder: 2 }],
        ),
        rule(
            2,
            vec![condition(Field::Body, Comparator::Contains, "urgent")],
            vec![Action::Star],
        ),
        Rule {
            accounts: vec![2],
            ..rule(
                3,
                vec![condition(Field::From, Comparator::Contains, "x")],
                vec![Action::Star],
            )
        },
        rule(
            4,
            vec![condition(Field::Subject, Comparator::Contains, "quiet")],
            vec![Action::DontNotify],
        ),
        rule(
            5,
            vec![condition(Field::Subject, Comparator::Contains, "later")],
            vec![Action::MarkRead],
        ),
    ];
    let (script, verdicts) = account_script(
        &rules,
        AccountId(1),
        &folders(),
        &dovecot(),
        Some("roundcube \"filters\""),
    );
    assert_eq!(
        script,
        "# Katna Mail's rules. Katna writes this script again whenever a rule\n\
         # changes: change them in Katna Mail, Settings > Folders & rules.\n\
         require [\"body\", \"fileinto\", \"imap4flags\", \"include\"];\n\
         include :personal \"roundcube \\\"filters\\\"\";\n\
         \n\
         # Rule 1\n\
         if header :contains \"from\" \"news@\" {\n\
         \x20   fileinto \"Archive\";\n\
         }\n\
         \n\
         # Rule 2\n\
         if body :text :contains \"urgent\" {\n\
         \x20   addflag \"\\\\Flagged\";\n\
         }\n"
    );
    assert_eq!(included(&script).as_deref(), Some("roundcube \"filters\""));
    assert_eq!(
        verdicts,
        [
            (1, Ok(())),
            (2, Ok(())),
            (
                4,
                Err(RunsNote::Action {
                    service: RunsOn::Sieve,
                    action: Action::DontNotify
                })
            ),
            (
                5,
                Err(RunsNote::Order {
                    service: RunsOn::Sieve
                })
            ),
        ]
    );

    // No rules, nothing included: a script that does nothing.
    let (empty, verdicts) = account_script(&[], AccountId(1), &folders(), &dovecot(), None);
    assert!(verdicts.is_empty());
    assert!(!empty.contains("require") && !empty.contains("if "));
    assert_eq!(included(&empty), None);
}

#[test]
fn portable_patterns() {
    for ok in [
        "^a.b$",
        r"x\.y",
        "[a-z]+@(example|test)\\.org",
        "a{2,3}",
        "(a|b)*",
    ] {
        assert!(portable_regex(ok), "{ok}");
    }
    for bad in [r"\d", r"\bword", "(?i)x", "a+?", "a*?", "a{2}?", "ü", "a**"] {
        assert!(!portable_regex(bad), "{bad}");
    }
}
