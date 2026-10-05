// SPDX-License-Identifier: GPL-3.0-or-later

//! When Katna's background service (`katna-daemon`) isn't running: the
//! window starts it by itself and says so only when that takes a while
//! (`docs/DESIGN.md`, Problems).
//!
//! - At start, and whenever the service goes away, the window starts it
//!   (`katna_dbus::start_daemon`), trying again for
//!   [`GIVE_UP_AFTER`]. Stored mail stays readable meanwhile.
//! - After [`WAITING_AFTER`] a grey line at the top of the list says the
//!   service is starting, as the offline line does.
//! - If it went away while the window was open and came back only because
//!   the window started it, a note says so.
//! - If it still won't start, an amber line says so, with Start again and
//!   Details: what went wrong, ready to copy, with the end of its log.
//!
//! The service also draws the tray icon, so that comes back with it.

use std::time::{Duration, Instant};

use futures_lite::StreamExt;
use gpui::{
    AnyElement, ClipboardItem, Context, FontWeight, KeyDownEvent, SharedString, Task, Window, div,
    prelude::*, rgba,
};
use katna_core::ids;
use katna_dbus::zbus::Connection;
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};

use super::MailWindow;
use super::problems::LineClick;
use crate::theme::{Theme, fade};
use crate::widgets::{ButtonStyle, FocusRing, button, icon, spinner};

/// How long the service may take to start before the grey line shows.
const WAITING_AFTER: Duration = Duration::from_secs(10);
/// How long the window keeps trying before the amber line shows.
const GIVE_UP_AFTER: Duration = Duration::from_secs(20);
/// The pauses between tries.
const RETRY: [Duration; 2] = [Duration::from_secs(2), Duration::from_secs(5)];
/// How long the window waits after the service went away before starting
/// it: an update restarts it within this time by itself (migrating the
/// store first, which can take a few seconds), and systemd restarts it 5 s
/// after a crash.
const GRACE: Duration = Duration::from_secs(10);
/// How many lines of the service's log Details shows.
const LOG_LINES: &str = "20";
/// The Details dialog's width.
const DIALOG_WIDTH: f32 = 520.0;

/// Where the service stands, as this window sees it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum State {
    #[default]
    Running,
    /// The window is starting it, since `since`. `note`: it went away
    /// while the window was in use, so say once it's back.
    Starting { since: Instant, note: bool },
    /// It didn't start; `details` says why.
    Failed { details: String },
}

/// The Details dialog.
struct DetailsDialog {
    shown: Spring,
    closing: bool,
    focused: bool,
}

#[derive(Default)]
pub(super) struct Service {
    state: State,
    details: Option<DetailsDialog>,
    _watch: Option<Task<()>>,
    _start: Option<Task<()>>,
    _timer: Option<Task<()>>,
}

impl MailWindow {
    /// Follows the service on `connection`: starts it now if it isn't
    /// running, and again whenever it goes away.
    pub(super) fn watch_service(&mut self, connection: Connection, cx: &mut Context<Self>) {
        self.service._watch = Some(cx.spawn(async move |this, cx| {
            if !katna_dbus::daemon_running(&connection).await {
                this.update(cx, |this, cx| this.start_service(false, cx))
                    .ok();
            }
            let owners = async {
                let dbus = katna_dbus::zbus::fdo::DBusProxy::new(&connection).await?;
                dbus.receive_name_owner_changed_with_args(&[(0, ids::DAEMON_BUS_NAME)])
                    .await
            };
            let mut owners = match owners.await {
                Ok(owners) => owners,
                Err(err) => {
                    tracing::info!("not following the service: {err}");
                    return;
                }
            };
            while let Some(changed) = owners.next().await {
                let gone = changed
                    .args()
                    .map(|args| args.new_owner().is_none())
                    .unwrap_or(false);
                let followed = if gone {
                    // An update restarts it by itself in a moment.
                    cx.background_executor().timer(GRACE).await;
                    if katna_dbus::daemon_running(&connection).await {
                        continue;
                    }
                    this.update(cx, |this, cx| {
                        if this.service.state == State::Running {
                            this.start_service(true, cx);
                        }
                    })
                } else {
                    // Back: started by the window, a terminal or systemd.
                    this.update(cx, |this, cx| this.service_back(cx))
                };
                if followed.is_err() {
                    break;
                }
            }
        }));
    }

