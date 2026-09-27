// SPDX-License-Identifier: GPL-3.0-or-later

//! The operation queue (plan task 1.6): changes made in the apps are applied
//! to the store at once and replayed on the server by the account's worker
//! (`docs/ARCHITECTURE.md` §6.1).
//!
//! - [`set_flags`], [`move_messages`], [`copy_messages`],
//!   [`delete_messages`] and [`archive_messages`] edit the store and queue one operation per
//!   message. They never touch the network. Imported (`local`) accounts
//!   only change in the store.
//! - [`replay`] runs the due operations on a connection. A refused
//!   operation is tried again later; after [`MAX_ATTEMPTS`] it is dropped
//!   and the local change undone, so the store matches the server again.
//!
//! The worker replays before every sync, so a sync never overwrites a
//! local change that is still on its way to the server.

use std::collections::{HashMap, HashSet};

use katna_core::{AccountId, AccountKind};
use katna_store::{FolderId, FolderRole, MessageFlags, MessageId, Store, StoredFolder, ThreadId};
use serde::{Deserialize, Serialize};

use crate::{Error, Flags, MailBackend, Result};

/// Tries before an operation is given up.
pub const MAX_ATTEMPTS: u32 = 3;
/// Wait before trying a refused operation again, in seconds.
pub const RETRY_AFTER: i64 = 60;
/// Operations replayed per round.
const BATCH: u32 = 200;

/// One queued change, as stored in `op_queue.op_json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Op {
    /// Add and remove flags of `message` in `folder`. `uid` is `None` when
    /// the message was moved there and the server has not confirmed it yet;
    /// it is looked up at replay time.
    Flags {
        message: i64,
        folder: i64,
        path: String,
        uid: Option<u32>,
        add: u32,
        remove: u32,
    },
    /// Move `message` from `from` (where its UID is `uid`) to `to`. `uid`
    /// is `None` when an earlier move put the message in `from` and the
    /// server has not confirmed it yet (an undo right after an archive); it
    /// is looked up at replay time, after that earlier move has run.
    Move {
        message: i64,
        from: i64,
        from_path: String,
        uid: Option<u32>,
        to: i64,
        to_path: String,
    },
    /// Copy `message` from `from` (where its UID is `uid`, or looked up
    /// at replay time) to `to` as well; on Gmail this adds a label.
    Copy {
        message: i64,
        from: i64,
        from_path: String,
        uid: Option<u32>,
        to: i64,
        to_path: String,
    },
    /// Delete `message` in `folder` for good.
    Expunge {
        message: i64,
        path: String,
        uid: u32,
    },
    /// File the sent, outgoing `message` in `folder` (the Sent folder),
    /// then forget the outgoing copy.
    Append {
        message: i64,
        folder: i64,
        path: String,
    },
}

/// Why a local change could not be made.
#[derive(Debug, thiserror::Error)]
pub enum ChangeError {
    #[error("no message {0}")]
    UnknownMessage(i64),
    #[error("no folder {0}")]
    UnknownFolder(i64),
    #[error("{0}")]
    NotPossible(String),
    #[error(transparent)]
    Store(#[from] katna_store::Error),
}

/// What [`replay`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayReport {
    pub done: usize,
    /// Refused, to be tried again later.
    pub retried: usize,
    /// Given up; the local change was undone.
    pub failed: usize,
    /// Folders that received moved messages without the server telling
    /// their new UIDs; sync them to see the messages again.
    pub resync: Vec<(FolderId, String)>,
}

