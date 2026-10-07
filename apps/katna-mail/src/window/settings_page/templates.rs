// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Compose > Templates: the saved templates, and an editor for
//! one (its name, subject, text and files) with Save and Delete. Templates
//! are saved in compose (Templates > Save as template) or with New here;
//! the daemon writes them to `pim.db`.

use gpui::{
    AnyElement, Context, Div, Entity, Focusable, Subscription, Window, div, prelude::*, rgba,
};
use katna_dbus::{TemplateFileItem, TemplateItem};
use katna_i18n::tr;
use katna_store::TemplateFile;
use katna_ui::px;
use katna_ui::rich::{RichEvent, html};
use katna_ui::{InputEvent, RichEditor, TextInput};

use super::{MailWindow, control_column, label_column};
use crate::daemon;
use crate::data;
use crate::theme::Theme;
use crate::widgets::{field, line_field};
use crate::widgets::{filled_button, icon, icon_button, outlined_button, tip};

/// The template open in the editor.
pub(super) struct TemplateEditor {
    /// Its ID, or 0 for one not saved yet.
    id: i64,
    name: Entity<TextInput>,
    subject: Entity<TextInput>,
    body: Entity<RichEditor>,
    attachments: Vec<TemplateFile>,
    /// Something changed since it was opened or saved.
    changed: bool,
    _subscriptions: Vec<Subscription>,
}

