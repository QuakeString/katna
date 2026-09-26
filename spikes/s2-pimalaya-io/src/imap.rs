// SPDX-License-Identifier: GPL-3.0-or-later

//! IMAP backend: Pimalaya `io-imap` coroutines driven by [`Conn`].
//! Pimalaya and imap-types names end at this module's boundary.

use std::{
    fmt::Display,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use io_imap::{
    codec::CommandCodec,
    codec::fragmentizer::Fragmentizer,
    coroutine::{ImapCoroutine, ImapCoroutineState as S, ImapYield},
    rfc2177::idle::{ImapIdle, ImapIdleOptions, ImapIdleYield},
    rfc3501::{
        append::{ImapMessageAppend, ImapMessageAppendOptions},
        create::ImapMailboxCreate,
        fetch::{ImapMessageFetch, ImapMessageFetchOptions},
        list::ImapMailboxList,
        logout::ImapLogout,
        select::{ImapMailboxSelect, ImapMailboxSelectOptions},
    },
    send::ImapSend,
    session::{
        ImapSessionOpen, ImapSessionOpenError, ImapSessionOpenOptions, ImapSessionOpenYield as O,
        ImapSessionTransport,
    },
    types::{
        command::{Command, CommandBody},
        core::{NString, TagGenerator},
        envelope::Address as ImapAddress,
        fetch::{MacroOrMessageDataItemNames, MessageDataItem, MessageDataItemName},
        flag::{Flag, FlagFetch, FlagNameAttribute},
        mailbox::{ListMailbox, Mailbox},
        response::{Data, StatusKind},
        sequence::SequenceSet,
    },
};
use io_sasl::rfc4616::plain::SaslPlainCreds;

use crate::{
    Address, Credentials, Endpoint, Envelope, Error, Flags, Folder, FolderChange, FolderRole,
    FolderStatus, MailBackend, Result, Security,
    net::{Conn, Tls},
};

/// Largest single server response we accept (a full message literal).
const MAX_RESPONSE: u32 = 64 * 1024 * 1024;

pub struct ImapBackend {
    conn: Conn,
    frag: Fragmentizer,
    tags: TagGenerator,
    capabilities: Vec<String>,
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
                        "unix sockets not supported: {path}"
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

        Ok(Self {
            conn,
            frag,
            tags: TagGenerator::new(),
            capabilities: session.capability.iter().map(|c| c.to_string()).collect(),
        })
    }

    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    /// Runs one command coroutine to completion.
    async fn run<C, T, E>(&mut self, mut co: C) -> Result<T>
    where
        C: ImapCoroutine<Yield = ImapYield, Return = std::result::Result<T, E>>,
        E: Display,
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
                S::Complete(result) => return result.map_err(|e| Error::Protocol(e.to_string())),
            };
        }
    }
}

impl MailBackend for ImapBackend {
    async fn list_folders(&mut self) -> Result<Vec<Folder>> {
        let reference = Mailbox::try_from("").map_err(protocol)?;
        let pattern = ListMailbox::try_from("*").map_err(protocol)?;
        let listing = self.run(ImapMailboxList::new(reference, pattern)).await?;
        Ok(listing
            .into_iter()
            .map(|(mailbox, delimiter, attributes)| {
                let name = mailbox_name(&mailbox);
                let mut role = attributes.iter().find_map(folder_role);
                if matches!(mailbox, Mailbox::Inbox) {
                    role = Some(FolderRole::Inbox);
                }
                Folder {
                    name,
                    delimiter: delimiter.map(|d| d.inner()),
                    role,
                    selectable: !attributes.contains(&FlagNameAttribute::Noselect),
                }
            })
            .collect())
    }

