// SPDX-License-Identifier: GPL-3.0-or-later

//! Which mail rings and counts (`docs/ARCHITECTURE.md` §15.1.1).
//!
//! Each folder has a bell: whether its new mail notifies and whether its
//! unread mail counts on the taskbar and tray. By default only an inbox's
//! Primary tab (unclassified mail included) does both; `folder_alert`
//! keeps the folders and inbox tabs that differ. On top of that, an
//! account, a folder, a conversation or a sender can be muted, for a
//! while or until unmuted (`mute`). Muted mail still arrives and stays
//! unread; it only never notifies and is not counted.
//!
//! The notifier, the taskbar and tray count and the apps all ask here, so
//! they agree.

use std::collections::HashSet;

use katna_core::{AccountId, MailCategory};
use rusqlite::{OptionalExtension, named_params, params};

use crate::Store;
use crate::error::Result;
use crate::mail::{FolderId, MailBatch, MessageFlags, MessageId, ThreadId};
use crate::remote::FolderRole;

/// `folder_alert.category` of a folder that is not an inbox.
const WHOLE_FOLDER: i64 = 0;

/// What is muted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MuteTarget {
    Account(AccountId),
    Folder(FolderId),
    /// A conversation.
    Thread(ThreadId),
    /// Mail from this address (lower case), in every account.
    Sender(String),
}

impl MuteTarget {
    fn kind(&self) -> &'static str {
        match self {
            Self::Account(_) => "account",
            Self::Folder(_) => "folder",
            Self::Thread(_) => "thread",
            Self::Sender(_) => "sender",
        }
    }

    /// `(account_id, folder_id, thread_id, address)` columns.
    fn columns(&self) -> (Option<i64>, Option<i64>, Option<i64>, Option<String>) {
        match self {
            Self::Account(a) => (Some(a.0), None, None, None),
            Self::Folder(f) => (None, Some(f.0), None, None),
            Self::Thread(t) => (None, None, Some(t.0), None),
            Self::Sender(address) => (None, None, None, Some(address.trim().to_lowercase())),
        }
    }

    /// The `WHERE` clause naming this target, with parameters `?1`..`?5`
    /// (kind, then [`Self::columns`]).
    const MATCH: &'static str = "kind = ?1 AND account_id IS ?2 AND folder_id IS ?3
         AND thread_id IS ?4 AND address IS ?5";
}

/// One mute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mute {
    pub target: MuteTarget,
    /// What Settings shows: a subject, a name or an address.
    pub label: String,
    /// When it ends (Unix seconds); `None` until unmuted.
    pub until: Option<i64>,
    /// The mail service keeps it too (Gmail's mute, `$muted`).
    pub server: bool,
    pub created_at: i64,
}

/// A folder's (or inbox tab's) bell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bell {
    /// New mail shows a notification.
    pub notify: bool,
    /// Unread mail counts on the taskbar and tray.
    pub count: bool,
}

impl Bell {
    pub const ON: Self = Self {
        notify: true,
        count: true,
    };
    pub const OFF: Self = Self {
        notify: false,
        count: false,
    };

    /// The bell of a folder (`category` `None`) or an inbox tab nobody
    /// changed: on for an inbox's Primary tab, off elsewhere.
    pub fn default_for(role: Option<&str>, category: Option<MailCategory>) -> Self {
        let inbox = role == Some(FolderRole::Inbox.as_str());
        if inbox && category.unwrap_or_default() == MailCategory::Primary {
            Self::ON
        } else {
            Self::OFF
        }
    }
}

/// A bell that differs from the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FolderBell {
    pub folder: FolderId,
    /// The inbox tab, or `None` for the whole folder.
    pub category: Option<MailCategory>,
    pub bell: Bell,
}

