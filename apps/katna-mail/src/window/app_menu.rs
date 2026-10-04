// SPDX-License-Identifier: GPL-3.0-or-later

//! The application menu behind the ☰ button of the account card: the menu
//! bar the KDE global menu shows (`desktop::menu_bar`), for desktops that
//! have no global menu. Each menu opens its items to the left of the card,
//! or, where the window leaves no room there (a phone), in the card itself
//! under a row that goes back.

use gpui::{AnyElement, Context, FontWeight, KeyDownEvent, MouseButton, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_platform::dbusmenu::MenuItem;
use katna_ui::px;

use super::MailWindow;
use super::MenuKey;
use super::desktop;
use crate::theme::Theme;
use crate::widgets::{icon, icon_button, raised, tip};

/// As wide as the account card it replaces.
const WIDTH: f32 = 340.0;
/// Room the card keeps from the window's edges.
const MARGIN: f32 = 16.0;
const SUBMENU_WIDTH: f32 = 280.0;
const ROW_HEIGHT: f32 = 40.0;
const PADDING: f32 = 8.0;

/// The open application menu.
pub(super) struct AppMenu {
    items: Vec<MenuItem>,
    /// The menu whose items show.
    open: Option<usize>,
}

impl MailWindow {
    /// The ☰ button that turns the account card into the application menu.
    pub(super) fn app_menu_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        icon_button("app-menu", "menu", 22.0, th)
            .ml_auto()
            .tooltip(tip(tr!("app-menu"), th))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(|this, _, _, cx| {
                cx.stop_propagation();
                this.app_menu = Some(AppMenu {
                    items: desktop::menu_bar(cx),
                    open: None,
                });
                cx.notify();
            }))
            .into_any_element()
    }

    /// The card's width: no wider than the window allows.
    fn app_menu_width(&self) -> f32 {
        WIDTH.min(self.layout.shape.width - 2.0 * MARGIN)
    }

    /// Whether a menu's items open in the card itself: the window leaves no
    /// room for them beside it.
    pub(super) fn app_menu_drills(&self) -> bool {
        let left = self.layout.shape.width - MARGIN - self.app_menu_width();
        left < SUBMENU_WIDTH + 8.0
    }

    /// Opens menu `ix` of the application menu, or goes back from it.
    fn open_app_submenu(&mut self, ix: Option<usize>, cx: &mut Context<Self>) {
        if let Some(menu) = self.app_menu.as_mut()
            && menu.open != ix
        {
            menu.open = ix;
            cx.notify();
        }
    }

    /// Escape in the application menu: back out of a menu opened in the
    /// card. Returns whether it did.
    pub(super) fn app_menu_back(&mut self, cx: &mut Context<Self>) -> bool {
        if !self.app_menu_drills() {
            return false;
        }
        match self.app_menu.as_mut() {
            Some(menu) if menu.open.is_some() => {
                menu.open = None;
                cx.notify();
                true
            }
            _ => false,
        }
    }

    /// The application menu's card, in the account card's place.
    pub(super) fn render_app_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let menu = self.app_menu.as_ref()?;
        let drills = self.app_menu_drills();
        let card = raised(
            div()
                .id("app-menu")
                .key_context(crate::widgets::MENU_CONTEXT)
                .occlude()
                .absolute()
                .right(px(MARGIN))
                .top(px(4.0))
                .w(px(self.app_menu_width()))
                .p(px(PADDING))
                .flex()
                .flex_col()
                .gap(px(2.0))
                // Left goes back from a menu open in the card; Right opens
                // the one whose row has the keys.
                .on_key_down(cx.listener(move |this, event: &KeyDownEvent, _, cx| {
                    if event.keystroke.modifiers.modified() {
                        return;
                    }
                    let back = if this.app_menu_drills() {
                        "left"
                    } else {
                        "right"
                    };
                    if event.keystroke.key == back
                        && this.app_menu.as_ref().is_some_and(|m| m.open.is_some())
                    {
                        this.open_app_submenu(None, cx);
                        cx.stop_propagation();
                    }
                })),
            th,
            super::PANEL_RADIUS,
            2.0,
        )
        .text_color(rgba(th.text));
        // A phone: the open menu's items in the card, under a row back.
        if drills
            && let Some(ix) = menu.open
            && let Some(MenuItem::Submenu { label, items }) = menu.items.get(ix)
        {
            let back = div()
                .id("app-menu-back")
                .h(px(ROW_HEIGHT))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .rounded(px(8.0))
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .relative()
                .child(crate::widgets::hover_fade("hover-glow", Some(8.0), th))
                .menu_key(th)
                .tooltip(tip(tr!("app-menu-back"), th))
                .on_click(cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.open_app_submenu(None, cx);
                }))
                .child(icon("back", th.text_dim, 20.0))
                .child(without_mnemonic(label));
            let rule = div()
                .mx(px(8.0))
                .my(px(4.0))
                .h(px(1.0))
                .bg(rgba(th.divider));
            return Some(
                card.child(back)
                    .child(rule)
                    .children(self.submenu_rows(ix, items, th, cx))
                    .into_any_element(),
            );
        }
        let rows = menu.items.iter().enumerate().filter_map(|(ix, item)| {
            let MenuItem::Submenu { label, items } = item else {
                return None;
            };
            let open = menu.open == Some(ix);
            let row = div()
                .id(("app-menu-row", ix))
                .relative()
                .h(px(ROW_HEIGHT))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .rounded(px(8.0))
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .when(open, |d| d.bg(rgba(th.hover)))
                .hover(|s| s.bg(rgba(th.hover)))
                .menu_key(th)
                // Beside the card a menu opens as the pointer comes to its
                // row; in the card only on a click or tap.
                .when(!drills, |d| {
                    d.on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                        if *hovered {
                            this.open_app_submenu(Some(ix), cx);
                        }
                    }))
                })
                .on_key_down(cx.listener(move |this, event: &KeyDownEvent, _, cx| {
                    let open = if this.app_menu_drills() {
                        "right"
                    } else {
                        "left"
                    };
                    if !event.keystroke.modifiers.modified() && event.keystroke.key == open {
                        this.open_app_submenu(Some(ix), cx);
                        cx.stop_propagation();
                    }
                }))
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.open_app_submenu(Some(ix), cx);
                }))
                .child(without_mnemonic(label))
                .child(div().ml_auto().child(icon(
                    if drills {
                        "chevron-right"
                    } else {
                        "chevron-left"
                    },
                    th.text_dim,
                    18.0,
                )))
                .when(open && !drills, |d| {
                    d.child(self.render_submenu(ix, items, th, cx))
                });
            Some(row)
        });
        Some(card.children(rows).into_any_element())
    }

    /// The items of menu `ix`, beside its row on the left.
    fn render_submenu(
        &self,
        ix: usize,
        items: &[MenuItem],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let width = self.app_menu_width();
        raised(
            div()
                .id(("app-submenu", ix))
                .occlude()
                .absolute()
                // To the left of the card, the first item level with the row.
                .right(px(width - PADDING + 4.0))
                .top(px(-PADDING))
                .w(px(SUBMENU_WIDTH))
                .p(px(PADDING))
                .flex()
                .flex_col()
                .gap(px(2.0))
                .font_weight(FontWeight::NORMAL),
            th,
            super::PANEL_RADIUS,
            2.0,
        )
        .children(self.submenu_rows(ix, items, th, cx))
        .into_any_element()
    }

    /// The rows of menu `ix`'s items.
    fn submenu_rows(
        &self,
        ix: usize,
        items: &[MenuItem],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        items
            .iter()
            .enumerate()
            .map(|(n, item)| match item {
                MenuItem::Action {
                    label,
                    action,
                    shortcut,
                    ..
                } => {
                    let action = action.clone();
                    div()
                        .id(("app-menu-item", ix * 100 + n))
                        .h(px(36.0))
                        .px(px(16.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(16.0))
                        .rounded(px(8.0))
                        .text_size(px(14.0))
                        .cursor_pointer()
                        .relative()
                        .child(crate::widgets::hover_fade("hover-glow", Some(8.0), th))
                        .menu_key(th)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.app_menu = None;
                            this.account_menu = false;
                            this.run_action(&action, window, cx);
                            cx.notify();
                        }))
                        .child(without_mnemonic(label))
                        .child(
                            div()
                                .ml_auto()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(shortcut_text(shortcut)),
                        )
                        .into_any_element()
                }
                MenuItem::Separator => div()
                    .mx(px(16.0))
                    .my(px(4.0))
                    .h(px(1.0))
                    .bg(rgba(th.divider))
                    .into_any_element(),
                MenuItem::Submenu { .. } => div().into_any_element(),
            })
            .collect()
    }
}

/// `label` without the `_` that marks its mnemonic for the global menu.
fn without_mnemonic(label: &str) -> String {
    label.replacen('_', "", 1)
}

/// Keys as the global menu's shortcuts name them, such as
/// `["Control", "N"]`, written as `Ctrl+N`.
fn shortcut_text(keys: &[String]) -> String {
    keys.iter()
        .map(|key| match key.as_str() {
            "Control" => "Ctrl",
            key => key,
        })
        .collect::<Vec<_>>()
        .join("+")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_and_shortcuts_read_as_text() {
        assert_eq!(without_mnemonic("_New Message"), "New Message");
        assert_eq!(without_mnemonic("Select _All"), "Select All");
        assert_eq!(
            shortcut_text(&["Control".into(), "Shift".into(), "N".into()]),
            "Ctrl+Shift+N"
        );
        assert_eq!(shortcut_text(&[]), "");
    }
}
