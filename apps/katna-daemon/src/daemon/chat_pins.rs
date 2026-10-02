// SPDX-License-Identifier: GPL-3.0-or-later

//! Pins in a chat (`docs/ARCHITECTURE.md`, the chat view), kept on this computer:
//! no mail service has pins inside a conversation.

use katna_store::{MessageId, Pinned};

use super::{CommandError, Daemon, Notice, unix_now};

impl Daemon {
    /// Pins `what` of `message` first in its conversation, taking off pin
    /// `replace` first. `None` when the conversation holds five pins or
    /// the same thing is pinned.
    pub fn pin_in_chat(
        &self,
        message: MessageId,
        what: Pinned,
        label: &str,
        replace: Option<i64>,
    ) -> Result<Option<i64>, CommandError> {
        let (account, siblings) = {
            let store = self.store();
            let stored = store
                .messages_by_id(&[message])?
                .into_iter()
                .next()
                .ok_or(CommandError::UnknownMessage(message.0))?;
            let siblings = match stored.thread_id {
                Some(thread) => store.thread_messages(thread)?,
                None => vec![message],
            };
            (stored.account, siblings)
        };
        let pinned = {
            let mut store = self.store();
            let mut batch = store.mail_batch()?;
            if let Some(replace) = replace {
                batch.unpin_in_chat(replace)?;
            }
            let pinned = batch.pin_in_chat(message, &siblings, &what, label, unix_now())?;
            batch.commit()?;
            pinned
        };
        let _ = self.notices.try_send(Notice::MailChanged(account));
        Ok(pinned)
    }

    /// Takes off pin `id`.
    pub fn unpin_in_chat(&self, id: i64) -> Result<(), CommandError> {
        {
            let mut store = self.store();
            let mut batch = store.mail_batch()?;
            batch.unpin_in_chat(id)?;
            batch.commit()?;
        }
        self.mail_changed_everywhere();
        Ok(())
    }

    /// Puts a conversation's pins in this order.
    pub fn order_chat_pins(&self, ids: &[i64]) -> Result<(), CommandError> {
        {
            let mut store = self.store();
            let mut batch = store.mail_batch()?;
            batch.order_chat_pins(ids)?;
            batch.commit()?;
        }
        self.mail_changed_everywhere();
        Ok(())
    }
}
