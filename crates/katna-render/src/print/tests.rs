// SPDX-License-Identifier: GPL-3.0-or-later

use super::*;
use crate::html;

/// A font installed on most Linux systems, if this one has it.
fn system_fonts() -> Option<PrintFonts> {
    let read = |name: &str| {
        [
            "/usr/share/fonts/truetype/dejavu/",
            "/usr/share/fonts/TTF/",
            "/usr/share/fonts/dejavu/",
        ]
        .iter()
        .find_map(|dir| std::fs::read(format!("{dir}{name}")).ok())
        .map(|data| PrintFont { data, index: 0 })
    };
    Some(PrintFonts {
        regular: read("DejaVuSans.ttf")?,
        bold: read("DejaVuSans-Bold.ttf"),
        italic: None,
        bold_italic: None,
        mono: read("DejaVuSansMono.ttf"),
    })
}

fn pages(pdf: &[u8]) -> usize {
    let pdf = String::from_utf8_lossy(pdf);
    let count = pdf.split("/Type/Pages/Count ").nth(1).unwrap();
    count[..count.find('/').unwrap()].parse().unwrap()
}

fn strip(message: &PrintMessage, options: PrintOptions) -> Option<Strip> {
    let fonts = system_fonts()?;
    let faces = Faces::new(&fonts).unwrap();
    Some(flow::lay_out(
        "Subject",
        std::slice::from_ref(message),
        487.0,
        734.0,
        &faces,
        options,
        true,
    ))
}

fn html_message(source: &str) -> PrintMessage {
    PrintMessage {
        from: "Ada <ada@example.org>".into(),
        document: Some(html::document(source, &|_| None)),
        ..PrintMessage::default()
    }
}

/// Where each piece of text was drawn: (text, x, baseline, color).
fn texts(strip: &Strip) -> Vec<(String, f32, f32, Color)> {
    strip
        .items
        .iter()
        .filter_map(|p| match &p.item {
            Item::Text {
                x,
                baseline,
                color,
                text,
                ..
            } => Some((text.clone(), *x, *baseline, *color)),
            _ => None,
        })
        .collect()
}

fn find(strip: &Strip, word: &str) -> (String, f32, f32, Color) {
    texts(strip)
        .into_iter()
        .find(|t| t.0 == word)
        .unwrap_or_else(|| panic!("{word} not drawn"))
}

#[test]
fn prints_a_conversation_over_pages() {
    let Some(fonts) = system_fonts() else {
        eprintln!("no DejaVu Sans here; skipped");
        return;
    };
    let long = "All work and no play makes Jack a dull boy. ".repeat(400);
    let messages = vec![
        PrintMessage {
            from: "Ada Lovelace <ada@example.org>".into(),
            date: "Wed, 16 Sep 2026, 23:48".into(),
            to: "To: bob@example.org".into(),
            body: format!("Dear Bob,\n\n{long}\n\nAda"),
            attachments: vec!["notes.pdf".into()],
            ..PrintMessage::default()
        },
        PrintMessage {
            from: "Bob <bob@example.org>".into(),
            body: "Thanks!\nhttps://example.org/".to_owned() + &"x".repeat(300),
            ..PrintMessage::default()
        },
    ];
    let pdf = conversation_pdf(
        "Notes",
        &messages,
        Paper::A4,
        &fonts,
        PrintOptions::default(),
    )
    .unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pages(&pdf) >= 3, "{} pages", pages(&pdf));
}

#[test]
fn paper_from_millimetres() {
    let letter = Paper::from_mm(215.9, 279.4).unwrap();
    assert_eq!(
        (letter.width.round(), letter.height.round()),
        (612.0, 792.0)
    );
    assert!(Paper::from_mm(10.0, 10.0).is_none());
}

