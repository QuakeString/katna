// SPDX-License-Identifier: GPL-3.0-or-later

//! The Update dialog (`docs/ARCHITECTURE.md` §21.2), from Help > Check for
//! Updates and the notification that an update is ready: the installed
//! version beside the new one, with when each was built, its commit and
//! the download's size, the new version's What's new and every change
//! since the installed one, the download's progress, and Update and
//! restart, which installs the download behind the system's password
//! prompt and opens Katna Mail again where it was. The daemon checks and
//! downloads.

use futures_lite::StreamExt;
use gpui::{
    Animation, AnimationExt, AnyElement, Context, FocusHandle, FontWeight, KeyDownEvent,
    MouseButton, SharedString, Task, Window, div, prelude::*, relative, rgba,
};
use jiff::tz::TimeZone;
use katna_core::update::Manifest;
use katna_dbus::zbus::Connection;
use katna_dbus::{UpdateStatus, update_state as state};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{px, unpx};

use super::{CheckForUpdates, MailWindow, PANEL_RADIUS};
use crate::theme::{Theme, fade};
use crate::updater::{self, InstallError};
use crate::widgets::{FocusRing, elevation, filled_button, icon, outlined_button};
use crate::{daemon, format, whats_new};

const WIDTH: f32 = 560.0;

/// Updates as the window shows them.
#[derive(Default)]
pub(super) struct Updates {
    /// From the daemon; `None` until it answered.
    status: Option<UpdateStatus>,
    /// What the version on offer brings, from the daemon.
    details: Option<Manifest>,
    /// The password prompt is open, or the package being installed.
    installing: bool,
    /// Why the last install did not happen.
    problem: Option<String>,
    /// The Update dialog, while it shows.
    dialog: Option<Dialog>,
    watch: Option<Task<()>>,
    install: Option<Task<()>>,
}

struct Dialog {
    focus: FocusHandle,
    closing: bool,
    shown: Spring,
    /// Every change is listed, not only the count.
    all_changes: bool,
}

