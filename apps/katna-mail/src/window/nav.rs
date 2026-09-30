// SPDX-License-Identifier: GPL-3.0-or-later

//! The top bar (menu, the app's name, search box, settings, account), the
//! navigation with the folders, which folds away, and Compose, which sits
//! over the folders and moves into the app rail when they fold.

use std::f32::consts::FRAC_PI_2;

use gpui::{
    AnimationExt, AnyElement, Context, ElementId, FontWeight, ListAlignment, ListState,
    MouseButton, MouseDownEvent, PathBuilder, SharedString, SpringAnimation, Transformation,
    canvas, div, list, point, prelude::*, radians, rgba, svg,
};
use katna_ui::Ripple;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;

use super::apps::APP_RAIL_WIDTH;
use super::tour::Spot;
use super::{
    Compose, FocusSearch, Hover, Listing, MailWindow, NAV_ROW_INSET, NAV_WIDTH, PANEL_RADIUS,
    SEARCH_CONTEXT, ToggleNavigation, ToggleSettings, compose,
};
use katna_core::AccountKind;
use katna_i18n::tr;

use crate::format;
use crate::sidebar::{self, Role, Unified};
use crate::theme::{Theme, fade, mix};
use crate::widgets::{
    ScaledEdge, elevation, icon, icon_button, icon_button_colored, katna_mark, keys_ring, tip,
};

/// How far the floating folder pane stands off the rail and the top bar.
const FLOAT_GAP: f32 = 8.0;
/// How far its notch reaches toward the rail, and half its height.
const NOTCH: f32 = 8.0;

const NAV_ROW_HEIGHT: f32 = 32.0;
/// The gap between a folder's pill and the pane's far edge.
const NAV_ROW_END: f32 = 16.0;
/// The gap around a folder's arrow, inside its pill's rounded end.
const CHEVRON_GAP: f32 = (NAV_ROW_HEIGHT - 20.0) / 2.0;
/// Where folder icons and headings start, from the pane's edge: after the
/// inset, the arrow and a gap.
const NAV_TEXT_LEFT: f32 = NAV_ROW_INSET + CHEVRON_GAP + 20.0 + 4.0;
const SEARCH_HEIGHT: f32 = 40.0;
/// How opaque the idle search box is in a blurred window: frosted glass
/// that shows the blur behind it. Focused, it is solid.
const SEARCH_GLASS_ALPHA: f32 = 0.4;
/// How strong the idle glass search box's faint edge is.
const SEARCH_GLASS_EDGE: f32 = 0.22;

/// The button at the top of a page's side panel (Create contact, Create
/// task), in the size, shape and colours of Mail's Compose over the
/// folders: only its icon and word change. Wrap it in a flex `div` so a
/// column does not stretch it.
pub(super) fn side_create_button(
    id: &'static str,
    icon_name: &str,
    label: String,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .flex_none()
        .ml(px(NAV_ROW_INSET))
        .mt(px(super::COMPOSE_TOP))
        .mb(px(super::COMPOSE_NAV_ROOM
            - super::COMPOSE_TOP
            - super::COMPOSE_HEIGHT))
        .h(px(super::COMPOSE_HEIGHT))
        .pl(px(16.0))
        .pr(px(24.0))
        .flex()
        .flex_row()
        .items_center()
        .overflow_hidden()
        .rounded(px(super::COMPOSE_RADIUS))
        .bg(rgba(th.compose))
        .text_color(rgba(th.compose_text))
        .hover(|s| s.shadow(elevation(th, 1.5)))
        .cursor_pointer()
        .child(Ripple::new(id, rgba(th.ripple)).rounded(super::COMPOSE_RADIUS))
        .child(icon(icon_name, th.compose_text, 24.0))
        .child(
            div()
                .flex_none()
                .pl(px(12.0))
                .text_size(px(super::COMPOSE_TEXT_SIZE))
                .font_weight(FontWeight::MEDIUM)
                .whitespace_nowrap()
                .child(label),
        )
}

/// A line of a page's side list (Calendar, Contacts, Tasks, Notes) in the
/// shape of Mail's folders: a full pill inset from both edges of the pane,
/// with its icon and label where a folder's are. Add a count or other end
/// pieces as children.
pub(super) fn side_row(
    id: impl Into<ElementId>,
    icon_name: &str,
    label: impl IntoElement,
    on: bool,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    let id = id.into();
    let text = if on { th.nav_selected_text } else { th.text };
    div()
        .id(id.clone())
        .relative()
        .flex_none()
        .h(px(NAV_ROW_HEIGHT))
        .ml(px(NAV_ROW_INSET))
        .mr(px(NAV_ROW_END))
        .pl(px(NAV_TEXT_LEFT - NAV_ROW_INSET))
        .pr(px(12.0))
        .flex()
        .flex_row()
        .items_center()
        .rounded_full()
        .cursor_pointer()
        .text_size(px(14.0))
        .text_color(rgba(text))
        .when(on, |d| {
            d.bg(rgba(th.nav_selected)).font_weight(FontWeight::BOLD)
        })
        .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
        .child(Ripple::new(id, rgba(th.ripple)).rounded(NAV_ROW_HEIGHT / 2.0))
        .child(icon(icon_name, if on { text } else { th.text_dim }, 20.0))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .pl(px(18.0))
                .truncate()
                .child(label),
        )
}
/// The line the app's name rolls through on the top bar.
const TITLE_LINE: f32 = 28.0;

impl MailWindow {
    pub(super) fn render_top_start(
        &self,
        th: &Theme,
        titles: (f32, f32),
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        // The mark and the app's name sit beside the menu button on a
        // desktop and a tablet; a phone has its search pill there.
        let shown = 1.0 - self.layout.shape.phone;
        // How far the folders show: docked beside the list on a desktop,
        // or a phone's or tablet's drawer.
        // The other pages' side column, folded on its own, on a desktop.
        let page = self.app != super::RailApp::Mail;
        let docked = if page && self.layout.shape.is_desktop() {
            self.page_side_t
        } else {
            self.reserve_spring.value()
        };
        let open = docked.max(self.layout.drawer_t()).clamp(0.0, 1.0);
        let menu = div()
            .id("menu-button")
            .relative()
            .ml(px(6.0))
            .size(px(48.0))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .tooltip(tip(
                match (page, open > 0.5) {
                    (false, true) => tr!("folders-hide"),
                    (false, false) => tr!("folders-show"),
                    (true, true) => tr!("side-pane-hide"),
                    (true, false) => tr!("side-pane-show"),
                },
                th,
            ))
            .on_click(cx.listener(|this, _, window, cx| {
                this.toggle_navigation(&ToggleNavigation, window, cx)
            }))
            .child(Ripple::new("menu-ripple", rgba(th.ripple)).centered())
            .child(self.tour_mark(Spot::Menu))
            // A panel whose left part fills while the folders show, fading
            // with the pane as it opens or folds.
            .child(
                div()
                    .relative()
                    .size(px(24.0))
                    .child(
                        svg()
                            .path("icons/folders-pane.svg")
                            .size_full()
                            .text_color(rgba(th.text_dim)),
                    )
                    .child(
                        svg()
                            .path("icons/folders-pane-fill.svg")
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .text_color(rgba(fade(th.text_dim, open))),
                    ),
            )
            .into_any_element();
        let mut start = vec![menu];
        if shown > 0.001 {
            let label = self.layout.shape.title_label();
            start.push(
                div()
                    .flex_none()
                    .ml(px(super::TOP_BAR_GAP - super::BAR_ITEM_GAP))
                    .w(px(super::title_width(label, titles) * shown))
                    .overflow_hidden()
                    .opacity(shown)
                    .child(self.render_title(th, label, titles))
                    .into_any_element(),
            );
        }
        start
    }

