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

pub mod agenda;
mod session;
pub use session::session;
mod start;
pub use start::ensure_daemon;

/// One server of a new account. An empty `host` means "none".
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct ServerSpec {
    pub host: String,
    pub port: u16,
    /// `tls`, `starttls` or `plain`.
    pub security: String,
    pub username: String,
    /// Accept self-signed certificates: for test servers on this computer
    /// or the local network only; connecting to a public address with it
    /// fails.
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
    /// The provider it signs in to with OAuth2 (`google`, `microsoft`),
    /// or empty for a password. With [`state::AUTH_FAILED`], `SignIn`
    /// signs it in again.
    pub sign_in: String,
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
    /// The server refused the password. `SetPassword` or `SyncNow` retries;
    /// for an account that signs in with OAuth2, `SignIn`.
    pub const AUTH_FAILED: &str = "auth-failed";
}

/// A mail template for `SaveTemplate`; `id` 0 saves a new one.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct TemplateItem {
    pub id: i64,
    pub name: String,
    pub subject: String,
    /// The formatted body, pictures inside as `data:` URIs.
    pub html: String,
    /// The same body as plain text.
    pub text: String,
    pub attachments: Vec<TemplateFileItem>,
}

/// A file that goes with a template.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct TemplateFileItem {
    pub name: String,
    pub mime: String,
    pub data: Vec<u8>,
}

/// A note for `SaveNote`; `id` 0 saves a new one.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct NoteItem {
    pub id: i64,
    /// The mail account whose Notes folder keeps it; 0 for this computer
    /// only.
    pub account: i64,
    pub title: String,
    /// Plain text; lines starting "☐ " or "☑ " are checklist items.
    pub body: String,
    /// 0 for none, else a number in the Notes palette.
    pub color: i64,
    pub pinned: bool,
    pub archived: bool,
    pub labels: Vec<String>,
    /// The `Message-ID` of the mail the note is about, or empty.
    pub link: String,
    /// `body` formatted, as HTML with one paragraph per line; empty when
    /// it has no formatting.
    pub html: String,
}

/// A file going up to Google Drive or OneDrive for a message, from
/// `DriveUpload`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct DriveUpload {
    pub id: i64,
    pub account: i64,
    pub name: String,
    /// Bytes Drive has, and the file's size.
    pub sent: u64,
    pub size: u64,
    /// See [`drive_state`].
    pub state: String,
    /// Where recipients open it, once uploaded.
    pub link: String,
    /// Why it failed, or empty.
    pub error: String,
}

/// Values of [`DriveUpload::state`].
pub mod drive_state {
    pub const UPLOADING: &str = "uploading";
    pub const DONE: &str = "done";
    pub const FAILED: &str = "failed";
    /// The account's sign-in did not allow Drive (it was added before
    /// Katna asked, or the box was unticked): `SignIn` again, then upload
    /// again.
    pub const NEEDS_PERMISSION: &str = "needs-permission";
}

/// A file or folder in an account's cloud drive, from `CloudList`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct CloudEntry {
    pub id: String,
    pub name: String,
    /// The file's type; empty for a folder.
    pub mime: String,
    /// Bytes; 0 for folders and the drive's own documents.
    pub size: u64,
    /// When it last changed, as Unix seconds; 0 when the drive doesn't say.
    pub modified: i64,
    pub folder: bool,
    /// One of the drive's own documents (a Google Doc), which opens as a
    /// PDF and is attached as a link.
    pub native: bool,
    /// Where people open it in the browser.
    pub link: String,
    /// For `CloudThumbnail`; empty when the drive has no picture of it.
    pub thumbnail: String,
}

/// One page of a drive listing, from `CloudList`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct CloudListing {
    /// See [`cloud_state`].
    pub state: String,
    /// Why it failed, or empty.
    pub error: String,
    pub items: Vec<CloudEntry>,
    /// Asks `CloudList` for the next page; empty on the last.
    pub next: String,
}

/// What `CloudList` looks through: its `place` argument.
pub mod cloud_place {
    /// The folder whose id is the `what` argument; `root` is the top.
    pub const FOLDER: &str = "folder";
    /// What other people shared with the account.
    pub const SHARED: &str = "shared";
    /// The whole drive, for the words in `what`.
    pub const SEARCH: &str = "search";
}

/// Values of [`CloudListing::state`].
pub mod cloud_state {
    pub const OK: &str = "ok";
    /// The account's sign-in doesn't let Katna read the drive (it was
    /// signed in before Katna asked, or the box was unticked): `SignIn`
    /// again.
    pub const NEEDS_PERMISSION: &str = "needs-permission";
    /// Katna can't browse this account's drive (yet).
    pub const UNSUPPORTED: &str = "unsupported";
    /// The drive couldn't be reached; `error` says why.
    pub const FAILED: &str = "failed";
}

/// States of an account's calendar sync, from `CalendarStatus`.
pub mod calendar_state {
    /// Synced, or about to be.
    pub const OK: &str = "ok";
    /// The account's sign-in did not allow Katna into its calendars (it
    /// was signed in before Katna asked): `SignIn` again.
    pub const NEEDS_SIGN_IN: &str = "needs-sign-in";
    /// The provider has its calendar API switched off for Katna (Google
    /// Calendar API not enabled in Katna's Google Cloud project). The
    /// detail names the API and the page that turns it on
    /// (`katna_core::api_off`), or says why in words.
    pub const NOT_ENABLED: &str = "not-enabled";
    /// The last sync failed; the detail says why. It is tried again.
    pub const ERROR: &str = "error";
    /// The account has no calendars Katna can reach; the detail says
    /// what was asked and what it answered (may be empty).
    pub const NONE: &str = "none";
    /// A mail account signed in with a password at a provider that lets
    /// Katna in only through its own sign-in (Google, Microsoft): `SignIn`
    /// with that provider. The detail is the provider
    /// (`OAuthProvider::as_str`).
    pub const USE_SIGN_IN: &str = "use-sign-in";
}

