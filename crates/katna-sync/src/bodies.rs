// SPDX-License-Identifier: GPL-3.0-or-later

//! Sync level 3 (`docs/ARCHITECTURE.md` §6.2 and §6.3): full messages for
//! the offline window, and single messages on request.
//!
//! Bodies go into the blob store with `BODY.PEEK[]`, so downloading never
//! marks mail as read. The snippet and attachment flag are taken from the
//! full message, replacing level 1's guess from the headers.

use std::collections::HashMap;

use katna_core::AccountId;
use katna_store::{FolderId, MessageId, Store};

use crate::{Error, MailBackend, Result};

/// Messages fetched in one command.
pub const BODY_CHUNK: u32 = 25;

/// Which messages are kept offline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OfflineWindow {
    /// Messages dated within this many days; `None` for all.
    pub days: Option<u32>,
    /// Larger messages are only fetched on request.
    pub max_size: u64,
}

impl Default for OfflineWindow {
    /// The last 30 days, messages up to 10 MB.
    fn default() -> Self {
        Self {
            days: Some(30),
            max_size: 10 * 1024 * 1024,
        }
    }
}

impl OfflineWindow {
    /// The oldest date in the window, in Unix seconds.
    fn since(&self, now: i64) -> Option<i64> {
        self.days.map(|days| now - i64::from(days) * 86_400)
    }
}

/// Downloads the bodies of `folder`'s messages in `window`, newest first.
/// SELECTs `path` only when there is something to fetch. Returns how many
/// bodies were stored.
pub async fn download_bodies<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    folder: FolderId,
    path: &str,
    window: &OfflineWindow,
    now: i64,
) -> Result<usize> {
    let since = window.since(now);
    let mut selected = false;
    let mut stored = 0;
    loop {
        let wanted = store.messages_without_body(folder, since, window.max_size, BODY_CHUNK)?;
        if wanted.is_empty() {
            break;
        }
        if !selected {
            backend.select(path).await?;
            selected = true;
        }
        let saved = fetch_and_save(backend, store, &wanted).await?;
        stored += saved;
        // Messages expunged since the last sync come back empty; level-1
        // sync removes them. Stop instead of asking again.
        if saved < wanted.len() {
            break;
        }
    }
    if stored > 0 {
        tracing::debug!(path, stored, "downloaded bodies");
    }
    Ok(stored)
}

/// Downloads one message of `account` now, wherever it is on the server.
/// Leaves its folder selected.
pub async fn fetch_body<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    message: MessageId,
) -> Result<()> {
    let Some((owner, folder, uid)) = store.remote_location(message)? else {
        return Err(Error::Rejected(format!(
            "message {} is not on a server",
            message.0
        )));
    };
    if owner != account {
        return Err(Error::Rejected(format!(
            "message {} belongs to account {owner}",
            message.0
        )));
    }
    backend.select(&folder.path).await?;
    if fetch_and_save(backend, store, &[(message, uid)]).await? == 0 {
        return Err(Error::Rejected(format!(
            "message {} is no longer in {}",
            message.0, folder.path
        )));
    }
    Ok(())
}

/// Fetches `wanted` from the selected folder and stores what came back.
async fn fetch_and_save<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    wanted: &[(MessageId, u32)],
) -> Result<usize> {
    let ids: HashMap<u32, MessageId> = wanted.iter().map(|&(id, uid)| (uid, id)).collect();
    let uids: Vec<u32> = wanted.iter().map(|&(_, uid)| uid).collect();
    let fetched = backend.fetch_bodies(&uids).await?;
    let mut batch = store.mail_batch()?;
    let mut saved = 0;
    for (uid, raw) in &fetched {
        let Some(&id) = ids.get(uid) else {
            continue;
        };
        let parsed = katna_import::parse_message(raw).unwrap_or_default();
        batch.set_message_body(id, raw, parsed.snippet.as_deref(), parsed.has_attachments)?;
        // A message stored before threading: thread it now (does nothing
        // for one that already has its thread and category).
        let references = parsed.reference_strs();
        let facts = katna_store::Backfill {
            in_reply_to: parsed.in_reply_to.as_deref(),
            references: &references,
            gm_thread_id: None,
            category: Some(parsed.category),
        };
        batch.backfill_message(id, &facts)?;
        saved += 1;
    }
    batch.commit()?;
    Ok(saved)
}
