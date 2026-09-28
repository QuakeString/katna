// SPDX-License-Identifier: GPL-3.0-or-later

//! A conversation laid out as a PDF for printing: its subject, then each
//! message's sender, date, recipients, body and attachment names. The mail
//! app hands the PDF to the desktop's print dialog.
//!
//! An HTML message prints as the reading pane shows it (`flow`): its
//! tables, boxes, colors, borders, lists, quotes and pictures, scaled so
//! the mail's 16 px text is 10.5 pt. [`PrintOptions`] can leave out its
//! background colors (the formatting stays), or print the text alone as
//! "simple text".
//!
//! Everything is placed on one tall strip first and then cut into pages,
//! between lines of text and never through one, or through a picture or a
//! short table row. Text is shaped with the same shaper that measures it,
//! in the desktop's UI font with its bold, italic and monospace faces.

mod flow;

use std::collections::HashMap;
use std::sync::Arc;

use krilla::color::rgb;
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule};
use krilla::surface::Surface;
use krilla::text::{Font, TextDirection};
use krilla::{Data, Document as Pdf};

use crate::html::{Color, Document, ImageKind};

/// The room left around the text, in points (19 mm).
const MARGIN: f32 = 54.0;

/// One message of the conversation, as it is printed.
#[derive(Debug, Clone, Default)]
pub struct PrintMessage {
    /// "Ada Lovelace <ada@example.org>".
    pub from: String,
    /// The date as the reader shows it.
    pub date: String,
    /// "To: …", and "Cc: …" when there is one; empty to leave out.
    pub to: String,
    pub cc: String,
    /// The text, HTML already converted to text: printed as simple text,
    /// and when there is no `document`.
    pub body: String,
    /// The HTML body laid out, as the reader draws it.
    pub document: Option<Document>,
    /// Remote pictures of `document` the reader has fetched (the user
    /// allowed them), by URL.
    pub images: RemoteImages,
    /// The attachments' file names.
    pub attachments: Vec<String>,
    /// The line above them: "2 attachments".
    pub attachments_label: String,
}

/// Pictures from the web, by URL: their kind and bytes.
pub type RemoteImages = HashMap<String, (ImageKind, Arc<[u8]>)>;

/// How a conversation prints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintOptions {
    /// Only the text of each message, without its formatting.
    pub simple: bool,
    /// The background colors of HTML mail, which printers often leave out
    /// to save ink. Without them light text is darkened to stay readable.
    pub backgrounds: bool,
}

impl Default for PrintOptions {
    /// As the reader shows it.
    fn default() -> Self {
        Self {
            simple: false,
            backgrounds: true,
        }
    }
}

/// The paper's size in points (1/72 inch), as the print dialog chose it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Paper {
    pub width: f32,
    pub height: f32,
}

impl Paper {
    pub const A4: Self = Self {
        width: 595.28,
        height: 841.89,
    };

    /// US Letter, 8.5 × 11 inches.
    pub const LETTER: Self = Self {
        width: 612.0,
        height: 792.0,
    };

    /// A size given in millimetres. `None` for one too small to hold a
    /// line of text.
    pub fn from_mm(width: f64, height: f64) -> Option<Self> {
        let point = |mm: f64| (mm * 72.0 / 25.4) as f32;
        let paper = Self {
            width: point(width),
            height: point(height),
        };
        (paper.width >= 4.0 * MARGIN && paper.height >= 4.0 * MARGIN).then_some(paper)
    }
}

/// A font file (or one face of a collection).
#[derive(Clone)]
pub struct PrintFont {
    pub data: Vec<u8>,
    pub index: u32,
}

/// The faces a conversation prints in. Those missing fall back to
/// `regular` (bold italic to bold, then italic).
#[derive(Clone)]
pub struct PrintFonts {
    pub regular: PrintFont,
    pub bold: Option<PrintFont>,
    pub italic: Option<PrintFont>,
    pub bold_italic: Option<PrintFont>,
    pub mono: Option<PrintFont>,
}

/// Why no PDF could be made.
#[derive(Debug)]
pub enum PrintError {
    /// The font could not be read.
    Font,
    /// krilla refused the document.
    Pdf(String),
}

impl std::fmt::Display for PrintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Font => f.write_str("the font could not be read"),
            Self::Pdf(err) => write!(f, "the PDF could not be written: {err}"),
        }
    }
}

