// SPDX-License-Identifier: GPL-3.0-or-later

//! SVG pictures from mail (in a message's body, on the web or attached),
//! drawn to bitmaps here so the UI toolkit never parses one itself.
//!
//! GPUI draws SVG with usvg's default settings, whose `<image href="…">`
//! resolver reads any local path an SVG names: `/dev/zero` never ends, a
//! FIFO blocks, and the user's own pictures would show in a message.
//! Here nothing an SVG links to is loaded, on disk or elsewhere; pictures
//! embedded in it as `data:` URLs are drawn only when their size is sane,
//! compressed (`svgz`) data is refused, and the bitmap is kept to
//! [`MAX_SIDE`] and [`MAX_PIXELS`].

use std::io::Cursor;
use std::sync::Arc;

use image::{ImageReader, Rgba, RgbaImage};
use resvg::{tiny_skia, usvg};

use crate::picture::Error;

/// Larger SVG files are not read at all.
pub const MAX_BYTES: usize = 16 * 1024 * 1024;
/// A drawing is at most this many pixels on a side...
pub const MAX_SIDE: u32 = 4096;
/// ...and this many in all (64 MB of RGBA).
pub const MAX_PIXELS: u32 = 16_000_000;
/// A picture embedded in an SVG is drawn only up to this size: resvg
/// decodes it with no limits of its own.
const MAX_EMBEDDED_SIDE: u32 = 8192;
const MAX_EMBEDDED_PIXELS: u64 = 32_000_000;

/// The first bytes of gzip data (`svgz`).
const GZIP: [u8; 2] = [0x1f, 0x8b];

/// An SVG drawn to a bitmap.
#[derive(Debug, Clone)]
pub struct Drawing {
    /// Straight (not premultiplied) RGBA.
    pub image: RgbaImage,
    /// The picture's own size in CSS pixels. `image` is drawn at the
    /// asked scale of it, or smaller where the limits stop it.
    pub size: (f32, f32),
}

/// Draws the SVG in `bytes` at `scale` times its own size (2 is sharp on
/// a high-density screen), at most `max_side` pixels on a side and never
/// more than [`MAX_SIDE`] and [`MAX_PIXELS`].
pub fn draw(bytes: &[u8], scale: f32, max_side: u32) -> Result<Drawing, Error> {
    if bytes.len() > MAX_BYTES {
        return Err(Error("SVG too large".into()));
    }
    if bytes.starts_with(&GZIP) {
        // usvg would inflate it without a limit.
        return Err(Error("compressed SVG".into()));
    }
    let text = std::str::from_utf8(bytes).map_err(|e| Error(e.to_string()))?;
    let tree = usvg::Tree::from_str(text, &options()).map_err(|e| Error(e.to_string()))?;
    let size = tree.size();
    let (w, h) = (size.width(), size.height());
    let s = fit(w, h, scale, max_side);
    let (pw, ph) = (
        ((w * s).round() as u32).clamp(1, MAX_SIDE),
        ((h * s).round() as u32).clamp(1, MAX_SIDE),
    );
    let mut pixmap = tiny_skia::Pixmap::new(pw, ph).ok_or_else(|| Error("empty SVG".into()))?;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(pw as f32 / w, ph as f32 / h),
        &mut pixmap.as_mut(),
    );
    let image = RgbaImage::from_fn(pw, ph, |x, y| {
        pixmap.pixel(x, y).map_or(Rgba([0; 4]), |pixel| {
            let pixel = pixel.demultiply();
            Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
        })
    });
    Ok(Drawing {
        image,
        size: (w, h),
    })
}

/// The scale a `w` × `h` picture is drawn at: `scale`, or less to stay
/// within `max_side` and the limits.
fn fit(w: f32, h: f32, scale: f32, max_side: u32) -> f32 {
    let side = max_side.clamp(1, MAX_SIDE) as f32 / w.max(h);
    let area = (f64::from(MAX_PIXELS) / (f64::from(w) * f64::from(h))).sqrt() as f32;
    scale.min(side).min(area)
}

fn options() -> usvg::Options<'static> {
    let mut options = usvg::Options {
        fontdb: crate::table::fonts(),
        ..usvg::Options::default()
    };
    options.image_href_resolver = usvg::ImageHrefResolver {
        resolve_data: Box::new(embedded),
        // A path or a URL: never followed.
        resolve_string: Box::new(|_, _| None),
    };
    options
}

