// SPDX-License-Identifier: GPL-3.0-or-later

//! Threads and categories for messages stored before schema v2
//! (`docs/ARCHITECTURE.md` §6.5).
//!
//! Reads each message's header back from the blob store and hands the
//! facts to [`MailBatch::backfill_message`](katna_store::MailBatch), one
//! small transaction per batch, so sync keeps getting the write lock in
//! between. Progress is the data itself (messages without a thread or
//! category), so it resumes after a restart and running it twice does
//! nothing more. Messages whose body was never downloaded are left to
//! `katna-sync`, which fetches their headers again.

use katna_store::{Backfill, MailCategory, MessageId, Store};

use crate::parse::{HeaderLinks, parse_links};

/// Messages per transaction.
pub const BATCH: u32 = 500;

/// What one [`step`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Step {
    /// The last message looked at; pass it to the next step.
    pub last: MessageId,
    /// Messages looked at.
    pub seen: usize,
    /// Messages that got a thread or a category.
    pub changed: usize,
}

/// Backfills up to `limit` messages with ID above `after`. Returns `None`
/// when none is left.
pub fn step(store: &mut Store, after: MessageId, limit: u32) -> katna_store::Result<Option<Step>> {
    let candidates = store.unthreaded_with_body(after, limit)?;
    let Some(&(last, _)) = candidates.last() else {
        return Ok(None);
    };
    // Read and parse before taking the write lock.
    let mut facts = Vec::with_capacity(candidates.len());
    for (id, hash) in &candidates {
        let links = store
            .blobs()
            .get(hash)?
            .and_then(|raw| parse_links(&raw))
            // Unreadable: still give it a thread of its own and a tab, so
            // it is not looked at again.
            .unwrap_or(HeaderLinks {
                in_reply_to: None,
                references: Vec::new(),
                category: MailCategory::Primary,
            });
        facts.push((*id, links));
    }
    let mut batch = store.mail_batch()?;
    let mut changed = 0;
    for (id, links) in &facts {
        let references = links.reference_strs();
        let backfill = Backfill {
            in_reply_to: links.in_reply_to.as_deref(),
            references: &references,
            gm_thread_id: None,
            category: Some(links.category),
        };
        if batch.backfill_message(*id, &backfill)? {
            changed += 1;
        }
    }
    batch.commit()?;
    Ok(Some(Step {
        last,
        seen: candidates.len(),
        changed,
    }))
}

/// Runs [`step`] until nothing is left or `keep_going` (called after each
/// step with the messages changed so far) returns false. Returns the
/// number of messages changed.
pub fn run(
    store: &mut Store,
    batch: u32,
    mut keep_going: impl FnMut(u64) -> bool,
) -> katna_store::Result<u64> {
    let mut after = MessageId(0);
    let mut changed = 0u64;
    while let Some(step) = step(store, after, batch)? {
        after = step.last;
        changed += step.changed as u64;
        if !keep_going(changed) {
            break;
        }
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Options, StoreSink, import_maildir};
    use katna_core::Paths;
    use katna_store::{Mode, ThreadId};
    use std::fs;

    #[test]
    fn threads_and_classifies_an_old_store() {
        let corpus = tempfile::tempdir().unwrap();
        let write = |rel: &str, body: &str| {
            let path = corpus.path().join(rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, body).unwrap();
        };
        // The reply comes first in ID order.
        write(
            "inbox/cur/1:2,S",
            "Message-ID: <2@x>\r\nIn-Reply-To: <1@x>\r\nFrom: bob@example.org\r\n\
             Subject: Re: Plan\r\n\r\nOk\r\n",
        );
        write(
            "inbox/cur/2:2,S",
            "Message-ID: <1@x>\r\nFrom: ada@example.org\r\nSubject: Plan\r\n\r\nPlan\r\n",
        );
        write(
            "inbox/cur/3:2,S",
            "Message-ID: <3@x>\r\nFrom: Shop <news@shop.example>\r\nSubject: 20% off\r\n\
             List-Unsubscribe: <https://shop.example/u>\r\nX-Campaign: 7\r\n\r\nBuy\r\n",
        );

        let data = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(data.path());
        let mut store = katna_store::Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = StoreSink::local_account(&mut store, "old").unwrap();
        let mut sink = StoreSink::new(&mut store, account.id);
        import_maildir(corpus.path(), &mut sink, &Options::default(), |_| {}).unwrap();

        let threads = |store: &Store| -> Vec<(Option<ThreadId>, Option<MailCategory>)> {
            store
                .messages_after(MessageId(0), 10)
                .unwrap()
                .into_iter()
                .map(|m| (m.thread_id, m.category))
                .collect()
        };
        let imported = threads(&store);
        assert_eq!(imported[0].0, imported[1].0, "the importer threads");
        assert_eq!(imported[2].1, Some(MailCategory::Promotions));

        store.reset_threads().unwrap();
        assert!(threads(&store).iter().all(|t| *t == (None, None)));

        assert_eq!(run(&mut store, 2, |_| true).unwrap(), 3);
        assert_eq!(threads(&store), imported);
        assert_eq!(run(&mut store, 2, |_| true).unwrap(), 0, "nothing left");
        assert_eq!(store.unthreaded_count().unwrap(), 0);
    }
}
