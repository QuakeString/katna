// SPDX-License-Identifier: GPL-3.0-or-later

//! Walks the parsed HTML once and builds the [`Document`].

use std::collections::HashSet;
use std::sync::Arc;

use super::css::{self, Length};
use super::dom::{DOCUMENT, Data, Dom, NodeId};
use super::{
    Align, Block, BoxBlock, BoxKind, BoxStyle, Document, Image, ImageKind, ImageSource, Inline,
    Run, RunStyle, TextBlock,
};
use crate::MAX_BODY_BYTES;
use katna_core::bidi::{Direction, first_strong};

/// Elements whose whole subtree is dropped.
const DROPPED: &[&str] = &[
    "applet", "area", "audio", "base", "button", "canvas", "datalist", "embed", "frame",
    "frameset", "head", "iframe", "input", "link", "map", "meta", "noembed", "noframes",
    "noscript", "object", "option", "param", "script", "select", "source", "style", "template",
    "textarea", "title", "track", "video",
];

/// Elements that start a new block.
const BLOCKS: &[&str] = &[
    "address",
    "article",
    "aside",
    "center",
    "dd",
    "details",
    "dialog",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "legend",
    "main",
    "nav",
    "p",
    "pre",
    "section",
    "summary",
    "caption",
];

/// Parts of a URL that mark a tracking pixel.
const TRACKER_PATHS: &[&str] = &[
    "/wf/open",
    "/track/open",
    "/tracking/open",
    "/trk/open",
    "/open.php",
    "/open.aspx",
    "/open.gif",
    "/pixel.gif",
    "/pixel.png",
    "/beacon",
    "/e/o/",
];

/// Past this many boxes deep, further boxes are flattened into their
/// parent, so a pathological message cannot build a tree the UI chokes on.
const MAX_BOX_DEPTH: usize = 40;
/// Widest a table cell's longest word can make it, so one long link does
/// not push the rest of its row out of the pane.
const MAX_CELL_MIN: f32 = 240.0;
/// Most the cells of one row can ask for in all.
const MAX_ROW_MIN: f32 = 560.0;
/// Past this many elements deep, the rest is dropped. Every level costs
/// stack (several kilobytes in a debug build), and no real mail nests
/// anywhere near this deep.
const MAX_DEPTH: usize = 320;
/// At most this many blocks and inlines in all.
const MAX_ITEMS: usize = 40_000;

#[derive(Clone)]
struct Ctx {
    run: RunStyle,
    align: Align,
    pre: bool,
    /// `cellpadding` of the table being walked.
    cell_padding: f32,
    /// The cell border of a `<table border>`.
    cell_border: Option<(f32, u32)>,
    /// Boxes open around this element.
    boxes: usize,
    /// Elements open around this element.
    depth: usize,
    /// Nesting of lists, for the bullet shape.
    lists: usize,
    /// The table being walked has no width, so it is as wide as its
    /// content, as a signature's layout table is.
    fit: bool,
    /// The direction the nearest `dir` attribute gives.
    dir: Option<Direction>,
    /// The element's own `dir`, where it turns the other way from what
    /// is around it.
    box_dir: Option<Direction>,
}

/// Blocks being collected for one box, and its open paragraph.
#[derive(Default)]
struct Out {
    blocks: Vec<Block>,
    inlines: Vec<Inline>,
    align: Align,
    pre: bool,
    dir: Option<Direction>,
    /// The paragraph had a `<br>` (so a blank one still takes a line).
    had_break: bool,
}

impl Out {
    fn start(&mut self, ctx: &Ctx) {
        if self.inlines.is_empty() {
            self.align = ctx.align;
            self.pre = ctx.pre;
            self.dir = ctx.dir;
        }
    }

    /// The paragraph's last character, if its last inline is text.
    fn last_char(&self) -> Option<char> {
        match self.inlines.last() {
            Some(Inline::Text(run)) => run.text.chars().last(),
            Some(Inline::Image(_)) => Some('x'),
            None => None,
        }
    }

    fn push_str(&mut self, text: &str, style: &RunStyle) {
        if let Some(Inline::Text(run)) = self.inlines.last_mut()
            && run.style == *style
        {
            run.text.push_str(text);
            return;
        }
        self.inlines.push(Inline::Text(Run {
            text: text.to_owned(),
            style: style.clone(),
        }));
    }

    /// Adds text, collapsing white space unless in `<pre>`.
    fn text(&mut self, text: &str, ctx: &Ctx) {
        self.start(ctx);
        if ctx.pre {
            let text = text.replace("\r\n", "\n").replace('\t', "    ");
            self.push_str(&text, &ctx.run);
            return;
        }
        let mut collapsed = String::with_capacity(text.len());
        let mut space = matches!(self.last_char(), None | Some(' ' | '\n'));
        for ch in text.chars() {
            if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\x0c') {
                if !space {
                    collapsed.push(' ');
                    space = true;
                }
            } else if ch != '\u{200b}' && ch != '\u{feff}' && ch != '\u{ad}' {
                collapsed.push(ch);
                space = false;
            }
        }
        if !collapsed.is_empty() {
            self.push_str(&collapsed, &ctx.run);
        }
    }

    fn line_break(&mut self, ctx: &Ctx) {
        self.start(ctx);
        // A space before a break is never shown.
        if let Some(Inline::Text(run)) = self.inlines.last_mut() {
            let kept = run.text.trim_end_matches(' ').len();
            run.text.truncate(kept);
        }
        self.push_str("\n", &ctx.run);
        self.had_break = true;
    }

    fn image(&mut self, image: Image, ctx: &Ctx) {
        self.start(ctx);
        self.inlines.push(Inline::Image(image));
    }

    /// Ends the open paragraph.
    fn flush(&mut self) {
        let mut inlines = std::mem::take(&mut self.inlines);
        let had_break = std::mem::take(&mut self.had_break);
        // Trailing spaces, and one trailing break, are not shown.
        if let Some(Inline::Text(run)) = inlines.last_mut() {
            let kept = run.text.trim_end_matches(' ').len();
            run.text.truncate(kept);
            if !self.pre && run.text.ends_with('\n') {
                run.text.pop();
            }
        }
        inlines.retain(|i| !matches!(i, Inline::Text(run) if run.text.is_empty()));
        let blank = inlines.iter().all(|i| match i {
            Inline::Text(run) => run.text.chars().all(char::is_whitespace),
            Inline::Image(_) => false,
        });
        if blank {
            // `<p>&nbsp;</p>` and `<div><br></div>` are blank lines.
            let has_nbsp = inlines
                .iter()
                .any(|i| matches!(i, Inline::Text(run) if run.text.contains('\u{a0}')));
            if !(had_break || has_nbsp) {
                return;
            }
            let style = match inlines.first() {
                Some(Inline::Text(run)) => run.style.clone(),
                _ => RunStyle::default(),
            };
            inlines = vec![Inline::Text(Run {
                text: " ".to_owned(),
                style,
            })];
        }
        let dir = self.dir.or_else(|| {
            inlines.iter().find_map(|i| match i {
                Inline::Text(run) => first_strong(&run.text),
                Inline::Image(_) => None,
            })
        });
        let mut align = self.align;
        // `text-align: right` in a right-to-left paragraph is where it
        // starts anyway (senders set it to right-align Arabic or Hebrew).
        if dir == Some(Direction::Rtl) && align == Align::End {
            align = Align::Start;
        }
        self.blocks.push(Block::Text(TextBlock {
            inlines,
            align,
            preformatted: self.pre,
            dir,
        }));
    }
}

