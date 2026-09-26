// SPDX-License-Identifier: GPL-3.0-or-later

//! How the window follows its width (`docs/ARCHITECTURE.md` §13.9). Wide,
//! it is the desktop layout. Tablet-sized, the folders fold into a drawer
//! over the list. Phone-sized, it becomes
//! the mobile webmail layout: a search pill with the menu and account
//! inside, the list edge to edge with sender pictures, a Compose button
//! floating at the bottom, the apps in a bar along the bottom, and an open
//! conversation sliding in over the list.
//!
//! Every change between layouts moves on springs, so nothing jumps: the
//! rail slides out as the bottom bar rises, the search box grows into the
//! pill, the cards' margins and corners melt away.

use gpui::{
    AnimationExt, AnyElement, Context, FontWeight, SpringAnimation, Window, div, prelude::*, px,
    rgba,
};
use katna_ui::Ripple;
use katna_ui::motion::{self, Spring, lerp};

use super::apps::{APP_RAIL_WIDTH, App as RailApp};
use super::{Compose, MailWindow, NAV_WIDTH, ToggleSettings};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{avatar, elevation, icon, tip};

/// Narrower windows use the phone layout.
pub(super) const PHONE_BELOW: f32 = 600.0;
/// Windows this wide and wider use the desktop layout.
pub(super) const DESKTOP_FROM: f32 = 1080.0;
/// A tablet-sized window shows the reading pane beside the list (with the
/// three-pane setting) from this width; narrower, the conversation slides
/// over the list as on a phone.
pub(super) const TABLET_SPLIT_FROM: f32 = 840.0;
/// A layout is left only this far past its threshold, so a window resized
/// right at a threshold does not flip back and forth.
const HYSTERESIS: f32 = 12.0;
/// The bar with the apps along the bottom of a phone-sized window.
pub(super) const BOTTOM_BAR_HEIGHT: f32 = 72.0;
/// A tablet narrower than this shows Compose as its pencil alone.
const COMPOSE_FOLD_BELOW: f32 = 760.0;
/// The top bar's Compose button with its margin, folded and whole.
const COMPOSE_FOLDED: f32 = 50.0;
const COMPOSE_ROOM: f32 = 148.0;
/// A phone's Compose button folds to its pencil once the list has scrolled
/// down this far in one go, and grows back after this far up.
const FAB_FOLD_AFTER: f32 = 24.0;
const FAB_UNFOLD_AFTER: f32 = 120.0;
/// Room for the word "Compose" on a phone's Compose button.
const FAB_LABEL_WIDTH: f32 = 80.0;
const FAB_SIZE: f32 = 56.0;
const FAB_RADIUS: f32 = 16.0;
/// A phone's navigation drawer leaves this much of the window beside it.
const DRAWER_MARGIN: f32 = 56.0;

/// The three layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Size {
    Phone,
    Tablet,
    Desktop,
}

impl Size {
    /// The layout for a window `width` wide, when it now shows `now`.
    pub(super) fn for_width(width: f32, now: Self) -> Self {
        let phone_below = if now == Self::Phone {
            PHONE_BELOW + HYSTERESIS
        } else {
            PHONE_BELOW - HYSTERESIS
        };
        let desktop_from = if now == Self::Desktop {
            DESKTOP_FROM - HYSTERESIS
        } else {
            DESKTOP_FROM + HYSTERESIS
        };
        if width < phone_below {
            Self::Phone
        } else if width >= desktop_from {
            Self::Desktop
        } else {
            Self::Tablet
        }
    }

    /// Whether the reading pane can sit beside the list.
    pub(super) fn splits(self, width: f32) -> bool {
        match self {
            Self::Phone => false,
            Self::Tablet => width >= TABLET_SPLIT_FROM,
            Self::Desktop => true,
        }
    }
}

