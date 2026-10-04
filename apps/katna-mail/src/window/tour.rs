// SPDX-License-Identifier: GPL-3.0-or-later

//! The tour of the window: the page dims around one part at a time (Compose,
//! the search box, the menu button, the apps, the tabs, the list, quick
//! settings, the account) and a card beside it says what it is for. It can
//! be skipped at any point; quick settings starts it again.
//!
//! The parts mark themselves with [`MailWindow::tour_mark`], which records
//! where they were drawn.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gpui::{
    AnyElement, Bounds, Context, FocusHandle, FontWeight, KeyDownEvent, MouseButton, PathBuilder,
    Pixels, Window, canvas, deferred, div, point, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::anchored;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::unpx;

use super::add_account::text_button;
use super::{MailWindow, PANEL_RADIUS};
use crate::theme::{Theme, fade};
use crate::widgets::{ScaledEdge, elevation, filled_button};

const CARD_WIDTH: f32 = 340.0;
/// Room kept around a lit part.
const PAD: f32 = 6.0;
/// Corners of the lit box.
const RADIUS: f32 = 12.0;

/// A part of the window the tour shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Spot {
    Compose,
    Search,
    Menu,
    Apps,
    Tabs,
    List,
    Settings,
    Account,
}

/// The stops, in order: the part, and the message ids of a title and of
/// what to say.
const STOPS: [(Spot, &str, &str); 8] = [
    (Spot::Compose, "tour-compose-title", "tour-compose-text"),
    (Spot::Search, "tour-search-title", "tour-search-text"),
    (Spot::Menu, "tour-menu-title", "tour-menu-text"),
    (Spot::Apps, "tour-apps-title", "tour-apps-text"),
    (Spot::Tabs, "tour-tabs-title", "tour-tabs-text"),
    (Spot::List, "tour-list-title", "tour-list-text"),
    (Spot::Settings, "tour-settings-title", "tour-settings-text"),
    (Spot::Account, "tour-account-title", "tour-account-text"),
];

/// Where each part is drawn in this frame, in window coordinates.
pub(super) type Marks = Rc<RefCell<HashMap<Spot, Bounds<Pixels>>>>;

pub(super) struct Tour {
    /// The stop shown, or `None` for the opening card.
    stop: Option<usize>,
    /// The lit box moves from here to the current part.
    from: Option<[f32; 4]>,
    spring: Spring,
    focus: FocusHandle,
    /// Waiting for the parts to be drawn once before the first stop.
    waiting: bool,
}

