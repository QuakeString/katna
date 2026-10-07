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
    AnimationExt, AnyElement, Context, Decorations, FontWeight, SpringAnimation, Window, div,
    prelude::*, rgba,
};
use katna_ui::Ripple;
use katna_ui::WindowDrag;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::unpx;

use super::apps::{APP_RAIL_WIDTH, App as RailApp};
use super::{MailWindow, NAV_ROW_INSET, NAV_WIDTH, ToggleSettings};
use crate::theme::{Theme, fade};
use crate::widgets::{TOOLBAR_HEIGHT, elevation, icon, tip};

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
/// A tablet narrower than this shows the Katna mark alone at the top left,
/// without the app's name.
const TITLE_FOLD_BELOW: f32 = 760.0;
/// A phone's Compose button folds to its pencil once the list has scrolled
/// down this far in one go, and grows back after this far up.
const FAB_FOLD_AFTER: f32 = 24.0;
const FAB_UNFOLD_AFTER: f32 = 120.0;
/// A phone's search row and list toolbar come back once the list has
/// turned back up this far.
const ROWS_RETURN_AFTER: f32 = 24.0;
/// Room for the word on a phone's big button ("Compose", "New contact").
const FAB_LABEL_WIDTH: f32 = 120.0;
/// The least the list scrolls down before its Back to top button shows.
const TO_TOP_AFTER: f32 = 240.0;
/// From further down than this many screens, the list jumps to that many
/// screens from its top and glides the rest of the way.
const GLIDE_SCREENS: f32 = 2.0;
pub(super) const FAB_SIZE: f32 = 56.0;
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
    /// How much of the app's name shows beside the mark on the top bar.
    pub label: f32,
    /// Where the conversation is when it slides over the list: 0 = the
    /// list, 1 = the conversation.
    pub page: f32,
    /// The room the window buttons take at the two ends of the top bar.
    pub room: (f32, f32),
    /// How far the rail has gone because only Mail is on and its folders
    /// sit beside the list: nothing to switch to, and Compose heads the
    /// folders. 1 = gone.
    pub rail_gone: f32,
    /// How far the phone's bottom bar has gone because only Mail is on.
    pub solo: f32,
    /// How much of a phone's search row and list toolbar shows: they slide
    /// away as the list moves on. 1 = all of them.
    pub rows: f32,
    /// The top bar slides away with the rows. Not when it holds the
    /// window's own buttons (Katna's window frame).
    pub top_bar_slides: bool,
}

impl Shape {
    /// How far a phone's top bar has slid up out of the window.
    pub(super) fn top_bar_hidden(&self) -> f32 {
        if self.top_bar_slides {
            super::TOP_BAR_HEIGHT * (1.0 - self.rows)
        } else {
            0.0
        }
    }

    pub(super) fn is_phone(&self) -> bool {
        self.size == Size::Phone
    }

    pub(super) fn is_desktop(&self) -> bool {
        self.size == Size::Desktop
    }

    /// The height the bottom bar takes. It sinks away while a
    /// conversation is open over the list.
    pub(super) fn bottom_bar(&self) -> f32 {
        BOTTOM_BAR_HEIGHT * self.phone * (1.0 - self.page.clamp(0.0, 1.0)) * (1.0 - self.solo)
    }

    /// The width the app rail takes.
    pub(super) fn rail(&self) -> f32 {
        APP_RAIL_WIDTH * (1.0 - self.phone) * (1.0 - self.rail_gone)
    }

    /// The margin around the cards, which a phone does without.
    pub(super) fn card_margin(&self) -> f32 {
        super::CARD_GAP * (1.0 - self.phone)
    }

    /// How much of the app's name the top bar shows: all of it on a
    /// desktop and a wide tablet, folding away as a tablet narrows.
    pub(super) fn title_label(&self) -> f32 {
        self.label
    }

