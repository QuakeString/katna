// SPDX-License-Identifier: GPL-3.0-or-later

//! Which daemon a program talks to, when a package update replaced Katna
//! while it ran (`docs/ARCHITECTURE.md` §21.2, Running while updated).

use std::collections::HashMap;

use crate::{API_LEVEL, PimProxy};

/// What the daemon's `Version()` answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonVersion {
    /// Its build, as [`katna_core::update::VERSION`] writes it.
    pub version: String,
    /// The `Pim1` level it speaks ([`API_LEVEL`]).
    pub api: u32,
    /// Each database's schema version, by [`crate::schema`] name.
    pub schemas: HashMap<String, u32>,
}

impl DaemonVersion {
    /// Whether the daemon is a later build than this program, which then
    /// runs the build from before an update: it should restart.
    pub fn newer_than_this(&self) -> bool {
        self.newer_than(katna_core::update::VERSION)
    }

    /// Whether the daemon is a later build than `version`, or speaks a
    /// later `Pim1`.
    fn newer_than(&self, version: &str) -> bool {
        self.api > API_LEVEL || katna_core::update::newer(version, &self.version)
    }
}

/// Asks the daemon on `connection` which build it is. Asking also has it
/// check whether an update replaced it, and restart once idle if so.
/// `Ok(None)` from a daemon older than `Version()`, which restarts by
/// itself within half a minute of an update.
pub async fn daemon_version(connection: &zbus::Connection) -> zbus::Result<Option<DaemonVersion>> {
    let pim = PimProxy::new(connection).await?;
    match pim.version().await {
        Ok((version, api, schemas)) => Ok(Some(DaemonVersion {
            version,
            api,
            schemas,
        })),
        Err(zbus::Error::MethodError(name, _, _))
            if name.as_str() == "org.freedesktop.DBus.Error.UnknownMethod" =>
        {
            Ok(None)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn daemon(version: &str, api: u32) -> DaemonVersion {
        DaemonVersion {
            version: version.to_owned(),
            api,
            schemas: HashMap::new(),
        }
    }

    #[test]
    fn a_later_daemon_or_level_means_restart() {
        let this = "0.0.0.r765.g3928dee";
        assert!(daemon("0.0.0.r766.gabc", API_LEVEL).newer_than(this));
        assert!(daemon(this, API_LEVEL + 1).newer_than(this));
        assert!(!daemon(this, API_LEVEL).newer_than(this));
        assert!(!daemon("0.0.0.r764.gabc", API_LEVEL).newer_than(this));
        // A build from source is never told to restart by the version.
        assert!(!daemon("0.0.0.r766.gabc", API_LEVEL).newer_than("dev"));
    }
}
