// SPDX-License-Identifier: GPL-3.0-or-later

//! One rule for the menus and popovers that open over the window: Escape
//! closes the top one, and a press anywhere outside it closes it, the top
//! bar included. The presses come to each popover's scrim, which covers the
//! whole window; Escape is caught before any shortcut, so it works with
//! nothing focused and never also goes back to the list.

use gpui::{
    AnyElement, Context, MouseButton, MouseDownEvent, Window, deferred, div, prelude::*, px,
};

use super::MailWindow;

impl MailWindow {
    /// Watches for Escape in this window while a popover is open.
    pub(super) fn watch_escape(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let this = cx.entity().downgrade();
        let handle = window.window_handle();
        let intercept = cx.intercept_keystrokes(move |event, window, cx| {
            let stroke = &event.keystroke;
            if stroke.key != "escape"
                || stroke.modifiers.modified()
                || window.window_handle() != handle
            {
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
            self.close_context_menu(cx);
            true
        } else if self.menu.take().is_some()
            || self.files_menu.take().is_some()
            || std::mem::take(&mut self.account_menu)
            || self.dismiss_search_panel(window, cx)
        {
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
            || self.new_label.is_some()
            || self.tour.is_some()
            || self.files.viewer.is_some()
    }
}
