// SPDX-License-Identifier: GPL-3.0-or-later

//! Sharing and printing contacts (`docs/ARCHITECTURE.md` §8.6), as Google
//! Contacts does: a person's page shows them as a QR code a phone's camera
//! saves (their vCard), and prints them; "Print" under "Fix and manage"
//! prints everyone on show, through the print preview mail uses.

use std::sync::Arc;

use gpui::{AnyElement, Context, FontWeight, RenderImage, Window, div, img, prelude::*, rgba};
use katna_core::contact::Card;
use katna_dav::vcard;
use katna_i18n::tr;
use katna_preview::image::{Rgba, RgbaImage};
use katna_render::print::PrintMessage;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{px, unpx};
use qrcodegen::{QrCode, QrCodeEcc};

use super::MailWindow;
use super::attachments::bitmap;
use super::contacts_page::{birthday, kind_label, merge};
use super::print::PrintJob;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, elevation, filled_button};

/// Pixels per square of the code, drawn once and shown at [`QR_SIDE`].
const MODULE: u32 = 8;
/// The white margin around it, in squares, as phones need.
const QUIET: u32 = 4;
const QR_SIDE: f32 = 240.0;
const DIALOG_WIDTH: f32 = 360.0;

/// A person shown as a QR code.
pub(super) struct QrShare {
    name: String,
    /// `None` when even their shortest card is too long for a code.
    image: Option<Arc<RenderImage>>,
    closing: bool,
    shown: Spring,
}

/// `card` as a phone saves it: without its notes, picture or id, and
/// without addresses and links when `short`.
fn qr_vcard(card: &Card, short: bool) -> String {
    let mut card = card.clone();
    card.note.clear();
    card.photo_url.clear();
    if short {
        card.addresses.clear();
        card.urls.clear();
    }
    vcard::write("", &card, &[], None)
        .split("\r\n")
        .filter(|line| !line.is_empty() && !line.starts_with("UID:"))
        .map(|line| format!("{line}\r\n"))
        .collect()
}

/// The QR code of `card`, shortened when it does not fit in one.
fn qr_code(card: &Card) -> Option<QrCode> {
    [false, true]
        .into_iter()
        .find_map(|short| QrCode::encode_text(&qr_vcard(card, short), QrCodeEcc::Medium).ok())
}

/// `code` drawn black on white, with its margin.
fn qr_image(code: &QrCode) -> RgbaImage {
    let squares = code.size() as u32 + 2 * QUIET;
    let side = squares * MODULE;
    RgbaImage::from_fn(side, side, |x, y| {
        let (col, row) = (
            (x / MODULE) as i32 - QUIET as i32,
            (y / MODULE) as i32 - QUIET as i32,
        );
        if code.get_module(col, row) {
            Rgba([0, 0, 0, 255])
        } else {
            Rgba([255, 255, 255, 255])
        }
    })
}

/// A person as they print: their name, job and every detail, one a line.
fn print_message(name: &str, card: &Card) -> PrintMessage {
    let typed = |value: &str, kind: &str| {
        let kind = kind_label(kind);
        if kind.is_empty() {
            value.to_owned()
        } else {
            tr!("contacts-print-typed", value = value, kind = kind)
        }
    };
    let mut lines: Vec<String> = Vec::new();
    lines.extend(card.emails.iter().map(|e| typed(&e.value, &e.kind)));
    lines.extend(card.phones.iter().map(|p| typed(&p.value, &p.kind)));
    for address in &card.addresses {
        lines.push(typed(&address.lines().join(", "), &address.kind));
    }
    if let Some(day) = birthday(&card.birthday) {
        lines.push(tr!("contacts-print-birthday", day = day));
    }
    lines.extend(card.urls.iter().map(|u| typed(&u.value, &u.kind)));
    if !card.nickname.is_empty() {
        lines.push(tr!("contacts-print-nickname", name = card.nickname.clone()));
    }
    if !card.note.trim().is_empty() {
        lines.push(String::new());
        lines.push(card.note.trim().to_owned());
    }
    PrintMessage {
        from: name.to_owned(),
        date: card.job(),
        body: lines.join("\n"),
        ..PrintMessage::default()
    }
}

impl MailWindow {
    /// Shows `card`, `name`'s, as a QR code.
    pub(super) fn open_contact_qr(&mut self, name: String, card: &Card, cx: &mut Context<Self>) {
        let image = qr_code(card).map(|code| bitmap(qr_image(&code)));
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.contacts.qr = Some(QrShare {
            name,
            image,
            closing: false,
            shown,
        });
        cx.notify();
    }

    pub(super) fn contact_qr_open(&self) -> bool {
        self.contacts.qr.as_ref().is_some_and(|q| !q.closing)
    }

    pub(super) fn close_contact_qr(&mut self, cx: &mut Context<Self>) {
        if let Some(qr) = &mut self.contacts.qr {
            qr.closing = true;
            qr.shown.set(0.0);
        }
        cx.notify();
    }

