// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > User feedback (`docs/ARCHITECTURE.md` §19.2): whether crash
//! reports are saved on this computer, and the saved ones to view, copy
//! or delete. Sending reports and usage statistics, only if the user
//! agrees, comes later and will live here too.

use std::time::{Duration, Instant};

use gpui::{AnyElement, ClipboardItem, Context, FontWeight, div, prelude::*, px, rgba};
use katna_core::crash::{self, Report};

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
    read: Instant,
}

/// "Katna Mail" for `katna-mail`.
fn app_name(app: &str) -> &str {
    match app {
        "katna-mail" => "Katna Mail",
        "katna-daemon" => "Background service",
        "katnactl" => "katnactl",
        other => other,
    }
}

impl MailWindow {
    fn saved_reports(&mut self) -> &[Report] {
        let stale = self
            .saved_reports
            .as_ref()
            .is_none_or(|saved| saved.read.elapsed() > FRESH);
        if stale {
            self.saved_reports = Some(SavedReports {
                reports: crash::reports(&self.paths.crash_dir()),
                read: Instant::now(),
            });
        }
        self.saved_reports
            .as_ref()
            .map_or(&[], |saved| saved.reports.as_slice())
    }

    pub(super) fn feedback_section(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let save = self.config.feedback.save_crash_reports;
        let reports = self.saved_reports().to_vec();
        let mut list = div().flex().flex_col();
        if reports.is_empty() {
            list = list.child(
                div()
                    .px(px(8.0))
                    .py(px(8.0))
                    .text_size(px(14.0))
                    .text_color(rgba(th.text_faint))
                    .child("No crash reports are saved."),
            );
        }
        for (i, report) in reports.iter().enumerate() {
            list = list.child(self.report_row(i, report, th, cx));
        }
        if !reports.is_empty() {
            list = list.child(
                div().pt(px(8.0)).pl(px(8.0)).flex().child(
                    outlined_button("feedback-delete-all", "Delete all", th)
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
                    .child(
                        "Katna sends nothing anywhere. Crash reports stay on this computer, \
                         for you to look at or attach to a bug report.",
                    ),
            )
            .child(self.row(
                "Crash reports",
                Some("Written when Katna Mail or its background service crashes."),
                self.switch_row(
                    "page-save-crash-reports",
                    "Save crash reports on this computer",
                    "Your home folder, user and computer names and email addresses are left out",
                    save,
                    Change::SaveCrashReports(!save),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row(
                "Saved crash reports",
                Some("The newest 20 are kept."),
                list,
                th,
            ))
            .child(
                self.row(
                    "Sharing",
                    Some(
                        "A later version will ask whether to send crash reports and anonymous \
                     usage statistics to help improve Katna. Both stay off unless you \
                     turn them on, and you can turn them off here at any time.",
                    ),
                    div()
                        .px(px(8.0))
                        .py(px(8.0))
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_faint))
                        .child("Nothing is shared."),
                    th,
                ),
            )
            .into_any_element()
    }

    fn report_row(
        &self,
        i: usize,
        report: &Report,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let when = report
            .unix_time()
            .and_then(|t| format::local(t, &self.tz))
            .map(format::long_date)
            .unwrap_or_default();
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
                .hover(|s| s.bg(rgba(th.hover)))
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
                    .tooltip(tip("Open the report", th))
                    .on_click(cx.listener(move |_, _, _, cx| cx.open_with_system(&view.path)))
                    .child("View"),
            )
            .child(
                link("feedback-copy")
                    .tooltip(tip("Copy it to paste into a bug report", th))
                    .on_click(cx.listener(move |this, _, _, cx| match copy.read() {
                        Ok(text) => {
                            cx.write_to_clipboard(ClipboardItem::new_string(text));
                            this.show_snackbar("Crash report copied.", None, cx);
                        }
                        Err(err) => this.show_snackbar(
                            format!("Could not read the crash report: {err}"),
                            None,
                            cx,
                        ),
                    }))
                    .child("Copy"),
            )
            .child(
                link("feedback-delete")
                    .text_color(rgba(th.error))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Err(err) = std::fs::remove_file(&delete.path) {
                            this.show_snackbar(
                                format!("Could not delete the crash report: {err}"),
                                None,
                                cx,
                            );
                        }
                        this.saved_reports = None;
                        cx.notify();
                    }))
                    .child("Delete"),
            )
            .into_any_element()
    }

    fn delete_crash_reports(&mut self, cx: &mut Context<Self>) {
        match crash::delete_all(&self.paths.crash_dir()) {
            Ok(()) => self.show_snackbar("Crash reports deleted.", None, cx),
            Err(err) => self.show_snackbar(
                format!("Could not delete the crash reports: {err}"),
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
