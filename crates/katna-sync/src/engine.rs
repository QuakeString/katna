// SPDX-License-Identifier: GPL-3.0-or-later

//! Sync level 1 (`docs/ARCHITECTURE.md` §6.2): folders, flags and indexed
//! headers of every message, into the store. No bodies yet.
//!
//! Per folder:
//!
//! 1. SELECT. If UIDVALIDITY changed, the stored UIDs mean nothing any more:
//!    forget the folder's messages and start over.
//! 2. Flags of known messages: only those changed since the stored
//!    HIGHESTMODSEQ when the server has CONDSTORE, otherwise all of them.
//! 3. New messages: headers and attachments (from `BODYSTRUCTURE`) for UIDs
//!    above the highest stored one, newest first, in chunks going back in
//!    time, so a new account shows its recent mail first. Each chunk is
//!    committed on its own, and the UIDs still to fetch are saved before
//!    each one, so an interrupted first sync keeps its progress and the
//!    next run fills in the older mail.
//! 4. Expunged messages: with QRESYNC, the UIDs the flag fetch reported as
//!    `VANISHED (EARLIER)`; when the message count still does not add up,
//!    compare the server's UID list with ours.
//! 5. Messages stored before threading and without a downloaded body:
//!    their headers again, so they get a thread and a category (at most
//!    [`REFRESH_PER_SYNC`] per run). Likewise their structure for those
//!    with attachments but no attachment list (stored before lists were
//!    read), so the message list can show their files.
//! 6. Gmail messages stored before schema v4, one row per label: their
//!    `X-GM-MSGID`, so copies of one message become one row
//!    ([`MailBatch::adopt_gm_msgid`](katna_store::MailBatch::adopt_gm_msgid)).
//!    Once per folder; nothing changes on the server.
//! 7. Gmail (`X-GM-EXT-1`), in every folder: Gmail's own categories, with
//!    one `X-GM-RAW "category:…"` search per tab; the first time for every
//!    message, then for new ones. Every folder, because each server copy
//!    is its own message: the same mail in the inbox and in All Mail must
//!    land in the same tab.
//! 8. Save UIDVALIDITY, HIGHESTMODSEQ, UIDNEXT and the message count seen
//!    at SELECT. Changes made during the sync have higher mod-sequences and
//!    UIDs, so the next run sees them, and so does [`stale_folders`].
//!
//! New messages get their thread (with Gmail's `X-GM-THRID` when there is
//! one) and category as they are stored.
//!
//! The store is only written between network calls, never while waiting
//! on the server.

use std::collections::{HashMap, HashSet};

use katna_core::{AccountId, MailCategory};
use katna_store::{
    Adopted, Backfill, FolderId, MessageFlags, NewAttachment, NewParticipant, RemoteMessage, Store,
    StoredFolder,
};
use serde::{Deserialize, Serialize};

use crate::{
    Error, FlagState, Flags, Folder, FolderStatus, IMPORTANT, MailBackend, MessageHeaders, Result,
};

/// How many messages one header fetch asks for.
pub const CHUNK: u32 = 500;

/// What syncing one folder did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FolderReport {
    pub path: String,
    pub added: usize,
    pub flags_changed: usize,
    pub removed: usize,
    /// UIDVALIDITY changed, so the folder was downloaded again.
    pub reset: bool,
    /// Messages stored before threading that got their thread or category
    /// from headers fetched again, inbox messages whose Gmail category
    /// changed, and Gmail copies merged into one message.
    pub backfilled: usize,
}

/// At most this many old messages per folder and sync get their headers
/// fetched again for threading, so a big backlog does not hold up new mail.
pub const REFRESH_PER_SYNC: u32 = 5_000;

/// Gmail's category tabs, besides Primary, in the order a message that
/// matches several is given one.
const GMAIL_CATEGORIES: [MailCategory; 4] = [
    MailCategory::Social,
    MailCategory::Promotions,
    MailCategory::Updates,
    MailCategory::Forums,
];

