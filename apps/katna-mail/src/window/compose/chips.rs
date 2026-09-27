// SPDX-License-Identifier: GPL-3.0-or-later

//! Recipients as chips, as in Gmail. An address becomes a rounded chip
//! showing the name alone, or the address when there is no name, once a
//! comma, semicolon, Enter or Tab ends it, the field is left, a suggestion
//! is picked or several addresses are pasted. A chip with a name has a
//! small arrow that shows its address. Double-clicking a chip puts it back
//! as text to fix, in its place. Backspace in an empty field selects the
//! last chip, and a second Backspace removes it.
//!
//! Text that is not an address stays as a chip with a red outline, and
//! Send refuses to go while one is there.

use gpui::{
    AnyElement, ClickEvent, Context, Focusable, FontWeight, SharedString, Window, anchored,
    deferred, div, point, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::{TextInput, px};

use super::MailWindow;
use super::recipients::{Field, last_entry, mailbox};
use crate::outgoing;
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, raised, tip};

const HEIGHT: f32 = 26.0;
/// The widest a chip gets before its label is cut short.
const MAX_WIDTH: f32 = 280.0;
/// The least room left for typing after the chips.
const TYPING: f32 = 96.0;
const CARD_AVATAR: f32 = 40.0;

/// One recipient.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::window) struct Chip {
    name: Option<String>,
    email: String,
    /// The text as typed, for one that is not an address.
    typed: String,
    valid: bool,
}

impl Chip {
    fn new(name: Option<String>, email: String) -> Self {
        Self {
            name: name.filter(|n| !n.trim().is_empty()),
            typed: String::new(),
            email,
            valid: true,
        }
    }

    /// The chips for one typed entry: an address, `Name <address>`, or
    /// addresses separated by spaces, as pasted from elsewhere. Anything
    /// else is one chip that is not an address.
    fn parse(entry: &str) -> Vec<Chip> {
        let entry = entry.trim();
        if entry.is_empty() {
            return Vec::new();
        }
        if let Ok(list) = outgoing::parse_addresses(entry) {
            return list
                .into_iter()
                .map(|m| Chip::new(m.name, m.email))
                .collect();
        }
        let words: Vec<&str> = entry.split_whitespace().collect();
        if words.len() > 1 && words.iter().all(|w| outgoing::valid_email(w)) {
            return words
                .into_iter()
                .map(|w| Chip::new(None, w.to_owned()))
                .collect();
        }
        vec![Chip {
            name: None,
            email: String::new(),
            typed: entry.to_owned(),
            valid: false,
        }]
    }

    /// What the chip shows: the name, else the address.
    fn label(&self) -> &str {
        if !self.valid {
            &self.typed
        } else {
            self.name.as_deref().unwrap_or(&self.email)
        }
    }

    /// The chip as text again: `Name <address>`.
    fn text(&self) -> String {
        if self.valid {
            mailbox(self.name.as_deref(), &self.email)
        } else {
            self.typed.clone()
        }
    }
}

/// The chips of To, Cc and Bcc.
#[derive(Debug, Default)]
pub(in crate::window) struct Chips {
    lists: [Vec<Chip>; 3],
    /// The chip Backspace or a click selected.
    selected: Option<(Field, usize)>,
    /// The chip whose address card is open.
    open: Option<(Field, usize)>,
    /// Where the address being fixed after a double-click goes back.
    editing: Option<(Field, usize)>,
}

impl Chips {
    pub fn new(to: &str, cc: &str, bcc: &str) -> Self {
        Self {
            lists: [chips_of(to), chips_of(cc), chips_of(bcc)],
            ..Default::default()
        }
    }

    pub fn get(&self, field: Field) -> &[Chip] {
        &self.lists[field.ix()]
    }

    pub fn set(&mut self, field: Field, text: &str) {
        self.lists[field.ix()] = chips_of(text);
        self.selected = None;
        self.open = None;
        self.editing = None;
    }

    /// The field as text, the chips first, then what is still being typed.
    pub fn text(&self, field: Field, typing: &str) -> String {
        let mut parts: Vec<String> = self.get(field).iter().map(Chip::text).collect();
        if !typing.trim().is_empty() {
            parts.push(typing.trim().to_owned());
        }
        parts.join(", ")
    }

    /// The addresses of every chip, to leave out of suggestions.
    pub fn emails(&self) -> Vec<String> {
        self.lists
            .iter()
            .flatten()
            .filter(|c| c.valid)
            .map(|c| c.email.clone())
            .collect()
    }

