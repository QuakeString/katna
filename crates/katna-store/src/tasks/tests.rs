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
        .task_pushed(id, pending.stamp, &remote("T1", "draft the memo"))
        .unwrap();
    let again = store.pending_tasks(list).unwrap();
    assert_eq!(again.len(), 1);
    assert_eq!(again[0].remote_id.as_deref(), Some("T1"));

    store
        .task_pushed(id, again[0].stamp, &remote("T1", "draft the memo today"))
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
        .task_pushed(id, pending.stamp, &remote("A", "pay rent"))
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