/// `folder.sync_state`: what the engine remembers about a folder besides
/// UIDVALIDITY and HIGHESTMODSEQ.
#[derive(Debug, Default, Serialize, Deserialize)]
struct FolderState {
    /// Gmail categories were read for every message of this folder; from
    /// then on only new messages are asked about.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    gmail_categories: bool,
    /// Every message of this folder has its Gmail message ID, or the
    /// server is not Gmail (step 6 is done).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    gmail_msgids: bool,
    /// UIDNEXT and message count at the last sync, to tell from a STATUS
    /// whether the folder changed since.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uid_next: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    exists: Option<u32>,
    /// UID ranges (first, last) below the highest stored UID that are not
    /// downloaded yet: newer mail comes first, so an interrupted sync
    /// leaves older mail here for the next run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    missing: Vec<(u32, u32)>,
}

impl FolderState {
    fn of(folder: &StoredFolder) -> Self {
        folder
            .sync_state
            .as_deref()
            .and_then(|json| serde_json::from_str(json).ok())
            .unwrap_or_default()
    }
}

/// What [`stale_folders`] found.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct StaleFolders {
    /// Stored folders that changed on the server since their last sync.
    pub changed: Vec<(FolderId, String)>,
    /// Folders the store does not know yet: only a full sync adds them.
    pub unknown: bool,
}

/// Compares the server's `statuses` (path and [`MailBackend::status`]) with
/// what the last sync of each folder saw. Without CONDSTORE a flag change
/// does not show, so full syncs still catch those.
pub fn stale_folders(
    store: &Store,
    account: AccountId,
    statuses: &[(String, FolderStatus)],
) -> Result<StaleFolders> {
    let stored: HashMap<String, StoredFolder> = store
        .folders(account)?
        .into_iter()
        .map(|f| (f.path.clone(), f))
        .collect();
    let mut out = StaleFolders::default();
    for (path, status) in statuses {
        let Some(folder) = stored.get(path) else {
            out.unknown = true;
            continue;
        };
        let state = FolderState::of(folder);
        let changed = folder.uidvalidity != status.uid_validity
            || (status.highest_modseq.is_some() && folder.highestmodseq != status.highest_modseq)
            || (state.uid_next.is_some() && state.uid_next != status.uid_next)
            || state.exists != Some(status.exists);
        if changed {
            out.changed.push((folder.id, path.clone()));
        }
    }
    Ok(out)
}

/// Syncs the folder list of `account`, then every selectable folder, the
/// inbox first.
pub async fn sync_account<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
) -> Result<Vec<FolderReport>> {
    sync_account_live(backend, store, account, |_| {}).await
}

/// [`sync_account`], calling `stored` with how many messages each commit
/// added (0 for the folder list), so the mail can show as it arrives
/// rather than once every folder is done: a new account's first sync can
/// take many minutes.
pub async fn sync_account_live<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    mut stored: impl FnMut(usize),
) -> Result<Vec<FolderReport>> {
    let mut folders = sync_folders(backend, store, account).await?;
    stored(0);
    // The inbox first: it is what shows while a new account downloads,
    // and servers such as Dovecot list it last.
    folders.sort_by_key(|(folder, _)| folder.role != Some(crate::FolderRole::Inbox));
    let mut reports = Vec::with_capacity(folders.len());
    for (folder, id) in folders {
        if !folder.selectable {
            continue;
        }
        match sync_folder_live(backend, store, account, id, &folder.name, &mut stored).await {
            Ok(report) => reports.push(report),
            // Deleted since LIST, or not ours to read: skip it this time.
            Err(Error::Rejected(reason)) => {
                tracing::info!(path = folder.name, %reason, "skipping folder");
            }
            Err(error) => return Err(error),
        }
    }
    Ok(reports)
}