    /// Adds `chips` to `field`: where the one being fixed was, else last.
    fn add(&mut self, field: Field, chips: Vec<Chip>) {
        let list = &mut self.lists[field.ix()];
        match self.editing.take() {
            Some((f, at)) if f == field => {
                let at = at.min(list.len());
                list.splice(at..at, chips);
            }
            _ => list.extend(chips),
        }
        self.selected = None;
    }

    fn remove(&mut self, field: Field, ix: usize) -> Option<Chip> {
        let list = &mut self.lists[field.ix()];
        let chip = (ix < list.len()).then(|| list.remove(ix));
        self.selected = None;
        self.open = None;
        if let Some((f, at)) = &mut self.editing
            && *f == field
            && ix < *at
        {
            *at -= 1;
        }
        chip
    }
}

/// `text` as a field of chips, as in a draft or a `mailto:` link.
pub(in crate::window) fn chips_of(text: &str) -> Vec<Chip> {
    entries(text).into_iter().flat_map(Chip::parse).collect()
}

/// `text` as it reads once turned into chips and back: what a field opened
/// with `text` holds.
pub(in crate::window) fn normalized(text: &str) -> String {
    let chips: Vec<String> = chips_of(text).iter().map(Chip::text).collect();
    chips.join(", ")
}

/// The entries of `text`, split at commas and semicolons outside quotes
/// and angle brackets.
fn entries(mut text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    loop {
        let (start, entry) = last_entry(text);
        out.push(entry);
        if start == 0 {
            break;
        }
        text = &text[..start - 1];
    }
    out.reverse();
    out.retain(|e| !e.trim().is_empty());
    out
}

impl MailWindow {
    pub(super) fn recipient_input(&self, field: Field) -> Option<&gpui::Entity<TextInput>> {
        let compose = self.compose.as_ref()?;
        Some(match field {
            Field::To => &compose.to,
            Field::Cc => &compose.cc,
            Field::Bcc => &compose.bcc,
        })
    }

    /// Turns the addresses typed in `field` into chips: those ended by a
    /// comma or semicolon, or all of them when `all`. Whether any was made.
    pub(super) fn commit_recipients(
        &mut self,
        field: Field,
        all: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(input) = self.recipient_input(field).cloned() else {
            return false;
        };
        let text = input.read(cx).text().to_owned();
        let (done, rest) = if all {
            (text.as_str(), "")
        } else {
            let (start, rest) = last_entry(&text);
            (&text[..start], rest)
        };
        let chips: Vec<Chip> = entries(done).into_iter().flat_map(Chip::parse).collect();
        if done.is_empty() {
            if all && let Some(compose) = &mut self.compose {
                // Left empty: the next address goes after the chips.
                compose.chips.editing = None;
            }
            return false;
        }
        let made = !chips.is_empty();
        let rest = rest.trim_start().to_owned();
        if let Some(compose) = &mut self.compose {
            compose.chips.add(field, chips);
        }
        input.update(cx, |input, cx| input.set_text(rest, cx));
        self.chips_changed(field, cx);
        made
    }

    /// Puts a picked suggestion in `field` as a chip, in place of what was
    /// being typed.
    pub(super) fn add_recipient(
        &mut self,
        field: Field,
        name: Option<String>,
        email: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(compose) = &mut self.compose {
            compose.chips.add(field, vec![Chip::new(name, email)]);
        }
        if let Some(input) = self.recipient_input(field).cloned() {
            input.update(cx, |input, cx| input.set_text("", cx));
        }
        self.chips_changed(field, cx);
    }

    /// The placeholder shows only while the field has no chips.
    pub(super) fn chips_changed(&mut self, field: Field, cx: &mut Context<Self>) {
        let empty = self
            .compose
            .as_ref()
            .is_some_and(|c| c.chips.get(field).is_empty());
        if field == Field::To
            && let Some(input) = self.recipient_input(field).cloned()
        {
            input.update(cx, |input, cx| {
                input.set_placeholder(if empty {
                    tr!("compose-recipients")
                } else {
                    String::new()
                });
                cx.notify();
            });
        }
        cx.notify();
    }

    /// Puts chip `ix` back as text to fix, in its place.
    fn edit_chip(&mut self, field: Field, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_recipients(field, true, cx);
        let Some(chip) = self
            .compose
            .as_mut()
            .and_then(|c| c.chips.remove(field, ix))
        else {
            return;
        };
        if let Some(compose) = &mut self.compose {
            compose.chips.editing = Some((field, ix));
        }
        if let Some(input) = self.recipient_input(field).cloned() {
            input.update(cx, |input, cx| input.set_text(chip.text(), cx));
            window.focus(&input.focus_handle(cx), cx);
        }
        self.chips_changed(field, cx);
    }

