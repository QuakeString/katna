// SPDX-License-Identifier: GPL-3.0-or-later

//! When an update leaves Katna unable to start: safe mode, restoring the
//! copies made before the update, and the debug report
//! (`docs/ARCHITECTURE.md` §21.2, "After an update").
//!
//! - The daemon decides safe mode and records it in `health.toml`; this
//!   window reads it there, so it knows even when the daemon can't start.
//! - In safe mode an amber line at the top of the list has Try again,
//!   Restore and Details. Stored mail stays readable; nothing syncs.
//! - Only the daemon writes the databases, so Try again and Restore leave
//!   a request in `safe-mode-request.toml` and restart the daemon, which
//!   runs it before it opens anything.
//! - A restore first moves the data as it is into a folder, so nothing is
//!   lost; a note says it is done, with Show folder.

use std::path::Path;
use std::time::Duration;

use gpui::{
    AnyElement, ClipboardItem, Context, FontWeight, KeyDownEvent, Task, Window, div, prelude::*,
    rgba,
};
use katna_core::Paths;
use katna_core::health::{Health, Request};
use katna_i18n::{format, tr};
use katna_store::restore::{RestorePoint, restore_points};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};

use super::MailWindow;
use super::problems::LineClick;
use crate::daemon::Command;
use crate::theme::{Theme, fade};
use crate::widgets::{ButtonStyle, FocusRing, button, icon, radio, spinner};

/// The Restore dialog's width.
const DIALOG_WIDTH: f32 = 520.0;
/// How long a restart may take to let go of the bus name.
const STOP_WAIT: Duration = Duration::from_secs(10);
/// A restore this recent is told of when the window opens.
const RECENT: i64 = 10 * 60;

/// What this window asked of the daemon and waits for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Working {
    TryingAgain,
    Restoring { from: i64 },
}

/// The Restore dialog.
struct RestoreDialog {
    shown: Spring,
    closing: bool,
    focused: bool,
    points: Vec<RestorePoint>,
    picked: usize,
}

#[derive(Default)]
pub(super) struct SafeMode {
    health: Health,
    working: Option<Working>,
    dialog: Option<RestoreDialog>,
    /// The restore already told of, by when it was done.
    noted: i64,
    _load: Option<Task<()>>,
    _request: Option<Task<()>>,
}

/// The local day and time of Unix second `at`, as the dialog and lines say
/// it.
fn when(at: i64) -> String {
    jiff::Timestamp::from_second(at)
        .map(|at| format::day_month_time(at.to_zoned(jiff::tz::TimeZone::system()).datetime()))
        .unwrap_or_default()
}

/// What a restore point puts back, in words.
fn covers(point: &RestorePoint) -> String {
    let names = point.names();
    let parts: Vec<String> = [
        ("mail.db", "safe-restore-mail"),
        ("pim.db", "safe-restore-pim"),
        ("blobs.db", "safe-restore-blobs"),
    ]
    .into_iter()
    .filter(|(name, _)| names.iter().any(|n| n == name))
    .map(|(_, id)| tr!(id))
    .collect();
    parts.join(" · ")
}

/// Unix seconds now.
fn now() -> i64 {
    jiff::Timestamp::now().as_second()
}

impl MailWindow {
    /// Reads `health.toml` again: whether the daemon is in safe mode, and
    /// whether a restore just finished.
    pub(super) fn load_health(&mut self, cx: &mut Context<Self>) {
        let file = self.paths.health_file();
        self.safe._load = Some(cx.spawn(async move |this, cx| {
            let health = cx
                .background_executor()
                .spawn(async move { Health::load(&file) })
                .await;
            this.update(cx, |this, cx| this.health_loaded(health, cx))
                .ok();
        }));
    }

    fn health_loaded(&mut self, health: Health, cx: &mut Context<Self>) {
        let restoring = matches!(self.safe.working, Some(Working::Restoring { .. }));
        if let Some(restored) = &health.restored
            && restored.at > self.safe.noted
            && (restoring || now() - restored.at < RECENT)
        {
            self.safe.noted = restored.at;
            if restored.error.is_none() {
                // Its databases are new files now.
                self.reopen(cx);
            }
            match &restored.error {
                None => self.show_snackbar_action(
                    tr!("safe-restored", when = when(restored.from)),
                    tr!("safe-show-folder"),
                    Command::RevealPath(restored.saved.clone()),
                    cx,
                ),
                Some(err) => {
                    self.show_snackbar(tr!("safe-restore-failed", error = err.clone()), None, cx)
                }
            }
        }
        self.safe.health = health;
        self.safe.working = None;
        cx.notify();
    }

