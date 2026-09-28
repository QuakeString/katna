// SPDX-License-Identifier: GPL-3.0-or-later

//! The protocol traits and the data types they exchange. Nothing here
//! depends on a protocol library.

use std::{
    collections::HashMap, fmt, future::Future, ops::RangeInclusive, sync::Arc, time::Duration,
};

use crate::{Error, Result, oauth::TokenSource};

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

/// How to log in: a user name with a password (app passwords included) or
/// with OAuth2 access tokens.
#[derive(Clone)]
pub struct Credentials {
    pub user: String,
    pub secret: Secret,
}

/// What proves who the user is.
#[derive(Clone)]
pub enum Secret {
    Password(String),
    /// Sent with SASL XOAUTH2; the source refreshes them.
    OAuth2(Arc<TokenSource>),
}

/// What to send for one login.
pub(crate) enum Login {
    Password(String),
    Token(String),
}

impl Credentials {
    /// A user name and password.
    pub fn new(user: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            secret: Secret::Password(password.into()),
        }
    }

    /// A user name and the access tokens of `tokens`.
    pub fn oauth2(user: impl Into<String>, tokens: Arc<TokenSource>) -> Self {
        Self {
            user: user.into(),
            secret: Secret::OAuth2(tokens),
        }
    }

    /// The password, or a fresh access token.
    pub(crate) async fn login(&self) -> Result<Login> {
        match &self.secret {
            Secret::Password(password) => Ok(Login::Password(password.clone())),
            Secret::OAuth2(tokens) => Ok(Login::Token(tokens.access_token().await?)),
        }
    }

    /// After a refused login: whether trying once more may work, because
    /// the access token was dropped and the next one is fresh.
    pub(crate) fn retry_after_refusal(&self) -> bool {
        match &self.secret {
            Secret::Password(_) => false,
            Secret::OAuth2(tokens) => tokens.forget_access_token(),
        }
    }
}

// Never print the password or token, not even in debug logs.
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

/// How much of the account's mail storage is used, from the server's
/// quota (IMAP QUOTA, RFC 9208). Both in bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quota {
    pub used: u64,
    pub limit: u64,
}

/// A mailbox address from a message header.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Address {
    pub name: Option<String>,
    pub email: String,
}

/// The keyword of mail marked important (RFC 8457). Backends that keep
/// importance some other way, like Gmail's Important label, report and
/// take it as this keyword too.
pub const IMPORTANT: &str = "$Important";

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
    /// The header fields in [`MessageHeaders::fields`], raw, as the server
    /// sent them. Decoding belongs to the MIME layer.
    pub header: Vec<u8>,
    /// Gmail's thread ID (`X-GM-THRID`), when the server has `X-GM-EXT-1`.
    pub gm_thread_id: Option<u64>,
    /// Gmail's message ID (`X-GM-MSGID`): the same in every folder (label)
    /// that shows the message.
    pub gm_msgid: Option<u64>,
    /// The attachments named by the message's structure (IMAP
    /// `BODYSTRUCTURE`); `None` when the server sent none we could read.
    pub attachments: Option<Vec<AttachmentPart>>,
}

/// An attachment of a message, from its structure: known before the body
/// is downloaded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AttachmentPart {
    /// IMAP body section, like `2` or `1.3`.
    pub part: String,
    /// `type/subtype`, lower case.
    pub mime: String,
    /// Decoded file name.
    pub filename: Option<String>,
    /// Decoded size in bytes, estimated from the encoded size.
    pub size: u64,
}

impl MessageHeaders {
    /// Every header field sync level 1 asks for: [`MessageHeaders::FIELDS`]
    /// and the ones the inbox-category classifier reads
    /// ([`katna_core::category::CLASSIFIER_HEADERS`]).
    pub fn fields() -> impl Iterator<Item = &'static str> {
        Self::FIELDS
            .into_iter()
            .chain(katna_core::category::CLASSIFIER_HEADERS.iter().copied())
    }

    /// The header fields sync level 1 asks for (`docs/ARCHITECTURE.md`
    /// §6.2), besides the classifier's. `Content-Type` is there to guess
    /// attachments when the server sends no usable `BODYSTRUCTURE`.
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