    /// Prints the people whose cards are `people`, under `title`.
    pub(super) fn print_contacts(
        &mut self,
        title: String,
        people: Vec<Vec<i64>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if people.is_empty() {
            self.show_snackbar(tr!("contacts-print-none"), None, cx);
            return;
        }
        let paths = self.paths.clone();
        let family = self.font.as_ref().map(ToString::to_string);
        cx.spawn_in(window, async move |this, cx| {
            let read = cx
                .background_executor()
                .spawn(async move {
                    let mut messages = Vec::with_capacity(people.len());
                    for ids in &people {
                        let cards = crate::data::saved_cards(&paths, ids)?;
                        let card = merge(&cards);
                        messages.push(print_message(&card.display_name(), &card));
                    }
                    Ok::<_, String>(messages)
                })
                .await;
            this.update_in(cx, |this, window, cx| match read {
                Ok(messages) => {
                    let job = PrintJob::text(title, messages, family);
                    this.open_print_preview(Arc::new(job), window, cx);
                }
                Err(err) => this.show_snackbar(err, None, cx),
            })
            .ok();
        })
        .detach();
    }

    pub(super) fn render_contact_qr(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let qr = self.contacts.qr.as_mut()?;
        let t = qr.shown.tick(window, reduce);
        if qr.closing && qr.shown.settled() {
            if let Some(image) = self.contacts.qr.take().and_then(|q| q.image) {
                window.drop_image(image).ok();
            }
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let qr = self.contacts.qr.as_ref()?;
        let code = match &qr.image {
            Some(image) => div()
                .mt(px(20.0))
                .size(px(QR_SIDE))
                .rounded(px(8.0))
                .overflow_hidden()
                .child(img(image.clone()).size(px(QR_SIDE)))
                .into_any_element(),
            None => div()
                .mt(px(20.0))
                .text_size(px(14.0))
                .text_color(rgba(th.error))
                .child(tr!("contacts-qr-too-long"))
                .into_any_element(),
        };
        let body = div()
            .flex()
            .flex_col()
            .items_center()
            .px(px(24.0))
            .pt(px(24.0))
            .pb(px(20.0))
            .child(
                div()
                    .w_full()
                    .text_size(px(22.0))
                    .line_height(px(30.0))
                    .truncate()
                    .child(qr.name.clone()),
            )
            .child(code)
            .child(
                div()
                    .mt(px(16.0))
                    .w_full()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("contacts-qr-about")),
            )
            .child(
                div()
                    .mt(px(20.0))
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .child(
                        filled_button("contact-qr-done", tr!("contacts-qr-done"), th)
                            .focus_ring_filled(th)
                            .on_click(cx.listener(|this, _, _, cx| this.close_contact_qr(cx))),
                    ),
            );
        let vw = unpx(window.viewport_size().width);
        let card = div()
            .id("contact-qr-dialog")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .occlude()
            .w(px(DIALOG_WIDTH.min(vw - 32.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(super::PANEL_RADIUS))
            .map(|d| crate::widgets::frosted(d, th, th.surface, super::PANEL_RADIUS))
            .text_color(rgba(th.text))
            .font_weight(FontWeight::NORMAL)
            .shadow(elevation(th, 3.0))
            .child(body);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("contact-qr-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_contact_qr(cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use katna_core::contact::{Name, PostalAddress, Typed};

    use super::*;

    fn asha() -> Card {
        Card {
            name: Name {
                given: "Asha".into(),
                family: "Rao".into(),
                ..Name::default()
            },
            emails: vec![Typed::new("asha@example.in", "work")],
            phones: vec![Typed::new("+91 98765 43210", "mobile")],
            note: "Met at the fair".into(),
            photo_url: "https://example.in/a.jpg".into(),
            ..Card::default()
        }
    }

    #[test]
    fn the_code_holds_the_card_without_notes_or_picture() {
        let text = qr_vcard(&asha(), false);
        assert!(text.starts_with("BEGIN:VCARD\r\n"));
        assert!(text.ends_with("END:VCARD\r\n"));
        assert!(text.contains("asha@example.in"));
        assert!(text.contains("98765"));
        assert!(!text.contains("fair"));
        assert!(!text.contains("a.jpg"));
        assert!(!text.contains("UID"));
        let parsed = vcard::parse(&text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].card.name.given, "Asha");
    }

    #[test]
    fn a_long_card_is_shortened_to_fit() {
        let mut card = asha();
        card.addresses = (0..40)
            .map(|i| PostalAddress {
                street: format!("{i} A rather long street name in a far town"),
                city: "Kolkata".into(),
                ..PostalAddress::default()
            })
            .collect();
        assert!(QrCode::encode_text(&qr_vcard(&card, false), QrCodeEcc::Medium).is_err());
        assert!(qr_code(&card).is_some());
        let image = qr_image(&qr_code(&asha()).unwrap());
        assert_eq!(image.width() % MODULE, 0);
        // The margin is white.
        assert_eq!(image.get_pixel(0, 0).0, [255, 255, 255, 255]);
    }

    #[test]
    fn a_person_prints_every_detail() {
        let message = print_message("Asha Rao", &asha());
        assert_eq!(message.from, "Asha Rao");
        assert!(message.body.contains("asha@example.in"));
        assert!(message.body.contains("Met at the fair"));
    }
}
