// SPDX-License-Identifier: GPL-3.0-or-later

//! A task's details in a dialog, as Google Tasks' task editor: the title,
//! details (notes), a due day on a month grid, a time and how it repeats.
//! Save sends only what changed; Ctrl+Z puts the task back as it was.

use gpui::{
    AnyElement, Context, Entity, FocusHandle, Focusable, FontWeight, Pixels, Size, Subscription,
    Window, anchored, deferred, div, point, prelude::*, rgba,
};
use jiff::civil::{Date, Time};
use katna_i18n::{format, tr};
use katna_ui::{InputEvent, TextArea, TextInput, px, unpx};

use super::super::compose::schedule;
use super::super::{MailWindow, UndoStep};
use super::today;
use crate::daemon::Command;
use crate::tasks::{TaskCommand, TaskEdit};
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, elevation, filled_button, icon, icon_button, tip};

/// The widest the dialog gets.
const WIDTH: f32 = 600.0;
/// One day on the month grid.
const DAY: f32 = 34.0;

/// How a task repeats, as the dialog offers it.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Repeat {
    Never,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    /// A rule from the service the dialog has no button for (every second
    /// Tuesday); kept as it is unless another is picked.
    Other(String),
}

impl Repeat {
    fn from_rule(rule: &str) -> Self {
        match rule {
            "" => Self::Never,
            "FREQ=DAILY" => Self::Daily,
            "FREQ=WEEKLY" => Self::Weekly,
            "FREQ=MONTHLY" => Self::Monthly,
            "FREQ=YEARLY" => Self::Yearly,
            other => Self::Other(other.to_owned()),
        }
    }

    fn rule(&self) -> String {
        match self {
            Self::Never => String::new(),
            Self::Daily => "FREQ=DAILY".into(),
            Self::Weekly => "FREQ=WEEKLY".into(),
            Self::Monthly => "FREQ=MONTHLY".into(),
            Self::Yearly => "FREQ=YEARLY".into(),
            Self::Other(rule) => rule.clone(),
        }
    }

    fn label(&self) -> String {
        match self {
            Self::Never => tr!("tasks-repeat-never"),
            Self::Daily => tr!("tasks-repeat-daily"),
            Self::Weekly => tr!("tasks-repeat-weekly"),
            Self::Monthly => tr!("tasks-repeat-monthly"),
            Self::Yearly => tr!("tasks-repeat-yearly"),
            Self::Other(_) => tr!("tasks-repeat-other"),
        }
    }
}

/// The open dialog.
pub(super) struct Details {
    id: i64,
    title: Entity<TextInput>,
    notes: Entity<TextArea>,
    time: Entity<TextInput>,
    /// The month the grid shows.
    month: Date,
    day: Option<Date>,
    repeat: Repeat,
    focus: FocusHandle,
    /// The window's size when opened, to center the dialog in.
    viewport: Size<Pixels>,
    _subscriptions: Vec<Subscription>,
}

/// Minutes after midnight as a time of day.
fn time_of(minutes: u32) -> Time {
    Time::new(
        i8::try_from(minutes / 60).unwrap_or(0),
        i8::try_from(minutes % 60).unwrap_or(0),
        0,
        0,
    )
    .unwrap_or_default()
}

fn minutes_of(time: Time) -> u32 {
    u32::try_from(i32::from(time.hour()) * 60 + i32::from(time.minute())).unwrap_or(0)
}

