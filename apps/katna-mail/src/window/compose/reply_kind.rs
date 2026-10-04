// SPDX-License-Identifier: GPL-3.0-or-later

//! The head of a reply written in its conversation, as in Gmail: one line
//! naming the recipients until it is clicked, which opens the From, To, Cc
//! and Bcc rows; a click in the text closes them again. The arrow beside
//! the reply icon turns the message into a reply, a reply to all or a
//! forward, keeping what was written.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, Div, Focusable, MouseButton, Task, Window, anchored, canvas, deferred,
    div, point, prelude::*, rgba, svg,
};
use katna_i18n::tr;
use katna_ui::rich::{Block, Doc};
use katna_ui::{px, unpx};

use super::super::MailWindow;
use super::chips::Chip;
use super::recipients::Field;
use super::tools::Popup;
use super::{Kind, Original, Threading, draft, trim_quote};
use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, menu, menu_item_icon, tip};

/// The line a forward starts its copy of the message with.
const FORWARDED: &str = "---------- Forwarded message ---------";
/// The height of the notch on the kind menu, pointing at its button.
const NOTCH: f32 = 8.0;

/// How long the rows take to grow in or shrink away.
const ROWS_GLIDE: Duration = Duration::from_millis(200);
/// How near the top of the conversation the card may go as it grows.
const CARD_ROOM: f32 = 12.0;

/// The rows of an inline reply's head growing in or shrinking away, the
/// card growing upwards: the conversation scrolls by as much as they grow.
#[derive(Default)]
pub(in crate::window) struct RowsGlide {
    /// Whether the rows were open when last drawn.
    was_open: Cell<Option<bool>>,
    /// Their height, measured while they show.
    natural: Rc<Cell<f32>>,
    /// How far the conversation scrolled on as they opened, to scroll
    /// back as they close.
    lifted: Cell<f32>,
    run: RefCell<Option<RowsRun>>,
}

struct RowsRun {
    opening: bool,
    /// Opening, it starts once the rows were measured.
    start: Cell<Option<Instant>>,
    /// The height the rows were last drawn at.
    drawn: Cell<f32>,
    _tick: Task<()>,
}

impl RowsRun {
    /// How much of the rows shows, 0 to 1, and whether it is over.
    fn shown(&self) -> (f32, bool) {
        let Some(start) = self.start.get() else {
            return (if self.opening { 0.0 } else { 1.0 }, false);
        };
        let t = (start.elapsed().as_secs_f32() / ROWS_GLIDE.as_secs_f32()).min(1.0);
        let eased = 1.0 - (1.0 - t).powi(3);
        (if self.opening { eased } else { 1.0 - eased }, t >= 1.0)
    }
}

/// What a chip shows in the one-line head: the name and the address, or
/// only the name when there are several.
fn summary_label(chip: &Chip, several: bool) -> String {
    match (chip.name(), several) {
        (Some(name), true) => name.to_owned(),
        (Some(name), false) => format!("{name} ({})", chip.email()),
        (None, _) => chip.email().to_owned(),
    }
}

