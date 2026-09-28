// SPDX-License-Identifier: GPL-3.0-or-later

//! Updates in About (`docs/ARCHITECTURE.md` §21.2): whether Katna is up
//! to date, a newer version to download, its download's progress, and
//! Update, which says Katna Mail will restart, installs the download
//! behind the system's password prompt and opens Katna Mail again where
//! it was. The daemon checks and downloads; the notification that an
//! update is ready opens About here, on the confirmation.

use futures_lite::StreamExt;
use gpui::{AnyElement, Context, FontWeight, Task, Window, div, prelude::*, rgba};
use katna_dbus::zbus::Connection;
use katna_dbus::{UpdateStatus, update_state as state};
use katna_i18n::tr;
use katna_ui::px;

use super::MailWindow;
use crate::daemon;
use crate::theme::{Theme, fade};
use crate::updater::{self, InstallError};
use crate::widgets::{filled_button, icon, outlined_button};

/// Updates as the window shows them.
#[derive(Default)]
pub(super) struct Updates {
    /// From the daemon; `None` until it answered.
    status: Option<UpdateStatus>,
    /// Update was pressed: the card says Katna Mail will restart and
    /// waits for the go-ahead.
    confirm: bool,
    /// The notification's Update was pressed before the daemon said where
    /// the update stands: ask once it says the update is ready.
    confirm_when_ready: bool,
    /// The password prompt is open, or the package being installed.
    installing: bool,
    /// Why the last install did not happen.
    problem: Option<String>,
    watch: Option<Task<()>>,
    install: Option<Task<()>>,
}

impl MailWindow {
    /// Follows where an update stands.
    pub(super) fn watch_updates(&mut self, connection: Connection, cx: &mut Context<Self>) {
        self.updates.watch = Some(cx.spawn(async move |this, cx| {
            let Ok(mut changes) = daemon::update_changes(&connection).await else {
                return;
            };
            loop {
                let status = daemon::update_status(&connection).await.ok();
                let alive = this
                    .update(cx, |this, cx| {
                        let ready = status.as_ref().is_some_and(|s| s.state == state::READY);
                        if ready && std::mem::take(&mut this.updates.confirm_when_ready) {
                            this.updates.confirm = true;
                        }
                        this.updates.status = status;
                        cx.notify();
                    })
                    .is_ok();
                if !alive || changes.next().await.is_none() {
                    break;
                }
            }
        }));
    }

    /// Opens About on the update, asking to install it when it is ready
    /// (the notification's Update button).
    pub(super) fn show_update(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.about_open() {
            self.open_about(window, cx);
        }
        match &self.updates.status {
            Some(status) => self.updates.confirm = status.state == state::READY,
            None => self.updates.confirm_when_ready = true,
        }
        self.updates.problem = None;
        cx.notify();
    }

    fn check_for_update(&mut self, cx: &mut Context<Self>) {
        self.updates.problem = None;
        if let Some(connection) = self.daemon.clone() {
            cx.background_executor()
                .spawn(async move {
                    if let Err(err) = daemon::check_for_update(&connection).await {
                        tracing::warn!(%err, "cannot check for updates");
                    }
                })
                .detach();
        }
    }

    fn download_update(&mut self, cx: &mut Context<Self>) {
        if let Some(connection) = self.daemon.clone() {
            cx.background_executor()
                .spawn(async move {
                    if let Err(err) = daemon::download_update(&connection).await {
                        tracing::warn!(%err, "cannot download the update");
                    }
                })
                .detach();
        }
    }

