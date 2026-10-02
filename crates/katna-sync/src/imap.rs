// SPDX-License-Identifier: GPL-3.0-or-later

//! IMAP backend: Pimalaya's `io-imap` coroutines driven by [`Conn`].
//!
//! Pimalaya and `imap-types` names end at this module's boundary. Where
//! io-imap 0.6 loses server updates, we build the command ourselves on its
//! public `ImapSend` building block (NOOP, IDLE); see
//! `docs/spikes/s2-pimalaya-io.md` for the list of problems.

use std::{
    collections::HashMap,
    fmt::Display,
    ops::RangeInclusive,
    pin::pin,
    time::{Duration, Instant},
};

use async_io::Timer;
use futures_lite::FutureExt;
use imap_codec::{
    CommandCodec, IdleDoneCodec, ResponseCodec,
    encode::Encoder,
    fragmentizer::{DecodeMessageError, FragmentInfo, Fragmentizer},
};
use imap_types::{
    body::{BodyStructure, SpecificFields},
    command::{Command, CommandBody, FetchModifier, SelectParameter},
    core::{AString, Atom, IString, NString, TagGenerator, Vec1},
    envelope::Address as ImapAddress,
    extensions::enable::CapabilityEnable,
    extensions::idle::IdleDone,
    fetch::{MacroOrMessageDataItemNames, MessageDataItem, MessageDataItemName, Section},
    flag::{Flag, FlagFetch, FlagNameAttribute, StoreType},
    mailbox::{ListMailbox, Mailbox},
    response::{Capability, Data, Response, Status, StatusKind},
    search::SearchKey,
    sequence::{SeqOrUid, Sequence, SequenceSet},
    status::{StatusDataItem, StatusDataItemName},
};
use io_imap::{
    coroutine::{ImapCoroutine, ImapCoroutineState as S, ImapYield},
    rfc3501::{
        append::{ImapMessageAppend, ImapMessageAppendError, ImapMessageAppendOptions},
        copy::{ImapMessageCopy, ImapMessageCopyError, ImapMessageCopyOptions},
        create::{ImapMailboxCreate, ImapMailboxCreateError},
        delete::{ImapMailboxDelete, ImapMailboxDeleteError},
        fetch::{ImapMessageFetch, ImapMessageFetchError, ImapMessageFetchOptions},
        list::{ImapMailboxList, ImapMailboxListError},
        rename::{ImapMailboxRename, ImapMailboxRenameError},
        search::{ImapMessageSearch, ImapMessageSearchError, ImapMessageSearchOptions},
        select::{ImapMailboxSelect, ImapMailboxSelectError, ImapMailboxSelectOptions},
        store::{ImapMessageStoreError, ImapMessageStoreOptions, ImapMessageStoreSilent},
        subscribe::{ImapMailboxSubscribe, ImapMailboxSubscribeError},
    },
    rfc4315::expunge_uid::{ImapMessageExpungeUid, ImapMessageExpungeUidError},
    rfc6851::r#move::{ImapMessageMove, ImapMessageMoveError, ImapMessageMoveOptions},
    send::{ImapSend, ImapSendError, ImapSendOutput},
    session::{
        ImapSessionOpen, ImapSessionOpenError, ImapSessionOpenOptions, ImapSessionOpenYield as O,
        ImapSessionTransport,
    },
};
use io_sasl::{mechanism::Sasl, rfc4616::plain::SaslPlainCreds, xoauth2::SaslXoauth2Creds};

use crate::{
    Address, AttachmentPart, Credentials, Endpoint, Envelope, Error, FlagChanges, FlagState, Flags,
    Folder, FolderChange, FolderRole, FolderStatus, IMPORTANT, MUTED, MailBackend, MessageHeaders,
    Quota, Result, Security, Wait,
    backend::Login,
    net::{Conn, Tls},
};

/// Largest single server response we accept (a full message literal).
const MAX_RESPONSE: u32 = 64 * 1024 * 1024;

/// One authenticated IMAP connection.
pub struct ImapBackend {
    conn: Conn,
    frag: Fragmentizer,
    tags: TagGenerator,
    capabilities: Vec<Capability<'static>>,
    /// QRESYNC is enabled: expunges arrive as `VANISHED`.
    qresync: bool,
}

impl ImapBackend {
    /// Connects, negotiates TLS and logs in with SASL PLAIN, or XOAUTH2
    /// for OAuth2 accounts. A refused access token is renewed once.
    pub async fn connect(endpoint: &Endpoint, creds: &Credentials, tls: Tls) -> Result<Self> {
        match Self::connect_once(endpoint, creds, tls.clone()).await {
            Err(Error::Auth(_)) if creds.retry_after_refusal() => {
                Self::connect_once(endpoint, creds, tls).await
            }
            result => result,
        }
    }

    async fn connect_once(endpoint: &Endpoint, creds: &Credentials, tls: Tls) -> Result<Self> {
        let transport = match endpoint.security {
            Security::Tls => ImapSessionTransport::Tls {
                host: endpoint.host.clone(),
                port: endpoint.port,
            },
            Security::StartTls | Security::Plain => ImapSessionTransport::Tcp {
                host: endpoint.host.clone(),
                port: endpoint.port,
            },
        };
        let opts = ImapSessionOpenOptions {
            starttls: endpoint.security == Security::StartTls,
            ..Default::default()
        };
        let sasl = match creds.login().await? {
            Login::Password(password) => Sasl::Plain(SaslPlainCreds {
                authzid: None,
                authcid: creds.user.clone(),
                passwd: password.into(),
            }),
            Login::Token(token) => Sasl::Xoauth2(SaslXoauth2Creds {
                username: creds.user.clone(),
                token: token.into(),
            }),
        };
        let mut co = ImapSessionOpen::new(transport, Some(sasl), opts);

        let mut conn = Conn::new(tls);
        let mut frag = Fragmentizer::new(MAX_RESPONSE);
        let mut state = co.resume(&mut frag, None);
        let session = loop {
            state = match state {
                S::Yielded(O::WantsTcpConnect { host, port }) => {
                    conn.connect_tcp(&host, port).await?;
                    co.resume(&mut frag, None)
                }
                S::Yielded(O::WantsTlsConnect { host, port }) => {
                    conn.connect_tls(&host, port).await?;
                    co.resume(&mut frag, None)
                }
                S::Yielded(O::WantsTlsUpgrade) => {
                    conn.upgrade_tls().await?;
                    co.resume(&mut frag, None)
                }
                S::Yielded(O::WantsUnixConnect(path)) => {
                    return Err(Error::Protocol(format!(
                        "unix sockets are not supported: {path}"
                    )));
                }
                S::Yielded(O::WantsWrite(bytes)) => {
                    conn.write_all(&bytes).await?;
                    co.resume(&mut frag, None)
                }
                S::Yielded(O::WantsRead) => {
                    let bytes = conn.read().await?;
                    co.resume(&mut frag, Some(bytes))
                }
                S::Complete(Ok(session)) => break session,
                S::Complete(Err(err)) => return Err(session_error(err)),
            };
        };
        tracing::debug!(host = endpoint.host, "IMAP session open");

        let mut backend = Self {
            conn,
            frag,
            tags: TagGenerator::new(),
            capabilities: session.capability,
            qresync: false,
        };
        if backend.has(&Capability::QResync) {
            backend.qresync = backend.enable("QRESYNC").await?;
        }
        Ok(backend)
    }

    /// `ENABLE` one extension. Returns whether the server enabled it.
    async fn enable(&mut self, name: &'static str) -> Result<bool> {
        let wanted = CapabilityEnable::try_from(name).map_err(protocol)?;
        let body = CommandBody::Enable {
            capabilities: Vec1::from(wanted.clone()),
        };
        match self.command(body).await {
            Ok(data) => Ok(data.iter().any(
                |d| matches!(d, Data::Enabled { capabilities } if capabilities.contains(&wanted)),
            )),
            Err(Error::Rejected(why)) => {
                tracing::warn!(%why, name, "server refused ENABLE");
                Ok(false)
            }
            Err(e) => Err(e),
        }
    }

