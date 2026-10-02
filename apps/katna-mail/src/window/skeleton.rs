// SPDX-License-Identifier: GPL-3.0-or-later

//! The window behind the first-start pages: the parts that are there
//! before any account (the apps, Compose, the list's toolbar) as they
//! will look, and quiet placeholder shapes where folders, mail and a
//! conversation will be. It follows the reading pane, density and colors
//! picked on the Look page. Nothing in it answers clicks: the first-start
//! pages cover it with a scrim.

use std::time::Duration;

use gpui::{Animation, AnimationExt, AnyElement, Context, Div, div, prelude::*, rgba};
use katna_core::config::{Density, ReadingPane};
use katna_i18n::tr;
use katna_ui::px;

use super::apps::{APP_RAIL_WIDTH, App as RailApp};
use super::{COMPOSE_HEIGHT, COMPOSE_RADIUS, MailWindow, NAV_ROW_INSET, NAV_WIDTH, SPLIT_GAP};
use crate::theme::{Theme, fade};
use crate::widgets::{card_outline, card_shadow, icon};

/// One breath of the placeholders, dim to bright and back.
const PULSE: Duration = Duration::from_millis(1800);

/// Widths of the placeholder lines, so the rows do not look stamped.
const SENDERS: [f32; 7] = [96.0, 120.0, 84.0, 132.0, 108.0, 90.0, 116.0];
const SUBJECTS: [f32; 7] = [0.62, 0.48, 0.70, 0.55, 0.40, 0.66, 0.52];
const FOLDERS: [f32; 8] = [64.0, 72.0, 56.0, 88.0, 48.0, 60.0, 76.0, 52.0];

impl MailWindow {
    /// The window as it will be, without an account's content.
    pub(super) fn render_skeleton(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let shape = self.layout.shape;
        let reduce = cx.reduce_motion();
        let desktop = shape.is_desktop();
        let phone = shape.is_phone();
        let compact = self.config.mail.density == Density::Compact;
        let split =
            self.config.mail.reading_pane == ReadingPane::Right && shape.size.splits(shape.width);
        let margin = shape.card_margin();
        let radius = shape.card_radius();
        let outline = shape.card_outline();

        let list = card(th, radius, outline)
            .child(list_toolbar(th, phone))
            .child(breathing(
                "skeleton-rows",
                div()
                    .flex()
                    .flex_col()
                    .children((0..14).map(|i| mail_row(i, phone, compact, th))),
                reduce,
            ));
        let cards = div()
            .flex_1()
            .min_w_0()
            .h_full()
            .pr(px(margin))
            .pb(px(margin))
            .flex()
            .flex_row()
            .gap(px(SPLIT_GAP))
            .child(div().flex_1().min_w_0().h_full().child(list))
            .when(split, |d| {
                d.child(div().flex_1().min_w_0().h_full().child(
                    card(th, radius, outline).child(breathing(
                        "skeleton-reader",
                        reader(th),
                        reduce,
                    )),
                ))
            });

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_row()
                    .when(!phone, |d| d.child(rail(th)))
                    .when(desktop, |d| {
                        d.child(breathing("skeleton-folders", folders(th), reduce))
                    })
                    // A tablet shows no folders; the list keeps a gap from
                    // the rail as it does then.
                    .when(!desktop && !phone, |d| d.child(div().w(px(margin))))
                    .child(cards),
            )
            .when(phone, |d| d.child(bottom_bar(th)))
            .into_any_element()
    }

    /// The top bar's start while the first-start pages show: the folders
    /// button and the app's name, drawn but not working.
    pub(super) fn skeleton_top_start(&self, th: &Theme, titles: (f32, f32)) -> Vec<AnyElement> {
        let shape = self.layout.shape;
        let shown = 1.0 - shape.phone;
        let mut start = vec![
            div()
                .ml(px(6.0))
                .size(px(48.0))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .child(icon("folders-pane", th.text_dim, 24.0))
                .into_any_element(),
        ];
        if shown > 0.001 {
            let label = shape.title_label();
            start.push(
                div()
                    .flex_none()
                    .ml(px(super::TOP_BAR_GAP - super::BAR_ITEM_GAP))
                    .w(px(super::title_width(label, titles) * shown))
                    .overflow_hidden()
                    .opacity(shown)
                    .child(self.render_title(th, label, titles))
                    .into_any_element(),
            );
        }
        start
    }
}

