// SPDX-License-Identifier: GPL-3.0-or-later

//! Snooze, as in webmail: a menu of suggested times (later today,
//! tomorrow, this weekend, next week) and a date and time picker. The
//! background service moves the conversation to the Snoozed folder and
//! brings it back to the Inbox, unread and on top, at that time, even with
//! the app closed (`katna-daemon`, `docs/ARCHITECTURE.md` §10.1).

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, Hsla, MouseButton,
    Pixels, Point, Subscription, Window, deferred, div, ease_out_quint, prelude::*, rgba,
};
use jiff::civil::{Date, Time, Weekday};
use jiff::tz::TimeZone;
use jiff::{Timestamp, Zoned};
use katna_i18n::{format, tr};
use katna_ui::anchored;
use katna_ui::{InputEvent, TextInput, px};

use super::MenuKey;
use super::compose::schedule;
use super::{Act, MailWindow};
use crate::data::EntryKey;
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, icon_button, raised};

const MENU_WIDTH: f32 = 300.0;

/// The open snooze menu, for the lines `keys`; or the reminder menu of
/// notes, the same times and picker.
pub(super) struct SnoozeMenu {
    keys: Vec<EntryKey>,
    /// The notes it sets a reminder on, when it is their menu.
    notes: Vec<i64>,
    /// Those notes have a reminder, which it can take off.
    reminded: bool,
    /// The follow-up (by outbox entry) it moves, when it is the menu of
    /// a follow-up's Edit.
    follow_up: Option<i64>,
    /// Where it opens, in the window.
    at: Point<Pixels>,
    picker: Option<Picker>,
}

/// The date and time picker.
struct Picker {
    /// The month the calendar shows.
    month: Date,
    day: Date,
    time: Entity<TextInput>,
    _events: Subscription,
}

/// A suggested time: its name and when.
pub(super) struct Preset {
    pub label: String,
    pub at: Zoned,
}

/// The suggested times at `now`: later today (6 PM, until 5 PM), tomorrow
/// morning, this weekend (Saturday morning, Monday to Thursday) and next
/// week (Monday morning).
pub(super) fn presets(now: &Zoned) -> Vec<Preset> {
    let at = |date: Date, hour: i8| -> Option<Zoned> {
        date.to_datetime(Time::constant(hour, 0, 0, 0))
            .to_zoned(now.time_zone().clone())
            .ok()
    };
    let today = now.date();
    let mut presets = Vec::new();
    if now.hour() < 17 {
        presets.extend(at(today, 18).map(|at| Preset {
            label: tr!("snooze-later-today"),
            at,
        }));
    }
    if let Ok(tomorrow) = today.tomorrow() {
        presets.extend(at(tomorrow, 8).map(|at| Preset {
            label: tr!("snooze-tomorrow"),
            at,
        }));
    }
    let weekday = today.weekday();
    if matches!(
        weekday,
        Weekday::Monday | Weekday::Tuesday | Weekday::Wednesday | Weekday::Thursday
    ) && let Ok(saturday) = today.nth_weekday(1, Weekday::Saturday)
    {
        presets.extend(at(saturday, 8).map(|at| Preset {
            label: tr!("snooze-this-weekend"),
            at,
        }));
    }
    if let Ok(monday) = today.nth_weekday(1, Weekday::Monday) {
        presets.extend(at(monday, 8).map(|at| Preset {
            label: tr!("snooze-next-week"),
            at,
        }));
    }
    presets
}

/// Tomorrow at 8 in the morning in `tz`, as Unix seconds: a mute's end.
pub(super) fn tomorrow_morning(tz: &TimeZone) -> Option<i64> {
    let now = Timestamp::now().to_zoned(tz.clone());
    now.date()
        .tomorrow()
        .ok()?
        .to_datetime(Time::constant(8, 0, 0, 0))
        .to_zoned(tz.clone())
        .ok()
        .map(|at| at.timestamp().as_second())
}

