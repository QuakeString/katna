// SPDX-License-Identifier: GPL-3.0-or-later

//! Conversations (`docs/ARCHITECTURE.md` §6.5).
//!
//! A message joins the thread of any Message-ID it has or refers to
//! (`thread_key`); when it links two threads they merge. This gives the
//! same threads whatever order messages arrive in, and a reply finds its
//! thread even when its parent was never downloaded. The reply tree inside
//! a thread is worked out when shown ([`Store::thread_tree`]).
//!
//! The thread table's `last_date`, `message_count` and `flags_summary` are
//! not kept up to date; [`Store::folder_threads`] computes them.

use katna_core::AccountId;
use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::blob::BlobHash;
use crate::error::Result;
use crate::jwz::{self, Item};
use crate::mail::{FolderId, MailBatch, MessageFlags, MessageId};

pub use crate::jwz::ThreadNode;

/// Row ID of a thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ThreadId(pub i64);

/// A message not yet in a thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unthreaded {
    pub id: MessageId,
    pub account: AccountId,
    /// The ancestors if known from sync; otherwise read them from the blob.
    pub refs: Option<Vec<String>>,
    pub blob_hash: Option<BlobHash>,
}

/// One conversation in a folder's list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadSummary {
    /// `None` for a message the threader has not reached yet; it stands
    /// alone until then.
    pub thread: Option<ThreadId>,
    /// Subject of the first message.
    pub subject: String,
    /// Date of the newest message in the folder (Unix seconds, 0 if none).
    pub last_date: i64,
    /// Messages in the thread, in any folder.
    pub messages: u32,
    pub unread: u32,
    /// The newest message in the thread.
    pub latest: MessageId,
}

/// A thread's message as the reply tree needs it.
struct TreeRow {
    id: MessageId,
    message_id: Option<String>,
    refs: Vec<String>,
    date: Option<i64>,
}

fn split_refs(text: &str) -> Vec<String> {
    text.split(' ')
        .filter(|r| !r.is_empty())
        .map(str::to_owned)
        .collect()
}

