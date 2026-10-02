// SPDX-License-Identifier: GPL-3.0-or-later

//! Spreadsheets and word processor documents in the attachment viewer.
//!
//! A spreadsheet is a grid on white with column letters (kept at the top
//! while scrolling) and row numbers, only the rows on screen laid out, and
//! a tab per sheet at the foot. A document is laid out as one long white
//! page: headings, lists, tables and bold/italic/underline/strike-through
//! runs, only the paragraphs on screen laid out.
//!
//! A document's text selects like a message's. A spreadsheet selects
//! cells: click one, drag or Shift+click for a range, click a column
//! letter or row number for all of it; Ctrl+C copies them tab-separated,
//! which pastes as cells into other spreadsheets.

use std::ops::Range;
use std::rc::Rc;

use gpui::ListHorizontalSizingBehavior;
use gpui::{
    AnyElement, ClipboardItem, Context, DispatchPhase, FontStyle, FontWeight, HighlightStyle,
    ListAlignment, ListState, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels,
    Point, SharedString, StrikethroughStyle, UnderlineStyle, UniformListScrollHandle, canvas, div,
    list, prelude::*, rgba, uniform_list,
};
use katna_preview::document::{Align, Block, Document, Paragraph, Style};
use katna_preview::sheet::{self, MAX_ROWS, Sheet, Workbook};
use katna_ui::px;
use katna_ui::unpx;

use super::super::select::{self, Key, Marker, MenuAct};
use super::{Content, Viewer};
use crate::theme::Theme;
use crate::widgets::ScaledEdge;

// Paper colors, the same in light and dark themes, like a printed page.
const PAPER: u32 = 0xffffffff;
/// The corners of the spreadsheet's panel.
const PANEL_RADIUS: f32 = 8.0;
const INK: u32 = 0x202124ff;
const INK_DIM: u32 = 0x5f6368ff;
const GRID: u32 = 0xe0e3e7ff;
const HEADER: u32 = 0xf1f3f4ff;
/// "Slide 3" above each slide's page, on the dark backdrop.
const SLIDE_LABEL: u32 = 0xffffffb3;
const SHEET_GREEN: u32 = 0x188038ff;
/// Column letters and row numbers of selected cells.
const PICKED: u32 = 0xd3e3fdff;

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
    /// The selected cells: where the selection started and where it ends
    /// now, as (row, column) in the sheet's rows.
    cells: Option<(Cell, Cell)>,
    /// A drag is extending the selection, by `Drag`.
    dragging: Option<Drag>,
    /// The right-click menu, where the pointer was.
    pub(super) menu: Option<Point<Pixels>>,
}

type Cell = (usize, usize);

/// What a drag over the sheet selects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Drag {
    Cells,
    Rows,
    Columns,
}