impl std::error::Error for PrintError {}

/// Lays `messages` out under `subject` on `paper` and returns the PDF.
pub fn conversation_pdf(
    subject: &str,
    messages: &[PrintMessage],
    paper: Paper,
    fonts: &PrintFonts,
    options: PrintOptions,
) -> Result<Vec<u8>, PrintError> {
    let faces = Faces::new(fonts)?;
    let width = paper.width - 2.0 * MARGIN;
    let height = paper.height - 2.0 * MARGIN;
    let strip = flow::lay_out(subject, messages, width, height, &faces, options, true);
    match render(&strip, paper, &faces) {
        // A picture krilla could not read after all: print without them.
        Err(_) if strip.pictures => {
            let strip = flow::lay_out(subject, messages, width, height, &faces, options, false);
            render(&strip, paper, &faces)
        }
        done => done,
    }
}

/// A face with what it takes to both measure and draw text.
#[derive(Clone)]
struct Face {
    font: Font,
    data: Arc<Vec<u8>>,
    index: u32,
}

impl Face {
    fn new(font: &PrintFont) -> Result<Self, PrintError> {
        let data = Arc::new(font.data.clone());
        rustybuzz::Face::from_slice(&data, font.index).ok_or(PrintError::Font)?;
        let pdf_font = Font::new(Data::from(data.clone()), font.index).ok_or(PrintError::Font)?;
        Ok(Self {
            font: pdf_font,
            data,
            index: font.index,
        })
    }
}

/// The five faces, by the `REGULAR` … `MONO` indexes.
struct Faces([Face; 5]);

const REGULAR: usize = 0;
const BOLD: usize = 1;
const ITALIC: usize = 2;
const BOLD_ITALIC: usize = 3;
const MONO: usize = 4;

impl Faces {
    fn new(fonts: &PrintFonts) -> Result<Self, PrintError> {
        let regular = Face::new(&fonts.regular)?;
        // A face that does not load is left out, as if there were none.
        let load = |font: &Option<PrintFont>| font.as_ref().and_then(|f| Face::new(f).ok());
        let bold = load(&fonts.bold);
        let italic = load(&fonts.italic);
        let bold_italic = load(&fonts.bold_italic)
            .or_else(|| bold.clone())
            .or_else(|| italic.clone());
        let mono = load(&fonts.mono);
        Ok(Self([
            regular.clone(),
            bold.unwrap_or_else(|| regular.clone()),
            italic.unwrap_or_else(|| regular.clone()),
            bold_italic.unwrap_or_else(|| regular.clone()),
            mono.unwrap_or(regular),
        ]))
    }

    fn get(&self, face: usize) -> &Face {
        &self.0[face.min(MONO)]
    }
}

/// What is drawn, on the strip.
enum Item {
    Text {
        x: f32,
        baseline: f32,
        face: usize,
        size: f32,
        color: Color,
        text: String,
    },
    /// A rectangle, or with `ring` only its outline that wide.
    Shape {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: Color,
        ring: Option<f32>,
    },
    Picture {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        image: krilla::image::Image,
    },
}

/// An item and how far down the strip it reaches.
struct Placed {
    top: f32,
    bottom: f32,
    item: Item,
}

/// The conversation on one strip as wide as the page's text.
#[derive(Default)]
struct Strip {
    items: Vec<Placed>,
    /// Stretches no page may end inside: lines, pictures, short rows.
    keep: Vec<(f32, f32)>,
    height: f32,
    /// Whether it draws any picture.
    pictures: bool,
}

/// Where the page that starts at `top` of the strip ends: as low as
/// `room` allows without cutting what must stay whole. Something taller
/// than a page is cut where the page is full.
fn page_end(keep: &[(f32, f32)], top: f32, room: f32) -> f32 {
    let limit = top + room;
    let near: Vec<(f32, f32)> = keep
        .iter()
        .copied()
        .filter(|&(t, _)| t > top + 0.5 && t < limit)
        .collect();
    let mut cut = limit;
    loop {
        let mut moved = false;
        for &(t, b) in &near {
            if t < cut - 0.01 && b > cut + 0.01 {
                cut = t;
                moved = true;
            }
        }
        if !moved {
            break;
        }
    }
    if cut <= top + 1.0 { limit } else { cut }
}

