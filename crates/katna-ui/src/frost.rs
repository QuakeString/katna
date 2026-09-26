// SPDX-License-Identifier: GPL-3.0-or-later

//! Frosted glass for floating panels: a translucent fill over a blur of
//! what is behind the panel. The blur is drawn by Katna's copy of GPUI's
//! renderer (`vendor/gpui-pre-wgpu/KATNA.md`) for a quad whose border
//! colour is its marker.
//!
//! GPUI paints a box shadow under the whole element, which would darken
//! the glass, so [`glass`] paints the shadow only outside the panel.

use gpui::{
    BorderStyle, Bounds, BoxShadow, ContentMask, Hsla, IntoElement, Pixels, Styled, Window, canvas,
    point, px, quad, size,
};

/// Whether the renderer can blur behind a panel. False before the first
/// frame is drawn and where the window's surface cannot be copied from.
pub fn supported() -> bool {
    gpui_wgpu::backdrop_blur_supported()
}

/// The glass of a frosted panel, as the panel's first child: `fill` with
/// corners of `radius` over a blur of `blur` device pixels of what is
/// behind, and `shadows` only outside the panel. The panel itself paints
/// no background or shadow, and does not clip its children.
pub fn glass(fill: Hsla, radius: Pixels, blur: f32, shadows: Vec<BoxShadow>) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, (), window, _| {
            // A quad whose border colour is the marker: the renderer blurs
            // what is under it, then fills it.
            window.paint_quad(quad(
                bounds,
                radius,
                fill,
                px(0.0),
                gpui_wgpu::backdrop_blur_marker(blur),
                BorderStyle::Solid,
            ));
            paint_outside(bounds, radius, &shadows, window);
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

fn paint_outside(
    bounds: Bounds<Pixels>,
    radius: Pixels,
    shadows: &[BoxShadow],
    window: &mut Window,
) {
    let reach = shadows
        .iter()
        .map(|s| s.offset.x.abs().max(s.offset.y.abs()) + s.spread_radius + s.blur_radius * 3.0)
        .fold(px(0.0), |a, b| a.max(b));
    if reach <= px(0.0) {
        return;
    }
    for piece in outside(bounds, radius, reach, window.scale_factor()) {
        window.with_content_mask(Some(ContentMask { bounds: piece }), |window| {
            window.paint_drop_shadows(bounds, radius.into(), shadows);
        });
    }
}

/// The area within `reach` of `bounds` but outside its rounded shape: four
/// bands, and the rounded corners one device pixel row at a time.
fn outside(
    bounds: Bounds<Pixels>,
    radius: Pixels,
    reach: Pixels,
    scale: f32,
) -> Vec<Bounds<Pixels>> {
    let (left, top) = (bounds.origin.x, bounds.origin.y);
    let (right, bottom) = (left + bounds.size.width, top + bounds.size.height);
    let rect = |x: Pixels, y: Pixels, w: Pixels, h: Pixels| Bounds {
        origin: point(x, y),
        size: size(w, h),
    };
    let mut pieces = vec![
        rect(
            left - reach,
            top - reach,
            bounds.size.width + reach * 2.0,
            reach,
        ),
        rect(left - reach, bottom, bounds.size.width + reach * 2.0, reach),
        rect(left - reach, top, reach, bounds.size.height),
        rect(right, top, reach, bounds.size.height),
    ];
    let radius = f32::from(radius)
        .min(f32::from(bounds.size.width) / 2.0)
        .min(f32::from(bounds.size.height) / 2.0)
        .max(0.0)
        * scale;
    let row = px(1.0 / scale);
    for i in 0..radius.ceil() as u32 {
        let cut = px(corner_cut(radius, i) / scale);
        if cut <= px(0.0) {
            continue;
        }
        let (upper, lower) = (top + row * i as f32, bottom - row * (i + 1) as f32);
        for (x, y) in [
            (left, upper),
            (right - cut, upper),
            (left, lower),
            (right - cut, lower),
        ] {
            pieces.push(rect(x, y, cut, row));
        }
    }
    pieces
}

/// How far row `row` (0 = the outer row) of a corner of `radius` is cut
/// in, in the same units.
fn corner_cut(radius: f32, row: u32) -> f32 {
    let dy = radius - (row as f32 + 0.5);
    radius - (radius * radius - dy * dy).max(0.0).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_cut_most_at_the_edge() {
        assert!(corner_cut(8.0, 0) > corner_cut(8.0, 4));
        assert!(corner_cut(8.0, 7) < 0.1);
    }

    #[test]
    fn outside_stays_outside() {
        let b = Bounds {
            origin: point(px(10.0), px(20.0)),
            size: size(px(100.0), px(50.0)),
        };
        let pieces = outside(b, px(8.0), px(12.0), 1.0);
        // Four bands and up to eight rows per corner.
        assert!(pieces.len() > 4 && pieces.len() <= 4 + 32);
        let centre = point(px(60.0), px(45.0));
        assert!(pieces.iter().all(|p| !p.contains(&centre)));
        // Corner rows sit inside the bounding box, at its corners.
        let row = pieces[4];
        assert_eq!(row.origin, point(px(10.0), px(20.0)));
    }
}
