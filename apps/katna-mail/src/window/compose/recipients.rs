// SPDX-License-Identifier: GPL-3.0-or-later

//! Address suggestions under To, Cc and Bcc, as in Gmail: from the first
//! letter typed, the people written with most and most lately first, typos
//! forgiven ([`katna_search::contacts`]). Up and Down move, Enter or Tab
//! picks, Escape closes. A picked address becomes a chip (`chips`).
//!
//! The address book is shared by every window and read in the background:
//! the saved copy first, then the store again when it is more than a few
//! minutes old.

use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, Div, DragMoveEvent, FontWeight, Global, HighlightStyle, KeyDownEvent,
    SharedString, StyledText, anchored, deferred, div, point, prelude::*, rgba,
};
use katna_core::config::AppKind;
use katna_search::contacts::{ContactBook, Suggestion};
use katna_ui::px;
use katna_ui::text_input::{Backspace, Cancel, Delete, Down, Left, Right, Submit, Up};

use super::super::FocusNext;
use super::MailWindow;
use super::chips::ChipDrag;
use crate::data;
use crate::outgoing;
use crate::theme::Theme;
use crate::widgets::raised;

/// Suggestions shown at once.
const SHOWN: usize = 8;
/// How long the address book is used before it is read again.
const FRESH: Duration = Duration::from_secs(10 * 60);
const ROW: f32 = 56.0;
const AVATAR: f32 = 32.0;

/// A recipient field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Field {
    To,
    Cc,
    Bcc,
}

impl Field {
    pub fn ix(self) -> usize {
        self as usize
    }
}

/// The suggestions open under a field.
pub(in crate::window) struct Suggestions {
    field: Field,
    items: Vec<Suggestion>,
    selected: usize,
}

/// The address book, shared by every window.
#[derive(Default)]
struct Book {
    book: Option<ContactBook>,
    loading: bool,
    read: Option<Instant>,
    /// Whether the book has the saved contacts (Contacts was on).
    saved: bool,
}

impl Global for Book {}

impl MailWindow {
    /// Reads the address book if it is missing or old: the saved copy
    /// first, so suggestions work at once, then the store. With Contacts
    /// off, only the people mailed are suggested.
    pub(in crate::window) fn load_address_book(&mut self, cx: &mut Context<Self>) {
        let with_saved = self.config.app_on(AppKind::Contacts);
        let book = cx.default_global::<Book>();
        let fresh = book.read.is_some_and(|read| read.elapsed() < FRESH);
        if book.loading || (fresh && book.saved == with_saved) {
            return;
        }
        book.loading = true;
        let cached = book.book.is_none() || book.saved != with_saved;
        let paths = self.paths.clone();
        cx.spawn(async move |_, cx| {
            if cached {
                let paths = paths.clone();
                let cached = cx
                    .background_executor()
                    .spawn(async move { data::cached_address_book(&paths, with_saved) })
                    .await;
                cx.update(|cx| {
                    let book = cx.default_global::<Book>();
                    if book.book.is_none() || book.saved != with_saved {
                        // Not the other kind's book, even with no copy.
                        book.book = cached;
                        book.saved = with_saved;
                    }
                });
            }
            let started = Instant::now();
            let fresh = cx
                .background_executor()
                .spawn(async move {
                    let book = data::address_book(&paths, with_saved)?;
                    data::save_address_book(&paths, &book, with_saved);
                    Ok::<_, String>(book)
                })
                .await;
            cx.update(|cx| {
                let book = cx.default_global::<Book>();
                book.loading = false;
                book.read = Some(Instant::now());
                match fresh {
                    Ok(fresh) => {
                        book.saved = with_saved;
                        tracing::debug!(
                            "address book: {} addresses in {:?}",
                            fresh.len(),
                            started.elapsed()
                        );
                        book.book = Some(fresh);
                    }
                    Err(err) => tracing::warn!("{err}"),
                }
            });
        })
        .detach();
    }

