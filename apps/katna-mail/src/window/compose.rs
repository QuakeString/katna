// SPDX-License-Identifier: GPL-3.0-or-later

//! The compose window: "New Message" docked at the bottom right, as in
//! webmail, with To (and Cc, Bcc), Subject, the formatted body with the
//! signature, attachments, and the bar with Send (and schedule send), the
//! formatting options, attach, link, emoji, photo, signature and more
//! options, and discard. Compose opens it; it can be minimized to its
//! title bar or opened large in the middle.
//!
//! Reply, Reply all and Forward write inline instead, at the foot of the
//! open conversation, and can pop out into the window.
//!
//! Send hands the message to the background service's outbox, which holds
//! it for the undo-send delay (or until the scheduled time); the
//! snackbar's Undo takes it back and opens it again.
//!
//! `tools` draws the bars and their menus, `attach` handles files and
//! pictures, `schedule` the times of schedule send, `popout` the message
//! in a window of its own.

mod attach;
mod checks;
mod popout;
mod recipients;
mod schedule;
mod scheduled;
mod security;
mod signature_editor;
mod tools;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, Entity, ExternalPaths, FocusHandle, Focusable, FontWeight, Hsla,
    ScrollHandle, SharedString, Subscription, Task, Window, canvas, div, prelude::*, rgba,
};
use katna_core::AccountId;
use katna_dbus::OutboxItem;
use katna_i18n::tr;
use katna_render::{Address, MessageView};
use katna_store::MessageId;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::rich::{
    Block, Doc, GrammarCheck, Palette, Para, RichEditor, RichEvent, SpellCheck, Suggest, html,
};
use katna_ui::unpx;
use katna_ui::{InputEvent, InputGrammarMenu, TextInput};

use super::{MailWindow, SNACKBAR_TIME};
use crate::daemon::{self, Command};
use crate::data::EntryKey;
use crate::format;
use crate::grammar;
use crate::outgoing::{self, Mailbox, Outgoing, Part};
use crate::signatures;
use crate::spell::{self, Speller};
use crate::suggest::{Phrases, Suggester};
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, icon, tip};

pub(super) use attach::Attachment;
use checks::Passed;
use recipients::{Field, Suggestions};
pub(super) use scheduled::NAV_KEY as SCHEDULED_NAV_KEY;
use security::Sealing;
pub(super) use signature_editor::signature_content;
use tools::Popup;

const WIDTH: f32 = 560.0;
const MAX_HEIGHT: f32 = 620.0;
const MINIMIZED_WIDTH: f32 = 300.0;
const TITLE_HEIGHT: f32 = 40.0;
/// How long after an edit the editor has drawn its new cursor.
const CURSOR_SETTLE: Duration = Duration::from_millis(24);
/// How long the scroll to a reply that opens inline takes.
const REVEAL: Duration = Duration::from_millis(280);
/// How far below the top of the conversation an opened reply's cursor may
/// end up when the card is taller than the view.
const REVEAL_ABOVE: f32 = 120.0;

/// What the window starts from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    New,
    Reply,
    ReplyAll,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Open,
    Minimized,
    Full,
    /// At the foot of the conversation it answers.
    Inline,
    /// In a window of its own (`popout`).
    Window,
}

pub(super) struct Compose {
    to: Entity<TextInput>,
    cc: Entity<TextInput>,
    bcc: Entity<TextInput>,
    subject: Entity<TextInput>,
    body: Entity<RichEditor>,
    attachments: Vec<Attachment>,
    /// The fields as they were opened, to tell whether anything was
    /// written.
    start: Draft,
    /// The message this replies to.
    thread: Threading,
    kind: Kind,
    /// The conversation an inline reply belongs to.
    conversation: Option<EntryKey>,
    /// The conversation a reply or forward answers, which Send and
    /// archive archives.
    answering: Option<EntryKey>,
    /// The account the message goes out from, chosen when it opened.
    from: Option<AccountId>,
    show_cc: bool,
    show_bcc: bool,
    /// Addresses suggested for the recipient being typed.
    suggest: Option<Suggestions>,
    mode: Mode,
    /// Sign and encrypt.
    sealing: Sealing,
    /// The signature in the body, a [`katna_core::config::Signature::id`].
    signature: Option<u32>,
    /// The formatting bar (Aa) shows.
    format_bar: bool,
    /// The open menu or dialog, if any.
    popup: Option<Popup>,
    /// Fields of the link and schedule dialogs and the emoji search.
    dialog: tools::Dialog,
    shown: Spring,
    closing: bool,
    body_scroll: ScrollHandle,
    /// The quoted message a reply answers, kept out of the text behind a
    /// "..." button until it is opened, as in Gmail. It is still sent.
    trimmed: Option<Vec<Block>>,
    /// The text of the conversation a reply answers, for writing
    /// suggestions.
    answered: String,
    /// Where an inline reply was last drawn, to keep its Send row at the
    /// bottom of the conversation while the rest scrolls under it.
    stick: Rc<Cell<Stick>>,
    /// The underline of grammar mistakes in the subject.
    grammar_color: Hsla,
    _subscriptions: Vec<Subscription>,
}

impl Compose {
    /// What the fields hold now.
    fn fields(&self, cx: &gpui::App) -> Draft {
        let text = |input: &Entity<TextInput>| input.read(cx).text().to_owned();
        Draft {
            to: text(&self.to),
            cc: text(&self.cc),
            bcc: text(&self.bcc),
            subject: text(&self.subject),
            body: {
                let mut body = self.body.read(cx).doc().clone();
                body.blocks.extend(self.trimmed.iter().flatten().cloned());
                body
            },
        }
    }

    /// Something was written that closing would lose.
    fn touched(&self, cx: &gpui::App) -> bool {
        !self.attachments.is_empty() || self.fields(cx) != self.start
    }

    fn title(&self, cx: &gpui::App) -> SharedString {
        let subject = self.subject.read(cx).text().trim();
        if subject.is_empty() {
            tr!("compose-new-message").into()
        } else {
            subject.to_owned().into()
        }
    }

    /// Where the message goes back to from its own window: an answer to a
    /// conversation to the end of it (or the compose window if that
    /// conversation is no longer open), anything else the compose window.
    fn docked_mode(&self) -> Mode {
        if self.conversation.is_some() {
            Mode::Inline
        } else {
            Mode::Open
        }
    }

    fn plain(&self, cx: &gpui::App) -> bool {
        self.body.read(cx).is_plain()
    }
}

/// Where an inline reply's card and its Send row sit in the conversation,
/// in pixels from the top of its content (so scrolling leaves them be).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Stick {
    card_top: f32,
    footer_top: f32,
    footer_height: f32,
}

/// What the compose windows share: the spelling dictionary, loaded once,
/// and the scheduled mail.
#[derive(Default)]
pub(super) struct Writing {
    speller: Option<Rc<Speller>>,
    /// Loading, or why it could not be loaded.
    speller_state: SpellerState,
    /// Harper's grammar rules, in their helper process while a message is
    /// open.
    grammar: Option<Arc<dyn GrammarCheck>>,
    /// Phrases learned from the sent mail, loaded on the first message
    /// written.
    phrases: Option<Rc<Phrases>>,
    phrases_loading: bool,
    /// Messages waiting for their scheduled time, soonest first.
    scheduled: Vec<OutboxItem>,
    /// The list of scheduled mail shows.
    scheduled_open: bool,
    watch: Option<Task<()>>,
    /// The signature being edited on the Settings page, and its bar.
    signature_editor: Option<Entity<RichEditor>>,
    signature_tools: Option<signature_editor::SignatureTools>,
    /// The window of a popped-out message.
    compose_window: Option<popout::Handle>,
    /// That window has the desktop's title bar rather than Katna's.
    popout_server_frame: bool,
}

