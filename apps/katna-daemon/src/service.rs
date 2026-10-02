// SPDX-License-Identifier: GPL-3.0-or-later

//! The `in.invenia.katna.Pim1` interface on the session bus.

use std::sync::Arc;

use async_channel::Receiver;
use katna_core::{AccountId, Pop3Keep, ids};
use katna_dbus::{
    AccountStatus, DriveUpload, KatnaAccount, KatnaDevice, NewImapAccount, NewPop3Account,
    NoteItem, OutboxItem, ServerSpec, TemplateItem, UpdateStatus, flag, mute,
};
use katna_store::{Bell, FolderId, MailCategory, MessageFlags, MessageId, Pinned};
use zbus::{fdo, object_server::SignalEmitter};

use crate::daemon::{CommandError, Daemon, MuteOf, Notice};
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
            ) -> fdo::Result<(NewImapAccount, ServerSpec, String, String, bool)> {
                let (account, pop3, found) = self.daemon.discover_account(&address).await?;
                let sign_in = found
                    .oauth
                    .map(|p| p.as_str().to_owned())
                    .unwrap_or_default();
                Ok((
                    account,
                    pop3,
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

            async fn set_pop3_keep(
                &self,
                account: i64,
                leave_on_server: bool,
                keep_days: u32,
                delete_with_local: bool,
            ) -> fdo::Result<()> {
                let keep = Pop3Keep {
                    leave_on_server,
                    days: (keep_days > 0).then_some(keep_days),
                    delete_with_local,
                };
                Ok(self.daemon.set_pop3_keep(AccountId(account), keep).await?)
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

            async fn sync_folder(&self, folder: i64) -> fdo::Result<()> {
                Ok(self.daemon.sync_folder(FolderId(folder)).await?)
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

            async fn mute(
                &self,
                kind: &str,
                id: i64,
                address: &str,
                until: i64,
            ) -> fdo::Result<()> {
                let what = mute_of(kind, id, address)?;
                Ok(self.daemon.mute(what, (until != 0).then_some(until))?)
            }

            async fn unmute(&self, kind: &str, id: i64, address: &str) -> fdo::Result<()> {
                Ok(self.daemon.unmute(mute_of(kind, id, address)?)?)
            }

            async fn pin_in_chat(
                &self,
                message: i64,
                kind: &str,
                file: i64,
                text: &str,
                label: &str,
                replace: i64,
            ) -> fdo::Result<i64> {
                let what =
                    match kind {
                        "mail" => Pinned::Mail,
                        "file" => Pinned::File(usize::try_from(file).map_err(|_| {
                            CommandError::InvalidArgs(format!("no attachment {file}"))
                        })?),
                        "text" => Pinned::Text(text.to_owned()),
                        other => {
                            return Err(
                                CommandError::InvalidArgs(format!("no pin kind {other}")).into()
                            );
                        }
                    };
                Ok(self
                    .daemon
                    .pin_in_chat(
                        MessageId(message),
                        what,
                        label,
                        (replace != 0).then_some(replace),
                    )?
                    .unwrap_or(0))
            }

            async fn unpin_in_chat(&self, id: i64) -> fdo::Result<()> {
                Ok(self.daemon.unpin_in_chat(id)?)
            }

            async fn order_chat_pins(&self, ids: Vec<i64>) -> fdo::Result<()> {
                Ok(self.daemon.order_chat_pins(&ids)?)
            }

            async fn set_bell(
                &self,
                folder: i64,
                category: i64,
                notify: bool,
                count: bool,
            ) -> fdo::Result<()> {
                let category =
                    match category {
                        0 => None,
                        n => Some(MailCategory::from_storage(n).ok_or_else(|| {
                            CommandError::InvalidArgs(format!("no inbox tab {n}"))
                        })?),
                    };
                Ok(self
                    .daemon
                    .set_bell(FolderId(folder), category, Bell { notify, count })?)
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

            async fn save_contact(
                &self,
                contact: i64,
                book: i64,
                card: String,
            ) -> fdo::Result<i64> {
                Ok(self.daemon.save_contact(contact, book, &card).await?)
            }

            async fn delete_contacts(&self, ids: Vec<i64>) -> fdo::Result<()> {
                Ok(self.daemon.delete_contacts(&ids).await?)
            }

            async fn import_contacts(&self, book: i64, cards: String) -> fdo::Result<Vec<i64>> {
                Ok(self.daemon.import_contacts(book, &cards).await?)
            }

            async fn save_other_contact(&self, id: i64) -> fdo::Result<i64> {
                Ok(self.daemon.save_other_contact(id).await?)
            }

            async fn set_contact_labels(
                &self,
                contact: i64,
                labels: Vec<String>,
            ) -> fdo::Result<()> {
                Ok(self.daemon.set_contact_labels(contact, labels).await?)
            }

            async fn rename_contact_label(&self, old: String, new: String) -> fdo::Result<()> {
                Ok(self.daemon.rename_contact_label(&old, &new).await?)
            }

            async fn save_note(&self, note: NoteItem) -> fdo::Result<i64> {
                Ok(self.daemon.save_note(note)?)
            }

            async fn trash_notes(&self, ids: Vec<i64>, trashed: bool) -> fdo::Result<u32> {
                Ok(self.daemon.trash_notes(&ids, trashed)?)
            }

            async fn order_notes(&self, ids: Vec<i64>) -> fdo::Result<u32> {
                Ok(self.daemon.order_notes(&ids)?)
            }

            async fn delete_notes(&self, ids: Vec<i64>) -> fdo::Result<u32> {
                Ok(self.daemon.delete_notes(&ids)?)
            }

            async fn relabel_notes(
                &self,
                ids: Vec<i64>,
                old: String,
                new: String,
            ) -> fdo::Result<u32> {
                Ok(self.daemon.relabel_notes(&ids, &old, &new)?)
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

            async fn meeting_link(&self, account: i64) -> fdo::Result<String> {
                Ok(self.daemon.meeting_link(AccountId(account)).await?)
            }

            async fn set_calendar_hidden(&self, id: i64, hidden: bool) -> fdo::Result<()> {
                Ok(self.daemon.set_calendar_hidden(id, hidden)?)
            }

            async fn add_calendar(
                &self,
                account: i64,
                name: String,
                color: String,
            ) -> fdo::Result<i64> {
                Ok(self.daemon.add_calendar(account, &name, &color).await?)
            }

            async fn rename_calendar(&self, id: i64, name: String) -> fdo::Result<()> {
                Ok(self.daemon.rename_calendar(id, &name).await?)
            }

            async fn set_calendar_color(&self, id: i64, color: String) -> fdo::Result<String> {
                Ok(self.daemon.set_calendar_color(id, &color).await?)
            }

            async fn delete_calendar(&self, id: i64, delete: bool) -> fdo::Result<()> {
                Ok(self.daemon.delete_calendar(id, delete).await?)
            }

            async fn calendar_status(&self) -> fdo::Result<Vec<(i64, String, String)>> {
                Ok(self.daemon.calendar_status()?)
            }

            async fn contacts_status(&self) -> fdo::Result<Vec<(i64, String, String)>> {
                Ok(self.daemon.contacts_status()?)
            }

            async fn tasks_status(&self) -> fdo::Result<Vec<(i64, String, String)>> {
                Ok(self.daemon.tasks_status()?)
            }

            async fn edit_event(&self, json: String) -> fdo::Result<i64> {
                Ok(self.daemon.edit_event(&json)?)
            }

            async fn fetch_image(&self, url: String) -> fdo::Result<Vec<u8>> {
                Ok(self.daemon.fetch_image(&url).await?)
            }

            async fn sender_picture(&self, address: String) -> fdo::Result<Vec<u8>> {
                Ok(self.daemon.sender_picture(&address).await?)
            }

            async fn company_of(&self, address: String, website: String) -> fdo::Result<String> {
                Ok(self.daemon.company_of(&address, &website).await?)
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

            async fn ai_rephrase(
                &self,
                text: String,
                tone: String,
                instruction: String,
            ) -> (String, String, u32, String) {
                match self.daemon.ai_rephrase(&text, &tone, &instruction).await {
                    Ok(done) => (
                        done.text,
                        done.plan.kind,
                        done.plan.days_left.unwrap_or(0),
                        String::new(),
                    ),
                    Err(err) => (String::new(), String::new(), 0, err.problem().to_owned()),
                }
            }

            async fn ai_complete(&self, before: String, answered: String) -> (String, String) {
                match self.daemon.ai_complete(&before, &answered).await {
                    Ok(done) => (done.text, String::new()),
                    Err(err) => (String::new(), err.problem().to_owned()),
                }
            }

            async fn ai_summarize(
                &self,
                newest: i64,
                request: String,
            ) -> (String, String, u32, String) {
                let Ok(request) = serde_json::from_str(&request) else {
                    return (
                        String::new(),
                        String::new(),
                        0,
                        katna_ai::wire::problem::FAILED.to_owned(),
                    );
                };
                match self.daemon.ai_summarize(MessageId(newest), &request).await {
                    Ok((summary, plan)) => (
                        summary,
                        plan.kind,
                        plan.days_left.unwrap_or(0),
                        String::new(),
                    ),
                    Err(err) => (String::new(), String::new(), 0, err.problem().to_owned()),
                }
            }

            async fn ai_draft(&self, request: String) -> (String, String, u32, String) {
                let Ok(request) = serde_json::from_str(&request) else {
                    return (
                        String::new(),
                        String::new(),
                        0,
                        katna_ai::wire::problem::FAILED.to_owned(),
                    );
                };
                match self.daemon.ai_draft(&request).await {
                    Ok(done) => (
                        done.text,
                        done.plan.kind,
                        done.plan.days_left.unwrap_or(0),
                        String::new(),
                    ),
                    Err(err) => (String::new(), String::new(), 0, err.problem().to_owned()),
                }
            }

            async fn set_ai_key(&self, key: String) -> fdo::Result<()> {
                Ok(self.daemon.set_ai_key(&key).await?)
            }

            async fn ai_key_saved(&self) -> fdo::Result<bool> {
                Ok(self.daemon.ai_key_saved().await?)
            }

            async fn ai_models(&self, provider: String, address: String) -> (Vec<String>, String) {
                match self.daemon.ai_models(&provider, &address).await {
                    Ok(models) => (models, String::new()),
                    Err(err) => (Vec::new(), err.problem().to_owned()),
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
            async fn calendar_changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

            #[zbus(signal)]
            async fn contacts_changed(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
        }
    };
}

katna_core::with_dbus_names!(pim_interface);

fn ids(messages: &[i64]) -> Vec<MessageId> {
    messages.iter().copied().map(MessageId).collect()
}

/// What [`katna_dbus::mute`] `kind` and `id` or `address` name.
fn mute_of(kind: &str, id: i64, address: &str) -> Result<MuteOf, CommandError> {
    Ok(match kind {
        mute::ACCOUNT => MuteOf::Account(AccountId(id)),
        mute::FOLDER => MuteOf::Folder(FolderId(id)),
        mute::CONVERSATION => MuteOf::Conversation(MessageId(id)),
        mute::SENDER => MuteOf::Sender(address.to_owned()),
        other => {
            return Err(CommandError::InvalidArgs(format!(
                "cannot mute {other:?} (account, folder, conversation or sender)"
            )));
        }
    })
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
            flag::MUTED => MessageFlags::MUTED,
            other => {
                return Err(CommandError::InvalidArgs(format!(
                    "unknown flag {other:?} \
                     (seen, answered, flagged, draft, forwarded, important or muted)"
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
            Notice::CalendarChanged => {
                // The clock shows the events too.
                if let Err(err) = crate::agenda::changed(&agenda).await {
                    tracing::warn!(%err, "could not tell the clock");
                }
                PimService::calendar_changed(&emitter).await
            }
            Notice::ContactsChanged => PimService::contacts_changed(&emitter).await,
            Notice::TasksChanged => crate::agenda::AgendaService::changed(&agenda).await,
        };
        if let Err(err) = sent {
            tracing::warn!(%err, ?notice, "could not send a signal");
        }
    }
}
