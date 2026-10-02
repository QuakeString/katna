// SPDX-License-Identifier: GPL-3.0-or-later

//! The window frame: a toolbar under the compositor's title bar (SSD), or our
//! own header bar, shadow, rounded corners and resize edges (CSD).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, BoxShadow, ClickEvent, Context, CursorStyle, Decorations, Div,
    FontWeight, Global, HitboxBehavior, Hsla, IntoElement, MouseButton, ParentElement, PathBuilder,
    Pixels, ResizeEdge, SharedString, Size, Styled, Tiling, TitlebarOptions, Window,
    WindowAppearance, WindowBackgroundAppearance, WindowButton, WindowButtonLayout,
    WindowControlArea, WindowDecorations, WindowOptions, canvas, div, point, prelude::*, rgba,
    size,
};
use katna_ui::px;
use katna_ui::unpx;

use crate::desktop::{DecorationMode, Environment, Preset, Session};
use crate::geometry::{Edge, FrameGeometry, RESIZE_HANDLE, Rect, Sides};
use crate::tokens::{ChromeColors, ChromeTokens, Shadow, with_alpha};

/// GNOME HIG minimum window size (360×294 logical pixels), in the
/// desktop's pixels, whatever Katna's own scale.
#[allow(clippy::disallowed_methods)]
pub const MIN_WINDOW_SIZE: Size<Pixels> = Size {
    width: gpui::px(360.0),
    height: gpui::px(294.0),
};

/// How the app wants its windows to look: a GPUI global that every
/// [`WindowChrome`] follows as it changes ([`WindowChrome::sync_look`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Look {
    /// Katna's own frame where the desktop would draw one
    /// ([`Environment::own_frame`]).
    pub own_frame: bool,
    /// A translucent background that the compositor blurs, where it can.
    pub blur: bool,
    /// The corner radius of Katna's frame, in logical pixels; `None` keeps
    /// the preset's.
    pub radius: Option<u8>,
    /// The thin line around Katna's frame.
    pub border: bool,
    /// How opaque that line is, in percent; `None` keeps the preset's.
    pub border_opacity: Option<u8>,
    /// How opaque a blurred window's background is, in percent; `None`
    /// keeps [`crate::tokens::blur_alpha`].
    pub blur_opacity: Option<u8>,
}

impl Default for Look {
    fn default() -> Self {
        Self {
            own_frame: false,
            blur: false,
            radius: None,
            border: true,
            border_opacity: None,
            blur_opacity: None,
        }
    }
}

impl Global for Look {}

impl Look {
    /// Whether the compositor blurs what is behind a window (KWin's blur
    /// effect, or Windows'). Elsewhere [`Look::blur`] has no effect.
    pub fn blur_available() -> bool {
        katna_ui::native::compositor_blur()
    }
}

/// The margin, in logical pixels, around the visible frame of a window
/// opened in `env`: its shadow and resize edges under CSD, none under SSD.
pub(crate) fn surface_margin(env: &Environment) -> f32 {
    // Windows draws the shadow and the resize edges of every window.
    if cfg!(windows) {
        return 0.0;
    }
    match env.requested_decorations() {
        DecorationMode::Client if env.full_client_frame() => {
            ChromeTokens::new(env.preset(), false).shadow_inset
        }
        DecorationMode::Client => RESIZE_HANDLE,
        DecorationMode::Server => 0.0,
    }
}

/// Space between the ends of the header bar and what it holds.
const BAR_PADDING: f32 = 6.0;

/// Padding on each side of a group of window buttons. The buttons sit
/// centred in the bar, so this puts them as far from the window's side as
/// from its top.
fn button_side(t: &ChromeTokens, bar_height: f32) -> f32 {
    ((bar_height - t.button_size) / 2.0 - BAR_PADDING).max(0.0)
}