/// When snoozed mail comes back, for the snackbar and tooltips:
/// "Sun, Sep 27, 2026, 8:00 AM".
pub(super) fn describe(at: i64, tz: &TimeZone) -> String {
    Timestamp::from_second(at)
        .map(|at| format::long(at.to_zoned(tz.clone()).datetime()))
        .unwrap_or_default()
}

impl MailWindow {
    /// Opens the snooze menu for `keys` at `at`.
    pub(super) fn open_snooze_menu(
        &mut self,
        keys: Vec<EntryKey>,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        if keys.is_empty() {
            return;
        }
        self.menu = None;
        self.context_menu = None;
        self.snooze_menu = Some(SnoozeMenu {
            keys,
            notes: Vec::new(),
            reminded: false,
            follow_up: None,
            at,
            picker: None,
        });
        cx.notify();
    }

    /// Opens the reminder menu of notes `ids` at `at`; `reminded` offers
    /// to take their reminder off.
    pub(super) fn open_remind_menu(
        &mut self,
        ids: Vec<i64>,
        reminded: bool,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        if ids.is_empty() {
            return;
        }
        self.menu = None;
        self.context_menu = None;
        self.snooze_menu = Some(SnoozeMenu {
            keys: Vec::new(),
            notes: ids,
            reminded,
            follow_up: None,
            at,
            picker: None,
        });
        cx.notify();
    }

    /// Opens the menu of times for the follow-up of outbox entry `outbox`
    /// at `at`: Edit on the card of a conversation it waits on.
    pub(super) fn open_follow_up_times(
        &mut self,
        outbox: i64,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.menu = None;
        self.context_menu = None;
        self.snooze_menu = Some(SnoozeMenu {
            keys: Vec::new(),
            notes: Vec::new(),
            reminded: false,
            follow_up: Some(outbox),
            at,
            picker: None,
        });
        cx.notify();
    }

    /// Whether the menu of times is open (not its date picker).
    pub(super) fn snooze_times_open(&self) -> bool {
        self.snooze_menu
            .as_ref()
            .is_some_and(|m| m.picker.is_none())
    }

    /// Closes the snooze menu or its picker. Returns whether one was open.
    pub(super) fn close_snooze_menu(&mut self, cx: &mut Context<Self>) -> bool {
        let open = self.snooze_menu.take().is_some();
        if open {
            cx.notify();
        }
        open
    }

    fn snooze_until(&mut self, until: Timestamp, cx: &mut Context<Self>) {
        let Some(menu) = self.snooze_menu.take() else {
            return;
        };
        if !menu.notes.is_empty() {
            self.remind_notes(menu.notes, Some(until.as_second()), cx);
            return;
        }
        if let Some(outbox) = menu.follow_up {
            self.move_follow_up(outbox, until.as_second(), cx);
            return;
        }
        self.act(Act::Snooze(until.as_second()), menu.keys, cx);
    }

