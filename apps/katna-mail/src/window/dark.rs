// SPDX-License-Identifier: GPL-3.0-or-later

//! Dark mode for HTML mail.
//!
//! Mail is written for a white page. In a dark theme its colors are
//! remapped, the way mail apps with a dark mode do it: light backgrounds
//! become dark ones of the same hue (white becomes the reading pane
//! itself), dark backgrounds stay, and text that would be hard to read on
//! its new background has its lightness flipped and, if needed, pushed
//! further until it reads well. Colors that already work, like white text
//! on a strong blue button, are left alone. Images are not changed.

/// Contrast text is brought up to against its background (WCAG AA).
const READABLE: f32 = 4.5;

/// Text already this readable keeps its color, so a sender's pairing like
/// white on a strong blue button stays as it was.
const KEEP: f32 = 3.0;

/// Backgrounds this light or lighter are darkened.
const LIGHT_BACKGROUND: f32 = 0.35;

pub(super) struct Dark {
    /// Where white goes: the reading pane.
    surface: u32,
    /// The perceived lightness (CIE L*) of `surface`.
    base: f32,
}

impl Dark {
    pub(super) fn new(surface: u32) -> Self {
        Self {
            surface,
            base: lightness(luminance(surface)),
        }
    }

    /// A background color for the dark page.
    pub(super) fn background(&self, color: u32) -> u32 {
        let alpha = color & 0xff;
        if alpha == 0 || luminance(color) < LIGHT_BACKGROUND {
            return color;
        }
        let (h, s, l) = hsl(color);
        if l >= 0.97 {
            return (self.surface & 0xffff_ff00) | alpha;
        }
        // As much darker than the pane as the color was darker than white,
        // judged by eye, then found by lowering its HSL lightness.
        let target = self.base + (100.0 - lightness(luminance(color))) * 0.6;
        // Near-white tints (cream, pale grey-blue) turn muddy when kept.
        let s = if l > 0.9 { s * 0.4 } else { s * 0.8 };
        let (mut low, mut high) = (0.0, l);
        for _ in 0..16 {
            let mid = (low + high) / 2.0;
            if lightness(luminance(rgb(h, s, mid))) < target {
                low = mid;
            } else {
                high = mid;
            }
        }
        rgb(h, s, low) | alpha
    }

    /// A text color readable on `background` (already a dark-page color).
    pub(super) fn text(&self, color: u32, background: u32) -> u32 {
        if contrast(color, background) >= KEEP {
            return color;
        }
        let alpha = color & 0xff;
        let (h, s, l) = hsl(color);
        let dark_background = luminance(background) < 0.18;
        let mut l = if dark_background {
            l.max(1.0 - l)
        } else {
            l.min(1.0 - l)
        };
        let step = if dark_background { 0.02 } else { -0.02 };
        loop {
            let out = rgb(h, s, l) | alpha;
            if contrast(out, background) >= READABLE || !(0.0..=1.0).contains(&(l + step)) {
                return out;
            }
            l += step;
        }
    }
}

fn channels(color: u32) -> [f32; 3] {
    [24, 16, 8].map(|shift| ((color >> shift) & 0xff) as f32 / 255.0)
}

/// Relative luminance (WCAG).
fn luminance(color: u32) -> f32 {
    let [r, g, b] = channels(color).map(|c| {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    });
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// CIE L* (0..100) of a relative luminance.
fn lightness(y: f32) -> f32 {
    if y > 216.0 / 24389.0 {
        116.0 * y.cbrt() - 16.0
    } else {
        y * 24389.0 / 27.0
    }
}

fn contrast(a: u32, b: u32) -> f32 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

/// Hue (0..1), saturation and lightness.
fn hsl(color: u32) -> (f32, f32, f32) {
    let [r, g, b] = channels(color);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    if d == 0.0 {
        return (0.0, 0.0, l);
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let h = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (h / 6.0, s, l)
}

/// `0xRRGGBB00` from hue, saturation and lightness.
fn rgb(h: f32, s: f32, l: f32) -> u32 {
    let l = l.clamp(0.0, 1.0);
    let s = s.clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h6 = h * 6.0;
    let x = c * (1.0 - (h6.rem_euclid(2.0) - 1.0).abs());
    let (r, g, b) = match h6 as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    [r, g, b]
        .map(|v| ((v + m).clamp(0.0, 1.0) * 255.0).round() as u32)
        .iter()
        .fold(0, |out, v| (out << 8) | v)
        << 8
}

#[cfg(test)]
mod tests {
    use super::*;

    const SURFACE: u32 = 0x1f1f1fff;

    #[test]
    fn hsl_round_trips() {
        for color in [0x1a73e800, 0xffcc0000, 0x22222200, 0xd9302500, 0x00800000] {
            let (h, s, l) = hsl(color);
            assert_eq!(rgb(h, s, l), color, "{color:08x}");
        }
    }

    #[test]
    fn white_becomes_the_pane() {
        let dark = Dark::new(SURFACE);
        assert_eq!(dark.background(0xffffffff), SURFACE);
        assert_eq!(dark.background(0xfafafaff), SURFACE);
    }

    #[test]
    fn light_backgrounds_darken_and_dark_ones_stay() {
        let dark = Dark::new(SURFACE);
        for light in [0xf1f3f4ff, 0xe8f0feff, 0xffcc00ff, 0xddddddff] {
            let out = dark.background(light);
            assert!(luminance(out) < 0.1, "{light:08x} -> {out:08x}");
        }
        for kept in [0x1a73e8ff, 0x000000ff, 0x202124ff, 0xd93025ff] {
            assert_eq!(dark.background(kept), kept);
        }
    }

    #[test]
    fn dark_text_becomes_readable() {
        let dark = Dark::new(SURFACE);
        for ink in [0x000000ff, 0x222222ff, 0x1a0dabff, 0x5f6368ff, 0x999999ff] {
            let out = dark.text(ink, SURFACE);
            assert!(contrast(out, SURFACE) >= READABLE, "{ink:08x} -> {out:08x}");
        }
        // A dark blue link stays blue.
        let (h, _, _) = hsl(dark.text(0x1a0dabff, SURFACE));
        assert!((h - hsl(0x1a0dabff).0).abs() < 0.02);
    }

    #[test]
    fn readable_text_is_kept() {
        let dark = Dark::new(SURFACE);
        assert_eq!(dark.text(0xffffffff, 0x1a73e8ff), 0xffffffff);
        assert_eq!(dark.text(0xe8eaedff, SURFACE), 0xe8eaedff);
    }
}
