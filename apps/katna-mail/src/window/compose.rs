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
//! Closing a message saves it as a draft, and Discard throws it away with
//! Undo (`drafts`); a draft opened from Drafts is written on here.
//!
//! `tools` draws the bars and their menus, `attach` handles files and
//! pictures, `schedule` the times of schedule send, `popout` the message
//! in a window of its own.

pub(super) mod attach;
mod chat_box;
mod checks;
mod chips;
mod drafts;
mod drive;
mod follow_up;
mod outbox;
mod paste;
mod popout;
mod quote;
mod recipients;
pub(in crate::window) mod rephrase;
mod reply_kind;
pub(super) mod schedule;
mod scheduled;
mod security;
mod sent;
mod signature_editor;
mod templates;
mod tools;
mod tracking;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, Decorations, DragMoveEvent, Entity, ExternalPaths, FocusHandle, Focusable,
    FontWeight, Hsla, ScrollHandle, SharedString, Subscription, Task, Window, canvas, div,
    prelude::*, rgba,
};
use katna_core::AccountId;
use katna_core::config::SEND_FROM_CURRENT;
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

use super::{MailWindow, RephraseSelection, SNACKBAR_TIME, SendMail};
use crate::daemon::{self, Command};
use crate::data::EntryKey;
use crate::format;
use crate::grammar;
use crate::outgoing::{self, Mailbox, Outgoing, Part};
use crate::signatures;
use crate::spell::{self, Speller};
use crate::suggest::{Phrases, Suggester};
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, icon, menu, menu_item, tip};

pub(super) use attach::Attachment;
use checks::Passed;
use chips::Chips;
pub(super) use outbox::NAV_KEY as OUTBOX_NAV_KEY;
pub(super) use outbox::reason_text as outbox_reason;
pub(super) use quote::{signature_name, signature_tag};
pub(in crate::window) use recipients::address_suggestions;
use recipients::{Field, Suggestions};
pub(super) use scheduled::NAV_KEY as SCHEDULED_NAV_KEY;
use security::Sealing;
pub(super) use sent::{Sending, SentCard};
pub(super) use signature_editor::signature_content;
use tools::Popup;
pub(super) use tools::below_end_over;

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
/// Below this width an inline reply's lock, signature and tracking
/// buttons leave the From row for one of their own.
const NARROW_REPLY: f32 = 560.0;
/// How strong the lines between the recipient rows are: fainter than
/// other dividers.
const FAINT_LINE: f32 = 0.55;
/// The width of the To, Cc, Bcc and From labels, so the fields line up.
const LABEL_WIDTH: f32 = 36.0;

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
    /// Files too large for mail, going through Google Drive.
    drive: Vec<drive::DriveFile>,
    /// Send was pressed while files were still going up to Drive: it
    /// goes once they are there.
    send_when_uploaded: bool,
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
    /// The `Message-ID` its drafts carry, the same at every save so each
    /// replaces the one before.
    message_id: String,
    /// The account whose Drafts folder has it saved, if any.
    saved: Option<AccountId>,
    /// What the last save while writing held, so an unchanged message is
    /// not saved again.
    autosaved: Option<(Draft, usize)>,
    /// Saving while writing, for the title bar.
    draft_status: DraftStatus,
    show_cc: bool,
    show_bcc: bool,
    /// The recipients of To, Cc and Bcc; their fields hold only what is
    /// being typed after them.
    chips: Chips,
    /// Addresses suggested for the recipient being typed.
    suggest: Option<Suggestions>,
    mode: Mode,
    /// Sign and encrypt.
    sealing: Sealing,
    /// The signature in the body, a [`katna_core::config::Signature::id`].
    signature: Option<u32>,
    /// The formatting bar (Aa) shows.
    format_bar: bool,
    /// The chat reply box's formatting bar sliding in (1) or out (0).
    format_slide: Spring,
    /// The chat formatting bar's height as last drawn: a narrow pane wraps
    /// its tools onto more lines.
    format_height: std::rc::Rc<std::cell::Cell<f32>>,
    /// The open menu or dialog, if any.
    popup: Option<Popup>,
    /// What happens if nobody replies: a reminder, or a follow-up.
    follow_up: follow_up::FollowUp,
    /// Fields of the link and schedule dialogs and the emoji search.
    dialog: tools::Dialog,
    shown: Spring,
    /// The width of the room a sheet was last laid out in, 0 until then:
    /// the window's frame may take more than its own reckoning says.
    sheet_width: Rc<Cell<f32>>,
    closing: bool,
    body_scroll: ScrollHandle,
    /// The quoted message a reply answers, kept out of the text behind a
    /// "..." button until it is opened, as in Gmail. It is still sent.
    quote: quote::Quote,
    /// Where the shown quote and the text were drawn, and the quote
    /// opening or closing.
    quote_view: Rc<Cell<quote::QuoteView>>,
    quote_glide: Option<quote::Glide>,
    /// The text of the conversation a reply answers, for writing
    /// suggestions.
    answered: String,
    /// Where an inline reply was last drawn, to keep its Send row at the
    /// bottom of the conversation while the rest scrolls under it.
    stick: Rc<Cell<Stick>>,
    /// The underline of grammar mistakes in the subject.
    grammar_color: Hsla,
    /// Pictures just pasted or dropped, while the choice between the text
    /// and the attachments shows.
    picture_choice: Option<paste::PictureChoice>,
    /// A template was put in, so its fields are filled again on Send.
    from_template: bool,
    /// The attachment list, which scrolls when it holds many files.
    attach_scroll: ScrollHandle,
    /// The message a reply or forward answers, to write it again as
    /// another kind.
    source: Option<MessageId>,
    /// The files a forward brought along from its message.
    forwarded: Vec<Arc<Vec<u8>>>,
    /// An inline reply shows its From, To, Cc and Bcc rows rather than
    /// one line naming the recipients.
    header_open: bool,
    /// The recipient field the cursor is in; the others fold up when they
    /// hold many.
    active_field: Option<Field>,
    /// Where the chips of each field were drawn, and each field's lines
    /// after the first, which scroll.
    chip_layout: Rc<chips::ChipLayout>,
    chip_scroll: [ScrollHandle; 3],
    /// Those rows growing in or shrinking away.
    rows_glide: reply_kind::RowsGlide,
    /// The Rephrase card, while it is open.
    rephrase: Option<rephrase::Rephrase>,
    /// How deep the text's undo went with the rephrased text put in, for
    /// the snackbar's Undo.
    rephrased: Option<usize>,
    /// Other wordings of the subject, while their card is open.
    subject_ideas: Option<rephrase::subject::SubjectIdeas>,
    /// When a press outside last put them away, so that pressing the
    /// sparkle closes them rather than asking again.
    subject_ideas_closed: Option<std::time::Instant>,
    /// The user agreed to send text of this encrypted message for
    /// rephrasing.
    ai_encrypted_ok: bool,
    /// Written in the chat view's reply box.
    chat: Option<chat_box::ChatReply>,
    _subscriptions: Vec<Subscription>,
}

impl Compose {
    /// What the fields hold now.
    fn fields(&self, cx: &gpui::App) -> Draft {
        let text = |input: &Entity<TextInput>| input.read(cx).text().to_owned();
        let recipients = |field, input| self.chips.text(field, &text(input));
        Draft {
            to: recipients(Field::To, &self.to),
            cc: recipients(Field::Cc, &self.cc),
            bcc: recipients(Field::Bcc, &self.bcc),
            subject: text(&self.subject),
            body: {
                let mut body = self.body.read(cx).doc().clone();
                // The chat's reply box keeps the signature out of sight.
                if let Some(chat) = &self.chat {
                    body.blocks.extend(chat.held.iter().cloned());
                }
                body.blocks.extend(self.quote.hidden().iter().cloned());
                body
            },
        }
    }

    /// Something was written that closing would lose.
    fn touched(&self, cx: &gpui::App) -> bool {
        self.files_changed() || !self.drive.is_empty() || self.fields(cx) != self.start
    }

