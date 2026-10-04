// SPDX-License-Identifier: GPL-3.0-or-later

//! Help > Keyboard shortcuts (and `?`): every shortcut with its keys in a
//! dialog over the window, to look up and close again, with a box to find
//! one. It only shows them: keys are changed in Settings > Shortcuts.

use gpui::{
    AnyElement, Context, Entity, FocusHandle, Focusable, KeyDownEvent, MouseButton, ScrollHandle,
    Subscription, Window, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::tokens::{space, text};
use katna_ui::{InputEvent, TextInput, px, unpx};

use super::keymap::{self, Group, SHORTCUTS};
use super::select::{SHORTCUTS_SLOT, selectable};
use super::settings::heading;
use super::settings_page::key_cap;
use super::{MailWindow, ShowShortcuts};
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, field, icon, icon_button, tip};

/// Wide enough for the two columns of groups side by side.
const WIDTH: f32 = 760.0;
/// A group's column: below twice this the groups stack.
const COLUMN: f32 = 320.0;
/// What a shortcut does, before its keys.
const LABEL_WIDTH: f32 = 170.0;

pub(super) struct ShortcutsDialog {
    focus: FocusHandle,
    search: Entity<TextInput>,
    _search_changed: Subscription,
    /// The list's scroll, which slides under the header.
    scroll: ScrollHandle,
    closing: bool,
    shown: Spring,
}

impl MailWindow {
    pub(super) fn show_shortcuts(
        &mut self,
        _: &ShowShortcuts,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.shortcuts_dialog_open() {
            self.close_shortcuts_dialog(window, cx);
            return;
        }
        self.settings_open = false;
        self.menu = None;
        let accent = rgba(self.theme(window).accent).into();
        let search = cx.new(|cx| {
            let mut input = TextInput::new(tr!("shortcuts-dialog-search"), cx);
            input.set_accent(accent);
            input
        });
        let changed = cx.subscribe(&search, |this, _, event: &InputEvent, cx| {
            if *event == InputEvent::Changed {
                if let Some(dialog) = &this.shortcuts_dialog {
                    dialog.scroll.set_offset(gpui::point(px(0.0), px(0.0)));
                }
                cx.notify();
            }
        });
        let focus = search.read(cx).focus_handle(cx);
        window.focus(&focus, cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.ui_text.clear();
        self.shortcuts_dialog = Some(ShortcutsDialog {
            focus: cx.focus_handle(),
            search,
            _search_changed: changed,
            scroll: ScrollHandle::new(),
            closing: false,
            shown,
        });
        cx.notify();
    }

    /// The dialog is open and not on its way out.
    pub(super) fn shortcuts_dialog_open(&self) -> bool {
        self.shortcuts_dialog.as_ref().is_some_and(|d| !d.closing)
    }

    pub(super) fn close_shortcuts_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(dialog) = &mut self.shortcuts_dialog
            && !dialog.closing
        {
            dialog.closing = true;
            dialog.shown.set(0.0);
            match &self.settings_page {
                Some(page) => window.focus(&page.focus, cx),
                None => window.focus(&self.list_focus, cx),
            }
        }
        cx.notify();
    }

    fn shortcuts_dialog_key(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Escape reaches `popovers` first; this is for when it does not.
        if event.keystroke.key == "escape" {
            self.close_shortcuts_dialog(window, cx);
            cx.stop_propagation();
        }
    }

