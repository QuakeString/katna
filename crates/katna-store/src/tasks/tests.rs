// SPDX-License-Identifier: GPL-3.0-or-later

use super::*;
use crate::Mode;
use katna_core::{AccountKind, Paths};

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(dir.path());
    let store = Store::open(&paths, Mode::ReadWrite).unwrap();
    (dir, store)
}

fn titles(tasks: &[Task]) -> Vec<&str> {
    tasks.iter().map(|task| task.title.as_str()).collect()
}

fn account(store: &mut Store, address: &str) -> AccountId {
    store
        .add_account(AccountKind::Imap, address, address)
        .unwrap()
        .id
}

fn remote(id: &str, title: &str) -> RemoteTask {
    RemoteTask {
        remote_id: id.into(),
        title: title.into(),
        etag: format!("e-{id}-{title}"),
        ..RemoteTask::default()
    }
}

fn gmail_list(store: &mut Store, account: AccountId) -> i64 {
    let lists = [RemoteTaskList {
        remote_id: "L1".into(),
        title: "My Tasks".into(),
        is_default: true,
    }];
    assert!(store.sync_task_lists(account, &lists).unwrap());
    store.account_task_lists(account).unwrap()[0].0
}

#[test]
fn lists_due_tasks_first_by_day() {
    let (_dir, mut store) = store();
    store.add_task("someday", "").unwrap();
    store.add_task("friday", "2026-10-02").unwrap();
    store.add_task("today", "2026-09-29").unwrap();
    let tasks = store.tasks(0).unwrap();
    assert_eq!(titles(&tasks), ["today", "friday", "someday"]);
    assert_eq!(tasks[0].due, "2026-09-29");
    assert_eq!(tasks[2].due, "");
}

#[test]
fn done_tasks_leave_the_list_after_a_while() {
    let (_dir, mut store) = store();
    let id = store.add_task("renew domain", "").unwrap();
    assert!(store.set_task_done(id, true).unwrap());
    let done = store.tasks(0).unwrap();
    let done_at = done[0].done_at.expect("ticked off");
    assert_eq!(titles(&store.tasks(done_at).unwrap()), ["renew domain"]);
    assert!(store.tasks(done_at + 1).unwrap().is_empty());

    assert!(store.set_task_done(id, false).unwrap());
    assert_eq!(store.tasks(i64::MAX).unwrap()[0].done_at, None);
    assert!(!store.set_task_done(id + 1, true).unwrap());
}

#[test]
fn deleting_returns_the_task() {
    let (_dir, mut store) = store();
    let id = store.add_task("call the bank", "2026-09-30").unwrap();
    let task = store.delete_task(id).unwrap().expect("existed");
    assert_eq!(task.title, "call the bank");
    assert_eq!(task.due, "2026-09-30");
    assert!(store.tasks(0).unwrap().is_empty());
    assert_eq!(store.delete_task(id).unwrap(), None);
}

#[test]
fn local_tasks_are_never_pending() {
    let (_dir, mut store) = store();
    let list = store.default_task_list().unwrap();
    let lists = store.task_lists().unwrap();
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].account, None);
    store.add_task("water the plants", "").unwrap();
    assert!(store.pending_tasks(list).unwrap().is_empty());
}

#[test]
fn new_tasks_go_to_the_first_synced_default_list() {
    let (_dir, mut store) = store();
    let early = store.add_task("from the clock", "2026-10-01").unwrap();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    assert_eq!(store.default_task_list().unwrap(), list);

    // The clock's earlier tasks move there once, to be sent.
    assert_eq!(store.move_out_local_tasks().unwrap(), 1);
    assert_eq!(store.move_out_local_tasks().unwrap(), 0);
    let pending = store.pending_tasks(list).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].task.id, early);
    assert_eq!(pending[0].remote_id, None);
    assert!(
        store
            .task_lists()
            .unwrap()
            .iter()
            .all(|l| l.account.is_some())
    );

    let id = store.add_task("later", "").unwrap();
    assert_eq!(store.task(id).unwrap().unwrap().list, list);
}

