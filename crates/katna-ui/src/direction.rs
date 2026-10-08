// SPDX-License-Identifier: GPL-3.0-or-later

//! Right-to-left layout (Arabic, Hebrew, Persian, Urdu): GPUI mirrors
//! flex rows, padding, margins, insets, borders and text alignment by
//! itself (`vendor/gpui-pre/KATNA.md`). These help code that places or
//! reads positions by hand: painted lines, drags, sliders, swipes.

use gpui::{Bounds, LayoutDirection, Pixels, Window};

/// Sets `window`'s layout direction: right to left when `rtl`. Call it
/// every frame (it costs nothing when nothing changes), so a change of
/// language turns every window around at once.
pub fn follow(window: &mut Window, rtl: bool) {
    window.set_layout_direction(if rtl {
        LayoutDirection::Rtl
    } else {
        LayoutDirection::Ltr
    });
}

/// Whether what is being drawn now lays out right to left.
pub fn is_rtl(window: &Window) -> bool {
    window.layout_direction().is_rtl()
}

/// How far `x` is from the start (left, or right in a right-to-left
/// layout) of `bounds`.
pub fn from_start(x: Pixels, bounds: Bounds<Pixels>, rtl: bool) -> Pixels {
    if rtl {
        bounds.right() - x
    } else {
        x - bounds.left()
    }
}

/// The x that is `distance` in from the start of `bounds`.
pub fn at_start(distance: Pixels, bounds: Bounds<Pixels>, rtl: bool) -> Pixels {
    if rtl {
        bounds.right() - distance
    } else {
        bounds.left() + distance
    }
}

/// A horizontal movement read along the line: toward the end positive.
pub fn along(dx: Pixels, rtl: bool) -> Pixels {
    if rtl { -dx } else { dx }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{point, size};

    #[test]
    fn distances_count_from_the_start() {
        let bounds = Bounds::new(
            point(Pixels::from(10.0), Pixels::ZERO),
            size(Pixels::from(100.0), Pixels::from(20.0)),
        );
        let x = Pixels::from(30.0);
        assert_eq!(from_start(x, bounds, false), Pixels::from(20.0));
        assert_eq!(from_start(x, bounds, true), Pixels::from(80.0));
        assert_eq!(at_start(Pixels::from(20.0), bounds, false), x);
        assert_eq!(at_start(Pixels::from(80.0), bounds, true), x);
        assert_eq!(along(Pixels::from(5.0), true), Pixels::from(-5.0));
    }
}