impl Writing {
    /// How many messages wait for their scheduled time.
    pub(super) fn scheduled_count(&self) -> usize {
        self.scheduled.len()
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
enum SpellerState {
    #[default]
    NotLoaded,
    Loading,
    Failed(String),
    Ready,
}

/// The message a reply or forward starts from.
pub(super) struct Original<'a> {
    pub view: &'a MessageView,
    /// Its date as the reader shows it.
    pub date: String,
}

/// What the fields of a compose window hold.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct Draft {
    to: String,
    cc: String,
    bcc: String,
    subject: String,
    body: Doc,
}

/// The `In-Reply-To` and `References` of a reply.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Threading {
    in_reply_to: Option<String>,
    references: Vec<String>,
}

impl Threading {
    fn of(kind: Kind, view: Option<&MessageView>) -> Self {
        let Some(view) = view.filter(|_| matches!(kind, Kind::Reply | Kind::ReplyAll)) else {
            return Self::default();
        };
        let mut references = view.references.clone();
        references.extend(view.message_id.clone());
        Self {
            in_reply_to: view.message_id.clone(),
            references,
        }
    }
}

/// A message handed to the outbox, kept so that Undo can reopen it.
pub(super) struct Unsent {
    draft: Draft,
    thread: Threading,
    sealing: Sealing,
    signature: Option<u32>,
    attachments: Vec<Attachment>,
    plain: bool,
    from: Option<AccountId>,
    answering: Option<EntryKey>,
    /// Puts back the conversation Send and archive archived.
    pub(super) unarchive: Option<Command>,
}

fn address(a: &Address) -> String {
    match a.name.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
        Some(name) => format!("{name} <{}>", a.email),
        None => a.email.clone(),
    }
}

fn addresses<'a>(list: impl IntoIterator<Item = &'a Address>) -> String {
    let mut seen = std::collections::HashSet::new();
    list.into_iter()
        .filter(|a| seen.insert(a.email.to_lowercase()))
        .map(address)
        .collect::<Vec<_>>()
        .join(", ")
}

/// `subject` with `prefix` ("Re:" or "Fwd:") once.
fn prefixed(prefix: &str, subject: &str) -> String {
    let subject = subject.trim();
    let has = subject
        .get(..prefix.len())
        .is_some_and(|start| start.eq_ignore_ascii_case(prefix));
    if has {
        subject.to_owned()
    } else if subject.is_empty() {
        prefix.to_owned()
    } else {
        format!("{prefix} {subject}")
    }
}

fn para(text: impl Into<String>) -> Block {
    Block::Para(Para::plain(text))
}

/// The message's text, quoted one level deeper.
fn quoted(body: &str) -> Vec<Block> {
    let mut doc = html::from_plain(body.trim_end());
    for block in &mut doc.blocks {
        if let Block::Para(p) = block {
            p.style.quote += 1;
        }
    }
    doc.blocks
}

/// The quoted conversation in a reply's `doc`, with its "On ... wrote:"
/// line.
fn thread_text(doc: &Doc) -> String {
    let paras: Vec<&str> = doc
        .blocks
        .iter()
        .filter_map(|b| match b {
            Block::Para(p) => Some(p),
            _ => None,
        })
        .skip_while(|p| p.style.quote == 0 && !p.text.trim_end().ends_with("wrote:"))
        .filter(|p| !p.style.signature)
        .map(|p| p.text.as_str())
        .collect();
    paras.join("\n")
}

/// Takes the quoted message off the end of a reply's `doc`: the "On ...
/// wrote:" line, the blank line before it and the quote after it.
fn trim_quote(doc: &mut Doc) -> Option<Vec<Block>> {
    fn para(b: &Block) -> Option<&Para> {
        match b {
            Block::Para(p) => Some(p),
            _ => None,
        }
    }
    let at = doc.blocks.iter().rposition(|b| {
        para(b).is_some_and(|p| p.style.quote == 0 && p.text.trim_end().ends_with("wrote:"))
    })?;
    let quote = &doc.blocks[at + 1..];
    if quote.is_empty()
        || !quote
            .iter()
            .all(|b| para(b).is_some_and(|p| p.style.quote > 0))
    {
        return None;
    }
    let blank = at > 1
        && para(&doc.blocks[at - 1])
            .is_some_and(|p| p.text.is_empty() && !p.style.signature && p.style.quote == 0);
    Some(doc.blocks.split_off(if blank { at - 1 } else { at }))
}

fn draft(
    kind: Kind,
    original: Option<&Original>,
    is_me: impl Fn(&str) -> bool,
    signature: Option<Doc>,
) -> Draft {
    let mut body = Doc {
        blocks: vec![para("")],
    };
    if let Some(signature) = signature.filter(|s| !s.is_blank()) {
        body.blocks.push(para(""));
        let at = body.blocks.len();
        katna_ui::rich::insert_signature_doc(&mut body, at, signature);
    }
    let Some(Original { view, date }) = original.filter(|_| kind != Kind::New) else {
        return Draft {
            body,
            ..Draft::default()
        };
    };
    let sender = view.from.first();
    let from_text = sender.map(address).unwrap_or_default();
    match kind {
        Kind::New => unreachable!(),
        Kind::Reply | Kind::ReplyAll => {
            // A reply to my own message goes to its recipients.
            let mine = sender.is_some_and(|a| is_me(&a.email));
            let mut to: Vec<&Address> = if mine {
                view.to.iter().collect()
            } else {
                sender.into_iter().collect()
            };
            let mut cc = Vec::new();
            if kind == Kind::ReplyAll {
                to.extend(view.to.iter().filter(|a| !is_me(&a.email)));
                cc.extend(view.cc.iter().filter(|a| !is_me(&a.email)));
            }
            let to_emails: Vec<String> = to.iter().map(|a| a.email.to_lowercase()).collect();
            cc.retain(|a| !to_emails.contains(&a.email.to_lowercase()));
            body.blocks.push(para(""));
            body.blocks
                .push(para(format!("On {date}, {from_text} wrote:")));
            body.blocks.extend(quoted(&view.body));
            Draft {
                to: addresses(to),
                cc: addresses(cc),
                subject: prefixed("Re:", &view.subject),
                body,
                ..Draft::default()
            }
        }
        Kind::Forward => {
            body.blocks.push(para(""));
            body.blocks
                .push(para("---------- Forwarded message ---------"));
            body.blocks.push(para(format!("From: {from_text}")));
            body.blocks.push(para(format!("Date: {date}")));
            body.blocks.push(para(format!("Subject: {}", view.subject)));
            body.blocks
                .push(para(format!("To: {}", addresses(&view.to))));
            if !view.cc.is_empty() {
                body.blocks
                    .push(para(format!("Cc: {}", addresses(&view.cc))));
            }
            body.blocks.push(para(""));
            body.blocks
                .extend(html::from_plain(view.body.trim_end()).blocks);
            Draft {
                subject: prefixed("Fwd:", &view.subject),
                body,
                ..Draft::default()
            }
        }
    }
}

