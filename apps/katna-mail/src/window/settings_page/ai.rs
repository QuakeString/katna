// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Compose > Writing help with AI (`docs/ARCHITECTURE.md`
//! §16.5): where Rephrase and the longer suggestions go (Katna AI, the
//! user's own service, or nowhere), the own service's model, address and
//! key (kept by the daemon in the Secret Service), and encrypted mail.

use gpui::{
    AnyElement, Context, Div, Entity, Focusable, Subscription, Window, div, prelude::*, rgba,
};
use katna_ai::provider::{OTHER, PRESETS, preset};
use katna_core::config::{Ai, AiSource};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::{InputEvent, TextInput};

use super::{MailWindow, SAVE_DELAY, chip, control_column, field_box};
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::{icon, outlined_button};
use crate::window::settings::Change;

/// The fields of the user's own service.
pub(super) struct AiFields {
    model: Entity<TextInput>,
    address: Entity<TextInput>,
    key: Entity<TextInput>,
    /// Whether a key is saved, once the daemon has said.
    key_saved: Option<bool>,
    _subscriptions: [Subscription; 3],
}

impl AiFields {
    pub(super) fn new(ai: &Ai, accent: gpui::Hsla, cx: &mut Context<MailWindow>) -> Self {
        let input = |placeholder: String, text: &str, cx: &mut Context<MailWindow>| {
            let text = text.to_owned();
            cx.new(|cx| {
                let mut input = TextInput::new(placeholder, cx);
                input.set_text(text, cx);
                input.set_accent(accent);
                input
            })
        };
        let model = input(preset(&ai.provider).model.to_owned(), &ai.model, cx);
        let address = input("http://localhost:11434/v1".to_owned(), &ai.address, cx);
        let key = input(tr!("settings-ai-key-paste"), "", cx);
        key.update(cx, |input, cx| input.set_masked(true, cx));
        let subscriptions = [
            cx.subscribe(&model, |this, input, event: &InputEvent, cx| {
                if *event == InputEvent::Changed {
                    let text = input.read(cx).text().trim().to_owned();
                    this.set_ai_text(|ai| &mut ai.model, text, cx);
                }
            }),
            cx.subscribe(&address, |this, input, event: &InputEvent, cx| {
                if *event == InputEvent::Changed {
                    let text = input.read(cx).text().trim().to_owned();
                    this.set_ai_text(|ai| &mut ai.address, text, cx);
                }
            }),
            cx.subscribe(&key, |this, _, event: &InputEvent, cx| {
                if *event == InputEvent::Submit {
                    this.save_ai_key(false, cx);
                }
            }),
        ];
        Self {
            model,
            address,
            key,
            key_saved: None,
            _subscriptions: subscriptions,
        }
    }
}

