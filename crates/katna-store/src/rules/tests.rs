// SPDX-License-Identifier: GPL-3.0-or-later

use katna_core::{AccountKind, Paths};

use super::*;
use crate::remote::FolderRole;
use crate::{Mode, NewMessage, NewParticipant};

fn person(role: ParticipantRole, email: &str, name: Option<&str>) -> StoredParticipant {
    StoredParticipant {
        role,
        email_norm: email.to_owned(),
        domain: email.rsplit('@').next().unwrap_or_default().to_owned(),
        display_name: name.map(str::to_owned),
    }
}

/// A message from Bob at Example to Alice, cc Carol, with a PDF.
fn mail() -> MailFacts {
    MailFacts {
        account: AccountId(1),
        subject: "Invoice 42 for September".to_owned(),
        participants: vec![
            person(
                ParticipantRole::From,
                "bob@example.org",
                Some("Bob Builder"),
            ),
            person(ParticipantRole::To, "alice@katna.test", None),
            person(ParticipantRole::Cc, "carol@example.net", Some("Carol")),
            person(ParticipantRole::Bcc, "dave@hidden.test", None),
            person(ParticipantRole::ReplyTo, "billing@example.org", None),
        ],
        attachment_names: vec!["Invoice-42.PDF".to_owned()],
        has_attachments: true,
        body: "Please pay by Friday.\nThanks".to_owned(),
        category: MailCategory::Updates,
        mailing_list: false,
    }
}

fn condition(field: Field, comparator: Comparator, value: &str) -> Condition {
    Condition {
        field,
        comparator,
        value: value.to_owned(),
    }
}

fn rule(conditions: Vec<Condition>) -> Rule {
    Rule {
        name: "Test".to_owned(),
        conditions,
        actions: vec![Action::MarkRead],
        accounts: vec![1],
        ..Rule::default()
    }
}

fn hits(field: Field, comparator: Comparator, value: &str) -> bool {
    Matcher::new(rule(vec![condition(field, comparator, value)]))
        .unwrap()
        .matches(&mail())
}

#[test]
fn every_comparator_compares_text_without_case() {
    use Comparator::*;
    let s = Field::Subject;
    assert!(hits(s, Contains, "INVOICE"));
    assert!(!hits(s, Contains, "receipt"));
    assert!(hits(s, NotContains, "receipt"));
    assert!(!hits(s, NotContains, "invoice"));
    assert!(hits(s, BeginsWith, "invoice 4"));
    assert!(!hits(s, BeginsWith, "42"));
    assert!(hits(s, EndsWith, "SEPTEMBER"));
    assert!(!hits(s, EndsWith, "invoice"));
    assert!(hits(s, Equals, "invoice 42 for september"));
    assert!(!hits(s, Equals, "invoice 42"));
    assert!(hits(s, Matches, r"^invoice \d+ "));
    assert!(!hits(s, Matches, r"^\d+"));
}

#[test]
fn addresses_match_on_name_and_email() {
    use Comparator::*;
    assert!(hits(Field::From, Contains, "builder"), "the name");
    assert!(hits(Field::From, EndsWith, "@example.org"), "the email");
    assert!(hits(Field::From, Equals, "bob@example.org"));
    assert!(hits(Field::From, Equals, "Bob Builder"));
    assert!(hits(Field::From, BeginsWith, "bob"));
    assert!(!hits(Field::From, Contains, "alice"));
    assert!(hits(Field::To, Equals, "alice@katna.test"));
    assert!(!hits(Field::To, Contains, "carol"));
    assert!(hits(Field::Cc, Contains, "carol"));
    assert!(hits(Field::AnyRecipient, Contains, "carol"), "cc");
    assert!(hits(Field::AnyRecipient, Contains, "alice"), "to");
    assert!(hits(Field::AnyRecipient, Contains, "hidden.test"), "bcc");
    assert!(
        !hits(Field::AnyRecipient, Contains, "bob"),
        "not the sender"
    );
    assert!(hits(Field::ReplyTo, BeginsWith, "billing@"));
    assert!(hits(Field::From, NotContains, "alice"));
    assert!(!hits(Field::From, NotContains, "bob"));
    assert!(hits(Field::From, Matches, r"^bob@.*\.org$"));
}

