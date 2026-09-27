// SPDX-License-Identifier: GPL-3.0-or-later

//! The document a [`RichEditor`](super::RichEditor) edits: paragraphs of
//! styled text, tables and images, and the edits on them. No GPUI here, so
//! it is tested on its own.
//!
//! Every place text lives is a [`Para`]: a top-level paragraph or a table
//! cell. A [`Path`] names one and a [`Pos`] is a byte offset in one. Top
//! paragraphs hold no newline (Enter splits them); a cell's text may hold
//! `\n` for its line breaks.

use std::ops::Range;
use std::sync::Arc;

use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

/// The typefaces offered in the font menu. The HTML names them by family;
/// the editor draws them with the closest installed font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Font {
    #[default]
    Sans,
    Serif,
    Fixed,
    Wide,
    Narrow,
    ComicSans,
    Garamond,
    Georgia,
    Tahoma,
    Trebuchet,
    Verdana,
}

impl Font {
    pub const ALL: [Font; 11] = [
        Font::Sans,
        Font::Serif,
        Font::Fixed,
        Font::Wide,
        Font::Narrow,
        Font::ComicSans,
        Font::Garamond,
        Font::Georgia,
        Font::Tahoma,
        Font::Trebuchet,
        Font::Verdana,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Font::Sans => "Sans Serif",
            Font::Serif => "Serif",
            Font::Fixed => "Fixed Width",
            Font::Wide => "Wide",
            Font::Narrow => "Narrow",
            Font::ComicSans => "Comic Sans MS",
            Font::Garamond => "Garamond",
            Font::Georgia => "Georgia",
            Font::Tahoma => "Tahoma",
            Font::Trebuchet => "Trebuchet MS",
            Font::Verdana => "Verdana",
        }
    }

    /// The CSS `font-family` of the sent HTML.
    pub fn css(self) -> &'static str {
        match self {
            Font::Sans => "arial,sans-serif",
            Font::Serif => "\"times new roman\",serif",
            Font::Fixed => "monospace,monospace",
            Font::Wide => "\"arial black\",sans-serif",
            Font::Narrow => "\"arial narrow\",sans-serif",
            Font::ComicSans => "\"comic sans ms\",sans-serif",
            Font::Garamond => "garamond,\"times new roman\",serif",
            Font::Georgia => "georgia,serif",
            Font::Tahoma => "tahoma,sans-serif",
            Font::Trebuchet => "\"trebuchet ms\",sans-serif",
            Font::Verdana => "verdana,sans-serif",
        }
    }

    /// Installed families to draw it with, best first. Empty means the
    /// editor's own font.
    pub fn families(self) -> &'static [&'static str] {
        match self {
            Font::Sans => &[],
            Font::Serif => &[
                "Times New Roman",
                "Liberation Serif",
                "Noto Serif",
                "DejaVu Serif",
                "Tinos",
            ],
            Font::Fixed => &[
                "Liberation Mono",
                "Noto Sans Mono",
                "DejaVu Sans Mono",
                "Cousine",
                "Monospace",
            ],
            Font::Wide => &["Arial Black", "Noto Sans Black", "DejaVu Sans"],
            Font::Narrow => &[
                "Arial Narrow",
                "Liberation Sans Narrow",
                "DejaVu Sans Condensed",
                "Noto Sans Condensed",
            ],
            Font::ComicSans => &["Comic Sans MS", "Comic Neue", "Comic Relief"],
            Font::Garamond => &["Garamond", "EB Garamond", "Cormorant Garamond"],
            Font::Georgia => &["Georgia", "Gelasio", "Noto Serif"],
            Font::Tahoma => &["Tahoma", "Wine Tahoma", "DejaVu Sans"],
            Font::Trebuchet => &["Trebuchet MS", "Fira Sans", "Ubuntu"],
            Font::Verdana => &["Verdana", "DejaVu Sans", "Noto Sans"],
        }
    }
}

/// Text sizes of the size menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Size {
    Small,
    #[default]
    Normal,
    Large,
    Huge,
}

impl Size {
    pub const ALL: [Size; 4] = [Size::Small, Size::Normal, Size::Large, Size::Huge];

    pub fn label(self) -> &'static str {
        match self {
            Size::Small => "Small",
            Size::Normal => "Normal",
            Size::Large => "Large",
            Size::Huge => "Huge",
        }
    }

    /// Size relative to the editor's text.
    pub fn scale(self) -> f32 {
        match self {
            Size::Small => 0.8,
            Size::Normal => 1.0,
            Size::Large => 1.3,
            Size::Huge => 2.0,
        }
    }

    /// The CSS `font-size` of the sent HTML; none for normal text.
    pub fn css(self) -> Option<&'static str> {
        match self {
            Size::Small => Some("x-small"),
            Size::Normal => None,
            Size::Large => Some("large"),
            Size::Huge => Some("xx-large"),
        }
    }
}

/// How a run of characters looks. Colors are `0xRRGGBB`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CharStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub font: Font,
    pub size: Size,
    pub color: Option<u32>,
    pub background: Option<u32>,
    /// The address the text links to.
    pub link: Option<Arc<str>>,
}

impl CharStyle {
    /// Plain text: no formatting at all.
    pub fn is_plain(&self) -> bool {
        *self == CharStyle::default()
    }
}

/// A stretch of a paragraph's text in one style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub len: usize,
    pub style: CharStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum List {
    #[default]
    None,
    Bullet,
    Numbered,
}

/// How a paragraph is laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParaStyle {
    pub align: Align,
    pub list: List,
    /// Indent steps (list nesting for list items).
    pub indent: u8,
    /// Quote depth.
    pub quote: u8,
    /// Part of the signature, which the signature menu swaps as a whole.
    pub signature: bool,
    /// The background of a table cell, `0xRRGGBB`.
    pub fill: Option<u32>,
}

pub const MAX_INDENT: u8 = 8;

/// A paragraph, or the content of a table cell.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Para {
    pub text: String,
    /// Cover `text` exactly, in order; none when it is empty.
    pub runs: Vec<Run>,
    pub style: ParaStyle,
}

/// How large an image shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ImageSize {
    Small,
    #[default]
    BestFit,
    Original,
}

/// A picture in the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    /// Unique within the editor; names the image in the message.
    pub id: u64,
    pub name: String,
    pub mime: String,
    pub data: Arc<Vec<u8>>,
    /// Natural size in pixels, if it could be read.
    pub width: u32,
    pub height: u32,
    pub size: ImageSize,
}

