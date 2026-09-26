// SPDX-License-Identifier: GPL-3.0-or-later

//! The level-1 sync engine against an in-memory mail server.

use std::{collections::BTreeMap, future::Future, time::Duration};

use katna_core::{AccountId, AccountKind, Paths};
use katna_store::{FolderRole as StoreRole, Mode, Store};
use katna_sync::{
    Envelope, FlagState, Flags, Folder, FolderChange, FolderRole, FolderStatus, MailBackend,
    MessageHeaders, Result, Wait,
    engine::{self, CHUNK, FolderReport},
};

#[derive(Clone, Debug)]
struct Message {
    flags: Flags,
    modseq: u64,
    header: Vec<u8>,
}

#[derive(Clone, Debug)]
struct Mailbox {
    uid_validity: u32,
    uid_next: u32,
    messages: BTreeMap<u32, Message>,
}

/// A tiny IMAP-like server: folders of messages with UIDs, flags and a
/// global mod-sequence (CONDSTORE).
#[derive(Default)]
struct FakeServer {
    folders: BTreeMap<String, Mailbox>,
    modseq: u64,
    selected: Option<String>,
    /// Commands seen, for checking what the engine asked for.
    log: Vec<String>,
}

impl FakeServer {
    fn create(&mut self, name: &str, uid_validity: u32) {
        self.folders.insert(
            name.to_owned(),
            Mailbox {
                uid_validity,
                uid_next: 1,
                messages: BTreeMap::new(),
            },
        );
    }

    fn deliver(&mut self, folder: &str, subject: &str) -> u32 {
        self.modseq += 1;
        let modseq = self.modseq;
        let mailbox = self.folders.get_mut(folder).unwrap();
        let uid = mailbox.uid_next;
        mailbox.uid_next += 1;
        let header = format!(
            "From: Bob <Bob@Example.org>\r\nTo: alice@example.org\r\nSubject: {subject}\r\n\
             Date: Sat, 26 Sep 2026 10:00:00 +0000\r\nMessage-ID: <{subject}@example.org>\r\n\r\n"
        );
        mailbox.messages.insert(
            uid,
            Message {
                flags: Flags::default(),
                modseq,
                header: header.into_bytes(),
            },
        );
        uid
    }

    fn set_seen(&mut self, folder: &str, uid: u32) {
        self.modseq += 1;
        let message = self
            .folders
            .get_mut(folder)
            .unwrap()
            .messages
            .get_mut(&uid)
            .unwrap();
        message.flags.seen = true;
        message.modseq = self.modseq;
    }

    fn expunge(&mut self, folder: &str, uid: u32) {
        self.modseq += 1;
        self.folders.get_mut(folder).unwrap().messages.remove(&uid);
    }

    fn mailbox(&self) -> &Mailbox {
        &self.folders[self.selected.as_ref().unwrap()]
    }

    fn range(&self, first: u32, last: Option<u32>) -> impl Iterator<Item = (&u32, &Message)> {
        self.mailbox()
            .messages
            .range(first..=last.unwrap_or(u32::MAX))
    }
}

impl MailBackend for FakeServer {
    async fn list_folders(&mut self) -> Result<Vec<Folder>> {
        Ok(self
            .folders
            .keys()
            .map(|name| Folder {
                name: name.clone(),
                delimiter: Some('/'),
                role: (name == "INBOX").then_some(FolderRole::Inbox),
                selectable: true,
            })
            .collect())
    }

    async fn select(&mut self, folder: &str) -> Result<FolderStatus> {
        self.log.push(format!("SELECT {folder}"));
        self.selected = Some(folder.to_owned());
        let mailbox = self.mailbox();
        Ok(FolderStatus {
            exists: mailbox.messages.len() as u32,
            uid_validity: Some(mailbox.uid_validity),
            uid_next: Some(mailbox.uid_next),
            highest_modseq: Some(self.modseq),
        })
    }

    async fn fetch_envelopes(&mut self, _: u32, _: Option<u32>) -> Result<Vec<Envelope>> {
        unimplemented!()
    }

    async fn fetch_headers(
        &mut self,
        first: u32,
        last: Option<u32>,
    ) -> Result<Vec<MessageHeaders>> {
        self.log.push(format!("HEADERS {first}:{last:?}"));
        Ok(self
            .range(first, last)
            .map(|(uid, m)| MessageHeaders {
                uid: *uid,
                size: 1000,
                flags: m.flags.clone(),
                received: None,
                header: m.header.clone(),
            })
            .collect())
    }

    async fn fetch_flags(
        &mut self,
        first: u32,
        last: u32,
        changed_since: Option<u64>,
    ) -> Result<Vec<FlagState>> {
        self.log
            .push(format!("FLAGS {first}:{last} since {changed_since:?}"));
        let since = changed_since.unwrap_or(0);
        Ok(self
            .range(first, Some(last))
            .filter(|(_, m)| m.modseq > since)
            .map(|(uid, m)| FlagState {
                uid: *uid,
                flags: m.flags.clone(),
            })
            .collect())
    }

    async fn uids(&mut self) -> Result<Vec<u32>> {
        self.log.push("UIDS".into());
        Ok(self.mailbox().messages.keys().copied().collect())
    }

    async fn create_folder(&mut self, _: &str) -> Result<()> {
        unimplemented!()
    }

