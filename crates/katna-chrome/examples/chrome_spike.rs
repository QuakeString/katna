// SPDX-License-Identifier: GPL-3.0-or-later

//! Spike S1 (`docs/IMPLEMENTATION_PLAN.md` §4): a Katna Mail-shaped window
//! with native-looking chrome on KDE and GNOME.
//!
//! ```sh
//! cargo run -p katna-chrome --example chrome-spike
//! KATNA_DECORATIONS=client cargo run -p katna-chrome --example chrome-spike
//! ```
//!
//! The window shows what was detected and negotiated, and prints a line to
//! stderr whenever that changes, so the spike can be checked from a script.

use gpui::{
    AnyElement, App, Context, Decorations, IntoElement, ParentElement, Render, SharedString,
    Styled, Window, WindowAppearance, div, prelude::*, rgba, size,
};
use katna_chrome::tokens::with_alpha;
use katna_chrome::{Environment, WindowChrome, window_options};
use katna_core::ids::MAIL_APP_ID;
use katna_ui::px;
use katna_ui::unpx;

struct Spike {
    chrome: WindowChrome,
    last_report: SharedString,
}

impl Spike {
    fn report(&self, window: &Window, cx: &App) -> String {
        let env = self.chrome.environment();
        let negotiated = match window.window_decorations() {
            Decorations::Server => "server".to_owned(),
            Decorations::Client { tiling } => format!(
                "client tiling=[{}{}{}{}]",
                if tiling.top { "T" } else { "-" },
                if tiling.right { "R" } else { "-" },
                if tiling.bottom { "B" } else { "-" },
                if tiling.left { "L" } else { "-" },
            ),
        };
        let layout = cx
            .button_layout()
            .map(|l| {
                let side = |s: &[Option<gpui::WindowButton>]| {
                    s.iter()
                        .flatten()
                        .map(|b| b.id())
                        .collect::<Vec<_>>()
                        .join(",")
                };
                format!("{}:{}", side(&l.left), side(&l.right))
            })
            .unwrap_or_else(|| "none".into());
        let bounds = window.bounds();
        format!(
            "desktop={:?} session={:?} requested={:?} negotiated={} maximized={} \
             active={} dark={} button-layout={} viewport={}x{} bounds={}x{}",
            env.desktop,
            env.session,
            env.requested_decorations(),
            negotiated,
            window.is_maximized(),
            window.is_window_active(),
            matches!(
                window.appearance(),
                WindowAppearance::Dark | WindowAppearance::VibrantDark
            ),
            layout,
            unpx(window.viewport_size().width),
            unpx(window.viewport_size().height),
            unpx(bounds.size.width),
            unpx(bounds.size.height),
        )
    }
}

impl Render for Spike {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let report = self.report(window, cx);
        if report != self.last_report.as_ref() {
            eprintln!("chrome-spike: {report}");
            self.last_report = report.into();
        }
        let t = self.chrome.tokens(window);
        let pill = |label: &'static str| -> AnyElement {
            div()
                .id(label)
                .px(px(10.0))
                .py(px(4.0))
                .rounded(px(6.0))
                .bg(rgba(t.button_bg))
                .hover(|s| s.bg(rgba(t.button_bg_hover)))
                .text_size(px(13.0))
                .child(label)
                .into_any_element()
        };
        let sidebar = div()
            .w(px(220.0))
            .h_full()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .p(px(8.0))
            .border_r_1()
            .border_color(rgba(t.header_shade))
            .children(
                ["Inbox", "Starred", "Sent", "Drafts", "Acme Corp", "Initech"]
                    .into_iter()
                    .enumerate()
                    .map(|(i, name)| {
                        div()
                            .px(px(10.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .when(i == 0, |d| d.bg(rgba(with_alpha(t.accent, 0x33))))
                            .text_size(px(14.0))
                            .child(name)
                    }),
            );
        let details = div()
            .flex_1()
            .p(px(16.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .text_size(px(13.0))
            .child(div().text_size(px(18.0)).child("Window chrome spike (S1)"))
            .children(
                self.last_report
                    .split(' ')
                    .map(|kv| div().text_color(rgba(t.fg_dim)).child(kv.to_owned())),
            );
        let content = div()
            .size_full()
            .flex()
            .flex_row()
            .child(sidebar)
            .child(details)
            .into_any_element();

        self.chrome.render(
            vec![pill("Compose")],
            vec![pill("Search")],
            content,
            window,
            cx,
        )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let env = Environment::from_env();
        let options = window_options(&env, MAIL_APP_ID, "Inbox", size(px(960.0), px(640.0)), cx);
        cx.open_window(options, |window, cx| {
            cx.new(|cx| Spike {
                chrome: WindowChrome::new(env, "Inbox", window, cx),
                last_report: SharedString::default(),
            })
        })
        .expect("failed to open the window");
        cx.on_window_closed(|cx, _| cx.quit()).detach();
    });
}
