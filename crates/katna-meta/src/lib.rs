// SPDX-License-Identifier: GPL-3.0-or-later

//! Per-object metadata with expiration: the base for snooze and follow-up
//! reminders, later send later and undo send. See `docs/ARCHITECTURE.md`
//! §10.
//!
//! Values live in the `meta` table of `pim.db` ([`katna_store::MetaRow`]);
//! this crate gives them their types, and [`run`] is the scheduler the
//! daemon runs: it wakes at the next expiry and hands the due values to the
//! daemon, which acts on them and removes or moves their expiry.
//!
//! Everything here runs while the computer is on; nothing goes to a
//! server. Values survive restarts (they are in the store), and the
//! scheduler never sleeps longer than [`MAX_SLEEP`], so a suspend or a
//! change of the clock delays nothing by more than that.

use std::future::Future;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;
use katna_store::{MessageId, MetaRow, Store};
use serde::{Deserialize, Serialize};

mod follow_up;
mod nudge;
pub use follow_up::{WORK_END, WORK_START, counts_as_reply, replied, working_time};
pub use nudge::{
    NUDGE_AFTER, NUDGE_UNTIL, Nudge, asks, clear_nudge, nudge_of, nudges, set_nudge, waiting_nudges,
};

/// The longest the scheduler sleeps before looking at the clock again.
/// Timers stop while the computer sleeps; the wall clock does not.
pub const MAX_SLEEP: Duration = Duration::from_secs(60);

/// How long a conversation that came back stays on top of the Inbox.
pub const SURFACED_FOR: i64 = 14 * 24 * 3600;

/// What a value is attached to (`meta.object_kind`).
pub mod kind {
    /// A stored message (`message.id`).
    pub const MESSAGE: &str = "message";
    /// An outbox entry (`outbox.id`).
    pub const OUTBOX: &str = "outbox";
}

/// The features (`meta.plugin`).
pub mod plugin {
    pub const SNOOZE: &str = "snooze";
    pub const FOLLOW_UP: &str = "follow-up";
    /// Came back to the Inbox (from snooze, or as a reminder): the list
    /// sorts it by when it came back.
    pub const SURFACED: &str = "surfaced";
    /// A mail rule's "mark read after N days".
    pub const READ_AFTER: &str = "read-after";
    /// What the daemon found on a sent message for nudges: whether it
    /// asked something, and whether the user dismissed its nudge.
    pub const NUDGE: &str = "nudge";
}

/// A snoozed message, in the Snoozed folder until `until`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snooze {
    /// When it comes back (Unix seconds).
    pub until: i64,
    /// The folder it goes back to (`folder.id`), usually the Inbox.
    pub back_to: i64,
    /// The Snoozed folder it waits in.
    pub snoozed_in: i64,
    /// The newest message row when it was snoozed: a message of the
    /// conversation stored after it, in an Inbox, brings it back early.
    /// 0 (snoozed by an older Katna): only the time does.
    #[serde(default)]
    pub newest: i64,
}

/// "Remind me if nobody replies": on a sent message, by its outbox entry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUp {
    pub account: i64,
    /// The sent message's `Message-ID`, to find its copy in Sent.
    pub message_id: String,
    pub subject: String,
    /// When the reminder is due (Unix seconds).
    pub remind_at: i64,
    /// Seconds after sending, as chosen: 1, 3 or 7 days, or custom.
    pub after: i64,
    /// The follow-up Katna sends for the user when it is due (RFC 5322,
    /// threaded under the message, without `Date` and `Message-ID`); with
    /// none, the conversation only comes back to the Inbox.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// Seconds after the first follow-up to send it a second time if
    /// still nobody replied; 0 for once. There are never more than two.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub again: i64,
    /// The `Message-ID`s of the follow-ups sent so far: they are not
    /// replies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sent: Vec<String>,
    /// It fell due while the computer was off, and waits for the user to
    /// send it or stop it rather than go out late.
    #[serde(default, skip_serializing_if = "is_false")]
    pub waiting: bool,
}

