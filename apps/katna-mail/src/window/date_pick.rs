// SPDX-License-Identifier: GPL-3.0-or-later

//! Pick date & time, the one picker of Snooze, Remind me, Schedule send
//! and Follow up: it slides into the menu it was opened from, where it was
//! clicked, never a dialog in the middle of the window. A typed moment
//! ("tue 3pm"), the month, the day and the time; its back arrow (or Esc)
//! slides back to the menu's times. The menu that opens it keeps it and
//! says what its buttons do ([`PickHooks`]).

use crate::widgets::Tip as _;
use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, Hsla,
    Subscription, Window, div, ease_out_quint, prelude::*, rgba,
};
use jiff::Timestamp;
use jiff::civil::{Date, Time};
use katna_i18n::{format, tr};
use katna_ui::tokens::{space, text};
use katna_ui::{InputEvent, TextInput, px};

use super::MailWindow;
use super::compose::schedule;
use crate::theme::Theme;
use crate::widgets::{filled_button, icon_button};

/// How long the menu's times and the picker take to slide past each
/// other.
pub(super) const SLIDE: std::time::Duration = katna_ui::tokens::duration::BASE;
/// How far they slide.
pub(super) const SLIDE_BY: f32 = space::S8;
/// The picker's width.
const WIDTH: f32 = 330.0;

/// What a picker does, for the menu that holds it.
#[derive(Clone, Copy)]
pub(super) struct PickHooks {
    /// The picker, where its menu keeps it.
    pub find: fn(&mut MailWindow) -> Option<&mut DatePick>,
    /// Save, Done or Schedule send.
    pub done: fn(&mut MailWindow, &mut Window, &mut Context<MailWindow>),
    /// The back arrow and Esc: back to the menu's times.
    pub back: fn(&mut MailWindow, &mut Window, &mut Context<MailWindow>),
    /// Cancel: the menu closes.
    pub cancel: fn(&mut MailWindow, &mut Window, &mut Context<MailWindow>),
}

/// An open date and time picker.
pub(super) struct DatePick {
    /// The month the calendar shows.
    pub month: Date,
    pub day: Date,
    pub time: Entity<TextInput>,
    /// "tue 3pm", "in 2 hours": fills the day and the time.
    pub typed: Entity<TextInput>,
    /// Whether what is typed was not understood.
    pub unclear: bool,
    hooks: PickHooks,
    _events: [Subscription; 2],
}

impl DatePick {
    /// A picker on `day` at `time`, the typed field focused.
    pub fn new(
        day: Date,
        time: Time,
        accent: Hsla,
        hooks: PickHooks,
        window: &mut Window,
        cx: &mut Context<MailWindow>,
    ) -> Self {
        let clock = schedule::clock(time);
        let time = cx.new(|cx| {
            let mut input = TextInput::new(clock.clone(), cx);
            input.set_accent(accent);
            input.set_text(clock, cx);
            input.select_all_text(cx);
            input.set_stepper(Some(schedule::time_stepper()));
            input
        });
        let typed = cx.new(|cx| {
            let mut input = TextInput::new(tr!("snooze-type-placeholder"), cx);
            input.set_accent(accent);
            input
        });
        let on = move |typed: bool| {
            move |this: &mut MailWindow,
                  _: &Entity<TextInput>,
                  event: &InputEvent,
                  window: &mut Window,
                  cx: &mut Context<MailWindow>| match event {
                InputEvent::Submit => (hooks.done)(this, window, cx),
                InputEvent::Cancel => (hooks.back)(this, window, cx),
                InputEvent::Changed if typed => this.pick_typed(hooks, cx),
                InputEvent::Changed => cx.notify(),
            }
        };
        let events = [
            cx.subscribe_in(&time, window, on(false)),
            cx.subscribe_in(&typed, window, on(true)),
        ];
        window.focus(&typed.focus_handle(cx), cx);
        DatePick {
            month: day,
            day,
            time,
            typed,
            unclear: false,
            hooks,
            _events: events,
        }
    }

    /// The picked moment, or what to tell the user is wrong with it.
    pub fn moment(&self, tz: &jiff::tz::TimeZone, cx: &gpui::App) -> Result<Timestamp, String> {
        if self.unclear {
            let text = self.typed.read(cx).text().to_owned();
            return Err(tr!("snooze-type-unclear", text = text));
        }
        let text = self.time.read(cx).text().to_owned();
        let Some(time) = schedule::parse_time(&text) else {
            let example = schedule::clock(Time::constant(8, 0, 0, 0));
            return Err(tr!("schedule-not-a-time", text = text, example = example));
        };
        schedule::moment(self.day, time, tz).ok_or_else(|| tr!("schedule-no-such-time"))
    }
}

impl MailWindow {
    /// Reads the typed moment into the picker's day and time.
    fn pick_typed(&mut self, hooks: PickHooks, cx: &mut Context<Self>) {
        let now = Timestamp::now().to_zoned(self.tz.clone());
        let Some(pick) = (hooks.find)(self) else {
            return;
        };
        let text = pick.typed.read(cx).text().to_owned();
        let language = katna_i18n::current().language.tag.clone();
        let words = katna_core::quick_add::Words::for_language(&language);
        let at =
            katna_core::quick_add::moment(&text, now.datetime(), Time::constant(8, 0, 0, 0), words);
        pick.unclear = at.is_none() && !text.trim().is_empty();
        if let Some(at) = at {
            pick.day = at.date();
            pick.month = at.date();
            pick.time.update(cx, |input, cx| {
                input.set_text(schedule::clock(at.time()), cx);
            });
        }
        cx.notify();
    }