impl MailWindow {
    /// Opens the compose window. `source` picks the message a reply or
    /// forward starts from; by default the newest of the open conversation.
    pub(super) fn open_compose(
        &mut self,
        kind: Kind,
        source: Option<MessageId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let open = self.reader.as_ref().map(|r| r.key);
        if let Some(compose) = &mut self.compose
            && !compose.closing
            && compose.touched(cx)
        {
            let inline_here = compose.mode == Mode::Inline && compose.conversation == open;
            if inline_here {
                window.focus(&compose.body.focus_handle(cx), cx);
            } else if compose.mode == Mode::Window {
                self.pop_out_compose(window, cx);
            } else {
                if compose.mode == Mode::Minimized {
                    compose.mode = Mode::Open;
                }
                self.show_snackbar(tr!("compose-open-elsewhere"), None, cx);
            }
            cx.notify();
            return;
        }
        let date = |d: Option<i64>| {
            d.and_then(|d| format::local(d, &self.tz))
                .map(format::long_date)
                .unwrap_or_default()
        };
        let view = self.reader.as_ref().and_then(|reader| reader.view(source));
        let sealing = match kind {
            Kind::New => Sealing::default(),
            _ => Sealing::answering(self.reader.as_ref().and_then(|r| r.security(source))),
        };
        let original = view.map(|view| Original {
            view,
            date: date(view.date),
        });
        let accounts = &self.accounts;
        let is_me = |email: &str| {
            accounts
                .iter()
                .any(|a| a.address.eq_ignore_ascii_case(email))
        };
        let signature = self.signature_for(kind);
        let signature_doc = self
            .config
            .sending
            .signature(signature)
            .map(signatures::doc);
        let draft = draft(kind, original.as_ref(), is_me, signature_doc);
        let thread = Threading::of(kind, view);
        let reply = matches!(kind, Kind::Reply | Kind::ReplyAll) && !draft.to.is_empty();
        let start = draft.clone();
        // Answers to the open conversation are written inside it.
        let conversation = self
            .reader
            .as_ref()
            .filter(|_| self.reading && kind != Kind::New)
            .map(|r| r.key);
        let mode = if conversation.is_some() {
            Mode::Inline
        } else if self.config.sending.compose_full_screen {
            Mode::Full
        } else {
            Mode::Open
        };
        let plain = self.config.sending.plain_text;
        let answering = self
            .reader
            .as_ref()
            .filter(|_| kind != Kind::New)
            .map(|r| r.key);
        let from = self.compose_account(kind).map(|a| a.id);
        self.show_compose(draft, start, thread, signature, reply, window, cx);
        if let Some(compose) = &mut self.compose {
            compose.kind = kind;
            compose.mode = mode;
            compose.conversation = conversation;
            compose.answering = answering;
            compose.from = from;
            if plain {
                let doc = compose.body.read(cx).doc().clone();
                let plain_doc = html::from_plain(&html::to_plain(&doc));
                compose.body.update(cx, |editor, cx| {
                    editor.set_doc(plain_doc.clone(), plain_doc.start(), cx);
                    editor.set_plain(true, cx);
                });
                compose.start.body = plain_doc;
            }
            compose.sealing = sealing;
            if matches!(kind, Kind::Reply | Kind::ReplyAll) {
                let mut doc = compose.body.read(cx).doc().clone();
                compose.trimmed = trim_quote(&mut doc);
                if compose.trimmed.is_some() {
                    compose.body.update(cx, |editor, cx| {
                        editor.set_doc(doc.clone(), doc.start(), cx)
                    });
                }
            }
        }
        if mode == Mode::Inline {
            self.reveal_inline_reply(cx);
        }
    }

    /// Starts a new message filled in from a `mailto:` link. An unsent
    /// message already open stays, as it does for Compose.
    pub(super) fn open_mailto(
        &mut self,
        mail: crate::mailto::Mailto,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_compose(Kind::New, None, window, cx);
        let Some(compose) = &mut self.compose else {
            return;
        };
        if compose.kind != Kind::New || compose.touched(cx) {
            return;
        }
        let set = |field: &Entity<TextInput>, text: String, cx: &mut Context<Self>| {
            field.update(cx, |input, cx| input.set_text(text, cx));
        };
        set(&compose.to, mail.to.join(", "), cx);
        set(&compose.cc, mail.cc.join(", "), cx);
        set(&compose.bcc, mail.bcc.join(", "), cx);
        set(&compose.subject, mail.subject.clone(), cx);
        compose.show_cc |= !mail.cc.is_empty();
        compose.show_bcc |= !mail.bcc.is_empty();
        if !mail.body.is_empty() {
            // The text goes where the cursor waits, above the signature.
            let mut doc = compose.body.read(cx).doc().clone();
            let text = html::from_plain(mail.body.trim_end()).blocks;
            doc.blocks.splice(0..1.min(doc.blocks.len()), text);
            compose.body.update(cx, |editor, cx| {
                editor.set_doc(doc.clone(), doc.start(), cx)
            });
        }
        let focus = if mail.to.is_empty() {
            compose.to.focus_handle(cx)
        } else if mail.subject.is_empty() {
            compose.subject.focus_handle(cx)
        } else {
            compose.body.focus_handle(cx)
        };
        window.focus(&focus, cx);
        cx.notify();
    }

    /// Scrolls the conversation smoothly to the reply that just opened at
    /// its end, as Gmail does: to the end when the whole card fits, else
    /// just far enough that its first line, with the cursor, sits near the
    /// top.
    pub(super) fn reveal_inline_reply(&mut self, cx: &mut Context<Self>) {
        let Some(body) = self.compose.as_ref().map(|c| c.body.clone()) else {
            return;
        };
        let scroll = self.reader_scroll.clone();
        let reduce = cx.reduce_motion();
        cx.spawn(async move |this, cx| {
            // The frame that lays out the new card first.
            cx.background_executor().timer(CURSOR_SETTLE).await;
            let from = scroll.offset().y;
            // How far down the cursor may go: room above it for the card's
            // recipients and the end of the message being answered.
            let limit = cx.update(|cx| {
                body.read(cx)
                    .cursor_bounds()
                    .map(|cursor| from + scroll.bounds().top() + px(REVEAL_ABOVE) - cursor.top())
            });
            let start = Instant::now();
            loop {
                let t = if reduce {
                    1.0
                } else {
                    (start.elapsed().as_secs_f32() / REVEAL.as_secs_f32()).min(1.0)
                };
                let eased = 1.0 - (1.0 - t).powi(3);
                // The end can move while the card settles. Never upwards.
                let end = -scroll.max_offset().y;
                let to = limit.map_or(end, |limit| end.max(limit.min(from)));
                scroll.set_offset(gpui::point(scroll.offset().x, from + (to - from) * eased));
                if this.update(cx, |_, cx| cx.notify()).is_err() || t >= 1.0 {
                    return;
                }
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
            }
        })
        .detach();
    }

    /// The signature a new message starts with: for new mail the default;
    /// in a conversation the one the user signed their newest message in it
    /// with, else the default for replies.
    fn signature_for(&self, kind: Kind) -> Option<u32> {
        let sending = &self.config.sending;
        let id = match kind {
            Kind::New => sending.new_mail_signature,
            _ => self.signature_used().or(sending.reply_signature),
        };
        sending.signature(id).map(|s| s.id)
    }

    /// The signature of the user's newest message in the open conversation.
    fn signature_used(&self) -> Option<u32> {
        let (Some(reader), Ok(mail)) = (&self.reader, &self.mail) else {
            return None;
        };
        let signatures = &self.config.sending.signatures;
        if signatures.is_empty() {
            return None;
        }
        let is_me = |email: &str| {
            self.accounts
                .iter()
                .any(|a| a.address.eq_ignore_ascii_case(email))
        };
        mail.entry_messages(reader.key)
            .into_iter()
            .rev()
            .filter_map(|id| mail.raw(id))
            .map(|raw| katna_render::message_view(&raw))
            .filter(|view| view.from.iter().any(|a| is_me(&a.email)))
            .find_map(|view| signatures::used_in(&view.body, signatures))
    }

    /// Puts signature `id` (or none) in the open message in place of the
    /// one there.
    fn choose_signature(&mut self, id: Option<u32>, cx: &mut Context<Self>) {
        let new = self.config.sending.signature(id).map(signatures::doc);
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.popup = None;
        if compose.signature == id {
            cx.notify();
            return;
        }
        let plain = compose.plain(cx);
        let new = new.map(|doc| {
            if plain {
                html::from_plain(&html::to_plain(&doc))
            } else {
                doc
            }
        });
        compose
            .body
            .update(cx, |editor, cx| editor.replace_signature(new, cx));
        compose.signature = id;
        cx.notify();
    }

