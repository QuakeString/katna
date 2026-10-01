// SPDX-License-Identifier: GPL-3.0-or-later

//! Recipients as chips, as in Gmail. An address becomes a rounded chip
//! showing the name alone, or the address when there is no name, once a
//! comma, semicolon, Enter or Tab ends it, the field is left, a suggestion
//! is picked or several addresses are pasted. A chip with a name has a
//! small arrow that shows its address. Double-clicking a chip puts it back
//! as text to fix, in its place. Backspace in an empty field selects the
//! last chip, and a second Backspace removes it. Left and Right step
//! through the chips and back to typing.
//!
//! Text that is not an address stays as a chip with a red outline, and
//! Send refuses to go while one is there.

use std::cell::RefCell;

use gpui::{
    AnyElement, ClickEvent, Context, Focusable, FontWeight, SharedString, Window, anchored, canvas,
    deferred, div, point, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::{TextInput, px, unpx};

use super::MailWindow;
use super::recipients::{Field, last_entry, mailbox};
use crate::outgoing;
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, filled_button, icon, raised, tip};

const HEIGHT: f32 = 30.0;
/// The picture at a chip's left end.
const AVATAR: f32 = 24.0;
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

    /// The recipient, for a chip that is an address.
    pub(super) fn mailbox(&self) -> Option<crate::outgoing::Mailbox> {
        self.valid.then(|| crate::outgoing::Mailbox {
            name: self.name.clone(),
            email: self.email.clone(),
        })
    }

    /// The name, if the address has one.
    pub(super) fn name(&self) -> Option<&str> {
        if self.valid {
            self.name.as_deref()
        } else {
            None
        }
    }

    /// The address, or the text as typed when it is not one.
    pub(super) fn email(&self) -> &str {
        if self.valid { &self.email } else { &self.typed }
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

/// The most chips of a field measured to lay out its first line.
const MAX_CHIPS: usize = 512;
/// A chip's width besides its name: the picture, the x and the padding.
const CHIP_CHROME: f32 = 56.0;
/// The ", " between folded names.
const FOLD_COMMA: f32 = 8.0;
/// Room for the "N more" badge after folded names.
const FOLD_BADGE: f32 = 88.0;

/// Where the chips of To, Cc and Bcc were last drawn: each chip's width,
/// the field's and that of what ends its first line, to work out how many
/// chips fit on the first line.
#[derive(Debug)]
pub(in crate::window) struct ChipLayout {
    chips: RefCell<Vec<f32>>,
    field: RefCell<Vec<f32>>,
    trailing: RefCell<Vec<f32>>,
    /// A touchpad's scrolling not yet a whole line of chips.
    wheel: std::cell::Cell<f32>,
}

impl Default for ChipLayout {
    fn default() -> Self {
        Self {
            chips: RefCell::new(vec![0.0; 3 * MAX_CHIPS]),
            field: RefCell::new(vec![0.0; 3]),
            trailing: RefCell::new(vec![0.0; 3]),
            wheel: std::cell::Cell::new(0.0),
        }
    }
}

impl ChipLayout {
    /// Records a width; whether it changed.
    fn set(&self, list: &RefCell<Vec<f32>>, at: usize, width: f32) -> bool {
        let mut list = list.borrow_mut();
        let changed = (list[at] - width).abs() > 0.5;
        if changed {
            list[at] = width;
        }
        changed
    }

    /// Room for chips on a field's first line.
    fn first_room(&self, field: Field) -> f32 {
        let trailing = self.trailing.borrow()[field.ix()];
        self.field.borrow()[field.ix()] - trailing - if trailing > 0.0 { 8.0 } else { 0.0 }
    }

    /// How many of the `count` chips fit on a field's first line, once
    /// they were all measured.
    fn first_line(&self, field: Field, count: usize, trailing: bool) -> Option<usize> {
        let widths = self.chips.borrow();
        let widths =
            widths.get(field.ix() * MAX_CHIPS..field.ix() * MAX_CHIPS + count.min(MAX_CHIPS))?;
        // A chip just added is measured once drawn: until then it goes on
        // the lines under the first, so nothing else moves for a frame.
        let measured = count <= MAX_CHIPS
            && self.field.borrow()[field.ix()] > 0.0
            && (!trailing || self.trailing.borrow()[field.ix()] > 0.0)
            && widths.first().is_none_or(|w| *w > 0.0);
        if !measured {
            return None;
        }
        let room = self.first_room(field);
        let mut used = 0.0;
        Some(
            widths
                .iter()
                .take_while(|w| {
                    if **w <= 0.0 {
                        return false;
                    }
                    let next = used + if used > 0.0 { 4.0 } else { 0.0 } + **w;
                    let fits = next <= room;
                    if fits {
                        used = next;
                    }
                    fits
                })
                .count(),
        )
    }

    /// How many names fit on a folded field's line, before its "N more"
    /// badge: a name is about as wide as its chip without the picture, the
    /// x and the padding.
    fn folded_fits(&self, field: Field, count: usize) -> usize {
        let widths = self.chips.borrow();
        let base = field.ix() * MAX_CHIPS;
        let room = self.first_room(field) - FOLD_BADGE;
        let mut used = 0.0;
        widths[base..base + count.min(MAX_CHIPS)]
            .iter()
            .take_while(|w| {
                if **w <= 0.0 {
                    return false;
                }
                let name = (**w - CHIP_CHROME).max(0.0);
                let next = used + if used > 0.0 { FOLD_COMMA } else { 0.0 } + name;
                let fits = next <= room;
                if fits {
                    used = next;
                }
                fits
            })
            .count()
            .max(1)
    }

    /// The room left on a field's first line after its first `count`
    /// chips.
    fn room_left(&self, field: Field, count: usize) -> f32 {
        let widths = self.chips.borrow();
        let base = field.ix() * MAX_CHIPS;
        let used: f32 = widths[base..base + count.min(MAX_CHIPS)]
            .iter()
            .map(|w| w + 4.0)
            .sum();
        self.first_room(field) - used
    }
}

/// Draws `this` again after the frame being drawn.
fn notify_later(this: &gpui::WeakEntity<MailWindow>, cx: &mut gpui::App) {
    let this = this.clone();
    cx.defer(move |cx| {
        this.update(cx, |_, cx| cx.notify()).ok();
    });
}

/// The lines of chips shown under the first before the rest scroll.
const LINES: f32 = 4.0;
/// A line of chips and the gap under it.
const LINE: f32 = HEIGHT + 4.0;

/// The thumb beside a field's lines that scroll, showing where the lines
/// in view are; from where the list was last drawn.
pub(super) fn scrollbar(scroll: &gpui::ScrollHandle, th: &Theme) -> Option<gpui::Div> {
    let max = unpx(scroll.max_offset().y);
    let view = unpx(scroll.bounds().size.height);
    if max < 1.0 || view < 1.0 {
        return None;
    }
    let thumb = (view * view / (view + max)).max(20.0);
    let at = (-unpx(scroll.offset().y) / max).clamp(0.0, 1.0);
    Some(
        div()
            .absolute()
            .right(px(2.0))
            .top(px(at * (view - thumb)))
            .w(px(4.0))
            .h(px(thumb))
            .rounded_full()
            .bg(rgba(th.text_dim & 0xffff_ff00 | 0x80)),
    )
}

/// Draws the mail window again once the current update and frame are
/// done.
pub(super) fn notify_soon(cx: &mut Context<MailWindow>) {
    cx.spawn(async move |this, cx| {
        this.update(cx, |_, cx| cx.notify()).ok();
    })
    .detach();
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
    /// The chip being dragged to another field, while a drag is under way.
    dragging: Option<(Field, usize)>,
}

/// A chip on its way to another field: a lifted copy follows the pointer
/// while the chip itself fades where it was.
#[derive(Clone)]
pub(super) struct ChipDrag {
    pub field: Field,
    ix: usize,
    label: SharedString,
    valid: bool,
    th: Theme,
}

impl Render for ChipDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let th = &self.th;
        div()
            .max_w(px(MAX_WIDTH))
            .h(px(HEIGHT))
            .px(px(10.0))
            .flex()
            .items_center()
            .rounded_full()
            .border_1()
            .border_color(rgba(if self.valid { th.divider } else { th.error }))
            .bg(rgba(th.raised))
            .shadow(elevation(th, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(if self.valid { th.text } else { th.error }))
            .cursor_grabbing()
            .child(div().min_w_0().truncate().child(self.label.clone()))
    }
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
        // Typing goes on at the end of a long list.
        if let Some(c) = &self.compose {
            c.chip_scroll[field.ix()].scroll_to_bottom();
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

    /// The chip the x on it removes.
    fn remove_chip(&mut self, field: Field, ix: usize, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            compose.chips.remove(field, ix);
        }
        self.chips_changed(field, cx);
    }

    /// The chip being dragged, while a drag is under way.
    pub(super) fn chip_dragging(&self, cx: &gpui::App) -> Option<(Field, usize)> {
        let compose = self.compose.as_ref()?;
        compose.chips.dragging.filter(|_| cx.has_active_drag())
    }

    /// Notes which chip a drag carries, so it fades and the hidden Cc and
    /// Bcc rows open to take it.
    pub(super) fn chip_drag_moved(&mut self, drag: &ChipDrag, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose
            && compose.chips.dragging != Some((drag.field, drag.ix))
        {
            compose.chips.dragging = Some((drag.field, drag.ix));
            cx.notify();
        }
    }

    /// Moves a dragged chip to the end of `to`.
    pub(super) fn drop_chip(&mut self, drag: &ChipDrag, to: Field, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.chips.dragging = None;
        let from = drag.field;
        if from != to
            && compose
                .chips
                .get(from)
                .get(drag.ix)
                .is_some_and(|c| c.label() == drag.label.as_ref())
            && let Some(chip) = compose.chips.remove(from, drag.ix)
        {
            compose.chips.lists[to.ix()].push(chip);
            match to {
                Field::Cc => compose.show_cc = true,
                Field::Bcc => compose.show_bcc = true,
                Field::To => {}
            }
            self.chips_changed(from, cx);
            self.chips_changed(to, cx);
        }
        cx.notify();
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

    /// Left and Right step through the chips, as in Gmail: Left at the
    /// start of the field selects the chip before it, and Right from the
    /// last chip goes back to typing. Whether it was taken.
    pub(super) fn chip_arrow(&mut self, field: Field, left: bool, cx: &mut Context<Self>) -> bool {
        let Some(input) = self.recipient_input(field) else {
            return false;
        };
        let at_start = input.read(cx).caret_at_start();
        let Some(compose) = &mut self.compose else {
            return false;
        };
        let chips = &mut compose.chips;
        if chips.editing.is_some_and(|(f, _)| f == field) {
            return false;
        }
        let count = chips.get(field).len();
        chips.selected = match (chips.selected.filter(|(f, _)| *f == field), left) {
            (Some((_, ix)), true) => Some((field, ix.saturating_sub(1))),
            (Some((_, ix)), false) => (ix + 1 < count).then_some((field, ix + 1)),
            (None, true) if at_start && count > 0 => Some((field, count - 1)),
            (None, _) => return false,
        };
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
        trailing: Option<AnyElement>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let Some(input) = self.recipient_input(field) else {
            return div().into_any_element();
        };
        let list = compose.chips.get(field);
        let layout = compose.chip_layout.clone();
        let fits = layout.first_line(field, list.len(), trailing.is_some());
        let this = cx.entity().downgrade();
        // Records the field's width, to work out what fits on its first
        // line next time.
        let measure_field = {
            let (layout, this) = (layout.clone(), this.clone());
            canvas(
                move |bounds, _, cx| {
                    if layout.set(&layout.field, field.ix(), unpx(bounds.size.width)) {
                        notify_later(&this, cx);
                    }
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full()
        };
        let trailing = trailing.map(|trailing| {
            let (layout, this) = (layout.clone(), this.clone());
            div().relative().flex_none().child(trailing).child(
                canvas(
                    move |bounds, _, cx| {
                        if layout.set(&layout.trailing, field.ix(), unpx(bounds.size.width)) {
                            notify_later(&this, cx);
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            )
        });
        // Away from the field, many recipients fold into one line:
        // "Name, Name, 19 more", as in Gmail. A click opens them again.
        if let Some(shown) = fits.filter(|shown| {
            *shown < list.len() && compose.active_field != Some(field) && self.chips_at_rest(field)
        }) {
            let shown = layout
                .folded_fits(field, list.len())
                .max(shown)
                .min(list.len());
            let names = list[..shown]
                .iter()
                .map(|c| c.label().to_owned())
                .collect::<Vec<_>>()
                .join(", ");
            let focus = input.focus_handle(cx);
            return div()
                .id(("recipients-folded", field.ix()))
                .relative()
                .flex_1()
                .min_w_0()
                .min_h(px(40.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .cursor_text()
                .on_click(cx.listener(move |this, _, window, cx| {
                    // Open, so the field is there to take the cursor.
                    if let Some(c) = &mut this.compose {
                        c.active_field = Some(field);
                        c.chip_scroll[field.ix()].scroll_to_bottom();
                    }
                    window.focus(&focus, cx);
                    cx.notify();
                }))
                .child(
                    div()
                        .min_w_0()
                        .truncate()
                        .text_color(rgba(th.text))
                        .child(names),
                )
                .when(shown < list.len(), |d| {
                    d.child(
                        div()
                            .flex_none()
                            .px(px(6.0))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(rgba(th.divider))
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("compose-more-recipients", count = list.len() - shown)),
                    )
                })
                .child(div().flex_1())
                .children(trailing)
                .child(measure_field)
                .into_any_element();
        }
        let mut chips: Vec<AnyElement> = list
            .iter()
            .enumerate()
            .map(|(ix, chip)| {
                let (layout, this) = (layout.clone(), this.clone());
                div()
                    .relative()
                    .flex_shrink(1.0)
                    .min_w_0()
                    .flex()
                    .child(self.render_chip(field, ix, chip, th, cx))
                    .child(
                        canvas(
                            move |bounds, _, cx| {
                                let at = field.ix() * MAX_CHIPS + ix.min(MAX_CHIPS - 1);
                                if ix < MAX_CHIPS
                                    && layout.set(&layout.chips, at, unpx(bounds.size.width))
                                {
                                    notify_later(&this, cx);
                                }
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                    )
                    .into_any_element()
            })
            .collect();
        // An address being fixed is typed where its chip was, about as
        // wide as its text; otherwise typing goes after the chips.
        let at = match compose.chips.editing {
            Some((f, at)) if f == field && at < chips.len() => at,
            _ => chips.len(),
        };
        // As tall as a chip, so the lines of a long list keep one pitch.
        let typing = div()
            .h(px(HEIGHT))
            .flex()
            .items_center()
            .child(input.clone());
        let typing = if at < chips.len() {
            let chars = input.read(cx).text().chars().count() as f32;
            typing
                .flex_none()
                .w(px((chars * 8.0 + 16.0).clamp(TYPING, 2.0 * MAX_WIDTH)))
        } else {
            typing.flex_1().min_w(px(TYPING))
        };
        let editing = at < chips.len();
        chips.insert(at, typing.into_any_element());
        let wrap = |d: gpui::Div| d.flex().flex_row().flex_wrap().items_center().gap(px(4.0));
        // Until the chips were measured, or while one is being fixed, they
        // wrap beside what ends the first line.
        let Some(first) = fits.filter(|_| !editing) else {
            return div()
                .relative()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(8.0))
                .child(wrap(div().flex_1().min_w_0().py(px(5.0))).children(chips))
                .children(trailing.map(|t| t.h(px(40.0)).flex().items_center()))
                .child(measure_field)
                .into_any_element();
        };
        // Only the first line makes room for what ends it; the lines under
        // it run to the edge. A long list scrolls under its first line.
        let typing_fits = first == list.len() && layout.room_left(field, first) >= TYPING;
        let first = if typing_fits { first + 1 } else { first };
        let rest: Vec<AnyElement> = chips.drain(first..).collect();
        let scroll = compose.chip_scroll[field.ix()].clone();
        div()
            .relative()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .child(
                div()
                    .min_h(px(40.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(4.0))
                    .children(chips)
                    .when(!typing_fits, |d| {
                        // The room left on the first line puts the cursor
                        // where typing goes.
                        let focus = input.focus_handle(cx);
                        d.child(
                            div()
                                .id(("recipients-room", field.ix()))
                                .flex_1()
                                .self_stretch()
                                .cursor_text()
                                .on_click(move |_, window, cx| window.focus(&focus, cx)),
                        )
                    })
                    .children(trailing.map(|t| t.ml(px(4.0)))),
            )
            .when(!rest.is_empty(), |d| {
                // Wheel over the list, caught before the list and the
                // conversation see it: the list moves by whole lines, never
                // cutting one, and at its end the conversation scrolls.
                let wheel = {
                    let (scroll, layout) = (scroll.clone(), layout.clone());
                    canvas(
                        |_, _, _| {},
                        move |bounds, _, window, _| {
                            let (scroll, layout) = (scroll.clone(), layout.clone());
                            window.on_mouse_event(
                                move |event: &gpui::ScrollWheelEvent, phase, window, cx| {
                                    if phase != gpui::DispatchPhase::Capture
                                        || !bounds.contains(&event.position)
                                    {
                                        return;
                                    }
                                    let dy = unpx(event.delta.pixel_delta(window.line_height()).y);
                                    let max = unpx(scroll.max_offset().y);
                                    let now = scroll.offset();
                                    let before = unpx(now.y).clamp(-max, 0.0);
                                    let at_end = if dy > 0.0 {
                                        before >= -0.5
                                    } else {
                                        before <= -max + 0.5
                                    };
                                    if dy == 0.0 || at_end {
                                        layout.wheel.set(0.0);
                                        return;
                                    }
                                    let steps = match event.delta {
                                        gpui::ScrollDelta::Lines(_) => dy.signum(),
                                        gpui::ScrollDelta::Pixels(_) => {
                                            let sum = layout.wheel.get() + dy;
                                            let steps = (sum / LINE).trunc();
                                            layout.wheel.set(sum - steps * LINE);
                                            steps
                                        }
                                    };
                                    let to = ((before / LINE).round() + steps) * LINE;
                                    scroll.set_offset(point(now.x, px(to.clamp(-max, 0.0))));
                                    cx.stop_propagation();
                                    window.refresh();
                                },
                            );
                        },
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                };
                d.child(
                    div()
                        .relative()
                        .pb(px(6.0))
                        .child(
                            wrap(div())
                                .id(("recipient-lines", field.ix()))
                                .max_h(px(LINES * LINE - 4.0))
                                .overflow_y_scroll()
                                .track_scroll(&scroll)
                                .pr(px(10.0))
                                .children(rest),
                        )
                        .children(scrollbar(&scroll, th))
                        .child(wheel),
                )
            })
            .child(measure_field)
            .into_any_element()
    }

    /// Whether a field's chips can fold away: none selected, open, being
    /// fixed or dragged, and no suggestions under it.
    fn chips_at_rest(&self, field: Field) -> bool {
        let Some(compose) = &self.compose else {
            return false;
        };
        let chips = &compose.chips;
        let here = |at: Option<(Field, usize)>| at.is_some_and(|(f, _)| f == field);
        !(here(chips.selected)
            || here(chips.open)
            || here(chips.editing)
            || chips.dragging.is_some()
            || self.suggesting(field))
    }

    /// Cc and Bcc in one faint pill, a line between them, beside To; each
    /// leaves the pill once its row shows.
    pub(super) fn render_cc_bcc(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        let fields: Vec<Field> = [(Field::Cc, compose.show_cc), (Field::Bcc, compose.show_bcc)]
            .into_iter()
            .filter(|(_, shown)| !shown)
            .map(|(field, _)| field)
            .collect();
        let last = fields.len().checked_sub(1)?;
        let line = fade(th.divider, super::FAINT_LINE);
        let mut pill = div()
            .flex_none()
            .h(px(26.0))
            .flex()
            .flex_row()
            .items_center()
            .rounded_full()
            .border_1()
            .border_color(rgba(line))
            .text_size(px(13.0))
            .text_color(rgba(th.text_dim));
        for (ix, field) in fields.into_iter().enumerate() {
            let label = match field {
                Field::Cc => tr!("compose-cc"),
                _ => tr!("compose-bcc"),
            };
            if ix > 0 {
                pill = pill.child(div().flex_none().w(px(1.0)).h(px(14.0)).bg(rgba(line)));
            }
            pill = pill.child(
                div()
                    .id(("show-copy-field", field.ix()))
                    .h_full()
                    .px(px(10.0))
                    .flex()
                    .items_center()
                    .when(ix == 0, |d| d.rounded_l_full())
                    .when(ix == last, |d| d.rounded_r_full())
                    .cursor_pointer()
                    .hover(|s| s.text_color(rgba(th.text)).bg(rgba(th.hover)))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        if let Some(c) = &mut this.compose {
                            let input = if field == Field::Cc {
                                c.show_cc = true;
                                &c.cc
                            } else {
                                c.show_bcc = true;
                                &c.bcc
                            };
                            window.focus(&input.focus_handle(cx), cx);
                        }
                        cx.notify();
                    }))
                    .child(div().relative().top(px(-self.pill_lift(13.0))).child(label)),
            );
        }
        Some(pill.into_any_element())
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
        let dragged = self.chip_dragging(cx) == Some((field, ix));
        let id = field.ix() * 10_000 + ix;
        let arrow = chip.valid && chip.name.is_some();
        let color = if chip.valid { th.text } else { th.error };
        let drag = ChipDrag {
            field,
            ix,
            label: SharedString::from(chip.label().to_owned()),
            valid: chip.valid,
            th: *th,
        };
        div()
            .id(("recipient-chip", id))
            .relative()
            // In a narrow field it gets shorter rather than running out.
            .flex_shrink(1.0)
            .min_w_0()
            .max_w(px(MAX_WIDTH))
            .h(px(HEIGHT))
            .pl(px((HEIGHT - AVATAR) / 2.0 - 1.0))
            .pr(px(4.0))
            .when(dragged, |d| d.opacity(0.4))
            .on_drag(drag, |drag, _, _, cx| cx.new(|_| drag.clone()))
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
            .child(div().flex_none().mr(px(4.0)).child(self.person_avatar(
                chip.name.as_deref().unwrap_or(&chip.email),
                &chip.email,
                AVATAR,
            )))
            .child(
                div()
                    .min_w_0()
                    .relative()
                    .top(px(-self.pill_lift(14.0)))
                    .truncate()
                    .child(SharedString::from(chip.label().to_owned())),
            )
            .children(
                chip.valid
                    .then(|| self.muted_mark(&chip.email, 16.0, th))
                    .flatten()
                    .map(|mark| div().flex_none().ml(px(4.0)).child(mark)),
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
            // Always shown, so the chip keeps its width.
            .child(
                div()
                    .id(("recipient-chip-remove", id))
                    .flex_none()
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .tooltip(tip(tr!("recipient-remove"), th))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.remove_chip(field, ix, cx);
                    }))
                    .child(icon("close", th.text_dim, 16.0)),
            )
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
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .min_w_0()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.text))
                                    .truncate()
                                    .child(name),
                            )
                            .children(self.muted_mark(&chip.email, 16.0, th)),
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

    /// Opens the field with the bad address and puts the keys in it.
    pub(super) fn fix_bad_address(
        &mut self,
        field: Field,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(c) = &mut self.compose {
            c.popup = None;
            c.header_open = true;
            match field {
                Field::Cc => c.show_cc = true,
                Field::Bcc => c.show_bcc = true,
                Field::To => {}
            }
        }
        if let Some(input) = self.recipient_input(field).cloned() {
            window.focus(&input.focus_handle(cx), cx);
        }
        cx.notify();
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
                    cx.listener(move |this, _, window, cx| this.fix_bad_address(field, window, cx)),
                ),
            ))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured(field: f32, trailing: f32, chips: &[f32]) -> ChipLayout {
        let layout = ChipLayout::default();
        layout.set(&layout.field, Field::To.ix(), field);
        layout.set(&layout.trailing, Field::To.ix(), trailing);
        for (ix, width) in chips.iter().enumerate() {
            layout.set(&layout.chips, ix, *width);
        }
        layout
    }

    #[test]
    fn only_the_first_line_leaves_room_for_the_buttons() {
        // 500 wide, 192 for the buttons and the gap before them: two
        // 150-wide chips fit, the third goes to the lines under it.
        let layout = measured(500.0, 184.0, &[150.0, 150.0, 150.0]);
        assert_eq!(layout.first_line(Field::To, 3, true), Some(2));
        // A chip just added, not measured yet, goes under the first line.
        let layout = measured(500.0, 184.0, &[150.0]);
        assert_eq!(layout.first_line(Field::To, 2, true), Some(1));
        // Nothing measured yet: laid out as before.
        assert_eq!(
            measured(500.0, 184.0, &[]).first_line(Field::To, 1, true),
            None
        );
        assert_eq!(ChipLayout::default().first_line(Field::To, 0, false), None);
    }

    #[test]
    fn folded_names_take_less_room_than_their_chips() {
        // Names without their picture and x: 94 each, and ", " between.
        let layout = measured(500.0, 184.0, &[150.0; 5]);
        assert_eq!(layout.first_line(Field::To, 5, true), Some(2));
        assert_eq!(layout.folded_fits(Field::To, 5), 2);
        let wide = measured(800.0, 184.0, &[150.0; 5]);
        assert_eq!(wide.folded_fits(Field::To, 5), 5);
        // At least one name, however narrow the field.
        assert_eq!(
            measured(100.0, 184.0, &[150.0]).folded_fits(Field::To, 1),
            1
        );
    }

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
