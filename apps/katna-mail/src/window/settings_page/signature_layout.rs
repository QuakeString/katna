// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Compose > Signatures: a signature made from one of Katna's
//! twelve layouts ([`crate::signatures::layout`]). The fields are filled
//! in once and a layout and colour picked; the signature is written again
//! from them on every change and shown as the reader will see it, in a
//! light or dark reader or as plain text. Pictures are made small and
//! travel inside the mail. "Your own" goes back to the free editor, which
//! keeps the look as far as it can hold it, so Katna asks first.

use std::rc::Rc;

use gpui::{AnyElement, Context, Div, Entity, Subscription, Window, canvas, div, prelude::*, rgba};
use katna_core::config::{LayoutStyle, Signature, SignatureLayout};
use katna_i18n::tr;
use katna_render::html::{self as mail_html, Document};
use katna_render::signature::{Known, PhoneKind, Site};
use katna_ui::rich::{Block, Doc, html};
use katna_ui::tokens::{radius, space, text};
use katna_ui::{InputEvent, TextArea, TextInput, px};

use super::MailWindow;
use crate::signatures::layout;
use crate::theme::Theme;
use crate::widgets::Tip as _;
use crate::widgets::{
    area_field, choice_chip, color_swatch, color_wheel, filled_button, icon, icon_button,
    line_field, outlined_button,
};
use crate::window::scheme_color::Target;

/// Largest picture file read for a layout; it is made much smaller.
const MAX_SOURCE: u64 = 20 * 1024 * 1024;
/// A layout tile's drawing.
const THUMB: (f32, f32) = (72.0, 46.0);

/// The form of a signature made from a layout.
pub(super) struct LayoutForm {
    id: u32,
    /// Name, title, company, mobile, office, email, website.
    fields: Vec<Entity<TextInput>>,
    /// The address, a line or more.
    address: Entity<TextArea>,
    page: Entity<TextInput>,
    /// The signature laid out for the preview; `None` for plain text.
    shown: Option<Rc<Document>>,
    reader: Reader,
    _subscriptions: Vec<Subscription>,
}

/// How the preview shows the signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reader {
    Light,
    Dark,
    Text,
}

/// A question asked before a signature's content is replaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Confirm {
    /// Turn a signature written by hand into this layout.
    Use(LayoutStyle),
    /// Turn a layout signature into one written by hand.
    Edit,
}

/// The labels of [`LayoutForm::fields`], in order, then the address's.
fn field_labels() -> [String; 8] {
    [
        tr!("signature-layout-name"),
        tr!("signature-layout-job"),
        tr!("signature-layout-company"),
        tr!("signature-layout-mobile"),
        tr!("signature-layout-office"),
        tr!("signature-layout-email"),
        tr!("signature-layout-website"),
        tr!("signature-layout-address"),
    ]
}

fn field_values(l: &SignatureLayout) -> [&String; 8] {
    [
        &l.name, &l.title, &l.company, &l.mobile, &l.office, &l.email, &l.website, &l.address,
    ]
}

fn field_values_mut(l: &mut SignatureLayout) -> [&mut String; 8] {
    [
        &mut l.name,
        &mut l.title,
        &mut l.company,
        &mut l.mobile,
        &mut l.office,
        &mut l.email,
        &mut l.website,
        &mut l.address,
    ]
}

/// The signature's `(text, html)` as saved, and its laid out preview.
fn written(l: &SignatureLayout) -> (String, String, Option<Rc<Document>>) {
    let (text, raw) = layout::write(l);
    if raw.is_empty() {
        return (text, String::new(), None);
    }
    let cleaned = mail_html::clean(&raw, &Default::default());
    let mut next = 0;
    let block = html::html_block(&cleaned.html, &mut next);
    let shown = Rc::new(crate::window::rich::designed_document(&block));
    let doc = Doc {
        blocks: vec![Block::Html(block)],
    };
    let (_, html) = crate::window::compose::signature_content(&doc);
    (text, html, Some(shown))
}

