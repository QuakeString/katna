// SPDX-License-Identifier: GPL-3.0-or-later

//! One rule for the menus and popovers that open over the window: Escape
//! closes the top one, and a press anywhere outside it closes it, the top
//! bar included. The presses come to each popover's scrim, which covers the
//! whole window; Escape is caught before any shortcut, so it works with
//! nothing focused and never also goes back to the list.

use gpui::{
    AnyElement, Context, FocusHandle, MouseButton, MouseDownEvent, Window, deferred, div,
    prelude::*,
};
use katna_ui::px;

use super::{FocusNext, FocusPrevious, MailWindow};

/// Keeps Tab and Shift+Tab inside a dialog whose root tracks `focus`, as
/// in any desktop dialog: they go round its fields and buttons, never to
/// the window behind it.
pub(super) fn keep_tab_inside<E: InteractiveElement>(el: E, focus: &FocusHandle) -> E {
    let step = |focus: FocusHandle, forward: bool| {
        move |window: &mut Window, cx: &mut gpui::App| {
            cx.stop_propagation();
            // Past its last stop, on round to its first.
            for _ in 0..64 {
                if forward {
                    window.focus_next(cx);
                } else {
                    window.focus_prev(cx);
                }
                if focus.contains_focused(window, cx) && !focus.is_focused(window) {
                    return;
                }
            }
            window.focus(&focus, cx);
        }
    };
    let (next, prev) = (step(focus.clone(), true), step(focus.clone(), false));
    el.capture_action(move |_: &FocusNext, window, cx| next(window, cx))
        .capture_action(move |_: &FocusPrevious, window, cx| prev(window, cx))
}

/// Where menu items sit in the Tab order: after everything else, so Tab
/// in the window never stops at them before its own controls.
const MENU_TAB_INDEX: isize = 1 << 20;

/// Makes a menu item reachable with the arrow keys, as in every desktop
/// menu: Up and Down move to it, Enter or Space presses it, and it is
/// tinted while it has the keys.
pub(crate) trait MenuKey: Sized {
    fn menu_key(self, th: &crate::theme::Theme) -> Self;
}

impl MenuKey for gpui::Stateful<gpui::Div> {
    fn menu_key(self, th: &crate::theme::Theme) -> Self {
        let tint = gpui::rgba(th.hover);
        self.tab_index(MENU_TAB_INDEX)
            .focus_visible(move |s| s.bg(tint))
    }
}

impl MailWindow {
    /// Whether a menu is open, whose items the arrow keys go through.
    fn menu_open(&self) -> bool {
        self.context_menu.is_some()
            || self.nav_menu.is_some()
            || self.menu.is_some()
            || self.files_menu.is_some()
            || self.account_menu
            || self.app_menu.is_some()
            || self.snooze_times_open()
            || self.text.menu.is_some()
            || self.compose_popup_open()
    }

    /// Up, Down, Home or End while a menu is open: its items in turn.
    /// Returns whether the key was taken.
    fn menu_arrow(&self, key: &str, window: &mut Window, cx: &mut gpui::App) -> bool {
        if !self.menu_open() {
            return false;
        }
        let inside = |window: &Window| {
            window
                .context_stack()
                .iter()
                .any(|c| c.contains(crate::widgets::MENU_CONTEXT))
        };
        let from = window.focused(cx);
        // Moves one way until an item has the keys (the Tab order runs on
        // past the menu's last item to the window's first control).
        let step = |forward: bool, window: &mut Window, cx: &mut gpui::App| {
            for _ in 0..256 {
                if forward {
                    window.focus_next(cx);
                } else {
                    window.focus_prev(cx);
                }
                if inside(window) {
                    return true;
                }
            }
            false
        };
        // Out of the menu one way, then back to its first item that way.
        let to_end = |forward: bool, window: &mut Window, cx: &mut gpui::App| {
            if !inside(window) && !step(forward, window, cx) {
                return false;
            }
            for _ in 0..256 {
                if forward {
                    window.focus_next(cx);
                } else {
                    window.focus_prev(cx);
                }
                if !inside(window) {
                    break;
                }
            }
            step(!forward, window, cx)
        };
        let found = match key {
            "down" => step(true, window, cx),
            "up" => step(false, window, cx),
            "home" => to_end(false, window, cx),
            "end" => to_end(true, window, cx),
            _ => return false,
        };
        // A menu without items for the keys (a card of fields) keeps them
        // where they were.
        if !found && let Some(from) = from {
            window.focus(&from, cx);
        }
        found
    }

