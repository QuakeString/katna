// SPDX-License-Identifier: GPL-3.0-or-later

//! Sending mail (plan task 1.8, `docs/ARCHITECTURE.md` §11).
//!
//! 1. [`queue`] stores the message outside every folder and queues it in
//!    the `outbox` table for `now + delay`. The delay is undo send:
//!    [`cancel`] works until the message is handed to the server.
//! 2. [`run`] sends due messages over SMTP, one connection per message.
//!    The envelope comes from `From`, `To`, `Cc` and `Bcc`; the `Bcc`
//!    header itself is never sent.
//! 3. A sent message is filed in the Sent folder through the operation
//!    queue ([`ops::file_sent`]), except where the server does that itself
//!    (Gmail).
//!
//! Network failures are retried with growing waits for as long as it
//! takes. A server that refuses the message (or the login) gets
//! [`OutboxConfig::max_refusals`] tries; then the message is marked
//! failed and kept.

use std::{
    future::Future,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_channel::{Receiver, Sender};
use async_io::Timer;
use futures_lite::FutureExt;
use katna_core::AccountId;
use katna_store::{
    MessageFlags, MessageId, NewMessage, NewParticipant, OutboxEntry, SendState, Store,
};

use crate::tracking::{self, rewrite};
use crate::{Error, MailSender, Result, ops};

/// Most recipients a tracked message may have; more go out untracked.
pub const MAX_TRACKED_RECIPIENTS: usize = 50;

/// Opens SMTP connections for the outbox.
pub trait Outgoing: Send + Sync + 'static {
    type Sender: MailSender;

    /// Connects and logs in to the SMTP server of `account`.
    fn connect(&self, account: AccountId) -> impl Future<Output = Result<Self::Sender>> + Send;

    /// Whether the server files sent mail in the Sent folder by itself, as
    /// Gmail does.
    fn files_sent_mail(&self, account: AccountId) -> bool;

    /// The tracking server, when tracking is on (`docs/ARCHITECTURE.md`
    /// §16.1).
    fn tracking(&self) -> Option<tracking::Client> {
        None
    }

    /// The user's GnuPG, which signs and encrypts the tracked copies of
    /// signed and encrypted mail.
    fn gnupg(&self) -> katna_crypto::Gnupg {
        katna_crypto::Gnupg::new()
    }

    /// This computer's Katna Server token; the server takes it only while
    /// signed in to a Katna account (§16.2).
    fn tracking_token(&self) -> impl Future<Output = Result<String>> + Send {
        async { Err(Error::Rejected("tracking is off".into())) }
    }
}

/// Retry timing of the outbox.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutboxConfig {
    /// First wait after a failure.
    pub retry_min: Duration,
    /// Longest wait after network failures.
    pub retry_max: Duration,
    /// Tries a server may refuse before the message is marked failed.
    pub max_refusals: u32,
}

impl Default for OutboxConfig {
    fn default() -> Self {
        Self {
            retry_min: Duration::from_secs(30),
            retry_max: Duration::from_secs(15 * 60),
            max_refusals: 3,
        }
    }
}

/// A change of an outbox entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboxEvent {
    pub id: i64,
    pub account: AccountId,
    pub state: SendState,
    /// Why it is queued again or failed; empty otherwise.
    pub detail: String,
}

/// Why a message cannot be queued.
#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    Store(#[from] katna_store::Error),
}

/// Queues `raw` (an RFC 5322 message) from `account` to be sent at
/// `now + delay` seconds. Adds `Date` and `Message-ID` if they are
/// missing. Returns the outbox entry's ID.
pub fn queue(
    store: &mut Store,
    account: AccountId,
    raw: &[u8],
    delay: u32,
    now: i64,
) -> std::result::Result<i64, QueueError> {
    queue_held(
        store,
        account,
        raw,
        now + i64::from(delay),
        None,
        now,
        false,
    )
}

/// Like [`queue`]; with `tracked`, the message goes out as one tracked
/// copy per recipient when it can be (§11, §16.1).
pub fn queue_with(
    store: &mut Store,
    account: AccountId,
    raw: &[u8],
    delay: u32,
    now: i64,
    tracked: bool,
) -> std::result::Result<i64, QueueError> {
    queue_held(
        store,
        account,
        raw,
        now + i64::from(delay),
        None,
        now,
        tracked,
    )
}

