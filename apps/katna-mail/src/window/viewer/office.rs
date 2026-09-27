// SPDX-License-Identifier: GPL-3.0-or-later

//! Spreadsheets and word processor documents in the attachment viewer.
//!
//! A spreadsheet is a grid on white with column letters (kept at the top
//! while scrolling) and row numbers, only the rows on screen laid out, and
//! a tab per sheet at the foot. A document is laid out as one long white
//! page: headings, lists, tables and bold/italic/underline/strike-through
//! runs, only the paragraphs on screen laid out.

use std::ops::Range;
use std::rc::Rc;

use gpui::ListHorizontalSizingBehavior;
use gpui::{
    AnyElement, Context, FontStyle, FontWeight, HighlightStyle, ListAlignment, ListState,
    SharedString, StrikethroughStyle, StyledText, UnderlineStyle, UniformListScrollHandle, div,
    list, prelude::*, rgba, uniform_list,
};
use katna_preview::document::{Align, Block, Document, Paragraph, Style};
use katna_preview::sheet::{self, MAX_ROWS, Sheet, Workbook};
use katna_ui::px;
use katna_ui::unpx;

use super::{BAR_HEIGHT, Viewer};

// Paper colors, the same in light and dark themes, like a printed page.
const PAPER: u32 = 0xffffffff;
const INK: u32 = 0x202124ff;
const INK_DIM: u32 = 0x5f6368ff;
const GRID: u32 = 0xe0e3e7ff;
const HEADER: u32 = 0xf1f3f4ff;
/// "Slide 3" above each slide's page, on the dark backdrop.
const SLIDE_LABEL: u32 = 0xffffffb3;
const SHEET_GREEN: u32 = 0x188038ff;

/// A column is at least this wide at zoom 1, and at most...
const MIN_COLUMN: f32 = 64.0;
/// ...this wide.
const MAX_COLUMN: f32 = 320.0;
/// Rows looked at to size the columns.
const SIZING_ROWS: usize = 200;
/// A document page is this wide at zoom 1 (8.5 inches).
const PAGE_WIDTH: f32 = 816.0;

pub(super) struct SheetView {
    pub book: Rc<Workbook>,
    /// The sheet on show.
    pub current: usize,
    /// Each sheet's column widths at zoom 1, in logical pixels.
    widths: Vec<Rc<Vec<f32>>>,
    pub scroll: UniformListScrollHandle,
}

impl SheetView {
    pub fn new(book: Workbook) -> Self {
        let widths = book
            .sheets
            .iter()
            .map(|sheet| Rc::new(column_widths(sheet)))
            .collect();
        Self {
            book: Rc::new(book),
            current: 0,
            widths,
            scroll: UniformListScrollHandle::new(),
        }
    }
}

pub(super) struct DocumentView {
    doc: Rc<Document>,
    pub state: ListState,
    /// The zoom the paragraphs were measured at.
    measured: f32,
}

impl DocumentView {
    pub fn new(doc: Document) -> Self {
        // The last item is the notice that the rest was cut, or padding.
        let state = ListState::new(doc.blocks.len() + 1, ListAlignment::Top, px(800.0));
        Self {
            doc: Rc::new(doc),
            state,
            measured: 1.0,
        }
    }
}

/// Column widths that fit most of each column's text.
fn column_widths(sheet: &Sheet) -> Vec<f32> {
    (0..sheet.columns)
        .map(|col| {
            let longest = sheet
                .rows
                .iter()
                .take(SIZING_ROWS)
                .filter_map(|row| row.get(col))
                .map(|cell| cell.lines().next().unwrap_or("").chars().count())
                .max()
                .unwrap_or(0);
            (longest as f32 * 7.4 + 18.0).clamp(MIN_COLUMN, MAX_COLUMN)
        })
        .collect()
}

/// Whether a cell reads as a number, shown on the right as spreadsheets do.
fn numeric(cell: &str) -> bool {
    let cell = cell.trim().trim_end_matches('%');
    !cell.is_empty() && cell.replace([',', ' '], "").parse::<f64>().is_ok()
}

/// The room around the white panel: space for the side arrows on wide
/// windows, a small margin on narrow ones.
fn side_margin(vw: f32) -> f32 {
    if vw < 700.0 { 12.0 } else { 80.0 }
}

impl Viewer {
    pub(super) fn show_sheet(&mut self, ix: usize, cx: &mut Context<Self>) {
        if let super::Content::Sheet(view) = &mut self.content
            && ix < view.book.sheets.len()
            && ix != view.current
        {
            view.current = ix;
            view.scroll = UniformListScrollHandle::new();
            cx.notify();
        }
    }

