// SPDX-License-Identifier: GPL-3.0-or-later

//! PDF pages as bitmaps, drawn by hayro (pure Rust, on the CPU).

use std::sync::Arc;

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::{LoadPdfError, Pdf};
use hayro::vello_cpu::color::palette::css::WHITE;
use hayro::vello_cpu::kurbo::Affine;
use hayro::{RenderCache, RenderSettings};
use image::RgbaImage;

pub use crate::pdf_marks::SaveError;
pub use crate::pdf_text::TextLine;

/// A page is never drawn larger than this many pixels on a side...
pub const MAX_SIDE: f32 = 8192.0;
/// ...or than this many pixels in all (about 128 MB of RGBA).
pub const MAX_PIXELS: f32 = 32_000_000.0;

/// Why a PDF cannot be shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// It needs a password.
    Locked,
    /// It is damaged, or not a PDF.
    Invalid,
}

/// An open PDF. It can be shared between threads; each [`Document::render`]
/// draws one page.
pub struct Document {
    pdf: Pdf,
    /// Each page's size in points (1/72 inch), rotation applied.
    sizes: Vec<(f32, f32)>,
}

impl Document {
    pub fn open(bytes: Vec<u8>) -> Result<Self, Error> {
        let pdf = Pdf::new(Arc::new(bytes)).map_err(|err| match err {
            LoadPdfError::Decryption(_) => Error::Locked,
            LoadPdfError::Invalid => Error::Invalid,
        })?;
        let sizes: Vec<_> = pdf
            .pages()
            .iter()
            .map(|page| {
                let (w, h) = page.render_dimensions();
                (w.max(1.0), h.max(1.0))
            })
            .collect();
        if sizes.is_empty() {
            return Err(Error::Invalid);
        }
        Ok(Self { pdf, sizes })
    }

    /// The number of pages; at least one.
    pub fn pages(&self) -> usize {
        self.sizes.len()
    }

    /// The size of page `page` in points, rotation applied.
    pub fn page_size(&self, page: usize) -> (f32, f32) {
        self.sizes.get(page).copied().unwrap_or((612.0, 792.0))
    }

    /// Draws page `page` on white at `scale` pixels per point, made smaller
    /// if needed to stay within [`MAX_SIDE`] and [`MAX_PIXELS`].
    pub fn render(&self, page: usize, scale: f32) -> Option<RgbaImage> {
        let pages = self.pdf.pages();
        let pdf_page = pages.get(page)?;
        let (w, h) = self.page_size(page);
        let scale = fit_scale(w, h, scale);
        let settings = RenderSettings {
            x_scale: scale,
            y_scale: scale,
            bg_color: WHITE,
            ..Default::default()
        };
        let cache = RenderCache::new();
        let pixmap = hayro::render(pdf_page, &cache, &InterpreterSettings::default(), &settings);
        let (width, height) = (u32::from(pixmap.width()), u32::from(pixmap.height()));
        if width == 0 || height == 0 {
            return None;
        }
        // On an opaque white page premultiplied and straight alpha agree.
        RgbaImage::from_raw(width, height, pixmap.data_as_u8_slice().to_vec())
    }
}

impl Document {
    /// The text of page `page`, line by line, with where each character
    /// is drawn, in points from the page's top left.
    pub fn text(&self, page: usize) -> Vec<TextLine> {
        self.pdf
            .pages()
            .get(page)
            .map(crate::pdf_text::lines)
            .unwrap_or_default()
    }

    /// Whether marks can be saved into this PDF: not when it is encrypted
    /// or certified against changes. Reads the whole file again.
    pub fn can_mark(&self) -> Result<(), SaveError> {
        crate::pdf_marks::check(self.pdf.data().as_ref())
    }

    /// A copy of the file with `marks` added as annotations, the original
    /// bytes unchanged at its start.
    pub fn with_marks(&self, marks: &[crate::markup::Mark]) -> Result<Vec<u8>, SaveError> {
        let transforms: Vec<_> = self
            .pdf
            .pages()
            .iter()
            .map(|page| {
                // From points as drawn back to the page's own.
                let [a, b, c, d, e, f] = page.initial_transform(true).as_coeffs();
                Affine::new([a, b, c, d, e, f].map(f64::from))
                    .inverse()
                    .as_coeffs()
            })
            .collect();
        crate::pdf_marks::write(self.pdf.data().as_ref(), &transforms, marks)
    }
}