    /// The server's capabilities after login, as IMAP spells them.
    pub fn capabilities(&self) -> Vec<String> {
        self.capabilities.iter().map(|c| c.to_string()).collect()
    }

    fn has(&self, capability: &Capability<'_>) -> bool {
        self.capabilities.contains(capability)
    }

    /// Gmail's extensions (`X-GM-EXT-1`): thread IDs, labels, `X-GM-RAW`.
    fn is_gmail(&self) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.to_string().eq_ignore_ascii_case("X-GM-EXT-1"))
    }

    /// The QUOTA extension (RFC 2087, or RFC 9208's `QUOTA=RES-STORAGE`).
    fn has_quota(&self) -> bool {
        self.capabilities.iter().any(|c| {
            let c = c.to_string();
            c.eq_ignore_ascii_case("QUOTA") || c.eq_ignore_ascii_case("QUOTA=RES-STORAGE")
        })
    }

    /// Sends a command imap-codec cannot build or parse (Gmail's
    /// extensions) as a raw line, and returns the untagged responses as
    /// text, without the leading `* `. The command must not contain
    /// literals, and neither may its responses.
    async fn raw_command(&mut self, command: &str) -> Result<Vec<String>> {
        let tag = self.tags.generate();
        let tag = tag.inner().to_owned();
        let Self { conn, frag, .. } = self;
        conn.write_all(format!("{tag} {command}\r\n").as_bytes())
            .await?;
        let mut untagged = Vec::new();
        loop {
            while let Some(info) = frag.progress() {
                if !matches!(info, FragmentInfo::Line { .. }) || !frag.is_message_complete() {
                    continue;
                }
                let line = String::from_utf8_lossy(frag.message_bytes());
                let line = line.trim_end_matches(['\r', '\n']);
                if let Some(data) = line.strip_prefix("* ") {
                    if data.get(..3).is_some_and(|w| w.eq_ignore_ascii_case("BYE")) {
                        return Err(Error::Closed(format!("server said {data}")));
                    }
                    untagged.push(data.to_owned());
                } else if let Some(status) = line
                    .strip_prefix(tag.as_str())
                    .and_then(|rest| rest.strip_prefix(' '))
                {
                    let name = command.split(' ').take(2).collect::<Vec<_>>().join(" ");
                    return match status.get(..2) {
                        Some(ok) if ok.eq_ignore_ascii_case("OK") => Ok(untagged),
                        _ => Err(Error::Rejected(format!("{name}: {status}"))),
                    };
                }
            }
            match conn.read().await? {
                [] => return Err(Error::Closed("server closed the connection".into())),
                bytes => frag.enqueue_bytes(bytes),
            }
        }
    }

    /// Gmail thread and message IDs of UIDs `first..=last` (or `first..`).
    async fn gmail_ids(&mut self, first: u32, last: Option<u32>) -> Result<HashMap<u32, GmailIds>> {
        let range = match last {
            Some(last) => format!("{first}:{last}"),
            None => format!("{first}:*"),
        };
        let lines = self
            .raw_command(&format!("UID FETCH {range} (UID X-GM-THRID X-GM-MSGID)"))
            .await?;
        Ok(lines.iter().filter_map(|line| gmail_fetch(line)).collect())
    }

    /// Gmail's Important and Muted labels (`X-GM-LABELS`) of each message
    /// of the UID set `uids`.
    async fn gmail_labels(&mut self, uids: &str) -> Result<HashMap<u32, GmailLabels>> {
        let lines = self
            .raw_command(&format!("UID FETCH {uids} (UID X-GM-LABELS)"))
            .await?;
        Ok(lines.iter().filter_map(|line| gmail_labels(line)).collect())
    }

    /// On Gmail, adds [`IMPORTANT`] and [`MUTED`] to the flags of messages
    /// with the Important or Muted label: they are labels there, not
    /// keywords.
    async fn add_gmail_labels<'a>(
        &mut self,
        uids: &str,
        flags: impl Iterator<Item = (u32, &'a mut Flags)>,
    ) -> Result<()> {
        let labels = self.gmail_labels(uids).await?;
        for (uid, flags) in flags {
            let Some(labels) = labels.get(&uid) else {
                continue;
            };
            for (on, keyword) in [(labels.important, IMPORTANT), (labels.muted, MUTED)] {
                if on {
                    flags.keywords.retain(|k| !k.eq_ignore_ascii_case(keyword));
                    flags.keywords.push(keyword.to_owned());
                }
            }
        }
        Ok(())
    }

    /// The attachments of UIDs `first..=last` (or `first..`), from their
    /// `BODYSTRUCTURE`. A command of its own: a structure imap-codec cannot
    /// parse costs only that message's attachment list, never the message.
    async fn structures(
        &mut self,
        first: u32,
        last: Option<u32>,
    ) -> Result<HashMap<u32, Vec<AttachmentPart>>> {
        let items = vec![MessageDataItemName::Uid, MessageDataItemName::BodyStructure];
        let fetched = match self.uid_fetch_range(first, last, items, Vec::new()).await {
            Ok(fetched) => fetched,
            Err(Error::Rejected(why)) => {
                tracing::warn!(%why, "server refused BODYSTRUCTURE; guessing attachments");
                return Ok(HashMap::new());
            }
            Err(e) => return Err(e),
        };
        Ok(fetched
            .into_iter()
            .filter_map(|items| {
                let mut uid = None;
                let mut parts = None;
                for item in items {
                    match item {
                        MessageDataItem::Uid(value) => uid = Some(value.get()),
                        MessageDataItem::BodyStructure(structure) => {
                            let mut out = Vec::new();
                            attachment_parts(&structure, "", false, &mut out);
                            parts = Some(out);
                        }
                        _ => {}
                    }
                }
                Some((uid?, parts?))
            })
            .collect())
    }

    /// `UID FETCH first:last` (or `first:*`). Returns each message's items,
    /// in UID order, without messages below `first` (`n:*` always matches
    /// the last message, even when its UID is lower).
    /// `UID FETCH` of the UIDs in `set`. Keeps only messages whose UID
    /// passes `keep` (`n:*` also returns the last message when every UID
    /// is below `n`), in UID order.
    async fn uid_fetch(
        &mut self,
        set: &str,
        keep: impl Fn(u32) -> bool,
        items: Vec<MessageDataItemName<'static>>,
        modifiers: Vec<FetchModifier>,
    ) -> Result<Vec<Vec<MessageDataItem<'static>>>> {
        let set = SequenceSet::try_from(set).map_err(protocol)?;
        let items = MacroOrMessageDataItemNames::MessageDataItemNames(items);
        let opts = ImapMessageFetchOptions {
            uid: true,
            modifiers,
        };
        let fetched = self.run(ImapMessageFetch::new(set, items, opts)).await?;
        let mut messages: Vec<(u32, Vec<MessageDataItem<'static>>)> = fetched
            .into_values()
            .filter_map(|items| {
                let items: Vec<_> = items.into_iter().collect();
                let uid = items.iter().find_map(|item| match item {
                    MessageDataItem::Uid(uid) => Some(uid.get()),
                    _ => None,
                })?;
                keep(uid).then_some((uid, items))
            })
            .collect();
        messages.sort_by_key(|(uid, _)| *uid);
        Ok(messages.into_iter().map(|(_, items)| items).collect())
    }

    /// `UID FETCH first:last` (or `first:*`).
    async fn uid_fetch_range(
        &mut self,
        first: u32,
        last: Option<u32>,
        items: Vec<MessageDataItemName<'static>>,
        modifiers: Vec<FetchModifier>,
    ) -> Result<Vec<Vec<MessageDataItem<'static>>>> {
        let range = match last {
            Some(last) => format!("{first}:{last}"),
            None => format!("{first}:*"),
        };
        self.uid_fetch(&range, |uid| uid >= first, items, modifiers)
            .await
    }

    /// Runs one command coroutine to completion.
    async fn run<C, T, E>(&mut self, mut co: C) -> Result<T>
    where
        C: ImapCoroutine<Yield = ImapYield, Return = std::result::Result<T, E>>,
        E: Into<Error>,
    {
        let Self { conn, frag, .. } = self;
        let mut state = co.resume(frag, None);
        loop {
            state = match state {
                S::Yielded(ImapYield::WantsWrite(bytes)) => {
                    conn.write_all(&bytes).await?;
                    co.resume(frag, None)
                }
                S::Yielded(ImapYield::WantsRead) => {
                    let bytes = conn.read().await?;
                    co.resume(frag, Some(bytes))
                }
                S::Complete(result) => return result.map_err(Into::into),
            };
        }
    }

    /// Sends one message with `ImapSend`, which keeps every untagged
    /// response, and checks for BYE.
    async fn send<T>(
        &mut self,
        encoder: T,
        message: T::Message<'static>,
    ) -> Result<ImapSendOutput<T>>
    where
        T: Encoder + Send,
        T::Message<'static>: Send,
    {
        let out = self.run(ImapSend::new(encoder, message)).await?;
        if let Some(bye) = &out.bye {
            return Err(Error::Closed(format!("server said BYE: {}", bye.text)));
        }
        Ok(out)
    }

    /// Sends a command that ends with a tagged OK and returns its untagged
    /// data.
    async fn command(&mut self, body: CommandBody<'static>) -> Result<Vec<Data<'static>>> {
        let command = Command {
            tag: self.tags.generate(),
            body,
        };
        let name = command.body.name();
        let out = self.send(CommandCodec::new(), command).await?;
        expect_ok(name, &out)?;
        Ok(out.data)
    }

    /// IDLE on the selected folder. io-imap's own IDLE coroutine drops
    /// updates that arrive between our DONE and the server's tagged OK (S2
    /// problem 2), so we run the steps ourselves.
    async fn idle<I>(&mut self, max_wait: Duration, interrupt: I) -> Result<Wait<I::Output>>
    where
        I: Future + Send,
        I::Output: Send,
    {
        let deadline = Instant::now() + max_wait;

        // 0. Collect what changed since the last command. Stalwart 0.16
        //    reports such changes on NOOP only, not when IDLE starts.
        let pending = self.poll_changes().await?;
        if !pending.is_empty() {
            return Ok(Wait {
                changes: pending,
                interrupted: None,
            });
        }

        // 1. IDLE, up to the server's continuation request.
        let command = Command {
            tag: self.tags.generate(),
            body: CommandBody::Idle,
        };
        let out = self.send(CommandCodec::new(), command).await?;
        if let Some(tagged) = &out.tagged {
            return Err(match tagged.body.kind {
                StatusKind::Ok => Error::Protocol("IDLE ended before it started".into()),
                _ => Error::Rejected(format!("IDLE: {}", tagged.body.text)),
            });
        }
        if out.continuation_request.is_none() {
            return Err(Error::Protocol("IDLE was not accepted".into()));
        }
        let mut changes: Vec<FolderChange> = out.data.iter().filter_map(folder_change).collect();

        // 2. Wait for pushed updates, the deadline or the interrupt. A
        //    cancelled read keeps its bytes (`Conn::read_timeout`), so racing
        //    it against the interrupt is safe.
        let mut interrupt = pin!(interrupt);
        let mut interrupted = None;
        let codec = ResponseCodec::new();
        while changes.is_empty() {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }
            let Self { conn, frag, .. } = self;
            let read = async { conn.read_timeout(remaining).await.map(Event::Read) };
            let stop = async { Ok(Event::Interrupt(interrupt.as_mut().await)) };
            match read.or(stop).await? {
                Event::Interrupt(value) => {
                    interrupted = Some(value);
                    break;
                }
                Event::Read(None) => break,
                Event::Read(Some([])) => {
                    return Err(Error::Closed("server closed the connection".into()));
                }
                Event::Read(Some(bytes)) => {
                    frag.enqueue_bytes(bytes);
                    decode_pushed(frag, &codec, &mut changes)?;
                }
            }
        }

        // 3. DONE. `ImapSend` collects what arrives before the tagged OK.
        let out = self.send(IdleDoneCodec::new(), IdleDone).await?;
        expect_ok("IDLE", &out)?;
        changes.extend(out.data.iter().filter_map(folder_change));
        Ok(Wait {
            changes,
            interrupted,
        })
    }

    /// For servers without IDLE: sleep, then ask once with NOOP.
    async fn wait_then_poll<I>(
        &mut self,
        max_wait: Duration,
        interrupt: I,
    ) -> Result<Wait<I::Output>>
    where
        I: Future + Send,
        I::Output: Send,
    {
        let timer = async {
            Timer::after(max_wait).await;
            None
        };
        match async { Some(interrupt.await) }.or(timer).await {
            Some(value) => Ok(Wait {
                changes: Vec::new(),
                interrupted: Some(value),
            }),
            None => Ok(Wait {
                changes: self.poll_changes().await?,
                interrupted: None,
            }),
        }
    }
}