/// Queues `raw` from `account` to go out at `at` (Unix seconds), as
/// scheduled mail. After the undo delay (`delay` seconds) the outbox hands
/// it to an SMTP server that holds mail until then (RFC 4865
/// `FUTURERELEASE`), so it goes out with this computer off; other servers
/// get it at `at`. Returns the outbox entry's ID.
pub fn schedule(
    store: &mut Store,
    account: AccountId,
    raw: &[u8],
    delay: u32,
    at: i64,
    now: i64,
) -> std::result::Result<i64, QueueError> {
    let hand_over = now + i64::from(delay);
    queue_held(
        store,
        account,
        raw,
        hand_over,
        Some(at).filter(|&at| at > hand_over),
        now,
        false,
    )
}

fn queue_held(
    store: &mut Store,
    account: AccountId,
    raw: &[u8],
    send_at: i64,
    hold_until: Option<i64>,
    now: i64,
    tracked: bool,
) -> std::result::Result<i64, QueueError> {
    let asked = katna_store::take_delivery_receipt(raw);
    let receipt = asked.is_some();
    let raw = asked.as_deref().unwrap_or(raw);
    let envelope = envelope(raw).map_err(QueueError::Invalid)?;
    let domain = envelope
        .from
        .rsplit_once('@')
        .map_or("katna.invalid", |(_, domain)| domain);
    let raw = complete_headers(raw, now, domain);
    with_message(&raw, now, MessageFlags::SEEN, |message| {
        let mut batch = store.mail_batch()?;
        let id = batch.add_outgoing(account, message)?;
        let entry = if tracked {
            batch.queue_send_as(id, send_at, true)?
        } else {
            batch.queue_held(id, send_at, hold_until)?
        };
        if receipt {
            batch.ask_delivery_receipt(entry)?;
        }
        batch.commit()?;
        Ok(entry)
    })
}

/// Scheduled mail closer to its time than this (seconds) is sent as is,
/// not held by the server.
const HOLD_MARGIN: i64 = 60;

/// Reads `raw` (with `Date` and `Message-ID`) into the message the store
/// keeps, with `flags`, and hands it to `f`.
pub(crate) fn with_message<R>(
    raw: &[u8],
    now: i64,
    flags: MessageFlags,
    f: impl FnOnce(&NewMessage<'_>) -> R,
) -> R {
    let parsed = katna_import::parse_message(raw).unwrap_or_default();
    let participants: Vec<NewParticipant<'_>> = parsed
        .participants
        .iter()
        .map(|p| NewParticipant {
            role: p.role,
            email_norm: &p.email_norm,
            domain: &p.domain,
            display_name: p.display_name.as_deref(),
        })
        .collect();
    // A reply joins its conversation.
    let references: Vec<&str> = parsed.references.iter().map(String::as_str).collect();
    let message = NewMessage {
        raw,
        message_id_hdr: parsed.message_id.as_deref(),
        subject: parsed.subject.as_deref(),
        date: parsed.date.or(Some(now)),
        flags,
        has_attachments: parsed.has_attachments,
        list_id: None,
        snippet: parsed.snippet.as_deref(),
        participants: &participants,
        in_reply_to: parsed.in_reply_to.as_deref(),
        references: &references,
        category: Some(parsed.category),
    };
    f(&message)
}

/// Cancels a queued message. Returns `false` when it is already being
/// sent or done.
pub fn cancel(store: &mut Store, id: i64) -> katna_store::Result<bool> {
    let mut batch = store.mail_batch()?;
    let cancelled = batch.cancel_send(id)?;
    batch.commit()?;
    Ok(cancelled)
}

/// Forgets a cancelled or failed message. Returns `false` for any other.
pub fn discard(store: &mut Store, id: i64) -> katna_store::Result<bool> {
    let Some(entry) = store.outbox_entry(id)? else {
        return Ok(false);
    };
    if !matches!(entry.state, SendState::Cancelled | SendState::Failed) {
        return Ok(false);
    }
    let mut batch = store.mail_batch()?;
    batch.forget_outgoing(entry.message)?;
    batch.commit()?;
    Ok(true)
}

/// The SMTP envelope of a message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub from: String,
    pub to: Vec<String>,
}

/// Reads the envelope from the headers: the first `From` address, and
/// every `To`, `Cc` and `Bcc` address once.
pub fn envelope(raw: &[u8]) -> std::result::Result<Envelope, String> {
    use katna_store::ParticipantRole as Role;
    let parsed = katna_import::parse_message(raw).ok_or("the message has no headers")?;
    let from = parsed
        .participants
        .iter()
        .find(|p| p.role == Role::From && !p.domain.is_empty())
        .ok_or("the message has no From address")?
        .email_norm
        .clone();
    let mut to: Vec<String> = Vec::new();
    for p in &parsed.participants {
        if matches!(p.role, Role::To | Role::Cc | Role::Bcc)
            && !p.domain.is_empty()
            && !to.contains(&p.email_norm)
        {
            to.push(p.email_norm.clone());
        }
    }
    if to.is_empty() {
        return Err("the message has no recipients".into());
    }
    Ok(Envelope { from, to })
}