    /// Opens the compose window on `draft`; `start` is what counts as
    /// untouched.
    #[allow(clippy::too_many_arguments)]
    fn show_compose(
        &mut self,
        draft: Draft,
        start: Draft,
        thread: Threading,
        signature: Option<u32>,
        focus_body: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // A popped-out message left untouched gives way to the new one.
        self.close_compose_window(cx);
        let th = self.theme(window);
        let accent: Hsla = rgba(th.accent).into();
        let input = |placeholder: &str, text: &str, cx: &mut Context<Self>| {
            let (placeholder, text) = (placeholder.to_owned(), text.to_owned());
            cx.new(|cx| {
                let mut input = TextInput::new(placeholder, cx);
                input.set_text(text, cx);
                input.set_accent(accent);
                input
            })
        };
        let to = input(&tr!("compose-recipients"), &draft.to, cx);
        let cc = input("", &draft.cc, cx);
        let bcc = input("", &draft.bcc, cx);
        let subject = input(&tr!("compose-subject"), &draft.subject, cx);
        let speller = self.speller(cx);
        let grammar = self.grammar();
        let answered = thread_text(&draft.body);
        let suggest = self.suggestions(&answered, cx);
        subject.update(cx, |input, cx| {
            input.set_grammar_check(grammar.clone(), grammar_color(&th), cx)
        });
        let body = cx.new(|cx| {
            let mut editor = RichEditor::new("", cx);
            editor.set_palette(palette(&th));
            editor.set_doc(draft.body.clone(), draft.body.start(), cx);
            editor.set_spell_check(speller, cx);
            editor.set_grammar_check(grammar, cx);
            editor.set_suggest(suggest, cx);
            editor
        });
        let mut subscriptions = Vec::new();
        // Enter in a field moves on to the next one; typing a recipient
        // suggests addresses.
        let fields: [(&Entity<TextInput>, FocusHandle, Option<Field>); 4] = [
            (&to, subject.focus_handle(cx), Some(Field::To)),
            (&cc, subject.focus_handle(cx), Some(Field::Cc)),
            (&bcc, subject.focus_handle(cx), Some(Field::Bcc)),
            (&subject, body.focus_handle(cx), None),
        ];
        for (input, next, field) in fields {
            subscriptions.push(cx.subscribe_in(
                input,
                window,
                move |this, _, event: &InputEvent, window, cx| match (event, field) {
                    (InputEvent::Submit, _) => window.focus(&next, cx),
                    (InputEvent::Changed, Some(field)) => this.recipient_changed(field, cx),
                    (InputEvent::Changed | InputEvent::Cancel, _) => cx.notify(),
                },
            ));
        }
        subscriptions.push(
            cx.subscribe(&subject, |this, _, event: &InputGrammarMenu, cx| {
                if let Some(c) = &mut this.compose {
                    c.popup = Some(Popup::SubjectGrammar {
                        position: event.position,
                        issue: event.issue.clone(),
                    });
                    cx.notify();
                }
            }),
        );
        self.load_address_book(cx);
        subscriptions.push(cx.subscribe_in(
            &body,
            window,
            |this, _, event: &RichEvent, window, cx| match event {
                RichEvent::Submit => this.send_compose_default(window, cx),
                RichEvent::Changed => {
                    this.keep_cursor_in_view(cx);
                    cx.notify();
                }
                RichEvent::Selection => cx.notify(),
                RichEvent::Cancel => {
                    if let Some(c) = &mut this.compose
                        && c.popup.take().is_some()
                    {
                        cx.notify();
                    }
                }
                RichEvent::EditLink => this.open_link_dialog(window, cx),
                RichEvent::ContextMenu {
                    position,
                    misspelled,
                    grammar,
                } => this.open_compose_menu(*position, misspelled.clone(), grammar.clone(), cx),
            },
        ));
        let focus = if focus_body {
            body.focus_handle(cx)
        } else {
            to.focus_handle(cx)
        };
        window.focus(&focus, cx);
        let dialog = tools::Dialog::new(accent, cx);
        subscriptions.extend(dialog.subscribe(window, cx));
        self.compose = Some(Compose {
            to,
            show_cc: !draft.cc.is_empty(),
            cc,
            show_bcc: !draft.bcc.is_empty(),
            bcc,
            suggest: None,
            subject,
            body,
            attachments: Vec::new(),
            start,
            thread,
            kind: Kind::New,
            conversation: None,
            answering: None,
            from: None,
            mode: Mode::Open,
            sealing: Sealing::default(),
            signature,
            format_bar: false,
            popup: None,
            dialog,
            shown: Spring::new(motion::SLIDE, 0.0),
            closing: false,
            body_scroll: ScrollHandle::new(),
            trimmed: None,
            answered,
            stick: Rc::default(),
            grammar_color: grammar_color(&th),
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    /// The spelling dictionary if spell check is on; starts loading it the
    /// first time.
    fn speller(&mut self, cx: &mut Context<Self>) -> Option<Rc<dyn SpellCheck>> {
        if !self.config.sending.spell_check {
            return None;
        }
        if self.writing.speller_state == SpellerState::NotLoaded {
            self.writing.speller_state = SpellerState::Loading;
            let language = spell::language(&self.config.sending.spell_language);
            let personal = self
                .config_path
                .parent()
                .map(|dir| dir.join("dictionary"))
                .unwrap_or_default();
            cx.spawn(async move |this, cx| {
                let loaded = cx
                    .background_executor()
                    .spawn(async move { spell::load(&language, personal) })
                    .await;
                this.update(cx, |this, cx| {
                    match loaded {
                        Ok(speller) => {
                            this.writing.speller = Some(Rc::new(speller));
                            this.writing.speller_state = SpellerState::Ready;
                        }
                        Err(err) => {
                            tracing::info!("spell check: {err}");
                            this.writing.speller_state = SpellerState::Failed(err);
                        }
                    }
                    // The open message picks it up.
                    let speller = this.speller(cx);
                    if let Some(compose) = &this.compose {
                        compose
                            .body
                            .update(cx, |editor, cx| editor.set_spell_check(speller, cx));
                    }
                })
                .ok();
            })
            .detach();
        }
        self.writing
            .speller
            .clone()
            .map(|s| s as Rc<dyn SpellCheck>)
    }

    /// Harper's grammar rules if grammar checking is on; starts their
    /// helper process for the first message open.
    pub(super) fn grammar(&mut self) -> Option<Arc<dyn GrammarCheck>> {
        if !self.config.sending.grammar_check {
            self.writing.grammar = None;
            return None;
        }
        if self.writing.grammar.is_none() {
            let language = spell::language(&self.config.sending.spell_language);
            match grammar::Helper::start(&language, &self.config.general.language) {
                Ok(helper) => self.writing.grammar = Some(Arc::new(helper)),
                Err(err) => tracing::warn!("grammar checking: {err}"),
            }
        }
        self.writing.grammar.clone()
    }

    /// The open message checks grammar, or stops, as Settings says.
    pub(super) fn grammar_changed(&mut self, cx: &mut Context<Self>) {
        if self.compose.is_none() {
            self.writing.grammar = None;
            return;
        }
        let grammar = self.grammar();
        if let Some(compose) = &self.compose {
            let color = compose.grammar_color;
            compose.body.update(cx, |editor, cx| {
                editor.set_grammar_check(grammar.clone(), cx)
            });
            compose
                .subject
                .update(cx, |input, cx| input.set_grammar_check(grammar, color, cx));
        }
    }

    /// Writing suggestions if they are on; starts learning the phrases the
    /// first time.
    fn suggestions(&mut self, thread: &str, cx: &mut Context<Self>) -> Option<Rc<dyn Suggest>> {
        if !self.config.sending.writing_suggestions {
            return None;
        }
        if self.writing.phrases.is_none() && !self.writing.phrases_loading {
            self.writing.phrases_loading = true;
            let paths = self.paths.clone();
            cx.spawn(async move |this, cx| {
                let phrases = cx
                    .background_executor()
                    .spawn(async move { Phrases::learn(&paths) })
                    .await;
                this.update(cx, |this, cx| {
                    this.writing.phrases = Some(Rc::new(phrases));
                    this.writing.phrases_loading = false;
                    this.suggestions_changed(cx);
                })
                .ok();
            })
            .detach();
        }
        // Common phrases and the conversation until the sent mail is read.
        let sent = self
            .writing
            .phrases
            .clone()
            .unwrap_or_else(|| Rc::new(Phrases::built_in()));
        Some(Rc::new(Suggester::new(sent, thread)))
    }

    /// The open message suggests, or stops, as Settings says.
    pub(super) fn suggestions_changed(&mut self, cx: &mut Context<Self>) {
        let thread = self
            .compose
            .as_ref()
            .map(|c| c.answered.clone())
            .unwrap_or_default();
        let suggest = self.suggestions(&thread, cx);
        if let Some(compose) = &self.compose {
            compose
                .body
                .update(cx, |editor, cx| editor.set_suggest(suggest, cx));
        }
    }

    /// Scrolls the body so the cursor stays in view while typing. The
    /// editor reports where its cursor was drawn, so this waits for the
    /// frame that draws the change.
    fn keep_cursor_in_view(&mut self, cx: &mut Context<Self>) {
        let Some(compose) = &self.compose else {
            return;
        };
        // An inline reply has no scroll of its own: the conversation scrolls,
        // under its Send row.
        let (scroll, mut covered) = if compose.mode == Mode::Inline {
            (
                self.reader_scroll.clone(),
                compose.stick.get().footer_height,
            )
        } else {
            (compose.body_scroll.clone(), 0.0)
        };
        if compose.format_bar {
            covered += tools::FORMAT_BAR_COVER;
        }
        let body = compose.body.clone();
        cx.spawn(async move |_, cx| {
            cx.background_executor().timer(CURSOR_SETTLE).await;
            cx.update(|cx| {
                let Some(cursor) = body.read(cx).cursor_bounds() else {
                    return;
                };
                let mut view = scroll.bounds();
                view.size.height -= px(covered);
                let pad = px(12.0);
                let mut offset = scroll.offset();
                if cursor.bottom() + pad > view.bottom() {
                    offset.y -= cursor.bottom() + pad - view.bottom();
                } else if cursor.top() - pad < view.top() {
                    offset.y += view.top() - (cursor.top() - pad);
                } else {
                    return;
                }
                offset.y = offset.y.min(px(0.0));
                scroll.set_offset(offset);
                body.update(cx, |_, cx| cx.notify());
            });
        })
        .detach();
    }

    /// The account a message goes out from: for new mail the one chosen in
    /// Settings, if any; else that of the open folder (the one a reply
    /// answers in), or the first.
    fn compose_account(&self, kind: Kind) -> Option<&katna_core::Account> {
        let chosen = &self.config.sending.send_from;
        let fixed = (kind == Kind::New && !chosen.is_empty())
            .then(|| {
                self.accounts
                    .iter()
                    .find(|a| a.address.eq_ignore_ascii_case(chosen))
            })
            .flatten();
        let open = self.folder.and_then(|folder| self.tree.account_of(folder));
        fixed.or_else(|| {
            open.or_else(|| self.shown_account())
                .and_then(|id| self.accounts.iter().find(|a| a.id == id))
                .or_else(|| self.accounts.first())
        })
    }

    /// Sends the open message the way Settings chooses: a reply or forward
    /// also archives its conversation when Send and archive is the default.
    pub(super) fn send_compose_default(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let archive = self.config.sending.send_and_archive;
        self.send_compose(None, archive, Passed::default(), window, cx);
    }

    /// Sends the open message, now (after the undo delay) or at `at`. With
    /// `archive`, a reply or forward also archives the conversation it
    /// answers once the message is on its way. First it asks about a
    /// missing attachment or subject, unless `passed`.
    fn send_compose(
        &mut self,
        at: Option<jiff::Timestamp>,
        archive: bool,
        passed: Passed,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.popup = None;
        let draft = compose.fields(cx);
        let thread = compose.thread.clone();
        let sealing = compose.sealing;
        let kind = compose.kind;
        let chosen = compose.from;
        let answered = compose.answering;
        let answering = answered.filter(|_| archive && at.is_none());
        let signature = compose.signature;
        let attachments = compose.attachments.clone();
        let plain = compose.plain(cx);
        let parse = |text: &str| outgoing::parse_addresses(text);
        let (to, cc, bcc) = match (parse(&draft.to), parse(&draft.cc), parse(&draft.bcc)) {
            (Ok(to), Ok(cc), Ok(bcc)) => (to, cc, bcc),
            (Err(bad), ..) | (_, Err(bad), _) | (.., Err(bad)) => {
                self.show_snackbar(tr!("compose-bad-address", address = bad), None, cx);
                return;
            }
        };
        if to.is_empty() && cc.is_empty() && bcc.is_empty() {
            self.show_snackbar(tr!("compose-no-recipients"), None, cx);
            return;
        }
        let total: usize = attachments.iter().map(|a| a.data.len()).sum::<usize>()
            + draft.body.images().map(|i| i.data.len()).sum::<usize>();
        if total > attach::MAX_TOTAL {
            self.show_snackbar(
                tr!(
                    "compose-attachments-too-large",
                    size = format::size(total as u64),
                    limit = format::size(attach::MAX_TOTAL as u64)
                ),
                None,
                cx,
            );
            return;
        }
        let attached = attachments.len() + draft.body.images().count();
        if let Some(check) = checks::check(&draft.subject, &draft.body, attached, passed) {
            if let Some(c) = &mut self.compose {
                c.popup = Some(Popup::SendCheck {
                    check,
                    at,
                    archive,
                    passed,
                });
            }
            cx.notify();
            return;
        }
        let account = chosen
            .and_then(|id| self.accounts.iter().find(|a| a.id == id))
            .or_else(|| self.compose_account(kind));
        let Some(account) = account else {
            self.show_snackbar(tr!("compose-no-account"), None, cx);
            self.open_add_account(window, cx);
            return;
        };
        let from = Mailbox {
            name: Some(account.display_name.trim().to_owned()).filter(|n| !n.is_empty()),
            email: account.address.clone(),
        };
        let domain = account
            .address
            .rsplit_once('@')
            .map_or("katna.local", |(_, d)| d)
            .to_owned();
        let body_text = html::to_plain(&draft.body);
        // Each picture gets a name of its own in the message, even when
        // the same one was pasted twice.
        let mut doc = draft.body.clone();
        for (ix, block) in doc.blocks.iter_mut().enumerate() {
            if let Block::Image(image) = block {
                image.id = ix as u64;
            }
        }
        let (html_body, inline) = if plain {
            (None, Vec::new())
        } else {
            let seed = std::process::id() as u64 ^ jiff::Timestamp::now().as_millisecond() as u64;
            let cid = |id: u64| format!("ii_{seed:x}_{id}@{domain}");
            let html = html::to_html(&doc, &|image| format!("cid:{}", cid(image.id)));
            let inline = doc
                .images()
                .map(|image| Part {
                    name: image.name.clone(),
                    mime: image.mime.clone(),
                    data: image.data.clone(),
                    content_id: Some(cid(image.id)),
                })
                .collect();
            (Some(html), inline)
        };
        let delay = match at {
            Some(at) => {
                let seconds = at.as_second() - jiff::Timestamp::now().as_second();
                if seconds < 60 {
                    self.show_snackbar(tr!("compose-past-time"), None, cx);
                    return;
                }
                u32::try_from(seconds).unwrap_or(u32::MAX)
            }
            None => self.config.sending.undo_send_seconds,
        };
        let emails = |list: &[Mailbox]| list.iter().map(|m| m.email.clone()).collect::<Vec<_>>();
        let sender = account.address.clone();
        let visible = [emails(&to), emails(&cc)].concat();
        let hidden = emails(&bcc);
        let recipients: Vec<Mailbox> = to.iter().chain(&cc).chain(&bcc).cloned().collect();
        let raw = outgoing::build(&Outgoing {
            from: Some(from),
            to,
            cc,
            bcc,
            subject: draft.subject.clone(),
            body: body_text,
            in_reply_to: thread.in_reply_to.clone(),
            references: thread.references.clone(),
            html: html_body,
            inline,
            attachments: attachments.iter().map(Attachment::part).collect(),
            date: at.map(|at| schedule::rfc2822(at, &self.tz)),
        });
        let from = Some(account.id);
        let account = account.id.0;
        self.note_recipients(account, &recipients, cx);
        self.unsent = Some(Unsent {
            draft,
            thread,
            sealing,
            signature,
            attachments,
            plain,
            from,
            answering: answered,
            unarchive: None,
        });
        self.close_compose(false, cx);
        self.show_snackbar(
            if at.is_some() {
                tr!("compose-scheduling")
            } else {
                tr!("compose-sending")
            },
            None,
            cx,
        );
        let connection = self.daemon.clone();
        let when = at.map(|at| schedule::describe(at, &self.tz));
        cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    // Signed and encrypted before the outbox sees it.
                    let raw = security::seal(raw, sealing, sender, visible, hidden)?;
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::queue_send(&connection, account, &raw, delay).await
                })
                .await;
            this.update_in(cx, |this, window, cx| match result {
                Ok(id) => {
                    let text = match &when {
                        Some(when) => tr!("compose-scheduled", when = when.clone()),
                        None if answering.is_some() => tr!("compose-sent-archived"),
                        None => tr!("compose-sent"),
                    };
                    if let Some(key) = answering {
                        let unarchive = this.act_with(super::Act::Archive, vec![key], false, cx);
                        if let Some(unsent) = &mut this.unsent {
                            unsent.unarchive = unarchive;
                        }
                    }
                    let undo = (delay > 0).then_some(Command::UndoSend(id));
                    let time = if when.is_some() {
                        SNACKBAR_TIME * 2
                    } else {
                        Duration::from_secs(u64::from(delay)).max(SNACKBAR_TIME)
                    };
                    this.show_snackbar_for(text, undo, time, cx);
                    if when.is_some() {
                        this.scheduled_changed(cx);
                    }
                }
                Err(err) => {
                    // Nothing went out: the message comes back as it was.
                    this.reopen_unsent(window, cx);
                    this.show_snackbar(err, None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// Opens the message that was just handed to the outbox again, after
    /// Undo or a failure to queue it.
    pub(super) fn reopen_unsent(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Unsent {
            draft,
            thread,
            sealing,
            signature,
            attachments,
            plain,
            from,
            answering,
            unarchive: _,
        }) = self.unsent.take()
        else {
            return;
        };
        if self
            .compose
            .as_ref()
            .is_some_and(|c| !c.closing && c.touched(cx))
        {
            return;
        }
        self.show_compose(draft, Draft::default(), thread, signature, true, window, cx);
        if let Some(compose) = &mut self.compose {
            compose.attachments = attachments;
            compose
                .body
                .update(cx, |editor, cx| editor.set_plain(plain, cx));
            compose.sealing = sealing;
            compose.from = from;
            compose.answering = answering;
        }
    }

    fn close_compose(&mut self, discarded: bool, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            compose.closing = true;
            compose.popup = None;
            if compose.mode == Mode::Window {
                self.compose = None;
            }
        }
        self.close_compose_window(cx);
        if discarded {
            self.show_snackbar(tr!("compose-discarded"), None, cx);
        }
        cx.notify();
    }

    fn compose_mode(&mut self, mode: Mode, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            compose.mode = if compose.mode == mode {
                Mode::Open
            } else {
                mode
            };
            compose.popup = None;
        }
        cx.notify();
    }

    pub(super) fn render_compose(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // Harper's helper is large: it runs only while a message is open.
        if self.compose.is_none() && self.writing.grammar.is_some() {
            self.writing.grammar = None;
        }
        let (mode, conversation, closing) = self
            .compose
            .as_ref()
            .map(|c| (c.mode, c.conversation, c.closing))?;
        if mode == Mode::Window {
            if closing {
                self.compose = None;
            }
            return None;
        }
        if mode == Mode::Inline {
            let here = self.reading
                && self
                    .reader
                    .as_ref()
                    .is_some_and(|r| Some(r.key) == conversation);
            let touched = self.compose.as_ref().is_some_and(|c| c.touched(cx));
            if closing || (!here && !touched) {
                self.compose = None;
                return None;
            }
            if here {
                return None;
            }
            // The conversation was left with a reply half written: it
            // carries on in the window.
            if let Some(compose) = &mut self.compose {
                compose.mode = Mode::Open;
                compose.shown.snap(1.0);
            }
        }
        let compose = self.compose.as_mut()?;
        compose.shown.set(if compose.closing { 0.0 } else { 1.0 });
        let t = compose.shown.tick(window, reduce);
        if compose.closing && compose.shown.settled() {
            self.compose = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let compose = self.compose.as_ref()?;
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let mode = compose.mode;
        let title = compose.title(cx);

        let title_bar = div()
            .id("compose-title")
            .flex_none()
            .h(px(TITLE_HEIGHT))
            .pl(px(16.0))
            .pr(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .bg(rgba(if th.dark { th.menu } else { th.page }))
            // Its own corners too: the window's clip is square.
            .rounded_t(px(12.0))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| this.compose_mode(Mode::Minimized, cx)))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(title),
            )
            .child(
                // Minimized, the same button opens it again.
                small_button(
                    "compose-minimize",
                    if mode == Mode::Minimized {
                        "window-restore"
                    } else {
                        "minimize"
                    },
                    th,
                )
                .tooltip(tip(
                    if mode == Mode::Minimized {
                        tr!("compose-restore")
                    } else {
                        tr!("compose-minimize")
                    },
                    th,
                ))
                .on_click(cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.compose_mode(Mode::Minimized, cx)
                })),
            )
            .when(mode == Mode::Full, |d| {
                d.child(
                    small_button("compose-full", "close-full", th)
                        .tooltip(tip(tr!("compose-exit-full-screen"), th))
                        .on_click(cx.listener(|this, _, _, cx| {
                            cx.stop_propagation();
                            this.compose_mode(Mode::Full, cx)
                        })),
                )
            })
            .child(
                // Gmail's expand button, in a window of its own here.
                small_button("compose-pop-out", "open-full", th)
                    .tooltip(tip(tr!("compose-open-window"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        cx.stop_propagation();
                        this.pop_out_compose(window, cx)
                    })),
            )
            .child(
                small_button("compose-close", "close", th)
                    .tooltip(tip(tr!("compose-save-close"), th))
                    .on_click(cx.listener(|this, _, _, cx| {
                        cx.stop_propagation();
                        // Drafts are not saved yet, so closing loses the text.
                        let touched = this.compose.as_ref().is_some_and(|c| c.touched(cx));
                        this.close_compose(touched, cx)
                    })),
            );

        // On a phone, and a tablet too narrow for the reading pane, the
        // message is written on a sheet over the whole window, top bar and
        // all, as mobile mail does; minimized, it is a strip at the foot.
        let shape = self.layout.shape;
        let sheet = !shape.size.splits(shape.width) && mode != Mode::Minimized;
        let (width, height) = match mode {
            _ if sheet => (shape.width, vh),
            Mode::Minimized if shape.is_phone() => (shape.width - 16.0, TITLE_HEIGHT),
            Mode::Open | Mode::Inline | Mode::Window => {
                (WIDTH.min(vw - 32.0), MAX_HEIGHT.min(vh - 96.0))
            }
            Mode::Minimized => (MINIMIZED_WIDTH, TITLE_HEIGHT),
            Mode::Full => ((vw - 128.0).clamp(WIDTH, 1000.0), vh - 96.0),
        };
        let panel = div()
            .id("compose")
            .key_context("Compose")
            .occlude()
            .relative()
            .w(px(width))
            .map(|d| if sheet { d.h_full() } else { d.h(px(height)) })
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .map(|d| match mode {
                _ if sheet => d,
                Mode::Full => d.rounded(px(12.0)),
                _ => d.rounded_t(px(12.0)),
            })
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.drop_files(paths.paths().to_vec(), cx);
            }))
            .child(title_bar)
            .when(mode != Mode::Minimized, |d| {
                d.child(self.render_compose_fields(th, cx))
                    .child(self.render_compose_body(th, width, cx))
                    .child(self.render_attachments(th, cx))
                    .children(self.render_floating_format_bar(th, width - 24.0, cx))
                    .child(self.render_compose_actions(th, width, cx))
                    .child(self.render_drop_target(th))
                    .children(self.render_compose_dialog(th, cx))
            });

        Some(match mode {
            _ if sheet => div()
                .absolute()
                .top(px(lerp(48.0, 0.0, t) - super::TOP_BAR_HEIGHT))
                .bottom(px(-lerp(48.0, 0.0, t)))
                .left_0()
                .right_0()
                .opacity(t)
                .child(panel)
                .into_any_element(),
            Mode::Full => div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("compose-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.compose_mode(Mode::Full, cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(panel))
                .into_any_element(),
            _ => div()
                .absolute()
                .right(px(lerp(24.0, 8.0, shape.phone)))
                .bottom(px(shape.bottom_bar() + lerp(-48.0, 0.0, t)))
                .opacity(t)
                .child(panel)
                .into_any_element(),
        })
    }

    /// The reply being written at the end of conversation `key`, if any:
    /// the sender's picture beside a card with the recipients, the text and
    /// the Send row. The card grows with its text; the conversation scrolls.
    pub(super) fn render_inline_reply(
        &self,
        key: EntryKey,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self
            .compose
            .as_ref()
            .filter(|c| c.mode == Mode::Inline && !c.closing && c.conversation == Some(key))?;
        let (kind_icon, kind_label) = match compose.kind {
            Kind::ReplyAll => ("reply-all", tr!("reply-reply-all")),
            Kind::Forward => ("forward", tr!("reply-forward")),
            Kind::Reply | Kind::New => ("reply", tr!("reply-reply")),
        };
        let me = compose
            .from
            .and_then(|id| self.accounts.iter().find(|a| a.id == id))
            .or_else(|| self.compose_account(compose.kind))
            .map(|a| {
                let name = if a.display_name.trim().is_empty() {
                    a.address.clone()
                } else {
                    a.display_name.clone()
                };
                self.person_avatar(&name, &a.address, 40.0)
            });
        let header = div()
            .flex_none()
            .h(px(44.0))
            .pl(px(12.0))
            .pr(px(6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .text_size(px(14.0))
            .child(
                div()
                    .id("inline-kind")
                    .flex_none()
                    .tooltip(tip(kind_label, th))
                    .child(icon(kind_icon, th.text_dim, 20.0)),
            )
            .child(div().flex_1().min_w_0().child(compose.to.clone()))
            .when(!compose.show_cc, |d| {
                d.child(
                    div()
                        .id("inline-cc")
                        .px(px(4.0))
                        .rounded(px(4.0))
                        .text_color(rgba(th.text_dim))
                        .cursor_pointer()
                        .hover(|s| s.text_color(rgba(th.text)).bg(rgba(th.hover)))
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(c) = &mut this.compose {
                                c.show_cc = true;
                                window.focus(&c.cc.focus_handle(cx), cx);
                            }
                            cx.notify();
                        }))
                        .child(tr!("compose-cc")),
                )
            })
            .children(self.render_sealing(th, cx))
            .child(
                small_button("inline-pop-out", "open-full", th)
                    .tooltip(tip(tr!("compose-pop-out-reply"), th))
                    // Straight into a window of its own; docking it brings
                    // it back here.
                    .on_click(cx.listener(|this, _, window, cx| this.pop_out_compose(window, cx))),
            );
        let cc = compose.show_cc.then(|| {
            div()
                .flex_none()
                .mx(px(12.0))
                .h(px(36.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .text_size(px(14.0))
                .child(
                    div()
                        .flex_none()
                        .text_color(rgba(th.text_dim))
                        .child(tr!("compose-cc")),
                )
                .child(div().flex_1().min_w_0().child(compose.cc.clone()))
        });
        let focus = compose.body.focus_handle(cx);
        let body = div()
            .id("inline-body")
            .min_h(px(96.0))
            .px(px(12.0))
            .py(px(8.0))
            .text_size(px(14.0))
            .line_height(px(20.0))
            .cursor_text()
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(div().flex_none().child(compose.body.clone()))
            .children(self.render_trimmed(th, cx));
        let card_width = unpx(self.reader_scroll.bounds().size.width) - 100.0;
        // Like Gmail, the Send row stays at the bottom of the conversation
        // while the text runs on below it, and moves up with the card.
        let stuck = {
            let at = compose.stick.get();
            let view = self.reader_scroll.bounds().size.height;
            let bottom = unpx(view - self.reader_scroll.offset().y);
            let highest = at.card_top + STICK_BELOW;
            (at.footer_top + at.footer_height - bottom)
                .clamp(0.0, (at.footer_top - highest).max(0.0))
        };
        let card = div()
            .id("inline-reply")
            .key_context("Compose")
            .relative()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .rounded(px(12.0))
            .bg(rgba(th.surface))
            .border_1()
            .border_color(rgba(th.divider))
            .shadow(elevation(th, 1.5))
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.drop_files(paths.paths().to_vec(), cx);
            }))
            .child(measure(
                &self.reader_scroll,
                &compose.stick,
                cx,
                |stick, top, _| {
                    stick.card_top = top;
                },
            ))
            .child(header)
            .children(cc)
            .child(body)
            .child(self.render_attachments(th, cx))
            .child(
                div()
                    .relative()
                    .flex_none()
                    .child(measure(
                        &self.reader_scroll,
                        &compose.stick,
                        cx,
                        |stick, top, height| {
                            stick.footer_top = top;
                            stick.footer_height = height;
                        },
                    ))
                    .child(
                        div()
                            .relative()
                            .top(px(-stuck))
                            .rounded_b(px(12.0))
                            .bg(rgba(th.surface))
                            .border_t_1()
                            .border_color(if stuck > 0.0 {
                                rgba(th.divider)
                            } else {
                                rgba(0)
                            })
                            .children(self.render_floating_format_bar(
                                th,
                                card_width.max(320.0) - 24.0,
                                cx,
                            ))
                            .child(self.render_compose_actions(th, card_width.max(320.0), cx)),
                    ),
            )
            .child(self.render_drop_target(th))
            .children(self.render_compose_dialog(th, cx));
        Some(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(12.0))
                .pl(px(16.0))
                .pr(px(16.0))
                .pt(px(12.0))
                .pb(px(16.0))
                .children(me)
                .child(card)
                .into_any_element(),
        )
    }

    /// The "..." button under a reply's text that shows the quoted message.
    fn render_trimmed(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.compose.as_ref()?.trimmed.as_ref()?;
        let dot = || div().size(px(4.0)).rounded_full().bg(rgba(th.text_dim));
        Some(
            div()
                .id("show-trimmed")
                .mt(px(12.0))
                .w(px(30.0))
                .h(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_center()
                .gap(px(3.0))
                .rounded(px(8.0))
                .bg(rgba(th.chip))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .tooltip(tip(tr!("compose-show-trimmed"), th))
                .on_click(cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.show_trimmed(cx);
                }))
                .child(dot())
                .child(dot())
                .child(dot())
                .into_any_element(),
        )
    }

    /// Puts the quoted message back into the text of the reply.
    fn show_trimmed(&mut self, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        if let Some(blocks) = compose.trimmed.take() {
            compose
                .body
                .update(cx, |editor, cx| editor.append_blocks(blocks, cx));
        }
        cx.notify();
    }

    fn render_compose_fields(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let row = |label: String, input: &Entity<TextInput>| {
            div()
                .flex_none()
                .mx(px(16.0))
                .h(px(40.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .text_size(px(14.0))
                .when(!label.is_empty(), |d| {
                    d.child(div().flex_none().text_color(rgba(th.text_dim)).child(label))
                })
                .child(div().flex_1().min_w_0().child(input.clone()))
        };
        let link = |id: &'static str, label: String| {
            div()
                .id(id)
                .px(px(4.0))
                .rounded(px(4.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .cursor_pointer()
                .hover(|s| s.text_color(rgba(th.text)).bg(rgba(th.hover)))
                .child(label)
        };
        let to = self
            .recipient_row(row(tr!("compose-to"), &compose.to), Field::To, th, cx)
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .when(!compose.show_cc, |d| {
                        d.child(link("compose-cc", tr!("compose-cc")).on_click(cx.listener(
                            |this, _, window, cx| {
                                if let Some(c) = &mut this.compose {
                                    c.show_cc = true;
                                    window.focus(&c.cc.focus_handle(cx), cx);
                                }
                                cx.notify();
                            },
                        )))
                    })
                    .when(!compose.show_bcc, |d| {
                        d.child(
                            link("compose-bcc", tr!("compose-bcc")).on_click(cx.listener(
                                |this, _, window, cx| {
                                    if let Some(c) = &mut this.compose {
                                        c.show_bcc = true;
                                        window.focus(&c.bcc.focus_handle(cx), cx);
                                    }
                                    cx.notify();
                                },
                            )),
                        )
                    })
                    .children(self.render_sealing(th, cx)),
            );
        div()
            .flex_none()
            .flex()
            .flex_col()
            .child(to)
            .when(compose.show_cc, |d| {
                d.child(self.recipient_row(row(tr!("compose-cc"), &compose.cc), Field::Cc, th, cx))
            })
            .when(compose.show_bcc, |d| {
                d.child(self.recipient_row(
                    row(tr!("compose-bcc"), &compose.bcc),
                    Field::Bcc,
                    th,
                    cx,
                ))
            })
            .child(row(String::new(), &compose.subject))
            .into_any_element()
    }

    fn render_compose_body(&self, th: &Theme, width: f32, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let focus = compose.body.focus_handle(cx);
        let body = compose.body.clone();
        div()
            .id("compose-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .track_scroll(&compose.body_scroll)
            .px(px(16.0))
            .py(px(12.0))
            // The end of the text can scroll up from under the floating
            // formatting bar.
            .when(compose.format_bar, |d| {
                d.pb(px(12.0 + tools::FORMAT_BAR_COVER))
            })
            .text_size(px(14.0))
            .line_height(px(20.0))
            .text_color(rgba(th.text))
            .cursor_text()
            // A click below the text still puts the cursor in the body, at
            // its end.
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(
                div()
                    .flex_none()
                    .w(px((width - 32.0).max(80.0)))
                    .child(body),
            )
            .children(self.render_trimmed(th, cx))
            .into_any_element()
    }
}

/// How far below the top of an inline reply its stuck Send row stops: the
/// recipients and a line or two of text stay above it.
const STICK_BELOW: f32 = 96.0;

/// Records where its parent is drawn in the conversation `scroll`, in
/// pixels from the top of the content, and draws again when that moved.
fn measure(
    scroll: &ScrollHandle,
    stick: &Rc<Cell<Stick>>,
    cx: &Context<MailWindow>,
    set: impl Fn(&mut Stick, f32, f32) + 'static,
) -> impl IntoElement {
    let (scroll, stick, this) = (scroll.clone(), stick.clone(), cx.entity().downgrade());
    canvas(
        move |bounds, _, cx| {
            let top = unpx(bounds.top() - scroll.bounds().top() - scroll.offset().y);
            let mut at = stick.get();
            set(&mut at, top, unpx(bounds.size.height));
            if at != stick.get() {
                stick.set(at);
                // After this frame: a change asked for while drawing is lost.
                let this = this.clone();
                cx.defer(move |cx| {
                    this.update(cx, |_, cx| cx.notify()).ok();
                });
            }
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// The underline of grammar mistakes: amber, apart from spelling's red
/// and the blue of links.
fn grammar_color(th: &Theme) -> Hsla {
    rgba(if th.dark { 0xfdd663ff } else { 0xf9ab00ff }).into()
}

/// The editor's colors from the window's theme.
fn palette(th: &Theme) -> Palette {
    let color = |c: u32| -> Hsla { rgba(c).into() };
    Palette {
        accent: color(th.accent),
        link: color(if th.dark { 0x8ab4f8ff } else { 0x1a0dabff }),
        misspelled: color(th.error),
        grammar: grammar_color(th),
        rule: color(if th.dark { 0x5f6368ff } else { 0xccccccff }),
        surface: color(th.menu),
        text: color(th.text),
        hover: color(th.hover),
    }
}

/// A small button of the title bar.
fn small_button(id: &'static str, name: &'static str, th: &Theme) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .size(px(28.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(icon(name, th.text_dim, 18.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(name: Option<&str>, email: &str) -> Address {
        Address {
            name: name.map(str::to_owned),
            email: email.to_owned(),
        }
    }

    fn view() -> MessageView {
        MessageView {
            subject: "Gas prices".to_owned(),
            from: vec![addr(Some("Kay Mann"), "kay@enron.com")],
            to: vec![
                addr(None, "me@enron.com"),
                addr(Some("Bob"), "bob@enron.com"),
            ],
            cc: vec![addr(None, "sara@enron.com"), addr(None, "kay@enron.com")],
            body: "Hello.\n\nSee you.".to_owned(),
            message_id: Some("1@enron.com".to_owned()),
            references: vec!["0@enron.com".to_owned()],
            ..MessageView::default()
        }
    }

    fn me(email: &str) -> bool {
        email == "me@enron.com"
    }

    fn text(d: &Draft) -> String {
        html::to_plain(&d.body)
    }

    #[test]
    fn a_reply_keeps_its_quote_behind_the_dots() {
        let original = Original {
            view: &view(),
            date: "Tue, 25 Jun 2002, 22:23".to_owned(),
        };
        let signature = Some(html::from_plain("Kay\n"));
        let mut body = draft(Kind::Reply, Some(&original), me, signature).body;
        let full = body.clone();
        let quote = trim_quote(&mut body).expect("the quote");
        assert_eq!(html::to_plain(&body), "\n\n-- \nKay\n");
        let mut back = body.clone();
        back.blocks.extend(quote);
        assert_eq!(back, full);
        // Nothing to fold without a quote.
        let mut new = draft(Kind::New, None, me, None).body;
        assert!(trim_quote(&mut new).is_none());
    }

    #[test]
    fn new_mail_has_the_signature() {
        let d = draft(Kind::New, None, me, Some(html::from_plain("Kay\n")));
        assert_eq!(text(&d), "\n\n-- \nKay\n");
        assert_eq!(text(&draft(Kind::New, None, me, None)), "\n");
    }

    #[test]
    fn replies() {
        let view = view();
        let original = Original {
            view: &view,
            date: "Tue, 25 Jun 2002, 22:23".to_owned(),
        };
        let d = draft(Kind::Reply, Some(&original), me, None);
        assert_eq!(d.to, "Kay Mann <kay@enron.com>");
        assert_eq!(d.cc, "");
        assert_eq!(d.subject, "Re: Gas prices");
        assert_eq!(
            text(&d),
            "\n\nOn Tue, 25 Jun 2002, 22:23, Kay Mann <kay@enron.com> wrote:\n> Hello.\n>\n> See you.\n"
        );
        let d = draft(Kind::ReplyAll, Some(&original), me, None);
        assert_eq!(d.to, "Kay Mann <kay@enron.com>, Bob <bob@enron.com>");
        assert_eq!(d.cc, "sara@enron.com");
    }

    #[test]
    fn forwards() {
        let view = view();
        let original = Original {
            view: &view,
            date: "Tue".to_owned(),
        };
        let d = draft(
            Kind::Forward,
            Some(&original),
            me,
            Some(html::from_plain("K")),
        );
        assert_eq!(d.to, "");
        assert_eq!(d.subject, "Fwd: Gas prices");
        let body = text(&d);
        assert!(body.starts_with("\n\n-- \nK\n\n---------- Forwarded message ---------\n"));
        assert!(body.contains("From: Kay Mann <kay@enron.com>\nDate: Tue\n"));
        assert!(body.ends_with("\nHello.\n\nSee you.\n"));
    }

    #[test]
    fn prefixes_once() {
        assert_eq!(prefixed("Re:", "RE: x"), "RE: x");
        assert_eq!(prefixed("Re:", "x"), "Re: x");
        assert_eq!(prefixed("Fwd:", ""), "Fwd:");
    }
}
