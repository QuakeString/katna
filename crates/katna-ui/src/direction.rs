// SPDX-License-Identifier: GPL-3.0-or-later

//! Right-to-left layout (Arabic, Hebrew, Persian, Urdu): GPUI mirrors
//! flex rows, padding, margins, insets, borders and text alignment by
//! itself (`vendor/gpui-pre/KATNA.md`). These help code that places or
//! reads positions by hand: painted lines, drags, sliders, swipes,
//! arrow keys.

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

/// Whether `text` reads right to left, by its first letter of a strong
/// direction (Unicode's rule for a paragraph); `None` if it has none, as
/// with digits, spaces or nothing. Arrow keys move the caret the way the
/// text is drawn: in Arabic text Left goes forward.
pub fn text_rtl(text: &str) -> Option<bool> {
    text.chars().find_map(|c| {
        let rtl = matches!(
            c as u32,
            0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF | 0x10800..=0x10FFF | 0x1E800..=0x1EFFF
        ) && !c.is_numeric();
        if rtl {
            Some(true)
        } else if c.is_alphabetic() {
            Some(false)
        } else {
            None
        }
    })
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
        assert_eq!(text_rtl("١٢ مرحبا"), Some(true));
        assert_eq!(text_rtl("12 hello مرحبا"), Some(false));
        assert_eq!(text_rtl("12 - "), None);
    }
}
