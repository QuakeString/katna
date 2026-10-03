// SPDX-License-Identifier: GPL-3.0-or-later

//! The contact panel: a card beside the open conversation about one of its
//! people, the newest sender unless another is picked. It shows their
//! picture, name and address, their phone, title and company from their
//! signatures, their time of day, the mail exchanged with them, recent
//! conversations and files. Everything comes from the local mail
//! ([`crate::profile`]); nothing is looked up online.
//!
//! Desktop windows only, where the reader keeps room enough beside it; the
//! reader's toolbar shows and hides it. Elsewhere a click on a person
//! opens the card's summary as a popover ([`peek`]), or a sheet on a
//! phone.

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    Animation, AnimationExt, AnyElement, ClipboardItem, Context, FontWeight, Pixels, Point,
    ScrollHandle, SharedString, Window, canvas, div, ease_out_quint, prelude::*, rgba,
};
use katna_dav::Occurrence;
use katna_i18n::tr;
use katna_render::signature;
use katna_store::{ContactConversation, ContactFile};
use katna_ui::motion::{self, Spring};
use katna_ui::{px, unpx};

mod peek;

pub(super) use peek::ContactPeek;

use super::MailWindow;
use super::attachments::kind_badge;
use super::remote::logo;
use super::select::{Pieces, selectable};
use super::sheet::{Fill, Sheet};
use crate::daemon;
use crate::data::{Entry, EntryKey, RowFile};
use crate::format;
use crate::profile::{self, Profile};
use crate::theme::{Theme, mix};
use crate::widgets::{
    card_outline, icon, icon_button, icon_button_colored, tip, tonal_icon_button,
};

/// The card's width.
pub(super) const CONTACT_WIDTH: f32 = 300.0;
/// The gap between the reader and the card, as between the other cards.
const GAP: f32 = 16.0;
/// The panel shows only while the list and the reader keep this much
/// width beside it: with the list beside the reader, and with the reader
/// alone.
const KEEP_SPLIT: f32 = 900.0;
const KEEP_ALONE: f32 = 600.0;
/// A profile is read again when shown this long after it was read.
const STALE: Duration = Duration::from_secs(60);
/// Profiles kept; more empties the cache.
const KEEP_PROFILES: usize = 64;
/// The picture beside the name.
const PICTURE: f32 = 56.0;
/// The bar the round buttons stay in once scrolled to the top of the
/// card, beside the close button, with the person's picture this size.
const BAR: f32 = 52.0;
const BAR_PICTURE: f32 = 28.0;
/// The round buttons' size and gap under the name, and in the bar.
const ACTION: (f32, f32) = (40.0, 12.0);
const ACTION_STUCK: (f32, f32) = (36.0, 6.0);
/// How far the buttons are from the bar's top while in it, as the close
/// button is from the card's.
const BAR_INSET: f32 = (BAR - ACTION.0) / 2.0;
/// The scroll over which the bar's glass and picture come in, before the
/// buttons reach it.
const BAR_FADE: f32 = 48.0;
/// Where the buttons sit in the card's content until it is measured: its
/// top padding, the picture and the gap under it.
const ACTIONS_AT: f32 = 20.0 + PICTURE + 12.0;
/// The part the card's text is in the window's text selection: after the
/// conversation's messages, so a selection from one into the other keeps
/// their order.
const CONTACT_PART: usize = usize::MAX / 2;
/// Recent conversations shown before More.
const SHOWN: usize = 3;
/// A clickable line of a section, and the gap between lines.
const ROW: f32 = 32.0;
const ROW_GAP: f32 = 2.0;

/// People by lower-case address, with a name if the mail gives one.
type People = Rc<Vec<(String, Option<String>)>>;

/// The panel's state.
pub(super) struct ContactPanel {
    /// 0 = hidden, 1 = shown.
    spring: Spring,
    /// The person picked in "In this conversation", for that conversation.
    picked: Option<(EntryKey, String)>,
    /// The people of the conversation last shown.
    people: Option<(EntryKey, People)>,
    /// Profiles by address: `None` while one is read.
    profiles: HashMap<String, (Instant, Option<Rc<Profile>>)>,
    /// Companies by address, once asked for: `None` while asked, or when
    /// there is none.
    companies: HashMap<String, Option<Rc<Company>>>,
    /// The person whose recent conversations show in full (More).
    more: Option<String>,
    /// 0 = the first few conversations, 1 = all of them.
    more_spring: Spring,
    /// The panel folded the folders to make room: they unfold again when
    /// it goes.
    folded_nav: bool,
    /// The card's scroll.
    scroll: ScrollHandle,
    /// Where the round buttons' place is in the card's content, as last
    /// laid out, and the device pixels per design pixel there.
    actions_at: Rc<Cell<(f32, f32)>>,
    /// The folders were opened beside the panel by hand: the panel leaves
    /// them be until it next opens.
    nav_hold: bool,
    /// On a phone the card rises from the bottom instead.
    sheet: Sheet,
    /// Where the panel has no room, a summary of the card pops over
    /// where the name or picture was clicked.
    peek: Option<ContactPeek>,
}

impl ContactPanel {
    pub(super) fn new() -> Self {
        Self {
            spring: Spring::new(motion::SLIDE, 0.0),
            picked: None,
            people: None,
            profiles: HashMap::new(),
            companies: HashMap::new(),
            more: None,
            more_spring: Spring::new(motion::SMOOTH, 0.0),
            folded_nav: false,
            scroll: ScrollHandle::new(),
            actions_at: Rc::new(Cell::new((ACTIONS_AT, 1.0))),
            nav_hold: false,
            sheet: Sheet::new(),
            peek: None,
        }
    }

    /// Has every profile read again when next shown, the old one showing
    /// meanwhile: when the tasks made from mail change.
    pub(super) fn forget_profiles(&mut self) {
        let Some(long_ago) = Instant::now().checked_sub(STALE) else {
            return;
        };
        for (at, _) in self.profiles.values_mut() {
            *at = long_ago;
        }
    }
}

impl MailWindow {
    /// Whether the panel can show beside cards `available` wide: on a
    /// desktop, in the main window, with the list and reader left room.
    pub(super) fn contact_fits(&self, available: f32) -> bool {
        let rest = available - CONTACT_WIDTH - GAP;
        !self.detached
            && self.layout.shape.is_desktop()
            && rest >= if self.split() { KEEP_SPLIT } else { KEEP_ALONE }
    }

    /// Whether the panel would show, room aside.
    fn contact_wanted(&self) -> bool {
        self.config.mail.contact_panel
            && !self.agenda_open()
            && self.reading
            && self.reader.is_some()
            && !self.settings_in_main()
            && !self.detached
            && self.layout.shape.is_desktop()
    }

    /// Whether the panel can show beside the cards as they are now, with
    /// the folders folded if they are open: where its button is offered.
    pub(super) fn contact_offered(&self) -> bool {
        let folders = super::NAV_WIDTH * self.reserve_spring.value().clamp(0.0, 1.0);
        self.contact_fits(self.cards_width + self.contact_room() + folders)
    }

