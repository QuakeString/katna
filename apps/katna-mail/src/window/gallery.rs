// SPDX-License-Identifier: GPL-3.0-or-later

//! The Gallery, in development builds only (`katna-mail --page gallery`):
//! every shared control side by side in light and dark colors, so a change
//! to one shows everywhere it appears (`docs/DESIGN.md`).

use gpui::{
    AnyElement, Context, Div, Entity, FontWeight, ScrollHandle, Window, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::tokens::{elevation, radius, space, text};
use katna_ui::{TextInput, px};

use super::MailWindow;
use crate::theme::{Accent, Theme};
use crate::widgets::{
    CARD_REST, Check, Fold, avatar, card, checkbox, choice_chip, count_pill, filled_button,
    fold_arrow, fold_box, icon_button, line_field, menu, menu_item, outlined_button, pill_button,
    radio, raised, row, switch, tag, text_button, ticked_row, tonal_icon_button,
};

pub(super) struct Gallery {
    scroll: ScrollHandle,
    light: Entity<TextInput>,
    dark: Entity<TextInput>,
    /// The sample fold is open.
    open: bool,
    fold: Fold,
}

impl MailWindow {
    /// Opens the Gallery; only development builds offer it.
    pub(super) fn open_gallery(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !cfg!(debug_assertions) {
            return;
        }
        let input = |cx: &mut Context<Self>| cx.new(|cx| TextInput::new(tr!("gallery-hint"), cx));
        self.gallery = Some(Gallery {
            scroll: ScrollHandle::new(),
            light: input(cx),
            dark: input(cx),
            open: true,
            fold: Fold::default(),
        });
        window.refresh();
        cx.notify();
    }

    pub(super) fn close_gallery(&mut self, cx: &mut Context<Self>) -> bool {
        let open = self.gallery.take().is_some();
        if open {
            cx.notify();
        }
        open
    }

    pub(super) fn render_gallery(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let gallery = self.gallery.as_ref()?;
        let view = &self.config.mail;
        let system = &self.desktop_colors.colors;
        let accent = Accent::parse(&view.accent);
        let light = Theme::pick(false, view.colors(), accent, system);
        let dark = Theme::pick(true, view.colors(), accent, system);
        Some(
            div()
                .id("gallery")
                .occlude()
                .absolute()
                .inset_0()
                .flex()
                .flex_col()
                .bg(rgba(light.page))
                .child(
                    div()
                        .flex_none()
                        .h(px(56.0))
                        .px(px(space::S5))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(space::S3))
                        .text_size(px(text::TITLE))
                        .text_color(rgba(light.text))
                        .child(div().flex_1().child(tr!("gallery-title")))
                        .child(
                            icon_button("gallery-close", "close", 20.0, &light).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.close_gallery(cx);
                                }),
                            ),
                        ),
                )
                .child(
                    div()
                        .id("gallery-scroll")
                        .flex_1()
                        .min_h_0()
                        .overflow_y_scroll()
                        .track_scroll(&gallery.scroll)
                        .flex()
                        .flex_col()
                        // The columns sit in a row of their own, so they
                        // grow with their controls and the page scrolls.
                        .child(
                            div()
                                .flex_none()
                                .flex()
                                .flex_row()
                                .child(column("light", &light, &gallery.light, gallery, cx))
                                .child(column("dark", &dark, &gallery.dark, gallery, cx)),
                        ),
                )
                .into_any_element(),
        )
    }
}

