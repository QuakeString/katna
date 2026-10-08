// SPDX-License-Identifier: GPL-3.0-or-later

//! The color picker of the scheme editor and of Settings > Appearance >
//! Accent's color wheel: a popover beside the clicked swatch, its notch
//! pointing at it, with a saturation and brightness square, a hue bar, a
//! hex field, the scheme's other colors (in the editor) and the ones
//! picked lately. On Linux a dropper takes a color from anywhere on the
//! screen (the Screenshot portal's PickColor), and "System picker…" opens
//! KDE's or GNOME's own color dialog where one is installed.

use crate::widgets::Tip as _;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, Bounds, Context, DispatchPhase, Entity, Focusable, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString, Subscription, Window, canvas,
    deferred, div, linear_color_stop, linear_gradient, point, prelude::*, rgba,
};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_platform::colors::parse_css_color;
use katna_ui::{InputEvent, TextInput, px, unpx};

use super::MailWindow;
use super::account_color::color_label;
use super::notched::{self, notch};
use super::settings::Change;
use crate::theme::{ACCOUNT_COLORS, Accent, Theme, fade};
use crate::user_schemes::Seed;
use crate::widgets::icon;

const WIDTH: f32 = 264.0;
const PAD: f32 = 12.0;
const GAP: f32 = 10.0;
const SQUARE: f32 = 150.0;
const ROW: f32 = 32.0;
const CAPTION: f32 = 16.0;
const DOT: f32 = 22.0;
const LINK: f32 = 24.0;
const RADIUS: f32 = notched::RADIUS;
/// How many lately picked colors the popover keeps.
pub(super) const RECENT: usize = 8;

/// What the picker colors.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Target {
    /// A seed of the scheme editor's light (false) or dark side.
    Seed(bool, Seed),
    /// The accent, from Settings > Appearance > Accent's wheel.
    Accent,
    /// An account's own color, from the wheel after its colors.
    Account(AccountId),
}

/// Where each swatch that opens the picker was drawn.
pub(super) type Swatches = Rc<RefCell<HashMap<Target, Bounds<Pixels>>>>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Area {
    Square,
    Hue,
}

/// The open picker.
pub(super) struct ColorPicker {
    target: Target,
    color: u32,
    /// Hue (0..360), saturation and value (0..=1): kept apart from the
    /// color so the hue stays put on greys.
    hsv: [f32; 3],
    hex: Entity<TextInput>,
    dragging: Option<Area>,
    square: Rc<Cell<Option<Bounds<Pixels>>>>,
    hue: Rc<Cell<Option<Bounds<Pixels>>>>,
    _subscription: Subscription,
    /// When it closed and began to fade out.
    fading: Option<Instant>,
}

impl ColorPicker {
    pub(super) fn target(&self) -> Target {
        self.target
    }
}

impl MailWindow {
    /// Opens the picker for `target`, or closes it if it is open for that
    /// one already.
    pub(super) fn toggle_color_picker(
        &mut self,
        target: Target,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let open = self
            .color_picker
            .as_ref()
            .filter(|p| p.fading.is_none())
            .map(ColorPicker::target);
        self.close_color_picker(cx);
        if open == Some(target) {
            return;
        }
        let th = self.theme(window);
        let color = match target {
            Target::Seed(dark, seed) => {
                let Some(side) = self.scheme_editor.as_ref().and_then(|e| e.side(dark)) else {
                    return;
                };
                seed.get(side)
            }
            // The custom accent as picked; the theme may have shaded it
            // to read on the scheme.
            Target::Accent => match Accent::parse(&self.config.mail.accent) {
                Accent::Color(color) => color,
                _ => th.accent | 0xff,
            },
            Target::Account(id) => match self.account_address(id) {
                Some(address) => self.account_light(&address),
                None => return,
            },
        };
        let accent = rgba(th.accent).into();
        let hex = cx.new(|cx| {
            let mut input = TextInput::new("#000000", cx);
            input.set_accent(accent);
            input.set_text(super::scheme_editor::hex(color), cx);
            input
        });
        let subscription = cx.subscribe_in(
            &hex,
            window,
            move |this, field, event: &InputEvent, _, cx| match event {
                InputEvent::Changed => {
                    let text = field.read(cx).text().to_owned();
                    if let Some(color) = parse_css_color(&text) {
                        this.set_picker_color(color | 0xff, true, cx);
                    }
                }
                InputEvent::Submit | InputEvent::Cancel => this.close_color_picker(cx),
            },
        );
        self.color_picker = Some(ColorPicker {
            target,
            color,
            hsv: to_hsv(color),
            hex,
            dragging: None,
            square: Rc::default(),
            hue: Rc::default(),
            _subscription: subscription,
            fading: None,
        });
        cx.notify();
    }

