// SPDX-License-Identifier: GPL-3.0-or-later

//! A picture of table cells (a spreadsheet's cells pasted "as a picture"):
//! a PNG drawn with the system's fonts, the way a spreadsheet prints them.

use std::fmt::Write as _;
use std::sync::{Arc, OnceLock};

use resvg::{tiny_skia, usvg};

/// How a cell looks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cell {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    /// Text color, `0xRRGGBB`; black when none.
    pub color: Option<u32>,
    /// Background, `0xRRGGBB`; white when none.
    pub fill: Option<u32>,
    pub align: Align,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

const FONT_SIZE: f32 = 14.0;
const LINE: f32 = 19.0;
const PAD_X: f32 = 8.0;
const PAD_Y: f32 = 5.0;
const MIN_COL: f32 = 40.0;
/// Widest a column gets: longer lines are cut.
const MAX_COL: f32 = 600.0;
const BORDER: &str = "#c8c8c8";

fn fonts() -> Arc<usvg::fontdb::Database> {
    static FONTS: OnceLock<Arc<usvg::fontdb::Database>> = OnceLock::new();
    FONTS
        .get_or_init(|| {
            let mut db = usvg::fontdb::Database::new();
            db.load_system_fonts();
            Arc::new(db)
        })
        .clone()
}

fn options() -> usvg::Options<'static> {
    let mut options = usvg::Options {
        fontdb: fonts(),
        ..usvg::Options::default()
    };
    options.font_family = "sans-serif".into();
    options.image_href_resolver.resolve_string = Box::new(|_, _| None);
    options
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn text_attrs(cell: &Cell) -> String {
    let mut attrs = format!(
        "font-family=\"sans-serif\" font-size=\"{FONT_SIZE}\" fill=\"#{:06x}\"",
        cell.color.unwrap_or(0)
    );
    if cell.bold {
        attrs.push_str(" font-weight=\"bold\"");
    }
    if cell.italic {
        attrs.push_str(" font-style=\"italic\"");
    }
    attrs
}

/// The width of each line of each cell, measured with the system fonts.
fn measure(rows: &[Vec<Cell>]) -> Option<Vec<Vec<Vec<f32>>>> {
    let mut svg =
        String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\">");
    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            for (l, line) in cell.text.split('\n').enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                let _ = write!(
                    svg,
                    "<text id=\"t{r}_{c}_{l}\" x=\"0\" y=\"20\" {} xml:space=\"preserve\">{}</text>",
                    text_attrs(cell),
                    escape(line)
                );
            }
        }
    }
    svg.push_str("</svg>");
    let tree = usvg::Tree::from_str(&svg, &options()).ok()?;
    Some(
        rows.iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, cell)| {
                        cell.text
                            .split('\n')
                            .enumerate()
                            .map(|(l, _)| match tree.node_by_id(&format!("t{r}_{c}_{l}")) {
                                Some(usvg::Node::Text(text)) => text.bounding_box().width(),
                                Some(node) => node.abs_bounding_box().width(),
                                None => 0.0,
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect(),
    )
}

/// The cells as an SVG image and its size.
fn svg(rows: &[Vec<Cell>]) -> Option<(String, u32, u32)> {
    let cols = rows.iter().map(Vec::len).max()?;
    if cols == 0 {
        return None;
    }
    let widths = measure(rows)?;
    let mut col_width = vec![MIN_COL; cols];
    for row in &widths {
        for (c, lines) in row.iter().enumerate() {
            let widest = lines.iter().copied().fold(0.0, f32::max) + 2.0 * PAD_X;
            col_width[c] = col_width[c].max(widest.min(MAX_COL)).ceil();
        }
    }
    let row_height: Vec<f32> = rows
        .iter()
        .map(|row| {
            let lines = row
                .iter()
                .map(|c| c.text.split('\n').count())
                .max()
                .unwrap_or(1);
            (lines as f32 * LINE + 2.0 * PAD_Y).ceil()
        })
        .collect();
    let width = col_width.iter().sum::<f32>() + 1.0;
    let height = row_height.iter().sum::<f32>() + 1.0;
    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\">\
         <rect width=\"{width}\" height=\"{height}\" fill=\"#ffffff\"/>"
    );
    let mut y = 0.5;
    for (r, row) in rows.iter().enumerate() {
        let mut x = 0.5;
        for (c, &w) in col_width.iter().enumerate() {
            let cell = row.get(c).cloned().unwrap_or_default();
            let h = row_height[r];
            let fill = cell
                .fill
                .map_or("#ffffff".to_owned(), |f| format!("#{f:06x}"));
            let _ = write!(
                out,
                "<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"{fill}\" stroke=\"{BORDER}\" stroke-width=\"1\"/>"
            );
            let _ = write!(
                out,
                "<clipPath id=\"c{r}_{c}\"><rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\"/></clipPath>"
            );
            let (anchor, tx) = match cell.align {
                Align::Left => ("start", x + PAD_X),
                Align::Center => ("middle", x + w / 2.0),
                Align::Right => ("end", x + w - PAD_X),
            };
            for (l, line) in cell.text.split('\n').enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                let baseline = y + PAD_Y + l as f32 * LINE + FONT_SIZE;
                let _ = write!(
                    out,
                    "<text x=\"{tx}\" y=\"{baseline}\" text-anchor=\"{anchor}\" clip-path=\"url(#c{r}_{c})\" {} xml:space=\"preserve\">{}</text>",
                    text_attrs(&cell),
                    escape(line)
                );
            }
            x += w;
        }
        y += row_height[r];
    }
    out.push_str("</svg>");
    Some((out, width.ceil() as u32, height.ceil() as u32))
}

/// The cells, row by row, as a PNG; `None` when there is nothing to draw
/// or it would be huge.
pub fn png(rows: &[Vec<Cell>]) -> Option<Vec<u8>> {
    let (svg, width, height) = svg(rows)?;
    if u64::from(width) * u64::from(height) > 40_000_000 {
        return None;
    }
    let tree = usvg::Tree::from_str(&svg, &options()).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let image = image::RgbaImage::from_fn(width, height, |x, y| {
        pixmap.pixel(x, y).map_or(image::Rgba([255; 4]), |pixel| {
            let pixel = pixel.demultiply();
            image::Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
        })
    });
    let mut out = std::io::Cursor::new(Vec::new());
    image.write_to(&mut out, image::ImageFormat::Png).ok()?;
    Some(out.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_cells() {
        let cell = |text: &str| Cell {
            text: text.into(),
            ..Cell::default()
        };
        let rows = vec![
            vec![
                Cell {
                    bold: true,
                    fill: Some(0xffff00),
                    ..cell("Name")
                },
                cell("Amount"),
            ],
            vec![
                cell("Tea & <cake>"),
                Cell {
                    align: Align::Right,
                    ..cell("1,200")
                },
            ],
        ];
        let (svg, width, height) = svg(&rows).unwrap();
        assert!(svg.contains("Tea &amp; &lt;cake&gt;"));
        assert!(width >= 2 * MIN_COL as u32 && height >= 2 * LINE as u32);
        let bytes = png(&rows).unwrap();
        assert_eq!(&bytes[..4], b"\x89PNG");
        assert!(png(&[]).is_none());
    }
}