    pub(super) fn sheet_body(
        &self,
        view: &SheetView,
        zoom: f32,
        vw: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let book = view.book.clone();
        let current = view.current.min(book.sheets.len() - 1);
        let sheet = &book.sheets[current];
        let widths: Rc<Vec<f32>> = Rc::new(view.widths[current].iter().map(|w| w * zoom).collect());
        let row_height = (24.0 * zoom).round();
        let text_size = 13.0 * zoom;
        let last_row = sheet.origin.0 as usize + sheet.rows.len();
        let number_width = ((last_row.to_string().len() as f32) * 8.0 + 20.0) * zoom;
        let total_width = number_width + widths.iter().sum::<f32>();
        let margin = side_margin(vw);
        let scroll_x = unpx(view.scroll.0.borrow().base_handle.offset().x);

        // Column letters, moved with the grid's sideways scrolling.
        let first_col = sheet.origin.1;
        let header = div()
            .flex_none()
            .h(px(row_height))
            .overflow_hidden()
            .bg(rgba(HEADER))
            .border_b_1()
            .border_color(rgba(GRID))
            .child(
                div()
                    .relative()
                    .left(px(scroll_x))
                    .w(px(total_width))
                    .h_full()
                    .flex()
                    .flex_row()
                    .child(
                        div()
                            .flex_none()
                            .w(px(number_width))
                            .h_full()
                            .border_r_1()
                            .border_color(rgba(GRID)),
                    )
                    .children(widths.iter().enumerate().map(|(ix, w)| {
                        div()
                            .flex_none()
                            .w(px(*w))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .border_r_1()
                            .border_color(rgba(GRID))
                            .text_color(rgba(INK_DIM))
                            .child(sheet::column_name(first_col + ix as u32))
                    })),
            );

        let rows = {
            let book = book.clone();
            let widths = widths.clone();
            uniform_list(
                "viewer-sheet",
                sheet.rows.len(),
                move |range: Range<usize>, _, _| {
                    let sheet = &book.sheets[current];
                    range
                        .map(|ix| {
                            let cells = &sheet.rows[ix];
                            div()
                                .w(px(total_width))
                                .h(px(row_height))
                                .flex()
                                .flex_row()
                                .border_b_1()
                                .border_color(rgba(GRID))
                                .child(
                                    div()
                                        .flex_none()
                                        .w(px(number_width))
                                        .h_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .bg(rgba(HEADER))
                                        .border_r_1()
                                        .border_color(rgba(GRID))
                                        .text_color(rgba(INK_DIM))
                                        .child((sheet.origin.0 as usize + ix + 1).to_string()),
                                )
                                .children(cells.iter().zip(widths.iter()).map(|(cell, w)| {
                                    let line = cell.lines().next().unwrap_or("").to_owned();
                                    div()
                                        .flex_none()
                                        .w(px(*w))
                                        .h_full()
                                        .px(px(6.0))
                                        .flex()
                                        .items_center()
                                        .when(numeric(cell), |d| d.justify_end())
                                        .overflow_hidden()
                                        .border_r_1()
                                        .border_color(rgba(GRID))
                                        .child(div().truncate().child(line))
                                }))
                        })
                        .collect()
                },
            )
            .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
            .track_scroll(&view.scroll)
            .flex_1()
            .min_h_0()
        };

        let tabs = (book.sheets.len() > 1 || sheet.cut).then(|| {
            div()
                .id("viewer-sheet-tabs")
                .flex_none()
                .h(px(40.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(2.0))
                .overflow_x_scroll()
                .bg(rgba(HEADER))
                .border_t_1()
                .border_color(rgba(GRID))
                .text_size(px(13.0))
                .when(book.sheets.len() > 1, |d| {
                    d.children(book.sheets.iter().enumerate().map(|(ix, s)| {
                        let on = ix == current;
                        div()
                            .id(("viewer-sheet-tab", ix))
                            .flex_none()
                            .h(px(32.0))
                            .px(px(14.0))
                            .flex()
                            .items_center()
                            .rounded(px(6.0))
                            .cursor_pointer()
                            .text_color(rgba(if on { SHEET_GREEN } else { INK_DIM }))
                            .when(on, |d| {
                                d.bg(rgba(PAPER))
                                    .font_weight(FontWeight::MEDIUM)
                                    .border_b_2()
                                    .border_color(rgba(SHEET_GREEN))
                            })
                            .when(!on, |d| d.hover(|s| s.bg(rgba(0x0000000f))))
                            .on_click(cx.listener(move |this, _, _, cx| this.show_sheet(ix, cx)))
                            .child(SharedString::from(s.name.clone()))
                    }))
                })
                .when(sheet.cut, |d| {
                    d.child(div().flex_1()).child(
                        div()
                            .flex_none()
                            .px(px(8.0))
                            .text_color(rgba(INK_DIM))
                            .child(format!(
                                "Only the first {} rows and {} columns are shown",
                                sheet.rows.len().min(MAX_ROWS),
                                sheet.columns
                            )),
                    )
                })
        });

        div()
            .id("viewer-sheet-panel")
            .absolute()
            .top(px(BAR_HEIGHT + 8.0))
            .bottom(px(80.0))
            .left(px(margin))
            .right(px(margin))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(8.0))
            .bg(rgba(PAPER))
            .text_color(rgba(INK))
            .text_size(px(text_size))
            .occlude()
            // Keeps the column letters in step with the grid.
            .on_scroll_wheel(cx.listener(|_, _, _, cx| cx.notify()))
            .child(header)
            .child(rows)
            .children(tabs)
            .into_any_element()
    }