/// The SQL condition, over message `m`, that it rings: it is in a folder
/// (or inbox tab) whose bell has `column` on and that is not muted, and
/// neither its account, its conversation nor its sender is muted at
/// `:now`.
fn rings(column: &str) -> String {
    let primary = MailCategory::Primary.to_storage();
    let inbox = FolderRole::Inbox.as_str();
    format!(
        "EXISTS (SELECT 1 FROM message_location l
                 JOIN folder f ON f.id = l.folder_id
                 LEFT JOIN folder_alert a ON a.folder_id = f.id
                      AND a.category = CASE WHEN f.role = '{inbox}'
                          THEN coalesce(m.category, {primary}) ELSE {WHOLE_FOLDER} END
                 WHERE l.message_id = m.id
                   AND coalesce(a.{column},
                       f.role = '{inbox}' AND coalesce(m.category, {primary}) = {primary}) = 1
                   AND NOT EXISTS (SELECT 1 FROM mute q WHERE q.kind = 'folder'
                       AND q.folder_id = f.id AND (q.until IS NULL OR q.until > :now)))
         AND NOT EXISTS (SELECT 1 FROM mute q
                 WHERE (q.until IS NULL OR q.until > :now)
                   AND ((q.kind = 'account' AND q.account_id = m.account_id)
                     OR (q.kind = 'thread' AND q.thread_id = m.thread_id)
                     OR (q.kind = 'sender' AND q.address IN
                         (SELECT p.email_norm FROM participant p
                          WHERE p.message_id = m.id AND p.role = 'from'))))"
    )
}

fn category_column(category: Option<MailCategory>) -> i64 {
    category.map_or(WHOLE_FOLDER, MailCategory::to_storage)
}

fn mute_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Option<Mute>> {
    let kind: String = row.get(0)?;
    let target = match kind.as_str() {
        "account" => row
            .get::<_, Option<i64>>(1)?
            .map(|a| MuteTarget::Account(AccountId(a))),
        "folder" => row
            .get::<_, Option<i64>>(2)?
            .map(|f| MuteTarget::Folder(FolderId(f))),
        "thread" => row
            .get::<_, Option<i64>>(3)?
            .map(|t| MuteTarget::Thread(ThreadId(t))),
        "sender" => row.get::<_, Option<String>>(4)?.map(MuteTarget::Sender),
        _ => None,
    };
    let Some(target) = target else {
        return Ok(None);
    };
    Ok(Some(Mute {
        target,
        label: row.get(5)?,
        until: row.get(6)?,
        server: row.get(7)?,
        created_at: row.get(8)?,
    }))
}

