// SPDX-License-Identifier: GPL-3.0-or-later

//! The list card: the toolbar (select, refresh, more; or actions on the
//! ticked lines), the inbox tabs and the lines, one row each, or three
//! stacked lines when the list is narrow.

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, BoxShadow, Context, Div, FontWeight, HighlightStyle,
    ListOffset, SharedString, SpringAnimation, SpringConfig, Stateful, StyledText, anchored,
    deferred, div, ease_out_quint, linear_color_stop, linear_gradient, list, point, prelude::*,
    relative, rgba,
};
use katna_core::config::Density;
use katna_i18n::tr;
use katna_ui::Ripple;
use katna_ui::motion;
use katna_ui::px;

/// The lift of the line under the pointer: critically damped and slower
/// than other hover feedback, so it rises and settles without a jolt.
const ROW_LIFT: SpringConfig = SpringConfig::new(500.0, 44.7, 1.0);
/// How strongly a tick box, star or marker that is off shows while the
/// pointer is not over its line.
const OFF_REST: f32 = 0.45;
/// How long the quick actions of a line take to fade in.
const ACTIONS_IN: Duration = Duration::from_millis(160);
/// The attachment chips under a line: their line's extra height, their
/// height, widest size and spacing, and the "+N" button's size.
const CHIPS_LINE: f32 = 40.0;
pub(super) const CHIP_HEIGHT: f32 = 30.0;
const CHIP_WIDTH: f32 = 184.0;
const CHIP_GAP: f32 = 8.0;
const MORE_SIZE: f32 = 30.0;

use super::attachments::kind_badge;
use super::reader::Squeeze;
use super::{Act, LIST_CONTEXT, Listing, MailWindow, Menu, READER_CONTEXT, Reload, STACKED_BELOW};
use crate::data::{EntryKey, Row, RowFile};
use crate::format;
use crate::sidebar::Role;
use crate::theme::{Theme, fade, mix};
use crate::widgets::{
    TOOLBAR_HEIGHT, card_outline, card_shadow, icon, icon_button, icon_button_colored, menu,
    menu_item, menu_item_icon, placeholder, tip, toolbar,
};

const TAB_HEIGHT: f32 = 56.0;
const TAB_MAX_WIDTH: f32 = 240.0;

/// What Read, Unread, Starred or Unstarred in the select menu ticked: the
/// matching lines on screen, then, from the banner's link, every matching
/// line of the list, as Gmail does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Picked {
    pub pick: Pick,
    /// Matching lines on screen, ticked first.
    pub on_screen: usize,
    /// Every matching line of the list, loaded or not, for the link.
    pub all: Vec<EntryKey>,
    /// `all` is ticked, not only the lines on screen.
    pub whole: bool,
}

impl Picked {
    /// How many lines are ticked while this pick still holds.
    fn ticked(&self) -> usize {
        if self.whole {
            self.all.len()
        } else {
            self.on_screen
        }
    }
}

