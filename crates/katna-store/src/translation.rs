// SPDX-License-Identifier: GPL-3.0-or-later

//! Translations of messages, kept so a message is translated once
//! (`docs/ARCHITECTURE.md` §16.3).

use rusqlite::{OptionalExtension, params};

use crate::error::Result;
use crate::{MessageId, Store};

/// A stored translation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation {
    /// The language the message was in.
    pub source: String,
    /// The translated text.
    pub text: String,
}

fn hash(text: &str) -> [u8; 32] {
    *blake3::hash(text.as_bytes()).as_bytes()
}

impl Store {
    /// The translation of `message` into `target`, when one was made of
    /// this same `text`.
    pub fn translation(
        &self,
        message: MessageId,
        target: &str,
        text: &str,
    ) -> Result<Option<Translation>> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT source, text FROM translation
                 WHERE message_id = ?1 AND target = ?2 AND source_hash = ?3",
            )?
            .query_row(params![message.0, target, hash(text)], |row| {
                Ok(Translation {
                    source: row.get(0)?,
                    text: row.get(1)?,
                })
            })
            .optional()?)
    }

    /// Keeps the translation of `message`'s `text` into `target`,
    /// replacing an earlier one.
    pub fn save_translation(
        &mut self,
        message: MessageId,
        target: &str,
        text: &str,
        translation: &Translation,
        at: i64,
    ) -> Result<()> {
        self.check_writable()?;
        self.mail
            .prepare_cached(
                "INSERT OR REPLACE INTO translation
                 (message_id, target, source, source_hash, text, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?
            .execute(params![
                message.0,
                target,
                translation.source,
                hash(text),
                translation.text,
                at
            ])?;
        Ok(())
    }

    /// Forgets every translation; they are made again when asked for.
    pub fn forget_translations(&mut self) -> Result<usize> {
        self.check_writable()?;
        Ok(self.mail.execute("DELETE FROM translation", [])?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Added, MessageFlags, Mode, RemoteMessage};
    use katna_core::{AccountKind, Paths};

    #[test]
    fn keeps_a_translation_of_the_same_text() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.upsert_folder(account, "INBOX", None).unwrap();
        let added = batch
            .add_remote_message(
                account,
                inbox,
                &RemoteMessage {
                    uid: 1,
                    message_id_hdr: None,
                    subject: Some("Hola"),
                    date: Some(1_790_000_000),
                    size: 10,
                    flags: MessageFlags::SEEN,
                    keywords: &[],
                    has_attachments: false,
                    list_id: None,
                    participants: &[],
                    in_reply_to: None,
                    references: &[],
                    gm_thread_id: None,
                    gm_msgid: None,
                    category: None,
                    attachments: &[],
                },
            )
            .unwrap();
        batch.commit().unwrap();
        let Added::Message(id) = added else {
            panic!("stored as {added:?}");
        };

        assert_eq!(store.translation(id, "en", "Hola").unwrap(), None);
        let hello = Translation {
            source: "es".into(),
            text: "Hello".into(),
        };
        store
            .save_translation(id, "en", "Hola", &hello, 1_790_000_100)
            .unwrap();
        assert_eq!(store.translation(id, "en", "Hola").unwrap(), Some(hello));
        // Another text, or another language, is not this one.
        assert_eq!(store.translation(id, "en", "Hola, Ana").unwrap(), None);
        assert_eq!(store.translation(id, "de", "Hola").unwrap(), None);

        let reader = Store::open(&Paths::with_root(tmp.path()), Mode::ReadOnly).unwrap();
        assert!(reader.translation(id, "en", "Hola").unwrap().is_some());

        assert_eq!(store.forget_translations().unwrap(), 1);
        assert_eq!(store.translation(id, "en", "Hola").unwrap(), None);
    }
}
