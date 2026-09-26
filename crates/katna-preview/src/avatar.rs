// SPDX-License-Identifier: GPL-3.0-or-later

//! Sender logos made to fill the round avatar they are shown in.
//!
//! Website icons rarely fit a circle as they are: many carry a margin of
//! transparency or of their own background color, and some are a glyph on
//! nothing at all. [`fill_circle`] trims that margin, then either crops
//! the picture to the circle (an icon that is a picture edge to edge) or
//! centers the logo on a disc of its own background color, so no white
//! ring shows around it.

use std::io::Cursor;

use image::imageops::{self, FilterType};
use image::{DynamicImage, ImageReader, Limits, Rgba, RgbaImage};

use crate::picture::Error;

/// Pixels on a side of a finished avatar: sharp at 40 px on a 3× screen.
pub const SIDE: u32 = 128;
/// Larger pictures are scaled down to this before they are looked at.
const WORK_SIDE: u32 = 1024;
/// Alpha below this counts as see-through.
const SEEN: u8 = 24;
/// How far (per channel) a pixel may be from a background color and
/// still count as that background.
const NEAR: u8 = 28;
/// A logo on a disc spans this much of the disc's diameter corner to
/// corner, so every part of it stays inside the circle.
const LOGO_DIAGONAL: f32 = 0.84;
/// The disc behind a see-through logo that is too light for white.
const DARK: Rgba<u8> = Rgba([0x3c, 0x40, 0x43, 0xff]);
const WHITE: Rgba<u8> = Rgba([0xff, 0xff, 0xff, 0xff]);

/// Decodes a raster logo (PNG, JPEG, GIF, WebP, BMP or ICO) and makes it
/// a [`SIDE`]-pixel square that fills a circle.
pub fn from_bytes(bytes: &[u8]) -> Result<RgbaImage, Error> {
    let mut reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| Error(e.to_string()))?;
    let mut limits = Limits::default();
    limits.max_alloc = Some(64 * 1024 * 1024);
    limits.max_image_width = Some(8192);
    limits.max_image_height = Some(8192);
    reader.limits(limits);
    let image = reader.decode().map_err(|e| Error(e.to_string()))?;
    let image = if image.width() > WORK_SIDE || image.height() > WORK_SIDE {
        image.resize(WORK_SIDE, WORK_SIDE, FilterType::Triangle)
    } else {
        image
    };
    Ok(fill_circle(&image.into_rgba8()))
}

/// Draws an SVG logo (a BIMI logo, or an icon a website names) and makes
/// it a [`SIDE`]-pixel square that fills a circle. Nothing it links to is
/// loaded, on disk or elsewhere.
pub fn from_svg(bytes: &[u8]) -> Result<RgbaImage, Error> {
    use resvg::{tiny_skia, usvg};

    let mut options = usvg::Options::default();
    options.image_href_resolver.resolve_string = Box::new(|_, _| None);
    let tree = usvg::Tree::from_data(bytes, &options).map_err(|e| Error(e.to_string()))?;
    let size = tree.size();
    let scale = 512.0 / size.width().max(size.height());
    let (w, h) = (
        ((size.width() * scale).round() as u32).max(1),
        ((size.height() * scale).round() as u32).max(1),
    );
    let mut pixmap = tiny_skia::Pixmap::new(w, h).ok_or_else(|| Error("empty SVG".into()))?;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    let image = RgbaImage::from_fn(w, h, |x, y| {
        pixmap.pixel(x, y).map_or(Rgba([0; 4]), |pixel| {
            let pixel = pixel.demultiply();
            Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
        })
    });
    Ok(fill_circle(&image))
}