impl FollowUp {
    /// Whether it sends a follow-up for the user, rather than remind.
    pub fn sends(&self) -> bool {
        self.mail.is_some()
    }

    /// Whether another follow-up goes out after the next one.
    pub fn sends_again(&self) -> bool {
        self.again > 0 && self.sent.is_empty()
    }
}

fn is_zero(value: &i64) -> bool {
    *value == 0
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// A message that came back to the Inbox at `at`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Surfaced {
    pub at: i64,
}

/// A due value, typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Due {
    Snooze(MessageId, Snooze),
    /// By outbox ID.
    FollowUp(i64, FollowUp),
    Surfaced(MessageId),
    /// A message a mail rule marks read now.
    ReadAfter(MessageId),
    /// A value this version does not know, or cannot read.
    Other(MetaRow),
}

impl Due {
    pub fn from_row(row: MetaRow) -> Self {
        let parsed = match (row.object_kind.as_str(), row.plugin.as_str()) {
            (kind::MESSAGE, plugin::SNOOZE) => serde_json::from_str(&row.value_json)
                .ok()
                .map(|v| Self::Snooze(MessageId(row.object_id), v)),
            (kind::OUTBOX, plugin::FOLLOW_UP) => serde_json::from_str(&row.value_json)
                .ok()
                .map(|v| Self::FollowUp(row.object_id, v)),
            (kind::MESSAGE, plugin::SURFACED) => Some(Self::Surfaced(MessageId(row.object_id))),
            (kind::MESSAGE, plugin::READ_AFTER) => Some(Self::ReadAfter(MessageId(row.object_id))),
            _ => None,
        };
        parsed.unwrap_or(Self::Other(row))
    }
}

/// Snoozes `message` as `snooze` says.
pub fn set_snooze(
    store: &mut Store,
    message: MessageId,
    snooze: &Snooze,
) -> katna_store::Result<()> {
    store.set_meta(
        kind::MESSAGE,
        message.0,
        plugin::SNOOZE,
        &to_json(snooze),
        Some(snooze.until),
    )
}

/// The snooze of `message`, if it is snoozed.
pub fn snooze_of(store: &Store, message: MessageId) -> katna_store::Result<Option<Snooze>> {
    Ok(store
        .meta(kind::MESSAGE, message.0, plugin::SNOOZE)?
        .and_then(|row| serde_json::from_str(&row.value_json).ok()))
}

/// Every snoozed message, soonest back first.
pub fn snoozed(store: &Store) -> katna_store::Result<Vec<(MessageId, Snooze)>> {
    Ok(store
        .meta_of(plugin::SNOOZE)?
        .into_iter()
        .filter(|row| row.object_kind == kind::MESSAGE)
        .filter_map(|row| {
            let snooze = serde_json::from_str(&row.value_json).ok()?;
            Some((MessageId(row.object_id), snooze))
        })
        .collect())
}

pub fn clear_snooze(store: &mut Store, message: MessageId) -> katna_store::Result<bool> {
    store.remove_meta(kind::MESSAGE, message.0, plugin::SNOOZE)
}

/// Sets the follow-up reminder of outbox entry `outbox`. One waiting for
/// the user is not due again.
pub fn set_follow_up(
    store: &mut Store,
    outbox: i64,
    follow_up: &FollowUp,
) -> katna_store::Result<()> {
    store.set_meta(
        kind::OUTBOX,
        outbox,
        plugin::FOLLOW_UP,
        &to_json(follow_up),
        (!follow_up.waiting).then_some(follow_up.remind_at),
    )
}

pub fn follow_up_of(store: &Store, outbox: i64) -> katna_store::Result<Option<FollowUp>> {
    Ok(store
        .meta(kind::OUTBOX, outbox, plugin::FOLLOW_UP)?
        .and_then(|row| serde_json::from_str(&row.value_json).ok()))
}