struct Builder<'a> {
    dom: &'a Dom,
    inline_image: &'a dyn Fn(&str) -> Option<Arc<[u8]>>,
    doc: Document,
    bytes: usize,
    items: usize,
    /// List counters, innermost last.
    counters: Vec<usize>,
    /// Nodes left out with everything in them.
    skip: &'a HashSet<NodeId>,
}

pub(super) fn build(dom: &Dom, inline_image: &dyn Fn(&str) -> Option<Arc<[u8]>>) -> Document {
    build_from(dom, &[DOCUMENT], &HashSet::new(), inline_image)
}

/// Builds the nodes `roots`, in order, as one document, leaving out the
/// nodes in `skip`. A root starts from default styles: what its ancestors
/// set (a `<font color>` around it) is not carried in.
pub(super) fn build_from(
    dom: &Dom,
    roots: &[NodeId],
    skip: &HashSet<NodeId>,
    inline_image: &dyn Fn(&str) -> Option<Arc<[u8]>>,
) -> Document {
    let mut builder = Builder {
        dom,
        inline_image,
        skip,
        doc: Document::default(),
        bytes: 0,
        items: 0,
        counters: Vec::new(),
    };
    // Direction is carried in, unlike styles: a piece of a right-to-left
    // message still reads right to left.
    let dir = roots.first().and_then(|&root| {
        let mut at = dom.nodes[root].parent;
        while let Some(id) = at {
            let node = &dom.nodes[id];
            if let Some(dir) = node.attr("dir") {
                return Direction::from_html(dir);
            }
            at = node.parent;
        }
        None
    });
    let ctx = Ctx {
        run: RunStyle::default(),
        align: Align::Start,
        pre: false,
        cell_padding: 0.0,
        cell_border: None,
        boxes: 0,
        depth: 0,
        lists: 0,
        fit: false,
        dir,
        box_dir: None,
    };
    builder.doc.dir = dir;
    let mut out = Out::default();
    for &root in roots {
        if builder.full() {
            break;
        }
        if root == DOCUMENT {
            builder.children(root, &ctx, &mut out);
        } else {
            builder.node(root, &ctx, &mut out);
        }
    }
    out.flush();
    let mut doc = builder.doc;
    doc.blocks = out.blocks;
    doc
}