/// Options for opening a Katna window with the right decorations.
pub fn window_options(
    env: &Environment,
    app_id: &str,
    title: impl Into<SharedString>,
    initial_size: Size<Pixels>,
    cx: &App,
) -> WindowOptions {
    // Under CSD the surface includes the shadow margin; grow it so the
    // visible window has the requested size.
    let margin = surface_margin(env);
    let surface_size = size(
        initial_size.width + px(2.0 * margin),
        initial_size.height + px(2.0 * margin),
    );
    WindowOptions {
        window_bounds: Some(crate::placement::fitted(surface_size, cx)),
        titlebar: Some(TitlebarOptions {
            title: Some(title.into()),
            // Windows leaves the title bar to Katna for its own frame. It is
            // set when the window opens and cannot change after.
            appears_transparent: hides_title_bar(env),
            ..Default::default()
        }),
        app_id: Some(app_id.to_owned()),
        // GPUI switches the surface to transparent by itself when CSD is
        // negotiated; Opaque lets it promise an opaque region under SSD.
        window_background: WindowBackgroundAppearance::Opaque,
        window_decorations: Some(match env.requested_decorations() {
            DecorationMode::Server => WindowDecorations::Server,
            DecorationMode::Client => WindowDecorations::Client,
        }),
        window_min_size: Some(MIN_WINDOW_SIZE),
        ..Default::default()
    }
}

/// Whether a window opened in `env` leaves the title bar to Katna on
/// Windows, where GPUI decides that when the window opens.
fn hides_title_bar(env: &Environment) -> bool {
    cfg!(windows) && env.requested_decorations() == DecorationMode::Client
}

/// What the header bar (CSD) or toolbar (SSD) holds.
#[derive(Default)]
pub struct Bar {
    /// Items at the start of the bar, after any window buttons there.
    pub start: Vec<AnyElement>,
    /// An element centered on the bar in place of the window title.
    pub center: Option<AnyElement>,
    /// Items at the end of the bar, before any window buttons there.
    pub end: Vec<AnyElement>,
    /// Bar height, instead of the preset's.
    pub height: Option<f32>,
    /// Bar background, instead of the preset's. The bar then has no bottom
    /// border, so it can blend into the content below.
    pub background: Option<u32>,
}

/// Per-window chrome state. Keep one in the root view and call
/// [`WindowChrome::render`] from its `render`.
pub struct WindowChrome {
    env: RefCell<Environment>,
    title: SharedString,
    /// Set on header-bar mouse down; the move starts on the first mouse move
    /// so a double click can still maximize.
    drag_pending: Rc<Cell<bool>>,
    input_region: Cell<Option<Rect>>,
    /// The app's own light or dark choice, over the desktop's.
    dark: Cell<Option<bool>>,
    /// The desktop's color scheme, over the preset's colors.
    colors: Cell<Option<ChromeColors>>,
    /// The app's window background, over the preset's.
    backdrop: Cell<Option<u32>>,
    /// Whether this window may be blurred ([`WindowChrome::opaque`]).
    blur_allowed: bool,
    /// The window is translucent and blurred.
    blurred: Cell<bool>,
    /// Windows' own title bar is hidden and Katna's bar holds the window
    /// buttons: Katna's frame on Windows, fixed when the window opened.
    title_bar_hidden: bool,
    /// The [`Look`] as of the last [`WindowChrome::sync_look`].
    look: Cell<Look>,
    /// The preset's corner radius and border opacity in percent, before
    /// [`Look`] changes them, as of the last [`WindowChrome::tokens`].
    natural: Cell<(f32, u8)>,
}

