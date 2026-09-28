// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Mail's icon with an optional unread-count badge, for the tray
//! (`docs/ARCHITECTURE.md` §15.2). Tray protocols take raw pixels. The icon
//! comes pre-rendered at each tray size (`icons/*.rgba`, made from the logo
//! by `packaging/icons/render.py`), which keeps an SVG renderer out of the
//! daemon; the badge is drawn in code.
//!
//! The badge's shapes are drawn from their distance to each pixel, which
//! antialiases edges at any size.

/// An RGBA color with straight (not premultiplied) alpha, 0.0–1.0.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rgba([f32; 4]);

impl Rgba {
    fn hex(rgb: u32, alpha: f32) -> Self {
        let channel = |shift: u32| ((rgb >> shift) & 0xff) as f32 / 255.0;
        Self([channel(16), channel(8), channel(0), alpha])
    }
}

/// A square image, premultiplied RGBA.
struct Canvas {
    size: u32,
    pixels: Vec<[f32; 4]>,
}

impl Canvas {
    fn new(size: u32) -> Self {
        Self {
            size,
            pixels: vec![[0.0; 4]; (size * size) as usize],
        }
    }

    /// Paints over every pixel with the coverage and color that `shade`
    /// gives for the pixel's center, in `unit`-sized coordinates.
    fn paint(&mut self, unit: f32, shade: impl Fn(f32, f32, f32) -> (f32, Rgba)) {
        // One pixel, in the drawing's coordinates, for antialiasing.
        let pixel = 1.0 / unit;
        for y in 0..self.size {
            for x in 0..self.size {
                let (px, py) = ((x as f32 + 0.5) * pixel, (y as f32 + 0.5) * pixel);
                let (coverage, color) = shade(px, py, pixel);
                let alpha = coverage.clamp(0.0, 1.0) * color.0[3];
                if alpha <= 0.0 {
                    continue;
                }
                let dst = &mut self.pixels[(y * self.size + x) as usize];
                for (channel, source) in dst.iter_mut().zip(color.0).take(3) {
                    *channel = source * alpha + *channel * (1.0 - alpha);
                }
                dst[3] = alpha + dst[3] * (1.0 - alpha);
            }
        }
    }

    /// Pixels as ARGB32 in network byte order, as StatusNotifierItem wants.
    fn argb(&self) -> Vec<u8> {
        let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        let mut out = Vec::with_capacity(self.pixels.len() * 4);
        for [r, g, b, a] in &self.pixels {
            // Back to straight alpha.
            let un = |c: f32| if *a > 0.0 { c / a } else { 0.0 };
            out.extend([byte(*a), byte(un(*r)), byte(un(*g)), byte(un(*b))]);
        }
        out
    }
}

/// Coverage of a shape whose signed distance (negative inside) is `d`.
fn coverage(d: f32, pixel: f32) -> f32 {
    (0.5 - d / pixel).clamp(0.0, 1.0)
}

/// Signed distance from `(x, y)` to a rounded rectangle.
fn rounded_rect(x: f32, y: f32, rect: [f32; 4], radius: f32) -> f32 {
    let [left, top, width, height] = rect;
    let (cx, cy) = (left + width / 2.0, top + height / 2.0);
    let qx = (x - cx).abs() - width / 2.0 + radius;
    let qy = (y - cy).abs() - height / 2.0 + radius;
    let outside = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    outside + qx.max(qy).min(0.0) - radius
}

