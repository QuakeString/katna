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
//! 3. New messages: headers for UIDs above the highest stored one, in chunks,
//!    committed chunk by chunk so an interrupted first sync keeps its
//!    progress.
//! 4. Expunged messages: when the message count does not add up, compare the
//!    server's UID list with ours.
//! 5. Save UIDVALIDITY and the HIGHESTMODSEQ seen at SELECT. Changes made
//!    during the sync have higher mod-sequences, so the next run sees them.
//!
//! The store is only written between network calls, never while waiting
//! on the server.

use std::collections::HashSet;

use katna_core::AccountId;
use katna_store::{FolderId, MessageFlags, NewParticipant, RemoteMessage, Store};

use crate::{Error, FlagState, Flags, Folder, MailBackend, MessageHeaders, Result};

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
}

/// Syncs the folder list of `account`, then every selectable folder.
pub async fn sync_account<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
) -> Result<Vec<FolderReport>> {
    let folders = sync_folders(backend, store, account).await?;
    let mut reports = Vec::with_capacity(folders.len());
    for (folder, id) in folders {
        if !folder.selectable {
            continue;
        }
        match sync_folder(backend, store, account, id, &folder.name).await {
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
    let mut report = FolderReport {
        path: path.to_owned(),
        ..FolderReport::default()
    };
    let status = backend.select(path).await?;
    let stored = store.folders(account)?.into_iter().find(|f| f.id == folder);
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
    }
    let old_modseq = if report.reset { None } else { old_modseq };

    // 2. Flags of the messages we already have.
    let last_known = known.last().copied().unwrap_or(0);
    let modseq_unchanged = old_modseq.is_some()
        && status.highest_modseq.is_some()
        && old_modseq == status.highest_modseq;
    if last_known > 0 && !modseq_unchanged {
        let states = backend.fetch_flags(1, last_known, old_modseq).await?;
        report.flags_changed = save_flags(store, folder, &states)?;
    }

    // 3. New messages.
    let mut first = last_known + 1;
    let mut added_uids = 0;
    loop {
        let last = match status.uid_next {
            Some(next) if first >= next => break,
            Some(next) => Some((first.saturating_add(CHUNK - 1)).min(next - 1)),
            // Without UIDNEXT we cannot chunk: take the rest in one go.
            None => None,
        };
        let messages = backend.fetch_headers(first, last).await?;
        added_uids += messages.len();
        report.added += save_messages(store, account, folder, &messages)?;
        match last {
            Some(last) => first = last + 1,
            None => break,
        }
    }

    // 4. Expunges. Skipped when the numbers already agree.
    let expected = known.len() + added_uids;
    if status.exists as usize != expected && !known.is_empty() {
        let on_server: HashSet<u32> = backend.uids().await?.into_iter().collect();
        let gone: Vec<u32> = known
            .into_iter()
            .filter(|uid| !on_server.contains(uid))
            .collect();
        if !gone.is_empty() {
            let mut batch = store.mail_batch()?;
            report.removed = batch.remove_remote_messages(folder, &gone)?;
            batch.commit()?;
        }
    }

    // 5. Where to continue next time.
    let mut batch = store.mail_batch()?;
    batch.set_folder_state(folder, status.uid_validity, status.highest_modseq, None)?;
    batch.commit()?;
    tracing::debug!(?report, "folder synced");
    Ok(report)
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
        let remote = RemoteMessage {
            uid: message.uid,
            message_id_hdr: parsed.message_id.as_deref(),
            subject: parsed.subject.as_deref(),
            date: parsed.date.or(message.received),
            size: u64::from(message.size),
            flags,
            keywords: &keywords,
            has_attachments: looks_like_attachments(&message.header),
            list_id: parsed.list_id.as_deref(),
            participants: &participants,
            references: &parsed.references,
        };
        if matches!(
            batch.add_remote_message(account, folder, &remote)?,
            katna_store::Added::Message(_)
        ) {
            added += 1;
        }
    }
    batch.commit()?;
    Ok(added)
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
        } else {
            keywords.push(keyword.clone());
        }
    }
    (bits, keywords)
}

/// Until `BODYSTRUCTURE` is parsed: `multipart/mixed` usually means
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
            keywords: vec!["$Forwarded".into(), "$Label1".into()],
            ..Flags::default()
        };
        let (bits, keywords) = split_flags(&flags);
        assert_eq!(
            bits,
            MessageFlags::SEEN | MessageFlags::FLAGGED | MessageFlags::FORWARDED
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
