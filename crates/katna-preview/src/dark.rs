// SPDX-License-Identifier: GPL-3.0-or-later

//! Dark pages: a bright page shown dark for reading in dark mode.
//! Lightness is flipped and hue kept, so white paper turns a soft dark
//! grey, black ink a soft light grey, and a red heading stays red.
//! Photos are not flipped (a flipped photo looks like a negative); they
//! are dimmed instead. A scan (a picture that is mostly plain paper with
//! dark marks) is flipped like text.

use image::RgbaImage;

/// How much photos are dimmed to.
const PHOTO: f32 = 0.72;
/// A picture is a scan when at least this share of it is plain paper.
const SCAN_PAPER: f32 = 0.5;

/// One colour channel flipped: lightness turned over with hue and
/// saturation kept (invert, then the complement), squeezed between
/// 10% and 90% so the page is never pure black nor the ink pure white.
fn channel(c: u8, max: u8, min: u8) -> u8 {
    let flipped = 255.0 + f32::from(c) - f32::from(max) - f32::from(min);
    (25.5 + flipped * 0.8).round() as u8
}

/// `[r, g, b]` flipped for a dark page.
pub fn flip(rgb: [u8; 3]) -> [u8; 3] {
    let max = rgb[0].max(rgb[1]).max(rgb[2]);
    let min = rgb[0].min(rgb[1]).min(rgb[2]);
    rgb.map(|c| channel(c, max, min))
}

/// A `0xRRGGBBAA` colour flipped for a dark page, alpha kept.
pub fn flip_rgba(color: u32) -> u32 {
    let [r, g, b, a] = color.to_be_bytes();
    let [r, g, b] = flip([r, g, b]);
    u32::from_be_bytes([r, g, b, a])
}

/// Whether a pixel looks like plain paper: light and nearly grey.
fn paper(rgb: [u8; 3]) -> bool {
    let max = rgb[0].max(rgb[1]).max(rgb[2]);
    let min = rgb[0].min(rgb[1]).min(rgb[2]);
    max >= 200 && max - min <= 40
}

/// A picture on a page, in pixels: left, top, right, bottom.
pub type Area = (u32, u32, u32, u32);

/// `image` (a drawn page) made dark: flipped everywhere except inside
/// `pictures` that are photos, which are dimmed.
pub fn darken(image: &mut RgbaImage, pictures: &[Area]) {
    let (w, h) = image.dimensions();
    let photos: Vec<Area> = pictures
        .iter()
        .map(|&(x0, y0, x1, y1)| (x0.min(w), y0.min(h), x1.min(w), y1.min(h)))
        .filter(|&(x0, y0, x1, y1)| x1 > x0 && y1 > y0 && !scan(image, (x0, y0, x1, y1)))
        .collect();
    for (x, y, px) in image.enumerate_pixels_mut() {
        let rgb = [px[0], px[1], px[2]];
        let photo = photos
            .iter()
            .any(|&(x0, y0, x1, y1)| x >= x0 && x < x1 && y >= y0 && y < y1);
        let [r, g, b] = if photo {
            rgb.map(|c| (f32::from(c) * PHOTO).round() as u8)
        } else {
            flip(rgb)
        };
        px[0] = r;
        px[1] = g;
        px[2] = b;
    }
}

/// Whether the part of `image` in `area` is mostly plain paper, looked
/// at on a sparse grid.
fn scan(image: &RgbaImage, (x0, y0, x1, y1): Area) -> bool {
    let step = ((x1 - x0).max(y1 - y0) / 64).max(1);
    let (mut all, mut light) = (0u32, 0u32);
    for y in (y0..y1).step_by(step as usize) {
        for x in (x0..x1).step_by(step as usize) {
            let px = image.get_pixel(x, y);
            all += 1;
            light += u32::from(paper([px[0], px[1], px[2]]));
        }
    }
    all > 0 && light as f32 >= all as f32 * SCAN_PAPER
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn paper_goes_dark_and_ink_light() {
        assert_eq!(flip([255, 255, 255]), [26, 26, 26]);
        assert_eq!(flip([0, 0, 0]), [230, 230, 230]);
        // Red stays red: the strongest channel is still red.
        let [r, g, b] = flip([200, 30, 30]);
        assert!(r > g + 60 && g == b);
        assert_eq!(flip_rgba(0xffffff80), 0x1a1a1a80);
    }

    #[test]
    fn photos_dim_and_scans_flip() {
        let mut page = RgbaImage::from_pixel(40, 20, Rgba([255, 255, 255, 255]));
        // A photo on the left (dark green), a scan on the right (paper
        // with a black stroke).
        for y in 0..20 {
            for x in 0..20 {
                page.put_pixel(x, y, Rgba([40, 120, 60, 255]));
            }
            page.put_pixel(30, y, Rgba([0, 0, 0, 255]));
        }
        darken(&mut page, &[(0, 0, 20, 20), (20, 0, 40, 20)]);
        assert_eq!(page.get_pixel(5, 5).0, [29, 86, 43, 255]);
        assert_eq!(page.get_pixel(25, 5).0, [26, 26, 26, 255]);
        assert_eq!(page.get_pixel(30, 5).0, [230, 230, 230, 255]);
    }
}
