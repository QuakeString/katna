// SPDX-License-Identifier: GPL-3.0-or-later

//! The signature editor of the Settings page: the same rich editor as the
//! message, with a formatting bar like webmail's signature box (font,
//! size, bold, italic, underline, color, link, picture, alignment, lists,
//! indent, remove formatting, table). A signature is saved as HTML, with
//! its pictures inside, and as plain text.

use gpui::{
    AnyElement, Context, Entity, Focusable, MouseButton, Window, deferred, div, point, prelude::*,
    rgba,
};
use katna_i18n::tr;
use katna_ui::anchored;
use katna_ui::px;
use katna_ui::rich::{Align, Doc, Font, List, RichEditor, Size, html};
use katna_ui::{InputEvent, TextInput};

use super::super::MailWindow;
use super::palette;
use super::tools::{COLORS, format_button, format_dropdown, normalize_url, separator};
use crate::theme::Theme;
use crate::widgets::{menu, menu_item, tip};

/// Largest picture a signature takes: it is stored in the settings file.
const MAX_PICTURE: usize = 512 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Popup {
    Font,
    Size,
    Colors,
    Link,
}

/// The state of the signature editor's bar.
pub(in crate::window) struct SignatureTools {
    popup: Option<Popup>,
    link: Entity<TextInput>,
    _subscription: gpui::Subscription,
}

/// A signature's plain text and, when it has formatting, its HTML.
pub(in crate::window) fn signature_content(doc: &Doc) -> (String, String) {
    let text = html::to_plain(doc).trim_end().to_owned();
    let html = if doc.has_formatting() {
        html::to_html(doc, &|image| html::data_uri(image))
    } else {
        String::new()
    };
    (text, html)
}

impl MailWindow {
    /// An editor on `signature`, with the bar's state.
    pub(in crate::window) fn signature_editor(
        &mut self,
        signature: &katna_core::config::Signature,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<RichEditor> {
        let th = self.theme(window);
        let doc = crate::signatures::doc(signature);
        let accent: gpui::Hsla = rgba(th.accent).into();
        let link = cx.new(|cx| {
            let mut input = TextInput::new(tr!("compose-tool-link-address"), cx);
            input.set_accent(accent);
            input
        });
        let subscription = cx.subscribe_in(
            &link,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.apply_signature_link(window, cx),
                InputEvent::Cancel => {
                    if let Some(t) = &mut this.writing.signature_tools {
                        t.popup = None;
                    }
                    cx.notify();
                }
                InputEvent::Changed => {}
            },
        );
        self.writing.signature_tools = Some(SignatureTools {
            popup: None,
            link,
            _subscription: subscription,
        });
        let editor = cx.new(|cx| {
            let mut editor = RichEditor::new(tr!("signature-placeholder"), cx);
            editor.set_palette(palette(&th));
            editor.set_html_view(super::super::rich::html_view(th));
            editor.set_doc(doc.clone(), doc.start(), cx);
            editor
        });
        self.writing.signature_editor = Some(editor.clone());
        editor
    }

    fn signature_body(&self) -> Option<Entity<RichEditor>> {
        self.writing.signature_editor.clone()
    }

    fn toggle_signature_popup(
        &mut self,
        popup: Popup,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(tools) = &mut self.writing.signature_tools else {
            return;
        };
        tools.popup = if tools.popup == Some(popup) {
            None
        } else {
            Some(popup)
        };
        if tools.popup == Some(Popup::Link)
            && let Some(editor) = self.writing.signature_editor.clone()
        {
            let url = editor
                .read(cx)
                .link_at_cursor()
                .map(|(_, url)| url.to_string())
                .unwrap_or_default();
            let input = tools.link.clone();
            input.update(cx, |i, cx| {
                i.set_text(url, cx);
                i.select_all_text(cx);
            });
            window.focus(&input.focus_handle(cx), cx);
        }
        cx.notify();
    }