/// Adds and removes flags. Returns the accounts whose workers must replay.
pub fn set_flags(
    store: &mut Store,
    messages: &[MessageId],
    add: MessageFlags,
    remove: MessageFlags,
) -> Result<Vec<AccountId>, ChangeError> {
    let stored = store.messages_by_id(messages)?;
    if let Some(missing) = messages
        .iter()
        .find(|id| !stored.iter().any(|m| m.id == **id))
    {
        return Err(ChangeError::UnknownMessage(missing.0));
    }
    let mut plans = Vec::new();
    for message in &stored {
        let mut flags = MessageFlags::from_bits(message.flags.bits() | add.bits());
        flags = MessageFlags::from_bits(flags.bits() & !remove.bits());
        if flags == message.flags {
            continue;
        }
        let folders = folders_of(store, message.account)?;
        let mut ops = Vec::new();
        let locations = if is_synced(store, message.account)? {
            store.locations(message.id)?
        } else {
            Vec::new()
        };
        for location in locations {
            let Some(folder) = folders.get(&location.folder) else {
                continue;
            };
            ops.push(Op::Flags {
                message: message.id.0,
                folder: folder.id.0,
                path: folder.path.clone(),
                uid: location.uid,
                // Only what really changes, so replays do not undo other
                // clients' changes.
                add: flags.bits() & !message.flags.bits(),
                remove: message.flags.bits() & !flags.bits(),
            });
        }
        plans.push((message.id, message.account, flags, ops));
    }
    let mut batch = store.mail_batch()?;
    let mut accounts = Vec::new();
    for (id, account, flags, ops) in plans {
        batch.set_message_flags(id, flags)?;
        for op in ops {
            batch.enqueue_op(account, &encode(&op))?;
        }
        push_unique(&mut accounts, account);
    }
    batch.commit()?;
    Ok(accounts)
}

/// At most this many lines (conversations, or messages outside one) are
/// pinned at a time.
pub const MAX_PINS: usize = 10;

/// Pins or unpins messages at Unix time `now`. Pinning more than
/// [`MAX_PINS`] conversations is refused. Pins stay on this computer:
/// IMAP has none. Returns the accounts whose mail changed.
pub fn set_pinned(
    store: &mut Store,
    messages: &[MessageId],
    on: bool,
    now: i64,
) -> Result<Vec<AccountId>, ChangeError> {
    let stored = store.messages_by_id(messages)?;
    if let Some(missing) = messages
        .iter()
        .find(|id| !stored.iter().any(|m| m.id == **id))
    {
        return Err(ChangeError::UnknownMessage(missing.0));
    }
    // A conversation counts once, however many of its messages are pinned.
    let group = |thread: Option<ThreadId>, id: MessageId| match thread {
        Some(thread) => (true, thread.0),
        None => (false, id.0),
    };
    if on {
        let mut groups: HashSet<(bool, i64)> = store
            .pinned()?
            .iter()
            .map(|p| group(p.thread, p.message))
            .collect();
        let before = groups.len();
        groups.extend(stored.iter().map(|m| group(m.thread_id, m.id)));
        if groups.len() > MAX_PINS && groups.len() > before {
            return Err(ChangeError::NotPossible(format!(
                "You can pin up to {MAX_PINS} conversations. Unpin one to pin another."
            )));
        }
    }
    let mut batch = store.mail_batch()?;
    let mut accounts = Vec::new();
    for message in &stored {
        if batch.set_pinned(message.id, on.then_some(now))? {
            push_unique(&mut accounts, message.account);
        }
    }
    batch.commit()?;
    Ok(accounts)
}

/// Moves messages to `to`. Returns the accounts whose workers must replay.
pub fn move_messages(
    store: &mut Store,
    messages: &[MessageId],
    to: FolderId,
) -> Result<Vec<AccountId>, ChangeError> {
    move_messages_from(store, messages, None, to)
}

/// Moves messages out of folder `from` (any of theirs when `None`) to
/// `to`. Messages not in `from` stay. Returns the accounts whose workers
/// must replay.
pub fn move_messages_from(
    store: &mut Store,
    messages: &[MessageId],
    from: Option<FolderId>,
    to: FolderId,
) -> Result<Vec<AccountId>, ChangeError> {
    let mut plans = Vec::new();
    for &id in messages {
        let account = account_of(store, id)?;
        let folders = folders_of(store, account)?;
        let target = folders
            .get(&to)
            .ok_or(ChangeError::UnknownFolder(to.0))?
            .clone();
        // A Gmail message is in All Mail too; moving it from there would
        // only add a label, so move it from one of its other folders.
        let Some(source) = store
            .locations(id)?
            .into_iter()
            .filter(|location| location.folder != to)
            .filter(|location| from.is_none_or(|from| location.folder == from))
            .min_by_key(|location| {
                folders
                    .get(&location.folder)
                    .is_some_and(|f| f.role == Some(FolderRole::All))
            })
        else {
            continue; // already there
        };
        let from = &folders[&source.folder];
        let synced = is_synced(store, account)?;
        // Local folders have no UIDs; a message moved there a moment ago
        // gets its UID when that move runs on the server.
        let uid = source.uid;
        plans.push((
            account,
            synced,
            Op::Move {
                message: id.0,
                from: from.id.0,
                from_path: from.path.clone(),
                uid,
                to: target.id.0,
                to_path: target.path.clone(),
            },
        ));
    }
    let mut batch = store.mail_batch()?;
    let mut accounts = Vec::new();
    for (account, synced, op) in plans {
        let Op::Move {
            message, from, to, ..
        } = &op
        else {
            unreachable!("only moves are planned here");
        };
        batch.move_location(MessageId(*message), FolderId(*from), FolderId(*to), None)?;
        if synced {
            batch.enqueue_op(account, &encode(&op))?;
        }
        push_unique(&mut accounts, account);
    }
    batch.commit()?;
    Ok(accounts)
}

