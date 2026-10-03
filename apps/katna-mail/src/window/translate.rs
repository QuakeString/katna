// SPDX-License-Identifier: GPL-3.0-or-later

//! Automatic translation in the reading pane (`docs/ARCHITECTURE.md`
//! §16.4).
//!
//! A message whose language (found on this computer) is not the reading
//! language gets a bar above its text: "Translate to English". Pressing
//! it asks the daemon, which sends only the message's text to Katna's
//! server and keeps the answer; the translation then shows in place of
//! the message, with "Show original" to go back. Languages the user chose
//! to always translate are translated as soon as the message opens.
//! Encrypted mail is never offered: its text stays on this computer.

use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use gpui::{AnyElement, Context, SharedString, div, prelude::*, rgba};
use katna_dbus::translate_problem as problem;
use katna_i18n::tr;
use katna_store::MessageId;
use katna_ui::px;

use super::MailWindow;
use super::settings::Change;
use super::settings_page::Section;
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::icon;

/// Where a message's translation is.
enum State {
    /// Asked for; the daemon is working on it.
    Working,
    /// Shown in place of the message.
    Shown(Translated),
    /// Made, but the original is shown.
    Original(Translated),
    /// It failed: a [`katna_dbus::translate_problem`].
    Failed(String),
}

struct Translated {
    /// The language it was in.
    source: String,
    /// The translation in blocks of quoted and unquoted lines, as the
    /// reading pane shows a plain text message.
    blocks: Vec<(bool, SharedString)>,
    /// The translation as one text, for the chat's bubbles.
    text: String,
}

/// Translations of the messages opened this session.
#[derive(Default)]
pub(crate) struct Translations {
    states: RefCell<HashMap<MessageId, State>>,
    /// The language found in each message; `None` when unclear.
    detected: RefCell<HashMap<MessageId, Option<&'static str>>>,
    /// The languages the server translates into each reading language,
    /// and when it answered; `None` while asking.
    sources: RefCell<HashMap<String, Option<Answered>>>,
}

/// What the server said it translates, and when.
struct Answered {
    at: Instant,
    /// Its languages; empty when it could not be asked.
    sources: Vec<String>,
    /// It could not be asked because this computer is not signed in to a
    /// Katna account.
    sign_in: bool,
}

/// Whether to offer translating a message.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Offer {
    No,
    Yes,
    /// Yes, once signed in to a Katna account.
    SignIn,
}

/// How long to wait before asking a server that offered nothing (it was
/// unreachable, or translation is off) again.
const ASK_AGAIN: Duration = Duration::from_secs(10 * 60);

impl Translations {
    /// Forgets what the server offers, so it is asked again (after the
    /// reading language changed).
    pub(super) fn forget_sources(&self) {
        self.sources.borrow_mut().clear();
    }
}

/// The name of language `code` in that language, else its code.
fn language_name(code: &str) -> String {
    katna_translate::native_name(code)
        .map(str::to_owned)
        .unwrap_or_else(|| code.to_owned())
}

impl MailWindow {
    /// The language mail is translated into, as a LibreTranslate code.
    pub(super) fn reading_code(&self) -> String {
        let chosen = &self.config.mail.translation.reading_language;
        if chosen.is_empty() {
            katna_translate::code_for_tag(&katna_i18n::current().language.tag)
        } else {
            katna_translate::code_for_tag(chosen)
        }
    }

    /// The name of the reading language, as Settings shows it.
    pub(super) fn reading_language_name(&self) -> String {
        let chosen = &self.config.mail.translation.reading_language;
        katna_i18n::find(chosen)
            .map(|language| language.name.clone())
            .unwrap_or_else(|| language_name(&self.reading_code()))
    }

    /// The translation of message `id` to show in its place, if the user
    /// chose to see it.
    pub(super) fn translated_blocks(&self, id: MessageId) -> Option<Vec<(bool, SharedString)>> {
        match self.translations.states.borrow().get(&id) {
            Some(State::Shown(translated)) => Some(translated.blocks.clone()),
            _ => None,
        }
    }

    /// The translation of message `id` as one text, if the user chose to
    /// see it.
    pub(super) fn translated_text(&self, id: MessageId) -> Option<String> {
        match self.translations.states.borrow().get(&id) {
            Some(State::Shown(translated)) => Some(translated.text.clone()),
            _ => None,
        }
    }