    /// How much of the faint line around the cards shows: none on a
    /// phone, whose cards run edge to edge.
    pub(super) fn card_outline(&self) -> f32 {
        1.0 - self.phone
    }

    pub(super) fn card_radius(&self) -> f32 {
        super::PANEL_RADIUS * (1.0 - self.phone)
    }
}

/// The layout and its springs.
pub(super) struct Layout {
    /// `None` until the first frame, which takes its layout without motion.
    size: Option<Size>,
    phone: Spring,
    label: Spring,
    page: Spring,
    /// 0 = no drawer, 1 = the drawer is open over the dimmed window.
    scrim: Spring,
    /// The rail going when only Mail is on ([`Shape::rail_gone`]).
    rail_gone: Spring,
    /// The bottom bar going when only Mail is on ([`Shape::solo`]).
    solo: Spring,
    /// The navigation drawer of a phone or tablet is open.
    pub drawer: bool,
    /// How much of the word "Compose" a phone's Compose button shows.
    fab_label: Spring,
    /// How much of a phone's search row and list toolbar shows.
    rows: Spring,
    /// How far the list was scrolled last frame, and how far it has moved
    /// since it last turned (down positive, up negative).
    list_top: f32,
    list_run: f32,
    /// How much the Back to top button shows.
    to_top: Spring,
    /// The list gliding back to its top: where it is on the way, in pixels
    /// from the top, and where it was last frame.
    glide: Option<(Spring, f32)>,
    pub shape: Shape,
}

impl Layout {
    pub(super) fn new() -> Self {
        Self {
            size: None,
            phone: Spring::new(motion::SLIDE, 0.0),
            label: Spring::new(motion::SMOOTH, 1.0),
            page: Spring::new(motion::SLIDE, 0.0),
            scrim: Spring::new(motion::SMOOTH, 0.0),
            rail_gone: Spring::new(motion::SLIDE, 0.0),
            solo: Spring::new(motion::SLIDE, 0.0),
            drawer: false,
            fab_label: Spring::new(motion::SMOOTH, 1.0),
            rows: Spring::new(motion::SMOOTH, 1.0),
            list_top: 0.0,
            list_run: 0.0,
            to_top: Spring::new(motion::SMOOTH, 0.0),
            glide: None,
            shape: Shape {
                size: Size::Desktop,
                width: 1280.0,
                phone: 0.0,
                label: 1.0,
                page: 0.0,
                room: (0.0, 0.0),
                rail_gone: 0.0,
                solo: 0.0,
                rows: 1.0,
                top_bar_slides: false,
            },
        }
    }
}

impl Layout {
    /// How far a phone's or tablet's drawer is open, 0 to 1.
    pub(super) fn drawer_t(&self) -> f32 {
        self.scrim.value().clamp(0.0, 1.0)
    }

    /// How much the list's Back to top button shows, 0 to 1.
    pub(super) fn to_top_t(&self) -> f32 {
        self.to_top.value().clamp(0.0, 1.0)
    }

    /// The list stops gliding to its top: the wheel took it.
    pub(super) fn stop_glide(&mut self) {
        self.glide = None;
    }

