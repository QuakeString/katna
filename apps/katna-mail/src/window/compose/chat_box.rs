// SPDX-License-Identifier: GPL-3.0-or-later

//! The chat view's reply box: a rounded field at the foot of the chat that
//! always answers everyone, as a group chat does. It is the inline reply
//! underneath, so drafts, Ctrl+Enter, spelling and undo send work as
//! they do there; Aa opens the full formatting bar above it, and the
//! paperclip offers pictures, files, a template or another signature. The
//! signature is kept out of sight and added on Send. Replying to an
//! older bubble aims the reply at that mail, keeping what was written.

use gpui::{
    AnyElement, Context, ExternalPaths, Focusable, FontWeight, Window, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_store::MessageId;
use katna_ui::px;
use katna_ui::rich::{Block, Doc};
use katna_ui::unpx;

use super::recipients::Field;
use super::tools::{Popup, above, format_active, format_bar_bg, menu_divider};
use super::{Kind, Mode, Original, SendMail, Threading, draft, para, quote, trim_quote};
use crate::data::EntryKey;
use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, icon_button_colored, menu, menu_item_icon, tip};

use super::super::MailWindow;

/// The field's text grows to this height, then scrolls.
const MAX_TEXT: f32 = 180.0;

/// A reply written in the chat's reply box.
pub(in crate::window) struct ChatReply {
    /// The signature, out of sight while writing: added on Send.
    pub(super) held: Vec<Block>,
}

/// The blocks `signature` adds after a reply's text: a blank line, then
/// the signature.
pub(super) fn held_signature(signature: Option<Doc>) -> Vec<Block> {
    let mut doc = Doc {
        blocks: vec![para("")],
    };
    if let Some(signature) = signature.filter(|s| !s.is_blank()) {
        doc.blocks.push(para(""));
        let at = doc.blocks.len();
        katna_ui::rich::insert_signature_doc(&mut doc, at, signature);
    }
    doc.blocks.split_off(1)
}

impl MailWindow {
    /// The reply being written in the chat of conversation `key`.
    fn chat_compose(&self, key: EntryKey) -> Option<&super::Compose> {
        self.compose
            .as_ref()
            .filter(|c| c.mode == Mode::Inline && !c.closing && c.conversation == Some(key))
    }

    /// The mail the chat's reply answers, and whether it goes to everyone.
    pub(in crate::window) fn chat_reply_source(&self, key: EntryKey) -> Option<(MessageId, bool)> {
        let compose = self.chat_compose(key)?;
        Some((compose.source?, compose.kind == Kind::ReplyAll))
    }

    /// Starts the chat's reply to `source` (the newest mail by default),
    /// to everyone or to its sender only. A reply already written to
    /// starts answering that mail instead, keeping its text.
    pub(in crate::window) fn chat_reply(
        &mut self,
        source: Option<MessageId>,
        kind: Kind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(key) = self.reader.as_ref().map(|r| r.key) else {
            return;
        };
        if self.chat_compose(key).is_some_and(|c| c.touched(cx)) {
            self.aim_chat_reply(source, kind, cx);
        } else {
            self.open_compose(kind, source, window, cx);
            // The signature waits out of sight.
            self.adopt_chat_reply(key, cx);
        }
        if let Some(compose) = &self.compose {
            window.focus(&compose.body.focus_handle(cx), cx);
        }
        cx.notify();
    }

    /// Takes a reply written in conversation `key` before it showed as a
    /// chat (or opened again after Undo) into the reply box: its signature
    /// goes out of sight like the box's own.
    pub(in crate::window) fn adopt_chat_reply(&mut self, key: EntryKey, cx: &mut Context<Self>) {
        let Some(compose) = self.compose.as_mut().filter(|c| {
            c.mode == Mode::Inline && !c.closing && c.conversation == Some(key) && c.chat.is_none()
        }) else {
            return;
        };
        let mut doc = compose.body.read(cx).doc().clone();
        let blank = |b: &Block| matches!(b, Block::Para(p) if p.text.is_empty());
        let held = match doc
            .blocks
            .iter()
            .position(|b| matches!(b, Block::Para(p) if p.style.signature))
        {
            // With the blank line before it, when the text keeps a line.
            Some(at) if at > 1 && blank(&doc.blocks[at - 1]) => doc.blocks.split_off(at - 1),
            Some(at) if at > 0 => doc.blocks.split_off(at),
            _ => Vec::new(),
        };
        if !held.is_empty() {
            compose.body.update(cx, |editor, cx| {
                editor.set_doc(doc.clone(), doc.start(), cx)
            });
        }
        compose.chat = Some(ChatReply { held });
    }

