// SPDX-License-Identifier: GPL-3.0-or-later

//! Rephrasing the subject: once the user typed one, a sparkle at the end
//! of the subject row offers three other wordings of it, written from the
//! subject and what the message says so far. Picking one puts it in, and
//! the snackbar's Undo puts back what was typed.

use crate::widgets::Tip as _;
use std::time::{Duration, Instant};

use gpui::{AnyElement, Context, Focusable, Task, Window, div, prelude::*, rgba};
use katna_ai::draft::{DraftKind, DraftRequest, Length, Manner};
use katna_ai::summary::Mail;
use katna_ai::wire::problem;
use katna_i18n::tr;
use katna_ui::px;

use super::super::super::MailWindow;
use super::super::tools::below;
use super::{Fix, problem_text};
use crate::daemon::{self, Command};
use crate::theme::{Theme, fade};
use crate::widgets::{icon, icon_button_colored, menu, menu_item, outlined_button};

/// How soon after a press outside closed the card a click on the
/// sparkle counts as that same press.
const JUST_CLOSED: Duration = Duration::from_millis(400);

/// The subject's other wordings, while their card is open.
pub(in crate::window) struct SubjectIdeas {
    /// The subject they were asked for.
    asked: String,
    ideas: Ideas,
    _task: Task<()>,
}

enum Ideas {
    Loading,
    Ready(Vec<String>),
    /// A [`problem`] name.
    Failed(String),
}

