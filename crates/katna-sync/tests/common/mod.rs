// SPDX-License-Identifier: GPL-3.0-or-later

//! An in-memory, IMAP-like mail server for testing the sync engine and the
//! account worker without a network.

#![allow(dead_code)] // each test file uses a different part

use std::{
    collections::BTreeMap,
    future::Future,
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};

use async_io::Timer;
use futures_lite::FutureExt;
use katna_core::{AccountId, AccountKind, Paths};
use katna_store::{Mode, Store};
use katna_sync::{
    AttachmentPart, Envelope, Error, FlagChanges, FlagState, Flags, Folder, FolderChange,
    FolderRole, FolderStatus, MailBackend, MessageHeaders, Result, Wait, worker::Connector,
};

#[derive(Clone, Debug)]
pub struct Message {
    pub flags: Flags,
    pub modseq: u64,
    pub header: Vec<u8>,
    pub body: Vec<u8>,
    /// Gmail only: `X-GM-THRID`.
    pub gm_thread_id: Option<u64>,
    /// Gmail only: `X-GM-MSGID`, shared by the message's copies in other
    /// folders (labels).
    pub gm_msgid: Option<u64>,
    /// Gmail only: `promotions`, `social`, … (`None` for Primary).
    pub gmail_category: Option<String>,
    /// What `BODYSTRUCTURE` would name; `None` as if the server sent none.
    pub attachments: Option<Vec<AttachmentPart>>,
}

#[derive(Clone, Debug)]
pub struct Mailbox {
    pub uid_validity: u32,
    pub uid_next: u32,
    pub messages: BTreeMap<u32, Message>,
    /// Expunged UIDs and the mod-sequence of their expunge (QRESYNC).
    pub vanished: Vec<(u32, u64)>,
}

/// Server state: folders of messages with UIDs, flags and one global
/// mod-sequence (CONDSTORE).
#[derive(Default)]
pub struct State {
    pub folders: BTreeMap<String, Mailbox>,
    pub modseq: u64,
    /// Commands seen, for checking what the client asked for.
    pub log: Vec<String>,
    /// Connections opened so far.
    pub connects: u32,
    /// Refuse this many connection attempts before accepting.
    pub refuse: u32,
    /// Refuse logins with a wrong password.
    pub wrong_password: bool,
    /// Bumped to break every open connection.
    pub generation: u32,
    /// Report new UIDs on MOVE (UIDPLUS).
    pub no_uidplus: bool,
    /// Refuse (NO) flag changes, moves and expunges.
    pub refuse_changes: bool,
    /// Offer Gmail's thread IDs and `X-GM-RAW` search.
    pub gmail: bool,
    /// Report expunges with flag fetches (QRESYNC `VANISHED (EARLIER)`).
    pub qresync: bool,
}

/// A shared server; clones see the same state.
#[derive(Clone, Default)]
pub struct FakeServer(Arc<Mutex<State>>);

impl FakeServer {
    pub fn state(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap()
    }

    pub fn create(&self, name: &str, uid_validity: u32) {
        self.state().folders.insert(
            name.to_owned(),
            Mailbox {
                uid_validity,
                uid_next: 1,
                messages: BTreeMap::new(),
                vanished: Vec::new(),
            },
        );
    }

    pub fn remove_folder(&self, name: &str) {
        self.state().folders.remove(name);
    }

    pub fn deliver(&self, folder: &str, subject: &str) -> u32 {
        let header = format!(
            "From: Bob <Bob@Example.org>\r\nTo: alice@example.org\r\nSubject: {subject}\r\n\
             Date: Sat, 26 Sep 2026 10:00:00 +0000\r\nMessage-ID: <{subject}@example.org>\r\n\r\n"
        );
        self.deliver_header(folder, &header, &format!("Body of {subject}.\r\n"))
    }