    pub(super) fn document_body(&mut self, zoom: f32, vw: f32) -> AnyElement {
        let super::Content::Document(view) = &mut self.content else {
            return div().into_any_element();
        };
        if view.measured != zoom {
            view.measured = zoom;
            view.state.remeasure();
        }
        let doc = view.doc.clone();
        let page = (vw - 2.0 * side_margin(vw)).clamp(280.0, PAGE_WIDTH) * zoom;
        let pad = if vw < 700.0 { 20.0 } else { 72.0 } * zoom;
        let count = doc.blocks.len();
        div()
            .size_full()
            .pt(px(BAR_HEIGHT + 8.0))
            .child(
                list(view.state.clone(), move |ix, _, _| {
                    // Slides are pages of their own, each under its label.
                    if let Some(Block::Slide(n)) = doc.blocks.get(ix) {
                        return div()
                            .w_full()
                            .flex()
                            .justify_center()
                            .child(
                                div()
                                    .w(px(page))
                                    .pt(px(if ix == 0 { 8.0 } else { 28.0 } * zoom))
                                    .pb(px(8.0 * zoom))
                                    .text_size(px(13.0 * zoom))
                                    .text_color(rgba(SLIDE_LABEL))
                                    .child(SharedString::from(katna_i18n::tr!("viewer-slide", number = n))),
                            )
                            .into_any_element();
                    }
                    let slide_edge = |ix: Option<usize>| {
                        ix.and_then(|ix| doc.blocks.get(ix))
                            .is_some_and(|b| matches!(b, Block::Slide(_)))
                    };
                    let top = ix == 0 || slide_edge(ix.checked_sub(1));
                    let end = ix == count || slide_edge(Some(ix + 1));
                    let content: AnyElement = match doc.blocks.get(ix) {
                        Some(Block::Paragraph(p)) => paragraph(p, zoom, false),
                        Some(Block::Table(rows)) => table(rows, zoom),
                        Some(Block::Slide(_)) => div().into_any_element(),
                        None if doc.cut => div()
                            .pt(px(16.0 * zoom))
                            .text_size(px(13.0 * zoom))
                            .text_color(rgba(INK_DIM))
                            .child("The rest of this document is not shown. Open it in another app to read it all.")
                            .into_any_element(),
                        None => div().into_any_element(),
                    };
                    div()
                        .w_full()
                        .flex()
                        .justify_center()
                        .when(ix == count, |d| d.pb(px(96.0)))
                        .child(
                            div()
                                .w(px(page))
                                .px(px(pad))
                                .bg(rgba(PAPER))
                                .text_color(rgba(INK))
                                .when(top, |d| d.pt(px(pad)).rounded_t(px(4.0)))
                                .when(end, |d| d.pb(px(pad)).rounded_b(px(4.0)))
                                .child(content),
                        )
                        .into_any_element()
                })
                .size_full(),
            )
            .into_any_element()
    }
}