impl Builder<'_> {
    fn full(&self) -> bool {
        self.doc.truncated
    }

    fn count(&mut self, bytes: usize) {
        self.bytes += bytes;
        self.items += 1;
        if self.bytes > MAX_BODY_BYTES || self.items > MAX_ITEMS {
            self.doc.truncated = true;
        }
    }

    fn children(&mut self, id: NodeId, ctx: &Ctx, out: &mut Out) {
        for &child in &self.dom.nodes[id].children {
            if self.full() {
                return;
            }
            self.node(child, ctx, out);
        }
    }

    fn node(&mut self, id: NodeId, ctx: &Ctx, out: &mut Out) {
        if self.skip.contains(&id) {
            return;
        }
        let node = &self.dom.nodes[id];
        match &node.data {
            Data::Text(text) => {
                self.count(text.len());
                out.text(text, ctx);
            }
            Data::Element { .. } => {
                if ctx.depth < MAX_DEPTH {
                    self.element(id, ctx, out);
                }
            }
            Data::Document | Data::Other => {}
        }
    }

    fn element(&mut self, id: NodeId, parent: &Ctx, out: &mut Out) {
        let node = &self.dom.nodes[id];
        let tag = node.tag();
        // Elements outside HTML (inline SVG, MathML) have no tag here.
        if tag.is_empty() || DROPPED.contains(&tag) || node.attr("hidden").is_some() {
            return;
        }
        let decls = node
            .attr("style")
            .map(css::declarations)
            .unwrap_or_default();
        let get = |name: &str| {
            decls
                .iter()
                .rev()
                .find(|(n, _)| n == name)
                .map(|(_, v)| v.as_str())
        };
        if hidden(&get) {
            return;
        }
        self.count(0);
        let mut ctx = parent.clone();
        ctx.depth += 1;
        ctx.box_dir = None;
        if let Some(dir) = node.attr("dir") {
            ctx.dir = Direction::from_html(dir);
            ctx.box_dir = ctx
                .dir
                .filter(|&d| d != parent.dir.unwrap_or(Direction::Ltr));
        }

        // What the tag itself means, before its `style` attribute.
        let em = ctx.run.size;
        match tag {
            "b" | "strong" | "th" | "dt" => ctx.run.bold = true,
            "i" | "em" | "cite" | "var" | "dfn" | "address" => ctx.run.italic = true,
            "u" | "ins" => ctx.run.underline = true,
            "s" | "strike" | "del" => ctx.run.strike = true,
            "code" | "kbd" | "samp" | "tt" => ctx.run.monospace = true,
            "small" | "sub" | "sup" => ctx.run.size = em * 0.83,
            "big" => ctx.run.size = em * 1.2,
            "mark" => ctx.run.background = Some(0xffff00ff),
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                ctx.run.bold = true;
                ctx.run.size = em * heading_scale(tag);
            }
            "a" => {
                if let Some(href) = node.attr("href").and_then(safe_link) {
                    ctx.run.link = Some(href);
                    ctx.run.underline = true;
                    ctx.run.color = None;
                }
            }
            "font" => {
                if let Some(c) = node.attr("color").and_then(css::color) {
                    ctx.run.color = Some(c);
                    self.doc.styled = true;
                }
                if let Some(size) = node.attr("size").and_then(font_tag_size) {
                    ctx.run.size = size;
                }
                if node.attr("face").is_some_and(is_monospace) {
                    ctx.run.monospace = true;
                }
            }
            "center" => ctx.align = Align::Center,
            "pre" => {
                ctx.run.monospace = true;
                ctx.pre = true;
            }
            _ => {}
        }
        if let Some(align) = node.attr("align").and_then(align)
            && tag != "table"
            && tag != "img"
        {
            ctx.align = align;
        }
        self.apply_text_style(&get, &mut ctx);
        if ctx.run.size <= 1.5 {
            // Hidden "preheader" text.
            return;
        }

        match tag {
            "br" => out.line_break(&ctx),
            "img" => self.image(id, &get, &ctx, out),
            "hr" => {
                out.flush();
                out.blocks.push(Block::Rule);
            }
            "body" | "html" => {
                if let Some(dir) = node.attr("dir").and_then(Direction::from_html) {
                    self.doc.dir = Some(dir);
                }
                if let Some(bg) = background(node.attr("bgcolor"), &get) {
                    self.doc.background = Some(bg);
                    self.doc.styled = true;
                }
                self.children(id, &ctx, out);
            }
            "table" => self.table(id, &get, ctx, parent.align, out),
            "tr" => self.row(id, &get, ctx, out),
            // A cell outside a row (the parser puts rows around cells, so
            // this is rare): a plain box.
            "td" | "th" => {
                let style = self.box_style(id, &get, &ctx, [0.0, 0.0]);
                self.boxed(id, BoxKind::Stack, style, ctx, out);
            }
            "thead" | "tbody" | "tfoot" => self.children(id, &ctx, out),
            "ul" | "ol" | "menu" => {
                let start = node
                    .attr("start")
                    .and_then(|s| s.trim().parse::<usize>().ok())
                    .unwrap_or(1);
                self.counters.push(if tag == "ol" { start } else { 0 });
                ctx.lists += 1;
                // Lists inside lists get no space around them.
                let space = if ctx.lists > 1 { 0.0 } else { em };
                let mut style = self.box_style(id, &get, &ctx, [0.0, space]);
                style.padding[3] = style.padding[3].max(8.0);
                self.boxed(id, BoxKind::Stack, style, ctx, out);
                self.counters.pop();
            }
            "li" => {
                let marker = match self.counters.last_mut() {
                    Some(n) if *n > 0 => {
                        let marker = format!("{n}.");
                        *n += 1;
                        marker
                    }
                    _ => match ctx.lists {
                        0 | 1 => "•",
                        2 => "◦",
                        _ => "▪",
                    }
                    .to_owned(),
                };
                let style = self.box_style(id, &get, &ctx, [0.0, 0.0]);
                self.boxed(id, BoxKind::ListItem(marker), style, ctx, out);
            }
            "blockquote" => {
                let mut style = self.box_style(id, &get, &ctx, [0.0, em * 0.5]);
                // The app draws its own quote bar.
                style.border = None;
                style.padding[3] = style.padding[3].max(12.0);
                self.boxed(id, BoxKind::Quote, style, ctx, out);
            }
            "p" | "dl" | "pre" | "figure" => {
                let style = self.box_style(id, &get, &ctx, [0.0, em]);
                self.boxed(id, BoxKind::Stack, style, ctx, out);
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let space = ctx.run.size * 0.5;
                let style = self.box_style(id, &get, &ctx, [space, space]);
                self.boxed(id, BoxKind::Stack, style, ctx, out);
            }
            "dd" => {
                let mut style = self.box_style(id, &get, &ctx, [0.0, 0.0]);
                style.padding[3] = style.padding[3].max(24.0);
                self.boxed(id, BoxKind::Stack, style, ctx, out);
            }
            tag if BLOCKS.contains(&tag) || get("display").is_some_and(|d| d == "block") => {
                let style = self.box_style(id, &get, &ctx, [0.0, 0.0]);
                self.boxed(id, BoxKind::Stack, style, ctx, out);
            }
            _ => {
                let display = get("display").unwrap_or_default().to_ascii_lowercase();
                let bg = background(None, &get);
                let framed =
                    bg.is_some() || get("border").is_some_and(|b| border_value(b).is_some());
                let padded = get("padding").is_some() || get("padding-top").is_some();
                if display.starts_with("inline-block")
                    || display == "inline-table"
                    || (framed && padded)
                {
                    // A button: a box as wide as its text.
                    let mut style = self.box_style(id, &get, &ctx, [0.0, 0.0]);
                    style.inline = true;
                    style.align = parent.align;
                    self.boxed(id, BoxKind::Stack, style, ctx, out);
                } else {
                    if let Some(bg) = bg {
                        ctx.run.background = Some(bg);
                        self.doc.styled = true;
                    }
                    self.children(id, &ctx, out);
                }
            }
        }
    }

    /// The text styles of a `style` attribute.
    fn apply_text_style<'s>(&mut self, get: &dyn Fn(&str) -> Option<&'s str>, ctx: &mut Ctx) {
        let run = &mut ctx.run;
        if let Some(c) = get("color").and_then(css::color) {
            run.color = Some(c);
            self.doc.styled = true;
        }
        if let Some(size) = get("font-size").and_then(|v| css::font_size(v, run.size)) {
            run.size = size.min(72.0);
        }
        if let Some(weight) = get("font-weight") {
            run.bold = match weight.trim() {
                "bold" | "bolder" => true,
                "normal" | "lighter" => false,
                n => n.parse::<u32>().map(|n| n >= 600).unwrap_or(run.bold),
            };
        }
        if let Some(style) = get("font-style") {
            run.italic = matches!(style.trim(), "italic" | "oblique");
        }
        if let Some(family) = get("font-family") {
            run.monospace = is_monospace(family);
        }
        if let Some(decoration) = get("text-decoration").or_else(|| get("text-decoration-line")) {
            let decoration = decoration.to_ascii_lowercase();
            run.underline = decoration.contains("underline");
            run.strike = decoration.contains("line-through");
        }
        if let Some(align) = get("text-align").and_then(align) {
            ctx.align = align;
        }
        if let Some(space) = get("white-space") {
            ctx.pre = space.starts_with("pre");
        }
    }

    /// A box's own style from its attributes and `style`; `margin` is the
    /// tag's default space above and below.
    fn box_style<'s>(
        &mut self,
        id: NodeId,
        get: &dyn Fn(&str) -> Option<&'s str>,
        ctx: &Ctx,
        margin: [f32; 2],
    ) -> BoxStyle {
        let node = &self.dom.nodes[id];
        let em = ctx.run.size;
        let mut style = BoxStyle {
            margin,
            dir: ctx.box_dir,
            ..BoxStyle::default()
        };
        if let Some(bg) = background(node.attr("bgcolor"), get) {
            style.background = Some(bg);
            self.doc.styled = true;
        }
        if let Some(padding) = get("padding") {
            style.padding = css::sides(padding, em);
        }
        for (ix, side) in ["top", "right", "bottom", "left"].iter().enumerate() {
            if let Some(v) = get(&format!("padding-{side}")).and_then(|v| css::px(v, em)) {
                style.padding[ix] = v;
            }
        }
        if let Some(m) = get("margin") {
            let sides = css::sides(m, em);
            style.margin = [sides[0], sides[2]];
            let words: Vec<&str> = m.split_whitespace().collect();
            style.center = matches!(words.as_slice(), [_, "auto", ..]);
        }
        for (ix, side) in ["top", "bottom"].iter().enumerate() {
            if let Some(v) = get(&format!("margin-{side}")).and_then(|v| css::px(v, em)) {
                style.margin[ix] = v;
            }
        }
        if get("margin-left") == Some("auto") && get("margin-right") == Some("auto") {
            style.center = true;
        }
        style.width = get("width")
            .or(node.attr("width"))
            .and_then(|w| css::length(w, em))
            .filter(|w| *w != Length::Px(0.0));
        style.max_width = get("max-width").and_then(|w| css::px(w, em));
        if let Some(border) = get("border") {
            style.border = border_value(border);
        }
        if let Some(w) = get("border-width").and_then(|w| css::px(w, em)) {
            let color = get("border-color")
                .and_then(css::color)
                .unwrap_or(0x000000ff);
            style.border = Some((w, color));
        }
        let visible = |b: Option<(f32, u32)>| b.filter(|(w, c)| *w > 0.0 && c & 0xff != 0);
        style.border = visible(style.border);
        style.border_top = visible(get("border-top").and_then(border_value));
        style.border_bottom = visible(get("border-bottom").and_then(border_value));
        style.border_left = visible(get("border-left").and_then(border_value));
        style.border_right = visible(get("border-right").and_then(border_value));
        if let Some(r) =
            get("border-radius").and_then(|r| css::px(r.split_whitespace().next()?, em))
        {
            style.radius = r;
        }
        style
    }

    /// Walks `id`'s children into a new box and adds it to `out`.
    fn boxed(&mut self, id: NodeId, kind: BoxKind, style: BoxStyle, ctx: Ctx, out: &mut Out) {
        if ctx.boxes >= MAX_BOX_DEPTH {
            out.flush();
            self.children(id, &ctx, out);
            out.flush();
            return;
        }
        let mut inner_ctx = ctx;
        inner_ctx.boxes += 1;
        let mut inner = Out::default();
        self.children(id, &inner_ctx, &mut inner);
        inner.flush();
        out.flush();
        push_box(&mut out.blocks, kind, style, inner.blocks);
    }

    /// `inherited` is the text alignment around the table.
    // Kept out of `element`, whose stack frame every level of nesting pays.
    #[inline(never)]
    fn table<'s>(
        &mut self,
        id: NodeId,
        get: &dyn Fn(&str) -> Option<&'s str>,
        mut ctx: Ctx,
        inherited: Align,
        out: &mut Out,
    ) {
        let node = &self.dom.nodes[id];
        let mut style = self.box_style(id, get, &ctx, [0.0, 0.0]);
        // `<td align="center">` centers the tables in it; their text keeps
        // its own alignment, as in the quirks mode most mail is written for.
        if inherited == Align::Center
            || node
                .attr("align")
                .is_some_and(|a| a.eq_ignore_ascii_case("center"))
        {
            style.center = true;
        }
        ctx.align = get("text-align").and_then(align).unwrap_or_default();
        ctx.cell_border = None;
        ctx.fit = style.width.is_none();
        if let Some(w) = node.attr("border").and_then(|b| css::px(b, 16.0))
            && w > 0.0
        {
            ctx.cell_border = Some((1.0, 0x808080ff));
            if style.border.is_none() {
                style.border = Some((w, 0x808080ff));
            }
        }
        ctx.cell_padding = node
            .attr("cellpadding")
            .and_then(|p| css::px(p, 16.0))
            .unwrap_or(if ctx.cell_border.is_some() { 1.0 } else { 0.0 })
            .min(64.0);
        if ctx.boxes >= MAX_BOX_DEPTH {
            self.boxed(id, BoxKind::Stack, style, ctx, out);
            return;
        }
        let mut inner_ctx = ctx;
        inner_ctx.boxes += 1;
        let mut inner = Out::default();
        self.children(id, &inner_ctx, &mut inner);
        inner.flush();
        share_columns(&mut inner.blocks);
        out.flush();
        push_box(&mut out.blocks, BoxKind::Stack, style, inner.blocks);
    }

    #[inline(never)]
    fn row<'s>(
        &mut self,
        id: NodeId,
        get: &dyn Fn(&str) -> Option<&'s str>,
        ctx: Ctx,
        out: &mut Out,
    ) {
        out.flush();
        let style = self.box_style(id, get, &ctx, [0.0, 0.0]);
        let mut cells = Vec::new();
        let mut cell_ctx = ctx.clone();
        cell_ctx.boxes += 1;
        for &child in &self.dom.nodes[id].children {
            if self.full() {
                break;
            }
            let node = &self.dom.nodes[child];
            if !matches!(node.tag(), "td" | "th") || self.skip.contains(&child) {
                continue;
            }
            let decls = node
                .attr("style")
                .map(css::declarations)
                .unwrap_or_default();
            let get = |name: &str| {
                decls
                    .iter()
                    .rev()
                    .find(|(n, _)| n == name)
                    .map(|(_, v)| v.as_str())
            };
            if node.attr("hidden").is_some() || hidden(&get) {
                continue;
            }
            let mut ctx = cell_ctx.clone();
            ctx.depth += 1;
            if node.tag() == "th" {
                ctx.run.bold = true;
                ctx.align = Align::Center;
            }
            if let Some(a) = node.attr("align").and_then(align) {
                ctx.align = a;
            }
            self.apply_text_style(&get, &mut ctx);
            let mut style = self.box_style(child, &get, &ctx, [0.0, 0.0]);
            if get("padding").is_none() && !decls.iter().any(|(n, _)| n.starts_with("padding-")) {
                style.padding = [ctx.cell_padding; 4];
            }
            if style.border.is_none() {
                style.border = ctx.cell_border;
            }
            let mut inner = Out::default();
            if ctx.boxes >= MAX_BOX_DEPTH {
                self.children(child, &ctx, out);
                out.flush();
                continue;
            }
            self.children(child, &ctx, &mut inner);
            inner.flush();
            let empty = inner.blocks.is_empty();
            // An empty cell still holds its place when it has a width
            // or a color: it is a gutter or a colored bar.
            if empty && style.width.is_none() && style.background.is_none() {
                continue;
            }
            let nowrap = node.attr("nowrap").is_some()
                || get("white-space").is_some_and(|w| w.trim().eq_ignore_ascii_case("nowrap"));
            let border = style.border.map_or(0.0, |(w, _)| 2.0 * w);
            if !empty {
                style.min_width = (min_content(&inner.blocks, nowrap)
                    + style.padding[1]
                    + style.padding[3]
                    + border)
                    .min(MAX_CELL_MIN);
                // In a table as wide as its content, a cell of pictures (a
                // signature's logo) is as wide as they are; the cells of
                // text share the rest.
                if ctx.fit && style.width.is_none() && only_images(&inner.blocks) {
                    style.width = Some(Length::Px(style.min_width));
                }
            }
            cells.push(Block::Box(BoxBlock {
                kind: BoxKind::Stack,
                style,
                children: inner.blocks,
            }));
        }
        let has_content = cells.iter().any(|c| match c {
            Block::Box(b) => !b.children.is_empty(),
            _ => true,
        });
        if !has_content && style.background.is_none() {
            return;
        }
        fit_row_mins(&mut cells);
        if cells.len() == 1 {
            // A one-cell row is just the cell.
            let Some(Block::Box(mut cell)) = cells.pop() else {
                return;
            };
            if cell.style.background.is_none() {
                cell.style.background = style.background;
            }
            // Its width is the table's business.
            cell.style.width = None;
            cell.style.min_width = 0.0;
            push_box(&mut out.blocks, BoxKind::Stack, cell.style, cell.children);
            return;
        }
        out.blocks.push(Block::Box(BoxBlock {
            kind: BoxKind::Row,
            style,
            children: cells,
        }));
    }

    fn image<'s>(
        &mut self,
        id: NodeId,
        get: &dyn Fn(&str) -> Option<&'s str>,
        ctx: &Ctx,
        out: &mut Out,
    ) {
        let node = &self.dom.nodes[id];
        let em = ctx.run.size;
        let alt = node.attr("alt").unwrap_or_default().trim().to_owned();
        let width = get("width")
            .or(node.attr("width"))
            .and_then(|w| css::length(w, em));
        let height = get("height")
            .or(node.attr("height"))
            .and_then(|h| css::px(h, em));
        let src = node.attr("src").unwrap_or_default().trim();
        let tiny = matches!(width, Some(Length::Px(w)) if w <= 2.0)
            || matches!(height, Some(h) if h <= 2.0);
        let source = if let Some(cid) = src.strip_prefix("cid:") {
            let cid = cid.trim_start_matches('<').trim_end_matches('>');
            let found = (self.inline_image)(cid).and_then(data_source);
            if found.is_some() {
                self.doc.inline_ids.push(cid.to_owned());
            }
            found
        } else if let Some(data) = src.strip_prefix("data:") {
            data_url(data).and_then(data_source)
        } else if let Some(url) = remote_url(src) {
            let lower = url.to_ascii_lowercase();
            if tiny || tracker_path(&lower) {
                self.doc.trackers += 1;
                return;
            }
            self.doc.remote_images += 1;
            Some(ImageSource::Remote(url))
        } else {
            None
        };
        if tiny {
            return;
        }
        match source {
            Some(source) => {
                self.count(0);
                out.image(
                    Image {
                        source,
                        alt,
                        width,
                        height,
                        link: ctx.run.link.clone(),
                    },
                    ctx,
                );
            }
            None if !alt.is_empty() => out.text(&format!("[{alt}]"), ctx),
            None => {}
        }
    }
}