    /// Delivers a message with this exact header.
    pub fn deliver_header(&self, folder: &str, header: &str, body: &str) -> u32 {
        let mut state = self.state();
        state.modseq += 1;
        let modseq = state.modseq;
        let mailbox = state.folders.get_mut(folder).unwrap();
        let uid = mailbox.uid_next;
        mailbox.uid_next += 1;
        mailbox.messages.insert(
            uid,
            Message {
                flags: Flags::default(),
                modseq,
                header: header.as_bytes().to_vec(),
                body: body.as_bytes().to_vec(),
                gm_thread_id: None,
                gm_msgid: None,
                gmail_category: None,
                attachments: None,
            },
        );
        uid
    }

    /// Gives the message at `uid` in `folder` the attachments its
    /// structure names (`Some(vec![])`: a structure without any).
    pub fn set_attachments(&self, folder: &str, uid: u32, parts: Option<Vec<AttachmentPart>>) {
        let mut state = self.state();
        let message = state
            .folders
            .get_mut(folder)
            .unwrap()
            .messages
            .get_mut(&uid)
            .unwrap();
        message.attachments = parts;
    }

    /// Makes the server Gmail-like: thread IDs and `X-GM-RAW` search.
    pub fn set_gmail(&self, on: bool) {
        self.state().gmail = on;
    }

    /// Gives the message at `uid` in `folder` a Gmail message ID and shows
    /// it in `also` too, as Gmail does for a label. Returns its UID there.
    pub fn label(&self, folder: &str, uid: u32, gm_msgid: u64, also: &str) -> u32 {
        let mut state = self.state();
        state.modseq += 1;
        let modseq = state.modseq;
        let message = {
            let message = state
                .folders
                .get_mut(folder)
                .unwrap()
                .messages
                .get_mut(&uid)
                .unwrap();
            message.gm_msgid = Some(gm_msgid);
            message.clone()
        };
        let mailbox = state.folders.get_mut(also).unwrap();
        let new = mailbox.uid_next;
        mailbox.uid_next += 1;
        mailbox.messages.insert(new, Message { modseq, ..message });
        new
    }

    /// Sets Gmail's thread ID and category (`promotions`, …) of a message.
    pub fn set_gmail_labels(
        &self,
        folder: &str,
        uid: u32,
        thread: Option<u64>,
        category: Option<&str>,
    ) {
        let mut state = self.state();
        let message = state
            .folders
            .get_mut(folder)
            .unwrap()
            .messages
            .get_mut(&uid)
            .unwrap();
        message.gm_thread_id = thread;
        message.gmail_category = category.map(str::to_owned);
    }

    pub fn set_seen(&self, folder: &str, uid: u32) {
        let mut state = self.state();
        state.modseq += 1;
        let modseq = state.modseq;
        let message = state
            .folders
            .get_mut(folder)
            .unwrap()
            .messages
            .get_mut(&uid)
            .unwrap();
        message.flags.seen = true;
        message.modseq = modseq;
    }

    /// The flags of message `uid` in `folder`.
    pub fn flags(&self, folder: &str, uid: u32) -> Flags {
        self.state().folders[folder].messages[&uid].flags.clone()
    }

    /// The UIDs in `folder`.
    pub fn uids(&self, folder: &str) -> Vec<u32> {
        self.state().folders[folder]
            .messages
            .keys()
            .copied()
            .collect()
    }

    pub fn expunge(&self, folder: &str, uid: u32) {
        let mut state = self.state();
        state.modseq += 1;
        let modseq = state.modseq;
        let mailbox = state.folders.get_mut(folder).unwrap();
        if mailbox.messages.remove(&uid).is_some() {
            mailbox.vanished.push((uid, modseq));
        }
    }

    /// Drops every open connection, as a network change would.
    pub fn break_connections(&self) {
        self.state().generation += 1;
    }

    pub fn log(&self) -> Vec<String> {
        self.state().log.clone()
    }

    pub fn clear_log(&self) {
        self.state().log.clear();
    }

    /// A new connection (no failure injection).
    pub fn connection(&self) -> FakeConnection {
        let generation = self.state().generation;
        FakeConnection {
            server: self.clone(),
            selected: None,
            seen_modseq: 0,
            generation,
        }
    }
}

impl Connector for FakeServer {
    type Backend = FakeConnection;