/// Distance from `(x, y)` to the segment `a`–`b`.
fn segment(x: f32, y: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = dx * dx + dy * dy;
    let t = if len > 0.0 {
        (((x - a.0) * dx + (y - a.1) * dy) / len).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let (ex, ey) = (x - a.0 - t * dx, y - a.1 - t * dy);
    (ex * ex + ey * ey).sqrt()
}

/// Distance from `(x, y)` to a polyline.
fn polyline(x: f32, y: f32, points: &[(f32, f32)]) -> f32 {
    points
        .windows(2)
        .map(|pair| segment(x, y, pair[0], pair[1]))
        .fold(f32::INFINITY, f32::min)
}

const BADGE: u32 = 0xe5372d;

/// The icon at the tray's sizes: straight-alpha RGBA, row by row.
const PIXELS: &[(u32, &[u8])] = &[
    (16, include_bytes!("../icons/16.rgba")),
    (22, include_bytes!("../icons/22.rgba")),
    (24, include_bytes!("../icons/24.rgba")),
    (32, include_bytes!("../icons/32.rgba")),
    (48, include_bytes!("../icons/48.rgba")),
    (64, include_bytes!("../icons/64.rgba")),
];

/// Paints the icon, scaled from the nearest rendered size at or above `size`
/// (or the largest) when `size` isn't one of them.
fn draw_icon(canvas: &mut Canvas) {
    let size = canvas.size;
    let &(from, rgba) = PIXELS
        .iter()
        .find(|(s, _)| *s >= size)
        .unwrap_or(&PIXELS[PIXELS.len() - 1]);
    let step = from as f32 / size as f32;
    for y in 0..size {
        for x in 0..size {
            // Average the source pixels this pixel covers (box filter).
            let (x0, x1) = (x as f32 * step, (x + 1) as f32 * step);
            let (y0, y1) = (y as f32 * step, (y + 1) as f32 * step);
            let mut sum = [0.0f32; 4];
            let mut weight = 0.0;
            let (sx0, sy0) = (x0.floor() as u32, y0.floor() as u32);
            let (sx1, sy1) = ((x1.ceil() as u32).min(from), (y1.ceil() as u32).min(from));
            for sy in sy0..sy1 {
                let wy = (y1.min((sy + 1) as f32) - y0.max(sy as f32)).max(0.0);
                for sx in sx0..sx1 {
                    let wx = (x1.min((sx + 1) as f32) - x0.max(sx as f32)).max(0.0);
                    let i = ((sy * from + sx) * 4) as usize;
                    let a = rgba[i + 3] as f32 / 255.0;
                    let w = wx * wy;
                    for c in 0..3 {
                        sum[c] += rgba[i + c] as f32 / 255.0 * a * w;
                    }
                    sum[3] += a * w;
                    weight += w;
                }
            }
            if weight > 0.0 {
                canvas.pixels[(y * size + x) as usize] = sum.map(|v| v / weight);
            }
        }
    }
}

/// Strokes of each badge character in a box 0.6 wide and 1.0 tall, y down.
fn glyph(c: char) -> &'static [&'static [(f32, f32)]] {
    match c {
        '0' => &[&[
            (0.05, 0.18),
            (0.16, 0.0),
            (0.44, 0.0),
            (0.55, 0.18),
            (0.55, 0.82),
            (0.44, 1.0),
            (0.16, 1.0),
            (0.05, 0.82),
            (0.05, 0.18),
        ]],
        '1' => &[&[(0.12, 0.22), (0.36, 0.0), (0.36, 1.0)]],
        '2' => &[&[
            (0.05, 0.2),
            (0.16, 0.02),
            (0.44, 0.02),
            (0.55, 0.2),
            (0.55, 0.36),
            (0.05, 1.0),
            (0.58, 1.0),
        ]],
        '3' => &[
            &[
                (0.05, 0.1),
                (0.2, 0.0),
                (0.45, 0.0),
                (0.55, 0.13),
                (0.55, 0.35),
                (0.42, 0.48),
                (0.24, 0.48),
            ],
            &[
                (0.42, 0.48),
                (0.55, 0.61),
                (0.55, 0.87),
                (0.44, 1.0),
                (0.18, 1.0),
                (0.05, 0.9),
            ],
        ],
        '4' => &[&[(0.45, 1.0), (0.45, 0.0), (0.03, 0.68), (0.6, 0.68)]],
        '5' => &[&[
            (0.55, 0.0),
            (0.1, 0.0),
            (0.07, 0.45),
            (0.4, 0.42),
            (0.55, 0.55),
            (0.55, 0.87),
            (0.43, 1.0),
            (0.15, 1.0),
            (0.03, 0.9),
        ]],
        '6' => &[&[
            (0.5, 0.04),
            (0.36, 0.0),
            (0.18, 0.02),
            (0.05, 0.25),
            (0.05, 0.85),
            (0.17, 1.0),
            (0.43, 1.0),
            (0.55, 0.87),
            (0.55, 0.6),
            (0.43, 0.47),
            (0.17, 0.47),
            (0.05, 0.6),
        ]],
        '7' => &[&[(0.03, 0.0), (0.57, 0.0), (0.22, 1.0)]],
        '8' => &[
            &[
                (0.3, 0.0),
                (0.47, 0.04),
                (0.52, 0.22),
                (0.3, 0.46),
                (0.08, 0.22),
                (0.13, 0.04),
                (0.3, 0.0),
            ],
            &[
                (0.3, 0.46),
                (0.55, 0.62),
                (0.55, 0.87),
                (0.42, 1.0),
                (0.18, 1.0),
                (0.05, 0.87),
                (0.05, 0.62),
                (0.3, 0.46),
            ],
        ],
        '9' => &[&[
            (0.55, 0.4),
            (0.43, 0.53),
            (0.17, 0.53),
            (0.05, 0.4),
            (0.05, 0.13),
            (0.17, 0.0),
            (0.43, 0.0),
            (0.55, 0.15),
            (0.55, 0.75),
            (0.42, 0.97),
            (0.25, 1.0),
            (0.1, 0.96),
        ]],
        '+' => &[&[(0.05, 0.5), (0.55, 0.5)], &[(0.3, 0.22), (0.3, 0.78)]],
        _ => &[],
    }
}

/// Width of one character and the gap after it, in text heights.
const ADVANCE: f32 = 0.78;

/// What the badge says: the count, or `99+`.
pub fn badge_text(count: u64) -> String {
    if count > 99 {
        "99+".to_owned()
    } else {
        count.to_string()
    }
}

