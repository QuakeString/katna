// SPDX-License-Identifier: GPL-3.0-or-later

//! The contact panel: a card beside the open conversation about one of its
//! people, the newest sender unless another is picked. It shows their
//! picture, name and address, their phone, title and company from their
//! signatures, their time of day, the mail exchanged with them, recent
//! conversations and files. Everything comes from the local mail
//! ([`crate::profile`]); nothing is looked up online.
//!
//! Desktop windows only, where the reader keeps room enough beside it; the
//! reader's toolbar shows and hides it.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    Animation, AnimationExt, AnyElement, ClipboardItem, Context, FontWeight, SharedString, Window,
    div, ease_out_quint, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_store::{ContactConversation, ContactFile};
use katna_ui::motion::{self, Spring};
use katna_ui::px;

use super::MailWindow;
use super::attachments::kind_badge;
use super::select::{Pieces, selectable};
use crate::data::{Entry, EntryKey, RowFile};
use crate::format;
use crate::profile::{self, Profile};
use crate::theme::Theme;
use crate::widgets::{card_outline, card_shadow, icon, icon_button_colored, tip};

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
/// The part the card's text is in the window's text selection: after the
/// conversation's messages, so a selection from one into the other keeps
/// their order.
const CONTACT_PART: usize = usize::MAX / 2;

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
}