    /// Before a frame's springs: folds the folders when the panel is wanted
    /// and fits only without them, and unfolds them once the panel goes or
    /// the window has room for both. `room` is what the folders, the cards
    /// and the panel share.
    pub(super) fn fold_nav_for_contact(&mut self, room: f32) {
        self.settle_contact_peek();
        let wanted = self.contact_wanted();
        if !wanted {
            self.contact.nav_hold = false;
        }
        let beside = self.contact_fits(room - super::NAV_WIDTH);
        if self.contact.folded_nav {
            if !wanted || beside || !self.layout.shape.is_desktop() {
                self.contact.folded_nav = false;
                self.nav_open = true;
            }
        } else if wanted
            && self.app == super::apps::App::Mail
            && self.nav_docked()
            && !self.contact.nav_hold
            && !beside
            && self.contact_fits(room)
        {
            self.contact.folded_nav = true;
            self.nav_open = false;
        }
    }

    /// The folders were folded or unfolded by hand.
    pub(super) fn nav_toggled_by_hand(&mut self) {
        // Opened beside the panel: they stay until the panel next opens.
        self.contact.nav_hold = self.nav_open && self.contact_wanted();
        self.contact.folded_nav = false;
    }

    /// Whether the folders are folded only for the panel, so a restart
    /// shows them open.
    pub(super) fn nav_folded_for_contact(&self) -> bool {
        self.contact.folded_nav
    }

    /// Moves the panel toward shown or hidden for this frame, and returns
    /// the width it takes from cards `available` wide.
    pub(super) fn tick_contact(
        &mut self,
        available: f32,
        window: &Window,
        reduce: bool,
    ) -> (f32, f32) {
        // As the folders fold for it, it opens with them, against the room
        // it will have.
        let folding = super::NAV_WIDTH
            * (self.reserve_spring.value() - self.reserve_spring.target()).max(0.0);
        let open = self.config.mail.contact_panel
            && !self.agenda_open()
            && self.reading
            && self.reader.is_some()
            && !self.settings_in_main()
            && self.contact_fits(available + folding);
        self.contact.spring.set(if open { 1.0 } else { 0.0 });
        let t = self.contact.spring.tick(window, reduce).clamp(0.0, 1.0);
        let more = self.contact.more.is_some();
        self.contact.more_spring.set(if more { 1.0 } else { 0.0 });
        self.contact.more_spring.tick(window, reduce);
        let room = |t: f32| (CONTACT_WIDTH + GAP) * t;
        (room(t), room(self.contact.spring.target()))
    }

    /// Shows `email` (lower case) in the panel, opening it if it was put
    /// away: an address clicked at `at` in the open mail's details.
    /// A second click on the person whose card shows puts the panel away.
    pub(super) fn show_person(&mut self, email: &str, at: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(key) = self.reader.as_ref().map(|r| r.key) {
            self.show_contact_of(key, email, at, cx);
        }
    }

    /// Shows `email`'s card for conversation `key`, opening the panel if
    /// it is hidden: a click at `at` on a name or picture in the chat
    /// view. A second click on the person whose card shows puts the panel
    /// away. Where the panel has no room, the card's summary pops over
    /// at `at` instead.
    pub(super) fn show_contact_of(
        &mut self,
        key: EntryKey,
        email: &str,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let email = email.to_lowercase();
        // A phone has no room beside the chat: the card rises as a sheet.
        if self.layout.shape.is_phone() {
            self.contact.picked = Some((key, email));
            self.contact.sheet.show(true);
            cx.notify();
            return;
        }
        if !self.contact_offered() {
            self.contact.picked = Some((key, email));
            self.contact.peek = Some(ContactPeek::new(key, at));
            cx.notify();
            return;
        }
        self.contact.peek = None;
        let shown = self.config.mail.contact_panel;
        if shown && self.contact_person_shown().as_deref() == Some(email.as_str()) {
            self.toggle_contact_panel(cx);
            return;
        }
        self.contact.picked = Some((key, email));
        if !self.config.mail.contact_panel && self.contact_offered() {
            self.toggle_contact_panel(cx);
        }
        cx.notify();
    }

    /// The address whose card the panel shows for the open conversation:
    /// the one picked, else the newest sender.
    fn contact_person_shown(&mut self) -> Option<String> {
        let key = self.reader.as_ref().map(|r| r.key);
        let people = self.contact_people();
        self.contact
            .picked
            .as_ref()
            .filter(|(k, email)| Some(*k) == key && people.iter().any(|(e, _)| e == email))
            .map(|(_, email)| email.clone())
            .or_else(|| people.first().map(|(email, _)| email.clone()))
    }

    /// Shows or hides the panel, from the reader's toolbar.
    pub(super) fn toggle_contact_panel(&mut self, cx: &mut Context<Self>) {
        self.config.mail.contact_panel = !self.config.mail.contact_panel;
        // Asked for, it may fold folders opened beside it by hand.
        self.contact.nav_hold = false;
        self.save_config();
        cx.notify();
    }

    /// The reader toolbar's button for the panel, where it fits.
    pub(super) fn contact_toggle(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.contact_offered() {
            return None;
        }
        let on = self.config.mail.contact_panel;
        Some(
            icon_button_colored(
                "reader-contact",
                "contacts",
                20.0,
                if on { th.accent } else { th.text_dim },
                th,
            )
            .tooltip(tip(
                if on {
                    tr!("contact-panel-hide")
                } else {
                    tr!("contact-panel-show")
                },
                th,
            ))
            .on_click(cx.listener(|this, _, _, cx| this.toggle_contact_panel(cx)))
            .into_any_element(),
        )
    }

    /// The width the panel takes now.
    pub(super) fn contact_room(&self) -> f32 {
        (CONTACT_WIDTH + GAP) * self.contact.spring.value().clamp(0.0, 1.0)
    }

    /// The people of the open conversation, newest message first.
    fn contact_people(&mut self) -> People {
        let Some(key) = self.reader.as_ref().map(|r| r.key) else {
            return Rc::default();
        };
        if let Some((shown, people)) = &self.contact.people
            && *shown == key
        {
            return people.clone();
        }
        let people = match &self.mail {
            Ok(mail) => {
                let mut ids = mail.entry_messages(key);
                ids.reverse();
                mail.message_people(&ids)
            }
            Err(_) => Vec::new(),
        };
        let people = Rc::new(people);
        self.contact.people = Some((key, people.clone()));
        people
    }