    /// The Katna mark, "Katna" and the app's name. Switching apps rolls
    /// the name: the old one rolls down and out, the new one down into
    /// its place. A narrow tablet folds the words away, so the search box
    /// keeps its room.
    pub(super) fn render_title(
        &self,
        th: &Theme,
        label: f32,
        (brand, name): (f32, f32),
    ) -> AnyElement {
        let roll = self.title_roll.value();
        let word = |app: super::RailApp, top: f32, opacity: f32| {
            div()
                .absolute()
                .left_0()
                .top(px(top))
                .h(px(TITLE_LINE))
                .whitespace_nowrap()
                .opacity(opacity.clamp(0.0, 1.0))
                .child(app.label())
        };
        let rolling = roll < 0.999 && self.title_from != self.app;
        div()
            .h(px(super::TOP_BAR_HEIGHT))
            .flex()
            .flex_row()
            .items_center()
            .child(katna_mark(super::TITLE_MARK))
            .child(
                div()
                    .flex_none()
                    .pl(px(super::TITLE_MARK_GAP * label))
                    .w(px((super::TITLE_MARK_GAP
                        + brand
                        + super::TITLE_WORD_GAP
                        + name)
                        * label))
                    .overflow_hidden()
                    .opacity(label)
                    .flex()
                    .flex_row()
                    .items_center()
                    .text_size(px(super::TITLE_TEXT_SIZE))
                    .line_height(px(TITLE_LINE))
                    .text_color(rgba(th.text_dim))
                    .child(
                        div()
                            .flex_none()
                            .whitespace_nowrap()
                            .child(tr!("top-brand")),
                    )
                    .child(
                        div()
                            .relative()
                            .flex_none()
                            .ml(px(super::TITLE_WORD_GAP))
                            .w(px(name))
                            .h(px(TITLE_LINE))
                            .overflow_hidden()
                            .when(rolling, |d| {
                                d.child(word(self.title_from, TITLE_LINE * roll, 1.0 - roll))
                            })
                            .child(word(
                                self.app,
                                -TITLE_LINE * (1.0 - roll),
                                if rolling { roll } else { 1.0 },
                            )),
                    ),
            )
            .into_any_element()
    }

