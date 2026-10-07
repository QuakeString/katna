// SPDX-License-Identifier: GPL-3.0-or-later

//! Snooze and follow-up reminders (plan 7.5, 7.6; `docs/ARCHITECTURE.md`
//! §10), on the metadata scheduler of `katna-meta`.
//!
//! - Snooze moves a conversation's messages to the account's Snoozed
//!   folder (made on the server the first time, a label on Gmail) and
//!   remembers where they came from. When the time comes they go back,
//!   unread, sorted as if they had just arrived, with a notification.
//! - A follow-up reminder waits on a sent message. When it is due and the
//!   conversation has nothing newer (no reply, no second message; an
//!   auto-reply does not count), the message is put in the Inbox too (a
//!   label on Gmail), unread and on top, with a notification. Or Katna
//!   sends the follow-up the user wrote, in working hours, once or twice;
//!   one that fell due while the computer was off is not sent late but
//!   waits for the user.
//!
//! Both run only while the computer is on; one that fell due while it was
//! off fires when the daemon starts.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use jiff::tz::TimeZone;
use katna_core::{AccountId, AccountKind};
use katna_i18n::tr;
use katna_meta::{Due, FollowUp, Snooze};
use katna_store::{
    FolderId, FolderRole, MessageFlags, MessageId, ParticipantRole, SendState, Store, StoredFolder,
};
use katna_sync::ops::{self, ChangeError};

use super::{CommandError, Daemon, unix_now};

/// The folder snoozed mail waits in. Always this name on the server, so
/// Katna finds it again in any language; the app shows it translated.
pub const SNOOZED: &str = "Snoozed";

/// The earliest a snooze may end, from now.
const MIN_SNOOZE: i64 = 60;
/// How long a follow-up waits for its message to be sent or synced before
/// it is looked at again.
const RETRY: i64 = 3600;
/// Lines of a reminder about several conversations.
const LISTED: usize = 4;
/// How late a follow-up may still go out on its own: later than this (the
/// computer was off) it waits for the user.
const LATE: i64 = 24 * 3600;

/// Whether `path` is the Snoozed folder: at the top, or inside the Inbox
/// on servers that keep every folder there (`INBOX.Snoozed`).
pub fn is_snoozed_path(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower == "snoozed"
        || lower
            .strip_prefix("inbox")
            .and_then(|rest| rest.get(1..))
            .is_some_and(|name| name == "snoozed")
}

/// What a due value turned into: reminders to show.
struct Reminder {
    account: AccountId,
    summary: String,
    lines: Vec<String>,
    messages: Vec<MessageId>,
}

impl Daemon {
    /// Starts the scheduler.
    pub(super) fn start_scheduler(self: &Arc<Self>) {
        let (waker, wakes) = katna_meta::waker();
        if self.scheduler.set(waker).is_err() {
            return;
        }
        let daemon = Arc::downgrade(self);
        crate::threads::detach(
            "katna-scheduler",
            katna_meta::run(wakes, move |now| {
                let daemon = daemon.upgrade();
                async move {
                    let daemon = daemon?;
                    daemon.tick(now).await
                }
            }),
        );
    }

    /// Has the scheduler look at the clock and the values again.
    pub(super) fn wake_scheduler(&self) {
        if let Some(waker) = self.scheduler.get() {
            waker.wake();
        }
    }

