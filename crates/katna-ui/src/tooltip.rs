// SPDX-License-Identifier: GPL-3.0-or-later

//! A tooltip: a short label that shows after the pointer rests on an
//! element for a moment (GPUI's hover delay), as in Material Design.
//!
//! ```ignore
//! button.tooltip(Tooltip::text("Archive", bg, fg))
//! ```

use gpui::{
    AnyView, App, Context, Global, Hsla, IntoElement, Render, SharedString, Window, div,
    prelude::*, px,
};

/// The UI font of the app, for views drawn outside its main view (such as
/// tooltips). Set it with `cx.set_global` at startup.
pub struct UiFont(pub SharedString);

impl Global for UiFont {}

pub struct Tooltip {
    text: SharedString,
    bg: Hsla,
    fg: Hsla,
}

impl Tooltip {
    /// A builder for GPUI's `.tooltip()`: `text` on `bg`.
    pub fn text(
        text: impl Into<SharedString>,
        bg: impl Into<Hsla>,
        fg: impl Into<Hsla>,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
        let (text, bg, fg) = (text.into(), bg.into(), fg.into());
        move |_, cx| {
            let text = text.clone();
            cx.new(|_| Self { text, bg, fg }).into()
        }
    }
}

impl Render for Tooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let font = cx.try_global::<UiFont>().map(|f| f.0.clone());
        // Kept below and right of the pointer, clear of the cursor.
        div().pl(px(4.0)).pt(px(14.0)).child(
            div()
                // Long text wraps rather than running off the window.
                .max_w(px(320.0))
                .px(px(8.0))
                .py(px(4.0))
                .rounded(px(4.0))
                .bg(self.bg)
                .text_color(self.fg)
                .text_size(px(12.0))
                .line_height(px(16.0))
                .when_some(font, |d, font| d.font_family(font))
                .child(self.text.clone()),
        )
    }
}