/// A layout filled in from a signature written by hand, as the person
/// card reads one, and from the account it is sent from.
fn filled_from(signature: &Signature, style: LayoutStyle, address: &str) -> SignatureLayout {
    let mut l = SignatureLayout {
        style,
        colour: layout::hex(layout::COLOURS[0]),
        email: address.to_owned(),
        logo: first_picture(&signature.html).unwrap_or_default(),
        ..SignatureLayout::default()
    };
    let text = signature.text.trim();
    let Some(first) = text.lines().map(str::trim).find(|l| !l.is_empty()) else {
        return l;
    };
    l.name = first.to_owned();
    let known = Known {
        name: Some(first),
        email: address,
        ..Known::default()
    };
    let Some(d) = katna_render::signature::details(text, &known) else {
        return l;
    };
    l.title = d.title.unwrap_or_default();
    for phone in d.phones {
        let slot = match phone.kind {
            PhoneKind::Mobile | PhoneKind::WhatsApp => &mut l.mobile,
            _ => &mut l.office,
        };
        if slot.is_empty() {
            *slot = phone.number;
        }
    }
    if let Some(email) = d.emails.into_iter().next() {
        l.email = email;
    }
    l.company = d.company.name.unwrap_or_default();
    // "Accounts Manager · Demo Systems" on one line.
    if l.company.is_empty()
        && let Some((title, company)) = [" · ", " | ", " – ", " - ", ", "]
            .iter()
            .find_map(|sep| l.title.split_once(sep))
    {
        let (title, company) = (title.trim().to_owned(), company.trim().to_owned());
        l.title = title;
        l.company = company;
    }
    l.website = d.company.website.unwrap_or_default();
    l.pages = d
        .pages
        .into_iter()
        .chain(d.company.pages)
        .map(|p| p.url)
        .collect();
    l.address = d
        .company
        .offices
        .into_iter()
        .next()
        .map(|o| o.lines.join("\n"))
        .unwrap_or_default();
    l
}

impl MailWindow {
    /// Opens the form when the signature being edited uses a layout.
    pub(super) fn open_layout_form(
        &mut self,
        signature: &Signature,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(l) = signature.layout.clone() else {
            if let Some(page) = &mut self.settings_page {
                page.layout = None;
            }
            return;
        };
        let accent = rgba(self.theme(window).accent).into();
        let id = signature.id;
        let mut subscriptions = Vec::new();
        let [labels @ .., address_label] = field_labels();
        let fields: Vec<Entity<TextInput>> = labels
            .into_iter()
            .zip(field_values(&l))
            .map(|(label, value)| {
                cx.new(|cx| {
                    let mut input = TextInput::new(label, cx);
                    input.set_text(value.clone(), cx);
                    input.caret_to_start(cx);
                    input.set_accent(accent);
                    input
                })
            })
            .collect();
        for (n, field) in fields.iter().enumerate() {
            subscriptions.push(
                cx.subscribe(field, move |this, input, event: &InputEvent, cx| {
                    if *event == InputEvent::Changed {
                        let value = input.read(cx).text().to_owned();
                        this.change_layout(id, |l| *field_values_mut(l)[n] = value, cx);
                    }
                }),
            );
        }
        // Enter starts a new line, kept in the signature.
        let address = cx.new(|cx| {
            let mut area = TextArea::new(address_label, cx);
            area.set_text(l.address.clone(), 0, cx);
            area.set_accent(accent);
            area
        });
        subscriptions.push(
            cx.subscribe(&address, move |this, area, event: &InputEvent, cx| {
                if *event == InputEvent::Changed {
                    let value = area.read(cx).text().to_owned();
                    this.change_layout(id, |l| l.address = value, cx);
                }
            }),
        );
        let page = cx.new(|cx| {
            let mut input = TextInput::new(tr!("signature-layout-page-placeholder"), cx);
            input.set_accent(accent);
            input
        });
        subscriptions.push(
            cx.subscribe(&page, move |this, input, event: &InputEvent, cx| {
                if *event == InputEvent::Submit {
                    let url = input.read(cx).text().trim().to_owned();
                    if url.is_empty() {
                        return;
                    }
                    input.update(cx, |input, cx| input.set_text("", cx));
                    this.change_layout(id, |l| l.pages.push(url), cx);
                }
            }),
        );
        let (_, _, shown) = written(&l);
        if let Some(page_state) = &mut self.settings_page {
            page_state.layout = Some(LayoutForm {
                id,
                fields,
                address,
                page,
                shown,
                reader: Reader::Light,
                _subscriptions: subscriptions,
            });
        }
    }

    /// Changes the layout of signature `id` and writes it again.
    fn change_layout(
        &mut self,
        id: u32,
        change: impl FnOnce(&mut SignatureLayout),
        cx: &mut Context<Self>,
    ) {
        let Some(signature) = self
            .config
            .sending
            .signatures
            .iter_mut()
            .find(|s| s.id == id)
        else {
            return;
        };
        let Some(l) = &mut signature.layout else {
            return;
        };
        change(l);
        let (text, html, shown) = written(l);
        signature.text = text;
        signature.html = html;
        if let Some(form) = self.settings_page.as_mut().and_then(|p| p.layout.as_mut())
            && form.id == id
        {
            form.shown = shown;
        }
        self.save_soon(cx);
        cx.notify();
    }

    /// The colour (`0xrrggbb`) of the layout signature `id`.
    pub(in crate::window) fn layout_colour(&self, id: u32) -> Option<u32> {
        let l = self.config.sending.signature(Some(id))?.layout.as_ref()?;
        Some(layout::colour(l))
    }