impl MailWindow {
    /// Records where `spot` is drawn: put it among the children of a
    /// `relative()` element.
    pub(super) fn tour_mark(&self, spot: Spot) -> impl IntoElement + use<> {
        let marks = self.tour_marks.clone();
        canvas(
            move |bounds, _, _| {
                marks.borrow_mut().insert(spot, bounds);
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full()
    }

    /// Keeps where the parts were in the last frame and starts collecting
    /// this one's, so a part no longer drawn drops out. Called first thing
    /// in each frame.
    pub(super) fn tour_new_frame(&mut self) {
        self.tour_seen = std::mem::take(&mut *self.tour_marks.borrow_mut());
    }

    /// Starts the tour, with the opening card if `intro`.
    pub(super) fn start_tour(&mut self, intro: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_open = false;
        self.account_menu = false;
        self.close_context_menu(cx);
        self.close_nav_menu(cx);
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        self.tour = Some(Tour {
            stop: None,
            from: None,
            spring: Spring::new(motion::SLIDE, 1.0),
            focus,
            waiting: !intro,
        });
        cx.notify();
    }

    /// Moves `by` stops, skipping parts not on screen; past the last one,
    /// the tour ends.
    fn tour_step(&mut self, by: isize, cx: &mut Context<Self>) {
        let marks = self.tour_seen.clone();
        let Some(tour) = &mut self.tour else {
            return;
        };
        let mut at = tour.stop.map_or(-1, |s| s as isize);
        loop {
            at += by;
            if at < 0 {
                return;
            }
            let Some((spot, ..)) = STOPS.get(at as usize) else {
                self.end_tour(cx);
                return;
            };
            if marks.contains_key(spot) {
                break;
            }
        }
        tour.from = tour.stop.and_then(|s| lit_box(&marks, STOPS[s].0));
        tour.stop = Some(at as usize);
        tour.spring.snap(0.0);
        tour.spring.set(1.0);
        cx.notify();
    }

    fn end_tour(&mut self, cx: &mut Context<Self>) {
        self.tour = None;
        self.config.onboarding.done = true;
        self.save_config();
        cx.notify();
    }

    fn tour_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let intro = self.tour.as_ref().is_some_and(|t| t.stop.is_none());
        match event.keystroke.key.as_str() {
            "escape" => self.end_tour(cx),
            "right" | "enter" | "space" => self.tour_step(1, cx),
            "left" if !intro => self.tour_step(-1, cx),
            _ => return,
        }
        if self.tour.is_none() {
            window.focus(&self.list_focus, cx);
        }
        cx.stop_propagation();
    }

    pub(super) fn render_tour(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let reduce = cx.reduce_motion();
        if self.tour.as_ref()?.waiting {
            if self.tour_seen.is_empty() {
                window.request_animation_frame();
            } else if let Some(tour) = &mut self.tour {
                tour.waiting = false;
                self.tour_step(1, cx);
            }
        }
        let waiting = self.tour.as_ref()?.waiting;
        let tour = self.tour.as_mut()?;
        let t = tour.spring.tick(window, reduce).clamp(0.0, 1.0);
        let stop = tour.stop;
        let from = tour.from;
        let focus = tour.focus.clone();
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let marks = self.tour_seen.clone();
        let scrim = fade(0x0000_00ff, 0.55);

        let lit = stop.and_then(|s| lit_box(&marks, STOPS[s].0)).map(|to| {
            let from = from.unwrap_or(to);
            [0, 1, 2, 3].map(|i| lerp(from[i], to[i], t))
        });
        let count = STOPS
            .iter()
            .filter(|(spot, ..)| marks.contains_key(spot))
            .count();
        let number = stop.map_or(0, |s| {
            STOPS[..=s]
                .iter()
                .filter(|(spot, ..)| marks.contains_key(spot))
                .count()
        });
        let last = stop.is_some_and(|s| {
            STOPS[s + 1..]
                .iter()
                .all(|(spot, ..)| !marks.contains_key(spot))
        });

        // The page, dimmed everywhere but the lit part: the window with the
        // part's rounded box cut out (lyon fills even-odd).
        let dim = canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let mut path = PathBuilder::fill();
                let (o, size) = (bounds.origin, bounds.size);
                path.add_polygon(
                    &[
                        o,
                        point(o.x + size.width, o.y),
                        point(o.x + size.width, o.y + size.height),
                        point(o.x, o.y + size.height),
                    ],
                    true,
                );
                if let Some([x, y, w, h]) = lit {
                    let r = RADIUS.min(w / 2.0).min(h / 2.0);
                    let p = |x: f32, y: f32| point(o.x + px(x), o.y + px(y));
                    path.move_to(p(x + r, y));
                    path.line_to(p(x + w - r, y));
                    path.curve_to(p(x + w, y + r), p(x + w, y));
                    path.line_to(p(x + w, y + h - r));
                    path.curve_to(p(x + w - r, y + h), p(x + w, y + h));
                    path.line_to(p(x + r, y + h));
                    path.curve_to(p(x, y + h - r), p(x, y + h));
                    path.line_to(p(x, y + r));
                    path.curve_to(p(x + r, y), p(x, y));
                    path.close();
                }
                if let Ok(path) = path.build() {
                    window.paint_path(path, rgba(scrim));
                }
            },
        )
        .absolute()
        .size_full();
        let ring = lit.map(|[x, y, w, h]| {
            div()
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(w))
                .h(px(h))
                .rounded(px(RADIUS))
                .border_px(2.0)
                .border_color(rgba(th.accent))
        });

        let (title, text) = match stop {
            Some(s) => (tr!(STOPS[s].1), tr!(STOPS[s].2)),
            None => (tr!("tour-welcome-title"), tr!("tour-welcome-text")),
        };
        let buttons = match stop {
            None => div()
                .flex()
                .flex_row()
                .items_center()
                .child(div().flex_1())
                .child(
                    text_button("tour-skip", tr!("tour-not-now"), th)
                        .on_click(cx.listener(|this, _, _, cx| this.end_tour(cx))),
                )
                .child(
                    filled_button("tour-start", tr!("tour-start"), th)
                        .ml(px(8.0))
                        .on_click(cx.listener(|this, _, _, cx| this.tour_step(1, cx))),
                ),
            Some(_) => div()
                .flex()
                .flex_row()
                .items_center()
                .child(
                    text_button(
                        "tour-end",
                        if last {
                            tr!("tour-close")
                        } else {
                            tr!("tour-skip")
                        },
                        th,
                    )
                    .on_click(cx.listener(|this, _, _, cx| this.end_tour(cx))),
                )
                .child(div().flex_1())
                .when(number > 1, |d| {
                    d.child(
                        text_button("tour-back", tr!("tour-back"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.tour_step(-1, cx))),
                    )
                })
                .child(
                    filled_button(
                        "tour-next",
                        if last {
                            tr!("tour-done")
                        } else {
                            tr!("tour-next")
                        },
                        th,
                    )
                    .ml(px(8.0))
                    .on_click(cx.listener(|this, _, _, cx| this.tour_step(1, cx))),
                ),
        };
        let card = div()
            .w(px(CARD_WIDTH))
            .p(px(20.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .rounded(px(PANEL_RADIUS))
            .map(|d| crate::widgets::frosted(d, th, th.surface, PANEL_RADIUS))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 4.0))
            .when(stop.is_some(), |d| {
                d.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.accent))
                        .child(tr!("tour-step", step = number, total = count)),
                )
            })
            .child(div().text_size(px(18.0)).line_height(px(24.0)).child(title))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(th.text_dim))
                    .child(text),
            )
            .child(div().pt(px(8.0)).child(buttons));
        let (cx_, cy) = card_place(lit, vw, vh);

        let layer = div()
            .id("tour")
            .track_focus(&focus)
            .on_key_down(cx.listener(Self::tour_key))
            .relative()
            .w(px(vw))
            .h(px(vh))
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
            .child(dim)
            .children(ring)
            .when(!waiting, |d| {
                d.child(
                    div()
                        .absolute()
                        .left(px(cx_))
                        .top(px(cy))
                        .opacity(if stop.is_some() { t.max(0.3) } else { 1.0 })
                        .child(card),
                )
            });
        Some(
            deferred(anchored().position(point(px(0.0), px(0.0))).child(layer))
                .with_priority(6)
                .into_any_element(),
        )
    }
}

