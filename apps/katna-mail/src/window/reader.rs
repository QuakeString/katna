// SPDX-License-Identifier: GPL-3.0-or-later

//! The open conversation, laid out like webmail: an action bar (back,
//! archive, spam, delete, mark unread, move, more, "3 of 120"), the subject
//! with its folder chip, the messages (older ones folded to one line, a
//! "4 older messages" fold in long threads, the newest open) and, pinned
//! at the foot, Reply, Reply all and Forward, or the reply being written.

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, FontWeight, SharedString, div, ease_out_quint,
    prelude::*, px, rgba,
};
use katna_render::MessageView;
use katna_store::MessageId;

use super::compose::Kind;
use super::list::separator;
use super::{MailWindow, Menu, SelectNext, SelectPrevious};
use crate::daemon::Command;
use crate::data::{EntryKey, Mail, Row};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{
    avatar, icon, icon_button, icon_button_colored, pill_button, placeholder, tip, toolbar,
};

/// The reading view shows at most this many lines of a body.
const MAX_BODY_LINES: usize = 4000;
/// Fold the middle of a conversation when this many messages in a row are
/// folded.
const FOLD_AT: usize = 3;

/// An open conversation (or a single message).
pub(super) struct Conversation {
    pub key: EntryKey,
    subject: String,
    parts: Vec<Part>,
    /// The folded middle of a long conversation is shown.
    show_all: bool,
}

/// One message of the conversation.
struct Part {
    id: MessageId,
    row: Option<Rc<Row>>,
    expanded: bool,
    /// Loaded when first expanded.
    body: Option<Body>,
    /// The "to me, Bob ▾" details are open.
    details: bool,
}

struct Body {
    /// `None` when the message body is not stored (not downloaded yet).
    view: Option<MessageView>,
    /// The body, in blocks of consecutive quoted or unquoted lines.
    blocks: Vec<(bool, SharedString)>,
    cut: bool,
}

impl Conversation {
    /// The loaded message `id`, or by default the newest loaded one: what a
    /// reply or forward starts from.
    pub(super) fn view(&self, id: Option<MessageId>) -> Option<&MessageView> {
        fn view(part: &Part) -> Option<&MessageView> {
            part.body.as_ref().and_then(|b| b.view.as_ref())
        }
        match id {
            Some(id) => self.parts.iter().find(|p| p.id == id).and_then(view),
            None => self.parts.iter().rev().find_map(view),
        }
    }

    pub(super) fn load(mail: &mut Mail, key: EntryKey) -> Self {
        let ids = mail.entry_messages(key);
        let rows = mail.message_rows(&ids);
        let last = ids.len().saturating_sub(1);
        let parts: Vec<Part> = ids
            .iter()
            .zip(rows)
            .enumerate()
            .map(|(ix, (id, row))| {
                let unread = row.as_ref().is_some_and(|r| r.unread);
                let mut part = Part {
                    id: *id,
                    row,
                    expanded: ix == last || unread,
                    body: None,
                    details: false,
                };
                if part.expanded {
                    part.body = Some(read(mail, part.id));
                }
                part
            })
            .collect();
        let subject = parts
            .iter()
            .find_map(|p| p.row.as_ref().map(|r| r.subject.clone()))
            .unwrap_or_else(|| "(no subject)".to_owned());
        Self {
            key,
            subject,
            parts,
            show_all: false,
        }
    }

    /// Reads the messages' flags again, keeping what is open.
    pub(super) fn refresh(&mut self, mail: &mut Mail) {
        let ids = mail.entry_messages(self.key);
        let rows = mail.message_rows(&ids);
        let mut old: Vec<Part> = std::mem::take(&mut self.parts);
        self.parts = ids
            .into_iter()
            .zip(rows)
            .map(|(id, row)| match old.iter().position(|p| p.id == id) {
                Some(ix) => {
                    let mut part = old.swap_remove(ix);
                    part.row = row;
                    part
                }
                None => Part {
                    id,
                    row,
                    expanded: true,
                    body: Some(read(mail, id)),
                    details: false,
                },
            })
            .collect();
    }

