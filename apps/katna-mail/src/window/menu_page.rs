// SPDX-License-Identifier: GPL-3.0-or-later

//! Pages of the account card: the card itself, the application menu
//! behind its ☰ button and the language list behind its flag. Going from
//! one to another turns the page in place: the new page slides in from the
//! right while it fades in (from the left going back), as the date picker
//! does in the snooze menu, and the card's height eases to the new page's.

use crate::widgets::Tip as _;
use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    Animation, AnimationExt, AnyElement, Context, FontWeight, MouseButton, Stateful, Styled,
    canvas, div, ease_out_quint, prelude::*,
};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{duration, space, text};

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::icon_button;

/// How long a page takes to slide in.
const SLIDE: Duration = duration::BASE;
/// How far it slides.
const SLIDE_BY: f32 = space::S8;
/// How long a card opened from scratch takes to drop in.
const OPEN: Duration = Duration::from_millis(180);

/// The last page turn of the account card.
#[derive(Default)]
pub(super) struct MenuPage {
    /// Counts the turns, so each starts its motion afresh.
    seq: usize,
    /// The turn went back.
    back: bool,
    /// The card's height before the turn.
    from: f32,
    turned_at: Option<Instant>,
    /// The height of the page showing, measured as it is laid out.
    height: Rc<Cell<f32>>,
}

/// How a card's page comes in this frame.
#[derive(Clone, Copy)]
pub(super) struct PageMotion {
    seq: usize,
    /// A turn is under way, going back if `Some(true)`; else the card
    /// opened from scratch.
    turn: Option<bool>,
    from: f32,
    reduce: bool,
}

impl MailWindow {
    /// Turns the account card's page, forward or `back`.
    pub(super) fn turn_menu_page(&mut self, back: bool, cx: &mut Context<Self>) {
        let page = &mut self.menu_page;
        page.seq = page.seq.wrapping_add(1);
        page.back = back;
        page.from = page.height.get();
        page.turned_at = Some(Instant::now());
        cx.notify();
    }

    /// How the account card's page comes in this frame.
    pub(super) fn page_motion(&self, cx: &Context<Self>) -> PageMotion {
        let page = &self.menu_page;
        // A little longer than the slide, so it ends before the card
        // goes back to how it opens.
        let live = katna_ui::motion::time(SLIDE) + Duration::from_millis(150);
        let turning = page.from > 0.0 && page.turned_at.is_some_and(|at| at.elapsed() < live);
        PageMotion {
            seq: page.seq,
            turn: turning.then_some(page.back),
            from: page.from,
            reduce: cx.reduce_motion(),
        }
    }

    /// Escape on a page of the account card: back to the card. Returns
    /// whether it went back.
    pub(super) fn menu_page_back(&mut self, cx: &mut Context<Self>) -> bool {
        if self.app_menu.take().is_some() {
            self.turn_menu_page(true, cx);
            return true;
        }
        false
    }

    /// The account card's `card`, with `content` as its page: the card
    /// drops in when it opens, and on a page turn the page slides in while
    /// the card's height eases from the last page's. `natural` is the
    /// page's height when the card sets it, else it is measured; `pad` is
    /// the card's padding above and below the page.
    pub(super) fn menu_page_card(
        &self,
        name: &'static str,
        card: Stateful<gpui::Div>,
        content: gpui::Div,
        natural: Option<f32>,
        pad: f32,
        motion: PageMotion,
    ) -> AnyElement {
        let cell = self.menu_page.height.clone();
        let content = match natural {
            Some(height) => {
                cell.set(height);
                content
            }
            // The page's own height, so the next turn eases from it.
            None => content.flex_none().relative().child(
                canvas(
                    move |bounds, _, _| cell.set(bounds.size.height.as_f32() + 2.0 * pad),
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            ),
        };
        let measured = self.menu_page.height.clone();
        let PageMotion {
            seq,
            turn,
            from,
            reduce,
        } = motion;
        let by = if reduce { 0.0 } else { SLIDE_BY };
        let content = content.with_animation(
            (name, seq),
            Animation::new(katna_ui::motion::time(SLIDE)).with_easing(ease_out_quint()),
            move |el, t| match turn {
                Some(back) => {
                    let by = if back { -by } else { by };
                    el.relative().left(px(by * (1.0 - t))).opacity(t)
                }
                None => el,
            },
        );
        card.child(content)
            .with_animation(
                (name, seq.wrapping_add(1 << 20)),
                Animation::new(katna_ui::motion::time(if turn.is_some() {
                    SLIDE
                } else {
                    OPEN
                }))
                .with_easing(ease_out_quint()),
                move |el, t| match turn {
                    // Clipped to its height while that eases, so the
                    // incoming page never spills out.
                    Some(_) if t < 1.0 => {
                        let to = natural.unwrap_or_else(|| measured.get());
                        el.h(px(from + (to - from) * t)).overflow_hidden()
                    }
                    Some(_) => el,
                    None => el.opacity(t).mt(px(-8.0 * (1.0 - t))),
                },
            )
            .into_any_element()
    }
}

/// A page's header: its `title` on the left and, where the card's ☰
/// button is, a button `on_back` that goes back to the card.
pub(super) fn page_header(
    id: &'static str,
    title: String,
    th: &Theme,
    on_back: impl Fn(&mut MailWindow, &mut Context<MailWindow>) + 'static,
    cx: &mut Context<MailWindow>,
) -> AnyElement {
    div()
        .flex_none()
        .h(px(space::S8))
        .pl(px(space::S5 - space::S2))
        .pr(px(space::S2))
        .flex()
        .flex_row()
        .items_center()
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_size(px(text::SUBTITLE))
                .font_weight(FontWeight::MEDIUM)
                .child(title),
        )
        .child(
            icon_button(id, "back", 22.0, th)
                .tip(tr!("app-menu-back"), th)
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    on_back(this, cx);
                })),
        )
        .into_any_element()
}
