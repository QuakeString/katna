// SPDX-License-Identifier: GPL-3.0-or-later

//! The open conversation, laid out like webmail: an action bar (back,
//! archive, spam, delete, mark unread, move, more, "3 of 120"), the subject
//! with its folder chip, the messages (older ones folded to one line, a
//! "4 older messages" fold in long threads, the newest open) and, pinned
//! at the foot, Reply, Reply all and Forward, or the reply being written.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, FontWeight, SharedString, div, ease_out_quint,
    prelude::*, rgba,
};
use katna_i18n::tr;
use katna_render::MessageView;
use katna_render::html::Document;
use katna_store::{MessageFlags, MessageId};
use katna_ui::motion::lerp;
use katna_ui::px;
use katna_ui::unpx;

use super::compose::{Kind, SentCard};
use super::list::separator;
use super::rich::{self, Painter};
use super::{MailWindow, Menu, SelectNext, SelectPrevious};
use crate::daemon::Command;
use crate::data::{self, EntryKey, Mail, Row};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{
    card_outline, card_shadow, icon, icon_button, icon_button_colored, placeholder, tip, toolbar,
};

mod security;
mod ticks;
mod tracking;
use security::Secured;

/// The reading view shows at most this many lines of a body.
const MAX_BODY_LINES: usize = 4000;
/// Fold the middle of a conversation when this many messages in a row are
/// folded.
const FOLD_AT: usize = 3;
/// The column the sender's picture sits in, centered, beside an open
/// message; the text starts at its right edge.
const PICTURE_COLUMN: f32 = 72.0;
/// From the picture's left edge to the text's.
const PICTURE_COLUMN_INSET: f32 = PICTURE_COLUMN - (PICTURE_COLUMN - 40.0) / 2.0;
/// An open message shows its date and star button, and the subject its
/// full size, in panes at least this wide.
const STAR_FROM: f32 = 260.0;
/// How wide a message's text gets with "Limit the width of messages":
/// about 90 characters a line.
const MESSAGE_WIDTH: f32 = 760.0;

/// An open conversation (or a single message).
pub(super) struct Conversation {
    pub key: EntryKey,
    subject: String,
    parts: Vec<Part>,
    /// The folded middle of a long conversation is shown.
    show_all: bool,
    /// In a dark theme, its HTML mail keeps the sender's own colors
    /// rather than dark ones.
    pub original_colors: bool,
    /// The popover of who opened a sent message or followed its links.
    seen: Option<tracking::Seen>,
}

/// One message of the conversation.
/// Where a message's attachments come from ([`Conversation::attachment_source`]).
pub(super) enum AttachmentSource {
    /// The stored message.
    Stored,
    /// The message GnuPG opened; `encrypted` when it was decrypted, so
    /// its attachments must not reach the disk unasked.
    Opened { raw: Arc<Vec<u8>>, encrypted: bool },
    /// Encrypted or signed, and not opened.
    Sealed,
}

struct Part {
    id: MessageId,
    row: Option<Rc<Row>>,
    expanded: bool,
    /// Loaded when first expanded.
    body: Option<Body>,
    /// The "to me, Bob ▾" details are open.
    details: bool,
    /// How tall it was when last drawn, to grow or shrink it smoothly.
    height: Rc<Cell<f32>>,
    /// Its height when it was last opened or folded, and how many times
    /// that happened: each time starts a new height animation.
    from: f32,
    turns: u32,
    /// Sent with open and click tracking: what its recipients did.
    activity: Option<katna_store::MessageActivity>,
    /// A read receipt for one of the user's messages; `None` inside
    /// until looked at.
    receipt: Option<Option<crate::receipts::Receipt>>,
    /// The user's own message's `Message-ID`.
    message_id: Option<String>,
    /// For the user's own message: its recipients' delivery and read
    /// ticks, by address (lower case).
    ticks: std::collections::HashMap<String, ticks::Tick>,
    /// Where its eye button was last drawn, and the window's size then,
    /// for the popover to point at.
    eye: tracking::Anchor,
    /// A reply just sent, shown before the store has it: `true` once it
    /// went out. Its ID is a stand-in.
    pending: Option<bool>,
}

impl Part {
    fn new(id: MessageId, row: Option<Rc<Row>>, expanded: bool) -> Self {
        Self {
            id,
            row,
            expanded,
            body: None,
            details: false,
            height: Rc::default(),
            from: 0.0,
            turns: 0,
            activity: None,
            receipt: None,
            message_id: None,
            ticks: std::collections::HashMap::new(),
            eye: Rc::default(),
            pending: None,
        }
    }

    /// Opens or folds it, animating from how it looks now.
    fn set_expanded(&mut self, expanded: bool, mail: &Mail) {
        if expanded == self.expanded {
            return;
        }
        self.expanded = expanded;
        self.from = self.height.get();
        self.turns += 1;
        if expanded && self.body.is_none() {
            self.body = Some(read(mail, self.id));
        }
    }
}

struct Body {
    /// `None` when the message body is not stored (not downloaded yet).
    view: Option<MessageView>,
    /// The body, in blocks of consecutive quoted or unquoted lines.
    blocks: Vec<(bool, SharedString)>,
    cut: bool,
    /// The HTML body laid out, when the message has one.
    doc: Option<Document>,
    /// The remote images of `doc`.
    remote: Vec<String>,
    /// Encrypted or signed: what opening it found.
    security: Option<Secured>,
    /// The raw message, until it is handed to GnuPG.
    sealed: Option<Vec<u8>>,
    /// The message as GnuPG opened it (decrypted, or with the signature
    /// taken off), which the attachments are read from. In memory only.
    opened: Option<Arc<Vec<u8>>>,
}

impl Body {
    /// Encrypted (being opened, or opened): remote content stays blocked.
    fn encrypted(&self) -> bool {
        match &self.security {
            Some(Secured::Opening(protection)) => {
                matches!(protection, katna_crypto::Protection::Encrypted(_))
            }
            Some(Secured::Opened(security)) => security.encrypted(),
            None => false,
        }
    }
}

impl Conversation {
    /// A loaded message of it is HTML that sets its own colors.
    fn has_own_colors(&self) -> bool {
        self.parts.iter().any(|p| {
            p.body
                .as_ref()
                .and_then(|b| b.doc.as_ref())
                .is_some_and(|d| d.styled || d.background.is_some())
        })
    }

