// SPDX-License-Identifier: GPL-3.0-or-later

//! A conversation in a window of its own, apart from the mail window:
//! Shift+click on a line of the list, "Open in new window" on its
//! right-click menu, or the button on the open conversation's toolbar.
//! A plain click (or double-click) always opens it in place.
//!
//! The window is a second [`MailWindow`] that shows only the reading view.
//! It reads the store itself and follows the daemon's changes, so it stays
//! right when the conversation changes in the main window, and closes when
//! the conversation is archived, deleted or moved from it.

use gpui::{
    AnyElement, Context, Decorations, FontWeight, SharedString, Window, div, prelude::*, rgba, size,
};
use katna_chrome::{Bar, Environment, window_options};
use katna_core::Paths;
use katna_core::ids::MAIL_APP_ID;
use katna_i18n::tr;
use katna_store::{FolderId, MessageId};
use katna_ui::px;
use katna_ui::scale::desktop_px;
use katna_ui::unpx;

use super::compose::Kind;
use super::{Listing, MailWindow, READER_CONTEXT, WINDOW_CONTEXT};
use crate::data::{Entry, EntryKey};
use crate::sidebar::Role;

/// The size a conversation window opens at.
const WIDTH: f32 = 960.0;
const HEIGHT: f32 = 780.0;

/// Where the conversation was opened from, so moving it out (archive,
/// delete) works as it does from the list.
#[derive(Debug, Clone)]
struct Origin {
    folder: Option<FolderId>,
    show_recipients: bool,
    /// A reply of this kind to this message starts at once (a
    /// notification's Reply or Reply all), saying the text if any (typed
    /// into the notification).
    reply: Option<(MessageId, Kind, Option<String>)>,
}

impl MailWindow {
    /// Opens line `ix` of the list in a window of its own.
    pub(super) fn open_in_window(&mut self, ix: usize, cx: &mut Context<Self>) {
        if let Some(entry) = self.entries.get(ix).copied() {
            self.open_entry_in_window(entry, cx);
        }
    }

    /// Opens the open conversation in a window of its own too.
    pub(super) fn open_reader_in_window(&mut self, cx: &mut Context<Self>) {
        if let Some(entry) = self.reader.as_ref().and_then(|r| r.entry()) {
            self.open_entry_in_window(entry, cx);
        }
    }

    pub(super) fn open_entry_in_window(&mut self, entry: Entry, cx: &mut Context<Self>) {
        let origin = Origin {
            folder: self.listed_folder(),
            show_recipients: self.show_recipients,
            reply: None,
        };
        self.open_window(entry, origin, cx);
    }

    /// Opens the conversation of `message` in a window of its own, for a
    /// click on a notification, with a `reply` started if given (its Reply
    /// or Reply all), saying `text` (typed into the notification): the
    /// mail window stays where it is, whatever page it shows.
    pub(super) fn message_in_window(
        &mut self,
        message: MessageId,
        reply: Option<Kind>,
        text: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let mail = self.mail.as_ref().ok();
        let entry = match mail.and_then(|m| m.message_thread(message)) {
            Some(thread) => Entry {
                key: EntryKey::Thread(thread),
                latest: message,
            },
            None => Entry::message(message),
        };
        // Archive and delete work as from the account's Inbox.
        let inbox = mail
            .and_then(|m| m.message_account(message))
            .and_then(|account| self.tree.role_folder(account, Role::Inbox));
        let origin = Origin {
            folder: inbox.or_else(|| self.listed_folder()),
            show_recipients: false,
            reply: reply.map(|kind| (message, kind, text)),
        };
        self.open_window(entry, origin, cx);
    }

    /// Opens `entry` in a new window.
    fn open_window(&mut self, entry: Entry, origin: Origin, cx: &mut Context<Self>) {
        let title = self.line_subject(entry);
        let env = self.chrome.environment();
        let paths = self.paths.clone();
        let font = self.font.clone();
        let options = window_options(
            &env,
            MAIL_APP_ID,
            title.clone(),
            size(desktop_px(WIDTH), desktop_px(HEIGHT)),
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
                    this.show_snackbar(tr!("reader-window-failed"), None, cx);
                });
            }
        });
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
        if let Some((message, kind, text)) = origin.reply {
            if let Some(text) = text.filter(|t| !t.trim().is_empty()) {
                this.keep_reply(entry.key, text);
            }
            this.open_compose(kind, Some(message), window, cx);
        }
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
            let corners = self.chrome.content_corners(window);
            viewer.update(cx, |viewer, _| {
                viewer.th = th;
                viewer.corners = corners;
            });
        }
        let reduce = cx.reduce_motion();
        self.update_reply_row(unpx(window.viewport_size().width), window, reduce);
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
            .on_action(cx.listener(Self::add_to_tasks))
            .on_action(cx.listener(Self::mark_important))
            .on_action(cx.listener(Self::toggle_mute))
            .on_action(cx.listener(Self::mark_not_important))
            .child(self.render_reader_card(&th, cx));
        let compose = self.render_compose(&th, window, reduce, cx);
        let context_menu = self.render_context_menu(&th, window, cx);
        let snackbar = self.render_snackbar(&th, window, reduce, cx);
        self.dialog_text.begin(());
        let whats_new = self.render_whats_new(&th, window, reduce, cx);
        let about = self.render_about(&th, window, reduce, cx);
        let update_dialog = self.render_update_dialog(&th, window, reduce, cx);
        let content = div()
            .key_context(WINDOW_CONTEXT)
            .relative()
            .size_full()
            .p(px(8.0))
            .when(!server_frame, |d| d.pt_0())
            .bg(rgba(th.backdrop))
            .text_color(rgba(th.text))
            // Where the menu bar's actions start when the focus is lost.
            .child(div().absolute().size_0().track_focus(&self.window_focus))
            .child(card)
            .children(self.files.viewer.clone())
            .children(compose)
            .children(context_menu)
            .children(snackbar)
            .children(whats_new)
            .children(about)
            .children(update_dialog)
            .into_any_element();
        // The desktop's own title bar already names the window.
        if server_frame {
            let page = Self::detached_actions(div().size_full().child(content), cx);
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
        let frame = Self::detached_actions(self.chrome.render_bar(bar, content, window, cx), cx);
        match &self.font {
            Some(font) => frame.font_family(font.clone()).into_any_element(),
            None => frame.into_any_element(),
        }
    }

    /// The conversation window's own actions, on its outermost element so
    /// they run wherever the keyboard focus is.
    fn detached_actions(page: gpui::Div, cx: &mut Context<Self>) -> gpui::Div {
        page.on_action(cx.listener(Self::reply))
            .on_action(cx.listener(Self::reply_all))
            .on_action(cx.listener(Self::forward))
            .on_action(cx.listener(Self::move_to))
            .on_action(cx.listener(Self::undo_action))
            .on_action(cx.listener(Self::quit))
            .on_action(cx.listener(Self::show_whats_new_action))
            .on_action(cx.listener(Self::check_for_updates_action))
            .on_action(cx.listener(Self::show_about))
    }
}
