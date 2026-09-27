// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Experimental > Look & Feel: Katna's own window frame instead
//! of the desktop's, and a blurred, translucent window background. Both
//! apply at once to every open window (`katna_chrome::Look`).

use gpui::{AnyElement, Context, FontWeight, div, prelude::*, px, rgba};
use katna_chrome::{DecorationMode, Desktop, Look, Session};
use katna_core::config::{Config, WindowFrame};

use super::MailWindow;
use super::settings::{Change, heading};
use crate::theme::Theme;
use crate::widgets::switch;

/// The look the settings ask for.
pub fn look(config: &Config) -> Look {
    Look {
        own_frame: config.experimental.window_frame == WindowFrame::Katna,
        blur: config.experimental.blur,
    }
}

impl MailWindow {
    pub(super) fn experimental_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .pt(px(20.0))
                    .pb(px(4.0))
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child("Features still being tried out. They may change or go away."),
            )
            .child(div().pt(px(12.0)).child(heading("Look & Feel", th)))
            .child(self.row(
                "Window frame",
                Some("Who draws the title bar, the window buttons, the corners and the shadow."),
                self.frame_choice(th, cx),
                th,
            ))
            .child(self.row(
                "Blurred background",
                Some(
                    "The desktop shows through the top bar and the folders, blurred, and \
                     menus and popovers are frosted glass.",
                ),
                self.blur_switch(th, cx),
                th,
            ))
            .into_any_element()
    }

    fn frame_choice(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let env = self.chrome.environment();
        // GNOME on Wayland leaves every frame to the app: Native already is
        // Katna's frame there.
        if env.native_decorations() == DecorationMode::Client {
            return explain(
                "Your desktop leaves the frame to each app, so Katna already draws its own.",
                th,
            );
        }
        let desktop = match env.desktop {
            Desktop::Kde => "KDE",
            Desktop::Gnome => "GNOME",
            Desktop::Other(_) => "the desktop",
        };
        let frame = self.config.experimental.window_frame;
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(self.radio_row(
                "page-frame-native",
                match env.desktop {
                    Desktop::Kde => "Native: KDE's frame, in your Plasma theme",
                    _ => "Native: the desktop's frame",
                },
                frame == WindowFrame::Native,
                Change::WindowFrame(WindowFrame::Native),
                th,
                cx,
            ))
            .child(self.radio_row(
                "page-frame-katna",
                "Katna: the top bar becomes the title bar",
                frame == WindowFrame::Katna,
                Change::WindowFrame(WindowFrame::Katna),
                th,
                cx,
            ))
            .when(frame == WindowFrame::Katna, |d| {
                d.child(explain_owned(
                    format!(
                        "Katna draws rounded corners and its own shadow. The frame no longer \
                         follows the {desktop} theme; window rules still apply."
                    ),
                    th,
                ))
            })
            .into_any_element()
    }

    fn blur_switch(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        if Look::blur_available() {
            return self.switch_row(
                "page-blur",
                "Blur what is behind the window",
                "Mail stays on solid cards, so text keeps its contrast",
                self.config.experimental.blur,
                Change::Blur(!self.config.experimental.blur),
                th,
                cx,
            );
        }
        let env = self.chrome.environment();
        let why = match (&env.desktop, env.session) {
            (Desktop::Kde, _) => {
                "KDE's blur effect is off. Turn on Blur in System Settings, Window Management, \
                 Desktop Effects, then open Katna Mail again."
            }
            (Desktop::Gnome, _) => "GNOME does not blur what is behind windows.",
            (_, Session::X11) => "Your window manager does not blur what is behind windows.",
            (_, Session::Wayland) => "Your compositor does not blur what is behind windows.",
        };
        div()
            .flex()
            .flex_col()
            .child(
                // The switch, off and out of reach.
                div()
                    .py(px(8.0))
                    .px(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .opacity(0.45)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_size(px(14.0))
                            .child("Blur what is behind the window"),
                    )
                    .child(switch(0.0, th)),
            )
            .child(explain(why, th))
            .into_any_element()
    }
}

fn explain(text: &'static str, th: &Theme) -> AnyElement {
    explain_owned(text.to_owned(), th)
}

fn explain_owned(text: String, th: &Theme) -> AnyElement {
    div()
        .px(px(8.0))
        .pt(px(4.0))
        .text_size(px(12.0))
        .line_height(px(17.0))
        .font_weight(FontWeight::NORMAL)
        .text_color(rgba(th.text_faint))
        .child(text)
        .into_any_element()
}
