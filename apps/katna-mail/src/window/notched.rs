// SPDX-License-Identifier: GPL-3.0-or-later

//! Popovers with a notch pointing at what they belong to (the search's
//! custom dates, a Year view day, the contact peek, the colour picker, who
//! read a mail): the panel itself ([`popover`]), where it goes beside what
//! it points at, kept inside the window ([`place`]), and the notch on the
//! side facing it ([`notch`]). One radius, edge and depth for all of them
//! (`docs/DESIGN.md`).

use std::f32::consts::{FRAC_PI_2, PI};

use gpui::{
    AnyElement, Bounds, InteractiveElement, MouseButton, Pixels, Transformation, prelude::*,
    radians, rgba, svg,
};
use katna_ui::tokens::{elevation, radius};
use katna_ui::{px, unpx};

use crate::theme::Theme;
use crate::widgets::raised;

/// Every popover's corner radius.
pub(super) const RADIUS: f32 = radius::LG;

/// The notch's length out of the popover, and the popover's distance from
/// the chip and from the window's edges.
pub(super) const NOTCH: f32 = 10.0;
pub(super) const SPACE: f32 = 2.0;
pub(super) const MARGIN: f32 = 8.0;

/// Where a popover sits against what it points at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Side {
    Below,
    Above,
    Right,
    Left,
}

/// The popover's top left corner (`size`, corners of `radius`), its side
/// of the chip and where the notch
/// meets its edge (from its left for Below/Above, from its top otherwise).
/// Below the chip if it fits, else above, right, left; else below, kept
/// inside the window.
pub(super) fn place(
    chip: Bounds<Pixels>,
    size: (f32, f32),
    viewport: (f32, f32),
    radius: f32,
) -> (f32, f32, Side, f32) {
    let (w, h) = size;
    let (vw, vh) = viewport;
    let (left, top) = (unpx(chip.origin.x), unpx(chip.origin.y));
    let (right, bottom) = (left + unpx(chip.size.width), top + unpx(chip.size.height));
    let (cx, cy) = ((left + right) / 2.0, (top + bottom) / 2.0);
    let away = NOTCH + SPACE;
    let clamp = |v: f32, lo: f32, hi: f32| v.min(hi).max(lo);
    let x_centered = clamp(cx - w / 2.0, MARGIN, vw - w - MARGIN);
    let y_centered = clamp(cy - h / 2.0, MARGIN, vh - h - MARGIN);
    let along_x = |x: f32| clamp(cx - x, radius + NOTCH, w - radius - NOTCH);
    let along_y = |y: f32| clamp(cy - y, radius + NOTCH, h - radius - NOTCH);
    let below = bottom + away;
    if below + h <= vh - MARGIN {
        return (x_centered, below, Side::Below, along_x(x_centered));
    }
    let above = top - away - h;
    if above >= MARGIN {
        return (x_centered, above, Side::Above, along_x(x_centered));
    }
    let beside = right + away;
    if beside + w <= vw - MARGIN {
        return (beside, y_centered, Side::Right, along_y(y_centered));
    }
    let before = left - away - w;
    if before >= MARGIN {
        return (before, y_centered, Side::Left, along_y(y_centered));
    }
    let y = clamp(below, MARGIN, vh - h - MARGIN);
    (x_centered, y, Side::Below, along_x(x_centered))
}

/// The surface of a popover: the menu colour (frosted when the frost is
/// on), corners of [`RADIUS`], the edge of a box and a popover's depth. A
/// click inside it stays inside. Call it before adding the popover's
/// children, which must draw over the glass, and add [`notch`] last.
pub(super) fn popover<E: Styled + ParentElement + InteractiveElement>(panel: E, th: &Theme) -> E {
    raised(
        panel.border_1().border_color(rgba(th.outline)),
        th,
        RADIUS,
        elevation::POPOVER,
    )
    .occlude()
    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

/// The notch on the popover's edge facing the chip, `along` that edge (from
/// its left for Below/Above, from its top otherwise): a triangle in the
/// popover's edge colour with a popover-coloured one just inside it,
/// covering the edge where they meet.
pub(super) fn notch(side: Side, along: f32, th: &Theme) -> [AnyElement; 2] {
    // The turn of an upward triangle to face away from the popover.
    let turn = match side {
        Side::Below => 0.0,
        Side::Above => PI,
        Side::Right => -FRAC_PI_2,
        Side::Left => FRAC_PI_2,
    };
    let triangle = |scale: f32, inset: f32, color: u32| {
        // Its box, unturned, centred half its length out from the edge.
        let (bw, bh) = (20.0 * scale, NOTCH * scale);
        let reach = bh / 2.0 - inset;
        let out = |half: f32| px(-reach - half);
        let el = svg().path("icons/notch.svg").absolute().w(px(bw)).h(px(bh));
        let el = match side {
            Side::Below => el.top(out(bh / 2.0)).left(px(along - bw / 2.0)),
            Side::Above => el.bottom(out(bh / 2.0)).left(px(along - bw / 2.0)),
            Side::Right => el.left(out(bw / 2.0)).top(px(along - bh / 2.0)),
            Side::Left => el.right(out(bw / 2.0)).top(px(along - bh / 2.0)),
        };
        el.text_color(rgba(color))
            .with_transformation(Transformation::rotate(radians(turn)))
            .into_any_element()
    };
    [triangle(1.0, 0.0, th.outline), triangle(0.9, 1.0, th.menu)]
}