    /// Gives the layout signature `id` the colour `rgb` (`0xrrggbb`), from
    /// its swatches or the colour picker.
    pub(in crate::window) fn set_layout_colour(
        &mut self,
        id: u32,
        rgb: u32,
        cx: &mut Context<Self>,
    ) {
        self.change_layout(id, |l| l.colour = layout::hex(rgb & 0xffffff), cx);
    }

    /// A layout tile picked: changes the layout, or asks first when it
    /// would replace a signature written by hand, or go back to one.
    fn pick_layout(
        &mut self,
        id: u32,
        style: Option<LayoutStyle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(signature) = self.config.sending.signature(Some(id)).cloned() else {
            return;
        };
        let confirm = match (&signature.layout, style) {
            (Some(l), Some(style)) => {
                if l.style != style {
                    self.change_layout(id, |l| l.style = style, cx);
                }
                None
            }
            (Some(_), None) => Some(Confirm::Edit),
            (None, None) => None,
            (None, Some(style)) => {
                if signature.text.trim().is_empty() && signature.html.trim().is_empty() {
                    self.use_layout(id, style, window, cx);
                    None
                } else {
                    Some(Confirm::Use(style))
                }
            }
        };
        if let Some(page) = &mut self.settings_page {
            page.layout_confirm = confirm;
        }
        cx.notify();
    }

    /// Answers the question asked by [`Self::pick_layout`].
    fn confirm_layout(&mut self, id: u32, yes: bool, window: &mut Window, cx: &mut Context<Self>) {
        let confirm = self
            .settings_page
            .as_mut()
            .and_then(|p| p.layout_confirm.take());
        match confirm.filter(|_| yes) {
            Some(Confirm::Use(style)) => self.use_layout(id, style, window, cx),
            Some(Confirm::Edit) => {
                if let Some(s) = self
                    .config
                    .sending
                    .signatures
                    .iter_mut()
                    .find(|s| s.id == id)
                {
                    s.layout = None;
                }
                self.save_config();
                self.edit_signature(Some(id), window, cx);
            }
            None => {}
        }
        cx.notify();
    }

    /// Makes signature `id` one from layout `style`, filled in from it.
    fn use_layout(
        &mut self,
        id: u32,
        style: LayoutStyle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let address = self.signature_address();
        let Some(signature) = self
            .config
            .sending
            .signatures
            .iter_mut()
            .find(|s| s.id == id)
        else {
            return;
        };
        let l = filled_from(signature, style, &address);
        let (text, html, _) = written(&l);
        signature.layout = Some(l);
        signature.text = text;
        signature.html = html;
        self.save_config();
        self.edit_signature(Some(id), window, cx);
    }

    /// The address signatures are most likely sent from: the one new mail
    /// goes out from, else the first mail account's.
    fn signature_address(&self) -> String {
        let from = self.config.sending.send_from.trim();
        if from.contains('@') {
            return from.to_owned();
        }
        self.accounts
            .iter()
            .find(|a| a.kind.is_mail() && a.address.contains('@'))
            .map(|a| a.address.clone())
            .unwrap_or_default()
    }

