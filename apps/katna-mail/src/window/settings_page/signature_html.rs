// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Compose > Signatures > Paste HTML: a signature designed
//! elsewhere (a signature website, another mail app) pasted as HTML, or a
//! designed signature's HTML edited by hand. What is pasted is cleaned
//! ([`katna_render::html::clean`]), its pictures on the web are downloaded
//! once and carried inside the mail, and it is saved as one designed block
//! that the compose window shows as it will be sent and doesn't rewrite.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnyElement, Context, Entity, Focusable, Subscription, Task, Window, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_render::html::{self as mail_html, Document, LeftOut};
use katna_ui::rich::{Block, Doc, HtmlBlock, Image, html};
use katna_ui::tokens::{space, text};
use katna_ui::{InputEvent, TextArea, px};

use super::{MailWindow, control_column};
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::{field, filled_button, icon, outlined_button};

/// Typing pauses this long before pictures are fetched.
const SETTLE: Duration = Duration::from_millis(400);
/// Most pictures on the web one signature brings inside.
const MAX_PICTURES: usize = 10;
/// Largest picture brought inside: the signature is kept in the settings
/// file, and every message carries it.
const MAX_PICTURE: usize = 512 * 1024;

/// The Paste HTML panel.
pub(super) struct PasteHtml {
    /// The signature whose HTML is edited; `None` makes a new one.
    signature: Option<u32>,
    area: Entity<TextArea>,
    /// That signature's pictures, which its HTML names `cid:katna-N`.
    kept: Vec<Image>,
    /// Pictures from the web by address; `None` when one could not be had.
    fetched: HashMap<String, Option<Vec<u8>>>,
    fetching: bool,
    /// The signature as it would be saved.
    result: Option<Pasted>,
    settle: Option<Task<()>>,
    fetch: Option<Task<()>>,
    _subscription: Subscription,
}

struct Pasted {
    block: HtmlBlock,
    doc: Rc<Document>,
    left_out: LeftOut,
    /// Pictures from the web now inside, and their size.
    inside: usize,
    bytes: usize,
}