/// Where the pointer is on the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spot {
    Cell(Cell),
    /// A row's number.
    Row(usize),
    /// A column's letter.
    Column(usize),
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
            cells: None,
            dragging: None,
            menu: None,
        }
    }

    /// The sheet on show.
    fn sheet(&self) -> &Sheet {
        &self.book.sheets[self.current.min(self.book.sheets.len() - 1)]
    }

    /// The selection as top-left and bottom-right cells.
    fn range(&self) -> Option<(Cell, Cell)> {
        let ((r0, c0), (r1, c1)) = self.cells?;
        Some(((r0.min(r1), c0.min(c1)), (r0.max(r1), c0.max(c1))))
    }

    /// The grid at `zoom`: row height, row number width and column widths.
    fn grid(&self, zoom: f32) -> Grid {
        let sheet = self.sheet();
        let last_row = sheet.origin.0 as usize + sheet.rows.len();
        Grid {
            row_height: (24.0 * zoom).round(),
            number_width: ((last_row.to_string().len() as f32) * 8.0 + 20.0) * zoom,
            widths: self.widths[self.current.min(self.widths.len() - 1)]
                .iter()
                .map(|w| w * zoom)
                .collect(),
        }
    }

    /// What is under `at` (in window coordinates) on the grid at `zoom`;
    /// the header row when `header`.
    fn spot(&self, at: Point<Pixels>, zoom: f32, header: bool) -> Option<Spot> {
        let grid = self.grid(zoom);
        let handle = self.scroll.0.borrow().base_handle.clone();
        let view = handle.bounds();
        let offset = handle.offset();
        let sheet = self.sheet();
        let x = unpx(at.x - view.left() - offset.x) - grid.number_width;
        let column = (x >= 0.0).then(|| {
            let mut edge = 0.0;
            grid.widths
                .iter()
                .position(|w| {
                    edge += w;
                    x < edge
                })
                .unwrap_or(grid.widths.len().saturating_sub(1))
        });
        if header {
            return column.map(Spot::Column);
        }
        let y = unpx(at.y - view.top() - offset.y);
        let row = ((y / grid.row_height).floor().max(0.0) as usize)
            .min(sheet.rows.len().saturating_sub(1));
        Some(match column {
            Some(column) => Spot::Cell((row, column)),
            None => Spot::Row(row),
        })
    }

    /// The selected cells as text: columns apart by tabs, rows by line
    /// breaks, and cells with either (or quotes) quoted, as spreadsheets
    /// copy them.
    fn copied(&self) -> String {
        let Some(((r0, c0), (r1, c1))) = self.range() else {
            return String::new();
        };
        let sheet = self.sheet();
        let mut out = String::new();
        for row in r0..=r1.min(sheet.rows.len().saturating_sub(1)) {
            if row > r0 {
                out.push('\n');
            }
            for column in c0..=c1 {
                if column > c0 {
                    out.push('\t');
                }
                let cell = sheet.rows[row].get(column).map_or("", String::as_str);
                if cell.contains(['\t', '\n', '"']) {
                    out.push('"');
                    out.push_str(&cell.replace('"', "\"\""));
                    out.push('"');
                } else {
                    out.push_str(cell);
                }
            }
        }
        out
    }
}

/// A sheet's grid at the zoom on show, in logical pixels.
struct Grid {
    row_height: f32,
    number_width: f32,
    widths: Vec<f32>,
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

    /// Where each slide starts, by block, for a presentation; empty for a
    /// document.
    pub fn slide_starts(&self) -> Vec<usize> {
        (self.doc.blocks.iter().enumerate())
            .filter(|(_, block)| matches!(block, Block::Slide(_)))
            .map(|(ix, _)| ix)
            .collect()
    }

    /// The slide (from 0) at the top of the view, of `starts`.
    pub fn top_slide(&self, starts: &[usize]) -> usize {
        let top = self.state.logical_scroll_top().item_ix;
        starts.iter().rposition(|&ix| ix <= top).unwrap_or(0)
    }

    /// Scrolls slide `slide` (from 0) of `starts` to the top.
    pub fn go_to_slide(&self, slide: usize, starts: &[usize]) {
        if let Some(&item_ix) = starts.get(slide) {
            self.state.scroll_to(gpui::ListOffset {
                item_ix,
                offset_in_item: px(0.0),
            });
        }
    }

    /// All of the document's text, keyed as it is drawn: a block is a
    /// part, and a table's paragraphs its pieces, cell by cell.
    pub fn all_text(&self) -> Rc<Vec<(Key, SharedString)>> {
        let mut all = Vec::new();
        for (ix, block) in self.doc.blocks.iter().enumerate() {
            match block {
                Block::Paragraph(p) => all.push((Key::new(ix, 0), text_of(p))),
                Block::Table(rows) => {
                    let cells = rows.iter().flatten().flatten();
                    for (piece, p) in cells.enumerate() {
                        all.push((Key::new(ix, piece), text_of(p)));
                    }
                }
                Block::Slide(_) => {}
            }
        }
        Rc::new(all)
    }
}

/// A paragraph's text.
/// `color`, flipped for a dark page when `dark`.
fn ink(color: u32, dark: bool) -> gpui::Rgba {
    rgba(if dark {
        katna_preview::dark::flip_rgba(color)
    } else {
        color
    })
}