    /// Watches for Escape in this window while a popover is open.
    pub(super) fn watch_escape(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let this = cx.entity().downgrade();
        let handle = window.window_handle();
        let intercept = cx.intercept_keystrokes(move |event, window, cx| {
            let stroke = &event.keystroke;
            if stroke.modifiers.modified() || window.window_handle() != handle {
                return;
            }
            if matches!(stroke.key.as_str(), "up" | "down" | "home" | "end") {
                let taken = this
                    .update(cx, |this, cx| this.menu_arrow(&stroke.key, window, cx))
                    .unwrap_or(false);
                if taken {
                    cx.stop_propagation();
                }
                return;
            }
            if stroke.key != "escape" {
                return;
            }
            let closed = this
                .update(cx, |this, cx| this.dismiss_popover(window, cx))
                .unwrap_or(false);
            if closed {
                cx.stop_propagation();
            }
        });
        self._subscriptions.push(intercept);
    }

    /// Search options, over a scrim that closes them on a press outside.
    pub(super) fn search_panel_layer(
        &self,
        panel: AnyElement,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.search_panel = None;
                cx.notify();
            })
        };
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .child(
                deferred(
                    div()
                        .id("search-panel-scrim")
                        .absolute()
                        .top(px(-2000.0))
                        .left(px(-4000.0))
                        .w(px(8000.0))
                        .h(px(6000.0))
                        .occlude()
                        .on_mouse_down(MouseButton::Left, close())
                        .on_mouse_down(MouseButton::Right, close()),
                )
                .with_priority(1),
            )
            .child(deferred(panel).with_priority(2))
            .into_any_element()
    }

    /// Closes the top popover, if one is open.
    fn dismiss_popover(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        // A shortcut being recorded takes Escape as its own.
        if self
            .settings_page
            .as_ref()
            .is_some_and(|page| page.recording())
        {
            return false;
        }
        let closed = if self.context_menu.is_some() {
            if !self.context_menu_back(cx) {
                self.close_context_menu(cx);
            }
            true
        } else if self.nav_menu.is_some() {
            self.close_nav_menu(cx);
            true
        } else if self.close_delete_ask(cx) || self.close_snooze_menu(cx) || self.close_danger(cx) {
            true
        } else if self.print_preview_open() {
            self.close_print_preview(window, cx);
            true
        } else if self.contact_qr_open() {
            self.close_contact_qr(cx);
            true
        } else if self.share_ask_open() {
            self.close_share_ask(window, cx);
            true
        } else if self.whats_new_open() {
            self.close_whats_new(window, cx);
            true
        } else if self.update_dialog_open() {
            self.close_update_dialog(window, cx);
            true
        } else if self.about_open() {
            self.close_about(window, cx);
            true
        } else if self.dismiss_activity(cx)
            || self.close_seen(cx)
            || self.menu.take().is_some()
            || self.contacts.label_menu.take().is_some()
            || self.files_menu.take().is_some()
            || self.app_menu_back(cx)
            || (std::mem::take(&mut self.account_menu) && {
                self.app_menu = None;
                true
            })
            || self.language_picker.take().is_some()
            || self.dismiss_search_panel(window, cx)
        {
            true
        } else if self.nav_peek {
            // The folders opened over the list from the rail.
            self.nav_peek = false;
            self.peek_task = None;
            true
        } else if self.settings_open && !self.covered() {
            self.settings_open = false;
            true
        } else {
            false
        };
        if closed {
            cx.notify();
        }
        closed
    }

    /// Something over quick settings that takes Escape itself: compose,
    /// a dialog, the tour or an open attachment.
    fn covered(&self) -> bool {
        self.compose.is_some()
            || self.add_account.is_some()
            || self.danger.is_some()
            || self.delete_ask.is_some()
            || self.new_label.is_some()
            || self.contacts.label_dialog.is_some()
            || self.contacts.qr.is_some()
            || self.whats_new.is_some()
            || self.share_ask.is_some()
            || self.print_preview.is_some()
            || self.about.is_some()
            || self.tour.is_some()
            || self.files.viewer.is_some()
    }
}
