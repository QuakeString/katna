// SPDX-License-Identifier: GPL-3.0-or-later

//! The right-click menus of the Calendar page's side panel, in the same
//! card as the page's other menus (`calendar/menu.rs`):
//!
//! - on a calendar: Show only this, its color, Rename, and Delete or
//!   Remove from list (for one shared with the person; Google offers both
//!   for a calendar one manages);
//! - on an account's heading: New calendar, Show all or Hide all, and
//!   Account settings.
//!
//! What the account's service can't do stays in the menu, dimmed, with a
//! word on why. Names are typed in a box in the panel itself; a change goes
//! to the service first (`AddCalendar`, `RenameCalendar`,
//! `SetCalendarColor`, `DeleteCalendar`), and a rename or a new color says
//! so with Undo. Deleting asks first.

use gpui::{AnyElement, Context, Focusable, Stateful, Subscription, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_store::calendar::{Calendar, CalendarAccess, CalendarSource};
use katna_ui::px;
use katna_ui::text_input::{InputEvent, TextInput};

use super::super::MailWindow;
use super::super::context_menu::{Rows, Sub, menu_row_with};
use super::menu::{COLORS, CalTarget, color_name, dot, tick};
use super::{birthdays, calendar_color, calendar_name, parse_color};
use crate::daemon::{CalendarEdit, Command};
use crate::theme::Theme;

/// A calendar's name being typed in the side panel.
pub(in crate::window) struct Naming {
    pub(super) what: NameFor,
    pub(super) input: gpui::Entity<TextInput>,
    _subscription: Subscription,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::window) enum NameFor {
    /// A new name for this calendar.
    Rename(i64),
    /// A new calendar in this account (`None`: this computer).
    New(Option<i64>),
}

/// What the side panel's menus may do to a calendar, as its service
/// allows.
struct Can {
    rename: bool,
    /// Take it off the list, keeping it for its owner.
    unlist: bool,
    /// Delete it for everyone, or why not.
    delete: Result<bool, String>,
}

fn can(calendar: &Calendar, locals: usize) -> Can {
    let own = calendar.access == CalendarAccess::Owner;
    let main = || Err(tr!("calendar-why-main"));
    match calendar.source {
        CalendarSource::Local => Can {
            rename: true,
            unlist: false,
            delete: if locals > 1 {
                Ok(true)
            } else {
                Err(tr!("calendar-why-last"))
            },
        },
        // Google keeps both apart: a calendar one manages can go for
        // everyone, or only off one's own list.
        CalendarSource::Google => Can {
            rename: true,
            unlist: !calendar.is_primary,
            delete: if calendar.is_primary { main() } else { Ok(own) },
        },
        // Zoho lists every calendar read-only here, so whether one is the
        // person's own is asked when it is deleted.
        CalendarSource::Zoho => Can {
            rename: true,
            unlist: false,
            delete: if calendar.is_primary {
                main()
            } else {
                Ok(true)
            },
        },
        CalendarSource::Microsoft | CalendarSource::CalDav => Can {
            rename: own,
            unlist: !own && !calendar.is_primary,
            delete: if calendar.is_primary { main() } else { Ok(own) },
        },
    }
}

impl MailWindow {
    /// An item the service can't do: dimmed, with why at its end.
    fn off_item(
        &self,
        id: &'static str,
        name: &str,
        label: String,
        why: String,
        rh: f32,
        th: &Theme,
    ) -> Stateful<gpui::Div> {
        // Laid out as a menu row, but faint and without a hover.
        div()
            .id(id)
            .flex_none()
            .h(px(rh))
            .pl(px(16.0))
            .pr(px(24.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .text_color(rgba(th.text_faint))
            .child(
                div()
                    .flex_none()
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(crate::widgets::icon(name, th.text_faint, 20.0)),
            )
            .child(div().flex_1().min_w_0().truncate().child(label))
            .child(div().flex_none().text_size(px(12.0)).child(why))
    }

    /// The menu's items for a calendar or an account's heading.
    pub(super) fn side_menu_rows(
        &self,
        target: &CalTarget,
        rh: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (Rows, Vec<(Sub, f32)>) {
        let mut rows = Rows::new(rh);
        let mut parents = Vec::new();
        let item = |id: &'static str, name: &str, label: String| {
            self.context_item(id, name, label, rh, th, cx)
        };
        match target {
            CalTarget::Calendar(id) => {
                let id = *id;
                rows.item(
                    item("cal-only-this", "eye", tr!("calendar-menu-only-this")).on_click(
                        cx.listener(move |this, _, _, cx| {
                            this.close_context_menu(cx);
                            this.show_only_calendar(id, cx);
                        }),
                    ),
                );
                if id == birthdays::BIRTHDAYS {
                    rows.item(self.off_item(
                        "cal-rename",
                        "pen",
                        tr!("calendar-menu-rename"),
                        tr!("calendar-why-contacts"),
                        rh,
                        th,
                    ));
                    return (rows, parents);
                }
                let Some(calendar) = self.calendar.calendars.iter().find(|c| c.id == id) else {
                    return (rows, parents);
                };
                let locals = self
                    .calendar
                    .calendars
                    .iter()
                    .filter(|c| c.source == CalendarSource::Local)
                    .count();
                let can = can(calendar, locals);
                parents.push((
                    Sub::Color,
                    rows.item(self.context_parent(Sub::Color, rh, th, cx)),
                ));
                if can.rename {
                    rows.item(
                        item("cal-rename", "pen", tr!("calendar-menu-rename")).on_click(
                            cx.listener(move |this, _, window, cx| {
                                this.close_context_menu(cx);
                                this.start_calendar_naming(NameFor::Rename(id), window, cx);
                            }),
                        ),
                    );
                } else {
                    rows.item(self.off_item(
                        "cal-rename",
                        "pen",
                        tr!("calendar-menu-rename"),
                        tr!("calendar-why-owner"),
                        rh,
                        th,
                    ));
                }
                rows.rule(th);
                if can.unlist {
                    rows.item(
                        item("cal-unlist", "remove", tr!("calendar-menu-remove")).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.close_context_menu(cx);
                                this.ask_calendar_gone(id, false, cx);
                            }),
                        ),
                    );
                }
                match can.delete {
                    Ok(true) => {
                        rows.item(
                            item("cal-delete-calendar", "trash", tr!("calendar-menu-delete"))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.close_context_menu(cx);
                                    this.ask_calendar_gone(id, true, cx);
                                })),
                        );
                    }
                    // Someone else's: only off the list, above.
                    Ok(false) => {}
                    Err(why) => {
                        rows.item(self.off_item(
                            "cal-delete-calendar",
                            "trash",
                            tr!("calendar-menu-delete"),
                            why,
                            rh,
                            th,
                        ));
                    }
                }
            }
            CalTarget::Account(account) => {
                let account = *account;
                let reached = account.is_none_or(|a| {
                    self.calendar
                        .calendars
                        .iter()
                        .any(|c| c.account.map(|x| x.0) == Some(a))
                });
                if reached {
                    rows.item(
                        item("cal-new-calendar", "add", tr!("calendar-menu-new-calendar"))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.close_context_menu(cx);
                                this.start_calendar_naming(NameFor::New(account), window, cx);
                            })),
                    );
                } else {
                    rows.item(self.off_item(
                        "cal-new-calendar",
                        "add",
                        tr!("calendar-menu-new-calendar"),
                        tr!("calendar-why-unreached"),
                        rh,
                        th,
                    ));
                }
                let ids = self.account_calendars(account);
                if !ids.is_empty() {
                    let any_hidden = ids.iter().any(|id| self.calendar_hidden(*id));
                    let (name, label) = if any_hidden {
                        ("eye", tr!("calendar-menu-show-all"))
                    } else {
                        ("remove", tr!("calendar-menu-hide-all"))
                    };
                    rows.item(item("cal-show-all", name, label).on_click(cx.listener(
                        move |this, _, _, cx| {
                            this.close_context_menu(cx);
                            this.show_account_calendars(account, any_hidden, cx);
                        },
                    )));
                }
                if account.is_some() {
                    rows.rule(th);
                    rows.item(
                        item(
                            "cal-account-settings",
                            "settings",
                            tr!("calendar-menu-account-settings"),
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.close_context_menu(cx);
                            this.open_settings_page(
                                super::super::settings_page::Section::Accounts,
                                window,
                                cx,
                            );
                        })),
                    );
                }
            }
            _ => {}
        }
        (rows, parents)
    }

    /// The Color submenu of a calendar: the calendar's color ticked.
    pub(super) fn side_color_rows(&self, id: i64, rh: f32, th: &Theme, cx: &Context<Self>) -> Rows {
        let mut rows = Rows::new(rh);
        let current = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.color.to_ascii_lowercase())
            .unwrap_or_default();
        for (ix, hex) in COLORS.into_iter().enumerate() {
            rows.item(
                menu_row_with(
                    ("cal-calendar-color", ix),
                    dot(parse_color(hex).unwrap_or(th.accent)),
                    color_name(ix).into(),
                    th,
                    rh,
                )
                .when(current == hex, |d| d.child(tick(true, th)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.close_context_menu(cx);
                    this.recolor_calendar(id, hex, cx);
                })),
            );
        }
        rows
    }

    /// The calendars of `account` (`None`: this computer) in the panel.
    fn account_calendars(&self, account: Option<i64>) -> Vec<i64> {
        self.calendar
            .calendars
            .iter()
            .filter(|c| c.account.map(|a| a.0) == account)
            .map(|c| c.id)
            .collect()
    }

    /// Shows calendar `id` and hides every other.
    fn show_only_calendar(&mut self, id: i64, cx: &mut Context<Self>) {
        let ids: Vec<i64> = self.calendar.calendars.iter().map(|c| c.id).collect();
        for other in ids {
            if (other == id) == self.calendar_hidden(other) {
                self.toggle_calendar(other, cx);
            }
        }
        cx.notify();
    }

    /// Shows (`shown`) or hides every calendar of `account`.
    fn show_account_calendars(
        &mut self,
        account: Option<i64>,
        shown: bool,
        cx: &mut Context<Self>,
    ) {
        for id in self.account_calendars(account) {
            if shown == self.calendar_hidden(id) {
                self.toggle_calendar(id, cx);
            }
        }
        cx.notify();
    }

    /// Asks before calendar `id` goes: deleted (`delete`) or off the list.
    fn ask_calendar_gone(&mut self, id: i64, delete: bool, cx: &mut Context<Self>) {
        let Some(calendar) = self.calendar.calendars.iter().find(|c| c.id == id) else {
            return;
        };
        let account = calendar
            .account
            .and_then(|a| self.accounts.iter().find(|x| x.id == a))
            .map(|a| a.address.clone())
            .unwrap_or_default();
        let name = calendar_name(calendar);
        self.ask_calendar_removal(id, name, account, delete, cx);
    }

    /// A calendar went, as the dialog asked.
    pub(in crate::window) fn calendar_removed(
        &mut self,
        name: &str,
        delete: bool,
        cx: &mut Context<Self>,
    ) {
        let text = if delete {
            tr!("calendar-toast-deleted", name = name)
        } else {
            tr!("calendar-toast-removed", name = name)
        };
        self.show_snackbar(text, None, cx);
        self.load_calendar(cx);
    }

    /// Gives calendar `id` color `color`, with Undo.
    fn recolor_calendar(&mut self, id: i64, color: &str, cx: &mut Context<Self>) {
        // Undo brings back the colour it was drawn in, even when it had none.
        let Some(old) = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == id)
            .map(|c| format!("#{:06x}", calendar_color(c) >> 8))
        else {
            return;
        };
        if old.eq_ignore_ascii_case(color) {
            return;
        }
        let undo = Some(CalendarEdit::Recolor(id, old));
        self.send_calendar_edit(
            CalendarEdit::Recolor(id, color.to_owned()),
            tr!("calendar-toast-recolored"),
            undo,
            cx,
        );
    }

    /// Sends `edit`; says `done` with Undo (`undo`) once the service took
    /// it, or why not.
    fn send_calendar_edit(
        &mut self,
        edit: CalendarEdit,
        done: String,
        undo: Option<CalendarEdit>,
        cx: &mut Context<Self>,
    ) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => crate::daemon::connect().await?,
                    };
                    crate::daemon::edit_calendar(&connection, &edit).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(_) => {
                        let undo = undo.map(Command::Calendar);
                        if let Some(undo) = &undo {
                            this.remember(super::super::UndoStep::Command(undo.clone()));
                        }
                        this.show_snackbar(done, undo, cx);
                    }
                    Err(err) => {
                        tracing::info!(%err, "calendar not changed");
                        this.show_snackbar(tr!("calendar-edit-failed", reason = err), None, cx);
                    }
                }
                this.load_calendar(cx);
            })
            .ok();
        })
        .detach();
    }

    /// Opens the name box for `what` in the side panel.
    fn start_calendar_naming(
        &mut self,
        what: NameFor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let accent = rgba(self.theme(window).accent).into();
        let text = match what {
            NameFor::Rename(id) => self
                .calendar
                .calendars
                .iter()
                .find(|c| c.id == id)
                .map(calendar_name)
                .unwrap_or_default(),
            NameFor::New(account) => {
                // Its group opens, so the box shows.
                if self.calendar.folded.remove(&account) {
                    self.calendar.fold(account).turn();
                }
                String::new()
            }
        };
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("calendar-name-placeholder"), cx);
            input.set_accent(accent);
            input.set_text(text, cx);
            input.select_all_text(cx);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.finish_calendar_naming(true, window, cx),
                InputEvent::Cancel => this.finish_calendar_naming(false, window, cx),
                InputEvent::Changed => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.calendar.naming = Some(Naming {
            what,
            input,
            _subscription: subscription,
        });
        cx.notify();
    }

    /// Esc on the page while a calendar is being named puts the box away.
    pub(super) fn cancel_calendar_naming(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.calendar.naming.is_none() {
            return false;
        }
        self.finish_calendar_naming(false, window, cx);
        true
    }

    fn finish_calendar_naming(&mut self, save: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(naming) = self.calendar.naming.take() else {
            return;
        };
        window.focus(&self.calendar.focus, cx);
        cx.notify();
        let name = naming.input.read(cx).text().trim().to_owned();
        if !save || name.is_empty() {
            return;
        }
        match naming.what {
            NameFor::Rename(id) => {
                let old = self
                    .calendar
                    .calendars
                    .iter()
                    .find(|c| c.id == id)
                    .map(calendar_name)
                    .unwrap_or_default();
                if old == name {
                    return;
                }
                let undo = (!old.is_empty()).then_some(CalendarEdit::Rename(id, old));
                self.send_calendar_edit(
                    CalendarEdit::Rename(id, name),
                    tr!("calendar-toast-renamed"),
                    undo,
                    cx,
                );
            }
            NameFor::New(account) => {
                let done = tr!("calendar-toast-added", name = name.as_str());
                // The first colour no calendar here has yet.
                let color = COLORS
                    .iter()
                    .find(|c| {
                        !self
                            .calendar
                            .calendars
                            .iter()
                            .any(|have| have.color.eq_ignore_ascii_case(c))
                    })
                    .unwrap_or(&COLORS[6]);
                self.send_calendar_edit(
                    CalendarEdit::Add(account.unwrap_or(0), name, (*color).to_owned()),
                    done,
                    None,
                    cx,
                );
            }
        }
    }

    /// The name box, in place of a calendar's row or under its account's
    /// heading.
    pub(super) fn render_calendar_naming(
        &self,
        naming: &Naming,
        th: &Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let color = match naming.what {
            NameFor::Rename(id) => self
                .calendar
                .calendars
                .iter()
                .find(|c| c.id == id)
                .map_or(th.accent, calendar_color),
            NameFor::New(_) => th.accent,
        };
        div()
            // A click elsewhere with no name typed puts the box away, as
            // Esc does; a typed name waits for Enter.
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                let empty = this
                    .calendar
                    .naming
                    .as_ref()
                    .is_some_and(|n| n.input.read(cx).text().trim().is_empty());
                if empty {
                    this.finish_calendar_naming(false, window, cx);
                }
            }))
            .h(px(32.0))
            .pl(px(8.0))
            .pr(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded_full()
            .border_1()
            .border_color(rgba(th.accent))
            .text_size(px(14.0))
            .child(super::menu::dot(color))
            .child(div().flex_1().min_w_0().child(naming.input.clone()))
            .into_any_element()
    }
}
