// SPDX-License-Identifier: GPL-3.0-or-later

//! Account workers for IMAP, JMAP, POP3 and SMTP, the outbox and the operation
//! queue. Runs inside `katna-daemon`. See `docs/ARCHITECTURE.md` §6 and §11.
//!
//! So far this is the I/O layer (plan task 1.1):
//!
//! - [`MailBackend`] and [`MailSender`]: the protocol traits the sync engine
//!   uses, with Katna's own types ([`Folder`], [`Envelope`], …).
//! - [`imap::ImapBackend`] and [`smtp::SmtpSender`]: Pimalaya's sans-I/O
//!   `io-imap` and `io-smtp` coroutines, driven by our own networking
//!   ([`net`]). Pimalaya types never leave those two modules.
//! - [`connection`]: gives each connection to a single task and hands out
//!   cheap handles, because IMAP commands cannot be cancelled halfway.
//!
//! Everything is executor-independent: sockets and timers use the `async-io`
//! reactor, and [`connection::spawn`] returns a future for the caller to run.

mod backend;
pub mod connection;
mod error;
pub mod imap;
pub mod net;
pub mod smtp;

pub use backend::{
    Address, Credentials, Endpoint, Envelope, Flags, Folder, FolderChange, FolderRole,
    FolderStatus, MailBackend, MailSender, Security, Wait,
};
pub use error::{Error, Result};