impl WindowChrome {
    /// Creates the chrome and subscribes the view to the window changes that
    /// alter the frame (focus, color scheme, button layout).
    pub fn new<V: 'static>(
        env: Environment,
        title: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<V>,
    ) -> Self {
        cx.observe_window_activation(window, |_, _, cx| cx.notify())
            .detach();
        cx.observe_window_appearance(window, |_, _, cx| cx.notify())
            .detach();
        cx.observe_button_layout_changed(window, |_, _, cx| cx.notify())
            .detach();
        cx.observe_global::<Look>(|_, cx| cx.notify()).detach();
        let title_bar_hidden = hides_title_bar(&env);
        Self {
            title_bar_hidden,
            env: RefCell::new(env),
            title: title.into(),
            drag_pending: Rc::new(Cell::new(false)),
            input_region: Cell::new(None),
            dark: Cell::new(None),
            colors: Cell::new(None),
            backdrop: Cell::new(None),
            blur_allowed: true,
            blurred: Cell::new(false),
            look: Cell::new(Look::default()),
            natural: Cell::new((0.0, 0)),
        }
    }

    /// Keeps the window opaque whatever [`Look::blur`] says, for windows
    /// that are all content.
    pub fn opaque(mut self) -> Self {
        self.blur_allowed = false;
        self
    }

    /// The desktop and the frame asked for, as they are now.
    pub fn environment(&self) -> Environment {
        self.env.borrow().clone()
    }

    /// Whether the frame asked for differs from the one this window has
    /// until it opens again: on Windows the frame is chosen when a window
    /// opens.
    pub fn frame_on_reopen(&self) -> bool {
        hides_title_bar(&self.env.borrow()) != self.title_bar_hidden
    }

    /// Whether Katna's bar holds the window buttons.
    fn draws_buttons(&self, window: &Window) -> bool {
        self.title_bar_hidden || matches!(window.window_decorations(), Decorations::Client { .. })
    }

    /// Whether the window is translucent and blurred.
    pub fn blurred(&self) -> bool {
        self.blurred.get()
    }

    /// Brings the window in line with the [`Look`] global: asks for the
    /// other frame when [`Look::own_frame`] changed, and makes the
    /// background translucent and blurred, or opaque again. Call it at the
    /// start of the root view's `render`, before [`WindowChrome::tokens`].
    pub fn sync_look(&self, window: &mut Window, cx: &App) {
        let look = cx.try_global::<Look>().copied().unwrap_or_default();
        self.look.set(look);
        let switch = {
            let mut env = self.env.borrow_mut();
            (env.own_frame != look.own_frame).then(|| {
                env.own_frame = look.own_frame;
                env.requested_decorations()
            })
        };
        if let Some(mode) = switch {
            self.switch_frame(mode, window);
        }
        let blur = self.blur_allowed && look.blur && Look::blur_available();
        if self.blurred.replace(blur) != blur {
            window.set_background_appearance(if blur {
                WindowBackgroundAppearance::Blurred
            } else {
                WindowBackgroundAppearance::Opaque
            });
        }
        // The compositor's blur follows the frame's round corners.
        katna_ui::native::set_client_corner_radius(self.tokens(window).window_radius);
    }

    /// Makes the frame light (`Some(false)`) or dark (`Some(true)`) whatever
    /// the desktop uses, or follows the desktop again (`None`).
    pub fn set_dark(&self, dark: Option<bool>) {
        self.dark.set(dark);
    }

    /// Draws the frame in the desktop's color scheme (`Some`), or in the
    /// preset's own colors (`None`).
    pub fn set_colors(&self, colors: Option<ChromeColors>) {
        self.colors.set(colors);
    }

    /// Asks for the other frame. The window keeps its size on screen: KWin
    /// keeps the frame's size when the shadow margin comes, and on Wayland
    /// its next configure does the same both ways. On X11 the margin is
    /// taken off the window when it goes, or the window would grow by it.
    fn switch_frame(&self, mode: DecorationMode, window: &mut Window) {
        let env = self.env.borrow().clone();
        let resize = env.session == Session::X11
            && !window.is_maximized()
            && !window.is_fullscreen()
            && env.full_client_frame();
        let margin = 2.0 * self.tokens(window).shadow_inset;
        let size = window.viewport_size();
        window.request_decorations(match mode {
            DecorationMode::Server => WindowDecorations::Server,
            DecorationMode::Client => WindowDecorations::Client,
        });
        let now_client = matches!(window.window_decorations(), Decorations::Client { .. });
        if !resize || mode != DecorationMode::Server || now_client {
            return;
        }
        window.resize(gpui::size(
            (size.width - px(margin)).max(MIN_WINDOW_SIZE.width),
            (size.height - px(margin)).max(MIN_WINDOW_SIZE.height),
        ));
    }

    /// Paints the window's background in the app's own color (`Some`)
    /// rather than the preset's (`None`).
    pub fn set_backdrop(&self, color: Option<u32>) {
        self.backdrop.set(color);
    }

    /// Whether the desktop asks for a dark color scheme.
    pub fn desktop_dark(window: &Window) -> bool {
        matches!(
            window.appearance(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark
        )
    }

    /// Tokens for the current preset and color scheme.
    pub fn tokens(&self, window: &Window) -> ChromeTokens {
        let dark = self
            .dark
            .get()
            .unwrap_or_else(|| Self::desktop_dark(window));
        let preset = self.env.borrow().preset();
        let mut tokens = ChromeTokens::new(preset, dark);
        if preset == Preset::BreezeLike {
            tokens = tokens.with_button_size(crate::breeze::button_size());
        }
        let mut tokens = match self.colors.get() {
            Some(colors) => tokens.recolored(&colors),
            None => tokens,
        };
        if let Some(backdrop) = self.backdrop.get() {
            tokens.window_bg = backdrop;
        }
        let look = self.look.get();
        self.natural
            .set((tokens.window_radius, percent(tokens.outline & 0xff)));
        let tokens = with_look(tokens, &look);
        if self.blurred.get() {
            match look.blur_opacity {
                Some(opacity) => tokens.translucent_at(alpha(opacity)),
                None => tokens.translucent(),
            }
        } else {
            tokens
        }
    }

    /// The width the content gets: the window's surface less the shadow,
    /// resize margins and border that the frame draws around it.
    pub fn inner_width(&self, window: &Window) -> f32 {
        let width = unpx(window.viewport_size().width);
        let Decorations::Client { tiling } = window.window_decorations() else {
            return width;
        };
        let full = self.env.borrow().full_client_frame();
        if !full && tiling.top && tiling.right && tiling.bottom && tiling.left {
            return width;
        }
        let inset = if full {
            self.tokens(window).shadow_inset
        } else {
            RESIZE_HANDLE
        };
        // The margin and the border on an edge that is not tiled.
        let border = self.border_width();
        let edge = |tiled: bool| if tiled { 0.0 } else { inset + border };
        (width - edge(tiling.left) - edge(tiling.right)).max(0.0)
    }

    /// The preset's corner radius, in logical pixels, and its border's
    /// opacity, in percent: what Katna's frame has when [`Look`] leaves
    /// them be.
    pub fn natural_corners(&self) -> (f32, u8) {
        self.natural.get()
    }

    /// The width of the line around Katna's frame: none when [`Look`]
    /// turns it off.
    fn border_width(&self) -> f32 {
        if self.look.get().border { 1.0 } else { 0.0 }
    }

    /// The room the window buttons take at the start and at the end of
    /// the header bar, with their padding and the gap after them. Zero on a
    /// side without buttons, and on both under server-side decorations.
    /// `bar_height` is [`Bar::height`].
    pub fn button_room(&self, bar_height: Option<f32>, window: &Window, cx: &App) -> (f32, f32) {
        if !self.draws_buttons(window) {
            return (0.0, 0.0);
        }
        let t = self.tokens(window);
        let side = button_side(&t, bar_height.unwrap_or(t.header_height));
        let layout = cx.button_layout().unwrap_or_else(default_button_layout);
        let controls = window.window_controls();
        let room = |side_buttons: &[Option<WindowButton>]| {
            let n = side_buttons
                .iter()
                .flatten()
                .filter(|b| match b {
                    WindowButton::Minimize => controls.minimize,
                    WindowButton::Maximize => controls.maximize,
                    WindowButton::Close => true,
                })
                .count() as f32;
            if n == 0.0 {
                0.0
            } else {
                // Buttons and gaps, the group's padding, the bar's gap.
                n * t.button_size + (n - 1.0) * t.button_gap + 2.0 * side + BAR_PADDING
            }
        };
        (room(&layout.left), room(&layout.right))
    }

    /// Wraps `content` in the frame. `start` and `end` go at the two ends of
    /// the header bar (CSD) or the toolbar (SSD).
    pub fn render(
        &self,
        start: Vec<AnyElement>,
        end: Vec<AnyElement>,
        content: AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) -> Div {
        let bar = Bar {
            start,
            end,
            ..Bar::default()
        };
        self.render_bar(bar, content, window, cx)
    }

    /// Wraps `content` in the frame, with a custom [`Bar`].
    pub fn render_bar(
        &self,
        bar: Bar,
        content: AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) -> Div {
        let t = self.tokens(window);
        match window.window_decorations() {
            Decorations::Server => {
                self.set_input_region(None, window);
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .bg(rgba(t.window_bg))
                    .text_color(rgba(t.fg))
                    .child(self.bar(&t, self.title_bar_hidden, bar, window, cx))
                    .child(div().flex_1().min_h_0().child(content))
            }
            Decorations::Client { tiling } => {
                self.client_frame(&t, tiling, bar, content, window, cx)
            }
        }
    }

    fn set_input_region(&self, region: Option<Rect>, window: &Window) {
        if self.input_region.get() != region {
            self.input_region.set(region);
            let rects = region.map(|r| {
                [Bounds::new(
                    point(px(r.x), px(r.y)),
                    size(px(r.width), px(r.height)),
                )]
            });
            window.set_input_region(rects.as_ref().map(|r| r.as_slice()));
        }
    }

    fn client_frame(
        &self,
        t: &ChromeTokens,
        tiling: Tiling,
        bar: Bar,
        content: AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) -> Div {
        let full = self.env.borrow().full_client_frame();
        let inset = if full { t.shadow_inset } else { RESIZE_HANDLE };
        window.set_client_inset(px(inset));

        let tiled = Sides {
            top: tiling.top,
            right: tiling.right,
            bottom: tiling.bottom,
            left: tiling.left,
        };
        let viewport = window.viewport_size();
        let geometry = FrameGeometry {
            surface_width: unpx(viewport.width),
            surface_height: unpx(viewport.height),
            inset,
            tiled,
        };
        self.set_input_region(
            if tiled.all() {
                None
            } else {
                Some(geometry.input_region())
            },
            window,
        );

        // Minimal frames (tiling compositors) disappear entirely when tiled.
        if !full && tiled.all() {
            return div()
                .size_full()
                .bg(rgba(t.window_bg))
                .text_color(rgba(t.fg))
                .child(content);
        }

        let radius = if full { px(t.window_radius) } else { px(0.0) };
        let round = |a: bool, b: bool| if a || b { px(0.0) } else { radius };
        let (r_tl, r_tr) = (round(tiled.top, tiled.left), round(tiled.top, tiled.right));
        let (r_bl, r_br) = (
            round(tiled.bottom, tiled.left),
            round(tiled.bottom, tiled.right),
        );
        let focused = window.is_window_active();
        let shadows: Vec<BoxShadow> = if full && !tiled.all() {
            let layers = if focused {
                t.shadow_focused
            } else {
                t.shadow_unfocused
            };
            layers
                .iter()
                .filter(|s| s.color & 0xff != 0)
                .map(box_shadow)
                .collect()
        } else {
            Vec::new()
        };
        let width = self.border_width();
        let border = |is_tiled: bool| if is_tiled { px(0.0) } else { px(width) };
        // Inside the border the corners nest: the outer radius less its
        // width.
        let inner = |r: Pixels| (r - px(width)).max(px(0.0));

        let frame = div()
            .id("katna-window-frame")
            .size_full()
            .flex()
            .flex_col()
            .cursor(CursorStyle::Arrow)
            .bg(rgba(t.window_bg))
            .text_color(rgba(t.fg))
            .rounded_tl(r_tl)
            .rounded_tr(r_tr)
            .rounded_bl(r_bl)
            .rounded_br(r_br)
            .border_color(rgba(t.outline))
            .border_t(border(tiled.top))
            .border_r(border(tiled.right))
            .border_b(border(tiled.bottom))
            .border_l(border(tiled.left))
            // Katna's renderer draws it only outside the frame, so it does
            // not darken a translucent window (vendor/gpui-pre-wgpu).
            .shadow(shadows)
            // Keep pointer motion inside the frame from refreshing the
            // resize-cursor logic of the shadow area.
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .child(
                self.bar(t, true, bar, window, cx)
                    .rounded_tl(inner(r_tl))
                    .rounded_tr(inner(r_tr)),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .rounded_bl(inner(r_bl))
                    .rounded_br(inner(r_br))
                    .child(content),
            );

        div()
            .size_full()
            .when(!tiled.top, |d| d.pt(px(inset)))
            .when(!tiled.right, |d| d.pr(px(inset)))
            .when(!tiled.bottom, |d| d.pb(px(inset)))
            .when(!tiled.left, |d| d.pl(px(inset)))
            .child(resize_cursor_layer(geometry))
            .on_mouse_move(|_, window, _| window.refresh())
            .on_mouse_down(MouseButton::Left, move |e, window, _| {
                if let Some(edge) =
                    geometry_now(window, inset, tiled).resize_edge(x(e.position), y(e.position))
                {
                    window.start_window_resize(resize_edge(edge));
                }
            })
            .child(frame)
    }

    /// The header bar (CSD) or toolbar (SSD).
    fn bar(
        &self,
        t: &ChromeTokens,
        client_side: bool,
        bar: Bar,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Stateful<Div> {
        let Bar {
            start,
            center,
            end,
            height,
            background,
        } = bar;
        let focused = window.is_window_active();
        let bg = background.unwrap_or(if focused || !client_side {
            t.header_bg
        } else {
            t.header_bg_unfocused
        });
        // Centered on the whole bar, like the title of AdwHeaderBar. It has
        // no hitbox of its own, so the bar under it still drags the window.
        let middle = |child: AnyElement| {
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(child)
        };
        let bar_height = height.unwrap_or(t.header_height);
        let bar = div()
            .id("katna-header-bar")
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .flex_none()
            .h(px(bar_height))
            .px(px(BAR_PADDING))
            .gap(px(6.0))
            .bg(rgba(bg))
            .when(background.is_none(), |d| {
                d.border_b_1().border_color(rgba(t.header_shade))
            });

        if !client_side {
            // Under SSD the compositor owns the title bar; this is a plain
            // toolbar and window operations stay with the compositor.
            return bar
                .h(px(height.unwrap_or(t.header_height.max(40.0))))
                .when_some(center, |d, center| d.child(middle(center)))
                .children(start)
                .child(div().flex_1())
                .children(end);
        }

        let windows = self.title_bar_hidden;
        let layout = cx.button_layout().unwrap_or_else(default_button_layout);
        let controls = window.window_controls();
        let buttons = |side: &[Option<WindowButton>]| {
            side.iter()
                .flatten()
                .copied()
                .filter(|b| match b {
                    WindowButton::Minimize => controls.minimize,
                    WindowButton::Maximize => controls.maximize,
                    WindowButton::Close => true,
                })
                .map(|b| window_button(t, b, windows, window))
                .collect::<Vec<_>>()
        };
        let left = buttons(&layout.left);
        let right = buttons(&layout.right);

        let side = button_side(t, bar_height);
        let title_color = if focused { t.fg } else { t.fg_dim };
        let drag_down = self.drag_pending.clone();
        let drag_up = self.drag_pending.clone();
        let drag_out = self.drag_pending.clone();
        let drag_move = self.drag_pending.clone();

        // On Windows the bar's empty space is the title bar to Windows
        // (`window_control_area`), which moves, maximizes and snaps the
        // window and opens its menu itself.
        bar.when(!windows, |bar| {
            bar.on_mouse_down(MouseButton::Left, move |_, _, _| drag_down.set(true))
                .on_mouse_up(MouseButton::Left, move |_, _, _| drag_up.set(false))
                .on_mouse_down_out(move |_, _, _| drag_out.set(false))
                .on_mouse_move(move |_, window, _| {
                    if drag_move.replace(false) {
                        window.start_window_move();
                    }
                })
                .on_click(|e: &ClickEvent, window, _| {
                    if e.standard_click() && e.click_count() == 2 {
                        window.zoom_window();
                    }
                })
                .on_mouse_down(MouseButton::Right, |e, window, _| {
                    window.show_window_menu(e.position)
                })
        })
        .child(middle(center.unwrap_or_else(|| {
            div()
                .text_size(px(t.title_size))
                .font_weight(FontWeight(t.title_weight as f32))
                .text_color(rgba(title_color))
                .child(self.title.clone())
                .into_any_element()
        })))
        .when(!left.is_empty(), |b| {
            b.child(
                div()
                    .flex()
                    .gap(px(t.button_gap))
                    .px(px(side))
                    .children(left),
            )
        })
        .children(start)
        .child(
            div()
                .flex_1()
                .h_full()
                .when(windows, |d| d.window_control_area(WindowControlArea::Drag)),
        )
        .children(end)
        .when(!right.is_empty(), |b| {
            b.child(
                div()
                    .flex()
                    .gap(px(t.button_gap))
                    .px(px(side))
                    .children(right),
            )
        })
    }
}

/// `tokens` with the corner radius and border [`Look`] asks for.
fn with_look(mut tokens: ChromeTokens, look: &Look) -> ChromeTokens {
    if let Some(radius) = look.radius {
        tokens.window_radius = f32::from(radius);
    }
    if !look.border {
        tokens.outline = with_alpha(tokens.outline, 0);
    } else if let Some(opacity) = look.border_opacity {
        tokens.outline = with_alpha(tokens.outline, alpha(opacity));
    }
    tokens
}

/// A percentage as an alpha byte.
fn alpha(percent: u8) -> u8 {
    (f32::from(percent.min(100)) * 255.0 / 100.0).round() as u8
}

/// An alpha byte as a percentage.
fn percent(alpha: u32) -> u8 {
    (alpha.min(255) as f32 * 100.0 / 255.0).round() as u8
}

fn x(p: gpui::Point<Pixels>) -> f32 {
    unpx(p.x)
}

fn y(p: gpui::Point<Pixels>) -> f32 {
    unpx(p.y)
}

fn geometry_now(window: &Window, inset: f32, tiled: Sides) -> FrameGeometry {
    let viewport = window.viewport_size();
    FrameGeometry {
        surface_width: unpx(viewport.width),
        surface_height: unpx(viewport.height),
        inset,
        tiled,
    }
}

/// Paints nothing; sets the resize cursor while the pointer is on a resize
/// edge in the shadow area.
fn resize_cursor_layer(geometry: FrameGeometry) -> impl IntoElement {
    canvas(
        |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
        move |_, hitbox, window, _| {
            let p = window.mouse_position();
            let Some(edge) = geometry.resize_edge(x(p), y(p)) else {
                return;
            };
            let style = match edge {
                Edge::Top | Edge::Bottom => CursorStyle::ResizeUpDown,
                Edge::Left | Edge::Right => CursorStyle::ResizeLeftRight,
                Edge::TopLeft | Edge::BottomRight => CursorStyle::ResizeUpLeftDownRight,
                Edge::TopRight | Edge::BottomLeft => CursorStyle::ResizeUpRightDownLeft,
            };
            window.set_cursor_style(style, &hitbox);
        },
    )
    .absolute()
    .size_full()
}

fn resize_edge(edge: Edge) -> ResizeEdge {
    match edge {
        Edge::Top => ResizeEdge::Top,
        Edge::TopRight => ResizeEdge::TopRight,
        Edge::Right => ResizeEdge::Right,
        Edge::BottomRight => ResizeEdge::BottomRight,
        Edge::Bottom => ResizeEdge::Bottom,
        Edge::BottomLeft => ResizeEdge::BottomLeft,
        Edge::Left => ResizeEdge::Left,
        Edge::TopLeft => ResizeEdge::TopLeft,
    }
}

fn box_shadow(s: &Shadow) -> BoxShadow {
    BoxShadow {
        color: Hsla::from(rgba(s.color)),
        offset: point(px(s.x), px(s.y)),
        // GPUI's blur radius is the Gaussian's standard deviation, half
        // the CSS blur of the tokens.
        blur_radius: px(s.blur / 2.0),
        spread_radius: px(s.spread),
        inset: false,
    }
}

/// A window button; with `windows`, Windows presses it (and shows its
/// snap layouts over Maximize).
fn window_button(
    t: &ChromeTokens,
    button: WindowButton,
    windows: bool,
    window: &Window,
) -> AnyElement {
    let icon = match button {
        WindowButton::Close => Icon::Close,
        WindowButton::Minimize => Icon::Minimize,
        WindowButton::Maximize if window.is_maximized() => Icon::Restore,
        WindowButton::Maximize => Icon::Maximize,
    };
    let is_close = button == WindowButton::Close;
    let hover_bg = if is_close {
        t.close_bg_hover
    } else {
        t.button_bg_hover
    };
    let (preset, fg, icon_size) = (t.preset, t.fg, t.button_icon_size);
    // Breeze's lines stay about 1.5 px wide as its buttons grow.
    let stroke = (t.button_size / 16.0).max(1.5);
    div()
        .id(button.id())
        .flex()
        .items_center()
        .justify_center()
        .size(px(t.button_size))
        .rounded_full()
        .bg(rgba(t.button_bg))
        .hover(|s| s.bg(rgba(hover_bg)))
        .active(|s| s.bg(rgba(t.button_bg_active)))
        // Do not start a window move from a button. (Stopping mouse down
        // here would also swallow the click.)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(move |_, window, _| match button {
            WindowButton::Close => window.remove_window(),
            WindowButton::Minimize => window.minimize_window(),
            WindowButton::Maximize => window.zoom_window(),
        })
        .when(windows, |d| {
            d.window_control_area(match button {
                WindowButton::Close => WindowControlArea::Close,
                WindowButton::Minimize => WindowControlArea::Min,
                WindowButton::Maximize => WindowControlArea::Max,
            })
        })
        .when(!is_close || t.close_fg_hover == fg, |d| {
            d.child(icon_canvas(icon, preset, fg, icon_size, stroke))
        })
        // Breeze's close button turns red with a white cross: two crosses,
        // one shown while the pointer is over the button.
        .when(is_close && t.close_fg_hover != fg, |d| {
            let cross = |color, hovered: bool| {
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .opacity(if hovered { 0.0 } else { 1.0 })
                    .group_hover(CLOSE_GROUP, move |s| {
                        s.opacity(if hovered { 1.0 } else { 0.0 })
                    })
                    .child(icon_canvas(icon, preset, color, icon_size, stroke))
            };
            d.group(CLOSE_GROUP)
                .relative()
                .child(cross(fg, false))
                .child(cross(t.close_fg_hover, true))
        })
        .into_any_element()
}