/// Copies messages into `to` as well (on Gmail: adds its label), keeping
/// them where they are. Returns the accounts whose workers must replay.
pub fn copy_messages(
    store: &mut Store,
    messages: &[MessageId],
    to: FolderId,
) -> Result<Vec<AccountId>, ChangeError> {
    let mut plans = Vec::new();
    for &id in messages {
        let account = account_of(store, id)?;
        let folders = folders_of(store, account)?;
        let target = folders
            .get(&to)
            .ok_or(ChangeError::UnknownFolder(to.0))?
            .clone();
        let locations = store.locations(id)?;
        if locations.iter().any(|location| location.folder == to) {
            continue; // already there
        }
        // One whose UID is known, so the copy need not wait.
        let Some(source) = locations.into_iter().min_by_key(|l| l.uid.is_none()) else {
            continue;
        };
        let from = &folders[&source.folder];
        plans.push((
            account,
            is_synced(store, account)?,
            Op::Copy {
                message: id.0,
                from: from.id.0,
                from_path: from.path.clone(),
                uid: source.uid,
                to: target.id.0,
                to_path: target.path.clone(),
            },
        ));
    }
    let mut batch = store.mail_batch()?;
    let mut accounts = Vec::new();
    for (account, synced, op) in plans {
        let Op::Copy { message, to, .. } = &op else {
            unreachable!("only copies are planned here");
        };
        batch.add_location(MessageId(*message), FolderId(*to), None)?;
        if synced {
            batch.enqueue_op(account, &encode(&op))?;
        }
        push_unique(&mut accounts, account);
    }
    batch.commit()?;
    Ok(accounts)
}

/// Moves messages to their account's Trash; messages already there are
/// deleted for good. Returns the accounts whose workers must replay.
pub fn delete_messages(
    store: &mut Store,
    messages: &[MessageId],
) -> Result<Vec<AccountId>, ChangeError> {
    let mut to_trash: HashMap<FolderId, Vec<MessageId>> = HashMap::new();
    let mut expunge = Vec::new();
    for &id in messages {
        let account = account_of(store, id)?;
        let folders = folders_of(store, account)?;
        let trash = store.trash_folder(account)?;
        let locations = store.locations(id)?;
        match trash {
            Some(trash) if !locations.iter().any(|l| l.folder == trash) => {
                to_trash.entry(trash).or_default().push(id);
            }
            _ => {
                let synced = is_synced(store, account)?;
                for location in locations {
                    let folder = &folders[&location.folder];
                    let uid = match location.uid {
                        Some(uid) => Some(uid),
                        None if !synced => None,
                        None => {
                            return Err(ChangeError::NotPossible(format!(
                                "message {} is still being moved; try again in a moment",
                                id.0
                            )));
                        }
                    };
                    expunge.push((account, folder.id, folder.path.clone(), id, uid));
                }
            }
        }
    }
    let mut accounts = Vec::new();
    for (trash, ids) in to_trash {
        for account in move_messages(store, &ids, trash)? {
            push_unique(&mut accounts, account);
        }
    }
    let mut batch = store.mail_batch()?;
    for (account, folder, path, id, uid) in expunge {
        batch.remove_from_folder(id, folder)?;
        if let Some(uid) = uid {
            let op = Op::Expunge {
                message: id.0,
                path,
                uid,
            };
            batch.enqueue_op(account, &encode(&op))?;
        }
        push_unique(&mut accounts, account);
    }
    batch.commit()?;
    Ok(accounts)
}