impl MailWindow {
    /// The subject field, with the sparkle once there is a subject to
    /// rephrase and the card of other wordings under it.
    pub(in crate::window) fn render_subject_field(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let typed = compose
            .subject
            .read(cx)
            .text()
            .chars()
            .any(char::is_alphabetic);
        let open = compose.subject_ideas.is_some();
        let sparkle = ((typed && self.ai_allowed()) || open).then(|| {
            icon_button_colored(
                "compose-subject-rephrase",
                "sparkle",
                18.0,
                if open { th.accent } else { th.text_dim },
                th,
            )
            .flex_none()
            .when(!open, |d| d.tip(tr!("compose-ai-subject-tip"), th))
            // The subject keeps its focus.
            .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(|this, _, _, cx| this.toggle_subject_ideas(cx)))
        });
        div()
            .relative()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(div().flex_1().min_w_0().child(compose.subject.clone()))
            .children(sparkle)
            .when_some(compose.subject_ideas.as_ref(), |d, ideas| {
                d.child(below(self.render_subject_ideas(ideas, th, cx)))
            })
            .into_any_element()
    }

    fn render_subject_ideas(
        &self,
        ideas: &SubjectIdeas,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let title = div()
            .px(px(16.0))
            .pb(px(4.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .text_size(px(12.0))
            .text_color(rgba(th.text_faint))
            .child(icon("sparkle", th.accent, 14.0))
            .child(tr!("compose-ai-subject-title"));
        let body = match &ideas.ideas {
            Ideas::Loading => div()
                .flex()
                .flex_col()
                .children([0.85, 0.65, 0.75].map(|width| {
                    div().h(px(32.0)).px(px(16.0)).flex().items_center().child(
                        div()
                            .h(px(10.0))
                            .w(gpui::relative(width))
                            .rounded_full()
                            .bg(rgba(fade(th.text_faint, 0.25))),
                    )
                }))
                .into_any_element(),
            Ideas::Ready(list) => div()
                .flex()
                .flex_col()
                .children(list.iter().enumerate().map(|(n, idea)| {
                    let idea = idea.clone();
                    menu_item(("compose-subject-idea", n), &idea, th).on_click(cx.listener(
                        move |this, _, window, cx| this.use_subject(idea.clone(), window, cx),
                    ))
                }))
                .into_any_element(),
            Ideas::Failed(problem) => {
                let (text, fix) = problem_text(problem, &self.ai_service_name());
                let label = match fix {
                    Fix::Retry => tr!("compose-ai-try-again"),
                    Fix::Settings(_) => tr!("compose-ai-open-settings"),
                };
                div()
                    .px(px(16.0))
                    .py(px(4.0))
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap(px(8.0))
                    .child(div().text_color(rgba(th.text_dim)).child(text))
                    .child(
                        outlined_button("compose-subject-fix", label, th)
                            .h(px(30.0))
                            .px(px(14.0))
                            .on_click(cx.listener(move |this, _, window, cx| match fix {
                                Fix::Retry => this.ask_subject_ideas(cx),
                                Fix::Settings(section) => {
                                    this.close_subject_ideas(cx);
                                    this.open_settings_page(section, window, cx);
                                }
                            })),
                    )
                    .into_any_element()
            }
        };
        menu(th)
            .id("compose-subject-ideas")
            .w(px(360.0))
            .text_size(px(14.0))
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                if let Some(c) = &mut this.compose
                    && c.subject_ideas.is_some()
                {
                    c.subject_ideas_closed = Some(Instant::now());
                }
                this.close_subject_ideas(cx);
            }))
            .child(title)
            .child(body)
            .into_any_element()
    }

    fn toggle_subject_ideas(&mut self, cx: &mut Context<Self>) {
        // This press already put them away.
        if let Some(c) = &mut self.compose
            && c.subject_ideas_closed
                .take()
                .is_some_and(|at| at.elapsed() < JUST_CLOSED)
        {
            return;
        }
        if self
            .compose
            .as_ref()
            .is_some_and(|c| c.subject_ideas.is_some())
        {
            self.close_subject_ideas(cx);
        } else {
            self.ask_subject_ideas(cx);
        }
    }

    pub(in crate::window) fn close_subject_ideas(&mut self, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose
            && c.subject_ideas.take().is_some()
        {
            cx.notify();
        }
    }

    /// Asks for other wordings of the subject as it is now.
    fn ask_subject_ideas(&mut self, cx: &mut Context<Self>) {
        if !self.ai_allowed() {
            return;
        }
        let connection = self.daemon.clone();
        let Some(c) = &mut self.compose else {
            return;
        };
        let subject = c.subject.read(cx).text().trim().to_owned();
        if !subject.chars().any(char::is_alphabetic) {
            return;
        }
        let text = c.body.read(cx).own_text();
        let mails = if text.is_empty() {
            Vec::new()
        } else {
            vec![Mail {
                from: "me".to_owned(),
                when: String::new(),
                text,
                new: false,
            }]
        };
        let request = DraftRequest {
            kind: DraftKind::Subject,
            subject: subject.clone(),
            mails,
            me: String::new(),
            to: String::new(),
            ideas: true,
            idea: String::new(),
            length: Length::Short,
            manner: Manner::Friendly,
        };
        let task = cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect()
                            .await
                            .map_err(|_| problem::FAILED.to_owned())?,
                    };
                    daemon::ai_draft(&connection, &request).await
                })
                .await;
            this.update(cx, |this, cx| {
                let Some(ideas) = this.compose.as_mut().and_then(|c| c.subject_ideas.as_mut())
                else {
                    return;
                };
                ideas.ideas = match result {
                    Ok(done) => match serde_json::from_str::<Vec<String>>(&done.text) {
                        Ok(list) if !list.is_empty() => Ideas::Ready(list),
                        _ => Ideas::Failed(problem::FAILED.to_owned()),
                    },
                    Err(problem) => Ideas::Failed(problem),
                };
                cx.notify();
            })
            .ok();
        });
        c.subject_ideas = Some(SubjectIdeas {
            asked: subject,
            ideas: Ideas::Loading,
            _task: task,
        });
        cx.notify();
    }

    /// Puts `idea` in as the subject; the snackbar's Undo puts back what
    /// was typed.
    fn use_subject(&mut self, idea: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let Some(ideas) = c.subject_ideas.take() else {
            return;
        };
        let input = c.subject.clone();
        input.update(cx, |input, cx| input.set_text(idea, cx));
        window.focus(&input.focus_handle(cx), cx);
        self.show_snackbar(
            tr!("compose-ai-subject-done"),
            Some(Command::RestoreSubject(ideas.asked)),
            cx,
        );
        cx.notify();
    }

    /// Undo on the snackbar: the subject as the user typed it.
    pub(in crate::window) fn restore_subject(
        &mut self,
        subject: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(c) = &self.compose else {
            return;
        };
        let input = c.subject.clone();
        input.update(cx, |input, cx| input.set_text(subject, cx));
        window.focus(&input.focus_handle(cx), cx);
    }
}
