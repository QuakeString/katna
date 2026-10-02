// SPDX-License-Identifier: GPL-3.0-or-later

//! Frosted glass for floating panels: a translucent fill over a blur of
//! what is behind the panel. The blur is drawn by Katna's copy of GPUI's
//! renderer (`vendor/gpui-pre-wgpu/KATNA.md`) for a quad whose border
//! colour is its marker. That renderer also draws drop shadows only
//! outside their element, so a frosted panel keeps its usual box shadow.

use crate::scale::px;
use gpui::{BorderStyle, Bounds, Corners, Hsla, IntoElement, Pixels, Styled, canvas, quad};

/// Whether the renderer can blur behind a panel. False before the first
/// frame is drawn and where the window's surface cannot be copied from.
pub fn supported() -> bool {
    #[cfg(not(windows))]
    return gpui_wgpu::backdrop_blur_supported();
    // GPUI draws with Direct3D on Windows, without Katna's blur.
    #[cfg(windows)]
    return false;
}

/// The glass of a frosted panel, as the panel's first child: `fill` with
/// corners of `radius` (one for all four, or each its own) over a blur of
/// `blur` device pixels of what is behind. The panel itself paints no
/// background. In a panel that scrolls its own content the glass stays put
/// while the content scrolls.
pub fn glass(fill: Hsla, radius: impl Into<Corners<Pixels>>, blur: f32) -> impl IntoElement {
    let radius = radius.into();
    canvas(
        |_, _, _| (),
        move |bounds, (), window, _| {
            let bounds = unscrolled(bounds, window.content_mask().bounds);
            // A quad whose border colour is the marker: the renderer blurs
            // what is under it, then fills it.
            window.paint_quad(quad(
                bounds,
                radius,
                fill,
                px(0.0),
                marker(blur),
                BorderStyle::Solid,
            ));
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// A see-through card's fill, as the card's first child: what is drawn
/// under the card (a blurred window's tint) is cleared first, so the card
/// shows the desktop through `fill` alone rather than through the tint as
/// well. `solid` is what a renderer that cannot clear draws instead: the
/// card's colour, opaque. The card itself paints no background.
pub fn clear_fill(fill: Hsla, solid: Hsla, radius: impl Into<Corners<Pixels>>) -> impl IntoElement {
    let radius = radius.into();
    canvas(
        |_, _, _| (),
        move |bounds, (), window, _| {
            if let Some(eraser) = eraser() {
                window.paint_quad(quad(
                    bounds,
                    radius,
                    solid,
                    px(0.0),
                    eraser,
                    BorderStyle::Solid,
                ));
            }
            window.paint_quad(quad(
                bounds,
                radius,
                fill,
                px(0.0),
                gpui::transparent_black(),
                BorderStyle::Solid,
            ));
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

#[cfg(not(windows))]
fn eraser() -> Option<Hsla> {
    Some(gpui_wgpu::erase_marker())
}

/// Windows has no blurred window to clear.
#[cfg(windows)]
fn eraser() -> Option<Hsla> {
    None
}

/// Where the glass goes for `bounds` inside a panel clipped to `clip`. A
/// panel that scrolls its content moves its children, the glass too, but
/// clips them to itself: a clip the glass's size at another place is the
/// panel, where the glass belongs.
fn unscrolled(bounds: Bounds<Pixels>, clip: Bounds<Pixels>) -> Bounds<Pixels> {
    let close = |a: Pixels, b: Pixels| (a - b).abs() < px(1.0);
    if close(bounds.size.width, clip.size.width) && close(bounds.size.height, clip.size.height) {
        clip
    } else {
        bounds
    }
}

#[cfg(not(windows))]
fn marker(blur: f32) -> Hsla {
    gpui_wgpu::backdrop_blur_marker(blur)
}

/// Without the blur, the panel is just its fill.
#[cfg(windows)]
fn marker(_blur: f32) -> Hsla {
    gpui::transparent_black()
}

#[cfg(test)]
mod tests {
    use gpui::{point, size};

    use super::*;

    fn rect(y: f32, h: f32) -> Bounds<Pixels> {
        Bounds::new(point(px(10.0), px(y)), size(px(300.0), px(h)))
    }

    #[test]
    fn glass_stays_on_a_scrolled_panel() {
        // Scrolled 40 px: the glass moved up, the panel's clip did not.
        assert_eq!(
            unscrolled(rect(60.0, 200.0), rect(100.0, 200.0)),
            rect(100.0, 200.0)
        );
        // Not scrolled, and a clip that is not the panel's: as laid out.
        assert_eq!(
            unscrolled(rect(100.0, 200.0), rect(100.0, 200.0)),
            rect(100.0, 200.0)
        );
        assert_eq!(
            unscrolled(rect(100.0, 200.0), rect(0.0, 900.0)),
            rect(100.0, 200.0)
        );
    }
}
