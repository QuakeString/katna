// SPDX-License-Identifier: GPL-3.0-or-later

//! Pictures made ready for a signature built from one of Katna Mail's
//! layouts: a logo, a photo or a banner made small enough to travel
//! inside every message yet sharp at twice its shown size, and the
//! one-colour marks of the pages it links to, drawn in its colour.

use std::io::Cursor;

use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType as PngFilter, PngEncoder};
use image::imageops::FilterType;
use image::{DynamicImage, ImageEncoder, Rgba, RgbaImage};

use crate::Picture;
use crate::picture::{Error, read};

/// Pictures larger than this are not read at all.
const MAX_SIDE_READ: u32 = 8192;

/// What a picture is for, which decides its size and shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Kept whole, at most [`LOGO_BOX`].
    Logo,
    /// Cut to a circle [`PHOTO_SIDE`] pixels across.
    Photo,
    /// Kept whole, at most [`BANNER_BOX`].
    Banner,
}

/// Twice the largest box a layout shows a logo in.
pub const LOGO_BOX: (u32, u32) = (256, 128);
/// Twice the size a layout shows a photo at.
pub const PHOTO_SIDE: u32 = 136;
/// Twice the width a layout shows a banner at, and a sensible height.
pub const BANNER_BOX: (u32, u32) = (960, 400);

/// A picture ready to go inside the mail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Made {
    pub bytes: Vec<u8>,
    pub mime: &'static str,
    pub width: u32,
    pub height: u32,
}

/// `bytes` made ready to be `shape` in a signature.
pub fn make(bytes: &[u8], format: Picture, shape: Shape) -> Result<Made, Error> {
    let image = read(bytes, format, MAX_SIDE_READ)?;
    let image = match shape {
        Shape::Logo => backed(fit(image, LOGO_BOX).into_rgba8()),
        Shape::Photo => circle(
            &image
                .resize_to_fill(PHOTO_SIDE, PHOTO_SIDE, FilterType::Lanczos3)
                .into_rgba8(),
        ),
        Shape::Banner => fit(image, BANNER_BOX).into_rgba8(),
    };
    encode(&image)
}

/// `image` scaled down, never up, to fit in `(width, height)`.
fn fit(image: DynamicImage, (width, height): (u32, u32)) -> DynamicImage {
    if image.width() <= width && image.height() <= height {
        image
    } else {
        image.resize(width, height, FilterType::Lanczos3)
    }
}

/// A logo that is mostly dark ink on a see-through ground vanishes in a
/// dark reader: it is put on a soft white card with rounded corners.
fn backed(image: RgbaImage) -> RgbaImage {
    let pixels = image.pixels().count().max(1);
    let clear = image.pixels().filter(|p| p[3] < 250).count();
    let ink: Vec<f32> = image
        .pixels()
        .filter(|p| p[3] >= 128)
        .map(|p| {
            (0.2126 * f32::from(p[0]) + 0.7152 * f32::from(p[1]) + 0.0722 * f32::from(p[2])) / 255.0
        })
        .collect();
    let dark = !ink.is_empty() && ink.iter().sum::<f32>() / (ink.len() as f32) < 0.35;
    if clear * 20 < pixels || !dark {
        return image;
    }
    let pad = (image.width().min(image.height()) / 8).max(4);
    let (w, h) = (image.width() + 2 * pad, image.height() + 2 * pad);
    let radius = (w.min(h) as f32) * 0.18;
    let mut out = RgbaImage::from_fn(w, h, |x, y| {
        Rgba([255, 255, 255, cover(x, y, w, h, radius)])
    });
    image::imageops::overlay(&mut out, &image, i64::from(pad), i64::from(pad));
    out
}

/// How much of pixel `(x, y)` a `w` × `h` rounded rectangle covers, as
/// alpha.
fn cover(x: u32, y: u32, w: u32, h: u32, radius: f32) -> u8 {
    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
    let cx = px.clamp(radius, w as f32 - radius);
    let cy = py.clamp(radius, h as f32 - radius);
    let d = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
    ((radius + 0.5 - d).clamp(0.0, 1.0) * 255.0).round() as u8
}

