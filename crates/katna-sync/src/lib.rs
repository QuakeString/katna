// SPDX-License-Identifier: GPL-3.0-or-later

//! Account workers for IMAP, JMAP, POP3 and SMTP, the outbox and the operation
//! queue. Runs inside `katna-daemon`. See `docs/ARCHITECTURE.md` §6 and §11.
//!
//! So far: the I/O layer (plan task 1.1), the first parts of IMAP sync
//! (tasks 1.3 and 1.5), the account worker (task 1.4) and the operation
//! queue (task 1.6) and sending (task 1.8).
//!
//! - [`MailBackend`] and [`MailSender`]: the protocol traits the sync engine
//!   uses, with Katna's own types ([`Folder`], [`Envelope`], …).
//! - [`imap::ImapBackend`] and [`smtp::SmtpSender`]: Pimalaya's sans-I/O
//!   `io-imap` and `io-smtp` coroutines, driven by our own networking
//!   ([`net`]). Pimalaya types never leave those two modules.
//! - [`connection`]: gives each connection to a single task and hands out
//!   cheap handles, because IMAP commands cannot be cancelled halfway.
//! - [`engine`]: sync level 1 (folders, flags, headers) into the store.
//! - [`autoconfig`]: finds an address's IMAP and SMTP servers.
//! - [`bodies`]: sync level 3, full messages for the offline window and on
//!   request.
//! - [`oauth`]: signing in to Google and Microsoft accounts with OAuth2,
//!   and refreshing their access tokens.
//! - [`ops`]: the operation queue; local flag changes, moves and deletes,
//!   replayed on the server.
//! - [`outbox`]: queued outgoing mail, undo send, SMTP delivery and
//!   filing in Sent.
//! - [`pictures`]: remote images and sender pictures for the reading pane,
//!   which never uses the network itself.
//! - [`pop3`]: our own POP3 client, and downloading a maildrop into the
//!   store (task 1.10).
//! - [`worker`]: keeps an account in sync: IDLE, periodic full syncs,
//!   reconnecting with backoff.
//!
//! Everything is executor-independent: sockets and timers use the `async-io`
//! reactor, and [`connection::spawn`] returns a future for the caller to run.

pub mod autoconfig;
mod backend;
pub mod bodies;
pub mod connection;
pub mod drive;
pub mod engine;
mod error;
pub mod imap;
pub mod net;
pub mod oauth;
pub mod ops;
pub mod outbox;
pub mod pictures;
pub mod pop3;
pub mod smtp;
pub mod tracking;
pub mod worker;

pub use backend::{
    Address, AttachmentPart, Credentials, Endpoint, Envelope, FlagChanges, FlagState, Flags,
    Folder, FolderChange, FolderRole, FolderStatus, IMPORTANT, MailBackend, MailSender,
    MessageHeaders, Quota, Secret, Security, Wait,
};
pub use error::{Error, Result};