    /// The loaded message `id`, or by default the newest loaded one: what a
    /// reply or forward starts from.
    pub(super) fn view(&self, id: Option<MessageId>) -> Option<&MessageView> {
        fn view(part: &Part) -> Option<&MessageView> {
            part.body.as_ref().and_then(|b| b.view.as_ref())
        }
        match id {
            Some(id) => self.parts.iter().find(|p| p.id == id).and_then(view),
            None => self
                .parts
                .iter()
                .rev()
                .filter(|p| p.pending.is_none())
                .find_map(view),
        }
    }

    /// The message [`Self::view`] picks.
    pub(super) fn view_id(&self, id: Option<MessageId>) -> Option<MessageId> {
        let has_view = |p: &&Part| p.body.as_ref().is_some_and(|b| b.view.is_some());
        match id {
            Some(id) => self.parts.iter().find(|p| p.id == id).filter(has_view),
            None => self
                .parts
                .iter()
                .rev()
                .filter(|p| p.pending.is_none())
                .find(has_view),
        }
        .map(|p| p.id)
    }

    pub(super) fn subject(&self) -> &str {
        &self.subject
    }

    /// The line of the list it is, to open it elsewhere.
    pub(super) fn entry(&self) -> Option<data::Entry> {
        let latest = self.parts.iter().rev().find(|p| p.pending.is_none())?.id;
        Some(data::Entry {
            key: self.key,
            latest,
        })
    }

    /// Every message of the conversation, oldest first, as it is printed:
    /// the loaded ones as they are shown (protected ones as GnuPG opened
    /// them), folded ones read from the store.
    pub(super) fn printable(&self, mail: &Mail) -> Vec<Printable> {
        self.parts
            .iter()
            .map(|part| {
                let read_now;
                let body = match &part.body {
                    Some(body) => body,
                    None => {
                        read_now = read(mail, part.id);
                        &read_now
                    }
                };
                // Protected and not opened (or failed to open): only its
                // headers are known.
                let sealed = match &body.security {
                    Some(Secured::Opening(_)) => true,
                    Some(Secured::Opened(_)) => body.opened.is_none(),
                    None => false,
                };
                Printable {
                    id: part.id,
                    row: part.row.clone(),
                    view: body.view.clone(),
                    doc: body.doc.clone().filter(|_| !sealed),
                    encrypted: body.encrypted(),
                    sealed,
                }
            })
            .collect()
    }

    /// Where the attachments of message `id` are read from.
    pub(super) fn attachment_source(&self, id: MessageId) -> AttachmentSource {
        let Some(body) = self
            .parts
            .iter()
            .find(|p| p.id == id)
            .and_then(|p| p.body.as_ref())
        else {
            return AttachmentSource::Stored;
        };
        match (&body.opened, &body.security) {
            (Some(raw), _) => AttachmentSource::Opened {
                raw: raw.clone(),
                encrypted: body.encrypted(),
            },
            // Protected and not opened (yet): nothing to read.
            (None, Some(_)) => AttachmentSource::Sealed,
            (None, None) => AttachmentSource::Stored,
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
                let mut part = Part::new(*id, row, ix == last || unread);
                if part.expanded {
                    part.body = Some(read(mail, part.id));
                }
                part
            })
            .collect();
        let subject = parts
            .iter()
            .find_map(|p| p.row.as_ref().map(|r| r.subject.clone()))
            .unwrap_or_else(|| tr!("reader-no-subject"));
        let mut conversation = Self {
            key,
            subject,
            parts,
            show_all: false,
            original_colors: false,
            seen: None,
        };
        conversation.read_tracking(mail);
        conversation
    }

    /// Reads what the recipients of the user's tracked messages did, and
    /// which messages are read receipts for the user's mail.
    fn read_tracking(&mut self, mail: &Mail) {
        let mine = |part: &Part| part.row.as_ref().is_some_and(|r| mail.is_me(&r.sender));
        let any_mine = self.parts.iter().any(mine);
        for part in &mut self.parts {
            part.activity = part
                .row
                .as_ref()
                .and_then(|r| r.tracking)
                .and_then(|_| mail.activity(part.id));
            if any_mine && part.receipt.is_none() {
                part.receipt = Some((!mine(part)).then(|| mail.receipt(part.id)).flatten());
            }
        }
        for part in &mut self.parts {
            if part.message_id.is_none() && mine(part) {
                part.message_id = mail.message_id_header(part.id);
            }
        }
        let ticks: Vec<_> = self
            .parts
            .iter()
            .map(|part| match &part.message_id {
                Some(id) if mine(part) => {
                    // Read receipts in the conversation, with when they came.
                    let read: Vec<_> = self
                        .parts
                        .iter()
                        .filter_map(|p| {
                            let receipt = p.receipt.as_ref()?.as_ref()?;
                            (receipt.original.as_ref() == Some(id))
                                .then(|| (receipt, p.row.as_ref().and_then(|r| r.date)))
                        })
                        .collect();
                    ticks::ticks(&mail.receipts(id), &read, part.activity.as_ref())
                }
                _ => std::collections::HashMap::new(),
            })
            .collect();
        for (part, ticks) in self.parts.iter_mut().zip(ticks) {
            part.ticks = ticks;
        }
    }

    /// The read receipts in the conversation for `part`.
    fn receipts_for(&self, part: &Part) -> Vec<&crate::receipts::Receipt> {
        let Some(id) = &part.message_id else {
            return Vec::new();
        };
        self.parts
            .iter()
            .filter_map(|p| p.receipt.as_ref()?.as_ref())
            .filter(|r| r.original.as_ref() == Some(id))
            .collect()
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
                    // Downloaded since, by the sync or on request.
                    if part.body.as_ref().is_some_and(|b| b.view.is_none()) {
                        part.body = Some(read(mail, id));
                    }
                    part
                }
                None => Part {
                    body: Some(read(mail, id)),
                    ..Part::new(id, row, true)
                },
            })
            .collect();
        // Replies just sent stay until the store has them.
        self.parts
            .extend(old.into_iter().filter(|p| p.pending.is_some()));
        self.read_tracking(mail);
    }

    /// The `Message-ID`s of the user's stored messages in it.
    pub(super) fn own_message_ids(&self) -> Vec<String> {
        self.parts
            .iter()
            .filter(|p| p.pending.is_none())
            .filter_map(|p| p.message_id.clone())
            .collect()
    }

    /// Shows `cards`, replies just sent, after its stored messages, in
    /// place of those shown before.
    pub(super) fn place_sent<'a>(&mut self, cards: impl Iterator<Item = &'a SentCard>) {
        let (mut shown, stored): (Vec<Part>, Vec<Part>) = std::mem::take(&mut self.parts)
            .into_iter()
            .partition(|p| p.pending.is_some());
        self.parts = stored;
        for card in cards {
            let mut part = match shown.iter().position(|p| p.id == card.id) {
                Some(ix) => shown.swap_remove(ix),
                None => Part {
                    body: Some(shown_body(&card.raw)),
                    message_id: Some(card.message_id.clone()),
                    ..Part::new(card.id, Some(card.row.clone()), true)
                },
            };
            part.pending = Some(card.sent.is_some());
            self.parts.push(part);
        }
    }

    /// Open messages whose body is not stored yet.
    pub(super) fn missing_bodies(&self) -> Vec<MessageId> {
        self.parts
            .iter()
            .filter(|p| p.expanded && p.body.as_ref().is_some_and(|b| b.view.is_none()))
            .map(|p| p.id)
            .collect()
    }

    /// Reads message `id` again, once its body is downloaded.
    pub(super) fn reload_body(&mut self, id: MessageId, mail: &Mail) {
        if let Some(part) = self
            .parts
            .iter_mut()
            .find(|p| p.id == id && p.body.is_some())
        {
            part.body = Some(read(mail, id));
        }
    }

    pub(super) fn unread_messages(&self) -> Vec<MessageId> {
        self.parts
            .iter()
            .filter(|p| p.pending.is_none() && p.row.as_ref().is_some_and(|r| r.unread))
            .map(|p| p.id)
            .collect()
    }

    fn toggle(&mut self, ix: usize, mail: &Mail) {
        if let Some(part) = self.parts.get_mut(ix) {
            part.set_expanded(!part.expanded, mail);
        }
    }

    fn set_all(&mut self, expanded: bool, mail: &Mail) {
        let last = self.parts.len().saturating_sub(1);
        for (ix, part) in self.parts.iter_mut().enumerate() {
            part.set_expanded(expanded || ix == last, mail);
        }
        self.show_all = expanded;
    }

    fn all_expanded(&self) -> bool {
        self.parts.iter().all(|p| p.expanded)
    }

    /// Each open message's ID, sender address and remote images.
    pub(super) fn remote_content(&self) -> Vec<(MessageId, String, Vec<String>)> {
        self.parts
            .iter()
            .filter(|p| p.expanded)
            .filter_map(|p| {
                // Encrypted mail never loads remote content: a fetch would
                // tell the sender (or whoever altered the message) that it
                // was opened, and could leak its text (EFAIL).
                let body = p.body.as_ref().filter(|b| !b.encrypted())?;
                let sender = body.view.as_ref()?.from.first()?.email.clone();
                Some((p.id, sender, body.remote.clone()))
            })
            .collect()
    }

    /// Each open, downloaded message of the conversation.
    pub(super) fn open_views(&self) -> impl Iterator<Item = (MessageId, &MessageView)> {
        self.parts
            .iter()
            .filter(|p| p.expanded)
            .filter_map(|p| Some((p.id, p.body.as_ref()?.view.as_ref()?)))
    }
}