/// Adds a box to `blocks`, leaving out what draws nothing: a plain box is
/// replaced by its children, an empty box without a color is dropped.
fn push_box(blocks: &mut Vec<Block>, kind: BoxKind, style: BoxStyle, children: Vec<Block>) {
    let plain = style.background.is_none()
        && style.border.is_none()
        && style.border_top.is_none()
        && style.border_bottom.is_none()
        && style.border_left.is_none()
        && style.border_right.is_none()
        && !style.inline
        && style.padding == [0.0; 4]
        && style.width.is_none()
        && style.max_width.is_none()
        && style.min_width == 0.0
        && !style.center
        && style.dir.is_none();
    if children.is_empty() && (style.background.is_none() || kind != BoxKind::Stack) {
        return;
    }
    if kind == BoxKind::Stack && plain {
        if style.margin == [0.0; 2] {
            blocks.extend(children);
            return;
        }
        // A box that only adds space around one box: move the space in.
        if let [Block::Box(only)] = children.as_slice()
            && only.style.margin == [0.0; 2]
        {
            let mut only = only.clone();
            only.style.margin = style.margin;
            blocks.push(Block::Box(only));
            return;
        }
    }
    blocks.push(Block::Box(BoxBlock {
        kind,
        style,
        children,
    }));
}