    /// Closes the picker, keeping its color among the recent ones.
    pub(super) fn close_color_picker(&mut self, cx: &mut Context<Self>) {
        let Some(color) = self
            .color_picker
            .as_ref()
            .filter(|p| p.fading.is_none())
            .map(|p| p.color)
        else {
            return;
        };
        self.recent_colors.retain(|c| *c != color);
        self.recent_colors.insert(0, color);
        self.recent_colors.truncate(RECENT);
        // It fades out where it was.
        match notched::fade_out(cx) {
            Some(since) => {
                if let Some(picker) = &mut self.color_picker {
                    picker.fading = Some(since);
                }
            }
            None => self.color_picker = None,
        }
        cx.notify();
    }

    /// Closes the picker without keeping its color, when what it colors
    /// goes away.
    pub(super) fn drop_color_picker(&mut self, gone: impl Fn(Target) -> bool) {
        if self.color_picker.as_ref().is_some_and(|p| gone(p.target)) {
            self.color_picker = None;
        }
    }

    /// Gives the picked target `color`. `typed`: it came from the
    /// popover's hex field, which then keeps its text.
    fn set_picker_color(&mut self, color: u32, typed: bool, cx: &mut Context<Self>) {
        let Some(picker) = &mut self.color_picker else {
            return;
        };
        if picker.color == color {
            return;
        }
        picker.hsv = to_hsv(color);
        let hex = picker.hex.clone();
        self.color_target(color, cx);
        if !typed {
            hex.update(cx, |field, cx| {
                field.set_text(super::scheme_editor::hex(color), cx)
            });
        }
    }

    /// Gives what the picker colors `color`.
    fn color_target(&mut self, color: u32, cx: &mut Context<Self>) {
        let Some(picker) = &mut self.color_picker else {
            return;
        };
        picker.color = color;
        match picker.target {
            Target::Seed(dark, seed) => self.set_scheme_seed(dark, seed, color, cx),
            Target::Accent => self.apply(Change::Accent(Accent::Color(color)), cx),
            Target::Account(id) => {
                if let Some(address) = self.account_address(id) {
                    self.set_account_custom(&address, color, cx);
                }
            }
        }
    }

    /// Follows a target's own field: the picker open on it shows the
    /// color.
    pub(super) fn sync_color_picker(&mut self, target: Target, color: u32, cx: &mut Context<Self>) {
        let Some(picker) = &mut self.color_picker else {
            return;
        };
        if picker.target != target || picker.color == color {
            return;
        }
        picker.color = color;
        picker.hsv = to_hsv(color);
        let hex = picker.hex.clone();
        hex.update(cx, |field, cx| {
            field.set_text(super::scheme_editor::hex(color), cx)
        });
    }

    /// Moves the square's or the hue bar's knob to `at`.
    fn drag_color(&mut self, area: Area, at: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(picker) = &mut self.color_picker else {
            return;
        };
        let bounds = match area {
            Area::Square => picker.square.get(),
            Area::Hue => picker.hue.get(),
        };
        let Some(bounds) = bounds else {
            return;
        };
        let fx = ((unpx(at.x) - unpx(bounds.origin.x)) / unpx(bounds.size.width)).clamp(0.0, 1.0);
        let fy = ((unpx(at.y) - unpx(bounds.origin.y)) / unpx(bounds.size.height)).clamp(0.0, 1.0);
        match area {
            Area::Square => {
                picker.hsv[1] = fx;
                picker.hsv[2] = 1.0 - fy;
            }
            Area::Hue => picker.hsv[0] = (fx * 360.0).min(359.9),
        }
        let color = from_hsv(picker.hsv);
        let hex = picker.hex.clone();
        self.color_target(color, cx);
        hex.update(cx, |field, cx| {
            field.set_text(super::scheme_editor::hex(color), cx)
        });
    }

    fn set_dragging(&mut self, area: Option<Area>, cx: &mut Context<Self>) {
        if let Some(picker) = &mut self.color_picker {
            picker.dragging = area;
            cx.notify();
        }
    }

