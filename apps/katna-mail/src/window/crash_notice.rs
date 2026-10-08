// SPDX-License-Identifier: GPL-3.0-or-later

//! "Katna Mail closed unexpectedly last time": a note at the bottom of the
//! window after Katna Mail or the daemon crashed, with the report to view
//! or copy (`docs/ARCHITECTURE.md` §19.2). The reports stay on this
//! computer; nothing is sent.

use crate::widgets::Tip as _;
use gpui::{AnyElement, ClipboardItem, Context, Window, div, prelude::*, rgba};
use katna_core::crash::{self, Report};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{elevation, icon};

/// The programs whose crashes the note tells about.
const APPS: [&str; 2] = ["katna-mail", "katna-daemon"];

/// A crash report the user has not seen yet.
pub(super) struct CrashNotice {
    /// The newest one.
    report: Report,
    /// How many older unseen reports there are besides it.
    more: usize,
    shown: Spring,
}

impl CrashNotice {
    fn text(&self) -> String {
        let more = self.more;
        match self.report.app.as_str() {
            "katna-daemon" => tr!("crash-daemon", more = more),
            _ => tr!("crash-mail", more = more),
        }
    }
}

impl MailWindow {
    /// Picks up native crashes from `systemd-coredump`, then shows the note
    /// if a report is new. Runs in the background: `coredumpctl` can take
    /// a moment.
    pub(super) fn check_crashes(&mut self, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        let found = cx.background_executor().spawn(async move {
            crash::collect_core_dumps(&paths, &APPS);
            crash::unseen(&paths.crash_dir())
        });
        cx.spawn(async move |this, cx| {
            let unseen: Vec<Report> = found
                .await
                .into_iter()
                .filter(|report| APPS.contains(&report.app.as_str()))
                .collect();
            let Some(report) = unseen.first().cloned() else {
                return;
            };
            this.update(cx, |this, cx| {
                this.saved_reports = None;
                let mut shown = Spring::new(motion::SLIDE, 0.0);
                shown.set(1.0);
                this.crash_notice = Some(CrashNotice {
                    report,
                    more: unseen.len() - 1,
                    shown,
                });
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Hides the note and remembers that its reports were seen.
    fn close_crash_notice(&mut self, cx: &mut Context<Self>) {
        let Some(notice) = &mut self.crash_notice else {
            return;
        };
        if let Err(err) = crash::mark_seen(&self.paths.crash_dir(), &notice.report) {
            tracing::warn!(%err, "could not remember the crash report as seen");
        }
        notice.shown.set(0.0);
        cx.notify();
    }

    fn view_crash_report(&mut self, cx: &mut Context<Self>) {
        let Some(notice) = &self.crash_notice else {
            return;
        };
        cx.open_with_system(&notice.report.path);
        self.close_crash_notice(cx);
    }

    fn copy_crash_report(&mut self, cx: &mut Context<Self>) {
        let Some(notice) = &self.crash_notice else {
            return;
        };
        match notice.report.read() {
            Ok(text) => {
                cx.write_to_clipboard(ClipboardItem::new_string(text));
                self.close_crash_notice(cx);
                self.show_snackbar(tr!("feedback-copied"), None, cx);
            }
            Err(err) => {
                self.show_snackbar(
                    tr!("feedback-read-failed", error = err.to_string()),
                    None,
                    cx,
                );
            }
        }
    }

    pub(super) fn render_crash_notice(
        &mut self,
        th: &Theme,
        window: &Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let notice = self.crash_notice.as_mut()?;
        let s = notice.shown.tick(window, reduce);
        if notice.shown.target() == 0.0 && notice.shown.settled() {
            self.crash_notice = None;
            return None;
        }
        let s = s.max(0.0);
        let text = notice.text();
        // Same place and look as the snackbar, and above it while one shows.
        let shape = self.layout.shape;
        let edge = lerp(24.0, 8.0, shape.phone);
        let above = if self.snackbar.is_some() { 64.0 } else { 0.0 };
        let accent = if th.dark { th.nav_selected } else { 0xa8c7faff };
        let action = |id: &'static str, label: String| {
            div()
                .id(id)
                .flex_none()
                .px(px(12.0))
                .py(px(8.0))
                .rounded(px(4.0))
                .text_color(rgba(accent))
                .font_weight(gpui::FontWeight::MEDIUM)
                .cursor_pointer()
                .hover(|s| s.bg(rgba(0xffffff1f)))
                .child(label)
        };
        Some(
            div()
                .absolute()
                .left(px(edge))
                .when(shape.is_phone(), |d| d.right(px(edge)))
                .bottom(px(shape.bottom_bar() + above + lerp(-12.0, edge, s)))
                .opacity(s.min(1.0))
                .min_w(px(288.0))
                .max_w(px(640.0))
                .pl(px(16.0))
                .pr(px(4.0))
                .py(px(6.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(8.0))
                .rounded(px(6.0))
                .bg(rgba(th.snackbar))
                .text_color(rgba(th.snackbar_text))
                .text_size(px(14.0))
                .shadow(elevation(th, 3.0))
                .child(
                    self.copyable(text, th)
                        .flex_1()
                        .min_w(px(180.0))
                        .py(px(8.0)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .child(
                            action("crash-view", tr!("crash-view"))
                                .tip(tr!("crash-view-tooltip"), th)
                                .on_click(cx.listener(|this, _, _, cx| this.view_crash_report(cx))),
                        )
                        .child(
                            action("crash-copy", tr!("crash-copy"))
                                .tip(tr!("feedback-copy-tooltip"), th)
                                .on_click(cx.listener(|this, _, _, cx| this.copy_crash_report(cx))),
                        )
                        .child(
                            div()
                                .id("crash-close")
                                .size(px(36.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(0xffffff1f)))
                                .tip(tr!("crash-close"), th)
                                .on_click(cx.listener(|this, _, _, cx| this.close_crash_notice(cx)))
                                .child(icon("close", th.snackbar_text, 18.0)),
                        ),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notice(app: &str, more: usize) -> CrashNotice {
        CrashNotice {
            report: Report {
                name: format!("20260927T031603Z-{app}-1.txt"),
                path: "/tmp/r.txt".into(),
                app: app.into(),
            },
            more,
            shown: Spring::new(motion::SLIDE, 1.0),
        }
    }

    #[test]
    fn says_what_crashed() {
        assert_eq!(
            notice("katna-mail", 0).text(),
            "Katna Mail closed unexpectedly last time."
        );
        assert_eq!(
            notice("katna-daemon", 1).text(),
            "Katna's background service stopped unexpectedly. One more crash report is saved."
        );
        assert_eq!(
            notice("katna-mail", 3).text(),
            "Katna Mail closed unexpectedly last time. 3 more crash reports are saved."
        );
    }
}
