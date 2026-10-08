// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > User feedback (`docs/ARCHITECTURE.md` §19.2): whether crash
//! reports are saved on this computer and whether they are sent to help
//! fix them, the saved ones to view, copy or delete, anonymous usage
//! statistics (off until turned on; every counted feature listed, this
//! week's report and the install ID), and Send feedback.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use gpui::{AnyElement, ClipboardItem, Context, FontWeight, div, prelude::*, rgba};
use katna_core::crash::{self, Report};
use katna_core::usage::{self, Feature, WeeklyReport};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};

use super::MailWindow;
use super::settings::Change;
use crate::format;
use crate::theme::Theme;
use crate::widgets::{outlined_button, tip};

/// The list is read from disk again when it is older than this, so a
/// crash of the daemon shows up while the page is open.
const FRESH: Duration = Duration::from_secs(2);

/// The saved crash reports as last read, newest first.
pub(super) struct SavedReports {
    reports: Vec<Report>,
    /// Names of the ones sent to the crash tracker.
    sent: Vec<String>,
    read: Instant,
}

/// "Katna Mail" for `katna-mail`.
fn app_name(app: &str) -> String {
    match app {
        "katna-mail" => "Katna Mail".to_owned(),
        "katna-daemon" => tr!("feedback-app-daemon"),
        other => other.to_owned(),
    }
}

impl MailWindow {
    /// Notes for usage statistics that `feature` was used this week, while
    /// they are on (`katna_core::usage`).
    pub(super) fn note_usage(&mut self, feature: Feature) {
        if !self.config.feedback.send_usage_statistics {
            return;
        }
        let now = SystemTime::now();
        let week = usage::week(now);
        if self.usage_noted.0 != week {
            self.usage_noted = (week, Default::default());
        }
        if self.usage_noted.1.insert(feature) {
            usage::record(&self.paths, feature, now);
        }
    }

    /// Notes the counted feature a command to the daemon uses, if any.
    pub(super) fn note_command_usage(&mut self, command: &crate::daemon::Command) {
        use crate::daemon::Command;
        let feature = match command {
            Command::Several(commands) => {
                for command in commands {
                    self.note_command_usage(command);
                }
                return;
            }
            Command::Pin(_, true) | Command::PinInChat(..) => Feature::Pins,
            Command::Labels(_, add, _) if !add.is_empty() => Feature::Labels,
            Command::Snooze(..) | Command::SetFollowUp(..) => Feature::Snooze,
            Command::Event(_) | Command::Calendar(_) => Feature::Calendar,
            Command::WriteCards(_) | Command::ContactLabels(_) => Feature::Contacts,
            Command::Task(_) | Command::SaveNote(_) => Feature::TasksNotes,
            _ => return,
        };
        self.note_usage(feature);
    }

    /// The facts only Katna Mail sees, once a week while usage statistics
    /// are on.
    pub(super) fn record_usage_facts(&mut self, scale: f32) {
        let now = SystemTime::now();
        let env = |name: &str| std::env::var(name).unwrap_or_default();
        usage::record_fact(
            &self.paths,
            usage::fact::SCALE,
            usage::scale_bucket(scale),
            now,
        );
        usage::record_fact(
            &self.paths,
            usage::fact::DESKTOP,
            &env("XDG_CURRENT_DESKTOP"),
            now,
        );
        usage::record_fact(
            &self.paths,
            usage::fact::SESSION,
            &env("XDG_SESSION_TYPE"),
            now,
        );
    }

    /// What each frame shows that usage statistics count: the phone-width
    /// layout and Katna's own window frame, and the week's facts.
    pub(super) fn note_usage_each_frame(&mut self, window: &gpui::Window) {
        if !self.config.feedback.send_usage_statistics {
            return;
        }
        let week = usage::week(SystemTime::now());
        if self.usage_noted.0 != week {
            self.usage_noted = (week, Default::default());
            self.record_usage_facts(window.scale_factor());
        }
        if self.layout.shape.is_phone() {
            self.note_usage(Feature::PhoneLayout);
        }
        if self.config.experimental.window_frame == katna_core::config::WindowFrame::Katna {
            self.note_usage(Feature::OwnFrame);
        }
    }

    fn saved_reports(&mut self) -> Option<&SavedReports> {
        let stale = self
            .saved_reports
            .as_ref()
            .is_none_or(|saved| saved.read.elapsed() > FRESH);
        if stale {
            let dir = self.paths.crash_dir();
            self.saved_reports = Some(SavedReports {
                reports: crash::reports(&dir),
                sent: crash::sent(&dir),
                read: Instant::now(),
            });
        }
        self.saved_reports.as_ref()
    }