    /// Takes a color from anywhere on the screen with the desktop's
    /// dropper.
    #[cfg(not(windows))]
    fn pick_from_screen(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let picked = async { ashpd::desktop::Color::pick().send().await?.response() }.await;
            let color = match picked {
                Ok(color) => {
                    let channel = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
                    (channel(color.red()) << 24)
                        | (channel(color.green()) << 16)
                        | (channel(color.blue()) << 8)
                        | 0xff
                }
                Err(err) => {
                    tracing::info!("no color picked from the screen: {err}");
                    return;
                }
            };
            this.update(cx, |this, cx| this.set_picker_color(color, false, cx))
                .ok();
        })
        .detach();
    }

    /// Opens the desktop's own color dialog on the picked color.
    fn open_system_picker(&mut self, dialog: SystemDialog, cx: &mut Context<Self>) {
        let Some(color) = self.color_picker.as_ref().map(|p| p.color) else {
            return;
        };
        let start = super::scheme_editor::hex(color);
        let run = cx.background_spawn(async move {
            let output = match dialog {
                SystemDialog::Kde => std::process::Command::new("kdialog")
                    .args(["--getcolor", "--default", &start])
                    .output(),
                SystemDialog::Gnome => std::process::Command::new("zenity")
                    .args(["--color-selection", "--color", &start])
                    .output(),
            };
            let output = output.ok().filter(|o| o.status.success())?;
            parse_css_color(String::from_utf8_lossy(&output.stdout).trim())
        });
        cx.spawn(async move |this, cx| {
            if let Some(color) = run.await {
                this.update(cx, |this, cx| {
                    this.set_picker_color(color | 0xff, false, cx)
                })
                .ok();
            }
        })
        .detach();
    }

    /// The popover, beside the swatch it belongs to, when it is open for
    /// a target `here` takes: each place that shows swatches draws it.
    pub(super) fn render_color_picker(
        &self,
        here: impl Fn(Target) -> bool,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let picker = self.color_picker.as_ref().filter(|p| {
            here(p.target) && !p.fading.is_some_and(|since| notched::faded(since, cx))
        })?;
        let fading = picker.fading;
        let swatch = *self.color_swatches.borrow().get(&picker.target)?;
        let color = picker.color;
        let [h, s, v] = picker.hsv;
        let pure = from_hsv([h, 1.0, 1.0]);

        // In the editor, the scheme's other colors.
        let mut in_scheme: Vec<u32> = Vec::new();
        if let Target::Seed(dark, _) = picker.target
            && let Some(side) = self.scheme_editor.as_ref().and_then(|e| e.side(dark))
        {
            for seed in Seed::ALL {
                let c = seed.get(side);
                if !in_scheme.contains(&c) {
                    in_scheme.push(c);
                }
            }
        }
        // For an account, the standard account colors first.
        let standard = matches!(picker.target, Target::Account(_));
        let recent = self.recent_colors.clone();
        let system = SystemDialog::find();
        let dropper = cfg!(not(windows));
        let height = PAD * 2.0
            + if standard { DOT + GAP } else { 0.0 }
            + SQUARE
            + GAP
            + ROW
            + GAP
            + if in_scheme.is_empty() {
                0.0
            } else {
                GAP + CAPTION + 6.0 + DOT
            }
            + if recent.is_empty() {
                0.0
            } else {
                GAP + CAPTION + 6.0 + DOT
            }
            + if system.is_some() { GAP + LINK } else { 0.0 };
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let (x, y, side_of, along) = place_beside(swatch, (WIDTH, height), (vw, vh));

        let knob = |c: u32| {
            div()
                .absolute()
                .size(px(16.0))
                .rounded_full()
                .border_2()
                .border_color(rgba(0xffffffff))
                .shadow(crate::widgets::elevation(th, 1.0))
                .bg(rgba(c))
        };
        let entity = cx.entity();
        // While dragging, the pointer counts anywhere in the window.
        let tracker = |area: Area, bounds: Rc<Cell<Option<Bounds<Pixels>>>>| {
            let dragging = picker.dragging == Some(area);
            let entity = entity.clone();
            canvas(
                move |b, _, _| bounds.set(Some(b)),
                move |_, _, window, _| {
                    if !dragging {
                        return;
                    }
                    let moved = entity.clone();
                    window.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
                        if phase != DispatchPhase::Bubble {
                            return;
                        }
                        // A release missed between frames ends the drag.
                        if event.pressed_button != Some(MouseButton::Left) {
                            moved.update(cx, |this, cx| this.set_dragging(None, cx));
                            return;
                        }
                        moved.update(cx, |this, cx| {
                            // Listeners from a frame before the drag changed
                            // count for nothing.
                            let still = this
                                .color_picker
                                .as_ref()
                                .is_some_and(|p| p.dragging == Some(area));
                            if still {
                                this.drag_color(area, event.position, cx);
                            }
                        });
                    });
                    let released = entity.clone();
                    window.on_mouse_event(move |_: &MouseUpEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            released.update(cx, |this, cx| this.set_dragging(None, cx));
                        }
                    });
                },
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full()
        };
        let square = div()
            .id("picker-square")
            .relative()
            .h(px(SQUARE))
            .rounded(px(10.0))
            .bg(rgba(pure))
            .cursor_crosshair()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.set_dragging(Some(Area::Square), cx);
                    this.drag_color(Area::Square, event.position, cx);
                }),
            )
            // GPUI clips to rectangles, so each layer rounds itself.
            .child(
                div()
                    .absolute()
                    .size_full()
                    .rounded(px(10.0))
                    .bg(linear_gradient(
                        90.0,
                        linear_color_stop(rgba(0xffffffff), 0.0),
                        linear_color_stop(rgba(0xffffff00), 1.0),
                    )),
            )
            .child(
                div()
                    .absolute()
                    .size_full()
                    .rounded(px(10.0))
                    .bg(linear_gradient(
                        0.0,
                        linear_color_stop(rgba(0x000000ff), 0.0),
                        linear_color_stop(rgba(0x00000000), 1.0),
                    )),
            )
            .child(
                knob(color)
                    .left(px(s * (WIDTH - 2.0 * PAD) - 8.0))
                    .top(px((1.0 - v) * SQUARE - 8.0)),
            )
            .child(tracker(Area::Square, picker.square.clone()));
        let hue_width = WIDTH - 2.0 * PAD - if dropper { ROW + 8.0 } else { 0.0 };
        let mut segments = div().absolute().size_full().flex().flex_row();
        for i in 0..6 {
            let from = from_hsv([i as f32 * 60.0, 1.0, 1.0]);
            let to = from_hsv([((i + 1) % 6) as f32 * 60.0, 1.0, 1.0]);
            segments = segments.child(
                div()
                    .flex_1()
                    .h_full()
                    .when(i == 0, |d| d.rounded_l(px(7.0)))
                    .when(i == 5, |d| d.rounded_r(px(7.0)))
                    .bg(linear_gradient(
                        90.0,
                        linear_color_stop(rgba(from), 0.0),
                        linear_color_stop(rgba(to), 1.0),
                    )),
            );
        }
        let hue = div()
            .id("picker-hue")
            .relative()
            .flex_1()
            .h(px(14.0))
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.set_dragging(Some(Area::Hue), cx);
                    this.drag_color(Area::Hue, event.position, cx);
                }),
            )
            .child(segments)
            .child(
                knob(pure)
                    .left(px(h / 360.0 * hue_width - 8.0))
                    .top(px(-1.0)),
            )
            .child(tracker(Area::Hue, picker.hue.clone()));
        let hue_row = div()
            .h(px(ROW))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .when(dropper, |d| {
                d.child(
                    div()
                        .id("picker-dropper")
                        .size(px(ROW))
                        .flex_none()
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(10.0))
                        .cursor_pointer()
                        .relative()
                        .child(crate::widgets::hover_fade("hover-glow", Some(10.0), th))
                        .tip(tr!("scheme-picker-dropper"), th)
                        .on_click(cx.listener(|this, _, _, cx| {
                            #[cfg(not(windows))]
                            this.pick_from_screen(cx);
                            #[cfg(windows)]
                            let _ = (this, cx);
                        }))
                        .child(icon("eyedropper", th.text_dim, 18.0)),
                )
            })
            .child(hue);
        let focus = picker.hex.focus_handle(cx);
        let focused = focus.is_focused(window);
        let hex_row = div()
            .h(px(ROW))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .size(px(ROW))
                    .flex_none()
                    .rounded(px(10.0))
                    .bg(rgba(color))
                    .border_1()
                    .border_color(rgba(fade(th.text, 0.25))),
            )
            .child(
                div()
                    .id("picker-hex")
                    .flex_1()
                    .h(px(ROW))
                    .px(px(10.0))
                    .flex()
                    .items_center()
                    .rounded(px(8.0))
                    .bg(rgba(th.chip))
                    .border_1()
                    .border_color(rgba(if focused { th.accent } else { 0x00000000 }))
                    .text_size(px(13.0))
                    .cursor_text()
                    .on_click(move |_, window, cx| window.focus(&focus, cx))
                    .child(div().flex_1().child(picker.hex.clone())),
            );
        let caption = |text: String| {
            div()
                .h(px(CAPTION))
                .text_size(px(12.0))
                .text_color(rgba(th.text_faint))
                .child(text)
        };
        let dots = |name: &'static str, colors: Vec<u32>, cx: &mut Context<Self>| {
            let mut row = div().mt(px(6.0)).h(px(DOT)).flex().flex_row().gap(px(6.0));
            for (ix, c) in colors.into_iter().enumerate() {
                row = row.child(
                    div()
                        .id(SharedString::from(format!("picker-{name}-{ix}")))
                        .size(px(DOT))
                        .rounded(px(7.0))
                        .bg(rgba(c))
                        .border_1()
                        .border_color(rgba(if c == color {
                            th.text
                        } else {
                            fade(th.text, 0.25)
                        }))
                        .cursor_pointer()
                        .tip(super::scheme_editor::hex(c), th)
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.set_picker_color(c, false, cx)),
                        ),
                );
            }
            row
        };
        let swatch_bounds = swatch;
        let popover = div()
            .id("color-picker")
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(WIDTH))
            .h(px(height))
            .p(px(PAD))
            .flex()
            .flex_col()
            .map(|d| notched::popover(d, th))
            .text_color(rgba(th.text))
            .on_mouse_down_out(cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                // The swatch's own click toggles the picker.
                if !swatch_bounds.contains(&event.position) {
                    this.close_color_picker(cx);
                }
            }))
            .when(standard, |d| {
                let mut row = div()
                    .h(px(DOT))
                    .mb(px(GAP))
                    .flex()
                    .flex_row()
                    .justify_between();
                for (ix, &(name, light, _)) in ACCOUNT_COLORS.iter().enumerate() {
                    row = row.child(
                        div()
                            .id(("picker-standard", ix))
                            .size(px(DOT))
                            .rounded_full()
                            .bg(rgba(light))
                            .when(light == color, |d| d.border_2().border_color(rgba(th.text)))
                            .cursor_pointer()
                            .tip(color_label(name), th)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.set_picker_color(light, false, cx)
                            })),
                    );
                }
                d.child(row)
            })
            .child(square)
            .child(div().mt(px(GAP)).child(hue_row))
            .child(div().mt(px(GAP)).child(hex_row))
            .when(!in_scheme.is_empty(), |d| {
                d.child(
                    div()
                        .mt(px(GAP))
                        .child(caption(tr!("scheme-picker-in-scheme"))),
                )
                .child(dots("scheme", in_scheme, cx))
            })
            .when(!recent.is_empty(), |d| {
                d.child(
                    div()
                        .mt(px(GAP))
                        .child(caption(tr!("scheme-picker-recent"))),
                )
                .child(dots("recent", recent, cx))
            })
            .when_some(system, |d, dialog| {
                d.child(
                    div().mt(px(GAP)).flex().child(
                        div()
                            .id("picker-system")
                            .h(px(LINK))
                            .px(px(6.0))
                            .ml(px(-6.0))
                            .flex()
                            .items_center()
                            .rounded(px(6.0))
                            .text_size(px(13.0))
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .relative()
                            .child(crate::widgets::hover_fade("hover-glow", Some(6.0), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.open_system_picker(dialog, cx)
                            }))
                            .child(tr!("scheme-picker-system")),
                    ),
                )
            })
            .children(notch(side_of, along, th));
        let popover = match fading {
            Some(_) => notched::fading(popover, "color-picker-out"),
            None => popover.into_any_element(),
        };
        let layer = div().relative().w(px(vw)).h(px(vh)).child(popover);
        Some(
            deferred(
                katna_ui::anchored()
                    .position(point(px(0.0), px(0.0)))
                    .child(layer),
            )
            .with_priority(5)
            .into_any_element(),
        )
    }
}

