// SPDX-License-Identifier: GPL-3.0-or-later

//! Frosted glass for floating panels: a translucent fill over a blur of
//! what is behind the panel. The blur is drawn by Katna's copy of GPUI's
//! renderer (`vendor/gpui-pre-wgpu/KATNA.md`) for a quad whose border
//! colour is its marker. That renderer also draws drop shadows only
//! outside their element, so a frosted panel keeps its usual box shadow.

use gpui::{BorderStyle, Hsla, IntoElement, Pixels, Styled, canvas, px, quad};

/// Whether the renderer can blur behind a panel. False before the first
/// frame is drawn and where the window's surface cannot be copied from.
pub fn supported() -> bool {
    gpui_wgpu::backdrop_blur_supported()
}

/// The glass of a frosted panel, as the panel's first child: `fill` with
/// corners of `radius` over a blur of `blur` device pixels of what is
/// behind. The panel itself paints no background.
pub fn glass(fill: Hsla, radius: Pixels, blur: f32) -> impl IntoElement {
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
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}