impl Store {
    /// Mutes in force at `now`, newest first.
    pub fn mutes(&self, now: i64) -> Result<Vec<Mute>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT kind, account_id, folder_id, thread_id, address, label, until, server,
                    created_at
             FROM mute WHERE until IS NULL OR until > ?1 ORDER BY created_at DESC, id DESC",
        )?;
        let rows = stmt.query_map([now], mute_from_row)?;
        let mut out = Vec::new();
        for row in rows {
            out.extend(row?);
        }
        Ok(out)
    }

    /// The mute of `target` in force at `now`, if any.
    pub fn mute_of(&self, target: &MuteTarget, now: i64) -> Result<Option<Mute>> {
        Ok(self.mutes(now)?.into_iter().find(|m| &m.target == target))
    }

    /// When the next timed mute ends, if any is in force at `now`.
    pub fn next_mute_end(&self, now: i64) -> Result<Option<i64>> {
        Ok(self
            .mail
            .prepare_cached("SELECT min(until) FROM mute WHERE until > ?1")?
            .query_row([now], |row| row.get(0))?)
    }

    /// Conversations muted at `now`, for the list's marker.
    pub fn muted_threads(&self, now: i64) -> Result<HashSet<ThreadId>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT thread_id FROM mute WHERE kind = 'thread' AND thread_id IS NOT NULL
               AND (until IS NULL OR until > ?1)",
        )?;
        let rows = stmt.query_map([now], |row| row.get(0).map(ThreadId))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Whether `thread` is a Gmail conversation, which Gmail can mute.
    pub fn is_gmail_thread(&self, thread: ThreadId) -> Result<bool> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT 1 FROM message WHERE thread_id = ?1 AND gm_msgid IS NOT NULL LIMIT 1",
            )?
            .query_row([thread.0], |_| Ok(()))
            .optional()?
            .is_some())
    }

    /// Bells that differ from the default.
    pub fn folder_bells(&self) -> Result<Vec<FolderBell>> {
        let mut stmt = self
            .mail
            .prepare_cached("SELECT folder_id, category, notify, count FROM folder_alert")?;
        let rows = stmt.query_map([], |row| {
            Ok(FolderBell {
                folder: FolderId(row.get(0)?),
                category: MailCategory::from_storage(row.get(1)?),
                bell: Bell {
                    notify: row.get(2)?,
                    count: row.get(3)?,
                },
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Unread mail that counts on the taskbar and tray at `now`: every
    /// account's, each message once.
    pub fn counted_unread(&self, now: i64) -> Result<u64> {
        let sql = format!(
            "SELECT count(*) FROM message m WHERE (m.flags & :unwanted) = 0 AND {}",
            rings("count")
        );
        let unwanted = (MessageFlags::SEEN | MessageFlags::DELETED).bits();
        let count: i64 = self
            .mail
            .prepare_cached(&sql)?
            .query_row(named_params! {":unwanted": unwanted, ":now": now}, |row| {
                row.get(0)
            })?;
        Ok(count.unsigned_abs())
    }

    /// Unread mail of `account` newer than message `after` and dated
    /// `since` or later that notifies at `now`, oldest first; at most the
    /// newest `limit`.
    pub fn new_ringing_mail(
        &self,
        account: AccountId,
        after: MessageId,
        since: i64,
        limit: u32,
        now: i64,
    ) -> Result<Vec<MessageId>> {
        let sql = format!(
            "SELECT id FROM (
               SELECT m.id FROM message m
               WHERE m.account_id = :account AND m.id > :after AND (m.flags & :unwanted) = 0
                 AND m.date >= :since AND {}
               ORDER BY m.id DESC LIMIT :limit)
             ORDER BY id",
            rings("notify")
        );
        let unwanted = (MessageFlags::SEEN | MessageFlags::DELETED).bits();
        let mut stmt = self.mail.prepare_cached(&sql)?;
        let rows = stmt.query_map(
            named_params! {
                ":account": account.0,
                ":after": after.0,
                ":unwanted": unwanted,
                ":since": since,
                ":limit": limit,
                ":now": now,
            },
            |row| row.get(0).map(MessageId),
        )?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Those of `messages` that would still notify at `now`: unread and
    /// ringing. A notification whose mail no longer rings is closed.
    pub fn still_ringing(&self, messages: &[MessageId], now: i64) -> Result<Vec<MessageId>> {
        let sql = format!(
            "SELECT m.id FROM message m WHERE m.id = :id AND (m.flags & :unwanted) = 0 AND {}",
            rings("notify")
        );
        let unwanted = (MessageFlags::SEEN | MessageFlags::DELETED).bits();
        let mut stmt = self.mail.prepare_cached(&sql)?;
        let mut out = Vec::new();
        for id in messages {
            let found = stmt
                .query_row(
                    named_params! {":id": id.0, ":unwanted": unwanted, ":now": now},
                    |row| row.get(0).map(MessageId),
                )
                .optional()?;
            out.extend(found);
        }
        Ok(out)
    }
}

impl MailBatch<'_> {
    /// Mutes `target` until `until` (`None`: until unmuted), replacing an
    /// earlier mute of it.
    pub fn mute(
        &mut self,
        target: &MuteTarget,
        label: &str,
        until: Option<i64>,
        server: bool,
        now: i64,
    ) -> Result<()> {
        let (account, folder, thread, address) = target.columns();
        let tx = self.tx();
        tx.prepare_cached(&format!("DELETE FROM mute WHERE {}", MuteTarget::MATCH))?
            .execute(params![target.kind(), account, folder, thread, address])?;
        tx.prepare_cached(
            "INSERT INTO mute (kind, account_id, folder_id, thread_id, address, label, until,
                               server, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )?
        .execute(params![
            target.kind(),
            account,
            folder,
            thread,
            address,
            label,
            until,
            server,
            now
        ])?;
        Ok(())
    }

    /// Unmutes `target`. Returns whether it was muted.
    pub fn unmute(&mut self, target: &MuteTarget) -> Result<bool> {
        let (account, folder, thread, address) = target.columns();
        Ok(self
            .tx()
            .prepare_cached(&format!("DELETE FROM mute WHERE {}", MuteTarget::MATCH))?
            .execute(params![target.kind(), account, folder, thread, address])?
            > 0)
    }

    /// Forgets mutes that ended by `now`. Returns how many.
    pub fn drop_ended_mutes(&mut self, now: i64) -> Result<usize> {
        Ok(self
            .tx()
            .prepare_cached("DELETE FROM mute WHERE until IS NOT NULL AND until <= ?1")?
            .execute([now])?)
    }

    /// Sets the bell of `folder`, or of one of its inbox tabs (`None`:
    /// an inbox's Primary tab). A bell like the default is not kept.
    pub fn set_bell(
        &mut self,
        folder: FolderId,
        category: Option<MailCategory>,
        bell: Bell,
    ) -> Result<()> {
        let tx = self.tx();
        let role: Option<String> = tx
            .prepare_cached("SELECT role FROM folder WHERE id = ?1")?
            .query_row([folder.0], |row| row.get(0))
            .optional()?
            .flatten();
        // An inbox's bell is its Primary tab's (unclassified mail
        // included); other folders have one bell.
        let inbox = role.as_deref() == Some(FolderRole::Inbox.as_str());
        let category = if inbox {
            Some(category.unwrap_or_default())
        } else {
            None
        };
        let column = category_column(category);
        if bell == Bell::default_for(role.as_deref(), category) {
            tx.prepare_cached("DELETE FROM folder_alert WHERE folder_id = ?1 AND category = ?2")?
                .execute(params![folder.0, column])?;
        } else {
            tx.prepare_cached(
                "INSERT OR REPLACE INTO folder_alert (folder_id, category, notify, count)
                 VALUES (?1, ?2, ?3, ?4)",
            )?
            .execute(params![folder.0, column, bell.notify, bell.count])?;
        }
        Ok(())
    }

    /// Mutes or unmutes, for the mail service's sake, the conversations
    /// of messages that carry the service's mute flag: a conversation with
    /// a flagged message is muted (`server`), and one muted by the service
    /// whose messages all lost the flag is unmuted. Returns whether
    /// anything changed.
    pub fn follow_server_mutes(&mut self, now: i64) -> Result<bool> {
        let muted = MessageFlags::MUTED.bits();
        let tx = self.tx();
        let added = tx
            .prepare_cached(
                "INSERT INTO mute (kind, thread_id, label, server, created_at)
                 SELECT 'thread', t.id, t.subject_norm, 1, ?2 FROM thread t
                 WHERE t.id IN (SELECT m.thread_id FROM message m
                                WHERE (m.flags & ?1) != 0 AND m.thread_id IS NOT NULL)
                   AND NOT EXISTS (SELECT 1 FROM mute q WHERE q.kind = 'thread'
                                   AND q.thread_id = t.id)",
            )?
            .execute(params![muted, now])?;
        let removed = tx
            .prepare_cached(
                "DELETE FROM mute WHERE kind = 'thread' AND server = 1
                   AND NOT EXISTS (SELECT 1 FROM message m WHERE m.thread_id = mute.thread_id
                                   AND (m.flags & ?1) != 0)",
            )?
            .execute([muted])?;
        Ok(added + removed > 0)
    }
}

#[cfg(test)]
mod tests;
