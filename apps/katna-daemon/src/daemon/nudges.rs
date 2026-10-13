// SPDX-License-Identifier: GPL-3.0-or-later

//! Nudges: after a sync, looks once at each message the user sent in the
//! last two weeks for whether it asked something, and keeps the answer
//! beside it (`katna_meta::Nudge`). The app brings the questions nobody
//! answered in three days back to the top of the Inbox. Nothing changes on
//! the mail server. See `docs/ARCHITECTURE.md` §10.1.

use std::collections::HashSet;

use katna_core::AccountId;
use katna_meta::{NUDGE_UNTIL, Nudge};
use katna_store::{MessageId, Store};

use super::{CommandError, Daemon, unix_now};

impl Daemon {
    /// Looks at the sent mail not looked at yet, and forgets what it found
    /// on mail too old for a nudge. Settings > Inbox > Nudges turns it off.
    pub(super) fn find_nudges(&self) {
        if !super::settings(self.paths()).mail.nudges {
            return;
        }
        let now = unix_now();
        let mut store = self.store();
        let found = look_at_sent(&mut store, now);
        drop(store);
        match found {
            Ok(asking) => {
                // The apps show the new ones.
                for account in asking {
                    let _ = self.notices.try_send(super::Notice::MailChanged(account));
                }
            }
            Err(err) => tracing::warn!(%err, "looking at sent mail for nudges"),
        }
    }

    /// Dismisses the nudge on sent message `message`: it does not come
    /// back.
    pub fn dismiss_nudge(&self, message: i64) -> Result<(), CommandError> {
        let message = MessageId(message);
        let mut store = self.store();
        let mut nudge = katna_meta::nudge_of(&store, message)?
            .ok_or_else(|| CommandError::InvalidArgs(format!("no nudge on {}", message.0)))?;
        nudge.dismissed = true;
        katna_meta::set_nudge(&mut store, message, &nudge)?;
        let account = store.messages_by_id(&[message])?.first().map(|m| m.account);
        drop(store);
        tracing::info!(message = message.0, "nudge dismissed");
        if let Some(account) = account {
            let _ = self.notices.try_send(super::Notice::MailChanged(account));
        }
        Ok(())
    }
}

/// Records whether each sent message of the last [`NUDGE_UNTIL`] asked
/// something, once; returns the accounts with a new question. A message
/// whose body is not downloaded yet waits, unless its preview shows a
/// question.
fn look_at_sent(store: &mut Store, now: i64) -> katna_store::Result<Vec<AccountId>> {
    let mut known: HashSet<MessageId> = HashSet::new();
    let mut old = Vec::new();
    for (message, nudge) in katna_meta::nudges(store)? {
        if now - nudge.sent > NUDGE_UNTIL {
            old.push(message);
        } else {
            known.insert(message);
        }
    }
    for message in old {
        katna_meta::clear_nudge(store, message)?;
    }
    // Mail with a follow-up set has its own reminder.
    let mut followed: HashSet<MessageId> = HashSet::new();
    for (_, follow_up) in katna_meta::follow_ups(store)? {
        followed.extend(
            store.messages_with_header(AccountId(follow_up.account), &follow_up.message_id)?,
        );
    }
    let sent: Vec<MessageId> = store
        .sent_between(now - NUDGE_UNTIL, now)?
        .into_iter()
        .filter(|id| !known.contains(id))
        .collect();
    let mut asking = Vec::new();
    if sent.is_empty() {
        return Ok(asking);
    }
    for message in store.messages_by_id(&sent)? {
        let Some(date) = message.date else {
            continue;
        };
        let text = match message
            .blob_hash
            .as_ref()
            .and_then(|hash| store.blobs().get(hash).ok().flatten())
        {
            Some(raw) => katna_search::document::message_text(&raw).body,
            None => match &message.snippet {
                Some(snippet) if katna_meta::asks(snippet) => snippet.clone(),
                // Looked at again once its body is here.
                _ => continue,
            },
        };
        // Mailing lists and mail with a follow-up get no nudge.
        let asks =
            message.list_id.is_none() && !followed.contains(&message.id) && katna_meta::asks(&text);
        let nudge = Nudge {
            sent: date,
            asks,
            dismissed: false,
        };
        katna_meta::set_nudge(store, message.id, &nudge)?;
        if asks && !asking.contains(&message.account) {
            asking.push(message.account);
        }
    }
    tracing::debug!(accounts = asking.len(), "sent mail looked at for nudges");
    Ok(asking)
}