    /// Compose: a pill at the top of the folders while they are open
    /// beside the list, a square at the top of the app rail while they are
    /// folded, in a tablet's drawer, or on another app's page. It slides
    /// between the two as the folders open or fold, and the rail's apps
    /// move down to make room.
    pub(super) fn render_compose_button(
        &self,
        th: &Theme,
        text: f32,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let shape = self.layout.shape;
        let shown = self.compose_shown.value().clamp(0.0, 1.0) * (1.0 - shape.phone);
        if shown <= 0.001 {
            return None;
        }
        // 0 = in the rail, 1 = over the folders.
        let dock = self.compose_dock.value().clamp(0.0, 1.0);
        let left = lerp(
            super::COMPOSE_RAIL_LEFT,
            APP_RAIL_WIDTH + NAV_ROW_INSET,
            dock,
        ) - APP_RAIL_WIDTH * shape.phone;
        let top = super::COMPOSE_TOP;
        Some(
            div()
                .id("compose")
                .absolute()
                .left(px(left))
                .top(px(top))
                .h(px(super::COMPOSE_HEIGHT))
                .w(px(super::compose_width(dock, text)))
                .opacity(shown)
                .flex()
                .flex_row()
                .items_center()
                .overflow_hidden()
                .rounded(px(super::COMPOSE_RADIUS))
                .bg(rgba(th.compose))
                .text_color(rgba(th.compose_text))
                .hover(|s| s.shadow(elevation(th, 1.5)))
                .cursor_pointer()
                // In the rail, resting on it opens the folded folders over
                // the list, as resting on Mail does.
                .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                    this.hover_navigation(Hover::Compose, *hovered, cx)
                }))
                .on_click(cx.listener(|this, _, window, cx| this.compose(&Compose, window, cx)))
                .child(
                    Ripple::new("compose-ripple", rgba(th.ripple)).rounded(super::COMPOSE_RADIUS),
                )
                .child(self.tour_mark(Spot::Compose))
                .child(
                    div()
                        .flex_none()
                        .pl(px(16.0))
                        .child(icon("compose", th.compose_text, 24.0)),
                )
                .child(
                    div()
                        .flex_none()
                        .pl(px(12.0))
                        .opacity(dock * dock)
                        .text_size(px(super::COMPOSE_TEXT_SIZE))
                        .font_weight(FontWeight::MEDIUM)
                        .whitespace_nowrap()
                        .child(tr!("compose")),
                )
                .into_any_element(),
        )
    }

    /// The height of the account's name over the folders, when there is
    /// one: it stays put while the folders scroll.
    fn nav_header(&self) -> f32 {
        if matches!(
            self.nav_rows.first(),
            Some(sidebar::Row::Account { .. } | sidebar::Row::AllAccounts { .. })
        ) {
            NAV_ROW_HEIGHT
        } else {
            0.0
        }
    }

    /// The focus-search shortcut as a keycap after the empty box's
    /// placeholder, from the keys the shortcut settings give it now.
    fn render_search_hint(&self, th: &Theme, window: &gpui::Window) -> Option<AnyElement> {
        let keys = super::keymap::hint("search", &self.config.shortcuts)?;
        let words = if self.app == super::RailApp::Contacts {
            tr!("contacts-search")
        } else if self.app == super::RailApp::Calendar {
            tr!("calendar-search")
        } else {
            tr!("search-mail")
        };
        let placeholder =
            super::text_width(&words, 16.0, FontWeight::NORMAL, self.font.as_ref(), window);
        Some(
            div()
                .absolute()
                .top_0()
                .bottom_0()
                .left(px(placeholder + 12.0))
                .flex()
                .items_center()
                .child(
                    div()
                        .h(px(22.0))
                        .px(px(6.0))
                        .flex()
                        .items_center()
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(rgba(th.divider))
                        .text_size(px(12.0))
                        .line_height(px(16.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.text_faint))
                        .whitespace_nowrap()
                        .child(keys),
                )
                .into_any_element(),
        )
    }

    pub(super) fn render_search(
        &self,
        th: &Theme,
        width: f32,
        t: f32,
        window: &gpui::Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let available = self.mail.as_ref().is_ok_and(crate::data::Mail::has_index);
        let has_text = !self.search.read(cx).text().is_empty();
        let panel_open = self.search_panel.is_some();
        // While the Settings page is open the box searches settings.
        let settings = self.settings_page.is_some();
        // On the Contacts page it finds people and on the Calendar page
        // events, with no mail options.
        let contacts = matches!(
            self.app,
            super::RailApp::Contacts | super::RailApp::Calendar
        );
        // On a phone the box is a pill across the bar, with the menu button
        // and the account picture over its two ends.
        let phone = self.layout.shape.phone;
        div()
            .id("search-box")
            .key_context(SEARCH_CONTEXT)
            .relative()
            .w(px(width))
            .h(px(lerp(SEARCH_HEIGHT, 48.0, phone)))
            .pl(px(lerp(0.0, 56.0, phone)))
            .pr(px(lerp(0.0, 50.0, phone)))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .rounded_full()
            // In a blurred window it is frosted glass while idle, letting
            // more of the blur through than the bar around it, and turns
            // solid as it takes focus.
            .bg(rgba(if th.backdrop == 0 {
                fade(th.search, lerp(SEARCH_GLASS_ALPHA, 1.0, t.clamp(0.0, 1.0)))
            } else {
                th.search
            }))
            // Focused, it gains the accent edge every other field has. As
            // glass it keeps a faint edge while idle, so it stays visible.
            .border_px(2.0)
            .border_color(rgba(if th.backdrop == 0 {
                mix(fade(th.text, SEARCH_GLASS_EDGE), th.accent, t)
            } else {
                fade(th.accent, t.clamp(0.0, 1.0))
            }))
            .text_size(px(16.0))
            .line_height(px(24.0))
            .text_color(rgba(th.text))
            .when(!available && !settings && !contacts, |d| d.opacity(0.6))
            // A drag here selects text rather than moving the window.
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .child(self.tour_mark(Spot::Search))
            .when(phone < 0.999, |d| {
                d.child(
                    div()
                        .flex_none()
                        .w(px(40.0 * (1.0 - phone)))
                        .overflow_hidden()
                        .opacity(1.0 - phone)
                        .child(
                            icon_button("search-button", "search", 22.0, th)
                                .tooltip(tip(tr!("search"), th))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    let text = this.search.read(cx).text().trim().to_owned();
                                    if text.is_empty()
                                        || this.settings_page.is_some()
                                        || this.app == super::RailApp::Contacts
                                        || this.app == super::RailApp::Calendar
                                    {
                                        this.focus_search(&FocusSearch, window, cx);
                                    } else {
                                        this.start_search(text, cx);
                                    }
                                })),
                        ),
                )
            })
            .child(
                div()
                    .relative()
                    .flex_1()
                    .min_w_0()
                    // A narrow box cuts the keycap off rather than overlap.
                    .overflow_hidden()
                    .child(self.search.clone())
                    // The shortcut hint goes while the box has the keys, and
                    // comes back when they leave an empty box.
                    .children(
                        (!has_text && !settings && t < 0.999)
                            .then(|| self.render_search_hint(th, window))
                            .flatten()
                            .map(|hint| {
                                div()
                                    .absolute()
                                    .size_full()
                                    .top_0()
                                    .left_0()
                                    .opacity(1.0 - t.clamp(0.0, 1.0))
                                    .child(hint)
                            }),
                    ),
            )
            .when(has_text, |d| {
                d.child(
                    icon_button("search-clear", "close", 22.0, th)
                        .tooltip(tip(tr!("search-clear"), th))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.clear_search(cx);
                            this.focus_search(&FocusSearch, window, cx);
                        })),
                )
            })
            .when(!settings && !contacts, |d| {
                d.child(
                    icon_button_colored(
                        "search-options",
                        "tune",
                        22.0,
                        if panel_open { th.accent } else { th.text_dim },
                        th,
                    )
                    .tooltip(tip(tr!("search-options-show"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_search_panel(window, cx);
                    })),
                )
            })
            .into_any_element()
    }

    pub(super) fn render_top_end(&self, th: &Theme, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let settings = icon_button_colored(
            "settings-button",
            "settings",
            24.0,
            if self.settings_open {
                th.accent
            } else {
                th.text_dim
            },
            th,
        )
        .tooltip(tip(tr!("settings"), th))
        .on_click(
            cx.listener(|this, _, window, cx| this.toggle_settings(&ToggleSettings, window, cx)),
        )
        .child(self.tour_mark(Spot::Settings));
        // The account picture opens the account card; with no account yet,
        // the button adds one.
        // The mouse wheel over it switches accounts.
        let account = match self.pictured_account() {
            Some(account) => {
                let name = if account.display_name.trim().is_empty() {
                    account.address.clone()
                } else {
                    account.display_name.clone()
                };
                div()
                    .id("top-account")
                    .relative()
                    .p(px(4.0))
                    .rounded_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .when(self.account_menu, |d| d.bg(rgba(th.hover)))
                    .on_mouse_move(|_, _, cx| cx.stop_propagation())
                    .tooltip(tip(
                        if self.accounts.len() > 1 {
                            format!("{name}\n{}\n{}", account.address, tr!("account-wheel-hint"))
                        } else {
                            format!("{name}\n{}", account.address)
                        },
                        th,
                    ))
                    .on_scroll_wheel(cx.listener(|this, event, _, cx| {
                        cx.stop_propagation();
                        this.wheel_accounts(event, cx);
                    }))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.account_menu = !this.account_menu;
                        this.app_menu = None;
                        cx.notify();
                    }))
                    .child(self.render_rolling_avatar(32.0))
                    .child(self.tour_mark(Spot::Account))
                    .into_any_element()
            }
            None => icon_button_colored("top-account", "person-add", 22.0, th.text_dim, th)
                .tooltip(tip(tr!("account-add"), th))
                .on_click(cx.listener(|this, _, window, cx| this.open_add_account(window, cx)))
                .into_any_element(),
        };
        // A phone has Settings in its drawer.
        let phone = self.layout.shape.phone;
        let mut end = Vec::new();
        // The day's agenda, on the Mail page of a desktop window.
        if self.agenda_button_shown() {
            end.push(
                div()
                    .flex_none()
                    .mr(px(super::TOP_BAR_GAP - super::BAR_ITEM_GAP))
                    .child(self.render_agenda_button(th, cx))
                    .into_any_element(),
            );
        }
        if phone < 0.999 {
            end.push(
                div()
                    .flex_none()
                    .w(px(super::LANGUAGE_BUTTON_WIDTH * (1.0 - phone)))
                    .mr(px(
                        (super::TOP_BAR_GAP - super::BAR_ITEM_GAP) * (1.0 - phone)
                    ))
                    .overflow_hidden()
                    .opacity(1.0 - phone)
                    .child(self.render_language_button(th, cx))
                    .into_any_element(),
            );
            end.push(
                div()
                    .flex_none()
                    .w(px(40.0 * (1.0 - phone)))
                    .overflow_hidden()
                    .opacity(1.0 - phone)
                    .child(settings)
                    .into_any_element(),
            );
        }
        end.push(
            div()
                .ml(px(super::TOP_BAR_GAP - super::BAR_ITEM_GAP))
                .mr(px(8.0))
                .child(account)
                .into_any_element(),
        );
        end
    }

    /// The folders. Folded, the panel is gone; it opens over the list while
    /// the pointer rests on Mail in the app rail or on the panel itself.
    pub(super) fn render_navigation(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let t = self.nav_t.max(0.0);
        let reserve = self.reserve_spring.value().max(0.0);
        // How far the panel is open beyond the space it takes: it floats.
        // The two springs can differ by a hair once both are open, so a
        // trace of float is no float at all.
        let float = match (t - reserve).clamp(0.0, 1.0) {
            f if f < 0.01 => 0.0,
            f => f,
        };
        // On a phone or tablet it is a drawer, full height with square
        // corners.
        let shape = self.layout.shape;
        let drawer = !shape.is_desktop();
        // A drawer (opened with the menu on a phone or tablet) slides in
        // whole; the desktop's panel unfolds.
        let slides = !shape.is_desktop() && !self.nav_peek;
        let width = if slides {
            self.drawer_width()
        } else {
            NAV_WIDTH
        };
        // Compose heads the folders while they are open beside the list,
        // with the account's name under it; the folders scroll below.
        let skip = usize::from(self.nav_header() > 0.0);
        let head = (skip > 0).then(|| self.render_nav_row(0, th, cx));
        let compose_room =
            super::COMPOSE_NAV_ROOM * reserve.min(1.0) * self.compose_shown.value().clamp(0.0, 1.0);
        let list = list(
            self.nav_list.clone(),
            cx.processor(move |this, ix: usize, window, cx| {
                let th = this.theme(window);
                this.render_nav_item(ix, &th, cx)
            }),
        )
        .w(px(NAV_WIDTH))
        .flex_1()
        .min_h_0()
        .pb(px(16.0));
        // Floating, it stands clear of the rail and the top bar.
        let gap = if drawer { 0.0 } else { FLOAT_GAP * float };
        let panel = div()
            .id("navigation-panel")
            .map(|d| self.nav_keys(d, cx))
            .occlude()
            .absolute()
            .top(px(gap))
            .left(px(gap))
            .bottom(px(if drawer { 0.0 } else { 16.0 * float + gap }))
            .map(|d| {
                if slides {
                    d.left(px(-width * (1.0 - t))).w(px(width))
                } else {
                    d.w(px(width * t)).opacity(t.min(1.0))
                }
            })
            .flex()
            .flex_col()
            .pt(px(lerp(0.0, 12.0, float)))
            .overflow_hidden()
            // Floating, it keeps the page's color, lifted by its shadow; the
            // panel never turns into a card, so folding it cannot flash.
            .when(float > 0.0, |d| {
                d.bg(rgba(th.page))
                    .when(!drawer, |d| {
                        d.rounded(px(PANEL_RADIUS))
                            .border_1()
                            .border_color(rgba(fade(th.divider, float)))
                    })
                    .shadow(elevation(th, 3.0 * float))
            })
            .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                this.hover_navigation(Hover::Panel, *hovered, cx)
            }))
            .children(self.render_drawer_head(th))
            .child(div().flex_none().h(px(compose_room)))
            .children(head)
            .child(list)
            .children(self.render_storage(th))
            .children(self.render_drawer_foot(th, cx));
        let scrim_width = shape.width - shape.rail();
        div()
            .relative()
            .flex_none()
            .h_full()
            .w(px(NAV_WIDTH * reserve))
            .children(self.render_scrim(scrim_width, cx))
            // Clips the drawer as it slides out from the rail's edge.
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .bottom_0()
                    // Room for the panel's shadow.
                    .w(px(width + 24.0))
                    .overflow_hidden()
                    .child(panel)
                    .children((!drawer && float > 0.0).then(|| self.render_notch(gap, float, th))),
            )
            .into_any_element()
    }

    /// The notch on the floating panel's edge, pointing at what in the
    /// rail opened it.
    fn render_notch(&self, gap: f32, float: f32, th: &Theme) -> AnyElement {
        let middle = match self.peek_from {
            Hover::Compose => super::COMPOSE_TOP + super::COMPOSE_HEIGHT / 2.0,
            _ => self.rail_mail_middle(),
        };
        // It reaches over the panel's border there, so the two read as one.
        let (tip, base) = (gap - NOTCH, gap + 1.0);
        let fill = fade(th.page, float);
        let line = fade(th.divider, float);
        canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let o = bounds.origin;
                let p = |x: f32, y: f32| point(o.x + px(x), o.y + px(y));
                let (top, bottom) = (p(base, middle - NOTCH), p(base, middle + NOTCH));
                let tip = p(tip, middle);
                let mut path = PathBuilder::fill();
                path.add_polygon(&[top, tip, bottom], true);
                if let Ok(path) = path.build() {
                    window.paint_path(path, rgba(fill));
                }
                let mut edge = PathBuilder::stroke(px(1.0));
                edge.move_to(top);
                edge.line_to(tip);
                edge.line_to(bottom);
                if let Ok(path) = edge.build() {
                    window.paint_path(path, rgba(line));
                }
            },
        )
        .absolute()
        .size_full()
        .into_any_element()
    }

    fn render_nav_row(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        match &self.nav_rows[ix] {
            sidebar::Row::AllAccounts { expanded } => self.render_heading(
                ix,
                tr!("nav-all-accounts"),
                (*expanded, self.checking_all()),
                th,
                cx,
            ),
            sidebar::Row::Account {
                id, name, expanded, ..
            } => self.render_heading(
                ix,
                name.clone(),
                (*expanded, self.checking_account(*id)),
                th,
                cx,
            ),
            sidebar::Row::Labels { account } => {
                let account = *account;
                let gmail = self.tree.is_gmail(account);
                // Folders (Gmail's labels) are made on the server.
                let imap = self
                    .accounts
                    .iter()
                    .any(|a| a.id == account && a.kind == AccountKind::Imap);
                div()
                    .id(("nav-row", ix))
                    .h(px(NAV_ROW_HEIGHT))
                    .w(px(NAV_WIDTH - 16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .pl(px(NAV_TEXT_LEFT))
                    .text_size(px(15.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(div().flex_1().min_w_0().truncate().child(if gmail {
                        tr!("nav-labels")
                    } else {
                        tr!("nav-folders")
                    }))
                    .when(imap, |d| {
                        d.child(
                            icon_button(("nav-new-label", ix), "add", 20.0, th)
                                .size(px(32.0))
                                .tooltip(tip(
                                    if gmail {
                                        tr!("nav-label-new")
                                    } else {
                                        tr!("nav-folder-new")
                                    },
                                    th,
                                ))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    cx.stop_propagation();
                                    this.open_new_label(account, window, cx);
                                })),
                        )
                    })
                    .into_any_element()
            }
            sidebar::Row::Unified {
                view,
                unread,
                expanded,
            } => {
                let listing = Some(Listing::Unified {
                    view: *view,
                    account: None,
                });
                self.render_pill(
                    ix,
                    Pill {
                        key: view.key().into(),
                        depth: 0,
                        icon: unified_icon(*view),
                        label: view.title(),
                        unread: *unread,
                        selected: self.listing == listing,
                        bold: true,
                        chevron: Some(*expanded),
                        checking: (*view == Unified::Inbox && self.checking_all())
                            || self
                                .tree
                                .unified_folders(*view, None)
                                .into_iter()
                                .any(|f| self.checking_folder(f)),
                    },
                    th,
                    cx,
                )
            }
            sidebar::Row::UnifiedAccount {
                view,
                account,
                name,
                folder,
                unread,
            } => {
                let selected = match folder {
                    Some(f) => self.listing == Some(Listing::Folder(*f)),
                    None => {
                        self.listing
                            == Some(Listing::Unified {
                                view: *view,
                                account: Some(*account),
                            })
                    }
                };
                self.render_pill(
                    ix,
                    Pill {
                        key: format!("{}:{}", view.key(), account.0).into(),
                        depth: 1,
                        icon: unified_icon(*view),
                        label: name.clone(),
                        unread: *unread,
                        selected,
                        // Addresses are long; the count tells of new mail.
                        bold: false,
                        chevron: None,
                        checking: (*view == Unified::Inbox && self.checking_account(*account))
                            || folder.is_some_and(|f| self.checking_folder(f)),
                    },
                    th,
                    cx,
                )
            }
            sidebar::Row::Folder {
                key,
                depth,
                label,
                role,
                folder,
                unread,
                has_children,
                expanded,
            } => {
                let scheduled = key == compose::SCHEDULED_NAV_KEY;
                // Special folders show their name in the current language;
                // the user's own keep theirs.
                let label = if scheduled {
                    tr!("folder-scheduled")
                } else {
                    role.title().unwrap_or_else(|| label.clone())
                };
                self.render_pill(
                    ix,
                    Pill {
                        key: key.clone().into(),
                        depth: *depth,
                        icon: if scheduled {
                            "schedule"
                        } else {
                            role_icon(*role)
                        },
                        label,
                        unread: *unread,
                        selected: folder.is_some_and(|f| self.listing == Some(Listing::Folder(f))),
                        bold: true,
                        chevron: has_children.then_some(*expanded),
                        // New mail lands in the inbox.
                        checking: folder.is_some_and(|f| self.checking_folder(f))
                            || *role == Role::Inbox
                                && folder
                                    .and_then(|f| self.tree.account_of(f))
                                    .is_some_and(|a| self.checking_account(a)),
                    },
                    th,
                    cx,
                )
            }
        }
    }

    /// An account's name, or "All Accounts", over what it holds, with the
    /// arrow that folds it away; the whole line folds it.
    fn render_heading(
        &self,
        ix: usize,
        name: String,
        (expanded, checking): (bool, bool),
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .id(("nav-row", ix))
            .h(px(NAV_ROW_HEIGHT))
            .w(px(NAV_WIDTH - 16.0))
            .flex()
            .flex_row()
            .items_end()
            .pb(px(2.0))
            .pl(px(NAV_TEXT_LEFT))
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.text_faint))
            .cursor_pointer()
            .rounded_full()
            .when(self.nav_cursor_on(ix), |d| d.shadow(keys_ring(th)))
            // Not over its own right-click menu.
            .when(self.nav_menu.is_none(), |d| {
                d.tooltip(tip(
                    if expanded {
                        tr!("nav-collapse")
                    } else {
                        tr!("nav-expand")
                    },
                    th,
                ))
            })
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_nav_row(ix, cx)))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.open_nav_menu(ix, event.position, cx);
                }),
            )
            .child(div().min_w_0().pb(px(2.0)).truncate().child(name))
            .child(div().flex_1().pl(px(6.0)).pb(px(3.0)).when(checking, |d| {
                d.child(super::nav_menu::turning_arrow(
                    "heading-checking",
                    th.text_faint,
                    12.0,
                ))
            }))
            .child(
                div()
                    .id(("nav-heading-arrow", ix))
                    .flex_none()
                    .size(px(24.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(turning_chevron(
                        SharedString::from(format!("heading:{ix}")),
                        expanded,
                        th.text_dim,
                    )),
            )
            .into_any_element()
    }

    /// A folder's line: a pill with its icon, name and unread count, and
    /// the arrow that shows what it holds, when it holds some.
    fn render_pill(&self, ix: usize, pill: Pill, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Pill {
            key,
            depth,
            icon: icon_name,
            label,
            unread,
            selected,
            bold,
            chevron,
            checking,
        } = pill;
        let indent = 12.0 * depth as f32;
        let text = if selected {
            th.nav_selected_text
        } else {
            th.text
        };
        let bold = bold && (selected || unread > 0);
        // A folded line's arrow shows only while the pointer is over the
        // line, as the list's stars do; an open line keeps its arrow, and
        // so do the keys' line and a phone, which has no pointer.
        let arrow_rests = !self.nav_cursor_on(ix) && !self.layout.shape.is_phone();
        let chevron = chevron.map(|expanded| {
            div()
                .id(("nav-chevron", ix))
                .when(arrow_rests && !expanded, |d| {
                    d.opacity(0.0).group_hover(NAV_PILL, |s| s.opacity(1.0))
                })
                .absolute()
                // In the pill's rounded end, centred on it, so its hover
                // circle keeps an even gap to the pill's edge.
                .left(px(indent + CHEVRON_GAP))
                .top(px(6.0))
                .size(px(20.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .hover(|s| s.bg(rgba(th.hover)))
                .child(turning_chevron(key.clone(), expanded, th.text_dim))
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.toggle_nav_row(ix, cx);
                }))
        });
        // A full pill, inset from the pane's edge; the icon and
        // label stay where they were.
        let row = div()
            .id(("nav-row", ix))
            .group(NAV_PILL)
            .relative()
            .h(px(NAV_ROW_HEIGHT))
            .w(px(NAV_WIDTH - NAV_ROW_END - NAV_ROW_INSET))
            .pl(px(NAV_TEXT_LEFT - NAV_ROW_INSET + indent))
            .pr(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .rounded_full()
            .text_size(px(14.0))
            .text_color(rgba(text))
            .when(bold, |d| d.font_weight(FontWeight::BOLD))
            .when(!selected, |d| d.hover(|s| s.bg(rgba(th.hover))))
            .when(self.nav_cursor_on(ix), |d| d.shadow(keys_ring(th)))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, window, cx| this.click_nav_row(ix, window, cx)))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.open_nav_menu(ix, event.position, cx);
                }),
            )
            .child(Ripple::new(("nav-ripple", ix), rgba(th.ripple)).rounded(NAV_ROW_HEIGHT / 2.0))
            .child(icon(icon_name, text, 20.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .pl(px(18.0))
                    .truncate()
                    .child(label),
            )
            .when(checking, |d| {
                d.child(
                    div()
                        .flex_none()
                        .pl(px(8.0))
                        .child(super::nav_menu::turning_arrow(
                            "nav-checking",
                            th.text_dim,
                            14.0,
                        )),
                )
            })
            .when(unread > 0, |d| {
                d.child(
                    div()
                        .flex_none()
                        .pl(px(8.0))
                        .text_size(px(12.0))
                        .child(format::thousands(unread)),
                )
            })
            .children(chevron);
        // Named by the line rather than its place, which moves as lines
        // above fold or open.
        let row = row.with_spring(
            ElementId::Name(format!("nav-selected:{key}").into()),
            SpringAnimation::new(motion::SMOOTH).to(if selected { 1.0 } else { 0.0 }),
            {
                let bg = th.nav_selected;
                move |row, s: f32| {
                    if s > 0.001 {
                        row.bg(rgba(fade(bg, s)))
                    } else {
                        row
                    }
                }
            },
        );
        // The list lays lines out edge to edge, so the inset is
        // padding around the pill rather than its margin.
        div().pl(px(NAV_ROW_INSET)).child(row).into_any_element()
    }

    fn click_nav_row(&mut self, ix: usize, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let Some(row) = self.nav_rows.get(ix).cloned() else {
            return;
        };
        self.nav_cursor = Some(ix);
        self.nav_by_keys = false;
        // The list already shown stays as it is, without a blink, and goes
        // back to its top, as in Gmail.
        if let Some(next) = listing_of(&row)
            && self.showing(&next, cx)
        {
            self.glide_list_to_top(cx);
            self.picked_from_nav(window, cx);
            return;
        }
        match row {
            sidebar::Row::Folder {
                folder: Some(folder),
                ..
            }
            | sidebar::Row::UnifiedAccount {
                folder: Some(folder),
                ..
            } => {
                self.leave_listing(Listing::Folder(folder), cx);
                self.open_folder(folder, cx);
                self.picked_from_nav(window, cx);
            }
            sidebar::Row::Unified { view, .. } => {
                let listing = Listing::Unified {
                    view,
                    account: None,
                };
                self.leave_listing(listing, cx);
                self.open_unified(view, None, cx);
                self.picked_from_nav(window, cx);
            }
            sidebar::Row::UnifiedAccount {
                view,
                account,
                folder: None,
                ..
            } => {
                let listing = Listing::Unified {
                    view,
                    account: Some(account),
                };
                self.leave_listing(listing, cx);
                self.open_unified(view, Some(account), cx);
                self.picked_from_nav(window, cx);
            }
            sidebar::Row::Folder { key, .. } if key == compose::SCHEDULED_NAV_KEY => {
                self.leave_settings(window, cx);
                self.open_scheduled(cx);
            }
            _ => self.toggle_nav_row(ix, cx),
        }
    }

    /// Whether `listing` is on show as it is, with nothing over it: no
    /// conversation, search or Settings.
    fn showing(&self, listing: &Listing, cx: &Context<Self>) -> bool {
        self.listing.as_ref() == Some(listing)
            && !self.reading
            && self.search_panel.is_none()
            && self.settings_page.is_none()
            && self.search.read(cx).text().is_empty()
    }

    /// Before a line of the folder pane opens `next`: the search gives
    /// way, and the list fades in again when it shows the same.
    pub(super) fn leave_listing(&mut self, next: Listing, cx: &mut Context<Self>) {
        self.clear_search(cx);
        self.search_panel = None;
        if self.listing.as_ref() == Some(&next) && !self.reading {
            self.card_seq += 1;
        }
    }

    /// After a line of the folder pane opened a list.
    pub(super) fn picked_from_nav(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        self.leave_settings(window, cx);
        self.reader = None;
        // A folder picked from the opened navigation closes it.
        self.nav_peek = false;
        self.layout.drawer = false;
        self.peek_task = None;
        window.focus(&self.list_focus, cx);
    }

    /// A list picked in the folder pane while Settings is open takes its
    /// place, as in Gmail; folding a line does not.
    fn leave_settings(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        if self.settings_page.is_some() {
            self.close_settings_page(window, cx);
        }
    }

    /// The arrow of line `ix`: folds or opens what is under it.
    fn toggle_nav_row(&mut self, ix: usize, cx: &mut Context<Self>) {
        self.nav_cursor = Some(ix);
        self.nav_by_keys = false;
        match self.nav_rows.get(ix).cloned() {
            Some(sidebar::Row::AllAccounts { .. }) => self.toggle_all_accounts(cx),
            Some(sidebar::Row::Account { id, .. }) => self.toggle_account(id, cx),
            Some(sidebar::Row::Unified { view, .. }) => self.toggle(view.key(), cx),
            Some(sidebar::Row::Folder { key, .. }) => self.toggle(&key, cx),
            _ => {}
        }
    }

    fn toggle(&mut self, key: &str, cx: &mut Context<Self>) {
        if !self.expanded.remove(key) {
            self.expanded.insert(key.to_owned());
        }
        self.fold_nav();
        cx.notify();
    }

    /// Item `ix` of the folder pane's list: a line, or the lines sliding
    /// open or shut in a group that grows or shrinks.
    fn render_nav_item(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(item) = self.nav_items.get(ix) else {
            return div().into_any_element();
        };
        let (start, end) = (item.start, (item.start + item.len).min(self.nav_rows.len()));
        let Some(fold) = self.nav_fold.as_ref().filter(|f| f.start == start) else {
            return self.render_nav_row(start, th, cx);
        };
        let t = fold.t.clamp(0.0, 1.0);
        div()
            .h(px(NAV_ROW_HEIGHT * (end - start) as f32 * t))
            .overflow_hidden()
            .opacity(t)
            .child(
                div()
                    // The lines glide down from under the one above them
                    // as the group grows.
                    .mt(px(-NAV_ROW_HEIGHT * FOLD_GLIDE * (1.0 - t)))
                    .children((start..end).map(|ix| self.render_nav_row(ix, th, cx))),
            )
            .into_any_element()
    }

    /// Brings the folder pane's list up to date with its lines, telling it
    /// only what changed so it keeps its place.
    pub(super) fn sync_nav_list(&mut self) {
        if self.nav_synced != self.nav_rev {
            self.nav_synced = self.nav_rev;
            let skip = usize::from(self.nav_header() > 0.0);
            let mut items = Vec::with_capacity(self.nav_rows.len());
            let mut ix = skip;
            while ix < self.nav_rows.len() {
                let len = match &self.nav_fold {
                    Some(fold) if fold.start == ix => fold.len.max(1),
                    _ => 1,
                };
                items.push(NavItem {
                    start: ix,
                    len,
                    row: self.nav_rows[ix].clone(),
                });
                ix += len;
            }
            let old = &self.nav_items;
            let prefix = old.iter().zip(&items).take_while(|(a, b)| a == b).count();
            let suffix = old
                .iter()
                .rev()
                .zip(items.iter().rev())
                .take_while(|(a, b)| a == b)
                .count()
                .min(old.len().min(items.len()) - prefix);
            if prefix + suffix < old.len().max(items.len()) {
                self.nav_list
                    .splice(prefix..old.len() - suffix, items.len() - prefix - suffix);
            }
            self.nav_items = items;
        }
        // The group sliding open or shut changes its height every frame.
        if let Some(fold) = &self.nav_fold
            && let Some(ix) = self.nav_items.iter().position(|i| i.start == fold.start)
        {
            self.nav_list.remeasure_items(ix..ix + 1);
        }
    }

    /// Shows the folder pane's lines after an arrow folds or opens a line:
    /// what it holds slides open or shut.
    pub(super) fn fold_nav(&mut self) {
        let before = std::mem::take(&mut self.nav_rows);
        let after = self.nav_rows_now();
        self.nav_fold = None;
        self.nav_rev += 1;
        match fold_between(&before, &after) {
            Some((start, len, true)) => {
                self.nav_rows = after;
                self.nav_fold = Some(Fold::new(start, len, 0.0, 1.0));
            }
            // The lines stay until they have slid shut.
            Some((start, len, false)) => {
                let mut rows = after[..start].to_vec();
                rows.extend_from_slice(&before[start..start + len]);
                rows.extend_from_slice(&after[start..]);
                self.nav_rows = rows;
                self.nav_fold = Some(Fold::new(start, len, 1.0, 0.0));
            }
            None => self.nav_rows = after,
        }
    }

    /// Moves the sliding lines on; once shut they go.
    pub(super) fn tick_nav_fold(&mut self, window: &gpui::Window, reduce: bool) {
        let Some(fold) = &mut self.nav_fold else {
            return;
        };
        fold.t = fold.shown.tick(window, reduce);
        if fold.shown.settled() {
            let shut = fold.shown.target() == 0.0;
            self.nav_fold = None;
            self.nav_rev += 1;
            if shut {
                self.nav_rows = self.nav_rows_now();
            }
        }
    }
}