/// A message of the conversation, for printing.
pub(super) struct Printable {
    pub id: MessageId,
    pub row: Option<Rc<Row>>,
    /// `None` when it is not downloaded yet.
    pub view: Option<MessageView>,
    /// Its HTML body laid out, as the reader draws it.
    pub doc: Option<Document>,
    /// Encrypted: its remote pictures are never loaded.
    pub encrypted: bool,
    /// Encrypted or signed, and its text not opened.
    pub sealed: bool,
}

/// What the list of messages shows: a message, or a fold of several.
/// What the reading pane's toolbar leaves to the More menu where it is
/// too narrow for every button. Back, Archive, More and the Newer and
/// Older arrows always stay; the rest go one by one, least used first
/// (`Squeeze::DROP_ORDER`), so the arrows are never pushed off.
#[derive(Debug, Clone, Copy)]
pub(super) struct Squeeze {
    pub new_window: bool,
    pub print: bool,
    pub colors: bool,
    pub contact: bool,
    pub move_to: bool,
    pub unread: bool,
    pub spam: bool,
    /// The lines between the groups of buttons.
    pub separators: bool,
    pub delete: bool,
    /// "3 of 120" beside the arrows.
    pub position: bool,
}

/// A toolbar button's width and the toolbar's gap between buttons.
const BUTTON: f32 = 40.0;
const TOOL_GAP: f32 = 2.0;
/// A separator with its margins.
const SEPARATOR: f32 = 13.0;
/// The toolbar's padding, the card's edge and a little room to spare.
const TOOLBAR_FIXED: f32 = 16.0 + 6.0;

impl Squeeze {
    pub const NONE: Self = Self {
        new_window: false,
        print: false,
        colors: false,
        contact: false,
        move_to: false,
        unread: false,
        spam: false,
        separators: false,
        delete: false,
        position: false,
    };

    /// Fits the items `shown` into a toolbar `width` wide, leaving off
    /// those `start` already does and then the least used.
    fn fit(width: f32, shown: &Toolbar, start: Self) -> Self {
        let mut squeeze = start;
        let mut need = shown.width(&squeeze);
        for drop in Self::DROP_ORDER {
            if need <= width {
                break;
            }
            drop(&mut squeeze);
            need = shown.width(&squeeze);
        }
        squeeze
    }

    const DROP_ORDER: [fn(&mut Self); 10] = [
        |s| s.new_window = true,
        |s| s.print = true,
        |s| s.colors = true,
        |s| s.contact = true,
        |s| s.move_to = true,
        |s| s.unread = true,
        |s| s.spam = true,
        |s| s.separators = true,
        |s| s.delete = true,
        |s| s.position = true,
    ];
}

/// Which of the reading pane's toolbar items this window and message have
/// at all, before any squeezing.
struct Toolbar {
    back: bool,
    separators: bool,
    contact: bool,
    colors: bool,
    new_window: bool,
    /// The width of "3 of 120", or `None` without it.
    position: Option<f32>,
    arrows: bool,
}