/// Rows of one table line up, as in a browser: a column takes the width
/// the first row that sets one gives it, and in every row it is as wide as
/// the longest word of its cells. Rows with another number of cells
/// (spans, empty cells left out) keep their own.
fn share_columns(blocks: &mut [Block]) {
    let mut groups: Vec<(usize, Vec<usize>)> = Vec::new();
    for (ix, block) in blocks.iter().enumerate() {
        if let Block::Box(b) = block
            && b.kind == BoxKind::Row
        {
            let n = b.children.len();
            match groups.iter_mut().find(|(len, _)| *len == n) {
                Some((_, rows)) => rows.push(ix),
                None => groups.push((n, vec![ix])),
            }
        }
    }
    for (n, rows) in groups {
        if rows.len() < 2 {
            continue;
        }
        let mut widths: Vec<Option<Length>> = vec![None; n];
        let mut mins = vec![0.0f32; n];
        for &ix in &rows {
            let Block::Box(row) = &blocks[ix] else {
                continue;
            };
            for (col, cell) in row.children.iter().enumerate() {
                if let Block::Box(cell) = cell {
                    if widths[col].is_none() {
                        widths[col] = cell.style.width;
                    }
                    mins[col] = mins[col].max(cell.style.min_width);
                }
            }
        }
        for &ix in &rows {
            let Block::Box(row) = &mut blocks[ix] else {
                continue;
            };
            for (col, cell) in row.children.iter_mut().enumerate() {
                if let Block::Box(cell) = cell {
                    if cell.style.width.is_none() {
                        cell.style.width = widths[col];
                    }
                    cell.style.min_width = mins[col];
                }
            }
            fit_row_mins(&mut row.children);
        }
    }
}

/// Keeps the cells of a row from asking for more than a narrow pane has
/// in all: past [`MAX_ROW_MIN`], every cell's least width shrinks alike.
fn fit_row_mins(cells: &mut [Block]) {
    let total: f32 = cells
        .iter()
        .map(|c| match c {
            Block::Box(b) => b.style.min_width,
            _ => 0.0,
        })
        .sum();
    if total <= MAX_ROW_MIN {
        return;
    }
    let scale = MAX_ROW_MIN / total;
    for cell in cells {
        if let Block::Box(b) = cell {
            b.style.min_width *= scale;
        }
    }
}

/// An estimate of the narrowest `blocks` can be laid out: their longest
/// word or image, or with `nowrap` their longest line. Glyphs count a
/// little wider than they are on average: a cell a few pixels too wide
/// reads better than one that breaks a word.
/// How many pixels wide a picture carried in the message is, read from
/// its header (PNG, GIF, JPEG).
fn natural_width(source: &ImageSource) -> Option<f32> {
    let ImageSource::Data { bytes, .. } = source else {
        return None;
    };
    let b: &[u8] = bytes;
    let be16 = |i: usize| Some(u16::from_be_bytes([*b.get(i)?, *b.get(i + 1)?]));
    if b.starts_with(b"\x89PNG") {
        let w = u32::from_be_bytes(b.get(16..20)?.try_into().ok()?);
        return Some(w as f32);
    }
    if b.starts_with(b"GIF8") {
        return Some(f32::from(u16::from_le_bytes([*b.get(6)?, *b.get(7)?])));
    }
    if b.starts_with(&[0xff, 0xd8]) {
        // The first start-of-frame segment holds the size.
        let mut i = 2;
        while i + 9 < b.len() {
            if b[i] != 0xff {
                return None;
            }
            let marker = b[i + 1];
            let len = usize::from(be16(i + 2)?);
            if matches!(marker, 0xc0..=0xcf) && !matches!(marker, 0xc4 | 0xc8 | 0xcc) {
                return Some(f32::from(be16(i + 7)?));
            }
            i += 2 + len;
        }
    }
    None
}

/// Blocks that show only pictures.
fn only_images(blocks: &[Block]) -> bool {
    let mut any = false;
    let all = blocks.iter().all(|block| match block {
        Block::Text(t) => t.inlines.iter().all(|inline| match inline {
            Inline::Image(_) => {
                any = true;
                true
            }
            Inline::Text(run) => run.text.trim().is_empty(),
        }),
        Block::Box(b) => {
            b.kind != BoxKind::Row && only_images(&b.children) && {
                any = true;
                true
            }
        }
        Block::Rule => false,
    });
    all && any
}

fn min_content(blocks: &[Block], nowrap: bool) -> f32 {
    blocks
        .iter()
        .map(|block| match block {
            Block::Rule => 0.0,
            Block::Text(t) => text_min(t, nowrap || t.preformatted),
            Block::Box(b) => {
                let inner = match b.kind {
                    BoxKind::Row => b
                        .children
                        .iter()
                        .map(|c| min_content(std::slice::from_ref(c), nowrap))
                        .sum(),
                    BoxKind::ListItem(_) => 28.0 + min_content(&b.children, nowrap),
                    _ => min_content(&b.children, nowrap),
                };
                let border = b.style.border.map_or(0.0, |(w, _)| 2.0 * w);
                (inner + b.style.padding[1] + b.style.padding[3] + border).max(b.style.min_width)
            }
        })
        .fold(0.0, f32::max)
}

/// The longest unbreakable piece of a paragraph: a word (across runs, as
/// `<b>bold</b>face` is one word), an image, or a line when `nowrap`.
fn text_min(t: &TextBlock, nowrap: bool) -> f32 {
    let mut widest = 0.0f32;
    let mut word = 0.0f32;
    for inline in &t.inlines {
        match inline {
            Inline::Text(run) => {
                let s = &run.style;
                let glyph = s.size
                    * if s.monospace {
                        0.62
                    } else if s.bold {
                        0.6
                    } else {
                        0.55
                    };
                for c in run.text.chars() {
                    if c == '\n' || (!nowrap && c.is_whitespace()) {
                        widest = widest.max(word);
                        word = 0.0;
                    } else if !nowrap && is_wide(c) {
                        // East Asian text breaks between any two characters.
                        widest = widest.max(word).max(s.size);
                        word = 0.0;
                    } else {
                        word += if is_wide(c) { s.size } else { glyph };
                    }
                }
            }
            Inline::Image(image) => {
                let w = match image.width {
                    Some(Length::Px(w)) => w,
                    Some(_) => 0.0,
                    // Without a width, a picture inside is as wide as it is.
                    None => natural_width(&image.source).unwrap_or(0.0),
                };
                if nowrap {
                    word += w;
                } else {
                    widest = widest.max(word).max(w);
                    word = 0.0;
                }
            }
        }
    }
    widest.max(word)
}