/// The kinds a reply can turn into: icon, label.
fn kinds() -> [(Kind, &'static str, String); 3] {
    [
        (Kind::Reply, "reply", tr!("reply-reply")),
        (Kind::ReplyAll, "reply-all", tr!("reply-reply-all")),
        (Kind::Forward, "forward", tr!("reply-forward")),
    ]
}

/// Takes a forward's copy of the message, and the blank line before it,
/// off the end of `blocks`.
fn trim_forward(blocks: &mut Vec<Block>) {
    fn para(b: &Block) -> Option<&katna_ui::rich::Para> {
        match b {
            Block::Para(p) => Some(p),
            _ => None,
        }
    }
    let Some(at) = blocks
        .iter()
        .rposition(|b| para(b).is_some_and(|p| p.style.quote == 0 && p.text.trim() == FORWARDED))
    else {
        return;
    };
    let blank = at > 1
        && para(&blocks[at - 1])
            .is_some_and(|p| p.text.is_empty() && !p.style.signature && p.style.quote == 0);
    blocks.truncate(if blank { at - 1 } else { at });
}

impl MailWindow {
    /// The inline reply shows its rows: always while nobody is in To, Cc
    /// or Bcc yet.
    pub(super) fn reply_header_open(&self) -> bool {
        self.compose.as_ref().is_some_and(|c| {
            c.header_open
                || [Field::To, Field::Cc, Field::Bcc]
                    .iter()
                    .all(|f| c.chips.get(*f).is_empty())
        })
    }

    /// Starts the rows growing in or shrinking away when `open` changed
    /// since they were last drawn; how much of them shows while they do.
    pub(super) fn rows_shown(&self, open: bool, cx: &mut Context<Self>) -> Option<f32> {
        let compose = self.compose.as_ref()?;
        let glide = &compose.rows_glide;
        let was = glide.was_open.replace(Some(open));
        if was == Some(!open) && !cx.reduce_motion() {
            let tick = cx.spawn(async |this, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(16))
                        .await;
                    if this.update(cx, |_, cx| cx.notify()).is_err() {
                        break;
                    }
                }
            });
            let natural = if open { 0.0 } else { glide.natural.get() };
            *glide.run.borrow_mut() = Some(RowsRun {
                opening: open,
                start: Cell::new((!open).then(Instant::now)),
                drawn: Cell::new(natural),
                _tick: tick,
            });
            if open {
                glide.natural.set(0.0);
                glide.lifted.set(0.0);
            }
        }
        let mut run = glide.run.borrow_mut();
        let shown = run.as_ref().map(|run| {
            if run.start.get().is_none() && glide.natural.get() > 0.0 {
                run.start.set(Some(Instant::now()));
            }
            run.shown()
        });
        match shown {
            Some((_, true)) => {
                *run = None;
                if !open {
                    glide.natural.set(0.0);
                }
                None
            }
            Some((shown, false)) => Some(shown),
            None => None,
        }
    }

    /// The rows, cut to how much of them shows while they glide; the
    /// conversation scrolls by what they grew so the card's bottom stays.
    pub(super) fn glide_rows(
        &self,
        rows: Div,
        shown: Option<f32>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return rows.into_any_element();
        };
        let glide = &compose.rows_glide;
        let (natural, this) = (glide.natural.clone(), cx.entity().downgrade());
        let measure = canvas(
            move |bounds, _, cx| {
                let height = unpx(bounds.size.height);
                if natural.get() != height {
                    natural.set(height);
                    let this = this.clone();
                    cx.defer(move |cx| {
                        this.update(cx, |_, cx| cx.notify()).ok();
                    });
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();
        let rows = rows.relative().child(measure);
        let (Some(shown), Some(run)) =
            (shown, glide.run.borrow().as_ref().map(|r| r.drawn.clone()))
        else {
            return rows.into_any_element();
        };
        let height = glide.natural.get() * shown;
        let grown = height - run.replace(height);
        if grown != 0.0 {
            // Scrolled on by as much, the card's bottom stays where it was
            // and its top goes up, as long as the top stays in sight;
            // closing, it scrolls back as far. The Send row placed from the
            // last frame moves with the rows.
            let offset = unpx(self.reader_scroll.offset().y);
            let mut at = compose.stick.get();
            let lift = if grown > 0.0 {
                let room = (at.card_top + offset - CARD_ROOM).max(0.0);
                grown.min(room)
            } else {
                -(-grown).min(glide.lifted.get())
            };
            glide.lifted.set(glide.lifted.get() + lift);
            if lift != 0.0 {
                let y = (offset - lift).min(0.0);
                self.reader_scroll
                    .set_offset(point(self.reader_scroll.offset().x, px(y)));
            }
            at.footer_top += grown;
            compose.stick.set(at);
        }
        div()
            .flex_none()
            .h(px(height))
            .overflow_hidden()
            .opacity(shown)
            .child(rows)
            .into_any_element()
    }

    /// Opens the rows of the inline reply's head, with the cursor in To.
    fn open_reply_header(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose {
            c.header_open = true;
            window.focus(&c.to.focus_handle(cx), cx);
        }
        cx.notify();
    }

    /// Closes the rows again once the text is clicked, when nothing is
    /// half typed in them.
    pub(super) fn close_reply_header(&mut self, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let typing = [&c.to, &c.cc, &c.bcc]
            .iter()
            .any(|input| !input.read(cx).text().trim().is_empty());
        if c.header_open && !typing {
            c.header_open = false;
            cx.notify();
        }
    }

    /// The reply icon with its arrow, which opens the menu of kinds.
    pub(super) fn render_kind_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let current = compose.kind;
        let open = compose.popup == Some(Popup::Kind);
        let (_, name, label) = kinds()
            .into_iter()
            .find(|(k, ..)| *k == current)
            .unwrap_or_else(|| kinds()[0].clone());
        let items = kinds().into_iter().map(|(kind, name, label)| {
            menu_item_icon(("reply-kind", kind as usize), name, &label, th)
                .when(kind == current, |d| {
                    d.child(div().flex_1())
                        .child(icon("check", th.nav_selected_text, 20.0))
                })
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.switch_reply_kind(kind, window, cx);
                }))
        });
        let popup = open.then(|| {
            let panel = menu(th)
                .relative()
                .min_w(px(200.0))
                .border_1()
                .border_color(rgba(th.outline))
                .occlude()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                    if let Some(c) = &mut this.compose
                        && c.popup == Some(Popup::Kind)
                    {
                        c.popup = None;
                        cx.notify();
                    }
                }))
                .children(items)
                .children(notch(th));
            deferred(
                div().absolute().bottom_0().left_0().child(
                    anchored()
                        .offset(point(px(0.0), px(NOTCH + 2.0)))
                        .snap_to_window_with_margin(px(8.0))
                        .child(panel),
                ),
            )
            .with_priority(2)
        });
        div()
            .id("inline-kind")
            .relative()
            .flex_none()
            .h(px(32.0))
            .pl(px(4.0))
            .pr(px(2.0))
            .flex()
            .flex_row()
            .items_center()
            .rounded(px(6.0))
            .cursor_pointer()
            .when(open, |d| d.bg(rgba(th.hover)))
            .hover(|s| s.bg(rgba(th.hover)))
            .when(!open, |d| d.tooltip(tip(label, th)))
            .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Kind, cx)))
            .child(icon(name, th.text_dim, 20.0))
            .child(icon("drop-down", th.text_dim, 18.0))
            .children(popup)
            .into_any_element()
    }

    /// The one line of a closed head: whom the reply goes to. A click
    /// opens the rows.
    pub(super) fn render_reply_summary(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let to = compose.chips.get(Field::To);
        let cc = compose.chips.get(Field::Cc);
        let bcc = compose.chips.get(Field::Bcc);
        let several = to.len() + cc.len() + bcc.len() > 1;
        let names = |chips: &[Chip]| {
            chips
                .iter()
                .map(|c| summary_label(c, several))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let mut parts = Vec::new();
        if !to.is_empty() {
            parts.push(names(to));
        }
        if !cc.is_empty() {
            parts.push(tr!("compose-summary-cc", names = names(cc)));
        }
        if !bcc.is_empty() {
            parts.push(tr!("compose-summary-bcc", names = names(bcc)));
        }
        div()
            .id("inline-summary")
            .flex_1()
            .min_w_0()
            .h(px(32.0))
            .px(px(6.0))
            .flex()
            .items_center()
            .rounded(px(6.0))
            .cursor_pointer()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", Some(6.0), th))
            .tooltip(tip(tr!("compose-edit-recipients"), th))
            .on_click(cx.listener(|this, _, window, cx| this.open_reply_header(window, cx)))
            .child(
                div()
                    .min_w_0()
                    .overflow_hidden()
                    .text_ellipsis()
                    .whitespace_nowrap()
                    .text_color(rgba(th.text))
                    .child(parts.join(", ")),
            )
            .into_any_element()
    }

    /// Writes the message again as `kind`: new recipients, subject and
    /// quoted or forwarded message; the text written so far stays.
    pub(super) fn switch_reply_kind(
        &mut self,
        kind: Kind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.popup = None;
        if compose.kind == kind || compose.kind == Kind::New {
            cx.notify();
            return;
        }
        let source = compose.source;
        let Some(view) = self.reader.as_ref().and_then(|r| r.view(source)).cloned() else {
            cx.notify();
            return;
        };
        let date = view
            .date
            .and_then(|d| format::local(d, &self.tz))
            .map(format::long_date)
            .unwrap_or_default();
        let accounts = &self.accounts;
        let is_me = |email: &str| {
            accounts
                .iter()
                .any(|a| a.address.eq_ignore_ascii_case(email))
        };
        let original = Original { view: &view, date };
        let new = draft(kind, Some(&original), is_me, None);
        let Some(compose) = &mut self.compose else {
            return;
        };
        let old = compose.kind;
        compose.kind = kind;
        compose.thread = Threading::of(kind, Some(&view));
        compose.chips.set(Field::To, &new.to);
        compose.chips.set(Field::Cc, &new.cc);
        compose.show_cc |= !new.cc.is_empty();
        let subject = new.subject.clone();
        compose
            .subject
            .update(cx, |input, cx| input.set_text(subject, cx));
        // Between replies only the recipients change.
        let answers = |k: Kind| matches!(k, Kind::Reply | Kind::ReplyAll);
        if !(answers(old) && answers(kind)) {
            let mut doc: Doc = compose.body.read(cx).doc().clone();
            if old == Kind::Forward {
                trim_forward(&mut doc.blocks);
            } else {
                compose.quote.take_from(&mut doc.blocks);
            }
            // What the new kind puts after the text, without the empty
            // line the cursor starts on.
            doc.blocks.extend(new.body.blocks.into_iter().skip(1));
            if answers(kind) {
                compose.quote = super::quote::Quote::hidden_from(trim_quote(&mut doc));
            }
            compose.body.update(cx, |editor, cx| {
                editor.set_doc(doc.clone(), doc.start(), cx)
            });
        }
        let focus = if new.to.is_empty() {
            compose.header_open = true;
            compose.to.focus_handle(cx)
        } else {
            compose.body.focus_handle(cx)
        };
        window.focus(&focus, cx);
        if old == Kind::Forward {
            compose.drop_forwarded();
        }
        self.chips_changed(Field::To, cx);
        self.chips_changed(Field::Cc, cx);
        if kind == Kind::Forward {
            self.attach_forwarded(cx);
        }
        cx.notify();
    }
}

/// The notch on top of the kind menu, pointing up at its button: a
/// border-colored triangle with a menu-colored one just inside it.
fn notch(th: &Theme) -> [AnyElement; 2] {
    let triangle = |scale: f32, inset: f32, color: u32| {
        let (w, h) = (18.0 * scale, NOTCH * scale);
        svg()
            .path("icons/notch.svg")
            .absolute()
            .left(px(16.0 - w / 2.0 + 2.0))
            .top(px(-(h - inset)))
            .w(px(w))
            .h(px(h))
            .text_color(rgba(color))
            .into_any_element()
    };
    [triangle(1.0, 0.0, th.divider), triangle(0.9, 1.0, th.menu)]
}