impl ContactPanel {
    pub(super) fn new() -> Self {
        Self {
            spring: Spring::new(motion::SLIDE, 0.0),
            picked: None,
            people: None,
            profiles: HashMap::new(),
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

    /// Moves the panel toward shown or hidden for this frame, and returns
    /// the width it takes from cards `available` wide.
    pub(super) fn tick_contact(
        &mut self,
        available: f32,
        window: &Window,
        reduce: bool,
    ) -> (f32, f32) {
        let open = self.config.mail.contact_panel
            && self.reading
            && self.reader.is_some()
            && self.settings_page.is_none()
            && self.contact_fits(available);
        self.contact.spring.set(if open { 1.0 } else { 0.0 });
        let t = self.contact.spring.tick(window, reduce).clamp(0.0, 1.0);
        let room = |t: f32| (CONTACT_WIDTH + GAP) * t;
        (room(t), room(self.contact.spring.target()))
    }

    /// Shows or hides the panel, from the reader's toolbar.
    pub(super) fn toggle_contact_panel(&mut self, cx: &mut Context<Self>) {
        self.config.mail.contact_panel = !self.config.mail.contact_panel;
        self.save_config();
        cx.notify();
    }

    /// The reader toolbar's button for the panel, where it fits.
    pub(super) fn contact_toggle(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.contact_fits(self.cards_width + self.contact_room()) {
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
    fn contact_room(&self) -> f32 {
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
            cx.spawn(async move |this, cx| {
                let read = cx
                    .background_executor()
                    .spawn({
                        let address = address.clone();
                        async move { profile::read(&paths, &address) }
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

    fn render_contact_card(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (radius, outline) = (
            self.layout.shape.card_radius(),
            self.layout.shape.card_outline(),
        );
        let key = self.reader.as_ref().map(|r| r.key);
        let people = self.contact_people();
        let picked = self
            .contact
            .picked
            .as_ref()
            .filter(|(k, email)| Some(*k) == key && people.iter().any(|(e, _)| e == email))
            .map(|(_, email)| email.clone());
        let person = picked
            .and_then(|email| people.iter().find(|(e, _)| *e == email).cloned())
            .or_else(|| people.first().cloned());
        let body = match person {
            Some((email, name)) => {
                let profile = self.contact_profile(&email, cx);
                self.render_contact_body(&email, name.as_deref(), profile, &people, th, cx)
            }
            None => div().into_any_element(),
        };
        div()
            .id("contact-card")
            .size_full()
            .relative()
            .rounded(px(radius))
            .overflow_hidden()
            .bg(rgba(th.surface))
            .shadow(card_shadow(th, outline))
            .p(px(outline))
            .child(
                div()
                    .id("contact-scroll")
                    .size_full()
                    .overflow_y_scroll()
                    .child(body),
            )
            .children(card_outline(th, radius, outline))
            .into_any_element()
    }

    fn render_contact_body(
        &self,
        email: &str,
        name: Option<&str>,
        profile: Option<Rc<Profile>>,
        people: &[(String, Option<String>)],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let name = profile
            .as_ref()
            .and_then(|p| p.summary.name.clone())
            .or_else(|| name.map(str::to_owned));
        let shown_name = name.clone().unwrap_or_else(|| email.to_owned());
        // All of the card's text can be selected and copied, as the
        // conversation's can.
        let mut pieces = self.text.pieces(CONTACT_PART, th);
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(self.person_avatar(&shown_name, email, PICTURE))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        words(&mut pieces, shown_name.clone())
                            .text_size(px(16.0))
                            .line_height(px(22.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text)),
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

        let mut sections: Vec<AnyElement> = vec![header.into_any_element()];
        if let Some(profile) = &profile {
            if let Some(details) = self.contact_details(profile, &mut pieces, th, cx) {
                sections.push(details);
            }
            sections.push(self.contact_mail(profile, &mut pieces, th));
            if !profile.conversations.is_empty() {
                sections.push(self.contact_conversations(
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
        if people.len() > 1 {
            sections.push(self.contact_others(email, people, &mut pieces, th, cx));
        }
        if profile.is_some() {
            sections.push(
                words(&mut pieces, tr!("contact-local-only"))
                    .text_size(px(12.0))
                    .line_height(px(16.0))
                    .text_color(rgba(th.text_faint))
                    .into_any_element(),
            );
        }
        let mut body = div()
            .flex()
            .flex_col()
            .px(px(16.0))
            .pt(px(20.0))
            .pb(px(16.0));
        for (ix, section) in sections.into_iter().enumerate() {
            if ix > 0 {
                body = body.child(div().my(px(16.0)).h(px(1.0)).bg(rgba(th.divider)));
            }
            body = body.child(section);
        }
        // A new person fades in, as a conversation opens.
        selectable(body, Some(CONTACT_PART), cx)
            .with_animation(
                ("contact-person", person_number(email)),
                Animation::new(Duration::from_millis(220)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element()
    }

    /// Phone, title and company, and their time of day.
    fn contact_details(
        &self,
        profile: &Profile,
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let card = &profile.card;
        // The title, with the company under it in a quieter color.
        let work = card.title.as_ref().or(card.company.as_ref()).map(|first| {
            let under = card.title.as_ref().and(card.company.clone());
            div()
                .flex()
                .flex_col()
                .child(words(pieces, first.clone()))
                .children(under.map(|company| words(pieces, company).text_color(rgba(th.text_dim))))
                .into_any_element()
        });
        let phone = card
            .phone
            .clone()
            .map(|number| self.contact_phone(number, th, cx));
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
        let rows: Vec<(&str, AnyElement)> = [("work", work), ("phone", phone), ("schedule", time)]
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

    /// Their phone number: a click calls it (the desktop hands `tel:` to
    /// the phone app or KDE Connect), and a copy button shows on hover.
    fn contact_phone(&self, number: String, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let dial = format!("tel:{}", dialable(&number));
        let copied = number.clone();
        div()
            .id("contact-phone")
            .group("contact-phone")
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(
                div()
                    .id("contact-call")
                    .min_w_0()
                    .cursor_pointer()
                    .text_color(rgba(th.accent))
                    .hover(|s| s.underline())
                    .tooltip(tip(tr!("contact-call"), th))
                    .on_click(move |_, _, cx| cx.open_url(&dial))
                    .child(number),
            )
            .child(
                div()
                    .id("contact-copy-number")
                    .flex_none()
                    .size(px(24.0))
                    .my(px(-2.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .cursor_pointer()
                    .opacity(0.0)
                    .group_hover("contact-phone", |s| s.opacity(1.0))
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
        conversations: &[ContactConversation],
        pieces: &mut Pieces,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let now = format::local(jiff::Timestamp::now().as_second(), &self.tz);
        let open = self.reader.as_ref().map(|r| r.key);
        section(words(pieces, tr!("contact-conversations")), th)
            .children(conversations.iter().enumerate().map(|(ix, c)| {
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
            }))
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
                row(("contact-file", ix), th)
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
                                    .flex_1()
                                    .min_w_0()
                                    .truncate()
                                    .cursor_pointer()
                                    .text_color(rgba(th.text)),
                            )
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
    fn open_contact_entry(&mut self, entry: Entry, window: &mut Window, cx: &mut Context<Self>) {
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

/// A titled list in the panel.
fn section(title: gpui::Div, th: &Theme) -> gpui::Div {
    div().flex().flex_col().gap(px(2.0)).child(
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

/// `number` as a `tel:` URI wants it: its digits, after a `+` if it has one.
fn dialable(number: &str) -> String {
    let digits: String = number.chars().filter(char::is_ascii_digit).collect();
    if number.trim_start().starts_with('+') {
        format!("+{digits}")
    } else {
        digits
    }
}

/// A clickable line of a section, reaching the card's edges on hover.
fn row(id: impl Into<gpui::ElementId>, th: &Theme) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .mx(px(-8.0))
        .px(px(8.0))
        .h(px(32.0))
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