impl Image {
    /// Width it shows at when `room` pixels are free.
    pub fn display_width(&self, room: f32) -> f32 {
        let natural = if self.width == 0 {
            room
        } else {
            self.width as f32
        };
        match self.size {
            ImageSize::Small => natural.min(128.0).min(room),
            ImageSize::BestFit => natural.min(room),
            ImageSize::Original => natural,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    /// Rows of cells; every row has the same number.
    pub rows: Vec<Vec<Para>>,
}

impl Table {
    pub fn new(rows: usize, cols: usize) -> Self {
        Table {
            rows: vec![vec![Para::default(); cols.max(1)]; rows.max(1)],
        }
    }

    pub fn cols(&self) -> usize {
        self.rows.first().map_or(0, Vec::len)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Para(Para),
    Table(Table),
    Image(Image),
}

/// Names a paragraph: a top-level one, or a table cell `(row, col)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Path {
    pub block: usize,
    pub cell: Option<(usize, usize)>,
}

impl Path {
    pub fn top(block: usize) -> Self {
        Path { block, cell: None }
    }

    pub fn cell(block: usize, row: usize, col: usize) -> Self {
        Path {
            block,
            cell: Some((row, col)),
        }
    }
}

/// A byte offset in a paragraph. Orders as in the document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub path: Path,
    pub offset: usize,
}

impl Pos {
    pub fn new(path: Path, offset: usize) -> Self {
        Pos { path, offset }
    }
}

// Paragraph edits.

impl Para {
    pub fn new(text: impl Into<String>, style: CharStyle) -> Self {
        let text = text.into();
        let runs = if text.is_empty() {
            Vec::new()
        } else {
            vec![Run {
                len: text.len(),
                style,
            }]
        };
        Para {
            text,
            runs,
            style: ParaStyle::default(),
        }
    }

    pub fn plain(text: impl Into<String>) -> Self {
        Para::new(text, CharStyle::default())
    }

    pub fn with_style(mut self, style: ParaStyle) -> Self {
        self.style = style;
        self
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// The runs with their byte ranges.
    pub fn spans(&self) -> impl Iterator<Item = (Range<usize>, &CharStyle)> {
        let mut start = 0;
        self.runs.iter().map(move |run| {
            let range = start..start + run.len;
            start += run.len;
            (range, &run.style)
        })
    }

    /// The style text typed at `offset` takes: that of the character before
    /// it, or of the first one at the start.
    pub fn style_at(&self, offset: usize) -> CharStyle {
        let mut found = None;
        for (range, style) in self.spans() {
            if range.start < offset || (offset == 0 && range.start == 0) {
                found = Some(style);
            }
            if range.end >= offset && found.is_some() {
                break;
            }
        }
        found.cloned().unwrap_or_default()
    }

    /// Inserts `text` in `style` at `offset`.
    pub fn insert(&mut self, offset: usize, text: &str, style: &CharStyle) {
        if text.is_empty() {
            return;
        }
        let offset = clamp(&self.text, offset);
        self.text.insert_str(offset, text);
        let mut runs = Vec::with_capacity(self.runs.len() + 2);
        let mut start = 0;
        let mut placed = false;
        for run in self.runs.drain(..) {
            let end = start + run.len;
            if !placed && offset <= end && (offset > start || start == 0 || offset == start) {
                let before = offset - start;
                if before > 0 {
                    runs.push(Run {
                        len: before,
                        style: run.style.clone(),
                    });
                }
                runs.push(Run {
                    len: text.len(),
                    style: style.clone(),
                });
                if run.len > before {
                    runs.push(Run {
                        len: run.len - before,
                        style: run.style,
                    });
                }
                placed = true;
            } else {
                runs.push(run);
            }
            start = end;
        }
        if !placed {
            runs.push(Run {
                len: text.len(),
                style: style.clone(),
            });
        }
        self.runs = runs;
        self.normalize();
    }

    /// Removes the text in `range`.
    pub fn remove(&mut self, range: Range<usize>) {
        let range = clamp(&self.text, range.start)..clamp(&self.text, range.end);
        if range.is_empty() {
            return;
        }
        self.text.replace_range(range.clone(), "");
        let mut start = 0;
        for run in &mut self.runs {
            let end = start + run.len;
            let cut = range.end.min(end).saturating_sub(range.start.max(start));
            start = end;
            run.len -= cut;
        }
        self.normalize();
    }

    /// Changes the style of the text in `range`.
    pub fn restyle(&mut self, range: Range<usize>, f: &dyn Fn(&mut CharStyle)) {
        let range = clamp(&self.text, range.start)..clamp(&self.text, range.end);
        if range.is_empty() {
            return;
        }
        let mut runs = Vec::with_capacity(self.runs.len() + 2);
        for (span, style) in self.spans() {
            let inner = span.start.max(range.start)..span.end.min(range.end);
            if inner.is_empty() {
                runs.push(Run {
                    len: span.len(),
                    style: style.clone(),
                });
                continue;
            }
            if inner.start > span.start {
                runs.push(Run {
                    len: inner.start - span.start,
                    style: style.clone(),
                });
            }
            let mut changed = style.clone();
            f(&mut changed);
            runs.push(Run {
                len: inner.len(),
                style: changed,
            });
            if span.end > inner.end {
                runs.push(Run {
                    len: span.end - inner.end,
                    style: style.clone(),
                });
            }
        }
        self.runs = runs;
        self.normalize();
    }

    /// Whether every character in `range` matches; an empty range asks
    /// about the style at its start.
    pub fn all(&self, range: Range<usize>, test: &dyn Fn(&CharStyle) -> bool) -> bool {
        if range.is_empty() {
            return test(&self.style_at(range.start));
        }
        self.spans()
            .filter(|(span, _)| span.start < range.end && span.end > range.start)
            .all(|(_, style)| test(style))
    }

    /// Splits at `offset`: this keeps the text before it, the returned
    /// paragraph (with the same paragraph style) the rest.
    pub fn split_off(&mut self, offset: usize) -> Para {
        let offset = clamp(&self.text, offset);
        let tail_text = self.text.split_off(offset);
        let mut head = Vec::new();
        let mut tail = Vec::new();
        let mut start = 0;
        for run in self.runs.drain(..) {
            let end = start + run.len;
            if end <= offset {
                head.push(run);
            } else if start >= offset {
                tail.push(run);
            } else {
                head.push(Run {
                    len: offset - start,
                    style: run.style.clone(),
                });
                tail.push(Run {
                    len: end - offset,
                    style: run.style,
                });
            }
            start = end;
        }
        self.runs = head;
        Para {
            text: tail_text,
            runs: tail,
            style: self.style,
        }
    }

    /// Appends `other`'s text, keeping its styles.
    pub fn append(&mut self, other: Para) {
        self.text.push_str(&other.text);
        self.runs.extend(other.runs);
        self.normalize();
    }

    /// A copy of the text in `range` with its styles.
    pub fn slice(&self, range: Range<usize>) -> Para {
        let mut para = self.clone();
        let tail = para.split_off(range.end);
        drop(tail);
        para.split_off(range.start)
    }

    /// Drops empty runs and joins neighbours in the same style.
    fn normalize(&mut self) {
        let mut runs: Vec<Run> = Vec::with_capacity(self.runs.len());
        for run in self.runs.drain(..) {
            if run.len == 0 {
                continue;
            }
            match runs.last_mut() {
                Some(last) if last.style == run.style => last.len += run.len,
                _ => runs.push(run),
            }
        }
        self.runs = runs;
    }

    /// The link around `offset` and its range, if the text there links.
    pub fn link_at(&self, offset: usize) -> Option<(Range<usize>, Arc<str>)> {
        let spans: Vec<_> = self.spans().collect();
        let ix = spans.iter().position(|(range, style)| {
            style.link.is_some() && range.start <= offset && offset <= range.end
        })?;
        let link = spans[ix].1.link.clone()?;
        let mut range = spans[ix].0.clone();
        // Neighbouring runs of the same link in another style.
        for (other, style) in spans[..ix].iter().rev() {
            if style.link.as_ref() != Some(&link) {
                break;
            }
            range.start = other.start;
        }
        for (other, style) in &spans[ix + 1..] {
            if style.link.as_ref() != Some(&link) {
                break;
            }
            range.end = other.end;
        }
        Some((range, link))
    }
}

// The document.

/// Formatted text: paragraphs, tables and images.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doc {
    pub blocks: Vec<Block>,
}

impl Default for Doc {
    fn default() -> Self {
        Doc {
            blocks: vec![Block::Para(Para::default())],
        }
    }
}

impl Doc {
    pub fn para(&self, path: Path) -> Option<&Para> {
        match (self.blocks.get(path.block)?, path.cell) {
            (Block::Para(para), None) => Some(para),
            (Block::Table(table), Some((row, col))) => table.rows.get(row)?.get(col),
            _ => None,
        }
    }