/// Moves messages to their account's archive folder (`\Archive`, or
/// `\All` on Gmail). Returns the accounts whose workers must replay.
pub fn archive_messages(
    store: &mut Store,
    messages: &[MessageId],
) -> Result<Vec<AccountId>, ChangeError> {
    let mut by_archive: HashMap<FolderId, Vec<MessageId>> = HashMap::new();
    for &id in messages {
        let account = account_of(store, id)?;
        let folders = folders_of(store, account)?;
        let archive = [FolderRole::Archive, FolderRole::All]
            .into_iter()
            .find_map(|role| folders.values().find(|f| f.role == Some(role)))
            .ok_or_else(|| {
                ChangeError::NotPossible(format!("account {account} has no archive folder"))
            })?;
        by_archive.entry(archive.id).or_default().push(id);
    }
    let mut accounts = Vec::new();
    for (archive, ids) in by_archive {
        for account in move_messages(store, &ids, archive)? {
            push_unique(&mut accounts, account);
        }
    }
    Ok(accounts)
}

/// Files a sent message in its account's Sent folder: queues an APPEND,
/// files it locally for accounts without an IMAP server (POP3), or forgets
/// the outgoing copy when there is no Sent folder. Returns whether the
/// worker has something to replay.
pub fn file_sent(store: &mut Store, message: MessageId) -> Result<bool, ChangeError> {
    let account = account_of(store, message)?;
    let sent = folders_of(store, account)?
        .into_values()
        .find(|f| f.role == Some(FolderRole::Sent));
    let synced = is_synced(store, account)?;
    let mut batch = store.mail_batch()?;
    let queued = match sent {
        Some(sent) if !synced => {
            batch.file_outgoing(message, sent.id)?;
            false
        }
        Some(sent) => {
            let op = Op::Append {
                message: message.0,
                folder: sent.id.0,
                path: sent.path,
            };
            batch.enqueue_op(account, &encode(&op))?;
            true
        }
        None => {
            batch.forget_outgoing(message)?;
            false
        }
    };
    batch.commit()?;
    Ok(queued)
}

/// Runs the due operations of `account` on `backend`. Returns `Err` only
/// when the connection broke; the operation it was running stays queued.
/// Leaves some folder selected.
pub async fn replay<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    now: i64,
) -> Result<ReplayReport> {
    let mut report = ReplayReport::default();
    let mut selected: Option<String> = None;
    loop {
        let due = store.due_ops(account, now, BATCH)?;
        if due.is_empty() {
            break;
        }
        for queued in due {
            let op = match serde_json::from_str::<Op>(&queued.op_json) {
                Ok(op) => op,
                Err(err) => {
                    tracing::warn!(id = queued.id, %err, "dropping an unreadable operation");
                    let mut batch = store.mail_batch()?;
                    batch.fail_op(queued.id)?;
                    batch.commit()?;
                    report.failed += 1;
                    continue;
                }
            };
            // A move that handed its new UID to a later one: read the
            // queue again so that one runs with it.
            let mut requeued = false;
            let result = run(
                backend,
                store,
                account,
                &op,
                &mut selected,
                &mut report,
                &mut requeued,
            )
            .await;
            let mut batch = store.mail_batch()?;
            match result {
                Ok(()) => {
                    batch.finish_op(queued.id)?;
                    report.done += 1;
                }
                Err(Error::Rejected(reason)) => {
                    if queued.attempts + 1 >= MAX_ATTEMPTS {
                        tracing::warn!(?op, %reason, "giving up an operation");
                        batch.fail_op(queued.id)?;
                        undo(&mut batch, &op)?;
                        report.failed += 1;
                    } else {
                        tracing::info!(?op, %reason, "operation refused; retrying later");
                        batch.retry_op(queued.id, now + RETRY_AFTER)?;
                        report.retried += 1;
                    }
                }
                Err(error) => return Err(error),
            }
            batch.commit()?;
            if requeued {
                break;
            }
        }
    }
    Ok(report)
}

