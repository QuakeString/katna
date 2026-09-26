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

/// A new POP3 account for `AddPop3Account`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct NewPop3Account {
    pub display_name: String,
    pub address: String,
    pub pop3: ServerSpec,
    pub smtp: ServerSpec,
    /// Leave downloaded mail on the server. When `false`, mail is removed
    /// from the server as soon as it is stored.
    pub leave_on_server: bool,
    /// With `leave_on_server`: remove mail from the server this many days
    /// after downloading it. 0 keeps it.
    pub keep_days: u32,
    /// With `leave_on_server`: remove mail from the server once it is
    /// deleted for good in Katna.
    pub delete_with_local: bool,
}

/// What an account's sync is doing, from `Accounts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct AccountStatus {
    pub id: i64,
    /// `imap`, `pop3`, `local`, …
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

/// A message waiting to be sent, or recently sent, from `Outbox`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct OutboxItem {
    pub id: i64,
    pub account: i64,
    /// The stored message, readable in the store until it is filed.
    pub message: i64,
    pub subject: String,
    /// When it goes out, or went out (Unix seconds).
    pub send_at: i64,
    /// See [`send_state`].
    pub state: String,
    /// Why the last try failed, or empty.
    pub detail: String,
}

/// Values of [`OutboxItem::state`].
pub mod send_state {
    /// Waiting for its time: the undo delay, or a retry.
    pub const QUEUED: &str = "queued";
    /// Being handed to the SMTP server; too late to undo.
    pub const SENDING: &str = "sending";
    pub const SENT: &str = "sent";
    /// The server refused it for good.
    pub const FAILED: &str = "failed";
    /// Undone with `UndoSend`.
    pub const CANCELLED: &str = "cancelled";
}

/// Message flag names for `SetFlags`.
pub mod flag {
    pub const SEEN: &str = "seen";
    pub const ANSWERED: &str = "answered";
    pub const FLAGGED: &str = "flagged";
    pub const DRAFT: &str = "draft";
    pub const FORWARDED: &str = "forwarded";
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

            /// Like `AddImapAccount`, for a POP3 account. Its mail is
            /// downloaded into local folders (`INBOX`, `Sent`, `Trash`).
            fn add_pop3_account(&self, account: &NewPop3Account, password: &str)
            -> zbus::Result<i64>;

            /// Finds the IMAP and SMTP servers of `address` (provider
            /// settings, Thunderbird's ISPDB, DNS, then guesses), for
            /// `AddImapAccount`. Returns them and where they came from:
            /// `built-in`, `provider`, `ispdb`, `dns-srv`, `mx` or `guess`.
            /// An SMTP server with an empty host was not found.
            fn discover_account(&self, address: &str) -> zbus::Result<(NewImapAccount, String)>;

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

            /// Adds and removes flags (names from [`flag`]) on messages.
            /// Like every change below, it shows in the store at once
            /// (`MailChanged` follows) and reaches the server when the
            /// account is online. A change the server refuses three times
            /// is undone.
            fn set_flags(&self, messages: &[i64], add: &[&str], remove: &[&str])
                -> zbus::Result<()>;

            /// Moves messages to `folder` of the same account.
            fn move_messages(&self, messages: &[i64], folder: i64) -> zbus::Result<()>;

            /// Moves messages to the trash; deletes those already there for
            /// good, as when the account has no trash.
            fn delete_messages(&self, messages: &[i64]) -> zbus::Result<()>;

            /// Moves messages to the account's archive folder.
            fn archive_messages(&self, messages: &[i64]) -> zbus::Result<()>;

            /// Queues `message` (RFC 5322, with `Bcc` if any) from `account`
            /// to be sent in `delay` seconds; `UndoSend` works until then.
            /// Adds `Date` and `Message-ID` when missing. Once sent it is
            /// filed in the Sent folder. Returns the outbox ID.
            fn queue_send(&self, account: i64, message: &[u8], delay: u32) -> zbus::Result<i64>;

            /// Takes a queued message back. Returns `false` when it is
            /// already being sent.
            fn undo_send(&self, id: i64) -> zbus::Result<bool>;

            /// Forgets a cancelled or failed message. Returns whether it
            /// was one.
            fn discard_send(&self, id: i64) -> zbus::Result<bool>;

            /// Messages waiting to be sent, failed or cancelled.
            fn outbox(&self) -> zbus::Result<Vec<OutboxItem>>;

            /// Reads the settings file again; call after saving settings
            /// the daemon uses (`sync.metered`).
            fn reload_config(&self) -> zbus::Result<()>;

            /// Whether the daemon saves data as on a metered network (no
            /// bodies downloaded ahead of time).
            fn metered(&self) -> zbus::Result<bool>;

            /// Accounts were added or removed.
            #[zbus(signal)]
            fn accounts_changed(&self) -> zbus::Result<()>;

            /// An account's [`AccountStatus`] changed.
            #[zbus(signal)]
            fn sync_status_changed(&self, account: i64) -> zbus::Result<()>;

            /// Mail of `account` changed in the store; read the change journal.
            #[zbus(signal)]
            fn mail_changed(&self, account: i64) -> zbus::Result<()>;

            /// Outbox entry `id` changed state; see `Outbox`.
            #[zbus(signal)]
            fn outbox_changed(&self, id: i64) -> zbus::Result<()>;

            /// `Metered` changed.
            #[zbus(signal)]
            fn metered_changed(&self, metered: bool) -> zbus::Result<()>;
        }
    };
}

katna_core::with_dbus_names!(pim_proxy);