    /// Asks for a picture and puts it in the layout as `shape`.
    fn pick_layout_picture(
        &mut self,
        id: u32,
        shape: katna_preview::signature::Shape,
        cx: &mut Context<Self>,
    ) {
        let chosen = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(tr!("signature-picture-choose").into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let made = cx
                .background_executor()
                .spawn({
                    let name = name.clone();
                    async move {
                        let size = std::fs::metadata(&path).map_err(|e| e.to_string())?.len();
                        if size > MAX_SOURCE {
                            return Err(String::new());
                        }
                        let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
                        let mime = katna_ui::rich::image_mime(&name).unwrap_or("image/png");
                        let format = match katna_preview::kind(mime, &name) {
                            katna_preview::Kind::Picture(format) => format,
                            _ => return Err(String::new()),
                        };
                        katna_preview::signature::make(&bytes, format, shape).map_err(|e| e.0)
                    }
                })
                .await;
            this.update(cx, |this, cx| match made {
                Ok(made) => {
                    let uri = format!(
                        "data:{};base64,{}",
                        made.mime,
                        katna_ui::rich::html::base64_encode(&made.bytes)
                    );
                    this.change_layout(id, |l| *picture_mut(l, shape) = uri, cx);
                }
                Err(_) => this.show_snackbar(
                    tr!("signature-layout-picture-failed", name = name.as_str()),
                    None,
                    cx,
                ),
            })
            .ok();
        })
        .detach();
    }

    fn set_layout_reader(&mut self, reader: Reader, cx: &mut Context<Self>) {
        if let Some(form) = self.settings_page.as_mut().and_then(|p| p.layout.as_mut()) {
            form.reader = reader;
        }
        cx.notify();
    }

    /// The layout tiles, over the editor of signature `id`.
    pub(super) fn layout_tiles(&self, id: u32, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let signature = self.config.sending.signature(Some(id));
        let current = signature.and_then(|s| s.layout.as_ref());
        let colour = current.map_or(layout::COLOURS[0], layout::colour) << 8 | 0xff;
        let tiles =
            LayoutStyle::ALL.into_iter().enumerate().map(|(n, style)| {
                let on = current.map(|l| l.style) == Some(style);
                let label = layout::name(style);
                let drawing = thumb(style, colour, th);
                self.page_control(
                    div()
                        .id(("page-signature-layout", n))
                        .w(px(THUMB.0 + 2.0 * space::S2 + 2.0))
                        .p(px(space::S2))
                        .flex()
                        .flex_col()
                        .gap(px(space::S2))
                        .rounded(px(radius::SM))
                        .border_1()
                        .border_color(rgba(if on { th.accent } else { th.outline }))
                        .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
                        .cursor_pointer(),
                    th,
                    cx,
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.pick_layout(id, Some(style), window, cx)
                }))
                .child(
                    div()
                        .w(px(THUMB.0))
                        .h(px(THUMB.1))
                        .rounded(px(radius::XS))
                        .bg(rgba(th.hover))
                        .child(drawing),
                )
                .child(
                    div()
                        .text_size(px(text::CAPTION))
                        .text_color(rgba(if on { th.text } else { th.text_dim }))
                        .truncate()
                        .child(label),
                )
            });
        div()
            .flex()
            .flex_col()
            .gap(px(space::S2))
            .child(
                div()
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("signature-layout")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(space::S3))
                    .children(tiles),
            )
            .children(self.layout_question(id, th, cx).filter(|_| {
                matches!(
                    self.settings_page.as_ref().and_then(|p| p.layout_confirm),
                    Some(Confirm::Use(_))
                )
            }))
            .into_any_element()
    }

    /// The question asked before replacing the signature's content.
    fn layout_question(&self, id: u32, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let confirm = self.settings_page.as_ref()?.layout_confirm?;
        let (said, yes) = match confirm {
            Confirm::Use(style) => (
                tr!("signature-layout-use-confirm", layout = layout::name(style)),
                tr!("signature-layout-use"),
            ),
            Confirm::Edit => (
                tr!("signature-layout-edit-confirm"),
                tr!("signature-layout-edit"),
            ),
        };
        Some(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(space::S3))
                .p(px(space::S4))
                .rounded(px(radius::SM))
                .bg(rgba(th.hover))
                .child(
                    div()
                        .flex_1()
                        .min_w(px(200.0))
                        .text_size(px(text::SMALL))
                        .child(said),
                )
                .child(
                    outlined_button(
                        "page-signature-layout-no",
                        tr!("signature-layout-cancel"),
                        th,
                    )
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.confirm_layout(id, false, window, cx)
                    })),
                )
                .child(
                    filled_button("page-signature-layout-yes", yes, th)
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.confirm_layout(id, true, window, cx)
                        })),
                )
                .into_any_element(),
        )
    }

    /// The fields, pictures, pages, colour and preview of a layout
    /// signature, in place of the free editor.
    pub(super) fn layout_form(
        &self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let form = self.settings_page.as_ref()?.layout.as_ref()?;
        let form_confirm = self.settings_page.as_ref()?.layout_confirm;
        let id = form.id;
        let l = self.config.sending.signature(Some(id))?.layout.as_ref()?;
        let label = |said: String| {
            div()
                .text_size(px(text::CAPTION))
                .text_color(rgba(th.text_dim))
                .child(said)
        };
        let field = |n: usize, labels: &[String; 8]| {
            div()
                .flex_1()
                .min_w(px(160.0))
                .flex()
                .flex_col()
                .gap(px(space::S1))
                .child(label(labels[n].clone()))
                .child(line_field(
                    ("page-signature-layout-field", n),
                    &form.fields[n],
                    th,
                    cx,
                ))
        };
        let labels = field_labels();
        let pair = |a: usize, b: usize| {
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(space::S3))
                .child(field(a, &labels))
                .child(field(b, &labels))
        };
        let fields = div()
            .flex()
            .flex_col()
            .gap(px(space::S3))
            .child(pair(0, 1))
            .child(field(2, &labels))
            .child(pair(3, 4))
            .child(pair(5, 6))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(space::S1))
                    .child(label(labels[7].clone()))
                    .child(area_field(
                        "page-signature-layout-address",
                        &form.address,
                        th,
                        cx,
                    )),
            );
        let (logo, photo) = layout::shows(l.style);
        use katna_preview::signature::Shape;
        let pictures: Vec<(Shape, String, bool)> = [
            (Shape::Logo, tr!("signature-layout-logo"), logo),
            (Shape::Photo, tr!("signature-layout-photo-picture"), photo),
            (Shape::Banner, tr!("signature-layout-banner-picture"), true),
        ]
        .into_iter()
        .filter(|(shape, _, shown)| *shown || !picture(l, *shape).is_empty())
        .collect();
        let pictures = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(space::S3))
            .children(
                pictures
                    .into_iter()
                    .enumerate()
                    .map(|(n, (shape, name, _))| {
                        let uri = picture(l, shape);
                        let size = layout::picture_bytes(uri).map(|b| b.len());
                        let said = match size {
                            Some(size) => format!("{name} · {}", crate::format::size(size as u64)),
                            None => name,
                        };
                        self.page_control(
                            div()
                                .id(("page-signature-layout-picture", n))
                                .h(px(crate::widgets::CHIP_HEIGHT))
                                .pl(px(space::S3))
                                .pr(px(if size.is_some() { space::S1 } else { space::S4 }))
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(space::S2))
                                .rounded(px(radius::SM))
                                .border_1()
                                .border_color(rgba(th.outline))
                                .relative()
                                .child(crate::widgets::hover_fade(
                                    "hover-glow",
                                    Some(radius::SM),
                                    th,
                                ))
                                .cursor_pointer()
                                .text_size(px(text::SMALL)),
                            th,
                            cx,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.pick_layout_picture(id, shape, cx)
                        }))
                        .child(icon(
                            if size.is_some() { "image" } else { "add" },
                            th.text_dim,
                            16.0,
                        ))
                        .child(said)
                        .when(size.is_some(), |d| {
                            d.child(
                                icon_button(("page-signature-layout-remove", n), "close", 16.0, th)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        cx.stop_propagation();
                                        this.change_layout(
                                            id,
                                            |l| picture_mut(l, shape).clear(),
                                            cx,
                                        )
                                    })),
                            )
                        })
                    }),
            );
        let pages = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(space::S3))
            .children(l.pages.iter().enumerate().map(|(n, url)| {
                let site = Site::of(url);
                let shown = match site {
                    Site::Web => katna_render::signature::host(url),
                    site => site.name().to_owned(),
                };
                div()
                    .h(px(crate::widgets::CHIP_HEIGHT))
                    .pl(px(space::S3))
                    .pr(px(space::S1))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S2))
                    .rounded(px(radius::SM))
                    .border_1()
                    .border_color(rgba(th.outline))
                    .text_size(px(text::SMALL))
                    .child(icon(layout::mark_icon(site), th.text_dim, 16.0))
                    .child(shown)
                    .child(
                        icon_button(("page-signature-layout-page", n), "close", 16.0, th).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.change_layout(
                                    id,
                                    |l| {
                                        if n < l.pages.len() {
                                            l.pages.remove(n);
                                        }
                                    },
                                    cx,
                                )
                            }),
                        ),
                    )
            }))
            .child(div().flex_1().min_w(px(200.0)).child(line_field(
                "page-signature-layout-add-page",
                &form.page,
                th,
                cx,
            )));
        let current = layout::colour(l);
        // The wheel after the colours: the colour picker, for any other.
        let custom = (!layout::COLOURS.contains(&current)).then_some(current << 8 | 0xff);
        let target = Target::Signature(id);
        let swatches = self.color_swatches.clone();
        let wheel = self
            .page_control(
                color_wheel("page-signature-layout-wheel", custom, 32.0, th),
                th,
                cx,
            )
            .relative()
            .tip(tr!("settings-appearance-accent-more"), th)
            .on_click(
                cx.listener(move |this, _, window, cx| {
                    this.toggle_color_picker(target, window, cx)
                }),
            )
            .child(
                canvas(
                    move |bounds, _, _| {
                        swatches.borrow_mut().insert(target, bounds);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            );
        let colours = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(space::S2))
            .children(layout::COLOURS.into_iter().enumerate().map(|(n, c)| {
                self.page_control(
                    color_swatch(
                        ("page-signature-layout-colour", n),
                        c << 8 | 0xff,
                        c == current,
                        32.0,
                        th,
                    ),
                    th,
                    cx,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.set_layout_colour(id, c, cx);
                    this.sync_color_picker(target, c << 8 | 0xff, cx);
                }))
            }))
            .child(wheel)
            .children(self.render_color_picker(|t| t == target, th, window, cx));
        let section = |said: String, content: AnyElement| {
            div()
                .flex()
                .flex_col()
                .gap(px(space::S2))
                .child(label(said))
                .child(content)
        };
        let readers = [
            (Reader::Light, tr!("signature-layout-light")),
            (Reader::Dark, tr!("signature-layout-dark")),
            (Reader::Text, tr!("signature-layout-text")),
        ];
        let reader = if form.shown.is_none() {
            Reader::Text
        } else {
            form.reader
        };
        let reader_chips = div().flex().flex_row().gap(px(space::S2)).children(
            readers.into_iter().enumerate().map(|(n, (r, said))| {
                choice_chip(("page-signature-layout-reader", n), said, r == reader, th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.set_layout_reader(r, cx)))
            }),
        );
        let paper = match (reader, &form.shown) {
            (Reader::Light | Reader::Dark, Some(doc)) => {
                let dark = reader == Reader::Dark;
                let paper_th = Theme::new(dark);
                div()
                    .id("page-signature-layout-paper")
                    .max_h(px(360.0))
                    .overflow_y_scroll()
                    .p(px(space::S5))
                    .rounded(px(radius::MD))
                    .border_1()
                    .border_color(rgba(th.outline))
                    .bg(rgba(paper_th.surface))
                    .child(crate::window::rich::designed(&paper_th, doc))
                    .into_any_element()
            }
            _ => div()
                .p(px(space::S5))
                .rounded(px(radius::MD))
                .border_1()
                .border_color(rgba(th.outline))
                .font_family("monospace")
                .text_size(px(text::SMALL))
                .whitespace_normal()
                .children(
                    self.config
                        .sending
                        .signature(Some(id))
                        .map(|s| s.text.clone())
                        .unwrap_or_default()
                        .lines()
                        .map(|line| div().min_h(px(18.0)).child(line.to_owned()))
                        .collect::<Vec<_>>(),
                )
                .into_any_element(),
        };
        let inside = layout::pictures_size(&layout::write(l).1);
        let note = (inside > 0 && form.shown.is_some()).then(|| {
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S3))
                .text_size(px(text::SMALL))
                .text_color(rgba(th.text_faint))
                .child(
                    div()
                        .pt(px(space::S1))
                        .child(icon("info", th.text_faint, 14.0)),
                )
                .child(div().flex_1().min_w_0().child(tr!(
                    "signature-layout-inside",
                    size = crate::format::size(inside as u64)
                )))
        });
        let preview = div()
            .flex()
            .flex_col()
            .gap(px(space::S3))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .gap(px(space::S3))
                    .child(label(tr!("signature-layout-preview")))
                    .child(reader_chips),
            )
            .child(paper)
            .children(note);
        let asking = matches!(form_confirm, Some(Confirm::Edit));
        let free = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .justify_end()
            .gap(px(space::S3))
            .pt(px(space::S3))
            .border_t_1()
            .border_color(rgba(th.divider))
            .child(
                outlined_button(
                    "page-signature-layout-edit",
                    tr!("signature-layout-edit"),
                    th,
                )
                .map(|d| self.page_control(d, th, cx))
                .on_click(
                    cx.listener(move |this, _, window, cx| this.pick_layout(id, None, window, cx)),
                ),
            );
        let question = asking.then(|| self.layout_question(id, th, cx)).flatten();
        let details = div()
            .flex_1()
            .min_w(px(280.0))
            .flex()
            .flex_col()
            .gap(px(space::S5))
            .child(fields)
            .child(section(
                tr!("signature-layout-pictures"),
                pictures.into_any_element(),
            ))
            .child(section(
                tr!("signature-layout-pages"),
                pages.into_any_element(),
            ))
            .child(section(
                tr!("signature-layout-colour"),
                colours.into_any_element(),
            ));
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(space::S4))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(space::S6))
                        .child(div().flex_1().min_w(px(300.0)).child(preview))
                        .child(details),
                )
                .child(if asking {
                    div().children(question)
                } else {
                    free
                })
                .into_any_element(),
        )
    }
}