    pub(super) fn unread_messages(&self) -> Vec<MessageId> {
        self.parts
            .iter()
            .filter(|p| p.row.as_ref().is_some_and(|r| r.unread))
            .map(|p| p.id)
            .collect()
    }

    fn toggle(&mut self, ix: usize, mail: &Mail) {
        let Some(part) = self.parts.get_mut(ix) else {
            return;
        };
        part.expanded = !part.expanded;
        if part.expanded && part.body.is_none() {
            part.body = Some(read(mail, part.id));
        }
    }

    fn set_all(&mut self, expanded: bool, mail: &Mail) {
        let last = self.parts.len().saturating_sub(1);
        for (ix, part) in self.parts.iter_mut().enumerate() {
            part.expanded = expanded || ix == last;
            if part.expanded && part.body.is_none() {
                part.body = Some(read(mail, part.id));
            }
        }
        self.show_all = expanded;
    }

    fn all_expanded(&self) -> bool {
        self.parts.iter().all(|p| p.expanded)
    }

    /// Each open, downloaded message of the conversation.
    pub(super) fn open_views(&self) -> impl Iterator<Item = (MessageId, &MessageView)> {
        self.parts
            .iter()
            .filter(|p| p.expanded)
            .filter_map(|p| Some((p.id, p.body.as_ref()?.view.as_ref()?)))
    }
}

/// What the list of messages shows: a message, or a fold of several.
enum Shown {
    Part(usize),
    Fold(usize),
}

impl MailWindow {
    /// The reading pane beside the list: its own card.
    pub(super) fn render_reader_card(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .id("reader-card")
            .size_full()
            .flex()
            .flex_col()
            .rounded(px(super::PANEL_RADIUS))
            .overflow_hidden()
            .bg(rgba(th.surface))
            .child(self.render_reader_toolbar(th, cx))
            .child(div().flex_1().min_h_0().child(self.render_reader(th, cx)))
            .into_any_element()
    }