/// A message's `Message-ID` (without angle brackets) and its envelope
/// recipients.
fn sent_to(raw: &[u8]) -> Option<(String, Vec<String>)> {
    let message_id = katna_import::parse_message(raw)?.message_id?;
    Some((message_id, envelope(raw).ok()?.to))
}

/// `raw` without its `Bcc` header, which recipients must not see.
pub fn without_bcc(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len());
    let mut skipping = false;
    let mut rest = raw;
    while !rest.is_empty() {
        let end = rest
            .iter()
            .position(|&b| b == b'\n')
            .map_or(rest.len(), |at| at + 1);
        let (line, tail) = rest.split_at(end);
        if line == b"\r\n" || line == b"\n" {
            // End of the header: the body goes out unchanged.
            out.extend_from_slice(rest);
            break;
        }
        let continued = matches!(line.first(), Some(b' ' | b'\t'));
        if !continued {
            skipping = line.len() >= 4 && line[..4].eq_ignore_ascii_case(b"bcc:");
        }
        if !skipping {
            out.extend_from_slice(line);
        }
        rest = tail;
    }
    out
}

/// Adds `Date` and `Message-ID` headers when `raw` has none.
pub(crate) fn complete_headers(raw: &[u8], now: i64, domain: &str) -> Vec<u8> {
    let header_end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .or_else(|| raw.windows(2).position(|w| w == b"\n\n"))
        .unwrap_or(raw.len());
    let header = &raw[..header_end];
    let has = |name: &[u8]| {
        header.split(|&b| b == b'\n').any(|line| {
            line.len() > name.len()
                && line[..name.len()].eq_ignore_ascii_case(name)
                && line[name.len()] == b':'
        })
    };
    let mut extra = String::new();
    if !has(b"date") {
        extra.push_str(&format!("Date: {}\r\n", rfc5322_date(now)));
    }
    if !has(b"message-id") {
        extra.push_str(&format!("Message-ID: <{}@{domain}>\r\n", unique_id(now)));
    }
    let mut out = extra.into_bytes();
    out.extend_from_slice(raw);
    out
}

/// A Message-ID left part that no other message of this process or
/// another run shares.
fn unique_id(now: i64) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{now:x}.{nanos:x}.{:x}.{count}.katna", std::process::id())
}

/// `now` as an RFC 5322 date in UTC, for example
/// `Sat, 26 Sep 2026 10:00:00 +0000`.
pub(crate) fn rfc5322_date(now: i64) -> String {
    let days = now.div_euclid(86_400);
    let secs = now.rem_euclid(86_400);
    let (year, month, day) = civil_date(days);
    const WEEKDAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    format!(
        "{}, {day:02} {} {year} {:02}:{:02}:{:02} +0000",
        WEEKDAYS[days.rem_euclid(7) as usize],
        MONTHS[(month - 1) as usize],
        secs / 3600,
        secs / 60 % 60,
        secs % 60
    )
}

