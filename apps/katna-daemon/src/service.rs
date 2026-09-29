// SPDX-License-Identifier: GPL-3.0-or-later

//! The `in.invenia.katna.Pim1` interface on the session bus.

use std::sync::Arc;

use async_channel::Receiver;
use katna_core::{AccountId, ids};
use katna_dbus::{
    AccountStatus, DriveUpload, KatnaAccount, KatnaDevice, NewImapAccount, NewPop3Account,
    NoteItem, OutboxItem, TemplateItem, UpdateStatus, flag,
};
use katna_store::{FolderId, MessageFlags, MessageId};
use zbus::{fdo, object_server::SignalEmitter};

use crate::daemon::{CommandError, Daemon, Notice};
use crate::translate::TranslateError;

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

/// The [`katna_dbus::translate_problem`] of a failed translation.
fn problem(err: &TranslateError) -> &'static str {
    use katna_dbus::translate_problem as p;
    match err {
        TranslateError::Off => p::OFF,
        TranslateError::SameLanguage => p::SAME_LANGUAGE,
        TranslateError::Unsupported(..) => p::UNSUPPORTED,
        TranslateError::TooMany => p::TOO_MANY,
        TranslateError::SignIn => p::SIGN_IN,
        TranslateError::Server(err) => {
            tracing::info!(%err, "translation failed");
            p::FAILED
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
            ) -> fdo::Result<(NewImapAccount, String, String, bool)> {
                let (account, found) = self.daemon.discover_account(&address).await?;
                let sign_in = found
                    .oauth
                    .map(|p| p.as_str().to_owned())
                    .unwrap_or_default();
                Ok((
                    account,
                    found.source.as_str().to_owned(),
                    sign_in,
                    found.password,
                ))
            }

            async fn sign_in(
                &self,
                provider: String,
                account: i64,
                address: String,
            ) -> fdo::Result<i64> {
                let provider = provider.parse().map_err(fdo::Error::InvalidArgs)?;
                let account = (account != 0).then_some(AccountId(account));
                Ok(self.daemon.sign_in(provider, account, &address).await?.0)
            }

            async fn cancel_sign_in(&self) -> fdo::Result<bool> {
                Ok(self.daemon.cancel_sign_in())
            }

            async fn set_password(&self, account: i64, password: String) -> fdo::Result<()> {
                Ok(self
                    .daemon
                    .set_password(AccountId(account), password)
                    .await?)
            }

            async fn rename_account(&self, account: i64, name: String) -> fdo::Result<()> {
                Ok(self.daemon.rename_account(AccountId(account), &name)?)
            }

            async fn remove_account(&self, account: i64) -> fdo::Result<bool> {
                Ok(self.daemon.remove_account(AccountId(account)).await?)
            }

            async fn delete_all_data(&self) -> fdo::Result<()> {
                Ok(self.daemon.delete_all_data().await?)
            }

            async fn reset_cache(&self) -> fdo::Result<(u64, u64)> {
                let forgotten = self.daemon.reset_cache().await?;
                Ok((forgotten.messages as u64, forgotten.bytes))
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

            async fn set_pinned(&self, messages: Vec<i64>, on: bool) -> fdo::Result<()> {
                Ok(self.daemon.set_pinned(&ids(&messages), on)?)
            }

            async fn create_folder(
                &self,
                account: i64,
                name: &str,
                parent: i64,
            ) -> fdo::Result<i64> {
                let parent = (parent != 0).then_some(FolderId(parent));
                Ok(self
                    .daemon
                    .create_folder(AccountId(account), name, parent)
                    .await?
                    .0)
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

            async fn snooze(&self, messages: Vec<i64>, until: i64) -> fdo::Result<()> {
                Ok(self.daemon.snooze(&ids(&messages), until).await?)
            }

            async fn unsnooze(&self, messages: Vec<i64>) -> fdo::Result<()> {
                Ok(self.daemon.unsnooze(&ids(&messages))?)
            }

            async fn set_follow_up(&self, id: i64, after: i64) -> fdo::Result<()> {
                Ok(self.daemon.set_follow_up(id, after)?)
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

            async fn schedule_send(
                &self,
                account: i64,
                message: Vec<u8>,
                delay: u32,
                at: i64,
            ) -> fdo::Result<i64> {
                Ok(self
                    .daemon
                    .schedule_send(AccountId(account), &message, delay, at)?)
            }

            async fn server_hold_limit(&self, account: i64) -> fdo::Result<u64> {
                Ok(self.daemon.server_hold_limit(AccountId(account)).await?)
            }

            async fn server_delivery_receipts(&self, account: i64) -> fdo::Result<bool> {
                Ok(self
                    .daemon
                    .server_delivery_receipts(AccountId(account))
                    .await?)
            }

            async fn queue_tracked_send(
                &self,
                account: i64,
                message: Vec<u8>,
                delay: u32,
            ) -> fdo::Result<i64> {
                Ok(self
                    .daemon
                    .queue_tracked_send(AccountId(account), &message, delay)?)
            }

            async fn save_template(&self, template: TemplateItem) -> fdo::Result<i64> {
                Ok(self.daemon.save_template(template)?)
            }

            async fn rename_template(&self, id: i64, name: String) -> fdo::Result<bool> {
                Ok(self.daemon.rename_template(id, &name)?)
            }

            async fn delete_template(&self, id: i64) -> fdo::Result<bool> {
                Ok(self.daemon.delete_template(id)?)
            }

            async fn save_note(&self, note: NoteItem) -> fdo::Result<i64> {
                Ok(self.daemon.save_note(note)?)
            }

            async fn trash_notes(&self, ids: Vec<i64>, trashed: bool) -> fdo::Result<u32> {
                Ok(self.daemon.trash_notes(&ids, trashed)?)
            }

            async fn delete_notes(&self, ids: Vec<i64>) -> fdo::Result<u32> {
                Ok(self.daemon.delete_notes(&ids)?)
            }

            async fn undo_send(&self, id: i64) -> fdo::Result<bool> {
                Ok(self.daemon.undo_send(id)?)
            }

            async fn discard_send(&self, id: i64) -> fdo::Result<bool> {
                Ok(self.daemon.discard_send(id)?)
            }

            async fn save_draft(&self, account: i64, message: Vec<u8>) -> fdo::Result<i64> {
                Ok(self.daemon.save_draft(AccountId(account), &message)?)
            }

            async fn discard_draft(&self, account: i64, message_id: String) -> fdo::Result<()> {
                Ok(self.daemon.discard_draft(AccountId(account), &message_id)?)
            }

            /// Reads the settings file again (after Katna Mail saved it).
            async fn reload_config(&self) -> fdo::Result<()> {
                Ok(self.daemon.reload_config()?)
            }

            /// Whether the daemon saves data as on a metered network: from
            /// NetworkManager, or the `sync.metered` setting.
            async fn metered(&self) -> bool {
                self.daemon.metered()
            }

            async fn outbox(&self) -> fdo::Result<Vec<OutboxItem>> {
                Ok(self.daemon.outbox()?)
            }

            async fn drive_upload(&self, account: i64, path: String) -> fdo::Result<i64> {
                Ok(self.daemon.drive_upload(AccountId(account), &path).await?)
            }

            async fn drive_upload_status(&self, id: i64) -> fdo::Result<DriveUpload> {
                Ok(self.daemon.drive_upload_status(id)?)
            }

            async fn drive_cancel(&self, id: i64) -> fdo::Result<bool> {
                Ok(self.daemon.drive_cancel(id).await?)
            }

            async fn drive_share(
                &self,
                uploads: Vec<i64>,
                addresses: Vec<String>,
            ) -> fdo::Result<Vec<String>> {
                Ok(self.daemon.drive_share(&uploads, &addresses).await?)
            }

            async fn drive_share_with_link(&self, uploads: Vec<i64>) -> fdo::Result<Vec<String>> {
                Ok(self.daemon.drive_share_with_link(&uploads).await?)
            }

            async fn fetch_image(&self, url: String) -> fdo::Result<Vec<u8>> {
                Ok(self.daemon.fetch_image(&url).await?)
            }

            async fn sender_picture(&self, address: String) -> fdo::Result<Vec<u8>> {
                Ok(self.daemon.sender_picture(&address).await?)
            }

            async fn translate(
                &self,
                message: i64,
                text: String,
                source: String,
                target: String,
            ) -> (String, String, String) {
                self.daemon
                    .translate(MessageId(message), &text, &source, &target)
                    .await
                    .map_or_else(
                        |err| (String::new(), String::new(), problem(&err).to_owned()),
                        |done| (done.source, done.text, String::new()),
                    )
            }

            async fn translation_sources(&self, target: String) -> (Vec<String>, String) {
                match self.daemon.translation_sources(&target).await {
                    Ok(sources) => (sources, String::new()),
                    Err(err) => (Vec::new(), problem(&err).to_owned()),
                }
            }

            async fn update_status(&self) -> UpdateStatus {
                self.daemon.updates().status()
            }

            async fn update_details(&self) -> String {
                self.daemon.updates().details()
            }

            async fn check_for_update(&self) {
                self.daemon.updates().check_now();
            }

            async fn download_update(&self) {
                self.daemon.updates().download_now();
            }

            async fn katna_account(&self) -> fdo::Result<KatnaAccount> {
                Ok(self.daemon.katna()?.account().await?)
            }

            async fn katna_sign_up(
                &self,
                email: &str,
                password: &str,
            ) -> fdo::Result<KatnaAccount> {
                let account = self.daemon.katna()?.sign_up(email, password).await?;
                self.daemon.katna_changed();
                Ok(account)
            }

            async fn katna_sign_in(
                &self,
                email: &str,
                password: &str,
            ) -> fdo::Result<KatnaAccount> {
                let account = self.daemon.katna()?.sign_in(email, password).await?;
                self.daemon.katna_changed();
                Ok(account)
            }

            async fn katna_verify(&self, code: &str) -> fdo::Result<KatnaAccount> {
                let account = self.daemon.katna()?.verify(code).await?;
                self.daemon.katna_changed();
                Ok(account)
            }

            async fn katna_resend_code(&self) -> fdo::Result<()> {
                Ok(self.daemon.katna()?.resend_code().await?)
            }

            async fn katna_sign_out(&self) -> fdo::Result<()> {
                self.daemon.katna()?.sign_out().await?;
                self.daemon.katna_changed();
                Ok(())
            }

            async fn katna_devices(&self) -> fdo::Result<Vec<KatnaDevice>> {
                Ok(self.daemon.katna()?.devices().await?)
            }

            async fn katna_sign_out_device(&self, id: &str) -> fdo::Result<()> {
                Ok(self.daemon.katna()?.sign_out_device(id).await?)
            }

            async fn katna_change_password(&self, current: &str, new: &str) -> fdo::Result<()> {
                Ok(self.daemon.katna()?.change_password(current, new).await?)
            }

            async fn katna_reset_password(&self, email: &str) -> fdo::Result<()> {
                Ok(self.daemon.katna()?.reset_password(email).await?)
            }

            async fn katna_confirm_reset(
                &self,
                email: &str,
                code: &str,
                password: &str,
            ) -> fdo::Result<KatnaAccount> {
                let account = self
                    .daemon
                    .katna()?
                    .confirm_reset(email, code, password)
                    .await?;
                self.daemon.katna_changed();
                Ok(account)
            }

            async fn katna_delete_account(&self, password: &str) -> fdo::Result<()> {
                self.daemon.katna()?.delete_account(password).await?;
                self.daemon.katna_changed();
                Ok(())
            }

            #[zbus(signal)]
            async fn update_changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn katna_account_changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

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

            #[zbus(signal)]
            async fn metered_changed(
                emitter: &SignalEmitter<'_>,
                metered: bool,
            ) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn tracking_changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn drive_changed(emitter: &SignalEmitter<'_>, id: i64) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn contacts_changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
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
            flag::IMPORTANT => MessageFlags::IMPORTANT,
            other => {
                return Err(CommandError::InvalidArgs(format!(
                    "unknown flag {other:?} \
                     (seen, answered, flagged, draft, forwarded or important)"
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
    let agenda = match SignalEmitter::new(&connection, ids::AGENDA_OBJECT_PATH) {
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
            Notice::MeteredChanged(on) => PimService::metered_changed(&emitter, on).await,
            Notice::KatnaAccountChanged => PimService::katna_account_changed(&emitter).await,
            Notice::TrackingChanged => PimService::tracking_changed(&emitter).await,
            Notice::UpdateChanged => PimService::update_changed(&emitter).await,
            Notice::DriveChanged(id) => PimService::drive_changed(&emitter, id).await,
            Notice::ContactsChanged => PimService::contacts_changed(&emitter).await,
            Notice::TasksChanged => crate::agenda::AgendaService::changed(&agenda).await,
        };
        if let Err(err) = sent {
            tracing::warn!(%err, ?notice, "could not send a signal");
        }
    }
}
