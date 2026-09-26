// SPDX-License-Identifier: GPL-3.0-or-later

//! Threads messages the sync did not thread as it stored them: imported
//! mail and mail stored before threading existed (`docs/ARCHITECTURE.md`
//! §6.5). IMAP sync threads new messages itself.

use katna_core::AccountId;
use katna_store::Store;

/// How many messages [`thread_pending`] handles per transaction.
pub const BATCH: u32 = 2000;

/// Threads up to [`BATCH`] waiting messages, reading their ancestors from
/// the stored message when sync did not record them. Returns the accounts
/// whose threads changed; empty when nothing was waiting.
pub fn thread_pending(store: &mut Store) -> katna_store::Result<Vec<AccountId>> {
    let pending = store.unthreaded(BATCH)?;
    let mut work = Vec::with_capacity(pending.len());
    for message in pending {
        let refs = match message.refs {
            Some(refs) => refs,
            None => match message.blob_hash {
                Some(hash) => match store.blobs().get(&hash)? {
                    Some(raw) => katna_import::thread_headers(&raw).1,
                    None => Vec::new(),
                },
                // Only the Message-ID to go on.
                None => Vec::new(),
            },
        };
        work.push((message.id, message.account, refs));
    }
    let mut accounts = Vec::new();
    let mut batch = store.mail_batch()?;
    for (id, account, refs) in &work {
        batch.assign_thread(*id, refs)?;
        if !accounts.contains(account) {
            accounts.push(*account);
        }
    }
    batch.commit()?;
    Ok(accounts)
}