fn text_of(p: &Paragraph) -> SharedString {
    p.runs
        .iter()
        .map(|run| run.text.as_str())
        .collect::<String>()
        .into()
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

/// The zoom at which a document's page fills a viewer `vw` wide.
pub(super) fn document_fit_width(vw: f32) -> f32 {
    let room = vw - 2.0 * side_margin(vw);
    room / room.clamp(280.0, PAGE_WIDTH)
}

impl SheetView {
    /// The zoom at which every column of the sheet on show fits a viewer
    /// `vw` wide.
    pub(super) fn fit_width(&self, vw: f32) -> f32 {
        let grid = self.grid(1.0);
        let total = grid.number_width + grid.widths.iter().sum::<f32>();
        (vw - 2.0 * side_margin(vw) - 2.0) / total.max(1.0)
    }
}

impl Viewer {
    pub(super) fn show_sheet(&mut self, ix: usize, cx: &mut Context<Self>) {
        if let super::Content::Sheet(view) = &mut self.content
            && ix < view.book.sheets.len()
            && ix != view.current
        {
            view.current = ix;
            view.scroll = UniformListScrollHandle::new();
            view.cells = None;
            view.menu = None;
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
        let grid = view.grid(zoom);
        let widths: Rc<Vec<f32>> = Rc::new(grid.widths);
        let row_height = grid.row_height;
        let text_size = 13.0 * zoom;
        let number_width = grid.number_width;
        let total_width = number_width + widths.iter().sum::<f32>();
        let selected = view.range();
        let anchor = view.cells.map(|(anchor, _)| anchor);
        let accent = self.th.accent;
        let dark = self.dark_pages_shown();
        let tint = rgba((accent & 0xffff_ff00) | 0x26);
        let picked = ink(PICKED, dark);
        let in_rows =
            move |row: usize| selected.is_some_and(|((r0, _), (r1, _))| (r0..=r1).contains(&row));
        let in_columns = move |column: usize| {
            selected.is_some_and(|((_, c0), (_, c1))| (c0..=c1).contains(&column))
        };
        let margin = side_margin(vw);
        let scroll_x = unpx(view.scroll.0.borrow().base_handle.offset().x);

        // Column letters, moved with the grid's sideways scrolling.
        let first_col = sheet.origin.1;
        let header = div()
            .flex_none()
            .h(px(row_height))
            .overflow_hidden()
            // GPUI does not clip to the panel's rounded corners: the
            // letters' grey rounds its own.
            .rounded_t(px(PANEL_RADIUS))
            .bg(ink(HEADER, dark))
            .border_b_1()
            .border_color(ink(GRID, dark))
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
                            .border_color(ink(GRID, dark)),
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
                            .border_color(ink(GRID, dark))
                            .text_color(ink(INK_DIM, dark))
                            .when(in_columns(ix), |d| d.bg(picked))
                            .child(sheet::column_name(first_col + ix as u32))
                    })),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    this.press_cells(event, true, window, cx);
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event: &MouseDownEvent, _, cx| this.cell_menu_at(event, cx)),
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
                                .border_color(ink(GRID, dark))
                                .child(
                                    div()
                                        .flex_none()
                                        .w(px(number_width))
                                        .h_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .bg(ink(if in_rows(ix) { PICKED } else { HEADER }, dark))
                                        .border_r_1()
                                        .border_color(ink(GRID, dark))
                                        .text_color(ink(INK_DIM, dark))
                                        .child((sheet.origin.0 as usize + ix + 1).to_string()),
                                )
                                .children(widths.iter().enumerate().map(|(column, w)| {
                                    let cell = cells.get(column).map_or("", String::as_str);
                                    let line = cell.lines().next().unwrap_or("").to_owned();
                                    let on = in_rows(ix) && in_columns(column);
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
                                        .border_color(ink(GRID, dark))
                                        .when(on, |d| d.bg(tint))
                                        // The cell the selection started in, as
                                        // spreadsheets show the active cell.
                                        .when(anchor == Some((ix, column)), |d| {
                                            d.bg(ink(PAPER, dark))
                                                .border_px(2.0)
                                                .border_color(rgba(accent))
                                                .px(px(5.0))
                                        })
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
            .cursor(gpui::CursorStyle::Arrow)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    this.press_cells(event, false, window, cx);
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event: &MouseDownEvent, _, cx| this.cell_menu_at(event, cx)),
            )
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
                .rounded_b(px(PANEL_RADIUS))
                .bg(ink(HEADER, dark))
                .border_t_1()
                .border_color(ink(GRID, dark))
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
                            .text_color(ink(if on { SHEET_GREEN } else { INK_DIM }, dark))
                            .when(on, |d| {
                                d.bg(ink(PAPER, dark))
                                    .font_weight(FontWeight::MEDIUM)
                                    .border_b_2()
                                    .border_color(ink(SHEET_GREEN, dark))
                            })
                            .when(!on, |d| {
                                d.hover(move |s| {
                                    s.bg(rgba(if dark { 0xffffff14 } else { 0x0000000f }))
                                })
                            })
                            .on_click(cx.listener(move |this, _, _, cx| this.show_sheet(ix, cx)))
                            .child(SharedString::from(s.name.clone()))
                    }))
                })
                .when(sheet.cut, |d| {
                    d.child(div().flex_1()).child(
                        div()
                            .flex_none()
                            .px(px(8.0))
                            .text_color(ink(INK_DIM, dark))
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
            .top(px(8.0))
            .bottom(px(80.0))
            .left(px(margin))
            .right(px(margin))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(PANEL_RADIUS))
            .bg(ink(PAPER, dark))
            .text_color(ink(INK, dark))
            .text_size(px(text_size))
            .occlude()
            // Keeps the column letters in step with the grid.
            .on_scroll_wheel(cx.listener(|_, _, _, cx| cx.notify()))
            .child(header)
            .child(rows)
            .children(tabs)
            .into_any_element()
    }

    pub(super) fn document_body(
        &mut self,
        zoom: f32,
        vw: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let dark = self.dark_pages_shown();
        let super::Content::Document(view) = &mut self.content else {
            return div().into_any_element();
        };
        if view.measured != zoom {
            view.measured = zoom;
            view.state.remeasure();
        }
        let doc = view.doc.clone();
        let marker = self.text.marker(&self.th);
        let page = (vw - 2.0 * side_margin(vw)).clamp(280.0, PAGE_WIDTH) * zoom;
        let pad = if vw < 700.0 { 20.0 } else { 72.0 } * zoom;
        let count = doc.blocks.len();
        let viewer = cx.entity().downgrade();
        select::selectable(div(), None, cx)
            .size_full()
            .pt(px(8.0))
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
                        Some(Block::Paragraph(p)) => paragraph(p, zoom, false, &marker, Key::new(ix, 0), dark),
                        Some(Block::Table(rows)) => table(rows, zoom, &marker, ix, dark),
                        Some(Block::Slide(_)) => div().into_any_element(),
                        None if doc.cut => div()
                            .pt(px(16.0 * zoom))
                            .text_size(px(13.0 * zoom))
                            .text_color(ink(INK_DIM, dark))
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
                                .bg(ink(PAPER, dark))
                                .capture_any_mouse_down({
                                    let viewer = viewer.clone();
                                    move |_, _, cx| {
                                        viewer.update(cx, |this, _| this.backdrop = None).ok();
                                    }
                                })
                                .text_color(ink(INK, dark))
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

impl Viewer {
    pub(super) fn sheet_view(&self) -> Option<&SheetView> {
        match &self.content {
            Content::Sheet(view) => Some(view),
            _ => None,
        }
    }

    fn sheet_view_mut(&mut self) -> Option<&mut SheetView> {
        match &mut self.content {
            Content::Sheet(view) => Some(view),
            _ => None,
        }
    }

    /// A press on the grid (or its column letters, when `header`):
    /// selects a cell, row or column, or extends the selection with Shift.
    fn press_cells(
        &mut self,
        event: &MouseDownEvent,
        header: bool,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        window.focus(&self.focus, cx);
        let zoom = self.zoom_value();
        let Some(view) = self.sheet_view_mut() else {
            return;
        };
        view.menu = None;
        let Some(spot) = view.spot(event.position, zoom, header) else {
            return;
        };
        let last_row = view.sheet().rows.len().saturating_sub(1);
        let last_column = view.sheet().columns.saturating_sub(1);
        let (from, to, drag) = match spot {
            Spot::Cell(cell) => (cell, cell, Drag::Cells),
            Spot::Row(row) => ((row, 0), (row, last_column), Drag::Rows),
            Spot::Column(column) => ((0, column), (last_row, column), Drag::Columns),
        };
        view.cells = match view.cells {
            Some((anchor, _)) if event.modifiers.shift => Some((anchor, to)),
            _ => Some((from, to)),
        };
        view.dragging = Some(drag);
        cx.notify();
    }

    fn drag_cells(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if event.pressed_button != Some(MouseButton::Left) {
            self.release_cells(cx);
            return;
        }
        let zoom = self.zoom_value();
        let Some(view) = self.sheet_view_mut() else {
            return;
        };
        let (Some(drag), Some((anchor, head))) = (view.dragging, view.cells) else {
            return;
        };
        let Some(spot) = view.spot(event.position, zoom, false) else {
            return;
        };
        let (row, column) = match spot {
            Spot::Cell(cell) => cell,
            Spot::Row(row) => (row, 0),
            Spot::Column(column) => (head.0, column),
        };
        let head = match drag {
            Drag::Cells => (row, column),
            Drag::Rows => (row, head.1),
            Drag::Columns => (head.0, column),
        };
        if view.cells != Some((anchor, head)) {
            view.cells = Some((anchor, head));
            cx.notify();
        }
    }

    fn release_cells(&mut self, cx: &mut Context<Self>) {
        let Some(view) = self.sheet_view_mut() else {
            return;
        };
        view.dragging = None;
        // Pasted with the middle button, as on any Linux desktop.
        let copied = view.copied();
        if !copied.is_empty() {
            katna_ui::native::write_to_primary(cx, ClipboardItem::new_string(copied));
        }
    }

    pub(super) fn copy_cells(&mut self, cx: &mut Context<Self>) {
        let copied = self.sheet_view().map(SheetView::copied).unwrap_or_default();
        if !copied.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(copied));
        }
    }

    pub(super) fn select_all_cells(&mut self, cx: &mut Context<Self>) {
        if let Some(view) = self.sheet_view_mut() {
            let sheet = view.sheet();
            if !sheet.rows.is_empty() && sheet.columns > 0 {
                let end = (sheet.rows.len() - 1, sheet.columns - 1);
                view.cells = Some(((0, 0), end));
                cx.notify();
            }
        }
    }

    fn cell_menu_at(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        cx.stop_propagation();
        if let Some(view) = self.sheet_view_mut() {
            view.menu = Some(event.position);
            cx.notify();
        }
    }

    /// Follows the pointer while a drag selects cells, wherever it goes.
    pub(super) fn follow_cell_drags(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity().downgrade();
        let dragging = |viewer: &Viewer| viewer.sheet_view().is_some_and(|v| v.dragging.is_some());
        canvas(
            |_, _, _| {},
            move |_, _, window, _| {
                window.on_mouse_event({
                    let this = this.clone();
                    move |event: &MouseMoveEvent, phase, _, cx| {
                        let Some(this) = this.upgrade() else {
                            return;
                        };
                        if phase == DispatchPhase::Bubble && dragging(this.read(cx)) {
                            this.update(cx, |this, cx| this.drag_cells(event, cx));
                        }
                    }
                });
                window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                    let Some(this) = this.upgrade() else {
                        return;
                    };
                    if phase == DispatchPhase::Bubble
                        && event.button == MouseButton::Left
                        && dragging(this.read(cx))
                    {
                        this.update(cx, |this, cx| this.release_cells(cx));
                    }
                });
            },
        )
        .absolute()
        .size_0()
    }

    /// The right-click menu of the selected cells, when open.
    pub(super) fn cell_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let view = self.sheet_view()?;
        let at = view.menu?;
        Some(select::copy_menu(
            at,
            view.cells.is_some(),
            false,
            false,
            th,
            cx,
            |this: &mut Viewer, act, cx| {
                if let Some(view) = this.sheet_view_mut() {
                    view.menu = None;
                }
                match act {
                    MenuAct::Close | MenuAct::CopyAddress | MenuAct::Pin => {}
                    MenuAct::Copy => this.copy_cells(cx),
                    MenuAct::SelectAll => this.select_all_cells(cx),
                }
                cx.notify();
            },
        ))
    }
}