/// The first picture inside `html`, made into a logo: what a designed
/// signature most often starts with.
fn first_picture(html: &str) -> Option<String> {
    let start = html.find("data:image/")?;
    let rest = &html[start..];
    let end = rest.find(['"', '\'', ')', ' ']).unwrap_or(rest.len());
    let uri = &rest[..end];
    let mime = uri.strip_prefix("data:")?.split(';').next()?;
    let format = match katna_preview::kind(mime, "") {
        katna_preview::Kind::Picture(format) => format,
        _ => return None,
    };
    let bytes = layout::picture_bytes(uri)?;
    let made =
        katna_preview::signature::make(&bytes, format, katna_preview::signature::Shape::Logo)
            .ok()?;
    Some(format!(
        "data:{};base64,{}",
        made.mime,
        katna_ui::rich::html::base64_encode(&made.bytes)
    ))
}

/// What a signature is, under its name in the list: its layout, or
/// "Your own", or "Plain text".
pub(super) fn kind(signature: &Signature) -> String {
    match &signature.layout {
        Some(l) => layout::name(l.style),
        None if signature.html.trim().is_empty() => layout::name(LayoutStyle::Plain),
        None => tr!("signature-layout-own"),
    }
}

fn picture(l: &SignatureLayout, shape: katna_preview::signature::Shape) -> &String {
    use katna_preview::signature::Shape;
    match shape {
        Shape::Logo => &l.logo,
        Shape::Photo => &l.photo,
        Shape::Banner => &l.banner,
    }
}