/// The year, month and day of `days` since 1970-01-01 (Howard Hinnant's
/// civil-from-days).
pub(crate) fn civil_date(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// The daemon's side of the outbox: wakes it after queueing, and stops it
/// when dropped.
pub struct OutboxHandle {
    _stop: Sender<()>,
    wake: Sender<()>,
}

impl OutboxHandle {
    /// Looks at the queue again now, for example after [`queue`].
    pub fn wake(&self) {
        let _ = self.wake.try_send(());
    }
}

/// The outbox's side of an [`OutboxHandle`].
pub struct OutboxControl {
    stop: Receiver<()>,
    wake: Receiver<()>,
}

/// A new handle and the control it drives.
pub fn control() -> (OutboxHandle, OutboxControl) {
    let (stop_tx, stop) = async_channel::bounded(1);
    let (wake, wake_rx) = async_channel::bounded(1);
    (
        OutboxHandle {
            _stop: stop_tx,
            wake,
        },
        OutboxControl {
            stop,
            wake: wake_rx,
        },
    )
}

/// Sends queued mail until the [`OutboxHandle`] is dropped.
pub async fn run<O: Outgoing>(
    outgoing: O,
    mut store: Store,
    config: OutboxConfig,
    events: Sender<OutboxEvent>,
    control: OutboxControl,
) {
    match store.mail_batch().and_then(|mut batch| {
        batch.requeue_sending()?;
        batch.commit()
    }) {
        Ok(()) => {}
        Err(err) => tracing::warn!(%err, "could not requeue interrupted sends"),
    }
    // Waits after network failures grow together: they mean we are offline.
    let mut offline_wait = config.retry_min;
    loop {
        if control.stop.is_closed() {
            return;
        }
        release(&outgoing, &mut store, &events);
        let due = match store.due_sends(unix_now(), 20) {
            Ok(due) => due,
            Err(err) => {
                tracing::warn!(%err, "cannot read the outbox");
                Vec::new()
            }
        };
        for entry in &due {
            if control.stop.is_closed() {
                return;
            }
            match send(&outgoing, &mut store, &config, entry, &events).await {
                Ok(Delivery::Sent | Delivery::Waiting) => offline_wait = config.retry_min,
                Ok(Delivery::Offline) => {
                    offline_wait = (offline_wait * 2).min(config.retry_max);
                }
                Ok(Delivery::Refused) => {}
                Err(err) => tracing::warn!(id = entry.id, %err, "outbox store error"),
            }
        }
        // Wait for the next due message, a wake-up or the stop.
        let wait = match store.next_send_at() {
            Ok(Some(at)) => Duration::from_secs(at.saturating_sub(unix_now()).max(0) as u64),
            Ok(None) => Duration::from_secs(24 * 3600),
            Err(_) => config.retry_max,
        };
        if wait.is_zero() && !due.is_empty() {
            continue;
        }
        let stopped = async {
            Timer::after(wait.max(Duration::from_millis(100))).await;
            false
        }
        .or(async {
            match control.wake.recv().await {
                Ok(()) => false,
                Err(_) => true,
            }
        })
        .or(async {
            let _ = control.stop.recv().await;
            true
        })
        .await;
        if stopped {
            return;
        }
    }
}

enum Delivery {
    Sent,
    /// Scheduled mail waits for its time, here or for the server.
    Waiting,
    Offline,
    Refused,
}

/// Files the mail a server held that went out by now.
fn release<O: Outgoing>(outgoing: &O, store: &mut Store, events: &Sender<OutboxEvent>) {
    let released = match store.released_sends(unix_now()) {
        Ok(released) => released,
        Err(err) => {
            tracing::warn!(%err, "cannot read held mail");
            return;
        }
    };
    for entry in released {
        let cleared = store.mail_batch().and_then(|mut batch| {
            batch.clear_hold(entry.id)?;
            batch.commit()
        });
        if let Err(err) = cleared {
            tracing::warn!(id = entry.id, %err, "cannot file held mail");
            continue;
        }
        tracing::info!(id = entry.id, account = %entry.account, "held mail went out");
        file(outgoing, store, entry.account, entry.message);
        let _ = events.try_send(OutboxEvent {
            id: entry.id,
            account: entry.account,
            state: SendState::Sent,
            detail: String::new(),
        });
    }
}

/// What handing a message to the server came to.
enum Handover {
    /// Sent now.
    Sent,
    /// The server holds it until its time.
    Held,
    /// The server cannot hold it that long, or at all: hand it over again
    /// at this time.
    Later(i64),
}

/// Sends one entry and records the outcome.
async fn send<O: Outgoing>(
    outgoing: &O,
    store: &mut Store,
    config: &OutboxConfig,
    entry: &OutboxEntry,
    events: &Sender<OutboxEvent>,
) -> katna_store::Result<Delivery> {
    let changed = |state, detail: String| {
        let _ = events.try_send(OutboxEvent {
            id: entry.id,
            account: entry.account,
            state,
            detail,
        });
    };
    // Scheduled mail due soon waits here for its time; the server gets the
    // rest to hold.
    let now = unix_now();
    if let Some(at) = entry
        .hold_until
        .filter(|&at| at > now && at <= now + HOLD_MARGIN)
    {
        let mut batch = store.mail_batch()?;
        batch.set_send_state(entry.id, SendState::Queued, Some(at), None)?;
        batch.commit()?;
        return Ok(Delivery::Waiting);
    }
    {
        // In a block: a batch must not live across an await.
        let mut batch = store.mail_batch()?;
        batch.set_send_state(entry.id, SendState::Sending, None, None)?;
        batch.commit()?;
    }
    changed(SendState::Sending, String::new());

    let raw = store
        .messages_by_id(&[entry.message])?
        .into_iter()
        .next()
        .and_then(|m| m.blob_hash)
        .map(|hash| store.blobs().get(&hash))
        .transpose()?
        .flatten();
    let hold = entry.hold_until.filter(|&at| at > now);
    // For the ticks beside its recipients: when it went out to whom.
    let sent = raw.as_deref().and_then(sent_to);
    let sealed = raw
        .as_deref()
        .is_some_and(|raw| katna_crypto::protection(raw).is_some());
    let mut tracked = None;
    let result = match (raw, hold) {
        (Some(raw), Some(at)) => hand_over(outgoing, entry, &raw, at).await,
        (Some(raw), None) if entry.per_recipient => {
            // Signed or encrypted: tracked from its content, each copy
            // signed or encrypted again.
            let opened = match outgoing.tracking() {
                Some(_) => open_sealed(&raw, outgoing.gnupg()).await,
                None => Ok(None),
            };
            tracked = match &opened {
                Ok(opened) => {
                    let content = opened.as_ref().map_or(&raw[..], |s| &s.content[..]);
                    prepare_tracking(outgoing, store, entry, content).await?
                }
                Err(why) => {
                    tracing::info!(id = entry.id, why, "sending untracked");
                    None
                }
            };
            match (&tracked, opened) {
                (Some(plan), Ok(Some(opened))) => {
                    deliver_tracked(
                        outgoing,
                        store,
                        entry,
                        &opened.content,
                        plan,
                        Some(opened.how),
                    )
                    .await
                }
                (Some(plan), _) => deliver_tracked(outgoing, store, entry, &raw, plan, None).await,
                (None, _) => deliver(outgoing, entry, &raw).await,
            }
            .map(|()| Handover::Sent)
        }
        (Some(raw), None) => deliver(outgoing, entry, &raw)
            .await
            .map(|()| Handover::Sent),
        (None, _) => Err(Error::Rejected("the message is gone from the store".into())),
    };

    let now = unix_now();
    let mut batch = store.mail_batch()?;
    let (delivery, state, detail) = match result {
        Ok(Handover::Held) => {
            // Filed in Sent when it goes out; see `release`.
            batch.set_send_state(entry.id, SendState::Sent, None, None)?;
            if let Some((message_id, to)) = &sent {
                batch.receipt_sent(message_id, to, entry.hold_until.unwrap_or(now))?;
            }
            batch.commit()?;
            tracing::info!(id = entry.id, account = %entry.account, "held by the server");
            changed(SendState::Sent, String::new());
            return Ok(Delivery::Sent);
        }
        Ok(Handover::Sent) => {
            batch.set_send_state(entry.id, SendState::Sent, None, None)?;
            batch.clear_hold(entry.id)?;
            if let Some((message_id, to)) = &sent {
                batch.receipt_sent(message_id, to, now)?;
            }
            batch.commit()?;
            tracing::info!(id = entry.id, account = %entry.account, "sent");
            match &tracked {
                Some(plan) => {
                    store.tracking_sent(plan.message.id, now)?;
                    file_tracked(outgoing, store, entry, plan, sealed);
                }
                None => file(outgoing, store, entry.account, entry.message),
            }
            changed(SendState::Sent, String::new());
            return Ok(Delivery::Sent);
        }
        Ok(Handover::Later(at)) => {
            tracing::info!(
                id = entry.id,
                at,
                "the server cannot hold it; sending it then"
            );
            batch.set_send_state(entry.id, SendState::Queued, Some(at), None)?;
            (Delivery::Waiting, SendState::Queued, String::new())
        }
        Err(error @ (Error::Rejected(_) | Error::Auth(_))) => {
            let attempts = entry.attempts + 1;
            if attempts >= config.max_refusals {
                tracing::warn!(id = entry.id, %error, "giving up sending");
                batch.set_send_state(entry.id, SendState::Failed, None, Some(attempts))?;
                (Delivery::Refused, SendState::Failed, error.to_string())
            } else {
                let wait = config.retry_min * 2u32.pow(attempts - 1);
                batch.set_send_state(
                    entry.id,
                    SendState::Queued,
                    Some(now + seconds(wait)),
                    Some(attempts),
                )?;
                (Delivery::Refused, SendState::Queued, error.to_string())
            }
        }
        Err(error) => {
            tracing::info!(id = entry.id, %error, "cannot send now");
            batch.set_send_state(
                entry.id,
                SendState::Queued,
                Some(now + seconds(config.retry_min)),
                None,
            )?;
            (
                Delivery::Offline,
                SendState::Queued,
                format!("offline: {error}"),
            )
        }
    };
    // Told only once committed: whoever hears it reads the new state.
    batch.commit()?;
    changed(state, detail);
    Ok(delivery)
}

/// Connects to send `entry`, asking for delivery receipts if it wants them.
async fn connect<O: Outgoing>(outgoing: &O, entry: &OutboxEntry) -> Result<O::Sender> {
    let mut sender = outgoing.connect(entry.account).await?;
    sender.ask_for_receipts(entry.delivery_receipt);
    Ok(sender)
}

async fn deliver<O: Outgoing>(outgoing: &O, entry: &OutboxEntry, raw: &[u8]) -> Result<()> {
    let envelope = envelope(raw).map_err(Error::Rejected)?;
    let to: Vec<&str> = envelope.to.iter().map(String::as_str).collect();
    let mut sender = connect(outgoing, entry).await?;
    sender.send(&envelope.from, &to, without_bcc(raw)).await?;
    if let Err(error) = sender.quit().await {
        tracing::debug!(%error, "SMTP QUIT after sending");
    }
    Ok(())
}

/// Hands scheduled mail to the server to hold until `at`, when it can.
async fn hand_over<O: Outgoing>(
    outgoing: &O,
    entry: &OutboxEntry,
    raw: &[u8],
    at: i64,
) -> Result<Handover> {
    let envelope = envelope(raw).map_err(Error::Rejected)?;
    let to: Vec<&str> = envelope.to.iter().map(String::as_str).collect();
    let mut sender = connect(outgoing, entry).await?;
    let wait = at - unix_now();
    // A hold shorter than the margins is no use.
    let limit = sender
        .hold_limit()
        .await?
        .filter(|&limit| limit > 2 * HOLD_MARGIN as u64);
    let handover = match limit {
        Some(limit) if wait <= i64::try_from(limit).unwrap_or(i64::MAX) => {
            sender
                .send_held(&envelope.from, &to, without_bcc(raw), at)
                .await?;
            Handover::Held
        }
        // Too far off: again once it is within the server's limit.
        Some(limit) => Handover::Later(at - i64::try_from(limit).unwrap_or(0) + HOLD_MARGIN),
        None => Handover::Later(at),
    };
    if let Err(error) = sender.quit().await {
        tracing::debug!(%error, "SMTP QUIT after handing over");
    }
    Ok(handover)
}

/// A signed or encrypted message opened for tracking: its content, and
/// how each tracked copy is protected again.
struct Sealed {
    content: Vec<u8>,
    how: katna_crypto::Protect,
}

/// Marks the signed or encrypted tracked copies, whose tracking links are
/// out of sight, so Gmail's filed copies of them can be found (the clean
/// copy has none).
const SEALED_COPY: &str = "X-Katna-Copy";

/// Opens a signed or encrypted message (`Ok(None)` for others) with the
/// user's GnuPG, to track its content. `Err` when it can't be: it then
/// goes out as it is, untracked. Decrypting uses the sender's own key,
/// which every encrypted message from Katna is encrypted to.
async fn open_sealed(
    raw: &[u8],
    gnupg: katna_crypto::Gnupg,
) -> std::result::Result<Option<Sealed>, &'static str> {
    if katna_crypto::protection(raw).is_none() {
        return Ok(None);
    }
    let raw = raw.to_vec();
    let opened = blocking::unblock(move || katna_crypto::open(&raw, &gnupg)).await;
    let opened = opened.ok_or("signed or encrypted, but could not be opened")?;
    let security = &opened.security;
    if !security.whole {
        return Err("only part of it is signed or encrypted");
    }
    if security.encrypted() && !security.decrypted() {
        return Err("encrypted, and could not be decrypted");
    }
    Ok(Some(Sealed {
        how: katna_crypto::Protect {
            standard: security.standard,
            sign: !security.signatures.is_empty(),
            encrypt: security.encrypted(),
        },
        content: opened.raw,
    }))
}

