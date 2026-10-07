// SPDX-License-Identifier: GPL-3.0-or-later

//! After Send, as in Gmail: the snackbar counts the undo-send delay down
//! with Undo, then says "Message sent" (with a short sound) once the
//! message has really gone out. A reply shows at once as a message of its
//! conversation, where its compose card was, until the store has the copy
//! the mail server filed in Sent.

use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use gpui::Context;
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::MessageId;

use super::super::{MailWindow, SNACKBAR_TIME};
use crate::daemon::Command;
use crate::data::{EntryKey, Row};

/// How long "Sending…" may stay after the countdown before the message
/// is reported sent.
const SENDING_TIME: Duration = Duration::from_secs(60);
/// How long a sent reply stays shown when the mail server's copy never
/// joins its conversation (a forward, say).
const CARD_TIME: i64 = 10 * 60;
/// How often the undo ring beside a reply in the chat view is drawn.
const RING_FRAME: Duration = Duration::from_millis(33);
/// Outbox entries remembered as gone out, for a queue call that returns
/// after the message went.
const WENT_OUT: usize = 16;

/// Messages this window handed to the outbox.
#[derive(Default)]
pub(in crate::window) struct Sending {
    /// Waiting to go out: outbox entry, and whether its conversation was
    /// archived with it.
    waiting: Vec<(i64, bool)>,
    /// Entries seen going out, newest last.
    went_out: VecDeque<i64>,
    /// Replies shown in their conversations until the store has them.
    cards: Vec<SentCard>,
    /// The stand-in ID of the next card.
    next: i64,
}

/// A reply shown in its conversation before the store has it.
#[derive(Clone)]
pub(in crate::window) struct SentCard {
    /// The conversation it answers.
    pub(in crate::window) key: EntryKey,
    /// A stand-in message ID, below zero, that no stored message has.
    pub(in crate::window) id: MessageId,
    /// Its outbox entry, once queued.
    outbox: Option<i64>,
    /// Its `Message-ID`, without angle brackets, to find the stored copy.
    pub(in crate::window) message_id: String,
    /// The message as written, before signing or encryption.
    pub(in crate::window) raw: Arc<Vec<u8>>,
    pub(in crate::window) row: Rc<Row>,
    /// When it went out (Unix seconds); `None` while it waits.
    pub(in crate::window) sent: Option<i64>,
    /// Sent from the chat view: its undo countdown shows beside it rather
    /// than in a snackbar.
    chat: bool,
    /// Until when it can be taken back, and how long that was in all.
    countdown: Option<(Instant, Duration)>,
}

impl SentCard {
    /// Until when it can be taken back out of how long in all, while it
    /// waits in the chat view, and its outbox entry once queued.
    pub(in crate::window) fn countdown(&self) -> Option<(Option<i64>, Instant, Duration)> {
        let (until, total) = self.countdown?;
        Some((self.outbox, until, total))
    }
}

/// A `Message-ID` for a message about to be sent from `domain`, without
/// angle brackets.
pub(super) fn new_message_id(domain: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("katna.{now:x}.{:x}.{n}@{domain}", std::process::id())
}

/// The line of a reply just sent: from `me` (name and address).
pub(super) fn row(
    key: EntryKey,
    id: MessageId,
    account: Option<AccountId>,
    me: (String, String),
    subject: String,
    snippet: String,
) -> Row {
    let (name, address) = me;
    Row {
        key,
        id,
        account: account.unwrap_or(AccountId(0)),
        correspondent: if name.is_empty() {
            address.clone()
        } else {
            name
        },
        people: Vec::new(),
        sender: address,
        count: 1,
        subject,
        date: Some(jiff::Timestamp::now().as_second()),
        unread: false,
        flagged: false,
        important: false,
        pinned: false,
        snoozed_until: None,
        follow_up: None,
        attachments: false,
        files: Vec::new(),
        snippet,
        tracking: None,
        replied: false,
    }
}

impl Sending {
    /// The replies shown in conversation `key`.
    pub(in crate::window) fn cards(&self, key: EntryKey) -> impl Iterator<Item = &SentCard> {
        self.cards.iter().filter(move |c| c.key == key)
    }

    /// Adds a reply to show in conversation `key` (in the chat view with
    /// `chat`); returns its stand-in ID.
    pub(super) fn add_card(
        &mut self,
        key: EntryKey,
        message_id: String,
        raw: Arc<Vec<u8>>,
        chat: bool,
        row: impl FnOnce(MessageId) -> Row,
    ) -> MessageId {
        self.next += 1;
        let id = MessageId(-self.next);
        self.cards.push(SentCard {
            key,
            id,
            outbox: None,
            message_id,
            raw,
            row: Rc::new(row(id)),
            sent: None,
            chat,
            countdown: None,
        });
        id
    }

