// SPDX-License-Identifier: GPL-3.0-or-later

//! The protocol traits and the data types they exchange. Nothing here
//! depends on a protocol library.

use std::{fmt, future::Future, time::Duration};

use crate::Result;

/// How to reach a server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
    pub security: Security,
}

impl Endpoint {
    pub fn new(host: impl Into<String>, port: u16, security: Security) -> Self {
        Self {
            host: host.into(),
            port,
            security,
        }
    }
}

/// Transport security of an [`Endpoint`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Security {
    /// Implicit TLS (IMAPS 993, SMTPS 465).
    Tls,
    /// Plain connection upgraded with STARTTLS. Fails if the server does not
    /// offer STARTTLS.
    StartTls,
    /// No encryption. Only for local test servers.
    Plain,
}

/// Password credentials (app passwords included). OAuth2 comes in Phase 5.
#[derive(Clone)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

impl Credentials {
    pub fn new(user: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            password: password.into(),
        }
    }
}

// Never print the password, not even in debug logs.
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            .finish_non_exhaustive()
    }
}

/// A folder on the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Folder {
    /// Full name, decoded from IMAP's modified UTF-7.
    pub name: String,
    /// Hierarchy separator, if the server uses one.
    pub delimiter: Option<char>,
    pub role: Option<FolderRole>,
    /// `false` for folders that only hold other folders.
    pub selectable: bool,
}

/// Special-use role of a folder (RFC 6154, plus the inbox).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FolderRole {
    Inbox,
    All,
    Archive,
    Drafts,
    Flagged,
    Junk,
    Sent,
    Trash,
}

/// State of a folder, as reported when it is selected.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FolderStatus {
    /// Number of messages.
    pub exists: u32,
    pub uid_validity: Option<u32>,
    pub uid_next: Option<u32>,
    /// Only when the server supports CONDSTORE.
    pub highest_modseq: Option<u64>,
}

/// A mailbox address from a message header.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Address {
    pub name: Option<String>,
    pub email: String,
}

/// Message flags. Keywords keep their IMAP spelling (`$Forwarded`, …).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    pub seen: bool,
    pub answered: bool,
    pub flagged: bool,
    pub deleted: bool,
    pub draft: bool,
    pub keywords: Vec<String>,
}

/// Header-level data for the message list.
///
/// Header values are raw: RFC 2047 decoding belongs to the MIME layer, not
/// to the protocol.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Envelope {
    pub uid: u32,
    pub size: u32,
    pub flags: Flags,
    pub date: Option<String>,
    pub subject: Option<String>,
    pub from: Vec<Address>,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub message_id: Option<String>,
    pub in_reply_to: Option<String>,
}

/// What sync level 1 downloads for a message: flags, size and the header
/// fields the store indexes, but no body.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageHeaders {
    pub uid: u32,
    pub size: u32,
    pub flags: Flags,
    /// When the server received the message (IMAP INTERNALDATE), in Unix
    /// seconds. The fallback for messages without a `Date` header.
    pub received: Option<i64>,
    /// The header fields in [`MessageHeaders::FIELDS`], raw, as the server
    /// sent them. Decoding belongs to the MIME layer.
    pub header: Vec<u8>,
}

impl MessageHeaders {
    /// The header fields sync level 1 asks for (`docs/ARCHITECTURE.md`
    /// §6.2). `Content-Type` is there to spot likely attachments until
    /// `BODYSTRUCTURE` is parsed.
    pub const FIELDS: [&'static str; 14] = [
        "Date",
        "Subject",
        "From",
        "Sender",
        "Reply-To",
        "To",
        "Cc",
        "Bcc",
        "Message-ID",
        "In-Reply-To",
        "References",
        "List-Id",
        "Content-Type",
        "Authentication-Results",
    ];
}

/// Current flags of a message, from [`MailBackend::fetch_flags`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlagState {
    pub uid: u32,
    pub flags: Flags,
}

/// A change the server reported for the selected folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FolderChange {
    /// The folder now holds this many messages.
    Exists(u32),
    /// The message at this sequence number was removed. Later sequence
    /// numbers shift down by one.
    Expunged(u32),
    /// Flags changed for the message at this sequence number.
    FlagsChanged { seq: u32, flags: Flags },
}

/// Result of [`MailBackend::wait_for_changes`].
#[derive(Debug)]
pub struct Wait<T> {
    /// Changes the server reported while we waited. Empty when the wait
    /// timed out or was interrupted before anything happened.
    pub changes: Vec<FolderChange>,
    /// The output of the interrupt future, if it is what ended the wait.
    pub interrupted: Option<T>,
}

