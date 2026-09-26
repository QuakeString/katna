// SPDX-License-Identifier: GPL-3.0-or-later

//! The `in.invenia.katna.Pim1` interface on the session bus.

use std::sync::Arc;

use async_channel::Receiver;
use katna_core::{AccountId, ids};
use katna_dbus::{AccountStatus, NewImapAccount, NewPop3Account, OutboxItem, flag};
use katna_store::{FolderId, MessageFlags, MessageId};
use zbus::{fdo, object_server::SignalEmitter};

use crate::daemon::{CommandError, Daemon, Notice};

/// The object at `/in/invenia/katna/Pim1`.
pub struct PimService {
    daemon: Arc<Daemon>,
}

impl PimService {
    pub fn new(daemon: Arc<Daemon>) -> Self {
        Self { daemon }
    }
}

impl From<CommandError> for fdo::Error {
    fn from(err: CommandError) -> Self {
        let message = err.to_string();
        match err {
            CommandError::InvalidArgs(_) => Self::InvalidArgs(message),
            CommandError::AuthFailed(_) => Self::AuthFailed(message),
            CommandError::UnknownAccount(_)
            | CommandError::UnknownMessage(_)
            | CommandError::UnknownFolder(_) => Self::UnknownObject(message),
            CommandError::Failed(_) => Self::Failed(message),
        }
    }
}

// zbus needs the interface name as a literal; `with_dbus_names!` supplies it
// from `katna_core::ids`. Keep each method a one-line call into `Daemon`.
macro_rules! pim_interface {
    ($interface:tt, $bus_name:tt, $path:tt) => {
        #[zbus::interface(name = $interface)]
        impl PimService {
            async fn accounts(&self) -> fdo::Result<Vec<AccountStatus>> {
                Ok(self.daemon.accounts()?)
            }

            async fn add_imap_account(
                &self,
                account: NewImapAccount,
                password: String,
            ) -> fdo::Result<i64> {
                Ok(self.daemon.add_imap_account(account, password).await?.0)
            }

            async fn add_pop3_account(
                &self,
                account: NewPop3Account,
                password: String,
            ) -> fdo::Result<i64> {
                Ok(self.daemon.add_pop3_account(account, password).await?.0)
            }

            async fn discover_account(
                &self,
                address: String,
            ) -> fdo::Result<(NewImapAccount, String)> {
                let (account, source) = self.daemon.discover_account(&address).await?;
                Ok((account, source.to_owned()))
            }

            async fn set_password(&self, account: i64, password: String) -> fdo::Result<()> {
                Ok(self
                    .daemon
                    .set_password(AccountId(account), password)
                    .await?)
            }

            async fn remove_account(&self, account: i64) -> fdo::Result<bool> {
                Ok(self.daemon.remove_account(AccountId(account)).await?)
            }

            async fn sync_now(&self, account: i64) -> fdo::Result<()> {
                let account = (account != 0).then_some(AccountId(account));
                Ok(self.daemon.sync_now(account).await?)
            }

            async fn fetch_body(&self, message: i64) -> fdo::Result<()> {
                Ok(self.daemon.fetch_body(MessageId(message)).await?)
            }

            async fn set_flags(
                &self,
                messages: Vec<i64>,
                add: Vec<String>,
                remove: Vec<String>,
            ) -> fdo::Result<()> {
                let (add, remove) = (message_flags(&add)?, message_flags(&remove)?);
                Ok(self.daemon.set_flags(&ids(&messages), add, remove)?)
            }

            async fn move_messages(&self, messages: Vec<i64>, folder: i64) -> fdo::Result<()> {
                Ok(self
                    .daemon
                    .move_messages(&ids(&messages), FolderId(folder))?)
            }

            async fn delete_messages(&self, messages: Vec<i64>) -> fdo::Result<()> {
                Ok(self.daemon.delete_messages(&ids(&messages))?)
            }

            async fn archive_messages(&self, messages: Vec<i64>) -> fdo::Result<()> {
                Ok(self.daemon.archive_messages(&ids(&messages))?)
            }

            async fn queue_send(
                &self,
                account: i64,
                message: Vec<u8>,
                delay: u32,
            ) -> fdo::Result<i64> {
                Ok(self
                    .daemon
                    .queue_send(AccountId(account), &message, delay)?)
            }

            async fn undo_send(&self, id: i64) -> fdo::Result<bool> {
                Ok(self.daemon.undo_send(id)?)
            }

            async fn discard_send(&self, id: i64) -> fdo::Result<bool> {
                Ok(self.daemon.discard_send(id)?)
            }

            async fn outbox(&self) -> fdo::Result<Vec<OutboxItem>> {
                Ok(self.daemon.outbox()?)
            }

            #[zbus(signal)]
            async fn accounts_changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn sync_status_changed(
                emitter: &SignalEmitter<'_>,
                account: i64,
            ) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn mail_changed(emitter: &SignalEmitter<'_>, account: i64) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn outbox_changed(emitter: &SignalEmitter<'_>, id: i64) -> zbus::Result<()>;
        }
    };
}

katna_core::with_dbus_names!(pim_interface);

fn ids(messages: &[i64]) -> Vec<MessageId> {
    messages.iter().copied().map(MessageId).collect()
}

/// Flag names from [`katna_dbus::flag`] as bits.
fn message_flags(names: &[String]) -> Result<MessageFlags, CommandError> {
    let mut flags = MessageFlags::empty();
    for name in names {
        let bit = match name.as_str() {
            flag::SEEN => MessageFlags::SEEN,
            flag::ANSWERED => MessageFlags::ANSWERED,
            flag::FLAGGED => MessageFlags::FLAGGED,
            flag::DRAFT => MessageFlags::DRAFT,
            flag::FORWARDED => MessageFlags::FORWARDED,
            other => {
                return Err(CommandError::InvalidArgs(format!(
                    "unknown flag {other:?} (seen, answered, flagged, draft or forwarded)"
                )));
            }
        };
        flags.set(bit, true);
    }
    Ok(flags)
}

/// Sends a signal for every notice until the daemon goes away.
pub async fn emit_signals(connection: zbus::Connection, notices: Receiver<Notice>) {
    let emitter = match SignalEmitter::new(&connection, ids::PIM_OBJECT_PATH) {
        Ok(emitter) => emitter,
        Err(err) => {
            tracing::error!(%err, "no signal emitter");
            return;
        }
    };
    while let Ok(notice) = notices.recv().await {
        let sent = match notice {
            Notice::AccountsChanged => PimService::accounts_changed(&emitter).await,
            Notice::StatusChanged(id) => PimService::sync_status_changed(&emitter, id.0).await,
            Notice::MailChanged(id) => PimService::mail_changed(&emitter, id.0).await,
            Notice::OutboxChanged(id) => PimService::outbox_changed(&emitter, id).await,
        };
        if let Err(err) = sent {
            tracing::warn!(%err, ?notice, "could not send a signal");
        }
    }
}