    /// A message holding nothing worth keeping as a draft: no attachment,
    /// the people and subject as they came, and no text beyond spaces and
    /// invisible marks outside the signature and quote.
    fn wrote_nothing(&self, cx: &gpui::App) -> bool {
        if !self.attachments.is_empty() || !self.drive.is_empty() {
            return false;
        }
        let now = self.fields(cx);
        let start = &self.start;
        if (&now.to, &now.cc, &now.bcc, &now.subject)
            != (&start.to, &start.cc, &start.bcc, &start.subject)
        {
            return false;
        }
        let others = |doc: &Doc| {
            doc.blocks
                .iter()
                .filter(|b| !matches!(b, Block::Para(_)))
                .count()
        };
        let doc = self.body.read(cx).doc();
        others(doc) <= others(&start.body)
            && doc.blocks.iter().all(|block| match block {
                Block::Para(para) => {
                    para.style.signature || para.style.quote > 0 || !visible(&para.text)
                }
                _ => true,
            })
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
    /// How far the Send row was drawn above its place, to see after
    /// layout whether that is still right.
    stuck: f32,
    /// Device pixels per design pixel, which GPUI places elements on.
    device: f32,
}

impl Stick {
    /// How far the Send row goes up from its place so it stays at the
    /// bottom of the conversation `scroll`, but no higher than a little
    /// below the top of the card.
    fn stuck(&self, scroll: &ScrollHandle) -> f32 {
        // A scroll past either end is only put back when the conversation
        // is laid out, after the window drew; the offset it ends up with
        // is what counts.
        let max = unpx(scroll.max_offset().y).max(0.0);
        let offset = self.snap(unpx(scroll.offset().y).clamp(-max, 0.0));
        let bottom = unpx(scroll.bounds().size.height) - offset;
        let highest = self.card_top + STICK_BELOW;
        (self.footer_top + self.footer_height - bottom)
            .clamp(0.0, (self.footer_top - highest).max(0.0))
    }

    /// A scroll offset where GPUI draws it: on a whole device pixel. A
    /// touchpad scrolls by fractions of a pixel, and the Send row placed
    /// from the unrounded offset would land a pixel up or down each time.
    fn snap(&self, offset: f32) -> f32 {
        if self.device <= 0.0 {
            return offset;
        }
        let dev = offset * self.device;
        (dev.abs() - 0.5).ceil().copysign(dev) / self.device
    }
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
    /// Mail that has not gone out: waiting for a connection or a sign-in,
    /// or refused.
    outbox: Vec<OutboxItem>,
    /// The outbox's list shows.
    outbox_open: bool,
    watch: Option<Task<()>>,
    /// The signature being edited on the Settings page, and its bar.
    signature_editor: Option<Entity<RichEditor>>,
    signature_tools: Option<signature_editor::SignatureTools>,
    /// The window of a popped-out message.
    compose_window: Option<popout::Handle>,
    /// That window has the desktop's title bar rather than Katna's.
    popout_server_frame: bool,
    /// The saved templates, as last read.
    templates: Vec<katna_store::TemplateSummary>,
    /// The message just discarded, or closed without being saved, for
    /// Undo to open again.
    closed_draft: Option<Unsent>,
    /// How long each account's mail server holds scheduled mail, in
    /// seconds (0: it cannot), once asked.
    hold_limits: std::collections::HashMap<AccountId, u64>,
    /// Whether each account's mail server sends delivery receipts, once
    /// asked.
    delivery_receipts: std::collections::HashMap<AccountId, bool>,
    /// When a file manager last opened a new message with files, which
    /// the files that follow straight after join.
    files_opened: Option<std::time::Instant>,
    /// A reply put aside while a new message is written; it comes back
    /// when that message is sent or closed.
    pub(super) parked: Option<Compose>,
    /// The timer that saves the open message as a draft while it is
    /// written is running.
    autosaving: bool,
    /// The `Message-ID`s of drafts being saved while written.
    saving: Vec<String>,
    /// Drafts sent or discarded while a save was on its way: deleted once
    /// that save lands.
    drop_when_saved: Vec<String>,
}

/// Where saving the open message while it is written stands.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum DraftStatus {
    #[default]
    Unsaved,
    Saving,
    Saved,
    /// Its account could not save it (no Drafts folder, say): it is not
    /// tried again while written; closing it still tries, and says why.
    Failed,
}

impl Writing {
    /// How many messages wait for their scheduled time.
    pub(super) fn scheduled_count(&self) -> usize {
        self.scheduled.len()
    }

    /// How many messages have not gone out.
    pub(super) fn outbox_count(&self) -> usize {
        self.outbox.len()
    }