/// The top of the first page of `bytes`, `width` × `height` pixels, for
/// the attachment cards.
pub fn thumbnail(bytes: Vec<u8>, width: u32, height: u32) -> Option<RgbaImage> {
    let doc = Document::open(bytes).ok()?;
    let (w, _) = doc.page_size(0);
    let page = doc.render(0, width as f32 / w)?;
    let height = height.min(page.height());
    Some(image::imageops::crop_imm(&page, 0, 0, page.width().min(width), height).to_image())
}

/// `scale`, made smaller so a `w` × `h` point page stays within the limits.
fn fit_scale(w: f32, h: f32, scale: f32) -> f32 {
    let scale = scale.clamp(0.05, 16.0);
    let side = (MAX_SIDE / w.max(h)).min(scale);
    let area = (MAX_PIXELS / (w * h)).sqrt();
    side.min(area)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-page PDF (US Letter) with a black 100 × 100 pt square at the
    /// bottom left, cross-reference offsets computed.
    pub(crate) fn square_pdf() -> Vec<u8> {
        let content = b"0 0 0 rg 0 0 100 100 re f";
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_owned(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R >>".to_owned(),
            format!(
                "<< /Length {} >>\nstream\n{}\nendstream",
                content.len(),
                std::str::from_utf8(content).unwrap()
            ),
        ];
        let mut out = b"%PDF-1.4\n".to_vec();
        let mut offsets = Vec::new();
        for (i, body) in objects.iter().enumerate() {
            offsets.push(out.len());
            out.extend(format!("{} 0 obj\n{body}\nendobj\n", i + 1).as_bytes());
        }
        let xref = out.len();
        out.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes());
        for offset in offsets {
            out.extend(format!("{offset:010} 00000 n \n").as_bytes());
        }
        out.extend(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
                objects.len() + 1
            )
            .as_bytes(),
        );
        out
    }

    #[test]
    fn draws_a_page() {
        let doc = Document::open(square_pdf()).unwrap();
        assert_eq!(doc.pages(), 1);
        assert_eq!(doc.page_size(0), (612.0, 792.0));
        let page = doc.render(0, 0.5).unwrap();
        assert_eq!(page.dimensions(), (306, 396));
        // The square is at the bottom left, the rest is white paper.
        assert_eq!(page.get_pixel(10, 390).0, [0, 0, 0, 255]);
        assert_eq!(page.get_pixel(300, 10).0, [255, 255, 255, 255]);
    }

    #[test]
    fn thumbnails_show_the_top_of_the_first_page() {
        let thumb = thumbnail(square_pdf(), 306, 100).unwrap();
        assert_eq!(thumb.dimensions(), (306, 100));
        assert_eq!(thumb.get_pixel(10, 90).0, [255, 255, 255, 255]);
    }

    #[test]
    fn huge_pages_are_drawn_smaller() {
        assert_eq!(fit_scale(612.0, 792.0, 2.0), 2.0);
        let s = fit_scale(14_400.0, 14_400.0, 2.0);
        assert!(14_400.0 * s <= MAX_SIDE);
        let s = fit_scale(5000.0, 5000.0, 2.0);
        assert!(5000.0 * s * 5000.0 * s <= MAX_PIXELS * 1.001);
    }

    /// A one-page PDF with two lines of Helvetica text.
    fn text_pdf() -> Vec<u8> {
        let content = b"BT /F1 12 Tf 72 700 Td (Hello world) Tj 0 -20 Td (Second line) Tj ET";
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_owned(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R \
             /Resources << /Font << /F1 5 0 R >> >> >>"
                .to_owned(),
            format!(
                "<< /Length {} >>\nstream\n{}\nendstream",
                content.len(),
                std::str::from_utf8(content).unwrap()
            ),
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_owned(),
        ];
        let mut out = b"%PDF-1.4\n".to_vec();
        let mut offsets = Vec::new();
        for (i, body) in objects.iter().enumerate() {
            offsets.push(out.len());
            out.extend(format!("{} 0 obj\n{body}\nendobj\n", i + 1).as_bytes());
        }
        let xref = out.len();
        out.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes());
        for offset in offsets {
            out.extend(format!("{offset:010} 00000 n \n").as_bytes());
        }
        out.extend(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
                objects.len() + 1
            )
            .as_bytes(),
        );
        out
    }

    #[test]
    fn reads_the_text_of_a_page() {
        let doc = Document::open(text_pdf()).unwrap();
        let lines = doc.text(0);
        let texts: Vec<&str> = lines.iter().map(|l| l.text.as_str()).collect();
        assert_eq!(texts, ["Hello world", "Second line"]);
        let first = &lines[0];
        // 72 pt from the left, the baseline 92 pt from the top.
        assert!((first.left() - 72.0).abs() < 0.5, "{}", first.left());
        assert!(first.top < 92.0 && first.bottom > 92.0);
        assert!(first.right() > 120.0 && first.right() < 160.0);
        assert!(lines[1].top > first.top);
        // Every character has a place, left to right.
        assert_eq!(first.chars.len(), first.text.chars().count());
        assert!(first.chars.windows(2).all(|w| w[0].1 <= w[1].1));
        assert!(doc.text(5).is_empty());
        // No text at all.
        assert!(Document::open(square_pdf()).unwrap().text(0).is_empty());
    }

    #[test]
    fn marks_are_added_to_a_copy() {
        use crate::markup::{Kind, Mark, Quad, Shape};
        let original = text_pdf();
        let doc = Document::open(original.clone()).unwrap();
        assert_eq!(doc.can_mark(), Ok(()));
        let line = &doc.text(0)[0];
        let quad = Quad {
            left: line.left(),
            top: line.top,
            right: line.right(),
            bottom: line.bottom,
        };
        let marks = [
            Mark {
                page: 0,
                kind: Kind::Highlight,
                color: [1.0, 0.9, 0.0],
                shape: Shape::Text {
                    quads: vec![quad],
                    text: "Hello wörld".into(),
                },
            },
            Mark {
                page: 0,
                kind: Kind::Ink,
                color: [1.0, 0.0, 0.0],
                shape: Shape::Ink(vec![(300.0, 300.0), (400.0, 350.0)]),
            },
        ];
        let copy = doc.with_marks(&marks).unwrap();
        // An incremental update: the original, then the changes.
        assert!(copy.starts_with(&original));
        assert!(copy.len() > original.len());
        let saved = lopdf::Document::load_mem(&copy).unwrap();
        let page = saved.get_pages()[&1];
        let annots = saved
            .get_dictionary(page)
            .unwrap()
            .get(b"Annots")
            .unwrap()
            .as_array()
            .unwrap()
            .clone();
        let kinds: Vec<Vec<u8>> = annots
            .iter()
            .map(|a| {
                let dict = saved.get_dictionary(a.as_reference().unwrap()).unwrap();
                assert!(dict.has(b"AP"));
                dict.get(b"Subtype").unwrap().as_name().unwrap().to_vec()
            })
            .collect();
        assert_eq!(kinds, [b"Highlight".to_vec(), b"Ink".to_vec()]);
        // The highlight sits on the text in the page's own coordinates:
        // the baseline at 700 pt from the bottom.
        let highlight = saved
            .get_dictionary(annots[0].as_reference().unwrap())
            .unwrap();
        let rect: Vec<f32> = highlight
            .get(b"Rect")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_float().unwrap())
            .collect();
        assert!(rect[1] < 700.0 && rect[3] > 700.0, "{rect:?}");
        assert!((rect[0] - 72.0).abs() < 1.0, "{rect:?}");
        // hayro draws the new marks: yellow under the text, red for the pen.
        let drawn = Document::open(copy).unwrap();
        let page = drawn.render(0, 1.0).unwrap();
        let pixel = |x: u32, y: u32| page.get_pixel(x, y).0;
        let yellow = pixel(74, 84);
        assert!(
            yellow[0] > 200 && yellow[1] > 180 && yellow[2] < 80,
            "{yellow:?}"
        );
        let red = pixel(350, 325);
        assert!(red[0] > 200 && red[1] < 80, "{red:?}");
    }

    #[test]
    fn protected_pdfs_cannot_be_marked() {
        let mut doc = lopdf::Document::load_mem(&text_pdf()).unwrap();
        let mut perms = lopdf::Dictionary::new();
        perms.set("DocMDP", lopdf::Dictionary::new());
        doc.catalog_mut().unwrap().set("Perms", perms);
        let mut certified = Vec::new();
        doc.save_to(&mut certified).unwrap();
        let doc = Document::open(certified).unwrap();
        assert_eq!(doc.can_mark(), Err(SaveError::Protected));
        assert_eq!(doc.with_marks(&[]), Err(SaveError::Protected));
    }

    #[test]
    fn garbage_is_invalid() {
        assert_eq!(
            Document::open(b"not a pdf".to_vec()).err(),
            Some(Error::Invalid)
        );
    }
}