/// A character East Asian text sets a full em wide.
fn is_wide(c: char) -> bool {
    matches!(c as u32, 0x1100..=0x115f | 0x2e80..=0xa4cf | 0xac00..=0xd7a3 | 0xf900..=0xfaff | 0xfe30..=0xfe4f | 0xff00..=0xff60 | 0x20000..=0x3fffd)
}

fn hidden<'s>(get: &dyn Fn(&str) -> Option<&'s str>) -> bool {
    let is = |name: &str, values: &[&str]| {
        get(name).is_some_and(|v| {
            let v = v.trim().to_ascii_lowercase();
            values.iter().any(|x| v == *x)
        })
    };
    let zero = &["0", "0px", "0pt", "0em", "0%"];
    is("display", &["none"])
        || is("visibility", &["hidden", "collapse"])
        || is("opacity", &["0", "0.0", "0%"])
        || ((is("max-height", zero) || is("height", zero) || is("max-width", zero))
            && is("overflow", &["hidden"]))
}

fn heading_scale(tag: &str) -> f32 {
    match tag {
        "h1" => 2.0,
        "h2" => 1.5,
        "h3" => 1.17,
        "h5" => 0.83,
        "h6" => 0.67,
        _ => 1.0,
    }
}

fn align(value: &str) -> Option<Align> {
    match value.trim().to_ascii_lowercase().as_str() {
        "left" | "start" | "justify" => Some(Align::Start),
        "center" | "middle" | "-webkit-center" => Some(Align::Center),
        "right" | "end" => Some(Align::End),
        _ => None,
    }
}

fn is_monospace(family: &str) -> bool {
    let family = family.to_ascii_lowercase();
    [
        "monospace",
        "courier",
        "consolas",
        "menlo",
        "monaco",
        "mono",
    ]
    .iter()
    .any(|m| family.contains(m))
}

/// `<font size="1">` to `"7"`, in pixels.
fn font_tag_size(size: &str) -> Option<f32> {
    let size = size.trim();
    let n: i32 = match size.strip_prefix('+') {
        Some(rel) => 3 + rel.parse::<i32>().ok()?,
        None => match size.strip_prefix('-') {
            Some(rel) => 3 - rel.parse::<i32>().ok()?,
            None => size.parse().ok()?,
        },
    };
    Some(match n.clamp(1, 7) {
        1 => 10.0,
        2 => 13.0,
        3 => 16.0,
        4 => 18.0,
        5 => 24.0,
        6 => 32.0,
        _ => 48.0,
    })
}

/// A box's background: `background-color`, the color in a `background`
/// shorthand, or a `bgcolor` attribute. Fully transparent is none.
fn background<'s>(bgcolor: Option<&str>, get: &dyn Fn(&str) -> Option<&'s str>) -> Option<u32> {
    get("background-color")
        .and_then(css::color)
        .or_else(|| get("background")?.split_whitespace().find_map(css::color))
        .or_else(|| bgcolor.and_then(css::color))
        .filter(|c| c & 0xff != 0)
}

/// `1px solid #ccc` as width and color.
fn border_value(value: &str) -> Option<(f32, u32)> {
    let mut width = None;
    let mut color = None;
    let mut style_none = false;
    for word in value.split_whitespace() {
        match word.to_ascii_lowercase().as_str() {
            "none" | "hidden" => style_none = true,
            "thin" => width = Some(1.0),
            "medium" => width = Some(3.0),
            "thick" => width = Some(5.0),
            w => {
                if let Some(px) =
                    css::px(w, 16.0).filter(|_| w.starts_with(|c: char| c.is_ascii_digit()))
                {
                    width = Some(px);
                } else if let Some(c) = css::color(w) {
                    color = Some(c);
                }
            }
        }
    }
    (!style_none).then(|| (width.unwrap_or(3.0), color.unwrap_or(0x000000ff)))
}

/// A link target that is safe to open: `http`, `https` or `mailto`.
fn safe_link(href: &str) -> Option<String> {
    let href = href.trim();
    let lower = href.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://") || lower.starts_with("mailto:"))
        .then(|| href.to_owned())
}

/// A remote image URL, upgraded to `https`.
/// A lower-case address at a known open-tracking path.
pub(super) fn tracker_path(lower: &str) -> bool {
    TRACKER_PATHS.iter().any(|p| lower.contains(p))
}

pub(super) fn remote_url(src: &str) -> Option<String> {
    let lower = src.to_ascii_lowercase();
    if lower.starts_with("https://") {
        Some(src.to_owned())
    } else if lower.starts_with("http://") {
        Some(format!("https://{}", &src[7..]))
    } else if lower.starts_with("//") {
        Some(format!("https:{src}"))
    } else {
        None
    }
}

fn data_source(bytes: Arc<[u8]>) -> Option<ImageSource> {
    let kind = ImageKind::sniff(&bytes)?;
    Some(ImageSource::Data { kind, bytes })
}

/// The bytes of a `data:image/…;base64,…` URL (after `data:`).
fn data_url(data: &str) -> Option<Arc<[u8]>> {
    let (meta, payload) = data.split_once(',')?;
    let meta = meta.to_ascii_lowercase();
    if !meta.starts_with("image/") || !meta.ends_with(";base64") {
        return None;
    }
    base64(payload).map(Arc::from)
}