/// `image` as a [`SIDE`]-pixel opaque square whose inscribed circle it
/// fills.
pub fn fill_circle(image: &RgbaImage) -> RgbaImage {
    let Some(seen) = bounds(image, |p| p[3] >= SEEN) else {
        return RgbaImage::from_pixel(SIDE, SIDE, WHITE);
    };
    let image = crop(image, seen);
    // A solid tile (see-through rounded corners aside) of one color with
    // a mark on it; a glyph of one color on nothing is not one.
    let tile = opaque_share(&image) >= 0.85;
    if let Some(background) = border_color(&image).filter(|_| tile) {
        let Some(inner) = bounds(&image, |p| p[3] >= SEEN && !near(p, background)) else {
            return RgbaImage::from_pixel(SIDE, SIDE, background);
        };
        // A thin frame around a full picture: crop the frame away.
        let (w, h) = image.dimensions();
        let content = crop(&image, inner);
        if inner.2 * 100 >= w * 88 && inner.3 * 100 >= h * 88 && covers(&content) {
            return cover(&content);
        }
        return on_disc(&content, background);
    }
    if covers(&image) {
        return cover(&image);
    }
    // Not square enough to crop, or see-through: a disc behind it.
    let background = if tile {
        average_border(&image)
    } else if light(&image) {
        DARK
    } else {
        WHITE
    };
    on_disc(&image, background)
}

/// A rectangle as x, y, width, height.
type Rect = (u32, u32, u32, u32);

/// The smallest rectangle holding every pixel `keep` accepts.
fn bounds(image: &RgbaImage, keep: impl Fn(&Rgba<u8>) -> bool) -> Option<Rect> {
    let (mut left, mut top, mut right, mut bottom) = (u32::MAX, u32::MAX, 0, 0);
    for (x, y, pixel) in image.enumerate_pixels() {
        if keep(pixel) {
            left = left.min(x);
            top = top.min(y);
            right = right.max(x);
            bottom = bottom.max(y);
        }
    }
    (left != u32::MAX).then(|| (left, top, right - left + 1, bottom - top + 1))
}

fn crop(image: &RgbaImage, (x, y, w, h): Rect) -> RgbaImage {
    imageops::crop_imm(image, x, y, w, h).to_image()
}

fn near(pixel: &Rgba<u8>, color: Rgba<u8>) -> bool {
    pixel.0[..3]
        .iter()
        .zip(&color.0[..3])
        .all(|(a, b)| a.abs_diff(*b) <= NEAR)
}

/// The pixels along the edges of `image`.
fn border(image: &RgbaImage) -> impl Iterator<Item = &Rgba<u8>> {
    let (w, h) = image.dimensions();
    let rows = (0..w).flat_map(move |x| [(x, 0), (x, h - 1)]);
    let columns = (1..h.saturating_sub(1)).flat_map(move |y| [(0, y), (w - 1, y)]);
    rows.chain(columns).map(|(x, y)| image.get_pixel(x, y))
}

/// The one color nearly all of `image`'s edge is (see-through corners
/// aside, as on a rounded square), if it has one.
fn border_color(image: &RgbaImage) -> Option<Rgba<u8>> {
    let mut all = 0usize;
    let mut buckets = std::collections::HashMap::<[u8; 3], (usize, [u64; 3])>::new();
    for pixel in border(image) {
        all += 1;
        if pixel[3] < 250 {
            continue;
        }
        let key = [pixel[0] >> 4, pixel[1] >> 4, pixel[2] >> 4];
        let bucket = buckets.entry(key).or_default();
        bucket.0 += 1;
        for (sum, channel) in bucket.1.iter_mut().zip(&pixel.0[..3]) {
            *sum += u64::from(*channel);
        }
    }
    let opaque: usize = buckets.values().map(|b| b.0).sum();
    let (count, sums) = buckets.into_values().max_by_key(|b| b.0)?;
    if opaque * 2 < all || count * 10 < opaque * 9 {
        return None;
    }
    let n = count as u64;
    Some(Rgba([
        (sums[0] / n) as u8,
        (sums[1] / n) as u8,
        (sums[2] / n) as u8,
        0xff,
    ]))
}

/// Whether `image` is about square and solid across its inscribed
/// circle, so cropping it to the circle loses nothing that matters.
fn covers(image: &RgbaImage) -> bool {
    let (w, h) = image.dimensions();
    if w.max(h) * 4 > w.min(h) * 5 {
        return false;
    }
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let r = w.min(h) as f32 * 0.46;
    let samples = 96;
    let solid = (0..samples)
        .filter(|i| {
            let angle = *i as f32 / samples as f32 * std::f32::consts::TAU;
            let x = (cx + r * angle.cos()).clamp(0.0, w as f32 - 1.0) as u32;
            let y = (cy + r * angle.sin()).clamp(0.0, h as f32 - 1.0) as u32;
            image.get_pixel(x, y)[3] >= 200
        })
        .count();
    solid * 100 >= samples * 94
}