/// Where an account's contacts sync stands, as `ContactsStatus` reports it.
pub mod contacts_state {
    /// Synced, or about to be.
    pub const OK: &str = "ok";
    /// The account's sign-in or password did not let Katna into its
    /// contacts: sign in again (OAuth2), or check the password.
    pub const NEEDS_SIGN_IN: &str = "needs-sign-in";
    /// The provider has its contacts API switched off for Katna (People
    /// API not enabled in Katna's Google Cloud project). The detail names
    /// the API and the page that turns it on (`katna_core::api_off`), or
    /// says why in words.
    pub const NOT_ENABLED: &str = "not-enabled";
    /// The last sync failed; the detail says why. It is tried again.
    pub const ERROR: &str = "error";
    /// A mail account signed in with a password at a provider that lets
    /// Katna into contacts only through its own sign-in (Google,
    /// Microsoft). The detail is the provider (`OAuthProvider::as_str`).
    pub const USE_SIGN_IN: &str = "use-sign-in";
    /// The account has no address book Katna can reach; the detail says
    /// what the server answered (may be empty).
    pub const NONE: &str = "none";
}

/// States of an account's task sync, from `TasksStatus`: the same as a
/// calendar's (a Google sign-in without tasks, a refused password, the
/// Google Tasks API switched off, a failed sync, no task service).
pub use calendar_state as task_state;

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
    /// Scheduled mail the SMTP server holds until `send_at`; it cannot be
    /// taken back.
    pub const HELD: &str = "held";
    /// The server refused it for good.
    pub const FAILED: &str = "failed";
    /// Undone with `UndoSend`.
    pub const CANCELLED: &str = "cancelled";
}

/// The Katna account this computer is signed in to, from `KatnaAccount`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct KatnaAccount {
    /// Signed in; the fields below are empty otherwise.
    pub signed_in: bool,
    pub email: String,
    /// The address is confirmed with the mailed code. Server features
    /// work only then.
    pub verified: bool,
}

/// A computer signed in to the Katna account, from `KatnaDevices`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct KatnaDevice {
    pub id: String,
    /// Its host name.
    pub name: String,
    /// When it signed in and when it was last seen (Unix seconds).
    pub signed_in_at: i64,
    pub last_seen: i64,
    /// It is this computer.
    pub this: bool,
}

/// Why a Katna account command failed: the message of its D-Bus error,
/// so the app can say it in the user's language.
pub mod katna_error {
    /// Wrong address or password (`AuthFailed`).
    pub const WRONG_PASSWORD: &str = "wrong_password";
    /// The address already has a Katna account.
    pub const EXISTS: &str = "exists";
    pub const BAD_EMAIL: &str = "bad_email";
    /// Fewer than 8 characters.
    pub const SHORT_PASSWORD: &str = "short_password";
    pub const LONG_PASSWORD: &str = "long_password";
    pub const WRONG_CODE: &str = "wrong_code";
    /// The code expired or had too many wrong tries; ask for a new one.
    pub const CODE_EXPIRED: &str = "code_expired";
    /// Too many tries; wait a while.
    pub const TOO_MANY: &str = "too_many";
    /// The server could not mail the code.
    pub const MAIL_FAILED: &str = "mail_failed";
    /// Not signed in (any more).
    pub const SIGN_IN: &str = "sign_in";
    /// Katna Server could not be reached.
    pub const OFFLINE: &str = "offline";
    /// Anything else.
    pub const SERVER: &str = "server";
}

/// Where an update of Katna stands, from `UpdateStatus`
/// (`docs/ARCHITECTURE.md` §21.2). The daemon checks and downloads; Katna
/// Mail installs the downloaded file and restarts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct UpdateStatus {
    /// See [`update_state`].
    pub state: String,
    /// The version on offer, once a check found one.
    pub version: String,
    /// Bytes downloaded so far, and of how many.
    pub done: u64,
    pub total: u64,
    /// With [`update_state::READY`]: the downloaded package, checked
    /// against the SHA-256 below.
    pub file: String,
    pub sha256: String,
    /// When the last check finished (Unix seconds), or 0.
    pub checked: i64,
    /// Why the last check or download failed, or empty.
    pub detail: String,
}

/// Values of [`UpdateStatus::state`].
pub mod update_state {
    /// This build does not update itself: built from source, or a package
    /// whose package manager updates it.
    pub const UNSUPPORTED: &str = "unsupported";
    /// Not checked yet.
    pub const IDLE: &str = "idle";
    pub const CHECKING: &str = "checking";
    /// The newest build is installed.
    pub const UP_TO_DATE: &str = "up-to-date";
    /// A newer version waits to be downloaded (`DownloadUpdate`).
    pub const AVAILABLE: &str = "available";
    pub const DOWNLOADING: &str = "downloading";
    /// Downloaded and checked: Katna Mail can install it.
    pub const READY: &str = "ready";
    /// The last check failed; `detail` says why.
    pub const FAILED: &str = "failed";
    /// The download of `version` failed, even after trying again;
    /// `detail` says why. `DownloadUpdate` tries once more.
    pub const DOWNLOAD_FAILED: &str = "download-failed";
}