impl Toolbar {
    fn width(&self, squeeze: &Squeeze) -> f32 {
        let mut buttons = 2.0; // Archive and More
        let mut rest = 0.0;
        let mut items = 1.0; // the spacer
        let mut add = |on: bool, n: f32| {
            if on {
                buttons += n;
            }
        };
        add(self.back, 1.0);
        add(!squeeze.spam, 1.0);
        add(!squeeze.delete, 1.0);
        add(!squeeze.unread, 1.0);
        add(!squeeze.move_to, 1.0);
        add(self.contact && !squeeze.contact, 1.0);
        add(self.colors && !squeeze.colors, 1.0);
        add(!squeeze.print, 1.0);
        add(self.new_window && !squeeze.new_window, 1.0);
        add(self.arrows, 2.0);
        if self.separators && !squeeze.separators {
            let n = if self.back { 2.0 } else { 1.0 };
            rest += n * SEPARATOR;
            items += n;
        }
        if let Some(position) = self.position.filter(|_| !squeeze.position) {
            rest += position;
            items += 1.0;
        }
        // Archive, Report spam and Delete sit together without gaps.
        items += buttons - f32::from(!squeeze.spam) - f32::from(!squeeze.delete);
        TOOLBAR_FIXED + buttons * BUTTON + rest + items * TOOL_GAP
    }
}

enum Shown {
    Part(usize),
    Fold(usize),
}

impl MailWindow {
    /// Whether the open conversation can switch between its own colors
    /// and dark ones.
    pub(super) fn original_colors_offered(&self, th: &Theme) -> bool {
        th.dark
            && self.config.mail.dark_mail
            && self.reader.as_ref().is_some_and(|r| r.has_own_colors())
    }

    /// In a dark theme, the button that shows the open conversation's HTML
    /// mail in its sender's own colors, or back in dark ones. Only where
    /// a message sets its own colors, so there is something to switch.
    fn original_colors_toggle(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.original_colors_offered(th) {
            return None;
        }
        let on = self.reader.as_ref()?.original_colors;
        Some(
            icon_button_colored(
                "reader-original-colors",
                "contrast",
                20.0,
                if on { th.accent } else { th.text_dim },
                th,
            )
            .when(on, |d| d.bg(rgba(th.hover)))
            .tooltip(tip(
                if on {
                    tr!("reader-dark-colors")
                } else {
                    tr!("reader-original-colors")
                },
                th,
            ))
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(reader) = &mut this.reader {
                    reader.original_colors = !reader.original_colors;
                }
                cx.notify();
            }))
            .into_any_element(),
        )
    }
}

impl MailWindow {
    /// The reading pane beside the list: its own card.
    pub(super) fn render_reader_card(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (radius, outline) = (
            self.layout.shape.card_radius(),
            self.layout.shape.card_outline(),
        );
        div()
            .id("reader-card")
            .size_full()
            .flex()
            .flex_col()
            .relative()
            .rounded(px(radius))
            .overflow_hidden()
            .bg(rgba(th.surface))
            .shadow(card_shadow(th, outline))
            .p(px(outline))
            .child(self.render_reader_toolbar(th, cx))
            .child(div().flex_1().min_h_0().child(self.render_reader(th, cx)))
            .children(card_outline(th, radius, outline))
            .into_any_element()
    }

    /// "3 of 120" for the open conversation, or `None` where the toolbar
    /// has no Newer and Older (a phone, a conversation window).
    fn reader_position(&self) -> Option<(usize, String)> {
        let count = self.entries.len();
        if self.layout.shape.is_phone() || self.detached || count == 0 {
            return None;
        }
        let ix = self
            .reader
            .as_ref()
            .and_then(|r| self.entries.iter().position(|e| e.key == r.key))
            .or(self.selected)
            .unwrap_or(0);
        let text = tr!(
            "reader-position",
            position = ix as u64 + 1,
            total = count as u64
        );
        Some((ix, text))
    }

    /// What the reading pane's toolbar leaves to the More menu.
    pub(super) fn reader_squeeze(&self, th: &Theme) -> Squeeze {
        let phone = self.layout.shape.is_phone();
        let shown = Toolbar {
            back: !self.detached,
            separators: !phone,
            contact: self.contact_fits(self.cards_width + self.contact_room()),
            colors: self.original_colors_offered(th),
            new_window: !self.detached,
            // About 6.5 px a character at 12 px, and its 8 px padding.
            position: self
                .reader_position()
                .map(|(_, text)| text.chars().count() as f32 * 6.5 + 16.0),
            arrows: !phone && !self.detached,
        };
        // A phone keeps these in the More menu.
        let start = Squeeze {
            print: phone,
            new_window: phone,
            move_to: phone,
            ..Squeeze::NONE
        };
        Squeeze::fit(self.reader_width(), &shown, start)
    }