impl MailWindow {
    pub(in crate::window) fn task_open_details(
        &mut self,
        id: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(task) = self.tasks.task(id).cloned() else {
            return;
        };
        self.tasks.menu = None;
        self.tasks.adding = None;
        self.tasks.editing = None;
        self.tasks.picked = Some(id);
        let accent = rgba(self.theme(window).accent).into();
        let title = cx.new(|cx| {
            let mut input = TextInput::new(tr!("tasks-title-placeholder"), cx);
            input.set_accent(accent);
            input.set_text(task.title.clone(), cx);
            input
        });
        let notes = cx.new(|cx| {
            let mut area = TextArea::new(tr!("tasks-notes-placeholder"), cx);
            area.set_accent(accent);
            area.set_text(task.notes.clone(), task.notes.len(), cx);
            area
        });
        let clock = task
            .due_time
            .map(|m| schedule::clock(time_of(m)))
            .unwrap_or_default();
        let time = cx.new(|cx| {
            let mut input = TextInput::new(tr!("tasks-time-placeholder"), cx);
            input.set_accent(accent);
            input.set_text(clock, cx);
            input
        });
        let on_event =
            |this: &mut Self, event: &InputEvent, window: &mut Window, cx: &mut Context<Self>| {
                match event {
                    InputEvent::Submit => this.task_save_details(window, cx),
                    InputEvent::Cancel => this.task_close_details(window, cx),
                    InputEvent::Changed => {}
                }
            };
        let subscriptions = vec![
            cx.subscribe_in(&title, window, move |this, _, event, window, cx| {
                on_event(this, event, window, cx)
            }),
            cx.subscribe_in(&notes, window, move |this, _, event, window, cx| {
                on_event(this, event, window, cx)
            }),
            cx.subscribe_in(&time, window, move |this, _, event, window, cx| {
                on_event(this, event, window, cx)
            }),
        ];
        window.focus(&title.focus_handle(cx), cx);
        let day = task.due.parse::<Date>().ok();
        self.tasks.details = Some(Details {
            id,
            title,
            notes,
            time,
            month: day.unwrap_or_else(today),
            day,
            repeat: Repeat::from_rule(&task.repeat),
            focus: cx.focus_handle(),
            viewport: window.viewport_size(),
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    fn task_close_details(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tasks.details = None;
        // Back to the page it was opened on: Tasks, or the Calendar.
        if self.app == super::super::apps::App::Calendar {
            window.focus(&self.calendar.focus, cx);
        } else if let Some(focus) = &self.tasks.focus {
            window.focus(focus, cx);
        }
        cx.notify();
    }

    fn task_save_details(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(details) = &self.tasks.details else {
            return;
        };
        let id = details.id;
        let Some(task) = self.tasks.task(id).cloned() else {
            self.task_close_details(window, cx);
            return;
        };
        let title = details.title.read(cx).text().trim().to_owned();
        if title.is_empty() {
            window.focus(&details.title.focus_handle(cx), cx);
            return;
        }
        let notes = details.notes.read(cx).text().trim_end().to_owned();
        let typed = details.time.read(cx).text().trim().to_owned();
        let time = if typed.is_empty() {
            None
        } else if let Some(time) = schedule::parse_time(&typed) {
            Some(time)
        } else {
            let example = schedule::clock(Time::constant(16, 0, 0, 0));
            self.show_snackbar(
                tr!("tasks-not-a-time", text = typed, example = example),
                None,
                cx,
            );
            return;
        };
        // A time or a repeat needs a day: today, unless one is picked.
        let day = details
            .day
            .or_else(|| (time.is_some() || details.repeat != Repeat::Never).then(today));
        let due = day.map(|d| d.to_string()).unwrap_or_default();
        let due_time = day.and(time).map(minutes_of);
        let repeat = if day.is_some() {
            details.repeat.rule()
        } else {
            String::new()
        };
        let edit = TaskEdit {
            title: (title != task.title).then_some(title),
            notes: (notes != task.notes).then_some(notes),
            due: (due != task.due).then_some(due),
            due_time: (due_time != task.due_time).then_some(due_time),
            repeat: (repeat != task.repeat).then_some(repeat),
            ..TaskEdit::default()
        };
        self.task_close_details(window, cx);
        if edit == TaskEdit::default() {
            return;
        }
        // Shown at once; the store follows.
        if let Some(Ok(board)) = &mut self.tasks.board
            && let Some(shown) = board
                .columns
                .iter_mut()
                .flat_map(|c| c.tasks.iter_mut())
                .find(|t| t.id == id)
        {
            if let Some(title) = &edit.title {
                shown.title = title.clone();
            }
            if let Some(notes) = &edit.notes {
                shown.notes = notes.clone();
            }
            if let Some(due) = &edit.due {
                shown.due = due.clone();
            }
            if let Some(due_time) = edit.due_time {
                shown.due_time = due_time;
            }
            if let Some(repeat) = &edit.repeat {
                shown.repeat = repeat.clone();
            }
        }
        self.send_task(TaskCommand::Edit(id, edit), None, None, cx);
        // Ctrl+Z puts every field back.
        self.remember(UndoStep::Command(Command::Task(Box::new(
            TaskCommand::Edit(id, TaskEdit::all_of(&task)),
        ))));
    }

    fn task_details_pick(&mut self, day: Option<Date>, cx: &mut Context<Self>) {
        if let Some(details) = &mut self.tasks.details {
            details.day = day;
            if let Some(day) = day {
                details.month = day;
            } else {
                details.repeat = Repeat::Never;
            }
        }
        cx.notify();
    }

    fn task_details_repeat(&mut self, repeat: Repeat, cx: &mut Context<Self>) {
        if let Some(details) = &mut self.tasks.details {
            if repeat != Repeat::Never && details.day.is_none() {
                let day = today();
                details.day = Some(day);
                details.month = day;
            }
            details.repeat = repeat;
        }
        cx.notify();
    }

    fn task_details_delete(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = self.tasks.details.as_ref().map(|d| d.id) else {
            return;
        };
        self.task_close_details(window, cx);
        self.task_delete(id, cx);
    }

    // --- Drawing -------------------------------------------------------------

    pub(in crate::window) fn render_task_details(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let details = self.tasks.details.as_ref()?;
        let vw = unpx(details.viewport.width);
        let vh = unpx(details.viewport.height);
        let width = WIDTH.min(vw - 32.0);
        let field = |id: &'static str| {
            div()
                .id(id)
                .px(px(12.0))
                .flex()
                .flex_row()
                .items_center()
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(th.divider))
                .text_size(px(14.0))
        };
        let label = |text: String| {
            div()
                .pb(px(6.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(text)
        };

        let title = field("task-details-title")
            .h(px(48.0))
            .text_size(px(18.0))
            .child(div().flex_1().min_w_0().child(details.title.clone()));
        let notes = field("task-details-notes")
            .items_start()
            .py(px(10.0))
            .min_h(px(84.0))
            .max_h(px(180.0))
            .overflow_y_scroll()
            .gap(px(12.0))
            .line_height(px(20.0))
            .child(div().pt(px(1.0)).child(icon("notes", th.text_dim, 18.0)))
            .child(div().flex_1().min_w_0().child(details.notes.clone()));

        // The month grid.
        let now = today();
        let month = details.month;
        let picked = details.day;
        let days = schedule::month_grid(month, format::first_weekday())
            .into_iter()
            .enumerate()
            .map(|(ix, date)| {
                let selected = picked == Some(date);
                let other = date.month() != month.month();
                div()
                    .id(("task-details-day", ix))
                    .size(px(DAY))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .text_size(px(13.0))
                    .cursor_pointer()
                    .when(other, |d| d.text_color(rgba(th.text_faint)))
                    .when(date == now && !selected, |d| {
                        d.border_1()
                            .border_color(rgba(th.accent))
                            .text_color(rgba(th.accent))
                    })
                    .when(selected, |d| {
                        d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                    })
                    .when(!selected, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        // A second click on the picked day takes it away.
                        let day = (!selected).then_some(date);
                        this.task_details_pick(day, cx)
                    }))
                    .child(format::number(date.day() as u64))
            })
            .collect::<Vec<_>>();
        let step = |months: i32| {
            move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                if let Some(details) = &mut this.tasks.details
                    && let Ok(m) = details
                        .month
                        .first_of_month()
                        .checked_add(jiff::Span::new().months(months))
                {
                    details.month = m;
                }
                cx.notify();
            }
        };
        let weekdays = format::weekdays_short().into_iter().map(|(_, d)| {
            div()
                .size(px(DAY))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .child(d)
        });
        let calendar = div()
            .flex_none()
            .w(px(7.0 * DAY))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(32.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .pl(px(8.0))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(format::month_year(month)),
                    )
                    .child(
                        icon_button("task-details-prev", "chevron-left", 20.0, th)
                            .size(px(32.0))
                            .on_click(cx.listener(step(-1))),
                    )
                    .child(
                        icon_button("task-details-next", "chevron-right", 20.0, th)
                            .size(px(32.0))
                            .on_click(cx.listener(step(1))),
                    ),
            )
            .child(
                div()
                    .mt(px(4.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .children(weekdays)
                    .children(days),
            );

        // The picked day, the time and the repeat beside the grid.
        let day_text = match picked {
            Some(day) => format::day_month_year(day.to_datetime(Time::midnight())),
            None => tr!("tasks-no-date"),
        };
        let day_row = field("task-details-day-shown")
            .h(px(40.0))
            .gap(px(10.0))
            .child(icon(
                "calendar",
                if picked.is_some() {
                    th.accent
                } else {
                    th.text_dim
                },
                18.0,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .when(picked.is_none(), |d| d.text_color(rgba(th.text_faint)))
                    .child(day_text),
            )
            .when(picked.is_some(), |d| {
                d.pr(px(4.0)).child(
                    icon_button("task-details-no-day", "close", 18.0, th)
                        .size(px(28.0))
                        .tooltip(tip(tr!("tasks-no-date"), th))
                        .on_click(cx.listener(|this, _, _, cx| this.task_details_pick(None, cx))),
                )
            });
        let time_row = field("task-details-time")
            .h(px(40.0))
            .gap(px(10.0))
            .child(icon("schedule", th.text_dim, 18.0))
            .child(div().flex_1().min_w_0().child(details.time.clone()));
        let mut choices = vec![
            Repeat::Never,
            Repeat::Daily,
            Repeat::Weekly,
            Repeat::Monthly,
            Repeat::Yearly,
        ];
        if let Repeat::Other(_) = details.repeat {
            choices.push(details.repeat.clone());
        }
        let repeats = choices.into_iter().enumerate().map(|(ix, repeat)| {
            let on = repeat == details.repeat;
            div()
                .id(("task-details-repeat", ix))
                .focus_ring(th)
                .h(px(32.0))
                .px(px(12.0))
                .flex()
                .items_center()
                .rounded(px(8.0))
                .border_1()
                .text_size(px(13.0))
                .cursor_pointer()
                .map(|d| {
                    if on {
                        d.border_color(rgba(fade(th.accent, 0.5)))
                            .bg(rgba(fade(th.accent, 0.12)))
                            .text_color(rgba(th.accent))
                            .font_weight(FontWeight::MEDIUM)
                    } else {
                        d.border_color(rgba(th.divider))
                            .text_color(rgba(th.text_dim))
                            .hover(|s| s.bg(rgba(th.hover)))
                    }
                })
                .child(repeat.label())
                .on_click(
                    cx.listener(move |this, _, _, cx| this.task_details_repeat(repeat.clone(), cx)),
                )
        });
        let when = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .child(label(tr!("tasks-date")))
            .child(day_row)
            .child(div().mt(px(12.0)).child(time_row))
            .child(div().mt(px(20.0)).child(label(tr!("tasks-repeat"))))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(8.0))
                    .children(repeats),
            );
        // Side by side when there is room, else one above the other.
        let side_by_side = width >= 7.0 * DAY + 48.0 + 24.0 + 200.0;
        let date_part = div()
            .mt(px(20.0))
            .flex()
            .gap(px(24.0))
            .map(|d| {
                if side_by_side {
                    d.flex_row()
                } else {
                    d.flex_col()
                }
            })
            .child(calendar)
            .child(when);

        let cancel = div()
            .id("task-details-cancel")
            .focus_ring(th)
            .h(px(36.0))
            .px(px(16.0))
            .flex()
            .items_center()
            .rounded_full()
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.accent))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(fade(th.accent, 0.08))))
            .on_click(cx.listener(|this, _, window, cx| this.task_close_details(window, cx)))
            .child(tr!("tasks-cancel"));
        let save = filled_button("task-details-save", tr!("tasks-save"), th)
            .on_click(cx.listener(|this, _, window, cx| this.task_save_details(window, cx)));
        let delete = icon_button("task-details-delete", "trash", 20.0, th)
            .size(px(36.0))
            .tooltip(tip(tr!("tasks-delete"), th))
            .on_click(cx.listener(|this, _, window, cx| this.task_details_delete(window, cx)));

        let focus = details.focus.clone();
        let card = div()
            .id("task-details")
            .track_focus(&focus)
            .map(|d| super::super::popovers::keep_tab_inside(d, &focus))
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, window, cx| {
                let keys = &event.keystroke;
                if keys.key == "escape" {
                    cx.stop_propagation();
                    this.task_close_details(window, cx);
                } else if keys.key == "enter" && keys.modifiers.control {
                    cx.stop_propagation();
                    this.task_save_details(window, cx);
                }
            }))
            .occlude()
            .w(px(width))
            .max_h(px(vh * 0.9))
            .overflow_y_scroll()
            .px(px(24.0))
            .pt(px(24.0))
            .pb(px(20.0))
            .flex()
            .flex_col()
            .rounded(px(15.0))
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .child(title)
            .child(div().mt(px(12.0)).child(notes))
            .child(date_part)
            .child(
                div()
                    .mt(px(24.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(delete)
                    .child(div().flex_1())
                    .child(cancel)
                    .child(save),
            );
        Some(
            deferred(
                anchored().position(point(px(0.0), px(0.0))).child(
                    div()
                        .id("task-details-scrim")
                        .w(px(vw))
                        .h(px(vh))
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgba(0x0000_0066))
                        .occlude()
                        .on_click(
                            cx.listener(|this, _, window, cx| this.task_close_details(window, cx)),
                        )
                        .child(
                            // Clicks inside the card stay there.
                            div()
                                .id("task-details-card")
                                .on_click(|_, _, cx| cx.stop_propagation())
                                .child(card),
                        ),
                ),
            )
            .with_priority(5)
            .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_rules_round_trip() {
        for rule in [
            "",
            "FREQ=DAILY",
            "FREQ=WEEKLY",
            "FREQ=MONTHLY",
            "FREQ=YEARLY",
        ] {
            assert_eq!(Repeat::from_rule(rule).rule(), rule);
        }
        let other = "FREQ=WEEKLY;INTERVAL=2;BYDAY=TU";
        assert_eq!(Repeat::from_rule(other), Repeat::Other(other.into()));
        assert_eq!(Repeat::from_rule(other).rule(), other);
    }

    #[test]
    fn minutes_and_times_agree() {
        assert_eq!(minutes_of(time_of(16 * 60 + 5)), 16 * 60 + 5);
        assert_eq!(time_of(0), Time::midnight());
    }
}
