// SPDX-License-Identifier: GPL-3.0-or-later

//! The sliders of Settings > Experimental. Blur > Custom blur amount: how
//! far the frosted menus and dialogs blur what is under them, and how
//! opaque their tint and a blurred window background are. With the switch
//! off, the frost follows KDE's blur strength, or Katna's own when there is
//! none (`katna_platform::blur`). Under the window blur, how opaque the
//! frosted cards are. Window frame > Katna: the corners'
//! roundness and the border's opacity. Dragging shows the change as it goes
//! and saves on letting go; the arrow keys, Home, End and the mouse wheel
//! step it.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, Bounds, Context, DispatchPhase, KeyDownEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, ScrollWheelEvent, canvas, div, prelude::*, relative,
    rgba,
};
use katna_core::config::{FROST_BLUR, FROST_OPACITY, PANE_OPACITY};
use katna_i18n::tr;
use katna_platform::blur;
use katna_ui::px;

use super::MailWindow;
use super::account_roll::Notches;
use super::settings::Change;
use crate::theme::Theme;
use crate::widgets::ScaledEdge;

/// The knob's width.
const KNOB: f32 = 18.0;
/// The frost's blur at KDE's lightest strength, in pixels; KDE's strongest
/// (and default) is Katna's default, [`FROST_BLUR`].
const KDE_LIGHT: f32 = 4.0;
/// How much clearer than the cards the room behind a chat's bubbles is,
/// in percent: the bubbles carry their own fill.
const CHAT_CLEARER: u8 = 5;
/// How opaque that room is, in percent, when the cards are solid.
const CHAT_OPACITY: u8 = 70;
/// How opaque the open search box is, in percent, like a menu's glass a
/// little more solid, so what is typed stays clear.
const SEARCH_OPACITY: u8 = 62;

/// The corners' roundness, in pixels, on the slider and in its field.
pub(super) const RADIUS_RANGE: std::ops::RangeInclusive<u32> = 0..=32;

/// One of the sliders.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Amount {
    /// The blur, in pixels.
    Blur,
    /// The tint's opacity, in percent.
    Opacity,
    /// The window's corner radius, in pixels.
    Radius,
    /// The window border's opacity, in percent.
    BorderOpacity,
    /// The frosted cards' opacity, in percent.
    PaneOpacity,
}

impl Amount {
    /// The range and the step.
    fn range(self) -> (u8, u8, u8) {
        match self {
            Amount::Blur => (4, 48, 2),
            Amount::Opacity => (10, 95, 5),
            Amount::Radius => (0, *RADIUS_RANGE.end() as u8, 1),
            Amount::BorderOpacity => (5, 100, 5),
            Amount::PaneOpacity => (30, 95, 5),
        }
    }

    /// Katna's own value, where it does not depend on the desktop.
    fn default(self) -> Option<u8> {
        match self {
            Amount::Blur => Some(FROST_BLUR),
            Amount::Opacity => Some(FROST_OPACITY),
            Amount::PaneOpacity => Some(PANE_OPACITY),
            Amount::Radius | Amount::BorderOpacity => None,
        }
    }

    fn change(self, value: u8) -> Change {
        match self {
            Amount::Blur => Change::FrostBlur(value),
            Amount::Opacity => Change::FrostOpacity(value),
            Amount::Radius => Change::WindowRadius(value),
            Amount::BorderOpacity => Change::WindowBorderOpacity(value),
            Amount::PaneOpacity => Change::PaneOpacity(value),
        }
    }

    fn id(self) -> &'static str {
        match self {
            Amount::Blur => "page-frost-blur",
            Amount::Opacity => "page-frost-opacity",
            Amount::Radius => "page-window-radius",
            Amount::BorderOpacity => "page-window-border-opacity",
            Amount::PaneOpacity => "page-pane-opacity",
        }
    }

    fn index(self) -> usize {
        self as usize
    }

    fn clamp(self, value: i32) -> u8 {
        let (min, max, _) = self.range();
        value.clamp(i32::from(min), i32::from(max)) as u8
    }

    /// The value at `x` on `track`, on a step.
    fn value_at(self, track: Bounds<Pixels>, x: Pixels) -> u8 {
        let (min, max, step) = self.range();
        let t = if track.size.width > px(0.0) {
            ((x - track.left()) / track.size.width).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let value = f32::from(min) + t * f32::from(max - min);
        let steps = ((value - f32::from(min)) / f32::from(step)).round() as i32;
        self.clamp(i32::from(min) + steps * i32::from(step))
    }

    /// How far along the track `value` is, 0 to 1.
    fn fraction(self, value: u8) -> f32 {
        let (min, max, _) = self.range();
        f32::from(value.clamp(min, max) - min) / f32::from(max - min)
    }
}

