// SPDX-License-Identifier: GPL-3.0-or-later

//! D-Bus API (`in.invenia.katna.Pim1`) shared by `katna-daemon` and its clients.
//! See `docs/ARCHITECTURE.md` §14.
//!
//! This crate holds the types on the wire and the client proxy
//! ([`PimProxy`]). The daemon serves the interface itself.
//!
//! Errors use the standard `org.freedesktop.DBus.Error.*` names:
//! `AuthFailed` for a refused password, `InvalidArgs` for bad input,
//! `UnknownObject` for an unknown account and `Failed` for anything else.

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

pub use zbus;

/// One server of a new account. An empty `host` means "none".
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct ServerSpec {
    pub host: String,
    pub port: u16,
    /// `tls`, `starttls` or `plain`.
    pub security: String,
    pub username: String,
    /// Accept self-signed certificates (local test servers only).
    pub accept_invalid_certs: bool,
}

/// A new IMAP account for `AddImapAccount`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct NewImapAccount {
    pub display_name: String,
    pub address: String,
    pub imap: ServerSpec,
    pub smtp: ServerSpec,
}

/// What an account's sync is doing, from `Accounts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct AccountStatus {
    pub id: i64,
    /// `imap`, `local`, …
    pub kind: String,
    pub display_name: String,
    pub address: String,
    /// See [`state`].
    pub state: String,
    /// The last error, or empty.
    pub detail: String,
    /// When the last sync finished (Unix seconds), or 0.
    pub last_sync: i64,
}

/// Values of [`AccountStatus::state`].
pub mod state {
    /// The daemon does not sync this kind of account (for example `local`).
    pub const NOT_SYNCED: &str = "not-synced";
    /// Connecting or reconnecting.
    pub const CONNECTING: &str = "connecting";
    /// Connected and in sync; waiting for changes.
    pub const ONLINE: &str = "online";
    /// The connection failed; the daemon retries on its own.
    pub const OFFLINE: &str = "offline";
    /// The server refused the password. `SetPassword` or `SyncNow` retries.
    pub const AUTH_FAILED: &str = "auth-failed";
}

macro_rules! pim_proxy {
    ($interface:tt, $bus_name:tt, $path:tt) => {
        /// Client side of `in.invenia.katna.Pim1`.
        #[zbus::proxy(interface = $interface, default_service = $bus_name, default_path = $path)]
        pub trait Pim {
            /// All accounts with their sync state.
            fn accounts(&self) -> zbus::Result<Vec<AccountStatus>>;

            /// Checks the login, saves the password in the Secret Service,
            /// adds the account and starts syncing it. Returns its ID.
            fn add_imap_account(&self, account: &NewImapAccount, password: &str)
            -> zbus::Result<i64>;

            /// Checks and saves a new password, then syncs.
            fn set_password(&self, account: i64, password: &str) -> zbus::Result<()>;

            /// Stops syncing an account and deletes it, its mail and its
            /// password. Returns whether it existed.
            fn remove_account(&self, account: i64) -> zbus::Result<bool>;

            /// Syncs every folder of `account` now (0: every account).
            fn sync_now(&self, account: i64) -> zbus::Result<()>;

            /// Downloads the full message `message` if it is not stored yet.
            /// Returns once it is in the store (`MailChanged` follows).
            fn fetch_body(&self, message: i64) -> zbus::Result<()>;

            /// Accounts were added or removed.
            #[zbus(signal)]
            fn accounts_changed(&self) -> zbus::Result<()>;

            /// An account's [`AccountStatus`] changed.
            #[zbus(signal)]
            fn sync_status_changed(&self, account: i64) -> zbus::Result<()>;

            /// Mail of `account` changed in the store; read the change journal.
            #[zbus(signal)]
            fn mail_changed(&self, account: i64) -> zbus::Result<()>;
        }
    };
}

katna_core::with_dbus_names!(pim_proxy);