    async fn select(&mut self, folder: &str) -> Result<FolderStatus> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let data = self
            .run(ImapMailboxSelect::new(
                mailbox,
                ImapMailboxSelectOptions::default(),
            ))
            .await?;
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
        let mut envelopes: Vec<Envelope> = fetched
            .into_values()
            .map(|items| {
                let mut envelope = Envelope::default();
                for item in items.into_iter() {
                    match item {
                        MessageDataItem::Uid(uid) => envelope.uid = uid.get(),
                        MessageDataItem::Rfc822Size(size) => envelope.size = size,
                        MessageDataItem::Flags(flags) => envelope.flags = convert_flags(&flags),
                        MessageDataItem::Envelope(e) => {
                            envelope.date = text(&e.date);
                            envelope.subject = text(&e.subject);
                            envelope.from = addresses(&e.from);
                            envelope.to = addresses(&e.to);
                            envelope.cc = addresses(&e.cc);
                            envelope.message_id = text(&e.message_id);
                            envelope.in_reply_to = text(&e.in_reply_to);
                        }
                        _ => {}
                    }
                }
                envelope
            })
            .collect();
        envelopes.sort_by_key(|e| e.uid);
        Ok(envelopes)
    }

    async fn create_folder(&mut self, folder: &str) -> Result<()> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        self.run(ImapMailboxCreate::new(mailbox)).await
    }

    async fn poll_changes(&mut self) -> Result<Vec<FolderChange>> {
        // io-imap's `ImapNoop` drops the untagged responses, which are the
        // whole point of a NOOP. Its public `ImapSend` building block keeps
        // them, so we frame the command ourselves.
        let command = Command {
            tag: self.tags.generate(),
            body: CommandBody::Noop,
        };
        let out = self
            .run(ImapSend::new(CommandCodec::new(), command))
            .await?;
        if let Some(bye) = out.bye {
            return Err(Error::Protocol(format!("BYE {}", bye.text)));
        }
        match out.tagged {
            Some(tagged) if tagged.body.kind == StatusKind::Ok => {}
            other => return Err(Error::Protocol(format!("NOOP failed: {other:?}"))),
        }
        Ok(out.data.iter().filter_map(folder_change).collect())
    }

    async fn append(&mut self, folder: &str, message: Vec<u8>) -> Result<()> {
        let mailbox = Mailbox::try_from(folder.to_owned()).map_err(protocol)?;
        let opts = ImapMessageAppendOptions::default();
        self.run(ImapMessageAppend::new(mailbox, message, opts))
            .await?;
        Ok(())
    }

    async fn wait_for_changes(&mut self, max_wait: Duration) -> Result<Vec<FolderChange>> {
        let Self { conn, frag, .. } = self;
        let done = Arc::new(AtomicBool::new(false));
        let mut co = ImapIdle::new(done.clone(), ImapIdleOptions::default());
        let deadline = Instant::now() + max_wait;
        let mut changes = Vec::new();

        let mut state = co.resume(frag, None);
        loop {
            state = match state {
                S::Yielded(ImapIdleYield::WantsWrite(bytes)) => {
                    conn.write_all(&bytes).await?;
                    co.resume(frag, None)
                }
                S::Yielded(ImapIdleYield::WantsRead) if done.load(Ordering::SeqCst) => {
                    // DONE was sent; wait for the tagged OK.
                    let bytes = conn.read().await?;
                    co.resume(frag, Some(bytes))
                }
                S::Yielded(ImapIdleYield::WantsRead) => {
                    let remaining = deadline.saturating_duration_since(Instant::now());
                    match conn.read_timeout(remaining).await? {
                        Some(bytes) => co.resume(frag, Some(bytes)),
                        None => {
                            // Our timer, not the coroutine's: without the
                            // `client` feature io-imap has no clock.
                            done.store(true, Ordering::SeqCst);
                            co.resume(frag, None)
                        }
                    }
                }
                S::Yielded(ImapIdleYield::Event(event)) => {
                    changes.extend(event.data.iter().filter_map(folder_change));
                    if !changes.is_empty() {
                        done.store(true, Ordering::SeqCst);
                    }
                    co.resume(frag, None)
                }
                S::Complete(Ok(())) => return Ok(changes),
                S::Complete(Err(err)) => return Err(Error::Protocol(err.to_string())),
            };
        }
    }

    async fn logout(mut self) -> Result<()> {
        self.run(ImapLogout::new()).await?;
        self.conn.close().await
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

fn text(value: &NString<'_>) -> Option<String> {
    value
        .0
        .as_ref()
        .map(|s| String::from_utf8_lossy(s.as_ref()).into_owned())
}

fn addresses(list: &[ImapAddress<'_>]) -> Vec<Address> {
    list.iter()
        .filter_map(|a| {
            let mailbox = text(&a.mailbox)?;
            let email = match text(&a.host) {
                Some(host) => format!("{mailbox}@{host}"),
                // RFC 3501 group syntax markers carry no host.
                None => return None,
            };
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
