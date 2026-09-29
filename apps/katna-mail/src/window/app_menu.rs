// SPDX-License-Identifier: GPL-3.0-or-later

//! The application menu behind the ☰ button of the account card: the menu
//! bar the KDE global menu shows (`desktop::menu_bar`), for desktops that
//! have no global menu. Each menu opens its items to the left of the card.

use gpui::{AnyElement, Context, FontWeight, MouseButton, div, prelude::*, rgba};
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

    /// The application menu's card, in the account card's place.
    pub(super) fn render_app_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let menu = self.app_menu.as_ref()?;
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
                .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                    if let Some(menu) = this.app_menu.as_mut()
                        && *hovered
                        && menu.open != Some(ix)
                    {
                        menu.open = Some(ix);
                        cx.notify();
                    }
                }))
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(menu) = this.app_menu.as_mut() {
                        menu.open = Some(ix);
                        cx.notify();
                    }
                }))
                .child(without_mnemonic(label))
                .child(
                    div()
                        .ml_auto()
                        .child(icon("chevron-left", th.text_dim, 18.0)),
                )
                .when(open, |d| d.child(self.render_submenu(ix, items, th, cx)));
            Some(row)
        });
        Some(
            raised(
                div()
                    .id("app-menu")
                    .key_context(crate::widgets::MENU_CONTEXT)
                    .occlude()
                    .absolute()
                    .right(px(16.0))
                    .top(px(4.0))
                    .w(px(WIDTH))
                    .p(px(PADDING))
                    .flex()
                    .flex_col()
                    .gap(px(2.0)),
                th,
                super::PANEL_RADIUS,
                2.0,
            )
            .text_color(rgba(th.text))
            .children(rows)
            .into_any_element(),
        )
    }

    /// The items of menu `ix`, beside its row on the left.
    fn render_submenu(
        &self,
        ix: usize,
        items: &[MenuItem],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let rows = items.iter().enumerate().map(|(n, item)| match item {
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
                    .hover(|s| s.bg(rgba(th.hover)))
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
        });
        raised(
            div()
                .id(("app-submenu", ix))
                .occlude()
                .absolute()
                // To the left of the card, the first item level with the row.
                .right(px(WIDTH - PADDING + 4.0))
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
        .children(rows)
        .into_any_element()
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