    /// Counts sent mail in the address book at once, so its recipients
    /// come up first next time.
    pub(super) fn note_recipients(
        &mut self,
        account: i64,
        recipients: &[outgoing::Mailbox],
        cx: &mut Context<Self>,
    ) {
        let now = jiff::Timestamp::now().as_second();
        let recipients: Vec<_> = recipients
            .iter()
            .map(|m| (m.email.clone(), m.name.clone()))
            .collect();
        if let Some(book) = cx.default_global::<Book>().book.as_mut() {
            book.note_sent(account, &recipients, now);
        }
    }

    /// Makes chips of the addresses a comma or semicolon ended, and
    /// suggests addresses for the one being typed in `field`.
    pub(super) fn recipient_changed(&mut self, field: Field, cx: &mut Context<Self>) {
        let Some(input) = self.recipient_input(field) else {
            return;
        };
        // A space after a chip's comma starts nothing, so the placeholder
        // can come back once the chips are gone.
        let text = input.read(cx).text();
        if text.starts_with(char::is_whitespace) {
            let trimmed = text.trim_start().to_owned();
            input
                .clone()
                .update(cx, |input, cx| input.set_text(trimmed, cx));
            return;
        }
        if last_entry(input.read(cx).text()).0 > 0 {
            // Changing the text again brings this back for the rest.
            self.commit_recipients(field, false, cx);
            return;
        }
        let typed = input.read(cx).text().to_owned();
        // The account the message goes out from.
        let account = self.compose.as_ref().and_then(|compose| {
            compose
                .from
                .or_else(|| self.compose_account(compose.kind).map(|a| a.id))
                .map(|id| id.0)
        });
        // Everyone already added to To, Cc or Bcc.
        let skip = self
            .compose
            .as_ref()
            .map(|c| c.chips.emails())
            .unwrap_or_default();
        let items = address_suggestions(&typed, account, &skip, cx);
        if let Some(compose) = &mut self.compose {
            compose.suggest = (!items.is_empty()).then_some(Suggestions {
                field,
                items,
                selected: 0,
            });
        }
        cx.notify();
    }

    pub(super) fn close_suggestions(&mut self, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose
            && compose.suggest.take().is_some()
        {
            cx.notify();
        }
    }

    /// Puts suggestion `ix` in place of what is being typed.
    fn pick_suggestion(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let Some(suggest) = compose.suggest.take() else {
            return;
        };
        let Some(item) = suggest.items.into_iter().nth(ix) else {
            return;
        };
        self.add_recipient(suggest.field, item.name, item.email, cx);
    }

    fn move_suggestion(&mut self, by: isize, cx: &mut Context<Self>) {
        if let Some(suggest) = self.compose.as_mut().and_then(|c| c.suggest.as_mut()) {
            let len = suggest.items.len() as isize;
            suggest.selected = (suggest.selected as isize + by).rem_euclid(len) as usize;
            cx.notify();
        }
    }

    pub(super) fn suggesting(&self, field: Field) -> bool {
        self.compose
            .as_ref()
            .and_then(|c| c.suggest.as_ref())
            .is_some_and(|s| s.field == field)
    }