    /// Whether one of them waits for the user: refused, or waiting for a
    /// sign-in. Its count is amber then.
    pub(super) fn outbox_needs_you(&self) -> bool {
        self.outbox
            .iter()
            .any(|i| outbox::why(i).is_some_and(|w| w.needs_you()))
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

/// Whether `text` shows anything: spaces, zero-width marks and soft
/// hyphens do not.
fn visible(text: &str) -> bool {
    text.chars().any(|c| {
        !c.is_whitespace()
            && !matches!(
                c,
                '\u{200B}'..='\u{200D}' | '\u{2060}' | '\u{FEFF}' | '\u{00AD}'
            )
    })
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

/// The HTML of `body` and the pictures it shows, by `cid:` names under
/// `domain`; nothing for plain text.
fn body_parts(body: &Doc, plain: bool, domain: &str) -> (Option<String>, Vec<Part>) {
    if plain {
        return (None, Vec::new());
    }
    // Each picture gets a name of its own in the message, even when the
    // same one was pasted twice.
    let mut doc = body.clone();
    let mut next = 0;
    for block in &mut doc.blocks {
        let images = match block {
            Block::Image(image) => std::slice::from_mut(image),
            Block::Html(designed) => designed.images.as_mut_slice(),
            _ => &mut [],
        };
        for image in images {
            image.id = next;
            next += 1;
        }
    }
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
}

/// A message handed to the outbox, kept so that Undo can reopen it.
pub(super) struct Unsent {
    draft: Draft,
    thread: Threading,
    sealing: Sealing,
    signature: Option<u32>,
    attachments: Vec<Attachment>,
    drive: Vec<drive::DriveFile>,
    plain: bool,
    from: Option<AccountId>,
    answering: Option<EntryKey>,
    /// Puts back the conversation Send and archive archived.
    pub(super) unarchive: Option<Command>,
    /// The draft's `Message-ID`, and the account it is saved in; for a
    /// draft reopened from Drafts or after Discard.
    message_id: Option<String>,
    saved: Option<AccountId>,
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

/// `at` in Unix seconds, or now.
fn at_or_now(at: Option<jiff::Timestamp>) -> i64 {
    at.unwrap_or_else(jiff::Timestamp::now).as_second()
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

/// A message already being written when another is started.
#[derive(Debug, Clone, Copy)]
struct Existing {
    mode: Mode,
    closing: bool,
    touched: bool,
    /// It answers the conversation open now.
    here: bool,
    /// Its window of its own is still there.
    window_open: bool,
    /// It is a reply or forward, and the message started is a new one.
    answer_then_new: bool,
}

/// What starting another message does with the one being written.
#[derive(Debug, Clone, Copy, PartialEq)]
enum OnNew {
    /// Its reply is in the open conversation: the cursor goes back to it.
    Focus,
    /// It is in its own window: that comes forward.
    RaiseWindow,
    /// It shows elsewhere in the mail window, and stays.
    ShowElsewhere,
    /// Its window is gone without closing it: it is saved as a draft and
    /// the new message starts.
    SaveAndReplace,
    /// Nothing was written in it: the new message takes its place.
    Replace,
    /// A reply being written when Compose is pressed: it is put aside,
    /// as written, until the new message is sent or closed.
    Park,
}

impl Existing {
    fn on_new_message(self) -> OnNew {
        if self.closing || !self.touched {
            OnNew::Replace
        } else if self.answer_then_new && self.mode != Mode::Window {
            OnNew::Park
        } else if self.mode == Mode::Inline && self.here {
            OnNew::Focus
        } else if self.mode == Mode::Window {
            if self.window_open {
                OnNew::RaiseWindow
            } else {
                OnNew::SaveAndReplace
            }
        } else {
            OnNew::ShowElsewhere
        }
    }
}

impl MailWindow {
    /// Starts a reply to message `id` of the open conversation with `file`
    /// attached (a marked copy from the viewer). A reply already being
    /// written to the conversation gets the file instead.
    pub(super) fn reply_with_file(
        &mut self,
        id: MessageId,
        file: &katna_render::AttachmentFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_compose(Kind::Reply, Some(id), window, cx);
        let open = self.reader.as_ref().map(|r| r.key);
        let Some(compose) = &mut self.compose else {
            return;
        };
        let answers = compose.kind != Kind::New
            && (compose.source == Some(id) || open.is_some() && compose.answering == open);
        if !answers {
            return;
        }
        if compose.used_bytes(cx) + file.bytes.len() > attach::MAX_TOTAL {
            let problem = tr!(
                "compose-file-too-large",
                name = file.name.clone(),
                limit = attach::limit_text()
            );
            self.show_snackbar(problem, None, cx);
            return;
        }
        compose.attachments.push(Attachment {
            name: file.name.clone(),
            mime: file.mime.clone(),
            data: Arc::new(file.bytes.clone()),
        });
        compose.attach_scroll.scroll_to_bottom();
        cx.notify();
    }

    /// Starts a new mail with `file` attached: "Forward the file" on the
    /// Files page. A new mail already being written gets the file instead.
    pub(super) fn new_mail_with_file(
        &mut self,
        file: &katna_render::AttachmentFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_compose(Kind::New, None, window, cx);
        let Some(compose) = &mut self.compose else {
            return;
        };
        if compose.kind != Kind::New {
            return;
        }
        if compose.used_bytes(cx) + file.bytes.len() > attach::MAX_TOTAL {
            let problem = tr!(
                "compose-file-too-large",
                name = file.name.clone(),
                limit = attach::limit_text()
            );
            self.show_snackbar(problem, None, cx);
            return;
        }
        compose.attachments.push(Attachment {
            name: file.name.clone(),
            mime: file.mime.clone(),
            data: Arc::new(file.bytes.clone()),
        });
        compose.attach_scroll.scroll_to_bottom();
        cx.notify();
    }

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
        let window_open = self
            .writing
            .compose_window
            .is_some_and(|handle| handle.update(cx, |_, _, _| ()).is_ok());
        let existing = self.compose.as_ref().map(|c| Existing {
            mode: c.mode,
            closing: c.closing,
            touched: c.touched(cx),
            here: c.conversation == open,
            window_open,
            answer_then_new: c.kind != Kind::New && kind == Kind::New,
        });
        let outcome = existing.map(Existing::on_new_message);
        // Answering the conversation whose reply was put aside: that reply
        // comes back as it was written.
        if kind != Kind::New
            && matches!(outcome, Some(OnNew::Replace) | None)
            && open.is_some()
            && self
                .writing
                .parked
                .as_ref()
                .is_some_and(|p| p.conversation == open)
        {
            self.compose = self.writing.parked.take();
            if let Some(c) = &self.compose {
                window.focus(&c.body.focus_handle(cx), cx);
            }
            if self
                .compose
                .as_ref()
                .is_some_and(|c| c.mode == Mode::Inline)
            {
                self.reveal_inline_reply(cx);
            }
            cx.notify();
            return;
        }
        match outcome {
            Some(OnNew::Focus) => {
                if let Some(c) = &self.compose {
                    window.focus(&c.body.focus_handle(cx), cx);
                }
                cx.notify();
                return;
            }
            Some(OnNew::RaiseWindow) => {
                self.pop_out_compose(window, cx);
                cx.notify();
                return;
            }
            Some(OnNew::ShowElsewhere) => {
                if let Some(c) = &mut self.compose
                    && c.mode == Mode::Minimized
                {
                    c.mode = Mode::Open;
                }
                self.show_snackbar(tr!("compose-open-elsewhere"), None, cx);
                cx.notify();
                return;
            }
            Some(OnNew::SaveAndReplace) => self.close_compose_saving(cx),
            Some(OnNew::Park) => {
                if let Some(mut reply) = self.compose.take() {
                    reply.popup = None;
                    self.writing.parked = Some(reply);
                }
            }
            Some(OnNew::Replace) | None => {}
        }
        let date = |d: Option<i64>| {
            d.and_then(|d| format::local(d, &self.tz))
                .map(format::long_date)
                .unwrap_or_default()
        };
        let view = self.reader.as_ref().and_then(|reader| reader.view(source));
        let sealing = match kind {
            Kind::New => Sealing::new_message(),
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
        let source = self.reader.as_ref().and_then(|r| r.view_id(source));
        self.show_compose(draft, start, thread, signature, reply, window, cx);
        if let Some(compose) = &mut self.compose {
            compose.kind = kind;
            compose.source = source;
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
                compose.quote = quote::Quote::hidden_from(trim_quote(&mut doc));
                if compose.quote.is_hidden() {
                    compose.body.update(cx, |editor, cx| {
                        editor.set_doc(doc.clone(), doc.start(), cx)
                    });
                }
            }
        }
        if kind == Kind::Forward {
            self.attach_forwarded(cx);
        }
        self.adopt_kept_reply(cx);
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
        compose.chips.set(Field::To, &mail.to.join(", "));
        compose.chips.set(Field::Cc, &mail.cc.join(", "));
        compose.chips.set(Field::Bcc, &mail.bcc.join(", "));
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
        self.chips_changed(Field::To, cx);
    }

    /// Opens a new message with `subject` and `blocks` as its text, above
    /// the signature, the cursor in To: a note sent as mail.
    pub(in crate::window) fn open_compose_with(
        &mut self,
        subject: String,
        blocks: Vec<katna_ui::rich::Block>,
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
        compose
            .subject
            .update(cx, |input, cx| input.set_text(subject, cx));
        let mut doc = compose.body.read(cx).doc().clone();
        doc.blocks.splice(0..1.min(doc.blocks.len()), blocks);
        compose.body.update(cx, |editor, cx| {
            editor.set_doc(doc.clone(), doc.start(), cx)
        });
        let focus = compose.to.focus_handle(cx);
        window.focus(&focus, cx);
    }

    /// Sends the open new message from `account`, as picking it in From.
    pub(super) fn send_compose_from(&mut self, account: AccountId) {
        if let Some(compose) = &mut self.compose
            && self.accounts.iter().any(|a| a.id == account)
        {
            compose.from = Some(account);
        }
    }

    /// Scrolls the conversation smoothly to the reply that just opened at
    /// its end, as Gmail does: to the end when the whole card fits, else
    /// just far enough that its first line, with the cursor, sits near the
    /// top.
    pub(super) fn reveal_inline_reply(&mut self, cx: &mut Context<Self>) {
        self.scroll_to_inline_reply(false, cx);
    }

    /// Scrolls back up to the reply after it got shorter, when the cursor
    /// in it went out of sight above; never down.
    pub(super) fn scroll_back_to_inline_reply(&mut self, cx: &mut Context<Self>) {
        self.scroll_to_inline_reply(true, cx);
    }

    fn scroll_to_inline_reply(&mut self, back: bool, cx: &mut Context<Self>) {
        // The chat's reply box is always in sight, under the feed.
        if self.chat_shown() {
            return;
        }
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
                let to = if back {
                    limit.map_or(from, |limit| limit.max(from).min(px(0.0)))
                } else {
                    limit.map_or(end, |limit| end.max(limit.min(from)))
                };
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
    pub(in crate::window) fn signature_for(&self, kind: Kind) -> Option<u32> {
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
    pub(in crate::window) fn choose_signature(&mut self, id: Option<u32>, cx: &mut Context<Self>) {
        let new = self.config.sending.signature(id).map(signatures::doc);
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.popup = None;
        if compose.signature == id {
            cx.notify();
            return;
        }
        if let Some(chat) = &mut compose.chat {
            chat.held = chat_box::held_signature(new);
            compose.signature = id;
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
        // The recipients open as chips, and the fields empty.
        let chips = Chips::new(&draft.to, &draft.cc, &draft.bcc);
        let mut start = start;
        for text in [&mut start.to, &mut start.cc, &mut start.bcc] {
            *text = chips::normalized(text);
        }
        let placeholder = if chips.get(Field::To).is_empty() {
            tr!("compose-recipients")
        } else {
            String::new()
        };
        let to = input(&placeholder, "", cx);
        let cc = input("", "", cx);
        let bcc = input("", "", cx);
        let subject = input(&tr!("compose-subject"), &draft.subject, cx);
        let speller = self.speller(cx);
        let grammar = self.grammar();
        let answered = thread_text(&draft.body);
        let suggest = self.suggestions(&answered, cx);
        let complete = self.ai_complete(cx);
        subject.update(cx, |input, cx| {
            input.set_grammar_check(grammar.clone(), grammar_color(&th), cx)
        });
        let body = cx.new(|cx| {
            let mut editor = RichEditor::new("", cx);
            editor.set_palette(palette(&th));
            editor.set_html_view(super::rich::html_view(th));
            editor.set_doc(draft.body.clone(), draft.body.start(), cx);
            editor.set_spell_check(speller, cx);
            editor.set_grammar_check(grammar, cx);
            editor.set_suggest(suggest, cx);
            editor.set_complete(complete, cx);
            paste::setup(&mut editor);
            editor
        });
        let mut subscriptions = Vec::new();
        // Enter in a field moves on to the next one, once the recipient
        // typed there is a chip; typing a recipient suggests addresses, and
        // leaving the field makes a chip of it.
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
                    (InputEvent::Submit, Some(field)) => {
                        if !this.commit_recipients(field, true, cx) {
                            window.focus(&next, cx);
                        }
                    }
                    (InputEvent::Submit, None) => window.focus(&next, cx),
                    (InputEvent::Changed, Some(field)) => this.recipient_changed(field, cx),
                    (InputEvent::Changed, _) => {
                        // Typing closes a card of fixes.
                        this.close_hint(cx);
                        cx.notify()
                    }
                    (InputEvent::Cancel, _) => this.escape_compose(cx),
                },
            ));
            if let Some(field) = field {
                let focus = input.focus_handle(cx);
                // Focus moves while a frame is drawn: the fold changes on
                // the next one.
                subscriptions.push(cx.on_focus(&focus, window, move |this, _, cx| {
                    if let Some(c) = &mut this.compose {
                        c.active_field = Some(field);
                        // Typing goes on at the end of the list.
                        c.chip_scroll[field.ix()].scroll_to_bottom();
                    }
                    chips::notify_soon(cx);
                }));
                subscriptions.push(cx.on_blur(&focus, window, move |this, _, cx| {
                    if let Some(c) = &mut this.compose
                        && c.active_field == Some(field)
                    {
                        c.active_field = None;
                    }
                    // A click on a suggestion picks it instead.
                    if !this.suggesting(field) {
                        this.commit_recipients(field, true, cx);
                    }
                    chips::notify_soon(cx);
                }));
            }
        }
        // Typing in the subject or Escape puts its ideas away.
        subscriptions.push(cx.subscribe(&subject, |this, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Changed | InputEvent::Cancel) {
                this.close_subject_ideas(cx);
            }
        }));
        subscriptions.push(
            cx.subscribe(&subject, |this, _, event: &InputGrammarMenu, cx| {
                let popup = Popup::SubjectGrammar {
                    position: event.position,
                    issue: event.issue.clone(),
                    word: event.word,
                };
                if event.word.is_some() {
                    this.show_hint(popup, cx);
                } else if let Some(c) = &mut this.compose {
                    c.popup = Some(popup);
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
                    // The selection the card was for is gone.
                    if let Some(c) = &mut this.compose {
                        c.rephrase = None;
                    }
                    this.quote_changed(cx);
                    this.close_hint(cx);
                    this.keep_cursor_in_view(cx);
                    cx.notify();
                }
                RichEvent::Selection => {
                    // The chat's short box follows the arrow keys too.
                    if this.chat_shown() {
                        this.keep_cursor_in_view(cx);
                    }
                    cx.notify();
                }
                RichEvent::Cancel => this.escape_compose(cx),
                RichEvent::EditLink => this.open_link_dialog(window, cx),
                RichEvent::ContextMenu {
                    position,
                    misspelled,
                    grammar,
                } => this.open_compose_menu(*position, misspelled.clone(), grammar.clone(), cx),
                RichEvent::PasteFiles(paths) => this.paste_files(paths.clone(), cx),
                RichEvent::PastePictures(pictures) => {
                    this.place_pictures(pictures.clone(), true, cx)
                }
                RichEvent::PasteChoice(option) => this.choose_picture_place(*option, cx),
                RichEvent::Hint {
                    word,
                    at,
                    misspelled,
                    suggestions,
                    grammar,
                } => this.show_hint(
                    Popup::Hint {
                        word: *word,
                        at: *at,
                        misspelled: misspelled.clone(),
                        suggestions: suggestions.clone(),
                        grammar: grammar.clone(),
                    },
                    cx,
                ),
                // Compose turns neither on.
                RichEvent::OpenLink(_) | RichEvent::Pick(_) => {}
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
        let follow_up = follow_up::FollowUp::new(accent, cx);
        subscriptions.push(cx.subscribe_in(
            &follow_up.text,
            window,
            |this, _, event: &InputEvent, window, cx| {
                if let InputEvent::Cancel = event {
                    this.cancel_follow_up(window, cx);
                }
            },
        ));
        // Whether Track can be used: the Katna account, read again.
        self.katna_load(window, cx);
        self.compose = Some(Compose {
            to,
            show_cc: !draft.cc.is_empty(),
            cc,
            show_bcc: !draft.bcc.is_empty(),
            bcc,
            chips,
            suggest: None,
            subject,
            body,
            attachments: Vec::new(),
            drive: Vec::new(),
            send_when_uploaded: false,
            start,
            thread,
            kind: Kind::New,
            conversation: None,
            answering: None,
            from: None,
            message_id: drafts::new_message_id(),
            saved: None,
            autosaved: None,
            draft_status: DraftStatus::Unsaved,
            mode: Mode::Open,
            sealing: Sealing::new_message(),
            signature,
            format_bar: false,
            format_slide: Spring::new(motion::SMOOTH, 0.0),
            format_height: Default::default(),
            popup: None,
            follow_up,
            dialog,
            shown: Spring::new(motion::SLIDE, 0.0),
            sheet_width: Rc::default(),
            closing: false,
            body_scroll: ScrollHandle::new(),
            quote: quote::Quote::None,
            quote_view: Rc::default(),
            quote_glide: None,
            answered,
            stick: Rc::default(),
            grammar_color: grammar_color(&th),
            picture_choice: None,
            from_template: false,
            attach_scroll: ScrollHandle::new(),
            source: None,
            forwarded: Vec::new(),
            header_open: false,
            active_field: None,
            chip_layout: Rc::default(),
            chip_scroll: Default::default(),
            rows_glide: reply_kind::RowsGlide::default(),
            rephrase: None,
            rephrased: None,
            subject_ideas: None,
            subject_ideas_closed: None,
            ai_encrypted_ok: false,
            chat: None,
            _subscriptions: subscriptions,
        });
        self.ask_delivery_receipts(cx);
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
        let complete = self.ai_complete(cx);
        if let Some(compose) = &self.compose {
            compose.body.update(cx, |editor, cx| {
                editor.set_suggest(suggest, cx);
                editor.set_complete(complete, cx);
            });
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
        // under its Send row. The chat's reply box scrolls its own text
        // once it is as tall as it grows.
        let chat = self.chat_shown() && compose.mode == Mode::Inline;
        let (scroll, mut covered) = if compose.mode == Mode::Inline && !chat {
            (
                self.reader_scroll.clone(),
                compose.stick.get().footer_height,
            )
        } else {
            (compose.body_scroll.clone(), 0.0)
        };
        if compose.format_bar && !chat {
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
                let pad = px(if chat { 9.0 } else { 12.0 });
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
    /// Settings, or the first account when none is chosen; for a reply or
    /// forward the account the open conversation is in, whatever list it
    /// was opened from (the unified inbox or search mix accounts); else
    /// that of the open folder, or the first.
    fn compose_account(&self, kind: Kind) -> Option<&katna_core::Account> {
        let chosen = &self.config.sending.send_from;
        let fixed = (kind == Kind::New && chosen != SEND_FROM_CURRENT)
            .then(|| {
                self.accounts
                    .iter()
                    .find(|a| a.address.eq_ignore_ascii_case(chosen))
                    .or_else(|| self.accounts.first())
            })
            .flatten();
        let answered = self
            .reader
            .as_ref()
            .filter(|_| kind != Kind::New)
            .and_then(|reader| reader.entry())
            .and_then(|entry| self.mail.as_ref().ok()?.message_account(entry.latest));
        let open = answered.or_else(|| self.folder.and_then(|folder| self.tree.account_of(folder)));
        fixed.or_else(|| {
            open.or_else(|| self.shown_account())
                .and_then(|id| self.accounts.iter().find(|a| a.id == id))
                .or_else(|| self.accounts.first())
        })
    }

    /// Sends the open message the way Settings chooses: a reply or forward
    /// also archives its conversation when Send and archive is the default.
    /// A reply just opened, empty: the draft written in the summary card
    /// and not sent goes in, as one step Undo takes back.
    fn adopt_kept_reply(&mut self, cx: &mut Context<Self>) {
        let Some(c) = &self.compose else {
            return;
        };
        if !matches!(c.kind, Kind::Reply | Kind::ReplyAll) {
            return;
        }
        let Some(key) = c.answering else {
            return;
        };
        let body = c.body.clone();
        if body.read(cx).own_text_end().is_some() {
            return;
        }
        if let Some(text) = self.take_kept_reply(key) {
            body.update(cx, |editor, cx| editor.insert_at_start(&text, cx));
        }
    }

    pub(super) fn send_compose_default(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // A chat goes on: its conversation is never archived.
        let chat = self.compose.as_ref().is_some_and(|c| c.chat.is_some());
        let archive = self.config.sending.send_and_archive && !chat;
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
        let mut draft = compose.fields(cx);
        let thread = compose.thread.clone();
        let sealing = compose.sealing;
        let kind = compose.kind;
        let chosen = compose.from;
        let chat = compose.chat.is_some();
        let answered = compose.answering;
        let answering = answered.filter(|_| archive && at.is_none());
        // Sent, the draft it was saved as goes, and one being saved goes
        // once it lands.
        let saved = compose.saved.map(|a| (a.0, compose.message_id.clone()));
        let sending_id = compose.message_id.clone();
        let signature = compose.signature;
        let attachments = compose.attachments.clone();
        let drive_files = compose.drive.clone();
        let plain = compose.plain(cx);
        // When to follow up, and what to send then, if anything.
        let follow_up_on = compose.follow_up.on();
        let follow_up_text = (compose.follow_up.send && !sealing.encrypt)
            .then(|| compose.follow_up.text.read(cx).text().trim().to_owned())
            .filter(|text| !text.is_empty());
        let follow_up_again = i64::from(compose.follow_up.again);
        let follow_up_sent = at_or_now(at);
        let follow_up = if follow_up_on {
            compose.follow_up.after_sending(follow_up_sent)
        } else {
            0
        };
        // Tracking needs a Katna account with a confirmed address.
        let track = sealing.track && self.katna_signed_in();
        // Delivery receipts where the mail server sends them (the daemon
        // leaves them out where it does not).
        let delivery = sealing.delivery && self.delivery_receipts_offered() != Some(false);
        if let Some((field, address)) = self.bad_recipient(cx) {
            if let Some(c) = &mut self.compose {
                c.popup = Some(Popup::BadAddress { field, address });
            }
            cx.notify();
            return;
        }
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
        if self.fill_template_fields(to.first().or(cc.first()), cx)
            && let Some(c) = &self.compose
        {
            draft = c.fields(cx);
        }
        let total: usize = attachments.iter().map(|a| a.data.len()).sum::<usize>()
            + draft.body.images().map(|i| i.data.len()).sum::<usize>();
        if total > attach::MAX_TOTAL {
            self.show_snackbar(
                tr!(
                    "compose-attachments-too-large",
                    size = format::size(total as u64),
                    limit = attach::limit_text()
                ),
                None,
                cx,
            );
            return;
        }
        let attached = attachments.len() + drive_files.len() + draft.body.images().count();
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
        let everyone = to.iter().chain(&cc).chain(&bcc);
        let everyone: Vec<String> = everyone.map(|m| m.email.clone()).collect();
        if self.drive_before_send(at, archive, passed, everyone, window, cx) {
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
        let mut body_text = html::to_plain(&draft.body);
        let (mut html_body, inline) = body_parts(&draft.body, plain, &domain);
        // Files in Drive go as links at the end, as in Gmail.
        body_text.push_str(&drive::links_text(&drive_files));
        if let Some(html) = &mut html_body {
            html.push_str(&drive::links_html(&drive_files));
        }
        // Known here, to find the stored copy of a reply shown at once.
        let message_id = sent::new_message_id(&domain);
        let snippet: String = body_text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .take(200)
            .collect();
        let me = (
            account.display_name.trim().to_owned(),
            account.address.clone(),
        );
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
        let draft_subject = draft.subject.clone();
        let follow_up_mail = follow_up_text.map(|text| {
            let signature = self
                .config
                .sending
                .signature(signature)
                .map(|s| html::to_plain(&signatures::doc(s)));
            let said = format!(
                "On {}, {} wrote:",
                format::local(follow_up_sent, &self.tz)
                    .map(format::long_date)
                    .unwrap_or_default(),
                match &from.name {
                    Some(name) => format!("{name} <{}>", from.email),
                    None => from.email.clone(),
                },
            );
            follow_up::build(&follow_up::Mail {
                from: &from,
                to: &to,
                cc: &cc,
                bcc: &bcc,
                subject: &draft.subject,
                message_id: &message_id,
                references: &thread.references,
                text: &text,
                signature: signature.as_deref(),
                quote: (&said, &body_text),
            })
        });
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
            message_id: Some(message_id.clone()),
            calendar: None,
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
            drive: drive_files,
            plain,
            from,
            answering: answered,
            unarchive: None,
            message_id: None,
            saved: None,
        });
        self.close_compose(cx);
        self.drop_when_saved(&sending_id);
        // A reply takes the place of its card in the conversation at once,
        // as in Gmail.
        let card = match answered {
            Some(key) if at.is_none() => {
                let subject = draft_subject.clone();
                let card =
                    self.sending
                        .add_card(key, message_id, Arc::new(raw.clone()), chat, |id| {
                            sent::row(key, id, from, me, subject, snippet)
                        });
                if chat {
                    self.chat_countdown(card, delay, cx);
                }
                self.show_sent_cards(cx);
                Some(card)
            }
            _ => None,
        };
        // The chat shows the reply waiting beside its countdown.
        if !(chat && card.is_some()) {
            self.show_snackbar(
                if at.is_some() {
                    tr!("compose-scheduling")
                } else {
                    tr!("compose-sending")
                },
                None,
                cx,
            );
        }
        let connection = self.daemon.clone();
        let when = at.map(|at| schedule::describe(at, &self.tz));
        let undo = self.config.sending.undo_send_seconds;
        let at = at.map(|at| at.as_second());
        cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    // Signed and encrypted before the outbox sees it.
                    let raw = security::seal(raw, sealing, sender.clone(), visible, hidden)?;
                    let raw = if sealing.receipt {
                        tracking::with_receipt(raw, &sender)
                    } else {
                        raw
                    };
                    let raw = if delivery {
                        tracking::with_delivery_receipt(raw)
                    } else {
                        raw
                    };
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    let id = match at {
                        // Undo works for the undo delay; then it may go
                        // to the mail server to hold. Scheduled mail is
                        // not tracked.
                        Some(at) => {
                            daemon::schedule_send(&connection, account, &raw, undo, at).await?
                        }
                        None if track => {
                            daemon::queue_tracked_send(&connection, account, &raw, delay).await?
                        }
                        None => daemon::queue_send(&connection, account, &raw, delay).await?,
                    };
                    if follow_up > 0 {
                        let set = match &follow_up_mail {
                            Some(mail) => {
                                daemon::set_follow_up_mail(
                                    &connection,
                                    id,
                                    follow_up,
                                    follow_up_again,
                                    mail,
                                )
                                .await
                            }
                            None => daemon::set_follow_up(&connection, id, follow_up).await,
                        };
                        if let Err(err) = set {
                            tracing::warn!(%err, "the follow-up was not set");
                        }
                    }
                    if let Some((account, message_id)) = saved
                        && let Err(err) =
                            daemon::discard_draft(&connection, account, &message_id).await
                    {
                        tracing::warn!(%err, "the sent message's draft stays");
                    }
                    Ok::<_, String>(id)
                })
                .await;
            this.update_in(cx, |this, window, cx| match result {
                Ok(id) => {
                    if let Some(card) = card {
                        this.sending.card_queued(card, id);
                    }
                    if let Some(key) = answering {
                        let unarchive = this.act_with(super::Act::Archive, vec![key], false, cx);
                        if let Some(unsent) = &mut this.unsent {
                            unsent.unarchive = unarchive;
                        }
                    }
                    match &when {
                        Some(when) => {
                            let undo = (delay > 0).then_some(Command::UndoSend(id));
                            this.show_snackbar_for(
                                tr!("compose-scheduled", when = when.clone()),
                                undo,
                                SNACKBAR_TIME * 2,
                                cx,
                            );
                            this.scheduled_changed(cx);
                        }
                        None => this.queued(id, delay, answering.is_some(), cx),
                    }
                }
                Err(err) => {
                    // Nothing went out: the message comes back as it was.
                    if let Some(card) = card {
                        this.sending.remove_card(card);
                        this.show_sent_cards(cx);
                    }
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
        let Some(unsent) = self.unsent.take() else {
            return;
        };
        if self
            .compose
            .as_ref()
            .is_some_and(|c| !c.closing && c.touched(cx))
        {
            return;
        }
        self.reopen_message(unsent, Draft::default(), window, cx);
    }

    /// Opens `unsent` in the compose window; `start` is what counts as
    /// untouched.
    fn reopen_message(
        &mut self,
        unsent: Unsent,
        start: Draft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Unsent {
            draft,
            thread,
            sealing,
            signature,
            attachments,
            drive,
            plain,
            from,
            answering,
            unarchive: _,
            message_id,
            saved,
        } = unsent;
        self.show_compose(draft, start, thread, signature, true, window, cx);
        if let Some(compose) = &mut self.compose {
            compose.attachments = attachments;
            compose.drive = drive;
            compose
                .body
                .update(cx, |editor, cx| editor.set_plain(plain, cx));
            compose.sealing = sealing;
            compose.from = from;
            compose.answering = answering;
            if let Some(message_id) = message_id {
                compose.message_id = message_id;
            }
            compose.saved = saved;
        }
    }

    /// Whether a menu or card is open over the message (not a hint).
    pub(super) fn compose_popup_open(&self) -> bool {
        self.compose
            .as_ref()
            .and_then(|c| c.popup.as_ref())
            .is_some_and(|p| !p.is_hint())
    }

    /// Esc in the message: first it closes a card or dialog over it; then
    /// the message itself closes, kept as a draft, as Esc does in Gmail,
    /// Outlook and Thunderbird. A reply written in the conversation stays.
    fn escape_compose(&mut self, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        if c.rephrase.is_some() {
            c.rephrase = None;
            cx.notify();
            return;
        }
        match &c.popup {
            Some(popup) if popup.is_hint() => self.close_hint(cx),
            Some(_) => {
                c.popup = None;
                cx.notify();
            }
            None if c.mode != Mode::Inline && !c.closing => self.close_compose_saving(cx),
            None => {
                self.drop_empty_reply(cx);
            }
        }
    }

    fn close_compose(&mut self, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            compose.closing = true;
            compose.popup = None;
            if compose.mode == Mode::Window {
                self.compose_gone(cx);
            }
        }
        self.close_compose_window(cx);
        cx.notify();
    }

    /// The message is gone: a reply put aside for it comes back.
    fn compose_gone(&mut self, cx: &mut Context<Self>) {
        self.compose = self.writing.parked.take();
        if self.compose.is_some() {
            chips::notify_soon(cx);
        }
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
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        // Harper's helper is large: it runs only while a message is open.
        if self.compose.is_none() && self.writing.grammar.is_some() {
            self.writing.grammar = None;
        }
        self.keep_saving(cx);
        let (mode, conversation, closing) = self
            .compose
            .as_ref()
            .map(|c| (c.mode, c.conversation, c.closing))?;
        if mode == Mode::Window {
            if closing {
                self.compose_gone(cx);
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
            // Left with nothing written: no draft, and no window.
            if !here && !closing && self.drop_empty_reply(cx) {
                self.compose_gone(cx);
                return None;
            }
            if closing || (!here && !touched) {
                self.compose_gone(cx);
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
            self.compose_gone(cx);
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let compose = self.compose.as_ref()?;
        let (vw, vh) = (self.room_width(), self.room_height(window));
        let mode = compose.mode;
        let title = compose.title(cx);
        let draft_status = match compose.draft_status {
            DraftStatus::Unsaved | DraftStatus::Failed => None,
            // Saved before, it keeps saying so rather than flickering.
            DraftStatus::Saving if compose.saved.is_some() => Some(tr!("compose-draft-saved")),
            DraftStatus::Saving => Some(tr!("compose-draft-saving")),
            DraftStatus::Saved => Some(tr!("compose-draft-saved")),
        };

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
                    .flex()
                    .flex_row()
                    .items_baseline()
                    .gap(px(8.0))
                    .child(
                        div()
                            .min_w_0()
                            .truncate()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(title),
                    )
                    // Saving while writing, in dim text beside the title.
                    .children(draft_status.map(|status| {
                        div()
                            .flex_none()
                            .text_size(px(12.5))
                            .text_color(rgba(th.text_faint))
                            .child(status)
                    })),
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
                        this.close_compose_saving(cx)
                    })),
            );

        // On a phone, and a tablet too narrow for the reading pane, the
        // message is written on a sheet over the whole window, top bar and
        // all, as mobile mail does; minimized, it is a strip at the foot.
        // With Katna's own frame the top bar holds the window buttons, so
        // the sheet stays below it.
        let shape = self.layout.shape;
        let sheet = !shape.size.splits(shape.width) && mode != Mode::Minimized;
        let under_bar = matches!(window.window_decorations(), Decorations::Client { .. });
        // The room as laid out last time, once there is one.
        let sheet_width = match compose.sheet_width.get() {
            w if w > 0.0 => w,
            _ => shape.width,
        };
        // The same gap as the cards keep from the window's edges, so the
        // window's corner lines up with the card's beside it.
        let gap = shape.card_margin().max(8.0 * shape.phone);
        let (width, height) = match mode {
            _ if sheet => (sheet_width, vh),
            Mode::Minimized if shape.is_phone() => (shape.width - 2.0 * gap, TITLE_HEIGHT),
            Mode::Open | Mode::Inline | Mode::Window => {
                (WIDTH.min(vw - 32.0), MAX_HEIGHT.min(vh - 96.0 - gap))
            }
            Mode::Minimized => (MINIMIZED_WIDTH, TITLE_HEIGHT),
            Mode::Full => ((vw - 128.0).clamp(WIDTH, 1000.0), vh - 96.0),
        };
        let panel = div()
            .id("compose")
            .key_context("Compose")
            .on_action(
                cx.listener(|this, _: &SendMail, window, cx| this.send_compose_default(window, cx)),
            )
            .on_action(cx.listener(|this, _: &RephraseSelection, window, cx| {
                this.toggle_rephrase(window, cx)
            }))
            .occlude()
            .relative()
            .map(|d| {
                if sheet {
                    d.size_full()
                } else {
                    d.w(px(width)).h(px(height))
                }
            })
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .map(|d| match mode {
                _ if sheet => d,
                _ => d.rounded(px(12.0)),
            })
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.drop_on_compose(paths, cx);
            }))
            .child(title_bar)
            .when(mode != Mode::Minimized, |d| {
                d.child(self.render_compose_fields(th, cx))
                    .child(self.render_compose_body(th, width, cx))
                    .child(self.render_attachments(th, cx))
                    .children(self.render_floating_format_bar(th, width - 24.0, cx))
                    .child(self.render_compose_actions(th, width, cx))
                    .child(self.render_drop_target(th))
            });

        Some(match mode {
            _ if sheet => {
                let measured = compose.sheet_width.clone();
                let this = cx.entity().downgrade();
                let top = if under_bar {
                    0.0
                } else {
                    -super::TOP_BAR_HEIGHT
                };
                div()
                    .absolute()
                    .top(px(lerp(48.0, 0.0, t) + top))
                    .bottom(px(-lerp(48.0, 0.0, t)))
                    .left_0()
                    .right_0()
                    .opacity(t)
                    .child(
                        canvas(
                            move |bounds, _, cx| {
                                let width = unpx(bounds.size.width);
                                if (width - measured.get()).abs() > 0.5 {
                                    measured.set(width);
                                    // After this frame: a change asked for
                                    // while drawing is lost.
                                    let this = this.clone();
                                    cx.defer(move |cx| {
                                        this.update(cx, |_, cx| cx.notify()).ok();
                                    });
                                }
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full(),
                    )
                    .child(panel)
                    .into_any_element()
            }
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
            // It floats clear of the window's edges: the same gap below
            // as to the right.
            _ => div()
                .absolute()
                .right(px(gap))
                .bottom(px(shape.bottom_bar() + gap + lerp(-48.0, 0.0, t)))
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
        let pop_out = small_button("inline-pop-out", "open-full", th)
            .tooltip(tip(tr!("compose-pop-out-reply"), th))
            // Straight into a window of its own; docking it brings it back
            // here.
            .on_click(cx.listener(|this, _, window, cx| this.pop_out_compose(window, cx)));
        let card_width = unpx(self.reader_scroll.bounds().size.width) - 100.0;
        // As in Gmail, one line names the recipients until it is clicked.
        let open = self.reply_header_open();
        // The security group goes beside From when there is room, else on
        // a row of its own under the recipients. No lines between these
        // rows; one under the last, above the text.
        let roomy = card_width >= NARROW_REPLY;
        let head_row = || {
            div()
                .flex_none()
                .min_h(px(44.0))
                .pl(px(8.0))
                .pr(px(6.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .text_size(px(14.0))
        };
        let header = if open {
            head_row()
                .child(self.render_kind_button(th, cx))
                .child(
                    div()
                        .flex_none()
                        .text_color(rgba(th.text_dim))
                        .child(tr!("compose-from")),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .children(self.render_from_row(th, cx)),
                )
                .when(roomy, |d| d.child(self.render_security_group(th, cx)))
                .child(pop_out)
        } else {
            head_row()
                .child(self.render_kind_button(th, cx))
                .child(self.render_reply_summary(th, cx))
                .child(pop_out)
        };
        let field_row = |label: String| {
            div()
                .flex_none()
                .mx(px(12.0))
                .min_h(px(40.0))
                .flex()
                .flex_row()
                // With chips on several lines, the label stays by the first.
                .items_start()
                .gap(px(8.0))
                .text_size(px(14.0))
                .child(
                    div()
                        .flex_none()
                        .min_w(px(28.0))
                        .h(px(40.0))
                        .flex()
                        .items_center()
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
        };
        // A chip being dragged can land in Cc or Bcc even while hidden.
        let dragging = self.chip_dragging(cx).is_some();
        // The rows grow in above the text, the card's bottom staying put.
        let rows_shown = self.rows_shown(open, cx);
        let rows =
            (open || rows_shown.is_some()).then(|| {
                let field = |field: AnyElement| {
                    div()
                        .flex_1()
                        .min_w_0()
                        .min_h(px(40.0))
                        .flex()
                        .items_center()
                        .child(field)
                };
                let to = field_row(tr!("compose-to")).child(field(self.render_recipient_field(
                    Field::To,
                    self.render_cc_bcc(th, cx),
                    th,
                    cx,
                )));
                let to = self.recipient_row(to, Field::To, th, cx);
                let cc = (compose.show_cc || dragging).then(|| {
                    let row = field_row(tr!("compose-cc"))
                        .child(field(self.render_recipient_field(Field::Cc, None, th, cx)));
                    self.recipient_row(row, Field::Cc, th, cx)
                });
                let bcc = (compose.show_bcc || dragging).then(|| {
                    let row = field_row(tr!("compose-bcc"))
                        .child(field(self.render_recipient_field(Field::Bcc, None, th, cx)));
                    self.recipient_row(row, Field::Bcc, th, cx)
                });
                let tools = (!roomy).then(|| {
                    div()
                        .flex_none()
                        .mx(px(12.0))
                        .h(px(40.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_end()
                        .child(self.render_security_group(th, cx))
                });
                let rows = div()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .child(to)
                    .children(cc)
                    .children(bcc)
                    .children(tools)
                    // A line under the last row too, before the text.
                    .child(
                        div()
                            .flex_none()
                            .mx(px(12.0))
                            .h(px(1.0))
                            .bg(rgba(th.faint_line(FAINT_LINE))),
                    );
                self.glide_rows(rows, rows_shown, cx)
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
            .on_click(cx.listener(move |this, _, window, cx| {
                window.focus(&focus, cx);
                this.close_reply_header(cx);
            }))
            .on_drag_move(
                cx.listener(|this, event: &DragMoveEvent<ExternalPaths>, _, cx| {
                    this.drag_over_body(event, cx);
                }),
            )
            .on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| {
                this.drop_on_body(paths, window, cx);
            }))
            .child(self.render_body_and_quote(th, cx))
            .children(self.render_trimmed(th, cx));
        // Like Gmail, the Send row stays at the bottom of the conversation
        // while the text runs on below it, and moves up with the card.
        let stuck = {
            let at = compose.stick.get();
            let stuck = at.stuck(&self.reader_scroll);
            compose.stick.set(Stick { stuck, ..at });
            stuck
        };
        let card = div()
            .id("inline-reply")
            .key_context("Compose")
            .on_action(
                cx.listener(|this, _: &SendMail, window, cx| this.send_compose_default(window, cx)),
            )
            .on_action(cx.listener(|this, _: &RephraseSelection, window, cx| {
                this.toggle_rephrase(window, cx)
            }))
            .relative()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .map(|d| crate::widgets::tile(d, th))
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.drop_on_compose(paths, cx);
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
            .children(rows)
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
                            // Held up at the bottom of the pane, the row has
                            // square corners and a strip of the card under
                            // it, so no quoted line shows below it when the
                            // pane edge and the row round to different
                            // pixels as the conversation scrolls.
                            .when(stuck <= 0.0, |d| d.rounded_b(px(12.0)))
                            .when(stuck > 0.0, |d| {
                                d.child(
                                    div()
                                        .absolute()
                                        .top_full()
                                        .left_0()
                                        .right_0()
                                        .h(px(STICK_SKIRT))
                                        .bg(rgba(th.surface)),
                                )
                            })
                            .bg(rgba(th.surface))
                            .border_t_1()
                            .border_color(if stuck > 0.0 {
                                rgba(th.divider)
                            } else {
                                rgba(0)
                            })
                            .children(self.render_floating_format_bar(th, card_width - 24.0, cx))
                            .child(self.render_compose_actions(th, card_width, cx)),
                    ),
            )
            .child(self.render_drop_target(th));
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

    fn render_compose_fields(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        // To, Cc, Bcc and From sit together without lines between them;
        // the only lines are above and below the subject.
        let row = |label: String, field: AnyElement| {
            div()
                .flex_none()
                .mx(px(16.0))
                .min_h(px(40.0))
                .flex()
                .flex_row()
                // With chips on several lines, the label stays by the first.
                .items_start()
                .gap(px(8.0))
                .text_size(px(14.0))
                .child(
                    div()
                        .flex_none()
                        .min_w(px(LABEL_WIDTH))
                        .h(px(40.0))
                        .flex()
                        .items_center()
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .min_h(px(40.0))
                        .flex()
                        .items_center()
                        .child(field),
                )
        };
        let from = self.render_from_row(th, cx);
        // The security group goes at the end of From; without a From row,
        // at the end of To.
        let to_tools = div()
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .children(self.render_cc_bcc(th, cx))
            .when(from.is_none(), |d| {
                d.child(self.render_security_group(th, cx))
            })
            .into_any_element();
        let to_field = self.render_recipient_field(Field::To, Some(to_tools), th, cx);
        let [cc_field, bcc_field] =
            [Field::Cc, Field::Bcc].map(|field| self.render_recipient_field(field, None, th, cx));
        // A chip being dragged can land in Cc or Bcc even while hidden.
        let dragging = self.chip_dragging(cx).is_some();
        let to = self.recipient_row(row(tr!("compose-to"), to_field), Field::To, th, cx);
        let line = rgba(th.faint_line(FAINT_LINE));
        div()
            .flex_none()
            .flex()
            .flex_col()
            .pt(px(4.0))
            .child(to)
            .when(compose.show_cc || dragging, |d| {
                d.child(self.recipient_row(row(tr!("compose-cc"), cc_field), Field::Cc, th, cx))
            })
            .when(compose.show_bcc || dragging, |d| {
                d.child(self.recipient_row(row(tr!("compose-bcc"), bcc_field), Field::Bcc, th, cx))
            })
            .children(from.map(|from| {
                row(
                    tr!("compose-from"),
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(div().flex_1().min_w_0().flex().child(from))
                        .child(self.render_security_group(th, cx))
                        .into_any_element(),
                )
            }))
            .child(
                div()
                    .flex_none()
                    .mx(px(16.0))
                    .mt(px(4.0))
                    .min_h(px(44.0))
                    .flex()
                    .items_center()
                    .border_t_1()
                    .border_b_1()
                    .border_color(line)
                    .text_size(px(15.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.render_subject_field(th, cx)),
            )
            .children(self.compose_offline_strip(th, cx))
            .into_any_element()
    }

    /// The account the message goes out from.
    pub(super) fn compose_from_id(&self) -> Option<AccountId> {
        let compose = self.compose.as_ref()?;
        compose
            .from
            .filter(|id| self.accounts.iter().any(|a| a.id == *id))
            .or_else(|| self.compose_account(compose.kind).map(|a| a.id))
    }

    /// The account the message goes out from, with the others to pick
    /// from under its arrow.
    fn render_from_row(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        let from = compose
            .from
            .and_then(|id| self.accounts.iter().find(|a| a.id == id))
            .or_else(|| self.compose_account(compose.kind))?;
        let open = compose.popup == Some(Popup::From);
        let several = self.accounts.len() > 1;
        let items = self.accounts.iter().enumerate().map(|(ix, account)| {
            let id = account.id;
            let chosen = account.id == from.id;
            menu_item(("compose-from-account", ix), &sender_label(account), th)
                .gap(px(12.0))
                .child(div().flex_1())
                .when(self.is_account_offline(id), |d| {
                    d.child(crate::widgets::tag(tr!("offline-tag"), th))
                })
                .when(chosen, |d| {
                    d.child(icon("check", th.nav_selected_text, 20.0))
                })
                .on_click(cx.listener(move |this, _, window, cx| {
                    if let Some(c) = &mut this.compose {
                        c.from = Some(id);
                        c.popup = None;
                        window.focus(&c.body.focus_handle(cx), cx);
                    }
                    this.ask_delivery_receipts(cx);
                    cx.notify();
                }))
        });
        // One quiet line: the name, then the address dimmed.
        let name = from.display_name.trim();
        let named = !name.is_empty() && !name.eq_ignore_ascii_case(&from.address);
        let button = div()
            .id("compose-from")
            .relative()
            .max_w_full()
            .h(px(30.0))
            .px(px(6.0))
            .ml(px(-6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .rounded(px(8.0))
            .when(open, |d| d.bg(rgba(th.hover)))
            .child(
                div()
                    .min_w_0()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .when(named, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .text_color(rgba(th.text))
                                .child(SharedString::from(name.to_owned())),
                        )
                    })
                    .child(
                        div()
                            .min_w_0()
                            .text_ellipsis()
                            .overflow_hidden()
                            .text_color(rgba(if named { th.text_dim } else { th.text }))
                            .child(SharedString::from(from.address.clone())),
                    ),
            )
            .when(self.is_account_offline(from.id), |d| {
                d.child(crate::widgets::tag(tr!("offline-tag"), th))
            })
            .when(several, |d| {
                d.cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .when(!open, |d| d.tooltip(tip(tr!("compose-from-choose"), th)))
                    .child(icon("chevron-down", th.text_dim, 16.0))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::From, cx)))
            })
            .when(open, |d| {
                d.child(tools::below(menu(th).min_w(px(280.0)).children(items)))
            });
        Some(
            div()
                .py(px(5.0))
                .flex_1()
                .flex()
                .min_w_0()
                .child(button)
                .into_any_element(),
        )
    }

    fn render_compose_body(&self, th: &Theme, width: f32, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let focus = compose.body.focus_handle(cx);
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
            .on_drag_move(
                cx.listener(|this, event: &DragMoveEvent<ExternalPaths>, _, cx| {
                    this.drag_over_body(event, cx);
                }),
            )
            .on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| {
                this.drop_on_body(paths, window, cx);
            }))
            .child(
                div()
                    .flex_none()
                    .w(px((width - 32.0).max(80.0)))
                    .child(self.render_body_and_quote(th, cx)),
            )
            .children(self.render_trimmed(th, cx))
            .into_any_element()
    }
}

/// How far below the top of an inline reply its stuck Send row stops: the
/// recipients and a line or two of text stay above it.
const STICK_BELOW: f32 = 96.0;
/// The card's color drawn under a Send row held at the bottom of the pane.
const STICK_SKIRT: f32 = 4.0;

/// Records where its parent is drawn in the conversation `scroll`, in
/// pixels from the top of the content, and draws again when that moved or
/// the Send row is no longer at the bottom of the pane.
fn measure(
    scroll: &ScrollHandle,
    stick: &Rc<Cell<Stick>>,
    cx: &Context<MailWindow>,
    set: impl Fn(&mut Stick, f32, f32) + 'static,
) -> impl IntoElement {
    let (scroll, stick, this) = (scroll.clone(), stick.clone(), cx.entity().downgrade());
    canvas(
        move |bounds, window, cx| {
            let mut at = stick.get();
            at.device = window.scale_factor() * katna_ui::scale::scale();
            // From the offset as drawn, so the place stays put while the
            // conversation scrolls.
            let top = unpx(bounds.top() - scroll.bounds().top()) - at.snap(unpx(scroll.offset().y));
            set(&mut at, top, unpx(bounds.size.height));
            // Also when the pane changed size or a scroll went past its
            // end since the Send row was placed.
            let placed = (at.stuck(&scroll) - at.stuck).abs() < 0.5;
            if at != stick.get() || !placed {
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
pub(in crate::window) fn palette(th: &Theme) -> Palette {
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
        inserted: color(if th.dark { 0x81c99533 } else { 0x1e8e3e24 }),
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
        .relative()
        .child(crate::widgets::hover_fade("hover-glow", None, th))
        .child(icon(name, th.text_dim, 18.0))
}

/// An account as its mail shows it: `Name <address>`, or the address.
fn sender_label(account: &katna_core::Account) -> String {
    let name = account.display_name.trim();
    if name.is_empty() || name.eq_ignore_ascii_case(&account.address) {
        account.address.clone()
    } else {
        format!("{name} <{}>", account.address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn existing(mode: Mode, window_open: bool, here: bool) -> Existing {
        Existing {
            mode,
            closing: false,
            touched: true,
            here,
            window_open,
            answer_then_new: false,
        }
    }

    #[test]
    fn designed_signature_pictures_are_parts() {
        let png = "data:image/png;base64,iVBORw0KGgo=";
        let mut next = 0;
        let designed = html::html_block(
            &format!(
                "<table><tr><td><img src=\"{png}\"></td><td><img src=\"{png}\"></td></tr></table>"
            ),
            &mut next,
        );
        let mut body = html::from_plain("Hi");
        body.blocks.push(Block::Html(designed));
        let (sent, inline) = body_parts(&body, false, "example.com");
        let sent = sent.expect("html");
        assert_eq!(inline.len(), 2);
        let ids: Vec<String> = inline.iter().filter_map(|p| p.content_id.clone()).collect();
        assert_ne!(ids[0], ids[1]);
        for id in ids {
            assert!(sent.contains(&format!("src=\"cid:{id}\"")), "{sent}");
        }
    }

    #[test]
    fn compose_puts_a_reply_aside() {
        // Compose while a reply is half written, inline here or docked:
        // the reply waits and the new message opens.
        for mode in [Mode::Inline, Mode::Open, Mode::Minimized] {
            let mut e = existing(mode, false, true);
            e.answer_then_new = true;
            assert_eq!(e.on_new_message(), OnNew::Park);
        }
        // In its own window it comes forward, as before.
        let mut e = existing(Mode::Window, true, false);
        e.answer_then_new = true;
        assert_eq!(e.on_new_message(), OnNew::RaiseWindow);
    }

    #[test]
    fn a_closed_popout_never_takes_over_the_next_reply() {
        // Its window was closed from its own title bar: saved, and the
        // reply starts fresh.
        assert_eq!(
            existing(Mode::Window, false, false).on_new_message(),
            OnNew::SaveAndReplace
        );
        assert_eq!(
            existing(Mode::Window, false, true).on_new_message(),
            OnNew::SaveAndReplace
        );
        // Still open: it comes forward.
        assert_eq!(
            existing(Mode::Window, true, false).on_new_message(),
            OnNew::RaiseWindow
        );
    }

    #[test]
    fn untouched_or_closing_messages_make_way() {
        let mut e = existing(Mode::Open, false, false);
        e.touched = false;
        assert_eq!(e.on_new_message(), OnNew::Replace);
        let mut e = existing(Mode::Window, false, false);
        e.closing = true;
        assert_eq!(e.on_new_message(), OnNew::Replace);
    }

    #[test]
    fn a_reply_here_gets_the_cursor_back() {
        assert_eq!(
            existing(Mode::Inline, false, true).on_new_message(),
            OnNew::Focus
        );
        assert_eq!(
            existing(Mode::Inline, false, false).on_new_message(),
            OnNew::ShowElsewhere
        );
    }

    #[test]
    fn scroll_offsets_snap_to_device_pixels() {
        let at = Stick {
            device: 1.25,
            ..Stick::default()
        };
        // -60.55 design pixels is -75.6875 device pixels: drawn at -76.
        assert!((at.snap(-60.55) - -60.8).abs() < 1e-4);
        assert!((at.snap(-60.3) - -60.0).abs() < 1e-4);
        // Not measured yet: as it is.
        assert_eq!(Stick::default().snap(-0.37), -0.37);
    }

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