#[test]
fn a_table_prints_as_cells_side_by_side() {
    let message = html_message(
        "<p>Dear Ada</p><table border=1><tr><td>S.No</td><td>Name</td><td>Policy</td></tr>\
         <tr><td>1</td><td>Bibhas</td><td>6610014121</td></tr></table>",
    );
    let Some(strip) = strip(&message, PrintOptions::default()) else {
        return;
    };
    let (_, x1, y1, _) = find(&strip, "S.No");
    let (_, x2, y2, _) = find(&strip, "Name");
    let (_, x3, y3, _) = find(&strip, "Policy");
    assert!((y1 - y2).abs() < 0.1 && (y2 - y3).abs() < 0.1);
    assert!(x1 < x2 && x2 < x3);
    let (_, bx, by, _) = find(&strip, "Bibhas");
    assert!((bx - x2).abs() < 0.1 && by > y2);
    // The cells' borders are drawn.
    assert!(
        strip
            .items
            .iter()
            .any(|p| matches!(p.item, Item::Shape { ring: Some(_), .. }))
    );
}

#[test]
fn backgrounds_can_be_left_out() {
    let message = html_message(
        "<div style=\"background:#202124;color:#ffffff\">Dark</div>\
         <p><b>Bold</b> and <i>italic</i></p>",
    );
    let Some(on) = strip(&message, PrintOptions::default()) else {
        return;
    };
    let dark = |strip: &Strip| {
        strip.items.iter().any(|p| {
            matches!(
                p.item,
                Item::Shape {
                    color: 0x2021_24ff,
                    ..
                }
            )
        })
    };
    assert!(dark(&on));
    assert_eq!(find(&on, "Dark").3, 0xffff_ffff);
    let off = strip(
        &message,
        PrintOptions {
            backgrounds: false,
            ..PrintOptions::default()
        },
    )
    .unwrap();
    assert!(!dark(&off));
    // White text on white paper would vanish: it is darkened.
    assert!(!flow_light(find(&off, "Dark").3));
    // The formatting stays.
    assert!(off.items.iter().any(|p| matches!(
        &p.item,
        Item::Text { text, face, .. } if text == "Bold" && *face == BOLD
    )));
}

fn flow_light(color: Color) -> bool {
    let [r, g, b, _] = color.to_be_bytes();
    u32::from(r) + u32::from(g) + u32::from(b) > 600
}

#[test]
fn simple_text_prints_the_text_alone() {
    let mut message = html_message("<table><tr><td>Left</td><td>Right</td></tr></table>");
    message.body = "Left Right".into();
    let Some(strip) = strip(
        &message,
        PrintOptions {
            simple: true,
            ..PrintOptions::default()
        },
    ) else {
        return;
    };
    let (_, x, y, _) = find(&strip, "Left");
    let (_, rx, ry, _) = find(&strip, "Right");
    // A row's cells stay on one line.
    assert_eq!(y, ry);
    // A few spaces apart, not a cell apart.
    assert!(rx - x < 60.0, "{x} {rx}");
    assert!(
        !strip
            .items
            .iter()
            .any(|p| matches!(p.item, Item::Shape { ring: Some(_), .. }))
    );
}

#[test]
fn lines_wrap_and_long_words_break() {
    let message = html_message(&format!(
        "<p>{}</p><p>{}</p>",
        "word ".repeat(200),
        "x".repeat(400)
    ));
    let Some(strip) = strip(&message, PrintOptions::default()) else {
        return;
    };
    let fonts = system_fonts().unwrap();
    let faces = Faces::new(&fonts).unwrap();
    let face = rustybuzz::Face::from_slice(&faces.0[0].data, 0).unwrap();
    for (text, x, _, _) in texts(&strip) {
        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(&text);
        let units: i32 = rustybuzz::shape(&face, &[], buffer)
            .glyph_positions()
            .iter()
            .map(|p| p.x_advance)
            .sum();
        let width = units as f32 / f32::from(face.units_per_em() as u16) * 10.5;
        assert!(x + width <= 487.5, "{text:?} at {x} is {width} wide");
    }
}

#[test]
fn pages_end_between_lines() {
    let keep = [(0.0, 10.0), (10.0, 20.0), (95.0, 108.0), (108.0, 120.0)];
    assert_eq!(page_end(&keep, 0.0, 100.0), 95.0);
    assert_eq!(page_end(&keep, 95.0, 100.0), 195.0);
    // Something taller than a page is cut where the page is full.
    assert_eq!(page_end(&[(0.0, 500.0)], 0.0, 100.0), 100.0);
}