    /// Gives a recipient row its keys for the suggestions and the chips,
    /// and draws the suggestions under it.
    pub(super) fn recipient_row(
        &self,
        row: Div,
        field: Field,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        // Taken before the field sees them, while suggestions are open.
        let take = move |this: &mut Self, cx: &mut Context<Self>| {
            if this.suggesting(field) {
                cx.stop_propagation();
                true
            } else {
                false
            }
        };
        let target = rgba(th.nav_selected);
        row.relative()
            // An address chip dragged from another field lands here.
            .on_drag_move(cx.listener(|this, event: &DragMoveEvent<ChipDrag>, _, cx| {
                let drag = event.drag(cx).clone();
                this.chip_drag_moved(&drag, cx)
            }))
            .drag_over::<ChipDrag>(
                move |s, drag, _, _| {
                    if drag.field == field { s } else { s.bg(target) }
                },
            )
            .on_drop(
                cx.listener(move |this, drag: &ChipDrag, _, cx| this.drop_chip(drag, field, cx)),
            )
            .capture_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                // Those keys act on the selected chip themselves.
                let key = event.keystroke.key.as_str();
                if !matches!(key, "left" | "right" | "backspace" | "delete")
                    || event.keystroke.modifiers.modified()
                {
                    this.chip_other_key(cx);
                }
            }))
            .capture_action(cx.listener(move |this, _: &Left, _, cx| {
                if this.chip_arrow(field, true, cx) {
                    cx.stop_propagation();
                }
            }))
            .capture_action(cx.listener(move |this, _: &Right, _, cx| {
                if this.chip_arrow(field, false, cx) {
                    cx.stop_propagation();
                }
            }))
            .capture_action(cx.listener(move |this, _: &Backspace, _, cx| {
                if this.chip_backspace(field, false, cx) {
                    cx.stop_propagation();
                }
            }))
            .capture_action(cx.listener(move |this, _: &Delete, _, cx| {
                if this.chip_backspace(field, true, cx) {
                    cx.stop_propagation();
                }
            }))
            .capture_action(cx.listener(move |this, _: &Submit, _, cx| {
                if take(this, cx) {
                    let ix = this.selected_suggestion();
                    this.pick_suggestion(ix, cx);
                }
            }))
            .capture_action(cx.listener(move |this, _: &FocusNext, _, cx| {
                if take(this, cx) {
                    let ix = this.selected_suggestion();
                    this.pick_suggestion(ix, cx);
                }
            }))
            .capture_action(cx.listener(move |this, _: &Cancel, _, cx| {
                if take(this, cx) {
                    this.close_suggestions(cx);
                }
            }))
            .on_action(cx.listener(move |this, _: &Up, _, cx| {
                if this.suggesting(field) {
                    this.move_suggestion(-1, cx);
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(move |this, _: &Down, _, cx| {
                if this.suggesting(field) {
                    this.move_suggestion(1, cx);
                } else {
                    cx.propagate();
                }
            }))
            .when(self.suggesting(field), |row| {
                row.children(self.render_suggestions(th, cx))
            })
    }

    fn selected_suggestion(&self) -> usize {
        self.compose
            .as_ref()
            .and_then(|c| c.suggest.as_ref())
            .map_or(0, |s| s.selected)
    }

    /// The list under the field, over the rest of the message.
    fn render_suggestions(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let suggest = self.compose.as_ref()?.suggest.as_ref()?;
        Some(self.suggestion_list(
            &suggest.items,
            suggest.selected,
            px(24.0),
            Self::pick_suggestion,
            Self::close_suggestions,
            th,
            cx,
        ))
    }

    /// Address suggestions under a field, `left` in from its start;
    /// `pick` takes the index of the one clicked.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::window) fn suggestion_list(
        &self,
        items: &[Suggestion],
        selected: usize,
        left: gpui::Pixels,
        pick: fn(&mut Self, usize, &mut Context<Self>),
        close: fn(&mut Self, &mut Context<Self>),
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let rows: Vec<AnyElement> = items
            .iter()
            .enumerate()
            .map(|(ix, item)| self.render_suggestion(ix, item, ix == selected, pick, th, cx))
            .collect();
        let list = raised(div(), th, 15.0, 3.0)
            .id("recipient-suggestions")
            .occlude()
            .w(px(420.0))
            .py(px(8.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_mouse_down_out(cx.listener(move |this, _, _, cx| close(this, cx)))
            .children(rows);
        div()
            .absolute()
            .top_full()
            .left(left)
            .child(
                deferred(
                    anchored()
                        .offset(point(px(0.0), px(-4.0)))
                        .snap_to_window_with_margin(px(8.0))
                        .child(list),
                )
                .with_priority(2),
            )
            .into_any_element()
    }

    fn render_suggestion(
        &self,
        ix: usize,
        item: &Suggestion,
        selected: bool,
        pick: fn(&mut Self, usize, &mut Context<Self>),
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let bold = HighlightStyle {
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        };
        let marked = |text: &str, marks: &[std::ops::Range<usize>]| {
            let marks = marks
                .iter()
                .filter(|m| m.end <= text.len() && text.is_char_boundary(m.start))
                .map(|m| (m.clone(), bold))
                .collect::<Vec<_>>();
            StyledText::new(SharedString::from(text.to_owned())).with_highlights(marks)
        };
        let (title, title_marks) = match &item.name {
            Some(name) => (name.as_str(), item.name_marks.as_slice()),
            None => (item.email.as_str(), item.email_marks.as_slice()),
        };
        div()
            .id(("recipient-suggestion", ix))
            .flex_none()
            .h(px(ROW))
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .cursor_pointer()
            .when(selected, |d| d.bg(rgba(th.hover)))
            .hover(|s| s.bg(rgba(th.hover)))
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    pick(this, ix, cx);
                }),
            )
            .child(self.person_avatar(
                item.name.as_deref().unwrap_or(&item.email),
                &item.email,
                AVATAR,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .min_w_0()
                                    .text_size(px(15.0))
                                    .text_color(rgba(th.text))
                                    .truncate()
                                    .child(marked(title, title_marks)),
                            )
                            .children(self.muted_mark(&item.email, 16.0, th)),
                    )
                    .when(item.name.is_some(), |d| {
                        d.child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgba(th.text_dim))
                                .truncate()
                                .child(marked(&item.email, &item.email_marks)),
                        )
                    }),
            )
            .into_any_element()
    }
}