fn picture_mut(l: &mut SignatureLayout, shape: katna_preview::signature::Shape) -> &mut String {
    use katna_preview::signature::Shape;
    match shape {
        Shape::Logo => &mut l.logo,
        Shape::Photo => &mut l.photo,
        Shape::Banner => &mut l.banner,
    }
}

/// How a shape in a tile's drawing is painted.
#[derive(Clone, Copy)]
enum Paint {
    /// The name: text at most strength.
    Ink,
    /// Other text.
    Faint,
    /// The layout's colour.
    Colour,
    /// A frame.
    Edge,
}

/// The tiles' drawings, as in the study: x, y, width, height, corner
/// radius and paint, on a 72 × 46 tile.
fn shapes(style: LayoutStyle) -> &'static [(f32, f32, f32, f32, f32, Paint)] {
    use Paint::*;
    match style {
        LayoutStyle::Classic => &[
            (8.0, 9.0, 40.0, 5.0, 2.0, Ink),
            (8.0, 17.0, 56.0, 3.0, 1.5, Faint),
            (8.0, 25.0, 46.0, 3.0, 1.5, Colour),
            (8.0, 31.0, 40.0, 3.0, 1.5, Faint),
            (8.0, 37.0, 50.0, 3.0, 1.5, Faint),
        ],
        LayoutStyle::LogoLeft => &[
            (6.0, 12.0, 20.0, 20.0, 5.0, Colour),
            (30.0, 8.0, 2.0, 30.0, 0.0, Colour),
            (36.0, 10.0, 28.0, 5.0, 2.0, Ink),
            (36.0, 18.0, 30.0, 3.0, 1.5, Faint),
            (36.0, 25.0, 24.0, 3.0, 1.5, Faint),
            (36.0, 32.0, 5.0, 5.0, 1.5, Colour),
            (43.0, 32.0, 5.0, 5.0, 1.5, Colour),
        ],
        LayoutStyle::Photo => &[
            (6.0, 12.0, 22.0, 22.0, 11.0, Faint),
            (34.0, 12.0, 28.0, 5.0, 2.0, Ink),
            (34.0, 20.0, 20.0, 3.0, 1.5, Colour),
            (34.0, 27.0, 30.0, 3.0, 1.5, Faint),
            (34.0, 33.0, 26.0, 3.0, 1.5, Faint),
        ],
        LayoutStyle::Band => &[
            (6.0, 18.0, 60.0, 22.0, 3.0, Edge),
            (6.0, 8.0, 60.0, 10.0, 3.0, Colour),
            (10.0, 23.0, 22.0, 3.0, 1.5, Faint),
            (38.0, 23.0, 22.0, 3.0, 1.5, Faint),
            (10.0, 30.0, 18.0, 3.0, 1.5, Faint),
            (38.0, 30.0, 20.0, 3.0, 1.5, Colour),
        ],
        LayoutStyle::OneLine => &[
            (6.0, 18.0, 16.0, 4.0, 2.0, Ink),
            (25.0, 18.0, 40.0, 4.0, 2.0, Faint),
            (6.0, 26.0, 26.0, 4.0, 2.0, Faint),
            (35.0, 26.0, 20.0, 4.0, 2.0, Colour),
        ],
        LayoutStyle::Centred => &[
            (30.0, 5.0, 12.0, 12.0, 3.0, Colour),
            (20.0, 20.0, 32.0, 5.0, 2.0, Ink),
            (16.0, 28.0, 40.0, 3.0, 1.5, Faint),
            (27.0, 35.0, 5.0, 5.0, 1.5, Colour),
            (34.0, 35.0, 5.0, 5.0, 1.5, Colour),
            (41.0, 35.0, 5.0, 5.0, 1.5, Colour),
        ],
        LayoutStyle::Banner => &[
            (6.0, 8.0, 30.0, 5.0, 2.0, Ink),
            (6.0, 16.0, 44.0, 3.0, 1.5, Faint),
            (6.0, 25.0, 60.0, 15.0, 3.0, Colour),
        ],
        LayoutStyle::Underline => &[
            (8.0, 8.0, 44.0, 6.0, 2.0, Ink),
            (8.0, 18.0, 12.0, 3.0, 1.5, Colour),
            (8.0, 26.0, 40.0, 3.0, 1.5, Faint),
            (8.0, 33.0, 48.0, 3.0, 1.5, Faint),
        ],
        LayoutStyle::SideBar => &[
            (8.0, 7.0, 4.0, 32.0, 2.0, Colour),
            (17.0, 9.0, 30.0, 5.0, 2.0, Ink),
            (17.0, 17.0, 20.0, 3.0, 1.5, Colour),
            (17.0, 24.0, 40.0, 3.0, 1.5, Faint),
            (17.0, 31.0, 34.0, 3.0, 1.5, Faint),
        ],
        LayoutStyle::Card => &[
            (5.0, 5.0, 62.0, 36.0, 5.0, Edge),
            (10.0, 10.0, 8.0, 8.0, 2.0, Colour),
            (21.0, 12.0, 20.0, 3.0, 1.5, Colour),
            (10.0, 22.0, 28.0, 4.0, 2.0, Ink),
            (10.0, 30.0, 22.0, 3.0, 1.5, Faint),
            (38.0, 30.0, 22.0, 3.0, 1.5, Faint),
        ],
        LayoutStyle::Monogram => &[
            (6.0, 12.0, 22.0, 22.0, 11.0, Colour),
            (34.0, 14.0, 28.0, 5.0, 2.0, Ink),
            (34.0, 23.0, 30.0, 3.0, 1.5, Faint),
            (34.0, 30.0, 24.0, 3.0, 1.5, Faint),
        ],
        LayoutStyle::Plain => &[
            (8.0, 9.0, 30.0, 3.0, 1.0, Ink),
            (8.0, 16.0, 50.0, 3.0, 1.0, Faint),
            (8.0, 23.0, 36.0, 3.0, 1.0, Faint),
            (8.0, 30.0, 44.0, 3.0, 1.0, Faint),
            (8.0, 37.0, 30.0, 3.0, 1.0, Faint),
        ],
    }
}