/// Draws a red pill in the top-right corner with `text` in white.
fn draw_badge(canvas: &mut Canvas, text: &str) {
    let size = canvas.size as f32;
    // In pixels: the pill's height, the text's height and stroke width.
    // Panel-sized icons need a big badge to be legible.
    let height = if size <= 32.0 {
        size * 0.58
    } else {
        size * 0.46
    }
    .max(8.0);
    let text_height = height * 0.56;
    let stroke = (height * 0.13).max(1.0);
    let chars: Vec<char> = text.chars().collect();
    let text_width = (chars.len() as f32 * ADVANCE - (ADVANCE - 0.6)) * text_height;
    let width = (text_width + height * 0.55).max(height).min(size);
    let rect = [size - width, 0.0, width, height];
    canvas.paint(1.0, |x, y, px| {
        let d = rounded_rect(x, y, rect, height / 2.0);
        (coverage(d, px), Rgba::hex(BADGE, 1.0))
    });
    let left = rect[0] + (width - text_width) / 2.0;
    let top = (height - text_height) / 2.0;
    canvas.paint(1.0, |x, y, px| {
        let mut d = f32::INFINITY;
        for (i, c) in chars.iter().enumerate() {
            let origin = left + i as f32 * ADVANCE * text_height;
            let (u, v) = ((x - origin) / text_height, (y - top) / text_height);
            for stroke_points in glyph(*c) {
                d = d.min(polyline(u, v, stroke_points) * text_height);
            }
        }
        (coverage(d - stroke / 2.0, px), Rgba::hex(0xffffff, 1.0))
    });
}

/// The icon at `size`×`size` pixels, with a badge saying `badge` if given,
/// as ARGB32 in network byte order.
pub fn app_icon_argb(size: u32, badge: Option<&str>) -> Vec<u8> {
    let mut canvas = Canvas::new(size);
    draw_icon(&mut canvas);
    if let Some(text) = badge.filter(|t| !t.is_empty()) {
        draw_badge(&mut canvas, text);
    }
    canvas.argb()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixel(argb: &[u8], size: u32, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * size + x) * 4) as usize;
        [argb[i], argb[i + 1], argb[i + 2], argb[i + 3]]
    }

    #[test]
    fn icon_has_a_transparent_corner_a_teal_disc_and_a_white_k() {
        for size in [16, 22, 24, 32, 48, 64] {
            let icon = app_icon_argb(size, None);
            assert_eq!(icon.len(), (size * size * 4) as usize);
            assert_eq!(pixel(&icon, size, 0, 0)[0], 0, "corner is transparent");
            // The disc's top, above the k.
            let [a, r, g, b] = pixel(&icon, size, size / 2, size / 10);
            assert_eq!(a, 255, "{size}");
            assert!(r < 30 && g > 110 && b > 110, "disc is teal: {r} {g} {b}");
            // The k's first stroke.
            let [a, r, g, b] = pixel(&icon, size, size / 10, size / 2);
            assert_eq!(a, 255, "{size}");
            assert!(r > 200 && g > 200 && b > 200, "k is white: {r} {g} {b}");
        }
    }

    #[test]
    fn every_rendered_size_is_whole() {
        for (size, rgba) in PIXELS {
            assert_eq!(rgba.len(), (size * size * 4) as usize, "{size}");
        }
    }

    #[test]
    fn badge_is_red_with_white_text_in_the_top_right_corner() {
        let size = 64;
        let plain = app_icon_argb(size, None);
        let badged = app_icon_argb(size, Some("7"));
        assert_ne!(plain, badged);
        // The pill's right end, away from the digit.
        let [a, r, g, b] = pixel(&badged, size, 60, 18);
        assert_eq!(a, 255);
        assert!(r > 200 && g < 90 && b < 90, "badge is red: {r} {g} {b}");
        // Some white pixels from the digit inside the pill.
        let white = (0..37)
            .flat_map(|y| (27..64).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                let [_, r, g, b] = pixel(&badged, size, x, y);
                r > 240 && g > 240 && b > 240
            })
            .count();
        assert!(white > 10, "{white} white pixels");
        // The bottom-left of the icon is untouched.
        assert_eq!(pixel(&plain, size, 10, 60), pixel(&badged, size, 10, 60));
    }

    #[test]
    fn badge_text_caps_at_99() {
        assert_eq!(badge_text(3), "3");
        assert_eq!(badge_text(99), "99");
        assert_eq!(badge_text(100), "99+");
    }

    #[test]
    fn every_badge_character_has_strokes() {
        for c in "0123456789+".chars() {
            assert!(!glyph(c).is_empty(), "{c}");
        }
    }

    #[test]
    fn small_sizes_and_wide_badges_stay_in_bounds() {
        for size in [16, 22, 24, 32] {
            let icon = app_icon_argb(size, Some("99+"));
            assert_eq!(icon.len(), (size * size * 4) as usize);
        }
    }
}