/// What the select menu ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Pick {
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
        self.list_width_of(self.cards_target) < STACKED_BELOW
    }

    /// The list card's width once the reading pane is where it is going.
    pub(super) fn list_width(&self) -> f32 {
        self.list_width_of(self.cards_width)
    }

    /// The list card's width beside the reading pane in `cards` width.
    fn list_width_of(&self, cards: f32) -> f32 {
        let open = self.split() && self.reading && self.reader.is_some();
        if open {
            let pane = (cards - super::SPLIT_GAP) * self.config.mail.reading_pane_share;
            cards - pane - super::SPLIT_GAP
        } else {
            cards
        }
    }

    pub(super) fn render_list_card(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let two_pane_reading = !self.split() && self.reading;
        let inner = if self.slides() {
            self.render_sliding(th, cx)
        } else {
            let (toolbar, body) = if two_pane_reading {
                (
                    self.render_reader_toolbar(th, cx),
                    self.render_reader(th, cx),
                )
            } else {
                self.render_list_parts(th, cx)
            };
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(toolbar)
                .child(fade_in(body, self.card_seq))
                .into_any_element()
        };
        // Beside a conversation, the list keeps its own keys: Up and Down
        // move in it, and the conversation has the keys once it is clicked,
        // or Tab or Enter goes to it.
        let reading_context = self.reading && !self.split();
        let (radius, outline) = (
            self.layout.shape.card_radius(),
            self.layout.shape.card_outline(),
        );
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
            .relative()
            .rounded(px(radius))
            .overflow_hidden()
            .bg(rgba(th.surface))
            .shadow(card_shadow(th, outline))
            .p(px(outline))
            // GPUI clips to rectangles, so the lines stop short of the
            // rounded bottom corners rather than showing square ones.
            .pb(px(radius.max(outline)))
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
            .on_action(cx.listener(Self::add_to_tasks))
            .on_action(cx.listener(Self::mark_important))
            .on_action(cx.listener(Self::mark_not_important))
            .on_action(cx.listener(Self::toggle_check))
            .on_action(cx.listener(Self::open_context_menu_key))
            .child(inner)
            .children(card_outline(th, radius, outline));
        card.into_any_element()
    }

    /// The list's toolbar and its tabs, banner and lines.
    fn render_list_parts(
        &mut self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (AnyElement, AnyElement) {
        // A phone has the tabs in its drawer.
        let tabs =
            (self.shows_tabs() && !self.layout.shape.is_phone()).then(|| self.render_tabs(th, cx));
        let banner = self.render_select_banner(th, cx);
        let list = self.render_list(th, cx);
        // A phone's toolbar slides up out of sight as the list moves on.
        let toolbar = self.render_list_toolbar(th, cx);
        let rows = self.layout.shape.rows;
        let toolbar = if self.layout.shape.is_phone() && rows < 0.999 {
            div()
                .flex_none()
                .h(px(TOOLBAR_HEIGHT * rows))
                .overflow_hidden()
                .child(div().mt(px(-TOOLBAR_HEIGHT * (1.0 - rows))).child(toolbar))
                .into_any_element()
        } else {
            toolbar
        };
        (
            toolbar,
            div()
                .size_full()
                .flex()
                .flex_col()
                .children(tabs)
                .children(banner)
                .child(
                    div()
                        .relative()
                        .flex_1()
                        .min_h_0()
                        .child(list)
                        .child(self.tour_mark(super::tour::Spot::List)),
                )
                .into_any_element(),
        )
    }

    /// The list with the open conversation sliding in over it from the
    /// right, as on a phone; the list drifts left and dims beneath.
    fn render_sliding(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = self.layout.shape.page;
        let shown = page.clamp(0.0, 1.0);
        let has_reader = self.reader.is_some();
        let list = (shown < 0.999 || !has_reader).then(|| {
            let (toolbar, body) = self.render_list_parts(th, cx);
            div()
                .absolute()
                .top_0()
                .bottom_0()
                .left(relative(-0.25 * shown))
                .w_full()
                .flex()
                .flex_col()
                .child(toolbar)
                .child(fade_in(body, self.card_seq))
                .when(has_reader && shown > 0.001, |d| {
                    d.child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .bg(rgba(fade(th.shadow, 0.5 * shown))),
                    )
                })
        });
        let reader = (has_reader && page > 0.001).then(|| {
            div()
                .absolute()
                .top_0()
                .bottom_0()
                .left(relative(1.0 - page))
                .w_full()
                .flex()
                .flex_col()
                .bg(rgba(th.surface))
                .when(shown < 0.999, |d| {
                    d.shadow(crate::widgets::elevation(th, 2.0))
                })
                .child(self.render_reader_toolbar(th, cx))
                .child(div().flex_1().min_h_0().child(self.render_reader(th, cx)))
        });
        div()
            .relative()
            .size_full()
            .overflow_hidden()
            .children(list)
            .children(reader)
            .into_any_element()
    }

    // Toolbar

    fn render_list_toolbar(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let count = self.entries.len();
        let checked = self.checked.len();
        let phone = self.layout.shape.is_phone();
        if phone && checked == 0 {
            return self.render_phone_list_bar(th, cx);
        }
        let page_checked = checked > 0
            && (self.page_pick == Some(checked)
                || self.visible_keys().all(|k| self.checked.contains(&k)));
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
                    .tooltip(tip(tr!("list-select"), th))
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
            bar = bar.child(self.refresh_button("refresh", th, cx)).child({
                let more = icon_button("list-more", "more", 20.0, th)
                    .when(self.menu != Some(Menu::ListMore), |d| {
                        d.tooltip(tip(tr!("list-more"), th))
                    })
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::ListMore, cx)));
                self.with_menu(more, Menu::ListMore, th, cx)
            });
        } else {
            let any_unread = self.checked_rows().iter().any(|r| r.unread);
            let read_button = if any_unread {
                icon_button("mark-read", "mark-read", 20.0, th)
                    .tooltip(tip(tr!("list-mark-read"), th))
                    .on_click(
                        cx.listener(|this, _, _, cx| this.act_on_targets(Act::Read(true), cx)),
                    )
            } else {
                icon_button("mark-unread", "mail", 20.0, th)
                    .tooltip(tip(tr!("list-mark-unread"), th))
                    .on_click(
                        cx.listener(|this, _, _, cx| this.act_on_targets(Act::Read(false), cx)),
                    )
            };
            bar = bar
                .child(self.action_buttons("list", Squeeze::NONE, th, cx))
                .child(separator(th))
                .child(read_button)
                .child({
                    let move_to = icon_button("list-move", "move-to", 20.0, th)
                        .when(self.menu != Some(Menu::MoveTo), |d| {
                            d.tooltip(tip(tr!("list-move-to"), th))
                        })
                        .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::MoveTo, cx)));
                    self.with_menu(move_to, Menu::MoveTo, th, cx)
                })
                .child({
                    let more = icon_button("list-more", "more", 20.0, th)
                        .when(self.menu != Some(Menu::ListMore), |d| {
                            d.tooltip(tip(tr!("list-more"), th))
                        })
                        .on_click(
                            cx.listener(|this, _, _, cx| this.toggle_menu(Menu::ListMore, cx)),
                        );
                    self.with_menu(more, Menu::ListMore, th, cx)
                });
        }
        let label: Option<SharedString> = match (&self.search_error, &self.listing) {
            (Some(err), _) => Some(err.clone()),
            (
                None,
                Some(Listing::Search {
                    corrected: Some(corrected),
                    ..
                }),
            ) => Some(tr!("list-results-corrected", query = corrected.as_str()).into()),
            (None, Some(Listing::Search { query, .. })) => {
                Some(tr!("list-results", query = query.as_str()).into())
            }
            _ => None,
        };
        let range = if count == 0 {
            String::new()
        } else {
            let start = self.visible.start.min(count - 1) + 1;
            let end = self.visible.end.clamp(start, count);
            match &self.listing {
                Some(Listing::Search {
                    total: Some(total), ..
                }) if *total > count && !self.config.mail.conversations => tr!(
                    "list-range-about",
                    first = start as u64,
                    last = end as u64,
                    total = *total as u64
                ),
                _ => tr!(
                    "list-range",
                    first = start as u64,
                    last = end as u64,
                    total = count as u64
                ),
            }
        };
        let search_instead = match (&self.search_error, &self.listing) {
            (
                None,
                Some(Listing::Search {
                    query,
                    corrected: Some(_),
                    ..
                }),
            ) => Some(query.clone()),
            _ => None,
        };
        let at_top = self.visible.start == 0;
        let at_end = self.visible.end >= count;
        if phone {
            return bar.child(div().flex_1()).into_any_element();
        }
        bar.child(
            div()
                .pl(px(8.0))
                .flex_1()
                .min_w_0()
                .flex()
                .items_center()
                .gap(px(12.0))
                .text_size(px(14.0))
                .child(
                    div()
                        .flex_shrink(1.0)
                        .min_w_0()
                        .truncate()
                        .text_color(rgba(th.text_dim))
                        .children(label),
                )
                .children(search_instead.map(|query| {
                    div()
                        .id("search-instead")
                        .flex_shrink(1.0)
                        .min_w_0()
                        .truncate()
                        .cursor_pointer()
                        .text_color(rgba(th.accent))
                        .child(tr!("list-search-instead", query = query.as_str()))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.search_verbatim(query.clone(), cx);
                        }))
                })),
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
                .tooltip(tip(tr!("list-newer"), th))
                .when(at_top, |d| d.opacity(0.4))
                .on_click(cx.listener(|this, _, _, cx| {
                    let page = this.visible.len().max(1);
                    let ix = this.visible.start.saturating_sub(page);
                    this.scroll_list_to(ix);
                    cx.notify();
                })),
        )
        .child(
            icon_button("page-down", "chevron-right", 20.0, th)
                .tooltip(tip(tr!("list-older"), th))
                .when(at_end, |d| d.opacity(0.4))
                .on_click(cx.listener(|this, _, _, cx| {
                    let ix = this.visible.end.min(this.entries.len().saturating_sub(1));
                    this.scroll_list_to(ix);
                    cx.notify();
                })),
        )
        .into_any_element()
    }

    /// A phone's bar over the list: what the list shows, refresh and more.
    /// Ticking a line (on its picture) brings the actions.
    fn render_phone_list_bar(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let label: SharedString = match (&self.search_error, &self.listing) {
            (Some(err), _) => err.clone(),
            (None, Some(Listing::Search { query, .. })) => {
                tr!("list-results", query = query.as_str()).into()
            }
            _ if self.shows_tabs() => self
                .tabs
                .get(self.tab)
                .map_or_else(SharedString::default, |t| t.label().into()),
            _ => self.folder_name().unwrap_or_default().into(),
        };
        let more = icon_button("list-more", "more", 20.0, th)
            .when(self.menu != Some(Menu::ListMore), |d| {
                d.tooltip(tip(tr!("list-more"), th))
            })
            .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::ListMore, cx)));
        toolbar(th)
            .pl(px(16.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.text_dim))
                    .child(label),
            )
            .child(self.refresh_button("refresh", th, cx))
            .child(self.with_menu(more, Menu::ListMore, th, cx))
            .into_any_element()
    }

    /// Archive, spam and delete, for the ticked lines or the open
    /// conversation.
    /// Refresh, whose arrow turns while mail is being checked for.
    fn refresh_button(
        &self,
        id: &'static str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        if !self.checking_mail() {
            return icon_button(id, "refresh", 20.0, th)
                .tooltip(tip(tr!("list-refresh"), th))
                .on_click(cx.listener(|this, _, window, cx| this.reload(&Reload, window, cx)));
        }
        // Like `icon_button`, with the arrow turning.
        div()
            .id(id)
            .size(px(40.0))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(tip(tr!("list-checking"), th))
            .child(super::nav_menu::turning_arrow(
                "refresh-turning",
                th.text_dim,
                20.0,
            ))
    }

    /// Archive, Report spam and Delete, less those `squeeze` leaves to
    /// the More menu.
    pub(super) fn action_buttons(
        &self,
        prefix: &'static str,
        squeeze: Squeeze,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        div()
            .flex()
            .flex_row()
            .child(
                icon_button((prefix, 1_usize), "archive", 20.0, th)
                    .tooltip(tip(tr!("list-archive"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.act_on_targets(Act::Archive, cx))),
            )
            .when(!squeeze.spam, |d| {
                d.child(
                    icon_button((prefix, 2_usize), "junk", 20.0, th)
                        .tooltip(tip(self.spam_label(false), th))
                        .on_click(cx.listener(|this, _, _, cx| this.act_on_targets(Act::Spam, cx))),
                )
            })
            .when(!squeeze.delete, |d| {
                d.child(
                    icon_button((prefix, 3_usize), "trash", 20.0, th)
                        .tooltip(tip(tr!("list-delete"), th))
                        .on_click(
                            cx.listener(|this, _, _, cx| this.act_on_targets(Act::Delete, cx)),
                        ),
                )
            })
    }

    /// Tells the list its lines changed; `keep_scroll` stays at the same
    /// line, otherwise it goes back to the top.
    pub(super) fn reset_list(&mut self, keep_scroll: bool) {
        let top = self.list_state.logical_scroll_top();
        let count = self.entries.len();
        self.list_state
            .reset(count, px(self.row_height()), keep_scroll.then_some(top));
        self.files_menu = None;
    }

    /// Scrolls the list so line `ix` is at the top.
    pub(super) fn scroll_list_to(&self, ix: usize) {
        self.list_state.scroll_to(ListOffset {
            item_ix: ix,
            offset_in_item: px(0.0),
        });
    }

    /// Works out which lines the list showed in its last frame.
    fn update_visible(&mut self) {
        let count = self.entries.len();
        let start = self.list_state.logical_scroll_top().item_ix.min(count);
        let viewport = self.list_state.viewport_bounds();
        let mut end = start;
        while end < count {
            match self.list_state.bounds_for_item(end) {
                Some(bounds) if bounds.top() < viewport.bottom() => end += 1,
                Some(_) => break,
                None => {
                    // Not drawn yet: as many as fit at the usual height.
                    let fit = (viewport.size.height / px(self.row_height())).ceil() as usize;
                    end = end.max(start + fit.max(1)).min(count);
                    break;
                }
            }
        }
        self.visible = start..end;
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
                    // Just under the button (where the button's row puts
                    // it), moved back inside the window when it would run
                    // past an edge (a phone's narrow window).
                    deferred(
                        anchored()
                            .offset(point(px(0.0), px(4.0)))
                            .snap_to_window_with_margin(px(8.0))
                            .child(
                                div().occlude().child(
                                    items.with_animation(
                                        ("menu", which as usize),
                                        Animation::new(Duration::from_millis(160))
                                            .with_easing(ease_out_quint()),
                                        |el, t| el.opacity(t).mt(px(-6.0 * (1.0 - t))),
                                    ),
                                ),
                            ),
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
                    (Pick::All, tr!("list-pick-all")),
                    (Pick::None, tr!("list-pick-none")),
                    (Pick::Read, tr!("list-pick-read")),
                    (Pick::Unread, tr!("list-pick-unread")),
                    (Pick::Starred, tr!("list-pick-starred")),
                    (Pick::Unstarred, tr!("list-pick-unstarred")),
                ]
                .into_iter()
                .map(|(pick, label)| {
                    menu_item(("pick", pick as usize), &label, th)
                        .on_click(cx.listener(move |this, _, _, cx| this.pick(pick, cx)))
                }),
            ),
            Menu::ListMore | Menu::ReaderMore => {
                let squeeze = (which == Menu::ReaderMore).then(|| self.reader_squeeze(th));
                let targets = if which == Menu::ListMore && self.checked.is_empty() {
                    None
                } else {
                    Some(())
                };
                match targets {
                    None => menu(th).child(
                        menu_item_icon(
                            "mark-all-read",
                            "mark-read",
                            &tr!("menu-mark-all-read"),
                            th,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            let keys = this.entries.iter().map(|e| e.key).collect();
                            this.act(Act::Read(true), keys, cx);
                        })),
                    ),
                    Some(()) => menu(th)
                        // What a narrow reading pane leaves off its toolbar.
                        .when(squeeze.is_some_and(|s| s.spam), |d| {
                            d.child(
                                menu_item_icon("more-spam", "junk", &self.spam_label(true), th)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.act_on_targets(Act::Spam, cx)
                                    })),
                            )
                        })
                        .when(squeeze.is_some_and(|s| s.delete), |d| {
                            d.child(
                                menu_item_icon("more-delete", "trash", &tr!("menu-delete"), th)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.act_on_targets(Act::Delete, cx)
                                    })),
                            )
                        })
                        .when(squeeze.is_some_and(|s| s.move_to), |d| {
                            d.child(
                                menu_item_icon("more-move-to", "move-to", &tr!("menu-move-to"), th)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.menu = Some(Menu::MoveTo);
                                        cx.notify();
                                    })),
                            )
                        })
                        .child(
                            menu_item_icon("more-read", "mark-read", &tr!("menu-mark-read"), th)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.act_on_targets(Act::Read(true), cx)
                                })),
                        )
                        .child(
                            menu_item_icon("more-unread", "mail", &tr!("menu-mark-unread"), th)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.mark_unread(&super::MarkUnread, window, cx)
                                })),
                        )
                        .child(
                            menu_item_icon("more-star", "star", &tr!("menu-star"), th).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.act_on_targets(Act::Star(true), cx)
                                }),
                            ),
                        )
                        .child(
                            menu_item_icon("more-unstar", "star-filled", &tr!("menu-unstar"), th)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.act_on_targets(Act::Star(false), cx)
                                })),
                        )
                        .child(
                            menu_item_icon(
                                "more-important",
                                "important",
                                &tr!("menu-important"),
                                th,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.act_on_targets(Act::Important(true), cx)
                            })),
                        )
                        .child(
                            menu_item_icon(
                                "more-not-important",
                                "important-filled",
                                &tr!("menu-not-important"),
                                th,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.act_on_targets(Act::Important(false), cx)
                            })),
                        )
                        .child(
                            menu_item_icon(
                                "more-add-to-tasks",
                                "tasks",
                                &tr!("menu-add-to-tasks"),
                                th,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.menu = None;
                                let keys = this.target_keys();
                                this.add_to_tasks_from(keys, cx);
                            })),
                        )
                        .child(
                            menu_item_icon(
                                "more-schedule-meeting",
                                "calendar",
                                &tr!("menu-schedule-meeting"),
                                th,
                            )
                            .on_click(cx.listener(
                                |this, _, window, cx| {
                                    this.menu = None;
                                    let key = this.target_keys().first().copied();
                                    this.schedule_meeting_from(key, window, cx);
                                },
                            )),
                        )
                        .child(
                            menu_item_icon(
                                "more-start-call",
                                "video",
                                &tr!("menu-start-call"),
                                th,
                            )
                            .on_click(cx.listener(
                                |this, _, window, cx| {
                                    this.menu = None;
                                    let key = this.target_keys().first().copied();
                                    this.start_call_from(key, window, cx);
                                },
                            )),
                        )
                        .child(
                            menu_item_icon("more-add-note", "notes", &tr!("menu-add-note"), th)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.menu = None;
                                    let keys = this.target_keys();
                                    this.add_note_from(keys, window, cx);
                                })),
                        )
                        .child(
                            menu_item_icon("more-pin", "pin", &tr!("menu-pin"), th).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.act_on_targets(Act::Pin(true), cx)
                                }),
                            ),
                        )
                        .child(
                            menu_item_icon("more-unpin", "pin-filled", &tr!("menu-unpin"), th)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.act_on_targets(Act::Pin(false), cx)
                                })),
                        )
                        .when(which == Menu::ReaderMore, |d| {
                            d.child(div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider)))
                                .child(
                                    menu_item_icon(
                                        "more-print",
                                        "print",
                                        &tr!("menu-print-all"),
                                        th,
                                    )
                                    .on_click(cx.listener(
                                        |this, _, window, cx| {
                                            this.menu = None;
                                            this.print_conversation(window, cx);
                                        },
                                    )),
                                )
                                .when(!self.detached, |d| {
                                    d.child(
                                        menu_item_icon(
                                            "more-new-window",
                                            "open-external",
                                            &tr!("menu-new-window"),
                                            th,
                                        )
                                        .on_click(
                                            cx.listener(|this, _, _, cx| {
                                                this.menu = None;
                                                this.open_reader_in_window(cx);
                                            }),
                                        ),
                                    )
                                })
                                .when(
                                    squeeze.is_some_and(|s| s.contact)
                                        && self
                                            .contact_fits(self.cards_width + self.contact_room()),
                                    |d| {
                                        let on = self.config.mail.contact_panel;
                                        d.child(
                                            menu_item_icon(
                                                "more-contact",
                                                "contacts",
                                                &if on {
                                                    tr!("contact-panel-hide")
                                                } else {
                                                    tr!("contact-panel-show")
                                                },
                                                th,
                                            )
                                            .on_click(
                                                cx.listener(|this, _, _, cx| {
                                                    this.menu = None;
                                                    this.toggle_contact_panel(cx);
                                                }),
                                            ),
                                        )
                                    },
                                )
                                .when(
                                    squeeze.is_some_and(|s| s.colors)
                                        && self.original_colors_offered(th),
                                    |d| {
                                        let on =
                                            self.reader.as_ref().is_some_and(|r| r.original_colors);
                                        d.child(
                                            menu_item_icon(
                                                "more-colors",
                                                "contrast",
                                                &if on {
                                                    tr!("reader-dark-colors")
                                                } else {
                                                    tr!("reader-original-colors")
                                                },
                                                th,
                                            )
                                            .on_click(
                                                cx.listener(|this, _, _, cx| {
                                                    this.menu = None;
                                                    if let Some(reader) = &mut this.reader {
                                                        reader.original_colors =
                                                            !reader.original_colors;
                                                    }
                                                    cx.notify();
                                                }),
                                            ),
                                        )
                                    },
                                )
                        }),
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
                            .child(tr!("menu-move-to-heading")),
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
                                            .child(icon(
                                                super::nav::role_icon(role),
                                                th.text_dim,
                                                18.0,
                                            ))
                                            .child(name)
                                    }),
                            ),
                    )
            }
        }
    }

    /// Keys of the lines on screen.
    fn visible_keys(&self) -> impl Iterator<Item = EntryKey> + '_ {
        let range =
            self.visible.start.min(self.entries.len())..self.visible.end.min(self.entries.len());
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
        let folder = self.listed_folder();
        match &mut self.mail {
            Ok(mail) => mail
                .rows(&entries, folder, self.show_recipients)
                .into_iter()
                .flatten()
                .map(|r| self.with_pending(r))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// `row` with changes the daemon has not confirmed yet.
    pub(super) fn with_pending(&self, row: Rc<Row>) -> Rc<Row> {
        match self.pending.get(&row.key) {
            Some(p)
                if p.unread.is_some()
                    || p.flagged.is_some()
                    || p.important.is_some()
                    || p.pinned.is_some() =>
            {
                let mut row = (*row).clone();
                row.unread = p.unread.unwrap_or(row.unread);
                row.flagged = p.flagged.unwrap_or(row.flagged);
                row.important = p.important.unwrap_or(row.important);
                row.pinned = p.pinned.unwrap_or(row.pinned);
                Rc::new(row)
            }
            _ => row,
        }
    }

    fn pick(&mut self, pick: Pick, cx: &mut Context<Self>) {
        self.menu = None;
        self.checked_all = false;
        self.page_pick = None;
        self.picked = None;
        self.checked.clear();
        match pick {
            Pick::None => {}
            // The lines on screen; the banner offers the whole list.
            Pick::All => {
                let range = self.visible.start.min(self.entries.len())
                    ..self.visible.end.min(self.entries.len());
                self.checked
                    .extend(self.entries[range].iter().map(|e| e.key));
                if self.checked.len() < self.entries.len() {
                    self.page_pick = Some(self.checked.len());
                } else if !self.checked.is_empty() {
                    self.checked_all = true;
                }
            }
            // The matching lines on screen; the banner offers every
            // matching line of the list, loaded or not.
            Pick::Read | Pick::Unread | Pick::Starred | Pick::Unstarred => {
                let folder = self.listed_folder();
                let show_recipients = self.show_recipients;
                let marks = match &mut self.mail {
                    Ok(mail) => mail.marks(&self.entries, folder, show_recipients),
                    Err(_) => Vec::new(),
                };
                let visible = self.visible.start.min(self.entries.len())
                    ..self.visible.end.min(self.entries.len());
                let mut all = Vec::new();
                for (ix, (entry, marks)) in self.entries.iter().zip(marks).enumerate() {
                    let pending = self.pending.get(&entry.key);
                    let unread = pending.and_then(|p| p.unread).unwrap_or(marks.unread);
                    let flagged = pending.and_then(|p| p.flagged).unwrap_or(marks.flagged);
                    let take = match pick {
                        Pick::Read => !unread,
                        Pick::Unread => unread,
                        Pick::Starred => flagged,
                        _ => !flagged,
                    };
                    if take {
                        all.push(entry.key);
                        if visible.contains(&ix) {
                            self.checked.insert(entry.key);
                        }
                    }
                }
                if all.is_empty() {
                    self.show_snackbar(
                        pick_none_text(pick, self.config.mail.conversations),
                        None,
                        cx,
                    );
                } else {
                    // None on screen: nothing to offer beyond, so take
                    // them all at once.
                    let whole = self.checked.is_empty() || self.checked.len() == all.len();
                    if whole {
                        self.checked.extend(all.iter().copied());
                    }
                    self.picked = Some(Picked {
                        pick,
                        on_screen: self.checked.len(),
                        all,
                        whole,
                    });
                }
            }
        }
        cx.notify();
    }

    /// "All 20 on screen are selected. Select all 1,234" when every line on
    /// screen is ticked and there are more.
    fn render_select_banner(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let on_screen = self.checked.len();
        let page_checked = on_screen > 0 && self.page_pick == Some(on_screen);
        let picked = self
            .picked
            .as_ref()
            .filter(|p| p.ticked() > 0 && p.ticked() == on_screen);
        if !(self.checked_all || page_checked || picked.is_some()) {
            return None;
        }
        let kind = if self.config.mail.conversations {
            "conversation"
        } else {
            "message"
        };
        let folder = self.folder_name();
        let total = self.entries.len() as u64;
        let (text, link) = match (self.checked_all, folder) {
            (false, folder) if let Some(picked) = picked => {
                let pick = pick_name(picked.pick);
                let all = picked.all.len() as u64;
                match (picked.whole, folder) {
                    (true, Some(folder)) => (
                        tr!(
                            "list-selected-picked-in",
                            pick = pick,
                            count = all,
                            kind = kind,
                            folder = folder
                        ),
                        tr!("list-clear-selection"),
                    ),
                    (true, None) => (
                        tr!(
                            "list-selected-picked",
                            pick = pick,
                            count = all,
                            kind = kind
                        ),
                        tr!("list-clear-selection"),
                    ),
                    (false, folder) => (
                        tr!(
                            "list-selected-picked-screen",
                            pick = pick,
                            count = on_screen as u64,
                            kind = kind
                        ),
                        match folder {
                            Some(folder) => tr!(
                                "list-select-picked-in",
                                pick = pick,
                                count = all,
                                kind = kind,
                                folder = folder
                            ),
                            None => {
                                tr!("list-select-picked", pick = pick, count = all, kind = kind)
                            }
                        },
                    ),
                }
            }
            (true, Some(folder)) => (
                tr!(
                    "list-selected-all-in",
                    count = total,
                    kind = kind,
                    folder = folder
                ),
                tr!("list-clear-selection"),
            ),
            (true, None) => (
                tr!("list-selected-all", count = total, kind = kind),
                tr!("list-clear-selection"),
            ),
            (false, folder) => (
                tr!(
                    "list-selected-screen",
                    count = on_screen as u64,
                    kind = kind
                ),
                match folder {
                    Some(folder) => tr!(
                        "list-select-all-in",
                        count = total,
                        kind = kind,
                        folder = folder
                    ),
                    None => tr!("list-select-all", count = total, kind = kind),
                },
            ),
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
                            let ticked = this.checked.len();
                            let picked = this
                                .picked
                                .as_ref()
                                .filter(|p| p.ticked() == ticked)
                                .map(|p| p.whole);
                            if picked == Some(false)
                                && let Some(picked) = this.picked.as_mut()
                            {
                                // "Select all 2,000 unread conversations".
                                this.checked.extend(picked.all.iter().copied());
                                picked.whole = true;
                            } else if this.checked_all || picked.is_some() {
                                this.checked.clear();
                                this.checked_all = false;
                                this.page_pick = None;
                                this.picked = None;
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
        let count = self.tabs.len().max(1);
        let width = (self.list_width() / count as f32).min(TAB_MAX_WIDTH);
        let at = self.tab_spring.value();
        let selected = self.tab;
        let color = self
            .tabs
            .get(selected)
            .map_or(th.accent, |t| th.tabs[t.color]);
        let tabs = self.tabs.iter().enumerate().map(|(ix, tab)| {
            let on = ix == selected;
            let tint = th.tabs[tab.color];
            let unread: u64 = tab
                .categories
                .iter()
                .filter_map(|c| self.category_unread.get(c))
                .sum();
            let compact = width < 150.0;
            div()
                .id(("tab", ix))
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
                .on_click(cx.listener(move |this, _, _, cx| this.open_tab(ix, cx)))
                .child(Ripple::new(("tab-ripple", ix), rgba(th.ripple)).rounded(0.0))
                .child(icon(tab.icon, if on { tint } else { th.text_dim }, 20.0))
                .when(width >= 116.0, |d| {
                    d.child(
                        div()
                            .flex()
                            .flex_col()
                            .items_start()
                            .min_w_0()
                            .child(div().truncate().child(tab.label()))
                            .when(unread > 0 && !on && ix != 0, |d| {
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
                                        .child(tr!("tab-new", count = unread)),
                                )
                            }),
                    )
                })
                .when(width < 116.0, |d| d.justify_center().pl(px(8.0)))
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
            .child(self.tour_mark(super::tour::Spot::Tabs))
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

    fn render_list(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        if self.entries.is_empty() {
            let text = match &self.listing {
                Some(Listing::Search { .. }) => tr!("list-empty-search"),
                Some(Listing::Folder(_)) if self.first_sync => {
                    return first_sync_placeholder(th);
                }
                Some(Listing::Folder(_)) if self.shows_tabs() => match self.tabs.get(self.tab) {
                    Some(tab) => tr!("list-empty-tab", tab = tab.label()),
                    None => tr!("list-empty-tab-unknown"),
                },
                Some(Listing::Folder(_) | Listing::Unified { .. }) => match self.folder_name() {
                    Some(folder) => tr!("list-empty-folder", folder = folder),
                    None => tr!("list-empty-folder-unknown"),
                },
                None => String::new(),
            };
            return placeholder(&text, th);
        }
        self.update_visible();
        // Read the lines on show in one go; each line then finds its row.
        let folder = self.listed_folder();
        let ahead =
            self.visible.start.saturating_sub(10)..(self.visible.end + 30).min(self.entries.len());
        if let Ok(mail) = &mut self.mail {
            mail.rows(&self.entries[ahead], folder, self.show_recipients);
        }
        let shape = (self.stacked(), self.row_height().to_bits());
        if self.list_shape != shape {
            self.list_shape = shape;
            self.list_state.remeasure();
        }
        self.list_state.follow();
        list(
            self.list_state.state().clone(),
            cx.processor(|this, ix: usize, window, cx| {
                let ix = this.list_state.line(ix);
                let Some(entry) = this.entries.get(ix).copied() else {
                    return div().into_any_element();
                };
                let th = this.theme(window);
                let folder = this.listed_folder();
                let row = match &mut this.mail {
                    Ok(mail) => mail
                        .rows(&[entry], folder, this.show_recipients)
                        .pop()
                        .flatten(),
                    Err(_) => None,
                };
                let row = row.map(|r| this.with_pending(r));
                let row = this.render_row(ix, entry.key, row, &th, cx);
                this.fetch_pictures(cx);
                row
            }),
        )
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
        let line_height = self.row_height();
        let stacked = self.stacked();
        let has_chips = row.as_ref().is_some_and(|r| !r.files.is_empty());
        let height = line_height
            + match (has_chips, stacked) {
                (false, _) => 0.0,
                (true, false) => CHIPS_LINE,
                (true, true) => CHIPS_LINE - 4.0,
            };
        let hovered = self.hovered == Some(ix);
        let under_hovered = ix > 0 && self.hovered == Some(ix - 1);
        let cursor = self.selected == Some(ix);
        let checked = self.checked.contains(&key);
        let open = self.split() && self.reader.as_ref().is_some_and(|r| r.key == key);
        // The cursor is grey while the conversation beside has the keys.
        let keys_here = !self.reader_keys;
        let unread = row.as_ref().is_some_and(|r| r.unread);
        let background = if checked {
            th.checked_row
        } else if open {
            mix(th.surface, th.accent, if keys_here { 0.12 } else { 0.07 })
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
            .on_click(
                cx.listener(move |this, event: &gpui::ClickEvent, window, cx| {
                    // Shift+click opens the line in a window of its own;
                    // any other click opens it here.
                    if event.modifiers().shift {
                        this.open_in_window(ix, cx);
                    } else {
                        this.open(ix, window, cx);
                    }
                }),
            )
            .on_mouse_down(
                gpui::MouseButton::Right,
                cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                    this.open_context_menu(ix, key, event.position, cx)
                }),
            )
            .child(Ripple::new(("row-ripple", ix), rgba(th.ripple)).rounded(0.0))
            // The shadow of the lifted row above, which this row would
            // otherwise paint over.
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(4.0))
                    .with_spring(
                        ("row-drop", ix),
                        SpringAnimation::new(ROW_LIFT).to(if under_hovered { 1.0 } else { 0.0 }),
                        {
                            let shadow = th.shadow;
                            move |el, s: f32| {
                                el.bg(linear_gradient(
                                    180.0,
                                    linear_color_stop(
                                        rgba(fade(shadow, 0.3 * s.clamp(0.0, 1.0))),
                                        0.0,
                                    ),
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
                    .bg(rgba(if keys_here { th.accent } else { th.text_faint }))
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
            // The line takes a tint of the accent color; shadows barely
            // show on dark pages, so there it also lightens.
            let lit = if th.dark {
                mix(mix(background, 0xffffffff, 0.05), th.accent, 0.12)
            } else {
                mix(background, th.accent, 0.07)
            };
            base.with_spring(
                ("row-lift", ix),
                SpringAnimation::new(ROW_LIFT).to(if hovered { 1.0 } else { 0.0 }),
                move |el, s: f32| {
                    let s = s.clamp(0.0, 1.0);
                    if s > 0.001 {
                        el.bg(rgba(mix(background, lit, s))).shadow(vec![
                            BoxShadow {
                                color: rgba(fade(shadow, 0.5 * s)).into(),
                                offset: point(px(0.0), px(1.0)),
                                blur_radius: px(2.0),
                                spread_radius: px(0.0),
                                inset: false,
                            },
                            BoxShadow {
                                color: rgba(fade(shadow, 0.25 * s)).into(),
                                offset: point(px(0.0), px(1.0 * s)),
                                blur_radius: px(3.0),
                                spread_radius: px(0.0),
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
                    .child(tr!("row-removed")),
            );
        };
        let now = jiff::Timestamp::now().as_second();
        // Snoozed mail shows when it comes back instead.
        let date = row
            .snoozed_until
            .or(row.date)
            .and_then(|d| format::local(d, &self.tz))
            .zip(format::local(now, &self.tz))
            .map(|(d, now)| format::list_date(d, now))
            .unwrap_or_default();
        let weight = if row.unread {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        };
        // As in Gmail, a tick box, star or marker that is off rests dim and
        // comes up to full contrast while the pointer is over its line. A
        // phone has no pointer, so there they stay as they are.
        let rest = !hovered && !self.layout.shape.is_phone();
        let off = |id: &'static str, name: &'static str, size: f32| {
            div()
                .with_spring(
                    (id, ix),
                    SpringAnimation::new(ROW_LIFT).to(if rest { 0.0 } else { 1.0 }),
                    move |el, s: f32| el.opacity(OFF_REST + (1.0 - OFF_REST) * s.clamp(0.0, 1.0)),
                )
                .child(icon(name, th.text_dim, size))
                .into_any_element()
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
                this.page_pick = None;
                this.picked = None;
                cx.notify();
            }))
            .child(if checked {
                icon("checkbox-checked", th.text, 20.0)
            } else {
                off("row-check-rest", "checkbox", 20.0)
            });
        let flagged = row.flagged;
        let star = div()
            .id(("row-star", ix))
            .tooltip(tip(
                if row.flagged {
                    tr!("row-starred")
                } else {
                    tr!("row-not-starred")
                },
                th,
            ))
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
                off("row-star-rest", "star", 20.0)
            });
        let important = row.important;
        // Settings > Appearance > Important markers.
        let marker = self.config.mail.important_markers.then(|| {
            div()
                .id(("row-important", ix))
                .tooltip(tip(
                    if important {
                        tr!("row-important")
                    } else {
                        tr!("row-mark-important")
                    },
                    th,
                ))
                .size(px(32.0))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .hover(|s| s.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.act(Act::Important(!important), vec![key], cx);
                }))
                .child(if important {
                    icon("important-filled", th.important, 18.0)
                } else {
                    off("row-important-rest", "important", 18.0)
                })
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
        // The quick actions fade in over the date.
        let actions = hovered.then(|| {
            div()
                .child(self.hover_actions(ix, key, row.unread, row.pinned, th, cx))
                .with_animation(
                    ("row-actions", ix),
                    Animation::new(ACTIONS_IN).with_easing(ease_out_quint()),
                    |el, t| el.opacity(t),
                )
                .into_any_element()
        });
        let date = div()
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .text_size(px(12.0))
            .font_weight(weight)
            .whitespace_nowrap()
            .text_color(rgba(if row.unread { th.text } else { th.text_faint }))
            .when(row.pinned, |d| {
                d.child(
                    div()
                        .id(("row-pinned", ix))
                        .tooltip(tip(tr!("row-pinned"), th))
                        .child(icon("pin-filled", th.accent, 16.0)),
                )
            })
            .map(|d| match row.snoozed_until {
                Some(until) => d.text_color(rgba(th.accent)).child(
                    div()
                        .id(("row-snoozed", ix))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.0))
                        .tooltip(tip(
                            tr!(
                                "row-snoozed-until",
                                when = super::snooze::describe(until, &self.tz)
                            ),
                            th,
                        ))
                        .child(icon("schedule", th.accent, 16.0))
                        .child(date),
                ),
                None => d.child(date),
            });

        if stacked {
            let line_h = (line_height - 16.0) / 3.0;
            let line = |child: AnyElement| {
                div()
                    .h(px(line_h))
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
            // A phone shows the sender's picture, which ticks the line, and
            // keeps the star and Important marker on the right, as mobile
            // mail does; it has no hover toolbar to collide with them.
            // Wider, the tick, star and marker stand in a column on the left,
            // so the hover toolbar only ever covers the date.
            let phone = self.layout.shape.is_phone();
            let actions = if phone { None } else { actions };
            let lead_width = if phone { 68.0 } else { 44.0 };
            let (lead, side) = if phone {
                let lead = div()
                    .w(px(lead_width))
                    .flex_none()
                    .flex()
                    .flex_col()
                    .items_center()
                    .pt(px(2.0))
                    .child(self.line_picture(ix, &row.correspondent, &row.sender, checked, th, cx));
                (lead, Some((marker, star)))
            } else {
                // Each sits on one of the three lines, its hover circle
                // trimmed to the line so neighbors don't overlap.
                let size = (line_h + 4.0).min(32.0);
                let cell = |child: Stateful<Div>| {
                    div()
                        .h(px(line_h))
                        .flex()
                        .items_center()
                        .child(child.size(px(size)))
                };
                let lead = div()
                    .w(px(lead_width))
                    .flex_none()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(cell(check))
                    .child(cell(star))
                    .children(marker.map(cell));
                (lead, None)
            };
            return lifted(
                base.py(px(8.0)).child(lead).child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .pr(px(12.0))
                        .flex()
                        .flex_col()
                        .child(
                            line(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .child(correspondent)
                                    .into_any_element(),
                            )
                            .children(actions.or(Some(date.into_any_element()))),
                        )
                        .child(line(subject).children(side.map(|(marker, star)| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(8.0))
                                .children(marker)
                                .child(star)
                        })))
                        .child(
                            line(snippet)
                                .children(self.render_row_task(ix, key, th, cx))
                                .when(row.attachments && !has_chips, |d| {
                                    d.child(icon("attachment", th.text_faint, 16.0))
                                })
                                .children(tracking_mark(ix, &row, 16.0, th)),
                        )
                        .when(has_chips, |d| {
                            let room = self.list_width() - lead_width - 12.0;
                            d.child(self.file_chips(ix, &row, 0.0, room, th, cx))
                        }),
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
        let name_width = if wide { 200.0 } else { 150.0 };
        let first_line = div()
            .h(px(line_height))
            .flex_none()
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .pl(px(8.0))
            .child(check)
            .child(star)
            .children(marker)
            .child(
                div()
                    .w(px(name_width))
                    .flex_none()
                    .pl(px(8.0))
                    .pr(px(24.0))
                    .child(correspondent),
            )
            .child(div().flex_1().min_w_0().truncate().child(text))
            .children(
                self.render_row_task(ix, key, th, cx)
                    .map(|chip| div().pl(px(8.0)).child(chip)),
            )
            .when(row.attachments && !has_chips, |d| {
                d.child(
                    div()
                        .pl(px(8.0))
                        .child(icon("attachment", th.text_faint, 18.0)),
                )
            })
            .children(tracking_mark(ix, &row, 18.0, th).map(|mark| div().pl(px(8.0)).child(mark)))
            .child(
                div()
                    .flex_none()
                    .min_w(px(96.0))
                    .pl(px(16.0))
                    .pr(px(12.0))
                    .flex()
                    .justify_end()
                    .children(actions.or(Some(date.into_any_element()))),
            );
        // The chips line up under the subject.
        let chips_left = 8.0 + 3.0 * 32.0 + name_width;
        lifted(base.flex_col().child(first_line).when(has_chips, |d| {
            let room = self.list_width() - chips_left - 24.0;
            d.child(self.file_chips(ix, &row, chips_left, room, th, cx))
        }))
    }

    /// The attachment chips under a line: as many as fit (at most three),
    /// then a round "+N" button that lists the rest.
    fn file_chips(
        &self,
        ix: usize,
        row: &Row,
        left: f32,
        room: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (key, files) = (row.key, row.files.as_slice());
        let fit = |room: f32| ((room + CHIP_GAP) / (CHIP_WIDTH + CHIP_GAP)).floor() as usize;
        let mut shown = fit(room);
        if shown < files.len() {
            shown = fit(room - MORE_SIZE - CHIP_GAP);
        }
        let shown = shown.clamp(1, 3).min(files.len());
        let rest: Vec<RowFile> = files[shown..].to_vec();
        let chip = |n: usize, file: &RowFile| {
            let kind = katna_preview::kind(&file.mime, &file.name);
            let open = file.clone();
            let downloading = self.chip_downloading(file);
            div()
                .id(("row-file", ix * 4 + n))
                .relative()
                .h(px(CHIP_HEIGHT))
                .min_w(px(64.0))
                .max_w(px(CHIP_WIDTH))
                .flex_shrink(1.0)
                .flex()
                .items_center()
                .gap(px(8.0))
                .pl(px(8.0))
                .pr(px(14.0))
                .rounded_full()
                .border_1()
                .border_color(rgba(th.divider))
                .bg(rgba(th.surface))
                .when(!downloading, |d| {
                    d.cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .tooltip(tip(file.name.clone(), th))
                })
                .on_click(cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();
                    this.open_row_file(&open, window, cx);
                }))
                .children(self.chip_fill(file, true, th))
                .child(kind_badge(kind, 18.0))
                .child(
                    div()
                        .min_w_0()
                        .truncate()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(file.name.clone()),
                )
        };
        let chips: Vec<_> = files[..shown]
            .iter()
            .enumerate()
            .map(|(n, f)| chip(n, f))
            .collect();
        let more = (!rest.is_empty()).then(|| {
            let open = self.files_menu == Some(key);
            let names = rest
                .iter()
                .map(|f| f.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let button = div()
                .id(("row-more-files", ix))
                .size(px(MORE_SIZE))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .border_1()
                .border_color(rgba(th.divider))
                .bg(rgba(if open { th.hover } else { th.surface }))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("list-files-more", count = rest.len() as u64))
                .when(!open, |d| d.tooltip(tip(names, th)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.menu = None;
                    this.files_menu = if this.files_menu == Some(key) {
                        None
                    } else {
                        Some(key)
                    };
                    cx.notify();
                }));
            div()
                .relative()
                .child(button)
                .when(open, |d| d.children(self.files_popover(ix, &rest, th, cx)))
        });
        div()
            .h(px(CHIP_HEIGHT))
            .flex_none()
            .pl(px(left))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(CHIP_GAP))
            .children(chips)
            .children(more)
            .into_any_element()
    }

    /// The list the "+N" button opens: the attachments without a chip.
    fn files_popover(
        &self,
        ix: usize,
        files: &[RowFile],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> [AnyElement; 2] {
        let scrim = deferred(
            div()
                .id("files-scrim")
                .absolute()
                .top(px(-2000.0))
                .left(px(-4000.0))
                .w(px(8000.0))
                .h(px(6000.0))
                .occlude()
                .on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.files_menu = None;
                        cx.notify();
                    }),
                ),
        )
        .with_priority(1)
        .into_any_element();
        let items = files.iter().enumerate().map(|(n, file)| {
            let kind = katna_preview::kind(&file.mime, &file.name);
            let open = file.clone();
            let downloading = self.chip_downloading(file);
            div()
                .id(("files-item", n))
                .relative()
                .h(px(40.0))
                .px(px(16.0))
                .flex()
                .items_center()
                .gap(px(12.0))
                .when(!downloading, |d| {
                    d.cursor_pointer().hover(|s| s.bg(rgba(th.hover)))
                })
                .on_click(cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();
                    this.open_row_file(&open, window, cx);
                }))
                .children(self.chip_fill(file, false, th))
                .child(kind_badge(kind, 20.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_color(rgba(th.text))
                        .child(file.name.clone()),
                )
                .child(
                    div()
                        .flex_none()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(format::size(file.size)),
                )
        });
        let popover = deferred(
            anchored()
                .offset(point(px(0.0), px(6.0)))
                .snap_to_window_with_margin(px(8.0))
                .child(
                    menu(th)
                        .id(("files-menu", ix))
                        .w(px(320.0))
                        .max_h(px(320.0))
                        .overflow_y_scroll()
                        .occlude()
                        .text_size(px(14.0))
                        .children(items)
                        .with_animation(
                            ("files-menu", ix),
                            Animation::new(Duration::from_millis(160))
                                .with_easing(ease_out_quint()),
                            |el, t| el.opacity(t).mt(px(-6.0 * (1.0 - t))),
                        ),
                ),
        )
        .with_priority(2)
        .into_any_element();
        [scrim, popover]
    }

    /// Archive, delete, read/unread, pin and snooze buttons shown on the
    /// hovered line in place of its date.
    fn hover_actions(
        &self,
        ix: usize,
        key: EntryKey,
        unread: bool,
        pinned: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let button = |id: usize, name: &str, label: String| {
            icon_button_colored(("row-action", ix * 5 + id), name, 18.0, th.text_dim, th)
                .size(px(32.0))
                .tooltip(tip(label, th))
        };
        div()
            .flex()
            .flex_row()
            .items_center()
            .child(
                button(0, "archive", tr!("list-archive")).on_click(cx.listener(
                    move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.act(Act::Archive, vec![key], cx);
                    },
                )),
            )
            .child(button(1, "trash", tr!("list-delete")).on_click(cx.listener(
                move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.act(Act::Delete, vec![key], cx);
                },
            )))
            .child(
                button(
                    2,
                    if unread { "mark-read" } else { "mail" },
                    if unread {
                        tr!("list-mark-read")
                    } else {
                        tr!("list-mark-unread")
                    },
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.act(Act::Read(unread), vec![key], cx);
                })),
            )
            .child(
                button(
                    3,
                    if pinned { "pin-filled" } else { "pin" },
                    if pinned {
                        tr!("row-unpin")
                    } else {
                        tr!("row-pin")
                    },
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.act(Act::Pin(!pinned), vec![key], cx);
                })),
            )
            .map(|d| match self.folder_role() {
                Role::Snoozed => d.child(button(4, "inbox", tr!("list-unsnooze")).on_click(
                    cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.act(Act::Unsnooze, vec![key], cx);
                    }),
                )),
                Role::Drafts | Role::Sent | Role::Trash | Role::Junk => d,
                _ => d.child(
                    button(4, "schedule", tr!("list-snooze")).on_click(cx.listener(
                        move |this, event: &gpui::ClickEvent, _, cx| {
                            cx.stop_propagation();
                            this.open_snooze_menu(vec![key], event.position(), cx);
                        },
                    )),
                ),
            })
            .with_animation(
                ("row-actions", ix),
                Animation::new(Duration::from_millis(140)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element()
    }
}