/// A small drawing of `style` in `colour` (`0xrrggbbaa`).
fn thumb(style: LayoutStyle, colour: u32, th: &Theme) -> Div {
    let ink = th.text & 0xffff_ff00 | 0xcc;
    let faint = th.text & 0xffff_ff00 | 0x59;
    let edge = th.text & 0xffff_ff00 | 0x40;
    div()
        .relative()
        .size_full()
        .children(shapes(style).iter().map(|&(x, y, w, h, r, paint)| {
            let shape = div()
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(w))
                .h(px(h))
                .rounded(px(r));
            match paint {
                Paint::Ink => shape.bg(rgba(ink)),
                Paint::Faint => shape.bg(rgba(faint)),
                Paint::Colour => shape.bg(rgba(colour)),
                Paint::Edge => shape.border_1().border_color(rgba(edge)),
            }
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hand_written_signature_fills_the_fields() {
        let signature = Signature {
            text: "Demo Alam\nAccounts Manager\nDemo Systems Pvt. Ltd.\nM: +91 90000 12345\n\
                   O: +91 33 4000 5678\nwww.demosys.example\nhttps://www.linkedin.com/in/demo"
                .to_owned(),
            ..Signature::default()
        };
        let l = filled_from(&signature, LayoutStyle::Card, "demo@demosys.example");
        assert_eq!(l.style, LayoutStyle::Card);
        assert_eq!(l.name, "Demo Alam");
        assert_eq!(l.mobile, "+91 90000 12345");
        assert_eq!(l.office, "+91 33 4000 5678");
        assert_eq!(l.email, "demo@demosys.example");
        assert!(l.pages.iter().any(|p| p.contains("linkedin.com")), "{l:#?}");
        // Title and company on one line are split.
        let signature = Signature {
            text: "Demo Alam\nAccounts Manager · Demosys\n+91 98000 00000".to_owned(),
            ..Signature::default()
        };
        let l = filled_from(&signature, LayoutStyle::Classic, "");
        assert_eq!(
            (l.title.as_str(), l.company.as_str()),
            ("Accounts Manager", "Demosys"),
            "{l:#?}"
        );
        // An empty one gets the address alone.
        let l = filled_from(&Signature::default(), LayoutStyle::Classic, "a@b.example");
        assert_eq!((l.name.as_str(), l.email.as_str()), ("", "a@b.example"));
    }

    #[test]
    fn the_saved_signature_is_a_designed_block() {
        let l = SignatureLayout {
            style: LayoutStyle::SideBar,
            name: "Demo".to_owned(),
            ..SignatureLayout::default()
        };
        let (text, html, shown) = written(&l);
        assert_eq!(text, "Demo");
        assert!(html.contains(katna_ui::rich::html::HTML_START), "{html}");
        assert!(shown.is_some());
        let plain = SignatureLayout {
            style: LayoutStyle::Plain,
            ..l
        };
        let (_, html, shown) = written(&plain);
        assert!(html.is_empty() && shown.is_none());
    }
}
