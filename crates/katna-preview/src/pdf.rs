// SPDX-License-Identifier: GPL-3.0-or-later

//! PDF pages as bitmaps, drawn by hayro (pure Rust, on the CPU).

use std::sync::Arc;

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::page::Rotation;
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
    pdf: Arc<Pdf>,
    /// Each page's size in points (1/72 inch), as the file draws it.
    base: Vec<(f32, f32)>,
    /// Each page's size in points, [`Document::turn`] applied.
    sizes: Vec<(f32, f32)>,
    /// Quarter turns clockwise the pages are shown at, 0 to 3.
    turn: u8,
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
        Ok(Self {
            pdf: Arc::new(pdf),
            base: sizes.clone(),
            sizes,
            turn: 0,
        })
    }

    /// Quarter turns clockwise the pages are shown at, 0 to 3.
    pub fn turn(&self) -> u8 {
        self.turn
    }

    /// The same PDF shown turned `turn` quarter turns clockwise (from how
    /// the file draws it). Sizes, drawing, text and saved marks all follow.
    pub fn turned(&self, turn: u8) -> Self {
        let turn = turn % 4;
        let sizes = self
            .base
            .iter()
            .map(|&(w, h)| if turn % 2 == 1 { (h, w) } else { (w, h) })
            .collect();
        Self {
            pdf: self.pdf.clone(),
            base: self.base.clone(),
            sizes,
            turn,
        }
    }

    /// From points as the file draws page `page` to points as shown.
    fn turning(&self, page: usize) -> Affine {
        let (w, h) = self.base.get(page).copied().unwrap_or((612.0, 792.0));
        let (w, h) = (f64::from(w), f64::from(h));
        match self.turn {
            1 => Affine::new([0.0, 1.0, -1.0, 0.0, h, 0.0]),
            2 => Affine::new([-1.0, 0.0, 0.0, -1.0, w, h]),
            3 => Affine::new([0.0, -1.0, 1.0, 0.0, 0.0, w]),
            _ => Affine::IDENTITY,
        }
    }

    /// Page `page`'s transform from its own coordinates to points as shown.
    fn shown(&self, page: &hayro::hayro_syntax::page::Page<'_>, ix: usize) -> Affine {
        let [a, b, c, d, e, f] = page.initial_transform(true).as_coeffs();
        self.turning(ix) * Affine::new([a, b, c, d, e, f].map(f64::from))
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
        let (w, h) = self.base.get(page).copied()?;
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
        let image = RgbaImage::from_raw(width, height, pixmap.data_as_u8_slice().to_vec())?;
        Some(turn_image(image, self.turn))
    }

    /// Page `page` drawn as [`Document::render`] does, then made dark for
    /// reading in dark mode ([`crate::dark`]): photos dimmed, the rest
    /// (scans too) flipped.
    pub fn render_dark(&self, page: usize, scale: f32) -> Option<RgbaImage> {
        let mut image = self.render(page, scale)?;
        let pages = self.pdf.pages();
        let pdf_page = pages.get(page)?;
        let (w, h) = self.page_size(page);
        // The scale `render` used, from the page's width as shown.
        let px = f64::from(image.width()) / f64::from(w.max(1.0));
        let (iw, ih) = (f64::from(image.width()), f64::from(image.height()));
        let pictures: Vec<_> =
            crate::pdf_text::pictures(pdf_page, self.shown(pdf_page, page), (w, h))
                .into_iter()
                .map(|r| {
                    // Only whole pixels of the picture: edge pixels, part
                    // paper, flip with the page.
                    let at = |v: f64, max: f64| (v * px).clamp(0.0, max);
                    let (x0, y0) = (at(r.x0, iw).ceil(), at(r.y0, ih).ceil());
                    let (x1, y1) = (at(r.x1, iw).floor(), at(r.y1, ih).floor());
                    (x0 as u32, y0 as u32, x1 as u32, y1 as u32)
                })
                .collect();
        crate::dark::darken(&mut image, &pictures);
        Some(image)
    }
}

impl Document {
    /// The text of page `page`, line by line, with where each character
    /// is drawn, in points from the page's top left.
    pub fn text(&self, page: usize) -> Vec<TextLine> {
        let pages = self.pdf.pages();
        let Some(pdf_page) = pages.get(page) else {
            return Vec::new();
        };
        crate::pdf_text::lines(pdf_page, self.shown(pdf_page, page), self.page_size(page))
    }

    /// Whether marks can be saved into this PDF: not when it is encrypted
    /// or certified against changes. Reads the whole file again.
    pub fn can_mark(&self) -> Result<(), SaveError> {
        crate::pdf_marks::check(self.pdf.data().as_ref())
    }