/// Fades `body` in whenever `seq` changes: the card switched content.
fn fade_in(body: AnyElement, seq: usize) -> AnyElement {
    div()
        .flex_1()
        .min_h_0()
        .child(body)
        .with_animation(
            ("card", seq),
            Animation::new(Duration::from_millis(220)).with_easing(ease_out_quint()),
            // From half-drawn, so the card never shows a blank frame.
            |el, t| el.opacity(0.5 + 0.5 * t),
        )
        .into_any_element()
}

/// A thin vertical line between toolbar groups.
pub(super) fn separator(th: &Theme) -> Div {
    div()
        .mx(px(6.0))
        .w(px(1.0))
        .h(px(20.0))
        .bg(rgba(th.divider))
}

/// An empty folder while the first sync runs: the mail is on its way.
fn first_sync_placeholder(th: &Theme) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(12.0))
        .p(px(24.0))
        .child(
            div()
                .w(px(160.0))
                .h(px(4.0))
                .rounded_full()
                .overflow_hidden()
                .relative()
                .bg(rgba(fade(th.accent, 0.24)))
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .h_full()
                        .w(px(64.0))
                        .rounded_full()
                        .bg(rgba(th.accent))
                        .with_animation(
                            "first-sync",
                            Animation::new(Duration::from_millis(1300)).repeat(),
                            |bar, t| bar.left(px(-64.0 + 224.0 * t)),
                        ),
                ),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("list-first-sync")),
        )
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgba(th.text_faint))
                .child(tr!("list-first-sync-detail")),
        )
        .into_any_element()
}