/// `image` cropped to a centered square and scaled to [`SIDE`].
fn cover(image: &RgbaImage) -> RgbaImage {
    let square = DynamicImage::ImageRgba8(image.clone())
        .resize_to_fill(SIDE, SIDE, FilterType::Lanczos3)
        .into_rgba8();
    let mut out = RgbaImage::from_pixel(SIDE, SIDE, WHITE);
    imageops::overlay(&mut out, &square, 0, 0);
    out
}

/// `logo` centered on a disc of `background`, as large as fits.
fn on_disc(logo: &RgbaImage, background: Rgba<u8>) -> RgbaImage {
    let (w, h) = logo.dimensions();
    let diagonal = ((w * w + h * h) as f32).sqrt();
    let scale = LOGO_DIAGONAL * SIDE as f32 / diagonal;
    let (sw, sh) = (
        ((w as f32 * scale).round() as u32).clamp(1, SIDE),
        ((h as f32 * scale).round() as u32).clamp(1, SIDE),
    );
    let scaled = imageops::resize(logo, sw, sh, FilterType::Lanczos3);
    let mut out = RgbaImage::from_pixel(SIDE, SIDE, background);
    imageops::overlay(
        &mut out,
        &scaled,
        i64::from((SIDE - sw) / 2),
        i64::from((SIDE - sh) / 2),
    );
    out
}

/// The share of `image`'s pixels that are opaque.
fn opaque_share(image: &RgbaImage) -> f32 {
    let opaque = image.pixels().filter(|p| p[3] >= 250).count();
    opaque as f32 / image.pixels().len().max(1) as f32
}

/// The average color of `image`'s edge.
fn average_border(image: &RgbaImage) -> Rgba<u8> {
    let (mut n, mut sums) = (0u64, [0u64; 3]);
    for pixel in border(image) {
        n += 1;
        for (sum, channel) in sums.iter_mut().zip(&pixel.0[..3]) {
            *sum += u64::from(*channel);
        }
    }
    let n = n.max(1);
    Rgba([
        (sums[0] / n) as u8,
        (sums[1] / n) as u8,
        (sums[2] / n) as u8,
        0xff,
    ])
}