/// The layout at this frame. The fractions run from 0 to 1 as the window
/// turns into that layout, so the parts can move between them.
#[derive(Debug, Clone, Copy)]
pub(super) struct Shape {
    pub size: Size,
    /// The width inside the window frame.
    pub width: f32,
    pub phone: f32,
    /// How much of the word "Compose" shows on the top bar's button.
    pub label: f32,
    pub desktop: f32,
    /// Where the conversation is when it slides over the list: 0 = the
    /// list, 1 = the conversation.
    pub page: f32,
    /// The room the window buttons take at the two ends of the top bar.
    pub room: (f32, f32),
}

impl Shape {
    pub(super) fn is_phone(&self) -> bool {
        self.size == Size::Phone
    }

    pub(super) fn is_desktop(&self) -> bool {
        self.size == Size::Desktop
    }

    /// The height the bottom bar takes. It sinks away while a
    /// conversation is open over the list.
    pub(super) fn bottom_bar(&self) -> f32 {
        BOTTOM_BAR_HEIGHT * self.phone * (1.0 - self.page.clamp(0.0, 1.0))
    }

    /// The width the app rail takes.
    pub(super) fn rail(&self) -> f32 {
        APP_RAIL_WIDTH * (1.0 - self.phone)
    }

    /// The margin around the cards, which a phone does without.
    pub(super) fn card_margin(&self) -> f32 {
        16.0 * (1.0 - self.phone)
    }

    /// How much of the word "Compose" the top bar's button shows: all of
    /// it on a desktop and a wide tablet, folding away as a tablet narrows.
    pub(super) fn compose_label(&self) -> f32 {
        self.label
    }

    /// The room the top bar's Compose button takes beside the menu button.
    pub(super) fn compose_room(&self) -> f32 {
        lerp(COMPOSE_FOLDED, COMPOSE_ROOM, self.compose_label())
    }

    pub(super) fn card_radius(&self) -> f32 {
        super::PANEL_RADIUS * (1.0 - self.phone)
    }

    /// How far a conversation's text is indented from the card's edge.
    pub(super) fn reader_indent(&self) -> f32 {
        lerp(72.0, 16.0, self.phone)
    }
}

/// The layout and its springs.
pub(super) struct Layout {
    /// `None` until the first frame, which takes its layout without motion.
    size: Option<Size>,
    phone: Spring,
    desktop: Spring,
    label: Spring,
    page: Spring,
    /// 0 = no drawer, 1 = the drawer is open over the dimmed window.
    scrim: Spring,
    /// The navigation drawer of a phone or tablet is open.
    pub drawer: bool,
    /// How much of the word "Compose" a phone's Compose button shows.
    fab_label: Spring,
    /// How far the list was scrolled last frame, and how far it has moved
    /// since it last turned (down positive, up negative).
    list_top: f32,
    list_run: f32,
    pub shape: Shape,
}

impl Layout {
    pub(super) fn new() -> Self {
        Self {
            size: None,
            phone: Spring::new(motion::SLIDE, 0.0),
            desktop: Spring::new(motion::SLIDE, 1.0),
            label: Spring::new(motion::SMOOTH, 1.0),
            page: Spring::new(motion::SLIDE, 0.0),
            scrim: Spring::new(motion::SMOOTH, 0.0),
            drawer: false,
            fab_label: Spring::new(motion::SMOOTH, 1.0),
            list_top: 0.0,
            list_run: 0.0,
            shape: Shape {
                size: Size::Desktop,
                width: 1280.0,
                phone: 0.0,
                label: 1.0,
                desktop: 1.0,
                page: 0.0,
                room: (0.0, 0.0),
            },
        }
    }
}

impl Layout {
    /// Folds a phone's Compose button to its pencil as the list scrolls
    /// down, and grows it back after a few steps up or at the top.
    fn fold_fab(&mut self, top: f32) {
        let moved = top - self.list_top;
        self.list_top = top;
        if top <= 1.0 {
            self.list_run = 0.0;
            self.fab_label.set(1.0);
            return;
        }
        if moved > 0.0 {
            self.list_run = self.list_run.max(0.0) + moved;
            if self.list_run >= FAB_FOLD_AFTER {
                self.fab_label.set(0.0);
            }
        } else if moved < 0.0 {
            self.list_run = self.list_run.min(0.0) + moved;
            if -self.list_run >= FAB_UNFOLD_AFTER {
                self.fab_label.set(1.0);
            }
        }
    }
}

