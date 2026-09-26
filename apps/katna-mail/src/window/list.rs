// SPDX-License-Identifier: GPL-3.0-or-later

//! The list card: the toolbar (select, refresh, more; or actions on the
//! ticked lines), the inbox tabs and the lines, one row each, or three
//! stacked lines when the list is narrow.

use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, BoxShadow, Context, Div, FontWeight, HighlightStyle,
    SharedString, SpringAnimation, Stateful, StyledText, deferred, div, ease_out_quint,
    linear_color_stop, linear_gradient, point, prelude::*, px, rgba, uniform_list,
};
use katna_core::config::Density;
use katna_ui::Ripple;
use katna_ui::motion;

use super::{
    Act, LIST_CONTEXT, Listing, MailWindow, Menu, READER_CONTEXT, Reload, STACKED_BELOW,
};
use crate::data::{Category, EntryKey, Row};
use crate::format;
use crate::theme::{Theme, fade, mix};
use crate::widgets::{
    icon, icon_button, icon_button_colored, menu, menu_item, placeholder,
    toolbar,
};

const TAB_HEIGHT: f32 = 56.0;
const TAB_MAX_WIDTH: f32 = 240.0;

/// What the select menu ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pick {
    All,
    None,
    Read,
    Unread,
    Starred,
    Unstarred,
}

impl MailWindow {
    /// Height of one line of the list.
    pub(super) fn row_height(&self) -> f32 {
        let compact = self.config.mail.density == Density::Compact;
        match (self.stacked(), compact) {
            (true, false) => 76.0,
            (true, true) => 64.0,
            (false, false) => 40.0,
            (false, true) => 32.0,
        }
    }

    /// Whether lines show as three stacked lines: the list is narrow.
    fn stacked(&self) -> bool {
        self.list_width() < STACKED_BELOW
    }

    /// The list card's width once the reading pane is where it is going.
    pub(super) fn list_width(&self) -> f32 {
        let open = self.split() && self.reading && self.reader.is_some();
        if open {
            let pane = (self.cards_width - super::SPLIT_GAP) * self.config.mail.reading_pane_share;
            self.cards_width - pane - super::SPLIT_GAP
        } else {
            self.cards_width
        }
    }

