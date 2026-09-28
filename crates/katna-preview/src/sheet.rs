// SPDX-License-Identifier: GPL-3.0-or-later

//! Spreadsheets for the viewer: Excel (xlsx, xlsm, xlsb, xls) and
//! OpenDocument (ods) workbooks read by calamine, and CSV/TSV files. Only
//! the values are kept (formulas show their last result), as text.

use std::io::Cursor;

use calamine::{Cell, Data, DataRef, Reader, Sheets, open_workbook_auto_from_rs};

/// At most this many rows of a sheet are shown...
pub const MAX_ROWS: usize = 20_000;
/// ...this many columns...
pub const MAX_COLUMNS: usize = 256;
/// ...and this many cells in the whole workbook.
pub const MAX_CELLS: usize = 2_000_000;
/// A cell shows at most this many characters.
const MAX_CELL_CHARS: usize = 1_000;
/// At most this much of a CSV file is read.
const MAX_CSV_BYTES: usize = 16 * 1024 * 1024;

/// A workbook: one sheet or more.
#[derive(Debug, Clone, PartialEq)]
pub struct Workbook {
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sheet {
    pub name: String,
    /// The row and column number (from 0) of `rows[0][0]`, so the viewer
    /// can label rows and columns as the spreadsheet app would.
    pub origin: (u32, u32),
    /// Cells as text, every row `columns` long.
    pub rows: Vec<Vec<String>>,
    pub columns: usize,
    /// Rows or columns past the limits were left out.
    pub cut: bool,
}

/// Why a spreadsheet cannot be shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

/// Reads an Excel or OpenDocument workbook.
///
/// calamine lays a sheet out as one grid from its first cell to its last,
/// so one cell in A1 and one in XFD1048576 would ask for billions of
/// cells and abort the app. Excel 2007 files (xlsx, xlsb) are therefore
/// read cell by cell and cut to the limits here; an Excel 97 file (xls),
/// which calamine lays out in full as it opens it, is refused when a
/// sheet spans more than [`MAX_XLS_SPAN`] cells.
pub fn open(bytes: Vec<u8>) -> Result<Workbook, Invalid> {
    if crate::ole::is_ole(&bytes) && !xls_fits(&bytes) {
        return Err(Invalid);
    }
    let mut book = open_workbook_auto_from_rs(Cursor::new(bytes)).map_err(|_| Invalid)?;
    let mut budget = MAX_CELLS;
    let mut sheets = Vec::new();
    for name in book.sheet_names() {
        let sheet =
            match &mut book {
                Sheets::Xlsx(xlsx) => xlsx.worksheet_cells_reader(&name).ok().map(|mut reader| {
                    sparse(name, &mut budget, || reader.next_cell().ok().flatten())
                }),
                Sheets::Xlsb(xlsb) => xlsb.worksheet_cells_reader(&name).ok().map(|mut reader| {
                    sparse(name, &mut budget, || reader.next_cell().ok().flatten())
                }),
                _ => book
                    .worksheet_range(&name)
                    .ok()
                    .map(|range| dense(name, &mut budget, &range)),
            };
        // A sheet that fails (a chart sheet, say) is skipped.
        sheets.extend(sheet);
    }
    if sheets.is_empty() {
        return Err(Invalid);
    }
    Ok(Workbook { sheets })
}

/// A sheet from a range calamine laid out (xls, ods).
fn dense(name: String, budget: &mut usize, range: &calamine::Range<Data>) -> Sheet {
    let origin = range.start().unwrap_or((0, 0));
    let (height, width) = range.get_size();
    let columns = width.min(MAX_COLUMNS);
    let rows_room = budget.checked_div(columns.max(1)).unwrap_or(0);
    let cut = width > MAX_COLUMNS || height > MAX_ROWS.min(rows_room);
    let rows: Vec<Vec<String>> = range
        .rows()
        .take(MAX_ROWS.min(rows_room))
        .map(|row| row.iter().take(columns).map(cell).collect())
        .collect();
    *budget = budget.saturating_sub(rows.len() * columns);
    Sheet {
        name,
        origin,
        rows,
        columns,
        cut,
    }
}

/// A sheet read one cell at a time from `next`, laid out from its first
/// row and column and cut to the limits, however far apart its cells are.
fn sparse<'a>(
    name: String,
    budget: &mut usize,
    mut next: impl FnMut() -> Option<Cell<DataRef<'a>>>,
) -> Sheet {
    let mut cells = Vec::new();
    let mut cut = false;
    while let Some(c) = next() {
        if matches!(c.get_value(), DataRef::Empty) {
            continue;
        }
        if cells.len() >= MAX_CELLS {
            cut = true;
            break;
        }
        let (row, column) = c.get_position();
        let text = cell(&Data::from(c.get_value().clone()));
        cells.push((row, column, text));
    }
    let top = cells.iter().map(|c| c.0).min().unwrap_or(0);
    let left = cells.iter().map(|c| c.1).min().unwrap_or(0);
    let bottom = cells.iter().map(|c| c.0).max().unwrap_or(0);
    let right = cells.iter().map(|c| c.1).max().unwrap_or(0);
    let height = (bottom - top) as usize + 1;
    let width = (right - left) as usize + 1;
    let columns = width.min(MAX_COLUMNS);
    let rows_room = budget.checked_div(columns).unwrap_or(0);
    let shown = height.min(MAX_ROWS).min(rows_room);
    cut |= width > columns || height > shown;
    let mut rows = vec![vec![String::new(); columns]; shown];
    for (row, column, text) in cells {
        let (r, c) = ((row - top) as usize, (column - left) as usize);
        if let Some(slot) = rows.get_mut(r).and_then(|row| row.get_mut(c)) {
            *slot = text;
        }
    }
    *budget = budget.saturating_sub(rows.len() * columns);
    Sheet {
        name,
        origin: (top, left),
        rows,
        columns,
        cut,
    }
}