    /// Acts on everything due at `now`. Returns the next expiry.
    async fn tick(self: &Arc<Self>, now: i64) -> Option<i64> {
        if self.closing() {
            return None;
        }
        if self.end_mutes(now) {
            self.mail_changed_everywhere();
        }
        let due = match katna_meta::due(&self.store(), now) {
            Ok(due) => due,
            Err(err) => {
                tracing::warn!(%err, "reading due metadata");
                return Some(now + katna_meta::MAX_SLEEP.as_secs() as i64);
            }
        };
        let mut snoozes = Vec::new();
        let mut reminders = Vec::new();
        for due in due {
            match due {
                Due::Snooze(message, snooze) => snoozes.push((message, snooze)),
                Due::FollowUp(outbox, follow_up) => {
                    match self.follow_up_due(outbox, &follow_up, now) {
                        Ok(reminder) => reminders.extend(reminder),
                        Err(err) => {
                            tracing::warn!(%err, outbox, "follow-up reminder");
                            let mut store = self.store();
                            let _ = katna_meta::clear_follow_up(&mut store, outbox);
                        }
                    }
                    self.follow_up_changed(follow_up.account);
                }
                Due::Surfaced(message) => {
                    let _ = katna_meta::clear_surfaced(&mut self.store(), message);
                }
                Due::ReadAfter(message) => self.read_after_due(message),
                Due::Other(row) => {
                    // A value of a newer version: kept, but not due again.
                    tracing::info!(plugin = row.plugin, "unknown metadata expired");
                    let _ = self.store().set_meta(
                        &row.object_kind,
                        row.object_id,
                        &row.plugin,
                        &row.value_json,
                        None,
                    );
                }
            }
        }
        if !snoozes.is_empty() {
            match self.wake_snoozed(&snoozes, now) {
                Ok(back) => reminders.extend(back),
                Err(err) => tracing::warn!(%err, "bringing snoozed mail back"),
            }
        }
        if let Some(notices) = self.new_mail_notices() {
            for reminder in reminders {
                notices
                    .remind(
                        &self.store,
                        reminder.account,
                        &reminder.summary,
                        &reminder.lines,
                        reminder.messages,
                    )
                    .await;
            }
        }
        let next = self.store().next_meta_expiry().ok().flatten();
        match (next, self.next_mute_end(now)) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    /// Snoozes `messages` until `until` (Unix seconds): they move to their
    /// account's Snoozed folder, which is made first if needed (on the
    /// server, so this needs it the first time). Messages already snoozed
    /// get the new time. Messages only in Sent, Drafts, Trash, Spam or All
    /// Mail stay where they are.
    pub async fn snooze(&self, messages: &[MessageId], until: i64) -> Result<(), CommandError> {
        if until < unix_now() + MIN_SNOOZE {
            return Err(CommandError::InvalidArgs(
                "a snooze ends at least a minute from now".into(),
            ));
        }
        let stored = self.store().messages_by_id(messages)?;
        if let Some(missing) = messages
            .iter()
            .find(|id| !stored.iter().any(|m| m.id == **id))
        {
            return Err(CommandError::UnknownMessage(missing.0));
        }
        let mut by_account: BTreeMap<AccountId, Vec<MessageId>> = BTreeMap::new();
        for message in &stored {
            by_account
                .entry(message.account)
                .or_default()
                .push(message.id);
        }
        for (account, ids) in by_account {
            let snoozed_in = self.snoozed_folder(account).await?;
            self.change(|store| snooze_in_store(store, &ids, until, snoozed_in))?;
        }
        self.wake_scheduler();
        Ok(())
    }

    /// Brings snoozed `messages` back where they were, now, as they are
    /// (read stays read). Others are left alone.
    pub fn unsnooze(&self, messages: &[MessageId]) -> Result<(), CommandError> {
        self.change(|store| {
            let mut accounts = Vec::new();
            for &message in messages {
                let Some(snooze) = katna_meta::snooze_of(store, message)? else {
                    continue;
                };
                accounts.extend(bring_back(store, message, &snooze)?);
                katna_meta::clear_snooze(store, message)?;
            }
            accounts.sort_by_key(|a| a.0);
            accounts.dedup();
            Ok(accounts)
        })?;
        self.wake_scheduler();
        Ok(())
    }

    /// Reminds the user `after` seconds after outbox entry `outbox` is sent
    /// if nobody replied by then; 0 takes the reminder back.
    pub fn set_follow_up(&self, outbox: i64, after: i64) -> Result<(), CommandError> {
        self.set_follow_up_with(outbox, after, None, 0)
    }

    /// Sends `mail` (a follow-up threaded under the message, without
    /// `Date` and `Message-ID`) `after` seconds after outbox entry `outbox`
    /// is sent if nobody replied by then, in working hours; with `again`,
    /// a second time that many seconds later. 0 takes it back.
    pub fn set_follow_up_mail(
        &self,
        outbox: i64,
        after: i64,
        again: i64,
        mail: &[u8],
    ) -> Result<(), CommandError> {
        let mail = std::str::from_utf8(mail)
            .map_err(|_| CommandError::InvalidArgs("the follow-up is not UTF-8 text".into()))?;
        let envelope = katna_sync::outbox::envelope(mail.as_bytes()).map_err(|err| {
            CommandError::InvalidArgs(format!("the follow-up cannot be sent: {err}"))
        })?;
        if envelope.to.is_empty() {
            return Err(CommandError::InvalidArgs(
                "the follow-up has no recipients".into(),
            ));
        }
        self.set_follow_up_with(outbox, after, Some(mail.to_owned()), again.max(0))
    }

    /// Moves the follow-up of outbox entry `outbox` to `at` (Unix
    /// seconds). One that waited for the user goes out then, in working
    /// hours.
    pub fn move_follow_up(&self, outbox: i64, at: i64) -> Result<(), CommandError> {
        if at < unix_now() + MIN_SNOOZE {
            return Err(CommandError::InvalidArgs(
                "the follow-up time has passed".into(),
            ));
        }
        let mut store = self.store();
        let mut follow_up = katna_meta::follow_up_of(&store, outbox)?
            .ok_or_else(|| CommandError::InvalidArgs(format!("no follow-up on {outbox}")))?;
        follow_up.remind_at = at;
        follow_up.waiting = false;
        katna_meta::set_follow_up(&mut store, outbox, &follow_up)?;
        drop(store);
        tracing::info!(outbox, "follow-up moved");
        self.follow_up_changed(follow_up.account);
        self.wake_scheduler();
        Ok(())
    }

    /// Tells the apps a follow-up of `account` changed: its lines and the
    /// Waiting for reply view show it.
    fn follow_up_changed(&self, account: i64) {
        let _ = self
            .notices
            .try_send(super::Notice::MailChanged(AccountId(account)));
    }

    /// Sends a follow-up that waits for the user now, as it is.
    pub fn send_follow_up_now(&self, outbox: i64) -> Result<(), CommandError> {
        let follow_up = katna_meta::follow_up_of(&self.store(), outbox)?
            .filter(FollowUp::sends)
            .ok_or_else(|| CommandError::InvalidArgs(format!("no follow-up on {outbox}")))?;
        self.send_follow_up(outbox, &follow_up, unix_now())?;
        self.follow_up_changed(follow_up.account);
        Ok(())
    }

    fn set_follow_up_with(
        &self,
        outbox: i64,
        after: i64,
        mail: Option<String>,
        again: i64,
    ) -> Result<(), CommandError> {
        let mut store = self.store();
        if after <= 0 {
            let account = katna_meta::follow_up_of(&store, outbox)?.map(|f| f.account);
            katna_meta::clear_follow_up(&mut store, outbox)?;
            drop(store);
            if let Some(account) = account {
                self.follow_up_changed(account);
            }
            return Ok(());
        }
        let entry = store
            .outbox_entry(outbox)?
            .ok_or_else(|| CommandError::InvalidArgs(format!("no outbox entry {outbox}")))?;
        if matches!(entry.state, SendState::Cancelled | SendState::Failed) {
            return Err(CommandError::InvalidArgs(format!(
                "outbox entry {outbox} will not be sent"
            )));
        }
        let message_id = store.message_id_header(entry.message)?.ok_or_else(|| {
            CommandError::InvalidArgs(format!("outbox entry {outbox} has no Message-ID"))
        })?;
        let follow_up = FollowUp {
            account: entry.account.0,
            message_id,
            subject: entry.subject,
            // Scheduled mail the server holds counts from when it goes out.
            remind_at: entry
                .hold_until
                .map_or(entry.send_at, |hold| hold.max(entry.send_at))
                .saturating_add(after),
            after,
            mail,
            again,
            ..FollowUp::default()
        };
        katna_meta::set_follow_up(&mut store, outbox, &follow_up)?;
        drop(store);
        tracing::info!(
            outbox,
            after,
            again,
            sends = follow_up.sends(),
            "follow-up set"
        );
        self.follow_up_changed(follow_up.account);
        self.wake_scheduler();
        Ok(())
    }

    /// The account's Snoozed folder, made if it has none.
    async fn snoozed_folder(&self, account: AccountId) -> Result<FolderId, CommandError> {
        let found = self
            .store()
            .folders(account)?
            .into_iter()
            .find(|f| is_snoozed_path(&f.path));
        if let Some(folder) = found {
            return Ok(folder.id);
        }
        if self.account(account)?.kind == AccountKind::Imap {
            return self.create_folder(account, SNOOZED, None).await;
        }
        // POP3 and imported mail: a folder on this computer.
        let mut store = self.store();
        let mut batch = store.mail_batch()?;
        let id = batch.ensure_folder(account, SNOOZED)?;
        batch.commit()?;
        Ok(id)
    }

    /// Brings `snoozes` back: to where they came from, unread, on top.
    /// Returns a reminder per account.
    fn wake_snoozed(
        &self,
        snoozes: &[(MessageId, Snooze)],
        now: i64,
    ) -> Result<Vec<Reminder>, CommandError> {
        let mut back: BTreeMap<AccountId, Vec<MessageId>> = BTreeMap::new();
        self.change(|store| {
            let mut accounts = Vec::new();
            for (message, snooze) in snoozes {
                let moved = bring_back(store, *message, snooze)?;
                katna_meta::clear_snooze(store, *message)?;
                let Some(&account) = moved.first() else {
                    continue; // moved elsewhere meanwhile, or gone
                };
                accounts.extend(ops::set_flags(
                    store,
                    &[*message],
                    MessageFlags::empty(),
                    MessageFlags::SEEN,
                )?);
                katna_meta::set_surfaced(store, *message, now)?;
                back.entry(account).or_default().push(*message);
                accounts.push(account);
            }
            accounts.sort_by_key(|a| a.0);
            accounts.dedup();
            Ok(accounts)
        })?;
        tracing::info!(
            count = back.values().map(Vec::len).sum::<usize>(),
            "snoozed mail back"
        );
        let store = self.store();
        Ok(back
            .into_iter()
            .map(|(account, messages)| Reminder {
                account,
                summary: tr!("notify-snooze-back"),
                lines: conversation_lines(&store, &messages),
                messages,
            })
            .collect())
    }

    /// A follow-up reminder of outbox entry `outbox` is due at `now`.
    /// Returns what to show, if anything.
    fn follow_up_due(
        &self,
        outbox: i64,
        follow_up: &FollowUp,
        now: i64,
    ) -> Result<Option<Reminder>, CommandError> {
        let account = AccountId(follow_up.account);
        let later = |store: &mut Store| {
            let mut again = follow_up.clone();
            again.remind_at = now + RETRY;
            katna_meta::set_follow_up(store, outbox, &again)
        };
        let mut store = self.store();
        match store.outbox_entry(outbox)?.map(|e| e.state) {
            // Not sent yet (a retry): look again later.
            Some(SendState::Queued | SendState::Sending) => {
                later(&mut store)?;
                return Ok(None);
            }
            Some(SendState::Cancelled | SendState::Failed) => {
                katna_meta::clear_follow_up(&mut store, outbox)?;
                return Ok(None);
            }
            Some(SendState::Sent) | None => {}
        }
        let copies = store.messages_with_header(account, &follow_up.message_id)?;
        if copies.is_empty() && now < follow_up.remind_at + 24 * 3600 {
            // Sent, but not in Sent yet (offline): wait for the sync.
            later(&mut store)?;
            return Ok(None);
        }
        let ours = &follow_up.sent;
        let mut replied = false;
        for &copy in &copies {
            replied |= store
                .later_in_thread(copy)?
                .iter()
                .any(|later| katna_meta::counts_as_reply(later, ours));
        }
        if replied {
            katna_meta::clear_follow_up(&mut store, outbox)?;
            tracing::info!(outbox, "follow-up not needed: the conversation went on");
            return Ok(None);
        }
        if follow_up.sends() {
            let tz = TimeZone::system();
            if now > katna_meta::working_time(follow_up.remind_at, &tz) + LATE {
                // Not sent a day or more late: it waits for the user.
                let mut waiting = follow_up.clone();
                waiting.waiting = true;
                katna_meta::set_follow_up(&mut store, outbox, &waiting)?;
                drop(store);
                tracing::info!(outbox, "follow-up waits: it fell due while off");
                let message = self.surface_sent(account, copies.first().copied(), now)?;
                return Ok(Some(Reminder {
                    account,
                    summary: tr!("notify-follow-up-waiting"),
                    lines: vec![tr!(
                        "notify-follow-up-waiting-to",
                        subject = subject_or_none(&follow_up.subject)
                    )],
                    messages: message.into_iter().collect(),
                }));
            }
            let at = katna_meta::working_time(now, &tz);
            if at > now {
                let mut later = follow_up.clone();
                later.remind_at = at;
                katna_meta::set_follow_up(&mut store, outbox, &later)?;
                return Ok(None);
            }
            drop(store);
            return self.send_follow_up(outbox, follow_up, now).map(Some);
        }
        katna_meta::clear_follow_up(&mut store, outbox)?;
        drop(store);
        let message = self.surface_sent(account, copies.first().copied(), now)?;
        tracing::info!(outbox, "follow-up reminder");
        Ok(Some(Reminder {
            account,
            summary: tr!("notify-no-reply"),
            lines: vec![tr!(
                "notify-no-reply-to",
                subject = subject_or_none(&follow_up.subject)
            )],
            messages: message.into_iter().collect(),
        }))
    }

    /// Puts the sent `message` of `account` in the Inbox too, unread and on
    /// top. Returns it.
    fn surface_sent(
        &self,
        account: AccountId,
        message: Option<MessageId>,
        now: i64,
    ) -> Result<Option<MessageId>, CommandError> {
        let inbox = self
            .store()
            .folders(account)?
            .into_iter()
            .find(|f| f.role == Some(FolderRole::Inbox))
            .map(|f| f.id);
        if let (Some(message), Some(inbox)) = (message, inbox) {
            self.change(|store| {
                let mut accounts = ops::copy_messages(store, &[message], inbox)?;
                accounts.extend(ops::set_flags(
                    store,
                    &[message],
                    MessageFlags::empty(),
                    MessageFlags::SEEN,
                )?);
                katna_meta::set_surfaced(store, message, now)?;
                accounts.dedup();
                Ok(accounts)
            })?;
        }
        Ok(message)
    }

    /// Sends the follow-up of outbox entry `outbox` now, and sets the
    /// second one if there is to be one.
    fn send_follow_up(
        &self,
        outbox: i64,
        follow_up: &FollowUp,
        now: i64,
    ) -> Result<Reminder, CommandError> {
        let account = AccountId(follow_up.account);
        let mail = follow_up.mail.as_deref().unwrap_or_default();
        let id = self.queue_send(account, mail.as_bytes(), 0)?;
        let mut store = self.store();
        let sent = match store.outbox_entry(id)? {
            Some(entry) => store.message_id_header(entry.message)?,
            None => None,
        };
        let mut next = follow_up.clone();
        next.sent.extend(sent);
        next.waiting = false;
        if follow_up.sends_again() {
            next.remind_at = now.saturating_add(next.again);
            katna_meta::set_follow_up(&mut store, outbox, &next)?;
        } else {
            katna_meta::clear_follow_up(&mut store, outbox)?;
        }
        let messages = store.messages_with_header(account, &follow_up.message_id)?;
        drop(store);
        tracing::info!(outbox, id, step = next.sent.len(), "follow-up sent");
        self.wake_scheduler();
        Ok(Reminder {
            account,
            summary: tr!("notify-follow-up-sent"),
            lines: vec![tr!(
                "notify-follow-up-sent-to",
                subject = subject_or_none(&follow_up.subject)
            )],
            messages,
        })
    }
}

/// Snoozes `ids` (of one account) in the store: records where each came
/// from and moves it to `snoozed_in`.
fn snooze_in_store(
    store: &mut Store,
    ids: &[MessageId],
    until: i64,
    snoozed_in: FolderId,
) -> Result<Vec<AccountId>, ChangeError> {
    let stored = store.messages_by_id(ids)?;
    let Some(account) = stored.first().map(|m| m.account) else {
        return Ok(Vec::new());
    };
    let folders: HashMap<FolderId, StoredFolder> = store
        .folders(account)?
        .into_iter()
        .map(|f| (f.id, f))
        .collect();
    let mut moves: BTreeMap<FolderId, Vec<MessageId>> = BTreeMap::new();
    for &id in ids {
        let locations = store.locations(id)?;
        if locations.iter().any(|l| l.folder == snoozed_in) {
            // Snoozed already: only the time changes.
            if let Some(mut snooze) = katna_meta::snooze_of(store, id)? {
                snooze.until = until;
                katna_meta::set_snooze(store, id, &snooze)?;
            }
            continue;
        }
        let Some(from) = source(&locations, &folders) else {
            continue;
        };
        katna_meta::set_snooze(
            store,
            id,
            &Snooze {
                until,
                back_to: from.0,
                snoozed_in: snoozed_in.0,
            },
        )?;
        moves.entry(from).or_default().push(id);
    }
    let mut accounts = Vec::new();
    for (from, ids) in moves {
        accounts.extend(ops::move_messages_from(
            store,
            &ids,
            Some(from),
            snoozed_in,
        )?);
    }
    accounts.dedup();
    Ok(accounts)
}

/// The folder a message is snoozed from: the Inbox if it is there, else
/// a folder of its own (a label, Archive), not Sent, Drafts, Trash, Spam or
/// All Mail.
fn source(
    locations: &[katna_store::Location],
    folders: &HashMap<FolderId, StoredFolder>,
) -> Option<FolderId> {
    let of = |role| {
        locations
            .iter()
            .find(|l| folders.get(&l.folder).is_some_and(|f| f.role == Some(role)))
            .map(|l| l.folder)
    };
    of(FolderRole::Inbox).or_else(|| {
        locations
            .iter()
            .find(|l| {
                folders.get(&l.folder).is_some_and(|f| {
                    !is_snoozed_path(&f.path) && matches!(f.role, None | Some(FolderRole::Archive))
                })
            })
            .map(|l| l.folder)
    })
}

/// Moves a snoozed message back to where it came from (the Inbox if that
/// folder is gone). Returns its account if it moved: nothing moves when it
/// left the Snoozed folder meanwhile.
fn bring_back(
    store: &mut Store,
    message: MessageId,
    snooze: &Snooze,
) -> Result<Vec<AccountId>, ChangeError> {
    let snoozed_in = FolderId(snooze.snoozed_in);
    if !store
        .locations(message)?
        .iter()
        .any(|l| l.folder == snoozed_in)
    {
        return Ok(Vec::new());
    }
    let Some(account) = store.messages_by_id(&[message])?.first().map(|m| m.account) else {
        return Ok(Vec::new());
    };
    let folders = store.folders(account)?;
    let to = folders
        .iter()
        .find(|f| f.id.0 == snooze.back_to)
        .or_else(|| folders.iter().find(|f| f.role == Some(FolderRole::Inbox)))
        .map(|f| f.id);
    let Some(to) = to else {
        return Ok(Vec::new());
    };
    ops::move_messages_from(store, &[message], Some(snoozed_in), to)
}

fn subject_or_none(subject: &str) -> String {
    if subject.trim().is_empty() {
        tr!("notify-no-subject")
    } else {
        subject.to_owned()
    }
}

/// "Sender: Subject" for each conversation of `messages`, the first few.
fn conversation_lines(store: &Store, messages: &[MessageId]) -> Vec<String> {
    let Ok(stored) = store.messages_by_id(messages) else {
        return Vec::new();
    };
    let mut threads = Vec::new();
    let mut lines = Vec::new();
    for message in &stored {
        if let Some(thread) = message.thread_id {
            if threads.contains(&thread) {
                continue;
            }
            threads.push(thread);
        }
        let from = message.first(ParticipantRole::From);
        let sender = from
            .and_then(|p| p.display_name.clone())
            .filter(|name| !name.trim().is_empty())
            .or_else(|| from.map(|p| p.email_norm.clone()))
            .unwrap_or_else(|| tr!("notify-unknown-sender"));
        let subject = if message.subject.trim().is_empty() {
            tr!("notify-no-subject")
        } else {
            message.subject.clone()
        };
        lines.push(format!("{sender}: {subject}"));
    }
    if lines.len() > LISTED {
        let more = lines.len() - LISTED;
        lines.truncate(LISTED);
        lines.push(tr!("notify-and-more", count = more));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_snoozed_folder() {
        assert!(is_snoozed_path("Snoozed"));
        assert!(is_snoozed_path("snoozed"));
        assert!(is_snoozed_path("INBOX.Snoozed"));
        assert!(is_snoozed_path("INBOX/Snoozed"));
        assert!(!is_snoozed_path("Work/Snoozed"));
        assert!(!is_snoozed_path("Snoozed old"));
        assert!(!is_snoozed_path("INBOX"));
    }
}
