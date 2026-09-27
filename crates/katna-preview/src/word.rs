// SPDX-License-Identifier: GPL-3.0-or-later

//! Word 97–2003 documents (.doc) for the viewer. A .doc is an OLE compound
//! file: the "WordDocument" stream holds the text and its formatting, and
//! the "0Table" or "1Table" stream says where each piece of text and each
//! run of formatting is ([MS-DOC]). This reads the main text with its
//! headings, lists, tables, alignment and bold, italic, underline and
//! strike-through, into the same [`Document`] as docx. Pictures, headers,
//! footers, notes, comments and field codes are left out (a field's result,
//! such as a link's text, is kept). Encrypted files and Word 6/95 files are
//! not read.

use std::collections::HashMap;

use crate::document::{Align, Builder, Counters, Document, Invalid, MAX_CHARS, Run, Style};
use crate::ole::{self, cp1252, symbol, u8_at, u16_at, u32_at};

/// Where the FIB lists the table-stream structures this reads.
const STSHF: usize = 1;
const PLCF_BTE_CHPX: usize = 12;
const PLCF_BTE_PAPX: usize = 13;
const CLX: usize = 33;
const PLF_LST: usize = 73;
const PLF_LFO: usize = 74;

/// Formatting pages are this long.
const PAGE: usize = 512;

/// Reads a .doc file.
pub fn open(bytes: Vec<u8>) -> Result<Document, Invalid> {
    let mut file = ole::open(bytes).ok_or(Invalid)?;
    let word = ole::stream(&mut file, "WordDocument").ok_or(Invalid)?;
    let fib = Fib::read(&word).ok_or(Invalid)?;
    let table_name = if fib.table_1 { "1Table" } else { "0Table" };
    let table = ole::stream(&mut file, table_name).ok_or(Invalid)?;
    let part = |ix: usize| -> &[u8] {
        fib.fc_lcb
            .get(ix)
            .and_then(|&(fc, lcb)| table.get(fc as usize..fc as usize + lcb as usize))
            .unwrap_or_default()
    };
    let pieces = pieces(part(CLX)).ok_or(Invalid)?;
    let chars = characters(&word, &pieces, fib.ccp_text);
    if chars.is_empty() && fib.ccp_text > 0 {
        return Err(Invalid);
    }
    let looks = Spans::read(&word, part(PLCF_BTE_CHPX), chpx_page);
    let paragraphs = Spans::read(&word, part(PLCF_BTE_PAPX), papx_page);
    let styles = styles(part(STSHF));
    // The list levels follow the list table rather than being counted in it.
    let lists_and_levels = fib
        .fc_lcb
        .get(PLF_LST)
        .filter(|&&(_, lcb)| lcb > 0)
        .and_then(|&(fc, _)| table.get(fc as usize..))
        .unwrap_or_default();
    let lists = Lists::read(lists_and_levels, part(PLF_LFO));
    Ok(build(&chars, &looks, &paragraphs, &styles, &lists))
}

/// The File Information Block at the start of "WordDocument": which table
/// stream to use, how long the main text is, and where things are.
struct Fib {
    table_1: bool,
    ccp_text: u32,
    fc_lcb: Vec<(u32, u32)>,
}

impl Fib {
    fn read(w: &[u8]) -> Option<Fib> {
        if u16_at(w, 0)? != 0xa5ec {
            return None;
        }
        // Word 6 and 95 files have an older layout.
        if u16_at(w, 2)? < 0x00c0 {
            return None;
        }
        let flags = u16_at(w, 0x0a)?;
        if flags & 0x0100 != 0 {
            // Encrypted or obfuscated.
            return None;
        }
        let mut at = 32;
        let csw = usize::from(u16_at(w, at)?);
        at += 2 + csw * 2;
        let cslw = usize::from(u16_at(w, at)?);
        at += 2;
        let ccp_text = u32_at(w, at + 12)?;
        at += cslw * 4;
        let pairs = usize::from(u16_at(w, at)?);
        at += 2;
        let fc_lcb = (0..pairs)
            .map_while(|i| Some((u32_at(w, at + i * 8)?, u32_at(w, at + i * 8 + 4)?)))
            .collect();
        Some(Fib {
            table_1: flags & 0x0200 != 0,
            ccp_text,
            fc_lcb,
        })
    }
}