enum Event<'a, T> {
    Read(Option<&'a [u8]>),
    Interrupt(T),
}

impl MailBackend for ImapBackend {
    async fn list_folders(&mut self) -> Result<Vec<Folder>> {
        let reference = Mailbox::try_from("").map_err(protocol)?;
        let pattern = ListMailbox::try_from("*").map_err(protocol)?;
        let listing = self.run(ImapMailboxList::new(reference, pattern)).await?;
        Ok(listing
            .into_iter()
            .map(|(mailbox, delimiter, attributes)| {
                let mut role = attributes.iter().find_map(folder_role);
                if matches!(mailbox, Mailbox::Inbox) {
                    role = Some(FolderRole::Inbox);
                }
                Folder {
                    name: mailbox_name(&mailbox),
                    delimiter: delimiter.map(|d| d.inner()),
                    role,
                    selectable: !attributes.contains(&FlagNameAttribute::Noselect),
                }
            })
            .collect())
    }

    async fn select(&mut self, folder: &str) -> Result<FolderStatus> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let mut opts = ImapMailboxSelectOptions::default();
        if self.has(&Capability::CondStore) {
            // Without it, servers leave out HIGHESTMODSEQ (S2 problem 6).
            opts.parameters.push(SelectParameter::CondStore);
        }
        let data = self.run(ImapMailboxSelect::new(mailbox, opts)).await?;
        Ok(FolderStatus {
            exists: data.exists.unwrap_or(0),
            uid_validity: data.uid_validity.map(|v| v.get()),
            uid_next: data.uid_next.map(|v| v.get()),
            highest_modseq: data.highest_mod_seq,
        })
    }

    async fn fetch_envelopes(&mut self, first: u32, last: Option<u32>) -> Result<Vec<Envelope>> {
        let items = vec![
            MessageDataItemName::Uid,
            MessageDataItemName::Flags,
            MessageDataItemName::Rfc822Size,
            MessageDataItemName::Envelope,
        ];
        let fetched = self.uid_fetch_range(first, last, items, Vec::new()).await?;
        Ok(fetched.into_iter().map(envelope).collect())
    }

    async fn fetch_headers(
        &mut self,
        first: u32,
        last: Option<u32>,
    ) -> Result<Vec<MessageHeaders>> {
        let fields = MessageHeaders::fields()
            .map(|name| AString::try_from(name).map_err(protocol))
            .collect::<Result<Vec<_>>>()?;
        let fields = Vec1::try_from(fields).map_err(protocol)?;
        let items = vec![
            MessageDataItemName::Uid,
            MessageDataItemName::Flags,
            MessageDataItemName::Rfc822Size,
            MessageDataItemName::InternalDate,
            MessageDataItemName::BodyExt {
                section: Some(Section::HeaderFields(None, fields)),
                partial: None,
                peek: true,
            },
        ];
        let fetched = self.uid_fetch_range(first, last, items, Vec::new()).await?;
        let mut messages: Vec<MessageHeaders> = fetched.into_iter().map(headers).collect();
        if !messages.is_empty() {
            let mut structures = self.structures(first, last).await?;
            for message in &mut messages {
                message.attachments = structures.remove(&message.uid);
            }
        }
        // imap-codec cannot parse X-GM-THRID, and would drop a whole FETCH
        // response that had it, so it comes in a second, small command.
        if self.is_gmail() && !messages.is_empty() {
            let ids = self.gmail_ids(first, last).await?;
            for message in &mut messages {
                if let Some(ids) = ids.get(&message.uid) {
                    message.gm_thread_id = ids.thread;
                    message.gm_msgid = ids.message;
                }
            }
            let range = match last {
                Some(last) => format!("{first}:{last}"),
                None => format!("{first}:*"),
            };
            let flags = messages.iter_mut().map(|m| (m.uid, &mut m.flags));
            self.add_gmail_labels(&range, flags).await?;
        }
        Ok(messages)
    }

    async fn fetch_bodies(&mut self, uids: &[u32]) -> Result<Vec<(u32, Vec<u8>)>> {
        if uids.is_empty() {
            return Ok(Vec::new());
        }
        let items = vec![
            MessageDataItemName::Uid,
            MessageDataItemName::BodyExt {
                section: None,
                partial: None,
                peek: true,
            },
        ];
        let wanted: std::collections::HashSet<u32> = uids.iter().copied().collect();
        let fetched = self
            .uid_fetch(
                &uid_set(uids),
                |uid| wanted.contains(&uid),
                items,
                Vec::new(),
            )
            .await?;
        Ok(fetched
            .into_iter()
            .filter_map(|items| {
                let mut uid = None;
                let mut body = None;
                for item in items {
                    match item {
                        MessageDataItem::Uid(value) => uid = Some(value.get()),
                        MessageDataItem::BodyExt { data, .. } => {
                            body = data.0.map(|d| d.as_ref().to_vec());
                        }
                        _ => {}
                    }
                }
                Some((uid?, body?))
            })
            .collect())
    }

    async fn fetch_flags(
        &mut self,
        first: u32,
        last: u32,
        changed_since: Option<u64>,
    ) -> Result<FlagChanges> {
        let since = changed_since.and_then(std::num::NonZeroU64::new);
        let items = vec![MessageDataItemName::Uid, MessageDataItemName::Flags];
        let fetched = match since {
            // QRESYNC: the flag changes and, as `VANISHED (EARLIER)`, the
            // expunges since `since`, in one command. io-imap's FETCH keeps
            // only FETCH data, so the command goes through `ImapSend`.
            Some(modseq) if self.qresync => {
                let body = CommandBody::Fetch {
                    sequence_set: SequenceSet::try_from(format!("{first}:{last}").as_str())
                        .map_err(protocol)?,
                    macro_or_item_names: MacroOrMessageDataItemNames::MessageDataItemNames(items),
                    uid: true,
                    modifiers: vec![FetchModifier::ChangedSince(modseq), FetchModifier::Vanished],
                };
                let mut flags = Vec::new();
                let mut vanished = Vec::new();
                for data in self.command(body).await? {
                    match data {
                        Data::Fetch { items, .. } => flags.push(flag_state(items.into_iter())),
                        Data::Vanished { known_uids, .. } => {
                            vanished.extend(uid_ranges(&known_uids));
                        }
                        _ => {}
                    }
                }
                flags.retain(|f| (first..=last).contains(&f.uid));
                flags.sort_by_key(|f| f.uid);
                return Ok(FlagChanges {
                    flags,
                    vanished: Some(vanished),
                });
            }
            Some(modseq) if self.has(&Capability::CondStore) => {
                let modifiers = vec![FetchModifier::ChangedSince(modseq)];
                self.uid_fetch_range(first, Some(last), items, modifiers)
                    .await?
            }
            _ => {
                self.uid_fetch_range(first, Some(last), items, Vec::new())
                    .await?
            }
        };
        let mut flags: Vec<FlagState> = fetched
            .into_iter()
            .map(|items| flag_state(items.into_iter()))
            .collect();
        if self.is_gmail() && !flags.is_empty() {
            let uids: Vec<u32> = flags.iter().map(|f| f.uid).collect();
            let states = flags.iter_mut().map(|f| (f.uid, &mut f.flags));
            self.add_gmail_labels(&uid_set(&uids), states).await?;
        }
        Ok(FlagChanges {
            flags,
            vanished: None,
        })
    }

    async fn uids(&mut self) -> Result<Vec<u32>> {
        let opts = ImapMessageSearchOptions { uid: true };
        let mut uids: Vec<u32> = self
            .run(ImapMessageSearch::new(Vec1::from(SearchKey::All), opts))
            .await?
            .into_iter()
            .map(|uid| uid.get())
            .collect();
        uids.sort_unstable();
        Ok(uids)
    }

    async fn status(&mut self, folder: &str) -> Result<FolderStatus> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let mut item_names = vec![
            StatusDataItemName::Messages,
            StatusDataItemName::UidNext,
            StatusDataItemName::UidValidity,
        ];
        if self.has(&Capability::CondStore) {
            item_names.push(StatusDataItemName::HighestModSeq);
        }
        let data = self
            .command(CommandBody::Status {
                mailbox,
                item_names: item_names.into(),
            })
            .await?;
        let items = data
            .into_iter()
            .find_map(|data| match data {
                Data::Status { items, .. } => Some(items),
                _ => None,
            })
            .ok_or_else(|| protocol(format!("no STATUS for {folder}")))?;
        let mut status = FolderStatus::default();
        for item in items.iter() {
            match item {
                StatusDataItem::Messages(n) => status.exists = *n,
                StatusDataItem::UidNext(n) => status.uid_next = Some(n.get()),
                StatusDataItem::UidValidity(n) => status.uid_validity = Some(n.get()),
                StatusDataItem::HighestModSeq(n) => status.highest_modseq = Some(*n),
                _ => {}
            }
        }
        Ok(status)
    }

    async fn store_flags(&mut self, uids: &[u32], flags: &Flags, add: bool) -> Result<()> {
        let mut flags = flags.clone();
        if self.is_gmail() && !uids.is_empty() {
            // Gmail keeps importance and mutes as labels.
            for (keyword, label) in [(IMPORTANT, "\\Important"), (MUTED, "\\Muted")] {
                if !flags
                    .keywords
                    .iter()
                    .any(|k| k.eq_ignore_ascii_case(keyword))
                {
                    continue;
                }
                flags.keywords.retain(|k| !k.eq_ignore_ascii_case(keyword));
                let sign = if add { '+' } else { '-' };
                self.raw_command(&format!(
                    "UID STORE {} {sign}X-GM-LABELS ({label})",
                    uid_set(uids)
                ))
                .await?;
            }
        }
        let flags = imap_flags(&flags)?;
        if uids.is_empty() || flags.is_empty() {
            return Ok(());
        }
        let set = SequenceSet::try_from(uid_set(uids).as_str()).map_err(protocol)?;
        let kind = if add {
            StoreType::Add
        } else {
            StoreType::Remove
        };
        let opts = ImapMessageStoreOptions { uid: true };
        self.run(ImapMessageStoreSilent::new(set, kind, flags, opts))
            .await
    }

    async fn move_messages(&mut self, uids: &[u32], folder: &str) -> Result<Vec<(u32, u32)>> {
        if uids.is_empty() {
            return Ok(Vec::new());
        }
        let set = SequenceSet::try_from(uid_set(uids).as_str()).map_err(protocol)?;
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let copied = if self.has(&Capability::Move) {
            let opts = ImapMessageMoveOptions { uid: true };
            self.run(ImapMessageMove::new(set, mailbox, opts)).await?
        } else {
            // RFC 6851 §3.3: COPY, then delete the originals.
            let opts = ImapMessageCopyOptions { uid: true };
            let copied = self.run(ImapMessageCopy::new(set, mailbox, opts)).await?;
            self.expunge(uids).await?;
            copied
        };
        // COPYUID lists source and destination UIDs in the same order.
        Ok(copied
            .map(|(_, from, to)| from.into_iter().zip(to).collect())
            .unwrap_or_default())
    }

    async fn copy_messages(&mut self, uids: &[u32], folder: &str) -> Result<Vec<(u32, u32)>> {
        if uids.is_empty() {
            return Ok(Vec::new());
        }
        let set = SequenceSet::try_from(uid_set(uids).as_str()).map_err(protocol)?;
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let opts = ImapMessageCopyOptions { uid: true };
        let copied = self.run(ImapMessageCopy::new(set, mailbox, opts)).await?;
        Ok(copied
            .map(|(_, from, to)| from.into_iter().zip(to).collect())
            .unwrap_or_default())
    }

    async fn expunge(&mut self, uids: &[u32]) -> Result<()> {
        if uids.is_empty() {
            return Ok(());
        }
        let deleted = Flags {
            deleted: true,
            ..Flags::default()
        };
        self.store_flags(uids, &deleted, true).await?;
        if !self.has(&Capability::UidPlus) {
            // Plain EXPUNGE would also remove what other clients marked
            // \Deleted; leave the messages marked instead. Clients hide
            // them, and failing here would make a move copy them again.
            tracing::debug!("no UIDPLUS: leaving messages marked \\Deleted");
            return Ok(());
        }
        let set = SequenceSet::try_from(uid_set(uids).as_str()).map_err(protocol)?;
        self.run(ImapMessageExpungeUid::new(set)).await?;
        Ok(())
    }

    async fn create_folder(&mut self, folder: &str) -> Result<()> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        self.run(ImapMailboxCreate::new(mailbox.clone())).await?;
        // Clients that show only subscribed folders show it too. Not every
        // server keeps subscriptions; the folder exists either way.
        if let Err(err) = self.run(ImapMailboxSubscribe::new(mailbox)).await {
            tracing::debug!(%err, folder, "SUBSCRIBE refused");
        }
        Ok(())
    }

    async fn rename_folder(&mut self, from: &str, to: &str) -> Result<()> {
        let old = Mailbox::try_from(from.to_owned()).map_err(protocol)?;
        let new = Mailbox::try_from(to.to_owned()).map_err(protocol)?;
        self.run(ImapMailboxRename::new(old, new.clone())).await?;
        // Not every server carries the subscription over.
        if let Err(err) = self.run(ImapMailboxSubscribe::new(new)).await {
            tracing::debug!(%err, to, "SUBSCRIBE refused");
        }
        Ok(())
    }

    async fn delete_folder(&mut self, folder: &str) -> Result<()> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        self.run(ImapMailboxDelete::new(mailbox)).await
    }

    async fn gmail_label(&mut self, uids: &[u32], label: &str, add: bool) -> Result<()> {
        if !self.is_gmail() {
            return Err(Error::Rejected("the server has no Gmail labels".into()));
        }
        if uids.is_empty() {
            return Ok(());
        }
        let sign = if add { '+' } else { '-' };
        self.raw_command(&format!(
            "UID STORE {} {sign}X-GM-LABELS ({})",
            uid_set(uids),
            gmail_label_arg(label)?
        ))
        .await?;
        Ok(())
    }

    async fn append_with_flags(
        &mut self,
        folder: &str,
        message: Vec<u8>,
        flags: &Flags,
    ) -> Result<()> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let opts = ImapMessageAppendOptions {
            flags: imap_flags(flags)?,
            ..ImapMessageAppendOptions::default()
        };
        self.run(ImapMessageAppend::new(mailbox, message, opts))
            .await?;
        Ok(())
    }

    async fn poll_changes(&mut self) -> Result<Vec<FolderChange>> {
        // io-imap's `ImapNoop` throws away the untagged responses, which are
        // the whole point of a NOOP (S2 problem 1).
        let data = self.command(CommandBody::Noop).await?;
        Ok(data.iter().filter_map(folder_change).collect())
    }

    async fn gmail_search(&mut self, first: u32, query: &str) -> Result<Option<Vec<u32>>> {
        if !self.is_gmail() {
            return Ok(None);
        }
        if query.contains(['"', '\\', '\r', '\n']) {
            return Err(Error::Protocol(format!(
                "unsupported Gmail query {query:?}"
            )));
        }
        let lines = self
            .raw_command(&format!("UID SEARCH UID {first}:* X-GM-RAW \"{query}\""))
            .await?;
        let mut uids: Vec<u32> = lines
            .iter()
            .filter_map(|line| search_results(line))
            .flatten()
            // `n:*` also matches the last message when every UID is lower.
            .filter(|&uid| uid >= first)
            .collect();
        uids.sort_unstable();
        uids.dedup();
        Ok(Some(uids))
    }

    async fn gmail_message_ids(&mut self, uids: &[u32]) -> Result<Option<HashMap<u32, u64>>> {
        if !self.is_gmail() {
            return Ok(None);
        }
        if uids.is_empty() {
            return Ok(Some(HashMap::new()));
        }
        let lines = self
            .raw_command(&format!("UID FETCH {} (UID X-GM-MSGID)", uid_set(uids)))
            .await?;
        Ok(Some(
            lines
                .iter()
                .filter_map(|line| gmail_fetch(line))
                .filter_map(|(uid, ids)| Some((uid, ids.message?)))
                .collect(),
        ))
    }

    async fn quota(&mut self) -> Result<Option<Quota>> {
        if !self.has_quota() {
            return Ok(None);
        }
        let lines = self.raw_command("GETQUOTAROOT INBOX").await?;
        Ok(lines.iter().find_map(|line| storage_quota(line)))
    }

    async fn wait_for_changes<I>(
        &mut self,
        max_wait: Duration,
        interrupt: I,
    ) -> Result<Wait<I::Output>>
    where
        I: Future + Send,
        I::Output: Send,
    {
        if self.has(&Capability::Idle) {
            self.idle(max_wait, interrupt).await
        } else {
            self.wait_then_poll(max_wait, interrupt).await
        }
    }

    async fn logout(mut self) -> Result<()> {
        // The server answers with BYE and then a tagged OK, so this does not
        // go through `send`, which treats BYE as an error.
        let command = Command {
            tag: self.tags.generate(),
            body: CommandBody::Logout,
        };
        let out = self
            .run(ImapSend::new(CommandCodec::new(), command))
            .await?;
        // `ImapSend` may stop at the BYE, before the tagged OK arrives.
        if out.bye.is_none() {
            expect_ok("LOGOUT", &out)?;
        }
        self.conn.close().await
    }
}