/// Drags and wheel turns on the sliders.
#[derive(Default)]
pub(super) struct FrostDrag {
    /// The slider being dragged, and the value under the pointer.
    dragging: Option<(Amount, u8)>,
    /// The tracks, as last laid out.
    tracks: [Rc<Cell<Option<Bounds<Pixels>>>>; 5],
    wheel: Notches,
}

impl FrostDrag {
    fn value(&self, amount: Amount) -> Option<u8> {
        self.dragging
            .filter(|(dragged, _)| *dragged == amount)
            .map(|(_, value)| value)
    }
}

impl MailWindow {
    /// The frost's blur, in pixels, and its tint's opacity, in percent:
    /// the sliders' (as dragged), or KDE's blur strength and Katna's
    /// opacity.
    pub(super) fn frost_amount(&self) -> (f32, u8) {
        let experimental = &self.config.experimental;
        if experimental.custom_frost {
            let drag = self.settings_page.as_ref().map(|page| &page.frost);
            let value = |amount: Amount, saved: u8| {
                drag.and_then(|drag| drag.value(amount)).unwrap_or(saved)
            };
            return (
                f32::from(value(Amount::Blur, experimental.frost_blur)),
                value(Amount::Opacity, experimental.frost_opacity),
            );
        }
        let full = f32::from(FROST_BLUR);
        let blur = self
            .desktop_colors
            .kde_blur
            .map_or(full, |strength| blur::kde_radius(strength, KDE_LIGHT, full));
        (blur, FROST_OPACITY)
    }

    /// How opaque the cards, the room behind a chat and the open search
    /// box are in a blurred window, in percent, each `None` while solid:
    /// the switches under Blur the window background, with the cards'
    /// slider as dragged.
    pub(super) fn pane_frost(&self) -> (Option<u8>, Option<u8>, Option<u8>) {
        let experimental = &self.config.experimental;
        let dragged = self
            .settings_page
            .as_ref()
            .and_then(|page| page.frost.value(Amount::PaneOpacity));
        let pane = experimental
            .frosted_panes
            .then(|| dragged.unwrap_or(experimental.pane_opacity));
        let chat = experimental
            .frosted_chat
            .then(|| pane.map_or(CHAT_OPACITY, |p| p.saturating_sub(CHAT_CLEARER)));
        let search = experimental.frosted_search.then_some(SEARCH_OPACITY);
        (pane, chat, search)
    }

    /// The corners' roundness now, in pixels: as set, or the frame's own.
    pub(super) fn window_radius_now(&self) -> u8 {
        self.saved_amount(Amount::Radius)
    }

    /// The roundness typed in its field, applied as it is typed.
    pub(super) fn type_window_radius(&mut self, text: &str, cx: &mut Context<Self>) {
        let Ok(value) = text.trim().parse::<u32>() else {
            return;
        };
        let value = value.clamp(*RADIUS_RANGE.start(), *RADIUS_RANGE.end()) as u8;
        if Some(value) != self.config.experimental.window_radius {
            self.apply(Change::WindowRadius(value), cx);
        }
    }

    /// Shows `value` in the roundness field, unless it already says so.
    pub(super) fn sync_radius_field(&self, value: u8, cx: &mut Context<Self>) {
        let Some(input) = self.settings_page.as_ref().map(|page| page.radius.clone()) else {
            return;
        };
        let typed = input.read(cx).text().trim().parse::<u32>().ok();
        if typed != Some(u32::from(value)) {
            input.update(cx, |input, cx| input.set_text(value.to_string(), cx));
        }
    }

    fn saved_amount(&self, amount: Amount) -> u8 {
        let experimental = &self.config.experimental;
        let (radius, border) = self.natural_corners();
        match amount {
            Amount::Blur => experimental.frost_blur,
            Amount::Opacity => experimental.frost_opacity,
            Amount::Radius => experimental.window_radius.unwrap_or(radius),
            Amount::BorderOpacity => experimental.window_border_opacity.unwrap_or(border),
            Amount::PaneOpacity => experimental.pane_opacity,
        }
    }

    /// The frame's own corner radius and border opacity, before the
    /// settings change them.
    fn natural_corners(&self) -> (u8, u8) {
        let (radius, border) = self.chrome.natural_corners();
        (radius.round().clamp(0.0, 255.0) as u8, border)
    }

    /// Where the slider's mark goes: Katna's default, or the frame's own.
    fn mark(&self, amount: Amount) -> u8 {
        let (radius, border) = self.natural_corners();
        amount.default().unwrap_or(match amount {
            Amount::Radius => radius,
            _ => border,
        })
    }