    /// Shows the Back to top button once the list is more than a screen
    /// down, and hides it again within half a screen of the top.
    fn show_to_top(&mut self, top: f32, screen: f32) {
        let screen = screen.max(TO_TOP_AFTER);
        if top > screen {
            self.to_top.set(1.0);
        } else if top < screen * 0.5 {
            self.to_top.set(0.0);
        }
    }

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
        let room = self
            .chrome
            .button_room(Some(super::TOP_BAR_HEIGHT), window, cx);
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
        let labelled = layout.label.target() > 0.5;
        let fold_below = if labelled {
            TITLE_FOLD_BELOW - HYSTERESIS
        } else {
            TITLE_FOLD_BELOW + HYSTERESIS
        };
        layout
            .label
            .set(if width >= fold_below { 1.0 } else { 0.0 });
        if first {
            layout.phone.snap(layout.phone.target());
            layout.label.snap(layout.label.target());
        }
        let label = layout.label.tick(window, reduce).clamp(0.0, 1.0);
        let phone = layout.phone.tick(window, reduce).clamp(0.0, 1.0);
        let solo = self.mail_only();
        let rail_gone =
            solo && self.nav_open && size == Size::Desktop && self.settings_page.is_none();
        let layout = &mut self.layout;
        layout.rail_gone.set(if rail_gone { 1.0 } else { 0.0 });
        layout.solo.set(if solo { 1.0 } else { 0.0 });
        if first {
            layout.rail_gone.snap(layout.rail_gone.target());
            layout.solo.snap(layout.solo.target());
        }
        let rail_gone = layout.rail_gone.tick(window, reduce).clamp(0.0, 1.0);
        let solo = layout.solo.tick(window, reduce).clamp(0.0, 1.0);
        // The shape's size decides `split` below, so it goes in first.
        layout.shape = Shape {
            size,
            width,
            phone,
            label,
            page: layout.shape.page,
            room,
            rail_gone,
            solo,
            rows: layout.shape.rows,
            top_bar_slides: layout.shape.top_bar_slides,
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
        self.glide_list(window, reduce);
        let top = unpx(self.list_state.scrolled());
        let screen = unpx(self.list_state.viewport_bounds().size.height);
        let layout = &mut self.layout;
        layout.fold_fab(top);
        layout.show_to_top(top, screen);
        if first {
            layout.fab_label.snap(layout.fab_label.target());
            layout.to_top.snap(layout.to_top.target());
        }
        layout.fab_label.tick(window, reduce);
        layout.to_top.tick(window, reduce);
        self.slide_rows(top, open, first, window, reduce);
        if !self.slides() {
            // The next conversation opened slides in from the edge again.
            self.layout.page.snap(0.0);
        }
    }

    /// Whether an opened conversation slides in over the list: a phone, or
    /// a tablet too narrow for the reading pane.
    /// Slides a phone's search row and list toolbar away once the list has
    /// moved on past them, and back as soon as it turns back up or reaches
    /// the top. The list keeps still on screen while they move: it takes
    /// up the room they leave, or give back.
    fn slide_rows(&mut self, top: f32, open: bool, first: bool, window: &Window, reduce: bool) {
        let slides = matches!(window.window_decorations(), Decorations::Server);
        // Settings cover the list on a phone: its rows come back.
        let covered = self.settings_open || self.settings_page.is_some();
        let layout = &mut self.layout;
        let phone = layout.size == Some(Size::Phone);
        let rows_height = TOOLBAR_HEIGHT + if slides { super::TOP_BAR_HEIGHT } else { 0.0 };
        let at_top = top <= 1.0;
        let listing = phone && !open && !covered && !layout.drawer;
        if !listing || at_top || -layout.list_run >= ROWS_RETURN_AFTER {
            layout.rows.set(1.0);
        } else if layout.list_run >= FAB_FOLD_AFTER && top > rows_height {
            layout.rows.set(0.0);
        }
        if first || !phone {
            layout.rows.snap(layout.rows.target());
        }
        let before = layout.rows.value().clamp(0.0, 1.0);
        let rows = layout.rows.tick(window, reduce).clamp(0.0, 1.0);
        layout.shape.rows = rows;
        layout.shape.top_bar_slides = slides;
        // Hiding by `moved` would lift the list by as much: scroll it back.
        // At the top the list comes down with the rows instead.
        let moved = rows_height * (before - rows);
        if phone && moved.abs() > 0.01 && !(moved < 0.0 && at_top) {
            self.list_state.scroll_by(px(-moved));
            self.layout.list_top -= moved;
        }
    }