/// The most cells an Excel 97 sheet may span, first cell to last (about
/// 128 MB as calamine lays it out).
const MAX_XLS_SPAN: u64 = 4_000_000;

/// Whether calamine can lay out every sheet of the Excel 97 workbook in
/// `bytes` within [`MAX_XLS_SPAN`]: each sheet's cells and its declared
/// dimensions are looked at before calamine allocates for them. Files
/// that are not Excel 97 workbooks are left for calamine to judge.
fn xls_fits(bytes: &[u8]) -> bool {
    let Some(mut ole) = crate::ole::open(bytes.to_vec()) else {
        return true;
    };
    let Some(stream) = ["Workbook", "Book", "WORKBOOK", "BOOK"]
        .into_iter()
        .find_map(|name| crate::ole::stream(&mut ole, name))
    else {
        return true;
    };
    // The workbook's own records name where each sheet starts.
    let mut starts = Vec::new();
    for (kind, data) in records(&stream, 0) {
        match kind {
            // BoundSheet8
            0x0085 => match crate::ole::u32_at(data, 0) {
                Some(at) => starts.push(at as usize),
                None => return false,
            },
            // EOF
            0x000A => break,
            _ => {}
        }
    }
    starts
        .into_iter()
        .all(|at| at <= stream.len() && xls_sheet_fits(&stream, at))
}

/// Whether the sheet whose records start at `at` spans at most
/// [`MAX_XLS_SPAN`] cells. The records read are the ones calamine makes
/// cells (and formulas) of.
fn xls_sheet_fits(stream: &[u8], at: usize) -> bool {
    use crate::ole::{u16_at, u32_at};
    let mut span = Span::default();
    for (kind, data) in records(stream, at) {
        let row = u16_at(data, 0).map(u32::from);
        let column = u16_at(data, 2).map(u32::from);
        match kind {
            // Dimensions: calamine reserves room for all of it.
            0x0200 => {
                let dimensions = match data.len() {
                    10 => (
                        row,
                        column,
                        u16_at(data, 4).map(u32::from),
                        u16_at(data, 6).map(u32::from),
                    ),
                    14 => (
                        u32_at(data, 0),
                        u32_at(data, 4),
                        u16_at(data, 8).map(u32::from),
                        u16_at(data, 10).map(u32::from),
                    ),
                    _ => continue,
                };
                let (Some(first_row), Some(rows_end), Some(first_column), Some(columns_end)) =
                    dimensions
                else {
                    continue;
                };
                // As calamine reads them (`parse_dimensions`); the ends are
                // one past the last row and column.
                let first_column = if first_column > 0xff || columns_end < first_column {
                    0
                } else {
                    first_column
                };
                if rows_end >= 1 && columns_end >= 1 {
                    let rows = (rows_end - 1).checked_sub(first_row);
                    let columns = (columns_end - 1).checked_sub(first_column);
                    match (rows, columns) {
                        (Some(rows), Some(columns))
                            if (u64::from(rows) + 1).saturating_mul(u64::from(columns) + 1)
                                <= MAX_XLS_SPAN => {}
                        _ => return false,
                    }
                }
            }
            // Number, Label, RString, BoolErr, Rk, LabelSst, Formula
            0x0203 | 0x0204 | 0x00D6 | 0x0205 | 0x027E | 0x00FD | 0x0006 => {
                if let (Some(row), Some(column)) = (row, column) {
                    span.add(row, column);
                }
            }
            // MulRk: a run of cells in one row.
            0x00BD => {
                let last = data
                    .len()
                    .checked_sub(2)
                    .and_then(|at| u16_at(data, at))
                    .map(u32::from);
                match (row, column, last) {
                    (Some(row), Some(first), Some(last)) if first <= last => {
                        span.add(row, first);
                        span.add(row, last);
                    }
                    // calamine would get this wrong.
                    (Some(_), Some(_), Some(_)) => return false,
                    _ => {}
                }
            }
            // EOF
            0x000A => break,
            _ => {}
        }
    }
    span.cells() <= MAX_XLS_SPAN
}

