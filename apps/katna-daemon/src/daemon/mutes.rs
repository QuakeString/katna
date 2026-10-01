// SPDX-License-Identifier: GPL-3.0-or-later

//! Folder bells and mutes (`docs/ARCHITECTURE.md` §15.1.1): what notifies
//! and counts on the taskbar and tray. The rule itself is the store's
//! (`katna_store::alerts`); this keeps it and the mail service in step.
//!
//! A muted conversation is muted at the mail service too where it can be:
//! Gmail's mute (its Muted label), elsewhere the `$muted` keyword (RFC
//! 9979). Gmail's mute is followed both ways, so a conversation muted or
//! unmuted on the phone is here too. On other servers the keyword is a
//! courtesy to other apps, and Katna's own record decides.

use katna_core::{AccountId, MailCategory};
use katna_store::{Bell, FolderId, MessageFlags, MessageId, MuteTarget};
use katna_sync::ops;

use super::{CommandError, Daemon, unix_now};

/// What to mute, as the apps name it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MuteOf {
    Account(AccountId),
    Folder(FolderId),
    /// The conversation of this message.
    Conversation(MessageId),
    /// Mail from this address.
    Sender(String),
}

impl Daemon {
    /// Mutes `what` until `until` (Unix seconds; `None` until unmuted).
    pub fn mute(&self, what: MuteOf, until: Option<i64>) -> Result<(), CommandError> {
        let now = unix_now();
        if until.is_some_and(|until| until <= now) {
            return Err(CommandError::InvalidArgs("a mute ends after now".into()));
        }
        match what {
            MuteOf::Conversation(message) => self.mute_conversation(message, until, true)?,
            other => {
                let (target, label) = self.target(&other)?;
                let mut store = self.store();
                let mut batch = store.mail_batch()?;
                batch.mute(&target, &label, until, false, now)?;
                batch.commit()?;
            }
        }
        self.mutes_changed();
        Ok(())
    }

    /// Unmutes `what`; nothing happens when it was not muted.
    pub fn unmute(&self, what: MuteOf) -> Result<(), CommandError> {
        match what {
            MuteOf::Conversation(message) => self.mute_conversation(message, None, false)?,
            other => {
                let (target, _) = self.target(&other)?;
                let mut store = self.store();
                let mut batch = store.mail_batch()?;
                batch.unmute(&target)?;
                batch.commit()?;
            }
        }
        self.mutes_changed();
        Ok(())
    }

    /// Sets whether `folder` (or its inbox tab `category`) notifies and
    /// counts.
    pub fn set_bell(
        &self,
        folder: FolderId,
        category: Option<MailCategory>,
        bell: Bell,
    ) -> Result<(), CommandError> {
        {
            let mut store = self.store();
            if !store.folder_summaries()?.iter().any(|f| f.id == folder) {
                return Err(CommandError::UnknownFolder(folder.0));
            }
            let mut batch = store.mail_batch()?;
            batch.set_bell(folder, category, bell)?;
            batch.commit()?;
        }
        self.mutes_changed();
        Ok(())
    }

    /// Mutes (`on`) or unmutes the conversation of `message`, here and at
    /// the mail service.
    fn mute_conversation(
        &self,
        message: MessageId,
        until: Option<i64>,
        on: bool,
    ) -> Result<(), CommandError> {
        let (thread, subject, ids, gmail) = {
            let store = self.store();
            let stored = store
                .messages_by_id(&[message])?
                .into_iter()
                .next()
                .ok_or(CommandError::UnknownMessage(message.0))?;
            let thread = stored.thread_id.ok_or_else(|| {
                CommandError::Failed(
                    "This conversation is still being sorted. Try again soon.".into(),
                )
            })?;
            let ids = store.thread_messages(thread)?;
            let gmail = store.is_gmail_thread(thread)?;
            (thread, stored.subject, ids, gmail)
        };
        {
            let mut store = self.store();
            let mut batch = store.mail_batch()?;
            if on {
                // Gmail keeps a mute for good; a timed one is Katna's alone.
                let server = gmail && until.is_none();
                batch.mute(
                    &MuteTarget::Thread(thread),
                    &subject,
                    until,
                    server,
                    unix_now(),
                )?;
            } else {
                batch.unmute(&MuteTarget::Thread(thread))?;
            }
            batch.commit()?;
        }
        // The mail service's mute is for good; a timed one stays here.
        let (add, remove) = match (on, until) {
            (true, None) => (MessageFlags::MUTED, MessageFlags::empty()),
            (true, Some(_)) => return Ok(()),
            (false, _) => (MessageFlags::empty(), MessageFlags::MUTED),
        };
        self.change(|store| ops::set_flags(store, &ids, add, remove))
    }