impl MailWindow {
    /// Opens template `id` in the editor, or a new one with `None`.
    fn edit_template(&mut self, id: Option<i64>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = id else {
            let template = katna_store::Template {
                name: tr!("settings-compose-template-new-name"),
                ..Default::default()
            };
            self.open_template_editor(template, window, cx);
            if let Some(e) = self.template_editor() {
                let name = e.name.clone();
                name.update(cx, |input, cx| input.select_all_text(cx));
                window.focus(&name.focus_handle(cx), cx);
            }
            return;
        };
        let paths = self.paths.clone();
        cx.spawn_in(window, async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move { data::template(&paths, id) })
                .await;
            this.update_in(cx, |this, window, cx| match found {
                Ok(Some(template)) => this.open_template_editor(template, window, cx),
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

    fn template_editor(&self) -> Option<&TemplateEditor> {
        self.settings_page.as_ref()?.template.as_ref()
    }

    fn open_template_editor(
        &mut self,
        template: katna_store::Template,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let th = self.theme(window);
        let accent: gpui::Hsla = rgba(th.accent).into();
        let input = |placeholder: String, text: String, cx: &mut Context<Self>| {
            cx.new(|cx| {
                let mut input = TextInput::new(placeholder, cx);
                input.set_text(text, cx);
                input.set_accent(accent);
                input
            })
        };
        let name = input(tr!("compose-tool-template-name"), template.name, cx);
        let subject = input(
            tr!("settings-compose-template-subject"),
            template.subject,
            cx,
        );
        let doc = if template.html.trim().is_empty() {
            html::from_plain(&template.text)
        } else {
            let mut next_image_id = 1;
            html::from_html(&template.html, &mut next_image_id)
        };
        let body = cx.new(|cx| {
            let mut editor = RichEditor::new(tr!("settings-compose-template-text"), cx);
            editor.set_palette(super::super::compose::palette(&th));
            editor.set_html_view(super::super::rich::html_view(th));
            editor.set_doc(doc.clone(), doc.start(), cx);
            editor
        });
        let changed = |this: &mut Self, cx: &mut Context<Self>| {
            if let Some(e) = this
                .settings_page
                .as_mut()
                .and_then(|p| p.template.as_mut())
            {
                e.changed = true;
            }
            cx.notify();
        };
        let subscriptions = vec![
            cx.subscribe_in(
                &name,
                window,
                move |this, _, event: &InputEvent, window, cx| match event {
                    InputEvent::Changed => changed(this, cx),
                    InputEvent::Submit => this.save_template_edit(window, cx),
                    InputEvent::Cancel => {}
                },
            ),
            cx.subscribe_in(
                &subject,
                window,
                move |this, _, event: &InputEvent, window, cx| match event {
                    InputEvent::Changed => changed(this, cx),
                    InputEvent::Submit => this.save_template_edit(window, cx),
                    InputEvent::Cancel => {}
                },
            ),
            cx.subscribe(&body, move |this, _, event: &RichEvent, cx| match event {
                RichEvent::Changed => changed(this, cx),
                RichEvent::Selection => cx.notify(),
                _ => {}
            }),
        ];
        if let Some(page) = &mut self.settings_page {
            page.template = Some(TemplateEditor {
                id: template.id,
                name,
                subject,
                body,
                attachments: template.attachments,
                changed: template.id == 0,
                _subscriptions: subscriptions,
            });
        }
        cx.notify();
    }

    fn remove_template_file(&mut self, ix: usize, cx: &mut Context<Self>) {
        if let Some(e) = self
            .settings_page
            .as_mut()
            .and_then(|p| p.template.as_mut())
            && ix < e.attachments.len()
        {
            e.attachments.remove(ix);
            e.changed = true;
            cx.notify();
        }
    }

    /// Saves the template being edited: a new one, or in place.
    fn save_template_edit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(e) = self.template_editor() else {
            return;
        };
        let name = e.name.read(cx).text().trim().to_owned();
        if name.is_empty() {
            self.show_snackbar(tr!("settings-compose-template-needs-name"), None, cx);
            return;
        }
        let doc = e.body.read(cx).doc().clone();
        let item = TemplateItem {
            id: e.id,
            name,
            subject: e.subject.read(cx).text().trim().to_owned(),
            html: if doc.has_formatting() {
                html::to_html(&doc, &html::data_uri)
            } else {
                String::new()
            },
            text: html::to_plain(&doc),
            attachments: e
                .attachments
                .iter()
                .map(|f| TemplateFileItem {
                    name: f.name.clone(),
                    mime: f.mime.clone(),
                    data: f.data.clone(),
                })
                .collect(),
        };
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::save_template(&connection, &item).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(id) => {
                        if let Some(e) = this
                            .settings_page
                            .as_mut()
                            .and_then(|p| p.template.as_mut())
                        {
                            e.id = id;
                            e.changed = false;
                        }
                        this.show_snackbar(tr!("settings-compose-template-saved"), None, cx);
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

    fn delete_template(&mut self, id: i64, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.settings_page {
            page.template = None;
        }
        cx.notify();
        if id == 0 {
            return;
        }
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::delete_template(&connection, id).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(()) => {
                        this.show_snackbar(tr!("settings-compose-template-deleted"), None, cx)
                    }
                    Err(err) => this.show_snackbar(
                        tr!("settings-compose-template-delete-failed", error = err),
                        None,
                        cx,
                    ),
                }
                this.load_templates(cx);
            })
            .ok();
        })
        .detach();
    }

    /// The Templates row: the list with New, and the editor beside it.
    pub(super) fn templates_row(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let editing = self.template_editor();
        let list =
            self.writing_templates()
                .iter()
                .map(|t| {
                    let on = editing.is_some_and(|e| e.id == t.id);
                    let id = t.id;
                    self.page_control(
                        crate::widgets::row(("page-template", id as usize), on, th),
                        th,
                        cx,
                    )
                    .px(px(katna_ui::tokens::space::S4))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.edit_template(Some(id), window, cx)
                    }))
                    .child(div().truncate().child(t.name.clone()))
                })
                .collect::<Vec<_>>();
        let empty = self.writing_templates().is_empty() && editing.is_none();
        let editor = editing.map(|e| self.render_template_editor(e, th, cx));
        self.row(
            tr!("settings-compose-templates"),
            Some(&tr!("settings-compose-templates-detail")),
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(16.0))
                .child(
                    label_column(200.0)
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .children(list)
                        .child(
                            outlined_button(
                                "page-template-new",
                                tr!("settings-compose-template-new"),
                                th,
                            )
                            .map(|d| self.page_control(d, th, cx))
                            .mt(px(8.0))
                            .justify_center()
                            .on_click(cx.listener(
                                |this, _, window, cx| this.edit_template(None, window, cx),
                            )),
                        ),
                )
                .children(editor)
                .when(empty, |d| {
                    d.child(self.quiet_note(tr!("settings-compose-no-templates"), th))
                }),
            th,
        )
    }

    fn render_template_editor(
        &self,
        e: &TemplateEditor,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let id = e.id;
        let body_focus = e.body.focus_handle(cx);
        let files = e.attachments.iter().enumerate().map(|(ix, f)| {
            div()
                .h(px(32.0))
                .pl(px(10.0))
                .pr(px(2.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(th.outline))
                .text_size(px(13.0))
                .child(icon("attachment", th.text_dim, 16.0))
                .child(div().max_w(px(200.0)).truncate().child(f.name.clone()))
                .child(
                    icon_button(("page-template-file-remove", ix), "close", 16.0, th)
                        .tooltip(tip(tr!("settings-compose-template-remove-file"), th))
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.remove_template_file(ix, cx)),
                        ),
                )
        });
        control_column(240.0)
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(line_field("page-template-name", &e.name, th, cx))
            .child(line_field("page-template-subject", &e.subject, th, cx))
            .child(
                field("page-template-text", &body_focus, th).child(
                    div()
                        .id("page-template-text-scroll")
                        .min_h(px(158.0))
                        .max_h(px(358.0))
                        .overflow_y_scroll()
                        .py(px(10.0))
                        .line_height(px(20.0))
                        .child(e.body.clone()),
                ),
            )
            .when(!e.attachments.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(6.0))
                        .children(files),
                )
            })
            .child(
                div()
                    .text_size(px(12.0))
                    .line_height(px(16.0))
                    .text_color(rgba(th.text_faint))
                    .child(tr!("settings-compose-template-fields")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .child(div().flex_1())
                    .child(
                        outlined_button(
                            "page-template-delete",
                            tr!("settings-compose-template-delete"),
                            th,
                        )
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(cx.listener(move |this, _, _, cx| this.delete_template(id, cx))),
                    )
                    .child(
                        filled_button(
                            "page-template-save",
                            tr!("settings-compose-template-save"),
                            th,
                        )
                        .map(|d| self.page_control(d, th, cx))
                        .when(!e.changed, |d| d.opacity(0.5))
                        .on_click(
                            cx.listener(|this, _, window, cx| this.save_template_edit(window, cx)),
                        ),
                    ),
            )
            .into_any_element()
    }
}
