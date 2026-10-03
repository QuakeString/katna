// SPDX-License-Identifier: GPL-3.0-or-later

//! Pins in a chat (`docs/ARCHITECTURE.md`, the chat view): a mail, one of its
//! files or text picked from it, pinned to the top of its conversation
//! when the conversation shows as a chat. Up to five a conversation, on
//! this computer only. A conversation's pins are read through its
//! messages, so they outlive its thread being merged or rebuilt.

use rusqlite::params;

use crate::mail::{MailBatch, MessageId};
use crate::{Result, Store};

/// The most pins a conversation holds.
pub const MAX_CHAT_PINS: usize = 5;

/// What a pin holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pinned {
    /// The whole mail.
    Mail,
    /// One of its files, by its place among the mail's attachments.
    File(usize),
    /// Text picked from it.
    Text(String),
}

impl Pinned {
    fn columns(&self) -> (&'static str, Option<i64>, Option<&str>) {
        match self {
            Self::Mail => ("mail", None, None),
            Self::File(order) => ("file", Some(*order as i64), None),
            Self::Text(text) => ("text", None, Some(text.as_str())),
        }
    }
}

/// A pin in a chat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatPin {
    pub id: i64,
    pub message: MessageId,
    pub what: Pinned,
    /// What the pin bar shows: a subject, a file name or the text.
    pub label: String,
    pub created_at: i64,
}

fn placeholders(n: usize) -> String {
    vec!["?"; n].join(", ")
}

fn pins_of(conn: &rusqlite::Connection, messages: &[MessageId]) -> rusqlite::Result<Vec<ChatPin>> {
    if messages.is_empty() {
        return Ok(Vec::new());
    }
    let mut stmt = conn.prepare(&format!(
        "SELECT id, message_id, kind, file_order, text, label, created_at FROM chat_pin
         WHERE message_id IN ({}) ORDER BY position, id",
        placeholders(messages.len())
    ))?;
    let ids = messages.iter().map(|m| m.0);
    let rows = stmt.query_map(rusqlite::params_from_iter(ids), |row| {
        let kind: String = row.get(2)?;
        let what = match kind.as_str() {
            "file" => Pinned::File(row.get::<_, Option<i64>>(3)?.unwrap_or(0).max(0) as usize),
            "text" => Pinned::Text(row.get::<_, Option<String>>(4)?.unwrap_or_default()),
            _ => Pinned::Mail,
        };
        Ok(ChatPin {
            id: row.get(0)?,
            message: MessageId(row.get(1)?),
            what,
            label: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    rows.collect()
}

impl Store {
    /// The pins of the conversation of `messages` (all its messages), in
    /// their order.
    pub fn chat_pins(&self, messages: &[MessageId]) -> Result<Vec<ChatPin>> {
        Ok(pins_of(&self.mail, messages)?)
    }
}

impl MailBatch<'_> {
    /// Pins `what` of `message` first among the pins of its conversation
    /// (`siblings`: all its messages). Returns the new pin's ID, or `None`
    /// when the conversation holds [`MAX_CHAT_PINS`] already or the same
    /// thing is pinned.
    pub fn pin_in_chat(
        &mut self,
        message: MessageId,
        siblings: &[MessageId],
        what: &Pinned,
        label: &str,
        now: i64,
    ) -> Result<Option<i64>> {
        let pins = pins_of(self.tx(), siblings)?;
        if pins.len() >= MAX_CHAT_PINS
            || pins.iter().any(|p| p.message == message && p.what == *what)
        {
            return Ok(None);
        }
        let first: i64 = if pins.is_empty() {
            0
        } else {
            self.tx().query_row(
                &format!(
                    "SELECT min(position) FROM chat_pin WHERE message_id IN ({})",
                    placeholders(siblings.len())
                ),
                rusqlite::params_from_iter(siblings.iter().map(|m| m.0)),
                |row| row.get(0),
            )?
        };
        let (kind, file_order, text) = what.columns();
        self.tx()
            .prepare_cached(
                "INSERT INTO chat_pin (message_id, kind, file_order, text, label, position,
                                       created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )?
            .execute(params![
                message.0,
                kind,
                file_order,
                text,
                label,
                first - 1,
                now
            ])?;
        Ok(Some(self.tx().last_insert_rowid()))
    }

    /// Takes off pin `id`. Returns whether there was one.
    pub fn unpin_in_chat(&mut self, id: i64) -> Result<bool> {
        Ok(self
            .tx()
            .prepare_cached("DELETE FROM chat_pin WHERE id = ?1")?
            .execute([id])?
            > 0)
    }

    /// Puts pins `ids` in this order.
    pub fn order_chat_pins(&mut self, ids: &[i64]) -> Result<()> {
        let mut stmt = self
            .tx()
            .prepare_cached("UPDATE chat_pin SET position = ?2 WHERE id = ?1")?;
        for (position, id) in ids.iter().enumerate() {
            stmt.execute(params![id, position as i64])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
