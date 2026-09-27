// SPDX-License-Identifier: GPL-3.0-or-later

//! What an attachment card shows of a spreadsheet, text file, document or
//! slides: the top-left cells or the first lines (of the first slide),
//! which the app draws small, like the page thumbnail of a PDF.

use crate::Kind;
use crate::{document, sheet, slides, text};

/// Rows and columns of a sheet a card shows.
const ROWS: usize = 8;
const COLUMNS: usize = 5;
/// Lines of a text or document a card shows...
const LINES: usize = 10;
/// ...and of a slide.
const SLIDE_LINES: usize = 6;
/// Characters of a cell or line worth keeping at card size.
const CHARS: usize = 80;
/// Bytes of a text file read for its first lines.
const TEXT_BYTES: usize = 16 * 1024;

/// A glance at a file, for its card.
#[derive(Debug, Clone, PartialEq)]
pub enum Glance {
    /// The top-left cells of the first sheet; rows have the same length.
    Cells(Vec<Vec<String>>),
    /// The first lines of a text file or document.
    Lines(Vec<Line>),
    /// The lines of the first slide with text, drawn centered like a slide.
    Slide(Vec<Line>),
}

/// A line of a [`Glance::Lines`].
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub text: String,
    /// A title or heading, drawn bold.
    pub heading: bool,
}

/// A glance at `bytes`, a file of `kind` named `name` (`tabs` for a
/// tab-separated file); none for other kinds, or an empty or broken file.
pub fn glance(kind: Kind, bytes: Vec<u8>, name: &str, tabs: bool) -> Option<Glance> {
    let glance = match kind {
        Kind::Sheet { csv: true } => cells(&sheet::csv(&bytes, name, tabs)),
        Kind::Sheet { csv: false } => cells(&sheet::open(bytes).ok()?),
        Kind::Text => {
            let (text, _) = text::decode(&bytes, TEXT_BYTES);
            let lines: Vec<Line> = text
                .lines()
                .take(LINES)
                .map(|line| Line {
                    text: short(&line.replace('\t', "    ")),
                    heading: false,
                })
                .collect();
            (!lines.is_empty()).then_some(Glance::Lines(lines))
        }
        Kind::Document => paragraphs(&document::open(bytes).ok()?),
        Kind::Slides => match paragraphs(&slides::open(bytes).ok()?)? {
            Glance::Lines(mut lines) => {
                lines.truncate(SLIDE_LINES);
                Some(Glance::Slide(lines))
            }
            other => Some(other),
        },
        Kind::Pdf | Kind::Picture(_) | Kind::Other => None,
    }?;
    let empty = match &glance {
        Glance::Cells(rows) => rows.iter().flatten().all(|c| c.trim().is_empty()),
        Glance::Lines(lines) | Glance::Slide(lines) => {
            lines.iter().all(|l| l.text.trim().is_empty())
        }
    };
    (!empty).then_some(glance)
}

/// The top-left cells of the first sheet that has any.
fn cells(book: &sheet::Workbook) -> Option<Glance> {
    let sheet = book.sheets.iter().find(|s| !s.rows.is_empty())?;
    let width = sheet
        .rows
        .iter()
        .take(ROWS)
        .map(Vec::len)
        .max()
        .unwrap_or(0)
        .clamp(1, COLUMNS);
    let rows = sheet
        .rows
        .iter()
        .take(ROWS)
        .map(|row| {
            (0..width)
                .map(|ix| row.get(ix).map_or_else(String::new, |c| short(c)))
                .collect()
        })
        .collect();
    Some(Glance::Cells(rows))
}