    pub(super) fn render_reader_toolbar(
        &mut self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let count = self.entries.len();
        let back = if self.split() {
            icon_button("reader-close", "close", 20.0, th).tooltip(tip(tr!("reader-close"), th))
        } else {
            icon_button("reader-back", "back", 20.0, th).tooltip(tip(tr!("reader-back"), th))
        }
        .on_click(
            cx.listener(|this, _, window, cx| this.close_message(&super::CloseMessage, window, cx)),
        );
        let phone = self.layout.shape.is_phone();
        let squeeze = self.reader_squeeze(th);
        let separators = !phone && !squeeze.separators;
        // A phone moves between conversations from the list; a
        // conversation window shows only its own.
        let position = self.reader_position();
        let ix = position.as_ref().map_or(0, |(ix, _)| *ix);
        let more = {
            let more = icon_button("reader-more", "more", 20.0, th)
                .when(
                    !matches!(self.menu, Some(Menu::ReaderMore | Menu::MoveTo)),
                    |d| d.tooltip(tip(tr!("reader-more"), th)),
                )
                .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::ReaderMore, cx)));
            let more = self.with_menu(more, Menu::ReaderMore, th, cx);
            // Move to, from the More menu, opens under it.
            if squeeze.move_to {
                self.with_menu(more, Menu::MoveTo, th, cx)
            } else {
                more
            }
        };
        // What gives way when the pane is narrow; the arrows never do.
        let actions = div()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            // A conversation window closes from its own frame.
            .when(!self.detached, |d| {
                d.child(back).when(separators, |d| d.child(separator(th)))
            })
            .child(self.action_buttons("reader", squeeze, th, cx))
            .when(separators, |d| d.child(separator(th)))
            .when(!squeeze.unread, |d| {
                d.child(
                    icon_button("reader-unread", "mail", 20.0, th)
                        .tooltip(tip(tr!("reader-mark-unread"), th))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.mark_unread(&super::MarkUnread, window, cx)
                        })),
                )
            })
            .when(!squeeze.move_to, |d| {
                d.child({
                    let move_to = icon_button("reader-move", "move-to", 20.0, th)
                        .when(self.menu != Some(Menu::MoveTo), |d| {
                            d.tooltip(tip(tr!("reader-move-to"), th))
                        })
                        .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::MoveTo, cx)));
                    self.with_menu(move_to, Menu::MoveTo, th, cx)
                })
            })
            .child(more)
            .child(div().flex_1())
            .when(!squeeze.contact, |d| {
                d.children(self.contact_toggle(th, cx))
            })
            .when(!squeeze.colors, |d| {
                d.children(self.original_colors_toggle(th, cx))
            })
            .when(!squeeze.print, |d| {
                d.child(
                    icon_button("reader-print", "print", 20.0, th)
                        .tooltip(tip(tr!("reader-print-all"), th))
                        .on_click(
                            cx.listener(|this, _, window, cx| this.print_conversation(window, cx)),
                        ),
                )
            })
            .when(!self.detached && !squeeze.new_window, |d| {
                d.child(
                    icon_button("reader-new-window", "open-external", 20.0, th)
                        .tooltip(tip(tr!("reader-new-window"), th))
                        .on_click(cx.listener(|this, _, _, cx| this.open_reader_in_window(cx))),
                )
            });
        let steps = position.map(|(_, text)| {
            div()
                .flex_none()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(2.0))
                .when(!squeeze.position, |d| {
                    d.child(
                        div()
                            .px(px(8.0))
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .whitespace_nowrap()
                            .child(text),
                    )
                })
                .child(
                    icon_button("newer", "chevron-left", 20.0, th)
                        .tooltip(tip(tr!("reader-newer"), th))
                        .when(ix == 0, |d| d.opacity(0.4))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.select_previous(&SelectPrevious, window, cx)
                        })),
                )
                .child(
                    icon_button("older", "chevron-right", 20.0, th)
                        .tooltip(tip(tr!("reader-older"), th))
                        .when(ix + 1 >= count, |d| d.opacity(0.4))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.select_next(&SelectNext, window, cx)
                        })),
                )
        });
        toolbar(th)
            .child(actions)
            .children(steps)
            .into_any_element()
    }

    pub(super) fn render_reader(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        self.open_sealed(cx);
        self.fetch_remote(cx);
        self.download_bodies(cx);
        self.request_thumbnails(cx);
        if let Some(key) = self.reader.as_ref().map(|r| r.key) {
            self.text.begin(key);
        }
        let Some(reader) = &self.reader else {
            return placeholder("", th);
        };
        if reader.parts.is_empty() {
            return placeholder(&tr!("reader-removed"), th);
        }
        let all_expanded = reader.all_expanded();
        let title = div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .pl(px(self.reader_indent()))
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
                            .map(|d| {
                                // A very narrow pane takes a smaller title so
                                // its words don't break.
                                if self.reader_width() < STAR_FROM {
                                    d.text_size(px(18.0)).line_height(px(24.0))
                                } else {
                                    d.text_size(px(22.0)).line_height(px(28.0))
                                }
                            })
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
                            tr!("reader-collapse-all")
                        } else {
                            tr!("reader-expand-all")
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

        // Fold runs of collapsed messages in the middle. `order` is the
        // messages as shown: oldest first, or newest first by the setting.
        let newest_first = self.config.mail.newest_first;
        let n = reader.parts.len();
        let order: Vec<usize> = if newest_first {
            (0..n).rev().collect()
        } else {
            (0..n).collect()
        };
        let mut shown = Vec::new();
        let mut at = 0;
        while at < n {
            let run_end = (at..n)
                .find(|&j| reader.parts[order[j]].expanded)
                .unwrap_or(n);
            let run = run_end - at;
            if !reader.show_all && at > 0 && run >= FOLD_AT && run_end < n {
                shown.push(Shown::Fold(run));
                at = run_end;
            } else {
                shown.push(Shown::Part(order[at]));
                at += 1;
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
        // the conversation scrolls. A reply is written at the end of the
        // conversation itself, as in Gmail: it grows with its text and
        // scrolls with the messages.
        let key = reader.key;
        let reply = self.render_inline_reply(key, th, cx);
        // Drafts are edited, not answered.
        let drafts_only = self.mail.as_ref().ok().is_some_and(|mail| {
            let ids: Vec<MessageId> = reader.parts.iter().map(|p| p.id).collect();
            mail.drafts(&ids).len() == ids.len()
        });
        let footer = (reply.is_none() && !drafts_only).then(|| self.render_reply_row(th, cx));
        // A reply goes next to the message it answers, the newest.
        let (reply_above, reply_below) = if newest_first {
            (reply, None)
        } else {
            (None, reply)
        };
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
                            .children(reply_above)
                            .children(parts)
                            .children(reply_below)
                            .map(|d| self.text_area(d, cx))
                            .with_animation(
                                ("open-conversation", key_number(key)),
                                Animation::new(Duration::from_millis(280))
                                    .with_easing(ease_out_quint()),
                                |el, t| el.opacity(t).mt(px(14.0 * (1.0 - t))),
                            ),
                    ),
            )
            .children(footer.map(|footer| {
                div()
                    .flex_none()
                    .border_t_1()
                    .border_color(rgba(th.divider))
                    .child(footer)
            }))
            .children(self.render_text_menu(th, cx))
            .into_any_element()
    }

    fn render_part_content(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(reader) = &self.reader else {
            return div().into_any_element();
        };
        let part = &reader.parts[ix];
        let turn = part_turn(reader.key, ix, part.turns);
        let last = ix + 1 == reader.parts.len();
        let row = part.row.clone();
        let view = part.body.as_ref().and_then(|b| b.view.as_ref());
        let (name, email) = match (view.and_then(|v| v.from.first()), &row) {
            (Some(from), _) => (from.label().to_owned(), from.email.clone()),
            // Not read yet: the list line knows the sender, so the picture
            // (when one was already fetched) stays the same once it opens.
            (None, Some(row)) => (row.correspondent.clone(), row.sender.clone()),
            (None, None) => (tr!("reader-unknown-sender"), String::new()),
        };
        let now = jiff::Timestamp::now().as_second();
        let date = row
            .as_ref()
            .and_then(|r| r.date)
            .or(view.and_then(|v| v.date));
        let unread = row.as_ref().is_some_and(|r| r.unread);
        let flagged = row.as_ref().is_some_and(|r| r.flagged);
        let id = part.id;
        // A reply just sent: not stored yet, so nothing acts on it.
        let pending = part.pending.is_some();
        let waiting = part.pending == Some(false);
        let toggle = cx.listener(move |this, _, _, cx| {
            if let (Some(reader), Ok(mail)) = (&mut this.reader, &this.mail) {
                reader.toggle(ix, mail);
            }
            cx.notify();
        });

        if !part.expanded {
            let snippet = row.as_ref().map(|r| r.snippet.clone()).unwrap_or_default();
            let short_date = if waiting {
                tr!("reader-sending")
            } else {
                date.and_then(|d| format::local(d, &self.tz))
                    .zip(format::local(now, &self.tz))
                    .map(|(d, now)| format::list_date(d, now))
                    .unwrap_or_default()
            };
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
                .child(self.person_avatar(&name, &email, 40.0))
                .child(turn_fade(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(16.0))
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
                        ),
                    turn,
                ))
                .into_any_element();
        }

        let long_date = if waiting {
            Some(tr!("reader-sending"))
        } else {
            date.and_then(|d| {
                let long = format::long_date(format::local(d, &self.tz)?);
                Some(match format::ago(d, now) {
                    Some(ago) => tr!("reader-date-ago", date = long, ago = ago),
                    None => long,
                })
            })
        }
        .unwrap_or_default();
        let full_names = self.config.mail.full_names;
        let names = |list: &[katna_render::Address]| {
            let mut seen = std::collections::HashSet::new();
            let people: Vec<(bool, &katna_render::Address)> = list
                .iter()
                .filter(|a| seen.insert(a.email.to_lowercase()))
                .map(|a| (self.is_me(&a.email), a))
                .collect();
            recipient_names(&people, full_names).join(", ")
        };
        let recipients = view.map(|v| {
            let mut all = v.to.clone();
            all.extend(v.cc.iter().cloned());
            if part.ticks.is_empty() {
                return div()
                    .min_w_0()
                    .truncate()
                    .child(tr!("reader-to", names = names(&all)))
                    .into_any_element();
            }
            // Each name with its delivered or read tick.
            let mut seen = std::collections::HashSet::new();
            let people: Vec<(bool, &katna_render::Address)> = all
                .iter()
                .filter(|a| seen.insert(a.email.to_lowercase()))
                .map(|a| (self.is_me(&a.email), a))
                .collect();
            let labels = recipient_names(&people, full_names);
            let count = labels.len();
            div()
                .min_w_0()
                .flex()
                .flex_row()
                .items_center()
                .overflow_hidden()
                .whitespace_nowrap()
                .child(div().flex_none().mr(px(4.0)).child(tr!("reader-to-label")))
                .children(people.iter().zip(labels).enumerate().map(
                    |(i, ((_, address), label))| {
                        let tick = part.ticks.get(&address.email.to_lowercase());
                        div()
                            .flex_none()
                            .flex()
                            .flex_row()
                            .items_center()
                            .child(label)
                            .children(self.render_tick(tick, ("part-tick", ix * 1000 + i), th))
                            .when(i + 1 < count, |d| d.child(div().mr(px(4.0)).child(",")))
                    },
                ))
                .into_any_element()
        });
        // Clicking \u{201c}to\u{201d} turns the details the other way from the
        // Full headers setting.
        let details = part.details != self.config.mail.full_headers;
        // A very narrow pane leaves starring to the toolbar's More menu and
        // the date to the details under "to", and lets the name shrink
        // further.
        let roomy = self.reader_width() >= STAR_FROM;
        let name_room = lerp(120.0, 48.0, self.reader_compact());
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
                    .min_w(px(name_room))
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
                                .child(recipients)
                                .child(icon("drop-down", th.text_faint, 18.0)),
                        )
                    }),
            )
            .when(roomy, |d| {
                // Gives way to the sender's name in a narrow pane.
                d.child(
                    div()
                        .min_w_0()
                        .truncate()
                        .pt(px(2.0))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(long_date.clone()),
                )
            })
            .when(!pending, |d| {
                d.children(self.seen_eye(ix, part, th, cx))
                    .children(self.seen_popover(ix, part, th, cx))
            })
            .when(roomy && !pending, |d| {
                d.child(
                    icon_button_colored(
                        ("part-star", ix),
                        if flagged { "star-filled" } else { "star" },
                        20.0,
                        if flagged { th.star } else { th.text_faint },
                        th,
                    )
                    .size(px(32.0))
                    .tooltip(tip(
                        if flagged {
                            tr!("reader-starred")
                        } else {
                            tr!("reader-not-starred")
                        },
                        th,
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.star_message(ix, id, !flagged, cx);
                    })),
                )
            })
            .when(!pending, |d| {
                d.child({
                    // Settings > General > Reply button.
                    let (kind, name, label) = if self.config.mail.reply_all {
                        (Kind::ReplyAll, "reply-all", tr!("reply-reply-all"))
                    } else {
                        (Kind::Reply, "reply", tr!("reply-reply"))
                    };
                    icon_button(("part-reply", ix), name, 20.0, th)
                        .tooltip(tip(label, th))
                        .size(px(32.0))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.open_compose(kind, Some(id), window, cx);
                        }))
                })
            });

        let details_box = (details && view.is_some()).then(|| {
            let view = view.expect("checked");
            let line = |label: String, value: String| {
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
                            .child(label),
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
                .child(line(tr!("reader-details-from"), full(&view.from)))
                .when(!view.to.is_empty(), |d| {
                    d.child(line(tr!("reader-details-to"), full(&view.to)))
                })
                .when(!view.cc.is_empty(), |d| {
                    d.child(line(tr!("reader-details-cc"), full(&view.cc)))
                })
                .child(line(tr!("reader-details-date"), long_date.clone()))
                .child(line(tr!("reader-details-subject"), view.subject.clone()))
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
                doc,
                ..
            }) => {
                let too_long = match doc {
                    Some(doc) => doc.truncated,
                    None => *cut || view.truncated,
                };
                let encrypted = part.body.as_ref().is_some_and(Body::encrypted);
                let blocked = encrypted && doc.as_ref().is_some_and(|d| d.remote_images > 0);
                let notes = [
                    too_long.then(|| tr!("reader-too-long")),
                    blocked.then(|| tr!("reader-encrypted-images")),
                ];
                let allowed = !encrypted && self.remote.allowed(id, &email);
                let banner = doc
                    .as_ref()
                    .filter(|doc| doc.remote_images > 0 && !allowed && !encrypted)
                    .map(|_| self.images_banner(ix, id, &email, th, cx));
                // Images the body shows are not listed again.
                let listed: Vec<_> = view
                    .attachments
                    .iter()
                    .enumerate()
                    .filter(|(_, a)| {
                        !doc.as_ref().is_some_and(|doc| {
                            a.content_id.as_ref().is_some_and(|id| {
                                doc.inline_ids.iter().any(|i| i.eq_ignore_ascii_case(id))
                            })
                        })
                    })
                    .collect();
                let attachments = (!pending).then(|| self.attachment_cards(id, &listed, th, cx));
                let translation = if pending {
                    None
                } else {
                    self.translation_bar(ix, id, &view.body, encrypted, th, cx)
                };
                let translated = self.translated_blocks(id);
                div()
                    .flex()
                    .flex_col()
                    .when(self.config.mail.limit_width, |d| d.max_w(px(MESSAGE_WIDTH)))
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
                    .children(banner)
                    .children(translation)
                    .child({
                        // Selection follows the order messages are shown in.
                        let slot = match self.reader.as_ref() {
                            Some(r) if self.config.mail.newest_first => r.parts.len() - 1 - ix,
                            _ => ix,
                        };
                        let mut pieces = self.text.pieces(slot, th);
                        let blocks = translated.as_ref().unwrap_or(blocks);
                        let text = match doc.as_ref().filter(|_| translated.is_none()) {
                            Some(doc) => div().child(
                                Painter::new(
                                    th,
                                    &self.remote.images,
                                    allowed,
                                    self.remote.mono(),
                                    self.config.mail.dark_mail
                                        && !self.reader.as_ref().is_some_and(|r| r.original_colors),
                                    pieces,
                                )
                                .document(doc),
                            ),
                            None => div().children(blocks.iter().map(|(quoted, text)| {
                                let (styled, holder) = pieces.piece(text.clone(), Vec::new());
                                holder
                                    .when(*quoted, |d| {
                                        d.pl(px(12.0))
                                            .border_l_2()
                                            .border_color(rgba(th.divider))
                                            .text_color(rgba(th.text_faint))
                                    })
                                    .child(styled)
                            })),
                        };
                        self.selectable_body(slot, text, cx)
                    })
                    .children(attachments.flatten())
                    .into_any_element()
            }
            _ => self.download_note(id, ix, th, cx),
        };

        // The picture sits where it does on the folded line, so only the
        // text changes when a message opens.
        div()
            .id(("part", ix))
            .flex()
            .flex_row()
            .pr(px(16.0))
            .pt(px(12.0))
            .pb(px(if last { 0.0 } else { 16.0 }))
            .when(ix > 0, |d| d.border_t_1().border_color(rgba(th.divider)))
            .child(
                div()
                    .w(px(PICTURE_COLUMN))
                    .flex_none()
                    .flex()
                    .justify_center()
                    .child(self.person_avatar(&name, &email, 40.0)),
            )
            .child(turn_fade(
                div()
                    .flex_1()
                    .min_w_0()
                    .max_w(px(960.0))
                    .child(header)
                    .child(
                        // On a phone, and in a narrow pane, the message takes
                        // the room under the picture too, from its left edge.
                        div()
                            .ml(px(-PICTURE_COLUMN_INSET * self.reader_compact()))
                            .children(details_box)
                            .children(self.security_banner(part, th, cx))
                            .children(self.tracking_banner(part, th))
                            .child(body),
                    ),
                turn,
            ))
            .into_any_element()
    }

    /// A message of the open conversation. Opening or folding it grows or
    /// shrinks it from the height it had, so the messages below slide.
    fn render_part(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(reader) = &self.reader else {
            return div().into_any_element();
        };
        let part = &reader.parts[ix];
        let measured = part.height.clone();
        let target = part.height.clone();
        let from = part.from;
        let content = div()
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_children_prepainted(move |bounds, _, _| {
                if let Some(bounds) = bounds.first() {
                    measured.set(unpx(bounds.size.height));
                }
            })
            .child(
                div()
                    .flex_none()
                    .child(self.render_part_content(ix, th, cx)),
            );
        match part_turn(reader.key, ix, part.turns) {
            None => content.into_any_element(),
            Some(id) => content
                .with_animation(
                    id,
                    Animation::new(TURN).with_easing(ease_out_cubic),
                    move |el, t| {
                        if t >= 1.0 {
                            el
                        } else {
                            el.h(px(from + (target.get() - from) * t))
                        }
                    },
                )
                .into_any_element(),
        }
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
        let ids = match &self.mail {
            Ok(mail) => data::flag_changes(&mail.with_copies(&[id]), MessageFlags::FLAGGED, on),
            Err(_) => vec![id],
        };
        let command = Command::Star(ids.clone(), on);
        let done = command.done_text(1, false);
        let undo = Command::Star(ids, !on);
        self.send(command, done, Some(undo), false, cx);
        cx.notify();
    }

    /// Whether `email` is one of the user's own addresses.
    fn is_me(&self, email: &str) -> bool {
        self.accounts
            .iter()
            .any(|a| a.address.eq_ignore_ascii_case(email))
    }
}

