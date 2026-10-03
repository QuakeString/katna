// SPDX-License-Identifier: GPL-3.0-or-later

//! The AI sparkle on an open note, as in Compose: Tidy the text, Turn it
//! into a checklist, or Summarise it, through the same service (the
//! user's own key, Ollama or Katna AI). The change is one step the
//! editor's Undo takes back. Hidden when AI is off.

use gpui::{AnyElement, Context, Focusable as _, Pixels, Point, Window, div, prelude::*, rgba};
use katna_core::config::AiSource;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::{Block, Doc, Para, ParaStyle};
use katna_ui::tokens::{elevation, radius, space, text};

use super::{MailWindow, UNTICKED, format};
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::{menu_item_icon, raised};

/// What the sparkle does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Help {
    Tidy,
    Checklist,
    Summarise,
}

impl Help {
    const ALL: [Help; 3] = [Help::Tidy, Help::Checklist, Help::Summarise];

    fn label(self) -> String {
        tr!(match self {
            Help::Tidy => "notes-ai-tidy",
            Help::Checklist => "notes-ai-checklist",
            Help::Summarise => "notes-ai-summarise",
        })
    }

    fn icon(self) -> &'static str {
        match self {
            Help::Tidy => "pen-sparkle",
            Help::Checklist => "checkbox-checked",
            Help::Summarise => "list-bulleted",
        }
    }

    /// What the service is asked (it never shows).
    fn instruction(self) -> &'static str {
        match self {
            Help::Tidy => {
                "Tidy this note: fix spelling, grammar and punctuation and make it read \
                 smoothly, keeping its meaning, language, facts and line breaks. \
                 Write plain text without Markdown."
            }
            Help::Checklist => {
                "Turn this note into a short checklist: one item per line, each a few words, \
                 in the note's language, with no bullets, numbers or Markdown."
            }
            Help::Summarise => {
                "Summarise this note in two or three short sentences in its language, \
                 as plain text without Markdown."
            }
        }
    }
}

/// The sparkle's menu, and a change on its way.
#[derive(Debug, Clone, Default)]
pub(super) struct AiState {
    /// The menu is open at this point.
    pub menu: Option<Point<Pixels>>,
    /// Waiting for the service.
    pub busy: Option<Help>,
}

/// Lines `text` as plain paragraphs.
fn plain_doc(text: &str) -> Vec<Block> {
    text.trim()
        .lines()
        .map(|line| Block::Para(Para::plain(line.trim_end())))
        .collect()
}

/// `text` as checklist items.
fn checklist(text: &str) -> Vec<Block> {
    text.lines()
        .map(|line| {
            line.trim()
                .trim_start_matches(['-', '*', '•', '☐', '☑'])
                .trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ')')
                .trim()
        })
        .filter(|line| !line.is_empty())
        .map(|line| Block::Para(Para::plain(format!("{UNTICKED}{line}"))))
        .collect()
}

impl MailWindow {
    /// Whether the sparkle shows: AI is on.
    pub(super) fn notes_ai_on(&self) -> bool {
        self.config.ai.source != AiSource::Off
    }

    /// Opens the sparkle's menu at `at`, or closes it.
    pub(super) fn toggle_note_ai(&mut self, at: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        editor.ai.menu = if editor.ai.menu.is_some() {
            None
        } else {
            Some(at)
        };
        cx.notify();
    }

    /// Asks the service for `help` on the open note's text.
    fn note_ai(&mut self, help: Help, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        editor.ai.menu = None;
        let text = format::text_of(editor.body.read(cx).doc());
        if text.trim().is_empty() {
            self.show_snackbar(tr!("notes-ai-empty"), None, cx);
            return;
        }
        editor.ai.busy = Some(help);
        let id = editor.id;
        let opening = editor.opening;
        let body = editor.body.clone();
        let connection = self.daemon.clone();
        let service = self.ai_service_name();
        cx.spawn_in(window, async move |this, cx| {
            let answer = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::ai_rephrase(
                        &connection,
                        &text,
                        katna_ai::prompt::Tone::Custom.id(),
                        help.instruction(),
                    )
                    .await
                    .map(|r| r.text)
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                let Some(editor) = this
                    .notes
                    .as_mut()
                    .and_then(|p| p.editor.as_mut())
                    .filter(|e| e.id == id && e.opening == opening)
                else {
                    return;
                };
                editor.ai.busy = None;
                let answer = match answer {
                    Ok(answer) if !answer.trim().is_empty() => answer,
                    Ok(_) => return,
                    Err(problem) => {
                        let (text, _) =
                            super::super::compose::rephrase::problem_text(&problem, &service);
                        this.show_snackbar(text, None, cx);
                        return;
                    }
                };
                body.update(cx, |area, cx| {
                    area.edit_doc(
                        |doc: &mut Doc| match help {
                            Help::Tidy => doc.blocks = plain_doc(&answer),
                            Help::Checklist => doc.blocks = checklist(&answer),
                            // The summary goes on top, quoted, over the text.
                            Help::Summarise => {
                                let mut top: Vec<Block> = plain_doc(&answer)
                                    .into_iter()
                                    .map(|b| match b {
                                        Block::Para(p) => Block::Para(p.with_style(ParaStyle {
                                            quote: 1,
                                            ..ParaStyle::default()
                                        })),
                                        other => other,
                                    })
                                    .collect();
                                top.push(Block::Para(Para::plain("")));
                                doc.blocks.splice(0..0, top);
                            }
                        },
                        cx,
                    )
                });
                window.focus(&body.focus_handle(cx), cx);
                let done = match help {
                    Help::Tidy => tr!("notes-ai-tidied"),
                    Help::Checklist => tr!("notes-ai-listed"),
                    Help::Summarise => tr!("notes-ai-summarised"),
                };
                this.show_snackbar(done, None, cx);
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    /// The sparkle's menu over the open note.
    pub(super) fn render_note_ai_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        let at = editor.ai.menu?;
        let menu = raised(
            div()
                .id("note-ai-menu")
                .occlude()
                .key_context(crate::widgets::MENU_CONTEXT)
                .min_w(px(220.0))
                .py(px(space::S3))
                .flex()
                .flex_col()
                .text_size(px(text::BODY))
                .text_color(rgba(th.text)),
            th,
            radius::SM,
            elevation::MENU,
        )
        .children(Help::ALL.into_iter().map(|help| {
            menu_item_icon(("note-ai", help as usize), help.icon(), &help.label(), th)
                .on_click(cx.listener(move |this, _, window, cx| this.note_ai(help, window, cx)))
        }));
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    gpui::deferred(
                        div()
                            .id("note-ai-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    if let Some(editor) =
                                        this.notes.as_mut().and_then(|p| p.editor.as_mut())
                                    {
                                        editor.ai.menu = None;
                                    }
                                    cx.notify();
                                }),
                            ),
                    )
                    .with_priority(3),
                )
                .child(
                    gpui::deferred(
                        gpui::anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(space::S3))
                            .child(menu),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_answer_becomes_checklist_items() {
        let blocks = checklist("- Sunscreen\n2. Chargers\n\n• Tickets");
        let texts: Vec<String> = blocks
            .iter()
            .filter_map(|b| match b {
                Block::Para(p) => Some(p.text.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(
            texts,
            [
                format!("{UNTICKED}Sunscreen"),
                format!("{UNTICKED}Chargers"),
                format!("{UNTICKED}Tickets")
            ]
        );
    }
}