    fn select_chip(
        &mut self,
        field: Field,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(compose) = &mut self.compose {
            compose.chips.selected = Some((field, ix));
            compose.chips.open = None;
        }
        if let Some(input) = self.recipient_input(field).cloned() {
            window.focus(&input.focus_handle(cx), cx);
        }
        cx.notify();
    }

    fn toggle_chip_card(&mut self, field: Field, ix: usize, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            let chips = &mut compose.chips;
            chips.open = (chips.open != Some((field, ix))).then_some((field, ix));
        }
        cx.notify();
    }

    fn close_chip_card(&mut self, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose
            && compose.chips.open.take().is_some()
        {
            cx.notify();
        }
    }

    /// Backspace (or Delete, when `delete`) on chips, before the field
    /// sees it: in an empty field Backspace selects the chip before the
    /// cursor, and a second one removes it. Whether it was taken.
    pub(super) fn chip_backspace(
        &mut self,
        field: Field,
        delete: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(input) = self.recipient_input(field) else {
            return false;
        };
        let empty = input.read(cx).text().is_empty();
        let Some(compose) = &mut self.compose else {
            return false;
        };
        let chips = &mut compose.chips;
        if let Some((_, ix)) = chips.selected.filter(|(f, _)| *f == field) {
            chips.remove(field, ix);
            self.chips_changed(field, cx);
            return true;
        }
        if delete || !empty {
            return false;
        }
        // The one before an address being fixed, else the last.
        let before = match chips.editing {
            Some((f, at)) if f == field => at,
            _ => chips.get(field).len(),
        };
        if before == 0 {
            return false;
        }
        chips.selected = Some((field, before - 1));
        cx.notify();
        true
    }

    /// Any other key lets go of a selected chip.
    pub(super) fn chip_other_key(&mut self, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose
            && compose.chips.selected.take().is_some()
        {
            cx.notify();
        }
    }

    /// The first recipient that is not an address, and its field.
    pub(super) fn bad_recipient(&self, cx: &gpui::App) -> Option<(Field, String)> {
        let compose = self.compose.as_ref()?;
        [Field::To, Field::Cc, Field::Bcc]
            .into_iter()
            .find_map(|field| {
                let typing = self.recipient_input(field)?.read(cx).text().to_owned();
                compose
                    .chips
                    .get(field)
                    .iter()
                    .cloned()
                    .chain(chips_of(&typing))
                    .find(|c| !c.valid)
                    .map(|c| (field, c.typed))
            })
    }

    /// The chips of `field` and the text being typed after them.
    pub(super) fn render_recipient_field(
        &self,
        field: Field,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let Some(input) = self.recipient_input(field) else {
            return div().into_any_element();
        };
        let mut chips: Vec<AnyElement> = compose
            .chips
            .get(field)
            .iter()
            .enumerate()
            .map(|(ix, chip)| self.render_chip(field, ix, chip, th, cx))
            .collect();
        // An address being fixed is typed where its chip was, about as
        // wide as its text; otherwise typing goes after the chips.
        let at = match compose.chips.editing {
            Some((f, at)) if f == field && at < chips.len() => at,
            _ => chips.len(),
        };
        let typing = div().child(input.clone());
        let typing = if at < chips.len() {
            let chars = input.read(cx).text().chars().count() as f32;
            typing
                .flex_none()
                .w(px((chars * 8.0 + 16.0).clamp(TYPING, 2.0 * MAX_WIDTH)))
        } else {
            typing.flex_1().min_w(px(TYPING))
        };
        chips.insert(at, typing.into_any_element());
        div()
            .flex_1()
            .min_w_0()
            .py(px(6.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(4.0))
            .children(chips)
            .into_any_element()
    }

    fn render_chip(
        &self,
        field: Field,
        ix: usize,
        chip: &Chip,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let chips = &self.compose.as_ref().map(|c| &c.chips);
        let selected = chips.is_some_and(|c| c.selected == Some((field, ix)));
        let open = chips.is_some_and(|c| c.open == Some((field, ix)));
        let id = field.ix() * 10_000 + ix;
        let arrow = chip.valid && chip.name.is_some();
        let color = if chip.valid { th.text } else { th.error };
        div()
            .id(("recipient-chip", id))
            .relative()
            .flex_none()
            .max_w(px(MAX_WIDTH))
            .h(px(HEIGHT))
            .pl(px(10.0))
            .pr(px(if arrow { 2.0 } else { 10.0 }))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .rounded_full()
            .border_1()
            .border_color(rgba(if chip.valid { th.divider } else { th.error }))
            .text_size(px(14.0))
            .text_color(rgba(color))
            .cursor_pointer()
            .when(selected, |d| d.bg(rgba(th.nav_selected)))
            .when(!selected, |d| d.hover(|s| s.bg(rgba(th.hover))))
            .when(!chip.valid, |d| {
                d.tooltip(tip(tr!("recipient-not-valid"), th))
            })
            .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                if event.click_count() >= 2 {
                    this.edit_chip(field, ix, window, cx);
                } else {
                    this.select_chip(field, ix, window, cx);
                }
            }))
            .child(
                div()
                    .min_w_0()
                    .truncate()
                    .child(SharedString::from(chip.label().to_owned())),
            )
            .when(arrow, |d| {
                d.child(
                    div()
                        .id(("recipient-chip-arrow", id))
                        .flex_none()
                        .size(px(20.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .when(!open, |d| d.tooltip(tip(tr!("recipient-show-address"), th)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.toggle_chip_card(field, ix, cx);
                        }))
                        .child(icon("chevron-down", th.text_dim, 18.0)),
                )
            })
            .when(open, |d| d.child(self.render_chip_card(chip, th, cx)))
            .into_any_element()
    }

    /// The name and address of a chip, under it.
    fn render_chip_card(&self, chip: &Chip, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let name = chip.name.clone().unwrap_or_default();
        let card = raised(div(), th, 15.0, 3.0)
            .id("recipient-chip-card")
            .occlude()
            .w(px(320.0))
            .p(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .cursor_default()
            .on_mouse_down_out(cx.listener(|this, _, _, cx| this.close_chip_card(cx)))
            .child(self.person_avatar(&name, &chip.email, CARD_AVATAR))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text))
                            .truncate()
                            .child(name),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_dim))
                            .truncate()
                            .child(chip.email.clone()),
                    ),
            );
        div()
            .absolute()
            .top_full()
            .left_0()
            .child(
                deferred(
                    anchored()
                        .offset(point(px(0.0), px(4.0)))
                        .snap_to_window_with_margin(px(8.0))
                        .child(card),
                )
                .with_priority(2),
            )
            .into_any_element()
    }

    /// Asked when Send finds a recipient that is not an address.
    pub(super) fn render_bad_address(
        &self,
        field: Field,
        address: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        Self::dialog_card(th, 380.0, tr!("recipient-bad-title"))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("recipient-bad-text", address = address.to_owned())),
            )
            .child(div().mt(px(20.0)).flex().flex_row().justify_end().child(
                filled_button("recipient-bad-fix", tr!("recipient-bad-fix"), th).on_click(
                    cx.listener(move |this, _, window, cx| {
                        if let Some(c) = &mut this.compose {
                            c.popup = None;
                            match field {
                                Field::Cc => c.show_cc = true,
                                Field::Bcc => c.show_bcc = true,
                                Field::To => {}
                            }
                        }
                        if let Some(input) = this.recipient_input(field).cloned() {
                            window.focus(&input.focus_handle(cx), cx);
                        }
                        cx.notify();
                    }),
                ),
            ))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(chips: &[Chip]) -> Vec<(&str, bool)> {
        chips.iter().map(|c| (c.label(), c.valid)).collect()
    }

    #[test]
    fn turns_text_into_chips() {
        let chips = chips_of("Kay Mann <kay@x.org>, bob@x.org; \"Doe, Jo\" <jo@x.org>,");
        assert_eq!(
            labels(&chips),
            [("Kay Mann", true), ("bob@x.org", true), ("Doe, Jo", true)]
        );
        assert_eq!(chips[2].email, "jo@x.org");
        // Pasted with spaces between.
        assert_eq!(
            labels(&chips_of("a@x.org b@y.org")),
            [("a@x.org", true), ("b@y.org", true)]
        );
        assert_eq!(
            labels(&chips_of("xyz, a@x.org")),
            [("xyz", false), ("a@x.org", true)]
        );
        assert!(chips_of(" , ;").is_empty());
    }

    #[test]
    fn reads_back_as_the_same_text() {
        let mut chips = Chips::new("Kay Mann <kay@x.org>,bob@x.org", "", "xyz");
        assert_eq!(
            chips.text(Field::To, "sara@"),
            "Kay Mann <kay@x.org>, bob@x.org, sara@"
        );
        assert_eq!(chips.text(Field::Bcc, ""), "xyz");
        assert_eq!(
            normalized("Kay Mann <kay@x.org>,bob@x.org"),
            chips.text(Field::To, "")
        );
        // A fixed address goes back where it was.
        let kay = chips.remove(Field::To, 0).unwrap();
        chips.editing = Some((Field::To, 0));
        chips.add(Field::To, vec![kay]);
        assert_eq!(chips.get(Field::To)[0].label(), "Kay Mann");
        assert_eq!(chips.emails(), ["kay@x.org", "bob@x.org"]);
    }
}