    pub(super) fn render_reader_toolbar(
        &mut self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let count = self.entries.len();
        let position = self
            .reader
            .as_ref()
            .and_then(|r| self.entries.iter().position(|e| e.key == r.key));
        let ix = position.or(self.selected).unwrap_or(0);
        let back = if self.split() {
            icon_button("reader-close", "close", 20.0, th).tooltip(tip("Close", th))
        } else {
            icon_button("reader-back", "back", 20.0, th).tooltip(tip("Back", th))
        }
        .on_click(
            cx.listener(|this, _, window, cx| this.close_message(&super::CloseMessage, window, cx)),
        );
        let narrow = self.split() && self.cards_width * self.config.mail.reading_pane_share < 520.0;
        toolbar(th)
            .child(back)
            .child(separator(th))
            .child(self.action_buttons("reader", th, cx))
            .child(separator(th))
            .child(
                icon_button("reader-unread", "mail", 20.0, th)
                    .tooltip(tip("Mark as unread", th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.mark_unread(&super::MarkUnread, window, cx)
                    })),
            )
            .when(!narrow, |d| {
                d.child({
                    let move_to = icon_button("reader-move", "move-to", 20.0, th)
                        .tooltip(tip("Move to", th))
                        .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::MoveTo, cx)));
                    self.with_menu(move_to, Menu::MoveTo, th, cx)
                })
            })
            .child({
                let more = icon_button("reader-more", "more", 20.0, th)
                    .tooltip(tip("More", th))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::ReaderMore, cx)));
                self.with_menu(more, Menu::ReaderMore, th, cx)
            })
            .child(div().flex_1())
            .when(!narrow && count > 0, |d| {
                d.child(
                    div()
                        .px(px(8.0))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(format!(
                            "{} of {}",
                            format::thousands(ix as u64 + 1),
                            format::thousands(count as u64)
                        )),
                )
            })
            .child(
                icon_button("newer", "chevron-left", 20.0, th)
                    .tooltip(tip("Newer", th))
                    .when(ix == 0, |d| d.opacity(0.4))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.select_previous(&SelectPrevious, window, cx)
                    })),
            )
            .child(
                icon_button("older", "chevron-right", 20.0, th)
                    .tooltip(tip("Older", th))
                    .when(ix + 1 >= count, |d| d.opacity(0.4))
                    .on_click(
                        cx.listener(|this, _, window, cx| {
                            this.select_next(&SelectNext, window, cx)
                        }),
                    ),
            )
            .into_any_element()
    }

    pub(super) fn render_reader(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        self.request_thumbnails(cx);
        let Some(reader) = &self.reader else {
            return placeholder("", th);
        };
        if reader.parts.is_empty() {
            return placeholder("This conversation was removed.", th);
        }
        let all_expanded = reader.all_expanded();
        let title = div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .pl(px(72.0))
            .pr(px(16.0))
            .pt(px(20.0))
            .pb(px(12.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w_full()
                            .text_size(px(22.0))
                            .line_height(px(28.0))
                            .text_color(rgba(th.text))
                            .child(reader.subject.clone()),
                    )
                    .when_some(self.folder_name(), |d, folder| {
                        d.child(
                            div()
                                .px(px(6.0))
                                .py(px(1.0))
                                .rounded(px(4.0))
                                .bg(rgba(th.chip))
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(folder),
                        )
                    }),
            )
            .when(reader.parts.len() > 1, |d| {
                d.child(
                    icon_button_colored(
                        "expand-all",
                        "expand",
                        20.0,
                        if all_expanded { th.accent } else { th.text_dim },
                        th,
                    )
                    .tooltip(tip(
                        if all_expanded {
                            "Collapse all"
                        } else {
                            "Expand all"
                        },
                        th,
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let (Some(reader), Ok(mail)) = (&mut this.reader, &this.mail) {
                            reader.set_all(!all_expanded, mail);
                        }
                        cx.notify();
                    })),
                )
            });

        // Fold runs of collapsed messages in the middle.
        let mut shown = Vec::new();
        let n = reader.parts.len();
        let mut ix = 0;
        while ix < n {
            let run_end = (ix..n).find(|&j| reader.parts[j].expanded).unwrap_or(n);
            let run = run_end - ix;
            if !reader.show_all && ix > 0 && run >= FOLD_AT && run_end < n {
                shown.push(Shown::Fold(run));
                ix = run_end;
            } else {
                shown.push(Shown::Part(ix));
                ix += 1;
            }
        }
        let parts: Vec<AnyElement> = shown
            .into_iter()
            .map(|s| match s {
                Shown::Part(ix) => self.render_part(ix, th, cx),
                Shown::Fold(count) => fold(count, th, cx),
            })
            .collect();

        // Reply, Reply all and Forward stay at the foot of the pane while
        // the conversation scrolls; a reply is written there too.
        let key = reader.key;
        let max_body = f32::from(self.reader_scroll.bounds().size.height) * 0.45;
        let footer = self
            .render_inline_reply(key, max_body, th, cx)
            .unwrap_or_else(|| {
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(12.0))
                    .pl(px(72.0))
                    .pr(px(24.0))
                    .py(px(14.0))
                    .child(
                        pill_button("reply", "reply", "Reply", th).on_click(cx.listener(
                            |this, _, window, cx| this.open_compose(Kind::Reply, None, window, cx),
                        )),
                    )
                    .child(
                        pill_button("reply-all", "reply-all", "Reply all", th).on_click(
                            cx.listener(|this, _, window, cx| {
                                this.open_compose(Kind::ReplyAll, None, window, cx)
                            }),
                        ),
                    )
                    .child(
                        pill_button("forward", "forward", "Forward", th).on_click(cx.listener(
                            |this, _, window, cx| {
                                this.open_compose(Kind::Forward, None, window, cx)
                            },
                        )),
                    )
                    .into_any_element()
            });
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .id("reader")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&self.reader_scroll)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .pb(px(24.0))
                            .child(title)
                            .children(parts)
                            .with_animation(
                                ("open-conversation", key_number(key)),
                                Animation::new(Duration::from_millis(280))
                                    .with_easing(ease_out_quint()),
                                |el, t| el.opacity(t).mt(px(14.0 * (1.0 - t))),
                            ),
                    ),
            )
            .child(
                div()
                    .flex_none()
                    .border_t_1()
                    .border_color(rgba(th.divider))
                    .child(footer),
            )
            .into_any_element()
    }

    fn render_part(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(reader) = &self.reader else {
            return div().into_any_element();
        };
        let part = &reader.parts[ix];
        let last = ix + 1 == reader.parts.len();
        let row = part.row.clone();
        let view = part.body.as_ref().and_then(|b| b.view.as_ref());
        let (name, email) = match (view.and_then(|v| v.from.first()), &row) {
            (Some(from), _) => (from.label().to_owned(), from.email.clone()),
            (None, Some(row)) => (row.correspondent.clone(), String::new()),
            (None, None) => ("(unknown sender)".to_owned(), String::new()),
        };
        let now = jiff::Timestamp::now().as_second();
        let date = row
            .as_ref()
            .and_then(|r| r.date)
            .or(view.and_then(|v| v.date));
        let unread = row.as_ref().is_some_and(|r| r.unread);
        let flagged = row.as_ref().is_some_and(|r| r.flagged);
        let id = part.id;
        let toggle = cx.listener(move |this, _, _, cx| {
            if let (Some(reader), Ok(mail)) = (&mut this.reader, &this.mail) {
                reader.toggle(ix, mail);
            }
            cx.notify();
        });

        if !part.expanded {
            let snippet = row.as_ref().map(|r| r.snippet.clone()).unwrap_or_default();
            let short_date = date
                .and_then(|d| format::local(d, &self.tz))
                .zip(format::local(now, &self.tz))
                .map(|(d, now)| format::list_date(d, now))
                .unwrap_or_default();
            return div()
                .id(("part", ix))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .pl(px(16.0))
                .pr(px(24.0))
                .py(px(12.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .on_click(toggle)
                .child(avatar(&name, &email, 40.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .truncate()
                                .text_size(px(14.0))
                                .font_weight(if unread {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::MEDIUM
                                })
                                .text_color(rgba(th.text))
                                .child(name),
                        )
                        .child(
                            div()
                                .truncate()
                                .text_size(px(13.0))
                                .text_color(rgba(th.text_faint))
                                .child(snippet),
                        ),
                )
                .child(
                    div()
                        .flex_none()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(short_date),
                )
                .into_any_element();
        }

        let long_date = date
            .and_then(|d| {
                let long = format::long_date(format::local(d, &self.tz)?);
                Some(match format::ago(d, now) {
                    Some(ago) => format!("{long} ({ago})"),
                    None => long,
                })
            })
            .unwrap_or_default();
        let names = |list: &[katna_render::Address]| {
            let mut seen = std::collections::HashSet::new();
            list.iter()
                .filter(|a| seen.insert(a.email.to_lowercase()))
                .map(|a| {
                    if self.is_me(&a.email) {
                        "me".to_owned()
                    } else {
                        a.label().to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        let recipients = view.map(|v| {
            let mut all = v.to.clone();
            all.extend(v.cc.iter().cloned());
            format!("to {}", names(&all))
        });
        let details = part.details;
        let header = div()
            .id(("part-header", ix))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(8.0))
            .when(!last, |d| d.cursor_pointer().on_click(toggle))
            .child(
                div()
                    .flex_1()
                    .min_w(px(120.0))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_baseline()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .min_w_0()
                                    .truncate()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgba(th.text))
                                    .child(name.clone()),
                            )
                            .when(!email.is_empty() && email != name, |d| {
                                // Takes only the room the name leaves.
                                d.child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .truncate()
                                        .text_size(px(12.0))
                                        .text_color(rgba(th.text_faint))
                                        .child(format!("<{email}>")),
                                )
                            }),
                    )
                    .when_some(recipients, |d, recipients| {
                        d.child(
                            div()
                                .id(("part-to", ix))
                                .flex()
                                .flex_row()
                                .items_center()
                                .cursor_pointer()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_faint))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    cx.stop_propagation();
                                    if let Some(part) =
                                        this.reader.as_mut().and_then(|r| r.parts.get_mut(ix))
                                    {
                                        part.details = !part.details;
                                    }
                                    cx.notify();
                                }))
                                .child(div().min_w_0().truncate().child(recipients))
                                .child(icon("drop-down", th.text_faint, 18.0)),
                        )
                    }),
            )
            .child(
                // Gives way to the sender's name in a narrow pane.
                div()
                    .min_w_0()
                    .truncate()
                    .pt(px(2.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(long_date.clone()),
            )
            .child(
                icon_button_colored(
                    ("part-star", ix),
                    if flagged { "star-filled" } else { "star" },
                    20.0,
                    if flagged { th.star } else { th.text_faint },
                    th,
                )
                .size(px(32.0))
                .tooltip(tip(if flagged { "Starred" } else { "Not starred" }, th))
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.star_message(ix, id, !flagged, cx);
                })),
            )
            .child(
                icon_button(("part-reply", ix), "reply", 20.0, th)
                    .tooltip(tip("Reply", th))
                    .size(px(32.0))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        cx.stop_propagation();
                        this.open_compose(Kind::Reply, Some(id), window, cx);
                    })),
            );

        let details_box = (details && view.is_some()).then(|| {
            let view = view.expect("checked");
            let line = |label: &str, value: String| {
                div()
                    .flex()
                    .flex_row()
                    .gap(px(12.0))
                    .child(
                        div()
                            .w(px(64.0))
                            .flex_none()
                            .flex()
                            .justify_end()
                            .text_color(rgba(th.text_faint))
                            .child(format!("{label}:")),
                    )
                    .child(div().flex_1().min_w_0().child(value))
            };
            let full = |list: &[katna_render::Address]| {
                list.iter()
                    .map(|a| match &a.name {
                        Some(name) => format!("{name} <{}>", a.email),
                        None => a.email.clone(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            div()
                .mt(px(8.0))
                .p(px(12.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(th.divider))
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .child(line("from", full(&view.from)))
                .when(!view.to.is_empty(), |d| d.child(line("to", full(&view.to))))
                .when(!view.cc.is_empty(), |d| d.child(line("cc", full(&view.cc))))
                .child(line("date", long_date.clone()))
                .child(line("subject", view.subject.clone()))
                .with_animation(
                    ("details", ix),
                    Animation::new(Duration::from_millis(180)).with_easing(ease_out_quint()),
                    |el, t| el.opacity(t),
                )
        });

        let body = match part.body.as_ref() {
            Some(Body {
                view: Some(view),
                blocks,
                cut,
            }) => {
                let notes = [
                    view.from_html
                        .then_some("This message is HTML; it is shown as plain text for now."),
                    (*cut || view.truncated).then_some("The message is too long to show in full."),
                ];
                let listed: Vec<_> = view.attachments.iter().enumerate().collect();
                let attachments = self.attachment_cards(id, &listed, th, cx);
                div()
                    .flex()
                    .flex_col()
                    .pt(px(16.0))
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(th.text))
                    .children(notes.into_iter().flatten().map(|note| {
                        div()
                            .mb(px(12.0))
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgba(th.read_row))
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim))
                            .child(note)
                    }))
                    .children(blocks.iter().map(|(quoted, text)| {
                        div()
                            .when(*quoted, |d| {
                                d.pl(px(12.0))
                                    .border_l_2()
                                    .border_color(rgba(th.divider))
                                    .text_color(rgba(th.text_faint))
                            })
                            .child(text.clone())
                    }))
                    .children(attachments)
                    .into_any_element()
            }
            _ => div()
                .pt(px(16.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_faint))
                .child("This message has not been downloaded yet.")
                .into_any_element(),
        };

        div()
            .id(("part", ix))
            .flex()
            .flex_row()
            .pr(px(16.0))
            .pt(px(16.0))
            .pb(px(if last { 0.0 } else { 16.0 }))
            .when(ix > 0, |d| d.border_t_1().border_color(rgba(th.divider)))
            .child(
                div()
                    .w(px(72.0))
                    .flex_none()
                    .flex()
                    .justify_center()
                    .child(avatar(&name, &email, 40.0)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .max_w(px(960.0))
                    .child(header)
                    .children(details_box)
                    .child(body),
            )
            .with_animation(
                ("part-open", key_number(reader.key) ^ ix),
                Animation::new(Duration::from_millis(220)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element()
    }

    /// Stars or unstars one message of the open conversation.
    fn star_message(&mut self, ix: usize, id: MessageId, on: bool, cx: &mut Context<Self>) {
        if let Some(part) = self.reader.as_mut().and_then(|r| r.parts.get_mut(ix))
            && let Some(row) = &part.row
        {
            let mut row = (**row).clone();
            row.flagged = on;
            part.row = Some(Rc::new(row));
        }
        if let Some(key) = self.reader.as_ref().map(|r| r.key) {
            let any = on
                || self
                    .reader
                    .iter()
                    .flat_map(|r| &r.parts)
                    .any(|p| p.row.as_ref().is_some_and(|r| r.flagged));
            self.pending.entry(key).or_default().flagged = Some(any);
        }
        self.send(Command::Star(vec![id], on), None, None, false, cx);
        cx.notify();
    }

    /// Whether `email` is one of the user's own addresses.
    fn is_me(&self, email: &str) -> bool {
        self.accounts
            .iter()
            .any(|a| a.address.eq_ignore_ascii_case(email))
    }
}

/// "4 older messages": a line with a round count that unfolds them.
fn fold(count: usize, th: &Theme, cx: &mut Context<MailWindow>) -> AnyElement {
    div()
        .id("fold")
        .relative()
        .h(px(28.0))
        .flex()
        .items_center()
        .cursor_pointer()
        .on_click(cx.listener(|this, _, _, cx| {
            if let Some(reader) = &mut this.reader {
                reader.show_all = true;
            }
            cx.notify();
        }))
        .child(
            div()
                .absolute()
                .left_0()
                .right_0()
                .top(px(10.0))
                .h(px(1.0))
                .bg(rgba(th.divider)),
        )
        .child(
            div()
                .absolute()
                .left_0()
                .right_0()
                .top(px(14.0))
                .h(px(1.0))
                .bg(rgba(th.divider)),
        )
        .child(
            div()
                .ml(px(28.0))
                .size(px(28.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .border_1()
                .border_color(rgba(fade(th.text_faint, 0.6)))
                .bg(rgba(th.surface))
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .hover(|s| s.bg(rgba(th.hover)))
                .child(count.to_string()),
        )
        .into_any_element()
}

fn key_number(key: EntryKey) -> usize {
    match key {
        EntryKey::Message(id) => id.0 as usize,
        // Apart from message numbers, so the two never share an element ID.
        EntryKey::Thread(thread) => (thread.0 as usize) | (1 << (usize::BITS - 1)),
    }
}

/// Loads message `id` for reading.
fn read(mail: &Mail, id: MessageId) -> Body {
    let Some(raw) = mail.raw(id) else {
        return Body {
            view: None,
            blocks: Vec::new(),
            cut: false,
        };
    };
    let view = katna_render::message_view(&raw);
    let (blocks, cut) = body_blocks(&view.body, MAX_BODY_LINES);
    Body {
        view: Some(view),
        blocks,
        cut,
    }
}

/// Splits a body into runs of quoted (`>`) and unquoted lines, at most
/// `max_lines` lines in all. Returns whether lines were left out.
fn body_blocks(body: &str, max_lines: usize) -> (Vec<(bool, SharedString)>, bool) {
    let mut blocks: Vec<(bool, String)> = Vec::new();
    let mut lines = body.lines();
    for line in lines.by_ref().take(max_lines) {
        let line = line.trim_end();
        let quoted = line.starts_with('>');
        match blocks.last_mut() {
            Some((q, text)) if *q == quoted => {
                text.push('\n');
                text.push_str(line);
            }
            _ => blocks.push((quoted, line.to_owned())),
        }
    }
    let cut = lines.next().is_some();
    (
        blocks
            .into_iter()
            .map(|(quoted, text)| (quoted, text.into()))
            .collect(),
        cut,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_blocks() {
        let body = "Hi,\n\nsee below.\n> old line 1\n>> older\nthanks\r\n";
        let (blocks, cut) = body_blocks(body, 100);
        let blocks: Vec<(bool, &str)> = blocks.iter().map(|(q, t)| (*q, t.as_ref())).collect();
        assert_eq!(
            blocks,
            [
                (false, "Hi,\n\nsee below."),
                (true, "> old line 1\n>> older"),
                (false, "thanks"),
            ]
        );
        assert!(!cut);
        let (blocks, cut) = body_blocks(body, 2);
        assert_eq!(blocks.len(), 1);
        assert!(cut);
    }
}
