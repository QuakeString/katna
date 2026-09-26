// SPDX-License-Identifier: GPL-3.0-or-later

//! Pictures decoded for the viewer and for attachment thumbnails, turned
//! upright by their EXIF orientation (phone photos) and kept to a size
//! worth drawing.

use std::io::Cursor;

use image::imageops::FilterType;
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits, RgbaImage};

use crate::Picture;

/// The viewer draws pictures at most this many pixels on a side; larger
/// ones are scaled down once when opened.
pub const VIEW_SIDE: u32 = 4096;
/// Nothing larger than this is decoded at all.
const MAX_DECODE_BYTES: u64 = 512 * 1024 * 1024;

/// Why a picture cannot be shown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

/// Decodes `bytes` at most `max_side` pixels on a side, upright.
pub fn decode(bytes: &[u8], format: Picture, max_side: u32) -> Result<RgbaImage, Error> {
    let image = read(bytes, format)?;
    let image = if image.width() > max_side || image.height() > max_side {
        image.resize(max_side, max_side, FilterType::Triangle)
    } else {
        image
    };
    Ok(image.into_rgba8())
}

/// A small picture covering at most `width` × `height` pixels, for the
/// attachment cards.
pub fn thumbnail(
    bytes: &[u8],
    format: Picture,
    width: u32,
    height: u32,
) -> Result<RgbaImage, Error> {
    let image = read(bytes, format)?;
    // Fill the card like CSS `object-fit: cover`: scale to cover, then crop.
    Ok(image
        .resize_to_fill(width, height, FilterType::Triangle)
        .into_rgba8())
}

fn read(bytes: &[u8], format: Picture) -> Result<DynamicImage, Error> {
    let format = match format {
        Picture::Png => ImageFormat::Png,
        Picture::Jpeg => ImageFormat::Jpeg,
        Picture::Gif => ImageFormat::Gif,
        Picture::Webp => ImageFormat::WebP,
        Picture::Bmp => ImageFormat::Bmp,
        Picture::Tiff => ImageFormat::Tiff,
        Picture::Svg => return Err(Error("SVG is drawn by the viewer".into())),
    };
    let mut reader = ImageReader::with_format(Cursor::new(bytes), format);
    // Senders mislabel pictures; trust the bytes over the MIME type.
    reader = reader
        .with_guessed_format()
        .map_err(|e| Error(e.to_string()))?;
    let mut limits = Limits::default();
    limits.max_alloc = Some(MAX_DECODE_BYTES);
    limits.max_image_width = Some(30_000);
    limits.max_image_height = Some(30_000);
    reader.limits(limits);
    let mut decoder = reader.into_decoder().map_err(|e| Error(e.to_string()))?;
    let orientation = decoder.orientation().ok();
    let mut image = DynamicImage::from_decoder(decoder).map_err(|e| Error(e.to_string()))?;
    if let Some(orientation) = orientation {
        image.apply_orientation(orientation);
    }
    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageEncoder, Rgba};

    fn png(width: u32, height: u32) -> Vec<u8> {
        let image = RgbaImage::from_fn(width, height, |x, _| {
            if x < width / 2 {
                Rgba([255, 0, 0, 255])
            } else {
                Rgba([0, 0, 255, 255])
            }
        });
        let mut out = Vec::new();
        image::codecs::png::PngEncoder::new(&mut out)
            .write_image(&image, width, height, image::ExtendedColorType::Rgba8)
            .unwrap();
        out
    }

    #[test]
    fn decodes_and_scales_down() {
        let bytes = png(400, 100);
        let full = decode(&bytes, Picture::Png, 4096).unwrap();
        assert_eq!(full.dimensions(), (400, 100));
        assert_eq!(full.get_pixel(0, 0).0, [255, 0, 0, 255]);
        let small = decode(&bytes, Picture::Png, 200).unwrap();
        assert_eq!(small.dimensions(), (200, 50));
    }

    #[test]
    fn mislabelled_pictures_still_decode() {
        let bytes = png(8, 8);
        assert_eq!(
            decode(&bytes, Picture::Jpeg, 64).unwrap().dimensions(),
            (8, 8)
        );
    }

    #[test]
    fn thumbnails_cover_the_card() {
        let thumb = thumbnail(&png(400, 100), Picture::Png, 90, 60).unwrap();
        assert_eq!(thumb.dimensions(), (90, 60));
    }

    #[test]
    fn garbage_is_an_error() {
        assert!(decode(b"nope", Picture::Png, 64).is_err());
    }
}
