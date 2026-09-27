// SPDX-License-Identifier: GPL-3.0-or-later

//! New-mail notifications (`docs/ARCHITECTURE.md` §15.1): after each sync,
//! unread mail that reached an account's inbox (Primary tab) since the last
//! look becomes one notification per account and sync, with Open, Reply all
//! (for one message), Mark as read and Archive. Notifications close when their mail is read or leaves
//! the inbox anywhere.

use std::{
    collections::HashMap,
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

/// Older mail is not news, even when it is new to the store (a folder
/// synced for the first time, mail moved back into the inbox).
const NEWER_THAN: Duration = Duration::from_secs(2 * 24 * 3600);
/// At most this many messages go into one notification.
const PER_NOTIFICATION: u32 = 50;

/// What was already looked at for one account.
#[derive(Clone, Copy, Debug)]
struct Seen {
    /// Messages up to this one were considered.
    after: MessageId,
    /// `false` until the account's first sync: a new account's mail is
    /// not news.
    primed: bool,
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
        let latest = store.latest_message(account)?;
        let seen = {
            let mut all = self.seen.lock().unwrap();
            let Some(seen) = all.get_mut(&account) else {
                return Ok(None);
            };
            let before = *seen;
            seen.after = latest.max(seen.after);
            seen.primed = true;
            before
        };
        if !seen.primed || !self.enabled.load(Ordering::Relaxed) {
            return Ok(None);
        }
        let since = unix_now().saturating_sub(NEWER_THAN.as_secs() as i64);
        let mut ids = store.new_inbox_mail(account, seen.after, since, PER_NOTIFICATION)?;
        ids.retain(|id| id.0 <= latest.0);
        if ids.is_empty() {
            return Ok(None);
        }
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
            let next = async {
                let signal = actions.next().await?;
                let args = signal.args().ok()?;
                Some(Got::Action(args.id, args.action_key.to_owned()))
            }
            .or(async {
                let signal = tokens.next().await?;
                let args = signal.args().ok()?;
                Some(Got::Token(args.id, args.activation_token.to_owned()))
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
                    if notices.shown.lock().unwrap().contains_key(&id) {
                        notices.tokens.lock().unwrap().insert(id, token);
                    }
                }
                Got::Closed(id) => {
                    notices.shown.lock().unwrap().remove(&id);
                    notices.tokens.lock().unwrap().remove(&id);
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
                        action::ARCHIVE => daemon
                            .archive_messages(&shown.messages)
                            .map_err(|e| e.to_string()),
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