/// Decodes complete responses pushed during IDLE into `changes`.
fn decode_pushed(
    frag: &mut Fragmentizer,
    codec: &ResponseCodec,
    changes: &mut Vec<FolderChange>,
) -> Result<()> {
    while let Some(info) = frag.progress() {
        if !matches!(info, FragmentInfo::Line { .. }) || !frag.is_message_complete() {
            continue;
        }
        match frag.decode_message(codec) {
            Ok(Response::Data(data)) => changes.extend(folder_change(&data)),
            Ok(Response::Status(Status::Bye(bye))) => {
                return Err(Error::Closed(format!("server said BYE: {}", bye.text)));
            }
            // Keep-alive status lines ("* OK Still here") and anything else.
            Ok(_) => {}
            Err(
                DecodeMessageError::DecodingFailure(_)
                | DecodeMessageError::DecodingRemainder { .. },
            ) => {
                // Like `ImapSend`: skip what we cannot parse rather than drop
                // the connection over one odd line.
                tracing::debug!("skipping an undecodable response during IDLE");
            }
            Err(err) => return Err(Error::Protocol(format!("during IDLE: {err:?}"))),
        }
    }
    Ok(())
}

fn expect_ok<T: Encoder>(name: &str, out: &ImapSendOutput<T>) -> Result<()> {
    match &out.tagged {
        Some(tagged) if tagged.body.kind == StatusKind::Ok => Ok(()),
        Some(tagged) => Err(Error::Rejected(format!("{name}: {}", tagged.body.text))),
        None => Err(Error::Protocol(format!("{name}: no tagged response"))),
    }
}