#[test]
fn body_and_attachments() {
    use Comparator::*;
    assert!(hits(Field::Body, Contains, "pay by friday"));
    assert!(hits(Field::Body, NotContains, "unsubscribe"));
    assert!(hits(Field::Body, Matches, r"(?m)^thanks$"));
    assert!(hits(Field::AttachmentName, EndsWith, ".pdf"));
    assert!(!hits(Field::AttachmentName, Contains, "xlsx"));
    assert!(hits(Field::AttachmentName, NotContains, "xlsx"));
    assert!(hits(Field::HasAttachment, Contains, ""));
    assert!(hits(Field::HasAttachment, Equals, "true"));
    assert!(!hits(Field::HasAttachment, Equals, "false"));
    let mut plain = mail();
    plain.has_attachments = false;
    plain.attachment_names.clear();
    let none = Matcher::new(rule(vec![condition(Field::HasAttachment, Equals, "no")])).unwrap();
    assert!(none.matches(&plain));
    assert!(!none.matches(&mail()));
}

#[test]
fn inbox_tab_and_mailing_list() {
    use Comparator::*;
    assert!(hits(Field::Tab, Equals, "updates"));
    assert!(hits(Field::Tab, Contains, "Updates"));
    assert!(!hits(Field::Tab, Equals, "promotions"));
    assert!(hits(Field::Tab, NotContains, "promotions"));
    assert!(!hits(Field::MailingList, Equals, "true"));
    assert!(hits(Field::MailingList, Equals, "no"));
    let mut list = mail();
    list.mailing_list = true;
    list.category = MailCategory::Forums;
    let from_list = Matcher::new(rule(vec![
        condition(Field::MailingList, Equals, ""),
        condition(Field::Tab, Equals, "forums"),
    ]))
    .unwrap();
    assert!(from_list.matches(&list));
    assert!(!from_list.matches(&mail()));
    let bad = rule(vec![condition(Field::Tab, Equals, "inbox")]);
    assert!(bad.validate().unwrap_err().contains("tab"));
    // A yes-or-no field needs no value.
    assert_eq!(
        rule(vec![condition(Field::MailingList, Equals, "")]).validate(),
        Ok(())
    );
}

#[test]
fn all_or_any_and_stop() {
    let one = condition(Field::Subject, Comparator::Contains, "invoice");
    let other = condition(Field::From, Comparator::Contains, "nobody");
    let mut both = rule(vec![one.clone(), other.clone()]);
    assert!(!Matcher::new(both.clone()).unwrap().matches(&mail()));
    both.match_mode = MatchMode::Any;
    assert!(Matcher::new(both).unwrap().matches(&mail()));

    let named = |name: &str, stop: bool, enabled: bool, accounts: Vec<i64>| {
        Matcher::new(Rule {
            name: name.to_owned(),
            stop,
            enabled,
            accounts,
            ..rule(vec![one.clone()])
        })
        .unwrap()
    };
    let list = [
        named("off", false, false, vec![1]),
        named("other account", false, true, vec![2]),
        named("first", false, true, vec![2, 1]),
        named("second", true, true, vec![1]),
        named("after stop", false, true, vec![1]),
    ];
    let names: Vec<&str> = matching(&list, &mail())
        .iter()
        .map(|m| m.rule().name.as_str())
        .collect();
    assert_eq!(names, ["first", "second"]);
}

