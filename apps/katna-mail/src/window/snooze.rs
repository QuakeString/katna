// SPDX-License-Identifier: GPL-3.0-or-later

//! Snooze, as in webmail: a menu of suggested times (later today,
//! tomorrow, this weekend, next week) and a date and time picker. The
//! background service moves the conversation to the Snoozed folder and
//! brings it back to the Inbox, unread and on top, at that time, even with
//! the app closed (`katna-daemon`, `docs/ARCHITECTURE.md` §10.1).

use gpui::{
    Animation, AnimationExt, AnyElement, ClickEvent, Context, Entity, Focusable, FontWeight, Hsla,
    MouseButton, Pixels, Point, Subscription, Window, deferred, div, ease_out_quint, prelude::*,
    relative, rgba,
};
use jiff::civil::{Date, Time};
use jiff::tz::TimeZone;
use jiff::{Timestamp, Zoned};
use katna_core::config::SnoozeTimes;
use katna_i18n::{format, tr};
use katna_ui::anchored;
use katna_ui::tokens::{radius, space, text};
use katna_ui::{InputEvent, TextInput, px};

use super::MenuKey;
use super::compose::schedule;
use super::{Act, MailWindow};
use crate::data::EntryKey;
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, icon_button, icon_tag, raised};
use katna_store::MessageId;

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
    /// On mail lines: Remind me beside Snooze, one click (or B and H)
    /// apart.
    remind: Option<Remind>,
}

/// Remind me, the other half of the mail lines' menu: it keeps the mail
/// where it is and makes a task in Tasks that notifies at the time.
pub(super) struct Remind {
    /// Remind me is picked, rather than Snooze.
    pub on: bool,
    /// What the task says; the subject when left empty.
    pub note: Entity<TextInput>,
    /// The time before the day the mail says something is due, if it says.
    pub before_due: Option<Zoned>,
}

/// The date and time picker.
struct Picker {
    /// The month the calendar shows.
    month: Date,
    day: Date,
    time: Entity<TextInput>,
    /// "tue 3pm", "in 2 hours": fills the day and the time.
    typed: Entity<TextInput>,
    /// Whether what is typed was not understood.
    unclear: bool,
    _events: [Subscription; 2],
}

/// A suggested time: its name and when.
pub(super) struct Preset {
    pub label: String,
    pub at: Zoned,
}

/// The suggested times at `now`, as `times` (Settings > Inbox > Snooze
/// times) has them: later today (6 PM, until an hour before), tomorrow
/// morning, this weekend (Saturday morning, two to five days ahead), next
/// week (Monday morning), and the user's own time when it is still to
/// come, soonest first.
pub(super) fn presets(now: &Zoned, times: &SnoozeTimes) -> Vec<Preset> {
    let clock = |minutes: u32| {
        let minutes = minutes.min(24 * 60 - 1);
        Time::new((minutes / 60) as i8, (minutes % 60) as i8, 0, 0).unwrap_or(Time::midnight())
    };
    let at = |date: Date, time: Time| -> Option<Zoned> {
        date.to_datetime(time)
            .to_zoned(now.time_zone().clone())
            .ok()
    };
    let morning = clock(times.morning);
    let today = now.date();
    let mut presets = Vec::new();
    let later = clock(times.later_today);
    if now
        .time()
        .checked_add(jiff::Span::new().hours(1))
        .is_ok_and(|t| t <= later)
    {
        presets.extend(at(today, later).map(|at| Preset {
            label: tr!("snooze-later-today"),
            at,
        }));
    }
    if let Ok(tomorrow) = today.tomorrow() {
        presets.extend(at(tomorrow, morning).map(|at| Preset {
            label: tr!("snooze-tomorrow"),
            at,
        }));
    }
    if let Ok(weekend) = today.nth_weekday(1, times.weekend.weekday())
        && (2..=5).contains(&(weekend - today).get_days())
    {
        presets.extend(at(weekend, morning).map(|at| Preset {
            label: tr!("snooze-this-weekend"),
            at,
        }));
    }
    if let Ok(week) = today.nth_weekday(1, times.next_week.weekday())
        && let Some(at) = at(week, morning)
        && !presets.iter().any(|p| p.at == at)
    {
        presets.push(Preset {
            label: tr!("snooze-next-week"),
            at,
        });
    }
    if let Some(own) = own_time(&times.own, now, morning)
        && !presets.iter().any(|p| p.at == own.at)
    {
        presets.push(own);
        presets.sort_by_key(|p| p.at.timestamp());
    }
    presets
}

