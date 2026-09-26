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

use crate::{Error, MailSender, Result, ops};

/// Opens SMTP connections for the outbox.
pub trait Outgoing: Send + Sync + 'static {
    type Sender: MailSender;

    /// Connects and logs in to the SMTP server of `account`.
    fn connect(&self, account: AccountId) -> impl Future<Output = Result<Self::Sender>> + Send;

    /// Whether the server files sent mail in the Sent folder by itself, as
    /// Gmail does.
    fn files_sent_mail(&self, account: AccountId) -> bool;
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
    let envelope = envelope(raw).map_err(QueueError::Invalid)?;
    let domain = envelope
        .from
        .rsplit_once('@')
        .map_or("katna.invalid", |(_, domain)| domain);
    let raw = complete_headers(raw, now, domain);
    let parsed = katna_import::parse_message(&raw).unwrap_or_default();
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
        raw: &raw,
        message_id_hdr: parsed.message_id.as_deref(),
        subject: parsed.subject.as_deref(),
        date: parsed.date.or(Some(now)),
        flags: MessageFlags::SEEN,
        has_attachments: parsed.has_attachments,
        list_id: None,
        snippet: parsed.snippet.as_deref(),
        participants: &participants,
        in_reply_to: parsed.in_reply_to.as_deref(),
        references: &references,
        category: Some(parsed.category),
    };
    let mut batch = store.mail_batch()?;
    let id = batch.add_outgoing(account, &message)?;
    let entry = batch.queue_send(id, now + i64::from(delay))?;
    batch.commit()?;
    Ok(entry)
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
fn complete_headers(raw: &[u8], now: i64, domain: &str) -> Vec<u8> {
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
fn rfc5322_date(now: i64) -> String {
    let days = now.div_euclid(86_400);
    let secs = now.rem_euclid(86_400);
    // Howard Hinnant's civil-from-days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
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
                Ok(Delivery::Sent) => offline_wait = config.retry_min,
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
    Offline,
    Refused,
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
    let result = match raw {
        Some(raw) => deliver(outgoing, entry.account, &raw).await,
        None => Err(Error::Rejected("the message is gone from the store".into())),
    };

    let now = unix_now();
    let mut batch = store.mail_batch()?;
    let delivery = match result {
        Ok(()) => {
            batch.set_send_state(entry.id, SendState::Sent, None, None)?;
            batch.commit()?;
            tracing::info!(id = entry.id, account = %entry.account, "sent");
            file(outgoing, store, entry.account, entry.message);
            changed(SendState::Sent, String::new());
            return Ok(Delivery::Sent);
        }
        Err(error @ (Error::Rejected(_) | Error::Auth(_))) => {
            let attempts = entry.attempts + 1;
            if attempts >= config.max_refusals {
                tracing::warn!(id = entry.id, %error, "giving up sending");
                batch.set_send_state(entry.id, SendState::Failed, None, Some(attempts))?;
                changed(SendState::Failed, error.to_string());
            } else {
                let wait = config.retry_min * 2u32.pow(attempts - 1);
                batch.set_send_state(
                    entry.id,
                    SendState::Queued,
                    Some(now + seconds(wait)),
                    Some(attempts),
                )?;
                changed(SendState::Queued, error.to_string());
            }
            Delivery::Refused
        }
        Err(error) => {
            tracing::info!(id = entry.id, %error, "cannot send now");
            batch.set_send_state(
                entry.id,
                SendState::Queued,
                Some(now + seconds(config.retry_min)),
                None,
            )?;
            changed(SendState::Queued, format!("offline: {error}"));
            Delivery::Offline
        }
    };
    batch.commit()?;
    Ok(delivery)
}

async fn deliver<O: Outgoing>(outgoing: &O, account: AccountId, raw: &[u8]) -> Result<()> {
    let envelope = envelope(raw).map_err(Error::Rejected)?;
    let to: Vec<&str> = envelope.to.iter().map(String::as_str).collect();
    let mut sender = outgoing.connect(account).await?;
    sender.send(&envelope.from, &to, without_bcc(raw)).await?;
    if let Err(error) = sender.quit().await {
        tracing::debug!(%error, "SMTP QUIT after sending");
    }
    Ok(())
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