    /// The profile of `email`, read in the background when not known or
    /// stale; the old one shows meanwhile.
    fn contact_profile(&mut self, email: &str, cx: &mut Context<Self>) -> Option<Rc<Profile>> {
        let known = self.contact.profiles.get(email).cloned();
        // Being read for the first time, or read a while ago.
        let reading = matches!(known, Some((_, None)));
        let stale = known.as_ref().is_none_or(|(at, _)| at.elapsed() >= STALE);
        if stale && !reading {
            if self.contact.profiles.len() >= KEEP_PROFILES {
                self.contact.profiles.clear();
            }
            // The old profile shows until the new one is read.
            let old = known.as_ref().and_then(|(_, p)| p.clone());
            self.contact
                .profiles
                .insert(email.to_owned(), (Instant::now(), old));
            let paths = self.paths.clone();
            let address = email.to_owned();
            let task_mails = self.tasks.open_mails.clone();
            cx.spawn(async move |this, cx| {
                let read = cx
                    .background_executor()
                    .spawn({
                        let address = address.clone();
                        async move { profile::read(&paths, &address, &task_mails) }
                    })
                    .await;
                this.update(cx, |this, cx| {
                    match read {
                        Ok(profile) => {
                            this.contact
                                .profiles
                                .insert(address, (Instant::now(), Some(Rc::new(profile))));
                        }
                        Err(err) => tracing::warn!("reading the contact of {address}: {err}"),
                    }
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
        known.and_then(|(_, p)| p)
    }

    /// Asks the daemon once for the company of the person at `email`:
    /// in the chat view, under the Sender pictures switch (§12).
    fn contact_company(&mut self, email: &str, website: Option<&str>, cx: &mut Context<Self>) {
        if !self.chat_shown() || !(self.config.mail.sender_pictures || self.remote.trusts(email)) {
            return;
        }
        let key = email.to_lowercase();
        if self.contact.companies.contains_key(&key) {
            return;
        }
        if self.contact.companies.len() >= KEEP_PROFILES {
            self.contact.companies.clear();
        }
        self.contact.companies.insert(key.clone(), None);
        let connection = self.daemon.clone();
        let website = website.unwrap_or_default().to_owned();
        cx.spawn(async move |this, cx| {
            let json = cx
                .background_executor()
                .spawn({
                    let key = key.clone();
                    async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        daemon::company_of(&connection, &key, &website).await
                    }
                })
                .await;
            let company = json
                .ok()
                .filter(|j| !j.is_empty())
                .and_then(|j| serde_json::from_str::<Company>(&j).ok())
                .filter(|c| !c.name.is_empty());
            let Some(company) = company else {
                return;
            };
            this.update(cx, |this, cx| {
                this.contact.companies.insert(key, Some(Rc::new(company)));
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// The panel, `t` of the way in, beside the cards.
    pub(super) fn render_contact_panel(
        &mut self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let t = self.contact.spring.value().clamp(0.0, 1.0);
        if t <= 0.001 {
            return None;
        }
        let room = crate::widgets::CARD_SHADOW_ROOM;
        let card = self.render_contact_card(th, cx);
        Some(
            // Stretched by the row to its height plus the shadow's room,
            // as the reading pane is.
            div()
                .flex_none()
                .w(px((CONTACT_WIDTH + GAP) * t + room))
                .mt(px(-room))
                .mb(px(-room))
                .mr(px(-room))
                .py(px(room))
                .pr(px(room))
                .pl(px(GAP * t))
                .overflow_hidden()
                .child(
                    div()
                        .w(px(CONTACT_WIDTH))
                        .h_full()
                        .ml(px(24.0 * (1.0 - t)))
                        .opacity(t)
                        .child(card),
                )
                .into_any_element(),
        )
    }

    /// On a phone, the card of the person picked in the chat, risen from
    /// the bottom.
    pub(super) fn render_contact_sheet(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !(self.layout.shape.is_phone() && self.reading && self.reader.is_some()) {
            if self.contact.sheet.is_shown() {
                self.contact.sheet = Sheet::new();
            }
            return None;
        }
        let body = div()
            .pb(px(8.0))
            .child(self.contact_card_body(false, th, cx).0)
            .into_any_element();
        self.bottom_sheet(
            "contact-sheet",
            &self.contact.sheet,
            Fill::Card,
            body,
            |this| Some(&mut this.contact.sheet),
            |this, cx| {
                this.contact.sheet.show(false);
                cx.notify();
            },
            th,
            window,
            cx,
        )
    }

    fn render_contact_card(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (radius, outline) = (
            self.layout.shape.card_radius(),
            self.layout.shape.card_outline(),
        );
        let (shadow, edge) = self.card_edges(0.0, outline);
        let (place, stuck) = self.contact_bar();
        let (body, actions) = self.contact_card_body(false, th, cx);
        let glass = (actions.is_some() && stuck > 0.0).then(|| {
            crate::widgets::frosted_top(
                div()
                    .absolute()
                    .top(px(outline))
                    .left(px(outline))
                    .right(px(outline))
                    .h(px(BAR))
                    .opacity(stuck),
                th,
                th.surface,
                (radius - outline).max(0.0),
            )
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .bottom_0()
                    .h(px(1.0))
                    .bg(rgba(th.divider)),
            )
        });
        let bar = actions.map(|actions| {
            div()
                .absolute()
                .left(px(outline))
                .right(px(outline))
                .top(px(outline + (place - BAR_INSET).max(0.0)))
                .h(px(BAR))
                .child(actions)
        });
        div()
            .id("contact-card")
            .size_full()
            .relative()
            .overflow_hidden()
            .map(|d| crate::widgets::card(d, th, th.pane(), radius, shadow))
            .p(px(outline))
            .child(
                div()
                    .id("contact-scroll")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.contact.scroll)
                    .child(body),
            )
            .children(glass)
            .children(bar)
            .children(card_outline(th, radius, edge))
            // Puts the panel away, as its toolbar button does: the same
            // round close button as the open mail's.
            .child(
                div().absolute().top(px(6.0)).right(px(6.0)).child(
                    icon_button("contact-close", "close", 20.0, th)
                        .tooltip(tip(tr!("contact-panel-hide"), th))
                        .on_click(cx.listener(|this, _, _, cx| this.toggle_contact_panel(cx))),
                ),
            )
            .into_any_element()
    }

    /// What the card shows of the person picked, else the newest sender,
    /// and the bar of round buttons drawn over it beside an open mail.
    /// The `summary` is the popover's: the name, the round buttons and
    /// the details, without the mail, conversations and files.
    fn contact_card_body(
        &mut self,
        summary: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (AnyElement, Option<AnyElement>) {
        let shown = self.contact_person_shown();
        let people = self.contact_people();
        let person = shown.and_then(|email| people.iter().find(|(e, _)| *e == email).cloned());
        // More folds back for another person.
        if self.contact.more.as_ref() != person.as_ref().map(|(email, _)| email) {
            self.contact.more = None;
            self.contact.more_spring.snap(0.0);
        }
        match person {
            Some((email, name)) => {
                // Mail only between the user's own addresses shows that
                // address, without the numbers of mail "with them".
                let profile = if self.is_own(&email) {
                    None
                } else {
                    self.contact_profile(&email, cx)
                };
                // Asked for here; the body finds it once it comes.
                if let Some(profile) = &profile {
                    self.contact_company(&email, profile.card.website.as_deref(), cx);
                }
                self.render_contact_body(&email, name.as_deref(), profile, &people, summary, th, cx)
            }
            None => (contact_empty(th), None),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_contact_body(
        &self,
        email: &str,
        name: Option<&str>,
        profile: Option<Rc<Profile>>,
        people: &[(String, Option<String>)],
        summary: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (AnyElement, Option<AnyElement>) {
        let own = self.is_own(email);
        let muted = !own && self.sender_muted(email);
        let known_name = profile.as_ref().and_then(|p| p.summary.name.clone());
        // The signature of the open mail, else of their newest one.
        let open_signature = self
            .reading
            .then(|| self.reader.as_ref()?.signature_of(email))
            .flatten();
        // A colleague signed the open mail from a shared address
        // ("accounts@"): the card is theirs, not that of a name the
        // address used on other mail.
        let signer = open_signature.as_deref().and_then(|s| {
            let signer = signature::signer(s)?;
            (known_name.is_none() || signature::someone_else(s, known_name.as_deref()))
                .then_some(signer)
        });
        let name = signer
            .clone()
            .or(known_name)
            .or_else(|| name.map(str::to_owned));
        // What older mail said of the person is not the signer's.
        let card = profile.as_ref().map(|p| match signer {
            Some(_) => profile::Card {
                website: p.card.website.clone(),
                ..profile::Card::default()
            },
            None => p.card.clone(),
        });
        let looked_up = self
            .contact
            .companies
            .get(&email.to_lowercase())
            .cloned()
            .flatten();
        let signature = open_signature.or_else(|| profile.as_ref()?.signature.clone());
        let details = signature.and_then(|signature| {
            let shown: Vec<&str> = card
                .iter()
                .flat_map(|c| [&c.title, &c.phone])
                .flatten()
                .map(String::as_str)
                .collect();
            let known_company = card
                .as_ref()
                .and_then(|c| c.company.as_deref())
                .or(looked_up.as_deref().map(|c| c.name.as_str()));
            signature::details(
                &signature,
                &signature::Known {
                    name: name.as_deref(),
                    email,
                    shown: &shown,
                    company: known_company,
                },
            )
        });
        let shown_name = name.clone().unwrap_or_else(|| email.to_owned());
        // All of the card's text can be selected and copied, as the
        // conversation's can.
        let mut pieces = self.text.pieces(CONTACT_PART, th);
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            // Clear of the close button in the corner.
            .when(!summary, |d| d.pr(px(30.0)))
            .child(self.person_avatar(&shown_name, email, PICTURE))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .child(
                                words(&mut pieces, shown_name.clone())
                                    .min_w_0()
                                    .text_size(px(16.0))
                                    .line_height(px(22.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.text)),
                            )
                            // Their mail is muted (§15.1.1).
                            .children(muted.then(|| self.muted_mark(email, 16.0, th)).flatten()),
                    )
                    .when(name.is_some(), |d| {
                        d.child(
                            words(&mut pieces, email.to_owned())
                                .text_size(px(13.0))
                                .line_height(px(18.0))
                                .text_color(rgba(th.text_dim))
                                .truncate(),
                        )
                    }),
            );

        let phone = card.as_ref().and_then(|c| c.phone.clone()).or_else(|| {
            details
                .as_ref()
                .and_then(|d| d.phones.first())
                .map(|p| p.number.clone())
        });
        // On a phone the card is a sheet with no close button, and the
        // buttons scroll with the rest, as in the popover.
        let sheet = self.layout.shape.is_phone() || summary;
        let stuck = if sheet { 0.0 } else { self.contact_bar().1 };
        let size = ACTION.0 + (ACTION_STUCK.0 - ACTION.0) * stuck;
        let mut actions = self.contact_actions(email, phone, size, th, cx);
        // Their address book entry: open it, or save them in one click.
        if !own && self.contacts.book.as_ref().is_some_and(|b| b.is_ok()) {
            actions = actions.child(self.contact_save_button(
                email,
                name.clone(),
                card.clone(),
                size,
                th,
                cx,
            ));
        }
        let actions = actions.gap(px(ACTION.1 + (ACTION_STUCK.1 - ACTION.1) * stuck));
        let (place, bar) = if sheet {
            (actions.mx(px(4.0)).mb(px(4.0)).into_any_element(), None)
        } else {
            self.contact_bar_parts(actions, stuck, &shown_name, email)
        };
        // Each section is a faintly tinted card, as in Google Contacts.
        let mut sections: Vec<AnyElement> = Vec::new();
        if own {
            sections.push(
                words(&mut pieces, tr!("contact-own-account"))
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text))
                    .into_any_element(),
            );
        }
        if let Some(profile) = &profile
            && let Some(card) = &card
            && let Some(details) =
                self.contact_details(profile, card, details.as_ref(), &mut pieces, th, cx)
        {
            sections.push(details);
        }
        // The company as its home page describes it, else as the
        // signature does.
        let company = looked_up.filter(|_| !summary).or_else(|| {
            if summary {
                return None;
            }
            let from = details.as_ref().map(|d| &d.company);
            let name = from
                .and_then(|c| c.name.clone().or_else(|| c.logo.clone()))
                .or_else(|| card.as_ref().and_then(|c| c.company.clone()))
                // Else it goes by its website.
                .or_else(|| from?.website.as_deref().map(signature::host))?;
            let says_more = from.is_some_and(|c| {
                c.website.is_some()
                    || !c.pages.is_empty()
                    || !c.offices.is_empty()
                    || c.group.is_some()
            });
            says_more.then(|| {
                Rc::new(Company {
                    name,
                    website: from.and_then(|c| c.website.clone()).unwrap_or_default(),
                    ..Company::default()
                })
            })
        });
        if let Some(company) = &company {
            sections.push(self.contact_company_section(
                email,
                company,
                details.as_ref().map(|d| &d.company),
                &mut pieces,
                th,
            ));
        }
        if let Some(profile) = profile.as_ref().filter(|_| !summary) {
            sections.push(self.contact_mail(profile, &mut pieces, th));
            if let Some(tasks) = self.contact_tasks(&profile.task_mails, &mut pieces, th, cx) {
                sections.push(tasks);
            }
            if !profile.meetings.is_empty() {
                sections.push(self.contact_meetings(&profile.meetings, &mut pieces, th, cx));
            }
            if !profile.conversations.is_empty() {
                sections.push(self.contact_conversations(
                    email,
                    &profile.conversations,
                    &mut pieces,
                    th,
                    cx,
                ));
            }
            if !profile.files.is_empty() {
                sections.push(self.contact_files(&profile.files, &mut pieces, th, cx));
            }
        }
        if !summary && people.len() > 1 {
            sections.push(self.contact_others(email, people, &mut pieces, th, cx));
        }
        let tint = card_tint(th);
        let foot = (profile.is_some() && !summary).then(|| {
            words(&mut pieces, tr!("contact-local-only"))
                .px(px(4.0))
                .text_size(px(12.0))
                .line_height(px(16.0))
                .text_color(rgba(th.text_faint))
        });
        let body = div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .px(px(12.0))
            .pt(px(20.0))
            .pb(px(16.0))
            .child(header.mx(px(4.0)))
            .child(place)
            .when(muted, |d| {
                d.child(
                    div()
                        .mx(px(4.0))
                        .mt(px(-4.0))
                        .text_size(px(13.0))
                        .line_height(px(18.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("quiet-sender-strip")),
                )
            })
            .children(sections.into_iter().map(|section| {
                div()
                    .rounded(px(16.0))
                    .bg(rgba(tint))
                    .px(px(16.0))
                    .py(px(14.0))
                    .child(section)
            }))
            .children(foot);
        // A new person fades in, as a conversation opens.
        let body = selectable(body, Some(CONTACT_PART), cx)
            .with_animation(
                ("contact-person", person_number(email)),
                Animation::new(katna_ui::motion::time(Duration::from_millis(220)))
                    .with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element();
        (body, bar)
    }

    /// Where the round buttons are in the card, scrolled, and how far
    /// (0 to 1) into the bar at its top: they scroll up with the name
    /// until they reach the top, then stay there in a frosted bar beside
    /// the close button, with the person's picture, as the rest goes on
    /// under them.
    fn contact_bar(&self) -> (f32, f32) {
        let (at, device) = self.contact.actions_at.get();
        let scroll = &self.contact.scroll;
        let max = unpx(scroll.max_offset().y).max(0.0);
        let scrolled = -snap(unpx(scroll.offset().y).clamp(-max, 0.0), device);
        let place = at - scrolled;
        (place, smoothstep((BAR_INSET + BAR_FADE - place) / BAR_FADE))
    }

    /// The round buttons' place in the card, and the bar they are drawn
    /// in over it: in the bar, the buttons make room for the close button
    /// and the person's picture comes in at the left.
    fn contact_bar_parts(
        &self,
        actions: gpui::Div,
        stuck: f32,
        shown_name: &str,
        email: &str,
    ) -> (AnyElement, Option<AnyElement>) {
        let picture = ((stuck - 0.4) / 0.6).clamp(0.0, 1.0);
        let bar = div()
            .size_full()
            .flex()
            .flex_row()
            .items_center()
            .pl(px(12.0))
            .pr(px(16.0 + 36.0 * stuck))
            .child(
                div()
                    .flex_none()
                    .opacity(picture)
                    .ml(px(-6.0 * (1.0 - picture)))
                    .child(self.person_avatar(shown_name, email, BAR_PICTURE)),
            )
            .child(div().flex_1())
            .child(actions)
            .with_animation(
                ("contact-bar", person_number(email)),
                Animation::new(katna_ui::motion::time(Duration::from_millis(220)))
                    .with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element();
        // The buttons' place, which the bar is drawn over.
        let (at, scroll) = (self.contact.actions_at.clone(), self.contact.scroll.clone());
        let place = div().mx(px(4.0)).mb(px(4.0)).h(px(ACTION.0)).child(
            canvas(
                move |bounds, window, _| {
                    let y = unpx(bounds.top() - scroll.bounds().top() - scroll.offset().y);
                    let device = window.scale_factor();
                    if (y - at.get().0).abs() > 0.25 || device != at.get().1 {
                        at.set((y, device));
                        window.refresh();
                    }
                },
                |_, _, _, _| {},
            )
            .size_full(),
        );
        (place.into_any_element(), Some(bar))
    }

    /// Whether `email` is one of the user's own addresses.
    fn is_own(&self, email: &str) -> bool {
        self.mail.as_ref().is_ok_and(|mail| mail.is_me(email))
    }

    /// Round tinted buttons under the name, as in Google Contacts: write
    /// to them, find the mail with them, and call them when their number
    /// is known.
    fn contact_actions(
        &self,
        email: &str,
        phone: Option<String>,
        size: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let button = |id: &'static str, name: &str, label: String| {
            tonal_icon_button(id, name, size, false, true, th).tooltip(tip(label, th))
        };
        let to = email.to_owned();
        let query = format!("from:{email} OR to:{email}");
        // Mute their mail, or unmute it: filled while muted.
        let mute =
            (!self.is_own(email)).then(|| {
                let muted = self.sender_muted(email);
                let address = email.to_owned();
                let label = if muted {
                    tr!("quiet-unmute-sender")
                } else {
                    tr!("quiet-mute-sender")
                };
                tonal_icon_button("contact-mute", "bell-off", size, muted, true, th)
                    .tooltip(tip(label, th))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.mute_sender(address.clone(), !muted, cx)
                    }))
            });
        div()
            .flex()
            .flex_row()
            .justify_end()
            .children(mute)
            .child(
                button("contact-email", "mail", tr!("contact-email")).on_click(cx.listener(
                    move |this, _, window, cx| {
                        let mail = crate::mailto::Mailto {
                            to: vec![to.clone()],
                            ..Default::default()
                        };
                        this.open_mailto(mail, window, cx);
                    },
                )),
            )
            .child(
                button("contact-search", "search", tr!("contact-search")).on_click(cx.listener(
                    move |this, _, window, cx| this.search_for(query.clone(), window, cx),
                )),
            )
            .children(phone.map(|number| {
                let dial = format!("tel:{}", dialable(&number));
                button("contact-call-button", "phone", tr!("contact-call"))
                    .on_click(move |_, _, cx| cx.open_url(&dial))
            }))
    }

    /// Add to contacts for someone not saved yet; Open contact for
    /// someone saved.
    fn contact_save_button(
        &self,
        email: &str,
        name: Option<String>,
        signature: Option<profile::Card>,
        size: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let saved = self.is_saved_contact(email);
        let adding = self.contacts.adding.contains(&email.trim().to_lowercase());
        let (glyph, label) = if saved {
            ("contacts", tr!("contact-open-contact"))
        } else {
            ("person-add", tr!("contact-add-to-contacts"))
        };
        let email = email.to_owned();
        tonal_icon_button("contact-save", glyph, size, false, !adding, th)
            .tooltip(tip(label, th))
            .when(!adding, |d| {
                d.on_click(cx.listener(move |this, _, window, cx| {
                    if this.is_saved_contact(&email) {
                        this.show_saved_contact(&email, cx);
                        window.focus(&this.window_focus, cx);
                    } else {
                        this.add_to_contacts(&email, name.clone(), signature.clone(), cx);
                    }
                }))
            })
            .into_any_element()
    }

    /// Phone, title and company, and their time of day.
    fn contact_details(
        &self,
        profile: &Profile,
        card: &profile::Card,
        details: Option<&signature::Details>,
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // The title, with the company under it in a quieter color.
        let title = card
            .title
            .as_ref()
            .or_else(|| details.and_then(|d| d.title.as_ref()));
        let work = title.or(card.company.as_ref()).map(|first| {
            let under = title.and(card.company.clone());
            div()
                .flex()
                .flex_col()
                .child(words(pieces, first.clone()))
                .children(under.map(|company| words(pieces, company).text_color(rgba(th.text_dim))))
                .into_any_element()
        });
        // Every number their signature gives, with its kind; the one read
        // from older mail too, when it is another.
        let mut numbers: Vec<(String, Option<signature::PhoneKind>)> = details
            .into_iter()
            .flat_map(|d| &d.phones)
            .map(|p| (p.number.clone(), Some(p.kind)))
            .collect();
        if let Some(number) = &card.phone
            && !numbers.iter().any(|(n, _)| same_number(n, number))
        {
            numbers.insert(0, (number.clone(), None));
        }
        let phones: Vec<AnyElement> = numbers
            .into_iter()
            .enumerate()
            .map(|(ix, (number, kind))| self.contact_phone(ix, number, kind, th, cx))
            .collect();
        let phone = (!phones.is_empty()).then(|| {
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .children(phones)
                .into_any_element()
        });
        let emails = details
            .filter(|d| !d.emails.is_empty())
            .map(|d| words(pieces, d.emails.join("\n")).into_any_element());
        // Any other line their signature has, quietly.
        let other = details.filter(|d| !d.other.is_empty()).map(|d| {
            words(pieces, d.other.join("\n"))
                .text_color(rgba(th.text_dim))
                .into_any_element()
        });
        let pages = details
            .filter(|d| !d.pages.is_empty())
            .map(|d| page_chips("contact-own-page", &d.pages, th));
        let time = profile.offset.and_then(|minutes| {
            let offset = jiff::tz::Offset::from_seconds(minutes * 60).ok()?;
            let now = jiff::Timestamp::now().as_second();
            let local = format::local(now, &jiff::tz::TimeZone::fixed(offset))?;
            Some(tr!(
                "contact-local-time",
                time = katna_i18n::format::time(local),
                offset = profile::offset_label(minutes)
            ))
        });
        let time = time.map(|t| words(pieces, t).into_any_element());
        let rows: Vec<(&str, AnyElement)> = [
            ("work", work),
            ("phone", phone),
            ("mail", emails),
            ("link", pages),
            ("info", other),
            ("schedule", time),
        ]
        .into_iter()
        .filter_map(|(icon, text)| Some((icon, text?)))
        .collect();
        if rows.is_empty() {
            return None;
        }
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .children(rows.into_iter().map(|(name, text)| {
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(12.0))
                        .child(div().pt(px(1.0)).child(icon(name, th.text_faint, 18.0)))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .text_size(px(14.0))
                                .line_height(px(20.0))
                                .text_color(rgba(th.text))
                                .child(text),
                        )
                }))
                .into_any_element(),
        )
    }

    /// The Company section: its logo, name, where it is and since when,
    /// what it does, Wikipedia's lines, and its pages.
    fn contact_company_section(
        &self,
        email: &str,
        company: &Company,
        from_signature: Option<&signature::CompanyDetails>,
        pieces: &mut Pieces,
        th: &Theme,
    ) -> AnyElement {
        let initial = company
            .name
            .chars()
            .find(|c| c.is_alphanumeric())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default();
        let picture = match self.domain_logo(email) {
            Some(picture) => logo(picture, 36.0),
            None => div()
                .size(px(36.0))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(9.0))
                .bg(rgba(th.chip))
                .text_size(px(15.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgba(th.text_dim))
                .child(initial)
                .into_any_element(),
        };
        let about = [
            (!company.place.is_empty()).then(|| company.place.clone()),
            company
                .founded
                .map(|year| tr!("contact-company-since", year = year.to_string())),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ");
        let text = |pieces: &mut Pieces, text: &str| {
            words(pieces, text.to_owned())
                .text_size(px(13.0))
                .line_height(px(19.0))
                .text_color(rgba(th.text))
        };
        // Its pages: the home page's, then the signature's it lacks.
        let mut pages: Vec<signature::Link> = std::iter::once(&company.website)
            .chain(from_signature.and_then(|c| c.website.as_ref()))
            .chain(&company.links)
            .filter(|url| !url.is_empty())
            .map(|url| signature::Link {
                site: signature::Site::of(url),
                url: url.clone(),
            })
            .collect();
        for link in from_signature.into_iter().flat_map(|c| &c.pages) {
            let host = signature::host(&link.url);
            let known = pages.iter().any(|p| {
                signature::host(&p.url) == host
                    && (p.site == signature::Site::Web
                        || p.url.trim_end_matches('/') == link.url.trim_end_matches('/'))
            });
            if !known {
                pages.push(link.clone());
            }
        }
        let mut seen = Vec::new();
        pages.retain(|p| {
            let key = (
                p.site,
                signature::host(&p.url),
                p.site != signature::Site::Web,
            );
            let key = if key.2 {
                p.url.trim_end_matches('/').to_owned()
            } else {
                key.1
            };
            if seen.contains(&key) {
                false
            } else {
                seen.push(key);
                true
            }
        });
        let group = from_signature.and_then(|c| c.group.clone());
        let offices = from_signature
            .map(|c| c.offices.as_slice())
            .unwrap_or_default()
            .iter()
            .map(|office| {
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(8.0))
                    .child(
                        div()
                            .pt(px(1.0))
                            .child(icon("location", th.text_faint, 16.0)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .children(office.label.clone().map(|label| {
                                words(pieces, label)
                                    .text_size(px(12.0))
                                    .line_height(px(17.0))
                                    .text_color(rgba(th.text_dim))
                            }))
                            .child(text(pieces, &office.lines.join("\n"))),
                    )
            })
            .collect::<Vec<_>>();
        let now = jiff::Timestamp::now().as_second();
        let checked = (company.checked > 0)
            .then(|| format::ago(company.checked, now))
            .flatten()
            .map(|when| {
                words(
                    pieces,
                    tr!(
                        "contact-company-from",
                        site = host_label(&company.website),
                        when = when
                    ),
                )
                .text_size(px(11.5))
                .line_height(px(16.0))
                .text_color(rgba(th.text_faint))
            });
        section(words(pieces, tr!("contact-company")), th)
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .child(picture)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(
                                words(pieces, company.name.clone())
                                    .text_size(px(14.0))
                                    .line_height(px(20.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.text)),
                            )
                            .when(!about.is_empty(), |d| {
                                d.child(
                                    words(pieces, about)
                                        .text_size(px(12.0))
                                        .line_height(px(16.0))
                                        .text_color(rgba(th.text_dim)),
                                )
                            })
                            // "Part of the Demo Group", from the signature.
                            .children(group.map(|group| {
                                words(pieces, group)
                                    .text_size(px(12.0))
                                    .line_height(px(16.0))
                                    .text_color(rgba(th.text_dim))
                            })),
                    ),
            )
            .when(!company.description.is_empty(), |d| {
                d.child(text(pieces, &company.description).text_color(rgba(th.text_dim)))
            })
            .when(!company.summary.is_empty(), |d| {
                d.child(text(pieces, &company.summary).text_color(rgba(th.text_dim)))
            })
            .children(offices)
            .when(!pages.is_empty(), |d| {
                d.child(page_chips("contact-company-page", &pages, th))
            })
            .children(checked)
            .into_any_element()
    }

    /// Their phone number: a click calls it (the desktop hands `tel:` to
    /// the phone app or KDE Connect), and a copy button shows on hover.
    fn contact_phone(
        &self,
        ix: usize,
        number: String,
        kind: Option<signature::PhoneKind>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        use signature::PhoneKind;
        let dial = format!("tel:{}", dialable(&number));
        let copied = number.clone();
        let group = SharedString::from(format!("contact-phone-{ix}"));
        let kind = kind.and_then(|kind| match kind {
            PhoneKind::Phone => None,
            PhoneKind::Mobile => Some(tr!("contact-phone-mobile")),
            PhoneKind::Direct => Some(tr!("contact-phone-direct")),
            PhoneKind::Office => Some(tr!("contact-phone-office")),
            PhoneKind::Fax => Some(tr!("contact-phone-fax")),
            PhoneKind::WhatsApp => Some(tr!("contact-phone-whatsapp")),
        });
        div()
            .id(("contact-phone", ix))
            .group(group.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(
                div()
                    .id(("contact-call", ix))
                    .min_w_0()
                    .cursor_pointer()
                    .text_color(rgba(th.accent))
                    .hover(|s| s.underline())
                    .tooltip(tip(tr!("contact-call"), th))
                    .on_click(move |_, _, cx| cx.open_url(&dial))
                    .child(number),
            )
            // "Mobile", "Direct", quietly after the number.
            .children(kind.map(|kind| {
                div()
                    .flex_none()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child(kind)
            }))
            .child(
                div()
                    .id(("contact-copy-number", ix))
                    .flex_none()
                    .size(px(24.0))
                    .my(px(-2.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .cursor_pointer()
                    .opacity(0.0)
                    .group_hover(group, |s| s.opacity(1.0))
                    .hover(|s| s.bg(rgba(th.hover)))
                    .tooltip(tip(tr!("contact-copy-number"), th))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(copied.clone()));
                        this.show_snackbar(tr!("contact-number-copied"), None, cx);
                    }))
                    .child(icon("copy", th.text_dim, 16.0)),
            )
            .into_any_element()
    }

    /// How much mail was exchanged, and when.
    fn contact_mail(&self, profile: &Profile, pieces: &mut Pieces, th: &Theme) -> AnyElement {
        let summary = &profile.summary;
        let date = |unix: Option<i64>| {
            unix.and_then(|d| format::local(d, &self.tz))
                .map(katna_i18n::format::day_month_year)
                .unwrap_or_default()
        };
        let count = words(pieces, tr!("contact-messages", count = summary.messages))
            .text_size(px(14.0))
            .line_height(px(20.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.text));
        let from_to = words(
            pieces,
            tr!(
                "contact-from-to",
                from = katna_i18n::format::number(summary.from_them),
                to = katna_i18n::format::number(summary.to_them)
            ),
        )
        .pb(px(4.0))
        .text_size(px(13.0))
        .line_height(px(18.0))
        .text_color(rgba(th.text_dim));
        let mut fact = |label: String, value: String| {
            div()
                .flex()
                .flex_row()
                .justify_between()
                .gap(px(12.0))
                .text_size(px(13.0))
                .line_height(px(18.0))
                .child(words(pieces, label).text_color(rgba(th.text_dim)))
                .child(words(pieces, value).text_color(rgba(th.text)))
        };
        let dates = summary.first.is_some().then(|| {
            [
                fact(tr!("contact-first"), date(summary.first)),
                fact(tr!("contact-latest"), date(summary.last)),
            ]
        });
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(count)
            .child(from_to)
            .children(dates.into_iter().flatten())
            .into_any_element()
    }

    fn contact_conversations(
        &self,
        email: &str,
        conversations: &[ContactConversation],
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let now = format::local(jiff::Timestamp::now().as_second(), &self.tz);
        let open = self.reader.as_ref().map(|r| r.key);
        let mut rows = conversations.iter().enumerate().map(|(ix, c)| {
            let key = match c.thread {
                Some(thread) => EntryKey::Thread(thread),
                None => EntryKey::Message(c.message),
            };
            let entry = Entry {
                key,
                latest: c.message,
            };
            let date = c
                .date
                .and_then(|d| format::local(d, &self.tz))
                .zip(now)
                .map(|(d, now)| format::list_date(d, now))
                .unwrap_or_default();
            let subject = if c.subject.trim().is_empty() {
                tr!("reader-no-subject")
            } else {
                c.subject.clone()
            };
            row(("contact-conversation", ix), th)
                .when(open == Some(key), |d| d.bg(rgba(th.hover)))
                .child(
                    words(pieces, subject)
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .cursor_pointer()
                        .text_color(rgba(th.text)),
                )
                .when(c.count > 1, |d| {
                    d.child(
                        words(pieces, katna_i18n::format::number(u64::from(c.count)))
                            .flex_none()
                            .cursor_pointer()
                            .text_color(rgba(th.text_faint)),
                    )
                })
                .child(
                    words(pieces, date)
                        .flex_none()
                        .cursor_pointer()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim)),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.open_contact_entry(entry, window, cx)
                }))
        });
        let first: Vec<_> = rows.by_ref().take(SHOWN).collect();
        let rest: Vec<_> = rows.collect();
        let mut list = section(words(pieces, tr!("contact-conversations")), th).children(first);
        if rest.is_empty() {
            return list.into_any_element();
        }
        // The rest unfolds smoothly under the first few, and folds back.
        let t = self.contact.more_spring.value().clamp(0.0, 1.0);
        let full = rest.len() as f32 * (ROW + ROW_GAP) - ROW_GAP;
        if t > 0.001 {
            list = list.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(ROW_GAP))
                    // Room for the rows' hover to reach the card's edges.
                    .mx(px(-8.0))
                    .px(px(8.0))
                    .h(px(full * t))
                    .opacity(t)
                    .overflow_hidden()
                    .children(rest),
            );
        }
        let more = self.contact.more.is_none();
        let person = email.to_owned();
        list.child(
            div().pt(px(4.0)).flex().flex_row().child(
                div()
                    .id("contact-more")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(2.0))
                    .mx(px(-8.0))
                    .pl(px(8.0))
                    .pr(px(4.0))
                    .h(px(28.0))
                    .rounded_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.accent))
                    .child(if more {
                        tr!("contact-more")
                    } else {
                        tr!("contact-less")
                    })
                    .child(icon(
                        if more { "chevron-down" } else { "chevron-up" },
                        th.accent,
                        18.0,
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.contact.more = more.then(|| person.clone());
                        cx.notify();
                    })),
            ),
        )
        .into_any_element()
    }

    /// Open tasks made from mail with them, due first first: a tick to
    /// complete one, and a click to open it on the Tasks page.
    fn contact_tasks(
        &self,
        mails: &[String],
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let tasks = self.tasks_of_mails(mails);
        if tasks.is_empty() {
            return None;
        }
        let today = super::tasks_page::today();
        let title = words(pieces, tr!("contact-tasks"));
        let rows: Vec<_> = tasks
            .into_iter()
            .map(|(task, done)| {
                let id = task.id;
                let due = super::tasks_page::due_label(task, today);
                row(("contact-task", id as usize), th)
                    .group("task-row")
                    .child(
                        div()
                            .id(("contact-task-tick", id as usize))
                            .flex_none()
                            .rounded_full()
                            .tooltip(tip(
                                if done {
                                    tr!("tasks-mark-open")
                                } else {
                                    tr!("tasks-mark-done")
                                },
                                th,
                            ))
                            .child(super::tasks_page::round_tick(done, true, th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                this.task_toggle_done(id, cx)
                            })),
                    )
                    .child(
                        words(pieces, task.title.clone())
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .cursor_pointer()
                            .text_color(rgba(if done { th.text_dim } else { th.text }))
                            .when(done, |d| d.line_through()),
                    )
                    .children(due.map(|(label, past)| {
                        words(pieces, label)
                            .flex_none()
                            .cursor_pointer()
                            .text_size(px(12.0))
                            .text_color(rgba(if past { th.error } else { th.text_dim }))
                    }))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.show_page(super::apps::App::Tasks, window, cx);
                        this.task_open_details(id, window, cx);
                    }))
            })
            .collect();
        Some(section(title, th).children(rows).into_any_element())
    }

    /// The next meetings with them; a click opens one in the Calendar.
    fn contact_meetings(
        &self,
        meetings: &[Occurrence],
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let tz = &self.tz;
        let today = jiff::Zoned::now().with_time_zone(tz.clone()).date();
        let rows: Vec<_> = meetings
            .iter()
            .map(|occurrence| {
                let data = &occurrence.event.data;
                let color = super::calendar::event_color(&self.calendar.calendars, occurrence);
                let start = super::calendar::civil(occurrence.start, tz);
                let when = if occurrence.all_day() {
                    katna_i18n::format::day_month(start)
                } else if start.date() == today {
                    katna_i18n::format::time(start)
                } else {
                    katna_i18n::format::day_month_time(start)
                };
                let title = if data.title.trim().is_empty() {
                    tr!("calendar-no-title")
                } else {
                    data.title.trim().to_owned()
                };
                let open = occurrence.clone();
                row(
                    SharedString::from(format!(
                        "contact-meeting-{}-{}",
                        occurrence.event.id, occurrence.start
                    )),
                    th,
                )
                .child(icon("event", color, 18.0))
                .child(
                    words(pieces, title)
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .cursor_pointer()
                        .text_color(rgba(th.text)),
                )
                .child(
                    words(pieces, when)
                        .flex_none()
                        .cursor_pointer()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim)),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.open_calendar_occurrence(open.clone(), window, cx)
                }))
            })
            .collect();
        section(words(pieces, tr!("contact-meetings")), th)
            .children(rows)
            .into_any_element()
    }

    fn contact_files(
        &self,
        files: &[ContactFile],
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        section(words(pieces, tr!("contact-files")), th)
            .children(files.iter().enumerate().map(|(ix, f)| {
                let file = RowFile {
                    message: f.message,
                    name: f.name.clone(),
                    mime: f.mime.clone(),
                    size: f.size,
                    nth: f.nth,
                    order: f.order,
                };
                let kind = katna_preview::kind(&f.mime, &f.name);
                // A file whose message isn't downloaded yet fills while it
                // downloads, as the list's attachment chips do.
                let fill = self.chip_fill(&file, false, th);
                row(("contact-file", ix), th)
                    .relative()
                    .overflow_hidden()
                    .children(fill)
                    .child(kind_badge(kind, 20.0))
                    .child(
                        words(pieces, f.name.clone())
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .cursor_pointer()
                            .text_color(rgba(th.text)),
                    )
                    .child(
                        words(pieces, format::size(f.size))
                            .flex_none()
                            .cursor_pointer()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim)),
                    )
                    .on_click(
                        cx.listener(move |this, _, window, cx| {
                            this.open_row_file(&file, window, cx)
                        }),
                    )
            }))
            .into_any_element()
    }

    /// The conversation's other people; a click shows one of them.
    fn contact_others(
        &self,
        shown: &str,
        people: &[(String, Option<String>)],
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let key = self.reader.as_ref().map(|r| r.key);
        section(words(pieces, tr!("contact-people")), th)
            .children(
                people
                    .iter()
                    .filter(|(email, _)| email != shown)
                    .take(8)
                    .enumerate()
                    .map(|(ix, (email, name))| {
                        let label = name.clone().unwrap_or_else(|| email.clone());
                        let pick = email.clone();
                        row(("contact-person", ix), th)
                            .child(self.person_avatar(&label, email, 24.0))
                            .child(
                                words(pieces, label)
                                    .min_w_0()
                                    .truncate()
                                    .cursor_pointer()
                                    .text_color(rgba(th.text)),
                            )
                            .children(self.muted_mark(email, 16.0, th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(key) = key {
                                    this.contact.picked = Some((key, pick.clone()));
                                }
                                cx.notify();
                            }))
                    }),
            )
            .into_any_element()
    }

    /// Opens a conversation from the panel: its line when the list shows
    /// it, else in the reader by itself.
    pub(super) fn open_contact_entry(
        &mut self,
        entry: Entry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ix) = self.entries.iter().position(|e| e.key == entry.key) {
            self.open(ix, window, cx);
            return;
        }
        if self.reader.as_ref().is_some_and(|r| r.key == entry.key) {
            return;
        }
        let Ok(mail) = &mut self.mail else {
            return;
        };
        let conversation = super::reader::Conversation::load(mail, entry.key);
        self.reader_scroll.set_offset(gpui::point(px(0.0), px(0.0)));
        self.reader = Some(conversation);
        self.read_timer = None;
        self.reading = true;
        cx.notify();
    }
}