/// An item of the folder pane's list: line `start`, or with `len` over 1
/// the lines sliding open or shut from there. Two are the same while their
/// lines are, wherever they start.
pub(super) struct NavItem {
    start: usize,
    len: usize,
    row: sidebar::Row,
}

impl PartialEq for NavItem {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.row == other.row
    }
}

/// A new list for the folder pane.
pub(super) fn nav_list() -> ListState {
    ListState::new(0, ListAlignment::Top, px(NAV_ROW_HEIGHT * 8.0))
}

/// How far, in lines, the lines sliding open glide down as they come.
const FOLD_GLIDE: f32 = 0.5;

/// Lines of the folder pane sliding open or shut under the line whose
/// arrow was pressed.
pub(super) struct Fold {
    /// Where the lines start in the pane's lines, and how many there are.
    start: usize,
    len: usize,
    /// How much of them shows, from 0 to 1.
    shown: Spring,
    t: f32,
}

impl Fold {
    fn new(start: usize, len: usize, from: f32, to: f32) -> Self {
        let mut shown = Spring::new(motion::SMOOTH, from);
        shown.set(to);
        Self {
            start,
            len,
            shown,
            t: from,
        }
    }
}

/// What changed between the pane's lines `before` and `after` when one
/// line folded or opened: where the lines under it start, how many there
/// are, and whether they are showing (true) or going. `None` when the
/// change is not one line folding or opening.
fn fold_between(before: &[sidebar::Row], after: &[sidebar::Row]) -> Option<(usize, usize, bool)> {
    let shorter = before.len().min(after.len());
    let prefix = before.iter().zip(after).take_while(|(a, b)| a == b).count();
    // The line whose arrow turned changed, so it is never in the suffix.
    let suffix = before
        .iter()
        .rev()
        .zip(after.iter().rev())
        .take_while(|(a, b)| a == b)
        .count()
        .min(shorter.saturating_sub(prefix + 1));
    let gone = before.len() - prefix - suffix;
    let came = after.len() - prefix - suffix;
    match (gone, came) {
        (1, n) if n > 1 => Some((prefix + 1, n - 1, true)),
        (n, 1) if n > 1 => Some((prefix + 1, n - 1, false)),
        _ => None,
    }
}