/// The user's own snooze time `text` ("monday 10:00") at `now`: named as
/// typed, when it is still to come.
pub(super) fn own_time(text: &str, now: &Zoned, morning: Time) -> Option<Preset> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let language = katna_i18n::current().language.tag.clone();
    let words = katna_core::quick_add::Words::for_language(&language);
    let at = katna_core::quick_add::moment(text, now.datetime(), morning, words)?
        .to_zoned(now.time_zone().clone())
        .ok()?;
    let mut label: Vec<char> = text.chars().collect();
    if let Some(first) = label.first_mut() {
        *first = first.to_uppercase().next().unwrap_or(*first);
    }
    Some(Preset {
        label: label.into_iter().collect(),
        at,
    })
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
        self.open_mail_times(keys, false, at, cx);
    }

    /// Opens the menu of times for mail lines `keys` at `at`, with Snooze
    /// or (`remind`) Remind me picked.
    pub(super) fn open_mail_times(
        &mut self,
        keys: Vec<EntryKey>,
        remind: bool,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        if keys.is_empty() {
            return;
        }
        let note = cx.new(|cx| TextInput::new(tr!("remind-note-placeholder"), cx));
        let before_due = self.before_due(&keys);
        self.menu = None;
        self.context_menu = None;
        self.snooze_menu = Some(SnoozeMenu {
            keys,
            notes: Vec::new(),
            reminded: false,
            follow_up: None,
            at,
            picker: None,
            remind: Some(Remind {
                on: remind,
                note,
                before_due,
            }),
        });
        cx.notify();
    }

    /// Switches the mail lines' menu between Snooze and Remind me.
    fn pick_remind(&mut self, on: bool, cx: &mut Context<Self>) {
        if let Some(remind) = self.snooze_menu.as_mut().and_then(|m| m.remind.as_mut()) {
            remind.on = on;
            cx.notify();
        }
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
            remind: None,
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
            remind: None,
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
        if let Some(remind) = menu.remind.filter(|r| r.on) {
            let note = remind.note.read(cx).text().trim().to_owned();
            self.remind_mails(menu.keys, until.as_second(), note, cx);
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
        let typed = cx.new(|cx| {
            let mut input = TextInput::new(tr!("snooze-type-placeholder"), cx);
            input.set_accent(accent);
            input
        });
        let typed_events =
            cx.subscribe_in(
                &typed,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => this.snooze_picked(cx),
                    InputEvent::Cancel => {
                        this.close_snooze_menu(cx);
                    }
                    InputEvent::Changed => this.snooze_typed(cx),
                },
            );
        window.focus(&typed.focus_handle(cx), cx);
        if let Some(menu) = &mut self.snooze_menu {
            menu.picker = Some(Picker {
                month: tomorrow,
                day: tomorrow,
                time,
                typed,
                unclear: false,
                _events: [events, typed_events],
            });
        }
        cx.notify();
    }

    /// Reads the typed moment into the picker's day and time.
    fn snooze_typed(&mut self, cx: &mut Context<Self>) {
        let Some(picker) = self.snooze_menu.as_ref().and_then(|m| m.picker.as_ref()) else {
            return;
        };
        let text = picker.typed.read(cx).text().to_owned();
        let time_input = picker.time.clone();
        let now = Timestamp::now().to_zoned(self.tz.clone());
        let language = katna_i18n::current().language.tag.clone();
        let words = katna_core::quick_add::Words::for_language(&language);
        let at =
            katna_core::quick_add::moment(&text, now.datetime(), Time::constant(8, 0, 0, 0), words);
        if let Some(at) = at {
            time_input.update(cx, |input, cx| {
                input.set_text(schedule::clock(at.time()), cx);
            });
        }
        if let Some(p) = self.snooze_menu.as_mut().and_then(|m| m.picker.as_mut()) {
            p.unclear = at.is_none() && !text.trim().is_empty();
            if let Some(at) = at {
                p.day = at.date();
                p.month = at.date();
            }
        }
        cx.notify();
    }

    /// Snoozes until the picker's date and time.
    fn snooze_picked(&mut self, cx: &mut Context<Self>) {
        let Some(picker) = self.snooze_menu.as_ref().and_then(|m| m.picker.as_ref()) else {
            return;
        };
        if picker.unclear {
            let text = picker.typed.read(cx).text().to_owned();
            self.show_snackbar(tr!("snooze-type-unclear", text = text), None, cx);
            return;
        }
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
        window: &Window,
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
                    .child(
                        div()
                            .occlude()
                            .child(self.render_snooze_times(th, window, cx)),
                    ),
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

    fn render_snooze_times(
        &self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let now = Timestamp::now().to_zoned(self.tz.clone());
        let (notes, reminded, follow_up) = self
            .snooze_menu
            .as_ref()
            .map_or((false, false, false), |m| {
                (!m.notes.is_empty(), m.reminded, m.follow_up.is_some())
            });
        let remind = self.snooze_menu.as_ref().and_then(|m| m.remind.as_ref());
        let item = |id: gpui::ElementId, label: String, at: &Zoned| {
            let when = at.timestamp();
            div()
                .id(id)
                .h(px(40.0))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .menu_key(th)
                .child(div().flex_1().min_w_0().truncate().child(label))
                .child(
                    div()
                        .flex_none()
                        .text_color(rgba(th.text_dim))
                        .child(format::day_month_time(at.datetime())),
                )
                .on_click(cx.listener(move |this, _, _, cx| this.snooze_until(when, cx)))
        };
        // A day the mail says something is due by comes first.
        let before_due = remind
            .filter(|r| r.on)
            .and_then(|r| r.before_due.as_ref())
            .map(|at| item("remind-before-due".into(), tr!("remind-before-due"), at));
        let items = presets(&now, &self.config.mail.snooze)
            .into_iter()
            .enumerate()
            .map(|(ix, preset)| item(("snooze-preset", ix).into(), preset.label, &preset.at));
        // The keys that open each, as they are set now.
        let snooze_key = super::keymap::hint("snooze", &self.config.shortcuts);
        let remind_key = super::keymap::hint("remind", &self.config.shortcuts);
        let header = match remind {
            Some(remind) => {
                let tab = |id: &'static str, name: &'static str, label: String, key, on: bool| {
                    let (fill, hover) = crate::widgets::tonal_fill(th);
                    div()
                        .id(id)
                        .h(px(32.0))
                        .px(px(space::S3))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(space::S2))
                        .rounded(px(radius::SM))
                        .border_1()
                        .border_color(rgba(if on { fill } else { th.outline }))
                        .when(on, |d| d.bg(rgba(fill)).text_color(rgba(th.accent)))
                        .when(!on, |d| d.hover(move |s| s.bg(rgba(hover))))
                        .cursor_pointer()
                        .text_size(px(text::BODY))
                        .child(icon(name, if on { th.accent } else { th.text_dim }, 16.0))
                        .child(label)
                        .when_some(key, |d, key| {
                            d.child(
                                div()
                                    .px(px(space::S2))
                                    .rounded(px(radius::XS))
                                    .border_1()
                                    .border_color(rgba(th.outline))
                                    .text_size(px(text::CAPTION))
                                    .text_color(rgba(th.text_dim))
                                    .child(key),
                            )
                        })
                };
                div()
                    .px(px(space::S5))
                    .pt(px(space::S2))
                    .pb(px(space::S3))
                    .flex()
                    .flex_col()
                    .gap(px(space::S3))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(space::S3))
                            .child(
                                tab(
                                    "snooze-tab",
                                    "snooze",
                                    tr!("snooze-tab"),
                                    snooze_key.clone(),
                                    !remind.on,
                                )
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.pick_remind(false, cx)),
                                ),
                            )
                            .child(
                                tab(
                                    "remind-tab",
                                    "bell-plus",
                                    tr!("remind-tab"),
                                    remind_key.clone(),
                                    remind.on,
                                )
                                .on_click(cx.listener(|this, _, _, cx| this.pick_remind(true, cx))),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(text::CAPTION))
                            .text_color(rgba(th.text_dim))
                            .child(if remind.on {
                                tr!("remind-says")
                            } else {
                                tr!("snooze-says")
                            }),
                    )
                    .into_any_element()
            }
            None => div()
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
                })
                .into_any_element(),
        };
        // The note, for Remind me: what the task will say.
        let note = remind.filter(|r| r.on).map(|r| {
            let focus = r.note.focus_handle(cx);
            div()
                .px(px(space::S5))
                .pt(px(space::S3))
                .pb(px(space::S2))
                .flex()
                .flex_col()
                .gap(px(space::S2))
                .child(
                    div()
                        .text_size(px(text::CAPTION))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.text_dim))
                        .child(tr!("remind-note")),
                )
                .child(
                    crate::widgets::field("remind-note", &focus, th)
                        .h(px(36.0))
                        .flex()
                        .items_center()
                        .child(r.note.clone()),
                )
        });
        let typing = remind.is_some_and(|r| r.note.focus_handle(cx).is_focused(window));
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
        // B and H switch between Snooze and Remind me, as they open them.
        .when(remind.is_some() && !typing, |d| {
            d.on_key_down(cx.listener(|this, e: &gpui::KeyDownEvent, _, cx| {
                if e.keystroke.modifiers.modified() {
                    return;
                }
                match e.keystroke.key.as_str() {
                    "b" => this.pick_remind(false, cx),
                    "h" => this.pick_remind(true, cx),
                    _ => return,
                }
                cx.stop_propagation();
            }))
        })
        .child(header)
        .children(before_due)
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
        .when_some(note, |d, note| {
            d.child(div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider)))
                .child(note)
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
                field()
                    .border_color(rgba(if picker.unclear {
                        th.warning
                    } else {
                        th.accent
                    }))
                    .child(picker.typed.clone()),
            )
            .child(
                div()
                    .mt(px(space::S2))
                    .mb(px(space::S3))
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(if picker.unclear {
                        th.warning
                    } else {
                        th.text_dim
                    }))
                    .child(if picker.unclear {
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
                            .border_color(rgba(th.outline))
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

impl MailWindow {
    /// A snoozed conversation open in a chat: a small line at its end, like
    /// the reminder's, with Change and Unsnooze.
    pub(super) fn render_chat_snoozed(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let reader = self.reader.as_ref()?;
        let mail = self.mail.as_ref().ok()?;
        let ids: Vec<MessageId> = reader.message_ids().into_iter().collect();
        let until = mail.snoozed_until(&ids)?;
        let key = reader.key;
        let when = Timestamp::from_second(until)
            .map(|at| schedule::short(&at.to_zoned(self.tz.clone())))
            .unwrap_or_default();
        let link = |id: &'static str, label: String| {
            div()
                .id(id)
                .flex_none()
                .px(px(space::S1))
                .rounded(px(radius::XS))
                .text_color(rgba(th.accent))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .hover(|s| s.underline())
                .child(label)
        };
        Some(
            icon_tag(
                "schedule",
                div()
                    .min_w_0()
                    .truncate()
                    .child(tr!("snooze-chat-line", date = when)),
                th,
            )
            .flex_shrink(1.0)
            .min_w_0()
            .self_center()
            .max_w(relative(0.9))
            .my(px(space::S3))
            .child(
                link("snoozed-change", tr!("snooze-chat-change")).on_click(cx.listener(
                    move |this, event: &ClickEvent, _, cx| {
                        this.open_mail_times(vec![key], false, event.position(), cx);
                    },
                )),
            )
            .child(
                link("snoozed-unsnooze", tr!("list-unsnooze")).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.act(super::Act::Unsnooze, vec![key], cx);
                    },
                )),
            )
            .into_any_element(),
        )
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
        names_with(now, &SnoozeTimes::default())
    }

    fn names_with(now: &Zoned, times: &SnoozeTimes) -> Vec<(String, String)> {
        presets(now, times)
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

    #[test]
    fn follows_the_users_own_times() {
        use katna_core::config::SnoozeDay;
        let times = SnoozeTimes {
            later_today: 20 * 60,
            morning: 9 * 60 + 30,
            weekend: SnoozeDay::Sunday,
            next_week: SnoozeDay::Sunday,
            own: "thursday 10am".into(),
        };
        // Tuesday afternoon, the week starting on Sunday: Sunday is both
        // the weekend and next week, so it shows once.
        assert_eq!(
            names_with(&at("2026-09-29", "14:00"), &times),
            [
                ("Later today".into(), "2026-09-29T20:00:00".into()),
                ("Tomorrow".into(), "2026-09-30T09:30:00".into()),
                ("Thursday 10am".into(), "2026-10-01T10:00:00".into()),
                ("This weekend".into(), "2026-10-04T09:30:00".into()),
            ]
        );
    }
}