fn session_error(err: ImapSessionOpenError) -> Error {
    match err {
        ImapSessionOpenError::Login(_)
        | ImapSessionOpenError::AuthPlain(_)
        | ImapSessionOpenError::AuthLogin(_)
        | ImapSessionOpenError::AuthXoauth2(_)
        | ImapSessionOpenError::AuthOauthbearer(_) => Error::Auth(err.to_string()),
        err => Error::Protocol(err.to_string()),
    }
}

impl From<ImapSendError> for Error {
    fn from(err: ImapSendError) -> Self {
        match err {
            ImapSendError::Eof => Error::Closed("server closed the connection".into()),
            err => Error::Protocol(err.to_string()),
        }
    }
}

/// io-imap gives every command its own error enum with the same shape.
macro_rules! command_errors {
    ($($ty:ident),* $(,)?) => {$(
        impl From<$ty> for Error {
            fn from(err: $ty) -> Self {
                match err {
                    $ty::No(_) | $ty::Bad(_) => Error::Rejected(err.to_string()),
                    $ty::Bye(text) => Error::Closed(format!("server said BYE: {text}")),
                    $ty::Send(err) => err.into(),
                    err => Error::Protocol(err.to_string()),
                }
            }
        }
    )*};
}

command_errors!(
    ImapMailboxListError,
    ImapMailboxSelectError,
    ImapMessageFetchError,
    ImapMailboxCreateError,
    ImapMailboxRenameError,
    ImapMailboxDeleteError,
    ImapMailboxSubscribeError,
    ImapMessageAppendError,
    ImapMessageSearchError,
    ImapMessageStoreError,
    ImapMessageCopyError,
    ImapMessageMoveError,
    ImapMessageExpungeUidError,
);