#[test]
fn pushing_clears_dirty_unless_changed_again() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    let id = store.add_task("draft the memo", "").unwrap();
    let pending = store.pending_tasks(list).unwrap().remove(0);

    // Edited again while the push was under way: stays dirty.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let fields = TaskFields {
        title: "draft the memo today".into(),
        ..TaskFields::default()
    };
    assert!(store.edit_task(id, &fields).unwrap());
    store
        .task_pushed(id, pending.stamp, &remote("T1", "draft the memo"), true)
        .unwrap();
    let again = store.pending_tasks(list).unwrap();
    assert_eq!(again.len(), 1);
    assert_eq!(again[0].remote_id.as_deref(), Some("T1"));

    store
        .task_pushed(
            id,
            again[0].stamp,
            &remote("T1", "draft the memo today"),
            true,
        )
        .unwrap();
    assert!(store.pending_tasks(list).unwrap().is_empty());
}

#[test]
fn pulled_tasks_arrive_change_and_go() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    let mut parent = remote("P", "plan the trip");
    parent.due = "2026-10-05".into();
    let mut step = remote("S", "book the train");
    step.parent = Some("P".into());
    assert!(
        store
            .sync_tasks(list, &[step.clone(), parent.clone()], true)
            .unwrap()
    );
    let tasks = store.tasks_in(list).unwrap();
    assert_eq!(titles(&tasks), ["plan the trip", "book the train"]);
    assert_eq!(tasks[1].parent, Some(tasks[0].id));
    assert_eq!(tasks[0].due, "2026-10-05");

    // Same etag: nothing to do.
    assert!(!store.sync_tasks(list, &[parent.clone()], false).unwrap());

    // Ticked off on the phone.
    let mut done = remote("P", "plan the trip");
    done.done_at = Some(1_000);
    done.etag = "e2".into();
    assert!(store.sync_tasks(list, &[done], false).unwrap());
    assert_eq!(store.tasks_in(list).unwrap()[0].done_at, Some(1_000));

    // Deleted on the phone.
    let gone = RemoteTask {
        deleted: true,
        ..remote("S", "")
    };
    assert!(store.sync_tasks(list, &[gone], false).unwrap());
    assert_eq!(titles(&store.tasks_in(list).unwrap()), ["plan the trip"]);

    // A full pull without it removes it.
    assert!(store.sync_tasks(list, &[], true).unwrap());
    assert!(store.tasks_in(list).unwrap().is_empty());
}

#[test]
fn local_edits_win_until_sent_and_extras_stay() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    store
        .sync_tasks(list, &[remote("A", "pay rent")], true)
        .unwrap();
    let id = store.tasks_in(list).unwrap()[0].id;
    let fields = TaskFields {
        title: "pay rent".into(),
        due: "2026-10-01".into(),
        due_time: Some(9 * 60),
        starred: true,
        ..TaskFields::default()
    };
    store.edit_task(id, &fields).unwrap();
    // Google's copy changed meanwhile; ours is not sent yet and wins.
    store
        .sync_tasks(list, &[remote("A", "pay the rent")], false)
        .unwrap();
    let pending = store.pending_tasks(list).unwrap().remove(0);
    assert_eq!(pending.task.title, "pay rent");
    store
        .task_pushed(id, pending.stamp, &remote("A", "pay rent"), true)
        .unwrap();

    // Google keeps no time or star; they stay here.
    let mut back = remote("A", "pay rent!");
    back.due = "2026-10-01".into();
    store.sync_tasks(list, &[back], false).unwrap();
    let task = store.task(id).unwrap().unwrap();
    assert_eq!(task.title, "pay rent!");
    assert_eq!(task.due_time, Some(9 * 60));
    assert!(task.starred);

    // To Do keeps them: its values win.
    let mut todo = remote("A", "pay rent!");
    todo.etag = "e2".into();
    todo.extras = Some(TaskExtras::default());
    todo.starred = Some(false);
    store.sync_tasks(list, &[todo], false).unwrap();
    let task = store.task(id).unwrap().unwrap();
    assert_eq!(task.due_time, Some(9 * 60));
    assert!(!task.starred);
}

