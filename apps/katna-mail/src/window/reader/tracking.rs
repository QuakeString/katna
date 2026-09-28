// SPDX-License-Identifier: GPL-3.0-or-later

//! Open and click tracking and read receipts in the reading view: beside
//! the star of a tracked message, an eye whose popover lists who opened
//! it or followed a link; above a read receipt, who read what
//! (`docs/ARCHITECTURE.md` §16.1).

use std::cell::Cell;
use std::f32::consts::PI;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnyElement, Bounds, Context, MouseButton, Pixels, Size, Task, Transformation, anchored, canvas,
    deferred, div, point, prelude::*, radians, rgba, svg,
};
use katna_i18n::tr;
use katna_store::RecipientActivity;
use katna_ui::px;
use katna_ui::unpx;

use super::Part;
use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, icon_button_colored, raised};
use crate::window::MailWindow;

/// Where an eye button was last drawn, and the window's size then.
pub(super) type Anchor = Rc<Cell<Option<(Bounds<Pixels>, Size<Pixels>)>>>;

/// The open popover: which message's, and whether the pointer is over
/// the eye or the popover, which keeps it open.
pub(in crate::window) struct Seen {
    ix: usize,
    over_eye: bool,
    over_popover: bool,
    /// Closes it a moment after the pointer leaves both, so it can cross
    /// the gap between them.
    closing: Option<Task<()>>,
}

const WIDTH: f32 = 340.0;
const PAD: f32 = 12.0;
const GAP: f32 = 8.0;
const LINE: f32 = 18.0;
const RADIUS: f32 = 12.0;
/// The notch's length out of the popover, and the popover's distance from
/// the eye and from the window's edges.
const NOTCH: f32 = 10.0;
const SPACE: f32 = 2.0;
const MARGIN: f32 = 8.0;
/// How long the popover stays after the pointer leaves it or the eye.
const LINGER: Duration = Duration::from_millis(250);