fn protocol(err: impl Display) -> Error {
    Error::Protocol(err.to_string())
}

fn mailbox_name(mailbox: &Mailbox<'_>) -> String {
    match mailbox {
        Mailbox::Inbox => "INBOX".to_owned(),
        Mailbox::Other(other) => String::from_utf8_lossy(other.inner().as_ref()).into_owned(),
    }
}

fn folder_role(attribute: &FlagNameAttribute<'_>) -> Option<FolderRole> {
    let FlagNameAttribute::Extension(_) = attribute else {
        return None;
    };
    Some(match attribute.to_string().to_ascii_lowercase().as_str() {
        "\\all" => FolderRole::All,
        "\\archive" => FolderRole::Archive,
        "\\drafts" => FolderRole::Drafts,
        "\\flagged" => FolderRole::Flagged,
        "\\junk" => FolderRole::Junk,
        "\\sent" => FolderRole::Sent,
        "\\trash" => FolderRole::Trash,
        _ => return None,
    })
}

fn envelope(items: impl IntoIterator<Item = MessageDataItem<'static>>) -> Envelope {
    let mut out = Envelope::default();
    for item in items {
        match item {
            MessageDataItem::Uid(uid) => out.uid = uid.get(),
            MessageDataItem::Rfc822Size(size) => out.size = size,
            MessageDataItem::Flags(flags) => out.flags = convert_flags(&flags),
            MessageDataItem::Envelope(e) => {
                out.date = text(&e.date);
                out.subject = text(&e.subject);
                out.from = addresses(&e.from);
                out.to = addresses(&e.to);
                out.cc = addresses(&e.cc);
                out.message_id = text(&e.message_id);
                out.in_reply_to = text(&e.in_reply_to);
            }
            _ => {}
        }
    }
    out
}

/// IMAP flags for the set ones in `flags`.
fn imap_flags(flags: &Flags) -> Result<Vec<Flag<'static>>> {
    let mut out = Vec::new();
    for (on, flag) in [
        (flags.seen, Flag::Seen),
        (flags.answered, Flag::Answered),
        (flags.flagged, Flag::Flagged),
        (flags.deleted, Flag::Deleted),
        (flags.draft, Flag::Draft),
    ] {
        if on {
            out.push(flag);
        }
    }
    for keyword in &flags.keywords {
        let atom = Atom::try_from(keyword.clone()).map_err(protocol)?;
        out.push(Flag::keyword(atom));
    }
    Ok(out)
}

/// A Gmail label (its folder path) as an `X-GM-LABELS` argument: in
/// modified UTF-7, like folder names on the wire, and quoted.
fn gmail_label_arg(label: &str) -> Result<String> {
    use base64::Engine as _;
    if label.is_empty() || label.contains(['\r', '\n', '\0']) {
        return Err(Error::Protocol(format!("unsupported label {label:?}")));
    }
    let mut encoded = String::with_capacity(label.len());
    let mut shifted: Vec<u8> = Vec::new();
    let flush = |shifted: &mut Vec<u8>, out: &mut String| {
        if !shifted.is_empty() {
            let b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&*shifted);
            out.push('&');
            out.push_str(&b64.replace('/', ","));
            out.push('-');
            shifted.clear();
        }
    };
    for c in label.chars() {
        if (' '..='~').contains(&c) {
            flush(&mut shifted, &mut encoded);
            match c {
                '&' => encoded.push_str("&-"),
                c => encoded.push(c),
            }
        } else {
            let mut units = [0u16; 2];
            for unit in c.encode_utf16(&mut units) {
                shifted.extend_from_slice(&unit.to_be_bytes());
            }
        }
    }
    flush(&mut shifted, &mut encoded);
    Ok(format!(
        "\"{}\"",
        encoded.replace('\\', "\\\\").replace('"', "\\\"")
    ))
}

/// A compact UID set: runs of consecutive UIDs become `a:b`.
fn uid_set(uids: &[u32]) -> String {
    let mut sorted = uids.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut runs: Vec<(u32, u32)> = Vec::new();
    for uid in sorted {
        match runs.last_mut() {
            Some((_, end)) if uid == *end + 1 => *end = uid,
            _ => runs.push((uid, uid)),
        }
    }
    let parts: Vec<String> = runs
        .into_iter()
        .map(|(start, end)| {
            if start == end {
                start.to_string()
            } else {
                format!("{start}:{end}")
            }
        })
        .collect();
    parts.join(",")
}

fn headers(items: impl IntoIterator<Item = MessageDataItem<'static>>) -> MessageHeaders {
    let mut out = MessageHeaders::default();
    for item in items {
        match item {
            MessageDataItem::Uid(uid) => out.uid = uid.get(),
            MessageDataItem::Rfc822Size(size) => out.size = size,
            MessageDataItem::Flags(flags) => out.flags = convert_flags(&flags),
            MessageDataItem::InternalDate(date) => out.received = Some(date.as_ref().timestamp()),
            MessageDataItem::BodyExt { data, .. } => {
                out.header = data.0.map(|d| d.as_ref().to_vec()).unwrap_or_default();
            }
            _ => {}
        }
    }
    out
}