/// Actions Katna Mail serves through `org.freedesktop.Application`
/// (`ActivateAction`) under its app ID, and the command-line flags that do
/// the same when it has to be started.
pub mod app_action {
    /// Show the Inbox.
    pub const OPEN_INBOX: &str = "open-inbox";
    /// Start a new message.
    pub const COMPOSE: &str = "compose";
    /// Open the settings.
    pub const PREFERENCES: &str = "preferences";
    /// Open one message; the parameter is its ID (`x`).
    pub const OPEN_MESSAGE: &str = "open-message";
    /// Open one message and start a reply to all; the parameter is its ID
    /// (`x`).
    pub const REPLY_ALL: &str = "reply-all";
    /// Put a query in the search box and search; the parameter is the
    /// query (`s`).
    pub const SEARCH: &str = "search";
    /// Close the app.
    pub const QUIT: &str = "quit";
    /// Show the downloaded update, ready to install (the Update button of
    /// the notification that an update is ready).
    pub const INSTALL_UPDATE: &str = "install-update";
    /// Show one page of the window: Mail, Calendar, Contacts, Tasks or
    /// Notes; the parameter is its name (`s`: `mail`, `calendar`,
    /// `contacts`, `tasks`, `notes`); `tasks:<id>` opens that task, and
    /// the Calendar takes a day too ([`calendar_page`]).
    pub const OPEN_PAGE: &str = "open-page";
    /// Start a new message with files attached ("Send with Katna Mail" in
    /// a file manager); the parameters are texts (`s`): the address to
    /// send from (empty for the usual one), then the files' full paths.
    /// Folders go as zips.
    pub const ATTACH: &str = "attach";

    /// The command-line flag that starts Katna Mail doing `action`, if it
    /// has one. The flags of [`takes_message`] actions are followed by the
    /// message ID, those of [`takes_text`] actions by the text.
    pub fn flag(action: &str) -> Option<&'static str> {
        match action {
            OPEN_INBOX => Some("--inbox"),
            COMPOSE => Some("--compose"),
            PREFERENCES => Some("--settings"),
            OPEN_MESSAGE => Some("--message"),
            INSTALL_UPDATE => Some("--update"),
            REPLY_ALL => Some("--reply-all"),
            SEARCH => Some("--search"),
            OPEN_PAGE => Some("--page"),
            ATTACH => Some("--attach"),
            _ => None,
        }
    }

    /// `open-page`'s parameter for the Calendar page on `day`
    /// (`YYYY-MM-DD`), with a new event on it started when `new`:
    /// `calendar:2026-10-01` or `calendar:2026-10-01:new`.
    pub fn calendar_page(day: &str, new: bool) -> String {
        if new {
            format!("calendar:{day}:{NEW_EVENT}")
        } else {
            format!("calendar:{day}")
        }
    }

    /// Takes `open-page`'s parameter apart: the page's name, what after it
    /// (the Calendar's day, a task's ID), and whether to start a new event.
    pub fn page_parts(page: &str) -> (&str, Option<&str>, bool) {
        let mut parts = page.splitn(3, ':');
        let name = parts.next().unwrap_or_default();
        let detail = parts.next().filter(|detail| !detail.is_empty());
        (name, detail, parts.next() == Some(NEW_EVENT))
    }

    const NEW_EVENT: &str = "new";

    #[cfg(test)]
    #[test]
    fn calendar_pages_round_trip() {
        assert_eq!(page_parts("tasks"), ("tasks", None, false));
        assert_eq!(page_parts("tasks:42"), ("tasks", Some("42"), false));
        let page = calendar_page("2026-10-01", false);
        assert_eq!(page_parts(&page), ("calendar", Some("2026-10-01"), false));
        let page = calendar_page("2026-10-01", true);
        assert_eq!(page_parts(&page), ("calendar", Some("2026-10-01"), true));
    }

    /// Whether `action`'s parameter is a message ID.
    pub fn takes_message(action: &str) -> bool {
        matches!(action, OPEN_MESSAGE | REPLY_ALL)
    }

    /// Whether `action`'s parameter is text.
    pub fn takes_text(action: &str) -> bool {
        matches!(action, SEARCH | OPEN_PAGE)
    }
}

/// Why `Translate` gave no translation: the third field of its answer,
/// empty when it did.
pub mod translate_problem {
    /// This build has no translation server.
    pub const OFF: &str = "off";
    /// The message is already in the language asked for; nothing was sent.
    pub const SAME_LANGUAGE: &str = "same-language";
    /// The server cannot translate from that language into this one.
    pub const UNSUPPORTED: &str = "unsupported";
    /// Over the server's limit for now.
    pub const TOO_MANY: &str = "too-many";
    /// This computer is not signed in to a Katna account with a confirmed
    /// address.
    pub const SIGN_IN: &str = "sign-in";
    /// The server could not be reached, or failed.
    pub const FAILED: &str = "failed";
}

/// Message flag names for `SetFlags`.
pub mod flag {
    pub const SEEN: &str = "seen";
    pub const ANSWERED: &str = "answered";
    pub const FLAGGED: &str = "flagged";
    pub const DRAFT: &str = "draft";
    pub const FORWARDED: &str = "forwarded";
    /// Marked important (`$Important`, or Gmail's Important label).
    pub const IMPORTANT: &str = "important";
    /// The conversation is muted at the mail service (`$muted`, or
    /// Gmail's Muted label). Use `Mute` to mute; this only reports it.
    pub const MUTED: &str = "muted";
}

/// What `Mute` and `Unmute` act on (`docs/ARCHITECTURE.md` §15.1.1).
pub mod mute {
    /// A whole account; `id` is the account.
    pub const ACCOUNT: &str = "account";
    /// A folder; `id` is the folder.
    pub const FOLDER: &str = "folder";
    /// A conversation; `id` is one of its messages.
    pub const CONVERSATION: &str = "conversation";
    /// Mail from `address`, in every account.
    pub const SENDER: &str = "sender";
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