/// What [`MailWindow::render_pill`] draws.
struct Pill {
    /// Names the line's arrow, so it turns only when its own line folds.
    key: SharedString,
    depth: usize,
    icon: &'static str,
    label: String,
    unread: u64,
    selected: bool,
    /// Whether it may show in bold, when selected or with unread mail.
    bold: bool,
    /// The arrow, pointing down when what it holds shows.
    chevron: Option<bool>,
    /// Mail is being checked for: a turning arrow beside the name.
    checking: bool,
}

/// An arrow that turns from pointing right to down as its line opens.
fn turning_chevron(key: SharedString, expanded: bool, color: u32) -> AnyElement {
    svg()
        .path("icons/chevron-right.svg")
        .size(px(16.0))
        .flex_none()
        .text_color(rgba(color))
        .with_spring(
            ElementId::Name(format!("nav-turn:{key}").into()),
            SpringAnimation::new(motion::SMOOTH).to(if expanded { 1.0 } else { 0.0 }),
            |arrow, t: f32| {
                arrow.with_transformation(Transformation::rotate(radians(FRAC_PI_2 * t)))
            },
        )
        .into_any_element()
}

/// The icon of a list of the unified inbox.
fn unified_icon(view: Unified) -> &'static str {
    match view {
        Unified::Unread => "unread",
        Unified::Starred => "star",
        Unified::Important => "important",
        _ => view.role().map_or("label", role_icon),
    }
}