    async fn connect(&self) -> Result<FakeConnection> {
        {
            let mut state = self.state();
            state.connects += 1;
            if state.wrong_password {
                return Err(Error::Auth("wrong password".into()));
            }
            if state.refuse > 0 {
                state.refuse -= 1;
                return Err(Error::Io(std::io::ErrorKind::ConnectionRefused.into()));
            }
        }
        Ok(self.connection())
    }
}

/// One connection to a [`FakeServer`].
pub struct FakeConnection {
    server: FakeServer,
    selected: Option<String>,
    /// The server state last reported to the client: changes after it
    /// end the next IDLE at once, as a real server's untagged responses do.
    seen_modseq: u64,
    generation: u32,
}

impl FakeConnection {
    /// Locks the state, failing if the connection was broken.
    fn guard(&self) -> Result<MutexGuard<'_, State>> {
        let state = self.server.state();
        if state.generation != self.generation {
            return Err(Error::Closed("connection reset".into()));
        }
        Ok(state)
    }

    /// Like [`Self::guard`], and logs the command.
    fn state(&self, command: String) -> Result<MutexGuard<'_, State>> {
        let mut state = self.guard()?;
        state.log.push(command);
        Ok(state)
    }

    fn selected(&self) -> &str {
        self.selected.as_deref().expect("a folder is selected")
    }
}

fn range(
    mailbox: &Mailbox,
    first: u32,
    last: Option<u32>,
) -> impl Iterator<Item = (&u32, &Message)> {
    mailbox.messages.range(first..=last.unwrap_or(u32::MAX))
}

impl MailBackend for FakeConnection {
    async fn list_folders(&mut self) -> Result<Vec<Folder>> {
        let state = self.state("LIST".into())?;
        Ok(state
            .folders
            .keys()
            .map(|name| Folder {
                name: name.clone(),
                delimiter: Some('/'),
                role: match name.as_str() {
                    "INBOX" => Some(FolderRole::Inbox),
                    "Trash" => Some(FolderRole::Trash),
                    "Archive" => Some(FolderRole::Archive),
                    "Sent" => Some(FolderRole::Sent),
                    "[Gmail]/All Mail" => Some(FolderRole::All),
                    _ => None,
                },
                selectable: true,
            })
            .collect())
    }

    async fn select(&mut self, folder: &str) -> Result<FolderStatus> {
        let status = {
            let state = self.state(format!("SELECT {folder}"))?;
            let Some(mailbox) = state.folders.get(folder) else {
                return Err(Error::Rejected(format!("no folder {folder}")));
            };
            FolderStatus {
                exists: mailbox.messages.len() as u32,
                uid_validity: Some(mailbox.uid_validity),
                uid_next: Some(mailbox.uid_next),
                highest_modseq: Some(state.modseq),
            }
        };
        self.seen_modseq = status.highest_modseq.unwrap();
        self.selected = Some(folder.to_owned());
        Ok(status)
    }

