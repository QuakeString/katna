// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna's shared controls, drawn in [`Theme`]'s colors: icons, tooltips,
//! buttons, card shadows and elevation (`docs/DESIGN.md`). They began in
//! Katna Mail and move here one group at a time as Katna PDF needs them;
//! Mail's `widgets.rs` re-exports them.

use gpui::{
    AnyElement, AnyView, App, BoxShadow, Div, FontWeight, Role, SharedString, Stateful, Window,
    div, point, prelude::*, rgba,
};

use crate::motion::lerp;
use crate::theme::{Theme, fade, mix};
use crate::tokens::{space, state, text};
use crate::{Glow, Ripple, Tooltip, WindowDrag, px};

pub fn icon(name: &str, color: u32, size: f32) -> AnyElement {
    // Back, reply, chevrons and the like turn around right to left.
    crate::icon_svg(name)
        .size(px(size))
        .flex_none()
        .text_color(rgba(color))
        .into_any_element()
}

/// A tooltip saying `text`, for `.tooltip()`: it shows once the pointer
/// rests on the element. Prefer [`Tip::tip`], which also names the
/// element to screen readers.
pub fn tip(
    text: impl Into<SharedString>,
    th: &Theme,
) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    Tooltip::text(text, rgba(th.snackbar), rgba(th.snackbar_text))
}

/// A tooltip that also names its element to screen readers (Orca, NVDA):
/// what the pointer shows is what they read out.
pub trait Tip: StatefulInteractiveElement + Sized {
    fn tip(self, text: impl Into<SharedString>, th: &Theme) -> Self {
        let text = text.into();
        self.aria_label(text.clone()).tooltip(tip(text, th))
    }
}

impl<E: StatefulInteractiveElement> Tip for E {}

/// A round icon button with a centered ripple.
pub fn icon_button(
    id: impl Into<gpui::ElementId>,
    name: &str,
    size: f32,
    th: &Theme,
) -> Stateful<Div> {
    icon_button_colored(id, name, size, th.text_dim, th)
}

pub fn icon_button_colored(
    id: impl Into<gpui::ElementId>,
    name: &str,
    size: f32,
    color: u32,
    th: &Theme,
) -> Stateful<Div> {
    icon_button_with(id, icon(name, color, size), th)
}

/// An [`icon_button_colored`] around any icon, such as Katna Mail's `fold_arrow`.
pub fn icon_button_with(
    id: impl Into<gpui::ElementId>,
    glyph: AnyElement,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    div()
        .id(id.clone())
        .role(Role::Button)
        .relative()
        .overflow_hidden()
        .size(px(40.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .keeps_press()
        // Keep the header bar from starting a window move.
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .child(Glow::new(("glow", id_hash(&id)), rgba(th.hover)))
        .child(Ripple::new(("ripple", id_hash(&id)), rgba(th.ripple)).centered())
        .child(glyph)
}

/// The shared hover fade (`katna_ui::Glow`) for a box that is not one of
/// the round or pill buttons above: it eases in and out instead of
/// switching, and follows Animation speed and Reduce motion. Make the box
/// `relative()` and put this first among its children; `radius` is its
/// corner radius, `None` for a pill or circle.
pub fn hover_fade(id: impl Into<gpui::ElementId>, radius: Option<f32>, th: &Theme) -> Glow {
    let glow = Glow::new(id, rgba(th.hover));
    match radius {
        Some(r) => glow.corners([r; 4]),
        None => glow.fade(),
    }
}

/// A stable number for a ripple's ID derived from its button's ID.
pub fn id_hash(id: &gpui::ElementId) -> usize {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    id.to_string().hash(&mut hasher);
    hasher.finish() as usize
}

/// An outlined button with an icon and a label. The label folds away as
/// `shown` goes from 1 to 0, leaving a round button with the icon alone;
/// `word` is the label's width, measured in the font it shows in.
pub fn pill_button(
    id: impl Into<gpui::ElementId>,
    name: &str,
    label: impl Into<SharedString>,
    word: f32,
    shown: f32,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    let t = shown.clamp(0.0, 1.0);
    let label = label.into();
    div()
        .id(id.clone())
        .role(Role::Button)
        .aria_label(label.clone())
        .relative()
        .overflow_hidden()
        .h(px(36.0))
        .pl(px(lerp(17.0, 16.0, t)))
        .pr(px(lerp(17.0, 22.0, t)))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0 * t))
        .rounded_full()
        .border_1()
        .border_color(rgba(fade(th.text_faint, 0.7)))
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_dim))
        .cursor_pointer()
        .keeps_press()
        .child(Glow::new(("glow", id_hash(&id)), rgba(th.hover)).fade())
        .child(Ripple::new(("ripple", id_hash(&id)), rgba(th.ripple)).border(1.0))
        .child(icon(name, th.text_dim, 20.0))
        .child(
            div()
                .flex_none()
                .whitespace_nowrap()
                .overflow_hidden()
                .when(t < 0.999, |d| d.w(px(word * t)).opacity(t))
                .child(label),
        )
}

