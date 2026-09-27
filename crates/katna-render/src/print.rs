// SPDX-License-Identifier: GPL-3.0-or-later

//! A conversation laid out as a PDF for printing: its subject, then each
//! message's sender, date, recipients, text and attachment names, on A4
//! pages. The mail app hands the PDF to the desktop's print dialog.
//!
//! Text is drawn in one font (the desktop's UI font), shaped with the same
//! shaper that measures it, and wrapped at spaces; a word wider than the
//! page is broken where it runs out.

use krilla::color::rgb;
use krilla::geom::{PathBuilder, Point, Rect};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule};
use krilla::surface::Surface;
use krilla::text::{Font, TextDirection};
use krilla::{Data, Document};

/// The room left around the text, in points (19 mm).
const MARGIN: f32 = 54.0;

const SUBJECT_SIZE: f32 = 16.0;
const TEXT_SIZE: f32 = 10.5;
const SMALL_SIZE: f32 = 9.0;
/// Line height as a share of the font size.
const LEADING: f32 = 1.4;

const TEXT: (u8, u8, u8) = (0x1f, 0x1f, 0x1f);
const DIM: (u8, u8, u8) = (0x5f, 0x63, 0x68);
const RULE: (u8, u8, u8) = (0xda, 0xdc, 0xe0);

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
    /// The text, HTML already converted to text.
    pub body: String,
    /// The attachments' file names.
    pub attachments: Vec<String>,
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
/// `bold` is the bold face of `regular`, when there is one.
pub fn conversation_pdf(
    subject: &str,
    messages: &[PrintMessage],
    paper: Paper,
    regular: &PrintFont,
    bold: Option<&PrintFont>,
) -> Result<Vec<u8>, PrintError> {
    let regular = Face::new(regular)?;
    let bold = match bold {
        Some(bold) => Face::new(bold)?,
        None => regular.clone(),
    };
    let mut lines = Vec::new();
    let width = paper.width - 2.0 * MARGIN;
    wrap(&mut lines, &bold, SUBJECT_SIZE, TEXT, subject, width);
    lines.push(Line::gap(6.0));
    for (ix, message) in messages.iter().enumerate() {
        lines.push(Line::rule());
        wrap(&mut lines, &bold, TEXT_SIZE, TEXT, &message.from, width);
        if !message.date.is_empty() {
            wrap(&mut lines, &regular, SMALL_SIZE, DIM, &message.date, width);
        }
        for recipients in [&message.to, &message.cc] {
            if !recipients.is_empty() {
                wrap(&mut lines, &regular, SMALL_SIZE, DIM, recipients, width);
            }
        }
        lines.push(Line::gap(8.0));
        for paragraph in message.body.lines() {
            wrap(&mut lines, &regular, TEXT_SIZE, TEXT, paragraph, width);
        }
        if !message.attachments.is_empty() {
            lines.push(Line::gap(8.0));
            let label = match message.attachments.len() {
                1 => "1 attachment".to_owned(),
                n => format!("{n} attachments"),
            };
            wrap(&mut lines, &bold, SMALL_SIZE, DIM, &label, width);
            for name in &message.attachments {
                wrap(&mut lines, &regular, SMALL_SIZE, DIM, name, width);
            }
        }
        if ix + 1 < messages.len() {
            lines.push(Line::gap(10.0));
        }
    }
    render(&lines, paper)
}

/// A face with what it takes to both measure and draw text.
#[derive(Clone)]
struct Face {
    font: Font,
    data: std::sync::Arc<Vec<u8>>,
    index: u32,
}

impl Face {
    fn new(font: &PrintFont) -> Result<Self, PrintError> {
        let data = std::sync::Arc::new(font.data.clone());
        rustybuzz::Face::from_slice(&data, font.index).ok_or(PrintError::Font)?;
        let pdf_font = Font::new(Data::from(data.clone()), font.index).ok_or(PrintError::Font)?;
        Ok(Self {
            font: pdf_font,
            data,
            index: font.index,
        })
    }

    /// How wide `text` is at `size`, shaped as krilla shapes it.
    fn width(&self, text: &str, size: f32) -> f32 {
        let Some(face) = rustybuzz::Face::from_slice(&self.data, self.index) else {
            return 0.0;
        };
        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(text);
        buffer.guess_segment_properties();
        let output = rustybuzz::shape(&face, &[], buffer);
        let units: i32 = output.glyph_positions().iter().map(|p| p.x_advance).sum();
        units as f32 / f32::from(face.units_per_em() as u16) * size
    }
}

/// One line of the laid-out document.
enum Line {
    Text {
        face: Face,
        size: f32,
        color: (u8, u8, u8),
        text: String,
    },
    /// Empty room.
    Gap(f32),
    /// A thin line across the page, before each message.
    Rule,
}

impl Line {
    fn gap(height: f32) -> Self {
        Self::Gap(height)
    }

    fn rule() -> Self {
        Self::Rule
    }

    fn height(&self) -> f32 {
        match self {
            Self::Text { size, .. } => size * LEADING,
            Self::Gap(height) => *height,
            Self::Rule => 12.0,
        }
    }
}

