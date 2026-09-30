// SPDX-License-Identifier: GPL-3.0-or-later

//! Popovers with a notch pointing at what they belong to (the search's
//! custom dates, a Year view day): where the popover goes beside it, kept
//! inside the window, and the notch on the side facing it.

use std::f32::consts::{FRAC_PI_2, PI};

use gpui::{AnyElement, Bounds, Pixels, Transformation, prelude::*, radians, rgba, svg};
use katna_ui::{px, unpx};

use crate::theme::Theme;

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

/// The notch on the popover's edge facing the chip: a border-colored
/// triangle with a popover-colored one just inside it, covering the
/// border where they meet.
pub(super) fn notch(side: Side, along: f32, (w, h): (f32, f32), th: &Theme) -> [AnyElement; 2] {
    // The middle of the notch's base on the popover's edge, the way out
    // and the turn of an upward triangle to face that way.
    let (base, out, turn) = match side {
        Side::Below => ((along, 0.0), (0.0, -1.0), 0.0),
        Side::Above => ((along, h), (0.0, 1.0), PI),
        Side::Right => ((0.0, along), (-1.0, 0.0), -FRAC_PI_2),
        Side::Left => ((w, along), (1.0, 0.0), FRAC_PI_2),
    };
    let triangle = |scale: f32, inset: f32, color: u32| {
        // Its middle, half its length out from where its base sits.
        let (bw, bh) = (20.0 * scale, NOTCH * scale);
        let reach = bh / 2.0 - inset;
        let (mx, my) = (base.0 + out.0 * reach, base.1 + out.1 * reach);
        svg()
            .path("icons/notch.svg")
            .absolute()
            .left(px(mx - bw / 2.0))
            .top(px(my - bh / 2.0))
            .w(px(bw))
            .h(px(bh))
            .text_color(rgba(color))
            .with_transformation(Transformation::rotate(radians(turn)))
            .into_any_element()
    };
    [triangle(1.0, 0.0, th.divider), triangle(0.9, 1.0, th.menu)]
}
