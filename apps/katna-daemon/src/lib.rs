// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna background service. See `docs/ARCHITECTURE.md` §9.
//!
//! The daemon is the only process that writes the store and talks to mail
//! servers. It runs one sync worker per account and serves
//! `in.invenia.katna.Pim1` on the session bus; owning the bus name keeps it
//! to a single instance.

pub mod daemon;
pub mod install;
pub mod secrets;
pub mod service;

use std::sync::Arc;

use katna_core::{Paths, ids};
use katna_sync::worker::WorkerConfig;
use zbus::fdo::{RequestNameFlags, RequestNameReply};

use crate::{daemon::Daemon, secrets::Secrets, service::PimService};

/// Why the daemon could not start.
#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error("another katna-daemon is already running")]
    AlreadyRunning,
    #[error("store: {0}")]
    Store(#[from] katna_store::Error),
    #[error("D-Bus: {0}")]
    DBus(#[from] zbus::Error),
    #[error("{0}")]
    Start(#[from] daemon::CommandError),
}

/// A daemon serving on a bus connection.
pub struct Instance {
    pub daemon: Arc<Daemon>,
    connection: zbus::Connection,
}

impl Instance {
    /// Serves the API on `connection`, takes the bus name and starts every
    /// account's worker.
    pub async fn start(
        paths: Paths,
        secrets: Secrets,
        config: WorkerConfig,
        connection: zbus::Connection,
    ) -> Result<Self, StartError> {
        let (daemon, notices) = Daemon::new(paths, secrets, config)?;
        connection
            .object_server()
            .at(ids::PIM_OBJECT_PATH, PimService::new(daemon.clone()))
            .await?;
        // Taken after the object is there, so activated calls find it.
        let reply = connection
            .request_name_with_flags(ids::DAEMON_BUS_NAME, RequestNameFlags::DoNotQueue.into())
            .await;
        match reply {
            Ok(RequestNameReply::PrimaryOwner | RequestNameReply::AlreadyOwner) => {}
            Ok(RequestNameReply::Exists | RequestNameReply::InQueue)
            | Err(zbus::Error::NameTaken) => return Err(StartError::AlreadyRunning),
            Err(err) => return Err(err.into()),
        }
        smol::spawn(service::emit_signals(connection.clone(), notices)).detach();
        daemon.start().await?;
        Ok(Self { daemon, connection })
    }

    /// Releases the bus name and stops every worker.
    pub async fn shutdown(self) {
        if let Err(err) = self.connection.release_name(ids::DAEMON_BUS_NAME).await {
            tracing::debug!(%err, "releasing the bus name");
        }
        self.daemon.shutdown().await;
    }
}
