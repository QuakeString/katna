// SPDX-License-Identifier: GPL-3.0-or-later

//! Address suggestions under To, Cc and Bcc, as in Gmail: from the first
//! letter typed, the people written with most and most lately first, typos
//! forgiven ([`katna_search::contacts`]). Up and Down move, Enter or Tab
//! picks, Escape closes; a comma starts the next address.
//!
//! The address book is shared by every window and read in the background:
//! the saved copy first, then the store again when it is more than a few
//! minutes old.

use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, Div, FontWeight, Global, HighlightStyle, SharedString, StyledText,
    anchored, deferred, div, point, prelude::*, rgba,
};
use katna_search::contacts::{ContactBook, Suggestion};
use katna_ui::text_input::{Cancel, Down, Submit, Up};
use katna_ui::{TextInput, px};

use super::super::FocusNext;
use super::MailWindow;
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
}

impl Global for Book {}

impl MailWindow {
    /// Reads the address book if it is missing or old: the saved copy
    /// first, so suggestions work at once, then the store.
    pub(super) fn load_address_book(&mut self, cx: &mut Context<Self>) {
        let book = cx.default_global::<Book>();
        if book.loading || book.read.is_some_and(|read| read.elapsed() < FRESH) {
            return;
        }
        book.loading = true;
        let saved = book.book.is_none();
        let paths = self.paths.clone();
        cx.spawn(async move |_, cx| {
            if saved {
                let paths = paths.clone();
                let cached = cx
                    .background_executor()
                    .spawn(async move { data::cached_address_book(&paths) })
                    .await;
                cx.update(|cx| {
                    let book = cx.default_global::<Book>();
                    if book.book.is_none() {
                        book.book = cached;
                    }
                });
            }
            let started = Instant::now();
            let fresh = cx
                .background_executor()
                .spawn(async move {
                    let book = data::address_book(&paths)?;
                    data::save_address_book(&paths, &book);
                    Ok::<_, String>(book)
                })
                .await;
            cx.update(|cx| {
                let book = cx.default_global::<Book>();
                book.loading = false;
                book.read = Some(Instant::now());
                match fresh {
                    Ok(fresh) => {
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

    fn recipient_input(&self, field: Field) -> Option<&gpui::Entity<TextInput>> {
        let compose = self.compose.as_ref()?;
        Some(match field {
            Field::To => &compose.to,
            Field::Cc => &compose.cc,
            Field::Bcc => &compose.bcc,
        })
    }

    /// Suggests addresses for what is being typed in `field`.
    pub(super) fn recipient_changed(&mut self, field: Field, cx: &mut Context<Self>) {
        let Some(input) = self.recipient_input(field) else {
            return;
        };
        let text = input.read(cx).text().to_owned();
        let (start, typed) = last_entry(&text);
        // The account the message goes out from.
        let account = self.compose.as_ref().and_then(|compose| {
            compose
                .from
                .or_else(|| self.compose_account(compose.kind).map(|a| a.id))
                .map(|id| id.0)
        });
        let items = match cx.try_global::<Book>().and_then(|b| b.book.as_ref()) {
            Some(book) if !typed.trim().is_empty() => {
                // Everyone already added to To, Cc or Bcc.
                let mut skip = added(&text[..start]);
                for other in [Field::To, Field::Cc, Field::Bcc] {
                    if other != field
                        && let Some(input) = self.recipient_input(other)
                    {
                        skip.extend(added(input.read(cx).text()));
                    }
                }
                let now = jiff::Timestamp::now().as_second();
                book.suggest(typed, account, now, &skip, SHOWN)
            }
            _ => Vec::new(),
        };
        if let Some(compose) = &mut self.compose {
            compose.suggest = (!items.is_empty()).then_some(Suggestions {
                field,
                items,
                selected: 0,
            });
        }
        cx.notify();
    }

    fn close_suggestions(&mut self, cx: &mut Context<Self>) {
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
        let Some(item) = suggest.items.get(ix) else {
            return;
        };
        let mailbox = mailbox(item.name.as_deref(), &item.email);
        let Some(input) = self.recipient_input(suggest.field).cloned() else {
            return;
        };
        input.update(cx, |input, cx| {
            let text = input.text().to_owned();
            let (start, _) = last_entry(&text);
            let before = text[..start].trim_end();
            let text = if before.is_empty() {
                format!("{mailbox}, ")
            } else {
                format!("{before} {mailbox}, ")
            };
            input.set_text(text, cx);
        });
        cx.notify();
    }

    fn move_suggestion(&mut self, by: isize, cx: &mut Context<Self>) {
        if let Some(suggest) = self.compose.as_mut().and_then(|c| c.suggest.as_mut()) {
            let len = suggest.items.len() as isize;
            suggest.selected = (suggest.selected as isize + by).rem_euclid(len) as usize;
            cx.notify();
        }
    }

    fn suggesting(&self, field: Field) -> bool {
        self.compose
            .as_ref()
            .and_then(|c| c.suggest.as_ref())
            .is_some_and(|s| s.field == field)
    }

    /// Gives a recipient row its keys for the suggestions, and draws them
    /// under it.
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
        row.relative()
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
        let rows: Vec<AnyElement> = suggest
            .items
            .iter()
            .enumerate()
            .map(|(ix, item)| self.render_suggestion(ix, item, ix == suggest.selected, th, cx))
            .collect();
        let list = raised(div(), th, 15.0, 3.0)
            .id("recipient-suggestions")
            .occlude()
            .w(px(420.0))
            .py(px(8.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_mouse_down_out(cx.listener(|this, _, _, cx| this.close_suggestions(cx)))
            .children(rows);
        Some(
            div()
                .absolute()
                .top_full()
                .left(px(24.0))
                .child(
                    deferred(
                        anchored()
                            .offset(point(px(0.0), px(-4.0)))
                            .snap_to_window_with_margin(px(8.0))
                            .child(list),
                    )
                    .with_priority(2),
                )
                .into_any_element(),
        )
    }

    fn render_suggestion(
        &self,
        ix: usize,
        item: &Suggestion,
        selected: bool,
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
                    this.pick_suggestion(ix, cx);
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
                            .text_size(px(15.0))
                            .text_color(rgba(th.text))
                            .truncate()
                            .child(marked(title, title_marks)),
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

/// Where the address being typed starts in `text`, and it: after the last
/// comma or semicolon outside quotes and angle brackets.
fn last_entry(text: &str) -> (usize, &str) {
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

/// The addresses in `text` that parse, skipping any that do not.
fn added(mut text: &str) -> Vec<String> {
    let mut emails = Vec::new();
    while !text.is_empty() {
        let (start, entry) = last_entry(text);
        if let Ok(parsed) = outgoing::parse_addresses(entry) {
            emails.extend(parsed.into_iter().map(|m| m.email));
        }
        text = &text[..start.saturating_sub(1)];
    }
    emails
}

/// `Name <email>`, the name quoted when it has to be.
fn mailbox(name: Option<&str>, email: &str) -> String {
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
    fn lists_added_addresses_past_broken_ones() {
        assert_eq!(
            added("a@x.org, not an address; \"Doe, J\" <j@y.org>,"),
            ["j@y.org", "a@x.org"]
        );
        assert!(added("").is_empty());
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