/// Every follow-up reminder waiting, by outbox ID, soonest first.
pub fn follow_ups(store: &Store) -> katna_store::Result<Vec<(i64, FollowUp)>> {
    Ok(store
        .meta_of(plugin::FOLLOW_UP)?
        .into_iter()
        .filter(|row| row.object_kind == kind::OUTBOX)
        .filter_map(|row| Some((row.object_id, serde_json::from_str(&row.value_json).ok()?)))
        .collect())
}

pub fn clear_follow_up(store: &mut Store, outbox: i64) -> katna_store::Result<bool> {
    store.remove_meta(kind::OUTBOX, outbox, plugin::FOLLOW_UP)
}

/// Marks `message` as back in the Inbox at `at`, for [`SURFACED_FOR`].
pub fn set_surfaced(store: &mut Store, message: MessageId, at: i64) -> katna_store::Result<()> {
    store.set_meta(
        kind::MESSAGE,
        message.0,
        plugin::SURFACED,
        &to_json(&Surfaced { at }),
        Some(at + SURFACED_FOR),
    )
}

/// Messages that came back to the Inbox, with when.
pub fn surfaced(store: &Store) -> katna_store::Result<Vec<(MessageId, i64)>> {
    Ok(store
        .meta_of(plugin::SURFACED)?
        .into_iter()
        .filter(|row| row.object_kind == kind::MESSAGE)
        .filter_map(|row| {
            let value: Surfaced = serde_json::from_str(&row.value_json).ok()?;
            Some((MessageId(row.object_id), value.at))
        })
        .collect())
}

pub fn clear_surfaced(store: &mut Store, message: MessageId) -> katna_store::Result<bool> {
    store.remove_meta(kind::MESSAGE, message.0, plugin::SURFACED)
}

/// Has a mail rule mark `message` read at `at` (Unix seconds).
pub fn set_read_after(store: &mut Store, message: MessageId, at: i64) -> katna_store::Result<()> {
    store.set_meta(kind::MESSAGE, message.0, plugin::READ_AFTER, "{}", Some(at))
}

pub fn clear_read_after(store: &mut Store, message: MessageId) -> katna_store::Result<bool> {
    store.remove_meta(kind::MESSAGE, message.0, plugin::READ_AFTER)
}

/// Every value due at `now`, typed, soonest first.
pub fn due(store: &Store, now: i64) -> katna_store::Result<Vec<Due>> {
    Ok(store
        .due_meta(now)?
        .into_iter()
        .map(Due::from_row)
        .collect())
}

fn to_json(value: &impl Serialize) -> String {
    serde_json::to_string(value).expect("metadata values serialize")
}

/// The current time, in Unix seconds.
pub fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}

/// How long to sleep at `now` when the next expiry is `next`: until then,
/// but never longer than [`MAX_SLEEP`].
pub fn sleep_for(now: i64, next: Option<i64>) -> Duration {
    match next {
        Some(next) if next <= now => Duration::ZERO,
        Some(next) => {
            Duration::from_secs(u64::try_from(next - now).unwrap_or(u64::MAX)).min(MAX_SLEEP)
        }
        None => MAX_SLEEP,
    }
}

/// Wakes the scheduler, after a new value or when the computer resumed.
#[derive(Debug, Clone)]
pub struct Waker(Sender<()>);

impl Waker {
    pub fn wake(&self) {
        let _ = self.0.try_send(());
    }
}

/// A scheduler's [`Waker`] and the receiver [`run`] takes.
pub fn waker() -> (Waker, Receiver<()>) {
    let (sender, receiver) = async_channel::bounded(1);
    (Waker(sender), receiver)
}