/// A picture embedded in the SVG as a `data:` URL: drawn by usvg's own
/// resolver when its size is within the limits. A nested SVG comes back
/// here for its own pictures.
fn embedded(mime: &str, data: Arc<Vec<u8>>, options: &usvg::Options) -> Option<usvg::ImageKind> {
    if data.starts_with(&GZIP) || data.len() > MAX_BYTES {
        return None;
    }
    if let Ok(reader) = ImageReader::new(Cursor::new(data.as_slice())).with_guessed_format()
        && reader.format().is_some()
    {
        let (w, h) = reader.into_dimensions().ok()?;
        if w > MAX_EMBEDDED_SIDE
            || h > MAX_EMBEDDED_SIDE
            || u64::from(w) * u64::from(h) > MAX_EMBEDDED_PIXELS
        {
            return None;
        }
    }
    (usvg::ImageHrefResolver::default_data_resolver())(mime, data, options)
}

/// `image` as PNG, for handing to a toolkit that decodes pictures itself.
pub fn png(image: &RgbaImage) -> Result<Vec<u8>, Error> {
    use image::ImageEncoder;
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out)
        .write_image(
            image,
            image.width(),
            image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| Error(e.to_string()))?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    const RED_SQUARE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10"><rect width="20" height="10" fill="#ff0000"/></svg>"##;

    #[test]
    fn draws_at_the_asked_scale() {
        let drawing = draw(RED_SQUARE.as_bytes(), 2.0, 4096).unwrap();
        assert_eq!(drawing.size, (20.0, 10.0));
        assert_eq!(drawing.image.dimensions(), (40, 20));
        assert_eq!(drawing.image.get_pixel(5, 5).0, [255, 0, 0, 255]);
    }

    /// The attack of the September 2026 audit: an `<image>` naming a
    /// device that never ends. Nothing is read and the drawing comes back
    /// at once.
    #[test]
    fn linked_files_are_never_read() {
        for href in ["/dev/zero", "file:///dev/zero", "/etc/passwd", "../x.png"] {
            let svg = format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="10" height="10"><image href="{href}" width="10" height="10"/><image xlink:href="{href}" width="10" height="10"/></svg>"#
            );
            let started = Instant::now();
            let drawing = draw(svg.as_bytes(), 1.0, 64).unwrap();
            assert!(started.elapsed() < Duration::from_secs(5));
            assert_eq!(drawing.image.dimensions(), (10, 10));
            // Nothing was drawn where the picture would be.
            assert!(drawing.image.pixels().all(|p| p.0[3] == 0), "{href}");
        }
        // A real picture on disk is not drawn either.
        let path = std::env::temp_dir().join(format!("katna-svg-test-{}.png", std::process::id()));
        std::fs::write(
            &path,
            png(&RgbaImage::from_pixel(10, 10, Rgba([0, 255, 0, 255]))).unwrap(),
        )
        .unwrap();
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><image href="{}" width="10" height="10"/></svg>"#,
            path.display()
        );
        let drawing = draw(svg.as_bytes(), 1.0, 64);
        let _ = std::fs::remove_file(&path);
        assert!(drawing.unwrap().image.pixels().all(|p| p.0[3] == 0));
    }

    #[test]
    fn huge_pictures_are_kept_small() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="60000" height="60000"><rect width="60000" height="60000" fill="#00f"/></svg>"##;
        let drawing = draw(svg.as_bytes(), 2.0, 100_000).unwrap();
        let (w, h) = drawing.image.dimensions();
        assert!(w <= MAX_SIDE && h <= MAX_SIDE);
        assert!(w * h <= MAX_PIXELS);
        assert_eq!(drawing.size, (60000.0, 60000.0));
        let small = draw(svg.as_bytes(), 2.0, 256).unwrap();
        assert_eq!(small.image.dimensions(), (256, 256));
    }

    #[test]
    fn embedded_pictures_are_drawn_within_limits() {
        let pixel = png(&RgbaImage::from_pixel(4, 4, Rgba([0, 255, 0, 255]))).unwrap();
        let base64 = base64(&pixel);
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="4" height="4"><image href="data:image/png;base64,{base64}" width="4" height="4"/></svg>"#
        );
        let drawing = draw(svg.as_bytes(), 1.0, 64).unwrap();
        assert_eq!(drawing.image.get_pixel(2, 2).0, [0, 255, 0, 255]);
    }

    #[test]
    fn compressed_and_garbage_are_errors() {
        assert!(draw(&[0x1f, 0x8b, 8, 0, 0, 0], 1.0, 64).is_err());
        assert!(draw(b"not an svg", 1.0, 64).is_err());
        assert!(draw(&[0xff, 0xfe, 0xfd], 1.0, 64).is_err());
    }

    fn base64(bytes: &[u8]) -> String {
        const ABC: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let n = chunk
                .iter()
                .enumerate()
                .fold(0u32, |n, (i, b)| n | u32::from(*b) << (16 - 8 * i));
            for i in 0..4 {
                if i <= chunk.len() {
                    out.push(ABC[(n >> (18 - 6 * i) & 63) as usize] as char);
                } else {
                    out.push('=');
                }
            }
        }
        out
    }
}
