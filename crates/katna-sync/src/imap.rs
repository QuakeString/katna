// SPDX-License-Identifier: GPL-3.0-or-later

//! IMAP backend: Pimalaya's `io-imap` coroutines driven by [`Conn`].
//!
//! Pimalaya and `imap-types` names end at this module's boundary. Where
//! io-imap 0.6 loses server updates, we build the command ourselves on its
//! public `ImapSend` building block (NOOP, IDLE); see
//! `docs/spikes/s2-pimalaya-io.md` for the list of problems.

use std::{
    fmt::Display,
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
    command::{Command, CommandBody, SelectParameter},
    core::{NString, TagGenerator},
    envelope::Address as ImapAddress,
    extensions::idle::IdleDone,
    fetch::{MacroOrMessageDataItemNames, MessageDataItem, MessageDataItemName},
    flag::{Flag, FlagFetch, FlagNameAttribute},
    mailbox::{ListMailbox, Mailbox},
    response::{Capability, Data, Response, Status, StatusKind},
    sequence::SequenceSet,
};
use io_imap::{
    coroutine::{ImapCoroutine, ImapCoroutineState as S, ImapYield},
    rfc3501::{
        append::{ImapMessageAppend, ImapMessageAppendError, ImapMessageAppendOptions},
        create::{ImapMailboxCreate, ImapMailboxCreateError},
        fetch::{ImapMessageFetch, ImapMessageFetchError, ImapMessageFetchOptions},
        list::{ImapMailboxList, ImapMailboxListError},
        select::{ImapMailboxSelect, ImapMailboxSelectError, ImapMailboxSelectOptions},
    },
    send::{ImapSend, ImapSendError, ImapSendOutput},
    session::{
        ImapSessionOpen, ImapSessionOpenError, ImapSessionOpenOptions, ImapSessionOpenYield as O,
        ImapSessionTransport,
    },
};
use io_sasl::rfc4616::plain::SaslPlainCreds;

use crate::{
    Address, Credentials, Endpoint, Envelope, Error, Flags, Folder, FolderChange, FolderRole,
    FolderStatus, MailBackend, Result, Security, Wait,
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
}

impl ImapBackend {
    /// Connects, negotiates TLS and logs in with SASL PLAIN.
    pub async fn connect(endpoint: &Endpoint, creds: &Credentials, tls: Tls) -> Result<Self> {
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
        let sasl = SaslPlainCreds {
            authzid: None,
            authcid: creds.user.clone(),
            passwd: creds.password.clone().into(),
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

        Ok(Self {
            conn,
            frag,
            tags: TagGenerator::new(),
            capabilities: session.capability,
        })
    }

    /// The server's capabilities after login, as IMAP spells them.
    pub fn capabilities(&self) -> Vec<String> {
        self.capabilities.iter().map(|c| c.to_string()).collect()
    }

    fn has(&self, capability: &Capability<'_>) -> bool {
        self.capabilities.contains(capability)
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
        let range = match last {
            Some(last) => format!("{first}:{last}"),
            None => format!("{first}:*"),
        };
        let set = SequenceSet::try_from(range.as_str()).map_err(protocol)?;
        let items = MacroOrMessageDataItemNames::MessageDataItemNames(vec![
            MessageDataItemName::Uid,
            MessageDataItemName::Flags,
            MessageDataItemName::Rfc822Size,
            MessageDataItemName::Envelope,
        ]);
        let opts = ImapMessageFetchOptions {
            uid: true,
            ..Default::default()
        };
        let fetched = self.run(ImapMessageFetch::new(set, items, opts)).await?;
        let mut envelopes: Vec<Envelope> = fetched.into_values().map(envelope).collect();
        envelopes.sort_by_key(|e| e.uid);
        // `first:*` always matches the last message, even below `first`.
        envelopes.retain(|e| e.uid >= first);
        Ok(envelopes)
    }

    async fn create_folder(&mut self, folder: &str) -> Result<()> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        self.run(ImapMailboxCreate::new(mailbox)).await
    }

    async fn append(&mut self, folder: &str, message: Vec<u8>) -> Result<()> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let opts = ImapMessageAppendOptions::default();
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
    ImapMessageAppendError,
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

fn folder_change(data: &Data<'_>) -> Option<FolderChange> {
    match data {
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