/// A tracked copy for `recipient` signed and/or encrypted as `how`: for
/// them (and the sender) only, marked for [`file_tracked`].
async fn seal_copy(
    gnupg: katna_crypto::Gnupg,
    copy: Vec<u8>,
    how: katna_crypto::Protect,
    sender: &str,
    recipient: &str,
    tracking_id: &str,
) -> Result<Vec<u8>> {
    let mut marked = format!("{SEALED_COPY}: {tracking_id}\r\n").into_bytes();
    marked.extend_from_slice(&copy);
    let recipients = katna_crypto::Recipients {
        sender: sender.to_owned(),
        visible: vec![recipient.to_owned()],
        hidden: Vec::new(),
    };
    blocking::unblock(move || katna_crypto::protect(&marked, how, &recipients, &gnupg))
        .await
        .map_err(|err| Error::Rejected(err.to_string()))
}

/// How a tracked message goes out: the tracking server's address and
/// each recipient's tracking ID.
struct TrackingPlan {
    base: String,
    message: katna_store::TrackedMessage,
}

/// Gets tracking IDs for `entry`'s recipients (once; a retry reuses them).
/// `raw` is the message, opened when it is signed or encrypted.
/// `Ok(None)` sends the message untracked: tracking is off or the server
/// cannot be reached, the message has neither an HTML version nor links
/// in its plain text, or it has too many recipients. Mail is never held
/// back for tracking.
async fn prepare_tracking<O: Outgoing>(
    outgoing: &O,
    store: &mut Store,
    entry: &OutboxEntry,
    raw: &[u8],
) -> katna_store::Result<Option<TrackingPlan>> {
    let Some(client) = outgoing.tracking() else {
        tracing::info!(id = entry.id, "tracking is off; sending untracked");
        return Ok(None);
    };
    let base = client.server().base().to_owned();
    let parsed = katna_import::parse_message(raw).unwrap_or_default();
    if let Some(message) = store.tracking_for_outbox(entry.id)? {
        // A retry of this message; an old entry's tracking where the
        // outbox gave its ID out again.
        if parsed.message_id.as_deref() == Some(message.message_id.as_str()) {
            return Ok(Some(TrackingPlan { base, message }));
        }
        tracing::info!(id = entry.id, "an old message's tracking had this ID");
        store.detach_tracking(entry.id)?;
    }
    let untracked = |why: &str| {
        tracing::info!(id = entry.id, why, "sending untracked");
        Ok(None)
    };
    let Some(links) = rewrite::links(raw) else {
        return untracked("no text to track");
    };
    if links.is_empty() && !rewrite::tracks_opens(raw) {
        return untracked("plain text without links");
    }
    let Ok(envelope) = envelope(raw) else {
        return untracked("no envelope");
    };
    if envelope.to.len() > MAX_TRACKED_RECIPIENTS {
        return untracked("too many recipients");
    }
    let Some(message_id) = parsed.message_id.clone() else {
        return untracked("no Message-ID");
    };
    let token = match outgoing.tracking_token().await {
        Ok(token) => token,
        Err(error) => {
            tracing::warn!(%error, "no tracking server token");
            return untracked("no tracking server token");
        }
    };
    let ids = match client.create(&token, envelope.to.len(), &links).await {
        Ok(ids) => ids,
        Err(error) => {
            tracing::warn!(%error, "the tracking server did not give IDs");
            return untracked("tracking server unavailable");
        }
    };
    let recipients: Vec<katna_store::NewRecipient<'_>> = envelope
        .to
        .iter()
        .zip(&ids)
        .map(|(email, id)| katna_store::NewRecipient {
            tracking_id: id,
            email,
            name: parsed
                .participants
                .iter()
                .find(|p| p.email_norm == *email)
                .and_then(|p| p.display_name.as_deref()),
        })
        .collect();
    store.start_tracking(
        entry.id,
        entry.account,
        &message_id,
        parsed.subject.as_deref().unwrap_or(""),
        &links,
        &recipients,
        unix_now(),
    )?;
    let message = store
        .tracking_for_outbox(entry.id)?
        .ok_or_else(|| katna_store::Error::InvalidData("tracking was not stored".into()))?;
    Ok(Some(TrackingPlan { base, message }))
}