    pub(super) fn render_list_card(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let two_pane_reading = !self.split() && self.reading;
        let (toolbar, body) = if two_pane_reading {
            (self.render_reader_toolbar(th, cx), self.render_reader(th, cx))
        } else {
            let tabs = self.shows_tabs().then(|| self.render_tabs(th, cx));
            let banner = self.render_select_banner(th, cx);
            let list = self.render_list(th, cx);
            (
                self.render_list_toolbar(th, cx),
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .children(tabs)
                    .children(banner)
                    .child(div().flex_1().min_h_0().child(list))
                    .into_any_element(),
            )
        };
        let reading_context = self.reading && (two_pane_reading || self.split());
        let card = div()
            .id("card")
            .key_context(if reading_context {
                READER_CONTEXT
            } else {
                LIST_CONTEXT
            })
            .track_focus(&self.list_focus)
            .size_full()
            .flex()
            .flex_col()
            .rounded(px(16.0))
            .overflow_hidden()
            .bg(rgba(th.surface))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::page_down))
            .on_action(cx.listener(Self::page_up))
            .on_action(cx.listener(Self::open_message))
            .on_action(cx.listener(Self::close_message))
            .on_action(cx.listener(Self::scroll_down))
            .on_action(cx.listener(Self::scroll_up))
            .on_action(cx.listener(Self::scroll_page_down))
            .on_action(cx.listener(Self::scroll_page_up))
            .on_action(cx.listener(Self::archive))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::report_spam))
            .on_action(cx.listener(Self::mark_read))
            .on_action(cx.listener(Self::mark_unread))
            .on_action(cx.listener(Self::toggle_star))
            .on_action(cx.listener(Self::toggle_check))
            .child(toolbar)
            .child(div().flex_1().min_h_0().child(body).with_animation(
                ("card", self.card_seq),
                Animation::new(Duration::from_millis(220)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            ));
        card.into_any_element()
    }

    // Toolbar

    fn render_list_toolbar(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let count = self.entries.len();
        let checked = self.checked.len();
        let page_checked = checked > 0 && self.visible_keys().all(|k| self.checked.contains(&k));
        let (box_icon, box_color) = match checked {
            0 => ("checkbox", th.text_dim),
            _ if page_checked || self.checked_all => ("checkbox-checked", th.text),
            _ => ("checkbox-partial", th.text),
        };
        let select = div()
            .id("select")
            .flex()
            .flex_row()
            .items_center()
            .h(px(40.0))
            .pl(px(8.0))
            .pr(px(2.0))
            .rounded(px(4.0))
            .hover(|s| s.bg(rgba(th.hover)))
            .child(
                div()
                    .id("select-box")
                    .size(px(28.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| {
                        let pick = if this.checked.is_empty() {
                            Pick::All
                        } else {
                            Pick::None
                        };
                        this.pick(pick, cx);
                    }))
                    .child(icon(box_icon, box_color, 20.0)),
            )
            .child(
                div()
                    .id("select-menu")
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::Select, cx)))
                    .child(icon("drop-down", th.text_dim, 20.0)),
            );
        let select = self.with_menu(select, Menu::Select, th, cx);
        let mut bar = toolbar(th).child(select);
        if checked == 0 {
            bar = bar
                .child(
                    icon_button("refresh", "refresh", 20.0, th).on_click(
                        cx.listener(|this, _, window, cx| this.reload(&Reload, window, cx)),
                    ),
                )
                .child({
                    let more = icon_button("list-more", "more", 20.0, th).on_click(
                        cx.listener(|this, _, _, cx| this.toggle_menu(Menu::ListMore, cx)),
                    );
                    self.with_menu(more, Menu::ListMore, th, cx)
                });
        } else {
            let any_unread = self.checked_rows().iter().any(|r| r.unread);
            let read_button = if any_unread {
                icon_button("mark-read", "mark-read", 20.0, th)
                    .on_click(cx.listener(|this, _, _, cx| this.act_on_targets(Act::Read(true), cx)))
            } else {
                icon_button("mark-unread", "mail", 20.0, th).on_click(
                    cx.listener(|this, _, _, cx| this.act_on_targets(Act::Read(false), cx)),
                )
            };
            bar = bar
                .child(self.action_buttons("list", th, cx))
                .child(separator(th))
                .child(read_button)
                .child({
                    let move_to = icon_button("list-move", "move-to", 20.0, th).on_click(
                        cx.listener(|this, _, _, cx| this.toggle_menu(Menu::MoveTo, cx)),
                    );
                    self.with_menu(move_to, Menu::MoveTo, th, cx)
                })
                .child({
                    let more = icon_button("list-more", "more", 20.0, th).on_click(
                        cx.listener(|this, _, _, cx| this.toggle_menu(Menu::ListMore, cx)),
                    );
                    self.with_menu(more, Menu::ListMore, th, cx)
                });
        }
        let label: Option<SharedString> = match (&self.search_error, &self.listing) {
            (Some(err), _) => Some(err.clone()),
            (None, Some(Listing::Search { query, .. })) => {
                Some(format!("Results for “{query}”").into())
            }
            _ => None,
        };
        let range = if count == 0 {
            String::new()
        } else {
            let start = self.visible.start.min(count - 1) + 1;
            let end = self.visible.end.clamp(start, count);
            let total = match &self.listing {
                Some(Listing::Search {
                    total: Some(total), ..
                }) if *total > count && !self.config.mail.conversations => {
                    format!("about {}", format::thousands(*total as u64))
                }
                _ => format::thousands(count as u64),
            };
            format!(
                "{}–{} of {total}",
                format::thousands(start as u64),
                format::thousands(end as u64)
            )
        };
        let at_top = self.visible.start == 0;
        let at_end = self.visible.end >= count;
        bar.child(
            div()
                .pl(px(8.0))
                .flex_1()
                .min_w_0()
                .truncate()
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .children(label),
        )
        .child(
            div()
                .flex_none()
                .px(px(8.0))
                .text_size(px(12.0))
                .text_color(rgba(th.text_faint))
                .child(range),
        )
        .child(
            icon_button("page-up", "chevron-left", 20.0, th)
                .when(at_top, |d| d.opacity(0.4))
                .on_click(cx.listener(|this, _, _, cx| {
                    let page = this.visible.len().max(1);
                    let ix = this.visible.start.saturating_sub(page);
                    this.list_scroll
                        .scroll_to_item(ix, gpui::ScrollStrategy::Top);
                    cx.notify();
                })),
        )
        .child(
            icon_button("page-down", "chevron-right", 20.0, th)
                .when(at_end, |d| d.opacity(0.4))
                .on_click(cx.listener(|this, _, _, cx| {
                    let ix = this.visible.end.min(this.entries.len().saturating_sub(1));
                    this.list_scroll
                        .scroll_to_item(ix, gpui::ScrollStrategy::Top);
                    cx.notify();
                })),
        )
        .into_any_element()
    }

    /// Archive, spam and delete, for the ticked lines or the open
    /// conversation.
    pub(super) fn action_buttons(
        &self,
        prefix: &'static str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        div()
            .flex()
            .flex_row()
            .child(
                icon_button((prefix, 1_usize), "archive", 20.0, th)
                    .on_click(cx.listener(|this, _, _, cx| this.act_on_targets(Act::Archive, cx))),
            )
            .child(
                icon_button((prefix, 2_usize), "junk", 20.0, th)
                    .on_click(cx.listener(|this, _, _, cx| this.act_on_targets(Act::Spam, cx))),
            )
            .child(
                icon_button((prefix, 3_usize), "trash", 20.0, th)
                    .on_click(cx.listener(|this, _, _, cx| this.act_on_targets(Act::Delete, cx))),
            )
    }

    pub(super) fn toggle_menu(&mut self, menu: Menu, cx: &mut Context<Self>) {
        self.menu = if self.menu == Some(menu) {
            None
        } else {
            Some(menu)
        };
        cx.notify();
    }

    /// Puts `anchor` in a box that also holds `which` menu when it is open,
    /// drawn over everything, with a scrim that closes it on a click
    /// elsewhere.
    pub(super) fn with_menu(
        &self,
        anchor: impl IntoElement,
        which: Menu,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let open = self.menu == Some(which);
        div()
            .relative()
            .child(anchor)
            .when(open, |d| {
                let items = self.menu_items(which, th, cx);
                d.child(
                    deferred(
                        div()
                            .id("menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.menu = None;
                                    cx.notify();
                                }),
                            ),
                    )
                    .with_priority(1),
                )
                .child(
                    deferred(
                        div()
                            .absolute()
                            .top(px(40.0))
                            .left(px(0.0))
                            .occlude()
                            .child(items.with_animation(
                                ("menu", which as usize),
                                Animation::new(Duration::from_millis(160))
                                    .with_easing(ease_out_quint()),
                                |el, t| el.opacity(t).mt(px(-6.0 * (1.0 - t))),
                            )),
                    )
                    .with_priority(2),
                )
            })
            .into_any_element()
    }

    fn menu_items(&self, which: Menu, th: &Theme, cx: &mut Context<Self>) -> Div {
        match which {
            Menu::Select => menu(th).children(
                [
                    (Pick::All, "All"),
                    (Pick::None, "None"),
                    (Pick::Read, "Read"),
                    (Pick::Unread, "Unread"),
                    (Pick::Starred, "Starred"),
                    (Pick::Unstarred, "Unstarred"),
                ]
                .into_iter()
                .map(|(pick, label)| {
                    menu_item(("pick", pick as usize), label, th)
                        .on_click(cx.listener(move |this, _, _, cx| this.pick(pick, cx)))
                }),
            ),
            Menu::ListMore | Menu::ReaderMore => {
                let targets = if which == Menu::ListMore && self.checked.is_empty() {
                    None
                } else {
                    Some(())
                };
                match targets {
                    None => menu(th).child(
                        menu_item("mark-all-read", "Mark all as read", th).on_click(cx.listener(
                            |this, _, _, cx| {
                                let keys = this.entries.iter().map(|e| e.key).collect();
                                this.act(Act::Read(true), keys, cx);
                            },
                        )),
                    ),
                    Some(()) => menu(th)
                        .child(
                            menu_item("more-read", "Mark as read", th).on_click(
                                cx.listener(|this, _, _, cx| this.act_on_targets(Act::Read(true), cx)),
                            ),
                        )
                        .child(
                            menu_item("more-unread", "Mark as unread", th).on_click(cx.listener(
                                |this, _, window, cx| {
                                    this.mark_unread(&super::MarkUnread, window, cx)
                                },
                            )),
                        )
                        .child(
                            menu_item("more-star", "Add star", th).on_click(
                                cx.listener(|this, _, _, cx| this.act_on_targets(Act::Star(true), cx)),
                            ),
                        )
                        .child(
                            menu_item("more-unstar", "Remove star", th).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.act_on_targets(Act::Star(false), cx)
                                }),
                            ),
                        ),
                }
            }
            Menu::MoveTo => {
                let current = self.folder;
                let folders = self
                    .account()
                    .map(|a| self.tree.folders_of(a))
                    .unwrap_or_default();
                menu(th)
                    .child(
                        div()
                            .px(px(16.0))
                            .pb(px(6.0))
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child("Move to:"),
                    )
                    .child(
                        div()
                            .id("move-to-list")
                            .max_h(px(360.0))
                            .overflow_y_scroll()
                            .children(
                                folders
                                    .into_iter()
                                    .filter(|(id, ..)| Some(*id) != current)
                                    .map(|(id, name, role)| {
                                        div()
                                            .id(("move", id.0 as usize))
                                            .h(px(32.0))
                                            .px(px(16.0))
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(12.0))
                                            .cursor_pointer()
                                            .hover(|s| s.bg(rgba(th.hover)))
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.act_on_targets(Act::MoveTo(id), cx)
                                            }))
                                            .child(icon(super::nav::role_icon(role), th.text_dim, 18.0))
                                            .child(name)
                                    }),
                            ),
                    )
            }
        }
    }

    /// Keys of the lines on screen.
    fn visible_keys(&self) -> impl Iterator<Item = EntryKey> + '_ {
        let range = self.visible.start.min(self.entries.len())..self.visible.end.min(self.entries.len());
        self.entries[range].iter().map(|e| e.key)
    }

    /// Rows of the ticked lines that are loaded.
    fn checked_rows(&mut self) -> Vec<Rc<Row>> {
        let entries: Vec<_> = self
            .entries
            .iter()
            .filter(|e| self.checked.contains(&e.key))
            .take(500)
            .copied()
            .collect();
        match &mut self.mail {
            Ok(mail) => mail
                .rows(&entries, self.show_recipients)
                .into_iter()
                .flatten()
                .map(|r| self.with_pending(r))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// `row` with changes the daemon has not confirmed yet.
    fn with_pending(&self, row: Rc<Row>) -> Rc<Row> {
        match self.pending.get(&row.key) {
            Some(p) if p.unread.is_some() || p.flagged.is_some() => {
                let mut row = (*row).clone();
                row.unread = p.unread.unwrap_or(row.unread);
                row.flagged = p.flagged.unwrap_or(row.flagged);
                Rc::new(row)
            }
            _ => row,
        }
    }

    fn pick(&mut self, pick: Pick, cx: &mut Context<Self>) {
        self.menu = None;
        self.checked_all = false;
        self.checked.clear();
        let range =
            self.visible.start.min(self.entries.len())..self.visible.end.min(self.entries.len());
        let entries = self.entries[range].to_vec();
        let rows: Vec<Option<Rc<Row>>> = match &mut self.mail {
            Ok(mail) => mail.rows(&entries, self.show_recipients),
            Err(_) => Vec::new(),
        };
        for (entry, row) in entries.iter().zip(rows) {
            let Some(row) = row.map(|r| self.with_pending(r)) else {
                continue;
            };
            let take = match pick {
                Pick::All => true,
                Pick::None => false,
                Pick::Read => !row.unread,
                Pick::Unread => row.unread,
                Pick::Starred => row.flagged,
                Pick::Unstarred => !row.flagged,
            };
            if take {
                self.checked.insert(entry.key);
            }
        }
        cx.notify();
    }

    /// "All 20 on screen are selected. Select all 1,234" when every line on
    /// screen is ticked and there are more.
    fn render_select_banner(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let on_screen = self.visible.len().min(self.entries.len());
        let page_checked = !self.checked.is_empty()
            && self.checked.len() == on_screen
            && self.visible_keys().all(|k| self.checked.contains(&k));
        if !(self.checked_all || page_checked && self.entries.len() > on_screen) {
            return None;
        }
        let noun = if self.config.mail.conversations {
            "conversations"
        } else {
            "messages"
        };
        let place = self
            .folder_name()
            .map(|f| format!(" in {f}"))
            .unwrap_or_default();
        let total = format::thousands(self.entries.len() as u64);
        let (text, link) = if self.checked_all {
            (format!("All {total} {noun}{place} are selected."), "Clear selection".to_owned())
        } else {
            (
                format!("All {on_screen} {noun} on screen are selected."),
                format!("Select all {total} {noun}{place}"),
            )
        };
        Some(
            div()
                .flex_none()
                .h(px(40.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_center()
                .gap(px(8.0))
                .bg(rgba(th.read_row))
                .border_b_1()
                .border_color(rgba(th.divider))
                .text_size(px(13.0))
                .child(text)
                .child(
                    div()
                        .id("select-all")
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.accent))
                        .cursor_pointer()
                        .on_click(cx.listener(|this, _, _, cx| {
                            if this.checked_all {
                                this.checked.clear();
                                this.checked_all = false;
                            } else {
                                this.checked = this.entries.iter().map(|e| e.key).collect();
                                this.checked_all = true;
                            }
                            cx.notify();
                        }))
                        .child(link),
                )
                .into_any_element(),
        )
    }

    // Tabs

    fn render_tabs(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let width = (self.list_width() / Category::ALL.len() as f32).min(TAB_MAX_WIDTH);
        let at = self.tab_spring.value();
        let selected = self.category;
        let color = th.tabs[selected.index()];
        let tabs = Category::ALL.iter().map(|&category| {
            let on = category == selected;
            let tint = th.tabs[category.index()];
            let unread = self.category_unread.get(&category).copied().unwrap_or(0);
            let compact = width < 150.0;
            div()
                .id(("tab", category.index()))
                .relative()
                .overflow_hidden()
                .flex_none()
                .w(px(width))
                .h(px(TAB_HEIGHT))
                .pl(px(if compact { 12.0 } else { 16.0 }))
                .pr(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(if compact { 10.0 } else { 16.0 }))
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(if on { tint } else { th.text_dim }))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, _, cx| this.open_category(category, cx)))
                .child(Ripple::new(("tab-ripple", category.index()), rgba(th.ripple)))
                .child(icon(category.icon(), if on { tint } else { th.text_dim }, 20.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(div().truncate().child(category.label()))
                        .when(unread > 0 && !on && category != Category::Primary, |d| {
                            d.child(
                                div()
                                    .mt(px(2.0))
                                    .px(px(6.0))
                                    .rounded_full()
                                    .bg(rgba(tint))
                                    .text_color(rgba(th.on_accent))
                                    .text_size(px(11.0))
                                    .line_height(px(16.0))
                                    .truncate()
                                    .child(format!("{} new", format::thousands(unread))),
                            )
                        }),
                )
        });
        div()
            .relative()
            .flex_none()
            .h(px(TAB_HEIGHT))
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(rgba(th.divider))
            .children(tabs)
            // The indicator slides to the open tab.
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .left(px(at * width + 8.0))
                    .w(px(width - 16.0))
                    .h(px(3.0))
                    .rounded_t(px(3.0))
                    .bg(rgba(color)),
            )
            .into_any_element()
    }

    // Lines

    fn render_list(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        if self.entries.is_empty() {
            let text = match &self.listing {
                Some(Listing::Search { .. }) => "No messages matched your search.".to_owned(),
                Some(Listing::Folder(_)) if self.shows_tabs() => {
                    format!("No mail in {}.", self.category.label())
                }
                Some(Listing::Folder(_)) => format!(
                    "No messages in {}.",
                    self.folder_name().unwrap_or_else(|| "this folder".into())
                ),
                None => String::new(),
            };
            return placeholder(&text, th);
        }
        uniform_list(
            "messages",
            self.entries.len(),
            cx.processor(|this, range: Range<usize>, window, cx| {
                if this.visible != range {
                    this.visible = range.clone();
                    cx.notify();
                }
                let th = this.theme(window);
                let entries = this.entries[range.clone()].to_vec();
                let rows = match &mut this.mail {
                    Ok(mail) => mail.rows(&entries, this.show_recipients),
                    Err(_) => vec![None; range.len()],
                };
                let rows: Vec<_> = rows
                    .into_iter()
                    .map(|r| r.map(|r| this.with_pending(r)))
                    .collect();
                range
                    .zip(entries)
                    .zip(rows)
                    .map(|((ix, entry), row)| this.render_row(ix, entry.key, row, &th, cx))
                    .collect::<Vec<_>>()
            }),
        )
        .track_scroll(&self.list_scroll)
        .size_full()
        .into_any_element()
    }

    fn render_row(
        &self,
        ix: usize,
        key: EntryKey,
        row: Option<Rc<Row>>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let height = self.row_height();
        let hovered = self.hovered == Some(ix);
        let under_hovered = ix > 0 && self.hovered == Some(ix - 1);
        let cursor = self.selected == Some(ix);
        let checked = self.checked.contains(&key);
        let open = self.split() && self.reader.as_ref().is_some_and(|r| r.key == key);
        let unread = row.as_ref().is_some_and(|r| r.unread);
        let background = if checked {
            th.checked_row
        } else if open {
            mix(th.surface, th.accent, 0.12)
        } else if unread {
            th.surface
        } else {
            th.read_row
        };
        let base = div()
            .id(("row", ix))
            .relative()
            .w_full()
            .h(px(height))
            .flex()
            .flex_row()
            .bg(rgba(background))
            .border_b_1()
            .border_color(rgba(th.divider))
            .text_size(px(14.0))
            .cursor_pointer()
            .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                if *hovered {
                    this.hovered = Some(ix);
                } else if this.hovered == Some(ix) {
                    this.hovered = None;
                }
                cx.notify();
            }))
            .on_click(cx.listener(move |this, _, window, cx| this.open(ix, window, cx)))
            .child(Ripple::new(("row-ripple", ix), rgba(th.ripple)))
            // The shadow of the lifted row above, which this row would
            // otherwise paint over.
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(5.0))
                    .with_spring(
                        ("row-drop", ix),
                        SpringAnimation::new(motion::QUICK).to(if under_hovered {
                            1.0
                        } else {
                            0.0
                        }),
                        {
                            let shadow = th.shadow;
                            move |el, s: f32| {
                                el.bg(linear_gradient(
                                    180.0,
                                    linear_color_stop(rgba(fade(shadow, 0.55 * s)), 0.0),
                                    linear_color_stop(rgba(fade(shadow, 0.0)), 1.0),
                                ))
                            }
                        },
                    ),
            )
            // The keyboard cursor: a bar that grows from the middle.
            .child(
                div()
                    .absolute()
                    .left_0()
                    .w(px(3.0))
                    .rounded_r(px(2.0))
                    .bg(rgba(th.accent))
                    .with_spring(
                        ("row-cursor", ix),
                        SpringAnimation::new(motion::SLIDE).to(if cursor { 1.0 } else { 0.0 }),
                        move |el, s: f32| {
                            let s = s.clamp(0.0, 1.0);
                            el.top(px(height / 2.0 * (1.0 - s))).h(px(height * s))
                        },
                    ),
            );
        let lifted = |base: Stateful<Div>| {
            let shadow = th.shadow;
            base.with_spring(
                ("row-lift", ix),
                SpringAnimation::new(motion::QUICK).to(if hovered { 1.0 } else { 0.0 }),
                move |el, s: f32| {
                    if s > 0.001 {
                        el.shadow(vec![
                            BoxShadow {
                                color: rgba(fade(shadow, 0.9 * s)).into(),
                                offset: point(px(0.0), px(1.0)),
                                blur_radius: px(2.0),
                                spread_radius: px(0.0),
                                inset: false,
                            },
                            BoxShadow {
                                color: rgba(fade(shadow, 0.45 * s)).into(),
                                offset: point(px(0.0), px(1.0)),
                                blur_radius: px(3.0),
                                spread_radius: px(1.0),
                                inset: false,
                            },
                        ])
                    } else {
                        el
                    }
                },
            )
            .into_any_element()
        };
        let Some(row) = row else {
            return lifted(
                base.items_center()
                    .pl(px(96.0))
                    .text_color(rgba(th.text_faint))
                    .child("This message was removed."),
            );
        };
        let now = jiff::Timestamp::now().as_second();
        let date = row
            .date
            .and_then(|d| format::local(d, &self.tz))
            .zip(format::local(now, &self.tz))
            .map(|(d, now)| format::list_date(d, now))
            .unwrap_or_default();
        let weight = if row.unread {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        };
        let check = div()
            .id(("row-check", ix))
            .size(px(32.0))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                if !this.checked.remove(&key) {
                    this.checked.insert(key);
                }
                this.checked_all = false;
                cx.notify();
            }))
            .child(if checked {
                icon("checkbox-checked", th.text, 20.0)
            } else {
                icon("checkbox", th.text_faint, 20.0)
            });
        let flagged = row.flagged;
        let star = div()
            .id(("row-star", ix))
            .size(px(32.0))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.act(Act::Star(!flagged), vec![key], cx);
            }))
            .child(if row.flagged {
                icon("star-filled", th.star, 20.0)
            } else {
                icon("star", th.text_faint, 20.0)
            });
        let correspondent = div()
            .flex()
            .flex_row()
            .min_w_0()
            .gap(px(4.0))
            .text_color(rgba(th.text))
            .child(
                div()
                    .min_w_0()
                    .truncate()
                    .font_weight(weight)
                    .child(row.correspondent.clone()),
            )
            .when(row.count > 1, |d| {
                d.child(
                    div()
                        .flex_none()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(row.count.to_string()),
                )
            });
        let actions = hovered.then(|| self.hover_actions(ix, key, row.unread, th, cx));
        let date = div()
            .flex_none()
            .text_size(px(12.0))
            .font_weight(weight)
            .text_color(rgba(if row.unread { th.text } else { th.text_faint }))
            .child(date);

        if self.stacked() {
            let line = |child: AnyElement| {
                div()
                    .h(px((height - 16.0) / 3.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(child)
            };
            let subject = div()
                .flex_1()
                .min_w_0()
                .truncate()
                .font_weight(weight)
                .text_color(rgba(th.text))
                .child(row.subject.clone())
                .into_any_element();
            let snippet = div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_color(rgba(th.text_faint))
                .child(row.snippet.clone())
                .into_any_element();
            return lifted(
                base.py(px(8.0))
                    .child(
                        div()
                            .w(px(44.0))
                            .flex_none()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(check),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .pr(px(12.0))
                            .flex()
                            .flex_col()
                            .child(
                                line(div().flex_1().min_w_0().child(correspondent).into_any_element())
                                    .children(actions.or(Some(date.into_any_element()))),
                            )
                            .child(line(subject).child(star))
                            .child(line(snippet).when(row.attachments, |d| {
                                d.child(icon("attachment", th.text_faint, 16.0))
                            })),
                    ),
            );
        }

        let mut text = row.subject.clone();
        let subject_end = text.len();
        if !row.snippet.is_empty() {
            text.push_str(" - ");
            text.push_str(&row.snippet);
        }
        let text_len = text.len();
        let text = StyledText::new(text).with_highlights(vec![
            (
                0..subject_end,
                HighlightStyle {
                    color: Some(rgba(th.text).into()),
                    font_weight: Some(weight),
                    ..Default::default()
                },
            ),
            (
                subject_end..text_len,
                HighlightStyle {
                    color: Some(rgba(th.text_faint).into()),
                    ..Default::default()
                },
            ),
        ]);
        let wide = self.list_width() > 1000.0;
        lifted(
            base.items_center()
                .pl(px(8.0))
                .child(check)
                .child(star)
                .child(
                    div()
                        .w(px(if wide { 200.0 } else { 150.0 }))
                        .flex_none()
                        .pl(px(8.0))
                        .pr(px(24.0))
                        .child(correspondent),
                )
                .child(div().flex_1().min_w_0().truncate().child(text))
                .when(row.attachments, |d| {
                    d.child(
                        div()
                            .pl(px(8.0))
                            .child(icon("attachment", th.text_faint, 18.0)),
                    )
                })
                .child(
                    div()
                        .flex_none()
                        .min_w(px(96.0))
                        .pl(px(16.0))
                        .pr(px(12.0))
                        .flex()
                        .justify_end()
                        .children(actions.or(Some(date.into_any_element()))),
                ),
        )
    }

    /// Archive, delete and read/unread buttons shown on the hovered line in
    /// place of its date.
    fn hover_actions(
        &self,
        ix: usize,
        key: EntryKey,
        unread: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let button = |id: usize, name: &str| {
            icon_button_colored(("row-action", ix * 4 + id), name, 18.0, th.text_dim, th)
                .size(px(32.0))
        };
        div()
            .flex()
            .flex_row()
            .items_center()
            .child(button(0, "archive").on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.act(Act::Archive, vec![key], cx);
            })))
            .child(button(1, "trash").on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.act(Act::Delete, vec![key], cx);
            })))
            .child(
                button(2, if unread { "mark-read" } else { "mail" }).on_click(cx.listener(
                    move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.act(Act::Read(unread), vec![key], cx);
                    },
                )),
            )
            .with_animation(
                ("row-actions", ix),
                Animation::new(Duration::from_millis(140)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element()
    }
}

/// A thin vertical line between toolbar groups.
pub(super) fn separator(th: &Theme) -> Div {
    div()
        .mx(px(6.0))
        .w(px(1.0))
        .h(px(20.0))
        .bg(rgba(th.divider))
}