/// A stretch of text in one place of "WordDocument".
struct Piece {
    cp: u32,
    cp_end: u32,
    /// Byte offset of the first character.
    fc: u32,
    /// One byte (Windows-1252) a character, not two (UTF-16).
    compressed: bool,
}

/// The piece table from the CLX: character positions to file offsets.
fn pieces(clx: &[u8]) -> Option<Vec<Piece>> {
    let mut at = 0;
    loop {
        match u8_at(clx, at)? {
            // A Prc: formatting of pieces, not needed here.
            1 => at += 3 + usize::from(u16_at(clx, at + 1)?),
            2 => {
                let lcb = u32_at(clx, at + 1)? as usize;
                let plc = clx.get(at + 5..at + 5 + lcb)?;
                let n = lcb.checked_sub(4)? / 12;
                let mut pieces = Vec::with_capacity(n);
                for i in 0..n {
                    let raw = u32_at(plc, 4 * (n + 1) + 8 * i + 2)?;
                    let compressed = raw & 0x4000_0000 != 0;
                    let fc = raw & 0x3fff_ffff;
                    pieces.push(Piece {
                        cp: u32_at(plc, 4 * i)?,
                        cp_end: u32_at(plc, 4 * (i + 1))?,
                        fc: if compressed { fc / 2 } else { fc },
                        compressed,
                    });
                }
                return Some(pieces);
            }
            _ => return None,
        }
    }
}

/// A character of the main text and where it is in the file, which is
/// what formatting is keyed on.
#[derive(Clone, Copy)]
struct Char {
    unit: u16,
    fc: u32,
}

fn characters(word: &[u8], pieces: &[Piece], ccp_text: u32) -> Vec<Char> {
    let mut out = Vec::new();
    for piece in pieces {
        let end = piece.cp_end.min(ccp_text);
        for cp in piece.cp..end {
            if out.len() > MAX_CHARS {
                return out;
            }
            let offset = cp - piece.cp;
            let (fc, unit) = if piece.compressed {
                let fc = piece.fc + offset;
                let Some(byte) = u8_at(word, fc as usize) else {
                    return out;
                };
                (fc, cp1252(byte) as u16)
            } else {
                let fc = piece.fc + offset * 2;
                let Some(unit) = u16_at(word, fc as usize) else {
                    return out;
                };
                (fc, unit)
            };
            out.push(Char { unit, fc });
        }
    }
    out
}

/// Formatting by file offset: sorted, non-overlapping ranges.
struct Spans<T>(Vec<Span<T>>);

/// Formatting `T` from one file offset up to another.
type Span<T> = (u32, u32, T);

/// Reads the spans of one formatting page.
type PageReader<T> = fn(&[u8], &mut Vec<Span<T>>);

impl<T: Clone + Default> Spans<T> {
    /// Reads the formatting pages a PlcBte lists, each with `page`.
    fn read(word: &[u8], plc: &[u8], page: PageReader<T>) -> Spans<T> {
        let mut spans = Vec::new();
        let n = plc.len().saturating_sub(4) / 8;
        for i in 0..n {
            let Some(pn) = u32_at(plc, 4 * (n + 1) + 4 * i) else {
                break;
            };
            let start = (pn & 0x3f_ffff) as usize * PAGE;
            if let Some(bytes) = word.get(start..start + PAGE) {
                page(bytes, &mut spans);
            }
        }
        spans.sort_by_key(|s| s.0);
        Spans(spans)
    }

    fn at(&self, fc: u32) -> T {
        let ix = self.0.partition_point(|s| s.0 <= fc);
        match ix.checked_sub(1).and_then(|ix| self.0.get(ix)) {
            Some((_, end, props)) if fc < *end => props.clone(),
            _ => T::default(),
        }
    }
}

/// How a run of characters looks.
#[derive(Clone, Copy, Default, PartialEq)]
struct Look {
    bold: bool,
    italic: bool,
    underline: bool,
    strike: bool,
    hidden: bool,
}