#[test]
fn deleting_a_synced_task_leaves_a_tombstone() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    let mut step = remote("S", "step");
    step.parent = Some("P".into());
    store
        .sync_tasks(list, &[remote("P", "parent"), step], true)
        .unwrap();
    let parent = store.tasks_in(list).unwrap()[0].id;
    store.delete_task(parent).unwrap().unwrap();
    assert!(store.tasks_in(list).unwrap().is_empty());
    let pending = store.pending_tasks(list).unwrap();
    assert_eq!(pending.len(), 2);
    assert!(pending.iter().all(|p| p.deleted));
    // A full pull doesn't bring them back before the deletes went out.
    store
        .sync_tasks(list, &[remote("P", "parent")], true)
        .unwrap();
    assert!(store.tasks_in(list).unwrap().is_empty());
    for p in pending {
        store.forget_task(p.task.id).unwrap();
    }
    assert!(store.pending_tasks(list).unwrap().is_empty());
}

#[test]
fn moving_between_lists_deletes_there_and_adds_here() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let from = gmail_list(&mut store, work);
    store
        .sync_tasks(from, &[remote("A", "file taxes")], true)
        .unwrap();
    let id = store.tasks_in(from).unwrap()[0].id;
    let to = store.add_task_list(Some(work), "Home").unwrap();
    assert!(store.move_task(id, to).unwrap());
    assert!(store.tasks_in(from).unwrap().is_empty());
    assert_eq!(titles(&store.tasks_in(to).unwrap()), ["file taxes"]);
    let tombstone = store.pending_tasks(from).unwrap().remove(0);
    assert!(tombstone.deleted);
    assert_eq!(tombstone.remote_id.as_deref(), Some("A"));
    let moved = store.pending_tasks(to).unwrap().remove(0);
    assert_eq!(moved.task.id, id);
    assert_eq!(moved.remote_id, None);
}

#[test]
fn lists_follow_the_service() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    store.sync_tasks(list, &[remote("A", "a")], true).unwrap();
    let home = store.add_task_list(Some(work), "Home").unwrap();
    let pending = store.pending_task_lists(work).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, home);
    store.task_list_pushed(home, "L2").unwrap();
    assert!(store.pending_task_lists(work).unwrap().is_empty());

    // Renamed on the phone; the first list deleted there.
    let lists = [RemoteTaskList {
        remote_id: "L2".into(),
        title: "Household".into(),
        is_default: false,
    }];
    assert!(store.sync_task_lists(work, &lists).unwrap());
    let titles: Vec<_> = store
        .task_lists()
        .unwrap()
        .into_iter()
        .filter(|l| l.account.is_some())
        .map(|l| l.title)
        .collect();
    assert_eq!(titles, ["Household"]);
    assert!(store.tasks_in(list).unwrap().is_empty());

    // Deleted here: hidden, then gone once the service deleted it.
    assert!(store.delete_task_list(home).unwrap());
    assert!(store.task_lists().unwrap().iter().all(|l| l.id != home));
    assert!(store.pending_task_lists(work).unwrap()[0].deleted);
    store.forget_task_list(home).unwrap();
    assert!(store.pending_task_lists(work).unwrap().is_empty());
}

#[test]
fn new_tasks_go_on_top_and_new_steps_at_the_end() {
    let (_dir, mut store) = store();
    let add = |store: &mut Store, parent: Option<i64>, title: &str| {
        let fields = TaskFields {
            title: title.into(),
            ..TaskFields::default()
        };
        store.add_task_to(1, parent, &fields).unwrap()
    };
    let trip = add(&mut store, None, "trip");
    add(&mut store, Some(trip), "trains");
    add(&mut store, Some(trip), "hotel");
    add(&mut store, None, "passport");
    assert_eq!(
        titles(&store.tasks_in(1).unwrap()),
        ["passport", "trip", "trains", "hotel"]
    );
}

#[test]
fn a_place_between_two_sorts_between_them() {
    let cases = [
        ("", Some("00000000000000000001")),
        ("00000000000000000001", Some("00000000000000000002")),
        ("00000000000000000001", Some("00000000000000000009")),
        ("00000000000000000009", None),
        ("", None),
        ("0999", Some("1")),
        ("15", Some("151")),
    ];
    for (low, high) in cases {
        let mid = between(low, high).unwrap();
        assert!(mid.as_str() > low, "{mid} after {low}");
        if let Some(high) = high {
            assert!(mid.as_str() < high, "{mid} before {high}");
        }
        assert!(!mid.ends_with('0'), "{mid}");
    }
    // Always room for another between.
    let mut high = "00000000000000000002".to_owned();
    for _ in 0..40 {
        let mid = between("00000000000000000001", Some(&high)).unwrap();
        assert!(mid.as_str() > "00000000000000000001" && mid < high);
        high = mid;
    }
    assert_eq!(between("5", Some("5")), None);
    assert_eq!(between("6", Some("5")), None);
    assert_eq!(between("", Some("00")), None);
}