impl MailWindow {
    /// Opens the panel: on signature `id`'s HTML, or empty for a new one.
    pub(super) fn open_paste_html(
        &mut self,
        id: Option<u32>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let designed = id
            .and_then(|id| self.config.sending.signature(Some(id)))
            .and_then(|s| {
                crate::signatures::doc(s)
                    .blocks
                    .into_iter()
                    .find_map(|b| match b {
                        Block::Html(block) => Some(block),
                        _ => None,
                    })
            });
        let (source, kept) = match designed {
            Some(block) => (block.html.to_string(), block.images),
            None => (String::new(), Vec::new()),
        };
        let accent = rgba(self.theme(window).accent).into();
        let area = cx.new(|cx| {
            let mut area = TextArea::new(tr!("signature-html-placeholder"), cx);
            area.set_text(source, 0, cx);
            area.set_accent(accent);
            area
        });
        let subscription =
            cx.subscribe_in(
                &area,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Changed => this.paste_html_changed(cx),
                    InputEvent::Cancel => this.close_paste_html(cx),
                    InputEvent::Submit => {}
                },
            );
        window.focus(&area.focus_handle(cx), cx);
        if let Some(page) = &mut self.settings_page {
            page.pasting = Some(PasteHtml {
                signature: id,
                area,
                kept,
                fetched: HashMap::new(),
                fetching: false,
                result: None,
                settle: None,
                fetch: None,
                _subscription: subscription,
            });
        }
        self.fetch_paste_pictures(cx);
    }

    fn pasting(&mut self) -> Option<&mut PasteHtml> {
        self.settings_page.as_mut()?.pasting.as_mut()
    }

    fn close_paste_html(&mut self, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.settings_page {
            page.pasting = None;
        }
        cx.notify();
    }

    /// Shows the HTML at once, and fetches its pictures once typing
    /// pauses.
    fn paste_html_changed(&mut self, cx: &mut Context<Self>) {
        self.paste_html_cleaned(cx);
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SETTLE).await;
            this.update(cx, |this, cx| this.fetch_paste_pictures(cx))
                .ok();
        });
        if let Some(p) = self.pasting() {
            p.settle = Some(task);
        }
    }

    /// Downloads the pictures on the web the HTML shows that are not
    /// downloaded yet, through the daemon, then cleans it again.
    fn fetch_paste_pictures(&mut self, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        let Some(p) = self.pasting() else {
            return;
        };
        let source = p.area.read(cx).text().to_owned();
        let wanted: Vec<String> = mail_html::web_pictures(&source)
            .into_iter()
            .take(MAX_PICTURES)
            .filter(|url| !p.fetched.contains_key(url))
            .collect();
        if wanted.is_empty() {
            self.paste_html_cleaned(cx);
            return;
        }
        p.fetching = true;
        let task = cx.spawn(async move |this, cx| {
            let got = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => Some(connection),
                        None => daemon::connect().await.ok(),
                    };
                    let mut got = Vec::new();
                    for url in wanted {
                        let bytes = match &connection {
                            Some(connection) => daemon::fetch_image(connection, &url).await.ok(),
                            None => None,
                        };
                        got.push((url, bytes.filter(|b| b.len() <= MAX_PICTURE)));
                    }
                    got
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(p) = this.pasting() {
                    p.fetched.extend(got);
                    p.fetching = false;
                }
                this.paste_html_cleaned(cx);
            })
            .ok();
        });
        if let Some(p) = self.pasting() {
            p.fetch = Some(task);
        }
        cx.notify();
    }

    /// Cleans the HTML as it stands, with the pictures fetched so far.
    fn paste_html_cleaned(&mut self, cx: &mut Context<Self>) {
        let Some(p) = self.settings_page.as_mut().and_then(|p| p.pasting.as_mut()) else {
            return;
        };
        let source = p.area.read(cx).text().to_owned();
        let fetched: HashMap<String, Vec<u8>> = p
            .fetched
            .iter()
            .filter_map(|(url, bytes)| Some((url.clone(), bytes.clone()?)))
            .collect();
        let cleaned = mail_html::clean(&source, &fetched);
        p.result = (!cleaned.html.is_empty()).then(|| {
            let inside: Vec<&Vec<u8>> = mail_html::web_pictures(&source)
                .iter()
                .filter_map(|url| fetched.get(url))
                .collect();
            let mut next = 0;
            let block = html::html_block_with(&cleaned.html, &p.kept, &mut next);
            let doc = Rc::new(crate::window::rich::designed_document(&block));
            Pasted {
                block,
                doc,
                left_out: cleaned.left_out,
                inside: inside.len(),
                bytes: inside.iter().map(|b| b.len()).sum(),
            }
        });
        cx.notify();
    }

    fn save_paste_html(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(page) = &mut self.settings_page else {
            return;
        };
        let Some(pasting) = page.pasting.take() else {
            return;
        };
        let Some(result) = pasting.result else {
            return;
        };
        let doc = Doc {
            blocks: vec![Block::Html(result.block)],
        };
        let (text, html) = crate::window::compose::signature_content(&doc);
        let id = match pasting.signature {
            Some(id) => id,
            None => self
                .config
                .sending
                .add_signature(tr!("signature-html-name"), String::new()),
        };
        if let Some(signature) = self
            .config
            .sending
            .signatures
            .iter_mut()
            .find(|s| s.id == id)
        {
            signature.text = text;
            signature.html = html;
        }
        self.save_config();
        self.edit_signature(Some(id), window, cx);
    }

    /// The panel, in place of the signature editor while it is open.
    pub(super) fn paste_html_panel(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let p = self.settings_page.as_ref()?.pasting.as_ref()?;
        let focus = p.area.focus_handle(cx);
        let faint = rgba(th.text_faint);
        let title = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_baseline()
            .gap(px(space::S3))
            .child(
                div()
                    .text_size(px(text::SUBTITLE))
                    .child(tr!("signature-html-title")),
            )
            .child(
                div()
                    .text_size(px(text::SMALL))
                    .text_color(faint)
                    .child(tr!("signature-html-subtitle")),
            );
        let code = field("page-signature-html", &focus, th).child(
            div()
                .id("page-signature-html-scroll")
                .min_h(px(120.0))
                .max_h(px(220.0))
                .overflow_y_scroll()
                .py(px(space::S3))
                .font_family("monospace")
                .text_size(px(text::SMALL))
                .child(p.area.clone()),
        );
        // Drawn on its own page, as the reading pane draws it.
        let preview = p.result.as_ref().map(|r| {
            div()
                .id("page-signature-html-preview")
                .max_h(px(320.0))
                .overflow_y_scroll()
                .child(crate::window::rich::designed(th, &r.doc))
        });
        let line = |name: &str, said: String| {
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S3))
                .text_size(px(text::SMALL))
                .text_color(faint)
                .child(
                    div()
                        .pt(px(space::S1))
                        .child(icon(name, th.text_faint, 14.0)),
                )
                .child(div().flex_1().min_w_0().child(said))
        };
        let mut notes = Vec::new();
        if p.fetching {
            notes.push(line("download", tr!("signature-html-fetching")));
        }
        if let Some(r) = &p.result {
            let out = r.left_out;
            if r.inside > 0 {
                let size = crate::format::size(r.bytes as u64);
                notes.push(line(
                    "check",
                    tr!(
                        "signature-html-pictures-inside",
                        count = r.inside,
                        size = size
                    ),
                ));
            }
            if out.web_pictures > 0 && !p.fetching {
                notes.push(line(
                    "info",
                    tr!("signature-html-pictures-web", count = out.web_pictures),
                ));
            }
            if out.active + out.trackers > 0 {
                notes.push(line("check", tr!("signature-html-removed")));
            }
            if out.style_sheets > 0 {
                notes.push(line("info", tr!("signature-html-style-sheet")));
            }
            if out.links > 0 {
                notes.push(line("info", tr!("signature-html-links")));
            }
            notes.push(line("check", tr!("signature-html-plain-text")));
        }
        let target = match p
            .signature
            .and_then(|id| self.config.sending.signature(Some(id)))
        {
            Some(s) => tr!("signature-html-replaces", name = s.name.clone()),
            None => tr!("signature-html-new", name = tr!("signature-html-name")),
        };
        let can_save = p.result.is_some();
        let buttons = div()
            .flex()
            .flex_row()
            .gap(px(space::S3))
            .child(
                outlined_button(
                    "page-signature-html-cancel",
                    tr!("signature-html-cancel"),
                    th,
                )
                .map(|d| self.page_control(d, th, cx))
                .on_click(cx.listener(|this, _, _, cx| this.close_paste_html(cx))),
            )
            .child(
                filled_button("page-signature-html-save", tr!("signature-html-save"), th)
                    .map(|d| self.page_control(d, th, cx))
                    .when(!can_save, |d| d.opacity(katna_ui::tokens::state::DISABLED))
                    .on_click(cx.listener(|this, _, window, cx| this.save_paste_html(window, cx))),
            );
        Some(
            control_column(240.0)
                .flex()
                .flex_col()
                .gap(px(space::S3))
                .child(title)
                .child(code)
                .children(preview)
                .child(div().flex().flex_col().gap(px(space::S2)).children(notes))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .justify_between()
                        .gap(px(space::S3))
                        .child(
                            div()
                                .text_size(px(text::SMALL))
                                .text_color(faint)
                                .child(target),
                        )
                        .child(buttons),
                )
                .into_any_element(),
        )
    }
}