/// The search box, drawn but not working.
/// On a phone it runs under the folders button, as the real one does.
pub(super) fn search_pill(th: &Theme, width: f32, phone: f32) -> AnyElement {
    div()
        .w(px(width))
        .h(px(katna_ui::motion::lerp(40.0, 48.0, phone)))
        .pl(px(katna_ui::motion::lerp(9.0, 64.0, phone)))
        .pr(px(12.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(10.0))
        .rounded_full()
        .bg(rgba(super::nav::search_fill(th, 0.0)))
        .border_1()
        .border_color(rgba(super::nav::search_edge(th, 0.0)))
        .text_size(px(16.0))
        .text_color(rgba(th.text_faint))
        .when(phone < 0.5, |d| d.child(icon("search", th.text_dim, 22.0)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .child(tr!("search-mail")),
        )
        .into_any_element()
}

/// A shape that stands in for text or a picture.
fn bone(th: &Theme) -> Div {
    div()
        .flex_none()
        .rounded_full()
        .bg(rgba(fade(th.text, 0.08)))
}

/// Placeholders breathe gently, as content that is on its way.
fn breathing(id: &'static str, el: Div, reduce: bool) -> AnyElement {
    if reduce {
        return el.into_any_element();
    }
    el.with_animation(id, Animation::new(PULSE).repeat(), |el, t| {
        // Dim to bright and back, softly at both ends.
        let wave = 0.5 - 0.5 * (t * std::f32::consts::TAU).cos();
        el.opacity(0.55 + 0.45 * wave)
    })
    .into_any_element()
}

fn card(th: &Theme, radius: f32, outline: f32) -> Div {
    div()
        .relative()
        .size_full()
        .flex()
        .flex_col()
        .overflow_hidden()
        .rounded(px(radius))
        .map(|d| crate::widgets::pane(d, th.pane(), th.surface, radius))
        .shadow(card_shadow(th, super::SHADOW_REST))
        .children(card_outline(th, radius, outline))
}

/// The apps: they are there with or without an account.
fn rail(th: &Theme) -> AnyElement {
    div()
        .flex_none()
        .w(px(APP_RAIL_WIDTH))
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .child(
            // Compose, where it sits while the folders show.
            div().h(px(super::COMPOSE_NAV_ROOM - COMPOSE_HEIGHT - 8.0)),
        )
        .children(RailApp::ALL.into_iter().map(|app| {
            let on = app == RailApp::Mail;
            div()
                .w(px(APP_RAIL_WIDTH))
                .pt(px(4.0))
                .pb(px(8.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .w(px(56.0))
                        .h(px(32.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .when(on, |d| d.bg(rgba(th.nav_selected)))
                        .child(icon(
                            app.icon(),
                            if on {
                                th.nav_selected_text
                            } else {
                                th.text_dim
                            },
                            22.0,
                        )),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .line_height(px(16.0))
                        .text_color(rgba(if on { th.text } else { th.text_dim }))
                        .child(app.label()),
                )
        }))
        .child(div().flex_1())
        .child(
            div()
                .pb(px(20.0))
                .child(icon("settings", th.text_dim, 22.0)),
        )
        .into_any_element()
}

/// Compose, then the folders an account brings, as placeholders.
fn folders(th: &Theme) -> Div {
    div()
        .flex_none()
        .w(px(NAV_WIDTH))
        .h_full()
        .pr(px(NAV_ROW_INSET))
        .flex()
        .flex_col()
        .child(
            div().pt(px(super::COMPOSE_TOP)).pb(px(16.0)).child(
                div()
                    .w(px(142.0))
                    .h(px(COMPOSE_HEIGHT))
                    .px(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .rounded(px(COMPOSE_RADIUS))
                    .bg(rgba(th.compose))
                    .text_size(px(14.0))
                    .text_color(rgba(th.compose_text))
                    .child(icon("compose", th.compose_text, 24.0))
                    .child(tr!("compose")),
            ),
        )
        .child(
            div()
                .pl(px(28.0))
                .pb(px(12.0))
                .child(bone(th).w(px(84.0)).h(px(10.0))),
        )
        .children(FOLDERS.iter().enumerate().map(|(i, width)| {
            div()
                .h(px(32.0))
                .pl(px(28.0))
                .pr(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(18.0))
                .child(bone(th).size(px(18.0)))
                .child(bone(th).w(px(*width)).h(px(10.0)))
                .child(div().flex_1())
                .when(i == 0, |d| d.child(bone(th).w(px(20.0)).h(px(10.0))))
        }))
}

/// The list's toolbar: select, refresh and more, there before any mail.
fn list_toolbar(th: &Theme, phone: bool) -> AnyElement {
    div()
        .flex_none()
        .h(px(if phone { 0.0 } else { 48.0 }))
        .px(px(16.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(20.0))
        .when(!phone, |d| {
            d.child(icon("checkbox", th.text_faint, 20.0))
                .child(icon("refresh", th.text_faint, 20.0))
                .child(icon("more", th.text_faint, 20.0))
        })
        .into_any_element()
}

/// A line of the list: tick, star, sender, subject and date on a wider
/// window; a picture and two lines on a phone.
fn mail_row(i: usize, phone: bool, compact: bool, th: &Theme) -> AnyElement {
    let sender = SENDERS[i % SENDERS.len()];
    let subject = SUBJECTS[i % SUBJECTS.len()];
    if phone {
        return div()
            .h(px(if compact { 64.0 } else { 76.0 }))
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .child(bone(th).size(px(40.0)))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .child(bone(th).w(px(sender)).h(px(10.0)))
                            .child(div().flex_1())
                            .child(bone(th).w(px(36.0)).h(px(8.0))),
                    )
                    .child(
                        div()
                            .w(gpui::relative(subject + 0.2))
                            .child(bone(th).w_full().h(px(8.0))),
                    ),
            )
            .into_any_element();
    }
    div()
        .h(px(if compact { 32.0 } else { 40.0 }))
        .px(px(16.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(16.0))
        .border_t_1()
        .border_color(rgba(super::list::row_line(th)))
        .child(bone(th).size(px(14.0)).rounded(px(3.0)))
        .child(bone(th).size(px(14.0)))
        .child(div().w(px(150.0)).child(bone(th).w(px(sender)).h(px(10.0))))
        .child(
            div().flex_1().min_w_0().child(
                div()
                    .w(gpui::relative(subject))
                    .child(bone(th).w_full().h(px(10.0))),
            ),
        )
        .child(bone(th).w(px(40.0)).h(px(10.0)))
        .into_any_element()
}

/// A conversation: a subject, who sent it, and lines of text.
fn reader(th: &Theme) -> Div {
    let line = |w: f32| {
        div()
            .w(gpui::relative(w))
            .child(bone(th).w_full().h(px(10.0)))
    };
    div()
        .p(px(24.0))
        .flex()
        .flex_col()
        .gap(px(14.0))
        .child(
            div()
                .w(gpui::relative(0.55))
                .pb(px(12.0))
                .child(bone(th).w_full().h(px(18.0))),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .child(bone(th).size(px(40.0)))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(bone(th).w(px(140.0)).h(px(10.0)))
                        .child(bone(th).w(px(96.0)).h(px(8.0))),
                ),
        )
        .child(div().h(px(12.0)))
        .child(line(0.92))
        .child(line(0.86))
        .child(line(0.9))
        .child(line(0.6))
        .child(div().h(px(8.0)))
        .child(line(0.88))
        .child(line(0.74))
}

/// The apps along the bottom of a phone.
fn bottom_bar(th: &Theme) -> AnyElement {
    div()
        .flex_none()
        .h(px(super::layout::BOTTOM_BAR_HEIGHT))
        .flex()
        .flex_row()
        .children(RailApp::ALL.into_iter().map(|app| {
            let on = app == RailApp::Mail;
            div()
                .flex_1()
                .min_w_0()
                .pt(px(12.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .w(px(if on { 56.0 } else { 32.0 }))
                        .h(px(32.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .when(on, |d| d.bg(rgba(th.nav_selected)))
                        .child(icon(
                            app.icon(),
                            if on {
                                th.nav_selected_text
                            } else {
                                th.text_dim
                            },
                            22.0,
                        )),
                )
                .child(
                    div()
                        .max_w_full()
                        .truncate()
                        .text_size(px(12.0))
                        .text_color(rgba(if on { th.text } else { th.text_dim }))
                        .child(app.label()),
                )
        }))
        .into_any_element()
}
