// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail templates in the compose window: the Templates button and its
//! menu, putting a template in the message, and saving the message as a
//! new template. Templates live in `pim.db`; the daemon saves them.

use std::sync::Arc;

use gpui::{AnyElement, Context, Focusable, Window, div, prelude::*, rgba};
use katna_dbus::{TemplateFileItem, TemplateItem};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::{Block, Doc, html};

use super::super::MailWindow;
use super::attach::Attachment;
use super::recipients::Field;
use super::tools::{Popup, above, menu_divider};
use crate::daemon;
use crate::data;
use crate::outgoing::Mailbox;
use crate::templates;
use crate::theme::Theme;
use crate::widgets::{icon_button, menu, menu_item, tip};

impl MailWindow {
    /// The Templates button beside the signature one, and its menu: the
    /// templates to put in, Save as template and Manage templates.
    pub(super) fn render_templates_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let open = compose.popup == Some(Popup::Templates);
        div()
            .relative()
            .child(
                icon_button("compose-templates", "template", super::tools::TRAY_ICON, th)
                    .size(px(super::tools::TRAY_TOOL))
                    .tooltip(tip(tr!("compose-tool-templates"), th))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_popup(Popup::Templates, cx);
                        this.load_templates(cx);
                    })),
            )
            .when(open, |d| d.child(above(self.templates_menu(th, cx))))
            .into_any_element()
    }

    /// The templates to put in, with Save as template and Manage.
    pub(super) fn templates_menu(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Div {
        let items = self
            .writing
            .templates
            .iter()
            .enumerate()
            .map(|(ix, t)| {
                let id = t.id;
                menu_item(("compose-template-item", ix), &t.name, th).on_click(
                    cx.listener(move |this, _, window, cx| this.insert_template(id, window, cx)),
                )
            })
            .collect::<Vec<_>>();
        let none = items.is_empty();
        menu(th)
            .w(px(260.0))
            .when(none, |d| {
                d.child(
                    div()
                        .px(px(16.0))
                        .py(px(8.0))
                        .text_size(px(13.0))
                        .line_height(px(18.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("compose-tool-templates-none")),
                )
            })
            .children(items)
            .child(menu_divider(th))
            .child(
                menu_item(
                    "compose-template-save",
                    &tr!("compose-tool-template-save"),
                    th,
                )
                .on_click(cx.listener(|this, _, window, cx| this.open_save_template(window, cx))),
            )
            .child(
                menu_item(
                    "compose-templates-manage",
                    &tr!("compose-tool-templates-manage"),
                    th,
                )
                .on_click(cx.listener(|this, _, window, cx| {
                    if let Some(c) = &mut this.compose {
                        c.popup = None;
                    }
                    this.open_settings_page(
                        super::super::settings_page::Section::Signatures,
                        window,
                        cx,
                    );
                })),
            )
    }

    /// Reads the list of templates again, for the menus that show them.
    pub(in crate::window) fn load_templates(&mut self, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        cx.spawn(async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move { data::templates(&paths) })
                .await;
            this.update(cx, |this, cx| {
                match found {
                    Ok(list) => this.writing.templates = list,
                    Err(err) => tracing::warn!("{err}"),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// The saved templates, as last read.
    pub(in crate::window) fn writing_templates(&self) -> &[katna_store::TemplateSummary] {
        &self.writing.templates
    }

    /// Puts template `id` in the open message: its text (with the fields
    /// filled in), its subject when there is none yet, and its files.
    fn insert_template(&mut self, id: i64, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose {
            c.popup = None;
        }
        let paths = self.paths.clone();
        cx.spawn_in(window, async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move { data::template(&paths, id) })
                .await;
            this.update_in(cx, |this, window, cx| match found {
                Ok(Some(template)) => this.put_template(template, window, cx),
                Ok(None) => this.load_templates(cx),
                Err(err) => {
                    tracing::warn!("{err}");
                    this.show_snackbar(tr!("compose-template-open-failed"), None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    fn put_template(
        &mut self,
        template: katna_store::Template,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let me = self.compose_sender();
        let Some(c) = &mut self.compose else {
            return;
        };
        let recipient = c
            .chips
            .get(Field::To)
            .iter()
            .find_map(|chip| chip.mailbox());
        let fields = templates::fields(recipient.as_ref(), &me);
        c.from_template = true;
        let body = c.body.clone();
        body.update(cx, |editor, cx| {
            editor.insert_template(&template.html, &template.text, &fields, cx)
        });
        let subject = c.subject.clone();
        if subject.read(cx).text().trim().is_empty() && !template.subject.trim().is_empty() {
            let mut text = template.subject.clone();
            for (field, value) in &fields {
                text = text.replace(field, value);
            }
            subject.update(cx, |input, cx| input.set_text(text, cx));
        }
        c.attachments
            .extend(template.attachments.into_iter().map(|file| Attachment {
                name: file.name,
                mime: file.mime,
                data: Arc::new(file.data),
            }));
        window.focus(&body.focus_handle(cx), cx);
        cx.notify();
    }

    /// The name the open message goes out under, for `{my name}`.
    pub(super) fn compose_sender(&self) -> Mailbox {
        let account = self.compose.as_ref().and_then(|c| {
            c.from
                .and_then(|id| self.accounts.iter().find(|a| a.id == id))
                .or_else(|| self.compose_account(c.kind))
        });
        account.map_or_else(
            || Mailbox {
                name: None,
                email: String::new(),
            },
            |a| Mailbox {
                name: Some(a.display_name.clone()),
                email: a.address.clone(),
            },
        )
    }

    /// Fills the fields a template left for the recipient, once the
    /// message is on its way and the recipient is known. Tells whether
    /// anything changed.
    pub(super) fn fill_template_fields(
        &mut self,
        recipient: Option<&Mailbox>,
        cx: &mut Context<Self>,
    ) -> bool {
        let me = self.compose_sender();
        let Some(c) = &mut self.compose else {
            return false;
        };
        if !c.from_template {
            return false;
        }
        let fields = templates::fields(recipient, &me);
        let body = c.body.clone();
        let mut changed = body.update(cx, |editor, cx| editor.fill_fields(&fields, cx));
        let subject = c.subject.clone();
        let text = subject.read(cx).text().to_owned();
        let mut filled = text.clone();
        for (field, value) in &fields {
            filled = filled.replace(field, value);
        }
        if filled != text {
            subject.update(cx, |input, cx| input.set_text(filled, cx));
            changed = true;
        }
        changed
    }

    fn open_save_template(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        c.popup = Some(Popup::SaveTemplate);
        let name = c.subject.read(cx).text().trim().to_owned();
        let input = c.dialog.template_name.clone();
        input.update(cx, |i, cx| {
            i.set_text(name, cx);
            i.select_all_text(cx);
        });
        window.focus(&input.focus_handle(cx), cx);
        cx.notify();
    }

    /// Saves the open message as a new template, under the typed name: its
    /// subject, its text without the signature, and its files.
    pub(super) fn save_template(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &self.compose else {
            return;
        };
        let name = c.dialog.template_name.read(cx).text().trim().to_owned();
        if name.is_empty() {
            return;
        }
        let mut doc = c.body.read(cx).doc().clone();
        doc.remove_signature();
        trim_blank_end(&mut doc);
        // The same name saves over that template.
        let id = self
            .writing
            .templates
            .iter()
            .find(|t| t.name.trim().eq_ignore_ascii_case(&name))
            .map_or(0, |t| t.id);
        let template = TemplateItem {
            id,
            name: name.clone(),
            subject: c.subject.read(cx).text().trim().to_owned(),
            html: if c.plain(cx) {
                String::new()
            } else {
                html::to_html(&doc, &html::data_uri)
            },
            text: html::to_plain(&doc),
            attachments: c
                .attachments
                .iter()
                .map(|a| TemplateFileItem {
                    name: a.name.clone(),
                    mime: a.mime.clone(),
                    data: a.data.to_vec(),
                })
                .collect(),
        };
        self.close_popup(window, cx);
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::save_template(&connection, &template).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(_) => {
                        this.show_snackbar(tr!("compose-template-saved", name = name), None, cx);
                        this.load_templates(cx);
                    }
                    Err(err) => this.show_snackbar(
                        tr!("compose-template-save-failed", error = err),
                        None,
                        cx,
                    ),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub(super) fn render_save_template_dialog(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        Self::dialog_card(th, 400.0, tr!("compose-tool-template-save-title"))
            .child(
                div()
                    .mb(px(12.0))
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("compose-tool-template-save-text")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("compose-tool-template-name")),
                    )
                    .child(
                        div()
                            .h(px(40.0))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .rounded(px(6.0))
                            .border_1()
                            .border_color(rgba(th.outline))
                            .text_size(px(14.0))
                            .child(compose.dialog.template_name.clone()),
                    ),
            )
            .child(self.dialog_buttons(
                th,
                tr!("compose-tool-template-save-ok"),
                cx,
                |this, window, cx| this.save_template(window, cx),
            ))
            .into_any_element()
    }
}

/// Drops empty paragraphs at the end, left where the signature was.
fn trim_blank_end(doc: &mut Doc) {
    while doc.blocks.len() > 1
        && matches!(doc.blocks.last(), Some(Block::Para(p)) if p.text.trim().is_empty())
    {
        doc.blocks.pop();
    }
}