/// How far a card's shadow reaches past its edges.
pub const CARD_SHADOW_ROOM: f32 = 4.0;

/// The very short shadow under a card, for a little depth. `t` fades it
/// away (0 on a phone, whose cards run edge to edge); a card at rest
/// beside the one with the keys gets [`CARD_REST`] of it. In light colors
/// the card also gets a crisp hairline ring ([`Theme::card_edge`]) and a
/// tighter shadow, so its edge stays sharp on the near-white page.
pub fn card_shadow(th: &Theme, t: f32) -> Vec<BoxShadow> {
    if t <= 0.001 {
        return Vec::new();
    }
    let t = t.min(1.0);
    if th.card_edge & 0xff == 0 {
        return vec![BoxShadow {
            color: rgba(fade(th.shadow, 0.3 * t)).into(),
            offset: point(px(0.0), px(1.0)),
            blur_radius: px(3.0),
            spread_radius: px(0.0),
            inset: false,
        }];
    }
    // The ring stays nearly full at rest (80%) so every card keeps its
    // edge; it fades in with the card only below that.
    let ring = (t / CARD_REST).min(1.0) * lerp(0.8, 1.0, t);
    vec![
        BoxShadow {
            color: rgba(fade(th.card_edge, ring)).into(),
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(1.0),
            inset: false,
        },
        BoxShadow {
            color: rgba(fade(th.shadow, 0.47 * t)).into(),
            offset: point(px(0.0), px(1.0)),
            blur_radius: px(2.0),
            spread_radius: px(0.0),
            inset: false,
        },
    ]
}

/// How strongly a card at rest beside the one with the keys shows its
/// shadow, as `t` of [`card_shadow`].
pub const CARD_REST: f32 = 0.15;

/// A faint line around a card (the list, the reading pane, Quick
/// settings). It is drawn over the card's content, so lines of the list
/// that fill the card's width do not hide it; the card keeps `t` px of
/// padding inside it. `t` fades it away (0 on a phone, edge to edge).
pub fn card_outline(th: &Theme, radius: f32, t: f32) -> Option<AnyElement> {
    (t > 0.001).then(|| {
        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .rounded(px(radius))
            .border_1()
            .border_color(rgba(fade(th.divider, t.min(1.0))))
            .into_any_element()
    })
}

/// How a button shows. Every labelled button is one of these
/// (`docs/DESIGN.md`); round icon buttons are [`icon_button`] and
/// [`tonal_icon_button`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonStyle {
    /// Accent fill: the primary action of a panel.
    Filled,
    /// An edge and accent text: a second action beside a filled one.
    Outlined,
    /// Accent text alone, with a soft hover: a light action inside a card.
    Text,
}

/// The height of a labelled button.
pub const BUTTON_HEIGHT: f32 = 36.0;