/// Mail access protocol: IMAP now, JMAP and POP3 later.
///
/// A backend is one connection. Its futures must run to completion: a
/// command dropped halfway leaves the connection out of step with the
/// server. [`crate::connection`] makes that safe for callers.
///
/// Futures are `Send`, so the daemon can use any executor.
pub trait MailBackend: Send + 'static {
    fn list_folders(&mut self) -> impl Future<Output = Result<Vec<Folder>>> + Send;

    /// Opens a folder for the commands below. Asks for CONDSTORE when the
    /// server has it, so [`FolderStatus::highest_modseq`] is filled in.
    fn select(&mut self, folder: &str) -> impl Future<Output = Result<FolderStatus>> + Send;

    /// Envelopes for UIDs `first..=last` (or `first..` when `last` is
    /// `None`) of the selected folder, in UID order.
    fn fetch_envelopes(
        &mut self,
        first: u32,
        last: Option<u32>,
    ) -> impl Future<Output = Result<Vec<Envelope>>> + Send;

    /// Flags, size and indexed header fields for UIDs `first..=last` (or
    /// `first..`) of the selected folder, in UID order.
    fn fetch_headers(
        &mut self,
        first: u32,
        last: Option<u32>,
    ) -> impl Future<Output = Result<Vec<MessageHeaders>>> + Send;

    /// The full raw messages at `uids` in the selected folder, as
    /// `(uid, bytes)` in UID order, without setting `\Seen`. UIDs that no
    /// longer exist are left out.
    fn fetch_bodies(
        &mut self,
        uids: &[u32],
    ) -> impl Future<Output = Result<Vec<(u32, Vec<u8>)>>> + Send;

    /// Adds (`add`) or removes the set flags and keywords of `flags` on
    /// `uids` of the selected folder.
    fn store_flags(
        &mut self,
        uids: &[u32],
        flags: &Flags,
        add: bool,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Moves `uids` of the selected folder to `folder`. Returns
    /// `(old UID, new UID)` pairs when the server reports them (UIDPLUS).
    fn move_messages(
        &mut self,
        uids: &[u32],
        folder: &str,
    ) -> impl Future<Output = Result<Vec<(u32, u32)>>> + Send;

    /// Deletes `uids` of the selected folder for good. Without UIDPLUS
    /// they are only marked `\Deleted`, so other clients' marks are kept.
    fn expunge(&mut self, uids: &[u32]) -> impl Future<Output = Result<()>> + Send;

    /// Flags for UIDs `first..=last` of the selected folder, in UID order.
    /// With `changed_since` (CONDSTORE), only messages whose flags changed
    /// after that mod-sequence.
    fn fetch_flags(
        &mut self,
        first: u32,
        last: u32,
        changed_since: Option<u64>,
    ) -> impl Future<Output = Result<Vec<FlagState>>> + Send;

    /// Every UID in the selected folder, ascending.
    fn uids(&mut self) -> impl Future<Output = Result<Vec<u32>>> + Send;

    fn create_folder(&mut self, folder: &str) -> impl Future<Output = Result<()>> + Send;

    /// Stores `message` in `folder` with `flags`.
    fn append_with_flags(
        &mut self,
        folder: &str,
        message: Vec<u8>,
        flags: &Flags,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Stores `message` in `folder` without flags.
    fn append(
        &mut self,
        folder: &str,
        message: Vec<u8>,
    ) -> impl Future<Output = Result<()>> + Send {
        async move {
            self.append_with_flags(folder, message, &Flags::default())
                .await
        }
    }

    /// Asks the server for changes to the selected folder (IMAP NOOP).
    fn poll_changes(&mut self) -> impl Future<Output = Result<Vec<FolderChange>>> + Send;

    /// Waits on the selected folder until the server reports a change,
    /// `max_wait` passes, or `interrupt` completes, whichever is first.
    /// Uses IMAP IDLE when the server has it and a single NOOP at the end
    /// otherwise.
    ///
    /// The connection is ready for the next command when this returns
    /// `Ok`, also after an interrupt. `interrupt` may be dropped unfinished,
    /// so it must be cancel-safe.
    fn wait_for_changes<I>(
        &mut self,
        max_wait: Duration,
        interrupt: I,
    ) -> impl Future<Output = Result<Wait<I::Output>>> + Send
    where
        I: Future + Send,
        I::Output: Send;

    /// Ends the session politely and closes the connection.
    fn logout(self) -> impl Future<Output = Result<()>> + Send;
}

/// Mail submission protocol (SMTP).
pub trait MailSender: Send + 'static {
    /// Sends `message` as is. `from` and `to` are the envelope addresses,
    /// which may differ from the headers (Bcc, mailing lists).
    ///
    /// After any error, SMTP leaves the transaction half done: drop the
    /// sender and connect again for the next message.
    fn send(
        &mut self,
        from: &str,
        to: &[&str],
        message: Vec<u8>,
    ) -> impl Future<Output = Result<()>> + Send;

    fn quit(self) -> impl Future<Output = Result<()>> + Send;
}