impl MailWindow {
    /// Asks the daemon whether a key of the user's own service is saved.
    pub(super) fn load_ai_key_saved(&mut self, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let saved = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::ai_key_saved(&connection).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(page) = &mut this.settings_page {
                    page.ai.key_saved = saved.ok();
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    /// Saves the key typed (or, with `remove`, deletes the saved one).
    fn save_ai_key(&mut self, remove: bool, cx: &mut Context<Self>) {
        let Some(page) = &self.settings_page else {
            return;
        };
        let key = if remove {
            String::new()
        } else {
            page.ai.key.read(cx).text().trim().to_owned()
        };
        if key.is_empty() && !remove {
            return;
        }
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let done = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::set_ai_key(&connection, &key).await
                })
                .await;
            this.update(cx, |this, cx| {
                match done {
                    Ok(()) => {
                        if let Some(page) = &mut this.settings_page {
                            page.ai.key_saved = Some(!remove);
                            page.ai.key.update(cx, |input, cx| input.set_text("", cx));
                        }
                        let text = if remove {
                            tr!("settings-ai-key-removed")
                        } else {
                            tr!("settings-ai-key-saved-toast")
                        };
                        this.show_snackbar(text, None, cx);
                    }
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Keeps a typed model or address, saved once typing pauses.
    fn set_ai_text(
        &mut self,
        field: fn(&mut Ai) -> &mut String,
        text: String,
        cx: &mut Context<Self>,
    ) {
        let value = field(&mut self.config.ai);
        if *value == text {
            return;
        }
        *value = text;
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SAVE_DELAY).await;
            this.update(cx, |this, _| this.save_config()).ok();
        });
        if let Some(page) = &mut self.settings_page {
            page.save = Some(task);
        } else {
            task.detach();
        }
    }

    /// A change of [`Change::AiSource`] and the like.
    pub(in crate::window) fn apply_ai(&mut self, change: Change, cx: &mut Context<Self>) {
        let ai = &mut self.config.ai;
        match change {
            Change::AiSource(source) => ai.source = source,
            Change::AiProvider(id) => {
                if ai.provider == id {
                    return;
                }
                ai.provider = id.to_owned();
                // The model was the last service's.
                ai.model.clear();
                if let Some(page) = &self.settings_page {
                    page.ai.model.update(cx, |input, cx| {
                        input.set_placeholder(preset(id).model);
                        input.set_text("", cx);
                    });
                }
            }
            Change::AiAutocomplete(on) => ai.autocomplete = on,
            Change::AiAnswered(on) => ai.autocomplete_answered = on,
            Change::AiEncrypted(on) => ai.encrypted = on,
            _ => return,
        }
        self.save_config();
        self.suggestions_changed(cx);
        cx.notify();
    }

    /// The switches under Writing suggestions for AI's longer ones.
    pub(super) fn ai_suggestion_switches(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let ai = &self.config.ai;
        div()
            .flex()
            .flex_col()
            .child(self.switch_row(
                "page-ai-autocomplete",
                tr!("settings-ai-autocomplete"),
                tr!("settings-ai-autocomplete-detail"),
                ai.autocomplete,
                Change::AiAutocomplete(!ai.autocomplete),
                th,
                cx,
            ))
            .when(ai.autocomplete, |d| {
                d.child(div().pl(px(24.0)).child(self.switch_row(
                    "page-ai-answered",
                    tr!("settings-ai-answered"),
                    tr!("settings-ai-answered-detail"),
                    ai.autocomplete_answered,
                    Change::AiAnswered(!ai.autocomplete_answered),
                    th,
                    cx,
                )))
            })
    }

    /// The rows of Writing help with AI.
    pub(super) fn ai_rows(&self, th: &Theme, cx: &mut Context<Self>) -> Vec<Div> {
        let ai = &self.config.ai;
        let sources = div().flex().flex_row().flex_wrap().gap(px(6.0)).children(
            [
                (AiSource::Katna, tr!("settings-ai-katna")),
                (AiSource::Own, tr!("settings-ai-own")),
                (AiSource::Off, tr!("settings-ai-off")),
            ]
            .into_iter()
            .enumerate()
            .map(|(n, (source, label))| {
                chip(("page-ai-source", n), label, ai.source == source, th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.apply_ai(Change::AiSource(source), cx)
                    }))
            }),
        );
        let explain = match ai.source {
            AiSource::Katna => Some(tr!("settings-ai-katna-detail")),
            AiSource::Own => Some(tr!("settings-ai-own-detail")),
            AiSource::Off => None,
        };
        let mut rows = vec![
            self.row(
                tr!("settings-ai"),
                Some(&tr!("settings-ai-detail")),
                control_column(240.0)
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(sources)
                    .children(explain.map(|text| {
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(text)
                    })),
                th,
            ),
        ];
        if ai.source == AiSource::Own {
            rows.push(self.row(
                tr!("settings-ai-service"),
                Some(&tr!("settings-ai-service-detail")),
                self.ai_service_controls(th, cx),
                th,
            ));
        }
        if ai.source != AiSource::Off {
            rows.push(self.row(
                tr!("settings-ai-encrypted-title"),
                None,
                self.switch_row(
                    "page-ai-encrypted",
                    tr!("settings-ai-encrypted"),
                    tr!("settings-ai-encrypted-detail"),
                    ai.encrypted,
                    Change::AiEncrypted(!ai.encrypted),
                    th,
                    cx,
                ),
                th,
            ));
        }
        rows
    }

    /// The service list, then its address, model and key.
    fn ai_service_controls(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(page) = &self.settings_page else {
            return div().into_any_element();
        };
        let ai = &self.config.ai;
        let chosen = preset(&ai.provider);
        let services =
            div().flex().flex_row().flex_wrap().gap(px(6.0)).children(
                PRESETS.iter().enumerate().map(|(n, p)| {
                    let label = if p.id == OTHER {
                        tr!("settings-ai-other")
                    } else {
                        p.name.to_owned()
                    };
                    let id = p.id;
                    chip(("page-ai-provider", n), label, chosen.id == id, th)
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.apply_ai(Change::AiProvider(id), cx)
                        }))
                }),
            );
        let field = |id: &'static str, label: String, input: &Entity<TextInput>| {
            let focus = input.focus_handle(cx);
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
                .child(
                    field_box(id, th)
                        .h(px(40.0))
                        .flex()
                        .items_center()
                        .on_click(move |_, window: &mut Window, cx| window.focus(&focus, cx))
                        .child(div().flex_1().child(input.clone())),
                )
        };
        let key_state = match page.ai.key_saved {
            Some(true) => Some(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(icon("check", th.text_dim, 16.0))
                    .child(tr!("settings-ai-key-saved"))
                    .child(
                        div()
                            .id("page-ai-key-remove")
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .hover(|s| s.underline())
                            .on_click(cx.listener(|this, _, _, cx| this.save_ai_key(true, cx)))
                            .child(tr!("settings-ai-key-remove")),
                    ),
            ),
            Some(false) if chosen.needs_key => Some(
                div()
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(tr!("settings-ai-key-none")),
            ),
            _ => None,
        };
        control_column(240.0)
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(services)
            .when(chosen.id == OTHER, |d| {
                d.child(field(
                    "page-ai-address",
                    tr!("settings-ai-address"),
                    &page.ai.address,
                ))
            })
            .child(field(
                "page-ai-model",
                tr!("settings-ai-model"),
                &page.ai.model,
            ))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_end()
                    .gap(px(8.0))
                    .child(div().flex_1().child(field(
                        "page-ai-key",
                        tr!("settings-ai-key"),
                        &page.ai.key,
                    )))
                    .child(
                        outlined_button("page-ai-key-save", tr!("settings-ai-key-save"), th)
                            .map(|d| self.page_control(d, th, cx))
                            .h(px(40.0))
                            .on_click(cx.listener(|this, _, _, cx| this.save_ai_key(false, cx))),
                    ),
            )
            .children(key_state)
            .into_any_element()
    }
}
