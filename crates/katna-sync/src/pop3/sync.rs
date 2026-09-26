// SPDX-License-Identifier: GPL-3.0-or-later

//! Downloads a POP3 maildrop into the store (plan task 1.10,
//! `docs/ARCHITECTURE.md` §6.4).
//!
//! POP3 mail is always fully local, stored like imported mail. The account
//! has local folders only: `INBOX`, `Sent` and `Trash`. Each session:
//!
//! 1. Lists the maildrop (`UIDL` and `LIST`).
//! 2. Downloads every message whose UIDL the store does not know, newest
//!    first, and stores each one with its UIDL at once, so a broken
//!    connection loses nothing.
//! 3. Deletes what [`Pop3Keep`] says should go: everything once stored,
//!    mail older than `days`, or mail deleted for good in Katna.
//! 4. Ends the session (`QUIT`), which is when the server really deletes.
//!    Only then does the store forget those UIDLs, and the ones the server
//!    no longer lists.

use std::collections::{HashMap, HashSet};

use katna_core::{AccountId, Pop3Keep};
use katna_store::{
    FolderId, FolderRole, MessageFlags, NewMessage, NewParticipant, Pop3Uidl, Store,
};

use super::{DropEntry, Maildrop};
use crate::Result;

/// The local folders of a POP3 account, with their roles.
pub const FOLDERS: [(&str, FolderRole); 3] = [
    ("INBOX", FolderRole::Inbox),
    ("Sent", FolderRole::Sent),
    ("Trash", FolderRole::Trash),
];

/// What one session did.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Pop3Report {
    /// Messages downloaded and stored.
    pub added: usize,
    /// Messages deleted from the server.
    pub deleted: usize,
}

/// Creates the local folders of a POP3 account. Returns the inbox.
pub fn ensure_folders(store: &mut Store, account: AccountId) -> Result<FolderId> {
    let mut batch = store.mail_batch()?;
    let mut inbox = None;
    for (path, role) in FOLDERS {
        let id = batch.upsert_folder(account, path, Some(role))?;
        if role == FolderRole::Inbox {
            inbox = Some(id);
        }
    }
    batch.commit()?;
    Ok(inbox.expect("FOLDERS has an inbox"))
}

/// Runs one session on `maildrop` and ends it. `progress` is called with
/// the number of messages stored so far, every [`PROGRESS_EVERY`].
pub async fn sync_account<M: Maildrop>(
    mut maildrop: M,
    store: &mut Store,
    account: AccountId,
    keep: &Pop3Keep,
    now: i64,
    mut progress: impl FnMut(usize) + Send,
) -> Result<Pop3Report> {
    let inbox = ensure_folders(store, account)?;
    let entries = maildrop.entries().await?;
    let known: HashMap<String, Pop3Uidl> = store
        .pop3_uidls(account)?
        .into_iter()
        .map(|row| (row.uidl.clone(), row))
        .collect();

    let mut report = Pop3Report::default();
    let mut delete: Vec<&DropEntry> = Vec::new();
    for entry in entries.iter().rev() {
        match known.get(&entry.uidl) {
            Some(row) => {
                if should_delete(row, keep, now) {
                    delete.push(entry);
                }
            }
            None => {
                let raw = maildrop.message(entry.number).await?;
                store_message(store, account, inbox, &entry.uidl, &raw, now)?;
                report.added += 1;
                if report.added % PROGRESS_EVERY == 0 {
                    progress(report.added);
                }
                if !keep.leave_on_server {
                    delete.push(entry);
                }
            }
        }
    }
    for entry in &delete {
        maildrop.delete(entry.number).await?;
    }
    maildrop.finish().await?;
    report.deleted = delete.len();

    // The server has now deleted them.
    let on_server: HashSet<&str> = entries.iter().map(|e| e.uidl.as_str()).collect();
    let forget: Vec<&str> = known
        .keys()
        .map(String::as_str)
        .filter(|uidl| !on_server.contains(uidl))
        .chain(delete.iter().map(|e| e.uidl.as_str()))
        .collect();
    if !forget.is_empty() {
        let mut batch = store.mail_batch()?;
        batch.forget_pop3_uidls(account, &forget)?;
        batch.commit()?;
    }
    Ok(report)
}

/// How often [`sync_account`] reports progress while downloading.
pub const PROGRESS_EVERY: usize = 100;

fn should_delete(row: &Pop3Uidl, keep: &Pop3Keep, now: i64) -> bool {
    !keep.leave_on_server
        || (keep.delete_with_local && row.message.is_none())
        || keep
            .days
            .is_some_and(|days| now - row.first_seen >= i64::from(days) * 86_400)
}

fn store_message(
    store: &mut Store,
    account: AccountId,
    inbox: FolderId,
    uidl: &str,
    raw: &[u8],
    now: i64,
) -> Result<()> {
    // Something that is not mail is still stored, rather than downloaded
    // again and again.
    let parsed = katna_import::parse_message(raw).unwrap_or_else(|| {
        tracing::warn!(uidl, "message has no header");
        katna_import::ParsedMessage::default()
    });
    let references = parsed.reference_strs();
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
    let new = NewMessage {
        raw,
        message_id_hdr: parsed.message_id.as_deref(),
        subject: parsed.subject.as_deref(),
        date: parsed.date.or(Some(now)),
        flags: MessageFlags::empty(),
        has_attachments: parsed.has_attachments,
        list_id: parsed.list_id.as_deref(),
        snippet: parsed.snippet.as_deref(),
        participants: &participants,
        in_reply_to: parsed.in_reply_to.as_deref(),
        references: &references,
        category: Some(parsed.category),
    };
    let mut batch = store.mail_batch()?;
    batch.add_pop3_message(account, inbox, uidl, &new, now)?;
    batch.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(message: Option<i64>, first_seen: i64) -> Pop3Uidl {
        Pop3Uidl {
            uidl: "u".into(),
            message: message.map(katna_store::MessageId),
            first_seen,
        }
    }

    #[test]
    fn keeps_mail_as_told() {
        let day = 86_400;
        let default = Pop3Keep::default();
        assert!(!should_delete(&row(Some(1), 0), &default, 400 * day));
        assert!(should_delete(&row(None, 0), &default, 0));

        let forever = Pop3Keep {
            delete_with_local: false,
            ..Pop3Keep::default()
        };
        assert!(!should_delete(&row(None, 0), &forever, 400 * day));

        let two_weeks = Pop3Keep {
            days: Some(14),
            ..forever
        };
        assert!(!should_delete(&row(Some(1), 0), &two_weeks, 14 * day - 1));
        assert!(should_delete(&row(Some(1), 0), &two_weeks, 14 * day));

        let fetch_and_delete = Pop3Keep {
            leave_on_server: false,
            ..Pop3Keep::default()
        };
        assert!(should_delete(&row(Some(1), 0), &fetch_and_delete, 0));
    }
}