    pub(super) fn feedback_section(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let save = self.config.feedback.save_crash_reports;
        let send = self.config.feedback.sending();
        let usage_on = self.config.feedback.send_usage_statistics;
        let (reports, sent) = self
            .saved_reports()
            .map(|saved| (saved.reports.clone(), saved.sent.clone()))
            .unwrap_or_default();
        let mut list = div().flex().flex_col();
        if reports.is_empty() {
            list = list.child(
                div()
                    .px(px(8.0))
                    .py(px(8.0))
                    .text_size(px(14.0))
                    .text_color(rgba(th.text_faint))
                    .child(tr!("feedback-none-saved")),
            );
        }
        for (i, report) in reports.iter().enumerate() {
            let sent = sent.contains(&report.name);
            list = list.child(self.report_row(i, report, sent, th, cx));
        }
        if !reports.is_empty() {
            list = list.child(
                div().pt(px(8.0)).pl(px(8.0)).flex().child(
                    outlined_button("feedback-delete-all", tr!("feedback-delete-all"), th)
                        .text_color(rgba(th.error))
                        .on_click(cx.listener(|this, _, _, cx| this.delete_crash_reports(cx))),
                ),
            );
        }
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .pt(px(20.0))
                    .pb(px(4.0))
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child(match (send, usage_on) {
                        (true, true) => tr!("feedback-intro-sending-usage"),
                        (true, false) => tr!("feedback-intro-sending"),
                        (false, true) => tr!("feedback-intro-usage-only"),
                        (false, false) => tr!("feedback-intro-local"),
                    }),
            )
            .child(self.row(
                tr!("feedback-crash-reports"),
                Some(&tr!("feedback-crash-reports-detail")),
                self.switch_row(
                    "page-save-crash-reports",
                    tr!("feedback-save"),
                    tr!("feedback-save-detail"),
                    save,
                    Change::SaveCrashReports(!save),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row(
                tr!("feedback-saved"),
                Some(&tr!("feedback-saved-detail", count = 20)),
                list,
                th,
            ))
            .child(
                self.row(
                    tr!("feedback-help-improve"),
                    Some(&tr!("feedback-help-improve-detail")),
                    div()
                        .flex()
                        .flex_col()
                        .child(self.switch_row(
                            "page-send-crash-reports",
                            tr!("feedback-send"),
                            tr!("feedback-send-detail"),
                            send,
                            Change::SendCrashReports(!send),
                            th,
                            cx,
                        ))
                        .child(self.switch_row(
                            "page-send-usage",
                            tr!("feedback-usage"),
                            tr!("feedback-usage-detail"),
                            usage_on,
                            Change::SendUsageStatistics(!usage_on),
                            th,
                            cx,
                        )),
                    th,
                ),
            )
            .child({
                let counted = self.counted(usage_on, th, cx);
                self.row(
                    tr!("feedback-counted"),
                    Some(&tr!("feedback-counted-detail")),
                    counted,
                    th,
                )
            })
            .child(
                self.row(
                    tr!("feedback-send-feedback"),
                    Some(&tr!("feedback-send-feedback-detail")),
                    div().pl(px(space::S3)).flex().child(
                        outlined_button(
                            "page-send-feedback",
                            tr!("feedback-send-feedback-button"),
                            th,
                        )
                        .on_click(
                            cx.listener(|this, _, window, cx| this.open_feedback_form(window, cx)),
                        ),
                    ),
                    th,
                ),
            )
            .into_any_element()
    }

    /// Every feature usage statistics count, and while they are on, this
    /// week's report and the install ID.
    fn counted(&mut self, on: bool, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let names = Feature::ALL.map(|f| tr!(f.label_id()));
        let column = |names: &[String]| {
            div()
                .flex_1()
                .min_w(px(160.0))
                .flex()
                .flex_col()
                .children(names.iter().map(|name| {
                    div()
                        .text_size(px(text::SMALL))
                        .text_color(rgba(th.text_dim))
                        .child(format!("• {name}"))
                }))
        };
        let half = names.len().div_ceil(2);
        let mut out = div()
            .px(px(space::S3))
            .py(px(space::S2))
            .flex()
            .flex_col()
            .gap(px(space::S3))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap_x(px(space::S5))
                    .child(column(&names[..half]))
                    .child(column(&names[half..])),
            )
            .child(
                div()
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(th.text_faint))
                    .child(tr!("feedback-counted-also")),
            );
        if !on {
            return out.into_any_element();
        }
        let now = SystemTime::now();
        let (id, _) = usage::install_id(&self.paths, now);
        let short = format!("{}…{}", &id[..4], &id[id.len() - 4..]);
        let link = |id: &'static str, words: String| {
            div()
                .id(id)
                .flex_none()
                .px(px(space::S3))
                .py(px(space::S2))
                .rounded(px(radius::FULL))
                .text_size(px(text::SMALL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.accent))
                .cursor_pointer()
                .relative()
                .child(crate::widgets::hover_fade(
                    "hover-glow",
                    Some(radius::LG),
                    th,
                ))
                .child(words)
        };
        let open = self.usage_report_open;
        out = out.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(space::S2))
                .child(
                    link(
                        "feedback-see-report",
                        if open {
                            tr!("feedback-hide-report")
                        } else {
                            tr!("feedback-see-report")
                        },
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.usage_report_open = !this.usage_report_open;
                        cx.notify();
                    })),
                )
                .child(
                    div()
                        .id("feedback-install-id")
                        .pl(px(space::S3))
                        .text_size(px(text::SMALL))
                        .text_color(rgba(th.text_dim))
                        .tooltip(tip(tr!("feedback-install-id-tooltip"), th))
                        .child(tr!("feedback-install-id", id = short)),
                )
                .child(
                    link(
                        "feedback-install-id-reset",
                        tr!("feedback-install-id-reset"),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        usage::new_install_id(&this.paths, SystemTime::now());
                        this.show_snackbar(tr!("feedback-install-id-new"), None, cx);
                        cx.notify();
                    })),
                ),
        );
        if open {
            let week = usage::week(now);
            let accounts = self.accounts.len();
            let report = WeeklyReport::build(&self.paths, week, accounts, id).text();
            let goes = usage::week_end(week)
                .duration_since(UNIX_EPOCH)
                .ok()
                .and_then(|d| format::local(d.as_secs() as i64, &self.tz))
                .map(katna_i18n::format::date)
                .unwrap_or_default();
            let copy = report.clone();
            out = out.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(space::S2))
                    .child(
                        div()
                            .px(px(space::S4))
                            .py(px(space::S3))
                            .rounded(px(radius::SM))
                            .border_1()
                            .border_color(rgba(th.outline))
                            .font_family("monospace")
                            .text_size(px(text::CAPTION))
                            .text_color(rgba(th.text_dim))
                            .children(
                                report
                                    .lines()
                                    .map(|line| self.copyable(line.to_owned(), th)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap(px(space::S3))
                            .child(
                                div()
                                    .flex_1()
                                    .min_w(px(200.0))
                                    .text_size(px(text::CAPTION))
                                    .text_color(rgba(th.text_faint))
                                    .child(tr!("feedback-report-goes", date = goes)),
                            )
                            .child(link("feedback-report-copy", tr!("text-copy")).on_click(
                                cx.listener(move |this, _, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(copy.clone()));
                                    this.show_snackbar(tr!("feedback-report-copied"), None, cx);
                                }),
                            )),
                    ),
            );
        }
        out.into_any_element()
    }

    fn report_row(
        &self,
        i: usize,
        report: &Report,
        sent: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let mut when = report
            .unix_time()
            .and_then(|t| format::local(t, &self.tz))
            .map(format::long_date)
            .unwrap_or_default();
        if sent {
            when = tr!("feedback-report-sent", date = when);
        }
        let link = |id: &'static str| {
            div()
                .id((id, i))
                .flex_none()
                .px(px(10.0))
                .py(px(6.0))
                .rounded(px(16.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.accent))
                .cursor_pointer()
                .relative()
                .child(crate::widgets::hover_fade("hover-glow", Some(16.0), th))
        };
        let (view, copy, delete) = (report.clone(), report.clone(), report.clone());
        div()
            .px(px(8.0))
            .py(px(4.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(4.0))
            .rounded(px(8.0))
            .hover(|s| s.bg(rgba(th.hover)))
            .child(
                div()
                    .flex_1()
                    .min_w(px(200.0))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .child(app_name(&report.app).to_string()),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(when),
                    ),
            )
            .child(
                link("feedback-view")
                    .tooltip(tip(tr!("feedback-view-tooltip"), th))
                    .on_click(cx.listener(move |_, _, _, cx| cx.open_with_system(&view.path)))
                    .child(tr!("feedback-view")),
            )
            .child(
                link("feedback-copy")
                    .tooltip(tip(tr!("feedback-copy-tooltip"), th))
                    .on_click(cx.listener(move |this, _, _, cx| match copy.read() {
                        Ok(text) => {
                            cx.write_to_clipboard(ClipboardItem::new_string(text));
                            this.show_snackbar(tr!("feedback-copied"), None, cx);
                        }
                        Err(err) => this.show_snackbar(
                            tr!("feedback-read-failed", error = err.to_string()),
                            None,
                            cx,
                        ),
                    }))
                    .child(tr!("text-copy")),
            )
            .child(
                link("feedback-delete")
                    .text_color(rgba(th.error))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Err(err) = std::fs::remove_file(&delete.path) {
                            this.show_snackbar(
                                tr!("feedback-delete-failed", error = err.to_string()),
                                None,
                                cx,
                            );
                        }
                        this.saved_reports = None;
                        cx.notify();
                    }))
                    .child(tr!("list-delete")),
            )
            .into_any_element()
    }

    fn delete_crash_reports(&mut self, cx: &mut Context<Self>) {
        match crash::delete_all(&self.paths.crash_dir()) {
            Ok(()) => self.show_snackbar(tr!("feedback-deleted-all"), None, cx),
            Err(err) => self.show_snackbar(
                tr!("feedback-delete-all-failed", error = err.to_string()),
                None,
                cx,
            ),
        }
        self.saved_reports = None;
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_programs() {
        assert_eq!(app_name("katna-mail"), "Katna Mail");
        assert_eq!(app_name("katna-daemon"), "Background service");
        assert_eq!(app_name("something"), "something");
    }
}