            /// Finds the IMAP, POP3 and SMTP servers of `address`
            /// (provider settings, Thunderbird's ISPDB, DNS, then guesses),
            /// for `AddImapAccount` or `AddPop3Account`. Returns the IMAP
            /// and SMTP servers, then the POP3 server, and where they came
            /// from: `built-in`, `provider`, `ispdb`, `dns-srv`, `mx` or
            /// `guess`. A server with an empty host was not found; there is
            /// always an IMAP or a POP3 server. Then the provider to sign in
            /// to with `SignIn` (`google`, `microsoft`, or empty), and
            /// whether a password works too (`false`: only `SignIn`).
            fn discover_account(
                &self,
                address: &str,
            ) -> zbus::Result<(NewImapAccount, ServerSpec, String, String, bool)>;

            /// Signs in to `provider` (`google` or `microsoft`) with OAuth2
            /// in the default browser, then adds the account that signed in,
            /// or signs `account` (0: none; else the account with that
            /// address, if any) in again. `address` fills in the provider's
            /// page (may be empty). Returns once the browser comes back (at
            /// most ten minutes), with the account's ID. `AuthFailed` when
            /// the user did not allow access. `zoho` needs `account`: it
            /// links Zoho's tasks and calendars to it, and its mail keeps
            /// its password.
            fn sign_in(&self, provider: &str, account: i64, address: &str) -> zbus::Result<i64>;

            /// Ends a `SignIn` still waiting for the browser. Returns
            /// whether one was.
            fn cancel_sign_in(&self) -> zbus::Result<bool>;

            /// Checks and saves a new password, then syncs.
            fn set_password(&self, account: i64, password: &str) -> zbus::Result<()>;

            /// Renames an account; an empty name goes back to the name its
            /// own mail is sent under, else its address.
            fn rename_account(&self, account: i64, name: &str) -> zbus::Result<()>;

            /// What a POP3 account does with mail on the server once it
            /// is downloaded, as in `NewPop3Account`: leave it there,
            /// then remove it after `keep_days` days (0: never) or once
            /// it is deleted for good in Katna.
            fn set_pop3_keep(
                &self,
                account: i64,
                leave_on_server: bool,
                keep_days: u32,
                delete_with_local: bool,
            ) -> zbus::Result<()>;

            /// Stops syncing an account and deletes it, its mail and its
            /// password. Returns whether it existed.
            fn remove_account(&self, account: i64) -> zbus::Result<bool>;

            /// Deletes everything Katna keeps on this computer: every
            /// account with its mail and password, contacts, calendars,
            /// the search index, the cache and the settings file. Mail
            /// servers are not touched. The daemon exits once it answers;
            /// the next call starts a new one with nothing stored.
            fn delete_all_data(&self) -> zbus::Result<()>;

            /// Deletes what was downloaded and can be downloaded again: the
            /// bodies and attachments of mail still on its IMAP server, the
            /// search index (rebuilt at once), sender pictures and
            /// translations, then syncs. Accounts, settings, flags, labels,
            /// pins and mail that exists only on this computer stay;
            /// servers are not touched.
            /// Returns how many messages lost their body and how many bytes
            /// of mail were deleted.
            fn reset_cache(&self) -> zbus::Result<(u64, u64)>;

            /// Syncs every folder of `account` now (0: every account).
            fn sync_now(&self, account: i64) -> zbus::Result<()>;

            /// Syncs only folder `folder` now (with its account's inbox).
            /// `SyncStatusChanged` follows for its account when done.
            fn sync_folder(&self, folder: i64) -> zbus::Result<()>;

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

            /// Pins messages (and so their conversations) to the top of the
            /// list, or unpins them. Pins stay on this computer. Pinning more
            /// than ten conversations fails with a message saying so.
            fn set_pinned(&self, messages: &[i64], on: bool) -> zbus::Result<()>;

            /// Mutes what `kind` ([`mute`]) and `id` or `address` name until
            /// `until` (Unix seconds; 0 until unmuted): its new mail does not
            /// notify and is not counted on the taskbar and tray, though it
            /// still arrives unread. A conversation is muted at the mail
            /// service too where it can be (Gmail's mute, `$muted`).
            /// `MailChanged` follows for every account.
            fn mute(&self, kind: &str, id: i64, address: &str, until: i64) -> zbus::Result<()>;

            /// Undoes [`Self::mute`]; nothing happens when it was not muted.
            fn unmute(&self, kind: &str, id: i64, address: &str) -> zbus::Result<()>;

            /// Pins something of `message` to the top of its conversation's
            /// chat: `kind` "mail" (the whole mail), "file" (its attachment
            /// number `file`) or "text" (`text`, picked from it). `label` is
            /// what the pin bar shows; `replace` a pin taken off first, or
            /// 0. Pins stay on this computer. Returns the pin's ID, or 0
            /// when the conversation holds five pins or the same thing is
            /// pinned already. `MailChanged` follows.
            fn pin_in_chat(
                &self,
                message: i64,
                kind: &str,
                file: i64,
                text: &str,
                label: &str,
                replace: i64,
            ) -> zbus::Result<i64>;

            /// Takes off a pin [`Self::pin_in_chat`] made.
            fn unpin_in_chat(&self, id: i64) -> zbus::Result<()>;

            /// Puts a conversation's pins in this order.
            fn order_chat_pins(&self, ids: &[i64]) -> zbus::Result<()>;

            /// Sets whether new mail in `folder` notifies and whether its
            /// unread mail counts on the taskbar and tray. `category` is an
            /// inbox tab (`MailCategory` storage number), or 0 for the whole
            /// folder. `MailChanged` follows for every account.
            fn set_bell(&self, folder: i64, category: i64, notify: bool, count: bool)
                -> zbus::Result<()>;