impl MailWindow {
    /// Picks the layout for the window's width and advances its motion.
    /// Call first thing in each frame.
    pub(super) fn update_layout(
        &mut self,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) {
        let width = self.chrome.inner_width(window);
        let room = self.chrome.button_room(window, cx);
        let first = self.layout.size.is_none();
        let size = Size::for_width(width, self.layout.size.unwrap_or(Size::Desktop));
        if self.layout.size != Some(size) {
            self.layout.size = Some(size);
            if size == Size::Desktop {
                self.layout.drawer = false;
            }
        }
        let layout = &mut self.layout;
        layout
            .phone
            .set(if size == Size::Phone { 1.0 } else { 0.0 });
        layout
            .desktop
            .set(if size == Size::Desktop { 1.0 } else { 0.0 });
        let labelled = layout.label.target() > 0.5;
        let fold_below = if labelled {
            COMPOSE_FOLD_BELOW - HYSTERESIS
        } else {
            COMPOSE_FOLD_BELOW + HYSTERESIS
        };
        layout
            .label
            .set(if width >= fold_below { 1.0 } else { 0.0 });
        if first {
            layout.phone.snap(layout.phone.target());
            layout.desktop.snap(layout.desktop.target());
            layout.label.snap(layout.label.target());
        }
        let label = layout.label.tick(window, reduce).clamp(0.0, 1.0);
        let phone = layout.phone.tick(window, reduce).clamp(0.0, 1.0);
        let desktop = layout.desktop.tick(window, reduce).clamp(0.0, 1.0);
        // The shape's size decides `split` below, so it goes in first.
        layout.shape = Shape {
            size,
            width,
            phone,
            label,
            desktop,
            page: layout.shape.page,
            room,
        };
        let open = self.slides() && self.reading && self.reader.is_some();
        let layout = &mut self.layout;
        layout.page.set(if open { 1.0 } else { 0.0 });
        if first {
            layout.page.snap(layout.page.target());
        }
        layout.shape.page = layout.page.tick(window, reduce);
        layout.scrim.set(if layout.drawer { 1.0 } else { 0.0 });
        layout.scrim.tick(window, reduce);
        let top = -f32::from(self.list_state.scroll_px_offset_for_scrollbar().y);
        let layout = &mut self.layout;
        layout.fold_fab(top);
        if first {
            layout.fab_label.snap(layout.fab_label.target());
        }
        layout.fab_label.tick(window, reduce);
        if !self.slides() {
            // The next conversation opened slides in from the edge again.
            self.layout.page.snap(0.0);
        }
    }

    /// Whether an opened conversation slides in over the list: a phone, or
    /// a tablet too narrow for the reading pane.
    pub(super) fn slides(&self) -> bool {
        !self.layout.shape.is_desktop() && !self.split()
    }

    /// Whether the conversation has slid away, so it can be forgotten.
    pub(super) fn page_closed(&self) -> bool {
        self.layout.page.target() == 0.0 && self.layout.page.settled()
    }

    /// The folders stay open beside the list: only on a desktop.
    pub(super) fn nav_docked(&self) -> bool {
        self.nav_open && self.layout.shape.is_desktop()
    }

    pub(super) fn close_drawer(&mut self, cx: &mut Context<Self>) {
        if self.layout.drawer {
            self.layout.drawer = false;
            cx.notify();
        }
    }

    /// The app rail, sliding out to the left as the window turns into a
    /// phone.
    pub(super) fn render_rail_slot(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let shape = self.layout.shape;
        div()
            .flex_none()
            .h_full()
            .w(px(shape.rail()))
            .overflow_hidden()
            .child(
                div()
                    .w(px(APP_RAIL_WIDTH))
                    .h_full()
                    .ml(px(-APP_RAIL_WIDTH * shape.phone))
                    .opacity(1.0 - shape.phone)
                    .child(self.render_app_rail(th, cx)),
            )
            .into_any_element()
    }