/// `image` (square) cut to the circle it holds, with a smooth edge, so
/// the photo is round even where a reader ignores rounded corners.
fn circle(image: &RgbaImage) -> RgbaImage {
    let side = image.width().min(image.height());
    let r = side as f32 / 2.0;
    RgbaImage::from_fn(side, side, |x, y| {
        let p = image.get_pixel(x, y);
        let d = ((x as f32 + 0.5 - r).powi(2) + (y as f32 + 0.5 - r).powi(2)).sqrt();
        let a = (r - d + 0.5).clamp(0.0, 1.0);
        Rgba([p[0], p[1], p[2], (f32::from(p[3]) * a).round() as u8])
    })
}

/// The smaller of PNG and, for a picture with nothing see-through, JPEG.
fn encode(image: &RgbaImage) -> Result<Made, Error> {
    let (width, height) = image.dimensions();
    let png = png(image)?;
    let opaque = image.pixels().all(|p| p[3] == 255);
    if opaque {
        let rgb = DynamicImage::ImageRgba8(image.clone()).into_rgb8();
        let mut jpeg = Vec::new();
        JpegEncoder::new_with_quality(&mut Cursor::new(&mut jpeg), 86)
            .write_image(&rgb, width, height, image::ExtendedColorType::Rgb8)
            .map_err(|e| Error(e.to_string()))?;
        if jpeg.len() < png.len() {
            return Ok(Made {
                bytes: jpeg,
                mime: "image/jpeg",
                width,
                height,
            });
        }
    }
    Ok(Made {
        bytes: png,
        mime: "image/png",
        width,
        height,
    })
}

fn png(image: &RgbaImage) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    PngEncoder::new_with_quality(&mut out, CompressionType::Best, PngFilter::Adaptive)
        .write_image(
            image,
            image.width(),
            image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| Error(e.to_string()))?;
    Ok(out)
}

/// A one-colour mark (an SVG drawn in black) as a `side`-pixel PNG in
/// `rgb` (`0xrrggbb`).
pub fn tinted_mark(svg: &[u8], rgb: u32, side: u32) -> Result<Vec<u8>, Error> {
    use resvg::{tiny_skia, usvg};

    let mut options = usvg::Options::default();
    options.image_href_resolver.resolve_string = Box::new(|_, _| None);
    let tree = usvg::Tree::from_data(svg, &options).map_err(|e| Error(e.to_string()))?;
    let size = tree.size();
    let scale = side as f32 / size.width().max(size.height());
    let mut pixmap = tiny_skia::Pixmap::new(side, side).ok_or_else(|| Error("empty".into()))?;
    let (dx, dy) = (
        (side as f32 - size.width() * scale) / 2.0,
        (side as f32 - size.height() * scale) / 2.0,
    );
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale).post_translate(dx, dy),
        &mut pixmap.as_mut(),
    );
    let [_, r, g, b] = rgb.to_be_bytes();
    let image = RgbaImage::from_fn(side, side, |x, y| {
        let alpha = pixmap.pixel(x, y).map_or(0, |p| p.alpha());
        Rgba([r, g, b, alpha])
    });
    png(&image)
}

/// `initials` in white on a circle of `rgb` (`0xrrggbb`), `side` pixels
/// across, as PNG: round in every reader, even where rounded corners are
/// ignored.
pub fn monogram(initials: &str, rgb: u32, side: u32) -> Result<Vec<u8>, Error> {
    use resvg::{tiny_skia, usvg};

    let text = initials
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{side}" height="{side}" viewBox="0 0 100 100"><circle cx="50" cy="50" r="50" fill="#{rgb:06x}"/><text x="50" y="50" dy="0.35em" text-anchor="middle" font-family="sans-serif" font-weight="bold" font-size="38" fill="#ffffff">{text}</text></svg>"##
    );
    let mut options = usvg::Options {
        fontdb: crate::table::fonts(),
        ..usvg::Options::default()
    };
    options.font_family = "sans-serif".into();
    let tree = usvg::Tree::from_str(&svg, &options).map_err(|e| Error(e.to_string()))?;
    let mut pixmap = tiny_skia::Pixmap::new(side, side).ok_or_else(|| Error("empty".into()))?;
    // The view box already scales it to `side`.
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let image = RgbaImage::from_fn(side, side, |x, y| {
        pixmap.pixel(x, y).map_or(Rgba([0; 4]), |p| {
            let p = p.demultiply();
            Rgba([p.red(), p.green(), p.blue(), p.alpha()])
        })
    });
    png(&image)
}