/// The hover group of the close button.
const CLOSE_GROUP: &str = "katna-close-button";

#[derive(Clone, Copy)]
enum Icon {
    Close,
    Minimize,
    Maximize,
    Restore,
}

/// Window button glyphs, drawn as strokes on a 16 px grid (own artwork).
fn icon_canvas(
    icon: Icon,
    preset: Preset,
    color: u32,
    icon_size: f32,
    stroke: f32,
) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let s = icon_size / 16.0;
            let o = bounds.origin;
            let p = |x: f32, y: f32| point(o.x + px(x * s), o.y + px(y * s));
            let mut path = PathBuilder::stroke(px(stroke));
            match (icon, preset) {
                (Icon::Close, _) => {
                    path.move_to(p(4.5, 4.5));
                    path.line_to(p(11.5, 11.5));
                    path.move_to(p(11.5, 4.5));
                    path.line_to(p(4.5, 11.5));
                }
                (Icon::Minimize, Preset::AdwaitaLike) => {
                    path.move_to(p(4.5, 11.0));
                    path.line_to(p(11.5, 11.0));
                }
                (Icon::Maximize, Preset::AdwaitaLike) => {
                    path.move_to(p(4.5, 4.5));
                    path.line_to(p(11.5, 4.5));
                    path.line_to(p(11.5, 11.5));
                    path.line_to(p(4.5, 11.5));
                    path.close();
                }
                (Icon::Restore, Preset::AdwaitaLike) => {
                    path.move_to(p(5.5, 5.5));
                    path.line_to(p(10.5, 5.5));
                    path.line_to(p(10.5, 10.5));
                    path.line_to(p(5.5, 10.5));
                    path.close();
                }
                (Icon::Minimize, Preset::BreezeLike) => {
                    path.move_to(p(4.0, 6.5));
                    path.line_to(p(8.0, 10.5));
                    path.line_to(p(12.0, 6.5));
                }
                (Icon::Maximize, Preset::BreezeLike) => {
                    path.move_to(p(4.0, 10.0));
                    path.line_to(p(8.0, 6.0));
                    path.line_to(p(12.0, 10.0));
                }
                (Icon::Restore, Preset::BreezeLike) => {
                    path.move_to(p(8.0, 4.0));
                    path.line_to(p(12.0, 8.0));
                    path.line_to(p(8.0, 12.0));
                    path.line_to(p(4.0, 8.0));
                    path.close();
                }
            }
            if let Ok(path) = path.build() {
                window.paint_path(path, rgba(color));
            }
        },
    )
    .size(px(icon_size))
}

/// The window buttons when the desktop names none: minimize, maximize and
/// close at the end, as on Windows and most Linux desktops.
fn default_button_layout() -> WindowButtonLayout {
    WindowButtonLayout {
        left: [None; 3],
        right: [
            Some(WindowButton::Minimize),
            Some(WindowButton::Maximize),
            Some(WindowButton::Close),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_sets_corners_and_border() {
        let t = ChromeTokens::new(Preset::BreezeLike, true);
        assert_eq!(with_look(t.clone(), &Look::default()), t);
        let look = Look {
            radius: Some(0),
            border_opacity: Some(100),
            ..Look::default()
        };
        let r = with_look(t.clone(), &look);
        assert_eq!(r.window_radius, 0.0);
        assert_eq!(r.outline, with_alpha(t.outline, 0xff));
        let off = with_look(
            t.clone(),
            &Look {
                border: false,
                ..look
            },
        );
        assert_eq!(off.outline & 0xff, 0);
        assert_eq!(off.outline >> 8, t.outline >> 8);
    }

    #[test]
    fn percent_rounds_alpha() {
        assert_eq!(percent(0), 0);
        assert_eq!(percent(0x33), 20);
        assert_eq!(percent(0xff), 100);
    }
}
