// SPDX-License-Identifier: GPL-3.0-or-later

//! Writes marks into a copy of a PDF as standard annotations (Highlight,
//! Underline, Squiggly, StrikeOut, Ink), each with its own appearance so
//! every reader draws it the same way. The copy is the original file
//! byte for byte with the changes added at its end (an incremental
//! update), which keeps digital signatures valid. Uses `lopdf`; hayro
//! only reads.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use lopdf::{Dictionary, IncrementalDocument, Object, ObjectId, Stream, StringFormat};

use crate::markup::{Kind, Mark, PEN_WIDTH, Shape};

/// Why marks cannot be saved into a PDF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveError {
    /// Encrypted, or certified against changes.
    Protected,
    /// Damaged, or not a PDF `lopdf` can read.
    Invalid,
}

/// A transform `[a, b, c, d, e, f]`, from points as drawn to the page's
/// own coordinates.
pub(crate) type Transform = [f64; 6];

/// Whether marks can be written into `bytes`.
pub(crate) fn check(bytes: &[u8]) -> Result<(), SaveError> {
    let doc = lopdf::Document::load_mem(bytes).map_err(|_| SaveError::Invalid)?;
    allowed(&doc)
}

fn allowed(doc: &lopdf::Document) -> Result<(), SaveError> {
    if doc.is_encrypted() || doc.was_encrypted() {
        return Err(SaveError::Protected);
    }
    // A certifying signature (DocMDP) may forbid any change.
    let certified = doc
        .catalog()
        .ok()
        .and_then(|root| root.get_deref(b"Perms", doc).ok())
        .and_then(|perms| perms.as_dict().ok())
        .is_some_and(|perms| perms.has(b"DocMDP"));
    if certified {
        return Err(SaveError::Protected);
    }
    Ok(())
}

/// `bytes` with `marks` added; `transforms` has one per page.
pub(crate) fn write(
    bytes: &[u8],
    transforms: &[Transform],
    marks: &[Mark],
) -> Result<Vec<u8>, SaveError> {
    let prev = lopdf::Document::load_mem(bytes).map_err(|_| SaveError::Invalid)?;
    allowed(&prev)?;
    let pages = prev.get_pages();
    let mut by_page: BTreeMap<usize, Vec<&Mark>> = BTreeMap::new();
    for mark in marks {
        by_page.entry(mark.page).or_default().push(mark);
    }
    let mut existing = BTreeMap::new();
    for page in by_page.keys() {
        if let Some(&id) = pages.get(&(*page as u32 + 1)) {
            existing.insert(*page, (id, annotations(&prev, id)));
        }
    }
    let mut doc = IncrementalDocument::create_from(bytes.to_vec(), prev);
    let mut count = 0;
    for (page, marks) in by_page {
        let (Some((page_id, mut annots)), Some(t)) = (existing.remove(&page), transforms.get(page))
        else {
            continue;
        };
        doc.opt_clone_object_to_new_document(page_id)
            .map_err(|_| SaveError::Invalid)?;
        for mark in marks {
            count += 1;
            if let Some(id) = annotation(&mut doc.new_document, page_id, mark, t, count) {
                annots.push(Object::Reference(id));
            }
        }
        let dict = doc
            .new_document
            .get_object_mut(page_id)
            .and_then(Object::as_dict_mut)
            .map_err(|_| SaveError::Invalid)?;
        dict.set("Annots", Object::Array(annots));
    }
    let mut out = Vec::with_capacity(bytes.len() + 4096);
    doc.save_to(&mut out).map_err(|_| SaveError::Invalid)?;
    Ok(out)
}