    /// Starts the service, trying again until [`GIVE_UP_AFTER`]. `note`:
    /// it went away while the window was in use, so say once it's back.
    fn start_service(&mut self, note: bool, cx: &mut Context<Self>) {
        let since = Instant::now();
        self.service.state = State::Starting { since, note };
        self.service._timer = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(WAITING_AFTER).await;
            this.update(cx, |_, cx| cx.notify()).ok();
        }));
        self.service._start = Some(cx.spawn(async move |this, cx| {
            let mut errors: Vec<String> = Vec::new();
            let mut tries = 0;
            let mut pauses = RETRY.iter().copied().chain(std::iter::repeat(RETRY[1]));
            loop {
                let started = cx
                    .background_executor()
                    .spawn(async move {
                        let connection = katna_dbus::session()
                            .await
                            .map_err(|err| format!("D-Bus: {err}"))?;
                        katna_dbus::start_daemon(&connection).await
                    })
                    .await;
                tries += 1;
                match started {
                    Ok(()) => {
                        this.update(cx, |this, cx| this.service_back(cx)).ok();
                        return;
                    }
                    Err(err) => {
                        tracing::warn!(%err, "cannot start the Katna service");
                        if !errors.contains(&err) {
                            errors.push(err);
                        }
                    }
                }
                let pause = pauses.next().unwrap_or(RETRY[1]);
                if since.elapsed() + pause >= GIVE_UP_AFTER {
                    break;
                }
                cx.background_executor().timer(pause).await;
            }
            let details = cx
                .background_executor()
                .spawn(async move { report(&errors, tries) })
                .await;
            this.update(cx, |this, cx| {
                if matches!(this.service.state, State::Starting { .. }) {
                    this.service.state = State::Failed { details };
                    this.service._timer = None;
                    cx.notify();
                }
            })
            .ok();
        }));
        cx.notify();
    }

    /// The service runs again: the lines go, the window reads what it
    /// missed, and a note says it was started again if it went away while
    /// the window was in use.
    fn service_back(&mut self, cx: &mut Context<Self>) {
        let note = match self.service.state {
            State::Running => return,
            State::Starting { note, .. } => note,
            State::Failed { .. } => false,
        };
        self.service.state = State::Running;
        self.service._start = None;
        self.service._timer = None;
        self.close_service_details(cx);
        if note {
            self.show_snackbar(tr!("service-started-again"), None, cx);
        }
        self.refresh(false, cx);
        self.check_first_sync(cx);
        self.check_problems(cx);
        cx.notify();
    }

    /// Start again: the whole start once more.
    fn start_service_again(&mut self, cx: &mut Context<Self>) {
        self.close_service_details(cx);
        self.start_service(false, cx);
    }

    /// The line at the top of the list while the service isn't running:
    /// grey while it starts (after [`WAITING_AFTER`]), amber once it
    /// didn't.
    pub(super) fn render_service_line(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        match &self.service.state {
            State::Running => None,
            State::Starting { since, .. } => {
                if since.elapsed() < WAITING_AFTER {
                    return None;
                }
                Some(
                    self.problem_line(
                        "service-starting".into(),
                        spinner("service-spinner", th.text_dim, 16.0),
                        tr!("service-starting"),
                        Vec::new(),
                        false,
                        None,
                        th,
                        cx,
                    )
                    .into_any(),
                )
            }
            State::Failed { .. } => {
                let again: LineClick = Box::new(|this, _, _, cx| this.start_service_again(cx));
                let details: LineClick = Box::new(|this, _, _, cx| this.open_service_details(cx));
                Some(
                    self.problem_line(
                        "service-failed".into(),
                        icon("warning", th.warning, 18.0),
                        tr!("service-failed"),
                        vec![
                            (tr!("service-start-again"), again),
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
        }
    }

    fn open_service_details(&mut self, cx: &mut Context<Self>) {
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.service.details = Some(DetailsDialog {
            shown,
            closing: false,
            focused: false,
        });
        cx.notify();
    }

    /// Closes Details. Returns whether it was open.
    pub(super) fn close_service_details(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(dialog) = self.service.details.as_mut().filter(|d| !d.closing) else {
            return false;
        };
        dialog.closing = true;
        dialog.shown.set(0.0);
        cx.notify();
        true
    }

    fn copy_service_details(&mut self, cx: &mut Context<Self>) {
        let State::Failed { details } = &self.service.state else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new_string(details.clone()));
        self.show_snackbar(tr!("feedback-copied"), None, cx);
    }

    /// Whether the Details dialog is open.
    pub(super) fn service_details_open(&self) -> bool {
        self.service.details.is_some()
    }

    /// The Details dialog: what went wrong, selectable, with Copy and
    /// Start again.
    pub(super) fn render_service_details(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let details = match &self.service.state {
            State::Failed { details } => details.clone(),
            _ => String::new(),
        };
        let dialog = self.service.details.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.service.details = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        if !dialog.focused {
            dialog.focused = true;
            window.focus(&self.dialog_focus, cx);
        }
        let body = div()
            .flex()
            .flex_col()
            .px(px(space::S6))
            .pt(px(space::S6))
            .pb(px(space::S5))
            .child(
                self.copyable(tr!("service-details-title"), th)
                    .text_size(px(text::TITLE))
                    .line_height(px(28.0)),
            )
            .child(
                div()
                    .mt(px(space::S3))
                    .text_size(px(text::BODY))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("service-details-body")),
            )
            .child(
                self.copyable(SharedString::from(details), th)
                    .mt(px(space::S4))
                    .px(px(space::S4))
                    .py(px(space::S3))
                    .rounded(px(radius::SM))
                    .bg(rgba(th.on_pane(th.read_row)))
                    .border_1()
                    .border_color(rgba(th.divider))
                    .font_family("monospace")
                    .text_size(px(text::CAPTION))
                    .line_height(px(18.0))
                    .text_color(rgba(th.text_dim)),
            )
            .child(
                div()
                    .mt(px(space::S6))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_end()
                    .gap(px(space::S3))
                    .child(
                        button("service-details-close", ButtonStyle::Text, th)
                            .focus_ring(th)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_service_details(cx);
                            }))
                            .child(tr!("service-details-close")),
                    )
                    .child(
                        button("service-details-copy", ButtonStyle::Outlined, th)
                            .focus_ring(th)
                            .on_click(cx.listener(|this, _, _, cx| this.copy_service_details(cx)))
                            .child(tr!("service-details-copy")),
                    )
                    .child(
                        button("service-details-again", ButtonStyle::Filled, th)
                            .focus_ring_filled(th)
                            .on_click(cx.listener(|this, _, _, cx| this.start_service_again(cx)))
                            .child(tr!("service-start-again")),
                    ),
            );
        let vw = self.room_width();
        let focus = self.dialog_focus.clone();
        let card = div()
            .id("service-details")
            .track_focus(&focus)
            .map(|d| super::popovers::keep_tab_inside(d, &focus))
            .on_key_down(cx.listener(move |this, event: &KeyDownEvent, window, cx| {
                let stroke = &event.keystroke;
                if stroke.key == "enter" && !stroke.modifiers.modified() && focus.is_focused(window)
                {
                    cx.stop_propagation();
                    this.start_service_again(cx);
                }
            }))
            .occlude()
            .w(px(DIALOG_WIDTH.min(vw - 32.0)))
            .flex()
            .flex_col()
            .map(|d| crate::widgets::dialog(d, th, th.surface))
            .text_color(rgba(th.text))
            .font_weight(FontWeight::NORMAL)
            .child(body);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("service-details-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_service_details(cx);
                        })),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}