            /// Creates a folder (a label, on Gmail) called `name` on the
            /// account's server, inside folder `parent` (0: at the top), and
            /// returns its ID; `MailChanged` follows. Fails while the
            /// server cannot be reached, and when the name is taken.
            fn create_folder(&self, account: i64, name: &str, parent: i64) -> zbus::Result<i64>;

            /// Moves messages to `folder` of the same account.
            fn move_messages(&self, messages: &[i64], folder: i64) -> zbus::Result<()>;

            /// Moves messages to the trash; deletes those already there for
            /// good, as when the account has no trash.
            fn delete_messages(&self, messages: &[i64]) -> zbus::Result<()>;

            /// Moves messages to the account's archive folder.
            fn archive_messages(&self, messages: &[i64]) -> zbus::Result<()>;

            /// Snoozes messages until `until` (Unix seconds, at least a
            /// minute ahead): they move to their account's `Snoozed` folder
            /// (made on the server the first time, so that needs it once)
            /// and come back where they were, unread, at that time, with a
            /// notification. Messages already snoozed get the new time;
            /// messages only in Sent, Drafts, Trash, Spam or All Mail stay.
            /// The times are in `pim.db`'s `meta` table (`katna-meta`).
            fn snooze(&self, messages: &[i64], until: i64) -> zbus::Result<()>;

            /// Brings snoozed messages back where they were now, as they
            /// are; others are left alone.
            fn unsnooze(&self, messages: &[i64]) -> zbus::Result<()>;

            /// Reminds the user `after` seconds after outbox entry `id` is
            /// sent if nobody replied by then: the message then shows in
            /// the Inbox too, unread and on top, with a notification. 0
            /// takes the reminder back; `UndoSend` does too.
            fn set_follow_up(&self, id: i64, after: i64) -> zbus::Result<()>;

            /// Queues `message` (RFC 5322, with `Bcc` if any) from `account`
            /// to be sent in `delay` seconds; `UndoSend` works until then.
            /// Adds `Date` and `Message-ID` when missing. Once sent it is
            /// filed in the Sent folder. Returns the outbox ID.
            fn queue_send(&self, account: i64, message: &[u8], delay: u32) -> zbus::Result<i64>;

            /// Like `QueueSend`, for mail scheduled to go out at `at` (Unix
            /// seconds): after `delay` seconds (undo send) it goes to an
            /// SMTP server that holds mail until then (`ServerHoldLimit`),
            /// or else it is sent at `at` while the daemon runs.
            fn schedule_send(
                &self,
                account: i64,
                message: &[u8],
                delay: u32,
                at: i64,
            ) -> zbus::Result<i64>;

            /// How long, in seconds, the SMTP server of `account` holds
            /// mail to send later (RFC 4865 `FUTURERELEASE`); 0 when it
            /// cannot. Logs in to ask the first time.
            fn server_hold_limit(&self, account: i64) -> zbus::Result<u64>;

            /// Whether the SMTP server of `account` mails the sender a
            /// delivery receipt per recipient (RFC 3461 `DSN`). A message
            /// queued with the `X-Katna-Delivery-Receipt` header asks for
            /// them where the server offers them. Logs in to ask the first
            /// time.
            fn server_delivery_receipts(&self, account: i64) -> zbus::Result<bool>;

            /// Like `QueueSend`, with open and click tracking: each
            /// recipient gets their own tracked copy (`docs/ARCHITECTURE.md`
            /// §11, §16.1). Mail that cannot be tracked (no HTML version,
            /// signed or encrypted, over 50 recipients, tracking off, not
            /// signed in to a Katna account or the server unreachable) goes
            /// out untracked.
            fn queue_tracked_send(
                &self,
                account: i64,
                message: &[u8],
                delay: u32,
            ) -> zbus::Result<i64>;
            /// Takes a queued message back. Returns `false` when it is
            /// already being sent.
            fn undo_send(&self, id: i64) -> zbus::Result<bool>;

            /// Saves a contact card (`katna_core::contact::Card` as JSON)
            /// over saved card `contact`, or as a new card in address book
            /// `book` (0: this computer) when `contact` is 0, writing the
            /// account's service first. Returns the card's id.
            fn save_contact(&self, contact: i64, book: i64, card: &str) -> zbus::Result<i64>;

            /// Deletes saved cards, from their accounts' services too.
            fn delete_contacts(&self, ids: &[i64]) -> zbus::Result<()>;

            /// Saves one of Google's other contacts in its account's
            /// contacts; returns the new card, or 0 when it comes with the
            /// next sync.
            fn save_other_contact(&self, id: i64) -> zbus::Result<i64>;

            /// Saves cards read from a file (a JSON list of `{card,
            /// labels}`) as new contacts in address book `book`; returns
            /// their ids.
            fn import_contacts(&self, book: i64, cards: &str) -> zbus::Result<Vec<i64>>;

            /// Gives a saved card exactly these labels, by name.
            fn set_contact_labels(&self, contact: i64, labels: &[String]) -> zbus::Result<()>;

            /// Renames a label in every address book; an empty new name
            /// takes the label away and keeps its people.
            fn rename_contact_label(&self, old: &str, new: &str) -> zbus::Result<()>;

            /// Forgets a cancelled or failed message. Returns whether it
            /// was one.
            fn discard_send(&self, id: i64) -> zbus::Result<bool>;

            /// Messages waiting to be sent, failed or cancelled.
            fn outbox(&self) -> zbus::Result<Vec<OutboxItem>>;

            /// Starts putting the file at `path` in the Google Drive or
            /// OneDrive of `account` (signed in with Google or Microsoft),
            /// for a message too large to carry it. Returns the upload's ID; `DriveChanged` tells
            /// how it goes.
            fn drive_upload(&self, account: i64, path: &str) -> zbus::Result<i64>;

