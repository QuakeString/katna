// SPDX-License-Identifier: GPL-3.0-or-later

//! The interface scale: Settings > Appearance > Scaling makes everything
//! bigger or smaller on top of the desktop's own scale.
//!
//! GPUI takes the display's scale from the desktop and has no way to add
//! to it, so Katna scales its own lengths instead: every length in the
//! GPUI crates goes through [`px`], which multiplies by the scale, and
//! every length read back from GPUI (a layout's bounds, the window's size,
//! the pointer) goes through [`unpx`], which divides by it. Code between
//! the two works in unscaled design units, so a change applies at the next
//! frame. Never use `gpui::px` in the app (clippy's `disallowed-methods`
//! stops it).

use std::cell::Cell;

use gpui::Pixels;

/// The smallest and the largest scale.
pub const MIN: f32 = 0.75;
pub const MAX: f32 = 2.0;

thread_local! {
    static SCALE: Cell<f32> = const { Cell::new(1.0) };
}

/// The interface scale, 1.0 by default.
pub fn scale() -> f32 {
    SCALE.with(Cell::get)
}

/// Sets the interface scale, kept between [`MIN`] and [`MAX`]. The windows
/// need a redraw to show it.
pub fn set_scale(scale: f32) {
    let scale = if scale.is_finite() { scale } else { 1.0 };
    SCALE.with(|s| s.set(scale.clamp(MIN, MAX)));
}

/// A length of `value` design pixels, scaled.
#[allow(clippy::disallowed_methods)]
pub fn px(value: f32) -> Pixels {
    gpui::px(value * scale())
}

/// A length in the desktop's pixels, whatever the scale: for a new
/// window's size, which the desktop fits to the screen.
#[allow(clippy::disallowed_methods)]
pub fn desktop_px(value: f32) -> Pixels {
    gpui::px(value)
}

/// A length GPUI measured, in design pixels.
pub fn unpx(value: Pixels) -> f32 {
    f32::from(value) / scale()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_and_clamps() {
        set_scale(1.5);
        assert_eq!(f32::from(px(10.0)), 15.0);
        assert_eq!(unpx(px(10.0)), 10.0);
        set_scale(9.0);
        assert_eq!(scale(), MAX);
        set_scale(f32::NAN);
        assert_eq!(scale(), 1.0);
    }
}