/// Collects the attachments of `structure`, whose body section is `part`
/// (`""` for the whole message). Parts of an attached message stay inside
/// it: the attached message is the attachment.
fn attachment_parts(
    structure: &BodyStructure<'_>,
    part: &str,
    alternative: bool,
    out: &mut Vec<AttachmentPart>,
) {
    match structure {
        BodyStructure::Multi {
            bodies, subtype, ..
        } => {
            let alternative = istring(subtype).eq_ignore_ascii_case("alternative");
            for (i, body) in bodies.as_ref().iter().enumerate() {
                let child = match part {
                    "" => (i + 1).to_string(),
                    _ => format!("{part}.{}", i + 1),
                };
                attachment_parts(body, &child, alternative, out);
            }
        }
        BodyStructure::Single {
            body,
            extension_data,
        } => {
            let (kind, subtype) = match &body.specific {
                SpecificFields::Basic { r#type, subtype } => (istring(r#type), istring(subtype)),
                SpecificFields::Message { .. } => ("message".into(), "rfc822".into()),
                SpecificFields::Text { subtype, .. } => ("text".into(), istring(subtype)),
            };
            let mime = format!("{kind}/{subtype}").to_ascii_lowercase();
            let disposition = extension_data
                .as_ref()
                .and_then(|ext| ext.tail.as_ref())
                .and_then(|tail| tail.disposition.as_ref());
            let pairs = |list: &[(IString<'_>, IString<'_>)]| -> Vec<(String, String)> {
                list.iter().map(|(k, v)| (istring(k), istring(v))).collect()
            };
            let disposition_params = disposition.map(|(_, p)| pairs(p)).unwrap_or_default();
            let type_params = pairs(&body.basic.parameter_list);
            let filename = katna_import::mime::part_filename(
                &borrowed(&disposition_params),
                &borrowed(&type_params),
            );
            let kind = disposition.map(|(kind, _)| istring(kind));
            let info = katna_import::mime::PartInfo {
                mime: &mime,
                disposition: kind.as_deref(),
                content_id: body.basic.id.0.is_some(),
                filename: filename.is_some(),
                alternative,
            };
            if !katna_import::mime::is_attachment(&info) {
                return;
            }
            let encoded = u64::from(body.basic.size);
            let base64 =
                istring(&body.basic.content_transfer_encoding).eq_ignore_ascii_case("base64");
            out.push(AttachmentPart {
                part: if part.is_empty() {
                    "1".into()
                } else {
                    part.into()
                },
                mime,
                filename,
                size: if base64 { encoded * 3 / 4 } else { encoded },
            });
        }
    }
}

fn borrowed(list: &[(String, String)]) -> Vec<(&str, &str)> {
    list.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect()
}

fn istring(value: &IString<'_>) -> String {
    String::from_utf8_lossy(value.as_ref()).into_owned()
}

/// `12 FETCH (X-GM-THRID 1278455344230334865 UID 4)` → `(4, 1278…)`.
/// A storage limit from one pebibyte (in KiB) up means no limit.
const UNLIMITED_KIB: u64 = 1 << 40;

/// The storage limit of an untagged `QUOTA root (STORAGE used limit …)`
/// response, whose numbers count units of 1024 bytes. `None` for other
/// responses and for roots without a storage limit.
fn storage_quota(line: &str) -> Option<Quota> {
    let rest = line
        .get(..6)?
        .eq_ignore_ascii_case("QUOTA ")
        .then(|| &line[6..])?;
    // The root may be a quoted string with spaces; the resources follow
    // it in the last parentheses.
    let list = rest.get(rest.rfind('(')? + 1..rest.rfind(')')?)?;
    let words: Vec<&str> = list.split_ascii_whitespace().collect();
    words
        .as_chunks::<3>()
        .0
        .iter()
        .find_map(|[name, used, limit]| {
            if !name.eq_ignore_ascii_case("STORAGE") {
                return None;
            }
            let used: u64 = used.parse().ok()?;
            let limit: u64 = limit.parse().ok()?;
            // Unlimited plans report a huge limit instead (a Google
            // Workspace account sent 7 × 2⁵⁰ KiB); no mailbox has a
            // pebibyte.
            (limit > 0 && limit < UNLIMITED_KIB).then(|| Quota {
                used: used.saturating_mul(1024),
                limit: limit.saturating_mul(1024),
            })
        })
}

/// Gmail's `X-GM-THRID` and `X-GM-MSGID` of one message.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct GmailIds {
    thread: Option<u64>,
    message: Option<u64>,
}

fn gmail_fetch(line: &str) -> Option<(u32, GmailIds)> {
    let (_, rest) = line.split_once(' ')?;
    let items = rest
        .strip_prefix("FETCH")
        .or_else(|| rest.strip_prefix("fetch"))?;
    let items = items.trim().strip_prefix('(')?.strip_suffix(')')?;
    let mut words = items.split_whitespace();
    let mut uid = None;
    let mut ids = GmailIds::default();
    while let Some(name) = words.next() {
        let value = words.next()?;
        if name.eq_ignore_ascii_case("UID") {
            uid = value.parse().ok();
        } else if name.eq_ignore_ascii_case("X-GM-THRID") {
            ids.thread = value.parse().ok();
        } else if name.eq_ignore_ascii_case("X-GM-MSGID") {
            ids.message = value.parse().ok();
        }
    }
    (ids != GmailIds::default()).then_some((uid?, ids))
}

/// Gmail system labels of one message that Katna keeps as flags.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct GmailLabels {
    important: bool,
    muted: bool,
}

/// `5 FETCH (UID 9 X-GM-LABELS (\Inbox "\\Important" Work))` → `(9,
/// important)`: the Important and Muted labels of the message at that UID.
fn gmail_labels(line: &str) -> Option<(u32, GmailLabels)> {
    let (_, rest) = line.split_once(' ')?;
    let items = rest
        .strip_prefix("FETCH")
        .or_else(|| rest.strip_prefix("fetch"))?;
    let items = items.trim().strip_prefix('(')?.strip_suffix(')')?;
    let mut tokens = fetch_tokens(items).into_iter();
    let (mut uid, mut labels) = (None, None);
    while let Some(token) = tokens.next() {
        let FetchToken::Word(name) = token else {
            continue;
        };
        match tokens.next() {
            Some(FetchToken::Word(value)) if name.eq_ignore_ascii_case("UID") => {
                uid = value.parse().ok();
            }
            Some(FetchToken::List(list)) if name.eq_ignore_ascii_case("X-GM-LABELS") => {
                let has = |label: &str| list.iter().any(|l| l.eq_ignore_ascii_case(label));
                labels = Some(GmailLabels {
                    important: has("\\Important"),
                    muted: has("\\Muted"),
                });
            }
            _ => {}
        }
    }
    Some((uid?, labels?))
}

/// A word (atom or unquoted string) or a list of words in a FETCH line.
#[derive(Debug, PartialEq, Eq)]
enum FetchToken {
    Word(String),
    List(Vec<String>),
}

/// Splits the items of a FETCH line into words and one-level lists.
fn fetch_tokens(text: &str) -> Vec<FetchToken> {
    let mut out = Vec::new();
    let mut list: Option<Vec<String>> = None;
    let mut chars = text.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' => {
                chars.next();
            }
            '(' => {
                chars.next();
                list = Some(Vec::new());
            }
            ')' => {
                chars.next();
                if let Some(words) = list.take() {
                    out.push(FetchToken::List(words));
                }
            }
            _ => {
                let mut word = String::new();
                if c == '"' {
                    chars.next();
                    while let Some(c) = chars.next() {
                        match c {
                            '\\' => word.extend(chars.next()),
                            '"' => break,
                            c => word.push(c),
                        }
                    }
                } else {
                    while let Some(&c) = chars.peek() {
                        if matches!(c, ' ' | '(' | ')') {
                            break;
                        }
                        word.push(c);
                        chars.next();
                    }
                }
                match &mut list {
                    Some(words) => words.push(word),
                    None => out.push(FetchToken::Word(word)),
                }
            }
        }
    }
    out
}