            /// Where upload `id` stands.
            fn drive_upload_status(&self, id: i64) -> zbus::Result<DriveUpload>;

            /// Stops upload `id` and moves its file to the Drive's bin: it
            /// was taken off the message. Returns whether there was one.
            fn drive_cancel(&self, id: i64) -> zbus::Result<bool>;

            /// Lets `addresses` view the files of `uploads`, without
            /// Google's sharing mail. Returns the addresses Drive would not
            /// share with (no Google account, or not allowed).
            fn drive_share(&self, uploads: &[i64], addresses: &[&str])
            -> zbus::Result<Vec<String>>;

            /// Lets anyone with the link view the files of `uploads`.
            /// Returns the links to put in the message, in order (OneDrive
            /// gives such a link its own address).
            fn drive_share_with_link(&self, uploads: &[i64]) -> zbus::Result<Vec<String>>;

            /// Whether the sign-in of `account` lets Katna browse its whole
            /// drive in Files (Google Drive for now). Asks no server.
            fn cloud_readable(&self, account: i64) -> zbus::Result<bool>;

            /// One page of the drive of `account`: a folder, what was
            /// shared with it, or a search (`place` is one of
            /// `cloud_place`, `what` the folder id or the words). `page`
            /// is the last listing's `next`, or empty for the first.
            fn cloud_list(&self, account: i64, place: &str, what: &str, page: &str)
            -> zbus::Result<CloudListing>;

            /// Downloads `entry` from the drive of `account` into Katna's
            /// cache and returns the file's path. A drive's own document
            /// comes as a PDF, its path ending in `.pdf`.
            fn cloud_fetch(&self, account: i64, entry: &CloudEntry) -> zbus::Result<String>;

            /// The picture behind a `CloudEntry::thumbnail` of `account`,
            /// about `width` pixels wide, as the drive sends it (PNG or
            /// JPEG).
            fn cloud_thumbnail(&self, account: i64, link: &str, width: u32)
            -> zbus::Result<Vec<u8>>;

            /// Whether the sign-in of `account` lets Katna upload into any
            /// folder of its drive. Asks no server.
            fn cloud_writable(&self, account: i64) -> zbus::Result<bool>;

            /// Starts uploading file or folder `path` (with everything in
            /// it) into folder `folder` of the drive of `account`, empty
            /// for the top. Returns an upload id followed as
            /// `DriveUploadStatus` and `DriveChanged`; `DriveCancel` stops it.
            fn cloud_upload(&self, account: i64, folder: &str, path: &str) -> zbus::Result<i64>;

            /// Links file `entry` of the drive of `account` to a message as
            /// a finished upload: its id works with `DriveShare` and
            /// `DriveShareWithLink`, and `DriveCancel` forgets it without
            /// touching the file.
            fn cloud_link(&self, account: i64, entry: &CloudEntry) -> zbus::Result<i64>;

            /// A new video call link from the mail service of `account`
            /// (Google Meet for Gmail), or an empty string when it has no
            /// meetings Katna may make; Katna Mail then makes a Jitsi link.
            fn meeting_link(&self, account: i64) -> zbus::Result<String>;

            /// Shows or hides the events of calendar `id` (the store's
            /// `calendar.id`) everywhere: the Calendar page, the agenda and
            /// the desktop's clock. Sends `CalendarChanged`.
            fn set_calendar_hidden(&self, id: i64, hidden: bool) -> zbus::Result<()>;

            /// Adds a calendar named `name` in `color` (`#rrggbb`, or empty
            /// for the service's pick) to `account`, on its service, or to
            /// this computer (0). Returns its `calendar.id`. Fails with the
            /// service's reason when it refuses.
            fn add_calendar(&self, account: i64, name: &str, color: &str) -> zbus::Result<i64>;

            /// Renames calendar `id`, on its service first.
            fn rename_calendar(&self, id: i64, name: &str) -> zbus::Result<()>;

            /// Gives calendar `id` colour `color` (`#rrggbb`), on its service
            /// first. Returns the colour it got: an Outlook calendar takes
            /// the nearest of Outlook's.
            fn set_calendar_color(&self, id: i64, color: &str) -> zbus::Result<String>;

            /// Deletes calendar `id` with its events (`delete`), or takes one
            /// shared with the person off their list, on its service first.
            /// Only Google tells the two apart; other services delete one's
            /// own calendar and unsubscribe from someone else's either way.
            fn delete_calendar(&self, id: i64, delete: bool) -> zbus::Result<()>;

            /// Where each account's calendar sync stands: its ID, a
            /// [`calendar_state`] and a detail for people (may be empty).
            fn calendar_status(&self) -> zbus::Result<Vec<(i64, String, String)>>;

            /// Where each account's contacts sync stands: its ID, a
            /// [`contacts_state`] and a detail for people (may be empty).
            /// `ContactsChanged` is sent when one changes.
            fn contacts_status(&self) -> zbus::Result<Vec<(i64, String, String)>>;

            /// Where each account's task sync stands: its ID, a
            /// [`task_state`] and a detail for people (may be empty).
            /// Changes come with the agenda's `Changed`.
            fn tasks_status(&self) -> zbus::Result<Vec<(i64, String, String)>>;

            /// Adds, changes, deletes or restores events, or answers an
            /// invitation: `json` is a `katna_store::calendar::EventChange`
            /// (tagged by `op`). The daemon writes it to the store at once
            /// and sends `CalendarChanged`, then sends it to the calendar's
            /// service; if the service refuses, the calendar syncs again,
            /// which undoes the change here. Returns the ID of the event
            /// added or changed (for one occurrence of a series, its
            /// changed occurrence; for "this and following", the new
            /// series), or 0. `InvalidArgs` for a calendar that can't be
            /// changed, an event that doesn't exist or a bad edit.
            fn edit_event(&self, json: &str) -> zbus::Result<i64>;