    /// The apps along the bottom of a phone-sized window.
    pub(super) fn render_bottom_bar(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let shape = self.layout.shape;
        if shape.bottom_bar() <= 0.01 {
            return None;
        }
        let items = RailApp::ALL.into_iter().map(|app| {
            let on = self.app == app;
            div()
                .id(("bottom-app", app as usize))
                .flex_1()
                .min_w_0()
                .h(px(BOTTOM_BAR_HEIGHT))
                .pt(px(12.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .cursor_pointer()
                .group("bottom-app")
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.close_drawer(cx);
                    this.open_app(app, cx)
                }))
                .child(
                    div()
                        .relative()
                        .overflow_hidden()
                        .w(px(56.0))
                        .h(px(32.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .group_hover("bottom-app", |s| s.bg(rgba(th.hover)))
                        .child(
                            Ripple::new(("bottom-ripple", app as usize), rgba(th.ripple))
                                .centered(),
                        )
                        .child(icon(
                            app.icon(),
                            if on {
                                th.nav_selected_text
                            } else {
                                th.text_dim
                            },
                            22.0,
                        ))
                        .with_spring(
                            ("bottom-pill", app as usize),
                            SpringAnimation::new(motion::SLIDE).to(if on { 1.0 } else { 0.0 }),
                            {
                                let bg = th.nav_selected;
                                move |el, s: f32| {
                                    let s = s.clamp(0.0, 1.0);
                                    if s > 0.001 {
                                        el.bg(rgba(fade(bg, s))).w(px(32.0 + 24.0 * s))
                                    } else {
                                        el
                                    }
                                }
                            },
                        ),
                )
                .child(
                    div()
                        .max_w_full()
                        .truncate()
                        .text_size(px(12.0))
                        .font_weight(if on {
                            FontWeight::BOLD
                        } else {
                            FontWeight::MEDIUM
                        })
                        .text_color(rgba(if on { th.text } else { th.text_dim }))
                        .child(app.label()),
                )
        });
        // It rises from under the window's edge.
        Some(
            div()
                .flex_none()
                .w_full()
                .h(px(shape.bottom_bar()))
                .overflow_hidden()
                .bg(rgba(th.page))
                .child(
                    div()
                        .h(px(BOTTOM_BAR_HEIGHT))
                        .px(px(4.0))
                        .flex()
                        .flex_row()
                        .border_t_1()
                        .border_color(rgba(th.divider))
                        .children(items),
                )
                .into_any_element(),
        )
    }

    /// The Compose button floating over the list of a phone, above the
    /// bottom bar and any note at the bottom.
    pub(super) fn render_phone_fab(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let shape = self.layout.shape;
        let shown = shape.phone
            * (1.0 - shape.page.clamp(0.0, 1.0))
            * (1.0 - self.layout.scrim.value().clamp(0.0, 1.0));
        let compose_open = self.compose.is_some();
        if shown <= 0.001
            || compose_open
            || self.settings_open
            || self.app != RailApp::Mail
            || self.mail.is_err()
            || self.accounts.is_empty()
        {
            return None;
        }
        let snackbar = self
            .snackbar
            .as_ref()
            .map_or(0.0, |s| s.shown.value().clamp(0.0, 1.0));
        let label = self.layout.fab_label.value().clamp(0.0, 1.0);
        Some(
            div()
                .absolute()
                .right(px(16.0))
                .bottom(px(shape.bottom_bar()
                    + lerp(-24.0, 16.0, shown)
                    + 64.0 * snackbar))
                .opacity(shown)
                .child(
                    fab_button("phone-compose", th)
                        .w_auto()
                        .pl(px(16.0))
                        .pr(px(lerp(16.0, 20.0, label)))
                        .justify_start()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .shadow(elevation(th, 3.0))
                        .when(label < 0.5, |d| d.tooltip(tip("Compose", th)))
                        .on_click(
                            cx.listener(|this, _, window, cx| this.compose(&Compose, window, cx)),
                        )
                        .child(icon("compose", th.compose_text, 24.0))
                        .child(
                            div()
                                .pl(px(12.0 * label))
                                .max_w(px(FAB_LABEL_WIDTH * label))
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .opacity(label)
                                .child("Compose"),
                        ),
                )
                .into_any_element(),
        )
    }

    /// Dims the window under the open drawer of a phone or tablet; a click
    /// on it closes the drawer.
    pub(super) fn render_scrim(&self, width: f32, cx: &mut Context<Self>) -> Option<AnyElement> {
        let t = self.layout.scrim.value().clamp(0.0, 1.0);
        if t <= 0.001 {
            return None;
        }
        Some(
            div()
                .id("drawer-scrim")
                .absolute()
                .top_0()
                .bottom_0()
                .left_0()
                .w(px(width))
                .bg(rgba(fade(0x0000_0052, t)))
                .on_click(cx.listener(|this, _, _, cx| this.close_drawer(cx)))
                .into_any_element(),
        )
    }

    /// The drawer's width: a phone's leaves a strip of the window beside it.
    pub(super) fn drawer_width(&self) -> f32 {
        let shape = self.layout.shape;
        lerp(
            NAV_WIDTH,
            (shape.width - DRAWER_MARGIN).min(304.0),
            shape.phone,
        )
        .max(NAV_WIDTH)
    }

    /// The top of the drawer of a phone or tablet: the app's name and, on a
    /// phone, the inbox tabs.
    pub(super) fn render_drawer_head(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let shape = self.layout.shape;
        if shape.is_desktop() || !self.layout.drawer {
            return None;
        }
        let tabs = (shape.is_phone() && self.shows_tabs()).then(|| {
            let rows = self.tabs.iter().enumerate().map(|(ix, tab)| {
                let on = ix == self.tab;
                let tint = th.tabs[tab.color];
                let unread: u64 = tab
                    .categories
                    .iter()
                    .filter_map(|c| self.category_unread.get(c))
                    .sum();
                drawer_row(("drawer-tab", ix), on, th)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.layout.drawer = false;
                        this.open_tab(ix, cx);
                        cx.notify();
                    }))
                    .child(icon(tab.icon, if on { tint } else { th.text }, 20.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .pl(px(18.0))
                            .truncate()
                            .child(tab.label),
                    )
                    .when(unread > 0 && ix != 0, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .px(px(8.0))
                                .rounded_full()
                                .bg(rgba(tint))
                                .text_color(rgba(th.on_accent))
                                .text_size(px(11.0))
                                .line_height(px(18.0))
                                .child(format!("{} new", format::thousands(unread))),
                        )
                    })
            });
            div()
                .flex()
                .flex_col()
                .pb(px(8.0))
                .mb(px(8.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .children(rows)
        });
        Some(
            div()
                .flex_none()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(48.0))
                        .pl(px(26.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .text_size(px(20.0))
                        .text_color(rgba(th.text))
                        .child(icon("mail", th.accent, 24.0))
                        .child("Katna Mail"),
                )
                .children(tabs)
                .into_any_element(),
        )
    }

    /// The foot of a phone's drawer: Settings, which lives in the rail
    /// elsewhere.
    pub(super) fn render_drawer_foot(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.layout.shape.is_phone() || !self.layout.drawer {
            return None;
        }
        Some(
            div()
                .flex_none()
                .pt(px(8.0))
                .pb(px(8.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .child(
                    drawer_row("drawer-settings", self.settings_open, th)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.layout.drawer = false;
                            this.toggle_settings(&ToggleSettings, window, cx);
                        }))
                        .child(icon("settings", th.text, 20.0))
                        .child(div().pl(px(18.0)).child("Settings")),
                )
                .into_any_element(),
        )
    }

    /// The picture at the start of a line on a phone, which ticks the line
    /// when clicked, as the check box does elsewhere.
    pub(super) fn line_picture(
        &self,
        ix: usize,
        name: &str,
        address: &str,
        checked: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let key = self.entries.get(ix).map(|e| e.key);
        div()
            .id(("row-picture", ix))
            .size(px(40.0))
            .flex_none()
            .rounded_full()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                let Some(key) = key else {
                    return;
                };
                if !this.checked.remove(&key) {
                    this.checked.insert(key);
                }
                this.checked_all = false;
                this.page_pick = None;
                cx.notify();
            }))
            .child(if checked {
                div()
                    .size(px(40.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(th.accent))
                    .child(icon("check", th.on_accent, 22.0))
                    .into_any_element()
            } else {
                avatar(name, address, 40.0)
            })
            .into_any_element()
    }
}