/// The select menu's choice as the messages about it name it.
fn pick_name(pick: Pick) -> &'static str {
    match pick {
        Pick::Read => "read",
        Pick::Unread => "unread",
        Pick::Starred => "starred",
        Pick::Unstarred => "unstarred",
        Pick::All | Pick::None => "other",
    }
}

/// "No unread conversations here", when a pick matches nothing.
fn pick_none_text(pick: Pick, conversations: bool) -> String {
    let kind = if conversations {
        "conversation"
    } else {
        "message"
    };
    tr!("list-picked-none", pick = pick_name(pick), kind = kind)
}

/// The eye on a line of mail sent with open and click tracking: in the
/// accent color once a recipient opened it, with who did in its tooltip.
fn tracking_mark(ix: usize, row: &Row, size: f32, th: &Theme) -> Option<AnyElement> {
    let tracked = row.tracking?;
    let text = if tracked.clicked > 0 {
        tr!(
            "row-tracking-clicked",
            opened = tracked.opened,
            recipients = tracked.recipients,
            clicked = tracked.clicked
        )
    } else if tracked.opened > 0 {
        tr!(
            "row-tracking-opened",
            opened = tracked.opened,
            recipients = tracked.recipients
        )
    } else {
        tr!("row-tracking-none")
    };
    let color = if tracked.opened > 0 {
        th.accent
    } else {
        th.text_faint
    };
    Some(
        div()
            .id(("row-tracking", ix))
            .flex_none()
            .child(icon("eye", color, size))
            .tooltip(tip(text, th))
            .into_any_element(),
    )
}