/// Runs one operation.
async fn run<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    op: &Op,
    selected: &mut Option<String>,
    report: &mut ReplayReport,
    requeued: &mut bool,
) -> Result<()> {
    match op {
        Op::Flags {
            message,
            folder,
            path,
            uid,
            add,
            remove,
        } => {
            let uid = match uid {
                Some(uid) => *uid,
                None => {
                    let location = store
                        .locations(MessageId(*message))?
                        .into_iter()
                        .find(|l| l.folder == FolderId(*folder));
                    match location {
                        Some(location) => location.uid.ok_or_else(|| {
                            Error::Rejected("the message's move is not confirmed yet".into())
                        })?,
                        // Moved or deleted since: nothing to do there.
                        None => return Ok(()),
                    }
                }
            };
            select(backend, selected, path).await?;
            for (bits, add) in [(*add, true), (*remove, false)] {
                if bits != 0 {
                    let flags = flags(MessageFlags::from_bits(bits));
                    backend.store_flags(&[uid], &flags, add).await?;
                }
            }
        }
        Op::Move {
            message,
            from,
            from_path,
            uid,
            to,
            to_path,
        } => {
            let uid = match uid {
                Some(uid) => *uid,
                None => store
                    .locations(MessageId(*message))?
                    .into_iter()
                    .find(|l| l.folder == FolderId(*from))
                    .and_then(|l| l.uid)
                    .ok_or_else(|| {
                        Error::Rejected("the message's earlier move is not confirmed yet".into())
                    })?,
            };
            let uid = &uid;
            select(backend, selected, from_path).await?;
            let moved = backend.move_messages(&[*uid], to_path).await?;
            let mut batch = store.mail_batch()?;
            let (message, to) = (MessageId(*message), FolderId(*to));
            match moved.iter().find(|(old, _)| old == uid) {
                // The local copy now carries its server UID.
                Some(&(_, new)) => {
                    if !batch.move_location(message, to, to, Some(new))? {
                        // Moved on locally meanwhile (Undo): the queued move
                        // out of `to` takes the UID.
                        *requeued = hand_uid_on(&mut batch, account, message, to, new)?;
                    }
                }
                // Without UIDPLUS the next sync of `to` adds it again.
                None => {
                    if batch.unconfirmed_in(message, to)? {
                        batch.remove_from_folder(message, to)?;
                    }
                    if !report.resync.iter().any(|(id, _)| *id == to) {
                        report.resync.push((to, to_path.clone()));
                    }
                }
            }
            batch.commit()?;
        }
        Op::Copy {
            message,
            from,
            from_path,
            uid,
            to,
            to_path,
        } => {
            let uid = match uid {
                Some(uid) => *uid,
                None => store
                    .locations(MessageId(*message))?
                    .into_iter()
                    .find(|l| l.folder == FolderId(*from))
                    .and_then(|l| l.uid)
                    .ok_or_else(|| {
                        Error::Rejected("the message's earlier move is not confirmed yet".into())
                    })?,
            };
            select(backend, selected, from_path).await?;
            let copied = backend.copy_messages(&[uid], to_path).await?;
            let mut batch = store.mail_batch()?;
            let (message, to) = (MessageId(*message), FolderId(*to));
            match copied.iter().find(|(old, _)| *old == uid) {
                Some(&(_, new)) => {
                    batch.move_location(message, to, to, Some(new))?;
                }
                // Without UIDPLUS the next sync of `to` adds it again.
                None => {
                    if batch.unconfirmed_in(message, to)? {
                        batch.remove_from_folder(message, to)?;
                    }
                    if !report.resync.iter().any(|(id, _)| *id == to) {
                        report.resync.push((to, to_path.clone()));
                    }
                }
            }
            batch.commit()?;
        }
        Op::Expunge { path, uid, .. } => {
            select(backend, selected, path).await?;
            backend.expunge(&[*uid]).await?;
        }
        Op::Append {
            message,
            folder,
            path,
        } => {
            let message = MessageId(*message);
            let raw = store
                .messages_by_id(&[message])?
                .into_iter()
                .next()
                .and_then(|m| m.blob_hash)
                .map(|hash| store.blobs().get(&hash))
                .transpose()?
                .flatten();
            // Forgotten meanwhile (the account was cleared): nothing to file.
            let Some(raw) = raw else { return Ok(()) };
            let seen = Flags {
                seen: true,
                ..Flags::default()
            };
            backend.append_with_flags(path, raw, &seen).await?;
            let mut batch = store.mail_batch()?;
            batch.forget_outgoing(message)?;
            batch.commit()?;
            let folder = FolderId(*folder);
            if !report.resync.iter().any(|(id, _)| *id == folder) {
                report.resync.push((folder, path.clone()));
            }
        }
    }
    Ok(())
}