/// The annotations page `id` already has.
fn annotations(doc: &lopdf::Document, id: ObjectId) -> Vec<Object> {
    let Ok(page) = doc.get_dictionary(id) else {
        return Vec::new();
    };
    match page.get(b"Annots") {
        Ok(Object::Array(items)) => items.clone(),
        Ok(Object::Reference(r)) => doc
            .get_object(*r)
            .and_then(Object::as_array)
            .cloned()
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn map(t: &Transform, (x, y): (f32, f32)) -> (f64, f64) {
    let (x, y) = (f64::from(x), f64::from(y));
    (t[0] * x + t[2] * y + t[4], t[1] * x + t[3] * y + t[5])
}

/// What an appearance draws, in the page's coordinates.
enum Draw {
    Fill(Vec<(f64, f64)>),
    Stroke(Vec<(f64, f64)>, f64),
}

/// The appearance of a mark: what it draws.
fn geometry(mark: &Mark, t: &Transform) -> Vec<Draw> {
    let line = |points: Vec<(f32, f32)>, width: f32| {
        Draw::Stroke(
            points.into_iter().map(|p| map(t, p)).collect(),
            f64::from(width),
        )
    };
    match &mark.shape {
        Shape::Ink(points) => vec![line(points.clone(), PEN_WIDTH)],
        Shape::Text { quads, .. } => quads
            .iter()
            .map(|q| {
                let h = (q.bottom - q.top).max(1.0);
                let width = (h * 0.07).max(0.6);
                match mark.kind {
                    Kind::Highlight | Kind::Ink => Draw::Fill(
                        [
                            (q.left, q.top),
                            (q.right, q.top),
                            (q.right, q.bottom),
                            (q.left, q.bottom),
                        ]
                        .into_iter()
                        .map(|p| map(t, p))
                        .collect(),
                    ),
                    Kind::Underline => {
                        let y = q.bottom - h * 0.1;
                        line(vec![(q.left, y), (q.right, y)], width)
                    }
                    Kind::StrikeOut => {
                        let y = q.top + h * 0.55;
                        line(vec![(q.left, y), (q.right, y)], width)
                    }
                    Kind::Squiggly => line(q.squiggle(), width),
                }
            })
            .collect(),
    }
}

fn annotation(
    doc: &mut lopdf::Document,
    page: ObjectId,
    mark: &Mark,
    t: &Transform,
    n: usize,
) -> Option<ObjectId> {
    let draws = geometry(mark, t);
    // The bounds of everything drawn, with room for the lines' width.
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for draw in &draws {
        let (points, pad) = match draw {
            Draw::Fill(points) => (points, 0.0),
            Draw::Stroke(points, width) => (points, width / 2.0 + 0.5),
        };
        for (x, y) in points {
            x0 = x0.min(x - pad);
            y0 = y0.min(y - pad);
            x1 = x1.max(x + pad);
            y1 = y1.max(y + pad);
        }
    }
    if x0 > x1 {
        return None;
    }
    let rect = || vec![real(x0), real(y0), real(x1), real(y1)];
    let [r, g, b] = mark.color;
    let mut content = String::new();
    if mark.kind == Kind::Highlight {
        content.push_str("/GS0 gs\n");
    }
    for draw in &draws {
        match draw {
            Draw::Fill(points) => {
                let _ = writeln!(content, "{r:.3} {g:.3} {b:.3} rg");
                path(&mut content, points);
                content.push_str("h f\n");
            }
            Draw::Stroke(points, width) => {
                let _ = writeln!(content, "{r:.3} {g:.3} {b:.3} RG {width:.2} w 1 J 1 j");
                path(&mut content, points);
                content.push_str("S\n");
            }
        }
    }
    let mut form = Dictionary::new();
    form.set("Type", "XObject");
    form.set("Subtype", "Form");
    form.set("BBox", rect());
    if mark.kind == Kind::Highlight {
        // Colour laid under the text, as a highlighter does.
        let mut state = Dictionary::new();
        state.set("Type", "ExtGState");
        state.set("BM", "Multiply");
        let mut states = Dictionary::new();
        states.set("GS0", state);
        let mut resources = Dictionary::new();
        resources.set("ExtGState", states);
        form.set("Resources", resources);
    }
    let appearance = doc.add_object(Stream::new(form, content.into_bytes()));

    let mut annot = Dictionary::new();
    annot.set("Type", "Annot");
    annot.set(
        "Subtype",
        match mark.kind {
            Kind::Highlight => "Highlight",
            Kind::Underline => "Underline",
            Kind::Squiggly => "Squiggly",
            Kind::StrikeOut => "StrikeOut",
            Kind::Ink => "Ink",
        },
    );
    annot.set("Rect", rect());
    annot.set(
        "C",
        vec![real(f64::from(r)), real(f64::from(g)), real(f64::from(b))],
    );
    // Printed with the page.
    annot.set("F", 4);
    annot.set("P", page);
    annot.set("NM", Object::string_literal(format!("katna-{n}")));
    let mut ap = Dictionary::new();
    ap.set("N", appearance);
    annot.set("AP", ap);
    match &mark.shape {
        Shape::Text { quads, text } => {
            // Upper left, upper right, lower left, lower right, per line.
            let mut points = Vec::new();
            for q in quads {
                for corner in [
                    (q.left, q.top),
                    (q.right, q.top),
                    (q.left, q.bottom),
                    (q.right, q.bottom),
                ] {
                    let (x, y) = map(t, corner);
                    points.push(real(x));
                    points.push(real(y));
                }
            }
            annot.set("QuadPoints", points);
            if !text.is_empty() {
                annot.set("Contents", text_string(text));
            }
        }
        Shape::Ink(points) => {
            let list: Vec<Object> = points
                .iter()
                .flat_map(|p| {
                    let (x, y) = map(t, *p);
                    [real(x), real(y)]
                })
                .collect();
            annot.set("InkList", vec![Object::Array(list)]);
            let mut border = Dictionary::new();
            border.set("W", real(f64::from(PEN_WIDTH)));
            annot.set("BS", border);
        }
    }
    Some(doc.add_object(annot))
}

fn path(out: &mut String, points: &[(f64, f64)]) {
    for (ix, (x, y)) in points.iter().enumerate() {
        let op = if ix == 0 { "m" } else { "l" };
        let _ = writeln!(out, "{x:.2} {y:.2} {op}");
    }
    if points.len() == 1 {
        // A dot: a line of no length, drawn with round caps.
        let (x, y) = points[0];
        let _ = writeln!(out, "{x:.2} {y:.2} l");
    }
}

fn real(v: f64) -> Object {
    Object::Real(((v * 100.0).round() / 100.0) as f32)
}

/// A PDF text string: plain for ASCII, else UTF-16 with a byte order mark.
fn text_string(text: &str) -> Object {
    if text.is_ascii() {
        Object::String(text.as_bytes().to_vec(), StringFormat::Literal)
    } else {
        let mut bytes = vec![0xfe, 0xff];
        for unit in text.encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        Object::String(bytes, StringFormat::Hexadecimal)
    }
}