    /// A copy of the file with `marks` added as annotations, the original
    /// bytes unchanged at its start. Turned pages are saved turned, so
    /// the copy opens the way it was marked.
    pub fn with_marks(&self, marks: &[crate::markup::Mark]) -> Result<Vec<u8>, SaveError> {
        let pages = self.pdf.pages();
        // From points as shown back to the page's own.
        let transforms: Vec<_> = pages
            .iter()
            .enumerate()
            .map(|(ix, page)| self.shown(page, ix).inverse().as_coeffs())
            .collect();
        let turns: Vec<i64> = if self.turn == 0 {
            Vec::new()
        } else {
            pages
                .iter()
                .map(|page| {
                    let own = match page.rotation() {
                        Rotation::None => 0,
                        Rotation::Horizontal => 90,
                        Rotation::Flipped => 180,
                        Rotation::FlippedHorizontal => 270,
                    };
                    (own + 90 * i64::from(self.turn)) % 360
                })
                .collect()
        };
        crate::pdf_marks::write(self.pdf.data().as_ref(), &transforms, &turns, marks)
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

/// `image` turned `turn` quarter turns clockwise.
pub fn turn_image(image: RgbaImage, turn: u8) -> RgbaImage {
    match turn % 4 {
        1 => image::imageops::rotate90(&image),
        2 => image::imageops::rotate180(&image),
        3 => image::imageops::rotate270(&image),
        _ => image,
    }
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

    #[test]
    fn dark_pages_dim_photos() {
        // A 2 × 1 dark green picture drawn 200 × 100 pt at the top left;
        // the rest of the page is white paper.
        let pdf = pdf_with(b"q 200 0 0 100 0 692 cm BI /W 2 /H 1 /CS /RGB /BPC 8 ID (x<(x< EI Q");
        let doc = Document::open(pdf).unwrap();
        let page = doc.render_dark(0, 1.0).unwrap();
        // The photo is dimmed, not flipped.
        let photo = page.get_pixel(100, 50).0;
        assert!(photo[1] > photo[0] + 40 && photo[1] < 120, "{photo:?}");
        // The paper is a dark grey.
        assert_eq!(page.get_pixel(400, 500).0, [26, 26, 26, 255]);
    }

    /// A one-page PDF (US Letter) with a black 100 × 100 pt square at the
    /// bottom left, cross-reference offsets computed.
    pub(crate) fn square_pdf() -> Vec<u8> {
        pdf_with(b"0 0 0 rg 0 0 100 100 re f")
    }

    /// A one-page PDF (US Letter) drawing `content`.
    fn pdf_with(content: &[u8]) -> Vec<u8> {
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
    fn turned_pages_draw_and_save_turned() {
        let doc = Document::open(square_pdf()).unwrap().turned(1);
        assert_eq!(doc.page_size(0), (792.0, 612.0));
        let page = doc.render(0, 0.5).unwrap();
        assert_eq!(page.dimensions(), (396, 306));
        // A clockwise turn brings the bottom left to the top left.
        assert_eq!(page.get_pixel(10, 10).0, [0, 0, 0, 255]);
        assert_eq!(page.get_pixel(10, 300).0, [255, 255, 255, 255]);
        let stroke = crate::markup::Mark {
            page: 0,
            kind: crate::markup::Kind::Ink,
            color: [1.0, 0.0, 0.0],
            shape: crate::markup::Shape::Ink(vec![(600.0, 400.0), (700.0, 400.0)]),
        };
        let saved = Document::open(doc.with_marks(&[stroke]).unwrap()).unwrap();
        // The copy opens turned, with the stroke where it was drawn.
        assert_eq!(saved.page_size(0), (792.0, 612.0));
        let page = saved.render(0, 0.5).unwrap();
        assert_eq!(page.get_pixel(10, 10).0, [0, 0, 0, 255]);
        let [r, g, b, _] = page.get_pixel(325, 200).0;
        assert!(r > 200 && g < 200 && b < 200, "{r} {g} {b}");
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
            Mark {
                page: 0,
                kind: Kind::Note,
                color: [0.2, 0.8, 0.2],
                shape: Shape::Note {
                    at: (500.0, 100.0),
                    text: "Check this".into(),
                },
            },
            Mark {
                page: 0,
                kind: Kind::FreeText,
                color: [0.0, 0.0, 1.0],
                shape: Shape::Box {
                    at: (100.0, 500.0),
                    width: 200.0,
                    size: 24.0,
                    text: "MMMM MMMM".into(),
                },
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
        assert_eq!(
            kinds,
            [
                b"Highlight".to_vec(),
                b"Ink".to_vec(),
                b"Text".to_vec(),
                b"FreeText".to_vec()
            ]
        );
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
        // The note's green square, and the text box's blue letters.
        let green = pixel(503, 108);
        assert!(green[1] > 180 && green[0] < 100, "{green:?}");
        let blue = (100..300)
            .flat_map(|x| (500..530).map(move |y| (x, y)))
            .filter(|&(x, y)| {
                let p = pixel(x, y);
                p[2] > 200 && p[0] < 80
            })
            .count();
        assert!(blue > 50, "{blue}");
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