/// A round-cornered square button in the Compose colors.
fn fab_button(id: &'static str, th: &Theme) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .flex_none()
        .size(px(FAB_SIZE))
        .flex()
        .flex_row()
        .items_center()
        .justify_center()
        .rounded(px(FAB_RADIUS))
        .bg(rgba(th.compose))
        .text_color(rgba(th.compose_text))
        .cursor_pointer()
        .shadow(elevation(th, 1.0))
        .hover(|s| s.shadow(elevation(th, 2.0)))
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .child(Ripple::new((id, 0_usize), rgba(th.ripple)).rounded(FAB_RADIUS))
}

/// A line of the drawer, shaped like the folder lines.
fn drawer_row(id: impl Into<gpui::ElementId>, on: bool, th: &Theme) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .h(px(40.0))
        .mr(px(16.0))
        .pl(px(26.0))
        .pr(px(12.0))
        .flex()
        .flex_row()
        .items_center()
        .rounded_r(px(20.0))
        .text_size(px(14.0))
        .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
        .when(on, |d| {
            d.bg(rgba(th.nav_selected)).font_weight(FontWeight::BOLD)
        })
        .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
        .cursor_pointer()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_pick_layouts() {
        for now in [Size::Phone, Size::Tablet, Size::Desktop] {
            assert_eq!(Size::for_width(360.0, now), Size::Phone);
            assert_eq!(Size::for_width(800.0, now), Size::Tablet);
            assert_eq!(Size::for_width(1280.0, now), Size::Desktop);
        }
    }

    #[test]
    fn a_threshold_needs_crossing_by_a_margin() {
        // Just under the phone threshold a tablet stays a tablet, and just
        // over it a phone stays a phone.
        assert_eq!(
            Size::for_width(PHONE_BELOW - 4.0, Size::Tablet),
            Size::Tablet
        );
        assert_eq!(Size::for_width(PHONE_BELOW + 4.0, Size::Phone), Size::Phone);
        assert_eq!(
            Size::for_width(PHONE_BELOW - 20.0, Size::Tablet),
            Size::Phone
        );
        assert_eq!(
            Size::for_width(PHONE_BELOW + 20.0, Size::Phone),
            Size::Tablet
        );
        assert_eq!(
            Size::for_width(DESKTOP_FROM + 4.0, Size::Tablet),
            Size::Tablet
        );
        assert_eq!(
            Size::for_width(DESKTOP_FROM - 4.0, Size::Desktop),
            Size::Desktop
        );
        assert_eq!(
            Size::for_width(DESKTOP_FROM + 20.0, Size::Tablet),
            Size::Desktop
        );
    }

    #[test]
    fn only_wide_windows_split() {
        assert!(!Size::Phone.splits(590.0));
        assert!(!Size::Tablet.splits(TABLET_SPLIT_FROM - 1.0));
        assert!(Size::Tablet.splits(TABLET_SPLIT_FROM));
        assert!(Size::Desktop.splits(1080.0));
    }

    #[test]
    fn compose_folds_scrolling_down_and_unfolds_after_a_few_steps_up() {
        let mut layout = Layout::new();
        let label = |l: &Layout| l.fab_label.target();
        layout.fold_fab(10.0);
        assert_eq!(label(&layout), 1.0, "a nudge keeps the word");
        layout.fold_fab(40.0);
        assert_eq!(label(&layout), 0.0);
        layout.fold_fab(400.0);
        layout.fold_fab(340.0);
        assert_eq!(label(&layout), 0.0, "one step up is not enough");
        layout.fold_fab(270.0);
        assert_eq!(label(&layout), 1.0);
        layout.fold_fab(300.0);
        assert_eq!(label(&layout), 0.0);
        layout.fold_fab(0.0);
        assert_eq!(label(&layout), 1.0, "the top of the list shows it");
    }
}
