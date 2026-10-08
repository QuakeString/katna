// SPDX-License-Identifier: GPL-3.0-or-later

//! Files too large for mail go through the sender's Google Drive, as in
//! Gmail, or OneDrive, as in Outlook: a file over the 25 MB limit starts
//! uploading as soon as it is attached, its chip shows how far it got,
//! and Send shares it with the recipients and puts its link in the
//! message. Nothing is asked unless it has to be: an account signed in
//! before Katna asked for Drive or OneDrive gets one Allow button, and
//! Send asks only when a recipient cannot be given access
//! (`docs/ARCHITECTURE.md` §6.6).

use std::path::PathBuf;

use gpui::{AnyElement, Context, FontWeight, Window, div, prelude::*, relative, rgba};
use katna_core::{AccountId, OAuthProvider};
use katna_dbus::{DriveUpload, drive_state};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::html::escape;

use super::super::MailWindow;
use super::checks::Passed;
use super::tools::Popup;
use crate::daemon;
use crate::format;
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, tip};

/// A file on its way to, or in, Google Drive or OneDrive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::window) struct DriveFile {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    /// The account whose Drive it goes to.
    pub account: AccountId,
    /// OneDrive (a Microsoft account) rather than Google Drive.
    pub onedrive: bool,
    /// The daemon's upload, once it started one.
    pub upload: Option<i64>,
    pub sent: u64,
    pub state: DriveState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::window) enum DriveState {
    Uploading,
    Done {
        link: String,
    },
    /// The account's sign-in did not allow Drive.
    NeedsPermission,
    Failed(String),
}

impl DriveFile {
    fn link(&self) -> Option<&str> {
        match &self.state {
            DriveState::Done { link } => Some(link),
            _ => None,
        }
    }
}

/// Who keeps the large files of `account`: Google Drive or OneDrive when
/// it signs in with Google or Microsoft.
pub(super) fn drive_provider(window: &MailWindow, account: AccountId) -> Option<OAuthProvider> {
    window
        .mail
        .as_ref()
        .ok()
        .and_then(|mail| mail.sign_in_provider(account))
}

/// The note shown when files go to the storage of `provider`.
pub(super) fn drive_note(provider: OAuthProvider, name: String, limit: String) -> String {
    match provider {
        OAuthProvider::Google => tr!("compose-drive-note", name = name, limit = limit),
        // Only Google and Microsoft accounts keep large files.
        OAuthProvider::Microsoft | OAuthProvider::Zoho => {
            tr!("compose-onedrive-note", name = name, limit = limit)
        }
    }
}

/// The links of the uploaded files, for the end of the message's text.
pub(super) fn links_text(files: &[DriveFile]) -> String {
    let mut text = String::new();
    for file in files {
        if let Some(link) = file.link() {
            text.push_str(&format!("\n\n{}\n{link}", file.name));
        }
    }
    text
}

/// The links of the uploaded files as cards under the message, like
/// Gmail's.
pub(super) fn links_html(files: &[DriveFile]) -> String {
    let mut html = String::new();
    for file in files {
        let Some(link) = file.link() else {
            continue;
        };
        let size = format::size(file.size);
        // A drive's own document has no size.
        let under = match (file.onedrive, file.size) {
            (true, 0) => tr!("compose-onedrive-card-name"),
            (false, 0) => tr!("compose-drive-card-name"),
            (true, _) => tr!("compose-onedrive-card-detail", size = size),
            (false, _) => tr!("compose-drive-card-detail", size = size),
        };
        html.push_str(&format!(
            "<div style=\"margin:12px 0 0\"><div style=\"display:inline-block;\
             border:1px solid #dadce0;border-radius:8px;padding:10px 14px;\
             font-family:sans-serif;font-size:14px;line-height:20px\">\
             <a href=\"{}\" style=\"color:#1a73e8;text-decoration:none;\
             font-weight:500\">{}</a><div style=\"color:#5f6368;font-size:12px;\
             line-height:16px\">{}</div></div></div>",
            escape(link),
            escape(&file.name),
            escape(&under)
        ));
    }
    html
}

impl MailWindow {
    /// Starts putting `path` in the Drive of `account`, with a chip that
    /// shows how it goes.
    pub(super) fn upload_to_drive(
        &mut self,
        path: PathBuf,
        name: String,
        size: u64,
        account: AccountId,
        cx: &mut Context<Self>,
    ) {
        let onedrive = drive_provider(self, account) == Some(OAuthProvider::Microsoft);
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.drive.push(DriveFile {
            path: path.clone(),
            name,
            size,
            account,
            onedrive,
            upload: None,
            sent: 0,
            state: DriveState::Uploading,
        });
        compose.attach_scroll.scroll_to_bottom();
        self.start_drive_upload(path, cx);
    }