fn chpx_page(page: &[u8], out: &mut Vec<Span<Look>>) {
    let runs = usize::from(page[PAGE - 1]);
    for i in 0..runs {
        let (Some(start), Some(end), Some(offset)) = (
            u32_at(page, 4 * i),
            u32_at(page, 4 * (i + 1)),
            u8_at(page, 4 * (runs + 1) + i),
        ) else {
            return;
        };
        let mut look = Look::default();
        if offset != 0 {
            let at = usize::from(offset) * 2;
            let cb = usize::from(page.get(at).copied().unwrap_or(0));
            let grpprl = page.get(at + 1..at + 1 + cb).unwrap_or_default();
            each_sprm(grpprl, |sprm, operand| {
                let on = || matches!(operand.first(), Some(1 | 0x81));
                match sprm {
                    0x0835 => look.bold = on(),
                    0x0836 => look.italic = on(),
                    0x0837 | 0x2a53 => look.strike = on(),
                    0x2a3e => look.underline = operand.first().is_some_and(|k| *k != 0),
                    0x083c => look.hidden = on(),
                    _ => {}
                }
            });
        }
        out.push((start, end, look));
    }
}

/// What a paragraph mark says about its paragraph.
#[derive(Clone, Copy, Default)]
struct Para {
    istd: u16,
    in_table: bool,
    /// The mark ends a table row rather than a cell.
    row_end: bool,
    align: Align,
    /// List (1-based LFO index) and level.
    list: Option<(u16, u8)>,
    /// An outline level set on the paragraph itself (0 = Heading 1).
    outline: Option<u8>,
}

fn papx_page(page: &[u8], out: &mut Vec<Span<Para>>) {
    let count = usize::from(page[PAGE - 1]);
    for i in 0..count {
        let (Some(start), Some(end), Some(offset)) = (
            u32_at(page, 4 * i),
            u32_at(page, 4 * (i + 1)),
            u8_at(page, 4 * (count + 1) + 13 * i),
        ) else {
            return;
        };
        let at = usize::from(offset) * 2;
        let data = match page.get(at).copied() {
            Some(0) => {
                let cb = usize::from(page.get(at + 1).copied().unwrap_or(0));
                page.get(at + 2..at + 2 + 2 * cb)
            }
            Some(cb) => page.get(at + 1..at + 2 * usize::from(cb)),
            None => None,
        }
        .unwrap_or_default();
        let mut para = Para {
            istd: u16_at(data, 0).unwrap_or(0),
            ..Para::default()
        };
        let (mut ilfo, mut ilvl) = (0u16, 0u8);
        each_sprm(data.get(2..).unwrap_or_default(), |sprm, operand| {
            let byte = operand.first().copied().unwrap_or(0);
            match sprm {
                0x2416 => para.in_table = byte != 0,
                0x2417 => para.row_end = byte != 0,
                0x6649 => para.in_table |= u32_at(operand, 0).unwrap_or(0) > 0,
                0x2403 | 0x2461 => {
                    para.align = match byte {
                        1 => Align::Center,
                        2 => Align::End,
                        _ => Align::Start,
                    }
                }
                0x260a => ilvl = byte.min(8),
                0x460b => ilfo = u16_at(operand, 0).unwrap_or(0),
                0x2640 => para.outline = (byte < 9).then_some(byte),
                _ => {}
            }
        });
        // 0xF801 and up are Word's "no list" markers.
        if ilfo != 0 && ilfo < 0xf801 {
            para.list = Some((ilfo, ilvl));
        }
        out.push((start, end, para));
    }
}

/// Calls `f` with each property modifier (sprm) and its operand.
fn each_sprm(grpprl: &[u8], mut f: impl FnMut(u16, &[u8])) {
    let mut at = 0;
    while let Some(sprm) = u16_at(grpprl, at) {
        at += 2;
        let len = match sprm >> 13 {
            0 | 1 => 1,
            2 | 4 | 5 => 2,
            3 => 4,
            7 => 3,
            // Variable length: a size first (two bytes for table
            // definitions, counting itself less one).
            _ if sprm == 0xd608 => {
                2 + usize::from(u16_at(grpprl, at).unwrap_or(0)).saturating_sub(1)
            }
            _ => 1 + usize::from(u8_at(grpprl, at).unwrap_or(0)),
        };
        let Some(operand) = grpprl.get(at..at + len) else {
            return;
        };
        f(sprm, operand);
        at += len;
    }
}