fn add(store: &mut Store, list: i64, parent: Option<i64>, title: &str) -> i64 {
    let fields = TaskFields {
        title: title.into(),
        ..TaskFields::default()
    };
    store.add_task_to(list, parent, &fields).unwrap()
}

#[test]
fn dragging_puts_a_task_where_it_was_let_go() {
    let (_dir, mut store) = store();
    let c = add(&mut store, 1, None, "c");
    let step = add(&mut store, 1, Some(c), "c1");
    let b = add(&mut store, 1, None, "b");
    let a = add(&mut store, 1, None, "a");
    assert_eq!(titles(&store.tasks_in(1).unwrap()), ["a", "b", "c", "c1"]);
    // Down, after c (its step stays with c).
    assert!(store.place_task(a, 1, Some(c)).unwrap());
    assert_eq!(titles(&store.tasks_in(1).unwrap()), ["b", "c", "c1", "a"]);
    // Up to the top.
    assert!(store.place_task(c, 1, None).unwrap());
    assert_eq!(titles(&store.tasks_in(1).unwrap()), ["c", "c1", "b", "a"]);
    // A new task still goes on top.
    add(&mut store, 1, None, "new");
    assert_eq!(
        titles(&store.tasks_in(1).unwrap()),
        ["new", "c", "c1", "b", "a"]
    );
    // Steps are not dragged; a list on this computer sends nothing.
    assert!(!store.place_task(step, 1, None).unwrap());
    assert!(store.pending_tasks(1).unwrap().is_empty());

    // Into another list, between two of its tasks.
    let home = store.add_task_list(None, "Home").unwrap();
    let y = add(&mut store, home, None, "y");
    let x = add(&mut store, home, None, "x");
    assert!(store.place_task(b, home, Some(x)).unwrap());
    assert_eq!(titles(&store.tasks_in(home).unwrap()), ["x", "b", "y"]);
    assert_eq!(titles(&store.tasks_in(1).unwrap()), ["new", "c", "c1", "a"]);
    // Undo puts it back where it was.
    assert!(store.place_task(b, 1, Some(c)).unwrap());
    assert_eq!(
        titles(&store.tasks_in(1).unwrap()),
        ["new", "c", "c1", "b", "a"]
    );
    assert!(store.place_task(y, home, None).unwrap());
    assert_eq!(titles(&store.tasks_in(home).unwrap()), ["y", "x"]);
}

fn google_account(store: &mut Store, address: &str) -> AccountId {
    let id = account(store, address);
    let settings = katna_core::AccountSettings {
        oauth: Some(OAuthProvider::Google),
        ..katna_core::AccountSettings::default()
    };
    store.set_account_settings(id, &settings).unwrap();
    id
}

fn placed(id: &str, title: &str, position: &str) -> RemoteTask {
    RemoteTask {
        position: position.into(),
        ..remote(id, title)
    }
}