impl MailWindow {
    /// The line above a read receipt: who read which message.
    pub(super) fn tracking_banner(&self, part: &Part, th: &Theme) -> Option<AnyElement> {
        let Some(Some(receipt)) = &part.receipt else {
            return None;
        };
        let who = receipt.who();
        let (color, text) = if receipt.displayed {
            (green(th), tr!("tracking-receipt-displayed", who = who))
        } else {
            (th.text_faint, tr!("tracking-receipt-other", who = who))
        };
        Some(
            div()
                .mt(px(12.0))
                .px(px(12.0))
                .py(px(8.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(th.divider))
                .text_size(px(13.0))
                .line_height(px(LINE))
                .child(line("read-receipt", color, text, th))
                .into_any_element(),
        )
    }

    /// The eye beside the star of a message sent with tracking, or one a
    /// read receipt came back for; hovering or clicking it opens who has
    /// seen it.
    pub(super) fn seen_eye(
        &self,
        ix: usize,
        part: &Part,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let reader = self.reader.as_ref()?;
        let receipts = reader.receipts_for(part).iter().any(|r| r.displayed);
        if part.activity.is_none() && !receipts {
            return None;
        }
        let seen = receipts
            || part
                .activity
                .iter()
                .flat_map(|a| &a.recipients)
                .any(|r| r.opens > 0 || r.clicks > 0);
        let anchor = part.eye.clone();
        Some(
            div()
                .relative()
                .child(
                    icon_button_colored(
                        ("part-seen", ix),
                        "eye",
                        20.0,
                        if seen { th.accent } else { th.text_faint },
                        th,
                    )
                    .size(px(32.0))
                    .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                        this.hover_seen(ix, Some(*hovered), None, cx)
                    }))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.hover_seen(ix, Some(true), None, cx);
                    })),
                )
                .child(
                    canvas(
                        move |bounds, window, _| anchor.set(Some((bounds, window.viewport_size()))),
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full(),
                )
                .into_any_element(),
        )
    }

    /// The pointer went over or off the eye (`eye`) or the popover
    /// (`popover`) of message `ix`.
    fn hover_seen(
        &mut self,
        ix: usize,
        eye: Option<bool>,
        popover: Option<bool>,
        cx: &mut Context<Self>,
    ) {
        let Some(reader) = self.reader.as_mut() else {
            return;
        };
        let seen = match &mut reader.seen {
            Some(seen) if seen.ix == ix => seen,
            // Leaving an eye whose popover isn't open.
            _ if eye != Some(true) => return,
            other => other.insert(Seen {
                ix,
                over_eye: false,
                over_popover: false,
                closing: None,
            }),
        };
        if let Some(eye) = eye {
            seen.over_eye = eye;
        }
        if let Some(popover) = popover {
            seen.over_popover = popover;
        }
        seen.closing = (!seen.over_eye && !seen.over_popover).then(|| {
            cx.spawn(async move |this, cx| {
                cx.background_executor().timer(LINGER).await;
                this.update(cx, |this, cx| {
                    let Some(reader) = this.reader.as_mut() else {
                        return;
                    };
                    if reader
                        .seen
                        .as_ref()
                        .is_some_and(|s| s.ix == ix && !s.over_eye && !s.over_popover)
                    {
                        reader.seen = None;
                        cx.notify();
                    }
                })
                .ok();
            })
        });
        cx.notify();
    }

    /// Closes the popover of who has seen a message, if it is open.
    pub(in crate::window) fn close_seen(&mut self, cx: &mut Context<Self>) -> bool {
        let closed = self
            .reader
            .as_mut()
            .is_some_and(|r| r.seen.take().is_some());
        if closed {
            cx.notify();
        }
        closed
    }

    /// Under the eye (or over it, near the window's bottom), with a notch
    /// pointing at it: a line for each person who opened the message or
    /// followed a link, and for each read receipt; or that nobody has yet.
    pub(super) fn seen_popover(
        &self,
        ix: usize,
        part: &Part,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let reader = self.reader.as_ref()?;
        if reader.seen.as_ref()?.ix != ix {
            return None;
        }
        let (eye, viewport) = part.eye.get()?;
        let mut lines: Vec<(&'static str, u32, String)> = part
            .activity
            .iter()
            .flat_map(|a| &a.recipients)
            .filter_map(|r| self.recipient_line(r, th))
            .collect();
        for receipt in reader
            .receipts_for(part)
            .into_iter()
            .filter(|r| r.displayed)
        {
            lines.push((
                "read-receipt",
                green(th),
                tr!("tracking-receipt", who = receipt.who()),
            ));
        }
        if lines.is_empty() {
            lines.push(("eye", th.text_faint, tr!("tracking-seen-none")));
        }

        // Its height, guessed from the lines' lengths, only picks the side.
        let text_width = WIDTH - 2.0 * PAD - 18.0 - GAP;
        let rows: f32 = lines
            .iter()
            .map(|(_, _, text)| {
                (text.chars().count() as f32 * 7.0 / text_width)
                    .ceil()
                    .max(1.0)
            })
            .sum();
        let height = 2.0 * PAD + rows * LINE + (lines.len() - 1) as f32 * GAP;
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let (left, top) = (unpx(eye.origin.x), unpx(eye.origin.y));
        let bottom = top + unpx(eye.size.height);
        let middle = left + unpx(eye.size.width) / 2.0;
        let away = NOTCH + SPACE;
        let below = bottom + away + height <= vh - MARGIN || top - away - height < MARGIN;
        let x = (middle - WIDTH / 2.0).min(vw - WIDTH - MARGIN).max(MARGIN);
        let along = (middle - x).clamp(RADIUS + NOTCH, WIDTH - RADIUS - NOTCH);

        let popover = div()
            .id(("seen-popover", ix))
            .absolute()
            .left(px(x))
            .map(|d| {
                if below {
                    d.top(px(bottom + away))
                } else {
                    d.bottom(px(vh - top + away))
                }
            })
            .w(px(WIDTH))
            .p(px(PAD))
            .flex()
            .flex_col()
            .gap(px(GAP))
            .border_1()
            .border_color(rgba(th.divider))
            .map(|d| raised(d, th, RADIUS, 4.0))
            .text_size(px(13.0))
            .line_height(px(LINE))
            .occlude()
            .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                this.hover_seen(ix, None, Some(*hovered), cx)
            }))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.close_seen(cx);
            }))
            .children(
                lines
                    .into_iter()
                    .map(|(name, color, text)| line(name, color, text, th)),
            )
            .children(notch(below, along, th));
        let layer = div().relative().w(px(vw)).h(px(vh)).child(popover);
        Some(
            deferred(anchored().position(point(px(0.0), px(0.0))).child(layer))
                .with_priority(3)
                .into_any_element(),
        )
    }

    /// What one recipient did, or `None` when they haven't opened it.
    fn recipient_line(
        &self,
        recipient: &RecipientActivity,
        th: &Theme,
    ) -> Option<(&'static str, u32, String)> {
        let who = recipient
            .name
            .clone()
            .filter(|n| !n.trim().is_empty())
            .unwrap_or_else(|| recipient.email.clone());
        let now = jiff::Timestamp::now().as_second();
        let when = recipient
            .last
            .and_then(|at| format::local(at / 1000, &self.tz))
            .zip(format::local(now, &self.tz))
            .map(|(at, now)| format::list_date(at, now))
            .unwrap_or_default();
        Some(if recipient.clicks > 0 && recipient.opens > 0 {
            let text = tr!(
                "tracking-opens-clicks",
                who = who,
                opens = recipient.opens,
                clicks = recipient.clicks,
                when = when
            );
            ("link", th.accent, text)
        } else if recipient.clicks > 0 {
            // Pictures turned off: the link shows it was read.
            let text = tr!(
                "tracking-clicked",
                who = who,
                clicks = recipient.clicks,
                when = when
            );
            ("link", th.accent, text)
        } else if recipient.opens > 0 {
            let text = tr!(
                "tracking-opened",
                who = who,
                count = recipient.opens,
                when = when
            );
            ("eye", th.accent, text)
        } else if recipient.maybe_opens > 0 {
            (
                "eye",
                th.text_faint,
                tr!("tracking-maybe-opened", who = who),
            )
        } else {
            return None;
        })
    }
}