fn base64(text: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    let mut acc = 0u32;
    let mut bits = 0;
    for b in text.bytes() {
        let v = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            b'=' => break,
            b' ' | b'\t' | b'\r' | b'\n' => continue,
            b'%' => return None,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::super::document;
    use super::*;

    fn doc(html: &str) -> Document {
        document(html, &|_| None)
    }

    /// The document as a compact outline for asserts.
    fn outline(blocks: &[Block]) -> String {
        blocks
            .iter()
            .map(|b| match b {
                Block::Rule => "---".to_owned(),
                Block::Text(t) => format!("\"{}\"", t.text()),
                Block::Box(b) => {
                    let kind = match &b.kind {
                        BoxKind::Stack => "box".to_owned(),
                        BoxKind::Row => "row".to_owned(),
                        BoxKind::Quote => "quote".to_owned(),
                        BoxKind::ListItem(m) => format!("li{m}"),
                    };
                    format!("{kind}[{}]", outline(&b.children))
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Every paragraph's text and direction, in order.
    fn directions(blocks: &[Block]) -> Vec<(String, Option<Direction>, Align)> {
        let mut out = Vec::new();
        for b in blocks {
            match b {
                Block::Text(t) => out.push((t.text(), t.dir, t.align)),
                Block::Box(b) => out.extend(directions(&b.children)),
                Block::Rule => {}
            }
        }
        out
    }

    #[test]
    fn paragraphs_read_their_own_way() {
        let d = doc("<div dir=\"ltr\"><p>Hi Sara,</p><p>شكرًا يا رافي</p>\
            <div dir=\"rtl\"><p style=\"text-align:right\">Invoice ٤</p><p>123</p>\
            <p dir=\"auto\">456</p></div></div><p>مرحبا</p><p>7</p>");
        use Direction::{Ltr, Rtl};
        assert_eq!(
            directions(&d.blocks),
            [
                ("Hi Sara,".to_owned(), Some(Ltr), Align::Start),
                // An explicit `dir` wins over the first strong character.
                ("شكرًا يا رافي".to_owned(), Some(Ltr), Align::Start),
                // Right-aligned right-to-left text starts at the right.
                ("Invoice ٤".to_owned(), Some(Rtl), Align::Start),
                ("123".to_owned(), Some(Rtl), Align::Start),
                ("456".to_owned(), None, Align::Start),
                ("مرحبا".to_owned(), Some(Rtl), Align::Start),
                ("7".to_owned(), None, Align::Start),
            ]
        );
        assert_eq!(d.dir, None);
        // Only a box turning the other way is kept for its direction.
        assert_eq!(
            outline(&d.blocks[..2]),
            "box[\"Hi Sara,\"] box[\"شكرًا يا رافي\"]"
        );
        let Block::Box(b) = &d.blocks[2] else {
            panic!("a box");
        };
        assert_eq!(b.style.dir, Some(Rtl));
        let d = doc("<html dir=\"rtl\"><body><p>Hello</p><p>٣</p></body></html>");
        assert_eq!(d.dir, Some(Rtl));
        assert_eq!(
            directions(&d.blocks)
                .into_iter()
                .map(|(_, dir, _)| dir)
                .collect::<Vec<_>>(),
            [Some(Rtl), Some(Rtl)]
        );
    }

    fn runs(block: &Block) -> Vec<&Run> {
        match block {
            Block::Text(t) => t
                .inlines
                .iter()
                .filter_map(|i| match i {
                    Inline::Text(r) => Some(r),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    #[test]
    fn paragraphs_and_white_space() {
        let d = doc("<p>  Hello\n   <b>big</b>  world </p><div>next<br>line<br></div>");
        assert_eq!(
            outline(&d.blocks),
            "box[\"Hello big world\"] \"next\nline\""
        );
        let r = runs(match &d.blocks[0] {
            Block::Box(b) => &b.children[0],
            _ => unreachable!(),
        });
        assert_eq!(r.len(), 3);
        assert!(r[1].style.bold && !r[0].style.bold);
        assert!(!d.styled);
    }

    #[test]
    fn blank_lines_from_gmail_markup() {
        let d = doc("<div>one</div><div><br></div><div>two</div><p>&nbsp;</p>");
        assert_eq!(outline(&d.blocks), "\"one\" \" \" \"two\" box[\" \"]");
    }

    #[test]
    fn scripts_styles_and_forms_never_show() {
        let d = doc("<html><head><title>T</title><style>p{color:red}</style>\
             <script>alert(1)</script></head><body>\
             <p onclick=\"x()\">ok</p><iframe src=\"https://e.test\"></iframe>\
             <form action=\"https://e.test\"><input value=\"secret\"><button>Go</button></form>\
             <svg><text>svg</text></svg><noscript>ns</noscript></body></html>");
        assert_eq!(outline(&d.blocks), "box[\"ok\"]");
    }

    #[test]
    fn hidden_preheaders_are_dropped() {
        let d = doc("<div style=\"display:none !important\">preheader</div>\
             <span style=\"font-size:0;max-height:0;overflow:hidden\">x</span>\
             <div style=\"max-height:0px; overflow:hidden\">y</div>\
             <div style=\"opacity:0\">z</div><div hidden>h</div>shown");
        assert_eq!(outline(&d.blocks), "\"shown\"");
    }

    #[test]
    fn links_are_limited_to_the_web_and_mail() {
        let d = doc(
            "<a href=\"https://e.test/a\">web</a> <a href=\"javascript:alert(1)\">js</a> \
             <a href=\"mailto:a@e.test\">mail</a> <a href=\"file:///etc/passwd\">file</a>",
        );
        let links: Vec<_> = runs(&d.blocks[0])
            .iter()
            .map(|r| (r.text.trim().to_owned(), r.style.link.clone()))
            .filter(|(t, _)| !t.is_empty())
            .collect();
        assert_eq!(
            links,
            [
                ("web".to_owned(), Some("https://e.test/a".to_owned())),
                ("js".to_owned(), None),
                ("mail".to_owned(), Some("mailto:a@e.test".to_owned())),
                ("file".to_owned(), None),
            ]
        );
    }

    #[test]
    fn a_signature_logo_keeps_its_width() {
        // A table without a width is as wide as its content: the logo's
        // cell is as wide as the logo, and the text has the rest.
        let d = doc("<table cellpadding=\"0\"><tr>\
             <td style=\"padding-right:14px\"><img src=\"data:image/png;base64,iVBORw0KGgo=\" width=\"64\" height=\"64\"></td>\
             <td style=\"border-left:2px solid #0e7c86;padding-left:14px\"><b>Demo Alam</b><br>Accounts</td>\
             </tr></table>");
        // The table adds nothing around its row.
        let Block::Box(row) = &d.blocks[0] else {
            unreachable!()
        };
        assert_eq!(row.kind, BoxKind::Row);
        let styles: Vec<&BoxStyle> = row
            .children
            .iter()
            .map(|c| match c {
                Block::Box(b) => &b.style,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(styles[0].width, Some(Length::Px(78.0)));
        assert_eq!(styles[1].width, None);
        assert_eq!(styles[1].border_left, Some((2.0, 0x0e7c86ff)));
        // A logo without a width is as wide as the picture is.
        let d = doc(
            "<table><tr><td style=\"padding-right:10px\"><img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAIAAAA=\"></td>\
             <td><b>Ravi Menon</b><br>Sales</td></tr></table>",
        );
        let Block::Box(row) = &d.blocks[0] else {
            unreachable!()
        };
        let Block::Box(logo) = &row.children[0] else {
            unreachable!()
        };
        assert_eq!(logo.style.width, Some(Length::Px(58.0)));
    }

    #[test]
    fn narrow_columns_keep_their_words_and_line_up() {
        // As GitHub's Actions mail: 1% columns around a wide one.
        let d = doc(
            "<table width=\"100%\" style=\"border-top:1px solid #d0d7de\">\
             <tr><th width=\"1%\">Status</th><th>Job</th><th width=\"1%\">Annotations</th></tr>\
             <tr><td><img src=\"data:image/png;base64,iVBORw0KGgo=\" width=\"24\" height=\"24\"></td>\
             <td><b>Windows package</b> / build<br>Succeeded in 39 minutes</td>\
             <td nowrap>1 annotation</td></tr></table>",
        );
        let Block::Box(table) = &d.blocks[0] else {
            unreachable!()
        };
        assert!(table.style.border_top.is_some() && table.style.border.is_none());
        let cells = |row: usize| -> Vec<BoxStyle> {
            let Block::Box(r) = &table.children[row] else {
                unreachable!()
            };
            assert_eq!(r.kind, BoxKind::Row);
            r.children
                .iter()
                .map(|c| match c {
                    Block::Box(b) => b.style.clone(),
                    _ => unreachable!(),
                })
                .collect()
        };
        let (head, body) = (cells(0), cells(1));
        // "Annotations" in bold is wider than "1 annotation" set on one line.
        assert!(
            head[2].min_width > 11.0 * 16.0 * 0.5,
            "{}",
            head[2].min_width
        );
        // "Status" is wider than the 24 px icon under it.
        assert!(head[0].min_width > 24.0);
        for col in 0..3 {
            assert_eq!(head[col].min_width, body[col].min_width);
            assert_eq!(head[col].width, body[col].width);
        }
        assert_eq!(body[0].width, Some(Length::Percent(0.01)));
    }

    #[test]
    fn newsletter_tables() {
        let d = doc(
            "<body bgcolor=\"#f3f2ef\"><table width=\"100%\"><tr><td align=\"center\">\
             <table width=\"600\" align=\"center\" bgcolor=\"#ffffff\" cellpadding=\"8\">\
             <tr><td style=\"color:#333;font-size:14px\">Hello <b>Ada</b></td></tr>\
             <tr><td width=\"70%\">left</td><td width=\"30%\" style=\"padding:0\">right</td></tr>\
             </table></td></tr></table></body>",
        );
        assert_eq!(d.background, Some(0xf3f2efff));
        assert!(d.styled);
        assert_eq!(
            outline(&d.blocks),
            "box[box[box[\"Hello Ada\"] row[box[\"left\"] box[\"right\"]]]]"
        );
        let Block::Box(outer) = &d.blocks[0] else {
            unreachable!()
        };
        assert_eq!(outer.style.width, Some(Length::Percent(1.0)));
        let Block::Box(inner) = &outer.children[0] else {
            unreachable!()
        };
        assert!(inner.style.center);
        let Block::Box(hello_box) = &inner.children[0] else {
            unreachable!()
        };
        let Block::Text(hello_text) = &hello_box.children[0] else {
            unreachable!()
        };
        assert_eq!(
            hello_text.align,
            Align::Start,
            "text does not inherit the cell's center"
        );
        assert_eq!(inner.style.width, Some(Length::Px(600.0)));
        assert_eq!(inner.style.background, Some(0xffffffff));
        let Block::Box(row) = &inner.children[1] else {
            unreachable!()
        };
        let [Block::Box(left), Block::Box(right)] = row.children.as_slice() else {
            unreachable!()
        };
        assert_eq!(left.style.padding, [8.0; 4]);
        assert_eq!(left.style.width, Some(Length::Percent(0.7)));
        assert_eq!(right.style.padding, [0.0; 4]);
        let Block::Box(hello) = &inner.children[0] else {
            unreachable!()
        };
        let r = runs(&hello.children[0]);
        assert_eq!(r[0].style.color, Some(0x333333ff));
        assert_eq!(r[0].style.size, 14.0);
        assert!(r[1].style.bold);
    }

    #[test]
    fn lists_quotes_rules_and_pre() {
        let d = doc(
            "<ol start=\"3\"><li>three</li><li>four<ul><li>sub</li></ul></li></ol>\
             <hr><blockquote>quoted</blockquote><pre>a  b\n c</pre>",
        );
        assert_eq!(
            outline(&d.blocks),
            "box[li3.[\"three\"] li4.[\"four\" box[li◦[\"sub\"]]]] --- quote[\"quoted\"] \
             box[\"a  b\n c\"]"
        );
    }

    #[test]
    fn images_inline_remote_and_trackers() {
        let png: &[u8] = b"\x89PNG\r\n\x1a\n rest";
        let d = document(
            "<img src=\"cid:logo@x\" alt=\"Logo\" width=\"120\">\
             <img src=\"http://cdn.e.test/hero.jpg\" width=\"600\" height=\"200\">\
             <img src=\"https://t.e.test/o.gif\" width=\"1\" height=\"1\">\
             <img src=\"https://sendgrid.test/wf/open?upn=x\">\
             <img src=\"data:image/png;base64,iVBORw0KGgo=\">\
             <img src=\"cid:missing\" alt=\"Chart\">",
            &|cid| (cid == "logo@x").then(|| Arc::from(png)),
        );
        assert_eq!(d.remote_images, 1);
        assert_eq!(d.trackers, 2);
        assert_eq!(d.inline_ids, ["logo@x"]);
        let Block::Text(t) = &d.blocks[0] else {
            unreachable!()
        };
        let images: Vec<&Image> = t
            .inlines
            .iter()
            .filter_map(|i| match i {
                Inline::Image(img) => Some(img),
                _ => None,
            })
            .collect();
        assert_eq!(images.len(), 3);
        assert!(matches!(
            &images[0].source,
            ImageSource::Data {
                kind: ImageKind::Png,
                ..
            }
        ));
        assert_eq!(images[0].width, Some(Length::Px(120.0)));
        assert_eq!(
            images[1].source,
            ImageSource::Remote("https://cdn.e.test/hero.jpg".into())
        );
        assert_eq!(images[1].height, Some(200.0));
        assert!(matches!(
            &images[2].source,
            ImageSource::Data { kind: ImageKind::Png, bytes } if bytes.len() == 8
        ));
        assert!(t.text().contains("[Chart]"));
    }

    #[test]
    fn buttons_and_highlights() {
        let d = doc(
            "<p style=\"text-align:center\"><a href=\"https://e.test/d\" \
             style=\"background:#6d28d9;color:#fff;padding:12px 20px;border-radius:6px\">\
             Download</a></p><p>a <span style=\"background-color:yellow\">marked</span> word</p>",
        );
        // The paragraph only adds space around the button.
        let Block::Box(button) = &d.blocks[0] else {
            panic!("{:?}", d.blocks)
        };
        assert!(button.style.inline);
        assert_eq!(button.style.align, Align::Center);
        assert_eq!(button.style.background, Some(0x6d28d9ff));
        assert_eq!(button.style.radius, 6.0);
        let r = runs(&button.children[0]);
        assert_eq!(r[0].style.link.as_deref(), Some("https://e.test/d"));
        assert_eq!(r[0].style.color, Some(0xffffffff));
        let Block::Box(p2) = &d.blocks[1] else {
            unreachable!()
        };
        let r = runs(&p2.children[0]);
        assert_eq!(r[1].text, "marked");
        assert_eq!(r[1].style.background, Some(0xffff00ff));
    }

    #[test]
    fn linked_images_keep_their_link() {
        let d = doc("<a href=\"https://e.test/\"><img src=\"https://e.test/b.png\"></a>");
        let Block::Text(t) = &d.blocks[0] else {
            unreachable!()
        };
        let Inline::Image(img) = &t.inlines[0] else {
            unreachable!()
        };
        assert_eq!(img.link.as_deref(), Some("https://e.test/"));
    }

    #[test]
    fn deep_nesting_is_bounded() {
        let html = "<div style=\"padding:1px\">".repeat(5000) + "deep";
        let d = doc(&html);
        fn depth(blocks: &[Block]) -> usize {
            blocks
                .iter()
                .map(|b| match b {
                    Block::Box(b) => 1 + depth(&b.children),
                    _ => 0,
                })
                .max()
                .unwrap_or(0)
        }
        assert!(depth(&d.blocks) <= MAX_BOX_DEPTH, "{}", depth(&d.blocks));
    }

    #[test]
    fn huge_bodies_are_cut() {
        let html = "<p>word word word</p>".repeat(40_000);
        let d = doc(&html);
        assert!(d.truncated);
    }

    #[test]
    fn plain_text_that_is_html() {
        assert!(super::super::looks_like_html(
            "\n<!DOCTYPE html>\n<html><body>x</body></html>\n"
        ));
        assert!(super::super::looks_like_html(
            "<table><tr><td>x</td></tr></table>"
        ));
        assert!(!super::super::looks_like_html(
            "Hi <b>, see <div> in the spec"
        ));
    }
}