#[test]
fn a_task_dragged_in_a_google_list_moves_there_too() {
    let (_dir, mut store) = store();
    let work = google_account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    let pulled = [
        placed("A", "a", "00000000000000000001"),
        placed("B", "b", "00000000000000000002"),
        placed("C", "c", "00000000000000000003"),
    ];
    store.sync_tasks(list, &pulled, true).unwrap();
    let id = |store: &Store, title: &str| {
        store
            .tasks_in(list)
            .unwrap()
            .into_iter()
            .find(|t| t.title == title)
            .unwrap()
            .id
    };
    let (a, b, c) = (id(&store, "a"), id(&store, "b"), id(&store, "c"));
    assert!(store.place_task(c, list, Some(a)).unwrap());
    assert_eq!(titles(&store.tasks_in(list).unwrap()), ["a", "c", "b"]);
    // Only the dragged one changed, and only its place.
    let pending = store.pending_tasks(list).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].task.id, c);
    assert!(!pending[0].edited);
    assert_eq!(pending[0].place, Some(Place::After("A".into())));
    // Changed again before it went: both go.
    let fields = TaskFields {
        title: "see".into(),
        ..TaskFields::default()
    };
    store.edit_task(c, &fields).unwrap();
    let pending = store.pending_tasks(list).unwrap().remove(0);
    assert!(pending.edited);
    assert_eq!(pending.place, Some(Place::After("A".into())));
    // A pull meanwhile doesn't undo it.
    store.sync_tasks(list, &pulled, true).unwrap();
    assert_eq!(titles(&store.tasks_in(list).unwrap()), ["a", "see", "b"]);
    // Google's own position once it went.
    let moved = placed("C", "see", "000000000000000000015");
    store.task_pushed(c, pending.stamp, &moved, true).unwrap();
    assert!(store.pending_tasks(list).unwrap().is_empty());
    assert_eq!(store.task(c).unwrap().unwrap().position, moved.position);

    // First: no task before it there.
    assert!(store.place_task(b, list, None).unwrap());
    assert_eq!(titles(&store.tasks_in(list).unwrap()), ["b", "a", "see"]);
    let pending = store.pending_tasks(list).unwrap().remove(0);
    assert_eq!(pending.place, Some(Place::First));
    // Sent but not placed yet: it stays marked, where it is here.
    store
        .task_pushed(b, pending.stamp, &placed("B", "b", "9"), false)
        .unwrap();
    let pending = store.pending_tasks(list).unwrap().remove(0);
    assert_eq!(pending.place, Some(Place::First));
    assert!(!pending.edited);
    assert_eq!(titles(&store.tasks_in(list).unwrap()), ["b", "a", "see"]);

    // After a task not on Google yet: it waits for it.
    let new = add(&mut store, list, None, "new");
    assert!(store.place_task(a, list, Some(new)).unwrap());
    let waiting = store
        .pending_tasks(list)
        .unwrap()
        .into_iter()
        .find(|p| p.task.id == a)
        .unwrap();
    assert_eq!(waiting.place, Some(Place::Waiting));
}

#[test]
fn another_services_order_is_kept_here() {
    let (_dir, mut store) = store();
    // A CalDAV account: its server keeps no order.
    let dav = account(&mut store, "me@example.test");
    let list = gmail_list(&mut store, dav);
    store
        .sync_tasks(list, &[remote("A", "a"), remote("B", "b")], true)
        .unwrap();
    let first: Vec<String> = store
        .tasks_in(list)
        .unwrap()
        .into_iter()
        .map(|t| t.title)
        .collect();
    let last = store.tasks_in(list).unwrap()[1].id;
    assert!(store.place_task(last, list, None).unwrap());
    let flipped = vec![first[1].clone(), first[0].clone()];
    assert_eq!(titles(&store.tasks_in(list).unwrap()), flipped);
    assert!(store.pending_tasks(list).unwrap().is_empty());
    // Changed on the server, which gives no place: the order stays.
    let changed = [
        RemoteTask {
            etag: "new-a".into(),
            ..remote("A", "a")
        },
        RemoteTask {
            etag: "new-b".into(),
            ..remote("B", "b")
        },
    ];
    assert!(store.sync_tasks(list, &changed, true).unwrap());
    assert_eq!(titles(&store.tasks_in(list).unwrap()), flipped);
}

