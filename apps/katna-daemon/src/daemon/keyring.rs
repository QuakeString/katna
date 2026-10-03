// SPDX-License-Identifier: GPL-3.0-or-later

//! Accounts waiting for the keyring. At login the Secret Service may still
//! be locked when the daemon starts, and its unlock prompt may be dismissed
//! (`Secrets::declined`). That is not a wrong password: those accounts wait,
//! offline, and start on their own once the user unlocks the keyring, with
//! no restart.

use std::sync::{Arc, Weak};
use std::time::Duration;

use katna_core::AccountId;
use katna_dbus::state;

use super::{Daemon, Status};

/// How often a locked keyring is looked at, without asking the user.
const LOOK_EVERY: Duration = Duration::from_secs(5);

/// What an account that waits for the keyring shows.
pub(super) const WAITING: &str =
    "waiting for the keyring (KWallet or GNOME Keyring) to be unlocked";

impl Daemon {
    /// `account` could not start because the keyring is locked: it waits.
    pub(super) fn wait_for_keyring(self: &Arc<Self>, account: AccountId) {
        self.set_status(account, Status::new(state::OFFLINE, WAITING));
        self.keyring_waiting.lock().unwrap().insert(account);
        let _ = self.keyring_wake.0.try_send(());
    }

    /// Starts the waiting accounts, and has the contacts, calendar and task
    /// sync go again, once the keyring is unlocked.
    async fn keyring_unlocked(self: &Arc<Self>) {
        let waiting = std::mem::take(&mut *self.keyring_waiting.lock().unwrap());
        tracing::info!(accounts = waiting.len(), "the keyring is unlocked");
        let accounts = match self.store().accounts() {
            Ok(accounts) => accounts,
            Err(err) => {
                tracing::warn!(%err, "listing accounts");
                return;
            }
        };
        for account in accounts.iter().filter(|a| waiting.contains(&a.id)) {
            self.start_account(account).await;
        }
        self.wake_calendars();
        self.wake_contacts();
        self.wake_task_sync();
    }
}

/// Looks at the keyring while accounts wait for it, until the daemon goes.
pub(super) async fn run(daemon: Weak<Daemon>, wake: async_channel::Receiver<()>) {
    loop {
        let waiting = match daemon.upgrade() {
            Some(daemon) => !daemon.keyring_waiting.lock().unwrap().is_empty(),
            None => return,
        };
        if !waiting {
            if wake.recv().await.is_err() {
                return;
            }
            continue;
        }
        smol::Timer::after(LOOK_EVERY).await;
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.secrets.unlocked_since().await {
            daemon.keyring_unlocked().await;
        }
    }
}
