// SPDX-License-Identifier: GPL-3.0-or-later

//! The message being written in a window of its own, apart from the mail
//! window: the expand button in the compose title bar pops it out, and a
//! button in the new window's bar docks it back.
//!
//! The message stays in [`MailWindow::compose`]; the window only draws it,
//! so sending, the snackbar and the outbox work as in the docked window.

use gpui::{
    AnyElement, App, Context, Decorations, Entity, ExternalPaths, FocusHandle, Focusable,
    FontWeight, Window, WindowBounds, WindowHandle, div, point, prelude::*, rgba, size,
};
use katna_chrome::{Bar, WindowChrome, window_options};
use katna_core::ids::MAIL_APP_ID;
use katna_ui::px;
use katna_ui::scale::desktop_px;
use katna_ui::unpx;

use super::super::MailWindow;
use super::Mode;
use crate::theme::Theme;
use crate::widgets::{icon, tip};

/// The size a popped-out message opens at.
const WIDTH: f32 = 720.0;
const HEIGHT: f32 = 660.0;

/// The root view of a popped-out compose window.
pub(in crate::window) struct ComposeWindow {
    mail: Entity<MailWindow>,
    chrome: WindowChrome,
    focus: FocusHandle,
}

impl MailWindow {
    /// Moves the message being written into a window of its own, or brings
    /// that window forward if it is already out.
    pub(in crate::window) fn pop_out_compose(&mut self, window: &Window, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.popup = None;
        if let Some(handle) = self.writing.compose_window
            && handle
                .update(cx, |_, window, _| window.activate_window())
                .is_ok()
        {
            return;
        }
        let env = self.chrome.environment();
        let mail = cx.entity();
        let body = compose.body.focus_handle(cx);
        let to = compose.to.focus_handle(cx);
        let empty_to = compose.to.read(cx).text().is_empty();
        let mut options = window_options(
            &env,
            MAIL_APP_ID,
            compose.title(cx),
            size(desktop_px(WIDTH), desktop_px(HEIGHT)),
            cx,
        );
        // Over the middle of the mail window, where the message was. On
        // Wayland the compositor places it instead.
        if let Some(WindowBounds::Windowed(bounds)) = &mut options.window_bounds {
            let main = window.bounds();
            bounds.origin = point(
                main.origin.x + (main.size.width - bounds.size.width) / 2.0,
                main.origin.y + (main.size.height - bounds.size.height) / 2.0,
            );
        }
        // The new window's first frame already looks for it there.
        compose.mode = Mode::Window;
        // Opened after this update, which holds the mail window.
        cx.defer(move |cx| {
            let closing = mail.clone();
            let opened = cx.open_window(options, |window, cx| {
                window.focus(if empty_to { &to } else { &body }, cx);
                // Closing the window closes the message, as its close
                // button in the mail window does.
                window.on_window_should_close(cx, move |_, cx| {
                    closing.update(cx, |this, cx| {
                        this.writing.compose_window = None;
                        let touched = this.compose.as_ref().is_some_and(|c| c.touched(cx));
                        if this
                            .compose
                            .as_ref()
                            .is_some_and(|c| c.mode == Mode::Window)
                        {
                            this.close_compose(touched, cx);
                        }
                    });
                    true
                });
                cx.new(|cx| ComposeWindow {
                    mail: mail.clone(),
                    // All of it is the message, on an opaque card.
                    chrome: WindowChrome::new(env, "New Message", window, cx).opaque(),
                    focus: cx.focus_handle(),
                })
            });
            mail.update(cx, |this, cx| {
                match opened {
                    Ok(handle) => this.writing.compose_window = Some(handle),
                    Err(err) => {
                        if let Some(c) = &mut this.compose {
                            c.mode = c.docked_mode();
                        }
                        tracing::warn!("cannot open a compose window: {err}");
                        this.show_snackbar("Could not open a new window.", None, cx);
                    }
                }
                cx.notify();
            });
        });
    }