    /// What a right-click menu's Translate row says for message `id`:
    /// translate it, show the original or the translation again, or that
    /// it is being translated. `None` when it cannot be translated.
    pub(super) fn translate_menu_label(&self, id: MessageId) -> Option<String> {
        self.plain_text_of(id)?;
        Some(match self.translations.states.borrow().get(&id) {
            Some(State::Working) => tr!("translate-working"),
            Some(State::Shown(_)) => tr!("translate-show-original"),
            Some(State::Original(_)) => tr!("translate-show-translation"),
            None | Some(State::Failed(_)) => {
                tr!(
                    "translate-to",
                    language = language_name(&self.reading_code())
                )
            }
        })
    }

    /// The right-click menu's Translate row for message `id`: translates
    /// it, even when no bar offered it, or switches between the
    /// translation and the original.
    pub(super) fn translate_from_menu(&mut self, id: MessageId, cx: &mut Context<Self>) {
        let Some(text) = self.plain_text_of(id) else {
            return;
        };
        let state = self.translations.states.borrow_mut().remove(&id);
        match state {
            Some(state @ (State::Shown(_) | State::Original(_) | State::Working)) => {
                self.translations.states.borrow_mut().insert(id, state);
                self.flip_translation(id);
            }
            None | Some(State::Failed(_)) => {
                let source = *self
                    .translations
                    .detected
                    .borrow_mut()
                    .entry(id)
                    .or_insert_with(|| katna_translate::detect(&text));
                let target = self.reading_code();
                self.start_translation(id, &text, source.unwrap_or("auto"), &target, cx);
            }
        }
        cx.notify();
    }