/// Sends each recipient who has not got it yet their own tracked copy,
/// each in its own SMTP transaction over one connection. The headers are
/// the same in every copy. With `sealed`, `raw` is the opened content of
/// a signed or encrypted message, and each copy is protected again.
async fn deliver_tracked<O: Outgoing>(
    outgoing: &O,
    store: &mut Store,
    entry: &OutboxEntry,
    raw: &[u8],
    plan: &TrackingPlan,
    sealed: Option<katna_crypto::Protect>,
) -> Result<()> {
    let envelope = envelope(raw).map_err(Error::Rejected)?;
    let clean = without_bcc(raw);
    let mut sender = None;
    let mut refused = Vec::new();
    for recipient in plan
        .message
        .recipients
        .iter()
        .filter(|r| r.sent_at.is_none())
    {
        let copy = rewrite::tracked_copy(
            &clean,
            &plan.base,
            &recipient.tracking_id,
            &plan.message.links,
        )
        .unwrap_or_else(|| clean.clone());
        let copy = match sealed {
            Some(how) => {
                seal_copy(
                    outgoing.gnupg(),
                    copy,
                    how,
                    &envelope.from,
                    &recipient.email,
                    &recipient.tracking_id,
                )
                .await?
            }
            None => copy,
        };
        if sender.is_none() {
            sender = Some(connect(outgoing, entry).await?);
        }
        let Some(open) = sender.as_mut() else {
            continue;
        };
        match open
            .send(&envelope.from, &[recipient.email.as_str()], copy)
            .await
        {
            Ok(()) => store.tracked_copy_sent(&recipient.tracking_id, unix_now())?,
            // One recipient refused: the others still get theirs, on a new
            // connection (SMTP is left mid-transaction after an error).
            Err(Error::Rejected(reason)) => {
                refused.push(format!("{}: {reason}", recipient.email));
                sender = None;
            }
            Err(error) => return Err(error),
        }
    }
    if let Some(open) = sender
        && let Err(error) = open.quit().await
    {
        tracing::debug!(%error, "SMTP QUIT after sending");
    }
    if refused.is_empty() {
        return Ok(());
    }
    let delivered: Vec<String> = store
        .tracking_for_outbox(plan.message.outbox_id)?
        .map(|m| {
            m.recipients
                .into_iter()
                .filter(|r| r.sent_at.is_some())
                .map(|r| r.email)
                .collect()
        })
        .unwrap_or_default();
    Err(Error::Rejected(format!(
        "not delivered to {}; delivered to {}",
        refused.join(", "),
        if delivered.is_empty() {
            "nobody".to_owned()
        } else {
            delivered.join(", ")
        }
    )))
}

