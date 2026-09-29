// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendar sets, as Fantastical has them: named groups of calendars
//! ("Work", "Personal") in the side column. A click shows the set's
//! calendars and hides the others; + saves the calendars on show as a new
//! set (`[[calendar.sets]]` in `config.toml`, `docs/ARCHITECTURE.md` §18).

use gpui::{
    AnyElement, Context, Focusable, FontWeight, Subscription, Window, div, prelude::*, rgba,
};
use katna_core::config::CalendarSet;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::text_input::{InputEvent, TextInput};

use super::super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{FocusRing, icon, icon_button, tip};

/// A new set's name being typed.
pub(in crate::window) struct Naming {
    input: gpui::Entity<TextInput>,
    _subscription: Subscription,
}

impl MailWindow {
    /// The calendars on show, in the side column's order.
    fn shown_calendars(&self) -> Vec<i64> {
        self.calendar
            .calendars
            .iter()
            .map(|c| c.id)
            .filter(|id| !self.calendar_hidden(*id))
            .collect()
    }

    /// Whether `set` is what is on show: its calendars that still exist
    /// are shown, and no others.
    fn set_on_show(&self, set: &CalendarSet) -> bool {
        self.calendar
            .calendars
            .iter()
            .all(|c| set.calendars.contains(&c.id) != self.calendar_hidden(c.id))
    }

    /// Shows set `ix`'s calendars and hides the others.
    fn apply_calendar_set(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(set) = self.config.calendar.sets.get(ix).cloned() else {
            return;
        };
        let ids: Vec<i64> = self.calendar.calendars.iter().map(|c| c.id).collect();
        for id in ids {
            if set.calendars.contains(&id) == self.calendar_hidden(id) {
                self.toggle_calendar(id, cx);
            }
        }
        cx.notify();
    }

    fn remove_calendar_set(&mut self, ix: usize, cx: &mut Context<Self>) {
        if ix < self.config.calendar.sets.len() {
            self.config.calendar.sets.remove(ix);
            self.save_config();
            cx.notify();
        }
    }

    /// Asks for the name of a new set of the calendars on show.
    fn start_calendar_set(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("calendar-set-name"), cx);
            input.set_accent(accent);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.finish_calendar_set(true, window, cx),
                InputEvent::Cancel => this.finish_calendar_set(false, window, cx),
                InputEvent::Changed => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.calendar.naming_set = Some(Naming {
            input,
            _subscription: subscription,
        });
        cx.notify();
    }

    /// Esc on the page while a set is being named puts the box away.
    pub(super) fn cancel_calendar_set(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.calendar.naming_set.is_none() {
            return false;
        }
        self.finish_calendar_set(false, window, cx);
        true
    }

    fn finish_calendar_set(&mut self, save: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(naming) = self.calendar.naming_set.take() else {
            return;
        };
        window.focus(&self.calendar.focus, cx);
        cx.notify();
        let name = naming.input.read(cx).text().trim().to_owned();
        if !save || name.is_empty() {
            return;
        }
        let calendars = self.shown_calendars();
        let sets = &mut self.config.calendar.sets;
        // The same name again takes the calendars on show now.
        match sets.iter_mut().find(|s| s.name == name) {
            Some(set) => set.calendars = calendars,
            None => sets.push(CalendarSet { name, calendars }),
        }
        self.save_config();
    }

    /// The sets in the side column, under a heading with +.
    pub(super) fn render_calendar_sets(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let sets = self
            .config
            .calendar
            .sets
            .iter()
            .enumerate()
            .map(|(ix, set)| {
                let on = self.set_on_show(set);
                let group = format!("calendar-set-{ix}");
                div()
                    .id(("calendar-set", ix))
                    .group(group.clone())
                    .h(px(32.0))
                    .pl(px(8.0))
                    .pr(px(4.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .when(on, |d| d.bg(rgba(th.nav_selected)))
                    .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .focus_ring(th)
                    .on_click(cx.listener(move |this, _, _, cx| this.apply_calendar_set(ix, cx)))
                    .child(icon(
                        "calendar",
                        if on {
                            th.nav_selected_text
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
                            .text_size(px(14.0))
                            .when(on, |d| {
                                d.font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.nav_selected_text))
                            })
                            .when(!on, |d| d.text_color(rgba(th.text)))
                            .child(set.name.clone()),
                    )
                    .child(
                        div().invisible().group_hover(group, |s| s.visible()).child(
                            icon_button(("calendar-set-remove", ix), "close", 16.0, th)
                                .tooltip(tip(tr!("calendar-set-remove"), th))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    cx.stop_propagation();
                                    this.remove_calendar_set(ix, cx);
                                })),
                        ),
                    )
            });
        let naming = self.calendar.naming_set.as_ref().map(|naming| {
            div()
                // A click elsewhere with no name typed puts the box away,
                // as Esc does; a typed name waits for Enter.
                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                    let empty = this
                        .calendar
                        .naming_set
                        .as_ref()
                        .is_some_and(|n| n.input.read(cx).text().trim().is_empty());
                    if empty {
                        this.finish_calendar_set(false, window, cx);
                    }
                }))
                .h(px(36.0))
                .px(px(12.0))
                .flex()
                .items_center()
                .rounded_full()
                .border_1()
                .border_color(rgba(th.accent))
                .text_size(px(14.0))
                .child(div().flex_1().child(naming.input.clone()))
        });
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(36.0))
                    .pl(px(8.0))
                    .pr(px(4.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text))
                            .child(tr!("calendar-sets")),
                    )
                    .child(
                        icon_button("calendar-set-add", "add", 20.0, th)
                            .focus_ring(th)
                            .tooltip(tip(tr!("calendar-set-add"), th))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.start_calendar_set(window, cx)
                            })),
                    ),
            )
            .children(sets)
            .children(naming)
            .into_any_element()
    }
}