    /// The bar above message `id` (the `ix`th of the conversation) whose
    /// plain text is `text`, when it is in another language that can be
    /// translated. Starts a translation the user chose to always have.
    /// `compact` draws it as a line inside a chat bubble.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn translation_bar(
        &self,
        ix: usize,
        id: MessageId,
        text: &str,
        encrypted: bool,
        compact: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let settings = &self.config.mail.translation;
        if encrypted {
            return None;
        }
        let source = *self
            .translations
            .detected
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| katna_translate::detect(text));
        let target = self.reading_code();
        let signed_in = self.katna_signed_in();
        let mut has_state = false;
        self.translations
            .states
            .borrow_mut()
            .retain(|message, state| {
                // Signed in since: offer it again.
                let stale =
                    signed_in && matches!(state, State::Failed(why) if why == problem::SIGN_IN);
                has_state |= *message == id && !stale;
                !stale
            });
        if !has_state {
            // Not offered, or nothing to translate.
            let source = source?;
            if !settings.offer
                || katna_translate::same_language(source, &target)
                || settings.never.iter().any(|l| l == source)
            {
                return None;
            }
            match self.server_translates(source, &target, cx) {
                Offer::No => return None,
                Offer::SignIn => {
                    self.translations
                        .states
                        .borrow_mut()
                        .insert(id, State::Failed(problem::SIGN_IN.to_owned()));
                }
                Offer::Yes => {
                    if settings.always.iter().any(|l| l == source) {
                        self.start_translation(id, text, source, &target, cx);
                    }
                }
            }
        }
        let source_name = source.map(language_name).unwrap_or_default();
        let states = self.translations.states.borrow();
        let text_owned = text.to_owned();
        let link = |label: String, key: &'static str| {
            div()
                .id((key, ix))
                .px(px(8.0))
                .py(px(2.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .text_color(rgba(th.accent))
                .hover(|s| s.bg(rgba(th.hover)))
                .child(label)
        };
        let (note, links): (String, Vec<AnyElement>) = match states.get(&id) {
            None | Some(State::Failed(_)) => {
                let failed = match states.get(&id) {
                    Some(State::Failed(why)) => Some(why.clone()),
                    _ => None,
                };
                let note = match failed.as_deref() {
                    None => tr!("translate-offer", language = source_name.clone()),
                    Some(problem::TOO_MANY) => tr!("translate-too-many"),
                    Some(problem::SIGN_IN) => {
                        tr!("translate-sign-in", language = source_name.clone())
                    }
                    Some(problem::UNSUPPORTED) => {
                        tr!("translate-unsupported", language = source_name.clone())
                    }
                    Some(_) => tr!("translate-failed"),
                };
                let target_name = language_name(&target);
                let text = text_owned.clone();
                let target_code = target.clone();
                let source_code = source.unwrap_or("auto");
                let signed_out = failed.as_deref() == Some(problem::SIGN_IN);
                let mut links = vec![if signed_out {
                    link(tr!("katna-sign-in"), "translate-sign-in")
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.open_settings_page(Section::Subscriptions, window, cx);
                        }))
                        .into_any_element()
                } else {
                    link(
                        if failed.is_some() {
                            tr!("translate-retry")
                        } else {
                            tr!("translate-to", language = target_name)
                        },
                        "translate",
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.translations.states.borrow_mut().remove(&id);
                        this.start_translation(id, &text, source_code, &target_code, cx);
                        cx.notify();
                    }))
                    .into_any_element()
                }];
                if let (None, Some(source)) = (&failed, source) {
                    links.push(
                        link(
                            tr!("translate-never", language = source_name.clone()),
                            "translate-never",
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.apply(Change::TranslateNever(source, true), cx);
                        }))
                        .into_any_element(),
                    );
                }
                (note, links)
            }
            Some(State::Working) => (tr!("translate-working"), Vec::new()),
            Some(State::Shown(translated)) => {
                let from = language_name(&translated.source);
                let always = settings.always.contains(&translated.source);
                let mut links = vec![
                    link(tr!("translate-show-original"), "translate-original")
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.flip_translation(id);
                            cx.notify();
                        }))
                        .into_any_element(),
                ];
                if let Some(source) =
                    katna_translate::known_code(&translated.source).filter(|_| !always)
                {
                    links.push(
                        link(
                            tr!("translate-always", language = from.clone()),
                            "translate-always",
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.apply(Change::TranslateAlways(source, true), cx);
                        }))
                        .into_any_element(),
                    );
                }
                let note = if translated.source == "auto" {
                    tr!("translate-done-unknown")
                } else {
                    tr!("translate-done", language = from)
                };
                (note, links)
            }
            Some(State::Original(_)) => (
                tr!("translate-offer", language = source_name.clone()),
                vec![
                    link(tr!("translate-show-translation"), "translate-again")
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.flip_translation(id);
                            cx.notify();
                        }))
                        .into_any_element(),
                ],
            ),
        };
        Some(
            div()
                .map(|d| {
                    if compact {
                        // A line at the top of the bubble.
                        d.mb(px(4.0)).ml(px(-2.0))
                    } else {
                        d.mb(px(12.0))
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(8.0))
                            .bg(rgba(th.read_row))
                    }
                })
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(4.0))
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .child(icon("translate", th.text_faint, 16.0))
                .child(div().pl(px(4.0)).pr(px(4.0)).child(note))
                .children(links)
                .into_any_element(),
        )
    }

    /// Whether the server translates `source` into `target`. Asks it the
    /// first time; until it answers, nothing is offered.
    fn server_translates(&self, source: &str, target: &str, cx: &mut Context<Self>) -> Offer {
        match self.translations.sources.borrow().get(target) {
            // Still asking.
            Some(None) => return Offer::No,
            Some(Some(answered)) if answered.sign_in && !self.katna_signed_in() => {
                return if katna_translate::known_code(source).is_some() {
                    Offer::SignIn
                } else {
                    Offer::No
                };
            }
            Some(Some(answered))
                if !answered.sign_in
                    && (!answered.sources.is_empty() || answered.at.elapsed() < ASK_AGAIN) =>
            {
                return if answered.sources.iter().any(|s| s == source) {
                    Offer::Yes
                } else {
                    Offer::No
                };
            }
            _ => {}
        }
        self.translations
            .sources
            .borrow_mut()
            .insert(target.to_owned(), None);
        let connection = self.daemon.clone();
        let target = target.to_owned();
        cx.spawn(async move |this, cx| {
            let asked = target.clone();
            let answer = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::translation_sources(&connection, &asked).await
                })
                .await;
            let (sources, sign_in) = match answer {
                Ok(sources) => (sources, false),
                Err(why) => {
                    tracing::info!(%why, "asking which languages can be translated");
                    (Vec::new(), why == problem::SIGN_IN)
                }
            };
            this.update(cx, |this, cx| {
                this.translations.sources.borrow_mut().insert(
                    target,
                    Some(Answered {
                        at: Instant::now(),
                        sources,
                        sign_in,
                    }),
                );
                cx.notify();
            })
            .ok();
        })
        .detach();
        Offer::No
    }

    /// Asks the daemon to translate message `id`, whose text `text` is in
    /// `source`.
    fn start_translation(
        &self,
        id: MessageId,
        text: &str,
        source: &str,
        target: &str,
        cx: &mut Context<Self>,
    ) {
        {
            let mut states = self.translations.states.borrow_mut();
            if matches!(states.get(&id), Some(State::Working | State::Shown(_))) {
                return;
            }
            states.insert(id, State::Working);
        }
        let connection = self.daemon.clone();
        let text = text.to_owned();
        let source = source.to_owned();
        let target = target.to_owned();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect()
                            .await
                            .map_err(|_| problem::FAILED.to_owned())?,
                    };
                    daemon::translate(&connection, id.0, &text, &source, &target).await
                })
                .await;
            this.update(cx, |this, cx| {
                let state = match result {
                    Ok((source, text)) => {
                        let (blocks, _) = super::reader::body_blocks(&text, usize::MAX);
                        State::Shown(Translated {
                            source,
                            blocks,
                            text,
                        })
                    }
                    Err(why) => State::Failed(why),
                };
                this.translations.states.borrow_mut().insert(id, state);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Switches message `id` between its translation and its original.
    fn flip_translation(&self, id: MessageId) {
        let mut states = self.translations.states.borrow_mut();
        let flipped = match states.remove(&id) {
            Some(State::Shown(translated)) => State::Original(translated),
            Some(State::Original(translated)) => State::Shown(translated),
            Some(other) => other,
            None => return,
        };
        states.insert(id, flipped);
    }

    /// Settings > General > Translation: offer translations, the reading
    /// language, and the languages always translated or never offered.
    pub(super) fn translation_settings(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let settings = &self.config.mail.translation;
        let offer = self.switch_row(
            "page-translate-offer",
            tr!("settings-translation-offer"),
            tr!("settings-translation-offer-detail"),
            settings.offer,
            Change::TranslateOffer(!settings.offer),
            th,
            cx,
        );
        let reading = div()
            .id("page-reading-language")
            .h(px(40.0))
            .max_w(px(320.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(th.outline))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(|this, event: &gpui::ClickEvent, window, cx| {
                this.toggle_reading_language_picker(event.position(), window, cx);
            }))
            .child(icon("translate", th.text_dim, 18.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(14.0))
                    .child(if settings.reading_language.is_empty() {
                        tr!(
                            "translate-reading-same-now",
                            language = self.reading_language_name()
                        )
                    } else {
                        self.reading_language_name()
                    }),
            )
            .child(icon("chevron-down", th.text_dim, 18.0));
        let languages = |label: String, codes: &[String], always: bool| {
            let chips: Vec<AnyElement> = codes
                .iter()
                .filter_map(|code| katna_translate::known_code(code))
                .map(|code| {
                    let change = if always {
                        Change::TranslateAlways(code, false)
                    } else {
                        Change::TranslateNever(code, false)
                    };
                    div()
                        .id((
                            if always {
                                "translate-always-chip"
                            } else {
                                "translate-never-chip"
                            },
                            code.as_ptr() as usize,
                        ))
                        .h(px(28.0))
                        .pl(px(10.0))
                        .pr(px(4.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.0))
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(rgba(th.outline))
                        .text_size(px(13.0))
                        .child(language_name(code))
                        .child(
                            div()
                                .id((
                                    if always {
                                        "translate-always-remove"
                                    } else {
                                        "translate-never-remove"
                                    },
                                    code.as_ptr() as usize,
                                ))
                                .p(px(2.0))
                                .rounded(px(6.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.apply(change, cx);
                                }))
                                .child(icon("close", th.text_dim, 14.0)),
                        )
                        .into_any_element()
                })
                .collect();
            let empty = chips.is_empty();
            div()
                .px(px(8.0))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(div().text_size(px(14.0)).child(label))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(6.0))
                        .children(chips)
                        .when(empty, |d| {
                            d.child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgba(th.text_faint))
                                    .child(self.copyable(tr!("settings-translation-none"), th)),
                            )
                        }),
                )
        };
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(offer)
            .when(settings.offer, |d| {
                d.child(
                    div()
                        .px(px(8.0))
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .child(tr!("settings-translation-reading")),
                        )
                        .child(reading),
                )
                .child(languages(
                    tr!("settings-translation-always"),
                    &settings.always,
                    true,
                ))
                .child(languages(
                    tr!("settings-translation-never"),
                    &settings.never,
                    false,
                ))
            })
            .into_any_element()
    }
}