#[test]
fn validation_says_what_is_wrong() {
    let good = rule(vec![condition(Field::Subject, Comparator::Contains, "x")]);
    assert_eq!(good.validate(), Ok(()));
    let bad = |change: &dyn Fn(&mut Rule)| {
        let mut rule = good.clone();
        change(&mut rule);
        rule.validate().unwrap_err()
    };
    assert!(bad(&|r| r.name = "  ".into()).contains("name"));
    assert!(bad(&|r| r.conditions.clear()).contains("condition"));
    assert!(bad(&|r| r.actions.clear()).contains("action"));
    assert!(bad(&|r| r.accounts.clear()).contains("account"));
    assert!(bad(&|r| r.conditions[0].value = String::new()).contains("value"));
    assert!(
        bad(&|r| r.conditions[0] = condition(Field::Body, Comparator::Matches, "("))
            .contains("regular")
    );
    assert!(bad(&|r| r.actions = vec![Action::Archive, Action::Trash]).contains("one place"));
    assert!(
        bad(&|r| r.actions = vec![Action::Move { folder: 3 }, Action::Archive])
            .contains("one place")
    );
    // A folder in each account (the daemon checks the accounts).
    let mut each = good.clone();
    each.actions = vec![Action::Move { folder: 3 }, Action::Move { folder: 8 }];
    assert_eq!(each.validate(), Ok(()));
    assert!(
        bad(&|r| r.actions = vec![Action::Forward {
            to: "nobody".into()
        }])
        .contains("email")
    );
    assert!(bad(&|r| r.actions = vec![Action::MarkReadAfter { days: 0 }]).contains("days"));
    let mut fine = good.clone();
    fine.conditions = vec![condition(Field::HasAttachment, Comparator::Contains, "")];
    fine.actions = vec![
        Action::Move { folder: 3 },
        Action::Forward {
            to: "me@example.org".into(),
        },
    ];
    assert_eq!(fine.validate(), Ok(()));
}

#[test]
fn json_is_short_to_write() {
    let rule: Rule = serde_json::from_str(
        r#"{"name": "Bills", "accounts": [1],
            "conditions": [{"field": "from", "comparator": "ends_with", "value": "@bank.test"}],
            "actions": [{"type": "move", "folder": 7}, {"type": "mark_read"},
                        {"type": "mark_read_after", "days": 3}]}"#,
    )
    .unwrap();
    assert!(rule.enabled);
    assert_eq!(rule.id, 0);
    assert_eq!(rule.match_mode, MatchMode::All);
    assert_eq!(rule.runs_on, RunsOn::Katna);
    assert_eq!(rule.actions[0], Action::Move { folder: 7 });
    let back: Rule = serde_json::from_str(&serde_json::to_string(&rule).unwrap()).unwrap();
    assert_eq!(back, rule);
}

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
    (dir, store)
}

#[test]
fn saves_lists_reorders_switches_and_deletes() {
    let (_dir, mut store) = store();
    let base = rule(vec![condition(Field::Subject, Comparator::Contains, "x")]);
    let ids: Vec<i64> = ["a", "b", "c"]
        .into_iter()
        .map(|name| {
            store
                .save_rule(&Rule {
                    name: name.to_owned(),
                    ..base.clone()
                })
                .unwrap()
        })
        .collect();
    let names = |store: &Store| -> Vec<String> {
        store.rules().unwrap().into_iter().map(|r| r.name).collect()
    };
    assert_eq!(names(&store), ["a", "b", "c"]);

    let mut b = store.rule(ids[1]).unwrap().unwrap();
    assert_eq!(b.position, 1);
    b.name = "B".into();
    b.match_mode = MatchMode::Any;
    b.stop = true;
    b.accounts = vec![1, 2];
    b.actions = vec![Action::AddLabel { folder: 9 }, Action::DontNotify];
    assert_eq!(store.save_rule(&b).unwrap(), ids[1]);
    assert_eq!(store.rule(ids[1]).unwrap().unwrap(), b);

    store.reorder_rules(&[ids[2], 999, ids[0]]).unwrap();
    assert_eq!(names(&store), ["c", "a", "B"]);
    // A new rule goes last.
    store
        .save_rule(&Rule {
            name: "d".into(),
            ..base.clone()
        })
        .unwrap();
    assert_eq!(names(&store), ["c", "a", "B", "d"]);

    assert!(
        store
            .fail_rule(ids[0], "the folder no longer exists")
            .unwrap()
    );
    let a = store.rule(ids[0]).unwrap().unwrap();
    assert!(!a.enabled);
    assert_eq!(a.last_error.as_deref(), Some("the folder no longer exists"));
    assert!(store.set_rule_enabled(ids[0], true).unwrap());
    let a = store.rule(ids[0]).unwrap().unwrap();
    assert!(a.enabled && a.last_error.is_none());
    store.fail_rule(ids[0], "again").unwrap();
    // Saving after an edit clears the error too, as saved.
    store.save_rule(&a).unwrap();
    assert_eq!(store.rule(ids[0]).unwrap().unwrap().last_error, None);
    assert!(store.set_rule_enabled(ids[0], false).unwrap());
    assert!(!store.set_rule_enabled(999, false).unwrap());

    assert!(store.delete_rule(ids[1]).unwrap());
    assert!(!store.delete_rule(ids[1]).unwrap());
    assert_eq!(names(&store), ["c", "a", "d"]);

    // A rule made from a starter keeps its key, also when saved again.
    let id = store
        .save_rule(&Rule {
            name: "Receipts".into(),
            starter: Some("receipts".into()),
            ..base.clone()
        })
        .unwrap();
    let mut saved = store.rule(id).unwrap().unwrap();
    assert_eq!(saved.starter.as_deref(), Some("receipts"));
    saved.name = "Bills".into();
    saved.starter = None;
    store.save_rule(&saved).unwrap();
    assert_eq!(
        store.rule(id).unwrap().unwrap().starter.as_deref(),
        Some("receipts")
    );
}