    /// The window's look with the slider being dragged, so the frame and a
    /// blurred background change as it moves.
    fn look_now(&self) -> katna_chrome::Look {
        let mut look = super::look(&self.config);
        let dragging = self
            .settings_page
            .as_ref()
            .and_then(|page| page.frost.dragging);
        match dragging {
            Some((Amount::Radius, value)) => look.radius = Some(value),
            Some((Amount::BorderOpacity, value)) => look.border_opacity = Some(value),
            Some((Amount::Opacity, value)) if self.config.experimental.custom_frost => {
                look.blur_opacity = Some(super::look::window_opacity(value));
            }
            _ => {}
        }
        look
    }

    /// Moves `amount` by `steps` steps and saves it.
    fn step_amount(&mut self, amount: Amount, steps: i32, cx: &mut Context<Self>) {
        let (_, _, step) = amount.range();
        let now = i32::from(self.saved_amount(amount));
        let next = amount.clamp(now + steps * i32::from(step));
        self.apply(amount.change(next), cx);
    }

    /// The two sliders under the Custom blur amount switch.
    pub(super) fn frost_sliders(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .px(px(8.0))
            .pb(px(4.0))
            .child(self.frost_slider(
                Amount::Blur,
                tr!("look-frost-blur"),
                tr!("look-frost-blur-light"),
                tr!("look-frost-blur-strong"),
                th,
                cx,
            ))
            .child(self.frost_slider(
                Amount::Opacity,
                tr!("look-frost-opacity"),
                tr!("look-frost-opacity-clear"),
                tr!("look-frost-opacity-solid"),
                th,
                cx,
            ))
            .into_any_element()
    }

    /// The frosted cards' opacity slider, under its switch.
    pub(super) fn pane_slider(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .px(px(8.0))
            .pb(px(4.0))
            .child(self.frost_slider(
                Amount::PaneOpacity,
                tr!("look-pane-opacity"),
                tr!("look-frost-opacity-clear"),
                tr!("look-frost-opacity-solid"),
                th,
                cx,
            ))
            .into_any_element()
    }