/// Paragraph styles by index, from the style sheet.
fn styles(stsh: &[u8]) -> Vec<Style> {
    let mut out = Vec::new();
    let (Some(cb_stshi), Some(count), Some(cb_base)) =
        (u16_at(stsh, 0), u16_at(stsh, 2), u16_at(stsh, 4))
    else {
        return out;
    };
    let mut at = 2 + usize::from(cb_stshi);
    for _ in 0..count {
        let Some(cb) = u16_at(stsh, at) else {
            break;
        };
        let std = stsh
            .get(at + 2..at + 2 + usize::from(cb))
            .unwrap_or_default();
        at += 2 + usize::from(cb);
        let sti = u16_at(std, 0).unwrap_or(0) & 0x0fff;
        let style = match sti {
            1..=9 => Style::Heading(sti.min(6) as u8),
            62 => Style::Title,
            74 => Style::Subtitle,
            _ => style_named(&style_name(std, usize::from(cb_base))),
        };
        out.push(style);
    }
    out
}

fn style_name(std: &[u8], base: usize) -> String {
    let Some(len) = u16_at(std, base) else {
        return String::new();
    };
    let units: Vec<u16> = (0..usize::from(len))
        .map_while(|i| u16_at(std, base + 2 + 2 * i))
        .collect();
    String::from_utf16_lossy(&units)
}

/// Styles other writers name but do not mark as Word's built-in ones.
fn style_named(name: &str) -> Style {
    let name = name.to_ascii_lowercase();
    match name.as_str() {
        "title" => Style::Title,
        "subtitle" => Style::Subtitle,
        _ => match name
            .strip_prefix("heading ")
            .and_then(|n| n.parse::<u8>().ok())
        {
            Some(n @ 1..=9) => Style::Heading(n.min(6)),
            _ => Style::Normal,
        },
    }
}

/// Numbered and bulleted lists: each list's levels, and the list each
/// paragraph's list index (LFO) points at.
#[derive(Default)]
struct Lists {
    lfo: Vec<i32>,
    lists: HashMap<i32, Vec<Level>>,
}

struct Level {
    start: u32,
    /// Number format (0 = 1, 2, 3; 23 = bullet; ...).
    nfc: u8,
    /// The number text: characters below 9 stand for that level's number.
    text: Vec<u16>,
}

impl Lists {
    fn read(lst: &[u8], lfo: &[u8]) -> Lists {
        let mut out = Lists::default();
        let count = u16_at(lst, 0).unwrap_or(0) as i16;
        let mut ids = Vec::new();
        for i in 0..usize::try_from(count).unwrap_or(0) {
            let at = 2 + 28 * i;
            let (Some(id), Some(flags)) = (u32_at(lst, at), u8_at(lst, at + 26)) else {
                return out;
            };
            ids.push((id as i32, if flags & 1 != 0 { 1 } else { 9 }));
        }
        let mut at = 2 + 28 * ids.len();
        for (id, count) in ids {
            let mut levels = Vec::new();
            for _ in 0..count {
                let (Some(start), Some(nfc), Some(chpx), Some(papx)) = (
                    u32_at(lst, at),
                    u8_at(lst, at + 4),
                    u8_at(lst, at + 24),
                    u8_at(lst, at + 25),
                ) else {
                    return out;
                };
                at += 28 + usize::from(chpx) + usize::from(papx);
                let len = usize::from(u16_at(lst, at).unwrap_or(0));
                let text = (0..len)
                    .map_while(|i| u16_at(lst, at + 2 + 2 * i))
                    .collect();
                at += 2 + 2 * len;
                levels.push(Level { start, nfc, text });
            }
            out.lists.insert(id, levels);
        }
        let count = u32_at(lfo, 0).unwrap_or(0) as usize;
        out.lfo = (0..count.min(lfo.len() / 16))
            .map_while(|i| u32_at(lfo, 4 + 16 * i).map(|id| id as i32))
            .collect();
        out
    }