/// Files the clean copy of a tracked message in Sent; where the server
/// filed every tracked copy itself (Gmail), those are deleted first.
fn file_tracked<O: Outgoing>(
    outgoing: &O,
    store: &mut Store,
    entry: &OutboxEntry,
    plan: &TrackingPlan,
    sealed: bool,
) {
    let result = if outgoing.files_sent_mail(entry.account) {
        // The copies' tracking links (plain text copies have no pixel),
        // or for signed and encrypted copies their mark.
        let marker = if sealed {
            format!("{SEALED_COPY}: ")
        } else {
            format!("{}/", plan.base)
        };
        ops::file_sent_tracked(
            store,
            entry.message,
            &plan.message.message_id,
            &marker,
            plan.message.recipients.len() as u32,
        )
        .map(drop)
    } else {
        ops::file_sent(store, entry.message).map(drop)
    };
    if let Err(err) = result {
        tracing::warn!(%err, "could not file a tracked message");
    }
}

/// Files a sent message in Sent, or forgets it where the server does that.
fn file<O: Outgoing>(outgoing: &O, store: &mut Store, account: AccountId, message: MessageId) {
    let result = if outgoing.files_sent_mail(account) {
        store.mail_batch().and_then(|mut batch| {
            batch.forget_outgoing(message)?;
            batch.commit()
        })
    } else {
        ops::file_sent(store, message)
            .map(drop)
            .map_err(|err| match err {
                ops::ChangeError::Store(err) => err,
                other => katna_store::Error::InvalidData(other.to_string()),
            })
    };
    if let Err(err) = result {
        tracing::warn!(%err, "could not file a sent message");
    }
}