/// What [`MailBackend::fetch_flags`] found.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FlagChanges {
    /// Current flags of the messages asked about (all of them, or those
    /// changed since the given mod-sequence).
    pub flags: Vec<FlagState>,
    /// UID ranges expunged since that mod-sequence, when the server told
    /// (QRESYNC `VANISHED (EARLIER)`); they may cover UIDs never seen.
    /// `None`: find expunged mail by comparing UID lists.
    pub vanished: Option<Vec<RangeInclusive<u32>>>,
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
    /// These UIDs were removed (QRESYNC's `VANISHED`, sent instead of
    /// `EXPUNGE` once QRESYNC is enabled).
    Vanished(Vec<RangeInclusive<u32>>),
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

    /// Copies `uids` of the selected folder to `folder` (on Gmail: adds
    /// its label). Returns `(old UID, new UID)` pairs when the server
    /// reports them (UIDPLUS).
    fn copy_messages(
        &mut self,
        uids: &[u32],
        folder: &str,
    ) -> impl Future<Output = Result<Vec<(u32, u32)>>> + Send;

    /// Deletes `uids` of the selected folder for good. Without UIDPLUS
    /// they are only marked `\Deleted`, so other clients' marks are kept.
    fn expunge(&mut self, uids: &[u32]) -> impl Future<Output = Result<()>> + Send;

    /// Flags for UIDs `first..=last` of the selected folder, in UID order.
    /// With `changed_since` (CONDSTORE), only messages whose flags changed
    /// after that mod-sequence; with QRESYNC also the UIDs expunged since.
    fn fetch_flags(
        &mut self,
        first: u32,
        last: u32,
        changed_since: Option<u64>,
    ) -> impl Future<Output = Result<FlagChanges>> + Send;

    /// Every UID in the selected folder, ascending.
    fn uids(&mut self) -> impl Future<Output = Result<Vec<u32>>> + Send;

    /// Counts of `folder` without selecting it (IMAP STATUS): message
    /// count, UIDVALIDITY, UIDNEXT and, with CONDSTORE, HIGHESTMODSEQ.
    /// Not for the selected folder, whose news arrive by themselves.
    fn status(&mut self, folder: &str) -> impl Future<Output = Result<FolderStatus>> + Send;

    /// Creates `folder` (a full path) and subscribes to it.
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

    /// Gmail search (`X-GM-RAW`, for example `category:promotions`) over
    /// UIDs `first..` of the selected folder, ascending. `Ok(None)` when the
    /// server is not Gmail (no `X-GM-EXT-1`).
    fn gmail_search(
        &mut self,
        first: u32,
        query: &str,
    ) -> impl Future<Output = Result<Option<Vec<u32>>>> + Send {
        let _ = (first, query);
        async { Ok(None) }
    }

    /// Gmail's `X-GM-MSGID` of the messages at `uids` in the selected
    /// folder. `Ok(None)` when the server is not Gmail (no `X-GM-EXT-1`).
    fn gmail_message_ids(
        &mut self,
        uids: &[u32],
    ) -> impl Future<Output = Result<Option<HashMap<u32, u64>>>> + Send {
        let _ = uids;
        async { Ok(None) }
    }

    /// The storage quota of the account's inbox. `Ok(None)` when the
    /// server has no QUOTA extension or sets no storage limit.
    fn quota(&mut self) -> impl Future<Output = Result<Option<Quota>>> + Send {
        async { Ok(None) }
    }

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

    /// The longest time, in seconds, the server holds a message before
    /// it goes out (SMTP `FUTURERELEASE`, RFC 4865), if it can. May ask
    /// the server: some list it only after login.
    fn hold_limit(&mut self) -> impl Future<Output = Result<Option<u64>>> + Send {
        async { Ok(None) }
    }

    /// Whether the server mails the sender delivery status notifications
    /// (SMTP `DSN`, RFC 3461). May ask the server.
    fn offers_receipts(&mut self) -> impl Future<Output = Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Asks for a delivery status notification for each recipient of the
    /// messages sent from now on, where the server
    /// [offers them](Self::offers_receipts); elsewhere they go out without.
    fn ask_for_receipts(&mut self, on: bool) {
        let _ = on;
    }

    /// Like [`send`](Self::send), but the server holds the message until
    /// `until` (Unix seconds). Only when [`hold_limit`](Self::hold_limit)
    /// allows it; SMTP cannot take the message back afterwards.
    fn send_held(
        &mut self,
        from: &str,
        to: &[&str],
        message: Vec<u8>,
        until: i64,
    ) -> impl Future<Output = Result<()>> + Send {
        let _ = (from, to, message, until);
        async { Err(Error::Rejected("the server cannot hold mail".into())) }
    }

    fn quit(self) -> impl Future<Output = Result<()>> + Send;
}