/// Whether the visible pixels of `image` are mostly light, so it needs a
/// dark disc to be seen.
fn light(image: &RgbaImage) -> bool {
    let (mut weight, mut sum) = (0f32, 0f32);
    for pixel in image.pixels() {
        let alpha = f32::from(pixel[3]) / 255.0;
        let luma = (0.2126 * f32::from(pixel[0])
            + 0.7152 * f32::from(pixel[1])
            + 0.0722 * f32::from(pixel[2]))
            / 255.0;
        weight += alpha;
        sum += alpha * luma;
    }
    weight > 0.0 && sum / weight > 0.8
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEAR: Rgba<u8> = Rgba([0, 0, 0, 0]);
    const RED: Rgba<u8> = Rgba([200, 20, 20, 255]);
    const BLUE: Rgba<u8> = Rgba([20, 40, 200, 255]);

    /// An icon `side` pixels square with `margin` see-through pixels
    /// around a rounded square of `fill`.
    fn rounded(side: u32, margin: u32, fill: impl Fn(u32, u32) -> Rgba<u8>) -> RgbaImage {
        let r = (side - 2 * margin) as f32 * 0.22;
        let (lo, hi) = (margin as f32, (side - margin) as f32);
        RgbaImage::from_fn(side, side, |x, y| {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            if fx < lo || fy < lo || fx > hi || fy > hi {
                return CLEAR;
            }
            let dx = (lo + r - fx).max(fx - (hi - r)).max(0.0);
            let dy = (lo + r - fy).max(fy - (hi - r)).max(0.0);
            if dx * dx + dy * dy > r * r {
                return CLEAR;
            }
            fill(x, y)
        })
    }

    fn close(a: Rgba<u8>, b: Rgba<u8>) -> bool {
        a[3] == 255 && near(&a, b)
    }

    #[test]
    fn a_padded_picture_icon_fills_the_circle() {
        // A gradient rounded square with a margin, like an apple-touch-icon.
        let icon = rounded(180, 20, |x, y| {
            Rgba([(x + 40) as u8, (y + 40) as u8, 180, 255])
        });
        let out = fill_circle(&icon);
        assert_eq!(out.dimensions(), (SIDE, SIDE));
        // The edge of the circle is the picture, not a white ring.
        for (x, y) in [(SIDE / 2, 2), (2, SIDE / 2), (SIDE / 2, SIDE - 3)] {
            let p = *out.get_pixel(x, y);
            assert!(!close(p, WHITE), "{x},{y}: {p:?}");
        }
    }

    #[test]
    fn a_logo_on_its_own_color_sits_on_a_disc_of_that_color() {
        // A red mark on a blue rounded square.
        let icon = rounded(180, 0, |x, y| {
            if (70..110).contains(&x) && (70..110).contains(&y) {
                RED
            } else {
                BLUE
            }
        });
        let out = fill_circle(&icon);
        assert!(close(*out.get_pixel(SIDE / 2, 2), BLUE));
        assert!(close(*out.get_pixel(SIDE / 2, SIDE / 2), RED));
        // The mark is enlarged to fill the disc.
        assert!(close(*out.get_pixel(SIDE / 2, SIDE / 2 - 30), RED));
    }

    #[test]
    fn a_white_margin_is_trimmed() {
        let icon = RgbaImage::from_fn(64, 64, |x, y| {
            if (28..36).contains(&x) && (28..36).contains(&y) {
                RED
            } else {
                WHITE
            }
        });
        let out = fill_circle(&icon);
        assert!(close(*out.get_pixel(SIDE / 2, 4), WHITE));
        assert!(close(*out.get_pixel(SIDE / 2 - 30, SIDE / 2), RED));
    }

    #[test]
    fn see_through_logos_get_a_disc_they_show_on() {
        let glyph = |color| {
            RgbaImage::from_fn(64, 64, move |x, y| {
                if (16..48).contains(&x) && (8..56).contains(&y) && (x + y) % 3 != 0 {
                    color
                } else {
                    CLEAR
                }
            })
        };
        let dark = fill_circle(&glyph(Rgba([10, 10, 10, 255])));
        assert!(close(*dark.get_pixel(SIDE / 2, 2), WHITE));
        let light = fill_circle(&glyph(WHITE));
        assert!(close(*light.get_pixel(SIDE / 2, 2), DARK));
    }

    #[test]
    fn a_wide_banner_is_not_cropped() {
        let banner = RgbaImage::from_fn(300, 100, |x, _| if x < 150 { RED } else { BLUE });
        let out = fill_circle(&banner);
        // Both halves stay in view.
        assert!(close(*out.get_pixel(SIDE / 2 - 40, SIDE / 2), RED));
        assert!(close(*out.get_pixel(SIDE / 2 + 40, SIDE / 2), BLUE));
    }

    #[test]
    fn a_solid_glyph_is_not_taken_for_a_background() {
        // A black "N" with nothing around it.
        let n = RgbaImage::from_fn(60, 60, |x, y| {
            if !(12..48).contains(&x) || x.abs_diff(y) < 8 {
                Rgba([0, 0, 0, 255])
            } else {
                CLEAR
            }
        });
        let out = fill_circle(&n);
        assert!(close(*out.get_pixel(SIDE / 2, 2), WHITE));
    }

    #[test]
    fn svg_logos_are_drawn() {
        let bimi = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            <rect width="100" height="100" fill="#1428c8"/>
            <circle cx="50" cy="50" r="10" fill="#c81414"/>
            <image href="/etc/passwd" width="100" height="100"/>
        </svg>"##;
        let out = from_svg(bimi).unwrap();
        assert!(close(*out.get_pixel(SIDE / 2, 2), BLUE));
        assert!(close(*out.get_pixel(SIDE / 2, SIDE / 2), RED));
        assert!(from_svg(b"<svg").is_err());
    }

    #[test]
    fn nothing_visible_is_a_white_disc() {
        let out = fill_circle(&RgbaImage::from_pixel(16, 16, CLEAR));
        assert!(close(*out.get_pixel(0, 0), WHITE));
        assert!(from_bytes(b"nope").is_err());
    }
}