    fn open_snooze_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let now = Timestamp::now().to_zoned(self.tz.clone());
        let tomorrow = now.date().tomorrow().unwrap_or(now.date());
        let accent: Hsla = rgba(self.theme(window).accent).into();
        let morning = schedule::clock(Time::constant(8, 0, 0, 0));
        let time = cx.new(|cx| {
            let mut input = TextInput::new(morning.clone(), cx);
            input.set_accent(accent);
            input.set_text(morning, cx);
            input.select_all_text(cx);
            input.set_stepper(Some(schedule::time_stepper()));
            input
        });
        let events = cx.subscribe_in(
            &time,
            window,
            |this, _, event: &InputEvent, _, cx| match event {
                InputEvent::Submit => this.snooze_picked(cx),
                InputEvent::Cancel => {
                    this.close_snooze_menu(cx);
                }
                InputEvent::Changed => cx.notify(),
            },
        );
        window.focus(&time.focus_handle(cx), cx);
        if let Some(menu) = &mut self.snooze_menu {
            menu.picker = Some(Picker {
                month: tomorrow,
                day: tomorrow,
                time,
                _events: events,
            });
        }
        cx.notify();
    }

    /// Snoozes until the picker's date and time.
    fn snooze_picked(&mut self, cx: &mut Context<Self>) {
        let Some(picker) = self.snooze_menu.as_ref().and_then(|m| m.picker.as_ref()) else {
            return;
        };
        let text = picker.time.read(cx).text().to_owned();
        let Some(time) = schedule::parse_time(&text) else {
            let example = schedule::clock(Time::constant(8, 0, 0, 0));
            self.show_snackbar(
                tr!("schedule-not-a-time", text = text, example = example),
                None,
                cx,
            );
            return;
        };
        let Some(at) = schedule::moment(picker.day, time, &self.tz) else {
            self.show_snackbar(tr!("schedule-no-such-time"), None, cx);
            return;
        };
        if at.as_second() < Timestamp::now().as_second() + 60 {
            let notes = self
                .snooze_menu
                .as_ref()
                .is_some_and(|m| !m.notes.is_empty());
            self.show_snackbar(
                if notes {
                    tr!("notes-remind-in-the-past")
                } else {
                    tr!("snooze-in-the-past")
                },
                None,
                cx,
            );
            return;
        }
        self.snooze_until(at, cx);
    }

    pub(super) fn render_snooze_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let menu = self.snooze_menu.as_ref()?;
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
                this.close_snooze_menu(cx);
            })
        };
        let scrim = deferred(
            div()
                .id("snooze-scrim")
                .absolute()
                .top(px(-2000.0))
                .left(px(-4000.0))
                .w(px(8000.0))
                .h(px(6000.0))
                .occlude()
                .when(menu.picker.is_some(), |d| d.bg(rgba(0x0000_0040)))
                .on_mouse_down(MouseButton::Left, close())
                .on_mouse_down(MouseButton::Right, close()),
        )
        .with_priority(3);
        let panel = match &menu.picker {
            Some(picker) => deferred(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .id("snooze-picker")
                            .occlude()
                            .child(self.render_snooze_picker(picker, th, cx)),
                    ),
            )
            .with_priority(4)
            .into_any_element(),
            None => deferred(
                anchored()
                    .position(menu.at)
                    .snap_to_window_with_margin(px(8.0))
                    .child(div().occlude().child(self.render_snooze_times(th, cx))),
            )
            .with_priority(4)
            .into_any_element(),
        };
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(scrim)
                .child(panel)
                .into_any_element(),
        )
    }

    fn render_snooze_times(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let now = Timestamp::now().to_zoned(self.tz.clone());
        let (notes, reminded, follow_up) = self
            .snooze_menu
            .as_ref()
            .map_or((false, false, false), |m| {
                (!m.notes.is_empty(), m.reminded, m.follow_up.is_some())
            });
        let items = presets(&now).into_iter().enumerate().map(|(ix, preset)| {
            let at = preset.at.timestamp();
            div()
                .id(("snooze-preset", ix))
                .h(px(40.0))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .menu_key(th)
                .child(div().flex_1().child(preset.label))
                .child(
                    div()
                        .text_color(rgba(th.text_dim))
                        .child(format::day_month_time(preset.at.datetime())),
                )
                .on_click(cx.listener(move |this, _, _, cx| this.snooze_until(at, cx)))
        });
        raised(
            div()
                .key_context(crate::widgets::MENU_CONTEXT)
                .w(px(MENU_WIDTH))
                .py(px(8.0))
                .flex()
                .flex_col()
                .text_size(px(14.0))
                .text_color(rgba(th.text)),
            th,
            8.0,
            3.0,
        )
        .child(
            div()
                .px(px(16.0))
                .pt(px(4.0))
                .pb(px(8.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(if notes {
                    tr!("notes-remind-me")
                } else if follow_up {
                    tr!("follow-up-card-edit-title")
                } else {
                    tr!("snooze-until")
                }),
        )
        .children(items)
        .child(div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider)))
        .child(
            div()
                .id("snooze-pick")
                .h(px(40.0))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .menu_key(th)
                .child(icon("calendar", th.text_dim, 20.0))
                .child(tr!("snooze-pick"))
                .on_click(cx.listener(|this, _, window, cx| this.open_snooze_picker(window, cx))),
        )
        .when(reminded, |d| {
            d.child(
                div()
                    .id("remind-off")
                    .h(px(40.0))
                    .px(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .menu_key(th)
                    .child(icon("bell-off", th.text_dim, 20.0))
                    .child(tr!("notes-remind-off"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(menu) = this.snooze_menu.take() {
                            this.remind_notes(menu.notes, None, cx);
                        }
                    })),
            )
        })
        .with_animation(
            "snooze-menu",
            Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
                140,
            )))
            .with_easing(ease_out_quint()),
            |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
        )
        .into_any_element()
    }

    fn render_snooze_picker(
        &self,
        picker: &Picker,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let today = Timestamp::now().to_zoned(self.tz.clone()).date();
        let (month, day) = (picker.month, picker.day);
        let days = schedule::month_grid(month, format::first_weekday())
            .into_iter()
            .enumerate()
            .map(|(ix, date)| {
                let past = date < today;
                let selected = date == day;
                let other = date.month() != month.month();
                div()
                    .id(("snooze-day", ix))
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
                                if let Some(p) =
                                    this.snooze_menu.as_mut().and_then(|m| m.picker.as_mut())
                                {
                                    p.day = date;
                                    p.month = date;
                                }
                                cx.notify();
                            }))
                    })
                    .child(format::number(date.day() as u64))
            })
            .collect::<Vec<_>>();
        let step = |months: i32| {
            move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                if let Some(p) = this.snooze_menu.as_mut().and_then(|m| m.picker.as_mut())
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
            .w(px(330.0))
            .p(px(24.0))
            .flex()
            .flex_col()
            .map(|d| crate::widgets::dialog(d, th, th.menu))
            .text_color(rgba(th.text))
            .child(
                div()
                    .mb(px(16.0))
                    .text_size(px(20.0))
                    .child(tr!("snooze-pick")),
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
                        icon_button("snooze-prev", "chevron-left", 20.0, th)
                            .size(px(32.0))
                            .on_click(cx.listener(step(-1))),
                    )
                    .child(
                        icon_button("snooze-next", "chevron-right", 20.0, th)
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
                        field()
                            .flex_1()
                            .border_color(rgba(th.divider))
                            .child(format::day_month_year(day.to_datetime(Time::midnight()))),
                    )
                    .child(
                        field()
                            .w(px(110.0))
                            .border_color(rgba(th.accent))
                            .child(picker.time.clone()),
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
                            .id("snooze-cancel")
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
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_snooze_menu(cx);
                            }))
                            .child(tr!("snooze-cancel")),
                    )
                    .child(
                        filled_button("snooze-save", tr!("snooze-save"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.snooze_picked(cx))),
                    ),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(date: &str, time: &str) -> Zoned {
        // CI images may have no zone database, so no zone by name.
        let at: jiff::civil::DateTime = format!("{date}T{time}").parse().unwrap();
        at.to_zoned(jiff::tz::TimeZone::UTC).unwrap()
    }

    fn names(now: &Zoned) -> Vec<(String, String)> {
        presets(now)
            .into_iter()
            .map(|p| (p.label, p.at.datetime().to_string()))
            .collect()
    }

    #[test]
    fn suggests_times_as_webmail_does() {
        // Tuesday afternoon: later today, tomorrow, the weekend, Monday.
        assert_eq!(
            names(&at("2026-09-29", "14:00")),
            [
                ("Later today".into(), "2026-09-29T18:00:00".into()),
                ("Tomorrow".into(), "2026-09-30T08:00:00".into()),
                ("This weekend".into(), "2026-10-03T08:00:00".into()),
                ("Next week".into(), "2026-10-05T08:00:00".into()),
            ]
        );
        // Friday evening: no "later today", no "this weekend".
        assert_eq!(
            names(&at("2026-10-02", "19:30")),
            [
                ("Tomorrow".into(), "2026-10-03T08:00:00".into()),
                ("Next week".into(), "2026-10-05T08:00:00".into()),
            ]
        );
    }
}
