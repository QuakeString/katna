// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Experimental > Look & Feel: Katna's own window frame instead
//! of the desktop's, a blurred, translucent window background, and frosted
//! menus and dialogs. They apply at once to every open window (`katna_chrome::Look`); on Windows
//! the frame changes when a window next opens.

use gpui::{AnyElement, Context, FontWeight, SharedString, div, prelude::*, rgba};
use katna_chrome::{DecorationMode, Desktop, Look, Session};
use katna_core::config::{Config, WindowFrame};
use katna_i18n::tr;
use katna_ui::px;

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
                    .child(tr!("look-intro")),
            )
            .child(div().pt(px(12.0)).child(heading(tr!("look-heading"), th)))
            .child(self.row(
                tr!("look-window-frame"),
                Some(&tr!("look-window-frame-detail")),
                self.frame_choice(th, cx),
                th,
            ))
            .child(self.row(
                tr!("look-blurred-background"),
                Some(&tr!("look-blurred-background-detail")),
                self.blur_switches(th, cx),
                th,
            ))
            .child(div().pt(px(12.0)).child(heading(tr!("chat-heading"), th)))
            .child(self.row(
                tr!("chat-view"),
                Some(&tr!("chat-view-detail")),
                self.switch_row(
                    "page-chat-view",
                    tr!("chat-view-switch"),
                    tr!("chat-view-switch-detail"),
                    self.config.experimental.chat_view,
                    Change::ChatView(!self.config.experimental.chat_view),
                    th,
                    cx,
                ),
                th,
            ))
            .into_any_element()
    }

    fn frame_choice(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let env = self.chrome.environment();
        // GNOME on Wayland leaves every frame to the app: Native already is
        // Katna's frame there.
        if env.native_decorations() == DecorationMode::Client {
            return explain(tr!("look-frame-client-side"), th);
        }
        let note = match env.desktop {
            _ if cfg!(windows) => tr!("look-frame-katna-note-windows"),
            Desktop::Kde => tr!("look-frame-katna-note-named", desktop = "KDE"),
            Desktop::Gnome => tr!("look-frame-katna-note-named", desktop = "GNOME"),
            Desktop::Other(_) => tr!("look-frame-katna-note"),
        };
        let frame = self.config.experimental.window_frame;
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(self.radio_row(
                "page-frame-native",
                match env.desktop {
                    _ if cfg!(windows) => tr!("look-frame-native-windows"),
                    Desktop::Kde => tr!("look-frame-native-kde"),
                    _ => tr!("look-frame-native"),
                },
                frame == WindowFrame::Native,
                Change::WindowFrame(WindowFrame::Native),
                th,
                cx,
            ))
            .child(self.radio_row(
                "page-frame-katna",
                tr!("look-frame-katna"),
                frame == WindowFrame::Katna,
                Change::WindowFrame(WindowFrame::Katna),
                th,
                cx,
            ))
            .when(frame == WindowFrame::Katna, |d| d.child(explain(note, th)))
            // Windows sets a window's frame when it opens.
            .when(self.chrome.frame_on_reopen(), |d| {
                d.child(explain(tr!("look-frame-on-reopen"), th))
            })
            .into_any_element()
    }

    fn blur_switches(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .child(self.window_blur_switch(th, cx))
            .child(self.frosted_popups_switch(th, cx))
            .into_any_element()
    }

    fn window_blur_switch(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        if Look::blur_available() {
            return self.switch_row(
                "page-blur",
                tr!("look-blur"),
                tr!("look-blur-detail"),
                self.config.experimental.blur,
                Change::Blur(!self.config.experimental.blur),
                th,
                cx,
            );
        }
        let env = self.chrome.environment();
        let why = match (&env.desktop, env.session) {
            (Desktop::Kde, _) => tr!("look-blur-off-kde"),
            (Desktop::Gnome, _) => tr!("look-blur-none-gnome"),
            (_, Session::X11) => tr!("look-blur-none-x11"),
            (_, Session::Wayland) => tr!("look-blur-none-wayland"),
        };
        unavailable(tr!("look-blur"), why, th)
    }

    /// Katna blurs under its own menus, so this needs no compositor.
    fn frosted_popups_switch(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        if katna_ui::frost::supported() {
            return self.switch_row(
                "page-frosted-popups",
                tr!("look-frosted-popups"),
                tr!("look-frosted-popups-detail"),
                self.config.experimental.frosted_popups,
                Change::FrostedPopups(!self.config.experimental.frosted_popups),
                th,
                cx,
            );
        }
        let why = if cfg!(windows) {
            tr!("look-frosted-popups-none-windows")
        } else {
            tr!("look-frosted-popups-none")
        };
        unavailable(tr!("look-frosted-popups"), why, th)
    }
}

/// A switch that cannot be used here: off and out of reach, with why.
fn unavailable(label: String, why: String, th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .py(px(8.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .opacity(0.45)
                .child(div().flex_1().min_w_0().text_size(px(14.0)).child(label))
                .child(switch(0.0, th)),
        )
        .child(explain(why, th))
        .into_any_element()
}

fn explain(text: impl Into<SharedString>, th: &Theme) -> AnyElement {
    div()
        .px(px(8.0))
        .pt(px(4.0))
        .text_size(px(12.0))
        .line_height(px(17.0))
        .font_weight(FontWeight::NORMAL)
        .text_color(rgba(th.text_faint))
        .child(text.into())
        .into_any_element()
}