    /// Aims the reply being written at `source`: what it quotes and
    /// threads under, and with [`Kind::Reply`] its sender only.
    fn aim_chat_reply(&mut self, source: Option<MessageId>, kind: Kind, cx: &mut Context<Self>) {
        let Some(reader) = &self.reader else {
            return;
        };
        let Some(view) = reader.view(source) else {
            return;
        };
        let date = view
            .date
            .and_then(|d| format::local(d, &self.tz))
            .map(format::long_date)
            .unwrap_or_default();
        let accounts = &self.accounts;
        let is_me = |email: &str| {
            accounts
                .iter()
                .any(|a| a.address.eq_ignore_ascii_case(email))
        };
        let mut aimed = draft(kind, Some(&Original { view, date }), is_me, None);
        let quoted = trim_quote(&mut aimed.body);
        let thread = Threading::of(kind, Some(view));
        let source = reader.view_id(source);
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.thread = thread;
        compose.source = source;
        compose.kind = kind;
        compose.quote = quote::Quote::hidden_from(quoted);
        compose.chips.set(Field::To, &aimed.to);
        compose.chips.set(Field::Cc, &aimed.cc);
        compose.show_cc = !aimed.cc.is_empty();
        cx.notify();
    }

    /// The reply box at the foot of conversation `key`'s chat. `names`
    /// are the people it goes to; `aimed` the sender and text of an older
    /// mail it answers.
    pub(in crate::window) fn render_chat_reply(
        &self,
        key: EntryKey,
        names: &str,
        aimed: Option<(String, String)>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let compose = self.chat_compose(key);
        let popup = compose.and_then(|c| c.popup.clone());
        let format_on = compose.is_some_and(|c| c.format_bar);
        let width = unpx(self.reader_scroll.bounds().size.width).max(320.0);
        // Opens the reply first when nothing is written yet.
        let start = move |this: &mut Self, window: &mut Window, cx: &mut Context<Self>| {
            if this.chat_compose(key).is_none() {
                this.chat_reply(None, Kind::ReplyAll, window, cx);
            }
        };
        let clip = div()
            .relative()
            .flex_none()
            .child(
                icon_button_colored("chat-attach", "attachment", 20.0, th.text_dim, th)
                    .size(px(38.0))
                    .tooltip(tip(tr!("chat-attach"), th))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        start(this, window, cx);
                        this.toggle_popup(Popup::ChatAttach, cx);
                    })),
            )
            .when(popup == Some(Popup::ChatAttach), |d| {
                d.child(above(self.chat_attach_menu(th, cx)))
            })
            .when(popup == Some(Popup::Templates), |d| {
                d.child(above(self.templates_menu(th, cx)))
            })
            .when(popup == Some(Popup::Signature), |d| {
                d.child(above(self.signature_menu(th, cx)))
            });
        let aa = div()
            .id("chat-format")
            .flex_none()
            .h(px(24.0))
            .px(px(5.0))
            .flex()
            .items_center()
            .rounded(px(6.0))
            .cursor_pointer()
            .text_size(px(13.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgba(if format_on { th.text } else { th.text_faint }))
            .when(format_on, |d| d.bg(rgba(format_active(th))))
            .hover(|s| s.text_color(rgba(th.text)))
            .tooltip(tip(tr!("compose-tool-formatting"), th))
            .on_click(cx.listener(move |this, _, window, cx| {
                start(this, window, cx);
                if let Some(c) = &mut this.compose {
                    c.format_bar = !c.format_bar;
                    c.popup = None;
                    window.focus(&c.body.focus_handle(cx), cx);
                }
                cx.notify();
            }))
            .child("Aa");
        let emoji = div()
            .relative()
            .flex_none()
            .child(
                icon_button_colored("chat-emoji", "emoji", 20.0, th.text_faint, th)
                    .size(px(28.0))
                    .tooltip(tip(tr!("compose-tool-emoji"), th))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        start(this, window, cx);
                        this.toggle_popup(Popup::Emoji, cx);
                        if let Some(c) = &this.compose
                            && c.popup == Some(Popup::Emoji)
                        {
                            let search = c.dialog.emoji_search.clone();
                            search.update(cx, |s, cx| s.set_text("", cx));
                            window.focus(&search.focus_handle(cx), cx);
                        }
                    })),
            )
            .when(popup == Some(Popup::Emoji), |d| {
                d.child(above(self.render_emoji_picker(th, cx)))
            });
        let text = match compose {
            Some(compose) => div()
                .id("chat-text")
                .flex_1()
                .min_w_0()
                .max_h(px(MAX_TEXT))
                .overflow_y_scroll()
                .py(px(9.0))
                .text_size(px(14.0))
                .line_height(px(20.0))
                .cursor_text()
                .child(compose.body.clone())
                .into_any_element(),
            None => div()
                .id("chat-placeholder")
                .flex_1()
                .min_w_0()
                .py(px(9.0))
                .truncate()
                .cursor_text()
                .text_size(px(14.0))
                .line_height(px(20.0))
                .text_color(rgba(th.text_faint))
                .on_click(cx.listener(move |this, _, window, cx| start(this, window, cx)))
                .child(tr!("chat-reply-to", names = names.to_owned()))
                .into_any_element(),
        };
        let field = div()
            .flex_1()
            .min_w_0()
            .min_h(px(40.0))
            .pl(px(6.0))
            .pr(px(10.0))
            .flex()
            .flex_row()
            .items_end()
            .gap(px(6.0))
            .rounded(px(20.0))
            .bg(rgba(th.bubble_other()))
            .child(div().pb(px(6.0)).child(emoji))
            .child(text)
            .child(div().pb(px(8.0)).child(aa));
        let send = div()
            .id("chat-send")
            .flex_none()
            .size(px(40.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(rgba(th.accent))
            .cursor_pointer()
            .hover(|s| s.opacity(0.9))
            .tooltip(tip(tr!("chat-send"), th))
            .on_click(cx.listener(move |this, _, window, cx| {
                if this.chat_compose(key).is_some() {
                    this.send_compose_default(window, cx);
                }
            }))
            .child(icon("send", th.on_accent, 18.0));
        let strip = aimed.map(|(name, said)| {
            div()
                .mx(px(16.0))
                .mb(px(6.0))
                .pl(px(10.0))
                .pr(px(4.0))
                .py(px(4.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .rounded(px(10.0))
                .bg(rgba(th.bubble_other()))
                .border_l_2()
                .border_color(rgba(th.accent))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgba(th.accent))
                                .child(tr!("chat-replying-to", name = name)),
                        )
                        .child(
                            div()
                                .truncate()
                                .text_size(px(13.0))
                                .text_color(rgba(th.text_dim))
                                .child(said),
                        ),
                )
                .child(
                    icon_button_colored("chat-aim-newest", "close", 18.0, th.text_dim, th)
                        .size(px(28.0))
                        .tooltip(tip(tr!("chat-reply-newest"), th))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.aim_chat_reply(None, Kind::ReplyAll, cx);
                        })),
                )
        });
        div()
            .id("chat-reply")
            .key_context("Compose")
            .on_action(
                cx.listener(|this, _: &SendMail, window, cx| this.send_compose_default(window, cx)),
            )
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.drop_on_compose(paths, cx);
            }))
            .flex_none()
            .flex()
            .flex_col()
            .pt(px(6.0))
            .pb(px(12.0))
            .children(strip)
            .when(compose.is_some(), |d| {
                d.child(div().mx(px(16.0)).child(self.render_attachments(th, cx)))
            })
            // The formatting bar sits above the box, pushing the feed up
            // rather than covering it; a narrow pane scrolls it sideways.
            .when(format_on, |d| {
                d.child(
                    div()
                        .id("chat-format-bar")
                        .mx(px(16.0))
                        .mb(px(6.0))
                        .max_w(px(width - 32.0))
                        .rounded_full()
                        .border_1()
                        .border_color(rgba(th.divider))
                        .bg(rgba(format_bar_bg(th)))
                        .overflow_x_scroll()
                        // Room for the last button inside the round end.
                        .child(div().flex_none().pr(px(12.0)).child(self.render_format_bar(
                            th,
                            width - 96.0,
                            cx,
                        ))),
                )
            })
            .child(
                div()
                    .px(px(8.0))
                    .pr(px(12.0))
                    .flex()
                    .flex_row()
                    .items_end()
                    .gap(px(6.0))
                    .child(clip)
                    .child(field)
                    .child(send),
            )
            .when(compose.is_some(), |d| {
                d.children(self.render_popup_scrim(cx))
                    .children(self.render_context_popup(th, cx))
                    .children(self.render_hint(th, cx))
                    .children(self.render_link_bubble(th, cx))
            })
            .into_any_element()
    }

    /// The paperclip's menu: a picture, a file, a template, a signature.
    fn chat_attach_menu(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Div {
        let item = |id: &'static str, name: &'static str, label: String| {
            menu_item_icon(id, name, &label, th)
        };
        menu(th)
            .w(px(220.0))
            .child(
                item("chat-attach-photo", "image", tr!("chat-attach-photo"))
                    .on_click(cx.listener(|this, _, _, cx| this.pick_files(true, cx))),
            )
            .child(
                item("chat-attach-file", "attachment", tr!("chat-attach-file"))
                    .on_click(cx.listener(|this, _, _, cx| this.pick_files(false, cx))),
            )
            .child(menu_divider(th))
            .child(
                item(
                    "chat-attach-template",
                    "template",
                    tr!("chat-attach-template"),
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    this.toggle_popup(Popup::Templates, cx);
                    this.load_templates(cx);
                })),
            )
            .child(
                item(
                    "chat-attach-signature",
                    "signature",
                    tr!("chat-attach-signature"),
                )
                .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Signature, cx))),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_signature_holds_nothing() {
        assert!(held_signature(None).is_empty());
    }
}