    /// Whether the daemon is in safe mode, as `health.toml` last said.
    pub(super) fn in_safe_mode(&self) -> bool {
        self.safe.health.safe_mode
    }

    /// The line at the top of the list for safe mode: grey while a restore
    /// runs, amber while in safe mode. `None` when the service's own lines
    /// say it (starting again after Try again).
    pub(super) fn render_safe_line(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        match self.safe.working {
            Some(Working::Restoring { from }) => {
                return Some(
                    self.problem_line(
                        "safe-restoring".into(),
                        spinner("safe-spinner", th.text_dim, 16.0),
                        tr!("safe-restoring", when = when(from)),
                        Vec::new(),
                        false,
                        None,
                        th,
                        cx,
                    )
                    .into_any(),
                );
            }
            Some(Working::TryingAgain) => return None,
            None => {}
        }
        if !self.in_safe_mode() || self.service_starting() {
            return None;
        }
        let again: LineClick = Box::new(|this, _, _, cx| this.safe_try_again(cx));
        let restore: LineClick = Box::new(|this, _, _, cx| this.open_restore(cx));
        let details: LineClick = Box::new(|this, _, _, cx| this.open_debug_report(cx));
        Some(
            self.problem_line(
                "safe-mode".into(),
                icon("warning", th.warning, 18.0),
                tr!("safe-line"),
                vec![
                    (tr!("safe-try-again"), again),
                    (tr!("safe-restore"), restore),
                    (tr!("problems-details"), details),
                ],
                false,
                None,
                th,
                cx,
            )
            .into_any(),
        )
    }

    /// Try again: the daemon starts normally, forgetting the failed starts.
    fn safe_try_again(&mut self, cx: &mut Context<Self>) {
        self.close_service_details(cx);
        self.safe.working = Some(Working::TryingAgain);
        self.send_safe_request(Request::TryAgain, cx);
    }

    /// Leaves `request` for the daemon, stops it if it runs, then starts it.
    fn send_safe_request(&mut self, request: Request, cx: &mut Context<Self>) {
        let file = self.paths.safe_mode_request_file();
        self.safe._request = Some(cx.spawn(async move |this, cx| {
            let executor = cx.background_executor().clone();
            let sent = cx
                .background_executor()
                .spawn(async move {
                    request.write(&file).map_err(|err| err.to_string())?;
                    let connection = katna_dbus::session()
                        .await
                        .map_err(|err| format!("D-Bus: {err}"))?;
                    if katna_dbus::daemon_running(&connection).await {
                        crate::daemon::restart_daemon(&connection).await?;
                        let since = std::time::Instant::now();
                        while katna_dbus::daemon_running(&connection).await
                            && since.elapsed() < STOP_WAIT
                        {
                            executor.timer(Duration::from_millis(200)).await;
                        }
                    }
                    Ok::<(), String>(())
                })
                .await;
            this.update(cx, |this, cx| {
                if let Err(err) = sent {
                    tracing::warn!(%err, "safe mode request");
                }
                this.start_service(false, cx);
            })
            .ok();
        }));
        cx.notify();
    }

    fn open_restore(&mut self, cx: &mut Context<Self>) {
        self.close_service_details(cx);
        let paths = self.paths.clone();
        self.safe._request = Some(cx.spawn(async move |this, cx| {
            let points = cx
                .background_executor()
                .spawn(async move { restore_points(&paths) })
                .await;
            this.update(cx, |this, cx| {
                let mut shown = Spring::new(motion::SMOOTH, 0.0);
                shown.set(1.0);
                this.safe.dialog = Some(RestoreDialog {
                    shown,
                    closing: false,
                    focused: false,
                    points,
                    picked: 0,
                });
                cx.notify();
            })
            .ok();
        }));
    }

    /// Closes the Restore dialog. Returns whether it was open.
    pub(super) fn close_restore(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(dialog) = self.safe.dialog.as_mut().filter(|d| !d.closing) else {
            return false;
        };
        dialog.closing = true;
        dialog.shown.set(0.0);
        cx.notify();
        true
    }

    /// Whether the Restore dialog is open.
    pub(super) fn restore_open(&self) -> bool {
        self.safe.dialog.is_some()
    }

    fn confirm_restore(&mut self, cx: &mut Context<Self>) {
        let Some(from) = self
            .safe
            .dialog
            .as_ref()
            .and_then(|d| d.points.get(d.picked))
            .map(|p| p.made_at)
        else {
            return;
        };
        self.close_restore(cx);
        // Windows can't move a file another program has open: let go of
        // the store until the daemon is back on the restored one.
        #[cfg(windows)]
        {
            self.mail = Err(crate::data::OpenError::Migrating(String::new()));
        }
        self.safe.working = Some(Working::Restoring { from });
        self.send_safe_request(Request::Restore { from }, cx);
    }