    /// Installs the downloaded update, then starts the new Katna Mail and
    /// quits; it opens where this one was.
    fn install_update(&mut self, cx: &mut Context<Self>) {
        let Some(status) = self
            .updates
            .status
            .clone()
            .filter(|s| s.state == state::READY)
        else {
            return;
        };
        self.updates.confirm = false;
        self.updates.installing = true;
        self.updates.problem = None;
        cx.notify();
        self.updates.install = Some(cx.spawn(async move |this, cx| {
            let installed = cx
                .background_executor()
                .spawn(async move { updater::install(status.file.as_ref(), &status.sha256) })
                .await;
            this.update(cx, |this, cx| {
                this.updates.installing = false;
                match installed {
                    Ok(()) => match updater::start_new() {
                        Ok(()) => {
                            tracing::info!(version = status.version, "updated; restarting");
                            cx.quit();
                        }
                        Err(err) => {
                            tracing::warn!(%err, "cannot start the new Katna Mail");
                            this.updates.problem =
                                Some(tr!("about-update-restart-failed", error = err.to_string()));
                        }
                    },
                    Err(InstallError::Cancelled) => {
                        this.updates.problem = Some(tr!("about-update-cancelled"));
                    }
                    Err(InstallError::Unsupported) => {
                        this.updates.problem = Some(tr!("about-update-unsupported"));
                    }
                    Err(InstallError::Failed(error)) => {
                        this.updates.problem = Some(tr!("about-update-failed", error = error));
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// The Updates box in About; nothing for a build that does not update
    /// itself.
    pub(super) fn update_card(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let status = self.updates.status.as_ref()?;
        if status.state == state::UNSUPPORTED {
            return None;
        }
        let version = status.version.as_str();
        let busy = self.updates.installing;
        let (title, detail): (String, Option<String>) = if busy {
            (
                tr!("about-update-installing", version = version),
                Some(tr!("about-update-installing-detail")),
            )
        } else if self.updates.confirm {
            (
                tr!("about-update-confirm", version = version),
                Some(tr!("about-update-confirm-detail")),
            )
        } else {
            match status.state.as_str() {
                state::CHECKING => (tr!("about-update-checking"), None),
                state::UP_TO_DATE => (tr!("about-update-up-to-date"), None),
                state::AVAILABLE => (tr!("about-update-available", version = version), None),
                state::DOWNLOADING => (
                    tr!(
                        "about-update-downloading",
                        version = version,
                        percent = percent(status)
                    ),
                    None,
                ),
                state::READY => (
                    tr!("about-update-ready", version = version),
                    Some(tr!("about-update-ready-detail")),
                ),
                state::FAILED => (
                    tr!("about-update-check-failed"),
                    (!status.detail.is_empty()).then(|| status.detail.clone()),
                ),
                _ => (tr!("about-update-not-checked"), None),
            }
        };
        let problem = self.updates.problem.clone().filter(|_| !busy);

        let buttons = div().flex().flex_row().flex_wrap().gap(px(8.0));
        let buttons = if busy {
            None
        } else if self.updates.confirm {
            Some(
                buttons
                    .child(
                        outlined_button("about-update-cancel", tr!("about-update-cancel"), th)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.updates.confirm = false;
                                cx.notify();
                            })),
                    )
                    .child(
                        filled_button("about-update-install", tr!("about-update-restart"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.install_update(cx))),
                    ),
            )
        } else {
            match status.state.as_str() {
                state::READY => Some(buttons.child(
                    filled_button("about-update-now", tr!("about-update-button"), th).on_click(
                        cx.listener(|this, _, _, cx| {
                            this.updates.confirm = true;
                            this.updates.problem = None;
                            cx.notify();
                        }),
                    ),
                )),
                state::AVAILABLE => Some(
                    buttons.child(
                        filled_button("about-update-download", tr!("about-update-download"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.download_update(cx))),
                    ),
                ),
                state::CHECKING | state::DOWNLOADING => None,
                _ => Some(
                    buttons.child(
                        outlined_button("about-update-check", tr!("about-update-check"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.check_for_update(cx))),
                    ),
                ),
            }
        };
        let progress = (status.state == state::DOWNLOADING && !busy).then(|| {
            let share = if status.total > 0 {
                (status.done as f32 / status.total as f32).clamp(0.0, 1.0)
            } else {
                0.0
            };
            div()
                .h(px(4.0))
                .w_full()
                .rounded_full()
                .bg(rgba(fade(th.accent, 0.2)))
                .child(
                    div()
                        .h_full()
                        .w(gpui::relative(share))
                        .rounded_full()
                        .bg(rgba(th.accent)),
                )
        });
        let ready = status.state == state::READY || self.updates.confirm;
        Some(
            div()
                .id("about-updates")
                .flex_none()
                .mx(px(24.0))
                .mt(px(20.0))
                .p(px(16.0))
                .flex()
                .flex_row()
                .items_start()
                .gap(px(12.0))
                .rounded(px(12.0))
                .bg(rgba(fade(th.accent, if th.dark { 0.16 } else { 0.07 })))
                .child(div().mt(px(1.0)).child(icon(
                    if ready { "download" } else { "refresh" },
                    th.accent,
                    22.0,
                )))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .text_size(px(15.0))
                                .line_height(px(21.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(title),
                        )
                        .children(detail.map(|detail| {
                            div()
                                .text_size(px(14.0))
                                .line_height(px(21.0))
                                .text_color(rgba(th.text_dim))
                                .child(detail)
                        }))
                        .children(problem.map(|problem| {
                            div()
                                .text_size(px(14.0))
                                .line_height(px(21.0))
                                .text_color(rgba(th.error))
                                .child(problem)
                        }))
                        .children(progress)
                        .children(buttons.map(|b| div().mt(px(4.0)).child(b))),
                )
                .into_any_element(),
        )
    }
}

/// How much of the download is done, in whole percent.
fn percent(status: &UpdateStatus) -> u64 {
    (status.done * 100)
        .checked_div(status.total)
        .unwrap_or(0)
        .min(100)
}