    pub fn para_mut(&mut self, path: Path) -> Option<&mut Para> {
        match (self.blocks.get_mut(path.block)?, path.cell) {
            (Block::Para(para), None) => Some(para),
            (Block::Table(table), Some((row, col))) => table.rows.get_mut(row)?.get_mut(col),
            _ => None,
        }
    }

    pub fn table(&self, block: usize) -> Option<&Table> {
        match self.blocks.get(block)? {
            Block::Table(table) => Some(table),
            _ => None,
        }
    }

    fn table_mut(&mut self, block: usize) -> Option<&mut Table> {
        match self.blocks.get_mut(block)? {
            Block::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Every paragraph in document order: top-level ones, and the cells of
    /// tables row by row. Images hold none.
    pub fn paths(&self) -> Vec<Path> {
        let mut paths = Vec::new();
        for (ix, block) in self.blocks.iter().enumerate() {
            match block {
                Block::Para(_) => paths.push(Path::top(ix)),
                Block::Table(table) => {
                    for (row, cells) in table.rows.iter().enumerate() {
                        for col in 0..cells.len() {
                            paths.push(Path::cell(ix, row, col));
                        }
                    }
                }
                Block::Image(_) => {}
            }
        }
        paths
    }

    pub fn start(&self) -> Pos {
        self.paths()
            .first()
            .map_or(Pos::new(Path::top(0), 0), |&path| Pos::new(path, 0))
    }

    pub fn end(&self) -> Pos {
        self.paths()
            .last()
            .map_or(Pos::new(Path::top(0), 0), |&path| {
                Pos::new(path, self.para(path).map_or(0, Para::len))
            })
    }

    /// `pos` made valid: an existing paragraph and a char boundary in it.
    pub fn clamp(&self, pos: Pos) -> Pos {
        if let Some(para) = self.para(pos.path) {
            return Pos::new(pos.path, clamp(&para.text, pos.offset));
        }
        let paths = self.paths();
        let path = paths
            .iter()
            .rev()
            .find(|path| **path <= pos.path)
            .or(paths.first())
            .copied();
        match path {
            Some(path) => Pos::new(path, self.para(path).map_or(0, Para::len)),
            None => Pos::new(Path::top(0), 0),
        }
    }

    /// The paragraph after `path` in document order.
    pub fn next_path(&self, path: Path) -> Option<Path> {
        let paths = self.paths();
        let ix = paths.iter().position(|p| *p == path)?;
        paths.get(ix + 1).copied()
    }

    pub fn prev_path(&self, path: Path) -> Option<Path> {
        let paths = self.paths();
        let ix = paths.iter().position(|p| *p == path)?;
        ix.checked_sub(1).map(|ix| paths[ix])
    }

    /// One grapheme to the right, crossing into the next paragraph.
    pub fn next_pos(&self, pos: Pos) -> Pos {
        let Some(para) = self.para(pos.path) else {
            return pos;
        };
        if pos.offset < para.len() {
            return Pos::new(pos.path, next_grapheme(&para.text, pos.offset));
        }
        self.next_path(pos.path).map_or(pos, |p| Pos::new(p, 0))
    }

    pub fn prev_pos(&self, pos: Pos) -> Pos {
        let Some(para) = self.para(pos.path) else {
            return pos;
        };
        if pos.offset > 0 {
            return Pos::new(pos.path, prev_grapheme(&para.text, pos.offset));
        }
        self.prev_path(pos.path)
            .map_or(pos, |p| Pos::new(p, self.para(p).map_or(0, Para::len)))
    }

    /// The paragraphs a selection from `start` to `end` touches, with the
    /// byte range of each.
    pub fn covered(&self, start: Pos, end: Pos) -> Vec<(Path, Range<usize>)> {
        let (start, end) = order(start, end);
        self.paths()
            .into_iter()
            .filter(|path| *path >= start.path && *path <= end.path)
            .filter_map(|path| {
                let len = self.para(path)?.len();
                let from = if path == start.path { start.offset } else { 0 };
                let to = if path == end.path { end.offset } else { len };
                Some((path, from.min(len)..to.min(len).max(from.min(len))))
            })
            .collect()
    }

    /// Blocks wholly inside the selection (images and tables it spans).
    fn whole_blocks(&self, start: Pos, end: Pos) -> Vec<usize> {
        (start.path.block + 1..end.path.block)
            .filter(|&ix| !matches!(self.blocks[ix], Block::Para(_)))
            .collect()
    }

    // Edits. Each returns where the cursor goes.

    /// Inserts text typed or pasted at `pos` in `style`. Newlines split top
    /// paragraphs and stay line breaks in cells.
    pub fn insert_text(&mut self, pos: Pos, text: &str, style: &CharStyle) -> Pos {
        let pos = self.clamp(pos);
        if pos.path.cell.is_some() || !text.contains('\n') {
            if let Some(para) = self.para_mut(pos.path) {
                para.insert(pos.offset, text, style);
            }
            return Pos::new(pos.path, pos.offset + text.len());
        }
        let mut pos = pos;
        for (ix, line) in text.split('\n').enumerate() {
            if ix > 0 {
                pos = self.split(pos);
            }
            pos = self.insert_text(pos, line, style);
        }
        pos
    }

    /// Splits the top paragraph at `pos` (Enter). In a cell, inserts a line
    /// break instead.
    pub fn split(&mut self, pos: Pos) -> Pos {
        let pos = self.clamp(pos);
        if pos.path.cell.is_some() {
            let style = self
                .para(pos.path)
                .map(|p| p.style_at(pos.offset))
                .unwrap_or_default();
            return self.insert_text(pos, "\n", &style);
        }
        let Some(para) = self.para_mut(pos.path) else {
            return pos;
        };
        let tail = para.split_off(pos.offset);
        let at = pos.path.block + 1;
        self.blocks.insert(at, Block::Para(tail));
        Pos::new(Path::top(at), 0)
    }

    /// Deletes from `start` to `end`. Paragraphs between go, tables and
    /// images wholly inside go, cells only partly covered are emptied; two
    /// top paragraphs at the ends join.
    pub fn delete(&mut self, start: Pos, end: Pos) -> Pos {
        let (start, end) = order(self.clamp(start), self.clamp(end));
        if start == end {
            return start;
        }
        if start.path == end.path {
            if let Some(para) = self.para_mut(start.path) {
                para.remove(start.offset..end.offset);
            }
            return start;
        }
        // Clear the covered text of each paragraph first.
        for (path, range) in self.covered(start, end) {
            if let Some(para) = self.para_mut(path) {
                para.remove(range);
            }
        }
        // Then drop what lies wholly between the ends, last first.
        let mut drop: Vec<usize> = (start.path.block + 1..end.path.block)
            .filter(|&ix| matches!(self.blocks[ix], Block::Para(_)))
            .collect();
        drop.extend(self.whole_blocks(start, end));
        let start_in_table = start.path.cell.is_some();
        let end_in_table = end.path.cell.is_some();
        drop.sort_unstable();
        drop.dedup();
        let mut end_block = end.path.block;
        for ix in drop.into_iter().rev() {
            self.blocks.remove(ix);
            end_block -= 1;
        }
        if !start_in_table && !end_in_table && end_block != start.path.block {
            // Join the end paragraph onto the start one.
            if let Block::Para(tail) = self.blocks.remove(end_block)
                && let Some(Block::Para(head)) = self.blocks.get_mut(start.path.block)
            {
                head.append(tail);
            }
        }
        self.clamp(start)
    }

    /// Backspace at the very start of paragraph `path`: drops a list, quote
    /// or indent first, then joins it to what is before: the previous
    /// paragraph, or removes an image; before a table it moves into its
    /// last cell.
    pub fn join_backward(&mut self, path: Path) -> Pos {
        let here = Pos::new(path, 0);
        if path.cell.is_some() {
            return self.prev_pos(here);
        }
        let Some(para) = self.para_mut(path) else {
            return here;
        };
        if para.style.list != List::None {
            para.style.list = List::None;
            return here;
        }
        if para.style.quote > 0 {
            para.style.quote -= 1;
            return here;
        }
        if para.style.indent > 0 {
            para.style.indent -= 1;
            return here;
        }
        if path.block == 0 {
            return here;
        }
        let prev = path.block - 1;
        match &self.blocks[prev] {
            Block::Image(_) => {
                self.blocks.remove(prev);
                Pos::new(Path::top(prev), 0)
            }
            Block::Table(_) => self.prev_pos(here),
            Block::Para(_) => {
                let Block::Para(tail) = self.blocks.remove(path.block) else {
                    unreachable!()
                };
                let Some(Block::Para(head)) = self.blocks.get_mut(prev) else {
                    unreachable!()
                };
                let offset = head.len();
                // An empty paragraph takes the look of the one it joins.
                if head.is_empty() && !tail.is_empty() {
                    head.style = tail.style;
                }
                head.append(tail);
                Pos::new(Path::top(prev), offset)
            }
        }
    }

    /// Delete at the very end of paragraph `path`: joins the next paragraph
    /// on, removes an image after it.
    pub fn join_forward(&mut self, path: Path) -> Pos {
        let len = self.para(path).map_or(0, Para::len);
        let here = Pos::new(path, len);
        if path.cell.is_some() {
            return here;
        }
        let next = path.block + 1;
        match self.blocks.get(next) {
            Some(Block::Image(_)) => {
                self.blocks.remove(next);
                here
            }
            Some(Block::Para(_)) => {
                let Block::Para(tail) = self.blocks.remove(next) else {
                    unreachable!()
                };
                if let Some(head) = self.para_mut(path) {
                    head.append(tail);
                }
                here
            }
            _ => here,
        }
    }

    /// Changes the character style from `start` to `end`.
    pub fn restyle(&mut self, start: Pos, end: Pos, f: &dyn Fn(&mut CharStyle)) {
        for (path, range) in self.covered(start, end) {
            if let Some(para) = self.para_mut(path) {
                para.restyle(range, f);
            }
        }
    }

    /// Whether all text from `start` to `end` matches `test`.
    pub fn all(&self, start: Pos, end: Pos, test: &dyn Fn(&CharStyle) -> bool) -> bool {
        let covered = self.covered(start, end);
        let mut any = false;
        for (path, range) in &covered {
            let Some(para) = self.para(*path) else {
                continue;
            };
            if range.is_empty() && covered.len() > 1 {
                continue;
            }
            any = true;
            if !para.all(range.clone(), test) {
                return false;
            }
        }
        any || covered.is_empty()
    }

    /// Changes the paragraph style of every paragraph from `start` to
    /// `end`.
    pub fn restyle_paras(&mut self, start: Pos, end: Pos, f: &dyn Fn(&mut ParaStyle)) {
        for (path, _) in self.covered(start, end) {
            if let Some(para) = self.para_mut(path) {
                f(&mut para.style);
            }
        }
    }

    /// The top paragraphs from `start` to `end`.
    pub fn paras_in(&self, start: Pos, end: Pos) -> Vec<&Para> {
        self.covered(start, end)
            .into_iter()
            .filter_map(|(path, _)| self.para(path))
            .collect()
    }

    /// Splits at `pos` and puts `block` between the halves; the text after
    /// it gets its own paragraph so there is always a place to type. In a
    /// cell, the block goes after the table.
    fn insert_block(&mut self, pos: Pos, block: Block) -> usize {
        let pos = self.clamp(pos);
        if pos.path.cell.is_some() {
            let at = pos.path.block + 1;
            self.blocks.insert(at, block);
            if !matches!(self.blocks.get(at + 1), Some(Block::Para(_))) {
                self.blocks.insert(at + 1, Block::Para(Para::default()));
            }
            return at;
        }
        let empty = self.para(pos.path).is_some_and(Para::is_empty);
        if empty {
            // An empty line becomes the block.
            let at = pos.path.block;
            self.blocks.insert(at, block);
            return at;
        }
        let after = self.split(pos);
        let at = after.path.block;
        if pos.offset == 0 {
            // Nothing before: keep the text after, drop the empty head.
            self.blocks.insert(at, block);
            self.blocks.remove(at - 1);
            return at - 1;
        }
        self.blocks.insert(at, block);
        at
    }

    /// Inserts a `rows` by `cols` table at `pos`; the cursor goes to its
    /// first cell.
    pub fn insert_table(&mut self, pos: Pos, rows: usize, cols: usize) -> Pos {
        let at = self.insert_block(pos, Block::Table(Table::new(rows, cols)));
        self.ensure_para_after(at);
        Pos::new(Path::cell(at, 0, 0), 0)
    }

    /// Inserts an image at `pos`; the cursor goes to the line after it.
    pub fn insert_image(&mut self, pos: Pos, image: Image) -> Pos {
        let at = self.insert_block(pos, Block::Image(image));
        self.ensure_para_after(at);
        Pos::new(Path::top(at + 1), 0)
    }

    fn ensure_para_after(&mut self, at: usize) {
        if !matches!(self.blocks.get(at + 1), Some(Block::Para(_))) {
            self.blocks.insert(at + 1, Block::Para(Para::default()));
        }
    }

    pub fn remove_block(&mut self, block: usize) -> Pos {
        if block < self.blocks.len() {
            self.blocks.remove(block);
        }
        if self.blocks.is_empty() {
            self.blocks.push(Block::Para(Para::default()));
        }
        let at = block.min(self.blocks.len() - 1);
        // Stand on a paragraph near where the block was.
        let path = self
            .paths()
            .into_iter()
            .find(|p| p.block >= at)
            .or_else(|| self.paths().last().copied())
            .unwrap_or(Path::top(0));
        Pos::new(path, 0)
    }

    pub fn image_mut(&mut self, block: usize) -> Option<&mut Image> {
        match self.blocks.get_mut(block)? {
            Block::Image(image) => Some(image),
            _ => None,
        }
    }

    /// Adds a row above or below the cell at `path`.
    pub fn insert_row(&mut self, path: Path, below: bool) -> Pos {
        let Some((row, col)) = path.cell else {
            return Pos::new(path, 0);
        };
        let Some(table) = self.table_mut(path.block) else {
            return Pos::new(path, 0);
        };
        let at = if below { row + 1 } else { row };
        let cols = table.cols();
        table.rows.insert(at, vec![Para::default(); cols]);
        Pos::new(Path::cell(path.block, at, col), 0)
    }

    /// Adds a column left or right of the cell at `path`.
    pub fn insert_col(&mut self, path: Path, right: bool) -> Pos {
        let Some((row, col)) = path.cell else {
            return Pos::new(path, 0);
        };
        let Some(table) = self.table_mut(path.block) else {
            return Pos::new(path, 0);
        };
        let at = if right { col + 1 } else { col };
        for cells in &mut table.rows {
            cells.insert(at.min(cells.len()), Para::default());
        }
        Pos::new(Path::cell(path.block, row, at), 0)
    }

    /// Removes the row of the cell at `path`; the last row takes the table.
    pub fn delete_row(&mut self, path: Path) -> Pos {
        let Some((row, col)) = path.cell else {
            return Pos::new(path, 0);
        };
        let Some(table) = self.table_mut(path.block) else {
            return Pos::new(path, 0);
        };
        if table.rows.len() <= 1 {
            return self.remove_block(path.block);
        }
        table.rows.remove(row);
        let row = row.min(table.rows.len() - 1);
        Pos::new(Path::cell(path.block, row, col), 0)
    }

    pub fn delete_col(&mut self, path: Path) -> Pos {
        let Some((row, col)) = path.cell else {
            return Pos::new(path, 0);
        };
        let Some(table) = self.table_mut(path.block) else {
            return Pos::new(path, 0);
        };
        if table.cols() <= 1 {
            return self.remove_block(path.block);
        }
        for cells in &mut table.rows {
            if col < cells.len() {
                cells.remove(col);
            }
        }
        let col = col.min(table.cols() - 1);
        Pos::new(Path::cell(path.block, row, col), 0)
    }

    /// The styled content from `start` to `end`, for copying.
    pub fn fragment(&self, start: Pos, end: Pos) -> Vec<Block> {
        let (start, end) = order(start, end);
        if start.path == end.path {
            return self
                .para(start.path)
                .map(|p| {
                    let mut slice = p.slice(start.offset..end.offset);
                    slice.style = ParaStyle::default();
                    vec![Block::Para(slice)]
                })
                .unwrap_or_default();
        }
        let mut blocks = Vec::new();
        for ix in start.path.block..=end.path.block {
            match &self.blocks[ix] {
                Block::Para(para) => {
                    let from = if ix == start.path.block {
                        start.offset
                    } else {
                        0
                    };
                    let to = if ix == end.path.block {
                        end.offset
                    } else {
                        para.len()
                    };
                    let mut slice = para.slice(from.min(para.len())..to.min(para.len()));
                    slice.style.signature = false;
                    blocks.push(Block::Para(slice));
                }
                Block::Table(table) => {
                    let whole = ix != start.path.block && ix != end.path.block;
                    if whole {
                        blocks.push(Block::Table(table.clone()));
                    } else {
                        // Part of a table copies as its cells' text.
                        for (path, range) in self.covered(start, end) {
                            if path.block == ix
                                && let Some(p) = self.para(path)
                            {
                                blocks.push(Block::Para(p.slice(range)));
                            }
                        }
                    }
                }
                Block::Image(image) => blocks.push(Block::Image(image.clone())),
            }
        }
        blocks
    }

    /// Inserts copied content at `pos`.
    pub fn insert_fragment(&mut self, pos: Pos, fragment: Vec<Block>) -> Pos {
        let mut pos = self.clamp(pos);
        let in_cell = pos.path.cell.is_some();
        let single = fragment.len() == 1 && matches!(fragment[0], Block::Para(_));
        if in_cell || single {
            // Text only, joined by line breaks.
            let mut first = true;
            for block in fragment {
                let Block::Para(para) = block else { continue };
                if !first {
                    pos = self.split(pos);
                }
                first = false;
                for (range, style) in para.spans() {
                    pos = self.insert_text(pos, &para.text[range], style);
                }
            }
            return pos;
        }
        let count = fragment.len();
        for (ix, block) in fragment.into_iter().enumerate() {
            match block {
                Block::Para(para) => {
                    if ix > 0 {
                        pos = self.split(pos);
                        // Whole paragraphs bring their look.
                        if ix + 1 < count
                            && let Some(target) = self.para_mut(pos.path)
                        {
                            target.style = para.style;
                        }
                    }
                    for (range, style) in para.spans() {
                        pos = self.insert_text(pos, &para.text[range], style);
                    }
                }
                other => {
                    let at = self.insert_block(pos, other);
                    self.ensure_para_after(at);
                    pos = Pos::new(Path::top(at + 1), 0);
                }
            }
        }
        pos
    }

    /// Removes the signature: its paragraphs and the pictures and tables
    /// between them. Returns where it was.
    pub fn remove_signature(&mut self) -> Option<usize> {
        let marked = |b: &Block| matches!(b, Block::Para(p) if p.style.signature);
        let first = self.blocks.iter().position(marked)?;
        let last = self.blocks.iter().rposition(marked)?;
        self.blocks.drain(first..=last);
        if self.blocks.is_empty() {
            self.blocks.push(Block::Para(Para::default()));
        }
        Some(first)
    }

    /// Puts `signature` (its lines, `-- ` first) at block `at`.
    pub fn insert_signature(&mut self, at: usize, signature: &str) {
        let signature = signature.trim_end();
        if signature.is_empty() {
            return;
        }
        let style = ParaStyle {
            signature: true,
            ..ParaStyle::default()
        };
        let lines = std::iter::once("-- ").chain(signature.lines());
        let at = at.min(self.blocks.len());
        for (ix, line) in lines.enumerate() {
            self.blocks
                .insert(at + ix, Block::Para(Para::plain(line).with_style(style)));
        }
    }

    /// Whether anything is formatted: plain text mode would lose it.
    pub fn has_formatting(&self) -> bool {
        self.blocks.iter().any(|block| match block {
            Block::Para(para) => {
                let style = ParaStyle {
                    quote: 0,
                    signature: false,
                    ..para.style
                };
                style != ParaStyle::default() || para.runs.iter().any(|r| !r.style.is_plain())
            }
            Block::Table(_) | Block::Image(_) => true,
        })
    }

    pub fn images(&self) -> impl Iterator<Item = &Image> {
        self.blocks.iter().filter_map(|b| match b {
            Block::Image(image) => Some(image),
            _ => None,
        })
    }

    /// Whether there is no text, table or image at all.
    pub fn is_blank(&self) -> bool {
        self.blocks.iter().all(|b| match b {
            Block::Para(p) => p.text.trim().is_empty(),
            _ => false,
        })
    }
}

/// `(a, b)` in document order.
pub fn order(a: Pos, b: Pos) -> (Pos, Pos) {
    if a <= b { (a, b) } else { (b, a) }
}

/// Numbers of numbered-list paragraphs: the item number for each top-level
/// block (0 when not numbered). Counting restarts after any block that is
/// not in a list, and per indent level.
pub fn list_numbers(doc: &Doc) -> Vec<usize> {
    let mut counters = [0usize; MAX_INDENT as usize + 1];
    doc.blocks
        .iter()
        .map(|block| {
            let Block::Para(para) = block else {
                counters = [0; MAX_INDENT as usize + 1];
                return 0;
            };
            let level = para.style.indent.min(MAX_INDENT) as usize;
            match para.style.list {
                List::None => {
                    counters = [0; MAX_INDENT as usize + 1];
                    0
                }
                List::Bullet => {
                    counters[level + 1..].fill(0);
                    counters[level] = 0;
                    0
                }
                List::Numbered => {
                    counters[level + 1..].fill(0);
                    counters[level] += 1;
                    counters[level]
                }
            }
        })
        .collect()
}

/// The marker drawn before a list item.
pub fn list_marker(style: &ParaStyle, number: usize) -> String {
    match style.list {
        List::None => String::new(),
        List::Bullet => match style.indent % 3 {
            0 => "\u{2022}".to_owned(),
            1 => "\u{25e6}".to_owned(),
            _ => "\u{25aa}".to_owned(),
        },
        List::Numbered => match style.indent % 3 {
            0 => format!("{number}."),
            1 => format!("{}.", alpha(number)),
            _ => format!("{}.", roman(number)),
        },
    }
}

fn alpha(mut n: usize) -> String {
    let mut out = Vec::new();
    while n > 0 {
        n -= 1;
        out.push(b'a' + (n % 26) as u8);
        n /= 26;
    }
    out.reverse();
    String::from_utf8(out).unwrap_or_default()
}

fn roman(mut n: usize) -> String {
    const TABLE: [(usize, &str); 13] = [
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ];
    let mut out = String::new();
    for (value, digits) in TABLE {
        while n >= value {
            out.push_str(digits);
            n -= value;
        }
    }
    out
}

// Text helpers.

pub(crate) fn clamp(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

pub(crate) fn prev_grapheme(text: &str, offset: usize) -> usize {
    let offset = clamp(text, offset);
    GraphemeCursor::new(offset, text.len(), true)
        .prev_boundary(text, 0)
        .ok()
        .flatten()
        .unwrap_or(0)
}

pub(crate) fn next_grapheme(text: &str, offset: usize) -> usize {
    let offset = clamp(text, offset);
    GraphemeCursor::new(offset, text.len(), true)
        .next_boundary(text, 0)
        .ok()
        .flatten()
        .unwrap_or(text.len())
}

pub(crate) fn floor_grapheme(text: &str, offset: usize) -> usize {
    let offset = clamp(text, offset);
    if GraphemeCursor::new(offset, text.len(), true)
        .is_boundary(text, 0)
        .unwrap_or(true)
    {
        offset
    } else {
        prev_grapheme(text, offset)
    }
}

fn is_word(segment: &str) -> bool {
    segment.chars().any(char::is_alphanumeric)
}

pub(crate) fn word_left(text: &str, offset: usize) -> usize {
    let offset = clamp(text, offset);
    text[..offset]
        .split_word_bound_indices()
        .rev()
        .find(|(_, segment)| is_word(segment))
        .map_or(0, |(ix, _)| ix)
}

pub(crate) fn word_right(text: &str, offset: usize) -> usize {
    let offset = clamp(text, offset);
    text[offset..]
        .split_word_bound_indices()
        .find(|(_, segment)| is_word(segment))
        .map_or(text.len(), |(ix, segment)| offset + ix + segment.len())
}

/// The word (or run of spaces or punctuation) at `offset`.
pub(crate) fn word_range(text: &str, offset: usize) -> Range<usize> {
    let offset = clamp(text, offset);
    let mut before = None;
    for (ix, segment) in text.split_word_bound_indices() {
        let range = ix..ix + segment.len();
        if range.contains(&offset) {
            if offset == range.start
                && !is_word(segment)
                && let Some((before, true)) = before
            {
                return before;
            }
            return range;
        }
        before = Some((range, is_word(segment)));
    }
    before.map_or(offset..offset, |(range, _)| range)
}

/// Width and height of a PNG, JPEG, GIF, WebP or BMP image from its header.
pub fn image_size(data: &[u8]) -> Option<(u32, u32)> {
    let be16 = |b: &[u8], i: usize| Some(u16::from_be_bytes([*b.get(i)?, *b.get(i + 1)?]) as u32);
    let le16 = |b: &[u8], i: usize| Some(u16::from_le_bytes([*b.get(i)?, *b.get(i + 1)?]) as u32);
    let be32 = |b: &[u8], i: usize| {
        Some(u32::from_be_bytes([
            *b.get(i)?,
            *b.get(i + 1)?,
            *b.get(i + 2)?,
            *b.get(i + 3)?,
        ]))
    };
    let le32 = |b: &[u8], i: usize| {
        Some(u32::from_le_bytes([
            *b.get(i)?,
            *b.get(i + 1)?,
            *b.get(i + 2)?,
            *b.get(i + 3)?,
        ]))
    };
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some((be32(data, 16)?, be32(data, 20)?));
    }
    if data.starts_with(b"GIF8") {
        return Some((le16(data, 6)?, le16(data, 8)?));
    }
    if data.starts_with(b"BM") {
        let h = le32(data, 22)? as i32;
        return Some((le32(data, 18)?, h.unsigned_abs()));
    }
    if data.starts_with(b"RIFF") && data.get(8..12) == Some(b"WEBP") {
        return match data.get(12..16)? {
            b"VP8 " => Some((le16(data, 26)? & 0x3fff, le16(data, 28)? & 0x3fff)),
            b"VP8L" => {
                let bits = le32(data, 21)?;
                Some(((bits & 0x3fff) + 1, ((bits >> 14) & 0x3fff) + 1))
            }
            b"VP8X" => {
                let w = le32(data, 24)? & 0xff_ffff;
                let h = le32(data, 27)? & 0xff_ffff;
                Some((w + 1, h + 1))
            }
            _ => None,
        };
    }
    if data.starts_with(&[0xff, 0xd8]) {
        let mut i = 2;
        while i + 9 < data.len() {
            if data[i] != 0xff {
                i += 1;
                continue;
            }
            let marker = data[i + 1];
            if matches!(marker, 0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf) {
                return Some((be16(data, i + 7)?, be16(data, i + 5)?));
            }
            if marker == 0xd8 || marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
                i += 2;
                continue;
            }
            i += 2 + be16(data, i + 2)? as usize;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bold() -> CharStyle {
        CharStyle {
            bold: true,
            ..CharStyle::default()
        }
    }

    fn doc(paras: &[&str]) -> Doc {
        Doc {
            blocks: paras.iter().map(|t| Block::Para(Para::plain(*t))).collect(),
        }
    }

    fn texts(doc: &Doc) -> Vec<String> {
        doc.blocks
            .iter()
            .map(|b| match b {
                Block::Para(p) => p.text.clone(),
                Block::Table(_) => "<table>".to_owned(),
                Block::Image(_) => "<img>".to_owned(),
            })
            .collect()
    }

    fn at(block: usize, offset: usize) -> Pos {
        Pos::new(Path::top(block), offset)
    }

    #[test]
    fn inserting_splits_runs() {
        let mut p = Para::plain("hello world");
        p.insert(5, " big", &bold());
        assert_eq!(p.text, "hello big world");
        let lens: Vec<_> = p.runs.iter().map(|r| (r.len, r.style.bold)).collect();
        assert_eq!(lens, vec![(5, false), (4, true), (6, false)]);
        // Same style merges.
        p.insert(0, "oh ", &CharStyle::default());
        assert_eq!(p.runs[0].len, 8);
        let mut e = Para::default();
        e.insert(0, "x", &bold());
        assert_eq!(
            e.runs,
            vec![Run {
                len: 1,
                style: bold()
            }]
        );
    }

    #[test]
    fn style_at_takes_the_previous_character() {
        let mut p = Para::plain("ab");
        p.restyle(1..2, &|s| s.bold = true);
        assert!(!p.style_at(0).bold);
        assert!(!p.style_at(1).bold);
        assert!(p.style_at(2).bold);
        let mut q = Para::plain("ab");
        q.restyle(0..1, &|s| s.italic = true);
        assert!(q.style_at(0).italic);
    }

    #[test]
    fn restyle_and_remove_keep_runs_consistent() {
        let mut p = Para::plain("abcdef");
        p.restyle(2..4, &|s| s.bold = true);
        assert_eq!(p.runs.len(), 3);
        assert!(p.all(2..4, &|s| s.bold));
        assert!(!p.all(1..4, &|s| s.bold));
        p.remove(1..5);
        assert_eq!(p.text, "af");
        assert_eq!(p.runs.len(), 1);
        assert_eq!(p.runs[0].len, 2);
        p.restyle(0..2, &|s| s.bold = false);
        assert_eq!(p.runs.iter().map(|r| r.len).sum::<usize>(), p.len());
    }

    #[test]
    fn split_and_append_round_trip() {
        let mut p = Para::plain("hello");
        p.restyle(1..4, &|s| s.italic = true);
        let original = p.clone();
        let tail = p.split_off(2);
        assert_eq!((p.text.as_str(), tail.text.as_str()), ("he", "llo"));
        p.append(tail);
        assert_eq!(p, original);
        assert_eq!(original.slice(1..3).text, "el");
        assert!(original.slice(1..3).runs.iter().all(|r| r.style.italic));
    }

    #[test]
    fn enter_and_multi_line_insert() {
        let mut d = doc(&["abcd"]);
        let pos = d.split(at(0, 2));
        assert_eq!(texts(&d), ["ab", "cd"]);
        assert_eq!(pos, at(1, 0));
        let pos = d.insert_text(at(0, 1), "1\n2\n3", &CharStyle::default());
        assert_eq!(texts(&d), ["a1", "2", "3b", "cd"]);
        assert_eq!(pos, at(2, 1));
    }

    #[test]
    fn deleting_across_paragraphs_joins_them() {
        let mut d = doc(&["one", "two", "three"]);
        let pos = d.delete(at(2, 2), at(0, 1));
        assert_eq!(texts(&d), ["oree"]);
        assert_eq!(pos, at(0, 1));
    }

    #[test]
    fn deleting_across_a_table_and_image_removes_them() {
        let mut d = doc(&["one", "two"]);
        d.blocks.insert(1, Block::Table(Table::new(2, 2)));
        d.blocks.insert(
            2,
            Block::Image(Image {
                id: 1,
                name: "a.png".into(),
                mime: "image/png".into(),
                data: Arc::new(vec![]),
                width: 0,
                height: 0,
                size: ImageSize::BestFit,
            }),
        );
        assert_eq!(texts(&d), ["one", "<table>", "<img>", "two"]);
        d.delete(at(0, 1), at(3, 1));
        assert_eq!(texts(&d), ["owo"]);
    }

    #[test]
    fn deleting_into_a_cell_empties_only_covered_text() {
        let mut d = doc(&["one", "two"]);
        d.blocks.insert(1, Block::Table(Table::new(1, 2)));
        *d.para_mut(Path::cell(1, 0, 0)).unwrap() = Para::plain("aa");
        *d.para_mut(Path::cell(1, 0, 1)).unwrap() = Para::plain("bb");
        d.delete(at(0, 2), Pos::new(Path::cell(1, 0, 1), 1));
        assert_eq!(texts(&d), ["on", "<table>", "two"]);
        assert_eq!(d.para(Path::cell(1, 0, 0)).unwrap().text, "");
        assert_eq!(d.para(Path::cell(1, 0, 1)).unwrap().text, "b");
    }

    #[test]
    fn backspace_at_start_drops_list_then_joins() {
        let mut d = doc(&["a", "b"]);
        if let Some(p) = d.para_mut(Path::top(1)) {
            p.style.list = List::Bullet;
        }
        assert_eq!(d.join_backward(Path::top(1)), at(1, 0));
        assert_eq!(d.para(Path::top(1)).unwrap().style.list, List::None);
        assert_eq!(d.join_backward(Path::top(1)), at(0, 1));
        assert_eq!(texts(&d), ["ab"]);
    }

    #[test]
    fn tables_insert_and_edit() {
        let mut d = doc(&["abcd"]);
        let pos = d.insert_table(at(0, 2), 2, 3);
        assert_eq!(texts(&d), ["ab", "<table>", "cd"]);
        assert_eq!(pos, Pos::new(Path::cell(1, 0, 0), 0));
        d.insert_row(pos.path, true);
        d.insert_col(pos.path, false);
        let t = d.table(1).unwrap();
        assert_eq!((t.rows.len(), t.cols()), (3, 4));
        d.delete_row(Path::cell(1, 0, 0));
        d.delete_col(Path::cell(1, 0, 0));
        let t = d.table(1).unwrap();
        assert_eq!((t.rows.len(), t.cols()), (2, 3));
        // In an empty document the table replaces the empty line.
        let mut e = Doc::default();
        e.insert_table(e.start(), 1, 1);
        assert_eq!(texts(&e), ["<table>", ""]);
        // Enter in a cell is a line break.
        let p = e.split(Pos::new(Path::cell(0, 0, 0), 0));
        assert_eq!(p.offset, 1);
        assert_eq!(e.para(Path::cell(0, 0, 0)).unwrap().text, "\n");
    }

    #[test]
    fn walking_crosses_cells_and_skips_images() {
        let mut d = doc(&["a", "b"]);
        d.insert_table(at(1, 0), 1, 2);
        assert_eq!(texts(&d), ["a", "<table>", "b"]);
        let mut pos = at(0, 1);
        pos = d.next_pos(pos);
        assert_eq!(pos, Pos::new(Path::cell(1, 0, 0), 0));
        pos = d.next_pos(pos);
        assert_eq!(pos, Pos::new(Path::cell(1, 0, 1), 0));
        pos = d.next_pos(pos);
        assert_eq!(pos, at(2, 0));
        assert_eq!(d.prev_pos(pos), Pos::new(Path::cell(1, 0, 1), 0));
    }

    #[test]
    fn restyle_spans_paragraphs() {
        let mut d = doc(&["one", "two"]);
        d.restyle(at(0, 1), at(1, 2), &|s| s.bold = true);
        assert!(d.all(at(0, 1), at(1, 2), &|s| s.bold));
        assert!(!d.all(at(0, 0), at(1, 2), &|s| s.bold));
    }

    #[test]
    fn fragments_copy_and_paste() {
        let mut d = doc(&["hello", "world"]);
        d.restyle(at(0, 0), at(0, 5), &|s| s.bold = true);
        let frag = d.fragment(at(0, 3), at(1, 2));
        assert_eq!(frag.len(), 2);
        let mut e = doc(&["[]"]);
        let pos = e.insert_fragment(at(0, 1), frag);
        assert_eq!(texts(&e), ["[lo", "wo]"]);
        assert_eq!(pos, at(1, 2));
        assert!(e.para(Path::top(0)).unwrap().style_at(3).bold);
        // One paragraph pastes inline.
        let mut f = doc(&["ab"]);
        let frag = d.fragment(at(0, 1), at(0, 3));
        f.insert_fragment(at(0, 1), frag);
        assert_eq!(texts(&f), ["aelb"]);
    }

    #[test]
    fn numbering_restarts() {
        let mut d = doc(&["a", "b", "c", "d"]);
        for ix in [0, 1, 3] {
            if let Some(p) = d.para_mut(Path::top(ix)) {
                p.style.list = List::Numbered;
            }
        }
        assert_eq!(list_numbers(&d), vec![1, 2, 0, 1]);
        assert_eq!(roman(14), "xiv");
        assert_eq!(alpha(28), "ab");
    }

    #[test]
    fn signatures_swap() {
        let mut d = doc(&["hi", ""]);
        d.insert_signature(2, "Kay\nEnron\n");
        assert_eq!(texts(&d), ["hi", "", "-- ", "Kay", "Enron"]);
        assert_eq!(d.remove_signature(), Some(2));
        assert_eq!(texts(&d), ["hi", ""]);
    }

    #[test]
    fn links_extend_over_style_changes() {
        let mut p = Para::plain("see the site now");
        let link: Arc<str> = "https://katna.invenia.in".into();
        p.restyle(4..12, &|s| s.link = Some(link.clone()));
        p.restyle(8..12, &|s| s.bold = true);
        let (range, found) = p.link_at(5).unwrap();
        assert_eq!(range, 4..12);
        assert_eq!(&*found, "https://katna.invenia.in");
        assert!(p.link_at(1).is_none());
    }

    #[test]
    fn reads_image_sizes() {
        let mut png = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR".to_vec();
        png.extend(640u32.to_be_bytes());
        png.extend(480u32.to_be_bytes());
        assert_eq!(image_size(&png), Some((640, 480)));
        let gif = b"GIF89a\x20\x00\x10\x00";
        assert_eq!(image_size(gif), Some((32, 16)));
        let jpeg = [
            0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x01,
            0x00, 0x02, 0x00, 0x03,
        ];
        assert_eq!(image_size(&jpeg), Some((512, 256)));
        assert_eq!(image_size(b"nope"), None);
    }
}