    /// Details: the debug report in the service's Details dialog.
    fn open_debug_report(&mut self, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        self.safe._request = Some(cx.spawn(async move |this, cx| {
            let report = cx
                .background_executor()
                .spawn(async move { debug_report(&paths) })
                .await;
            this.update(cx, |this, cx| this.open_report_details(report, cx))
                .ok();
        }));
    }

    /// Copies the debug report: About's button, any time.
    pub(super) fn copy_debug_report(&mut self, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        self.safe._request = Some(cx.spawn(async move |this, cx| {
            let report = cx
                .background_executor()
                .spawn(async move { debug_report(&paths) })
                .await;
            this.update(cx, |this, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(report));
                this.show_snackbar(tr!("safe-report-copied"), None, cx);
            })
            .ok();
        }));
    }

    /// The debug report's Restore… button.
    pub(super) fn report_restore(&mut self, cx: &mut Context<Self>) {
        self.open_restore(cx);
    }

    /// The Restore dialog: the copies made before the last updates, newest
    /// first, with what a restore keeps.
    pub(super) fn render_restore(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let width = DIALOG_WIDTH.min(self.room_width() - 32.0);
        let dialog = self.safe.dialog.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.safe.dialog = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        if !dialog.focused {
            dialog.focused = true;
            window.focus(&self.dialog_focus, cx);
        }
        let picked = dialog.picked;
        let empty = dialog.points.is_empty();
        let options: Vec<AnyElement> = dialog
            .points
            .iter()
            .enumerate()
            .map(|(ix, point)| {
                let on = ix == picked;
                div()
                    .id(("safe-restore-point", ix))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S3))
                    .px(px(space::S4))
                    .py(px(space::S3))
                    .when(ix > 0, |d| d.border_t_1().border_color(rgba(th.divider)))
                    .when(on, |d| d.bg(rgba(th.row_selected)))
                    .when(!on, |d| d.hover(|d| d.bg(rgba(th.hover))))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(dialog) = &mut this.safe.dialog {
                            dialog.picked = ix;
                            cx.notify();
                        }
                    }))
                    .child(radio(if on { 1.0 } else { 0.0 }, th))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .min_w_0()
                            .child(div().text_size(px(text::BODY)).child(when(point.made_at)))
                            .child(
                                div()
                                    .text_size(px(text::CAPTION))
                                    .text_color(rgba(th.text_faint))
                                    .child(covers(point)),
                            ),
                    )
                    .into_any_element()
            })
            .collect();
        let header = div()
            .flex()
            .flex_col()
            .px(px(space::S6))
            .pt(px(space::S6))
            .child(
                self.copyable(tr!("safe-restore-title"), th)
                    .text_size(px(text::TITLE))
                    .line_height(px(28.0)),
            )
            .child(
                div()
                    .mt(px(space::S3))
                    .text_size(px(text::BODY))
                    .text_color(rgba(th.text_dim))
                    .child(if empty {
                        tr!("safe-restore-none")
                    } else {
                        tr!("safe-restore-body")
                    }),
            );
        let list = (!empty).then(|| {
            div()
                .mt(px(space::S4))
                .mx(px(space::S6))
                .flex()
                .flex_col()
                .rounded(px(radius::MD))
                .border_1()
                .border_color(rgba(th.outline))
                .overflow_hidden()
                .children(options)
        });
        let keep = (!empty).then(|| {
            div()
                .mt(px(space::S4))
                .mx(px(space::S6))
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S3))
                .text_size(px(text::CAPTION))
                .text_color(rgba(th.text_dim))
                .child(icon("shield-check", th.text_faint, 16.0))
                .child(div().min_w_0().child(tr!("safe-restore-keep")))
        });
        let buttons = div()
            .px(px(space::S6))
            .pt(px(space::S6))
            .pb(px(space::S5))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .justify_end()
            .gap(px(space::S3))
            .child(
                button("safe-restore-cancel", ButtonStyle::Text, th)
                    .focus_ring(th)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.close_restore(cx);
                    }))
                    .child(tr!("safe-restore-cancel")),
            )
            .when(!empty, |d| {
                d.child(
                    button("safe-restore-confirm", ButtonStyle::Filled, th)
                        .focus_ring_filled(th)
                        .on_click(cx.listener(|this, _, _, cx| this.confirm_restore(cx)))
                        .child(tr!("safe-restore")),
                )
            });
        let focus = self.dialog_focus.clone();
        let card = div()
            .id("safe-restore")
            .track_focus(&focus)
            .map(|d| super::popovers::keep_tab_inside(d, &focus))
            .on_key_down(cx.listener(move |this, event: &KeyDownEvent, window, cx| {
                let stroke = &event.keystroke;
                if !focus.is_focused(window) || stroke.modifiers.modified() {
                    return;
                }
                let Some(dialog) = &mut this.safe.dialog else {
                    return;
                };
                let last = dialog.points.len().saturating_sub(1);
                match stroke.key.as_str() {
                    "enter" => {
                        cx.stop_propagation();
                        this.confirm_restore(cx);
                    }
                    "down" => {
                        dialog.picked = (dialog.picked + 1).min(last);
                        cx.notify();
                    }
                    "up" => {
                        dialog.picked = dialog.picked.saturating_sub(1);
                        cx.notify();
                    }
                    _ => {}
                }
            }))
            .occlude()
            .w(px(width))
            .max_h_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .map(|d| crate::widgets::dialog(d, th, th.surface))
            .text_color(rgba(th.text))
            .font_weight(FontWeight::NORMAL)
            .child(header)
            .children(list)
            .children(keep)
            .child(buttons);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .p(px(space::S4))
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("safe-restore-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_restore(cx);
                        })),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}