/// The names of a message's recipients for its \u{201c}to\u{201d} line, each
/// with whether it is one of the user's addresses: \u{201c}me\u{201d}, and
/// others by first name unless `full` (or two share a first name).
/// Recipients without a name show their address.
fn recipient_names(people: &[(bool, &katna_render::Address)], full: bool) -> Vec<String> {
    fn first(a: &katna_render::Address) -> Option<&str> {
        a.name.as_deref().map(first_name)
    }
    people
        .iter()
        .map(|(me, a)| {
            if *me {
                return tr!("reader-me");
            }
            match first(a) {
                Some(short)
                    if !full
                        && people
                            .iter()
                            .filter(|(me, b)| !me && first(b) == Some(short))
                            .count()
                            == 1 =>
                {
                    short.to_owned()
                }
                _ => a.label().to_owned(),
            }
        })
        .collect()
}

/// The first name in a display name: \u{201c}Ada\u{201d} from \u{201c}Ada Lovelace\u{201d} or
/// \u{201c}Lovelace, Ada\u{201d}. A name it cannot split (one word, or a title
/// such as \u{201c}Dr.\u{201d} first) stays whole.
fn first_name(name: &str) -> &str {
    let name = name.trim().trim_matches(['"', '\'']).trim();
    let given = match name.split_once(',') {
        Some((_, given)) => given.trim(),
        None => name,
    };
    match given.split_whitespace().next() {
        Some(word) if !word.ends_with('.') && word.chars().count() > 1 => word,
        _ => name,
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
                .child(format::thousands(count as u64)),
        )
        .into_any_element()
}