/// Wraps `text` to `width` and adds its lines. An empty paragraph adds a
/// blank line.
fn wrap(
    lines: &mut Vec<Line>,
    face: &Face,
    size: f32,
    color: (u8, u8, u8),
    text: &str,
    width: f32,
) {
    let text: String = text
        .chars()
        .map(|c| if c == '\t' { ' ' } else { c })
        .filter(|c| !c.is_control())
        .collect();
    let text = text.trim_end();
    let mut push = |line: String| {
        lines.push(Line::Text {
            face: face.clone(),
            size,
            color,
            text: line,
        });
    };
    if text.is_empty() {
        push(String::new());
        return;
    }
    let mut line = String::new();
    for word in text.split(' ') {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if face.width(&candidate, size) <= width {
            line = candidate;
            continue;
        }
        if !line.is_empty() {
            push(std::mem::take(&mut line));
        }
        // A word wider than the page: break it where it runs out.
        let mut rest = word;
        while face.width(rest, size) > width {
            let mut cut = rest.len();
            while cut > 0 && (!rest.is_char_boundary(cut) || face.width(&rest[..cut], size) > width)
            {
                cut -= 1;
            }
            if cut == 0 {
                cut = rest.chars().next().map_or(rest.len(), char::len_utf8);
            }
            push(rest[..cut].to_owned());
            rest = &rest[cut..];
        }
        line = rest.to_owned();
    }
    push(line);
}

fn fill(color: (u8, u8, u8)) -> Fill {
    Fill {
        paint: rgb::Color::new(color.0, color.1, color.2).into(),
        opacity: NormalizedF32::ONE,
        rule: FillRule::default(),
    }
}

/// Puts the lines on pages, starting a new page when one is full.
fn render(lines: &[Line], paper: Paper) -> Result<Vec<u8>, PrintError> {
    let mut document = Document::new();
    let settings = PageSettings::from_wh(paper.width, paper.height)
        .ok_or_else(|| PrintError::Pdf(format!("no paper of {paper:?}")))?;
    let mut rest = lines;
    loop {
        let mut page = document.start_page_with(settings.clone());
        let mut surface = page.surface();
        let mut y = MARGIN;
        let mut drawn = 0;
        for line in rest {
            let height = line.height();
            if y + height > paper.height - MARGIN && drawn > 0 {
                break;
            }
            draw(&mut surface, line, y, paper);
            y += height;
            drawn += 1;
        }
        surface.finish();
        page.finish();
        rest = &rest[drawn..];
        if rest.is_empty() {
            break;
        }
    }
    document
        .finish()
        .map_err(|err| PrintError::Pdf(format!("{err:?}")))
}

fn draw(surface: &mut Surface<'_>, line: &Line, top: f32, paper: Paper) {
    match line {
        Line::Text {
            face,
            size,
            color,
            text,
        } => {
            if text.is_empty() {
                return;
            }
            // The baseline, about 80% down the line's own height.
            let baseline = top + size * (LEADING - 1.0) / 2.0 + size * 0.8;
            surface.set_fill(Some(fill(*color)));
            surface.draw_text(
                Point::from_xy(MARGIN, baseline),
                face.font.clone(),
                *size,
                text,
                false,
                TextDirection::Auto,
            );
        }
        Line::Gap(_) => {}
        Line::Rule => {
            let Some(rect) = Rect::from_xywh(MARGIN, top + 5.5, paper.width - 2.0 * MARGIN, 0.75)
            else {
                return;
            };
            let mut path = PathBuilder::new();
            path.push_rect(rect);
            if let Some(path) = path.finish() {
                surface.set_fill(Some(fill(RULE)));
                surface.draw_path(&path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A font installed on most Linux systems, if this one has it.
    fn system_font() -> Option<PrintFont> {
        [
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            "/usr/share/fonts/dejavu/DejaVuSans.ttf",
        ]
        .iter()
        .find_map(|path| std::fs::read(path).ok())
        .map(|data| PrintFont { data, index: 0 })
    }

    #[test]
    fn prints_a_conversation_over_pages() {
        let Some(font) = system_font() else {
            eprintln!("no DejaVu Sans here; skipped");
            return;
        };
        let long = "All work and no play makes Jack a dull boy. ".repeat(400);
        let messages = vec![
            PrintMessage {
                from: "Ada Lovelace <ada@example.org>".into(),
                date: "Wed, 16 Sep 2026, 23:48".into(),
                to: "To: bob@example.org".into(),
                body: format!("Dear Bob,\n\n{long}\n\nAda"),
                attachments: vec!["notes.pdf".into()],
                ..PrintMessage::default()
            },
            PrintMessage {
                from: "Bob <bob@example.org>".into(),
                body: "Thanks!\nhttps://example.org/".to_owned() + &"x".repeat(300),
                ..PrintMessage::default()
            },
        ];
        let pdf = conversation_pdf("Notes", &messages, Paper::A4, &font, None).unwrap();
        assert!(pdf.starts_with(b"%PDF-"));
        let pdf = String::from_utf8_lossy(&pdf);
        let count = pdf.split("/Type/Pages/Count ").nth(1).unwrap();
        let pages: usize = count[..count.find('/').unwrap()].parse().unwrap();
        assert!(pages >= 3, "{pages} pages");
    }

    #[test]
    fn paper_from_millimetres() {
        let letter = Paper::from_mm(215.9, 279.4).unwrap();
        assert_eq!(
            (letter.width.round(), letter.height.round()),
            (612.0, 792.0)
        );
        assert!(Paper::from_mm(10.0, 10.0).is_none());
    }

    #[test]
    fn wraps_at_spaces_and_breaks_long_words() {
        let Some(font) = system_font() else {
            return;
        };
        let face = Face::new(&font).unwrap();
        let mut lines = Vec::new();
        wrap(&mut lines, &face, 10.0, TEXT, &"word ".repeat(100), 200.0);
        wrap(&mut lines, &face, 10.0, TEXT, &"x".repeat(200), 200.0);
        assert!(lines.len() > 5);
        for line in &lines {
            if let Line::Text { text, .. } = line {
                assert!(face.width(text, 10.0) <= 200.0, "{text:?}");
            }
        }
    }
}