/// Everything a bug report needs about Katna's state: versions, safe mode,
/// the self-check, the backups, the end of the service's log and the
/// crash reports. No mail, addresses or passwords.
pub(super) fn debug_report(paths: &Paths) -> String {
    let health = Health::load(&paths.health_file());
    let mut out = format!(
        "Katna debug report · {}\nKatna Mail {} · katna-daemon {} · {} {}\n",
        when(now()),
        crate::whats_new::VERSION,
        if health.version.is_empty() {
            "?"
        } else {
            &health.version
        },
        std::env::consts::OS,
        std::env::consts::ARCH,
    );
    out.push_str(&format!(
        "Safe mode: {}",
        if health.safe_mode { "on" } else { "off" }
    ));
    if health.safe_mode {
        out.push_str(&format!(" (after {} failed starts)", health.failed_starts));
    }
    out.push('\n');
    if health.checked_at > 0 {
        out.push_str(&format!(
            "\nSelf-check of {} at {}\n",
            health.version,
            when(health.checked_at)
        ));
        for (what, result) in &health.checks {
            out.push_str(&format!("  {what}: {result}\n"));
        }
    }
    if let Some(restored) = &health.restored {
        out.push_str(&format!(
            "\nRestored at {} from {}: {}\n",
            when(restored.at),
            when(restored.from),
            restored.error.as_deref().unwrap_or("ok")
        ));
    }
    let points = restore_points(paths);
    out.push_str("\nBackups:");
    if points.is_empty() {
        out.push_str(" none");
    }
    for point in &points {
        let versions: Vec<String> = point
            .copies
            .iter()
            .filter_map(|(db, backup)| {
                let name = db.file_name()?.to_string_lossy().into_owned();
                Some(format!("{name} v{}", backup.version))
            })
            .collect();
        out.push_str(&format!(
            "\n  {} ({})",
            when(point.made_at),
            versions.join(", ")
        ));
    }
    out.push('\n');
    if let Some(log) = super::service::log_tail() {
        out.push_str("\nRecent log:\n");
        out.push_str(log.trim_end());
        out.push('\n');
    }
    let week = std::time::SystemTime::now() - Duration::from_secs(7 * 24 * 3600);
    let crashes = std::fs::read_dir(paths.crash_dir())
        .map(|entries| {
            entries
                .filter_map(|e| e.ok()?.metadata().ok()?.modified().ok())
                .filter(|modified| *modified > week)
                .count()
        })
        .unwrap_or(0);
    out.push_str(&format!("\nCrash reports in the last 7 days: {crashes}"));
    out
}

/// Opens `path` in the file manager: the restored note's Show folder.
pub(super) fn reveal(path: &str, cx: &mut Context<MailWindow>) {
    cx.reveal_path(Path::new(path));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_report_says_safe_mode_and_the_backups() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut health = Health::default();
        health.begin_start(1);
        health.begin_start(2);
        health.begin_start(3);
        health.begin_start(4);
        health.checked(
            "r9",
            5,
            std::collections::BTreeMap::from([("mail.db".to_owned(), "malformed".to_owned())]),
        );
        health.save(&paths.health_file()).unwrap();
        let report = debug_report(&paths);
        assert!(report.contains("Safe mode: on (after 3 failed starts)"));
        assert!(report.contains("mail.db: malformed"));
        assert!(report.contains("Backups: none"));
        assert!(report.contains(crate::whats_new::VERSION));
    }
}