/// Makes the stored folder list match the server's. Folders gone from the
/// server are removed with their messages.
pub async fn sync_folders<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
) -> Result<Vec<(Folder, FolderId)>> {
    let remote = backend.list_folders().await?;
    let names: HashSet<&str> = remote.iter().map(|f| f.name.as_str()).collect();
    let gone: Vec<FolderId> = store
        .folders(account)?
        .into_iter()
        .filter(|f| !names.contains(f.path.as_str()))
        .map(|f| f.id)
        .collect();

    let mut batch = store.mail_batch()?;
    let mut out = Vec::with_capacity(remote.len());
    for folder in remote {
        let id = batch.upsert_folder(account, &folder.name, folder.role.map(role))?;
        out.push((folder, id));
    }
    for id in gone {
        batch.remove_folder(id)?;
    }
    batch.commit()?;
    Ok(out)
}

/// Brings one folder up to date.
pub async fn sync_folder<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    folder: FolderId,
    path: &str,
) -> Result<FolderReport> {
    sync_folder_live(backend, store, account, folder, path, &mut |_| {}).await
}

/// [`sync_folder`], calling `on_saved` after each chunk of new messages is
/// committed.
async fn sync_folder_live<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    folder: FolderId,
    path: &str,
    on_saved: &mut impl FnMut(usize),
) -> Result<FolderReport> {
    let mut report = FolderReport {
        path: path.to_owned(),
        ..FolderReport::default()
    };
    let status = backend.select(path).await?;
    let stored = store.folders(account)?.into_iter().find(|f| f.id == folder);
    let mut state = stored.as_ref().map(FolderState::of).unwrap_or_default();
    let (old_validity, old_modseq) = stored
        .map(|f| (f.uidvalidity, f.highestmodseq))
        .unwrap_or_default();

    // 1. UIDVALIDITY.
    let mut known = store.folder_uids(folder)?;
    if old_validity.is_some() && old_validity != status.uid_validity {
        tracing::info!(path, "UIDVALIDITY changed; downloading the folder again");
        let mut batch = store.mail_batch()?;
        batch.clear_folder(folder)?;
        batch.set_folder_state(folder, None, None, None)?;
        batch.commit()?;
        known.clear();
        report.reset = true;
        state = FolderState::default();
    }
    let old_modseq = if report.reset { None } else { old_modseq };

    // 2. Flags of the messages we already have.
    let last_known = known.last().copied().unwrap_or(0);
    let modseq_unchanged = old_modseq.is_some()
        && status.highest_modseq.is_some()
        && old_modseq == status.highest_modseq;
    let mut vanished = None;
    if last_known > 0 && !modseq_unchanged {
        let changes = backend.fetch_flags(1, last_known, old_modseq).await?;
        report.flags_changed = save_flags(store, folder, &changes.flags)?;
        vanished = changes.vanished;
    }

    // 3. New messages, newest first. Without UIDNEXT we cannot chunk:
    //    take them in one go.
    let mut added_uids = 0;
    let mut lowest_added = None;
    let mut ranges = Vec::new();
    match status.uid_next {
        Some(next) if last_known + 1 < next => ranges.push((last_known + 1, next - 1)),
        Some(_) => {}
        None => {
            let messages = backend.fetch_headers(last_known + 1, None).await?;
            added_uids += messages.len();
            lowest_added = messages.iter().map(|m| m.uid).min();
            let saved = save_messages(store, account, folder, &messages)?;
            if saved > 0 {
                on_saved(saved);
            }
            report.added += saved;
        }
    }
    ranges.append(&mut state.missing);
    while let Some(&(low, high)) = ranges.first() {
        let first = high.saturating_sub(CHUNK - 1).max(low);
        // Saved before the fetch, so a sync cut off after this chunk is
        // stored still knows what is below it.
        state.missing = ranges.clone();
        save_missing(store, folder, status.uid_validity, old_modseq, &state)?;

        let messages = backend.fetch_headers(first, Some(high)).await?;
        added_uids += messages.len();
        lowest_added = Some(lowest_added.map_or(first, |l: u32| l.min(first)));
        let saved = save_messages(store, account, folder, &messages)?;
        if saved > 0 {
            on_saved(saved);
        }
        report.added += saved;
        if first > low {
            ranges[0].1 = first - 1;
        } else {
            ranges.remove(0);
        }
    }
    state.missing = Vec::new();

    // 4. Expunges: the ones QRESYNC reported; the UID lists are compared
    //    only when the numbers still do not add up.
    let mut gone: Vec<u32> = match &vanished {
        Some(ranges) => known
            .iter()
            .copied()
            .filter(|uid| ranges.iter().any(|r| r.contains(uid)))
            .collect(),
        None => Vec::new(),
    };
    let expected = known.len() - gone.len() + added_uids;
    if status.exists as usize != expected && !known.is_empty() {
        let on_server: HashSet<u32> = backend.uids().await?.into_iter().collect();
        gone = known
            .into_iter()
            .filter(|uid| !on_server.contains(uid))
            .collect();
    }
    if !gone.is_empty() {
        let mut batch = store.mail_batch()?;
        report.removed = batch.remove_remote_messages(folder, &gone)?;
        batch.commit()?;
    }

    // 5. Headers again for messages stored before threading, and
    //    structures for those stored before attachment lists.
    report.backfilled += refresh_headers(backend, store, folder).await?;
    report.backfilled += refresh_structures(backend, store, folder).await?;

    // 6. Gmail copies stored before schema v4, once.
    if !state.gmail_msgids {
        let (merged, done) = merge_gmail_copies(backend, store, account, folder).await?;
        report.backfilled += merged;
        state.gmail_msgids = done;
    }

    // 7. Gmail's own categories: every message the first time, then new
    //    ones. Elsewhere `gmail_search` answers `None` without asking.
    let first = if !state.gmail_categories {
        Some(1)
    } else {
        lowest_added.filter(|_| added_uids > 0)
    };
    if let Some(first) = first
        && let Some(changed) = gmail_categories(backend, store, folder, first).await?
    {
        report.backfilled += changed;
        state.gmail_categories = true;
    }

    // 8. Where to continue next time.
    state.uid_next = status.uid_next;
    state.exists = Some(status.exists);
    let state = serde_json::to_string(&state).expect("plain struct serializes");
    let state = (state != "{}").then_some(state);
    let mut batch = store.mail_batch()?;
    batch.set_folder_state(
        folder,
        status.uid_validity,
        status.highest_modseq,
        state.as_deref(),
    )?;
    batch.commit()?;
    tracing::debug!(?report, "folder synced");
    Ok(report)
}