    async fn append(&mut self, _: &str, _: Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    async fn poll_changes(&mut self) -> Result<Vec<FolderChange>> {
        unimplemented!()
    }

    async fn wait_for_changes<I>(&mut self, _: Duration, _: I) -> Result<Wait<I::Output>>
    where
        I: Future + Send,
        I::Output: Send,
    {
        unimplemented!()
    }

    async fn logout(self) -> Result<()> {
        Ok(())
    }
}

fn setup() -> (tempfile::TempDir, Store, AccountId) {
    let tmp = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Test", "alice@example.org")
        .unwrap()
        .id;
    (tmp, store, account)
}

fn sync(server: &mut FakeServer, store: &mut Store, account: AccountId) -> Vec<FolderReport> {
    server.log.clear();
    smol::block_on(engine::sync_account(server, store, account)).unwrap()
}

fn report(path: &str, added: usize, flags_changed: usize, removed: usize) -> FolderReport {
    FolderReport {
        path: path.into(),
        added,
        flags_changed,
        removed,
        reset: false,
    }
}

#[test]
fn first_sync_then_incremental_changes() {
    let (_tmp, mut store, account) = setup();
    let mut server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    for i in 0..3 {
        server.deliver("INBOX", &format!("hello-{i}"));
    }

    let reports = sync(&mut server, &mut store, account);
    assert_eq!(
        reports,
        vec![report("Archive", 0, 0, 0), report("INBOX", 3, 0, 0)]
    );
    let folders = store.folders(account).unwrap();
    let inbox = folders.iter().find(|f| f.path == "INBOX").unwrap();
    assert_eq!(inbox.role, Some(StoreRole::Inbox));
    assert_eq!(inbox.uidvalidity, Some(1));
    assert_eq!(inbox.highestmodseq, Some(3));
    assert_eq!(store.folder_uids(inbox.id).unwrap(), vec![1, 2, 3]);

    // Nothing changed: no flag fetch, no header fetch past UIDNEXT.
    let reports = sync(&mut server, &mut store, account);
    assert_eq!(reports[1], report("INBOX", 0, 0, 0));
    assert!(
        !server
            .log
            .iter()
            .any(|c| c.starts_with("FLAGS") || c.starts_with("HEADERS")),
        "{:?}",
        server.log
    );

    // One new, one read, one expunged.
    server.deliver("INBOX", "hello-3");
    server.set_seen("INBOX", 1);
    server.expunge("INBOX", 2);
    let reports = sync(&mut server, &mut store, account);
    assert_eq!(reports[1], report("INBOX", 1, 1, 1));
    assert!(
        server.log.contains(&"FLAGS 1:3 since Some(3)".to_owned()),
        "{:?}",
        server.log
    );
    assert_eq!(store.folder_uids(inbox.id).unwrap(), vec![1, 3, 4]);
    let message = store
        .messages_in_folder(inbox.id)
        .unwrap()
        .into_iter()
        .find(|m| m.subject == "hello-0")
        .unwrap();
    assert!(message.flags.contains(katna_store::MessageFlags::SEEN));
    let from = message.first(katna_store::ParticipantRole::From).unwrap();
    assert_eq!(from.email_norm, "bob@example.org");
    assert_eq!(from.display_name.as_deref(), Some("Bob"));
}

#[test]
fn big_folder_is_fetched_in_chunks() {
    let (_tmp, mut store, account) = setup();
    let mut server = FakeServer::default();
    server.create("INBOX", 1);
    let total = CHUNK * 2 + 10;
    for i in 0..total {
        server.deliver("INBOX", &format!("m{i}"));
    }
    let reports = sync(&mut server, &mut store, account);
    assert_eq!(reports[0].added, total as usize);
    let fetches: Vec<_> = server
        .log
        .iter()
        .filter(|c| c.starts_with("HEADERS"))
        .collect();
    assert_eq!(
        fetches,
        [
            &format!("HEADERS 1:Some({CHUNK})"),
            &format!("HEADERS {}:Some({})", CHUNK + 1, CHUNK * 2),
            &format!("HEADERS {}:Some({total})", CHUNK * 2 + 1),
        ]
    );
}

#[test]
fn changed_uidvalidity_downloads_the_folder_again() {
    let (_tmp, mut store, account) = setup();
    let mut server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver("INBOX", "old-1");
    server.deliver("INBOX", "old-2");
    sync(&mut server, &mut store, account);

    // The server rebuilt the folder: new UIDVALIDITY, UIDs start again.
    server.create("INBOX", 2);
    server.deliver("INBOX", "new-1");
    let reports = sync(&mut server, &mut store, account);
    assert_eq!(
        reports[0],
        FolderReport {
            reset: true,
            ..report("INBOX", 1, 0, 0)
        }
    );
    let inbox = &store.folders(account).unwrap()[0];
    assert_eq!(inbox.uidvalidity, Some(2));
    let subjects: Vec<_> = store
        .messages_in_folder(inbox.id)
        .unwrap()
        .into_iter()
        .map(|m| m.subject)
        .collect();
    assert_eq!(subjects, ["new-1"]);
}

#[test]
fn deleted_folder_is_removed() {
    let (_tmp, mut store, account) = setup();
    let mut server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Old", 1);
    server.deliver("Old", "bye");
    sync(&mut server, &mut store, account);
    assert_eq!(store.folders(account).unwrap().len(), 2);

    server.folders.remove("Old");
    sync(&mut server, &mut store, account);
    let folders = store.folders(account).unwrap();
    assert_eq!(folders.len(), 1);
    assert_eq!(folders[0].path, "INBOX");
}