    /// Takes the list back to its top, gliding the last screens of the way.
    pub(super) fn glide_list_to_top(&mut self, cx: &mut Context<Self>) {
        let top = unpx(self.list_state.scrolled());
        if top <= 0.5 {
            return;
        }
        let screen = unpx(self.list_state.viewport_bounds().size.height).max(TO_TOP_AFTER);
        let from = top.min(screen * GLIDE_SCREENS);
        if top > from {
            self.list_state.scroll_by(px(from - top));
        }
        let mut spring = Spring::new(motion::SMOOTH, from);
        spring.set(0.0);
        self.layout.glide = Some((spring, from));
        cx.notify();
    }

    /// Moves the gliding list on by this frame's step. It moves by steps
    /// rather than to a place, so the phone's rows sliding back as it goes
    /// up keep the list where they leave it.
    fn glide_list(&mut self, window: &Window, reduce: bool) {
        let Some((spring, at)) = &mut self.layout.glide else {
            return;
        };
        let to = spring.tick(window, reduce);
        let step = to - *at;
        *at = to;
        let done = spring.settled() || to <= 0.5;
        self.list_state.scroll_by(px(step));
        if done {
            self.layout.glide = None;
            self.scroll_list_to(0);
        }
    }

    /// How wide the window's content is, for a dialog to fit inside it:
    /// less Katna's frame, its shadow and resize border, which the
    /// window's own size counts.
    pub(super) fn room_width(&self) -> f32 {
        self.layout.shape.width
    }

    /// How tall the window's content is, as [`Self::room_width`] the width.
    pub(super) fn room_height(&self, window: &Window) -> f32 {
        self.chrome.inner_height(window)
    }

    pub(super) fn slides(&self) -> bool {
        !self.layout.shape.is_desktop() && !self.split()
    }

    /// Whether the conversation has slid away, so it can be forgotten.
    pub(super) fn page_closed(&self) -> bool {
        self.layout.page.target() == 0.0 && self.layout.page.settled()
    }

    /// The folders stay open beside the list: only on a desktop, and not
    /// while Settings is open, whose own list of pages takes their place.
    pub(super) fn nav_docked(&self) -> bool {
        self.nav_open && self.layout.shape.is_desktop() && self.settings_page.is_none()
    }

    pub(super) fn close_drawer(&mut self, cx: &mut Context<Self>) {
        if self.layout.drawer {
            self.layout.drawer = false;
            cx.notify();
        }
    }