    /// The reply `id` was queued as outbox entry `outbox`.
    pub(super) fn card_queued(&mut self, id: MessageId, outbox: i64) {
        if let Some(card) = self.cards.iter_mut().find(|c| c.id == id) {
            card.outbox = Some(outbox);
        }
    }

    /// Takes away reply `id`: not sent after all.
    pub(super) fn remove_card(&mut self, id: MessageId) {
        self.cards.retain(|c| c.id != id);
    }

    /// Forgets the replies the store now has (by `Message-ID`, among
    /// `stored`), and those shown long enough after they went out.
    pub(in crate::window) fn prune(&mut self, stored: &[String]) {
        let now = jiff::Timestamp::now().as_second();
        self.cards.retain(|c| {
            !stored.iter().any(|m| m.eq_ignore_ascii_case(&c.message_id))
                && c.sent.is_none_or(|at| now - at < CARD_TIME)
        });
    }
}

impl MailWindow {
    /// Starts the undo countdown of chat reply `id`, `delay` seconds,
    /// beside its bubble.
    pub(super) fn chat_countdown(&mut self, id: MessageId, delay: u32, cx: &mut Context<Self>) {
        if delay == 0 {
            return;
        }
        let Some(card) = self.sending.cards.iter_mut().find(|c| c.id == id) else {
            return;
        };
        let total = Duration::from_secs(u64::from(delay));
        let until = Instant::now() + total;
        card.countdown = Some((until, total));
        // The ring runs back smoothly until then.
        cx.spawn(async move |this, cx| {
            while Instant::now() < until {
                cx.background_executor().timer(RING_FRAME).await;
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        })
        .detach();
    }

    /// Outbox entry `id` was queued to go out after `delay` seconds; with
    /// `archived`, its conversation was archived with it.
    pub(in crate::window) fn queued(
        &mut self,
        id: i64,
        delay: u32,
        archived: bool,
        cx: &mut Context<Self>,
    ) {
        let chat = self
            .sending
            .cards
            .iter()
            .any(|c| c.outbox == Some(id) && c.chat);
        if chat {
            // The countdown shows beside the reply in the chat, from Send.
        } else if delay > 0 {
            let until = Instant::now() + Duration::from_secs(u64::from(delay));
            self.show_countdown(
                tr!("compose-sending"),
                Command::UndoSend(id),
                until,
                Duration::from_secs(u64::from(delay)) + SENDING_TIME,
                cx,
            );
        } else {
            self.show_snackbar_for(tr!("compose-sending"), None, SENDING_TIME, cx);
        }
        self.sending.waiting.push((id, archived));
        // Gone already, before the outbox said which entry it was.
        if self.sending.went_out.contains(&id) {
            self.went_out(id, cx);
        }
    }

    /// Outbox entry `id` went out.
    pub(in crate::window) fn went_out(&mut self, id: i64, cx: &mut Context<Self>) {
        let now = jiff::Timestamp::now().as_second();
        let mut changed = false;
        for card in &mut self.sending.cards {
            if card.outbox == Some(id) && card.sent.is_none() {
                card.sent = Some(now);
                changed = true;
            }
        }
        if changed {
            self.show_sent_cards(cx);
        }
        let Some(at) = self.sending.waiting.iter().position(|(w, _)| *w == id) else {
            let went = &mut self.sending.went_out;
            went.push_back(id);
            if went.len() > WENT_OUT {
                went.pop_front();
            }
            return;
        };
        let (_, archived) = self.sending.waiting.remove(at);
        self.play_event_sound(katna_core::config::SoundEvent::Sent);
        // The chat shows it went with its ticks.
        if self
            .sending
            .cards
            .iter()
            .any(|c| c.outbox == Some(id) && c.chat)
        {
            return;
        }
        let text = if archived {
            tr!("compose-sent-archived")
        } else {
            tr!("compose-sent")
        };
        self.show_snackbar_for(text, None, SNACKBAR_TIME, cx);
    }

    /// Shows the replies just sent in the open conversation, and forgets
    /// those the store has now.
    pub(in crate::window) fn show_sent_cards(&mut self, cx: &mut Context<Self>) {
        if let Some(reader) = &mut self.reader {
            self.sending.prune(&reader.own_message_ids());
            reader.place_sent(self.sending.cards(reader.key));
        }
        cx.notify();
    }

    /// The server refused outbox entry `id` for good.
    pub(in crate::window) fn send_failed(&mut self, id: i64, cx: &mut Context<Self>) {
        self.sending.waiting.retain(|(w, _)| *w != id);
        let before = self.sending.cards.len();
        self.sending.cards.retain(|c| c.outbox != Some(id));
        if self.sending.cards.len() != before {
            self.show_sent_cards(cx);
        }
    }

    /// Send was undone for outbox entry `id`: its reply goes from the
    /// conversation.
    pub(in crate::window) fn send_undone(&mut self, id: i64, cx: &mut Context<Self>) {
        self.sending.waiting.retain(|(w, _)| *w != id);
        self.send_failed(id, cx);
    }
}
