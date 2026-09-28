// SPDX-License-Identifier: GPL-3.0-or-later

//! The event stream to one install: stored events after the one it last
//! saw, then new ones as they happen.

use std::collections::VecDeque;
use std::convert::Infallible;
use std::sync::Arc;

use axum::response::sse;
use futures_lite::Stream;
use tokio::sync::broadcast::{self, error::RecvError};

use crate::db::{Db, Event};

/// Stored events are read in pages of this many.
const PAGE: i64 = 500;

struct State {
    db: Db,
    install: String,
    last: i64,
    live: broadcast::Receiver<Arc<Event>>,
    ready: VecDeque<Event>,
    catch_up: bool,
}

/// `install`'s events after number `after`. `live` must be subscribed
/// before this is called.
pub fn events(
    db: Db,
    install: String,
    after: i64,
    live: broadcast::Receiver<Arc<Event>>,
) -> impl Stream<Item = Result<sse::Event, Infallible>> {
    let state = State {
        db,
        install,
        last: after,
        live,
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
                    .db
                    .events_after(&state.install, state.last, PAGE)
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
            match state.live.recv().await {
                Ok(event) if event.install == state.install && event.seq > state.last => {
                    state.ready.push_back(Event::clone(&event));
                }
                Ok(_) => {}
                Err(RecvError::Lagged(_)) => state.catch_up = true,
                Err(RecvError::Closed) => return None,
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