            /// Saves a mail template on this computer, in place of the one
            /// with its ID (0: a new one). Its name must not be empty, and
            /// its attachments are at most 20 MB. Returns its ID. Apps read
            /// templates from the store.
            fn save_template(&self, template: &TemplateItem) -> zbus::Result<i64>;

            /// Renames a template. Returns whether it exists.
            fn rename_template(&self, id: i64, name: &str) -> zbus::Result<bool>;

            /// Deletes a template. Returns whether it existed.
            fn delete_template(&self, id: i64) -> zbus::Result<bool>;

            /// Saves a note in place of the one with its ID (0: a new one,
            /// on top). A note of a mail account goes to that account's
            /// Notes folder too. Returns its ID. Apps read notes from the
            /// store.
            fn save_note(&self, note: &NoteItem) -> zbus::Result<i64>;

            /// Moves notes to Trash (`trashed` true), or back. Returns how
            /// many changed.
            fn trash_notes(&self, ids: &[i64], trashed: bool) -> zbus::Result<u32>;

            /// Deletes notes for good, here and on their servers. Returns
            /// how many existed.
            fn delete_notes(&self, ids: &[i64]) -> zbus::Result<u32>;

            /// Puts notes `ids` in this order, the first on top, in the
            /// places they had among themselves. Returns how many moved.
            fn order_notes(&self, ids: &[i64]) -> zbus::Result<u32>;

            /// Takes label `old` off notes `ids` and puts `new` on those
            /// that had it; with `old` empty, puts `new` on all of them,
            /// and with `new` empty, only takes `old` off. Renames, deletes
            /// and adds labels. Returns how many notes changed.
            fn relabel_notes(&self, ids: &[i64], old: &str, new: &str) -> zbus::Result<u32>;

            /// Saves `message` (RFC 5322, with `Bcc` if any) as a draft of
            /// `account` in its Drafts folder, here and on the server,
            /// in place of the copies saved before with the same
            /// `Message-ID`. Adds `Date` and `Message-ID` when missing.
            /// Returns the saved message's ID.
            fn save_draft(&self, account: i64, message: &[u8]) -> zbus::Result<i64>;

            /// Deletes every saved copy of the draft whose `Message-ID` is
            /// `message_id` from the Drafts folder of `account`, here and
            /// on the server.
            fn discard_draft(&self, account: i64, message_id: &str) -> zbus::Result<()>;

            /// Downloads a remote image of a message the user chose to show
            /// (`https`; `http` is upgraded), at most 8 MB, from port 443 of
            /// a public address only (never this computer or its local
            /// network). Fails for anything that is not an image. Apps
            /// never use the network themselves.
            fn fetch_image(&self, url: &str) -> zbus::Result<Vec<u8>>;

            /// The picture of the organization that sends from `address`
            /// (its BIMI logo, or its website's icon), looked up by its
            /// organizational domain and cached for a week. Empty when
            /// there is none, always for free-mail addresses, and when no
            /// mail from the address's domain was authenticated by the
            /// user's provider (DMARC or aligned DKIM).
            fn sender_picture(&self, address: &str) -> zbus::Result<Vec<u8>>;

            /// The company of the person at `address`, as JSON
            /// (`katna_sync::pictures::Company`): the one at `website` (the
            /// site their signature names; may be empty), else the one
            /// their address belongs to. Read from its home page, and from
            /// Wikipedia when Wikidata lists the same website; cached for a
            /// week. Empty for none, for free-mail addresses without a
            /// website, and under the same authentication rule as
            /// [`Self::sender_picture`].
            fn company_of(&self, address: &str, website: &str) -> zbus::Result<String>;

            /// Translates `text`, the plain text of `message` (HTML made
            /// plain, quotes and signature kept, never attachments), from
            /// `source` into `target`, LibreTranslate codes such as `es`,
            /// `en` or `zt`, with Katna Server. `source` is the language
            /// the caller found in it on this computer (`auto` when
            /// unclear): mail already in `target` is never sent. Returns
            /// the language it was in, the translation, and a
            /// [`translate_problem`] when there is none. Translations are
            /// kept in the store, so asking again needs no server.
            fn translate(
                &self,
                message: i64,
                text: &str,
                source: &str,
                target: &str,
            ) -> zbus::Result<(String, String, String)>;

            /// The languages Katna Server can translate into `target`, and
            /// a [`translate_problem`] when it could not be asked (`sign-in`
            /// while this computer is not signed in to a Katna account).
            fn translation_sources(&self, target: &str) -> zbus::Result<(Vec<String>, String)>;

            /// Rephrases `text`, the text the user selected in a message
            /// being written, in `tone` (a `katna_ai::Tone` id such as
            /// `clearer`; `instruction` is the user's own for `custom`),
            /// with the AI service the settings name. Returns the new text,
            /// the account's plan with Katna AI (`trial`, `paid`, or `own`
            /// for the user's own service) and the free days left, and a
            /// `katna_ai::wire::problem` when there is no text.
            fn ai_rephrase(
                &self,
                text: &str,
                tone: &str,
                instruction: &str,
            ) -> zbus::Result<(String, String, u32, String)>;

            /// The rest of the sentence at the end of `before`, the
            /// paragraph being written, with the space it needs first, or
            /// empty when the service is unsure; and a
            /// `katna_ai::wire::problem` when it could not be asked (`off`
            /// unless AI autocomplete is on). `answered`, the mail being
            /// answered, is sent only when the settings allow it.
            fn ai_complete(&self, before: &str, answered: &str) -> zbus::Result<(String, String)>;