/// What Details shows and Copy copies: the errors of every try, the end
/// of the service's log where systemd keeps one, and which Katna this is.
fn report(errors: &[String], tries: u32) -> String {
    let mut out = format!("katna-daemon did not start after {tries} tries.\n");
    for err in errors {
        out.push_str(err);
        out.push('\n');
    }
    if let Some(log) = log_tail() {
        out.push_str("\nLast lines of its log:\n");
        out.push_str(log.trim_end());
        out.push('\n');
    }
    out.push_str(&format!(
        "\nKatna Mail {} · {} {}",
        crate::whats_new::VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH
    ));
    out
}

/// The end of the service's log in the systemd journal, if there is one.
#[cfg(not(windows))]
fn log_tail() -> Option<String> {
    let output = std::process::Command::new("journalctl")
        .args([
            "--user",
            "--unit",
            "katna-daemon.service",
            "--lines",
            LOG_LINES,
            "--no-pager",
            "--output",
            "short",
        ])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    // journalctl's own "-- No entries --" says there is none.
    let empty = text
        .lines()
        .all(|line| line.trim().is_empty() || line.starts_with("-- "));
    (output.status.success() && !empty).then_some(text)
}

#[cfg(windows)]
fn log_tail() -> Option<String> {
    let _ = LOG_LINES;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_says_what_failed_and_which_katna() {
        let text = report(&["starting in.invenia.katna.Daemon: refused".into()], 3);
        assert!(text.starts_with("katna-daemon did not start after 3 tries.\n"));
        assert!(text.contains("refused"));
        assert!(text.contains(crate::whats_new::VERSION));
    }
}
