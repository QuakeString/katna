// SPDX-License-Identifier: GPL-3.0-or-later

//! The top bar (menu, name, search box, settings, account) and the
//! navigation (Compose and the folders), which folds to a rail.

use std::ops::Range;

use gpui::{
    AnyElement, Context, FontWeight, SpringAnimation, div, linear_color_stop, linear_gradient,
    prelude::*, px, rgba, uniform_list,
};
use katna_ui::Ripple;
use katna_ui::motion::{self, lerp};

use super::{
    FocusSearch, Listing, MailWindow, SEARCH_CONTEXT, ToggleNavigation, ToggleSettings,
};
use crate::format;
use crate::sidebar::{self, Role};
use crate::theme::{Theme, fade, mix};
use crate::widgets::{avatar, elevation, icon, icon_button, icon_button_colored};

use super::{Compose, NAV_WIDTH, RAIL_WIDTH};

const NAV_ROW_HEIGHT: f32 = 32.0;
const COMPOSE_WIDTH: f32 = 142.0;

impl MailWindow {
    pub(super) fn render_top_start(
        &self,
        th: &Theme,
        wide: bool,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let menu = icon_button("menu-button", "menu", 24.0, th)
            .ml(px(2.0))
            .on_click(cx.listener(|this, _, window, cx| {
                this.toggle_navigation(&ToggleNavigation, window, cx)
            }))
            .into_any_element();
        let logo = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .pl(px(4.0))
            .child(
                div()
                    .size(px(34.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(9.0))
                    .bg(linear_gradient(
                        135.0,
                        linear_color_stop(rgba(0x4f8df7ff), 0.0),
                        linear_color_stop(rgba(0x3949c9ff), 1.0),
                    ))
                    .child(icon("mail", 0xffffffff, 22.0)),
            )
            .when(wide, |d| {
                d.child(
                    div()
                        .text_size(px(21.0))
                        .text_color(rgba(th.text_dim))
                        .child("Katna Mail"),
                )
            })
            .into_any_element();
        vec![menu, logo]
    }

    pub(super) fn render_search(
        &self,
        th: &Theme,
        width: f32,
        t: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let available = self.mail.as_ref().is_ok_and(crate::data::Mail::has_index);
        let has_text = !self.search.read(cx).text().is_empty();
        let panel_open = self.search_panel.is_some();
        div()
            .id("search-box")
            .key_context(SEARCH_CONTEXT)
            .w(px(width))
            .h(px(48.0))
            .pl(px(4.0))
            .pr(px(4.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .rounded_full()
            .bg(rgba(mix(th.search, th.search_focused, t)))
            .shadow(elevation(th, 2.0 * t))
            .text_size(px(16.0))
            .line_height(px(24.0))
            .text_color(rgba(th.text))
            .when(!available, |d| d.opacity(0.6))
            // A drag here selects text rather than moving the window.
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .child(
                icon_button("search-button", "search", 22.0, th).on_click(cx.listener(
                    |this, _, window, cx| {
                        let text = this.search.read(cx).text().trim().to_owned();
                        if text.is_empty() {
                            this.focus_search(&FocusSearch, window, cx);
                        } else {
                            this.start_search(text, cx);
                        }
                    },
                )),
            )
            .child(div().flex_1().min_w_0().child(self.search.clone()))
            .when(has_text, |d| {
                d.child(
                    icon_button("search-clear", "close", 22.0, th).on_click(cx.listener(
                        |this, _, window, cx| {
                            this.clear_search(cx);
                            this.focus_search(&FocusSearch, window, cx);
                        },
                    )),
                )
            })
            .child(
                icon_button_colored(
                    "search-options",
                    "tune",
                    22.0,
                    if panel_open { th.accent } else { th.text_dim },
                    th,
                )
                .on_click(cx.listener(|this, _, window, cx| {
                    this.toggle_search_panel(window, cx);
                })),
            )
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
        .on_click(cx.listener(|this, _, window, cx| {
            this.toggle_settings(&ToggleSettings, window, cx)
        }))
        .into_any_element();
        let mut end = vec![settings];
        if let Some(account) = self.accounts.first() {
            let name = if account.display_name.trim().is_empty() {
                &account.address
            } else {
                &account.display_name
            };
            end.push(
                div()
                    .id("account")
                    .ml(px(4.0))
                    .mr(px(8.0))
                    .p(px(4.0))
                    .rounded_full()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_mouse_move(|_, _, cx| cx.stop_propagation())
                    .child(avatar(name, &account.address, 32.0))
                    .into_any_element(),
            );
        }
        end
    }

    pub(super) fn render_navigation(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let t = self.nav_t;
        let reserve = self.reserve_spring.value();
        // How far the panel is open beyond the space it takes: it floats.
        let float = (t - reserve).clamp(0.0, 1.0);
        let compose = div()
            .id("compose")
            .relative()
            .overflow_hidden()
            .ml(px(8.0))
            .h(px(56.0))
            .w(px(lerp(56.0, COMPOSE_WIDTH, t)))
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .rounded(px(16.0))
            .bg(rgba(th.compose))
            .text_color(rgba(th.compose_text))
            .hover(|s| s.shadow(elevation(th, 1.5)))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, window, cx| this.compose(&Compose, window, cx)))
            .child(Ripple::new("compose-ripple", rgba(th.ripple)))
            .child(
                div()
                    .pl(px(16.0))
                    .child(icon("compose", th.compose_text, 24.0)),
            )
            .child(
                div()
                    .pl(px(12.0))
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .opacity(t)
                    .child("Compose"),
            );
        let list = uniform_list(
            "navigation",
            self.nav_rows.len(),
            cx.processor(|this, range: Range<usize>, window, cx| {
                let th = this.theme(window);
                range
                    .map(|ix| this.render_nav_row(ix, &th, cx))
                    .collect::<Vec<_>>()
            }),
        )
        .track_scroll(&self.nav_scroll)
        .flex_1()
        .pb(px(16.0));
        let panel = div()
            .id("navigation-panel")
            .absolute()
            .top_0()
            .left_0()
            .bottom_0()
            .w(px(lerp(RAIL_WIDTH, NAV_WIDTH, t)))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .pt(px(8.0))
            .overflow_hidden()
            .bg(rgba(if float > 0.0 { th.surface } else { th.page }))
            .when(float > 0.0, |d| {
                d.rounded_r(px(16.0)).shadow(elevation(th, 3.0 * float))
            })
            .on_hover(
                cx.listener(|this, hovered: &bool, _, cx| this.hover_navigation(*hovered, cx)),
            )
            .child(compose)
            .child(list);
        div()
            .relative()
            .flex_none()
            .h_full()
            .w(px(lerp(RAIL_WIDTH, NAV_WIDTH, reserve)))
            .child(panel)
            .into_any_element()
    }

    fn render_nav_row(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let t = self.nav_t;
        match &self.nav_rows[ix] {
            sidebar::Row::Account { name, .. } => div()
                .id(("nav-row", ix))
                .h(px(NAV_ROW_HEIGHT))
                .flex()
                .items_end()
                .pb(px(4.0))
                .pl(px(26.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_faint))
                .opacity(t)
                .child(div().truncate().child(name.clone()))
                .into_any_element(),
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
                let selected = folder.is_some_and(|f| self.listing == Some(Listing::Folder(f)));
                let key = key.clone();
                let indent = 12.0 * *depth as f32 * t;
                let text = if selected {
                    th.nav_selected_text
                } else {
                    th.text
                };
                let bold = selected || *unread > 0;
                let chevron = div()
                    .id(("nav-chevron", ix))
                    .absolute()
                    .left(px(lerp(-10.0, 4.0 + indent, t)))
                    .top(px(6.0))
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .opacity(t)
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(icon(
                        if *expanded {
                            "chevron-down"
                        } else {
                            "chevron-right"
                        },
                        th.text_dim,
                        16.0,
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.toggle(&key, cx);
                    }));
                let row = div()
                    .id(("nav-row", ix))
                    .relative()
                    .overflow_hidden()
                    .h(px(NAV_ROW_HEIGHT))
                    .ml(px(lerp(8.0, 0.0, t)))
                    .w(px(lerp(56.0, NAV_WIDTH - 16.0, t)))
                    .pl(px(lerp(18.0, 26.0, t) + indent))
                    .pr(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .rounded_l(px(lerp(16.0, 0.0, t)))
                    .rounded_r(px(16.0))
                    .text_size(px(14.0))
                    .text_color(rgba(text))
                    .when(bold, |d| d.font_weight(FontWeight::BOLD))
                    .when(!selected, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .cursor_pointer()
                    .on_click(
                        cx.listener(move |this, _, window, cx| this.click_nav_row(ix, window, cx)),
                    )
                    .child(Ripple::new(("nav-ripple", ix), rgba(th.ripple)))
                    .child(
                        div()
                            .relative()
                            .child(icon(role_icon(*role), text, 20.0))
                            .when(*unread > 0 && t < 1.0, |d| {
                                d.child(
                                    div()
                                        .absolute()
                                        .top(px(-2.0))
                                        .right(px(-3.0))
                                        .size(px(8.0))
                                        .rounded_full()
                                        .border_1()
                                        .border_color(rgba(th.page))
                                        .bg(rgba(th.accent))
                                        .opacity(1.0 - t),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .pl(px(18.0))
                            .truncate()
                            .opacity(t)
                            .child(label.clone()),
                    )
                    .when(*unread > 0, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .pl(px(8.0))
                                .text_size(px(12.0))
                                .opacity(t)
                                .child(format::thousands(*unread)),
                        )
                    })
                    .when(*has_children, |d| d.child(chevron));
                row.with_spring(
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
                )
                .into_any_element()
            }
        }
    }

    fn click_nav_row(&mut self, ix: usize, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let Some(sidebar::Row::Folder { key, folder, .. }) = self.nav_rows.get(ix).cloned() else {
            return;
        };
        match folder {
            Some(folder) => {
                self.clear_search(cx);
                self.search_panel = None;
                if self.folder == Some(folder) && !self.reading {
                    self.card_seq += 1;
                }
                self.open_folder(folder, cx);
                self.reader = None;
                window.focus(&self.list_focus, cx);
            }
            None => self.toggle(&key, cx),
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

pub(super) fn role_icon(role: Role) -> &'static str {
    match role {
        Role::Inbox => "inbox",
        Role::Flagged => "star",
        Role::Drafts => "drafts",
        Role::Sent => "sent",
        Role::Archive => "archive",
        Role::All => "all-mail",
        Role::Junk => "junk",
        Role::Trash => "trash",
        Role::Other => "label",
    }
}
