// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Appearance > Colors and Accent: the color schemes as cards,
//! each with a small picture of the mail window on its light and dark
//! side in the accent color picked, grouped into Katna's built-in schemes
//! and the desktop's (`katna_platform::colors::DesktopScheme`); and the
//! accent choices under them.

use std::sync::Mutex;

use gpui::{
    AnimationExt, AnyElement, Context, Div, ElementId, MouseButton, MouseDownEvent, SharedString,
    SpringAnimation, Window, canvas, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::motion;
use katna_ui::{Ripple, px};

use super::MailWindow;
use super::scheme_color::Target;
use super::settings::Change;
use crate::schemes::{self, BUILT_IN};
use crate::theme::{Accent, Theme, fade, mix};
use crate::user_schemes;
use crate::widgets::{color_swatch, color_wheel};

/// The accent colors offered besides the scheme's and the desktop's:
/// blue, teal, green, yellow, orange, pink and violet.
const ACCENTS: [u32; 7] = [
    0x0b57d0ff, 0x00807fff, 0x2e9e4fff, 0xe2a400ff, 0xe8590cff, 0xd6336cff, 0x7048e8ff,
];
/// A scheme card's width; its picture is as high as [`PICTURE_HEIGHT`].
const CARD_WIDTH: f32 = 136.0;
const PICTURE_HEIGHT: f32 = 64.0;
/// The corner radius of a scheme card's picture.
const PICTURE_RADIUS: f32 = 8.0;
const SWATCH: f32 = 26.0;

impl MailWindow {
    /// The scheme cards of Settings > Appearance > Colors.
    pub(super) fn scheme_picker(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let picked = self.config.mail.colors().to_owned();
        let card = |id: &'static str,
                    name: SharedString,
                    tag: Option<SharedString>,
                    cx: &mut Context<Self>| {
            self.scheme_card(id, name, tag, id == picked, th, cx)
        };
        let built_in = std::iter::once(schemes::KATNA)
            .chain(BUILT_IN.iter().map(|s| s.id))
            .map(|id| card(id, scheme_name(id), None, cx))
            .collect::<Vec<_>>();
        let system = &self.desktop_colors.colors;
        let mut from_system = vec![card(
            schemes::SYSTEM,
            tr!("settings-appearance-colors-system").into(),
            Some(tr!("settings-appearance-colors-system-detail").into()),
            cx,
        )];
        let mut yours = Vec::new();
        for scheme in &system.schemes {
            let mine = scheme.id.starts_with(user_schemes::PREFIX);
            let tag = match (&scheme.light, &scheme.dark) {
                (Some(_), None) => Some(tr!("settings-appearance-colors-light-only").into()),
                (None, Some(_)) => Some(tr!("settings-appearance-colors-dark-only").into()),
                _ => None,
            };
            let card = card(intern(&scheme.id), scheme.name.clone().into(), tag, cx);
            if mine {
                yours.push(card);
            } else {
                from_system.push(card);
            }
        }
        yours.push(self.new_scheme_card(
            "scheme-new",
            "add",
            tr!("scheme-customize-card"),
            tr!("scheme-customize-card-detail"),
            th,
            cx,
        ));
        yours.push(self.new_scheme_card(
            "scheme-import",
            "upload",
            tr!("scheme-import-card"),
            tr!("scheme-import-card-detail"),
            th,
            cx,
        ));
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(group_heading(
                tr!("settings-appearance-colors-built-in"),
                th,
            ))
            .child(cards(built_in))
            .child(group_heading(
                tr!("settings-appearance-colors-from-system"),
                th,
            ))
            .child(cards(from_system))
            .child(group_heading(tr!("settings-appearance-colors-yours"), th))
            .child(cards(yours))
    }

    /// Customize… and Import… at the end of Yours: a card with a dashed
    /// picture holding `icon`.
    fn new_scheme_card(
        &self,
        id: &'static str,
        icon_name: &str,
        name: String,
        tag: String,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let import = id == "scheme-import";
        self.page_control(div().id(id), th, cx)
            .relative()
            .overflow_hidden()
            .w(px(CARD_WIDTH))
            .p(px(4.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .rounded(px(12.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, window, cx| {
                if import {
                    this.import_scheme(cx);
                } else {
                    let picked = intern(this.config.mail.colors());
                    this.customize_scheme(picked, window, cx);
                }
            }))
            .child(
                Ripple::new(
                    ElementId::Name(format!("{id}-ripple").into()),
                    rgba(th.ripple),
                )
                .rounded(12.0),
            )
            .child(
                div()
                    .h(px(PICTURE_HEIGHT))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(PICTURE_RADIUS))
                    .border_2()
                    .border_dashed()
                    .border_color(rgba(th.divider))
                    .child(crate::widgets::icon(icon_name, th.text_dim, 24.0)),
            )
            .child(
                div()
                    .px(px(2.0))
                    .flex()
                    .flex_col()
                    .text_size(px(13.0))
                    .child(div().min_w_0().truncate().child(name))
                    .child(
                        div()
                            .min_w_0()
                            .truncate()
                            .text_size(px(11.0))
                            .text_color(rgba(th.text_faint))
                            .child(tag),
                    ),
            )
            .into_any_element()
    }

    fn scheme_card(
        &self,
        id: &'static str,
        name: SharedString,
        tag: Option<SharedString>,
        on: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let system = &self.desktop_colors.colors;
        let accent = Accent::parse(&self.config.mail.accent);
        let side = |dark: bool| scheme_picture(&Theme::pick(dark, id, accent, system));
        // GPUI clips children to a rectangle, so each side rounds its own
        // outer corners, nested inside the 2 px ring.
        let inner = PICTURE_RADIUS - 2.0;
        // A scheme with one side shows it alone.
        let sides = match Theme::forced_dark(id, system) {
            Some(dark) if id != schemes::SYSTEM => vec![side(dark).rounded(px(inner))],
            _ => vec![
                side(false).rounded_l(px(inner)),
                side(true).rounded_r(px(inner)),
            ],
        };
        let element_id = ElementId::Name(format!("scheme-{id}").into());
        self.page_control(div().id(element_id), th, cx)
            .relative()
            .overflow_hidden()
            .w(px(CARD_WIDTH))
            .p(px(4.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .rounded(px(12.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| this.apply(Change::Colors(id), cx)))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.open_scheme_menu(id, event.position, cx);
                }),
            )
            .child(
                Ripple::new(
                    ElementId::Name(format!("scheme-ripple-{id}").into()),
                    rgba(th.ripple),
                )
                .rounded(12.0),
            )
            .child(
                div()
                    .id(ElementId::Name(format!("scheme-picture-{id}").into()))
                    .h(px(PICTURE_HEIGHT))
                    .flex()
                    .flex_row()
                    .rounded(px(PICTURE_RADIUS))
                    .overflow_hidden()
                    .border_2()
                    .children(sides)
                    .with_spring(
                        ElementId::Name(format!("scheme-ring-{id}").into()),
                        SpringAnimation::new(motion::SMOOTH).to(if on { 1.0 } else { 0.0 }),
                        {
                            let (off, accent) = (th.divider, th.accent);
                            move |el, s: f32| {
                                el.border_color(rgba(mix(off, accent, s.clamp(0.0, 1.0))))
                            }
                        },
                    ),
            )
            .child(
                div()
                    .px(px(2.0))
                    .flex()
                    .flex_col()
                    .text_size(px(13.0))
                    .child(div().min_w_0().truncate().child(name))
                    .children(tag.map(|tag| {
                        div()
                            .min_w_0()
                            .truncate()
                            .text_size(px(11.0))
                            .text_color(rgba(th.text_faint))
                            .child(tag)
                    })),
            )
            .into_any_element()
    }

    /// The accent choices of Settings > Appearance > Accent.
    pub(super) fn accent_picker(&self, th: &Theme, window: &Window, cx: &mut Context<Self>) -> Div {
        let picked = Accent::parse(&self.config.mail.accent);
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .child(self.accent_chip(
                Accent::Scheme,
                tr!("settings-appearance-accent-scheme"),
                picked,
                th,
                cx,
            ))
            .child(self.accent_chip(
                Accent::System,
                tr!("settings-appearance-accent-system"),
                picked,
                th,
                cx,
            ))
            .children(ACCENTS.map(|color| self.accent_swatch(color, picked, th, cx)))
            .child(self.accent_wheel(picked, th, cx))
            .children(self.render_color_picker(|t| t == Target::Accent, th, window, cx))
    }

    fn accent_chip(
        &self,
        accent: Accent,
        label: String,
        picked: Accent,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let on = accent == picked;
        let name = format!("accent-{}", accent.setting());
        self.page_control(div().id(ElementId::Name(name.clone().into())), th, cx)
            .relative()
            .overflow_hidden()
            .h(px(SWATCH + 6.0))
            .px(px(12.0))
            .flex()
            .items_center()
            .rounded_full()
            .text_size(px(13.0))
            .cursor_pointer()
            .bg(rgba(if on { th.nav_selected } else { th.chip }))
            .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
            .hover(|s| {
                s.bg(rgba(mix(
                    if on { th.nav_selected } else { th.chip },
                    th.text,
                    0.06,
                )))
            })
            .on_click(cx.listener(move |this, _, _, cx| this.apply(Change::Accent(accent), cx)))
            .child(
                Ripple::new(
                    ElementId::Name(format!("{name}-ripple").into()),
                    rgba(th.ripple),
                )
                .rounded((SWATCH + 6.0) / 2.0),
            )
            .child(label)
            .into_any_element()
    }

    fn accent_swatch(
        &self,
        color: u32,
        picked: Accent,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let accent = Accent::Color(color);
        let id = ElementId::Name(format!("accent-{}", accent.setting()).into());
        let swatch = color_swatch(id, color, accent == picked, SWATCH + 6.0, th);
        self.page_control(swatch, th, cx)
            .on_click(cx.listener(move |this, _, _, cx| this.apply(Change::Accent(accent), cx)))
            .into_any_element()
    }

    /// The wheel after the accent swatches: the color picker, for any
    /// other accent.
    fn accent_wheel(&self, picked: Accent, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let custom = match picked {
            Accent::Color(color) if !ACCENTS.contains(&color) => Some(color),
            _ => None,
        };
        let swatches = self.color_swatches.clone();
        let wheel = color_wheel("accent-wheel", custom, SWATCH + 6.0, th);
        self.page_control(wheel, th, cx)
            .relative()
            .tooltip(crate::widgets::tip(
                tr!("settings-appearance-accent-more"),
                th,
            ))
            .on_click(cx.listener(|this, _, window, cx| {
                this.toggle_color_picker(Target::Accent, window, cx)
            }))
            .child(
                canvas(
                    move |bounds, _, _| {
                        swatches.borrow_mut().insert(Target::Accent, bounds);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .into_any_element()
    }
}

/// `id` as a `&'static str` for [`Change::Colors`]: the desktop's scheme
/// ids are kept once each, so there are only ever as many as schemes seen.
pub(super) fn intern(id: &str) -> &'static str {
    static IDS: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
    let mut ids = IDS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(known) = ids.iter().find(|known| **known == id) {
        return known;
    }
    let leaked: &'static str = Box::leak(id.to_owned().into_boxed_str());
    ids.push(leaked);
    leaked
}

/// The name of the scheme with `id`, in the current language.
fn scheme_name(id: &str) -> SharedString {
    match schemes::built_in(id) {
        Some(scheme) => tr!(scheme.name),
        None => tr!("scheme-katna"),
    }
    .into()
}

fn group_heading(text: String, th: &Theme) -> Div {
    div()
        .pt(px(4.0))
        .text_size(px(12.0))
        .text_color(rgba(th.text_faint))
        .child(text)
}

fn cards(cards: Vec<AnyElement>) -> Div {
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap(px(8.0))
        .children(cards)
}

/// The mail window in miniature, in `th`: the top bar, Compose and the
/// selected folder at the left, and the list's card with an accent tab, a
/// line of text, a ticked row and a date.
pub(super) fn scheme_picture(th: &Theme) -> Div {
    let bar = |w: f32, h: f32, color: u32| div().w(px(w)).h(px(h)).rounded_full().bg(rgba(color));
    div()
        .flex_1()
        .h_full()
        .p(px(4.0))
        .flex()
        .flex_col()
        .gap(px(4.0))
        .bg(rgba(th.page))
        .child(div().h(px(5.0)).rounded_full().bg(rgba(th.search)))
        .child(
            div()
                .flex_1()
                .flex()
                .flex_row()
                .gap(px(3.0))
                .child(
                    div()
                        .w(px(16.0))
                        .flex()
                        .flex_col()
                        .gap(px(3.0))
                        .child(
                            div()
                                .w(px(14.0))
                                .h(px(7.0))
                                .rounded(px(3.0))
                                .bg(rgba(th.compose)),
                        )
                        .child(bar(16.0, 5.0, th.nav_selected))
                        .child(bar(10.0, 3.0, fade(th.text_dim, 0.6)))
                        .child(bar(12.0, 3.0, fade(th.text_dim, 0.6))),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .rounded_tl(px(4.0))
                        .bg(rgba(th.surface))
                        .p(px(3.0))
                        .flex()
                        .flex_col()
                        .gap(px(3.0))
                        .child(bar(12.0, 2.0, th.accent))
                        .child(div().h(px(3.0)).rounded_full().bg(rgba(th.text)))
                        .child(div().h(px(5.0)).rounded(px(1.0)).bg(rgba(th.checked_row)))
                        .child(bar(14.0, 3.0, th.text_faint)),
                ),
        )
}