/// Cuts the strip into pages.
fn render(strip: &Strip, paper: Paper, faces: &Faces) -> Result<Vec<u8>, PrintError> {
    let mut document = Pdf::new();
    let settings = PageSettings::from_wh(paper.width, paper.height)
        .ok_or_else(|| PrintError::Pdf(format!("no paper of {paper:?}")))?;
    let room = paper.height - 2.0 * MARGIN;
    let mut top = 0.0;
    loop {
        let end = page_end(&strip.keep, top, room).min(strip.height.max(top + 1.0));
        let mut page = document.start_page_with(settings.clone());
        let mut surface = page.surface();
        let clip =
            Rect::from_xywh(0.0, MARGIN, paper.width, (end - top).max(1.0)).and_then(|rect| {
                let mut path = PathBuilder::new();
                path.push_rect(rect);
                path.finish()
            });
        if let Some(clip) = &clip {
            surface.push_clip_path(clip, &FillRule::NonZero);
        }
        surface.push_transform(&Transform::from_translate(MARGIN, MARGIN - top));
        for placed in &strip.items {
            if placed.bottom > top + 0.01 && placed.top < end - 0.01 {
                draw(&mut surface, &placed.item, faces);
            }
        }
        surface.pop();
        if clip.is_some() {
            surface.pop();
        }
        surface.finish();
        page.finish();
        if end >= strip.height - 0.01 {
            break;
        }
        top = end;
    }
    document
        .finish()
        .map_err(|err| PrintError::Pdf(format!("{err:?}")))
}

fn fill(color: Color, rule: FillRule) -> Fill {
    let [r, g, b, a] = color.to_be_bytes();
    Fill {
        paint: rgb::Color::new(r, g, b).into(),
        opacity: NormalizedF32::new(f32::from(a) / 255.0).unwrap_or(NormalizedF32::ONE),
        rule,
    }
}

/// Adds a rectangle with rounded corners to `path`.
fn rounded(path: &mut PathBuilder, x: f32, y: f32, w: f32, h: f32, radius: f32) {
    let r = radius.min(w / 2.0).min(h / 2.0);
    if r <= 0.01 {
        if let Some(rect) = Rect::from_xywh(x, y, w, h) {
            path.push_rect(rect);
        }
        return;
    }
    // How far a corner's control points sit from its ends.
    let k = r * (1.0 - 0.5523);
    let (right, bottom) = (x + w, y + h);
    path.move_to(x + r, y);
    path.line_to(right - r, y);
    path.cubic_to(right - k, y, right, y + k, right, y + r);
    path.line_to(right, bottom - r);
    path.cubic_to(right, bottom - k, right - k, bottom, right - r, bottom);
    path.line_to(x + r, bottom);
    path.cubic_to(x + k, bottom, x, bottom - k, x, bottom - r);
    path.line_to(x, y + r);
    path.cubic_to(x, y + k, x + k, y, x + r, y);
    path.close();
}

fn draw(surface: &mut Surface<'_>, item: &Item, faces: &Faces) {
    match item {
        Item::Text {
            x,
            baseline,
            face,
            size,
            color,
            text,
        } => {
            surface.set_fill(Some(fill(*color, FillRule::NonZero)));
            surface.draw_text(
                Point::from_xy(*x, *baseline),
                faces.get(*face).font.clone(),
                *size,
                text,
                false,
                TextDirection::Auto,
            );
        }
        Item::Shape {
            x,
            y,
            w,
            h,
            radius,
            color,
            ring,
        } => {
            if *w <= 0.0 || *h <= 0.0 {
                return;
            }
            let mut path = PathBuilder::new();
            rounded(&mut path, *x, *y, *w, *h, *radius);
            if let Some(ring) = ring
                && *w > 2.0 * ring
                && *h > 2.0 * ring
            {
                rounded(
                    &mut path,
                    x + ring,
                    y + ring,
                    w - 2.0 * ring,
                    h - 2.0 * ring,
                    (radius - ring).max(0.0),
                );
            }
            if let Some(path) = path.finish() {
                surface.set_fill(Some(fill(*color, FillRule::EvenOdd)));
                surface.draw_path(&path);
            }
        }
        Item::Picture { x, y, w, h, image } => {
            let Some(size) = Size::from_wh(*w, *h) else {
                return;
            };
            surface.push_transform(&Transform::from_translate(*x, *y));
            surface.draw_image(image.clone(), size);
            surface.pop();
        }
    }
}

#[cfg(test)]
mod tests;