fn green(th: &Theme) -> u32 {
    if th.dark { 0x81c995ff } else { 0x188038ff }
}

/// An icon and its sentence.
fn line(name: &'static str, color: u32, text: String, th: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(GAP))
        .child(icon(name, color, 18.0))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_color(rgba(if color == th.text_faint {
                    th.text_dim
                } else {
                    th.text
                }))
                .child(text),
        )
}

/// The notch on the popover's edge facing the eye, `along` from its left:
/// a border-colored triangle with a popover-colored one just inside it,
/// covering the border where they meet.
fn notch(below: bool, along: f32, th: &Theme) -> [AnyElement; 2] {
    let triangle = |scale: f32, inset: f32, color: u32| {
        let (w, h) = (20.0 * scale, NOTCH * scale);
        // From the edge to the triangle's far side.
        let out = -(h - inset);
        svg()
            .path("icons/notch.svg")
            .absolute()
            .left(px(along - w / 2.0))
            .map(|d| {
                if below {
                    d.top(px(out))
                } else {
                    d.bottom(px(out))
                }
            })
            .w(px(w))
            .h(px(h))
            .text_color(rgba(color))
            .when(!below, |d| {
                d.with_transformation(Transformation::rotate(radians(PI)))
            })
            .into_any_element()
    };
    [triangle(1.0, 0.0, th.divider), triangle(0.9, 1.0, th.menu)]
}