/// Addresses from the address book for `typed`, those `account` writes
/// to first, leaving out `skip`; none until the book is read.
pub(in crate::window) fn address_suggestions(
    typed: &str,
    account: Option<i64>,
    skip: &[String],
    cx: &gpui::App,
) -> Vec<Suggestion> {
    match cx.try_global::<Book>().and_then(|b| b.book.as_ref()) {
        Some(book) if !typed.trim().is_empty() => {
            let now = jiff::Timestamp::now().as_second();
            book.suggest(typed, account, now, skip, SHOWN)
        }
        _ => Vec::new(),
    }
}

/// Where the address being typed starts in `text`, and it: after the last
/// comma or semicolon outside quotes and angle brackets.
pub(super) fn last_entry(text: &str) -> (usize, &str) {
    let (mut start, mut quoted, mut angle) = (0, false, false);
    for (ix, c) in text.char_indices() {
        match c {
            '"' if !angle => quoted = !quoted,
            '<' if !quoted => angle = true,
            '>' if !quoted => angle = false,
            ',' | ';' if !quoted && !angle => start = ix + 1,
            _ => {}
        }
    }
    (start, &text[start..])
}

/// `Name <email>`, the name quoted when it has to be.
pub(super) fn mailbox(name: Option<&str>, email: &str) -> String {
    match name.map(str::trim).filter(|n| !n.is_empty()) {
        None => email.to_owned(),
        Some(name) if name.contains([',', ';', '"', '<', '>', '@', '(', ')']) => {
            // The address parser reads no escapes, so quotes in the name go.
            format!("\"{}\" <{email}>", name.replace('"', ""))
        }
        Some(name) => format!("{name} <{email}>"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_address_being_typed() {
        assert_eq!(last_entry("ka"), (0, "ka"));
        assert_eq!(last_entry("Kay Mann <kay@x.org>, su"), (21, " su"));
        assert_eq!(last_entry("\"Doe, Jo\" <jo@x.org>; b"), (21, " b"));
        assert_eq!(last_entry("a@x.org,"), (8, ""));
    }

    #[test]
    fn writes_mailboxes_that_parse_back() {
        for (name, email) in [
            (Some("Kay Mann"), "kay@x.org"),
            (Some("Doe, Jo"), "jo@x.org"),
            (Some("Say \"Hi\" <now>"), "hi@x.org"),
            (None, "bob@x.org"),
        ] {
            let text = format!("{}, ", mailbox(name, email));
            let parsed = outgoing::parse_addresses(&text).unwrap();
            assert_eq!(parsed.len(), 1, "{text}");
            assert_eq!(parsed[0].email, email);
            let name = name.map(|n| n.replace('"', ""));
            assert_eq!(parsed[0].name, name, "{text}");
        }
    }
}
