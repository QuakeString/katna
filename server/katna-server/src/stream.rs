// SPDX-License-Identifier: GPL-3.0-or-later

//! The event stream to one install: stored events after the one it last
//! saw, then new ones as they happen.
//!
//! A stream ends when its install is signed out, deleted or no longer on
//! an account with a confirmed address: at once when that happens through
//! this server (signing out, a device signed out from another, a password
//! change or reset, deleting the account or the install), and otherwise at
//! the next check, every [`RECHECK`].

use std::collections::VecDeque;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::response::sse;
use tokio::sync::broadcast::{self, error::RecvError};
use tokio::time::{Interval, MissedTickBehavior};

use crate::db::{Db, Event};
use crate::routes::StreamSlot;

/// Stored events are read in pages of this many.
const PAGE: i64 = 500;

/// How often an open stream checks that its install is still signed in.
pub const RECHECK: Duration = Duration::from_secs(60);

/// Whose stream it is.
pub(crate) struct Stream {
    /// The database.
    pub db: Db,
    /// The install the events are for.
    pub install: String,
    /// The account it was signed in to when the stream opened.
    pub account: String,
    /// Its place among the install's open streams, given back when the
    /// stream is dropped.
    pub _slot: StreamSlot,
}

struct State {
    stream: Stream,
    last: i64,
    live: broadcast::Receiver<Arc<Event>>,
    /// `None` once the server stops sending these.
    signed_out: Option<broadcast::Receiver<Arc<str>>>,
    recheck: Interval,
    ready: VecDeque<Event>,
    catch_up: bool,
}

/// What woke a waiting stream.
enum Woken {
    Live(Result<Arc<Event>, RecvError>),
    SignedOut(Result<Arc<str>, RecvError>),
    Recheck,
}

/// `stream`'s events after number `after`. `live` and `signed_out` must be
/// subscribed before this is called.
pub(crate) fn events(
    stream: Stream,
    after: i64,
    live: broadcast::Receiver<Arc<Event>>,
    signed_out: broadcast::Receiver<Arc<str>>,
) -> impl futures_lite::Stream<Item = Result<sse::Event, Infallible>> {
    let mut recheck = tokio::time::interval_at(tokio::time::Instant::now() + RECHECK, RECHECK);
    recheck.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let state = State {
        stream,
        last: after,
        live,
        signed_out: Some(signed_out),
        recheck,
        ready: VecDeque::new(),
        catch_up: true,
    };
    futures_lite::stream::unfold(Some(state), |state| async move {
        let mut state = state?;
        loop {
            if let Some(event) = state.ready.pop_front() {
                state.last = event.seq;
                return Some((Ok(to_sse(&event)), Some(state)));
            }
            if state.catch_up {
                match state
                    .stream
                    .db
                    .events_after(&state.stream.install, state.last, PAGE)
                    .await
                {
                    Ok(stored) => {
                        state.catch_up = stored.len() as i64 == PAGE;
                        state.ready.extend(stored);
                        continue;
                    }
                    Err(error) => {
                        // The daemon reconnects and resumes where it was.
                        tracing::warn!(%error, "event stream ended");
                        return None;
                    }
                }
            }
            let woken = {
                let signed_out = async {
                    match state.signed_out.as_mut() {
                        Some(receiver) => receiver.recv().await,
                        None => std::future::pending().await,
                    }
                };
                tokio::select! {
                    received = state.live.recv() => Woken::Live(received),
                    received = signed_out => Woken::SignedOut(received),
                    _ = state.recheck.tick() => Woken::Recheck,
                }
            };
            let check = match woken {
                Woken::Live(Ok(event)) => {
                    if event.install == state.stream.install && event.seq > state.last {
                        state.ready.push_back(Event::clone(&event));
                    }
                    false
                }
                Woken::Live(Err(RecvError::Lagged(_))) => {
                    state.catch_up = true;
                    false
                }
                Woken::Live(Err(RecvError::Closed)) => return None,
                Woken::SignedOut(Ok(key)) => {
                    *key == *state.stream.install || *key == *state.stream.account
                }
                Woken::SignedOut(Err(RecvError::Lagged(_))) => true,
                Woken::SignedOut(Err(RecvError::Closed)) => {
                    state.signed_out = None;
                    false
                }
                Woken::Recheck => true,
            };
            if check {
                match state
                    .stream
                    .db
                    .still_signed_in(&state.stream.install, &state.stream.account)
                    .await
                {
                    Ok(true) => {}
                    Ok(false) => return None,
                    Err(error) => {
                        tracing::warn!(%error, "event stream ended");
                        return None;
                    }
                }
            }
        }
    })
}

fn to_sse(event: &Event) -> sse::Event {
    sse::Event::default()
        .id(event.seq.to_string())
        .event("track")
        .json_data(event)
        .unwrap_or_else(|_| sse::Event::default().comment("unencodable event"))
}