/// A paragraph: its text in its runs' looks, sized by its style.
fn paragraph(p: &Paragraph, zoom: f32, in_cell: bool) -> AnyElement {
    let (size, weight, color, before, after) = match p.style {
        Style::Title => (26.0, FontWeight::NORMAL, INK, 0.0, 12.0),
        Style::Subtitle => (17.0, FontWeight::NORMAL, INK_DIM, 0.0, 12.0),
        Style::Heading(1) => (20.0, FontWeight::SEMIBOLD, INK, 14.0, 8.0),
        Style::Heading(2) => (17.0, FontWeight::SEMIBOLD, INK, 12.0, 6.0),
        Style::Heading(3) => (15.0, FontWeight::SEMIBOLD, INK, 10.0, 4.0),
        Style::Heading(_) => (14.0, FontWeight::SEMIBOLD, INK_DIM, 8.0, 4.0),
        Style::Normal => (14.0, FontWeight::NORMAL, INK, 0.0, 8.0),
    };
    let size = size * zoom;
    // Table cells are tight: no space above or below their paragraphs.
    let (before, after) = if in_cell { (0.0, 0.0) } else { (before, after) };
    let text = styled(p);
    let body = div()
        .flex_1()
        .min_w_0()
        .when(p.align == Align::Center, |d| d.text_center())
        .when(p.align == Align::End, |d| d.text_right())
        .child(text);
    div()
        .w_full()
        .pt(px(before * zoom))
        .pb(px(after * zoom))
        .min_h(px(size * 1.5))
        .flex()
        .flex_row()
        .text_size(px(size))
        .line_height(px(size * 1.5))
        .font_weight(weight)
        .text_color(rgba(color))
        .when_some(p.list.as_ref(), |d, (level, marker)| {
            d.pl(px(24.0 * zoom * f32::from(*level))).child(
                div()
                    .flex_none()
                    .min_w(px(24.0 * zoom))
                    .pr(px(6.0 * zoom))
                    .child(SharedString::from(marker.clone())),
            )
        })
        .child(body)
        .into_any_element()
}

/// A paragraph's text with its bold, italic, underlined and struck-through
/// runs.
fn styled(p: &Paragraph) -> StyledText {
    let mut text = String::new();
    let mut highlights = Vec::new();
    for run in &p.runs {
        let start = text.len();
        text.push_str(&run.text);
        let style = HighlightStyle {
            font_weight: run.bold.then_some(FontWeight::BOLD),
            font_style: run.italic.then_some(FontStyle::Italic),
            underline: run.underline.then(|| UnderlineStyle {
                thickness: px(1.0),
                color: None,
                wavy: false,
            }),
            strikethrough: run.strike.then(|| StrikethroughStyle {
                thickness: px(1.0),
                color: None,
            }),
            ..Default::default()
        };
        if style != HighlightStyle::default() && start < text.len() {
            highlights.push((start..text.len(), style));
        }
    }
    StyledText::new(SharedString::from(text)).with_highlights(highlights)
}

/// A table: its cells side by side with thin rules, each cell its
/// paragraphs.
fn table(rows: &[Vec<Vec<Paragraph>>], zoom: f32) -> AnyElement {
    div()
        .w_full()
        .my(px(8.0 * zoom))
        .flex()
        .flex_col()
        .border_1()
        .border_color(rgba(GRID))
        .children(rows.iter().enumerate().map(|(r, row)| {
            div()
                .w_full()
                .flex()
                .flex_row()
                .when(r > 0, |d| d.border_t_1().border_color(rgba(GRID)))
                .children(row.iter().enumerate().map(|(c, cell)| {
                    div()
                        .flex_1()
                        .min_w_0()
                        .p(px(6.0 * zoom))
                        .when(c > 0, |d| d.border_l_1().border_color(rgba(GRID)))
                        .children(cell.iter().map(|p| {
                            let mut p = p.clone();
                            // Inside a cell even headings stay small.
                            if matches!(p.style, Style::Title | Style::Subtitle) {
                                p.style = Style::Heading(3);
                            }
                            paragraph(&p, zoom * 0.95, true)
                        }))
                }))
        }))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_go_right() {
        assert!(numeric("12.5"));
        assert!(numeric("-3"));
        assert!(numeric("1,234"));
        assert!(numeric("45%"));
        assert!(!numeric("Total"));
        assert!(!numeric(""));
        assert!(!numeric("2024-01-05"));
    }

    #[test]
    fn columns_fit_their_text() {
        let sheet = Sheet {
            name: "S".into(),
            origin: (0, 0),
            rows: vec![vec!["a".into(), "x".repeat(20), "y".repeat(200)]],
            columns: 3,
            cut: false,
        };
        let widths = column_widths(&sheet);
        assert_eq!(widths[0], MIN_COLUMN);
        assert!(widths[1] > MIN_COLUMN && widths[1] < MAX_COLUMN);
        assert_eq!(widths[2], MAX_COLUMN);
    }
}