    fn signature_edit(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        f: impl FnOnce(&mut RichEditor, &mut Context<RichEditor>),
    ) {
        if let Some(tools) = &mut self.writing.signature_tools {
            tools.popup = None;
        }
        let Some(editor) = self.signature_body() else {
            return;
        };
        editor.update(cx, f);
        window.focus(&editor.focus_handle(cx), cx);
        cx.notify();
    }

    fn on_signature<E>(
        &self,
        cx: &mut Context<Self>,
        f: impl Fn(&mut RichEditor, &mut Context<RichEditor>) + 'static,
    ) -> impl Fn(&E, &mut Window, &mut gpui::App) + 'static {
        cx.listener(move |this, _: &E, window, cx| this.signature_edit(window, cx, &f))
    }

    fn apply_signature_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(tools) = &self.writing.signature_tools else {
            return;
        };
        let url = normalize_url(tools.link.read(cx).text());
        let Some(editor) = self.signature_body() else {
            return;
        };
        let text = {
            let e = editor.read(cx);
            e.link_at_cursor()
                .map(|(text, _)| text)
                .unwrap_or_else(|| e.selected_text())
        };
        self.signature_edit(window, cx, move |e, cx| {
            if url.is_empty() {
                e.remove_link(cx);
            } else {
                let text = if text.trim().is_empty() {
                    url.trim_start_matches("mailto:").to_owned()
                } else {
                    text
                };
                e.set_link(&text, &url, cx);
            }
        });
    }

    fn pick_signature_picture(&mut self, cx: &mut Context<Self>) {
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
            let data = cx
                .background_executor()
                .spawn(async move { std::fs::read(&path) })
                .await;
            this.update(cx, |this, cx| {
                let data = match data {
                    Ok(data) if data.len() > MAX_PICTURE => {
                        this.show_snackbar(
                            tr!(
                                "signature-picture-too-big",
                                size = crate::format::size(MAX_PICTURE as u64)
                            ),
                            None,
                            cx,
                        );
                        return;
                    }
                    Ok(data) => data,
                    Err(err) => {
                        this.show_snackbar(
                            tr!(
                                "signature-picture-unreadable",
                                name = name.as_str(),
                                error = err.to_string()
                            ),
                            None,
                            cx,
                        );
                        return;
                    }
                };
                let Some(mime) = katna_ui::rich::image_mime(&name) else {
                    this.show_snackbar(tr!("signature-picture-kind"), None, cx);
                    return;
                };
                if let Some(editor) = this.signature_body() {
                    editor.update(cx, |e, cx| {
                        e.insert_image(name, mime.to_owned(), data, cx);
                    });
                }
            })
            .ok();
        })
        .detach();
    }

    /// The formatting bar under the signature being edited.
    pub(in crate::window) fn render_signature_tools(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let tools = self.writing.signature_tools.as_ref()?;
        let editor = self.writing.signature_editor.as_ref()?.read(cx);
        let style = editor.current_style();
        let para = editor.para_style();
        let popup = tools.popup;
        let open = |p: Popup| popup == Some(p);

        let font = div()
            .relative()
            .child(
                format_dropdown("sig-font", th)
                    .w(px(104.0))
                    .tooltip(tip(tr!("compose-tool-font"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_signature_popup(Popup::Font, window, cx)
                    }))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .child(style.font.label()),
                    )
                    .child(crate::widgets::icon("drop-down", th.text_dim, 18.0)),
            )
            .when(open(Popup::Font), |d| {
                let items = Font::ALL
                    .into_iter()
                    .enumerate()
                    .map(|(ix, font)| {
                        menu_item(("sig-font-item", ix), font.label(), th)
                            .on_click(self.on_signature(cx, move |e, cx| e.set_font(font, cx)))
                    })
                    .collect::<Vec<_>>();
                d.child(below(menu(th).w(px(200.0)).children(items)))
            });
        let size = div()
            .relative()
            .child(
                format_dropdown("sig-size", th)
                    .tooltip(tip(tr!("compose-tool-size"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_signature_popup(Popup::Size, window, cx)
                    }))
                    .child(crate::widgets::icon("text-size", th.text_dim, 18.0))
                    .child(crate::widgets::icon("drop-down", th.text_dim, 18.0)),
            )
            .when(open(Popup::Size), |d| {
                let items = Size::ALL
                    .into_iter()
                    .enumerate()
                    .map(|(ix, size)| {
                        menu_item(("sig-size-item", ix), size.label(), th)
                            .text_size(px(14.0 * size.scale()))
                            .on_click(self.on_signature(cx, move |e, cx| e.set_size(size, cx)))
                    })
                    .collect::<Vec<_>>();
                d.child(below(menu(th).w(px(160.0)).children(items)))
            });
        let colors = div()
            .relative()
            .child(
                format_dropdown("sig-color", th)
                    .tooltip(tip(tr!("compose-tool-text-color"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_signature_popup(Popup::Colors, window, cx)
                    }))
                    .child(super::tools::color_swatch(
                        style.color,
                        style.background,
                        th,
                    ))
                    .child(crate::widgets::icon("drop-down", th.text_dim, 18.0)),
            )
            .when(open(Popup::Colors), |d| {
                let swatches = COLORS
                    .iter()
                    .enumerate()
                    .map(|(ix, &c)| {
                        div()
                            .id(("sig-swatch", ix))
                            .size(px(18.0))
                            .rounded(px(3.0))
                            .bg(rgba((c << 8) | 0xff))
                            .border_1()
                            .border_color(rgba(if style.color == Some(c) {
                                th.text
                            } else {
                                th.divider
                            }))
                            .cursor_pointer()
                            .on_click(self.on_signature(cx, move |e, cx| e.set_color(Some(c), cx)))
                    })
                    .collect::<Vec<_>>();
                d.child(below(
                    menu(th)
                        .min_w(px(0.0))
                        .p(px(10.0))
                        .gap(px(6.0))
                        .child(
                            div()
                                .w(px(160.0))
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .gap(px(2.0))
                                .children(swatches),
                        )
                        .child(
                            menu_item("sig-color-default", &tr!("compose-tool-default-color"), th)
                                .px(px(4.0))
                                .on_click(self.on_signature(cx, |e, cx| e.set_color(None, cx))),
                        ),
                ))
            });
        let link = div()
            .relative()
            .child(
                format_button("sig-link", "link", style.link.is_some(), th)
                    .tooltip(tip(tr!("signature-link"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_signature_popup(Popup::Link, window, cx)
                    })),
            )
            .when(open(Popup::Link), |d| {
                d.child(below(
                    menu(th)
                        .w(px(360.0))
                        .px(px(12.0))
                        .gap(px(8.0))
                        .child(
                            div()
                                .h(px(36.0))
                                .px(px(10.0))
                                .flex()
                                .items_center()
                                .rounded(px(6.0))
                                .border_1()
                                .border_color(rgba(th.outline))
                                .child(div().flex_1().child(tools.link.clone())),
                        )
                        .child(
                            div().flex().flex_row().justify_end().child(
                                crate::widgets::filled_button(
                                    "sig-link-ok",
                                    tr!("signature-link-apply"),
                                    th,
                                )
                                .on_click(cx.listener(
                                    |this, _, window, cx| this.apply_signature_link(window, cx),
                                )),
                            ),
                        ),
                ))
            });
        let mut align = |id: &'static str, name: &'static str, value: Align, label: String| {
            format_button(id, name, para.align == value, th)
                .tooltip(tip(label, th))
                .on_click(self.on_signature(cx, move |e, cx| e.set_align(value, cx)))
        };
        let aligns = [
            align(
                "sig-left",
                "align-left",
                Align::Left,
                tr!("signature-align-left"),
            ),
            align(
                "sig-center",
                "align-center",
                Align::Center,
                tr!("signature-align-center"),
            ),
            align(
                "sig-right",
                "align-right",
                Align::Right,
                tr!("signature-align-right"),
            ),
        ];
        let scrim = popup.filter(|p| *p != Popup::Link).map(|_| {
            deferred(
                anchored().position(point(px(0.0), px(0.0))).child(
                    div()
                        .id("sig-popup-scrim")
                        .w(px(16384.0))
                        .h(px(16384.0))
                        .occlude()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                if let Some(t) = &mut this.writing.signature_tools {
                                    t.popup = None;
                                }
                                cx.notify();
                            }),
                        ),
                ),
            )
            .with_priority(1)
        });
        Some(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(1.0))
                .py(px(4.0))
                .child(font)
                .child(separator(th))
                .child(size)
                .child(separator(th))
                .child(
                    format_button("sig-bold", "format-bold", style.bold, th)
                        .tooltip(tip(tr!("signature-bold"), th))
                        .on_click(self.on_signature(cx, |e, cx| e.toggle_bold(cx))),
                )
                .child(
                    format_button("sig-italic", "format-italic", style.italic, th)
                        .tooltip(tip(tr!("signature-italic"), th))
                        .on_click(self.on_signature(cx, |e, cx| e.toggle_italic(cx))),
                )
                .child(
                    format_button("sig-underline", "format-underline", style.underline, th)
                        .tooltip(tip(tr!("signature-underline"), th))
                        .on_click(self.on_signature(cx, |e, cx| e.toggle_underline(cx))),
                )
                .child(colors)
                .child(separator(th))
                .child(link)
                .child(
                    format_button("sig-image", "image", false, th)
                        .tooltip(tip(tr!("signature-picture"), th))
                        .on_click(cx.listener(|this, _, _, cx| this.pick_signature_picture(cx))),
                )
                .child(
                    format_button("sig-table", "table", false, th)
                        .tooltip(tip(tr!("compose-tool-insert-table"), th))
                        .on_click(self.on_signature(cx, |e, cx| e.insert_table(2, 2, cx))),
                )
                .child(separator(th))
                .children(aligns)
                .child(
                    format_button(
                        "sig-numbered",
                        "list-numbered",
                        para.list == List::Numbered,
                        th,
                    )
                    .tooltip(tip(tr!("signature-numbered-list"), th))
                    .on_click(self.on_signature(cx, |e, cx| e.toggle_list(List::Numbered, cx))),
                )
                .child(
                    format_button(
                        "sig-bulleted",
                        "list-bulleted",
                        para.list == List::Bullet,
                        th,
                    )
                    .tooltip(tip(tr!("signature-bulleted-list"), th))
                    .on_click(self.on_signature(cx, |e, cx| e.toggle_list(List::Bullet, cx))),
                )
                .child(
                    format_button("sig-clear", "clear-format", false, th)
                        .tooltip(tip(tr!("signature-remove-formatting"), th))
                        .on_click(self.on_signature(cx, |e, cx| e.clear_formatting(cx))),
                )
                .children(scrim)
                .into_any_element(),
        )
    }
}

/// A popup under the element it belongs to (in a `relative` parent), kept
/// inside the window.
fn below(popup: impl IntoElement) -> AnyElement {
    deferred(
        div().absolute().top_0().left_0().child(
            anchored()
                .offset(point(px(0.0), px(32.0)))
                .snap_to_window_with_margin(px(8.0))
                .child(div().occlude().child(popup)),
        ),
    )
    .with_priority(2)
    .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_signatures_keep_no_html() {
        let (text, html) = signature_content(&html::from_plain("Kay\nEnron\n"));
        assert_eq!(text, "Kay\nEnron");
        assert_eq!(html, "");
        let mut next = 0;
        let rich = html::from_html("<div><b>Kay</b></div>", &mut next);
        let (text, html) = signature_content(&rich);
        assert_eq!(text, "Kay");
        assert!(html.contains("<b>Kay</b>"), "{html}");
    }
}