    /// The picker, titled `title`, its main button `done`; it slides in
    /// from the right over where the menu's times were.
    pub(super) fn render_date_pick(
        &self,
        pick: &DatePick,
        title: String,
        done: String,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let hooks = pick.hooks;
        let today = Timestamp::now().to_zoned(self.tz.clone()).date();
        let (month, day) = (pick.month, pick.day);
        let days = schedule::month_grid(month, format::first_weekday())
            .into_iter()
            .enumerate()
            .map(|(ix, date)| {
                let past = date < today;
                let selected = date == day;
                let other = date.month() != month.month();
                div()
                    .id(("pick-day", ix))
                    .size(px(36.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .text_size(px(13.0))
                    .when(other, |d| d.text_color(rgba(th.text_faint)))
                    .when(past, |d| d.opacity(0.38))
                    .when(date == today && !selected, |d| {
                        d.border_1().border_color(rgba(th.accent))
                    })
                    .when(selected, |d| {
                        d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                    })
                    .when(!past, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(p) = (hooks.find)(this) {
                                    p.day = date;
                                    p.month = date;
                                }
                                cx.notify();
                            }))
                    })
                    .child(format::number(date.day() as u64))
            })
            .collect::<Vec<_>>();
        let step = move |months: i32| {
            move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                if let Some(p) = (hooks.find)(this)
                    && let Ok(m) = p
                        .month
                        .first_of_month()
                        .checked_add(jiff::Span::new().months(months))
                {
                    p.month = m;
                }
                cx.notify();
            }
        };
        let weekdays = format::weekdays_short().into_iter().map(|(_, d)| {
            div()
                .size(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .child(d)
        });
        let field = || {
            div()
                .h(px(40.0))
                .px(px(12.0))
                .flex()
                .items_center()
                .rounded(px(6.0))
                .border_1()
                .text_size(px(14.0))
        };
        div()
            .w(px(WIDTH))
            .overflow_hidden()
            .map(|d| crate::widgets::dialog(d, th, th.menu))
            .text_color(rgba(th.text))
            .child(
                div()
                    .p(px(24.0))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .mb(px(16.0))
                            .ml(px(-space::S3))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(space::S2))
                            .text_size(px(20.0))
                            .child(
                                icon_button("pick-back", "back", 20.0, th)
                                    .flex_none()
                                    .size(px(32.0))
                                    .tip(tr!("snooze-back"), th)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        (hooks.back)(this, window, cx)
                                    })),
                            )
                            .child(div().min_w_0().child(title)),
                    )
                    .child(
                        field()
                            .border_color(rgba(if pick.unclear { th.warning } else { th.accent }))
                            .child(pick.typed.clone()),
                    )
                    .child(
                        div()
                            .mt(px(space::S2))
                            .mb(px(space::S3))
                            .text_size(px(text::CAPTION))
                            .text_color(rgba(if pick.unclear {
                                th.warning
                            } else {
                                th.text_dim
                            }))
                            .child(if pick.unclear {
                                tr!("snooze-type-hint-unclear")
                            } else {
                                tr!("snooze-type-hint")
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(format::month_year(month)),
                            )
                            .child(
                                icon_button("pick-prev", "chevron-left", 20.0, th)
                                    .size(px(32.0))
                                    .on_click(cx.listener(step(-1))),
                            )
                            .child(
                                icon_button("pick-next", "chevron-right", 20.0, th)
                                    .size(px(32.0))
                                    .on_click(cx.listener(step(1))),
                            ),
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .w(px(7.0 * 40.0))
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap_x(px(4.0))
                            .children(weekdays)
                            .children(days),
                    )
                    .child(
                        div()
                            .mt(px(12.0))
                            .flex()
                            .flex_row()
                            .gap(px(12.0))
                            .child(
                                field().flex_1().border_color(rgba(th.divider)).child(
                                    format::day_month_year(day.to_datetime(Time::midnight())),
                                ),
                            )
                            .child(
                                field()
                                    .w(px(110.0))
                                    .border_color(rgba(th.outline))
                                    .child(pick.time.clone()),
                            ),
                    )
                    .child(
                        div()
                            .mt(px(20.0))
                            .flex()
                            .flex_row()
                            .justify_end()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("pick-cancel")
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
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        (hooks.cancel)(this, window, cx)
                                    }))
                                    .child(tr!("snooze-cancel")),
                            )
                            .child(filled_button("pick-done", done, th).on_click(cx.listener(
                                move |this, _, window, cx| (hooks.done)(this, window, cx),
                            ))),
                    )
                    .with_animation(
                        "pick-in",
                        Animation::new(katna_ui::motion::time(SLIDE)).with_easing(ease_out_quint()),
                        |el, t| el.relative().left(px(SLIDE_BY * (1.0 - t))).opacity(t),
                    ),
            )
            .into_any_element()
    }
}

/// Slides a menu's times back in from the left when `back`, coming back
/// from the picker.
pub(super) fn slide_back<E: IntoElement + Styled + 'static>(
    id: &'static str,
    back: bool,
    el: E,
) -> AnyElement {
    el.with_animation(
        (id, usize::from(back)),
        Animation::new(katna_ui::motion::time(SLIDE)).with_easing(ease_out_quint()),
        move |el, t| {
            if back {
                el.relative().left(px(-SLIDE_BY * (1.0 - t))).opacity(t)
            } else {
                el
            }
        },
    )
    .into_any_element()
}
