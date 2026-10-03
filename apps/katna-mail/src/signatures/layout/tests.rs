// SPDX-License-Identifier: GPL-3.0-or-later

use katna_render::signature::{Known, PhoneKind, details};

use katna_preview::image;

use super::*;

fn demo(style: LayoutStyle) -> SignatureLayout {
    SignatureLayout {
        style,
        colour: "#1a56db".to_owned(),
        name: "Demo Alam".to_owned(),
        title: "Accounts Manager".to_owned(),
        company: "Demo Systems <Pvt> Ltd.".to_owned(),
        mobile: "+91 90000 12345".to_owned(),
        office: "+91 33 4000 5678".to_owned(),
        email: "accounts@demosys.example".to_owned(),
        website: "www.demosys.example".to_owned(),
        address: "53/1 Example Road, Howrah".to_owned(),
        pages: vec!["https://www.linkedin.com/company/demo".to_owned()],
        ..SignatureLayout::default()
    }
}

/// A `width` × `height` PNG as a `data:` URI.
fn picture(width: u32, height: u32) -> String {
    let image = image::RgbaImage::from_pixel(width, height, image::Rgba([1, 2, 3, 255]));
    let mut png = Vec::new();
    image::ImageEncoder::write_image(
        image::codecs::png::PngEncoder::new(&mut png),
        &image,
        width,
        height,
        image::ExtendedColorType::Rgba8,
    )
    .unwrap();
    format!("data:image/png;base64,{}", base64_encode(&png))
}

#[test]
fn the_text_twin_is_what_the_card_reads() {
    let text = text(&demo(LayoutStyle::Classic));
    assert_eq!(
        text,
        "Demo Alam\nAccounts Manager, Demo Systems <Pvt> Ltd.\nM: +91 90000 12345\n\
         O: +91 33 4000 5678\nE: accounts@demosys.example\nwww.demosys.example\n\
         https://www.linkedin.com/company/demo\n53/1 Example Road, Howrah"
    );
    let known = Known {
        name: Some("Demo Alam"),
        email: "accounts@demosys.example",
        ..Known::default()
    };
    let read = details(&text, &known).unwrap();
    let kinds: Vec<PhoneKind> = read.phones.iter().map(|p| p.kind).collect();
    assert_eq!(kinds, [PhoneKind::Mobile, PhoneKind::Office]);
}

#[test]
fn every_layout_is_written_and_escaped() {
    for style in LayoutStyle::ALL {
        let (text, html) = write(&demo(style));
        assert!(text.starts_with("Demo Alam"), "{style:?}");
        if style == LayoutStyle::Plain {
            assert!(html.is_empty());
            continue;
        }
        assert!(html.starts_with("<table"), "{style:?}");
        assert!(html.contains("Demo Alam"), "{style:?}");
        assert!(!html.contains("<Pvt>"), "{style:?}: {html}");
        assert!(!html.contains("<script"), "{style:?}");
    }
}

#[test]
fn page_marks_are_pictures_in_the_colour() {
    let html = html(&demo(LayoutStyle::Classic));
    assert!(html.contains(r#"<a href="https://www.linkedin.com/company/demo"><img alt="LinkedIn" width="18" height="18""#));
    let uri = html
        .split(r#"src=""#)
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap();
    let png = picture_bytes(uri).unwrap();
    let image = image::load_from_memory(&png).unwrap().into_rgba8();
    assert_eq!(image.dimensions(), (36, 36));
    assert!(image.pixels().any(|p| p.0 == [0x1a, 0x56, 0xdb, 255]));
}

#[test]
fn pictures_are_shown_at_half_their_size_in_their_box() {
    let mut layout = demo(LayoutStyle::LogoLeft);
    layout.logo = picture(256, 64);
    let html = html(&layout);
    // 128 × 32 fits the 120 × 64 box at 120 × 30.
    assert!(html.contains(r#"width="120" height="30""#), "{html}");
    // Not in a layout without a logo.
    layout.style = LayoutStyle::Classic;
    assert!(!html_of(&layout).contains(r#"width="120""#));
}

fn html_of(layout: &SignatureLayout) -> String {
    write(layout).1
}

#[test]
fn no_photo_shows_initials_and_empty_fields_leave_no_label() {
    let layout = SignatureLayout {
        style: LayoutStyle::Photo,
        name: "Demo  alam".to_owned(),
        mobile: "+91 1".to_owned(),
        ..SignatureLayout::default()
    };
    let html = html_of(&layout);
    assert!(
        html.contains(r#"<img alt="DA" width="68" height="68""#),
        "{html}"
    );
    assert!(html.contains("M:</span>"));
    assert!(!html.contains("O:</span>") && !html.contains("E:</span>"));
    assert_eq!(text(&layout), "Demo  alam\nM: +91 1");
}

#[test]
fn colours_and_initials() {
    let mut layout = demo(LayoutStyle::Classic);
    assert_eq!(colour(&layout), 0x1a56db);
    layout.colour = "nope".to_owned();
    assert_eq!(colour(&layout), COLOURS[0]);
    assert_eq!(initials("ada"), "A");
    assert_eq!(initials("Ada King Lovelace"), "AK");
    assert_eq!(initials(""), "");
}

#[test]
fn one_line_is_short() {
    let (text, html) = write(&demo(LayoutStyle::OneLine));
    assert_eq!(
        text,
        "Demo Alam\nAccounts Manager, Demo Systems <Pvt> Ltd.\n+91 90000 12345 · www.demosys.example"
    );
    assert!(!html.contains("<img"));
}

#[test]
fn pictures_inside_are_counted() {
    let html =
        r#"<img src="data:image/png;base64,AAAA"><img src='data:image/gif;base64,AAAAAAAA'>"#;
    assert_eq!(pictures_size(html), 3 + 6);
    assert_eq!(pictures_size("<b>none</b>"), 0);
}
