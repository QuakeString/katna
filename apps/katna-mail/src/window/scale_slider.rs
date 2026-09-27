// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Appearance > Scaling: a slider from a small "A" to a large
//! one, with a mark at 100%. Dragging previews the value and lets go to
//! apply it, so the slider doesn't grow under the pointer; the arrow keys,
//! Home and End apply at once. See `katna_ui::scale`.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    AnyElement, Bounds, Context, DispatchPhase, KeyDownEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, canvas, div, prelude::*, relative, rgba,
};
use katna_i18n::tr;
use katna_ui::px;

use super::MailWindow;
use super::settings::Change;
use crate::theme::Theme;
use crate::widgets::outlined_button;

/// The range, in percent, and the step.
pub(super) const MIN: u16 = 75;
pub(super) const MAX: u16 = 200;
const STEP: u16 = 5;
/// The knob's width.
const KNOB: f32 = 18.0;

/// Where a drag on the slider is.
#[derive(Default)]
pub(super) struct ScaleDrag {
    /// The value under the pointer while dragging.
    pub(super) value: Option<u16>,
    /// The track, as last laid out.
    pub(super) track: Rc<Cell<Option<Bounds<Pixels>>>>,
}

/// The value at `x` on `track`.
fn value_at(track: Bounds<Pixels>, x: Pixels) -> u16 {
    let t = if track.size.width > px(0.0) {
        ((x - track.left()) / track.size.width).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let value = f32::from(MIN) + t * f32::from(MAX - MIN);
    ((value / f32::from(STEP)).round() as u16 * STEP).clamp(MIN, MAX)
}

/// How far along the track `value` is, 0 to 1.
fn fraction(value: u16) -> f32 {
    f32::from(value.clamp(MIN, MAX) - MIN) / f32::from(MAX - MIN)
}

impl MailWindow {
    pub(super) fn scale_control(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let applied = self.config.mail.scale.clamp(MIN, MAX);
        let (dragging, track) = match &self.settings_page {
            Some(page) => (page.scale.value, page.scale.track.clone()),
            None => (None, Rc::default()),
        };
        let value = dragging.unwrap_or(applied);
        let at = fraction(value);
        let entity = cx.entity().downgrade();
        let down_track = track.clone();
        let paint_track = track.clone();
        let slider = self
            .page_control(div().id("page-scale"), th, cx)
            .flex_1()
            .min_w(px(160.0))
            .h(px(36.0))
            .px(px(KNOB / 2.0 + 4.0))
            .rounded(px(8.0))
            .cursor_pointer()
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                let now = this.config.mail.scale.clamp(MIN, MAX);
                let next = match event.keystroke.key.as_str() {
                    "left" | "down" => now.saturating_sub(STEP),
                    "right" | "up" => now + STEP,
                    "home" => MIN,
                    "end" => MAX,
                    _ => return,
                };
                cx.stop_propagation();
                this.apply(Change::Scale(next.clamp(MIN, MAX)), cx);
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    let Some(track) = down_track.get() else {
                        return;
                    };
                    if let Some(page) = this.settings_page.as_mut() {
                        page.scale.value = Some(value_at(track, event.position.x));
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
                    // The mark at 100%.
                    .child(
                        div()
                            .absolute()
                            .left(relative(fraction(100)))
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
                            .border_2()
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
                                        let value = value_at(bounds, event.position.x);
                                        moved
                                            .update(cx, |this, cx| {
                                                let Some(page) = this.settings_page.as_mut() else {
                                                    return;
                                                };
                                                if page.scale.value.is_some()
                                                    && page.scale.value != Some(value)
                                                {
                                                    page.scale.value = Some(value);
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
                                            let value = this
                                                .settings_page
                                                .as_mut()
                                                .and_then(|page| page.scale.value.take());
                                            if let Some(value) = value {
                                                this.apply(Change::Scale(value), cx);
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
        let letter = |size: f32| {
            div()
                .flex_none()
                .w(px(24.0))
                .flex()
                .justify_center()
                .text_size(px(size))
                .text_color(rgba(th.text_dim))
                .child(tr!("scale-letter"))
        };
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(4.0))
                    .child(letter(13.0))
                    .child(slider)
                    .child(letter(24.0)),
            )
            .child(
                div()
                    .h(px(36.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("scale-percent", percent = value)),
                    )
                    .when(applied != 100 && dragging.is_none(), |d| {
                        d.child(
                            outlined_button(
                                "page-scale-reset",
                                tr!("scale-reset", percent = 100),
                                th,
                            )
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(
                                cx.listener(|this, _, _, cx| this.apply(Change::Scale(100), cx)),
                            ),
                        )
                    }),
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
        let track = Bounds::new(point(px(100.0), px(0.0)), size(px(250.0), px(10.0)));
        assert_eq!(value_at(track, px(0.0)), MIN);
        assert_eq!(value_at(track, px(400.0)), MAX);
        assert_eq!(value_at(track, px(100.0 + 250.0 * fraction(100))), 100);
        assert_eq!(value_at(track, px(100.0 + 250.0 * fraction(152))), 150);
    }
}