pub(super) fn role_icon(role: Role) -> &'static str {
    match role {
        Role::Inbox => "inbox",
        Role::Flagged => "star",
        Role::Snoozed => "schedule",
        Role::Drafts => "drafts",
        Role::Sent => "sent",
        Role::Archive => "archive",
        Role::All => "all-mail",
        Role::Junk => "junk",
        Role::Trash => "trash",
        Role::Other => "label",
    }
}

/// The list a line of the folder pane opens, if it opens one.
fn listing_of(row: &sidebar::Row) -> Option<Listing> {
    match row {
        sidebar::Row::Folder {
            folder: Some(folder),
            ..
        }
        | sidebar::Row::UnifiedAccount {
            folder: Some(folder),
            ..
        } => Some(Listing::Folder(*folder)),
        sidebar::Row::Unified { view, .. } => Some(Listing::Unified {
            view: *view,
            account: None,
        }),
        sidebar::Row::UnifiedAccount {
            view,
            account,
            folder: None,
            ..
        } => Some(Listing::Unified {
            view: *view,
            account: Some(*account),
        }),
        _ => None,
    }
}

/// The key context of the folder pane while it has the keys.
const NAV_CONTEXT: &str = "Navigation";
/// The hover group of a folder's line, for its arrow.
const NAV_PILL: &str = "nav-pill";