/// Saves `state` with the stored UIDVALIDITY and HIGHESTMODSEQ, midway
/// through a sync, so the UIDs still missing survive an interruption.
fn save_missing(
    store: &mut Store,
    folder: FolderId,
    uid_validity: Option<u32>,
    modseq: Option<u64>,
    state: &FolderState,
) -> Result<()> {
    let json = serde_json::to_string(state).expect("plain struct serializes");
    let mut batch = store.mail_batch()?;
    batch.set_folder_state(folder, uid_validity, modseq, Some(&json))?;
    batch.commit()?;
    Ok(())
}

/// UIDs one `X-GM-MSGID` fetch asks for.
const GMAIL_ID_CHUNK: u32 = 1_000;

/// Gives messages stored without Gmail's message ID (before schema v4)
/// their ID, merging copies of one message stored under several labels.
/// Returns how many copies were merged, and whether every message has its
/// ID now (or the server is not Gmail).
async fn merge_gmail_copies<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    folder: FolderId,
) -> Result<(usize, bool)> {
    let (mut merged, mut done) = (0, true);
    let mut after = 0;
    loop {
        let uids = store.uids_without_gm_msgid(folder, after, GMAIL_ID_CHUNK)?;
        let Some(&last) = uids.last() else {
            break;
        };
        let Some(ids) = backend.gmail_message_ids(&uids).await? else {
            return Ok((merged, true));
        };
        let mut batch = store.mail_batch()?;
        for uid in uids {
            let Some(&gm_msgid) = ids.get(&uid) else {
                continue;
            };
            match batch.adopt_gm_msgid(account, folder, uid, gm_msgid)? {
                Adopted::Merged(_) => merged += 1,
                Adopted::Busy => done = false,
                Adopted::Set(_) | Adopted::Unchanged => {}
            }
        }
        batch.commit()?;
        after = last;
    }
    if merged > 0 {
        tracing::info!(
            folder = folder.0,
            merged,
            "merged Gmail copies stored before v4"
        );
    }
    Ok((merged, done))
}