/// The rows and columns cells were seen in.
#[derive(Default)]
struct Span(Option<(u32, u32, u32, u32)>);

impl Span {
    fn add(&mut self, row: u32, column: u32) {
        self.0 = Some(match self.0 {
            None => (row, row, column, column),
            Some((top, bottom, left, right)) => (
                top.min(row),
                bottom.max(row),
                left.min(column),
                right.max(column),
            ),
        });
    }

    fn cells(&self) -> u64 {
        self.0.map_or(0, |(top, bottom, left, right)| {
            (u64::from(bottom - top) + 1) * (u64::from(right - left) + 1)
        })
    }
}

/// The BIFF records of `stream` from `at` on, as (type, data), until one
/// runs past the end.
fn records(stream: &[u8], at: usize) -> impl Iterator<Item = (u16, &[u8])> {
    let mut rest = stream.get(at..).unwrap_or_default();
    std::iter::from_fn(move || {
        let kind = crate::ole::u16_at(rest, 0)?;
        let len = usize::from(crate::ole::u16_at(rest, 2)?);
        let data = rest.get(4..4 + len)?;
        rest = &rest[4 + len..];
        Some((kind, data))
    })
}

/// Reads a CSV file (or TSV when `tabs`), guessing the separator: comma,
/// semicolon (common in Europe), tab or bar.
pub fn csv(bytes: &[u8], name: &str, tabs: bool) -> Workbook {
    let (text, mut cut) = crate::text::decode(bytes, MAX_CSV_BYTES);
    let separator = if tabs { '\t' } else { guess_separator(&text) };
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    let mut cells = 0;
    while let Some(c) = chars.next() {
        if quoted {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
                    field.push('"');
                }
                '"' => quoted = false,
                _ => field.push(c),
            }
            continue;
        }
        match c {
            '"' if field.is_empty() => quoted = true,
            '\r' => {}
            '\n' => {
                row.push(clip(std::mem::take(&mut field)));
                cells += row.len();
                rows.push(std::mem::take(&mut row));
                if rows.len() >= MAX_ROWS || cells >= MAX_CELLS {
                    cut |= chars.peek().is_some();
                    break;
                }
            }
            c if c == separator => row.push(clip(std::mem::take(&mut field))),
            _ => field.push(c),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(clip(field));
        rows.push(row);
    }
    let width = rows.iter().map(Vec::len).max().unwrap_or(0);
    let columns = width.clamp(1, MAX_COLUMNS);
    cut |= width > MAX_COLUMNS;
    for row in &mut rows {
        row.resize(columns, String::new());
    }
    if rows.is_empty() {
        rows.push(vec![String::new()]);
    }
    let name = name
        .rsplit_once('.')
        .map_or(name, |(stem, _)| stem)
        .to_owned();
    Workbook {
        sheets: vec![Sheet {
            name,
            origin: (0, 0),
            rows,
            columns,
            cut,
        }],
    }
}

/// The separator that splits the first lines most evenly.
fn guess_separator(text: &str) -> char {
    let sample: Vec<&str> = text.lines().take(20).collect();
    [',', ';', '\t', '|']
        .into_iter()
        .max_by_key(|&sep| {
            let counts: Vec<usize> = sample
                .iter()
                .map(|line| outside_quotes(line, sep))
                .collect();
            let first = counts.first().copied().unwrap_or(0);
            // Lines agreeing with the first count; then how many fields.
            let agree = counts.iter().filter(|&&n| n == first && n > 0).count();
            (agree, first)
        })
        .unwrap_or(',')
}

fn outside_quotes(line: &str, sep: char) -> usize {
    let mut quoted = false;
    line.chars()
        .filter(|&c| {
            if c == '"' {
                quoted = !quoted;
            }
            c == sep && !quoted
        })
        .count()
}