/// The first lines of a document: its paragraphs, and the cells of its
/// tables a row to a line.
fn paragraphs(doc: &document::Document) -> Option<Glance> {
    let mut lines = Vec::new();
    for block in &doc.blocks {
        match block {
            document::Block::Paragraph(p) => lines.push(Line {
                text: short(&with_marker(p)),
                heading: !matches!(p.style, document::Style::Normal | document::Style::Subtitle),
            }),
            document::Block::Table(rows) => lines.extend(rows.iter().map(|row| {
                Line {
                    text: short(
                        &row.iter()
                            .map(|cell| cell.iter().map(|p| p.text()).collect::<Vec<_>>().join(" "))
                            .collect::<Vec<_>>()
                            .join("   "),
                    ),
                    heading: false,
                }
            })),
            // A card shows the first slide with text.
            document::Block::Slide(_) if lines.iter().any(|l| !l.text.trim().is_empty()) => break,
            document::Block::Slide(_) => lines.clear(),
        }
        if lines.len() >= LINES {
            break;
        }
    }
    // Leading blank paragraphs waste the little room there is.
    let first = lines.iter().position(|l| !l.text.trim().is_empty())?;
    lines.drain(..first);
    lines.truncate(LINES);
    Some(Glance::Lines(lines))
}

fn with_marker(p: &document::Paragraph) -> String {
    match &p.list {
        Some((level, marker)) => {
            format!("{}{marker} {}", "  ".repeat(*level as usize), p.text())
        }
        None => p.text(),
    }
}

/// `text` cut to what a card can show.
fn short(text: &str) -> String {
    text.chars().take(CHARS).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{docx, xlsx};

    #[test]
    fn sheets_show_their_top_left_cells() {
        let glance = glance(Kind::Sheet { csv: false }, xlsx(), "a.xlsx", false);
        let Some(Glance::Cells(rows)) = glance else {
            panic!("{glance:?}");
        };
        assert_eq!(rows[0], ["Item", "Cost"]);
        assert_eq!(rows[2], ["Total", "12.5"]);
    }

    #[test]
    fn csv_rows_are_padded_to_one_width() {
        let bytes = b"a,b,c\n1\n".to_vec();
        let glance = glance(Kind::Sheet { csv: true }, bytes, "x.csv", false);
        assert_eq!(
            glance,
            Some(Glance::Cells(vec![
                vec!["a".into(), "b".into(), "c".into()],
                vec!["1".into(), String::new(), String::new()],
            ]))
        );
    }

    #[test]
    fn text_shows_its_first_lines() {
        let text: String = (1..=20).map(|n| format!("line {n}\n")).collect();
        let Some(Glance::Lines(lines)) = glance(Kind::Text, text.into_bytes(), "a.txt", false)
        else {
            panic!();
        };
        assert_eq!(lines.len(), LINES);
        assert_eq!(lines[0].text, "line 1");
    }

    #[test]
    fn documents_show_headings_bold_and_list_markers() {
        let Some(Glance::Lines(lines)) = glance(Kind::Document, docx(), "a.docx", false) else {
            panic!();
        };
        assert_eq!(lines[0].text, "Quarterly report");
        assert!(lines[0].heading);
        assert!(!lines[2].heading);
        assert!(lines.iter().any(|l| l.text == "1. One"), "{lines:?}");
    }

    #[test]
    fn slides_show_their_first_slide() {
        let glance = glance(Kind::Slides, crate::tests::pptx(), "a.pptx", false);
        let Some(Glance::Slide(lines)) = glance else {
            panic!("{glance:?}");
        };
        assert_eq!(lines[0].text, "Roadmap");
        assert!(lines[0].heading);
        assert_eq!(lines[1].text, "Q1   42");
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn old_word_documents_have_a_glance() {
        let Some(Glance::Lines(lines)) =
            glance(Kind::Document, crate::tests::doc(), "a.doc", false)
        else {
            panic!();
        };
        assert_eq!(lines[0].text, "Report");
        assert!(lines[0].heading);
    }

    #[test]
    fn empty_and_other_files_have_no_glance() {
        assert_eq!(glance(Kind::Text, b"\n \n".to_vec(), "a.txt", false), None);
        assert_eq!(glance(Kind::Other, b"x".to_vec(), "a.bin", false), None);
        assert_eq!(
            glance(
                Kind::Sheet { csv: false },
                b"junk".to_vec(),
                "a.xlsx",
                false
            ),
            None
        );
    }
}