    pub(super) fn render_shortcuts_dialog(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let dialog = self.shortcuts_dialog.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.shortcuts_dialog = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let dialog = self.shortcuts_dialog.as_ref()?;
        let phone = self.layout.shape.is_phone();
        let vw = unpx(window.viewport_size().width);
        let width = if phone {
            vw
        } else {
            WIDTH.min(vw - 2.0 * space::S6)
        };
        let query = dialog.search.read(cx).text().trim().to_lowercase();
        let search_focus = dialog.search.read(cx).focus_handle(cx);
        let config = &self.config.shortcuts;
        let mut pieces = self.ui_text.slot_pieces(SHORTCUTS_SLOT, th);

        // The header stays put and the list scrolls under it; a line fades
        // in below it once the list has moved.
        let scrolled = (-unpx(dialog.scroll.offset().y) / 12.0).clamp(0.0, 1.0);
        let header = div()
            .flex_none()
            .border_b_1()
            .border_color(rgba(fade(th.divider, scrolled)))
            .pl(px(space::S6))
            .pr(px(space::S4))
            .pt(px(space::S5))
            .pb(px(space::S5))
            .flex()
            .flex_col()
            .gap(px(space::S4))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(selectable(
                        div().flex_1().min_w_0().child(
                            pieces
                                .words(tr!("shortcuts-dialog-title"))
                                .text_size(px(text::TITLE)),
                        ),
                        Some(pieces.part()),
                        cx,
                    ))
                    .child(
                        icon_button("shortcuts-dialog-close", "close", 20.0, th)
                            .tooltip(tip(tr!("shortcuts-dialog-close"), th))
                            .focus_ring(th)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.close_shortcuts_dialog(window, cx)
                            })),
                    ),
            )
            .child(
                div().pr(px(space::S3)).child(
                    field("shortcuts-dialog-search", &search_focus, th)
                        .h(px(40.0))
                        .flex()
                        .items_center()
                        .gap(px(space::S3))
                        .child(icon("search", th.text_dim, 18.0))
                        .child(div().flex_1().min_w_0().child(dialog.search.clone())),
                ),
            );

        let row_line = th.faint_line(0.35);
        let mut found = 0;
        let groups = Group::ALL.map(|group| {
            let rows: Vec<_> = SHORTCUTS
                .iter()
                .filter(|s| s.group == group)
                .filter(|s| s.app().is_none_or(|app| self.config.app_on(app)))
                .filter_map(|s| {
                    let title = s.title();
                    let keys = keymap::keys(s, config);
                    let labels: Vec<_> = keys.iter().map(|k| keymap::label(k)).collect();
                    let matches = query.is_empty()
                        || title.to_lowercase().contains(&query)
                        || labels.iter().any(|l| l.to_lowercase().contains(&query));
                    if !matches || keys.is_empty() {
                        return None;
                    }
                    let caps = keys.iter().zip(labels).map(|(keys, label)| {
                        let off = !config.single_keys && keymap::is_single_key(keys);
                        key_cap(label, off, th)
                    });
                    Some(
                        div()
                            .min_h(px(40.0))
                            .py(px(space::S2))
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap_x(px(space::S4))
                            .gap_y(px(space::S2))
                            .border_b_1()
                            .border_color(rgba(row_line))
                            .child(
                                pieces
                                    .words(title)
                                    .w(px(LABEL_WIDTH))
                                    .flex_none()
                                    .text_size(px(text::BODY)),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .items_center()
                                    .gap(px(space::S2))
                                    .children(caps),
                            ),
                    )
                })
                .collect();
            found += rows.len();
            (!rows.is_empty()).then(|| {
                div()
                    .flex()
                    .flex_col()
                    .child(heading(group.title(), th))
                    .children(rows)
            })
        });
        let [moving, actions, go_to, app] = groups;
        // Found shortcuts start at the left, not under an empty column.
        let mut left: Vec<_> = [moving, go_to].into_iter().flatten().collect();
        let mut right: Vec<_> = [actions, app].into_iter().flatten().collect();
        if left.is_empty() {
            std::mem::swap(&mut left, &mut right);
        }
        let column = || {
            div()
                .flex_basis(px(COLUMN))
                .flex_grow(1.0)
                .min_w_0()
                .flex()
                .flex_col()
        };
        let list = if found == 0 {
            div()
                .py(px(space::S7))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(space::S4))
                .text_color(rgba(th.text_dim))
                .text_size(px(text::BODY))
                .child(icon("search", th.text_faint, 40.0))
                .child(pieces.words(tr!("shortcuts-dialog-none")))
        } else {
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap_x(px(space::S7))
                .child(column().children(left))
                .child(column().children(right))
        };
        let body = div()
            .id("shortcuts-dialog-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .track_scroll(&dialog.scroll)
            .px(px(space::S6))
            .pb(px(space::S5))
            .child(selectable(list, Some(pieces.part()), cx));

        let card = div()
            .id("shortcuts-dialog")
            .map(|d| self.ui_text_area(d, cx))
            .track_focus(&dialog.focus)
            .map(|d| super::popovers::keep_tab_inside(d, &dialog.focus))
            .on_key_down(cx.listener(Self::shortcuts_dialog_key))
            .occlude()
            .w(px(width))
            .when(phone, |d| d.h_full())
            .when(!phone, |d| {
                // A fixed height, so finding a shortcut does not move it.
                d.h(px(super::about::dialog_max_height(window)))
                    .max_h_full()
                    .min_h_0()
            })
            .flex()
            .flex_col()
            .when(!phone, |d| crate::widgets::dialog(d, th, th.surface))
            .when(phone, |d| crate::widgets::frosted(d, th, th.surface, 0.0))
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(header)
            .child(body);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .when(!phone, |d| d.p(px(space::S6)))
                // No veil: the window stays as it is around the dialog.
                .child(
                    div()
                        .id("shortcuts-dialog-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.close_shortcuts_dialog(window, cx)
                        })),
                )
                .child(
                    div()
                        .max_h_full()
                        .when(phone, |d| d.h_full())
                        .flex()
                        .flex_col()
                        .opacity(t)
                        .mt(px(lerp(space::S6, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }
}
