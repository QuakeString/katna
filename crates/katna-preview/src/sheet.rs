// SPDX-License-Identifier: GPL-3.0-or-later

//! Spreadsheets for the viewer: Excel (xlsx, xlsm, xlsb, xls) and
//! OpenDocument (ods) workbooks read by calamine, and CSV/TSV files. Only
//! the values are kept (formulas show their last result), as text.

use std::io::Cursor;

use calamine::{Data, Reader, open_workbook_auto_from_rs};

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
pub fn open(bytes: Vec<u8>) -> Result<Workbook, Invalid> {
    let mut book = open_workbook_auto_from_rs(Cursor::new(bytes)).map_err(|_| Invalid)?;
    let mut budget = MAX_CELLS;
    let mut sheets = Vec::new();
    for name in book.sheet_names() {
        // A sheet that fails (a chart sheet, say) is skipped.
        let Ok(range) = book.worksheet_range(&name) else {
            continue;
        };
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
        budget = budget.saturating_sub(rows.len() * columns);
        sheets.push(Sheet {
            name,
            origin,
            rows,
            columns,
            cut,
        });
    }
    if sheets.is_empty() {
        return Err(Invalid);
    }
    Ok(Workbook { sheets })
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
}
