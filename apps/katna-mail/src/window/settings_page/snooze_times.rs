// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Inbox > Snooze times: what Later today, Tomorrow, This
//! weekend and Next week mean, and one time of the user's own ("monday
//! 10:00"), as in Spark (`docs/ARCHITECTURE.md` §10.1).

use gpui::{AnyElement, Context, Div, Entity, Subscription, div, prelude::*, rgba};
use jiff::civil::Time;
use jiff::{Timestamp, Zoned};
use katna_core::config::{SnoozeDay, SnoozeTimes};
use katna_i18n::{format, tr};
use katna_ui::tokens::{space, text};
use katna_ui::{InputEvent, TextInput, px};

use super::{MailWindow, SAVE_DELAY, chip};
use crate::theme::Theme;
use crate::widgets::line_field;
use crate::window::compose::schedule;

/// The fields of the snooze times.
pub(super) struct SnoozeFields {
    later: Entity<TextInput>,
    morning: Entity<TextInput>,
    own: Entity<TextInput>,
    _subscriptions: [Subscription; 3],
}

/// Minutes after midnight as a time.
fn time_of(minutes: u32) -> Time {
    let minutes = minutes.min(24 * 60 - 1);
    Time::new((minutes / 60) as i8, (minutes % 60) as i8, 0, 0).unwrap_or(Time::midnight())
}

impl SnoozeFields {
    pub(super) fn new(
        times: &SnoozeTimes,
        accent: gpui::Hsla,
        cx: &mut Context<MailWindow>,
    ) -> Self {
        let clock = |minutes: u32, cx: &mut Context<MailWindow>| {
            let text = schedule::clock(time_of(minutes));
            cx.new(|cx| {
                let mut input = TextInput::new(text.clone(), cx);
                input.set_text(text, cx);
                input.set_accent(accent);
                input.set_stepper(Some(schedule::time_stepper()));
                input
            })
        };
        let later = clock(times.later_today, cx);
        let morning = clock(times.morning, cx);
        let own_text = times.own.clone();
        let own = cx.new(|cx| {
            let mut input = TextInput::new(tr!("settings-snooze-own-placeholder"), cx);
            input.set_text(own_text, cx);
            input.set_accent(accent);
            input
        });
        let time_changed = |field: fn(&mut SnoozeTimes) -> &mut u32| {
            move |this: &mut MailWindow,
                  input: Entity<TextInput>,
                  event: &InputEvent,
                  cx: &mut Context<MailWindow>| {
                if *event == InputEvent::Changed
                    && let Some(time) = schedule::parse_time(input.read(cx).text())
                {
                    let minutes = (time.hour() as u32) * 60 + time.minute() as u32;
                    this.set_snooze_times(|times| *field(times) = minutes, cx);
                }
            }
        };
        let subscriptions = [
            cx.subscribe(&later, time_changed(|t| &mut t.later_today)),
            cx.subscribe(&morning, time_changed(|t| &mut t.morning)),
            cx.subscribe(&own, |this, input, event: &InputEvent, cx| {
                if *event == InputEvent::Changed {
                    let text = input.read(cx).text().trim().to_owned();
                    this.set_snooze_times(|times| times.own = text, cx);
                }
            }),
        ];
        Self {
            later,
            morning,
            own,
            _subscriptions: subscriptions,
        }
    }
}

