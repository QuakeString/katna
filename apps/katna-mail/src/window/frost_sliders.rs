// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Experimental > Blur > Custom blur amount: how far the
//! frosted menus and dialogs blur what is under them, and how opaque their
//! tint is. Dragging shows the frost change as it goes and saves on
//! letting go; the arrow keys, Home, End and the mouse wheel step it. With
//! the switch off, the frost follows KDE's blur strength, or Katna's own
//! when there is none (`katna_platform::blur`).

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, Bounds, Context, DispatchPhase, KeyDownEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, ScrollWheelEvent, canvas, div, prelude::*, relative,
    rgba,
};
use katna_core::config::{FROST_BLUR, FROST_OPACITY};
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

/// One of the two sliders.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Amount {
    /// The blur, in pixels.
    Blur,
    /// The tint's opacity, in percent.
    Opacity,
}

impl Amount {
    /// The range and the step.
    fn range(self) -> (u8, u8, u8) {
        match self {
            Amount::Blur => (4, 48, 2),
            Amount::Opacity => (10, 95, 5),
        }
    }

    fn default(self) -> u8 {
        match self {
            Amount::Blur => FROST_BLUR,
            Amount::Opacity => FROST_OPACITY,
        }
    }

    fn change(self, value: u8) -> Change {
        match self {
            Amount::Blur => Change::FrostBlur(value),
            Amount::Opacity => Change::FrostOpacity(value),
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

/// Drags and wheel turns on the two sliders.
#[derive(Default)]
pub(super) struct FrostDrag {
    /// The slider being dragged, and the value under the pointer.
    dragging: Option<(Amount, u8)>,
    /// The tracks, as last laid out.
    tracks: [Rc<Cell<Option<Bounds<Pixels>>>>; 2],
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

    fn saved_amount(&self, amount: Amount) -> u8 {
        let experimental = &self.config.experimental;
        match amount {
            Amount::Blur => experimental.frost_blur,
            Amount::Opacity => experimental.frost_opacity,
        }
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
        let id = match amount {
            Amount::Blur => "page-frost-blur",
            Amount::Opacity => "page-frost-opacity",
        };
        let slider = self
            .page_control(div().id(id), th, cx)
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
                            .left(relative(amount.fraction(amount.default())))
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
        for amount in [Amount::Blur, Amount::Opacity] {
            let (min, max, step) = amount.range();
            assert_eq!((max - min) % step, 0);
            assert_eq!(amount.value_at(track, px(0.0)), min);
            assert_eq!(amount.value_at(track, px(400.0)), max);
            let default = amount.default();
            assert_eq!((default - min) % step, 0);
            let x = px(100.0 + 220.0 * amount.fraction(default));
            assert_eq!(amount.value_at(track, x), default);
        }
    }
}