    async fn status(&mut self, folder: &str) -> Result<FolderStatus> {
        let state = self.state(format!("STATUS {folder}"))?;
        let Some(mailbox) = state.folders.get(folder) else {
            return Err(Error::Rejected(format!("no folder {folder}")));
        };
        Ok(FolderStatus {
            exists: mailbox.messages.len() as u32,
            uid_validity: Some(mailbox.uid_validity),
            uid_next: Some(mailbox.uid_next),
            highest_modseq: Some(state.modseq),
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
        let state = self.state(format!("HEADERS {first}:{last:?}"))?;
        let gmail = state.gmail;
        Ok(range(&state.folders[self.selected()], first, last)
            .map(|(uid, m)| MessageHeaders {
                uid: *uid,
                size: 1000,
                flags: m.flags.clone(),
                received: None,
                header: m.header.clone(),
                gm_thread_id: m.gm_thread_id.filter(|_| gmail),
                gm_msgid: m.gm_msgid.filter(|_| gmail),
                attachments: m.attachments.clone(),
            })
            .collect())
    }

    async fn gmail_search(&mut self, first: u32, query: &str) -> Result<Option<Vec<u32>>> {
        let state = self.state(format!("GMAIL {first}: {query}"))?;
        if !state.gmail {
            return Ok(None);
        }
        let category = query.strip_prefix("category:").expect("a category search");
        Ok(Some(
            range(&state.folders[self.selected()], first, None)
                .filter(|(_, m)| m.gmail_category.as_deref() == Some(category))
                .map(|(uid, _)| *uid)
                .collect(),
        ))
    }

    async fn fetch_bodies(&mut self, uids: &[u32]) -> Result<Vec<(u32, Vec<u8>)>> {
        let state = self.state(format!("BODIES {uids:?}"))?;
        let messages = &state.folders[self.selected()].messages;
        let mut sorted = uids.to_vec();
        sorted.sort_unstable();
        Ok(sorted
            .into_iter()
            .filter_map(|uid| {
                let message = messages.get(&uid)?;
                Some((uid, [message.header.as_slice(), &message.body].concat()))
            })
            .collect())
    }

    async fn fetch_flags(
        &mut self,
        first: u32,
        last: u32,
        changed_since: Option<u64>,
    ) -> Result<FlagChanges> {
        let state = self.state(format!("FLAGS {first}:{last} since {changed_since:?}"))?;
        let since = changed_since.unwrap_or(0);
        let mailbox = &state.folders[self.selected()];
        let flags = range(mailbox, first, Some(last))
            .filter(|(_, m)| m.modseq > since)
            .map(|(uid, m)| FlagState {
                uid: *uid,
                flags: m.flags.clone(),
            })
            .collect();
        let vanished = (state.qresync && changed_since.is_some()).then(|| {
            mailbox
                .vanished
                .iter()
                .filter(|(uid, modseq)| *modseq > since && (first..=last).contains(uid))
                .map(|(uid, _)| *uid..=*uid)
                .collect()
        });
        Ok(FlagChanges { flags, vanished })
    }

    async fn uids(&mut self) -> Result<Vec<u32>> {
        let state = self.state("UIDS".into())?;
        Ok(state.folders[self.selected()]
            .messages
            .keys()
            .copied()
            .collect())
    }

    async fn store_flags(&mut self, uids: &[u32], flags: &Flags, add: bool) -> Result<()> {
        let mut state = self.state(format!(
            "STORE {uids:?} {}{}",
            if add { "+" } else { "-" },
            flag_names(flags)
        ))?;
        if state.refuse_changes {
            return Err(Error::Rejected("NO not today".into()));
        }
        state.modseq += 1;
        let modseq = state.modseq;
        let folder = self.selected().to_owned();
        let messages = &mut state.folders.get_mut(&folder).unwrap().messages;
        for uid in uids {
            let Some(message) = messages.get_mut(uid) else {
                continue;
            };
            let f = &mut message.flags;
            for (on, flag) in [
                (flags.seen, &mut f.seen),
                (flags.answered, &mut f.answered),
                (flags.flagged, &mut f.flagged),
                (flags.deleted, &mut f.deleted),
                (flags.draft, &mut f.draft),
            ] {
                if on {
                    *flag = add;
                }
            }
            for keyword in &flags.keywords {
                f.keywords.retain(|k| k != keyword);
                if add {
                    f.keywords.push(keyword.clone());
                }
            }
            message.modseq = modseq;
        }
        Ok(())
    }

    async fn move_messages(&mut self, uids: &[u32], folder: &str) -> Result<Vec<(u32, u32)>> {
        let mut state = self.state(format!("MOVE {uids:?} {folder}"))?;
        if state.refuse_changes {
            return Err(Error::Rejected("NO not today".into()));
        }
        if !state.folders.contains_key(folder) {
            return Err(Error::Rejected(format!("NO no folder {folder}")));
        }
        state.modseq += 1;
        let modseq = state.modseq;
        let from = self.selected().to_owned();
        let mut moved = Vec::new();
        for uid in uids {
            let source = state.folders.get_mut(&from).unwrap();
            let Some(mut message) = source.messages.remove(uid) else {
                continue;
            };
            source.vanished.push((*uid, modseq));
            message.modseq = modseq;
            let target = state.folders.get_mut(folder).unwrap();
            let new = target.uid_next;
            target.uid_next += 1;
            target.messages.insert(new, message);
            moved.push((*uid, new));
        }
        if state.no_uidplus {
            moved.clear();
        }
        Ok(moved)
    }

    async fn expunge(&mut self, uids: &[u32]) -> Result<()> {
        let mut state = self.state(format!("EXPUNGE {uids:?}"))?;
        if state.refuse_changes {
            return Err(Error::Rejected("NO not today".into()));
        }
        state.modseq += 1;
        let modseq = state.modseq;
        let folder = self.selected().to_owned();
        let mailbox = state.folders.get_mut(&folder).unwrap();
        for uid in uids {
            if mailbox.messages.remove(uid).is_some() {
                mailbox.vanished.push((*uid, modseq));
            }
        }
        Ok(())
    }

    async fn create_folder(&mut self, _: &str) -> Result<()> {
        unimplemented!()
    }

    async fn append_with_flags(
        &mut self,
        folder: &str,
        message: Vec<u8>,
        flags: &Flags,
    ) -> Result<()> {
        let mut state = self.state(format!("APPEND {folder} {}", flag_names(flags)))?;
        state.modseq += 1;
        let modseq = state.modseq;
        let Some(mailbox) = state.folders.get_mut(folder) else {
            return Err(Error::Rejected(format!("no folder {folder}")));
        };
        let split = message
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .map_or(message.len(), |at| at + 4);
        let uid = mailbox.uid_next;
        mailbox.uid_next += 1;
        mailbox.messages.insert(
            uid,
            Message {
                flags: flags.clone(),
                modseq,
                header: message[..split].to_vec(),
                body: message[split..].to_vec(),
                gm_thread_id: None,
                gm_msgid: None,
                gmail_category: None,
                attachments: None,
            },
        );
        Ok(())
    }

    async fn poll_changes(&mut self) -> Result<Vec<FolderChange>> {
        unimplemented!()
    }

    /// Checks every 5 ms whether the mod-sequence moved.
    async fn wait_for_changes<I>(
        &mut self,
        max_wait: Duration,
        interrupt: I,
    ) -> Result<Wait<I::Output>>
    where
        I: Future + Send,
        I::Output: Send,
    {
        drop(self.state("IDLE".into())?);
        let start = self.seen_modseq;
        let deadline = Instant::now() + max_wait;
        let watch = async {
            loop {
                {
                    let state = self.guard()?;
                    if state.modseq != start {
                        let exists = state.folders[self.selected()].messages.len() as u32;
                        return Ok((
                            Some(state.modseq),
                            Wait {
                                changes: vec![FolderChange::Exists(exists)],
                                interrupted: None,
                            },
                        ));
                    }
                }
                if Instant::now() >= deadline {
                    return Ok((
                        None,
                        Wait {
                            changes: Vec::new(),
                            interrupted: None,
                        },
                    ));
                }
                Timer::after(Duration::from_millis(5)).await;
            }
        };
        let stop = async {
            Ok::<_, Error>((
                None,
                Wait {
                    changes: Vec::new(),
                    interrupted: Some(interrupt.await),
                },
            ))
        };
        let (seen, wait): (Option<u64>, _) = watch.or(stop).await?;
        if let Some(seen) = seen {
            self.seen_modseq = seen;
        }
        Ok(wait)
    }

    async fn logout(self) -> Result<()> {
        self.server.state().log.push("LOGOUT".into());
        Ok(())
    }
}

fn flag_names(flags: &Flags) -> String {
    let mut names: Vec<String> = [
        (flags.seen, "\\Seen"),
        (flags.answered, "\\Answered"),
        (flags.flagged, "\\Flagged"),
        (flags.deleted, "\\Deleted"),
        (flags.draft, "\\Draft"),
    ]
    .into_iter()
    .filter(|(on, _)| *on)
    .map(|(_, name)| name.to_owned())
    .collect();
    names.extend(flags.keywords.iter().cloned());
    names.join(" ")
}

/// A fresh store with one IMAP account.
pub fn store() -> (tempfile::TempDir, Store, AccountId) {
    let tmp = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Test", "alice@example.org")
        .unwrap()
        .id;
    (tmp, store, account)
}
