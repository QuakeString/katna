// SPDX-License-Identifier: GPL-3.0-or-later

//! New-mail notifications (`docs/ARCHITECTURE.md` §15.1): after each sync,
//! unread mail that reached an account's inbox (Primary tab) since the last
//! look becomes one notification per account and sync, with Open, Reply all
//! (for one message), Mark as read and Archive. Notifications close when their mail is read or leaves
//! the inbox anywhere.

use std::{
    collections::{HashMap, HashSet},
    sync::{
        Mutex, Weak,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures_lite::{FutureExt, StreamExt};
use katna_core::AccountId;
use katna_core::config::Notifications;
use katna_i18n::tr;
use katna_notify::{NewMail, Notifier, action};
use katna_store::{FolderRole, MessageFlags, MessageId, ParticipantRole, Store};
use zbus::zvariant::Value;

use crate::daemon::Daemon;
use crate::daemon::alarms::{self, Alarm};

/// Older mail is not news, even when it is new to the store (a folder
/// synced for the first time, mail moved back into the inbox).
const NEWER_THAN: Duration = Duration::from_secs(2 * 24 * 3600);
/// At most this many messages go into one notification.
const PER_NOTIFICATION: u32 = 50;

/// Unread inbox mail newer than this many is not looked for: a day of mail
/// for a busy account, far more than one notification shows.
const CANDIDATES: u32 = 1000;

/// What was already looked at for one account.
#[derive(Clone, Debug)]
struct Seen {
    /// Messages up to this one were stored before watching began.
    after: MessageId,
    /// `false` until the account's first sync: a new account's mail is
    /// not news.
    primed: bool,
    /// Newer messages already announced (or skipped while notifications
    /// were off). A message is news when it reaches the inbox, not when it
    /// is stored: Gmail's new mail can land in All Mail a sync before its
    /// inbox copy, and its tab can change from Updates to Primary.
    announced: HashSet<MessageId>,
}

impl Seen {
    /// Unread inbox mail dated `since` or later that was not looked at
    /// yet; nothing the first time, when all mail is old news.
    fn fresh(
        &mut self,
        store: &Store,
        account: AccountId,
        since: i64,
    ) -> katna_store::Result<Vec<MessageId>> {
        if !self.primed {
            self.after = store.latest_message(account)?;
            self.primed = true;
            return Ok(Vec::new());
        }
        Ok(store
            .new_inbox_mail(account, self.after, since, CANDIDATES)?
            .into_iter()
            .filter(|id| self.announced.insert(*id))
            .collect())
    }
}

/// New mail to announce: the account's address, what to show, and the
/// messages.
struct Found {
    origin: String,
    mails: Vec<NewMail>,
    messages: Vec<MessageId>,
}

/// A notification on screen.
#[derive(Clone, Debug)]
struct Shown {
    account: AccountId,
    messages: Vec<MessageId>,
}

pub(crate) struct NewMailNotices {
    notifier: Notifier,
    connection: zbus::Connection,
    enabled: AtomicBool,
    /// Notifications play the new-mail sound.
    sound: AtomicBool,
    seen: Mutex<HashMap<AccountId, Seen>>,
    shown: Mutex<HashMap<u32, Shown>>,
    /// Activation tokens, sent by the server just before an action.
    tokens: Mutex<HashMap<u32, String>>,
    /// Event reminders on show.
    events: Mutex<HashMap<u32, Alarm>>,
    /// Snoozed event reminders and when they show again.
    snoozed: Mutex<Vec<Alarm>>,
    /// Notes that mail was archived from a notification, for their Undo.
    archived: Mutex<HashMap<u32, Shown>>,
}

impl NewMailNotices {
    pub(crate) async fn new(
        connection: &zbus::Connection,
        settings: &Notifications,
    ) -> zbus::Result<Self> {
        Ok(Self {
            notifier: Notifier::new(connection).await?,
            connection: connection.clone(),
            enabled: AtomicBool::new(settings.new_mail),
            sound: AtomicBool::new(settings.sound),
            seen: Mutex::default(),
            shown: Mutex::default(),
            tokens: Mutex::default(),
            events: Mutex::default(),
            snoozed: Mutex::default(),
            archived: Mutex::default(),
        })
    }

    /// Applies the `notifications` settings.
    pub(crate) fn set(&self, settings: &Notifications) {
        self.enabled.store(settings.new_mail, Ordering::Relaxed);
        self.sound.store(settings.sound, Ordering::Relaxed);
    }

    /// Starts watching `account`: mail stored from now on is news. An
    /// account without any mail yet waits for its first sync.
    pub(crate) fn watch(&self, store: &Store, account: AccountId) {
        let latest = match store.latest_message(account) {
            Ok(latest) => latest,
            Err(err) => {
                tracing::warn!(%err, %account, "no new-mail notifications");
                return;
            }
        };
        self.seen.lock().unwrap().entry(account).or_insert(Seen {
            after: latest,
            primed: latest.0 > 0,
            announced: HashSet::new(),
        });
    }

    /// Shows that a tracked message was opened or a link in it followed,
    /// when notifications are on. Open shows `message` (the copy in Sent),
    /// when it is known.
    pub(crate) async fn tracking(
        &self,
        summary: &str,
        body: &str,
        account: AccountId,
        message: Option<MessageId>,
    ) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        match self.notifier.tracking(summary, body).await {
            Ok(id) => {
                if let Some(message) = message {
                    self.shown.lock().unwrap().insert(
                        id,
                        Shown {
                            account,
                            messages: vec![message],
                        },
                    );
                }
            }
            Err(err) => tracing::warn!(%err, "could not show a tracking notification"),
        }
    }

    /// Shows the reminder of a calendar event or a task.
    pub(crate) async fn event_reminder(&self, alarm: Alarm) {
        let sound = self.sound.load(Ordering::Relaxed);
        let shown = match alarm.task {
            Some(_) => {
                self.notifier
                    .task_reminder(&alarm.title, &alarm.lines, sound)
                    .await
            }
            None => {
                self.notifier
                    .event_reminder(
                        &alarm.title,
                        &alarm.lines,
                        !alarm.join_url.is_empty(),
                        sound,
                    )
                    .await
            }
        };
        match shown {
            Ok(id) => {
                self.events.lock().unwrap().insert(id, alarm);
            }
            Err(err) => tracing::warn!(%err, "could not show an event reminder"),
        }
    }

    /// The snoozed event reminders due at `now`, taken off the list.
    pub(crate) fn snoozed_due(&self, now: i64) -> Vec<Alarm> {
        let mut snoozed = self.snoozed.lock().unwrap();
        let (due, later) = snoozed.drain(..).partition(|a| a.at <= now);
        *snoozed = later;
        due
    }

    /// When the next snoozed event reminder shows.
    pub(crate) fn next_snoozed(&self) -> Option<i64> {
        self.snoozed.lock().unwrap().iter().map(|a| a.at).min()
    }

    /// Says that the update to `version` is downloaded, with an Update
    /// button. Returns the notification's ID.
    pub(crate) async fn update_ready(&self, version: &str) -> Option<u32> {
        self.notifier
            .update_ready(version)
            .await
            .map_err(|err| tracing::warn!(%err, "could not show that an update is ready"))
            .ok()
    }

    pub(crate) fn forget(&self, account: AccountId) {
        self.seen.lock().unwrap().remove(&account);
    }

    /// After a sync of `account`: announces its new mail, and closes
    /// notifications whose mail was read or moved.
    pub(crate) async fn synced(&self, store: &Mutex<Store>, account: AccountId) {
        let found = {
            let store = store.lock().unwrap();
            self.find_new(&store, account)
        };
        match found {
            Ok(Some(Found {
                origin,
                mails,
                messages,
            })) => match self
                .notifier
                .new_mail(&origin, &mails, 0, self.sound.load(Ordering::Relaxed))
                .await
            {
                Ok(id) => {
                    tracing::info!(%account, count = messages.len(), "new mail notified");
                    self.shown
                        .lock()
                        .unwrap()
                        .insert(id, Shown { account, messages });
                }
                Err(err) => tracing::warn!(%err, "could not show a notification"),
            },
            Ok(None) => {}
            Err(err) => tracing::warn!(%err, %account, "looking for new mail"),
        }
        let handled = self.handled(&store.lock().unwrap(), Some(account));
        self.close(handled).await;
    }

    /// Shows a reminder about `messages` of `account` (not empty): mail
    /// back from snooze, or a message nobody replied to. Its buttons act on
    /// `messages`, and it closes once they are read or out of the inbox.
    pub(crate) async fn remind(
        &self,
        store: &Mutex<Store>,
        account: AccountId,
        summary: &str,
        lines: &[String],
        messages: Vec<MessageId>,
    ) {
        let origin = store
            .lock()
            .unwrap()
            .accounts()
            .ok()
            .and_then(|accounts| accounts.into_iter().find(|a| a.id == account))
            .map(|a| a.address)
            .unwrap_or_default();
        let sound = self.sound.load(Ordering::Relaxed);
        match self.notifier.reminder(&origin, summary, lines, sound).await {
            Ok(id) => {
                self.shown
                    .lock()
                    .unwrap()
                    .insert(id, Shown { account, messages });
            }
            Err(err) => tracing::warn!(%err, "could not show a reminder"),
        }
    }

    /// The account's address, its new mail and their IDs, if any.
    fn find_new(&self, store: &Store, account: AccountId) -> katna_store::Result<Option<Found>> {
        let since = unix_now().saturating_sub(NEWER_THAN.as_secs() as i64);
        let mut ids = {
            let mut all = self.seen.lock().unwrap();
            let Some(seen) = all.get_mut(&account) else {
                return Ok(None);
            };
            seen.fresh(store, account, since)?
        };
        if ids.is_empty() || !self.enabled.load(Ordering::Relaxed) {
            return Ok(None);
        }
        // The newest, when there are more than one notification shows.
        ids.drain(..ids.len().saturating_sub(PER_NOTIFICATION as usize));
        let origin = store
            .accounts()?
            .into_iter()
            .find(|a| a.id == account)
            .map(|a| a.address)
            .unwrap_or_default();
        let mails = store
            .messages_by_id(&ids)?
            .into_iter()
            .map(|message| {
                let from = message.first(ParticipantRole::From);
                let sender = from
                    .and_then(|p| p.display_name.clone())
                    .filter(|name| !name.trim().is_empty())
                    .or_else(|| from.map(|p| p.email_norm.clone()))
                    .unwrap_or_else(|| tr!("notify-unknown-sender"));
                NewMail {
                    sender,
                    subject: message.subject,
                    preview: message.snippet,
                }
            })
            .collect();
        Ok(Some(Found {
            origin,
            mails,
            messages: ids,
        }))
    }

    /// Notifications (of `account`, or all) whose mail is all read or out
    /// of the inbox; they are forgotten, and should be closed.
    pub(crate) fn handled(&self, store: &Store, account: Option<AccountId>) -> Vec<u32> {
        let mut shown = self.shown.lock().unwrap();
        let done: Vec<u32> = shown
            .iter()
            .filter(|(_, s)| account.is_none_or(|a| s.account == a))
            .filter(|(_, s)| {
                store.messages_by_id(&s.messages).is_ok_and(|messages| {
                    !messages.iter().any(|m| {
                        !m.flags.contains(MessageFlags::SEEN)
                            && m.locations
                                .iter()
                                .any(|l| l.role.as_deref() == Some(FolderRole::Inbox.as_str()))
                    })
                })
            })
            .map(|(id, _)| *id)
            .collect();
        for id in &done {
            shown.remove(id);
        }
        done
    }

    /// Closes notifications `ids`.
    pub(crate) async fn close(&self, ids: Vec<u32>) {
        for id in ids {
            self.tokens.lock().unwrap().remove(&id);
            if let Err(err) = self.notifier.close(id).await {
                tracing::debug!(%err, id, "could not close a notification");
            }
        }
    }

    /// Serves clicks and buttons until the bus connection goes away.
    pub(crate) async fn serve_actions(daemon: Weak<Daemon>) {
        let Some(notices) = daemon.upgrade().and_then(|d| d.new_mail_notices()) else {
            return;
        };
        let proxy = notices.notifier.proxy().clone();
        let (Ok(mut actions), Ok(mut tokens), Ok(mut closed)) = (
            proxy.receive_action_invoked().await,
            proxy.receive_activation_token().await,
            proxy.receive_notification_closed().await,
        ) else {
            tracing::warn!("cannot watch notification actions");
            return;
        };
        drop(notices);
        enum Got {
            Action(u32, String),
            Token(u32, String),
            Closed(u32),
        }
        loop {
            // The server sends a click's token just before its action.
            // Both wait in their streams by the time the action is read,
            // so the token is taken first; read the other way round, the
            // action went out without it and the window could not come
            // forward on Wayland.
            let next = async {
                let signal = tokens.next().await?;
                let args = signal.args().ok()?;
                Some(Got::Token(args.id, args.activation_token.to_owned()))
            }
            .or(async {
                let signal = actions.next().await?;
                let args = signal.args().ok()?;
                Some(Got::Action(args.id, args.action_key.to_owned()))
            })
            .or(async {
                let signal = closed.next().await?;
                let args = signal.args().ok()?;
                Some(Got::Closed(args.id))
            })
            .await;
            let Some(got) = next else {
                return;
            };
            let Some(daemon) = daemon.upgrade() else {
                return;
            };
            let Some(notices) = daemon.new_mail_notices() else {
                return;
            };
            match got {
                Got::Token(id, token) => {
                    if notices.shown.lock().unwrap().contains_key(&id)
                        || notices.events.lock().unwrap().contains_key(&id)
                        || daemon.updates().is_notice(id)
                    {
                        notices.tokens.lock().unwrap().insert(id, token);
                    }
                }
                Got::Closed(id) => {
                    notices.archived.lock().unwrap().remove(&id);
                    notices.events.lock().unwrap().remove(&id);
                    daemon.updates().take_notice(id);
                    notices.shown.lock().unwrap().remove(&id);
                    notices.tokens.lock().unwrap().remove(&id);
                }
                Got::Action(id, _) if daemon.updates().take_notice(id) => {
                    // The notification itself, or its Update button: Katna
                    // Mail shows the update, ready to install.
                    let token = notices.tokens.lock().unwrap().remove(&id);
                    crate::mail_app::run(
                        &notices.connection,
                        Some(katna_dbus::app_action::INSTALL_UPDATE),
                        Vec::new(),
                        token,
                    )
                    .await;
                    notices.close(vec![id]).await;
                }
                Got::Action(id, key) if notices.events.lock().unwrap().contains_key(&id) => {
                    let Some(mut alarm) = notices.events.lock().unwrap().remove(&id) else {
                        continue;
                    };
                    let token = notices.tokens.lock().unwrap().remove(&id);
                    tracing::info!(id, key, "reminder action");
                    match key.as_str() {
                        // Only web links, whatever the event says.
                        action::JOIN if alarm.join_url.starts_with("https://") => {
                            crate::daemon::open_in_browser(&alarm.join_url).await;
                        }
                        action::JOIN => {}
                        action::SNOOZE => {
                            alarm.at = unix_now() + alarms::SNOOZE;
                            notices.snoozed.lock().unwrap().push(alarm);
                        }
                        action::DONE => {
                            if let Some(task) = alarm.task {
                                match daemon.set_task_done(task, true) {
                                    Ok(_) => {
                                        daemon.wake_task_sync();
                                        let _ = daemon
                                            .notices()
                                            .try_send(crate::daemon::Notice::TasksChanged);
                                    }
                                    Err(err) => {
                                        tracing::warn!(%err, task, "could not tick a task off");
                                    }
                                }
                            }
                        }
                        // The notification itself: the Calendar page, or
                        // Tasks for a task.
                        _ => {
                            let page = match alarm.task {
                                Some(task) => format!("tasks:{task}"),
                                None => "calendar".to_owned(),
                            };
                            crate::mail_app::run(
                                &notices.connection,
                                Some(katna_dbus::app_action::OPEN_PAGE),
                                vec![Value::from(page)],
                                token,
                            )
                            .await;
                        }
                    }
                    notices.close(vec![id]).await;
                }
                Got::Action(id, key) if notices.archived.lock().unwrap().contains_key(&id) => {
                    let Some(archived) = notices.archived.lock().unwrap().remove(&id) else {
                        continue;
                    };
                    if key == action::UNDO {
                        tracing::info!(id, "undo archive from a notification");
                        let inbox =
                            daemon
                                .store()
                                .folders(archived.account)
                                .ok()
                                .and_then(|folders| {
                                    folders
                                        .into_iter()
                                        .find(|f| f.role == Some(FolderRole::Inbox))
                                        .map(|f| f.id)
                                });
                        let undone = match inbox {
                            Some(inbox) => daemon
                                .move_messages(&archived.messages, inbox)
                                .map_err(|e| e.to_string()),
                            None => Err("the account has no inbox".to_owned()),
                        };
                        if let Err(err) = undone {
                            tracing::warn!(%err, "could not undo an archive");
                        }
                    }
                    notices.close(vec![id]).await;
                }
                Got::Action(id, key) => {
                    let Some(shown) = notices.shown.lock().unwrap().remove(&id) else {
                        continue;
                    };
                    let token = notices.tokens.lock().unwrap().remove(&id);
                    tracing::info!(id, key, "notification action");
                    let done = match key.as_str() {
                        action::MARK_READ => daemon
                            .set_flags(&shown.messages, MessageFlags::SEEN, MessageFlags::empty())
                            .map_err(|e| e.to_string()),
                        action::ARCHIVE => match daemon.archive_messages(&shown.messages) {
                            Ok(()) => {
                                notices.confirm_archived(&daemon, shown.clone()).await;
                                Ok(())
                            }
                            Err(err) => Err(err.to_string()),
                        },
                        action::OPEN => {
                            notices.open(shown.messages[0], false, token).await;
                            Ok(())
                        }
                        action::REPLY_ALL => {
                            notices.open(shown.messages[0], true, token).await;
                            Ok(())
                        }
                        _ => Ok(()),
                    };
                    if let Err(err) = done {
                        tracing::warn!(%err, key, "notification action failed");
                    }
                    notices.close(vec![id]).await;
                }
            }
        }
    }

    /// Says in a notification that `shown`'s mail was archived, with an
    /// Undo that puts it back in the inbox.
    async fn confirm_archived(&self, daemon: &Daemon, shown: Shown) {
        let subject = match shown.messages.as_slice() {
            [one] => daemon
                .store()
                .messages_by_id(&[*one])
                .ok()
                .and_then(|m| m.into_iter().next())
                .map(|m| m.subject),
            _ => None,
        };
        match self
            .notifier
            .archived(subject.as_deref(), shown.messages.len())
            .await
        {
            Ok(id) => {
                self.archived.lock().unwrap().insert(id, shown);
            }
            Err(err) => tracing::warn!(%err, "could not say that mail was archived"),
        }
    }

    /// Opens Katna Mail on `message`, with a reply to all started if
    /// `reply_all`: through its `org.freedesktop.Application` interface when
    /// it is running and has one, else by starting it.
    async fn open(&self, message: MessageId, reply_all: bool, token: Option<String>) {
        let params = vec![Value::from(message.0)];
        let action = Some(if reply_all {
            katna_dbus::app_action::REPLY_ALL
        } else {
            katna_dbus::app_action::OPEN_MESSAGE
        });
        crate::mail_app::run(&self.connection, action, params, token).await;
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_core::{AccountKind, MailCategory};
    use katna_store::{Added, Mode, RemoteMessage};

    fn message(uid: u32, gm_msgid: u64, category: MailCategory) -> RemoteMessage<'static> {
        RemoteMessage {
            uid,
            message_id_hdr: Some("1@example.org"),
            subject: Some("Hello"),
            date: Some(unix_now()),
            size: 1234,
            flags: MessageFlags::empty(),
            keywords: &[],
            has_attachments: false,
            list_id: None,
            participants: &[],
            in_reply_to: None,
            references: &[],
            gm_thread_id: None,
            gm_msgid: Some(gm_msgid),
            category: Some(category),
            attachments: &[],
        }
    }

    #[test]
    fn mail_is_news_when_it_reaches_the_inbox() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = katna_core::Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Gmail", "me@gmail.com")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let all_mail = batch
            .upsert_folder(account, "[Gmail]/All Mail", Some(FolderRole::All))
            .unwrap();
        batch.commit().unwrap();
        let mut seen = Seen {
            after: MessageId(0),
            primed: false,
            announced: HashSet::new(),
        };
        let since = unix_now() - 60;
        assert!(seen.fresh(&store, account, since).unwrap().is_empty());

        // All Mail syncs first: the message is stored, but not in the inbox.
        let mut batch = store.mail_batch().unwrap();
        let added = batch
            .add_remote_message(account, all_mail, &message(1, 7, MailCategory::Primary))
            .unwrap();
        let Added::Message(id) = added else {
            panic!("{added:?}");
        };
        batch.commit().unwrap();
        assert!(seen.fresh(&store, account, since).unwrap().is_empty());

        // Its inbox copy comes with the next sync, and is news once.
        let mut batch = store.mail_batch().unwrap();
        batch
            .add_remote_message(account, inbox, &message(1, 7, MailCategory::Primary))
            .unwrap();
        batch.commit().unwrap();
        assert_eq!(seen.fresh(&store, account, since).unwrap(), [id]);
        assert!(seen.fresh(&store, account, since).unwrap().is_empty());
    }
}