    /// The store's name for `what`, and what Settings shows for it.
    fn target(&self, what: &MuteOf) -> Result<(MuteTarget, String), CommandError> {
        let store = self.store();
        Ok(match what {
            MuteOf::Account(account) => {
                let found = store
                    .accounts()?
                    .into_iter()
                    .find(|a| a.id == *account)
                    .ok_or(CommandError::UnknownAccount(account.0))?;
                (MuteTarget::Account(*account), found.address)
            }
            MuteOf::Folder(folder) => {
                let found = store
                    .folder_summaries()?
                    .into_iter()
                    .find(|f| f.id == *folder)
                    .ok_or(CommandError::UnknownFolder(folder.0))?;
                (MuteTarget::Folder(*folder), found.path)
            }
            MuteOf::Sender(address) => {
                let address = address.trim().to_lowercase();
                if !address.contains('@') {
                    return Err(CommandError::InvalidArgs(format!(
                        "{address:?} is not a mail address"
                    )));
                }
                (MuteTarget::Sender(address.clone()), address)
            }
            MuteOf::Conversation(message) => {
                // Handled by `mute_conversation`; named here for errors.
                return Err(CommandError::UnknownMessage(message.0));
            }
        })
    }

    /// After a sync of a Gmail account: follows mutes made or undone at
    /// Gmail. Returns whether any changed.
    pub(super) fn follow_server_mutes(&self) -> bool {
        let mut store = self.store();
        let changed = store.mail_batch().and_then(|mut batch| {
            let changed = batch.follow_server_mutes(unix_now())?;
            batch.commit()?;
            Ok(changed)
        });
        changed.unwrap_or_else(|err| {
            tracing::warn!(%err, "following the mail service's mutes");
            false
        })
    }

    /// Forgets mutes that ended by `now`. Returns whether any did.
    pub(super) fn end_mutes(&self, now: i64) -> bool {
        let mut store = self.store();
        let ended = store.mail_batch().and_then(|mut batch| {
            let ended = batch.drop_ended_mutes(now)?;
            batch.commit()?;
            Ok(ended)
        });
        match ended {
            Ok(ended) => ended > 0,
            Err(err) => {
                tracing::warn!(%err, "ending mutes");
                false
            }
        }
    }

    /// When the next timed mute ends.
    pub(super) fn next_mute_end(&self, now: i64) -> Option<i64> {
        self.store().next_mute_end(now).ok().flatten()
    }

    /// After a mute or bell changed: notifications of now quiet mail
    /// close, every app and the taskbar count look again, and the
    /// scheduler learns when a timed mute ends.
    pub(super) fn mutes_changed(&self) {
        if let Some(notices) = self.new_mail_notices() {
            let handled = notices.handled(&self.store(), None);
            if !handled.is_empty() {
                smol::spawn(async move { notices.close(handled).await }).detach();
            }
        }
        self.mail_changed_everywhere();
        self.wake_scheduler();
    }

    /// Tells the apps and the taskbar count that every account's mail
    /// may look different.
    pub(super) fn mail_changed_everywhere(&self) {
        if let Ok(accounts) = self.store().accounts() {
            for account in accounts {
                let _ = self
                    .notices
                    .try_send(super::Notice::MailChanged(account.id));
            }
        }
    }
}
