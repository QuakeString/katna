// SPDX-License-Identifier: GPL-3.0-or-later

//! The top bar (menu, Compose, search box, settings, account) and the
//! navigation with the folders, which folds away.

use std::ops::Range;

use gpui::{
    AnimationExt, AnyElement, Context, FontWeight, SpringAnimation, Transformation, div,
    prelude::*, px, radians, rgba, svg, uniform_list,
};
use katna_ui::Ripple;
use katna_ui::motion::{self, lerp};

use super::tour::Spot;
use super::{
    Compose, FocusSearch, Hover, Listing, MailWindow, NAV_WIDTH, PANEL_RADIUS, SEARCH_CONTEXT,
    ToggleNavigation, ToggleSettings, compose,
};
use katna_core::AccountKind;

use crate::format;
use crate::sidebar::{self, Role};
use crate::theme::{Theme, fade, mix};
use crate::widgets::{avatar, elevation, icon, icon_button, icon_button_colored, tip};

const NAV_ROW_HEIGHT: f32 = 32.0;
const SEARCH_HEIGHT: f32 = 40.0;
const COMPOSE_RADIUS: f32 = 16.0;

impl MailWindow {
    pub(super) fn render_top_start(
        &self,
        th: &Theme,
        wide: bool,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        // The bars turn upright as the navigation folds away.
        let folded = 1.0 - self.reserve_spring.value().clamp(0.0, 1.0);
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
            .tooltip(tip("Main menu", th))
            .on_click(cx.listener(|this, _, window, cx| {
                this.toggle_navigation(&ToggleNavigation, window, cx)
            }))
            .child(Ripple::new("menu-ripple", rgba(th.ripple)).centered())
            .child(self.tour_mark(Spot::Menu))
            .child(
                svg()
                    .path("icons/menu.svg")
                    .size(px(24.0))
                    .text_color(rgba(th.text_dim))
                    .with_transformation(Transformation::rotate(radians(
                        folded * std::f32::consts::FRAC_PI_2,
                    ))),
            )
            .into_any_element();
        let compose = div()
            .id("compose")
            .relative()
            .ml(px(10.0))
            .h(px(48.0))
            .when(wide, |d| d.pr(px(24.0)))
            .when(!wide, |d| d.w(px(48.0)).justify_center())
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .rounded(px(COMPOSE_RADIUS))
            .bg(rgba(th.compose))
            .text_color(rgba(th.compose_text))
            .hover(|s| s.shadow(elevation(th, 1.5)))
            .cursor_pointer()
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .when(!wide, |d| d.tooltip(tip("Compose", th)))
            .on_click(cx.listener(|this, _, window, cx| this.compose(&Compose, window, cx)))
            .child(Ripple::new("compose-ripple", rgba(th.ripple)).rounded(COMPOSE_RADIUS))
            .child(self.tour_mark(Spot::Compose))
            .child(div().when(wide, |d| d.pl(px(16.0))).child(icon(
                "compose",
                th.compose_text,
                24.0,
            )))
            .when(wide, |d| {
                d.child(
                    div()
                        .pl(px(12.0))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child("Compose"),
                )
            })
            .into_any_element();
        let mut start = vec![menu];
        if self.mail.is_ok() && !self.accounts.is_empty() {
            start.push(compose);
        }
        start
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
            .relative()
            .w(px(width))
            .h(px(SEARCH_HEIGHT))
            .pl(px(2.0))
            .pr(px(2.0))
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
            .child(self.tour_mark(Spot::Search))
            .child(
                icon_button("search-button", "search", 22.0, th)
                    .tooltip(tip("Search", th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        let text = this.search.read(cx).text().trim().to_owned();
                        if text.is_empty() {
                            this.focus_search(&FocusSearch, window, cx);
                        } else {
                            this.start_search(text, cx);
                        }
                    })),
            )
            .child(div().flex_1().min_w_0().child(self.search.clone()))
            .when(has_text, |d| {
                d.child(
                    icon_button("search-clear", "close", 22.0, th)
                        .tooltip(tip("Clear search", th))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.clear_search(cx);
                            this.focus_search(&FocusSearch, window, cx);
                        })),
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
                .tooltip(tip("Show search options", th))
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
        .tooltip(tip("Settings", th))
        .on_click(
            cx.listener(|this, _, window, cx| this.toggle_settings(&ToggleSettings, window, cx)),
        )
        .child(self.tour_mark(Spot::Settings));
        // The account picture opens the account card; with no account yet,
        // the button adds one.
        let account = match self.accounts.first() {
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
                    .tooltip(tip(format!("{name}\n{}", account.address), th))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.account_menu = !this.account_menu;
                        cx.notify();
                    }))
                    .child(avatar(&name, &account.address, 32.0))
                    .child(self.tour_mark(Spot::Account))
                    .into_any_element()
            }
            None => icon_button_colored("top-account", "person-add", 22.0, th.text_dim, th)
                .tooltip(tip("Add an account", th))
                .on_click(cx.listener(|this, _, window, cx| this.open_add_account(window, cx)))
                .into_any_element(),
        };
        vec![
            settings.into_any_element(),
            div().mx(px(8.0)).child(account).into_any_element(),
        ]
    }

    /// The folders. Folded, the panel is gone; it opens over the list while
    /// the pointer rests on Mail in the app rail or on the panel itself.
    pub(super) fn render_navigation(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let t = self.nav_t.max(0.0);
        let reserve = self.reserve_spring.value().max(0.0);
        // How far the panel is open beyond the space it takes: it floats.
        let float = (t - reserve).clamp(0.0, 1.0);
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
        .w(px(NAV_WIDTH))
        .flex_1()
        .pb(px(16.0));
        let panel = div()
            .id("navigation-panel")
            .absolute()
            .top_0()
            .left_0()
            .bottom(px(16.0 * float))
            .w(px(NAV_WIDTH * t))
            .flex()
            .flex_col()
            .pt(px(lerp(0.0, 12.0, float)))
            .overflow_hidden()
            .opacity(t.min(1.0))
            .when(float > 0.0, |d| {
                d.bg(rgba(th.surface))
                    .rounded(px(PANEL_RADIUS))
                    .shadow(elevation(th, 3.0 * float))
            })
            .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                this.hover_navigation(Hover::Panel, *hovered, cx)
            }))
            .child(list);
        div()
            .relative()
            .flex_none()
            .h_full()
            .w(px(NAV_WIDTH * reserve))
            .child(panel)
            .into_any_element()
    }

    fn render_nav_row(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
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
                .child(div().truncate().child(name.clone()))
                .into_any_element(),
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
                    .pl(px(26.0))
                    .text_size(px(15.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(div().flex_1().min_w_0().truncate().child(if gmail {
                        "Labels"
                    } else {
                        "Folders"
                    }))
                    .when(imap, |d| {
                        d.child(
                            icon_button(("nav-new-label", ix), "add", 20.0, th)
                                .size(px(32.0))
                                .tooltip(tip(
                                    if gmail {
                                        "Create new label"
                                    } else {
                                        "Create new folder"
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
                let scheduled = key == compose::SCHEDULED_NAV_KEY;
                let key = key.clone();
                let indent = 12.0 * *depth as f32;
                let text = if selected {
                    th.nav_selected_text
                } else {
                    th.text
                };
                let bold = selected || *unread > 0;
                let chevron = div()
                    .id(("nav-chevron", ix))
                    .absolute()
                    .left(px(4.0 + indent))
                    .top(px(6.0))
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
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
                    .w(px(NAV_WIDTH - 16.0))
                    .pl(px(26.0 + indent))
                    .pr(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .rounded_r(px(16.0))
                    .text_size(px(14.0))
                    .text_color(rgba(text))
                    .when(bold, |d| d.font_weight(FontWeight::BOLD))
                    .when(!selected, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .cursor_pointer()
                    .on_click(
                        cx.listener(move |this, _, window, cx| this.click_nav_row(ix, window, cx)),
                    )
                    .child(
                        Ripple::new(("nav-ripple", ix), rgba(th.ripple))
                            .corners([0.0, 16.0, 16.0, 0.0]),
                    )
                    .child(icon(
                        if scheduled {
                            "schedule"
                        } else {
                            role_icon(*role)
                        },
                        text,
                        20.0,
                    ))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .pl(px(18.0))
                            .truncate()
                            .child(label.clone()),
                    )
                    .when(*unread > 0, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .pl(px(8.0))
                                .text_size(px(12.0))
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
                // A folder picked from the opened navigation closes it.
                self.nav_peek = false;
                self.peek_task = None;
                window.focus(&self.list_focus, cx);
            }
            None if key == compose::SCHEDULED_NAV_KEY => self.open_scheduled(cx),
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
