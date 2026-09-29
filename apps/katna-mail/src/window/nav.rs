// SPDX-License-Identifier: GPL-3.0-or-later

//! The top bar (menu, the app's name, search box, settings, account), the
//! navigation with the folders, which folds away, and Compose, which sits
//! over the folders and moves into the app rail when they fold.

use std::ops::Range;

use std::f32::consts::FRAC_PI_2;

use gpui::{
    AnimationExt, AnyElement, Context, ElementId, FontWeight, MouseButton, MouseDownEvent,
    PathBuilder, SharedString, SpringAnimation, Transformation, canvas, div, point, prelude::*,
    radians, rgba, svg, uniform_list,
};
use katna_ui::Ripple;
use katna_ui::motion::{self, lerp};
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
use crate::theme::{Theme, fade};
use crate::widgets::{
    elevation, icon, icon_button, icon_button_colored, katna_mark, keys_ring, tip,
};

/// How far the floating folder pane stands off the rail and the top bar.
const FLOAT_GAP: f32 = 8.0;
/// How far its notch reaches toward the rail, and half its height.
const NOTCH: f32 = 8.0;

const NAV_ROW_HEIGHT: f32 = 32.0;
/// The gap around a folder's arrow, inside its pill's rounded end.
const CHEVRON_GAP: f32 = (NAV_ROW_HEIGHT - 20.0) / 2.0;
/// Where folder icons and headings start, from the pane's edge: after the
/// inset, the arrow and a gap.
const NAV_TEXT_LEFT: f32 = NAV_ROW_INSET + CHEVRON_GAP + 20.0 + 4.0;
const SEARCH_HEIGHT: f32 = 40.0;
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
        let open = self
            .reserve_spring
            .value()
            .max(self.layout.drawer_t())
            .clamp(0.0, 1.0);
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
                if open > 0.5 {
                    tr!("folders-hide")
                } else {
                    tr!("folders-show")
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
    fn render_title(&self, th: &Theme, label: f32, (brand, name): (f32, f32)) -> AnyElement {
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
        let placeholder = super::text_width(
            &tr!("search-mail"),
            16.0,
            FontWeight::NORMAL,
            self.font.as_ref(),
            window,
        );
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
            // Active, it keeps its color and gains a faint edge. In a
            // blurred window it lets the blur through like the bar
            // around it, so it reads as the same frosted glass.
            .bg(rgba(if th.backdrop == 0 {
                fade(
                    th.search,
                    f32::from(katna_chrome::tokens::blur_alpha(th.dark)) / 255.0,
                )
            } else {
                th.search
            }))
            .border_1()
            .border_color(rgba(fade(th.text_faint, 0.5 * t.clamp(0.0, 1.0))))
            .text_size(px(16.0))
            .line_height(px(24.0))
            .text_color(rgba(th.text))
            .when(!available && !settings, |d| d.opacity(0.6))
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
                                    if text.is_empty() || this.settings_page.is_some() {
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
                    .children(
                        (!has_text && !settings)
                            .then(|| self.render_search_hint(th, window))
                            .flatten(),
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
            .when(!settings, |d| {
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
        let list = uniform_list(
            "navigation",
            self.nav_rows.len() - skip,
            cx.processor(move |this, range: Range<usize>, window, cx| {
                let th = this.theme(window);
                range
                    .map(|ix| this.render_nav_row(ix + skip, &th, cx))
                    .collect::<Vec<_>>()
            }),
        )
        .track_scroll(&self.nav_scroll)
        .w(px(NAV_WIDTH))
        .flex_1()
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
            .children(self.render_drawer_head(th, cx))
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
                (*expanded, self.checking_mail()),
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
                        checking: *view == Unified::Inbox && self.checking_mail(),
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
                        checking: *view == Unified::Inbox && self.checking_account(*account),
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
                        checking: *role == Role::Inbox
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
            .tooltip(tip(
                if expanded {
                    tr!("nav-collapse")
                } else {
                    tr!("nav-expand")
                },
                th,
            ))
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
        let chevron = chevron.map(|expanded| {
            div()
                .id(("nav-chevron", ix))
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
            .relative()
            .h(px(NAV_ROW_HEIGHT))
            .w(px(NAV_WIDTH - 16.0 - NAV_ROW_INSET))
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
        let row = row.with_spring(
            ("nav-selected", ix),
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
        // The list already shown stays as it is, without a blink.
        if let Some(next) = listing_of(&row)
            && self.showing(&next, cx)
        {
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
        self.rebuild_nav();
        cx.notify();
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
        let skip = usize::from(self.nav_header() > 0.0);
        if ix >= skip {
            self.nav_scroll
                .scroll_to_item(ix - skip, gpui::ScrollStrategy::Nearest);
        }
        // Each list opens as the keys reach it; the keys stay here.
        if listing_of(&self.nav_rows[ix]).is_some() {
            self.click_nav_row(ix, window, cx);
            window.focus(&self.nav_focus, cx);
        }
    }
}