/// Fetches the headers of messages that have no thread or category yet
/// and no stored body (stores from before threading), at most
/// [`REFRESH_PER_SYNC`] per call. Returns how many were updated.
async fn refresh_headers<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    folder: FolderId,
) -> Result<usize> {
    let uids = store.uids_needing_headers(folder, REFRESH_PER_SYNC)?;
    let mut updated = 0;
    for chunk in uid_runs(&uids) {
        let (first, last) = (chunk[0], chunk[chunk.len() - 1]);
        let wanted: HashSet<u32> = chunk.iter().copied().collect();
        let messages = backend.fetch_headers(first, Some(last)).await?;
        let mut batch = store.mail_batch()?;
        for message in messages.iter().filter(|m| wanted.contains(&m.uid)) {
            let parsed = katna_import::parse_message(&message.header).unwrap_or_default();
            let references = parsed.reference_strs();
            let facts = Backfill {
                in_reply_to: parsed.in_reply_to.as_deref(),
                references: &references,
                gm_thread_id: message.gm_thread_id,
                category: Some(parsed.category),
            };
            if batch.backfill_remote(folder, message.uid, &facts)? {
                updated += 1;
            }
        }
        batch.commit()?;
    }
    Ok(updated)
}

/// Attachment lists for messages of the selected `folder` that were
/// stored before lists were read from `BODYSTRUCTURE` and have no body
/// yet (those with one get theirs from `katna_import::backfill`). At most
/// [`REFRESH_PER_SYNC`] per call. Returns how many were updated.
async fn refresh_structures<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    folder: FolderId,
) -> Result<usize> {
    let uids = store.uids_needing_structure(folder, REFRESH_PER_SYNC)?;
    let mut updated = 0;
    for chunk in uid_runs(&uids) {
        let (first, last) = (chunk[0], chunk[chunk.len() - 1]);
        let wanted: HashSet<u32> = chunk.iter().copied().collect();
        let messages = backend.fetch_headers(first, Some(last)).await?;
        let mut batch = store.mail_batch()?;
        for message in messages.iter().filter(|m| wanted.contains(&m.uid)) {
            // A structure that still cannot be read is tried again next
            // time.
            let Some(parts) = &message.attachments else {
                continue;
            };
            let list: Vec<NewAttachment<'_>> = parts
                .iter()
                .map(|a| NewAttachment {
                    part: &a.part,
                    mime: &a.mime,
                    filename: a.filename.as_deref(),
                    size: a.size,
                })
                .collect();
            if batch.list_attachments_at(folder, message.uid, &list)? {
                updated += 1;
            }
        }
        batch.commit()?;
    }
    Ok(updated)
}