/// A rounded button in `style`, ready for its label (and an icon before
/// it, with [`ButtonStyle::Text`]): shape, colours, hover and ripple.
pub fn button(id: impl Into<gpui::ElementId>, style: ButtonStyle, th: &Theme) -> Stateful<Div> {
    let id = id.into();
    let ripple = id_hash(&id);
    let base = div()
        .id(id)
        .role(Role::Button)
        .relative()
        .overflow_hidden()
        .flex_none()
        .h(px(BUTTON_HEIGHT))
        .flex()
        .flex_row()
        .items_center()
        .rounded_full()
        .text_size(px(text::BODY))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .keeps_press();
    match style {
        ButtonStyle::Filled => base
            .px(px(space::S6))
            .bg(rgba(th.accent))
            .text_color(rgba(th.on_accent))
            .hover(|s| s.shadow(elevation(th, 1.0)))
            .child(Ripple::new(("ripple", ripple), rgba(0xffffff3d))),
        // 20, between S5 and S6: the edge makes the button look wider.
        ButtonStyle::Outlined => base
            .px(px(20.0))
            .border_1()
            .border_color(rgba(fade(th.text_faint, 0.7)))
            .text_color(rgba(th.accent))
            .child(Glow::new(("glow", ripple), rgba(th.hover)).fade())
            .child(Ripple::new(("ripple", ripple), rgba(th.ripple)).border(1.0)),
        ButtonStyle::Text => base
            .px(px(space::S4))
            .gap(px(6.0))
            .text_color(rgba(th.accent))
            .child(Glow::new(("glow", ripple), rgba(th.hover)).fade())
            .child(Ripple::new(("ripple", ripple), rgba(th.ripple))),
    }
}

/// A filled, rounded button (the primary action of a panel).
pub fn filled_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
    let label = label.into();
    button(id, ButtonStyle::Filled, th)
        .aria_label(label.clone())
        .child(label)
}

/// An outlined, rounded button with a label.
pub fn outlined_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
    let label = label.into();
    button(id, ButtonStyle::Outlined, th)
        .aria_label(label.clone())
        .child(label)
}

/// An accent label with an icon before it, and no edge or fill until the
/// pointer is over it ("Add email").
pub fn text_button(
    id: impl Into<gpui::ElementId>,
    name: &str,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
    let label = label.into();
    button(id, ButtonStyle::Text, th)
        .aria_label(label.clone())
        .child(icon(name, th.accent, 18.0))
        .child(label)
}

/// The fill of a [`tonal_icon_button`] at rest and under the pointer: the
/// accent, lightly, over the card.
pub fn tonal_fill(th: &Theme) -> (u32, u32) {
    let accent = th.accent | 0xff;
    let t = if th.dark { 0.16 } else { 0.17 };
    (
        mix(th.surface, accent, t),
        mix(th.surface, accent, t + 0.08),
    )
}

/// A round icon button on a light accent fill (a contact's mail, search
/// and call actions). `on` fills it with the accent (muted); one that is
/// not `enabled` dims and takes no clicks.
pub fn tonal_icon_button(
    id: impl Into<gpui::ElementId>,
    name: &str,
    size: f32,
    on: bool,
    enabled: bool,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    let ripple = id_hash(&id);
    let (rest, hover) = tonal_fill(th);
    let (fill, glyph) = if on {
        (th.accent | 0xff, th.on_accent)
    } else {
        (rest, th.accent)
    };
    div()
        .id(id)
        .role(Role::Button)
        .relative()
        .overflow_hidden()
        .size(px(size))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .bg(rgba(fill))
        .when(!enabled, |d| d.opacity(state::DISABLED))
        .when(enabled, |d| {
            d.cursor_pointer()
                .when(!on, |d| d.hover(move |s| s.bg(rgba(hover))))
                .child(Ripple::new(("ripple", ripple), rgba(th.ripple)).centered())
        })
        .child(icon(name, glyph, 20.0))
}

/// Material-style elevation: `level` 0 is flat, 3 floats well above. In
/// dark colors, where a shadow barely shows, a faint light edge
/// ([`Theme::rim`]) outlines it too; it fades in over the first level.
pub fn elevation(th: &Theme, level: f32) -> Vec<BoxShadow> {
    if level <= 0.001 {
        return Vec::new();
    }
    let t = (level / 3.0).min(1.0);
    let mut shadows = vec![
        BoxShadow {
            color: rgba(fade(th.shadow, 0.9 * t)).into(),
            offset: point(px(0.0), px(1.0)),
            blur_radius: px(2.0 * level.min(2.0)),
            spread_radius: px(0.0),
            inset: false,
        },
        BoxShadow {
            color: rgba(fade(th.shadow, 0.5 * t)).into(),
            offset: point(px(0.0), px(level)),
            blur_radius: px(3.0 * level),
            spread_radius: px(level / 2.0),
            inset: false,
        },
    ];
    // Last, so the shadows do not darken it.
    if th.rim & 0xff != 0 {
        shadows.push(BoxShadow {
            color: rgba(fade(th.rim, level.min(1.0))).into(),
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(1.0),
            inset: false,
        });
    }
    shadows
}