    /// Puts the popped-out message back in the mail window: a reply at the
    /// end of its conversation, anything else in the compose window.
    pub(super) fn dock_compose(&mut self, cx: &mut Context<Self>) {
        let mut inline = false;
        if let Some(c) = &mut self.compose {
            c.mode = c.docked_mode();
            c.popup = None;
            c.shown.snap(1.0);
            inline = c.mode == Mode::Inline;
        }
        self.close_compose_window(cx);
        if inline {
            self.reveal_inline_reply(cx);
        }
        cx.notify();
    }

    /// Closes the popped-out window, if any. The message itself is left as
    /// it is.
    pub(in crate::window) fn close_compose_window(&mut self, cx: &mut Context<Self>) {
        if let Some(handle) = self.writing.compose_window.take() {
            // Not from inside that window's own update.
            cx.defer(move |cx| {
                handle
                    .update(cx, |_, window, _| window.remove_window())
                    .ok();
            });
        }
    }

    /// The popped-out message filling `window`: the fields, the text, the
    /// attachments and the bars, without the docked window's title bar.
    fn render_compose_in_window(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        if compose.mode != Mode::Window || compose.closing {
            return None;
        }
        let width = unpx(window.viewport_size().width).max(360.0);
        // With the desktop's own title bar the back button moves to the
        // bottom bar (`tools`).
        self.writing.popout_server_frame =
            matches!(window.window_decorations(), Decorations::Server);
        let panel = div()
            .id("compose-window")
            .key_context("Compose")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.drop_files(paths.paths().to_vec(), cx);
            }))
            .child(self.render_compose_fields(th, cx))
            .child(self.render_compose_body(th, width, cx))
            .child(self.render_attachments(th, cx))
            .children(self.render_floating_format_bar(th, width - 24.0, cx))
            .child(self.render_compose_actions(th, width, cx))
            .child(self.render_drop_target(th))
            .children(self.render_compose_dialog(th, cx));
        Some(panel.into_any_element())
    }
}

impl Focusable for ComposeWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for ComposeWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Text without a size of its own follows Settings > Appearance > Scaling.
        window.set_rem_size(px(16.0));
        self.chrome.sync_look(window, cx);
        let chrome = &self.chrome;
        let drawn = self.mail.update(cx, |mail, cx| {
            let th = mail.theme_for(chrome, window);
            let content = mail.render_compose_in_window(&th, window, cx)?;
            let title = mail.compose.as_ref()?.title(cx);
            let dock = dock_button(&th, cx);
            Some((th, content, title, dock, mail.font.clone()))
        });
        let Some((th, content, title, dock, font)) = drawn else {
            // Sent, discarded or docked: the window has nothing left to show.
            window.remove_window();
            return div().into_any_element();
        };
        window.set_window_title(&title);
        // The desktop's title bar already names the window and holds its
        // buttons; no toolbar repeats it.
        if matches!(window.window_decorations(), Decorations::Server) {
            let frame = div()
                .size_full()
                .bg(rgba(th.surface))
                .text_color(rgba(th.text))
                .child(content);
            return match font {
                Some(font) => frame.font_family(font).into_any_element(),
                None => frame.into_any_element(),
            };
        }
        let bar = Bar {
            center: Some(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(title)
                    .into_any_element(),
            ),
            end: vec![dock],
            background: Some(th.surface),
            ..Bar::default()
        };
        let frame = self.chrome.render_bar(bar, content, window, cx);
        match font {
            Some(font) => frame.font_family(font).into_any_element(),
            None => frame.into_any_element(),
        }
    }
}

/// The bar button that puts the message back in the mail window.
fn dock_button(th: &Theme, cx: &mut Context<MailWindow>) -> AnyElement {
    div()
        .id("compose-dock")
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .tooltip(tip("Back to the mail window", th))
        .on_click(cx.listener(|this, _, _, cx| this.dock_compose(cx)))
        .child(icon("close-full", th.text_dim, 18.0))
        .into_any_element()
}

/// The window handle's type, for [`super::Writing`].
pub(in crate::window) type Handle = WindowHandle<ComposeWindow>;
