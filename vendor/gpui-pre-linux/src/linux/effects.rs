//! Background blur for translucent windows (added for Katna; not in
//! upstream GPUI).
//!
//! GPUI blurs a `WindowBackgroundAppearance::Blurred` window through
//! `org_kde_kwin_blur` only. Katna adds:
//!
//! - `ext_background_effect_v1`, which KWin 6.7 uses instead of
//!   `org_kde_kwin_blur`, preferred when the compositor offers it;
//! - `_KDE_NET_WM_BLUR_BEHIND_REGION` on X11 (KWin's blur effect);
//! - [`compositor_blur`], which says whether the compositor can blur at all;
//! - a blur region that leaves out the shadow margin of client-side
//!   decorations and follows their rounded corners
//!   ([`set_client_corner_radius`]).
//!
//! KWin reads the blur region relative to the window's frame (the window
//! geometry), not to the whole surface, and clips it to the frame. Under
//! server-side decorations the two are the same.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static KDE_BLUR: AtomicBool = AtomicBool::new(false);
static EXT_BLUR: AtomicBool = AtomicBool::new(false);
static X11_BLUR: AtomicBool = AtomicBool::new(false);
static CORNER_RADIUS: AtomicU32 = AtomicU32::new(0);

/// Whether the compositor blurs what is behind a window whose background
/// is `WindowBackgroundAppearance::Blurred`. Without it such a window is
/// only translucent.
pub fn compositor_blur() -> bool {
    EXT_BLUR.load(Ordering::Relaxed)
        || KDE_BLUR.load(Ordering::Relaxed)
        || X11_BLUR.load(Ordering::Relaxed)
}

/// The radius, in logical pixels, of the corners of client-side
/// decorations, which the blur region follows. Zero (the default) blurs
/// the whole frame rectangle.
pub fn set_client_corner_radius(radius: f32) {
    CORNER_RADIUS.store(radius.max(0.0).to_bits(), Ordering::Relaxed);
}

pub(crate) fn client_corner_radius() -> f32 {
    f32::from_bits(CORNER_RADIUS.load(Ordering::Relaxed))
}

#[allow(dead_code)]
pub(crate) fn set_kde_blur(available: bool) {
    KDE_BLUR.store(available, Ordering::Relaxed);
}

#[allow(dead_code)]
pub(crate) fn set_ext_blur(available: bool) {
    EXT_BLUR.store(available, Ordering::Relaxed);
}

#[allow(dead_code)]
pub(crate) fn ext_blur() -> bool {
    EXT_BLUR.load(Ordering::Relaxed)
}

#[allow(dead_code)]
pub(crate) fn set_x11_blur(available: bool) {
    X11_BLUR.store(available, Ordering::Relaxed);
}

/// Which corners of the frame are rounded: those whose two edges are not
/// tiled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RoundCorners {
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
}

/// The blur region of a `width` × `height` frame whose corners have
/// `radius`, as `(x, y, width, height)` rectangles relative to the frame.
/// The rounded corners are followed one pixel row at a time.
pub(crate) fn frame_region(
    width: i32,
    height: i32,
    radius: i32,
    corners: RoundCorners,
) -> Vec<(i32, i32, i32, i32)> {
    if width <= 0 || height <= 0 {
        return Vec::new();
    }
    let radius = radius.clamp(0, width.min(height) / 2);
    if radius == 0 {
        return vec![(0, 0, width, height)];
    }
    // How far row `i` of a corner (0 = the outer row) is cut in.
    let cut = |i: i32| {
        let r = radius as f32;
        let dy = r - (i as f32 + 0.5);
        (r - (r * r - dy * dy).max(0.0).sqrt()).round() as i32
    };
    let mut rects = Vec::with_capacity(2 * radius as usize + 1);
    for i in 0..radius {
        let c = cut(i);
        let left = |on: bool| if on { c } else { 0 };
        let top_l = left(corners.top_left);
        let top_r = left(corners.top_right);
        rects.push((top_l, i, width - top_l - top_r, 1));
        let bottom_l = left(corners.bottom_left);
        let bottom_r = left(corners.bottom_right);
        rects.push((bottom_l, height - 1 - i, width - bottom_l - bottom_r, 1));
    }
    rects.push((0, radius, width, height - 2 * radius));
    rects.retain(|r| r.2 > 0 && r.3 > 0);
    rects
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: RoundCorners = RoundCorners {
        top_left: true,
        top_right: true,
        bottom_left: true,
        bottom_right: true,
    };

    #[test]
    fn square_frame_is_one_rectangle() {
        assert_eq!(frame_region(100, 50, 0, ALL), vec![(0, 0, 100, 50)]);
        assert!(frame_region(0, 50, 4, ALL).is_empty());
    }

    #[test]
    fn rounded_corners_are_cut_row_by_row() {
        let rects = frame_region(100, 50, 5, ALL);
        // The outer rows are cut the most, the middle is whole.
        assert_eq!(rects[0].1, 0);
        assert!(rects[0].0 >= 2);
        assert_eq!(rects[0].0 + rects[0].2 + rects[0].0, 100);
        assert_eq!(*rects.last().unwrap(), (0, 5, 100, 40));
        // Every row of the frame is covered exactly once.
        let rows: i32 = rects.iter().map(|r| r.3).sum();
        assert_eq!(rows, 50);
    }

    #[test]
    fn tiled_corners_stay_square() {
        let corners = RoundCorners {
            top_left: false,
            top_right: true,
            bottom_left: false,
            bottom_right: false,
        };
        let rects = frame_region(100, 50, 5, corners);
        assert_eq!(rects[0].0, 0);
        assert!(rects[0].2 < 100);
        // Bottom rows reach both sides.
        assert_eq!(rects[1], (0, 49, 100, 1));
    }
}