/// Every control in one theme.
fn column(
    side: &'static str,
    th: &Theme,
    input: &Entity<TextInput>,
    gallery: &Gallery,
    cx: &mut Context<MailWindow>,
) -> Div {
    let open = gallery.open;
    let fold_head = div()
        .id(id_of(side, "fold-head"))
        .h(px(44.0))
        .px(px(space::S4))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(space::S3))
        .cursor_pointer()
        .child(div().flex_1().child(tr!("gallery-fold")))
        .child(fold_arrow(
            &format!("gallery-{side}-arrow"),
            &gallery.fold,
            open,
            th.text_dim,
            20.0,
        ))
        .on_click(cx.listener(|this, _, _, cx| {
            if let Some(gallery) = this.gallery.as_mut() {
                gallery.open = !gallery.open;
                gallery.fold.turn();
                cx.notify();
            }
        }));
    // An edge rather than a shadow: the fold clips what lies outside it.
    let fold_card = card(div(), th, th.pane(), radius::LG, 0.0)
        .w(px(320.0))
        .border_1()
        .border_color(rgba(th.divider))
        .child(fold_head)
        .when(open, |d| {
            d.child(
                div()
                    .px(px(space::S4))
                    .pb(px(space::S4))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("gallery-fold-body")),
            )
        });
    let id = |name: &str| gpui::SharedString::from(format!("gallery-{side}-{name}"));
    let line = || {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(space::S3))
    };
    div()
        .flex_1()
        .min_w_0()
        .p(px(space::S6))
        .flex()
        .flex_col()
        .gap(px(space::S6))
        .bg(rgba(th.page))
        .text_color(rgba(th.text))
        .text_size(px(text::BODY))
        .child(section(
            tr!("gallery-buttons"),
            th,
            div()
                .flex()
                .flex_col()
                .gap(px(space::S4))
                .child(
                    line()
                        .child(filled_button(id("filled"), tr!("gallery-primary"), th))
                        .child(outlined_button(
                            id("outlined"),
                            tr!("gallery-secondary"),
                            th,
                        ))
                        .child(text_button(id("text"), "add", tr!("gallery-add"), th)),
                )
                .child(
                    line()
                        .child(pill_button(
                            id("pill"),
                            "reply",
                            tr!("gallery-reply"),
                            40.0,
                            1.0,
                            th,
                        ))
                        .child(icon_button(id("icon"), "archive", 20.0, th))
                        .child(tonal_icon_button(
                            id("tonal"),
                            "mail",
                            40.0,
                            false,
                            true,
                            th,
                        ))
                        .child(tonal_icon_button(
                            id("tonal-on"),
                            "bell-off",
                            40.0,
                            true,
                            true,
                            th,
                        ))
                        .child(tonal_icon_button(
                            id("tonal-off"),
                            "phone",
                            40.0,
                            false,
                            false,
                            th,
                        )),
                ),
        ))
        .child(section(
            tr!("gallery-chips"),
            th,
            line()
                .child(choice_chip(id("chip-on"), tr!("gallery-picked"), true, th))
                .child(choice_chip(
                    id("chip-off"),
                    tr!("gallery-not-picked"),
                    false,
                    th,
                ))
                .child(tag(tr!("gallery-tag"), th)),
        ))
        .child(section(
            tr!("gallery-rows"),
            th,
            div()
                .flex()
                .flex_col()
                .child(
                    row(id("row-on"), true, th)
                        .child(div().flex_1().child(tr!("gallery-open-row")))
                        .child(count_pill(12, true, th)),
                )
                .child(
                    row(id("row"), false, th)
                        .child(div().flex_1().child(tr!("gallery-row")))
                        .child(count_pill(1234, false, th)),
                )
                .child(ticked_row(id("row-ticked"), th).child(tr!("gallery-ticked-row"))),
        ))
        .child(section(
            tr!("gallery-fields"),
            th,
            line_field(id("field"), input, th, cx),
        ))
        .child(section(
            tr!("gallery-toggles"),
            th,
            line()
                .child(switch(1.0, th))
                .child(switch(0.0, th))
                .child(checkbox(id("check-on"), Check::On, th))
                .child(checkbox(id("check-part"), Check::Partial, th))
                .child(checkbox(id("check-off"), Check::Off, th))
                .child(radio(1.0, th))
                .child(radio(0.0, th))
                .child(avatar("Ada Lovelace", "ada@example.com", 32.0)),
        ))
        .child(section(
            tr!("gallery-menu"),
            th,
            menu(th)
                .w(px(200.0))
                .child(menu_item(id("menu-1"), &tr!("gallery-reply"), th))
                .child(menu_item(id("menu-2"), &tr!("gallery-add"), th)),
        ))
        .child(section(
            tr!("gallery-cards"),
            th,
            line().children(
                [
                    (tr!("gallery-card-rest"), CARD_REST),
                    (tr!("gallery-card-active"), 1.0),
                ]
                .into_iter()
                .map(|(label, shadow)| {
                    card(div(), th, th.pane(), radius::LG, shadow)
                        .w(px(160.0))
                        .h(px(96.0))
                        .p(px(space::S4))
                        .text_color(rgba(th.text_dim))
                        .child(label)
                }),
            ),
        ))
        .child(section(
            tr!("gallery-motion"),
            th,
            fold_box(&format!("gallery-{side}-fold"), &gallery.fold, fold_card),
        ))
        .child(section(
            tr!("gallery-elevation"),
            th,
            line().children(
                [
                    elevation::PAGE,
                    elevation::CARD,
                    elevation::FLOAT,
                    elevation::MENU,
                    elevation::POPOVER,
                ]
                .into_iter()
                .map(|level| {
                    raised(div(), th, radius::LG, level)
                        .size(px(72.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgba(th.text_dim))
                        .child(format!("{level}"))
                }),
            ),
        ))
        .child(section(
            tr!("gallery-text"),
            th,
            div().flex().flex_col().gap(px(space::S2)).children(
                [
                    text::DISPLAY,
                    text::TITLE,
                    text::SUBTITLE,
                    text::BODY,
                    text::SMALL,
                    text::CAPTION,
                    text::MICRO,
                ]
                .into_iter()
                .map(|size| {
                    div()
                        .text_size(px(size))
                        .line_height(px(text::line_height(size)))
                        .child(tr!("gallery-sample", size = size as i64))
                }),
            ),
        ))
}

fn id_of(side: &str, name: &str) -> gpui::SharedString {
    gpui::SharedString::from(format!("gallery-{side}-{name}"))
}

/// A heading over its controls.
fn section(title: String, th: &Theme, body: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(space::S3))
        .child(
            div()
                .text_size(px(text::CAPTION))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(th.text_dim))
                .child(title),
        )
        .child(body)
}
