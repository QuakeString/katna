// SPDX-License-Identifier: GPL-3.0-or-later

//! Connections for messages the user opens before they are downloaded.
//! Each account gets one, apart from its worker's, so that an opened
//! message never waits for a sync (which can take minutes on a large
//! account). It is closed after a while without use.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use katna_core::AccountId;
use katna_sync::connection::Connection;

/// How long an unused connection stays open.
const IDLE: Duration = Duration::from_secs(120);

#[derive(Default)]
pub(crate) struct OnDemand {
    sessions: Mutex<HashMap<AccountId, Arc<Session>>>,
}

pub(crate) struct Session {
    /// Held for a whole request: selecting a folder and fetching from it
    /// must not interleave with another request.
    pub(crate) connection: smol::lock::Mutex<Option<Connection>>,
    last_used: Mutex<Instant>,
}

impl OnDemand {
    pub(crate) fn session(&self, account: AccountId) -> Arc<Session> {
        self.sessions
            .lock()
            .unwrap()
            .entry(account)
            .or_insert_with(|| {
                Arc::new(Session {
                    connection: smol::lock::Mutex::new(None),
                    last_used: Mutex::new(Instant::now()),
                })
            })
            .clone()
    }

    /// Notes a use of `session`, and closes its connection once it has
    /// not been used for [`IDLE`].
    pub(crate) fn used(&self, account: AccountId, session: &Arc<Session>) {
        *session.last_used.lock().unwrap() = Instant::now();
        let session = session.clone();
        smol::spawn(async move {
            smol::Timer::after(IDLE).await;
            if session.last_used.lock().unwrap().elapsed() < IDLE {
                return;
            }
            // Dropping the last handle logs out.
            if let Some(mut connection) = session.connection.try_lock()
                && connection.take().is_some()
            {
                tracing::debug!(%account, "closed the connection for opened messages");
            }
        })
        .detach();
    }
}