/// The lit box of `spot`, with room around it: x, y, width and height.
fn lit_box(marks: &HashMap<Spot, Bounds<Pixels>>, spot: Spot) -> Option<[f32; 4]> {
    let b = marks.get(&spot)?;
    Some([
        unpx(b.origin.x) - PAD,
        unpx(b.origin.y) - PAD,
        unpx(b.size.width) + 2.0 * PAD,
        unpx(b.size.height) + 2.0 * PAD,
    ])
}

/// Where the card goes: below the lit part if there is room, else above;
/// beside tall parts; in the middle when nothing is lit.
fn card_place(lit: Option<[f32; 4]>, vw: f32, vh: f32) -> (f32, f32) {
    // A little more than the card is tall.
    const CARD_HEIGHT: f32 = 230.0;
    const GAP: f32 = 12.0;
    let clamp_x = |x: f32| x.clamp(16.0, (vw - CARD_WIDTH - 16.0).max(16.0));
    let clamp_y = |y: f32| y.clamp(16.0, (vh - CARD_HEIGHT - 16.0).max(16.0));
    let Some([x, y, w, h]) = lit else {
        return (
            clamp_x((vw - CARD_WIDTH) / 2.0),
            clamp_y((vh - CARD_HEIGHT) / 2.0),
        );
    };
    if h > vh / 2.0 {
        let middle = clamp_y(y + h / 2.0 - CARD_HEIGHT / 2.0);
        if vw - (x + w) > CARD_WIDTH + 2.0 * GAP {
            return (x + w + GAP, middle);
        }
        if x > CARD_WIDTH + 2.0 * GAP {
            return (x - CARD_WIDTH - GAP, middle);
        }
        return (clamp_x(x + w / 2.0 - CARD_WIDTH / 2.0), middle);
    }
    let across = clamp_x(x + w / 2.0 - CARD_WIDTH / 2.0);
    if y + h + GAP + CARD_HEIGHT < vh {
        (across, y + h + GAP)
    } else {
        (across, clamp_y(y - GAP - CARD_HEIGHT))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_card_sits_below_small_parts_and_beside_tall_ones() {
        // The search box near the top: below it.
        let (_, y) = card_place(Some([340.0, 40.0, 720.0, 48.0]), 1400.0, 900.0);
        assert_eq!(y, 100.0);
        // The app rail down the left side: to its right.
        let (x, _) = card_place(Some([0.0, 90.0, 72.0, 800.0]), 1400.0, 900.0);
        assert_eq!(x, 84.0);
        // Nothing lit: in the middle.
        let (x, _) = card_place(None, 1400.0, 900.0);
        assert_eq!(x, 530.0);
    }

    #[test]
    fn a_part_at_the_bottom_gets_the_card_above() {
        let (_, y) = card_place(Some([100.0, 820.0, 200.0, 40.0]), 1400.0, 900.0);
        assert_eq!(y, 820.0 - 12.0 - 230.0);
    }
}