impl Store {
    /// Messages in a folder that have no thread yet, oldest first.
    pub fn unthreaded(&self, limit: u32) -> Result<Vec<Unthreaded>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT m.id, m.refs, m.blob_hash, m.account_id FROM message m
             WHERE m.thread_id IS NULL
               AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = m.id)
             ORDER BY m.id LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], |row| {
            let hash: Option<Vec<u8>> = row.get(2)?;
            Ok(Unthreaded {
                id: MessageId(row.get(0)?),
                account: AccountId(row.get(3)?),
                refs: row.get::<_, Option<String>>(1)?.map(|r| split_refs(&r)),
                blob_hash: hash.and_then(|h| {
                    <[u8; 32]>::try_from(h.as_slice())
                        .ok()
                        .map(BlobHash::from_bytes)
                }),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The thread of `message`.
    pub fn thread_of(&self, message: MessageId) -> Result<Option<ThreadId>> {
        Ok(self
            .mail
            .prepare_cached("SELECT thread_id FROM message WHERE id = ?1")?
            .query_row([message.0], |row| row.get::<_, Option<i64>>(0))
            .optional()?
            .flatten()
            .map(ThreadId))
    }

    /// The conversations with messages in `folder`, newest first.
    pub fn folder_threads(&self, folder: FolderId, limit: u32) -> Result<Vec<ThreadSummary>> {
        // Unthreaded messages group alone under their negated ID.
        let groups: Vec<(i64, i64)> = {
            let mut stmt = self.mail.prepare_cached(
                "SELECT coalesce(m.thread_id, -m.id) AS grp, max(coalesce(m.date, 0)) AS last
                 FROM message_location l JOIN message m ON m.id = l.message_id
                 WHERE l.folder_id = ?1
                 GROUP BY grp ORDER BY last DESC, grp DESC LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![folder.0, limit], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        let seen = MessageFlags::SEEN.bits();
        let mut summaries = Vec::with_capacity(groups.len());
        for (group, last_date) in groups {
            let (thread, filter) = if group > 0 {
                (Some(ThreadId(group)), "m.thread_id = ?1")
            } else {
                (None, "m.id = -?1")
            };
            let sql = format!(
                "SELECT count(*), coalesce(sum((m.flags & ?2) = 0), 0),
                        (SELECT m.id FROM message m
                          WHERE {filter} ORDER BY coalesce(m.date, 0) DESC, m.id DESC LIMIT 1),
                        (SELECT m.subject FROM message m
                          WHERE {filter} ORDER BY coalesce(m.date, 0), m.id LIMIT 1)
                 FROM message m
                 WHERE {filter}
                   AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = m.id)"
            );
            let (messages, unread, latest, subject) =
                self.mail
                    .prepare_cached(&sql)?
                    .query_row(params![group, seen], |row| {
                        Ok((
                            row.get::<_, u32>(0)?,
                            row.get::<_, u32>(1)?,
                            row.get::<_, Option<i64>>(2)?,
                            row.get::<_, Option<String>>(3)?,
                        ))
                    })?;
            let Some(latest) = latest else { continue };
            summaries.push(ThreadSummary {
                thread,
                subject: subject.unwrap_or_default(),
                last_date,
                messages,
                unread,
                latest: MessageId(latest),
            });
        }
        Ok(summaries)
    }

    /// The messages of `thread` in reply order, with their depth.
    pub fn thread_tree(&self, thread: ThreadId) -> Result<Vec<ThreadNode>> {
        let rows: Vec<TreeRow> = {
            let mut stmt = self.mail.prepare_cached(
                "SELECT m.id, m.message_id_hdr, m.refs, m.date FROM message m
                 WHERE m.thread_id = ?1
                   AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = m.id)
                 ORDER BY m.id",
            )?;
            let rows = stmt.query_map([thread.0], |row| {
                Ok(TreeRow {
                    id: MessageId(row.get(0)?),
                    message_id: row.get(1)?,
                    refs: row
                        .get::<_, Option<String>>(2)?
                        .map(|r| split_refs(&r))
                        .unwrap_or_default(),
                    date: row.get(3)?,
                })
            })?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        let items: Vec<Item<'_>> = rows
            .iter()
            .map(|row| Item {
                id: row.id,
                message_id: row.message_id.as_deref(),
                refs: &row.refs,
                date: row.date,
            })
            .collect();
        Ok(jwz::tree(&items))
    }
}

impl MailBatch<'_> {
    /// Puts `message` in the thread of its Message-ID or any of `refs`
    /// (its ancestors, oldest first), merging threads it links, and
    /// remembers `refs`. Returns the thread.
    pub fn assign_thread(&mut self, message: MessageId, refs: &[String]) -> Result<ThreadId> {
        let tx = self.tx();
        let (account, own, subject): (i64, Option<String>, String) = tx
            .prepare_cached(
                "SELECT account_id, message_id_hdr, subject FROM message WHERE id = ?1",
            )?
            .query_row([message.0], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?;
        let account = AccountId(account);
        let mut keys: Vec<&str> = Vec::with_capacity(refs.len() + 1);
        for key in own.iter().chain(refs) {
            // A key must not contain the separator of `refs`.
            if !key.is_empty() && !key.contains(' ') && !keys.contains(&key.as_str()) {
                keys.push(key);
            }
        }

        let mut threads: Vec<i64> = Vec::new();
        {
            let mut find = tx.prepare_cached(
                "SELECT thread_id FROM thread_key WHERE account_id = ?1 AND key = ?2",
            )?;
            for key in &keys {
                if let Some(thread) = find
                    .query_row(params![account.0, key], |row| row.get::<_, i64>(0))
                    .optional()?
                    && !threads.contains(&thread)
                {
                    threads.push(thread);
                }
            }
        }
        threads.sort_unstable();
        let target = match threads.first() {
            Some(&oldest) => oldest,
            None => {
                tx.prepare_cached("INSERT INTO thread (account_id, subject_norm) VALUES (?1, ?2)")?
                    .execute(params![account.0, normalize_subject(&subject)])?;
                tx.last_insert_rowid()
            }
        };
        for &other in threads.iter().skip(1) {
            tx.prepare_cached("UPDATE message SET thread_id = ?1 WHERE thread_id = ?2")?
                .execute(params![target, other])?;
            tx.prepare_cached("UPDATE thread_key SET thread_id = ?1 WHERE thread_id = ?2")?
                .execute(params![target, other])?;
            tx.prepare_cached("DELETE FROM thread WHERE id = ?1")?
                .execute([other])?;
        }
        {
            let mut insert = tx.prepare_cached(
                "INSERT OR IGNORE INTO thread_key (account_id, key, thread_id) VALUES (?1, ?2, ?3)",
            )?;
            for key in &keys {
                insert.execute(params![account.0, key, target])?;
            }
        }
        let refs_text: Vec<&str> = refs
            .iter()
            .map(String::as_str)
            .filter(|r| !r.is_empty() && !r.contains(' '))
            .collect();
        tx.prepare_cached("UPDATE message SET thread_id = ?2, refs = ?3 WHERE id = ?1")?
            .execute(params![message.0, target, refs_text.join(" ")])?;
        Ok(ThreadId(target))
    }
}

/// The subject without reply and forward prefixes, for grouping and
/// display: `Re: Fwd: AW: Lunch` → `Lunch`.
pub fn normalize_subject(subject: &str) -> String {
    let mut rest = subject.trim();
    loop {
        let lower = rest.to_ascii_lowercase();
        let prefix = ["re:", "fwd:", "fw:", "aw:", "sv:", "wg:", "vs:", "tr:"]
            .iter()
            .find(|p| lower.starts_with(**p));
        match prefix {
            Some(prefix) => rest = rest[prefix.len()..].trim_start(),
            None => return rest.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use katna_core::{AccountKind, Paths};

    use super::*;
    use crate::jwz::tests::{Rng, ancestors, forest};
    use crate::{Added, FolderRole, Mode, RemoteMessage};

    fn open() -> (tempfile::TempDir, Store, AccountId, FolderId) {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        batch.commit().unwrap();
        (tmp, store, account, inbox)
    }

    fn add(
        batch: &mut MailBatch<'_>,
        account: AccountId,
        folder: FolderId,
        uid: u32,
        message_id: Option<&str>,
        subject: &str,
        refs: &[String],
    ) -> MessageId {
        let message = RemoteMessage {
            uid,
            message_id_hdr: message_id,
            subject: Some(subject),
            date: Some(i64::from(uid) * 100),
            size: 1,
            flags: if uid.is_multiple_of(2) {
                MessageFlags::SEEN
            } else {
                MessageFlags::empty()
            },
            keywords: &[],
            has_attachments: false,
            list_id: None,
            participants: &[],
            references: refs,
        };
        match batch.add_remote_message(account, folder, &message).unwrap() {
            Added::Message(id) => id,
            other => panic!("{other:?}"),
        }
    }

    fn refs(ids: &[&str]) -> Vec<String> {
        ids.iter().map(|id| (*id).to_owned()).collect()
    }

    #[test]
    fn replies_join_and_link_threads() {
        let (_tmp, mut store, account, inbox) = open();
        let mut batch = store.mail_batch().unwrap();
        let a = add(&mut batch, account, inbox, 1, Some("a"), "Lunch", &[]);
        let unrelated = add(&mut batch, account, inbox, 2, Some("x"), "Other", &[]);
        // c's parent b is not here yet: c starts its own thread.
        let c = add(
            &mut batch,
            account,
            inbox,
            3,
            Some("c"),
            "Re: Lunch",
            &refs(&["b"]),
        );
        batch.commit().unwrap();
        let thread = |store: &Store, id| store.thread_of(id).unwrap().unwrap();
        assert_ne!(thread(&store, a), thread(&store, c));

        // b refers to a: it links both threads into one.
        let mut batch = store.mail_batch().unwrap();
        let b = add(
            &mut batch,
            account,
            inbox,
            4,
            Some("b"),
            "Re: Lunch",
            &refs(&["a"]),
        );
        batch.commit().unwrap();
        let joined = thread(&store, a);
        assert_eq!(thread(&store, b), joined);
        assert_eq!(thread(&store, c), joined);
        assert_ne!(thread(&store, unrelated), joined);

        let tree: Vec<(MessageId, u32)> = store
            .thread_tree(joined)
            .unwrap()
            .iter()
            .map(|n| (n.message, n.depth))
            .collect();
        assert_eq!(tree, [(a, 0), (b, 1), (c, 2)]);

        let threads = store.folder_threads(inbox, 10).unwrap();
        assert_eq!(threads.len(), 2);
        assert_eq!(threads[0].thread, Some(joined));
        assert_eq!(threads[0].subject, "Lunch");
        assert_eq!((threads[0].messages, threads[0].unread), (3, 2));
        assert_eq!(threads[0].latest, b);
        assert_eq!(threads[0].last_date, 400);
        assert_eq!(threads[1].latest, unrelated);
        assert_eq!(store.folder_threads(inbox, 1).unwrap().len(), 1);
    }

    #[test]
    fn unthreaded_messages_stand_alone_until_threaded() {
        let (_tmp, mut store, account, inbox) = open();
        let mut batch = store.mail_batch().unwrap();
        let raw = b"Message-ID: <r@x>\r\nIn-Reply-To: <a@x>\r\nSubject: Re: hi\r\n\r\nHi\r\n";
        let Added::Message(reply) = batch
            .add_message(
                account,
                inbox,
                &crate::NewMessage {
                    raw,
                    message_id_hdr: Some("r@x"),
                    subject: Some("Re: hi"),
                    date: Some(5),
                    flags: MessageFlags::SEEN,
                    has_attachments: false,
                    list_id: None,
                    snippet: None,
                    participants: &[],
                },
            )
            .unwrap()
        else {
            panic!()
        };
        batch.commit().unwrap();
        let pending = store.unthreaded(10).unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!((pending[0].id, pending[0].account), (reply, account));
        assert!(pending[0].refs.is_none() && pending[0].blob_hash.is_some());
        let threads = store.folder_threads(inbox, 10).unwrap();
        assert_eq!((threads[0].thread, threads[0].latest), (None, reply));

        let mut batch = store.mail_batch().unwrap();
        let thread = batch.assign_thread(reply, &refs(&["a@x"])).unwrap();
        batch.commit().unwrap();
        assert!(store.unthreaded(10).unwrap().is_empty());
        assert_eq!(
            store.folder_threads(inbox, 10).unwrap()[0].thread,
            Some(thread)
        );
    }

    /// Whatever order messages arrive in, and however their `References`
    /// are cut short, threads are the groups linked by shared IDs.
    #[test]
    fn threads_do_not_depend_on_arrival_order() {
        for seed in 1..40u64 {
            let mut rng = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15));
            let n = 1 + rng.below(30);
            let parents = forest(&mut rng, n);
            let ids: Vec<String> = (0..n).map(|i| format!("m{i}@t")).collect();
            let all_refs: Vec<Vec<String>> = (0..n)
                .map(|i| {
                    let chain: Vec<String> = ancestors(&parents, i)
                        .into_iter()
                        .map(|a| ids[a].clone())
                        .collect();
                    // Mailers keep only the last few References.
                    let keep = rng.below(chain.len() + 1).max(chain.len().min(1));
                    chain[chain.len() - keep..].to_vec()
                })
                .collect();
            let mut order: Vec<usize> = (0..n).filter(|_| rng.below(6) != 0).collect();
            rng.shuffle(&mut order);

            let (_tmp, mut store, account, inbox) = open();
            let mut batch = store.mail_batch().unwrap();
            let mut stored = HashMap::new();
            for (uid, &i) in order.iter().enumerate() {
                let id = add(
                    &mut batch,
                    account,
                    inbox,
                    uid as u32 + 1,
                    Some(&ids[i]),
                    "s",
                    &all_refs[i],
                );
                stored.insert(i, id);
            }
            batch.commit().unwrap();

            // Expected: union of messages that share any ID.
            let mut group: Vec<usize> = (0..n).collect();
            fn find(group: &mut [usize], mut x: usize) -> usize {
                while group[x] != x {
                    group[x] = group[group[x]];
                    x = group[x];
                }
                x
            }
            let index: HashMap<&str, usize> = ids
                .iter()
                .enumerate()
                .map(|(i, id)| (id.as_str(), i))
                .collect();
            for &i in &order {
                for r in &all_refs[i] {
                    let (a, b) = (find(&mut group, i), find(&mut group, index[r.as_str()]));
                    group[a] = b;
                }
            }
            for &i in &order {
                for &j in &order {
                    let same = find(&mut group, i) == find(&mut group, j);
                    let thread_i = store.thread_of(stored[&i]).unwrap();
                    let thread_j = store.thread_of(stored[&j]).unwrap();
                    assert_eq!(same, thread_i == thread_j, "seed {seed}: {i} and {j}");
                }
            }
        }
    }

    #[test]
    fn subjects_lose_reply_prefixes() {
        assert_eq!(normalize_subject("Re: Fwd: AW:  Lunch"), "Lunch");
        assert_eq!(normalize_subject("RE:re:x"), "x");
        assert_eq!(normalize_subject("Regarding lunch"), "Regarding lunch");
    }
}