/// How long a message takes to open or fold.
const TURN: Duration = Duration::from_millis(260);

/// The animation ID of a message's latest opening or folding; `None` until
/// it has been opened or folded.
fn part_turn(key: EntryKey, ix: usize, turns: u32) -> Option<SharedString> {
    (turns > 0).then(|| format!("part-turn-{}-{ix}-{turns}", key_number(key)).into())
}

/// Fades a message's text in after it opens or folds.
fn turn_fade(el: gpui::Div, turn: Option<SharedString>) -> AnyElement {
    match turn {
        None => el.into_any_element(),
        Some(id) => el
            .with_animation(
                SharedString::from(format!("{id}-text")),
                Animation::new(TURN).with_easing(ease_out_cubic),
                |el, t| el.opacity(0.3 + 0.7 * t),
            )
            .into_any_element(),
    }
}

/// Gentler than quint: the height change stays visible for most of
/// [`TURN`].
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
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
            doc: None,
            remote: Vec::new(),
            security: None,
            sealed: None,
            opened: None,
        };
    };
    match katna_crypto::protection(&raw) {
        Some(protection) => security::sealed(raw, protection),
        None => shown(&raw, None),
    }
}

/// The body of a message just sent, as written.
fn shown_body(raw: &[u8]) -> Body {
    shown(raw, None)
}