/// A paragraph: its text in its runs' looks, sized by its style,
/// selectable as the piece `key`.
fn paragraph(
    p: &Paragraph,
    zoom: f32,
    in_cell: bool,
    marker: &Marker,
    key: Key,
    dark: bool,
) -> AnyElement {
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
    let (text, highlights) = styled(p);
    let (text, holder) = marker.piece(key, text, highlights);
    let body = holder
        .flex_1()
        .min_w_0()
        // A short line is centered (or put right) as a whole, not only
        // its letters: GPUI draws a selection only across the line's own
        // width from the left.
        .when(p.align == Align::Center, |d| {
            d.flex().justify_center().text_center()
        })
        .when(p.align == Align::End, |d| {
            d.flex().justify_end().text_right()
        })
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
        .text_color(ink(color, dark))
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

/// A paragraph's text and its bold, italic, underlined and struck-through
/// runs.
fn styled(p: &Paragraph) -> (SharedString, Vec<(Range<usize>, HighlightStyle)>) {
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
    (SharedString::from(text), highlights)
}

/// A table: its cells side by side with thin rules, each cell its
/// paragraphs, the pieces of `part` in order.
fn table(
    rows: &[Vec<Vec<Paragraph>>],
    zoom: f32,
    marker: &Marker,
    part: usize,
    dark: bool,
) -> AnyElement {
    let mut piece = 0;
    div()
        .w_full()
        .my(px(8.0 * zoom))
        .flex()
        .flex_col()
        .border_1()
        .border_color(ink(GRID, dark))
        .children(rows.iter().enumerate().map(|(r, row)| {
            div()
                .w_full()
                .flex()
                .flex_row()
                .when(r > 0, |d| d.border_t_1().border_color(ink(GRID, dark)))
                .children(row.iter().enumerate().map(|(c, cell)| {
                    div()
                        .flex_1()
                        .min_w_0()
                        .p(px(6.0 * zoom))
                        .when(c > 0, |d| d.border_l_1().border_color(ink(GRID, dark)))
                        .children(cell.iter().map(|p| {
                            let mut p = p.clone();
                            // Inside a cell even headings stay small.
                            if matches!(p.style, Style::Title | Style::Subtitle) {
                                p.style = Style::Heading(3);
                            }
                            let key = Key::new(part, piece);
                            piece += 1;
                            paragraph(&p, zoom * 0.95, true, marker, key, dark)
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
    fn cells_copy_tab_separated() {
        let book = Workbook {
            sheets: vec![Sheet {
                name: "S".into(),
                origin: (0, 0),
                rows: vec![
                    vec!["Name".into(), "Note".into(), "Sum".into()],
                    vec!["Ann".into(), "two\nlines".into(), "3".into()],
                    vec!["Bo \"B\"".into()],
                ],
                columns: 3,
                cut: false,
            }],
        };
        let mut view = SheetView::new(book);
        assert_eq!(view.copied(), "");
        // Selected upward and leftward from the bottom right.
        view.cells = Some(((2, 2), (0, 0)));
        assert_eq!(
            view.copied(),
            "Name\tNote\tSum\nAnn\t\"two\nlines\"\t3\n\"Bo \"\"B\"\"\"\t\t"
        );
        view.cells = Some(((1, 2), (1, 2)));
        assert_eq!(view.copied(), "3");
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
