// SPDX-License-Identifier: GPL-3.0-or-later

//! Activity: how mail sent with open and click tracking did, listed under
//! "Activity" in the navigation once there is some (`docs/ARCHITECTURE.md`
//! §16.1). Open and click rates over all of it, then each message by how
//! many of its recipients opened it, so the subject lines that get read
//! stand out.

use gpui::{AnyElement, Context, FontWeight, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_store::MessageActivity;
use katna_ui::{px, unpx};

use super::MailWindow;
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, icon, icon_button, tip};

/// The navigation row of Activity.
pub(super) const NAV_KEY: &str = "katna:activity";

/// At most this many tracked messages are read.
const LIMIT: u32 = 500;

/// Opens and clicks over many messages: of every recipient of every
/// message, how many opened it and followed a link.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Rates {
    pub messages: usize,
    pub recipients: usize,
    pub opened: usize,
    pub clicked: usize,
}

impl Rates {
    pub fn of(list: &[MessageActivity]) -> Self {
        list.iter().fold(Self::default(), |sum, m| Self {
            messages: sum.messages + 1,
            recipients: sum.recipients + m.recipients.len(),
            opened: sum.opened + m.opened(),
            clicked: sum.clicked + m.clicked(),
        })
    }
}

/// `part` of `whole` in whole percent.
fn percent(part: usize, whole: usize) -> u64 {
    if whole == 0 {
        0
    } else {
        ((part as f64 / whole as f64) * 100.0).round() as u64
    }
}

/// Messages by how many of their recipients opened them, then clicked,
/// newest first among equals.
pub(super) fn by_open_rate(mut list: Vec<MessageActivity>) -> Vec<MessageActivity> {
    let rate = |m: &MessageActivity| {
        (
            percent(m.opened(), m.recipients.len()),
            percent(m.clicked(), m.recipients.len()),
        )
    };
    // Stable: the store lists newest first.
    list.sort_by_key(|m| std::cmp::Reverse(rate(m)));
    list
}

impl MailWindow {
    pub(super) fn open_activity(&mut self, cx: &mut Context<Self>) {
        self.activity = Some(self.read_activity());
        cx.notify();
    }

    /// Reads the activity again while it is open.
    pub(super) fn refresh_activity(&mut self) {
        if self.activity.is_some() {
            self.activity = Some(self.read_activity());
        }
    }

    fn read_activity(&self) -> Vec<MessageActivity> {
        self.mail
            .as_ref()
            .map(|mail| by_open_rate(mail.tracked(LIMIT)))
            .unwrap_or_default()
    }

    /// Whether there is tracked mail, for the navigation row.
    pub(super) fn has_activity(&self) -> bool {
        self.mail.as_ref().is_ok_and(|mail| mail.has_tracking())
    }

