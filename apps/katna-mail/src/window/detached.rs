// SPDX-License-Identifier: GPL-3.0-or-later

//! A conversation in a window of its own: double-clicking a line of the
//! list opens it there, apart from the mail window.
//!
//! The window is a second [`MailWindow`] that shows only the reading view.
//! It reads the store itself and follows the daemon's changes, so it stays
//! right when the conversation changes in the main window, and closes when
//! the conversation is archived, deleted or moved from it.

use std::time::Duration;

use gpui::{
    AnyElement, Context, Decorations, FontWeight, MouseDownEvent, SharedString, Window, div,
    prelude::*, px, rgba, size,
};
use katna_chrome::{Bar, Environment, window_options};
use katna_core::Paths;
use katna_core::ids::MAIL_APP_ID;
use katna_store::FolderId;

use super::{Listing, MailWindow, READER_CONTEXT, WINDOW_CONTEXT};
use crate::data::Entry;

/// How soon a second click must follow the first to count as a double
/// click, at most.
const DOUBLE_CLICK: Duration = Duration::from_millis(600);

/// The size a conversation window opens at.
const WIDTH: f32 = 960.0;
const HEIGHT: f32 = 780.0;

/// Where the conversation was opened from, so moving it out (archive,
/// delete) works as it does from the list.
#[derive(Debug, Clone, Copy)]
struct Origin {
    folder: Option<FolderId>,
    show_recipients: bool,
}

impl MailWindow {
    /// Opens line `ix` of the list in a window of its own.
    pub(super) fn open_in_window(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get(ix).copied() else {
            return;
        };
        let title = self.line_subject(entry);
        let env = self.chrome.environment();
        let paths = self.paths.clone();
        let font = self.font.clone();
        let origin = Origin {
            folder: self.listed_folder(),
            show_recipients: self.show_recipients,
        };
        let options = window_options(
            &env,
            MAIL_APP_ID,
            title.clone(),
            size(px(WIDTH), px(HEIGHT)),
            cx,
        );
        let this = cx.entity();
        let main = this.downgrade();
        // Opened after this update, which holds the mail window.
        cx.defer(move |cx| {
            let opened = cx.open_window(options, |window, cx| {
                cx.new(|cx| {
                    let mut view =
                        MailWindow::detached(env, paths, font, entry, origin, window, cx);
                    view.main = Some(main);
                    view
                })
            });
            if let Err(err) = opened {
                tracing::warn!("cannot open a message window: {err}");
                this.update(cx, |this, cx| {
                    this.show_snackbar("Could not open a new window.", None, cx);
                });
            }
        });
    }

    /// Without the reading pane, a line's first click replaces the list
    /// with the conversation, so the second click of a double-click lands
    /// there: it moves the conversation to its own window.
    pub(super) fn double_click_reader(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((at, ix)) = self.clicked else {
            return;
        };
        if event.click_count < 2 || !self.reading || self.split() || at.elapsed() > DOUBLE_CLICK {
            return;
        }
        self.clicked = None;
        cx.stop_propagation();
        self.close_message(&super::CloseMessage, window, cx);
        self.open_in_window(ix, cx);
    }

    /// The subject of a line, for the new window's title.
    fn line_subject(&mut self, entry: Entry) -> String {
        let folder = self.listed_folder();
        let show_recipients = self.show_recipients;
        self.mail
            .as_mut()
            .ok()
            .and_then(|mail| mail.rows(&[entry], folder, show_recipients).pop().flatten())
            .map_or_else(|| "Katna Mail".to_owned(), |row| row.subject.clone())
    }

    /// The view of a conversation window.
    fn detached(
        env: Environment,
        paths: Paths,
        font: Option<SharedString>,
        entry: Entry,
        origin: Origin,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::build(env, paths, font, window, cx);
        this.detached = true;
        this.load_tree();
        this.folder = origin.folder;
        this.listing = origin.folder.map(Listing::Folder);
        this.show_recipients = origin.show_recipients;
        this.entries = vec![entry];
        this.selected = Some(0);
        this.reading = true;
        this.load_reader(0, cx);
        this.listen(cx);
        this.watch_colors(cx);
        window.focus(&this.list_focus, cx);
        this
    }

    /// A conversation window: the reading view with its toolbar, and what
    /// opens over it.
    pub(super) fn render_detached(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Archived, deleted or moved away (or closed from its toolbar):
        // nothing is left to show.
        if !self.reading || self.reader.is_none() {
            window.remove_window();
            return div().into_any_element();
        }
        let th = self.theme(window);
        self.release_images(window, cx);
        if let Some(viewer) = &self.files.viewer {
            viewer.update(cx, |viewer, _| viewer.th = th);
        }
        let reduce = cx.reduce_motion();
        self.update_reply_row(f32::from(window.viewport_size().width), window, reduce);
        let title = self
            .reader
            .as_ref()
            .map(|r| r.subject().to_owned())
            .unwrap_or_default();
        window.set_window_title(&title);
        let server_frame = matches!(window.window_decorations(), Decorations::Server);

        let card = div()
            .key_context(READER_CONTEXT)
            .track_focus(&self.list_focus)
            .size_full()
            .on_action(cx.listener(Self::close_message))
            .on_action(cx.listener(Self::scroll_down))
            .on_action(cx.listener(Self::scroll_up))
            .on_action(cx.listener(Self::scroll_page_down))
            .on_action(cx.listener(Self::scroll_page_up))
            .on_action(cx.listener(Self::archive))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::report_spam))
            .on_action(cx.listener(Self::mark_read))
            .on_action(cx.listener(Self::mark_unread))
            .on_action(cx.listener(Self::toggle_star))
            .on_action(cx.listener(Self::mark_important))
            .on_action(cx.listener(Self::mark_not_important))
            .child(self.render_reader_card(&th, cx));
        let compose = self.render_compose(&th, window, reduce, cx);
        let context_menu = self.render_context_menu(&th, window, cx);
        let snackbar = self.render_snackbar(&th, window, reduce, cx);
        let content = div()
            .key_context(WINDOW_CONTEXT)
            .relative()
            .size_full()
            .p(px(8.0))
            .when(!server_frame, |d| d.pt_0())
            .bg(rgba(th.backdrop))
            .text_color(rgba(th.text))
            .on_action(cx.listener(Self::reply))
            .on_action(cx.listener(Self::reply_all))
            .on_action(cx.listener(Self::forward))
            .on_action(cx.listener(Self::move_to))
            .on_action(cx.listener(Self::undo_action))
            .on_action(cx.listener(Self::quit))
            .child(card)
            .children(self.files.viewer.clone())
            .children(compose)
            .children(context_menu)
            .children(snackbar)
            .into_any_element();
        // The desktop's own title bar already names the window.
        if server_frame {
            let page = div().size_full().child(content);
            return match &self.font {
                Some(font) => page.font_family(font.clone()).into_any_element(),
                None => page.into_any_element(),
            };
        }
        let bar = Bar {
            center: Some(
                div()
                    .max_w(px(WIDTH - 200.0))
                    .truncate()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(title)
                    .into_any_element(),
            ),
            background: Some(th.backdrop),
            ..Bar::default()
        };
        let frame = self.chrome.render_bar(bar, content, window, cx);
        match &self.font {
            Some(font) => frame.font_family(font.clone()).into_any_element(),
            None => frame.into_any_element(),
        }
    }
}