/// `SEARCH 3 5 8` (maybe followed by `(MODSEQ 99)`) → `[3, 5, 8]`.
fn search_results(line: &str) -> Option<Vec<u32>> {
    let (name, rest) = line.split_once(' ').unwrap_or((line, ""));
    if !name.eq_ignore_ascii_case("SEARCH") {
        return None;
    }
    Some(
        rest.split_whitespace()
            .take_while(|word| !word.starts_with('('))
            .filter_map(|word| word.parse().ok())
            .collect(),
    )
}

fn text(value: &NString<'_>) -> Option<String> {
    value
        .0
        .as_ref()
        .map(|s| String::from_utf8_lossy(s.as_ref()).into_owned())
}

fn addresses(list: &[ImapAddress<'_>]) -> Vec<Address> {
    list.iter()
        .filter_map(|a| {
            // RFC 3501 group syntax markers carry no host.
            let email = format!("{}@{}", text(&a.mailbox)?, text(&a.host)?);
            Some(Address {
                name: text(&a.name),
                email,
            })
        })
        .collect()
}

fn convert_flags(flags: &[FlagFetch<'_>]) -> Flags {
    let mut out = Flags::default();
    for flag in flags {
        match flag {
            FlagFetch::Flag(Flag::Seen) => out.seen = true,
            FlagFetch::Flag(Flag::Answered) => out.answered = true,
            FlagFetch::Flag(Flag::Flagged) => out.flagged = true,
            FlagFetch::Flag(Flag::Deleted) => out.deleted = true,
            FlagFetch::Flag(Flag::Draft) => out.draft = true,
            FlagFetch::Flag(other) => out.keywords.push(other.to_string()),
            FlagFetch::Recent => {}
        }
    }
    out
}

fn flag_state<'a>(items: impl Iterator<Item = MessageDataItem<'a>>) -> FlagState {
    let mut state = FlagState {
        uid: 0,
        flags: Flags::default(),
    };
    for item in items {
        match item {
            MessageDataItem::Uid(uid) => state.uid = uid.get(),
            MessageDataItem::Flags(flags) => state.flags = convert_flags(&flags),
            _ => {}
        }
    }
    state
}

/// The UID ranges of a `VANISHED` response (it never uses `*`).
fn uid_ranges(set: &SequenceSet) -> Vec<RangeInclusive<u32>> {
    let value = |v: &SeqOrUid| match v {
        SeqOrUid::Value(n) => Some(n.get()),
        SeqOrUid::Asterisk => None,
    };
    set.0
        .as_ref()
        .iter()
        .filter_map(|seq| match seq {
            Sequence::Single(v) => value(v).map(|n| n..=n),
            Sequence::Range(a, b) => {
                let (a, b) = (value(a)?, value(b)?);
                Some(a.min(b)..=a.max(b))
            }
        })
        .collect()
}

fn folder_change(data: &Data<'_>) -> Option<FolderChange> {
    match data {
        Data::Vanished { known_uids, .. } => Some(FolderChange::Vanished(uid_ranges(known_uids))),
        Data::Exists(n) => Some(FolderChange::Exists(*n)),
        Data::Expunge(seq) => Some(FolderChange::Expunged(seq.get())),
        Data::Fetch { seq, items } => items.as_ref().iter().find_map(|item| match item {
            MessageDataItem::Flags(flags) => Some(FolderChange::FlagsChanged {
                seq: seq.get(),
                flags: convert_flags(flags),
            }),
            _ => None,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uid_sets_are_compact() {
        assert_eq!(uid_set(&[7]), "7");
        assert_eq!(uid_set(&[5, 1, 2, 3, 9, 10, 3]), "1:3,5,9:10");
    }

    #[test]
    fn gmail_labels_are_quoted_in_modified_utf7() {
        assert_eq!(gmail_label_arg("Work/2026").unwrap(), "\"Work/2026\"");
        assert_eq!(
            gmail_label_arg("Tom & \"Jo\"").unwrap(),
            r#""Tom &- \"Jo\"""#
        );
        // RFC 3501 §5.1.3's example.
        assert_eq!(
            gmail_label_arg("~peter/mail/台北/日本語").unwrap(),
            "\"~peter/mail/&U,BTFw-/&ZeVnLIqe-\""
        );
        assert!(gmail_label_arg("a\r\nb").is_err());
    }

    #[test]
    fn parses_gmail_labels() {
        let gmail_important = |line| gmail_labels(line).map(|(uid, l)| (uid, l.important));
        assert_eq!(
            gmail_labels(r"8 FETCH (UID 13 X-GM-LABELS (\Inbox \Muted))"),
            Some((
                13,
                GmailLabels {
                    important: false,
                    muted: true
                }
            ))
        );
        assert_eq!(
            gmail_important(r#"5 FETCH (UID 9 X-GM-LABELS (\Inbox "\\Important" Work))"#),
            Some((9, true))
        );
        assert_eq!(
            gmail_important(r"5 FETCH (X-GM-LABELS (\Inbox \Important) UID 10)"),
            Some((10, true))
        );
        // A label the user named Important is not Gmail's own.
        assert_eq!(
            gmail_important(r#"6 FETCH (X-GM-LABELS (Important "My \"label\"") UID 11)"#),
            Some((11, false))
        );
        assert_eq!(
            gmail_important("7 FETCH (X-GM-LABELS () UID 12)"),
            Some((12, false))
        );
        assert_eq!(gmail_important("7 FETCH (UID 12 FLAGS (\\Seen))"), None);
        assert_eq!(
            fetch_tokens(r#"UID 3 X-GM-LABELS ("a b" c)"#),
            [
                FetchToken::Word("UID".into()),
                FetchToken::Word("3".into()),
                FetchToken::Word("X-GM-LABELS".into()),
                FetchToken::List(vec!["a b".into(), "c".into()]),
            ]
        );
    }

    #[test]
    fn parses_gmail_responses() {
        assert_eq!(
            gmail_fetch("12 FETCH (X-GM-THRID 1278455344230334865 UID 4)"),
            Some((
                4,
                GmailIds {
                    thread: Some(1_278_455_344_230_334_865),
                    message: None
                }
            ))
        );
        assert_eq!(
            gmail_fetch(
                "1 FETCH (UID 9 X-GM-THRID 18446744073709551614 X-GM-MSGID 1278455344230334866)"
            ),
            Some((
                9,
                GmailIds {
                    thread: Some(u64::MAX - 1),
                    message: Some(1_278_455_344_230_334_866)
                }
            ))
        );
        assert_eq!(gmail_fetch("1 FETCH (UID 9 FLAGS (\\Seen))"), None);
        assert_eq!(gmail_fetch("3 EXISTS"), None);
        assert_eq!(search_results("SEARCH 3 5 8"), Some(vec![3, 5, 8]));
        assert_eq!(search_results("SEARCH 3 (MODSEQ 9)"), Some(vec![3]));
        assert_eq!(search_results("SEARCH"), Some(vec![]));
        assert_eq!(search_results("3 EXISTS"), None);
    }

    #[test]
    fn quota_responses() {
        assert_eq!(
            storage_quota("QUOTA \"\" (STORAGE 10 512)"),
            Some(Quota {
                used: 10 * 1024,
                limit: 512 * 1024
            })
        );
        assert_eq!(
            storage_quota("QUOTA \"User quota\" (MESSAGE 3 1000 STORAGE 7 9)"),
            Some(Quota {
                used: 7 * 1024,
                limit: 9 * 1024
            })
        );
        assert_eq!(storage_quota("QUOTA \"\" (MESSAGE 3 1000)"), None);
        assert_eq!(storage_quota("QUOTAROOT INBOX \"\""), None);
        assert_eq!(storage_quota("QUOTA \"\" (STORAGE 1 0)"), None);
        assert_eq!(
            storage_quota("QUOTA \"\" (STORAGE 12 7881299347898368)"),
            None
        );
    }
}
