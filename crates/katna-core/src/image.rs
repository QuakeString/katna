// SPDX-License-Identifier: GPL-3.0-or-later

//! Recognizing image files by their first bytes, whatever a server or a
//! message says they are.

/// The image formats Katna shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageKind {
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Ico,
    Svg,
}

impl ImageKind {
    /// Finds the format from the first bytes of an image.
    pub fn sniff(bytes: &[u8]) -> Option<Self> {
        let starts = |magic: &[u8]| bytes.starts_with(magic);
        if starts(b"\x89PNG\r\n\x1a\n") {
            Some(Self::Png)
        } else if starts(b"\xff\xd8\xff") {
            Some(Self::Jpeg)
        } else if starts(b"GIF87a") || starts(b"GIF89a") {
            Some(Self::Gif)
        } else if bytes.len() >= 12 && starts(b"RIFF") && &bytes[8..12] == b"WEBP" {
            Some(Self::Webp)
        } else if starts(b"BM") && bytes.len() > 26 {
            Some(Self::Bmp)
        } else if starts(b"\0\0\x01\0") && bytes.len() > 22 {
            Some(Self::Ico)
        } else {
            let head = &bytes[..bytes.len().min(1024)];
            let head = String::from_utf8_lossy(head).to_ascii_lowercase();
            let head = head.trim_start_matches('\u{feff}').trim_start();
            let svg = head.starts_with("<svg")
                || ((head.starts_with("<?xml")
                    || head.starts_with("<!doctype svg")
                    || head.starts_with("<!--"))
                    && head.contains("<svg"));
            svg.then_some(Self::Svg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniffs_by_content() {
        assert_eq!(
            ImageKind::sniff(b"\x89PNG\r\n\x1a\nrest"),
            Some(ImageKind::Png)
        );
        assert_eq!(ImageKind::sniff(b"\xff\xd8\xff\xe0"), Some(ImageKind::Jpeg));
        assert_eq!(ImageKind::sniff(b"GIF89a..."), Some(ImageKind::Gif));
        assert_eq!(
            ImageKind::sniff(b"RIFF\0\0\0\0WEBPVP8 "),
            Some(ImageKind::Webp)
        );
        assert_eq!(
            ImageKind::sniff(
                b"<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\">"
            ),
            Some(ImageKind::Svg)
        );
        assert_eq!(
            ImageKind::sniff(&[
                0, 0, 1, 0, 1, 0, 16, 16, 0, 0, 1, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]),
            Some(ImageKind::Ico)
        );
        assert_eq!(ImageKind::sniff(b"<!DOCTYPE html><html>"), None);
        assert_eq!(ImageKind::sniff(b""), None);
    }
}