impl MailWindow {
    /// Follows where an update stands, and what the version on offer
    /// brings.
    pub(super) fn watch_updates(&mut self, connection: Connection, cx: &mut Context<Self>) {
        self.updates.watch = Some(cx.spawn(async move |this, cx| {
            let Ok(mut changes) = daemon::update_changes(&connection).await else {
                return;
            };
            loop {
                let status = daemon::update_status(&connection).await.ok();
                let offered = status
                    .as_ref()
                    .filter(|s| !s.version.is_empty())
                    .map(|s| s.version.clone());
                let known = this
                    .read_with(cx, |this, _| {
                        this.updates.details.as_ref().map(|d| d.version.clone())
                    })
                    .ok()
                    .flatten();
                // An older daemon has no details: the dialog shows less.
                let details = match &offered {
                    Some(version) if known.as_ref() != Some(version) => {
                        daemon::update_details(&connection).await.ok().flatten()
                    }
                    _ => None,
                };
                let alive = this
                    .update(cx, |this, cx| {
                        if let Some(details) = details {
                            this.updates.details = Some(details);
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

    /// Help > Check for Updates: opens the dialog and checks, unless a
    /// check or download is under way or an update waits.
    pub(super) fn check_for_updates_action(
        &mut self,
        _: &CheckForUpdates,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_update_dialog(window, cx);
        let busy = self.updates.status.as_ref().is_some_and(|s| {
            [
                state::CHECKING,
                state::AVAILABLE,
                state::DOWNLOADING,
                state::READY,
                state::UNSUPPORTED,
            ]
            .contains(&s.state.as_str())
        });
        if !busy {
            self.check_for_update(cx);
        }
    }

    /// Opens the dialog on the update (the notification's Update button).
    pub(super) fn show_update(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open_update_dialog(window, cx);
    }

    fn open_update_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_open = false;
        self.updates.problem = None;
        if self.update_dialog_open() {
            return;
        }
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.updates.dialog = Some(Dialog {
            focus,
            closing: false,
            shown,
            all_changes: false,
        });
        cx.notify();
    }

    /// The Update dialog is open and not on its way out.
    pub(super) fn update_dialog_open(&self) -> bool {
        self.updates.dialog.as_ref().is_some_and(|d| !d.closing)
    }

    /// Closes the dialog; a download goes on.
    pub(super) fn close_update_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.updates.installing {
            return;
        }
        if let Some(dialog) = &mut self.updates.dialog
            && !dialog.closing
        {
            dialog.closing = true;
            dialog.shown.set(0.0);
            window.focus(&self.list_focus, cx);
        }
        cx.notify();
    }

    fn update_dialog_key(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Escape reaches `popovers` first.
        if event.keystroke.key == "escape" {
            self.close_update_dialog(window, cx);
            cx.stop_propagation();
        }
    }

    fn check_for_update(&mut self, cx: &mut Context<Self>) {
        self.ask_daemon(cx, async |connection| {
            daemon::check_for_update(&connection).await
        });
    }

    fn download_update(&mut self, cx: &mut Context<Self>) {
        self.ask_daemon(cx, async |connection| {
            daemon::download_update(&connection).await
        });
    }

    /// Asks the daemon to check or download; the dialog says why it could
    /// not.
    fn ask_daemon(
        &mut self,
        cx: &mut Context<Self>,
        ask: impl AsyncFnOnce(Connection) -> Result<(), String> + 'static,
    ) {
        self.updates.problem = None;
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let asked = match connection {
                Some(connection) => ask(connection).await,
                None => Err(tr!("update-dialog-no-service")),
            };
            if let Err(err) = asked {
                tracing::warn!(%err, "cannot reach the daemon about updates");
                this.update(cx, |this, cx| {
                    this.updates.problem = Some(err);
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
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

    pub(super) fn render_update_dialog(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let dialog = self.updates.dialog.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.updates.dialog = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let dialog = self.updates.dialog.as_ref()?;
        let phone = self.layout.shape.is_phone();
        let vw = unpx(window.viewport_size().width);
        let width = if phone { vw } else { WIDTH.min(vw - 48.0) };

        let status = self.updates.status.clone().unwrap_or_else(|| UpdateStatus {
            state: state::IDLE.to_owned(),
            ..UpdateStatus::default()
        });
        let busy = self.updates.installing;
        let offered = !status.version.is_empty()
            && [
                state::AVAILABLE,
                state::DOWNLOADING,
                state::READY,
                state::DOWNLOAD_FAILED,
            ]
            .contains(&status.state.as_str());
        let details = self
            .updates
            .details
            .as_ref()
            .filter(|d| offered && d.version == status.version);

        // What happens now, and what comes next.
        let version = status.version.as_str();
        let (glyph, tone, title, detail): (&str, u32, String, Option<String>) = if busy {
            (
                "download",
                th.accent,
                tr!("about-update-installing", version = version),
                Some(tr!("about-update-installing-detail")),
            )
        } else {
            match status.state.as_str() {
                state::UNSUPPORTED => (
                    "info",
                    th.accent,
                    tr!("update-dialog-title"),
                    Some(tr!("about-update-unsupported")),
                ),
                state::CHECKING => ("refresh", th.accent, tr!("about-update-checking"), None),
                // The emoji stays out of the translation, so every
                // language gets it.
                state::UP_TO_DATE => (
                    "check-circle",
                    th.accent,
                    format!("{} {UP_TO_DATE_EMOJI}", tr!("about-update-up-to-date")),
                    checked_ago(status.checked).map(|ago| tr!("update-dialog-checked", ago = ago)),
                ),
                state::AVAILABLE => (
                    "download",
                    th.accent,
                    tr!("about-update-available", version = version),
                    None,
                ),
                state::DOWNLOADING => (
                    "download",
                    th.accent,
                    tr!(
                        "about-update-downloading",
                        version = version,
                        percent = percent(&status)
                    ),
                    Some(tr!("update-dialog-downloading-detail")),
                ),
                state::READY => (
                    "download",
                    th.accent,
                    tr!("about-update-ready", version = version),
                    Some(tr!("about-update-confirm-detail")),
                ),
                state::FAILED => (
                    "warning",
                    th.error,
                    tr!("about-update-check-failed"),
                    (!status.detail.is_empty()).then(|| status.detail.clone()),
                ),
                state::DOWNLOAD_FAILED => (
                    "warning",
                    th.error,
                    tr!("about-update-download-failed", version = version),
                    (!status.detail.is_empty()).then(|| status.detail.clone()),
                ),
                _ => ("refresh", th.accent, tr!("about-update-not-checked"), None),
            }
        };
        let problem = self.updates.problem.clone().filter(|_| !busy);

        let header = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(24.0))
            .flex()
            .flex_row()
            // The title and the lines under it, together, centred on the
            // icon.
            .items_center()
            .gap(px(16.0))
            .child(
                div()
                    .flex_none()
                    .size(px(48.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(fade(tone, if th.dark { 0.2 } else { 0.1 })))
                    .child(icon(glyph, tone, 26.0)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(div().text_size(px(20.0)).line_height(px(28.0)).child(title))
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
                    })),
            );

        // While checking or installing, a bar that runs to and fro where the
        // download's progress goes.
        let working = (busy || status.state == state::CHECKING).then(|| {
            div()
                .flex_none()
                .px(px(24.0))
                .pt(px(16.0))
                .child(working_bar(th))
        });
        let progress = (status.state == state::DOWNLOADING && !busy).then(|| {
            let share = if status.total > 0 {
                (status.done as f32 / status.total as f32).clamp(0.0, 1.0)
            } else {
                0.0
            };
            div()
                .flex_none()
                .px(px(24.0))
                .pt(px(16.0))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .h(px(6.0))
                        .w_full()
                        .rounded_full()
                        .bg(rgba(fade(th.accent, 0.2)))
                        .child(
                            div()
                                .h_full()
                                .w(gpui::relative(share))
                                .rounded_full()
                                .bg(rgba(th.accent)),
                        ),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!(
                            "update-dialog-progress",
                            done = format::size(status.done),
                            total = format::size(status.total)
                        )),
                )
        });

        // The installed version beside the new one.
        let installed = version_tile(
            Tile {
                id: "update-installed",
                label: tr!("update-dialog-installed"),
                version: whats_new::VERSION,
                title: whats_new::TITLE.map(str::to_owned),
                built: whats_new::built(),
                commit: katna_core::update::commit_of(whats_new::VERSION).map(str::to_owned),
                source: source(),
                size: None,
                new: false,
            },
            th,
        );
        let new = offered.then(|| {
            let commit = details
                .map(|d| d.commit.clone())
                .filter(|c| !c.is_empty())
                .or_else(|| katna_core::update::commit_of(version).map(str::to_owned));
            // The newest change is the new build's own commit.
            let title = details
                .and_then(|d| d.changes.first())
                .filter(|change| {
                    commit
                        .as_deref()
                        .is_some_and(|commit| commit.starts_with(&change.commit))
                })
                .map(|change| change.title.clone());
            version_tile(
                Tile {
                    id: "update-new",
                    label: tr!("update-dialog-new"),
                    version,
                    title,
                    built: details.map(|d| d.built).filter(|built| *built > 0),
                    commit,
                    source: None,
                    size: Some(details.map_or(status.total, |d| d.size)).filter(|size| *size > 0),
                    new: true,
                },
                th,
            )
        });
        let versions = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(20.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(12.0))
            .child(installed)
            .children(new);

        // The highlights this build does not have, newest first.
        let highlights: Vec<_> = details
            .map(|d| {
                d.highlights
                    .iter()
                    .filter(|h| !whats_new::has_highlight(&h.name))
                    .collect()
            })
            .unwrap_or_default();
        let whats_new_section = (!highlights.is_empty()).then(|| {
            div()
                .flex_none()
                .px(px(24.0))
                .pt(px(24.0))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(section_title(tr!("update-dialog-whats-new"), th))
                .children(highlights.into_iter().enumerate().map(|(ix, h)| {
                    div()
                        .id(("update-highlight", ix))
                        .p(px(14.0))
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(12.0))
                        .rounded(px(12.0))
                        .bg(rgba(fade(th.accent, if th.dark { 0.12 } else { 0.06 })))
                        .child(div().mt(px(1.0)).child(icon("sparkle", th.accent, 18.0)))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .line_height(px(20.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(h.title.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .line_height(px(19.0))
                                        .text_color(rgba(th.text_dim))
                                        .child(h.text.clone()),
                                ),
                        )
                }))
        });

        // Every commit since the installed one.
        let changes = details.map(|d| d.changes_since(whats_new::VERSION));
        let compare = whats_new::changelog_url_for(Some(whats_new::VERSION), version);
        let all_changes = dialog.all_changes;
        let changes_section = offered.then(|| {
            let (list, complete) = changes.unwrap_or((&[], false));
            let count = list.len();
            let toggle = (count > 0).then(|| {
                div()
                    .id("update-all-changes")
                    .mx(px(-12.0))
                    .px(px(12.0))
                    .py(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(dialog) = &mut this.updates.dialog {
                            dialog.all_changes = !dialog.all_changes;
                        }
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .child(if complete {
                                tr!("update-dialog-changes", count = count)
                            } else {
                                tr!("update-dialog-latest-changes", count = count)
                            }),
                    )
                    .child(icon(
                        if all_changes {
                            "chevron-down"
                        } else {
                            "chevron-right"
                        },
                        th.accent,
                        20.0,
                    ))
            });
            let rows = list
                .iter()
                .enumerate()
                .filter(|_| all_changes)
                .map(|(ix, change)| {
                    div()
                        .id(("update-change", ix))
                        .py(px(5.0))
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(12.0))
                        .child(
                            div()
                                .flex_none()
                                .w(px(64.0))
                                .text_size(px(12.0))
                                .line_height(px(19.0))
                                .text_color(rgba(th.text_dim))
                                .font_family("monospace")
                                .child(change.commit.chars().take(7).collect::<String>()),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .text_size(px(13.0))
                                .line_height(px(19.0))
                                .child(change.title.clone()),
                        )
                });
            div()
                .flex_none()
                .px(px(24.0))
                .pt(px(20.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(section_title(tr!("update-dialog-changes-title"), th))
                .children(toggle)
                .children(rows)
                .child(
                    div().mt(px(8.0)).flex().flex_row().child(
                        outlined_button("update-compare", tr!("update-dialog-compare"), th)
                            .focus_ring(th)
                            .gap(px(8.0))
                            .child(icon("open-external", th.accent, 16.0))
                            .on_click(move |_, _, cx| cx.open_url(&compare)),
                    ),
                )
        });

        let body = div()
            .id("update-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .pb(px(20.0))
            .flex()
            .flex_col()
            .child(header)
            .children(working)
            .children(progress)
            .child(versions)
            .children(whats_new_section)
            .children(changes_section);

        // Close for now, and the one step that moves the update on.
        let close_label = if offered || status.state == state::CHECKING {
            tr!("update-dialog-later")
        } else {
            tr!("update-dialog-close")
        };
        let close = (!busy).then(|| {
            outlined_button("update-close", close_label, th)
                .focus_ring(th)
                .on_click(cx.listener(|this, _, window, cx| this.close_update_dialog(window, cx)))
        });
        let next = if busy {
            None
        } else {
            match status.state.as_str() {
                state::READY => Some(
                    filled_button("update-install", tr!("about-update-restart"), th)
                        .focus_ring_filled(th)
                        .on_click(cx.listener(|this, _, _, cx| this.install_update(cx))),
                ),
                state::AVAILABLE => Some(
                    filled_button("update-download", tr!("about-update-download"), th)
                        .focus_ring_filled(th)
                        .on_click(cx.listener(|this, _, _, cx| this.download_update(cx))),
                ),
                state::DOWNLOAD_FAILED => Some(
                    filled_button("update-retry", tr!("about-update-retry"), th)
                        .focus_ring_filled(th)
                        .on_click(cx.listener(|this, _, _, cx| this.download_update(cx))),
                ),
                state::CHECKING | state::DOWNLOADING | state::UNSUPPORTED => None,
                _ => Some(
                    filled_button("update-check", tr!("about-update-check"), th)
                        .focus_ring_filled(th)
                        .on_click(cx.listener(|this, _, _, cx| this.check_for_update(cx))),
                ),
            }
        };
        let footer = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(12.0))
            .pb(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_end()
            .gap(px(8.0))
            .border_t_1()
            .border_color(rgba(th.divider))
            .children(close)
            .children(next);

        let card = div()
            .id("update-dialog")
            .track_focus(&dialog.focus)
            .map(|d| super::popovers::keep_tab_inside(d, &dialog.focus))
            .on_key_down(cx.listener(Self::update_dialog_key))
            .occlude()
            .w(px(width))
            .when(phone, |d| d.h_full())
            .when(!phone, |d| d.max_h_full().min_h_0())
            .flex()
            .flex_col()
            .overflow_hidden()
            .when(!phone, |d| {
                d.rounded(px(PANEL_RADIUS)).shadow(elevation(th, 3.0))
            })
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(body)
            .child(footer);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .when(!phone, |d| d.p(px(24.0)))
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("update-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(
                            cx.listener(|this, _, window, cx| this.close_update_dialog(window, cx)),
                        ),
                )
                .child(
                    div()
                        .max_h_full()
                        .when(phone, |d| d.h_full())
                        .flex()
                        .flex_col()
                        .opacity(t)
                        .mt(px(lerp(24.0, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }
}

/// A small heading over a part of the dialog.
fn section_title(text: String, th: &Theme) -> impl IntoElement {
    div()
        .text_size(px(12.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_dim))
        .child(text)
}

/// A version's box: what it is, when it was built, its commit (a link to
/// it) and, for the new one, the download's size.
/// After "Katna Mail is up to date".
const UP_TO_DATE_EMOJI: &str = "🎉";

/// When the daemon last checked, as "5 minutes ago".
fn checked_ago(checked: i64) -> Option<String> {
    (checked > 0)
        .then(|| format::ago(checked, jiff::Timestamp::now().as_second()))
        .flatten()
}

/// Where this build came from, for packages that update themselves.
fn source() -> Option<String> {
    match katna_core::update::Package::current() {
        katna_core::update::Package::Arch => Some(tr!("update-dialog-source-arch")),
        katna_core::update::Package::Other => None,
    }
}

/// A bar with no end: a stretch of the accent colour that runs across,
/// as the Add account card shows while the daemon works.
fn working_bar(th: &Theme) -> impl IntoElement {
    div()
        .relative()
        .h(px(6.0))
        .w_full()
        .rounded_full()
        .overflow_hidden()
        .bg(rgba(fade(th.accent, 0.2)))
        .child(
            div()
                .absolute()
                .top_0()
                .h_full()
                .w(relative(0.4))
                .rounded_full()
                .bg(rgba(th.accent))
                .with_animation(
                    "update-working",
                    Animation::new(std::time::Duration::from_millis(1300)).repeat(),
                    |bar, t| bar.left(relative(lerp(-0.4, 1.0, t))),
                ),
        )
}

/// What a version tile shows.
struct Tile<'a> {
    id: &'static str,
    label: String,
    version: &'a str,
    /// The first line of the build's commit: its newest change.
    title: Option<String>,
    built: Option<i64>,
    commit: Option<String>,
    /// Where the installed build came from.
    source: Option<String>,
    size: Option<u64>,
    /// The version on offer: tinted, its number in the accent colour.
    new: bool,
}

fn version_tile(tile: Tile, th: &Theme) -> impl IntoElement {
    let Tile {
        id,
        label,
        version,
        title,
        built,
        commit,
        source,
        size,
        new,
    } = tile;
    let fact = |text: String| {
        div()
            .text_size(px(13.0))
            .line_height(px(19.0))
            .text_color(rgba(th.text_dim))
            .child(text)
    };
    let title = title.map(|title| {
        div()
            .mb(px(4.0))
            .text_size(px(13.0))
            .line_height(px(19.0))
            .child(tr!("update-dialog-latest-change", title = title))
    });
    let built = built
        .and_then(|unix| format::local(unix, &TimeZone::system()))
        .map(|date| tr!("update-dialog-built", date = format::long_date(date)));
    let commit = commit.map(|commit| {
        let short: String = commit.chars().take(7).collect();
        let url = format!("{}/commit/{commit}", env!("CARGO_PKG_REPOSITORY"));
        div()
            .id(SharedString::from(format!("{id}-commit")))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .text_size(px(13.0))
            .line_height(px(19.0))
            .text_color(rgba(th.accent))
            .cursor_pointer()
            .hover(|s| s.underline())
            .on_click(move |_, _, cx| cx.open_url(&url))
            .child(tr!("update-dialog-commit", commit = short))
            .child(icon("open-external", th.accent, 14.0))
    });
    div()
        .id(id)
        .flex_1()
        .min_w(px(200.0))
        .p(px(16.0))
        .flex()
        .flex_col()
        .gap(px(2.0))
        .rounded(px(12.0))
        .bg(rgba(if new {
            fade(th.accent, if th.dark { 0.24 } else { 0.08 })
        } else {
            th.chip
        }))
        .child(
            div()
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(if new { th.accent } else { th.text_dim }))
                .child(label),
        )
        .child(
            div()
                .mt(px(2.0))
                .mb(px(4.0))
                .text_size(px(16.0))
                .line_height(px(22.0))
                .font_weight(FontWeight::MEDIUM)
                .when(new, |v| v.text_color(rgba(th.accent)))
                .child(version.to_owned()),
        )
        .children(title)
        .children(built.map(fact))
        .children(source.map(fact))
        .children(commit)
        .children(size.map(|size| fact(tr!("update-dialog-size", size = format::size(size)))))
}

/// How much of the download is done, in whole percent.
fn percent(status: &UpdateStatus) -> u64 {
    (status.done * 100)
        .checked_div(status.total)
        .unwrap_or(0)
        .min(100)
}