    /// A new message with file `entry` of the drive of `account` on it
    /// as a link: Files' Attach for a file too big for mail or one of the
    /// drive's own documents.
    pub(in crate::window) fn new_mail_with_link(
        &mut self,
        account: AccountId,
        entry: katna_dbus::CloudEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_compose(super::Kind::New, None, window, cx);
        if self
            .compose
            .as_ref()
            .is_some_and(|c| c.kind == super::Kind::New)
        {
            self.link_drive_file(account, entry, cx);
        }
    }

    /// Compose is open, not on its way out.
    pub(in crate::window) fn compose_writing(&self) -> bool {
        self.compose.as_ref().is_some_and(|c| !c.closing)
    }

    /// Takes down Compose's open popup, as the attach picker opens; and
    /// whether Compose is open to attach to.
    pub(in crate::window) fn compose_takes_files(&mut self) -> bool {
        match &mut self.compose {
            Some(compose) if !compose.closing => {
                compose.popup = None;
                true
            }
            _ => false,
        }
    }

    /// Puts file `entry`, already in the drive of `account`, on the
    /// message as a link chip, shared with the recipients at Send like an
    /// uploaded file. Taking the chip off leaves the file in the drive.
    pub(in crate::window) fn link_drive_file(
        &mut self,
        account: AccountId,
        entry: katna_dbus::CloudEntry,
        cx: &mut Context<Self>,
    ) {
        let Some(connection) = self.daemon.clone() else {
            self.show_snackbar(daemon::NOT_RUNNING.to_owned(), None, cx);
            return;
        };
        let onedrive = drive_provider(self, account) == Some(OAuthProvider::Microsoft);
        // No file on this computer: the drive's id keeps the chip apart.
        let path = PathBuf::from(format!("drive:{}:{}", account.0, entry.id));
        let Some(compose) = &mut self.compose else {
            return;
        };
        if compose.drive.iter().any(|f| f.path == path) {
            return;
        }
        compose.drive.push(DriveFile {
            path: path.clone(),
            name: entry.name.clone(),
            size: entry.size,
            account,
            onedrive,
            upload: None,
            sent: entry.size,
            state: DriveState::Done {
                link: entry.link.clone(),
            },
        });
        compose.attach_scroll.scroll_to_bottom();
        cx.spawn(async move |this, cx| {
            let linked = daemon::cloud_link(&connection, account.0, &entry).await;
            this.update(cx, |this, cx| {
                let Some(file) = this
                    .compose
                    .as_mut()
                    .and_then(|c| c.drive.iter_mut().find(|f| f.path == path))
                else {
                    if let Ok(id) = linked {
                        this.cancel_drive_upload(id, cx);
                    }
                    return;
                };
                match linked {
                    Ok(id) => file.upload = Some(id),
                    Err(err) => file.state = DriveState::Failed(err),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Asks the daemon to upload the chip of `path`, again after a
    /// failure or a sign-in.
    fn start_drive_upload(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            self.set_drive_state(&path, DriveState::Failed(daemon::NOT_RUNNING.into()), cx);
            return;
        };
        let Some(file) = self
            .compose
            .as_mut()
            .and_then(|c| c.drive.iter_mut().find(|f| f.path == path))
        else {
            return;
        };
        file.state = DriveState::Uploading;
        file.sent = 0;
        let account = file.account.0;
        self.watch_drive(cx);
        cx.spawn(async move |this, cx| {
            let started = daemon::drive_upload(&connection, account, &path.to_string_lossy()).await;
            this.update(cx, |this, cx| {
                let Some(file) = this
                    .compose
                    .as_mut()
                    .and_then(|c| c.drive.iter_mut().find(|f| f.path == path))
                else {
                    // Taken off before the upload started.
                    if let Ok(id) = started {
                        this.cancel_drive_upload(id, cx);
                    }
                    return;
                };
                match started {
                    Ok(id) => file.upload = Some(id),
                    Err(err) => file.state = DriveState::Failed(err),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn set_drive_state(&mut self, path: &PathBuf, state: DriveState, cx: &mut Context<Self>) {
        if let Some(file) = self
            .compose
            .as_mut()
            .and_then(|c| c.drive.iter_mut().find(|f| &f.path == path))
        {
            file.state = state;
            cx.notify();
        }
    }

    /// Follows the daemon's uploads, once one started.
    fn watch_drive(&mut self, cx: &mut Context<Self>) {
        if self.drive_watch.is_some() {
            return;
        }
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self.drive_watch = Some(cx.spawn(async move |this, cx| {
            use futures_lite::StreamExt;
            let Ok(mut changes) = daemon::drive_changes(&connection).await else {
                return;
            };
            while let Some(id) = changes.next().await {
                let Ok(status) = daemon::drive_upload_status(&connection, id).await else {
                    continue;
                };
                if this
                    .update(cx, |this, cx| this.drive_changed(status, cx))
                    .is_err()
                {
                    return;
                }
            }
        }));
    }

    fn drive_changed(&mut self, status: DriveUpload, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let Some(file) = compose
            .drive
            .iter_mut()
            .find(|f| f.upload == Some(status.id))
        else {
            return;
        };
        file.sent = status.sent;
        file.state = match status.state.as_str() {
            drive_state::DONE => DriveState::Done { link: status.link },
            drive_state::NEEDS_PERMISSION => DriveState::NeedsPermission,
            drive_state::FAILED => DriveState::Failed(status.error),
            _ => DriveState::Uploading,
        };
        cx.notify();
    }

    fn cancel_drive_upload(&mut self, id: i64, cx: &mut Context<Self>) {
        if let Some(connection) = self.daemon.clone() {
            cx.background_executor()
                .spawn(async move {
                    if let Err(err) = daemon::drive_cancel(&connection, id).await {
                        tracing::warn!(%err, "cannot remove the file from Drive");
                    }
                })
                .detach();
        }
    }

    /// Takes chip `ix` off the message, and its file out of Drive.
    fn remove_drive_file(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        if ix >= compose.drive.len() {
            return;
        }
        let file = compose.drive.remove(ix);
        if let Some(id) = file.upload {
            self.cancel_drive_upload(id, cx);
        }
        cx.notify();
    }

    /// Signs the account of chip `ix` in with Google or Microsoft again,
    /// now allowing Drive or OneDrive, then uploads its files that waited
    /// for it.
    fn allow_drive(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some((account, onedrive)) = self
            .compose
            .as_ref()
            .and_then(|c| c.drive.get(ix))
            .map(|f| (f.account, f.onedrive))
        else {
            return;
        };
        let address = self
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.address.clone())
            .unwrap_or_default();
        cx.spawn(async move |this, cx| {
            let provider = if onedrive {
                OAuthProvider::Microsoft
            } else {
                OAuthProvider::Google
            };
            let signed_in = daemon::sign_in(&connection, provider, Some(account.0), &address).await;
            this.update(cx, |this, cx| {
                if signed_in.is_err() {
                    return;
                }
                let waiting: Vec<PathBuf> = this
                    .compose
                    .iter()
                    .flat_map(|c| &c.drive)
                    .filter(|f| f.account == account && f.state == DriveState::NeedsPermission)
                    .map(|f| f.path.clone())
                    .collect();
                for path in waiting {
                    this.start_drive_upload(path, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// Before Send: waits for uploads under way, then shares the files
    /// with the recipients. Returns whether sending has to stop here (it
    /// goes on by itself once it can).
    pub(super) fn drive_before_send(
        &mut self,
        at: Option<jiff::Timestamp>,
        archive: bool,
        passed: Passed,
        recipients: Vec<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(compose) = &mut self.compose else {
            return false;
        };
        if compose.drive.is_empty() || passed.shared {
            return false;
        }
        if let Some(file) = compose
            .drive
            .iter()
            .find(|f| matches!(f.state, DriveState::NeedsPermission | DriveState::Failed(_)))
        {
            let name = file.name.clone();
            let text = if file.onedrive {
                tr!("compose-onedrive-not-uploaded", name = name)
            } else {
                tr!("compose-drive-not-uploaded", name = name)
            };
            self.show_snackbar(text, None, cx);
            return true;
        }
        if let Some(file) = compose
            .drive
            .iter()
            .find(|f| f.state == DriveState::Uploading)
        {
            // Sent by itself once the uploads are done, as in Gmail.
            if !std::mem::replace(&mut compose.send_when_uploaded, true) {
                let text = tr!(
                    "compose-drive-sends-when-uploaded",
                    name = file.name.clone()
                );
                self.show_snackbar(text, None, cx);
                cx.spawn_in(window, async move |this, cx| {
                    loop {
                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(300))
                            .await;
                        let go_on = this.update_in(cx, |this, window, cx| {
                            let Some(c) = &mut this.compose else {
                                return false;
                            };
                            if !c.send_when_uploaded {
                                return false;
                            }
                            if c.drive.iter().any(|f| f.state == DriveState::Uploading) {
                                return true;
                            }
                            c.send_when_uploaded = false;
                            this.send_compose(at, archive, passed, window, cx);
                            false
                        });
                        if !matches!(go_on, Ok(true)) {
                            return;
                        }
                    }
                })
                .detach();
            }
            cx.notify();
            return true;
        }
        let Some(connection) = self.daemon.clone() else {
            return false;
        };
        let uploads: Vec<i64> = compose.drive.iter().filter_map(|f| f.upload).collect();
        let onedrive = compose.drive.iter().any(|f| f.onedrive);
        cx.spawn_in(window, async move |this, cx| {
            let refused = daemon::drive_share(&connection, &uploads, &recipients).await;
            this.update_in(cx, |this, window, cx| match refused {
                Ok(refused) if refused.is_empty() => {
                    this.send_compose(at, archive, passed.with_shared(), window, cx);
                }
                Ok(refused) => {
                    if let Some(c) = &mut this.compose {
                        c.popup = Some(Popup::DriveShare {
                            refused,
                            at,
                            archive,
                            passed,
                        });
                    }
                    cx.notify();
                }
                Err(err) => this.show_snackbar(share_failed(onedrive, err), None, cx),
            })
            .ok();
        })
        .detach();
        true
    }

    /// Send's question when Drive would not share with some recipients.
    pub(super) fn render_drive_share(
        &self,
        refused: &[String],
        at: Option<jiff::Timestamp>,
        archive: bool,
        passed: Passed,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let onedrive = self
            .compose
            .as_ref()
            .is_some_and(|c| c.drive.iter().any(|f| f.onedrive));
        let (count, addresses) = (refused.len() as u64, refused.join(", "));
        let text = if onedrive {
            tr!(
                "compose-onedrive-share-text",
                count = count,
                addresses = addresses
            )
        } else {
            tr!(
                "compose-drive-share-text",
                count = count,
                addresses = addresses
            )
        };
        let plain = |id: &'static str, label: String| {
            div()
                .id(id)
                .h(px(36.0))
                .px(px(16.0))
                .flex()
                .items_center()
                .rounded_full()
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.accent))
                .cursor_pointer()
                .relative()
                .child(crate::widgets::hover_fade("hover-glow", None, th))
                .child(label)
        };
        Self::dialog_card(th, 460.0, tr!("compose-drive-share-title"))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(text),
            )
            .child(
                div()
                    .mt(px(20.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .justify_end()
                    .gap(px(8.0))
                    .child(
                        plain("drive-share-cancel", tr!("compose-drive-share-cancel")).on_click(
                            cx.listener(|this, _, _, cx| {
                                if let Some(c) = &mut this.compose {
                                    c.popup = None;
                                }
                                cx.notify();
                            }),
                        ),
                    )
                    .child(
                        plain("drive-share-without", tr!("compose-drive-send-without")).on_click(
                            cx.listener(move |this, _, window, cx| {
                                this.send_compose(at, archive, passed.with_shared(), window, cx)
                            }),
                        ),
                    )
                    .child(
                        filled_button("drive-share-link", tr!("compose-drive-share-link"), th)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.share_with_link_and_send(at, archive, passed, window, cx)
                            })),
                    ),
            )
            .into_any_element()
    }

    pub(super) fn share_with_link_and_send(
        &mut self,
        at: Option<jiff::Timestamp>,
        archive: bool,
        passed: Passed,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (Some(connection), Some(compose)) = (self.daemon.clone(), &mut self.compose) else {
            return;
        };
        compose.popup = None;
        let uploads: Vec<i64> = compose.drive.iter().filter_map(|f| f.upload).collect();
        let onedrive = compose.drive.iter().any(|f| f.onedrive);
        cx.spawn_in(window, async move |this, cx| {
            let shared = daemon::drive_share_with_link(&connection, &uploads).await;
            this.update_in(cx, |this, window, cx| match shared {
                Ok(links) => {
                    // OneDrive's shared link is another address than the
                    // file's own.
                    if let Some(c) = &mut this.compose {
                        for (id, link) in uploads.iter().zip(links) {
                            if let Some(file) = c.drive.iter_mut().find(|f| f.upload == Some(*id))
                                && let DriveState::Done { link: old } = &mut file.state
                            {
                                *old = link;
                            }
                        }
                    }
                    this.send_compose(at, archive, passed.with_shared(), window, cx)
                }
                Err(err) => this.show_snackbar(share_failed(onedrive, err), None, cx),
            })
            .ok();
        })
        .detach();
    }

    /// The chip of Drive file `ix`: its name, how far the upload got, and
    /// what to do when it cannot go on.
    pub(super) fn render_drive_chip(
        &self,
        ix: usize,
        file: &DriveFile,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let action = |id: &'static str, label: String| {
            div()
                .id((id, ix))
                .flex_none()
                .h(px(24.0))
                .px(px(8.0))
                .flex()
                .items_center()
                .rounded_full()
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.accent))
                .cursor_pointer()
                .relative()
                .child(crate::widgets::hover_fade("hover-glow", None, th))
                .child(label)
        };
        let detail: AnyElement = match &file.state {
            DriveState::Uploading => {
                let percent = (file.sent * 100).checked_div(file.size).unwrap_or(0);
                div()
                    .flex_none()
                    .text_color(rgba(th.text_dim))
                    .child(tr!("compose-drive-uploading", percent = percent))
                    .into_any_element()
            }
            DriveState::Done { .. } if file.size == 0 => div().into_any_element(),
            DriveState::Done { .. } => div()
                .flex_none()
                .text_color(rgba(th.text_dim))
                .child(tr!(
                    "compose-attachment-size",
                    size = format::size(file.size)
                ))
                .into_any_element(),
            DriveState::NeedsPermission => {
                let (label, why) = if file.onedrive {
                    (
                        tr!("compose-onedrive-allow"),
                        tr!("compose-onedrive-allow-tip"),
                    )
                } else {
                    (tr!("compose-drive-allow"), tr!("compose-drive-allow-tip"))
                };
                action("drive-allow", label)
                    .tooltip(tip(why, th))
                    .on_click(cx.listener(move |this, _, _, cx| this.allow_drive(ix, cx)))
                    .into_any_element()
            }
            DriveState::Failed(error) => {
                let path = file.path.clone();
                // The Drive API switched off in Katna's Google Cloud
                // project: say which, and offer the page that turns
                // it on.
                let off = katna_core::api_off::parse(error);
                let why = match off {
                    Some((api, _)) => tr!("google-api-off", api = api),
                    None => error.clone(),
                };
                let retry =
                    action("drive-retry", tr!("compose-drive-retry"))
                        .tooltip(tip(why, th))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.start_drive_upload(path.clone(), cx)
                        }));
                match off {
                    Some((api, url)) => {
                        let url = url.to_owned();
                        div()
                            .flex_none()
                            .flex()
                            .flex_row()
                            .child(
                                action("drive-turn-on", tr!("google-api-turn-on"))
                                    .tooltip(tip(tr!("google-api-turn-on-tooltip", api = api), th))
                                    .on_click(move |_, _, cx| cx.open_url(&url)),
                            )
                            .child(retry)
                            .into_any_element()
                    }
                    None => retry.into_any_element(),
                }
            }
        };
        let progress = match file.state {
            DriveState::Uploading if file.size > 0 => {
                Some((file.sent as f32 / file.size as f32).clamp(0.02, 1.0))
            }
            _ => None,
        };
        div()
            .id(("drive-file", ix))
            .relative()
            .overflow_hidden()
            .h(px(36.0))
            .max_w(px(280.0))
            .pl(px(10.0))
            .pr(px(4.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded(px(8.0))
            .bg(rgba(th.chip))
            .text_size(px(13.0))
            .when_some(file.link().map(str::to_owned), |d, link| {
                let text = if file.onedrive {
                    tr!("compose-onedrive-tip")
                } else {
                    tr!("compose-drive-tip")
                };
                // A click opens the file on the drive, to check it is the
                // right one.
                d.tooltip(tip(text, th))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.chip_hover())))
                    .on_click(move |_, _, cx| cx.open_url(&link))
            })
            .child(icon("cloud", th.accent, 18.0))
            .child(
                div()
                    .min_w_0()
                    .truncate()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.accent))
                    .child(file.name.clone()),
            )
            .child(detail)
            .child(
                div()
                    .id(("drive-remove", ix))
                    .flex_none()
                    .size(px(24.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .cursor_pointer()
                    .relative()
                    .child(crate::widgets::hover_fade("hover-glow", None, th))
                    .tooltip(tip(tr!("compose-remove-attachment"), th))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.remove_drive_file(ix, cx)
                    }))
                    .child(icon("close", th.text_dim, 16.0)),
            )
            .children(progress.map(|done| {
                div()
                    .absolute()
                    .left_0()
                    .bottom_0()
                    .h(px(3.0))
                    .w(relative(done))
                    .bg(rgba(th.accent))
            }))
            .into_any_element()
    }
}

/// Why sharing failed, in the words of the file's storage.
fn share_failed(onedrive: bool, error: String) -> String {
    if onedrive {
        tr!("compose-onedrive-share-failed", error = error)
    } else {
        tr!("compose-drive-share-failed", error = error)
    }
}