/// The scheduler: calls `tick` with the time now, which acts on what is due
/// and returns the next expiry; sleeps until then (at most [`MAX_SLEEP`]),
/// or until woken. Ends when every [`Waker`] is gone.
pub async fn run<F, Fut>(wakes: Receiver<()>, mut tick: F)
where
    F: FnMut(i64) -> Fut,
    Fut: Future<Output = Option<i64>>,
{
    loop {
        let next = tick(unix_now()).await;
        let sleep = sleep_for(unix_now(), next);
        let woken = async {
            async_io::Timer::after(sleep).await;
            true
        }
        .or(async { wakes.recv().await.is_ok() })
        .await;
        if !woken {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_core::Paths;
    use katna_store::Mode;

    #[test]
    fn sleeps_are_capped() {
        assert_eq!(sleep_for(100, Some(90)), Duration::ZERO);
        assert_eq!(sleep_for(100, Some(130)), Duration::from_secs(30));
        assert_eq!(sleep_for(100, Some(100 + 3 * 86_400)), MAX_SLEEP);
        assert_eq!(sleep_for(100, None), MAX_SLEEP);
    }

    #[test]
    fn values_round_trip_and_come_due() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let snooze = Snooze {
            until: 500,
            back_to: 1,
            snoozed_in: 9,
            newest: 3,
        };
        set_snooze(&mut store, MessageId(4), &snooze).unwrap();
        let follow_up = FollowUp {
            account: 1,
            message_id: "<a@b>".into(),
            subject: "Offer".into(),
            remind_at: 300,
            after: 86_400,
            ..FollowUp::default()
        };
        set_follow_up(&mut store, 2, &follow_up).unwrap();
        set_surfaced(&mut store, MessageId(5), 100).unwrap();
        store
            .set_meta("message", 6, "from-the-future", "{}", Some(50))
            .unwrap();

        assert_eq!(snooze_of(&store, MessageId(4)).unwrap(), Some(snooze));
        assert_eq!(snoozed(&store).unwrap(), [(MessageId(4), snooze)]);
        assert_eq!(follow_ups(&store).unwrap(), [(2, follow_up.clone())]);
        assert_eq!(surfaced(&store).unwrap(), [(MessageId(5), 100)]);

        let due_now = due(&store, 400).unwrap();
        assert_eq!(due_now.len(), 2);
        assert!(matches!(due_now[0], Due::Other(_)));
        assert_eq!(due_now[1], Due::FollowUp(2, follow_up));
        assert_eq!(
            due(&store, 100 + SURFACED_FOR).unwrap().last(),
            Some(&Due::Surfaced(MessageId(5)))
        );

        assert!(clear_snooze(&mut store, MessageId(4)).unwrap());
        assert!(clear_follow_up(&mut store, 2).unwrap());
        assert!(clear_surfaced(&mut store, MessageId(5)).unwrap());
        assert_eq!(snooze_of(&store, MessageId(4)).unwrap(), None);
    }

    #[test]
    fn follow_ups_read_old_values_and_waiting_ones_are_not_due() {
        let old =
            r#"{"account":1,"message_id":"<a@b>","subject":"Offer","remind_at":300,"after":60}"#;
        let read: FollowUp = serde_json::from_str(old).unwrap();
        assert!(!read.sends() && !read.waiting && read.sent.is_empty());
        assert_eq!(to_json(&read), old);

        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
        let waiting = FollowUp {
            remind_at: 300,
            mail: Some("Subject: Re: Offer\r\n\r\nHi\r\n".into()),
            again: 7 * 86_400,
            waiting: true,
            ..FollowUp::default()
        };
        assert!(waiting.sends() && waiting.sends_again());
        set_follow_up(&mut store, 3, &waiting).unwrap();
        assert!(due(&store, 1_000).unwrap().is_empty());
        assert_eq!(follow_up_of(&store, 3).unwrap(), Some(waiting));
    }

    #[test]
    fn a_wake_ticks_at_once_and_the_last_waker_stops_it() {
        let (waker, wakes) = waker();
        let waker = std::cell::RefCell::new(Some(waker));
        let mut ticks = 0;
        smol::block_on(run(wakes, |_| {
            ticks += 1;
            match ticks {
                // Woken: the next tick comes without the hour's wait.
                1 => waker.borrow().as_ref().unwrap().wake(),
                // The last waker goes: the scheduler stops.
                _ => drop(waker.borrow_mut().take()),
            }
            async { Some(unix_now() + 3600) }
        }));
        assert_eq!(ticks, 2);
    }
}