/// The folder pane by keyboard, as in Thunderbird, Outlook and KDE's
/// apps: F6 or Tab gives it the keys; Up and Down go through its lines and
/// open each list at once; Right opens what a line holds and Left folds
/// it; Enter (or Space) goes on to the list, or folds a heading.
impl MailWindow {
    /// Gives the pane its keys.
    fn nav_keys(
        &self,
        panel: gpui::Stateful<gpui::Div>,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<gpui::Div> {
        panel
            .key_context(NAV_CONTEXT)
            .track_focus(&self.nav_focus)
            .on_key_down(cx.listener(Self::nav_key))
    }

    /// Whether the pane can take the keys: open beside the list.
    pub(super) fn nav_reachable(&self) -> bool {
        self.nav_docked() && !self.nav_rows.is_empty()
    }

    /// The line the keys are on while the pane has them.
    fn nav_cursor_on(&self, ix: usize) -> bool {
        self.nav_keys_shown && self.nav_by_keys && self.nav_cursor == Some(ix)
    }

    /// The pane takes the keys, on the line of the list on show.
    pub(super) fn focus_nav(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let shown = self
            .nav_rows
            .iter()
            .position(|row| listing_of(row).is_some_and(|l| self.listing.as_ref() == Some(&l)));
        self.nav_cursor = shown.or(self.nav_cursor).or(Some(0));
        self.nav_by_keys = true;
        window.focus(&self.nav_focus, cx);
        cx.notify();
    }

    fn nav_key(
        &mut self,
        event: &gpui::KeyDownEvent,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let stroke = &event.keystroke;
        if stroke.modifiers.modified() || !self.nav_focus.is_focused(window) {
            return;
        }
        let last = self.nav_rows.len().saturating_sub(1);
        let at = self.nav_cursor.unwrap_or(0).min(last);
        let expanded = match self.nav_rows.get(at) {
            Some(
                sidebar::Row::AllAccounts { expanded }
                | sidebar::Row::Unified { expanded, .. }
                | sidebar::Row::Account { expanded, .. },
            ) => Some(*expanded),
            Some(sidebar::Row::Folder {
                has_children: true,
                expanded,
                ..
            }) => Some(*expanded),
            _ => None,
        };
        match stroke.key.as_str() {
            "down" => self.nav_move(at, 1, window, cx),
            "up" => self.nav_move(at, -1, window, cx),
            "home" => self.nav_move(0, 0, window, cx),
            "end" => self.nav_move(last, 0, window, cx),
            "right" if expanded == Some(false) => self.toggle_nav_row(at, cx),
            "right" => self.nav_move(at, 1, window, cx),
            "left" if expanded == Some(true) => self.toggle_nav_row(at, cx),
            "left" => self.nav_move(at, -1, window, cx),
            "enter" | "space" => {
                if listing_of(&self.nav_rows[at]).is_some() {
                    self.click_nav_row(at, window, cx);
                    window.focus(&self.list_focus, cx);
                } else {
                    self.toggle_nav_row(at, cx);
                }
            }
            _ => return,
        }
        self.nav_by_keys = true;
        cx.stop_propagation();
        cx.notify();
    }

    /// Moves the keys `by` lines from `from` (0: onto `from` itself), past
    /// lines that do nothing, and opens the list of the line reached.
    fn nav_move(
        &mut self,
        from: usize,
        by: isize,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let usable = |row: &sidebar::Row| !matches!(row, sidebar::Row::Labels { .. });
        let count = self.nav_rows.len() as isize;
        let step = if by == 0 { 1 } else { by.signum() };
        let mut ix = from as isize + by;
        while (0..count).contains(&ix) && !usable(&self.nav_rows[ix as usize]) {
            ix += step;
        }
        if !(0..count).contains(&ix) {
            return;
        }
        let ix = ix as usize;
        self.nav_cursor = Some(ix);
        if let Some(item) = self
            .nav_items
            .iter()
            .position(|i| (i.start..i.start + i.len).contains(&ix))
        {
            self.nav_list.scroll_to_reveal_item(item);
        }
        // Each list opens as the keys reach it; the keys stay here.
        if listing_of(&self.nav_rows[ix]).is_some() {
            self.click_nav_row(ix, window, cx);
            window.focus(&self.nav_focus, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder(key: &str, expanded: bool) -> sidebar::Row {
        sidebar::Row::Folder {
            key: key.to_owned(),
            depth: 0,
            label: key.to_owned(),
            role: Role::Other,
            folder: None,
            unread: 0,
            has_children: expanded,
            expanded,
        }
    }

    #[test]
    fn folding_finds_the_lines_under_the_arrow() {
        let shut = [folder("a", false), folder("b", false), folder("z", false)];
        let open = [
            folder("a", false),
            folder("b", true),
            folder("b/1", false),
            folder("b/2", false),
            folder("z", false),
        ];
        assert_eq!(fold_between(&shut, &open), Some((2, 2, true)));
        assert_eq!(fold_between(&open, &shut), Some((2, 2, false)));
        // The last line opening, with nothing under it.
        assert_eq!(fold_between(&shut[..2], &open[..4]), Some((2, 2, true)));
        // Anything else changes at once.
        assert_eq!(fold_between(&shut, &shut), None);
        let renamed = [folder("a", false), folder("c", false), folder("z", false)];
        assert_eq!(fold_between(&shut, &renamed), None);
    }
}