    /// The corners' roundness slider, or the border's opacity
    /// (`border`), under Window frame > Katna.
    pub(super) fn frame_sliders(
        &self,
        border: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let slider = if border {
            self.frost_slider(
                Amount::BorderOpacity,
                tr!("look-window-border-opacity"),
                tr!("look-window-border-faint"),
                tr!("look-window-border-strong"),
                th,
                cx,
            )
        } else {
            let slider = self.frost_slider(
                Amount::Radius,
                tr!("look-window-radius"),
                tr!("look-window-radius-square"),
                tr!("look-window-radius-round"),
                th,
                cx,
            );
            // Beside it, the number to type.
            let field = self.settings_page.as_ref().map(|page| {
                let input = page.radius.clone();
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(super::settings_page::number_field(
                        "page-window-radius-field",
                        &input,
                        RADIUS_RANGE,
                        th,
                        cx,
                    ))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("look-px")),
                    )
            });
            div()
                .flex()
                .items_center()
                .gap(px(16.0))
                .child(div().flex_1().min_w_0().child(slider))
                .children(field)
                .into_any_element()
        };
        div()
            .px(px(8.0))
            .pb(px(4.0))
            .child(slider)
            .into_any_element()
    }

    fn frost_slider(
        &self,
        amount: Amount,
        label: String,
        low: String,
        high: String,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let saved = self.saved_amount(amount);
        let (dragging, track) = match &self.settings_page {
            Some(page) => (
                page.frost.value(amount),
                page.frost.tracks[amount.index()].clone(),
            ),
            None => (None, Rc::default()),
        };
        let at = amount.fraction(dragging.unwrap_or(saved));
        let entity = cx.entity().downgrade();
        let down_track = track.clone();
        let paint_track = track.clone();
        let mark = amount.fraction(self.mark(amount));
        let slider = self
            .page_control(div().id(amount.id()), th, cx)
            .h(px(36.0))
            .px(px(KNOB / 2.0 + 4.0))
            .rounded(px(8.0))
            .cursor_pointer()
            .on_key_down(cx.listener(move |this, event: &KeyDownEvent, _, cx| {
                let (min, max, _) = amount.range();
                match event.keystroke.key.as_str() {
                    "left" | "down" => this.step_amount(amount, -1, cx),
                    "right" | "up" => this.step_amount(amount, 1, cx),
                    "home" => this.apply(amount.change(min), cx),
                    "end" => this.apply(amount.change(max), cx),
                    _ => return,
                }
                cx.stop_propagation();
            }))
            // Up for more, down for less.
            .on_scroll_wheel(cx.listener(move |this, event: &ScrollWheelEvent, _, cx| {
                cx.stop_propagation();
                let Some(page) = this.settings_page.as_mut() else {
                    return;
                };
                let step = page.frost.wheel.turn(event.delta, Instant::now());
                if step != 0 {
                    this.step_amount(amount, -step, cx);
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    let Some(track) = down_track.get() else {
                        return;
                    };
                    if let Some(page) = this.settings_page.as_mut() {
                        page.frost.dragging =
                            Some((amount, amount.value_at(track, event.position.x)));
                        cx.notify();
                    }
                }),
            )
            .child(
                div()
                    .relative()
                    .size_full()
                    // The line, filled up to the knob.
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .top(px(16.0))
                            .h(px(4.0))
                            .rounded_full()
                            .bg(rgba(th.divider)),
                    )
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .top(px(16.0))
                            .h(px(4.0))
                            .w(relative(at))
                            .rounded_full()
                            .bg(rgba(th.accent)),
                    )
                    // The mark at Katna's default.
                    .child(
                        div()
                            .absolute()
                            .left(relative(mark))
                            .top(px(6.0))
                            .w(px(1.0))
                            .h(px(24.0))
                            .bg(rgba(th.text_faint)),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(relative(at))
                            .ml(px(-KNOB / 2.0))
                            .top(px(18.0 - KNOB / 2.0))
                            .size(px(KNOB))
                            .rounded_full()
                            .bg(rgba(th.accent))
                            .border_px(2.0)
                            .border_color(rgba(th.surface)),
                    )
                    .child(
                        canvas(
                            move |bounds, _, _| track.set(Some(bounds)),
                            move |_, _, window, _| {
                                if dragging.is_none() {
                                    return;
                                }
                                // While dragging, the pointer counts
                                // anywhere in the window.
                                let (moved, released) = (entity.clone(), entity.clone());
                                let track = paint_track.clone();
                                window.on_mouse_event(
                                    move |event: &MouseMoveEvent, phase, _, cx| {
                                        let Some(bounds) = track.get() else {
                                            return;
                                        };
                                        if phase != DispatchPhase::Bubble {
                                            return;
                                        }
                                        let value = amount.value_at(bounds, event.position.x);
                                        moved
                                            .update(cx, |this, cx| {
                                                let Some(page) = this.settings_page.as_mut() else {
                                                    return;
                                                };
                                                if page.frost.dragging.is_some()
                                                    && page.frost.value(amount) != Some(value)
                                                {
                                                    page.frost.dragging = Some((amount, value));
                                                    if amount == Amount::Radius {
                                                        this.sync_radius_field(value, cx);
                                                    }
                                                    let look = this.look_now();
                                                    cx.set_global(look);
                                                    cx.notify();
                                                }
                                            })
                                            .ok();
                                    },
                                );
                                window.on_mouse_event(move |_: &MouseUpEvent, phase, _, cx| {
                                    if phase != DispatchPhase::Bubble {
                                        return;
                                    }
                                    released
                                        .update(cx, |this, cx| {
                                            let dragged = this
                                                .settings_page
                                                .as_mut()
                                                .and_then(|page| page.frost.dragging.take());
                                            if let Some((amount, value)) = dragged {
                                                this.apply(amount.change(value), cx);
                                                cx.notify();
                                            }
                                        })
                                        .ok();
                                });
                            },
                        )
                        .absolute()
                        .size_full(),
                    ),
            );
        let end = |text: String| {
            div()
                .text_size(px(12.0))
                .text_color(rgba(th.text_faint))
                .child(text)
        };
        div()
            .flex()
            .flex_col()
            .pt(px(8.0))
            .child(div().text_size(px(14.0)).child(label))
            .child(slider)
            .child(
                div()
                    .px(px(4.0))
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(end(low))
                    .child(end(high)),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use gpui::{point, size};

    use super::*;

    #[test]
    fn values_snap_to_steps_inside_the_range() {
        let track = Bounds::new(point(px(100.0), px(0.0)), size(px(220.0), px(10.0)));
        for amount in [
            Amount::Blur,
            Amount::Opacity,
            Amount::Radius,
            Amount::BorderOpacity,
            Amount::PaneOpacity,
        ] {
            let (min, max, step) = amount.range();
            assert_eq!((max - min) % step, 0);
            assert_eq!(amount.value_at(track, px(0.0)), min);
            assert_eq!(amount.value_at(track, px(400.0)), max);
            let Some(default) = amount.default() else {
                continue;
            };
            assert_eq!((default - min) % step, 0);
            let x = px(100.0 + 220.0 * amount.fraction(default));
            assert_eq!(amount.value_at(track, x), default);
        }
    }
}
