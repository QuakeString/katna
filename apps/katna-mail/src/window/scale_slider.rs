// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Appearance > Scaling: a switch to follow the desktop's
//! scale, and under it a slider for Katna's own, with the value at its
//! end. While the switch is on the slider is dimmed at the desktop's
//! value. Dragging previews the value and lets go to apply it, so the
//! slider doesn't grow under the pointer; the arrow keys, Home and End
//! apply at once. The value counts the screen's pixels as the desktop
//! does, so 200% is twice as big as at a desktop scale of 100% on any
//! screen; see `katna_ui::scale`.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    AnyElement, Bounds, Context, DispatchPhase, KeyDownEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Window, canvas, div, prelude::*, relative, rgba,
};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{space, state, text};

use super::MailWindow;
use super::settings::Change;
use crate::theme::Theme;
use crate::widgets::ScaledEdge;

/// The range, in percent, and the step.
pub(super) const MIN: u16 = 75;
pub(super) const MAX: u16 = 300;
const STEP: u16 = 5;
/// Room for the value at the slider's end, so the slider keeps its
/// length from 75% to 300%.
const VALUE_WIDTH: f32 = 44.0;
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
fn value_at(track: Bounds<Pixels>, x: Pixels, rtl: bool) -> u16 {
    let t = if track.size.width > px(0.0) {
        (katna_ui::direction::from_start(x, track, rtl) / track.size.width).clamp(0.0, 1.0)
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

/// `percent` on the slider's steps, inside its range.
fn snap(percent: f32) -> u16 {
    let percent = if percent.is_finite() { percent } else { 100.0 };
    ((percent / f32::from(STEP)).round() as u16 * STEP).clamp(MIN, MAX)
}

/// The desktop's scale, in whole percent.
fn desktop_percent(desktop: f32) -> u16 {
    (desktop * 100.0).round().clamp(1.0, f32::from(u16::MAX)) as u16
}

/// What Katna's lengths are multiplied by for its own scale `own` on a
/// desktop at `desktop`; 1 while it follows the desktop.
pub(super) fn factor(own: Option<u16>, desktop: f32) -> f32 {
    match own {
        Some(percent) if desktop > 0.0 => f32::from(percent) / 100.0 / desktop,
        _ => 1.0,
    }
}

impl MailWindow {
    /// Reads the desktop's scale for this window, which changes as it
    /// moves between screens, and scales Katna to match: the desktop's
    /// own, or Katna's own in its place. An older version's scale on top
    /// of the desktop's becomes Katna's own here, the first time the
    /// desktop's is known.
    pub(super) fn sync_scale(&mut self, window: &Window, cx: &mut Context<Self>) {
        let desktop = window.scale_factor();
        if !desktop.is_finite() || desktop <= 0.0 {
            return;
        }
        self.desktop_scale = desktop;
        let view = &mut self.config.mail;
        if view.own_scale.is_none() && view.scale != 100 {
            view.own_scale = Some(snap(f32::from(view.scale) * desktop));
            view.scale = 100;
            self.save_config();
        }
        let want = factor(self.config.mail.own_scale, desktop)
            .clamp(katna_ui::scale::MIN, katna_ui::scale::MAX);
        if (want - katna_ui::scale::scale()).abs() > 0.001 {
            katna_ui::scale::set_scale(want);
            // Every row is a new height; the other windows draw again
            // after this one.
            self.list_state.remeasure();
            cx.defer(|cx| cx.refresh_windows());
        }
    }

    pub(super) fn scale_control(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let desktop = desktop_percent(self.desktop_scale);
        let own = self.config.mail.own_scale;
        let follows = own.is_none();
        let applied = own.map_or(desktop, |own| own.clamp(MIN, MAX));
        let (dragging, track) = match &self.settings_page {
            Some(page) if !follows => (page.scale.value, page.scale.track.clone()),
            _ => (None, Rc::default()),
        };
        let value = dragging.unwrap_or(applied);
        let at = fraction(value);
        let entity = cx.entity().downgrade();
        let down_track = track.clone();
        let paint_track = track.clone();
        let slider = div()
            .id("page-scale")
            .flex_1()
            .min_w(px(160.0))
            .h(px(36.0))
            .px(px(KNOB / 2.0))
            .rounded(px(katna_ui::tokens::radius::SM));
        // While it follows the desktop, the slider shows the desktop's
        // scale in the disabled colour. Mixed rather than see-through, so
        // the line doesn't show through the knob.
        let fill = if follows {
            crate::theme::mix(th.surface, th.accent, state::DISABLED)
        } else {
            th.accent
        };
        let slider = if follows {
            slider
        } else {
            self.page_control(slider, th, cx)
                .cursor_pointer()
                .on_key_down(cx.listener(move |this, event: &KeyDownEvent, window, cx| {
                    let now = this
                        .config
                        .mail
                        .own_scale
                        .unwrap_or(desktop)
                        .clamp(MIN, MAX);
                    let next = match katna_ui::direction::arrow(
                        &event.keystroke.key,
                        katna_ui::direction::is_rtl(window),
                    ) {
                        "left" | "down" => now.saturating_sub(STEP),
                        "right" | "up" => now + STEP,
                        "home" => MIN,
                        "end" => MAX,
                        _ => return,
                    };
                    cx.stop_propagation();
                    this.apply(Change::Scale(Some(next.clamp(MIN, MAX))), cx);
                }))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                        let Some(track) = down_track.get() else {
                            return;
                        };
                        if let Some(page) = this.settings_page.as_mut() {
                            page.scale.value = Some(value_at(
                                track,
                                event.position.x,
                                katna_ui::direction::is_rtl(window),
                            ));
                            cx.notify();
                        }
                    }),
                )
        };
        let slider = slider.child(
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
                        .bg(rgba(th.switch_off)),
                )
                .child(
                    div()
                        .absolute()
                        .left_0()
                        .top(px(16.0))
                        .h(px(4.0))
                        .w(relative(at))
                        .rounded_full()
                        .bg(rgba(fill)),
                )
                // The mark at the desktop's scale.
                .when(!follows && (MIN..=MAX).contains(&desktop), |d| {
                    d.child(
                        div()
                            .absolute()
                            .left(relative(fraction(desktop)))
                            .top(px(6.0))
                            .w(px(1.0))
                            .h(px(24.0))
                            .bg(rgba(th.text_faint)),
                    )
                })
                .child(
                    div()
                        .absolute()
                        .left(relative(at))
                        .ml(px(-KNOB / 2.0))
                        .top(px(18.0 - KNOB / 2.0))
                        .size(px(KNOB))
                        .rounded_full()
                        .bg(rgba(fill))
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
                                move |event: &MouseMoveEvent, phase, window, cx| {
                                    let Some(bounds) = track.get() else {
                                        return;
                                    };
                                    if phase != DispatchPhase::Bubble {
                                        return;
                                    }
                                    let value = value_at(
                                        bounds,
                                        event.position.x,
                                        katna_ui::direction::is_rtl(window),
                                    );
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
                                            this.apply(Change::Scale(Some(value)), cx);
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
        let value_text = div()
            .flex_none()
            .w(px(VALUE_WIDTH))
            .flex()
            .justify_end()
            .text_size(px(text::BODY))
            .text_color(rgba(if follows { th.text_dim } else { th.accent }))
            .child(tr!("scale-percent", percent = value));
        div()
            .flex()
            .flex_col()
            .gap(px(space::S1))
            .child(self.switch_row(
                "page-scale-desktop",
                tr!("scale-desktop"),
                tr!("scale-desktop-detail", percent = desktop),
                follows,
                // Off starts Katna's own at the desktop's, so nothing moves.
                Change::Scale(if follows {
                    Some(snap(f32::from(desktop)))
                } else {
                    None
                }),
                th,
                cx,
            ))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S4))
                    // In line with the switch row's text and switch: the
                    // knob's edge under the text, the value under the
                    // switch's end.
                    .px(px(space::S3))
                    .child(slider)
                    .child(value_text),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use gpui::{point, size};

    use super::*;

    #[test]
    fn own_scale_replaces_the_desktops() {
        assert_eq!(factor(None, 1.8), 1.0);
        assert!((factor(Some(180), 1.8) - 1.0).abs() < 1e-6);
        assert!((factor(Some(100), 2.0) - 0.5).abs() < 1e-6);
        assert_eq!(snap(1.25 * 180.0), 225);
        assert_eq!(snap(f32::NAN), 100);
        assert_eq!(desktop_percent(1.75), 175);
    }

    #[test]
    fn values_snap_to_steps_inside_the_range() {
        let track = Bounds::new(point(px(100.0), px(0.0)), size(px(250.0), px(10.0)));
        assert_eq!(value_at(track, px(0.0), false), MIN);
        assert_eq!(value_at(track, px(400.0), false), MAX);
        assert_eq!(
            value_at(track, px(100.0 + 250.0 * fraction(100)), false),
            100
        );
        assert_eq!(
            value_at(track, px(100.0 + 250.0 * fraction(152)), false),
            150
        );
        // Right to left, the track starts at its right end.
        assert_eq!(value_at(track, px(400.0), true), MIN);
        assert_eq!(value_at(track, px(0.0), true), MAX);
    }
}