impl MailWindow {
    /// Changes the snooze times and saves them once typing pauses.
    fn set_snooze_times(&mut self, change: impl FnOnce(&mut SnoozeTimes), cx: &mut Context<Self>) {
        let mut times = self.config.mail.snooze.clone();
        change(&mut times);
        if times == self.config.mail.snooze {
            return;
        }
        self.config.mail.snooze = times;
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SAVE_DELAY).await;
            this.update(cx, |this, _| this.save_config()).ok();
        });
        if let Some(page) = &mut self.settings_page {
            page.save = Some(task);
        } else {
            task.detach();
        }
        cx.notify();
    }

    /// The snooze times, one line each.
    pub(super) fn snooze_times_rows(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(page) = &self.settings_page else {
            return div().into_any_element();
        };
        let fields = &page.snooze;
        let times = &self.config.mail.snooze;
        let line = |label: String, detail: Option<String>, control: AnyElement| -> Div {
            div()
                .min_h(px(48.0))
                .py(px(space::S2))
                .px(px(space::S3))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(space::S4))
                .child(
                    div()
                        .flex_1()
                        .min_w(px(160.0))
                        .flex()
                        .flex_col()
                        .child(div().text_size(px(text::BODY)).child(label))
                        .children(detail.map(|d| {
                            div()
                                .text_size(px(text::CAPTION))
                                .text_color(rgba(th.text_faint))
                                .child(d)
                        })),
                )
                .child(control)
        };
        let time_field = |id: &'static str, input: &Entity<TextInput>, cx: &mut Context<Self>| {
            div()
                .w(px(130.0))
                .child(line_field(id, input, th, cx))
                .into_any_element()
        };
        let days = |id: &'static str,
                    choices: [SnoozeDay; 3],
                    now: SnoozeDay,
                    set: fn(&mut SnoozeTimes, SnoozeDay),
                    cx: &mut Context<Self>| {
            div()
                .flex()
                .flex_row()
                .gap(px(space::S2))
                .children(choices.into_iter().enumerate().map(|(ix, day)| {
                    let date = jiff::civil::date(2024, 1, 1)
                        .checked_add(
                            jiff::Span::new()
                                .days(i64::from(day.weekday().to_monday_zero_offset())),
                        )
                        .unwrap_or(jiff::civil::date(2024, 1, 1));
                    let name = format::weekday_long(date.to_datetime(Time::midnight()));
                    self.page_control(chip((id, ix), name, day == now, th), th, cx)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.set_snooze_times(|times| set(times, day), cx);
                        }))
                }))
                .into_any_element()
        };
        // The user's own time: where it would come back now.
        let now = Timestamp::now().to_zoned(self.tz.clone());
        let own = times.own.trim();
        let own_next = super::super::snooze::own_time(own, &now, time_of(times.morning));
        let own_note = match (&own_next, own.is_empty()) {
            (_, true) => None,
            (Some(next), false) => Some((
                tr!("settings-snooze-own-next", date = next_date(&next.at)),
                th.text_faint,
            )),
            (None, false) => Some((tr!("snooze-type-hint-unclear"), th.warning)),
        };
        let own_control = div()
            .w(px(280.0))
            .max_w_full()
            .flex_none()
            .flex()
            .flex_col()
            .gap(px(space::S2))
            .child(line_field("page-snooze-own", &fields.own, th, cx))
            .children(own_note.map(|(note, color)| {
                div()
                    .px(px(space::S2))
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(color))
                    .child(note)
            }))
            .into_any_element();
        div()
            .flex()
            .flex_col()
            .child(line(
                tr!("snooze-later-today"),
                None,
                time_field("page-snooze-later", &fields.later, cx),
            ))
            .child(line(
                tr!("settings-snooze-morning"),
                Some(tr!("settings-snooze-morning-detail")),
                time_field("page-snooze-morning", &fields.morning, cx),
            ))
            .child(line(
                tr!("snooze-this-weekend"),
                None,
                days(
                    "page-snooze-weekend",
                    SnoozeDay::WEEKEND,
                    times.weekend,
                    |t, d| t.weekend = d,
                    cx,
                ),
            ))
            .child(line(
                tr!("snooze-next-week"),
                None,
                days(
                    "page-snooze-week",
                    SnoozeDay::WEEK,
                    times.next_week,
                    |t, d| t.next_week = d,
                    cx,
                ),
            ))
            .child(line(
                tr!("settings-snooze-own"),
                Some(tr!("settings-snooze-own-detail")),
                own_control,
            ))
            .into_any_element()
    }
}

/// "Mon, Oct 12, 2026, 10:00 AM".
fn next_date(at: &Zoned) -> String {
    format::long(at.datetime())
}