/// Splits ascending `uids` into runs for one header fetch each: at most
/// [`CHUNK`] UIDs, spanning at most `2 × CHUNK` UIDs, so a sparse list
/// does not fetch everything in between.
fn uid_runs(uids: &[u32]) -> Vec<&[u32]> {
    let mut runs = Vec::new();
    let mut start = 0;
    for i in 1..=uids.len() {
        let end_run =
            i == uids.len() || i - start >= CHUNK as usize || uids[i] - uids[start] >= 2 * CHUNK;
        if end_run {
            runs.push(&uids[start..i]);
            start = i;
        }
    }
    runs
}

/// Sets Gmail's categories on the messages at UIDs `first..` of the inbox
/// `folder` (selected): one `X-GM-RAW "category:…"` search per tab, the
/// rest Primary. Returns `None` when the server is not Gmail, otherwise how
/// many messages changed.
async fn gmail_categories<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    folder: FolderId,
    first: u32,
) -> Result<Option<usize>> {
    let mut found: HashMap<u32, MailCategory> = HashMap::new();
    for category in GMAIL_CATEGORIES {
        let query = format!("category:{}", category.as_str());
        let Some(uids) = backend.gmail_search(first, &query).await? else {
            return Ok(None);
        };
        for uid in uids {
            found.entry(uid).or_insert(category);
        }
    }
    let categories: Vec<(u32, MailCategory)> = store
        .folder_uids(folder)?
        .into_iter()
        .filter(|&uid| uid >= first)
        .map(|uid| {
            let category = found.get(&uid).copied().unwrap_or(MailCategory::Primary);
            (uid, category)
        })
        .collect();
    let mut batch = store.mail_batch()?;
    let changed = batch.set_categories(folder, &categories)?;
    batch.commit()?;
    Ok(Some(changed))
}

fn save_flags(store: &mut Store, folder: FolderId, states: &[FlagState]) -> Result<usize> {
    let mut batch = store.mail_batch()?;
    let mut changed = 0;
    for state in states {
        let (flags, keywords) = split_flags(&state.flags);
        if batch
            .set_remote_flags(folder, state.uid, flags, &keywords)?
            .is_some()
        {
            changed += 1;
        }
    }
    batch.commit()?;
    Ok(changed)
}

fn save_messages(
    store: &mut Store,
    account: AccountId,
    folder: FolderId,
    messages: &[MessageHeaders],
) -> Result<usize> {
    let mut batch = store.mail_batch()?;
    let mut added = 0;
    for message in messages {
        let parsed = katna_import::parse_message(&message.header).unwrap_or_default();
        let participants: Vec<NewParticipant<'_>> = parsed
            .participants
            .iter()
            .map(|p| NewParticipant {
                role: p.role,
                email_norm: &p.email_norm,
                domain: &p.domain,
                display_name: p.display_name.as_deref(),
            })
            .collect();
        let (flags, keywords) = split_flags(&message.flags);
        let references = parsed.reference_strs();
        let attachments: Vec<NewAttachment<'_>> = message
            .attachments
            .iter()
            .flatten()
            .map(|a| NewAttachment {
                part: &a.part,
                mime: &a.mime,
                filename: a.filename.as_deref(),
                size: a.size,
            })
            .collect();
        let has_attachments = match &message.attachments {
            Some(parts) => !parts.is_empty(),
            None => looks_like_attachments(&message.header),
        };
        let remote = RemoteMessage {
            uid: message.uid,
            message_id_hdr: parsed.message_id.as_deref(),
            subject: parsed.subject.as_deref(),
            date: parsed.date.or(message.received),
            size: u64::from(message.size),
            flags,
            keywords: &keywords,
            has_attachments,
            list_id: parsed.list_id.as_deref(),
            participants: &participants,
            in_reply_to: parsed.in_reply_to.as_deref(),
            references: &references,
            gm_thread_id: message.gm_thread_id,
            gm_msgid: message.gm_msgid,
            category: Some(parsed.category),
            attachments: &attachments,
        };
        // A Gmail message already stored under another label is new here
        // too, but stays one message.
        match batch.add_remote_message(account, folder, &remote)? {
            katna_store::Added::Message(id) => {
                added += 1;
                record_auth_results(&mut batch, id, &message.header, &parsed.participants)?;
            }
            katna_store::Added::Location(id) => {
                added += 1;
                // Stored under the other label before its structure was
                // read: list its files now.
                if message.attachments.is_some() {
                    batch.list_attachments(id, &attachments)?;
                }
            }
            katna_store::Added::Duplicate(_) => {}
        }
    }
    batch.commit()?;
    Ok(added)
}