#[test]
fn read_only_stores_read_rules_but_do_not_save_them() {
    let (dir, mut store) = store();
    store
        .save_rule(&rule(vec![condition(
            Field::Subject,
            Comparator::Contains,
            "x",
        )]))
        .unwrap();
    let mut read_only = Store::open(&Paths::with_root(dir.path()), Mode::ReadOnly).unwrap();
    assert_eq!(read_only.rules().unwrap().len(), 1);
    assert!(matches!(
        read_only.save_rule(&Rule::default()),
        Err(Error::ReadOnly)
    ));
}

const NOW: i64 = 1_790_000_000;
const DAY: i64 = 86_400;

/// Stores a message in `folder` from `from`, `days_ago` days old.
fn add(
    store: &mut Store,
    account: AccountId,
    folder: crate::FolderId,
    subject: &str,
    from: &str,
    days_ago: i64,
    flags: MessageFlags,
) -> MessageId {
    let raw = format!("Subject: {subject}\r\nFrom: {from}\r\n\r\nBody of {subject}\r\n");
    let participants = [NewParticipant {
        role: ParticipantRole::From,
        email_norm: from,
        domain: from.rsplit('@').next().unwrap(),
        display_name: None,
    }];
    let mut batch = store.mail_batch().unwrap();
    let added = batch
        .add_message(
            account,
            folder,
            &NewMessage {
                raw: raw.as_bytes(),
                message_id_hdr: None,
                subject: Some(subject),
                date: Some(NOW - days_ago * DAY),
                flags,
                has_attachments: false,
                list_id: None,
                snippet: Some("a snippet"),
                participants: &participants,
                in_reply_to: None,
                references: &[],
                category: None,
            },
        )
        .unwrap();
    batch.commit().unwrap();
    match added {
        crate::Added::Message(id) => id,
        other => panic!("{other:?}"),
    }
}