            /// Sums up a conversation: `request` is a
            /// `katna_ai::summary::SummarizeRequest` as JSON and `newest`
            /// the newest of its mails sent, which the summary is kept
            /// with in `mail.db`. Returns the summary as JSON
            /// (`katna_ai::summary::Summary`), the plan and free days left
            /// as for `AiRephrase`, and a `katna_ai::wire::problem` when
            /// there is none.
            fn ai_summarize(
                &self,
                newest: i64,
                request: &str,
            ) -> zbus::Result<(String, String, u32, String)>;

            /// Writes a first draft of a reply or a forward's note, or
            /// ideas for one: `request` is a `katna_ai::draft::DraftRequest`
            /// as JSON. Returns the draft (ideas as a JSON array of
            /// strings), the plan and free days left as for `AiRephrase`,
            /// and a `katna_ai::wire::problem` when there is none.
            fn ai_draft(&self, request: &str) -> zbus::Result<(String, String, u32, String)>;

            /// Saves the key of the user's own AI service in the Secret
            /// Service; an empty key deletes it.
            fn set_ai_key(&self, key: &str) -> zbus::Result<()>;

            /// Whether a key of the user's own AI service is saved.
            fn ai_key_saved(&self) -> zbus::Result<bool>;

            /// The models the user's own AI service `provider` (a
            /// `katna_ai::provider` id; `address` for `other`) offers to
            /// the saved key, and a `katna_ai::wire::problem` when it
            /// could not be asked.
            fn ai_models(&self, provider: &str, address: &str)
            -> zbus::Result<(Vec<String>, String)>;

            /// Reads the settings file again; call after saving settings
            /// the daemon uses (`sync.metered`).
            fn reload_config(&self) -> zbus::Result<()>;

            /// Whether the daemon saves data as on a metered network (no
            /// bodies downloaded ahead of time).
            fn metered(&self) -> zbus::Result<bool>;

            /// The Katna account (for Katna Server's features) this
            /// computer is signed in to. Asks the server, so it also
            /// notices a sign-out from another computer; offline it answers
            /// what it last knew.
            fn katna_account(&self) -> zbus::Result<KatnaAccount>;

            /// Creates a Katna account and signs this computer in; the
            /// server mails a code for `KatnaVerify`. Errors carry a
            /// [`katna_error`] name as their message, like every Katna
            /// account command.
            fn katna_sign_up(&self, email: &str, password: &str) -> zbus::Result<KatnaAccount>;

            /// Signs this computer in to a Katna account.
            fn katna_sign_in(&self, email: &str, password: &str) -> zbus::Result<KatnaAccount>;

            /// Confirms the account's address with the mailed code.
            fn katna_verify(&self, code: &str) -> zbus::Result<KatnaAccount>;

            /// Mails a new code for `KatnaVerify`.
            fn katna_resend_code(&self) -> zbus::Result<()>;

            /// Signs this computer out. It stays registered with the server.
            fn katna_sign_out(&self) -> zbus::Result<()>;

            /// The computers signed in to the account.
            fn katna_devices(&self) -> zbus::Result<Vec<KatnaDevice>>;

            /// Signs another computer out.
            fn katna_sign_out_device(&self, id: &str) -> zbus::Result<()>;

            /// Changes the password; other computers are signed out.
            fn katna_change_password(&self, current: &str, new: &str) -> zbus::Result<()>;

            /// Mails a code for `KatnaConfirmReset` to `email`, if it has an
            /// account.
            fn katna_reset_password(&self, email: &str) -> zbus::Result<()>;

            /// Sets a new password with the mailed code and signs this
            /// computer in; other computers are signed out.
            fn katna_confirm_reset(&self, email: &str, code: &str, password: &str)
            -> zbus::Result<KatnaAccount>;

            /// Deletes the Katna account and everything the server keeps for
            /// it. Mail on this computer is not touched.
            fn katna_delete_account(&self, password: &str) -> zbus::Result<()>;

            /// Where an update of Katna stands.
            fn update_status(&self) -> zbus::Result<UpdateStatus>;

            /// What the version on offer brings, as the JSON of its
            /// `katna_core::update::Manifest`, or empty before a check found
            /// one. A string, so the manifest can grow without changing the
            /// interface.
            fn update_details(&self) -> zbus::Result<String>;

            /// Looks for a newer version now, also on a metered connection,
            /// and downloads it when `updates.auto_download` is on.
            fn check_for_update(&self) -> zbus::Result<()>;

            /// Downloads the version a check found.
            fn download_update(&self) -> zbus::Result<()>;

            /// `UpdateStatus` changed.
            #[zbus(signal)]
            fn update_changed(&self) -> zbus::Result<()>;

            /// `KatnaAccount` changed.
            #[zbus(signal)]
            fn katna_account_changed(&self) -> zbus::Result<()>;

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

            /// Drive upload `id` moved on; see `DriveUploadStatus`.
            #[zbus(signal)]
            fn drive_changed(&self, id: i64) -> zbus::Result<()>;

            /// Calendars or their events changed in the store, or
            /// `CalendarStatus` did: read them again.
            #[zbus(signal)]
            fn calendar_changed(&self) -> zbus::Result<()>;

            /// `Metered` changed.
            #[zbus(signal)]
            fn metered_changed(&self, metered: bool) -> zbus::Result<()>;

            /// A tracked message was opened or a link in it followed; read
            /// the tracking tables of the store again.
            #[zbus(signal)]
            fn tracking_changed(&self) -> zbus::Result<()>;

            /// Saved contacts changed; read them from the store again.
            #[zbus(signal)]
            fn contacts_changed(&self) -> zbus::Result<()>;
        }
    };
}

katna_core::with_dbus_names!(pim_proxy);