/// Keeps what the provider's `Authentication-Results` said about the
/// sender of the new message `id` (`header` holds its header fields), so
/// sender pictures are only looked up for authenticated domains.
pub(crate) fn record_auth_results(
    batch: &mut katna_store::MailBatch<'_>,
    id: katna_store::MessageId,
    header: &[u8],
    participants: &[katna_import::Participant],
) -> Result<()> {
    let from = participants
        .iter()
        .find(|p| p.role == katna_store::ParticipantRole::From)
        .map_or("", |p| p.domain.as_str());
    if let Some(verdict) = crate::auth_results::verdict(header, from) {
        batch.set_auth_results(id, &verdict.to_json())?;
    }
    Ok(())
}

/// Splits protocol flags into the store's bit set and the other keywords.
fn split_flags(flags: &Flags) -> (MessageFlags, Vec<String>) {
    let mut bits = MessageFlags::empty();
    bits.set(MessageFlags::SEEN, flags.seen);
    bits.set(MessageFlags::ANSWERED, flags.answered);
    bits.set(MessageFlags::FLAGGED, flags.flagged);
    bits.set(MessageFlags::DRAFT, flags.draft);
    bits.set(MessageFlags::DELETED, flags.deleted);
    let mut keywords = Vec::new();
    for keyword in &flags.keywords {
        if keyword.eq_ignore_ascii_case("$Forwarded") {
            bits |= MessageFlags::FORWARDED;
        } else if keyword.eq_ignore_ascii_case(IMPORTANT) {
            bits |= MessageFlags::IMPORTANT;
        } else {
            keywords.push(keyword.clone());
        }
    }
    (bits, keywords)
}

/// Without a usable `BODYSTRUCTURE`: `multipart/mixed` usually means
/// attachments.
fn looks_like_attachments(header: &[u8]) -> bool {
    let text = String::from_utf8_lossy(header).to_ascii_lowercase();
    text.lines()
        .any(|line| line.starts_with("content-type:") && line.contains("multipart/mixed"))
}

fn role(role: crate::FolderRole) -> katna_store::FolderRole {
    use crate::FolderRole as R;
    use katna_store::FolderRole as S;
    match role {
        R::Inbox => S::Inbox,
        R::All => S::All,
        R::Archive => S::Archive,
        R::Drafts => S::Drafts,
        R::Flagged => S::Flagged,
        R::Junk => S::Junk,
        R::Sent => S::Sent,
        R::Trash => S::Trash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_flags() {
        let flags = Flags {
            seen: true,
            flagged: true,
            keywords: vec!["$Forwarded".into(), "$Label1".into(), "$important".into()],
            ..Flags::default()
        };
        let (bits, keywords) = split_flags(&flags);
        assert_eq!(
            bits,
            MessageFlags::SEEN
                | MessageFlags::FLAGGED
                | MessageFlags::FORWARDED
                | MessageFlags::IMPORTANT
        );
        assert_eq!(keywords, vec!["$Label1".to_owned()]);
    }

    #[test]
    fn spots_multipart_mixed() {
        assert!(looks_like_attachments(
            b"Subject: x\r\nContent-Type: multipart/mixed; boundary=\"b\"\r\n\r\n"
        ));
        assert!(!looks_like_attachments(
            b"Content-Type: multipart/alternative; boundary=b\r\n\r\n"
        ));
    }
}