    /// The marker of the next item of list `ilfo` at `level`.
    fn marker(&self, counters: &mut Counters, ilfo: u16, level: u8) -> String {
        let id = self.lfo.get(usize::from(ilfo) - 1);
        let levels = id.and_then(|id| self.lists.get(id));
        // Outline numbering on headings often has a level with no number.
        if levels
            .and_then(|ls| ls.get(usize::from(level)))
            .is_some_and(|l| l.nfc == 255 || l.text.is_empty())
        {
            return String::new();
        }
        let key = id.map_or_else(|| format!("lfo{ilfo}"), |id| id.to_string());
        counters.next(&key, level, &|l| {
            let Some(level) = levels.and_then(|ls| ls.get(usize::from(l))) else {
                return ("bullet".to_owned(), String::new(), 1);
            };
            let format = match level.nfc {
                1 => "upperRoman",
                2 => "lowerRoman",
                3 => "upperLetter",
                4 => "lowerLetter",
                23 => "bullet",
                255 => "none",
                _ => "decimal",
            };
            // Word's number text uses character n for "level n's number";
            // docx's is "%n+1", which the shared counter fills in.
            let mut text = String::new();
            for unit in &level.text {
                match *unit {
                    n @ 0..=8 => text.push_str(&format!("%{}", n + 1)),
                    u => text.push(symbol(char::from_u32(u32::from(u)).unwrap_or('•'))),
                }
            }
            if format == "bullet" && !text.trim().is_empty() {
                // The bullet the list asks for, rather than a generic one.
                return ("decimal".to_owned(), text, level.start);
            }
            (format.to_owned(), text, level.start)
        })
    }
}

/// Lays the characters out as paragraphs and tables.
fn build(
    chars: &[Char],
    looks: &Spans<Look>,
    paragraphs: &Spans<Para>,
    styles: &[Style],
    lists: &Lists,
) -> Document {
    let mut out = Builder::default();
    let mut counters = Counters::default();
    // Open fields: whether each has reached its result yet.
    let mut fields: Vec<bool> = Vec::new();
    let mut in_table = false;
    // The last cell ended; the next paragraph starts a new one.
    let mut cell_done = false;
    let mut start = 0;
    while start < chars.len() {
        if out.full() {
            out.doc.cut = true;
            break;
        }
        // A paragraph runs to its mark: a paragraph or section end, or a
        // table cell or row end.
        let end = chars[start..]
            .iter()
            .position(|c| matches!(c.unit, 0x0d | 0x07 | 0x0c))
            .map_or(chars.len(), |p| start + p);
        let para = chars
            .get(end)
            .map_or_else(Para::default, |c| paragraphs.at(c.fc));
        if para.in_table != in_table {
            if para.in_table {
                out.start_table();
                out.start_row();
            } else {
                out.end_table();
            }
            in_table = para.in_table;
            cell_done = false;
        }
        if in_table && para.row_end {
            // The row's own mark, with nothing to show.
            out.start_row();
            cell_done = false;
            start = end + 1;
            continue;
        }
        if cell_done {
            out.start_cell(1);
            cell_done = false;
        }
        out.start_paragraph();
        if let Some(p) = out.paragraph.as_mut() {
            p.style = match para.outline {
                Some(level) => Style::Heading(level.min(5) + 1),
                None => styles
                    .get(usize::from(para.istd))
                    .copied()
                    .unwrap_or_default(),
            };
            p.align = para.align;
            if let Some((ilfo, level)) = para.list {
                let marker = lists.marker(&mut counters, ilfo, level);
                p.list = (!marker.is_empty()).then_some((level, marker));
            }
        }
        let mut text = String::new();
        let mut look = Look::default();
        let mut high = None;
        for c in &chars[start..end] {
            match c.unit {
                0x13 => fields.push(false),
                0x14 => {
                    if let Some(result) = fields.last_mut() {
                        *result = true;
                    }
                }
                0x15 => {
                    fields.pop();
                }
                _ => {}
            }
            // Field marks, and a field's code up to its result: not shown.
            if matches!(c.unit, 0x13..=0x15) || fields.iter().any(|result| !result) {
                continue;
            }
            let shown = match c.unit {
                0x09 => Some('\t'),
                0x0b => Some('\n'),
                0x1e => Some('-'),
                // Pictures, drawn objects, note and comment marks, optional
                // hyphens and other control characters.
                0x00..=0x1f => None,
                u @ 0xd800..=0xdbff => {
                    high = Some(u);
                    None
                }
                u @ 0xdc00..=0xdfff => high
                    .take()
                    .and_then(|h| char::decode_utf16([h, u]).next()?.ok()),
                u => char::from_u32(u32::from(u)),
            };
            let Some(shown) = shown else {
                continue;
            };
            let here = looks.at(c.fc);
            if here.hidden {
                continue;
            }
            if here != look && !text.is_empty() {
                out.text(&text, &run(look));
                text.clear();
            }
            look = here;
            text.push(shown);
        }
        if !text.is_empty() {
            out.text(&text, &run(look));
        }
        out.end_paragraph();
        if in_table && chars.get(end).is_some_and(|c| c.unit == 0x07) {
            cell_done = true;
        }
        start = end + 1;
    }
    out.finish()
}