#[test]
fn candidates_and_preview_look_at_recent_inbox_mail_from_others() {
    let (_dir, mut store) = store();
    let account = store
        .add_account(AccountKind::Imap, "Alice", "Alice@Katna.test")
        .unwrap()
        .id;
    let (inbox, archive) = {
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let archive = batch
            .upsert_folder(account, "Archive", Some(FolderRole::Archive))
            .unwrap();
        batch.commit().unwrap();
        (inbox, archive)
    };
    let none = MessageFlags::empty();
    let new = add(
        &mut store,
        account,
        inbox,
        "Invoice 1",
        "bob@bank.test",
        1,
        none,
    );
    add(
        &mut store,
        account,
        inbox,
        "Invoice 2",
        "bob@bank.test",
        40,
        none,
    );
    add(
        &mut store,
        account,
        archive,
        "Invoice 3",
        "bob@bank.test",
        1,
        none,
    );
    add(
        &mut store,
        account,
        inbox,
        "Invoice 4",
        "alice@katna.test",
        1,
        none,
    );
    add(
        &mut store,
        account,
        inbox,
        "Invoice 5",
        "bob@bank.test",
        1,
        MessageFlags::DRAFT,
    );
    let later = add(
        &mut store,
        account,
        inbox,
        "Hello",
        "bob@bank.test",
        2,
        none,
    );

    let ids = |list: Vec<StoredMessage>| list.into_iter().map(|m| m.id).collect::<Vec<_>>();
    let since = NOW - 30 * DAY;
    assert_eq!(
        ids(store.rule_candidates(account, MessageId(0), since).unwrap()),
        [new, later],
        "not old, archived, own or draft mail"
    );
    assert_eq!(
        ids(store.rule_candidates(account, new, since).unwrap()),
        [later]
    );

    let mut invoices = rule(vec![condition(
        Field::Subject,
        Comparator::BeginsWith,
        "invoice",
    )]);
    invoices.accounts = vec![account.0];
    invoices.enabled = false;
    assert_eq!(store.rule_preview(&invoices, 30, NOW, |_| None).unwrap(), 1);
    assert_eq!(store.rule_preview(&invoices, 60, NOW, |_| None).unwrap(), 2);
    invoices.accounts = vec![account.0 + 1];
    assert_eq!(store.rule_preview(&invoices, 60, NOW, |_| None).unwrap(), 0);

    // The body comes from the caller, else the snippet stands in.
    let mut body = rule(vec![condition(
        Field::Body,
        Comparator::Contains,
        "body of hello",
    )]);
    body.accounts = vec![account.0];
    assert_eq!(store.rule_preview(&body, 30, NOW, |_| None).unwrap(), 0);
    let text = |m: &StoredMessage| Some(format!("Body of {}", m.subject));
    assert_eq!(store.rule_preview(&body, 30, NOW, text).unwrap(), 1);
    body.conditions[0].value = "snippet".into();
    assert_eq!(store.rule_preview(&body, 30, NOW, |_| None).unwrap(), 2);
}

#[test]
fn keeps_where_rules_run_and_why() {
    let (_dir, mut store) = store();
    let account = store
        .add_account(AccountKind::Imap, "Alice", "alice@katna.test")
        .unwrap()
        .id;
    let mut saved = rule(vec![condition(Field::Subject, Comparator::Contains, "x")]);
    saved.accounts = vec![account.0];
    // What an app sends is not where it runs: the daemon says.
    saved.runs_on = RunsOn::Sieve;
    let id = store.save_rule(&saved).unwrap();
    assert_eq!(store.rule(id).unwrap().unwrap().runs_on, RunsOn::Katna);

    let note = RunsNote::Action {
        service: RunsOn::Gmail,
        action: Action::DontNotify,
    };
    assert!(store.set_rule_runs(id, RunsOn::Katna, Some(&note)).unwrap());
    let read = store.rule(id).unwrap().unwrap();
    assert_eq!(read.runs_note, Some(note.clone()));
    assert_eq!(store.rules().unwrap()[0].runs_note, Some(note));

    assert!(store.set_rule_runs(id, RunsOn::Sieve, None).unwrap());
    let read = store.rule(id).unwrap().unwrap();
    assert_eq!((read.runs_on, read.runs_note), (RunsOn::Sieve, None));
    // Saving keeps it.
    store
        .save_rule(&Rule {
            id,
            ..saved.clone()
        })
        .unwrap();
    assert_eq!(store.rule(id).unwrap().unwrap().runs_on, RunsOn::Sieve);
    assert!(!store.set_rule_runs(999, RunsOn::Katna, None).unwrap());

    let remote = RemoteRule {
        rule_id: id,
        runs_on: RunsOn::Gmail,
        remote_ids: vec!["f1".into(), "f2".into()],
        spec: "spec".into(),
    };
    store.put_remote_rule(account, &remote).unwrap();
    store.put_remote_rule(account, &remote).unwrap();
    assert_eq!(store.remote_rules(account).unwrap(), [remote]);
    assert_eq!(store.rules_on_service(account).unwrap(), [id]);
    // The filters of a deleted rule stay known, to be deleted.
    store.delete_rule(id).unwrap();
    assert_eq!(store.rules_on_service(account).unwrap(), [id]);
    store.drop_remote_rule(account, id).unwrap();
    assert!(store.remote_rules(account).unwrap().is_empty());

    assert_eq!(store.rule_server(account).unwrap(), None);
    let server = RuleServer {
        sieve: true,
        extensions: vec!["fileinto".into(), "imap4flags".into()],
        checked_at: 5,
    };
    store.set_rule_server(account, &server).unwrap();
    assert_eq!(store.rule_server(account).unwrap(), Some(server));
}
