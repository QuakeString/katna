// SPDX-License-Identifier: GPL-3.0-or-later

//! Follows the tracking server's event stream (`docs/ARCHITECTURE.md`
//! §16.1) while any message was sent with tracking: keeps each open and
//! click in the store, tells Katna Mail, and shows a notification when a
//! person first opens a message or follows one of its links. Opens from
//! Apple's privacy proxy (maybe opened) and from security scanners are
//! kept, never announced.

use std::sync::Weak;
use std::time::Duration;

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_store::{TrackingEvent, TrackingNews};
use katna_sync::tracking::ServerEvent;

use crate::daemon::{Daemon, Notice};

/// The server sends a keep-alive every 30 s; silence longer than this
/// means the connection is gone.
const IDLE: Duration = Duration::from_secs(90);
/// Waits after failures grow up to this.
const MAX_BACKOFF: Duration = Duration::from_secs(15 * 60);
/// How often to look again while nothing is tracked.
const QUIET: Duration = Duration::from_secs(3600);

/// Runs until the daemon is gone.
pub(crate) async fn run(daemon: Weak<Daemon>, wake: Receiver<()>) {
    let mut backoff = Duration::from_secs(5);
    loop {
        let pause = match follow(&daemon).await {
            Step::Stop => return,
            // The stream ended normally (the server restarted): at once.
            Step::Reconnect => {
                backoff = Duration::from_secs(5);
                Duration::from_secs(1)
            }
            Step::Failed => {
                let pause = backoff;
                backoff = (backoff * 2).min(MAX_BACKOFF);
                pause
            }
            Step::Idle => QUIET,
        };
        let woken = async {
            let _ = wake.recv().await;
        };
        woken
            .or(async {
                async_io::Timer::after(pause).await;
            })
            .await;
    }
}

enum Step {
    Stop,
    Reconnect,
    Failed,
    /// Tracking is off, nothing was tracked, or no token yet.
    Idle,
}

async fn follow(daemon: &Weak<Daemon>) -> Step {
    let (client, token, after) = {
        let Some(strong) = daemon.upgrade() else {
            return Step::Stop;
        };
        if strong.closing() {
            return Step::Stop;
        }
        let Some(client) = strong.tracking_client() else {
            return Step::Idle;
        };
        let (tracked, after) = {
            let store = strong.store();
            (
                store.has_tracking().unwrap_or(false),
                store.last_tracking_seq().unwrap_or(0),
            )
        };
        if !tracked {
            return Step::Idle;
        }
        let Some(token) = strong.tracking_token().await else {
            return Step::Idle;
        };
        (client, token, after)
    };
    let mut stream = match client.events(&token, after).await {
        Ok(stream) => stream,
        Err(error) => {
            tracing::info!(%error, "tracking events unavailable; trying later");
            return Step::Failed;
        }
    };
    tracing::debug!(after, "following tracking events");
    loop {
        match stream.next(IDLE).await {
            Ok(Some(event)) => {
                let Some(strong) = daemon.upgrade() else {
                    return Step::Stop;
                };
                keep(&strong, event).await;
            }
            Ok(None) => return Step::Reconnect,
            Err(error) => {
                tracing::info!(%error, "tracking event stream broke");
                return Step::Failed;
            }
        }
    }
}

/// Stores one event and announces it.
async fn keep(daemon: &Daemon, event: ServerEvent) {
    let stored = TrackingEvent {
        seq: event.seq,
        tracking_id: event.id,
        kind: event.kind,
        link: event.link,
        source: event.source,
        at: event.at,
    };
    let news = match daemon.store().add_tracking_event(&stored) {
        Ok(news) => news,
        Err(err) => {
            tracing::warn!(%err, "could not keep a tracking event");
            return;
        }
    };
    let Some(news) = news else { return };
    let _ = daemon.notices().try_send(Notice::TrackingChanged);
    if news.first
        && stored.source == "person"
        && let Some(notices) = daemon.new_mail_notices()
    {
        let (summary, body) = text(&stored.kind, &news);
        let account = news.message.account;
        let sent = daemon
            .store()
            .filed_message(account, &news.message.message_id)
            .ok()
            .flatten();
        notices.tracking(&summary, &body, account, sent).await;
    }
}

/// A notification's title and text: "Bea opened Proposal v2", or "Bea
/// clicked a link in Proposal v2" with the link.
fn text(kind: &str, news: &TrackingNews) -> (String, String) {
    use katna_i18n::tr;
    let who = news
        .recipient
        .name
        .clone()
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| news.recipient.email.clone());
    let subject = if news.message.subject.trim().is_empty() {
        tr!("notify-no-subject")
    } else {
        news.message.subject.clone()
    };
    if kind == "click" {
        (
            tr!("notify-tracking-clicked", who = who, subject = subject),
            news.link.clone().unwrap_or_default(),
        )
    } else {
        (
            tr!("notify-tracking-opened", who = who, subject = subject),
            String::new(),
        )
    }
}