/// The panel when the conversation names no one, not even the user.
fn contact_empty(th: &Theme) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(12.0))
        .p(px(24.0))
        .child(icon("contacts", th.text_faint, 40.0))
        .child(
            div()
                .text_size(px(14.0))
                .line_height(px(20.0))
                .text_color(rgba(th.text_dim))
                .text_center()
                .child(tr!("contact-nobody")),
        )
        .into_any_element()
}

/// A titled list in the panel.
fn section(title: gpui::Div, th: &Theme) -> gpui::Div {
    div().flex().flex_col().gap(px(ROW_GAP)).child(
        title
            .pb(px(6.0))
            .text_size(px(13.0))
            .line_height(px(18.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.text_dim)),
    )
}

/// `text` as a run of the card's selectable text, in a box to style.
fn words(pieces: &mut Pieces, text: impl Into<SharedString>) -> gpui::Div {
    let (styled, holder) = pieces.piece(text.into(), Vec::new());
    holder.child(styled)
}

/// A company as the daemon describes it
/// (`katna_sync::pictures::Company`).
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
struct Company {
    name: String,
    description: String,
    place: String,
    founded: Option<i32>,
    website: String,
    links: Vec<String>,
    summary: String,
    checked: i64,
}

/// A row of pages: a site the card has a mark for as a round chip with
/// Katna's one-colour mark, any other as a chip with its host.
fn page_chips(id: &'static str, pages: &[signature::Link], th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap(px(6.0))
        .children(pages.iter().enumerate().map(|(ix, link)| {
            let url = link.url.clone();
            let chip = div()
                .id((id, ix))
                .h(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(rgba(th.chip))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .tooltip(tip(link.url.clone(), th))
                .on_click(move |_, _, cx| cx.open_url(&url));
            match brand_icon(link.site) {
                Some(name) => chip.w(px(24.0)).child(icon(name, th.text_dim, 14.0)),
                None => chip
                    .px(px(10.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(page_label(&link.url)),
            }
        }))
        .into_any_element()
}

/// What a page with no mark of its own is called: Wikipedia, else its
/// host.
fn page_label(url: &str) -> String {
    let host = host_label(url);
    if host == "wikipedia.org" || host.ends_with(".wikipedia.org") {
        "Wikipedia".to_owned()
    } else {
        host
    }
}

/// Whether two phone numbers are one: the same last ten digits.
fn same_number(a: &str, b: &str) -> bool {
    let tail = |n: &str| {
        let digits: Vec<char> = n.chars().filter(char::is_ascii_digit).collect();
        digits[digits.len().saturating_sub(10)..]
            .iter()
            .collect::<String>()
    };
    let (a, b) = (tail(a), tail(b));
    a.len() >= 7 && a == b
}

/// Katna's one-colour mark for a site a signature links to.
fn brand_icon(site: signature::Site) -> Option<&'static str> {
    use signature::Site;
    Some(match site {
        Site::Web => return None,
        Site::LinkedIn => "brand-linkedin",
        Site::X => "brand-x",
        Site::Facebook => "brand-facebook",
        Site::Instagram => "brand-instagram",
        Site::YouTube => "brand-youtube",
        Site::GitHub => "brand-github",
        Site::WhatsApp => "brand-whatsapp",
        Site::Telegram => "brand-telegram",
    })
}

/// `example.com` from `https://www.example.com/`.
fn host_label(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let host = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    host.strip_prefix("www.").unwrap_or(host).to_owned()
}

/// `number` as a `tel:` URI wants it: its digits, after a `+` if it has one.
fn dialable(number: &str) -> String {
    let digits: String = number.chars().filter(char::is_ascii_digit).collect();
    if number.trim_start().starts_with('+') {
        format!("+{digits}")
    } else {
        digits
    }
}

/// The panel's cards: the card's own color with a faint touch of the
/// accent, in light and dark.
fn card_tint(th: &Theme) -> u32 {
    mix(
        th.surface,
        opaque_accent(th),
        if th.dark { 0.07 } else { 0.05 },
    )
}

/// The round buttons under the name, and under the pointer.
fn opaque_accent(th: &Theme) -> u32 {
    th.accent | 0xff
}

/// A clickable line of a section, reaching the card's edges on hover.
fn row(id: impl Into<gpui::ElementId>, th: &Theme) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .mx(px(-8.0))
        .px(px(8.0))
        .h(px(ROW))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(10.0))
        .rounded(px(8.0))
        .text_size(px(13.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
}

/// A stable number for an address, for its fade-in.
fn person_number(email: &str) -> usize {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    email.hash(&mut hasher);
    hasher.finish() as usize
}

/// 0 below 0, 1 above 1, and an S-curve between, so the bar comes in
/// gently and settles gently.
fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A scroll offset where GPUI draws it: on a whole device pixel, so the
/// bar moves with the content it sits over rather than a pixel off.
fn snap(offset: f32, device: f32) -> f32 {
    if device <= 0.0 {
        return offset;
    }
    let dev = offset * device;
    (dev.abs() - 0.5).ceil().copysign(dev) / device
}