/// The body of `raw` as the reading view shows it.
fn shown(raw: &[u8], security: Option<Secured>) -> Body {
    let view = katna_render::message_view(raw);
    // Mail only partly encrypted shows as plain text: text around the
    // opened part could be anyone's, and as HTML it could wrap the opened
    // text into a link to their site (EFAIL).
    let partly_encrypted = matches!(
        &security,
        Some(Secured::Opened(security)) if security.encrypted() && !security.whole
    );
    let doc = katna_render::message_document(raw).filter(|_| !partly_encrypted);
    let (blocks, cut) = match doc {
        Some(_) => (Vec::new(), false),
        None => body_blocks(&view.body, MAX_BODY_LINES),
    };
    let remote = doc.as_ref().map(rich::remote_urls).unwrap_or_default();
    Body {
        view: Some(view),
        blocks,
        cut,
        doc,
        remote,
        security,
        sealed: None,
        opened: None,
    }
}

/// Splits a body into runs of quoted (`>`) and unquoted lines, at most
/// `max_lines` lines in all. Returns whether lines were left out.
pub(super) fn body_blocks(body: &str, max_lines: usize) -> (Vec<(bool, SharedString)>, bool) {
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
    fn first_names() {
        assert_eq!(first_name("Ada Lovelace"), "Ada");
        assert_eq!(first_name("Lovelace, Ada"), "Ada");
        assert_eq!(first_name("\"Ada Lovelace\""), "Ada");
        assert_eq!(first_name("Dr. Ada Lovelace"), "Dr. Ada Lovelace");
        assert_eq!(first_name("Ada"), "Ada");
        assert_eq!(first_name("J Smith"), "J Smith");
    }

    #[test]
    fn recipients_by_first_name_unless_full_or_shared() {
        let address = |name: Option<&str>, email: &str| katna_render::Address {
            name: name.map(str::to_owned),
            email: email.to_owned(),
        };
        let me = address(Some("Sam Roy"), "sam@example.org");
        let ada = address(Some("Ada Lovelace"), "ada@example.org");
        let ada_b = address(Some("Ada Byron"), "byron@example.org");
        let bare = address(None, "ops@example.org");
        let people = [(true, &me), (false, &ada), (false, &bare)];
        assert_eq!(
            recipient_names(&people, false),
            ["me", "Ada", "ops@example.org"]
        );
        assert_eq!(
            recipient_names(&people, true),
            ["me", "Ada Lovelace", "ops@example.org"]
        );
        let twins = [(false, &ada), (false, &ada_b)];
        assert_eq!(
            recipient_names(&twins, false),
            ["Ada Lovelace", "Ada Byron"]
        );
    }

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

    #[test]
    fn toolbar_gives_way_least_used_first_and_keeps_the_arrows() {
        let shown = Toolbar {
            back: true,
            separators: true,
            contact: true,
            colors: true,
            new_window: true,
            position: Some(80.0),
            arrows: true,
        };
        let wide = Squeeze::fit(2000.0, &shown, Squeeze::NONE);
        assert!(!wide.new_window && !wide.position && !wide.delete);
        let mut last = 0;
        for width in (150..900).rev().step_by(10) {
            let squeeze = Squeeze::fit(width as f32, &shown, Squeeze::NONE);
            let dropped = [
                squeeze.new_window,
                squeeze.print,
                squeeze.colors,
                squeeze.contact,
                squeeze.move_to,
                squeeze.unread,
                squeeze.spam,
                squeeze.separators,
                squeeze.delete,
                squeeze.position,
            ];
            let n = dropped.iter().filter(|d| **d).count();
            // One by one, in order, and never back as it narrows.
            assert!(dropped.iter().take(n).all(|d| *d));
            assert!(n >= last);
            last = n;
            // What stays fits, down to Back, Archive, More and the arrows.
            if n < dropped.len() {
                assert!(shown.width(&squeeze) <= width as f32);
            }
        }
        assert_eq!(last, 10);
    }
}