fn seconds(wait: Duration) -> i64 {
    wait.as_secs().max(1) as i64
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcc_is_removed_with_its_continuation_lines() {
        let raw = b"From: a@x.org\r\nBcc: b@x.org,\r\n c@x.org\r\nTo: d@x.org\r\n\r\nBcc: body\r\n";
        assert_eq!(
            without_bcc(raw),
            b"From: a@x.org\r\nTo: d@x.org\r\n\r\nBcc: body\r\n".to_vec()
        );
    }

    #[test]
    fn envelope_takes_every_recipient_once() {
        let raw = b"From: Ada <Ada@X.org>\r\nTo: b@x.org, c@x.org\r\nCc: b@x.org\r\n\
                    Bcc: d@x.org\r\nSubject: hi\r\n\r\nHi.\r\n";
        let found = envelope(raw).unwrap();
        assert_eq!(found.from, "ada@x.org");
        assert_eq!(found.to, ["b@x.org", "c@x.org", "d@x.org"]);
        assert!(envelope(b"From: a@x.org\r\nSubject: hi\r\n\r\n").is_err());
        assert!(envelope(b"To: a@x.org\r\n\r\n").is_err());
    }

    #[test]
    fn missing_headers_are_added() {
        let raw = b"From: a@x.org\r\nTo: b@x.org\r\n\r\nHi.\r\n";
        let done = String::from_utf8(complete_headers(raw, 1_790_416_800, "x.org")).unwrap();
        assert!(
            done.starts_with("Date: Sat, 26 Sep 2026 10:00:00 +0000\r\n"),
            "{done}"
        );
        assert!(done.contains("@x.org>\r\nFrom: a@x.org"), "{done}");
        let kept = b"Date: Mon, 1 Jan 2024 00:00:00 +0000\r\nMessage-ID: <a@b>\r\n\r\n";
        assert_eq!(complete_headers(kept, 0, "x.org"), kept.to_vec());
        assert_eq!(rfc5322_date(0), "Thu, 01 Jan 1970 00:00:00 +0000");
        assert_eq!(rfc5322_date(951_782_400), "Tue, 29 Feb 2000 00:00:00 +0000");
    }
}
