// SPDX-License-Identifier: GPL-3.0-or-later

//! Right-to-left layout (Arabic, Hebrew, Persian, Urdu): GPUI mirrors
//! flex rows, padding, margins, insets, borders and text alignment by
//! itself (`vendor/gpui-pre/KATNA.md`). These help code that places or
//! reads positions by hand: painted lines, drags, sliders, swipes,
//! arrow keys; and laying out mail in its own direction.

use gpui::{Bounds, LayoutDirection, Pixels, Styled, Window};
pub use katna_core::bidi::Direction;

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

/// An arrow key read along the line, so lists, menus, tabs and sliders
/// move the way their items are drawn: in a right-to-left layout Left
/// goes toward the end, the way Right does in a left-to-right one, and
/// reads as `"right"`. Other keys come back as they are.
pub fn arrow(key: &str, rtl: bool) -> &str {
    match key {
        "left" if rtl => "right",
        "right" if rtl => "left",
        key => key,
    }
}

/// `element` laid out in `dir` (its text aligned to that direction's
/// start, its rows from that side); `None` leaves it as what holds it.
/// Mail and its paragraphs read their own way whatever the window's
/// (`katna_core::bidi`).
pub fn directed<E: Styled>(element: E, dir: Option<Direction>) -> E {
    match dir {
        Some(Direction::Ltr) => element.layout_ltr(),
        Some(Direction::Rtl) => element.layout_rtl(),
        None => element,
    }
}

/// Whether Left moves the caret forward in `text`: it reads right to
/// left by its first strong letter (`katna_core::bidi::first_strong`),
/// or, with none (digits, nothing), the window does.
pub fn left_goes_on(text: &str, window: &Window) -> bool {
    katna_core::bidi::first_strong(text).map_or_else(|| is_rtl(window), Direction::is_rtl)
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
        assert_eq!(arrow("left", true), "right");
        assert_eq!(arrow("right", true), "left");
        assert_eq!(arrow("left", false), "left");
        assert_eq!(arrow("up", true), "up");
    }
}