#[test]
fn labels_are_kept_and_shared_with_notes() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    let fields = TaskFields {
        title: "pay electricity".into(),
        labels: vec!["Home".into(), "Bills".into(), "Home".into(), " ".into()],
        ..TaskFields::default()
    };
    let id = store.add_task_to(list, None, &fields).unwrap();
    assert_eq!(store.task(id).unwrap().unwrap().labels, ["Home", "Bills"]);

    // A service without labels (None) leaves them; one with labels wins.
    let pending = store.pending_tasks(list).unwrap().remove(0);
    store
        .task_pushed(id, pending.stamp, &remote("A", "pay electricity"), true)
        .unwrap();
    let mut back = remote("A", "pay electricity");
    back.etag = "e2".into();
    store.sync_tasks(list, &[back.clone()], false).unwrap();
    assert_eq!(store.task(id).unwrap().unwrap().labels, ["Home", "Bills"]);
    back.etag = "e3".into();
    back.labels = Some(vec!["Work".into()]);
    back.starred = Some(true);
    store.sync_tasks(list, &[back], false).unwrap();
    let task = store.task(id).unwrap().unwrap();
    assert_eq!(task.labels, ["Work"]);
    assert!(task.starred);

    // Notes' labels and tasks' labels are one set.
    let note = crate::notes::Note {
        labels: vec!["home".into(), "Trip".into()],
        ..crate::notes::Note::default()
    };
    store.save_note(&note).unwrap();
    assert_eq!(store.labels_in_use().unwrap(), ["home", "Trip", "Work"]);
}

#[test]
fn files_go_to_the_service_or_stay_here() {
    let (_dir, mut store) = store();
    let work = account(&mut store, "work@example.test");
    let list = gmail_list(&mut store, work);
    store
        .sync_tasks(list, &[remote("A", "file taxes")], true)
        .unwrap();
    let id = store.tasks_in(list).unwrap()[0].id;
    let file = store
        .add_task_file(id, "/tmp/Form 16.pdf", "application/pdf", b"%PDF-1.7")
        .unwrap()
        .unwrap();
    let files = store.files_of_task(id).unwrap();
    assert_eq!(files[0].name, "Form 16.pdf");
    assert_eq!(files[0].size, 8);
    assert_eq!(store.task_file_data(file).unwrap().unwrap(), b"%PDF-1.7");

    // Waiting to go; the service took it.
    let pending = store.pending_task_files(list).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].task_remote, "A");
    store.task_file_pushed(file, Some("F1")).unwrap();
    assert!(store.pending_task_files(list).unwrap().is_empty());

    // The service lists it and one more, which Katna asks for.
    let theirs = RemoteFile {
        remote_id: "F2".into(),
        name: "receipt.png".into(),
        mime: "image/png".into(),
        size: 3,
        data: None,
    };
    let ours = RemoteFile {
        remote_id: "F1".into(),
        ..RemoteFile::default()
    };
    let synced = store
        .sync_task_files(list, "A", &[ours.clone(), theirs.clone()])
        .unwrap();
    assert_eq!(synced.wanted, [theirs.clone()]);
    let with_data = RemoteFile {
        data: Some(b"png".to_vec()),
        ..theirs
    };
    assert!(store.add_remote_task_file(list, "A", &with_data).unwrap());
    assert_eq!(store.files_of_task(id).unwrap().len(), 2);

    // Removed on the service: gone here.
    let synced = store.sync_task_files(list, "A", &[ours]).unwrap();
    assert!(synced.changed);
    assert_eq!(store.files_of_task(id).unwrap().len(), 1);

    // Removed here: a tombstone until the service removed it.
    assert!(store.remove_task_file(file).unwrap());
    assert!(store.files_of_task(id).unwrap().is_empty());
    let pending = store.pending_task_files(list).unwrap();
    assert!(pending[0].deleted);
    store.forget_task_file(file).unwrap();
    assert!(store.pending_task_files(list).unwrap().is_empty());

    // One the service can't keep stays here, and is no longer pending.
    let local = store
        .add_task_file(id, "big.zip", "", &[0; 16])
        .unwrap()
        .unwrap();
    store.task_file_pushed(local, None).unwrap();
    assert!(store.pending_task_files(list).unwrap().is_empty());
    let kept = store.task_file(local).unwrap().unwrap();
    assert!(kept.local_only);
    assert_eq!(kept.mime, "application/octet-stream");
    // The service's list doesn't drop it.
    store.sync_task_files(list, "A", &[]).unwrap();
    assert!(store.task_file(local).unwrap().is_some());

    // Moved to a list on this computer: it goes along, to send afresh.
    let here = store.add_task_list(None, "Mine").unwrap();
    store.move_task(id, here).unwrap();
    let moved = store.task_file(local).unwrap().unwrap();
    assert!(!moved.local_only);
    assert_eq!(moved.remote_id, None);
}
