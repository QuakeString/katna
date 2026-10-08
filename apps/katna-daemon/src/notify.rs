// SPDX-License-Identifier: GPL-3.0-or-later

//! New-mail notifications (`docs/ARCHITECTURE.md` §15.1): after each sync,
//! unread mail that rings (by default: reached an account's inbox, Primary
//! tab, and nothing about it is muted; §15.1.1) since the last look becomes
//! one notification per account and sync, with Open, Mark as read and
//! Archive, and for one message Peek (more of it, in the same notification)
//! and Reply: typed into the notification where the desktop can (§15.1.2),
//! sent from here after the undo time, else Katna Mail's reply window, and
//! a button that copies a one-time code or opens a verify link (§15.1.3).
//! Notifications close when their mail is read or leaves the inbox
//! anywhere, or is muted.

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
use katna_core::config::{Notifications, SoundEvent, Sounds};
use katna_i18n::tr;
use katna_notify::{NewMail, Notifier, View, action};
use katna_platform::sound;
use katna_store::{FolderRole, MessageFlags, MessageId, ParticipantRole, Store, StoredMessage};
use katna_sync::mail_actions::{self, LinkKind, Shortcut};
use katna_sync::quick_reply::{self, Mailbox};
use zbus::zvariant::Value;

use crate::daemon::Daemon;
use crate::daemon::alarms::{self, Alarm};
use crate::needs_you::{self, NeedsYou};

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
    /// New mail a mail rule has yet to run on (it waits for its body):
    /// not news until then.
    held: HashSet<MessageId>,
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
            .new_ringing_mail(account, self.after, since, CANDIDATES, unix_now())?
            .into_iter()
            .filter(|id| !self.held.contains(id) && self.announced.insert(*id))
            .collect())
    }
}

/// New mail to announce: the account's address, what to show, and the
/// messages.
struct Found {
    origin: String,
    mails: Vec<NewMail>,
    messages: Vec<MessageId>,
    /// One message's code or link.
    shortcut: Option<Shortcut>,
}

/// A reply typed into a notification, waiting out the undo time.
#[derive(Clone, Debug)]
struct Replied {
    /// Its outbox entry.
    outbox: i64,
    /// The message it answers.
    message: MessageId,
    text: String,
}

/// A notification on screen.
#[derive(Clone, Debug)]
struct Shown {
    account: AccountId,
    messages: Vec<MessageId>,
    /// A new-mail notification, which also closes once its mail is muted
    /// or its folder stops notifying. Reminders stay until read or moved.
    new_mail: bool,
    /// One message's code or link, which its button copies or opens.
    shortcut: Option<Shortcut>,
}

pub(crate) struct NewMailNotices {
    notifier: Notifier,
    connection: zbus::Connection,
    enabled: AtomicBool,
    /// The sounds notifications play.
    sounds: Mutex<Sounds>,
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
    /// Notes that a reply typed into a notification is on its way, for
    /// their Undo.
    replied: Mutex<HashMap<u32, Replied>>,
    /// The outbox entries of those replies, until they go out.
    outgoing: Mutex<HashSet<i64>>,
    /// What needs the user, told once each, with its notification's ID
    /// while that shows.
    told: Mutex<HashMap<needs_you::Key, Option<u32>>>,
    /// Those notifications on show, and the Katna Mail page each opens.
    problems: Mutex<HashMap<u32, String>>,
}

impl NewMailNotices {
    pub(crate) async fn new(
        connection: &zbus::Connection,
        settings: &Notifications,
        sounds: &Sounds,
    ) -> zbus::Result<Self> {
        Ok(Self {
            notifier: Notifier::new(connection).await?,
            connection: connection.clone(),
            enabled: AtomicBool::new(settings.new_mail),
            sounds: Mutex::new(sounds.clone()),
            seen: Mutex::default(),
            shown: Mutex::default(),
            tokens: Mutex::default(),
            events: Mutex::default(),
            snoozed: Mutex::default(),
            archived: Mutex::default(),
            replied: Mutex::default(),
            outgoing: Mutex::default(),
            told: Mutex::default(),
            problems: Mutex::default(),
        })
    }

    /// Applies the `notifications` and `sounds` settings.
    pub(crate) fn set(&self, settings: &Notifications, sounds: &Sounds) {
        self.enabled.store(settings.new_mail, Ordering::Relaxed);
        *self.sounds.lock().unwrap() = sounds.clone();
    }