    /// The Activity dialog, when opened from the navigation.
    pub(super) fn render_activity(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let list = self.activity.as_ref()?;
        let viewport = window.viewport_size();
        let height = (unpx(viewport.height) - 160.0).clamp(240.0, 640.0);
        let rates = Rates::of(list);
        let stat = |value: String, label: String| {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(div().text_size(px(28.0)).line_height(px(36.0)).child(value))
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
        };
        let rate = |part: usize, whole: usize| {
            tr!(
                "activity-percent",
                percent = format::thousands(percent(part, whole))
            )
        };
        let summary = div()
            .flex_none()
            .px(px(24.0))
            .py(px(16.0))
            .flex()
            .flex_row()
            .gap(px(16.0))
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(stat(
                format::thousands(rates.messages as u64),
                tr!("activity-messages"),
            ))
            .child(stat(
                rate(rates.opened, rates.recipients),
                tr!("activity-open-rate"),
            ))
            .child(stat(
                rate(rates.clicked, rates.recipients),
                tr!("activity-click-rate"),
            ));
        let bar = |part: usize, whole: usize, color: u32| {
            let share = percent(part, whole) as f32 / 100.0;
            div()
                .h(px(6.0))
                .w_full()
                .rounded(px(3.0))
                .bg(rgba(fade(color, 0.16)))
                .child(
                    div()
                        .h_full()
                        .w(gpui::relative(share))
                        .rounded(px(3.0))
                        .bg(rgba(color)),
                )
        };
        let now = jiff::Timestamp::now().as_second();
        let rows = list.iter().enumerate().map(|(ix, message)| {
            let whole = message.recipients.len();
            let subject = if message.subject.trim().is_empty() {
                tr!("activity-no-subject")
            } else {
                message.subject.clone()
            };
            let when = message
                .sent_at
                .and_then(|at| format::local(at, &self.tz))
                .zip(format::local(now, &self.tz))
                .map(|(at, now)| format::list_date(at, now))
                .unwrap_or_default();
            div()
                .id(("activity", ix))
                .flex_none()
                .px(px(24.0))
                .py(px(12.0))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .truncate()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(subject),
                        )
                        .child(
                            div()
                                .flex_none()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(when),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(16.0))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(tr!(
                                    "activity-opened",
                                    opened = message.opened(),
                                    recipients = whole
                                ))
                                .child(bar(message.opened(), whole, th.accent)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(tr!(
                                    "activity-clicked",
                                    clicked = message.clicked(),
                                    recipients = whole
                                ))
                                .child(bar(message.clicked(), whole, th.accent)),
                        ),
                )
        });
        let empty = list.is_empty().then(|| {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(12.0))
                .text_color(rgba(th.text_dim))
                .text_size(px(14.0))
                .child(icon("activity", th.text_faint, 48.0))
                .child(tr!("activity-nothing"))
        });
        let close = cx.listener(|this, _, _, cx| {
            this.activity = None;
            cx.notify();
        });
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, 1.0)))
                .child(
                    div()
                        .id("activity-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(close),
                )
                .child(
                    div()
                        .id("activity-dialog")
                        .occlude()
                        .w(px(600.0_f32.min(unpx(viewport.width) - 32.0)))
                        .h(px(height))
                        .flex()
                        .flex_col()
                        .rounded(px(16.0))
                        .overflow_hidden()
                        .bg(rgba(th.surface))
                        .shadow(elevation(th, 3.0))
                        .child(
                            div()
                                .flex_none()
                                .h(px(64.0))
                                .pl(px(24.0))
                                .pr(px(12.0))
                                .flex()
                                .flex_row()
                                .items_center()
                                .border_b_1()
                                .border_color(rgba(th.divider))
                                .child(
                                    div()
                                        .flex_1()
                                        .text_size(px(20.0))
                                        .child(tr!("folder-activity")),
                                )
                                .child(
                                    icon_button("activity-close", "close", 20.0, th)
                                        .tooltip(tip(tr!("activity-close"), th))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.activity = None;
                                            cx.notify();
                                        })),
                                ),
                        )
                        .when(!list.is_empty(), |d| {
                            d.child(summary).child(
                                div()
                                    .flex_none()
                                    .px(px(24.0))
                                    .pt(px(12.0))
                                    .pb(px(4.0))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.text_dim))
                                    .child(tr!("activity-by-open-rate")),
                            )
                        })
                        .child(
                            div()
                                .id("activity-list")
                                .flex_1()
                                .min_h_0()
                                .flex()
                                .flex_col()
                                .overflow_y_scroll()
                                .children(rows)
                                .children(empty),
                        ),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use katna_store::RecipientActivity;

    use super::*;

    fn message(subject: &str, opened: usize, clicked: usize, of: usize) -> MessageActivity {
        MessageActivity {
            subject: subject.into(),
            sent_at: Some(0),
            links: 1,
            recipients: (0..of)
                .map(|i| RecipientActivity {
                    email: format!("r{i}@example.org"),
                    name: None,
                    opens: u32::from(i < opened),
                    maybe_opens: 0,
                    clicks: u32::from(i < clicked),
                    last: None,
                })
                .collect(),
        }
    }

    #[test]
    fn rates_count_recipients() {
        let list = [message("a", 1, 0, 2), message("b", 2, 1, 2)];
        assert_eq!(
            Rates::of(&list),
            Rates {
                messages: 2,
                recipients: 4,
                opened: 3,
                clicked: 1
            }
        );
        assert_eq!(percent(3, 4), 75);
        assert_eq!(percent(0, 0), 0);
    }

    #[test]
    fn best_subject_lines_first() {
        let list = vec![
            message("newest, unread", 0, 0, 3),
            message("half", 1, 0, 2),
            message("all, clicked", 2, 1, 2),
            message("all", 3, 0, 3),
        ];
        let subjects: Vec<String> = by_open_rate(list).into_iter().map(|m| m.subject).collect();
        assert_eq!(subjects, ["all, clicked", "all", "half", "newest, unread"]);
    }
}