fn clip(text: String) -> String {
    match text.char_indices().nth(MAX_CELL_CHARS) {
        Some((end, _)) => format!("{}…", &text[..end]),
        None => text,
    }
}

/// A cell's value as the spreadsheet app would show it, roughly.
fn cell(data: &Data) -> String {
    match data {
        Data::Empty => String::new(),
        Data::String(text) => clip(text.clone()),
        Data::Int(n) => n.to_string(),
        Data::Float(n) => number(*n),
        Data::Bool(b) => if *b { "TRUE" } else { "FALSE" }.to_owned(),
        Data::DateTime(when) if when.is_duration() => {
            let seconds = (when.as_f64() * 86_400.0).round() as i64;
            let sign = if seconds < 0 { "-" } else { "" };
            let seconds = seconds.unsigned_abs();
            format!(
                "{sign}{}:{:02}:{:02}",
                seconds / 3600,
                seconds / 60 % 60,
                seconds % 60
            )
        }
        Data::DateTime(when) => {
            let (y, mo, d, h, mi, s, _) = when.to_ymd_hms_milli();
            let day = when.as_f64() >= 1.0;
            let time = (h, mi, s) != (0, 0, 0);
            match (day, time) {
                (true, false) => format!("{y:04}-{mo:02}-{d:02}"),
                (false, _) => format!("{h:02}:{mi:02}:{s:02}"),
                (true, true) => format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02}"),
            }
        }
        Data::DateTimeIso(text) | Data::DurationIso(text) => text.clone(),
        Data::Error(err) => err.to_string(),
    }
}