    /// The sound of `event`, if it plays one.
    fn sound(&self, event: SoundEvent) -> Option<String> {
        let sounds = self.sounds.lock().unwrap();
        sounds
            .playing(event)
            .map(|chosen| sound::resolve(event, &sounds.set, chosen))
    }

    /// What the notification server is to play of `sound`: a toast plays
    /// the Windows sounds itself; elsewhere servers such as Plasma's play
    /// none, and toasts no others, so [`Self::ring`] does.
    fn server_sound(sound: &Option<String>) -> Option<&str> {
        sound.as_deref().filter(|id| sound::toast_plays(id))
    }

    /// Plays `sound` for a notification just shown, where the server does
    /// not, unless Do not disturb is on.
    async fn ring(&self, sound: Option<String>) {
        if let Some(id) = sound
            && !sound::toast_plays(&id)
            && !sound::quiet(&self.connection).await
        {
            sound::play(&id);
        }
    }

    /// Plays `sound` now, unless Do not disturb is on: for what shows no
    /// notification of its own.
    async fn play(&self, sound: Option<String>) {
        if let Some(id) = sound
            && !sound::quiet(&self.connection).await
        {
            sound::play(&id);
        }
    }

    /// Outbox entry `outbox` went out: a reply typed into a notification
    /// plays the Sent sound, as Katna Mail's own do.
    pub(crate) async fn went_out(&self, outbox: i64) {
        if self.outgoing.lock().unwrap().remove(&outbox) {
            self.play(self.sound(SoundEvent::Sent)).await;
        }
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
            held: HashSet::new(),
        });
    }

    /// A mail rule said not to notify about `messages` of `account`.
    pub(crate) fn quiet(&self, account: AccountId, messages: &[MessageId]) {
        if let Some(seen) = self.seen.lock().unwrap().get_mut(&account) {
            seen.announced.extend(messages.iter().copied());
        }
    }

    /// New mail of `account` that waits for a mail rule: nothing is shown
    /// for it until it is no longer held.
    pub(crate) fn hold(&self, account: AccountId, messages: &[MessageId]) {
        if let Some(seen) = self.seen.lock().unwrap().get_mut(&account) {
            seen.held = messages.iter().copied().collect();
        }
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
                            new_mail: false,
                            shortcut: None,
                        },
                    );
                }
            }
            Err(err) => tracing::warn!(%err, "could not show a tracking notification"),
        }
    }

    /// Shows the reminder of a calendar event or a task.
    pub(crate) async fn event_reminder(&self, alarm: Alarm) {
        let sound = self.sound(SoundEvent::Reminders);
        let shown = match alarm.task {
            None if alarm.note.is_some() => {
                self.notifier
                    .note_reminder(&alarm.title, &alarm.lines, Self::server_sound(&sound))
                    .await
            }
            Some(_) => {
                self.notifier
                    .task_reminder(&alarm.title, &alarm.lines, Self::server_sound(&sound))
                    .await
            }
            None => {
                self.notifier
                    .event_reminder(
                        &alarm.title,
                        &alarm.lines,
                        !alarm.join_url.is_empty(),
                        Self::server_sound(&sound),
                    )
                    .await
            }
        };
        match shown {
            Ok(id) => {
                self.events.lock().unwrap().insert(id, alarm);
                self.ring(sound).await;
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

    /// Shows one notification for each thing in `now` that needs the
    /// user and wasn't told yet; closes those of what was fixed, which are
    /// told again should they come back.
    pub(crate) async fn needs_you(&self, now: &[NeedsYou]) {
        let (fixed, new): (Vec<u32>, Vec<&NeedsYou>) = {
            let mut told = self.told.lock().unwrap();
            let mut fixed = Vec::new();
            told.retain(|key, id| {
                let lasts = now.iter().any(|n| n.key() == *key);
                if !lasts {
                    fixed.extend(*id);
                }
                lasts
            });
            let new = now
                .iter()
                .filter(|n| !told.contains_key(&n.key()))
                .collect();
            (fixed, new)
        };
        for problem in new {
            let (summary, body, fix) = problem.notification();
            let id = match self.notifier.needs_you(&summary, &body, &fix).await {
                Ok(id) => {
                    self.problems.lock().unwrap().insert(id, problem.page());
                    Some(id)
                }
                Err(err) => {
                    tracing::warn!(%err, "could not say that something needs the user");
                    None
                }
            };
            tracing::info!(problem = ?problem.key(), "told the user");
            self.told.lock().unwrap().insert(problem.key(), id);
        }
        for id in &fixed {
            self.problems.lock().unwrap().remove(id);
        }
        self.close(fixed).await;
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
        let sound = self.sound(SoundEvent::NewMail);
        let replies = self.notifier.takes_replies().await;
        match found {
            Ok(Some(Found {
                origin,
                mails,
                messages,
                shortcut,
            })) => match self
                .notifier
                .new_mail(
                    &origin,
                    &mails,
                    View::Short,
                    replies,
                    0,
                    Self::server_sound(&sound),
                )
                .await
            {
                Ok(id) => {
                    self.ring(sound).await;
                    tracing::info!(%account, count = messages.len(), "new mail notified");
                    self.shown.lock().unwrap().insert(
                        id,
                        Shown {
                            account,
                            messages,
                            new_mail: true,
                            shortcut,
                        },
                    );
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
        let sound = self.sound(SoundEvent::MailBack);
        match self
            .notifier
            .reminder(&origin, summary, lines, Self::server_sound(&sound))
            .await
        {
            Ok(id) => {
                self.ring(sound).await;
                self.shown.lock().unwrap().insert(
                    id,
                    Shown {
                        account,
                        messages,
                        new_mail: false,
                        shortcut: None,
                    },
                );
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
        let stored = store.messages_by_id(&ids)?;
        let shortcut = match &stored[..] {
            [one] => find_shortcut(store, one),
            _ => None,
        };
        let mut mails: Vec<NewMail> = stored.into_iter().map(new_mail).collect();
        if let [mail] = &mut mails[..] {
            mail.shortcut = shortcut.as_ref().map(button);
        }
        Ok(Some(Found {
            origin,
            mails,
            messages: ids,
            shortcut,
        }))
    }

    /// Notifications (of `account`, or all) whose mail is all read or out
    /// of the inbox, and new-mail notifications whose mail no longer
    /// notifies (muted, or its folder's bell turned off); they are
    /// forgotten, and should be closed.
    pub(crate) fn handled(&self, store: &Store, account: Option<AccountId>) -> Vec<u32> {
        let mut shown = self.shown.lock().unwrap();
        let done: Vec<u32> = shown
            .iter()
            .filter(|(_, s)| account.is_none_or(|a| s.account == a))
            .filter(|(_, s)| {
                if s.new_mail
                    && store
                        .still_ringing(&s.messages, unix_now())
                        .is_ok_and(|ringing| ringing.is_empty())
                {
                    return true;
                }
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
        let (Ok(mut actions), Ok(mut tokens), Ok(mut closed), Ok(mut replies)) = (
            proxy.receive_action_invoked().await,
            proxy.receive_activation_token().await,
            proxy.receive_notification_closed().await,
            proxy.receive_notification_replied().await,
        ) else {
            tracing::warn!("cannot watch notification actions");
            return;
        };
        drop(notices);
        enum Got {
            Action(u32, String),
            Token(u32, String),
            Closed(u32),
            Replied(u32, String),
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
            .or(async {
                let signal = replies.next().await?;
                let args = signal.args().ok()?;
                Some(Got::Replied(args.id, args.text.to_owned()))
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
                        || notices.replied.lock().unwrap().contains_key(&id)
                        || notices.problems.lock().unwrap().contains_key(&id)
                        || daemon.updates().is_notice(id)
                    {
                        notices.tokens.lock().unwrap().insert(id, token);
                    }
                }
                Got::Closed(id) => {
                    notices.archived.lock().unwrap().remove(&id);
                    notices.replied.lock().unwrap().remove(&id);
                    notices.events.lock().unwrap().remove(&id);
                    notices.problems.lock().unwrap().remove(&id);
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
                Got::Action(id, _) if notices.problems.lock().unwrap().contains_key(&id) => {
                    // The notification itself or its button: Katna Mail
                    // opens the fix (New password, Sign in, the Outbox).
                    let Some(page) = notices.problems.lock().unwrap().remove(&id) else {
                        continue;
                    };
                    let token = notices.tokens.lock().unwrap().remove(&id);
                    crate::mail_app::run(
                        &notices.connection,
                        Some(katna_dbus::app_action::OPEN_PAGE),
                        vec![Value::from(page)],
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
                            crate::daemon::open_in_browser(&alarm.join_url, token.as_deref()).await;
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
                        // The notification itself: the mail a task was
                        // made from (Remind me), else the Calendar page,
                        // or Tasks for a task, or the note.
                        _ if let Some(message) = alarm.task.and_then(|t| task_mail(&daemon, t)) => {
                            notices.open(message, false, token).await;
                        }
                        _ => {
                            let page = match (alarm.task, alarm.note) {
                                (Some(task), _) => format!("tasks:{task}"),
                                (_, Some(note)) => format!("notes:{note}"),
                                _ => "calendar".to_owned(),
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
                Got::Replied(id, text) => {
                    let Some(shown) = notices.shown.lock().unwrap().remove(&id) else {
                        continue;
                    };
                    let token = notices.tokens.lock().unwrap().remove(&id);
                    notices.send_reply(&daemon, id, shown, text, token).await;
                }
                Got::Action(id, key) if notices.replied.lock().unwrap().contains_key(&id) => {
                    let Some(replied) = notices.replied.lock().unwrap().remove(&id) else {
                        continue;
                    };
                    let token = notices.tokens.lock().unwrap().remove(&id);
                    notices.close(vec![id]).await;
                    match key.as_str() {
                        action::UNDO => {
                            let undone = daemon.undo_send(replied.outbox).unwrap_or_else(|err| {
                                tracing::warn!(%err, "could not undo a reply");
                                false
                            });
                            tracing::info!(id, undone, "undo a reply from a notification");
                            if undone {
                                notices.outgoing.lock().unwrap().remove(&replied.outbox);
                            }
                            // Kept, to write on in Katna Mail; once gone,
                            // the conversation shows it.
                            let text = undone.then_some(replied.text);
                            notices.reply_in_app(replied.message, text, token).await;
                        }
                        _ => notices.open(replied.message, false, token).await,
                    }
                }
                // Typing a reply does not close the notification; Plasma
                // only says the field opened, if anything.
                Got::Action(_, key) if key == action::INLINE_REPLY => {}
                // Copying or opening a link leaves the notification and its
                // mail as they are.
                Got::Action(id, key) if key == action::COPY_CODE || key == action::OPEN_LINK => {
                    let shortcut = notices
                        .shown
                        .lock()
                        .unwrap()
                        .get(&id)
                        .and_then(|s| s.shortcut.clone());
                    let token = notices.tokens.lock().unwrap().remove(&id);
                    tracing::info!(id, key, "notification shortcut");
                    match shortcut {
                        Some(Shortcut::Code(code)) => {
                            let copied = crate::clipboard::copy(&notices.connection, &code).await;
                            if let Err(err) = notices.notifier.code_copied(&code, copied).await {
                                tracing::warn!(%err, "could not say that a code was copied");
                            }
                        }
                        // Only ever after a click on the button that names
                        // where it goes.
                        Some(Shortcut::Link { url, .. }) => {
                            crate::daemon::open_in_browser(&url, token.as_deref()).await;
                        }
                        None => {}
                    }
                }
                Got::Action(id, key) if key == action::PEEK => {
                    let shown = notices.shown.lock().unwrap().get(&id).cloned();
                    if let Some(shown) = shown {
                        notices.peek(&daemon, id, shown).await;
                    }
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
                        action::REPLY => {
                            notices.reply_in_app(shown.messages[0], None, token).await;
                            Ok(())
                        }
                        action::SNOOZE_HOUR | action::SNOOZE_TOMORROW => {
                            let until = if key == action::SNOOZE_HOUR {
                                Some(unix_now() + 3600)
                            } else {
                                let config = crate::daemon::settings(daemon.paths());
                                tomorrow_morning(config.mail.snooze.morning)
                            };
                            match until {
                                Some(until) => daemon
                                    .snooze_conversations(&shown.messages, until)
                                    .await
                                    .map_err(|e| e.to_string()),
                                None => Ok(()),
                            }
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

    /// Shows more of `shown`'s one message in notification `id`, in place,
    /// with Reply, Reply all and Archive.
    async fn peek(&self, daemon: &Daemon, id: u32, shown: Shown) {
        let [message] = shown.messages[..] else {
            return;
        };
        let found = {
            let store = daemon.store();
            let origin = store
                .accounts()
                .ok()
                .and_then(|accounts| accounts.into_iter().find(|a| a.id == shown.account))
                .map(|a| a.address)
                .unwrap_or_default();
            store
                .messages_by_id(&[message])
                .ok()
                .and_then(|m| m.into_iter().next())
                .map(|stored| {
                    let text = original(&store, &stored)
                        .map(|o| o.text)
                        .filter(|t| !t.is_empty())
                        .or_else(|| stored.snippet.clone())
                        .unwrap_or_default();
                    (origin, new_mail(stored), text)
                })
        };
        let Some((origin, mut mail, text)) = found else {
            return;
        };
        mail.shortcut = shown.shortcut.as_ref().map(button);
        let replies = self.notifier.takes_replies().await;
        let view = View::Peek { text: &text };
        match self
            .notifier
            .new_mail(&origin, &[mail], view, replies, id, None)
            .await
        {
            Ok(new_id) if new_id != id => {
                // A server that shows it anew: it is the one to follow.
                self.shown.lock().unwrap().remove(&id);
                self.shown.lock().unwrap().insert(new_id, shown);
            }
            Ok(_) => {}
            Err(err) => tracing::warn!(%err, "could not peek at a message"),
        }
    }

    /// Sends `text`, typed into notification `id` about `shown`'s message,
    /// as a reply to its sender after the undo time, and says so in a
    /// note. Mail that cannot be answered from here (its body is not
    /// downloaded) opens in Katna Mail with the reply started.
    async fn send_reply(
        &self,
        daemon: &Daemon,
        id: u32,
        shown: Shown,
        text: String,
        token: Option<String>,
    ) {
        let message = shown.messages[0];
        if text.trim().is_empty() {
            return;
        }
        let built = self.build_reply(daemon, message, &text);
        let (account, raw, name, undo) = match built {
            Ok(built) => built,
            Err(err) => {
                tracing::warn!(%err, "could not send a reply from a notification");
                self.close(vec![id]).await;
                self.reply_in_app(message, Some(text), token).await;
                return;
            }
        };
        let outbox = match daemon.queue_send(account, &raw, undo) {
            Ok(outbox) => outbox,
            Err(err) => {
                tracing::warn!(%err, "could not queue a reply from a notification");
                self.close(vec![id]).await;
                self.reply_in_app(message, Some(text), token).await;
                return;
            }
        };
        tracing::info!(id, outbox, "reply from a notification");
        self.outgoing.lock().unwrap().insert(outbox);
        // Answered: read.
        if let Err(err) = daemon.set_flags(&[message], MessageFlags::SEEN, MessageFlags::empty()) {
            tracing::debug!(%err, "could not mark an answered message read");
        }
        // A new note rather than the same one changed: some servers close
        // the notification once a reply is typed into it, and that closing
        // must not take the note with it.
        self.close(vec![id]).await;
        match self.notifier.reply_sent(&name, &text, undo, 0).await {
            Ok(note) => {
                self.replied.lock().unwrap().insert(
                    note,
                    Replied {
                        outbox,
                        message,
                        text,
                    },
                );
            }
            Err(err) => tracing::warn!(%err, "could not say that a reply is on its way"),
        }
    }

    /// The reply saying `text` to `message`: its account, the message,
    /// whom it goes to, and the undo time in seconds.
    fn build_reply(
        &self,
        daemon: &Daemon,
        message: MessageId,
        text: &str,
    ) -> Result<(AccountId, Vec<u8>, String, u32), String> {
        let settings = crate::daemon::settings(daemon.paths());
        let store = daemon.store();
        let stored = store
            .messages_by_id(&[message])
            .map_err(|e| e.to_string())?
            .into_iter()
            .next()
            .ok_or("the message is gone")?;
        let original = original(&store, &stored).ok_or("its body is not downloaded")?;
        let account = store
            .accounts()
            .map_err(|e| e.to_string())?
            .into_iter()
            .find(|a| a.id == stored.account)
            .ok_or("its account is gone")?;
        drop(store);
        let from = Mailbox {
            name: Some(account.display_name.trim().to_owned()).filter(|n| !n.is_empty()),
            email: account.address.clone(),
        };
        let sending = &settings.sending;
        let signature = sending
            .reply_signature
            .and_then(|id| sending.signatures.iter().find(|s| s.id == id))
            .map(|s| s.text.as_str())
            .unwrap_or_default();
        // The quote's first line, in the interface's language as Katna
        // Mail's own replies have it: it is part of the mail, for whoever
        // reads it. A name in the other direction from the sentence keeps
        // its own between isolation marks (Fluent sets them in a
        // right-to-left language). The date is the mail's in numbers, which
        // read in any language: the daemon formats no dates in words (it
        // leaves ICU out to stay small).
        let sender = original
            .from
            .as_ref()
            .map(Mailbox::text)
            .unwrap_or_default();
        let sender = if !katna_i18n::rtl() && katna_core::bidi::has_rtl(&sender) {
            katna_core::bidi::isolate(&sender)
        } else {
            sender
        };
        let intro = match original
            .date
            .and_then(|d| jiff::Timestamp::from_second(d).ok())
        {
            Some(date) => {
                let date = date.to_zoned(jiff::tz::TimeZone::system()).datetime();
                tr!(
                    "notify-reply-quote-header",
                    date = numeric_date(date),
                    from = sender
                )
            }
            None => tr!("notify-reply-quote-header-no-date", from = sender),
        };
        let raw = quick_reply::build(&original, &from, text, signature, &intro)
            .ok_or("it has no sender")?;
        let to = original.reply_to.as_ref().map_or_else(String::new, |to| {
            to.name
                .clone()
                .filter(|n| !n.trim().is_empty())
                .unwrap_or_else(|| to.email.clone())
        });
        Ok((account.id, raw, to, sending.undo_send_seconds))
    }

    /// Opens Katna Mail on `message` with a reply to its sender started,
    /// saying `text` if given.
    async fn reply_in_app(&self, message: MessageId, text: Option<String>, token: Option<String>) {
        let mut params = vec![Value::from(message.0)];
        if let Some(text) = text {
            params.push(Value::from(text));
        }
        crate::mail_app::run(
            &self.connection,
            Some(katna_dbus::app_action::REPLY),
            params,
            token,
        )
        .await;
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

/// `date` as numbers, year first ("2026-10-07 14:05"): the same in every
/// language, without ICU.
fn numeric_date(date: jiff::civil::DateTime) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute()
    )
}

/// Tomorrow at `morning` (minutes after midnight) here, as the app's
/// snooze menu offers it.
fn tomorrow_morning(morning: u32) -> Option<i64> {
    let tz = jiff::tz::TimeZone::system();
    let now = jiff::Timestamp::now().to_zoned(tz.clone());
    let morning = morning.min(24 * 60 - 1);
    let at = now
        .date()
        .tomorrow()
        .ok()?
        .at((morning / 60) as i8, (morning % 60) as i8, 0, 0)
        .to_zoned(tz)
        .ok()?;
    Some(at.timestamp().as_second())
}

/// The mail task `task` was made from, while it is still in a folder.
fn task_mail(daemon: &Daemon, task: i64) -> Option<MessageId> {
    let store = daemon.store();
    let task = store.task(task).ok()??;
    if task.mail.is_empty() {
        return None;
    }
    store.message_with_header(&task.mail).ok()?
}

/// `message` as a reply reads it, when its body is downloaded.
fn original(store: &Store, message: &StoredMessage) -> Option<quick_reply::Original> {
    let raw = store.blobs().get(message.blob_hash.as_ref()?).ok()??;
    quick_reply::original(&raw)
}

/// The one-time code or verify link in `message`, when its body is
/// downloaded.
fn find_shortcut(store: &Store, message: &StoredMessage) -> Option<Shortcut> {
    let raw = store.blobs().get(message.blob_hash.as_ref()?).ok()??;
    mail_actions::shortcut(&raw)
}

/// The button for `shortcut`.
fn button(shortcut: &Shortcut) -> katna_notify::Shortcut {
    match shortcut {
        Shortcut::Code(code) => katna_notify::Shortcut::Code(code.clone()),
        Shortcut::Link { kind, domain, .. } => {
            let domain = domain.clone();
            match kind {
                LinkKind::Verify => katna_notify::Shortcut::Verify { domain },
                LinkKind::Confirm => katna_notify::Shortcut::Confirm { domain },
                LinkKind::Activate => katna_notify::Shortcut::Activate { domain },
            }
        }
    }
}

/// `message` as a new-mail notification shows it.
fn new_mail(message: StoredMessage) -> NewMail {
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
        shortcut: None,
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
            held: HashSet::new(),
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