/// A bar of `rgb` (`0xrrggbb`) with round ends, `width` × `height`
/// pixels, as PNG.
pub fn bar(rgb: u32, width: u32, height: u32) -> Result<Vec<u8>, Error> {
    let [_, r, g, b] = rgb.to_be_bytes();
    let radius = width.min(height) as f32 / 2.0;
    let image = RgbaImage::from_fn(width, height, |x, y| {
        Rgba([r, g, b, cover(x, y, width, height, radius)])
    });
    png(&image)
}

/// The size of a picture from its header.
pub fn size(bytes: &[u8]) -> Option<(u32, u32)> {
    crate::picture::dimensions(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_of(image: &RgbaImage) -> Vec<u8> {
        png(image).unwrap()
    }

    #[test]
    fn a_big_logo_is_made_small() {
        let logo = RgbaImage::from_pixel(1200, 400, Rgba([14, 124, 134, 255]));
        let made = make(&png_of(&logo), Picture::Png, Shape::Logo).unwrap();
        assert_eq!((made.width, made.height), (256, 85));
        assert!(made.bytes.len() < 40 * 1024);
        // Never made larger.
        let small = RgbaImage::from_pixel(40, 20, Rgba([14, 124, 134, 255]));
        let made = make(&png_of(&small), Picture::Png, Shape::Logo).unwrap();
        assert_eq!((made.width, made.height), (40, 20));
    }

    #[test]
    fn a_dark_logo_on_nothing_gets_a_white_card() {
        let logo = RgbaImage::from_fn(64, 64, |x, _| {
            if x < 32 {
                Rgba([10, 10, 10, 255])
            } else {
                Rgba([0, 0, 0, 0])
            }
        });
        let made = make(&png_of(&logo), Picture::Png, Shape::Logo).unwrap();
        assert_eq!((made.width, made.height), (80, 80));
        let back = image::load_from_memory(&made.bytes).unwrap().into_rgba8();
        assert_eq!(back.get_pixel(40, 8).0, [255, 255, 255, 255]);
        assert_eq!(back.get_pixel(0, 0)[3], 0, "rounded corner");
        // A light one on nothing, or a dark one on its own ground, stays.
        let light = RgbaImage::from_fn(64, 64, |x, _| {
            if x < 32 {
                Rgba([240, 200, 40, 255])
            } else {
                Rgba([0, 0, 0, 0])
            }
        });
        let made = make(&png_of(&light), Picture::Png, Shape::Logo).unwrap();
        assert_eq!((made.width, made.height), (64, 64));
    }

    #[test]
    fn a_photo_is_round() {
        let photo = RgbaImage::from_pixel(600, 400, Rgba([200, 150, 120, 255]));
        let made = make(&png_of(&photo), Picture::Png, Shape::Photo).unwrap();
        assert_eq!((made.width, made.height), (PHOTO_SIDE, PHOTO_SIDE));
        assert_eq!(made.mime, "image/png");
        let back = image::load_from_memory(&made.bytes).unwrap().into_rgba8();
        assert_eq!(back.get_pixel(0, 0)[3], 0);
        assert_eq!(back.get_pixel(68, 68)[3], 255);
    }

    #[test]
    fn a_monogram_and_a_bar() {
        let made = monogram("DA", 0x0e7c86, 104).unwrap();
        let back = image::load_from_memory(&made).unwrap().into_rgba8();
        assert_eq!(back.dimensions(), (104, 104));
        assert_eq!(back.get_pixel(0, 0)[3], 0);
        assert_eq!(back.get_pixel(52, 8).0, [14, 124, 134, 255]);
        // The whole circle, not cut at the edge.
        assert_eq!(back.get_pixel(52, 100).0, [14, 124, 134, 255]);
        assert_eq!(back.get_pixel(100, 100)[3], 0);
        let made = bar(0x0e7c86, 72, 6).unwrap();
        let back = image::load_from_memory(&made).unwrap().into_rgba8();
        assert_eq!(back.dimensions(), (72, 6));
        assert_eq!(back.get_pixel(36, 3).0, [14, 124, 134, 255]);
    }

    #[test]
    fn a_mark_takes_the_colour() {
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><rect x="4" y="4" width="16" height="16" fill="black"/></svg>"#;
        let mark = tinted_mark(svg, 0x0e7c86, 36).unwrap();
        let back = image::load_from_memory(&mark).unwrap().into_rgba8();
        assert_eq!(back.dimensions(), (36, 36));
        assert_eq!(back.get_pixel(18, 18).0, [14, 124, 134, 255]);
        assert_eq!(back.get_pixel(1, 1)[3], 0);
    }
}