/// A number without floating-point noise: `0.1 + 0.2` shows as 0.3.
fn number(n: f64) -> String {
    if n.is_finite() && n.fract() == 0.0 && n.abs() < 1e15 {
        return format!("{n:.0}");
    }
    if n != 0.0 && !(1e-6..1e15).contains(&n.abs()) {
        return format!("{n:e}");
    }
    let text = format!("{n:.10}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text == "-0" { "0" } else { text }.to_owned()
}

/// The name of column `ix` (from 0): A, B, … Z, AA, AB, …
pub fn column_name(ix: u32) -> String {
    let mut n = ix as u64 + 1;
    let mut name = Vec::new();
    while n > 0 {
        let rem = ((n - 1) % 26) as u8;
        name.push(char::from(b'A' + rem));
        n = (n - 1) / 26;
    }
    name.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_quotes_separators_and_ragged_rows() {
        let book = csv(
            b"name,note,n\r\n\"Doe, Jane\",\"said \"\"hi\"\"\nthen left\",3\nshort\n",
            "people.csv",
            false,
        );
        let sheet = &book.sheets[0];
        assert_eq!(sheet.name, "people");
        assert_eq!(sheet.columns, 3);
        assert_eq!(sheet.rows[0], ["name", "note", "n"]);
        assert_eq!(sheet.rows[1], ["Doe, Jane", "said \"hi\"\nthen left", "3"]);
        assert_eq!(sheet.rows[2], ["short", "", ""]);
        assert_eq!(sheet.rows.len(), 3);
        assert!(!sheet.cut);
    }

    #[test]
    fn csv_separator_is_guessed() {
        let book = csv(b"a;b;c\n1,5;2;3\n", "x.csv", false);
        assert_eq!(book.sheets[0].rows[1], ["1,5", "2", "3"]);
        let book = csv(b"a\tb\n1\t2\n", "x.tsv", true);
        assert_eq!(book.sheets[0].rows[1], ["1", "2"]);
    }

    #[test]
    fn csv_limits() {
        let long = "1,2\n".repeat(MAX_ROWS + 3);
        let book = csv(long.as_bytes(), "x.csv", false);
        assert_eq!(book.sheets[0].rows.len(), MAX_ROWS);
        assert!(book.sheets[0].cut);
    }

    #[test]
    fn numbers_and_columns() {
        assert_eq!(number(0.1 + 0.2), "0.3");
        assert_eq!(number(42.0), "42");
        assert_eq!(number(-2.5), "-2.5");
        assert_eq!(number(1e20), "1e20");
        assert_eq!(column_name(0), "A");
        assert_eq!(column_name(25), "Z");
        assert_eq!(column_name(26), "AA");
        assert_eq!(column_name(701), "ZZ");
        assert_eq!(column_name(702), "AAA");
    }

    #[test]
    fn workbooks() {
        let book = open(crate::tests::xlsx()).unwrap();
        assert_eq!(book.sheets.len(), 2);
        let first = &book.sheets[0];
        assert_eq!(first.name, "Budget");
        assert_eq!(first.origin, (0, 0));
        assert_eq!(first.rows[0], ["Item", "Cost"]);
        assert_eq!(first.rows[1], ["Paper", "12.5"]);
        assert_eq!(first.rows[2], ["Total", "12.5"]);
        assert_eq!(book.sheets[1].name, "Notes");
        assert_eq!(open(b"not a spreadsheet".to_vec()), Err(Invalid));
    }

    /// An xlsx workbook of one sheet holding `cells`, as (reference, text).
    fn one_sheet(cells: &[(&str, &str)]) -> Vec<u8> {
        let cells: String = cells
            .iter()
            .map(|(at, text)| format!(r#"<c r="{at}" t="inlineStr"><is><t>{text}</t></is></c>"#))
            .collect();
        let sheet = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row>{cells}</row></sheetData></worksheet>"#
        );
        crate::tests::zip(&[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#,
            ),
            (
                "_rels/.rels",
                r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Far" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", &sheet),
        ])
    }

    /// The attack of the September 2026 audit: two cells at opposite
    /// corners of the largest sheet Excel allows. Laid out whole that is
    /// 17 billion cells; here it is cut to the limits at once.
    #[test]
    fn far_apart_cells_are_cut_not_laid_out() {
        let book = open(one_sheet(&[("A1", "near"), ("XFD1048576", "far")])).unwrap();
        let sheet = &book.sheets[0];
        assert_eq!(sheet.origin, (0, 0));
        assert_eq!(sheet.columns, MAX_COLUMNS);
        assert_eq!(sheet.rows.len(), MAX_CELLS / MAX_COLUMNS);
        assert_eq!(sheet.rows[0][0], "near");
        assert!(sheet.cut);
        assert!(sheet.rows.iter().flatten().all(|c| c != "far"));
    }

    #[test]
    fn sparse_sheets_start_at_their_first_cell() {
        let book = open(one_sheet(&[("C5", "a"), ("E7", "b")])).unwrap();
        let sheet = &book.sheets[0];
        assert_eq!(sheet.origin, (4, 2));
        assert_eq!(sheet.columns, 3);
        assert_eq!(sheet.rows.len(), 3);
        assert_eq!(sheet.rows[0], ["a", "", ""]);
        assert_eq!(sheet.rows[2], ["", "", "b"]);
        assert!(!sheet.cut);
    }

    /// An Excel 97 workbook whose one sheet holds `records` (type, data).
    fn xls(records: &[(u16, Vec<u8>)]) -> Vec<u8> {
        use std::io::Write;
        let record = |kind: u16, data: &[u8]| {
            let mut out = kind.to_le_bytes().to_vec();
            out.extend((data.len() as u16).to_le_bytes());
            out.extend(data);
            out
        };
        let bof = record(0x0809, &[0, 6, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mut sheet_bof = bof.clone();
        sheet_bof[6] = 0x10;
        let eof = record(0x000A, &[]);
        // BoundSheet8: where the sheet starts, visible, a worksheet, "Far".
        let start = (bof.len() + 4 + 11 + eof.len()) as u32;
        let mut bound = start.to_le_bytes().to_vec();
        bound.extend([0, 0, 3, 0, b'F', b'a', b'r']);
        let mut stream = bof;
        stream.extend(record(0x0085, &bound));
        stream.extend(&eof);
        assert_eq!(stream.len() as u32, start);
        stream.extend(sheet_bof);
        for (kind, data) in records {
            stream.extend(record(*kind, data));
        }
        stream.extend(eof);
        let mut file = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        file.create_stream("/Workbook")
            .unwrap()
            .write_all(&stream)
            .unwrap();
        file.flush().unwrap();
        file.into_inner().into_inner()
    }

    fn number_record(row: u16, column: u16) -> (u16, Vec<u8>) {
        let mut data = row.to_le_bytes().to_vec();
        data.extend(column.to_le_bytes());
        data.extend([0, 0]);
        data.extend(1.0f64.to_le_bytes());
        (0x0203, data)
    }

    #[test]
    fn excel_97_sheets_too_wide_to_lay_out_are_refused() {
        assert!(xls_fits(&xls(&[number_record(0, 0), number_record(10, 3)])));
        let far = xls(&[number_record(0, 0), number_record(65_535, 65_535)]);
        assert!(!xls_fits(&far));
        assert_eq!(open(far), Err(Invalid));
        // Declared dimensions alone make calamine reserve room.
        let mut dimensions = 0u32.to_le_bytes().to_vec();
        dimensions.extend(4_000_000u32.to_le_bytes());
        dimensions.extend([0, 0, 0xff, 0, 0, 0]);
        assert!(!xls_fits(&xls(&[(0x0200, dimensions)])));
    }
}