fn run(look: Look) -> Run {
    Run {
        text: String::new(),
        bold: look.bold,
        italic: look.italic,
        underline: look.underline,
        strike: look.strike,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Block, Paragraph};

    #[test]
    fn word_97_documents() {
        let doc = open(crate::tests::doc()).unwrap();
        let Block::Paragraph(heading) = &doc.blocks[0] else {
            panic!("{doc:?}");
        };
        assert_eq!(heading.style, Style::Heading(1));
        assert_eq!(heading.text(), "Report");
        let Block::Paragraph(body) = &doc.blocks[1] else {
            panic!("{doc:?}");
        };
        // The link field shows its result, not its code.
        assert_eq!(body.text(), "Hello bold world link \u{1f600}");
        assert!(body.runs.iter().any(|r| r.bold && r.text == "bold"));
        assert!(body.runs.iter().all(|r| !r.bold || r.text == "bold"));
        let Block::Table(rows) = &doc.blocks[2] else {
            panic!("{doc:?}");
        };
        let cells: Vec<Vec<String>> = rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| c.iter().map(Paragraph::text).collect())
                    .collect()
            })
            .collect();
        assert_eq!(cells, [["A", "B"]]);
        let Block::Paragraph(last) = &doc.blocks[3] else {
            panic!("{doc:?}");
        };
        assert_eq!(last.text(), "Done");
        assert_eq!(doc.blocks.len(), 4);
    }

    #[test]
    fn documents_open_from_any_word_format() {
        // A .doc goes through the same entry point as docx and odt.
        assert!(crate::document::open(crate::tests::doc()).is_ok());
    }

    #[test]
    fn broken_and_foreign_files_are_invalid() {
        assert_eq!(open(b"not a doc".to_vec()), Err(Invalid));
        assert_eq!(open(crate::tests::ppt()), Err(Invalid));
        let mut doc = crate::tests::doc();
        doc.truncate(doc.len() / 3);
        assert!(open(doc).is_err());
    }

    #[test]
    fn encrypted_documents_are_not_read() {
        let mut file = ole::open(crate::tests::doc()).unwrap();
        let mut word = ole::stream(&mut file, "WordDocument").unwrap();
        let table = ole::stream(&mut file, "0Table").unwrap();
        // fEncrypted, bit 8 of the flags.
        word[0x0b] |= 0x01;
        let bytes = crate::tests::ole(&[("WordDocument", word), ("0Table", table)]);
        assert_eq!(open(bytes), Err(Invalid));
    }

    #[test]
    fn toggles_and_windows_1252() {
        assert_eq!(cp1252(0x93), '\u{201c}');
        assert_eq!(cp1252(b'a'), 'a');
        let mut spans = Vec::new();
        let mut page = vec![0u8; PAGE];
        page[PAGE - 1] = 1;
        page[4] = 10;
        page[8] = 0x80;
        page[0x100..0x107].copy_from_slice(&[6, 0x35, 0x08, 0x81, 0x36, 0x08, 0x80]);
        chpx_page(&page, &mut spans);
        let (_, _, look) = spans[0];
        assert!(look.bold);
        assert!(!look.italic);
    }
}