    /// The app rail, sliding out to the left as the window turns into a
    /// phone, or when only Mail is on and its folders are open.
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
                    .ml(px(shape.rail() - APP_RAIL_WIDTH))
                    .opacity((1.0 - shape.phone) * (1.0 - shape.rail_gone))
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
        // The names under the icons follow the setting the rail's follow;
        // without them the icons sit in the middle of the bar.
        let labels = self.config.mail.app_labels;
        let apps: Vec<RailApp> = self.apps().collect();
        let items =
            apps.into_iter().map(|app| {
                let on = self.app == app;
                div()
                    .id(("bottom-app", app as usize))
                    .flex_1()
                    .min_w_0()
                    .h(px(BOTTOM_BAR_HEIGHT))
                    .flex()
                    .flex_col()
                    .items_center()
                    .cursor_pointer()
                    .keeps_press()
                    .when(!labels, |d| d.tooltip(tip(app.label(), th)))
                    .group("bottom-app")
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.close_drawer(cx);
                        this.show_page(app, window, cx)
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
                                SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE))
                                    .to(if on { 1.0 } else { 0.0 }),
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
                            .line_height(px(16.0))
                            .font_weight(if on {
                                FontWeight::BOLD
                            } else {
                                FontWeight::MEDIUM
                            })
                            .text_color(rgba(if on { th.text } else { th.text_dim }))
                            .child(app.label())
                            .with_spring(
                                ("bottom-label", app as usize),
                                SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE))
                                    .to(if labels { 1.0 } else { 0.0 }),
                                |el, s: f32| {
                                    let s = s.clamp(0.0, 1.0);
                                    el.h(px(16.0 * s)).opacity(s)
                                },
                            ),
                    )
                    .with_spring(
                        ("bottom-room", app as usize),
                        SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE))
                            .to(if labels { 1.0 } else { 0.0 }),
                        |el, s: f32| {
                            let s = s.clamp(0.0, 1.0);
                            // The icon's pill alone is centered (32 of 72).
                            el.pt(px(lerp(20.0, 12.0, s))).gap(px(4.0 * s))
                        },
                    )
            });
        // It rises from under the window's edge.
        Some(
            div()
                .flex_none()
                .w_full()
                .h(px(shape.bottom_bar()))
                .overflow_hidden()
                .bg(rgba(th.backdrop))
                // Along the window's bottom edge, so round with its corners.
                .rounded_bl(px(self.bottom_corners.0))
                .rounded_br(px(self.bottom_corners.1))
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

    /// The big button floating over a phone's page (Compose in Mail, the
    /// page's own action elsewhere), above the bottom bar and any note at
    /// the bottom.
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
            || self.page_editor_open()
            || self.settings_open
            || self.settings_page.is_some()
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
        let word = self.primary_button().1;
        // Upload asks first whether files or a folder go up.
        let upload = self.drive_upload_here();
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
                        .when(label < 0.5, |d| d.tooltip(tip(word.clone(), th)))
                        .on_click(cx.listener(move |this, e: &gpui::ClickEvent, window, cx| {
                            if upload {
                                this.open_upload_menu(e.position(), cx);
                            } else {
                                this.primary_action(window, cx);
                            }
                        }))
                        .child(self.primary_icon(th))
                        .child(
                            div()
                                .pl(px(12.0 * label))
                                .max_w(px(FAB_LABEL_WIDTH * label))
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .opacity(label)
                                .child(self.primary_label()),
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
                // A click on it only closes the drawer.
                .occlude()
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

    /// The top of the drawer of a phone or tablet: the app's name.
    pub(super) fn render_drawer_head(&self, th: &Theme) -> Option<AnyElement> {
        let shape = self.layout.shape;
        if shape.is_desktop() || !self.layout.drawer {
            return None;
        }
        Some(
            div()
                .flex_none()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(48.0))
                        // The logo centred on the folder icons below.
                        .pl(px(26.0 + 12.0 - super::TITLE_MARK / 2.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(super::TITLE_MARK_GAP))
                        .text_size(px(20.0))
                        .text_color(rgba(th.text))
                        // The same logo as the top bar's on a desktop.
                        .child(crate::widgets::katna_mark(super::TITLE_MARK, th))
                        .child("Katna Mail"),
                )
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
                        .child(div().pl(px(18.0)).child(katna_i18n::tr!("settings"))),
                )
                .child(
                    drawer_row("drawer-language", self.language_picker_open(), th)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.layout.drawer = false;
                            this.toggle_language_picker(None, window, cx);
                        }))
                        .child(super::language::flag(
                            &katna_i18n::current().language.flag,
                            th,
                        ))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .pl(px(14.0))
                                .truncate()
                                .child(katna_i18n::tr!("language-setting")),
                        )
                        .child(
                            div()
                                .flex_none()
                                .pr(px(8.0))
                                .text_color(rgba(th.text_dim))
                                .child(katna_i18n::current().language.name.clone()),
                        ),
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
            .keeps_press()
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
                this.picked = None;
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
                self.person_avatar(name, address, 40.0)
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
        .keeps_press()
        // The line under it neither lights up nor takes the click.
        .occlude()
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
        .ml(px(NAV_ROW_INSET))
        .mr(px(16.0))
        .pl(px(26.0 - NAV_ROW_INSET))
        .pr(px(12.0))
        .flex()
        .flex_row()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .text_color(rgba(if on { th.row_selected_text } else { th.text }))
        .when(on, |d| {
            d.bg(rgba(th.row_selected)).font_weight(FontWeight::BOLD)
        })
        .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
        .cursor_pointer()
        .keeps_press()
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
