// SPDX-License-Identifier: GPL-3.0-or-later

//! Conversations summed up by AI (`docs/ARCHITECTURE.md` §16.5), kept on
//! this computer. A summary is found through the newest mail it covers,
//! so it outlives its thread being merged or rebuilt; the store keeps its
//! JSON as it is and never reads it.

use rusqlite::params;

use crate::mail::{MailBatch, MessageId};
use crate::{Result, Store};

/// What a summary covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummaryKind {
    /// The whole conversation.
    All,
    /// Only the mails that were unread when it was asked: a catch-up.
    New,
}

impl SummaryKind {
    fn id(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::New => "new",
        }
    }
}

/// A conversation's summary as kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredSummary {
    /// The newest mail it covers.
    pub message: MessageId,
    pub kind: SummaryKind,
    /// How many mails it covers.
    pub mails: u32,
    /// The summary, as JSON.
    pub body: String,
    /// Who answered: `katna` or `own`.
    pub service: String,
    pub created_at: i64,
}

fn placeholders(n: usize) -> String {
    vec!["?"; n].join(", ")
}

fn summaries_of(
    conn: &rusqlite::Connection,
    messages: &[MessageId],
) -> rusqlite::Result<Vec<StoredSummary>> {
    if messages.is_empty() {
        return Ok(Vec::new());
    }
    let mut stmt = conn.prepare(&format!(
        "SELECT message_id, kind, mails, body, service, created_at FROM conversation_summary
         WHERE message_id IN ({}) ORDER BY created_at DESC, id DESC",
        placeholders(messages.len())
    ))?;
    let rows = stmt.query_map(
        rusqlite::params_from_iter(messages.iter().map(|m| m.0)),
        |row| {
            let kind: String = row.get(1)?;
            Ok(StoredSummary {
                message: MessageId(row.get(0)?),
                kind: if kind == "new" {
                    SummaryKind::New
                } else {
                    SummaryKind::All
                },
                mails: row.get::<_, i64>(2)?.max(0) as u32,
                body: row.get(3)?,
                service: row.get(4)?,
                created_at: row.get(5)?,
            })
        },
    )?;
    rows.collect()
}

impl Store {
    /// The summaries of the conversation of `messages` (all its
    /// messages), the newest first: at most one of each kind.
    pub fn conversation_summaries(&self, messages: &[MessageId]) -> Result<Vec<StoredSummary>> {
        Ok(summaries_of(&self.mail, messages)?)
    }
}

impl MailBatch<'_> {
    /// Keeps a summary of `kind` of the conversation of `siblings` (all
    /// its messages) whose newest covered mail is `message`, in place of
    /// the one of that kind kept before.
    #[allow(clippy::too_many_arguments)]
    pub fn save_summary(
        &mut self,
        message: MessageId,
        siblings: &[MessageId],
        kind: SummaryKind,
        mails: u32,
        body: &str,
        service: &str,
        now: i64,
    ) -> Result<()> {
        if !siblings.is_empty() {
            let ids = siblings.iter().map(|m| m.0).chain([message.0]);
            self.tx().execute(
                &format!(
                    "DELETE FROM conversation_summary WHERE kind = ? AND message_id IN ({})",
                    placeholders(siblings.len() + 1)
                ),
                rusqlite::params_from_iter(
                    std::iter::once(rusqlite::types::Value::from(kind.id().to_owned()))
                        .chain(ids.map(rusqlite::types::Value::from)),
                ),
            )?;
        }
        self.tx()
            .prepare_cached(
                "INSERT INTO conversation_summary (message_id, kind, mails, body, service,
                                                   created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?
            .execute(params![
                message.0,
                kind.id(),
                i64::from(mails),
                body,
                service,
                now
            ])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