/// The desktop's own color dialog, where one is installed.
#[derive(Clone, Copy)]
enum SystemDialog {
    Kde,
    Gnome,
}

impl SystemDialog {
    fn find() -> Option<Self> {
        if cfg!(windows) {
            return None;
        }
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        let kde = desktop.split(':').any(|d| d.eq_ignore_ascii_case("KDE"));
        let installed = |name: &str| {
            std::env::var_os("PATH").is_some_and(|path| {
                std::env::split_paths(&path).any(|dir| dir.join(name).is_file())
            })
        };
        if kde && installed("kdialog") {
            Some(Self::Kde)
        } else if installed("zenity") {
            Some(Self::Gnome)
        } else if installed("kdialog") {
            Some(Self::Kde)
        } else {
            None
        }
    }
}

/// Beside the swatch if the popover fits there (right, then left), else
/// wherever [`notched::place`] puts it.
fn place_beside(
    swatch: Bounds<Pixels>,
    (w, h): (f32, f32),
    (vw, vh): (f32, f32),
) -> (f32, f32, notched::Side, f32) {
    let (left, top) = (unpx(swatch.origin.x), unpx(swatch.origin.y));
    let (right, bottom) = (
        left + unpx(swatch.size.width),
        top + unpx(swatch.size.height),
    );
    let cy = (top + bottom) / 2.0;
    let away = notched::NOTCH + notched::SPACE;
    let margin = notched::MARGIN;
    let y = (cy - 40.0).min(vh - h - margin).max(margin);
    let along = (cy - y).clamp(RADIUS + notched::NOTCH, h - RADIUS - notched::NOTCH);
    if right + away + w <= vw - margin {
        return (right + away, y, notched::Side::Right, along);
    }
    if left - away - w >= margin {
        return (left - away - w, y, notched::Side::Left, along);
    }
    notched::place(swatch, (w, h), (vw, vh), RADIUS)
}