/// Undoes the local side of an operation the server refused for good.
/// Gives the queued move of `message` out of `folder` that waits for its
/// UID there the `uid` the server reported. Returns whether there was one.
fn hand_uid_on(
    batch: &mut katna_store::MailBatch<'_>,
    account: AccountId,
    message: MessageId,
    folder: FolderId,
    uid: u32,
) -> katna_store::Result<bool> {
    for queued in batch.pending_ops(account)? {
        let Ok(mut op) = serde_json::from_str::<Op>(&queued.op_json) else {
            continue;
        };
        if let Op::Move {
            message: m,
            from,
            uid: waiting @ None,
            ..
        } = &mut op
            && *m == message.0
            && *from == folder.0
        {
            *waiting = Some(uid);
            batch.set_op_json(queued.id, &encode(&op))?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn undo(batch: &mut katna_store::MailBatch<'_>, op: &Op) -> katna_store::Result<()> {
    match op {
        // The next sync fetches every flag of the folder again.
        Op::Flags { folder, .. } => batch.forget_modseq(FolderId(*folder)),
        Op::Move {
            message,
            from,
            uid,
            to,
            ..
        } => {
            batch.move_location(MessageId(*message), FolderId(*to), FolderId(*from), *uid)?;
            Ok(())
        }
        Op::Copy { message, to, .. } => {
            let (message, to) = (MessageId(*message), FolderId(*to));
            if batch.unconfirmed_in(message, to)? {
                batch.remove_from_folder(message, to)?;
            }
            Ok(())
        }
        // Gone locally; it stays on the server and in other clients.
        Op::Expunge { .. } => Ok(()),
        // It was sent; only the copy in Sent is missing.
        Op::Append { message, .. } => batch.forget_outgoing(MessageId(*message)),
    }
}

async fn select<B: MailBackend>(
    backend: &mut B,
    selected: &mut Option<String>,
    path: &str,
) -> Result<()> {
    if selected.as_deref() != Some(path) {
        *selected = None;
        backend.select(path).await?;
        *selected = Some(path.to_owned());
    }
    Ok(())
}

/// Protocol flags for store flag bits.
fn flags(bits: MessageFlags) -> Flags {
    let mut keywords = Vec::new();
    if bits.contains(MessageFlags::FORWARDED) {
        keywords.push("$Forwarded".to_owned());
    }
    if bits.contains(MessageFlags::IMPORTANT) {
        keywords.push(crate::IMPORTANT.to_owned());
    }
    Flags {
        seen: bits.contains(MessageFlags::SEEN),
        answered: bits.contains(MessageFlags::ANSWERED),
        flagged: bits.contains(MessageFlags::FLAGGED),
        deleted: bits.contains(MessageFlags::DELETED),
        draft: bits.contains(MessageFlags::DRAFT),
        keywords,
    }
}

fn account_of(store: &Store, id: MessageId) -> Result<AccountId, ChangeError> {
    store
        .messages_by_id(&[id])?
        .first()
        .map(|m| m.account)
        .ok_or(ChangeError::UnknownMessage(id.0))
}

fn folders_of(
    store: &Store,
    account: AccountId,
) -> Result<HashMap<FolderId, StoredFolder>, ChangeError> {
    Ok(store
        .folders(account)?
        .into_iter()
        .map(|folder| (folder.id, folder))
        .collect())
}

/// Whether `account`'s folders live on a server. Changes to imported and
/// POP3 mail stay local.
fn is_synced(store: &Store, account: AccountId) -> Result<bool, ChangeError> {
    Ok(store
        .accounts()?
        .into_iter()
        .any(|a| a.id == account && matches!(a.kind, AccountKind::Imap | AccountKind::Jmap)))
}

fn encode(op: &Op) -> String {
    serde_json::to_string(op).expect("operations serialize")
}

fn push_unique(accounts: &mut Vec<AccountId>, account: AccountId) {
    if !accounts.contains(&account) {
        accounts.push(account);
    }
}
