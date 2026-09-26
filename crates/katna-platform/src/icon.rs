// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Mail's icon drawn in code, with an optional unread-count badge, for
//! the tray (`docs/ARCHITECTURE.md` §15.2). Tray protocols take raw pixels,
//! and drawing the few shapes of `packaging/icons/in.invenia.katna.Mail.svg`
//! here keeps an SVG renderer out of the daemon.
//!
//! Shapes are drawn from their distance to each pixel, which antialiases
//! edges at any size. Coordinates are those of the 128×128 SVG.

/// An RGBA color with straight (not premultiplied) alpha, 0.0–1.0.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rgba([f32; 4]);

impl Rgba {
    fn hex(rgb: u32, alpha: f32) -> Self {
        let channel = |shift: u32| ((rgb >> shift) & 0xff) as f32 / 255.0;
        Self([channel(16), channel(8), channel(0), alpha])
    }

    fn mix(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let mut out = [0.0; 4];
        for (i, v) in out.iter_mut().enumerate() {
            *v = self.0[i] + (other.0[i] - self.0[i]) * t;
        }
        Self(out)
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

const TILE_TOP: u32 = 0x4f9cf9;
const TILE_BOTTOM: u32 = 0x1c5fd4;
const SHADOW: u32 = 0x0b2f6e;
const PAPER_TOP: u32 = 0xffffff;
const PAPER_BOTTOM: u32 = 0xe6eefb;
const FOLD: u32 = 0xc3d4f2;
const BADGE: u32 = 0xe5372d;

/// Draws the app icon: an envelope on a rounded blue square.
fn draw_icon(canvas: &mut Canvas) {
    let unit = canvas.size as f32 / 128.0;
    let solid = |rgb, alpha| Rgba::hex(rgb, alpha);
    canvas.paint(unit, |x, y, px| {
        let d = rounded_rect(x, y, [8.0, 10.0, 112.0, 112.0], 26.0);
        (coverage(d, px), solid(SHADOW, 0.25))
    });
    canvas.paint(unit, |x, y, px| {
        let d = rounded_rect(x, y, [8.0, 6.0, 112.0, 112.0], 26.0);
        let t = (y - 6.0) / 112.0;
        (
            coverage(d, px),
            solid(TILE_TOP, 1.0).mix(solid(TILE_BOTTOM, 1.0), t),
        )
    });
    canvas.paint(unit, |x, y, px| {
        let d = rounded_rect(x, y, [26.0, 39.0, 76.0, 54.0], 8.0);
        (coverage(d, px), solid(SHADOW, 0.2))
    });
    canvas.paint(unit, |x, y, px| {
        let d = rounded_rect(x, y, [26.0, 36.0, 76.0, 54.0], 8.0);
        let t = (y - 36.0) / 54.0;
        (
            coverage(d, px),
            solid(PAPER_TOP, 1.0).mix(solid(PAPER_BOTTOM, 1.0), t),
        )
    });
    canvas.paint(unit, |x, y, px| {
        let fold = [
            (28.0, 88.0),
            (57.0, 63.0),
            (64.0, 59.0),
            (71.0, 63.0),
            (100.0, 88.0),
        ];
        let d = polyline(x, y, &fold) - 1.5;
        (coverage(d, px), solid(FOLD, 1.0))
    });
    canvas.paint(unit, |x, y, px| {
        let flap = [
            (29.0, 40.0),
            (58.0, 64.0),
            (64.0, 67.0),
            (70.0, 64.0),
            (99.0, 40.0),
        ];
        let d = polyline(x, y, &flap) - 2.5;
        (coverage(d, px), solid(TILE_BOTTOM, 1.0))
    });
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
    fn icon_has_a_transparent_corner_a_blue_tile_and_white_paper() {
        let size = 64;
        let icon = app_icon_argb(size, None);
        assert_eq!(icon.len(), (size * size * 4) as usize);
        assert_eq!(pixel(&icon, size, 0, 0)[0], 0, "corner is transparent");
        let [a, r, g, b] = pixel(&icon, size, 8, 32);
        assert_eq!(a, 255);
        assert!(b > 200 && r < 100, "tile is blue: {r} {g} {b}");
        let [a, r, g, b] = pixel(&icon, size, 32, 25);
        assert_eq!(a, 255);
        assert!(r > 220 && g > 220 && b > 220, "paper is white: {r} {g} {b}");
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