/// `0xRRGGBBff` from hue (0..360), saturation and value (0..=1).
fn from_hsv([h, s, v]: [f32; 3]) -> u32 {
    let f = |n: f32| {
        let k = (n + h / 60.0) % 6.0;
        v - v * s * k.min(4.0 - k).clamp(0.0, 1.0)
    };
    let channel = |x: f32| (x * 255.0).round().clamp(0.0, 255.0) as u32;
    (channel(f(5.0)) << 24) | (channel(f(3.0)) << 16) | (channel(f(1.0)) << 8) | 0xff
}

/// Hue, saturation and value of `0xRRGGBBAA`.
fn to_hsv(color: u32) -> [f32; 3] {
    let channel = |shift: u32| ((color >> shift) & 0xff) as f32 / 255.0;
    let (r, g, b) = (channel(24), channel(16), channel(8));
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    let h = if d == 0.0 {
        0.0
    } else if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    [h * 60.0, if max == 0.0 { 0.0 } else { d / max }, max]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_round_trips() {
        for color in [
            0x000000ff, 0xffffffff, 0xff0000ff, 0x00ff00ff, 0x0000ffff, 0x4bc9d3ff, 0x131416ff,
            0xdf3131ff, 0x707070ff,
        ] {
            assert_eq!(from_hsv(to_hsv(color)), color, "{color:08x}");
        }
        assert_eq!(from_hsv([120.0, 1.0, 1.0]), 0x00ff00ff);
        assert_eq!(to_hsv(0x808080ff)[1], 0.0);
    }

    #[test]
    fn sits_beside_the_swatch() {
        use gpui::{point, size};
        let swatch = Bounds::new(point(px(100.0), px(300.0)), size(px(22.0), px(22.0)));
        let (x, _, side, _) = place_beside(swatch, (WIDTH, 300.0), (1200.0, 800.0));
        assert_eq!(side, notched::Side::Right);
        assert!(x > 122.0);
        // Near the right edge it opens to the left.
        let swatch = Bounds::new(point(px(1000.0), px(300.0)), size(px(22.0), px(22.0)));
        let (x, _, side, _) = place_beside(swatch, (WIDTH, 300.0), (1200.0, 800.0));
        assert_eq!(side, notched::Side::Left);
        assert!(x + WIDTH < 1000.0);
    }
}
