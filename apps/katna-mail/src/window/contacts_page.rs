// SPDX-License-Identifier: GPL-3.0-or-later

//! The Contacts page (`docs/ARCHITECTURE.md` §8.6), laid out like Google
//! Contacts: a column with Contacts, Frequent and the labels; the saved
//! contacts of every account in a list (the same person saved in several
//! accounts once); and a contact's own page with its details, the accounts
//! that keep it, and buttons to write to them or find their mail.
//!
//! The daemon syncs the contacts; this page reads them from the store and
//! reads them again on `ContactsChanged`. Saved pictures also show beside
//! the person's mail.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use futures_lite::StreamExt;
use gpui::{
    AnimationExt, AnyElement, Context, Entity, FontWeight, RenderImage, Task, div, prelude::*,
    rgba, uniform_list,
};
use katna_core::contact::Card;
use katna_core::{Account, AccountKind, OAuthProvider};
use katna_i18n::tr;
use katna_store::{AddressBook, BookSource, BookState, SavedContact, StoredCard};
use katna_ui::{InputEvent, Ripple, TextInput, px};
use zbus::Connection;

use super::MailWindow;
use super::account_status::{AccountStatus, Of, Say};
use super::apps::App;
use super::contacts_edit::{Deleted, Editor, PendingDelete, visible};
use super::contacts_labels::{LabelDialog, LabelMenu};
use crate::daemon;
use crate::data::SavedBook;
use crate::theme::{Theme, mix};
use crate::widgets::{icon, placeholder, tip};

/// Width of the column with Contacts, Frequent and the labels.
const NAV_WIDTH: f32 = 248.0;
/// Height of a line of the list.
const ROW: f32 = 52.0;
/// The list shows its columns (email, phone, job, labels) from this wide;
/// narrower, each row is the name with the address under it.
const COLUMNS_FROM: f32 = 640.0;
/// What a round button on a contact's page does when clicked.
type OnClick = Box<dyn Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App)>;

/// Which people the page lists.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) enum View {
    #[default]
    Contacts,
    /// The people the mail was exchanged with most ([`super::apps`]).
    Frequent,
    /// Google's other contacts ([`super::contacts_other`]).
    Other,
    /// Suggested duplicates ([`super::contacts_merge`]).
    Merge,
    Label(String),
    /// The people saved in one account.
    Account(i64),
}

/// A line of the list: a heading or a person.
#[derive(Debug, Clone, Copy)]
enum Line {
    Heading(Heading, usize),
    Person(usize),
}

#[derive(Debug, Clone, Copy)]
enum Heading {
    Starred,
    Contacts,
}

/// The page's state, kept by the window.
#[derive(Default)]
pub(super) struct ContactsPage {
    /// `None` until first read.
    pub(super) book: Option<Result<Rc<SavedBook>, String>>,
    load: Option<Task<()>>,
    watch: Option<Task<()>>,
    pub(super) view: View,
    /// What the top bar's search box holds for contacts, and for mail
    /// while the page shows.
    query: String,
    mail_query: Option<String>,
    /// The person whose page is open, with their cards once read.
    pub(super) open: Option<Open>,
    /// Pictures of saved people by their first card, `None` while loading
    /// or when there is none; and by lower-case address, the key to use.
    photos: HashMap<i64, Option<Arc<RenderImage>>>,
    pub(super) by_email: HashMap<String, i64>,
    has_photo: BTreeSet<i64>,
    wanted: RefCell<BTreeSet<i64>>,
    /// The form of a contact being made or changed.
    pub(super) edit: Option<Editor>,
    /// People deleted whose Undo is still on screen, by first card.
    pub(super) hidden: BTreeSet<i64>,
    pub(super) pending_delete: Option<PendingDelete>,
    /// The last people deleted, for a late Undo.
    pub(super) deleted: Vec<Deleted>,
    /// The card just saved: its person opens once read.
    pub(super) open_after_load: Option<i64>,
    /// The labels menu, and the dialog naming a label.
    pub(super) label_menu: Option<LabelMenu>,
    pub(super) label_dialog: Option<LabelDialog>,
    /// Labels of people as just changed, by first card, until read back.
    pub(super) shown_labels: HashMap<i64, Vec<String>>,
    /// Addresses being added from Mail.
    pub(super) adding: BTreeSet<String>,
    /// Other contacts being saved, hidden meanwhile.
    pub(super) saving_others: BTreeSet<i64>,
    /// A person shown as a QR code.
    pub(super) qr: Option<super::contacts_share::QrShare>,
    /// Where each account's contacts sync stands.
    pub(super) accounts: AccountStatus,
}

pub(super) struct Open {
    pub(super) person: SavedContact,
    pub(super) cards: Option<Result<Vec<StoredCard>, String>>,
    _task: Task<()>,
}

impl MailWindow {
    /// Reads the saved contacts, and again whenever the daemon says they
    /// changed.
    pub(super) fn watch_contacts(&mut self, connection: Connection, cx: &mut Context<Self>) {
        self.load_contacts(cx);
        self.contacts.watch = Some(cx.spawn(async move |this, cx| {
            let Ok(mut changes) = daemon::contacts_changes(&connection).await else {
                return;
            };
            while changes.next().await.is_some() {
                // Pictures come a few at a time: one read for a burst.
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(400))
                    .await;
                while let Some(Some(())) = futures_lite::future::poll_once(changes.next()).await {}
                if this.update(cx, |this, cx| this.load_contacts(cx)).is_err() {
                    return;
                }
            }
        }));
    }

    pub(super) fn load_contacts(&mut self, cx: &mut Context<Self>) {
        self.load_account_status(Of::Contacts, cx);
        let paths = self.paths.clone();
        self.contacts.load = Some(cx.spawn(async move |this, cx| {
            let book = cx
                .background_executor()
                .spawn(async move {
                    let book = crate::data::saved_contacts(&paths)?;
                    let with_photos = crate::data::people_with_photos(&paths).unwrap_or_default();
                    Ok::<_, String>((book, with_photos))
                })
                .await;
            this.update(cx, |this, cx| {
                match book {
                    Ok((book, with_photos)) => {
                        let page = &mut this.contacts;
                        page.shown_labels.clear();
                        page.saving_others.clear();
                        page.by_email.clear();
                        for person in &book.people {
                            let Some(&first) = person.ids.first() else {
                                continue;
                            };
                            for email in &person.emails {
                                page.by_email.entry(email.clone()).or_insert(first);
                            }
                        }
                        // Pictures that changed load again.
                        page.photos.clear();
                        page.has_photo = book
                            .people
                            .iter()
                            .filter(|p| p.ids.iter().any(|id| with_photos.contains(id)))
                            .filter_map(|p| p.ids.first().copied())
                            .collect();
                        // A contact just saved opens.
                        if let Some(id) = page.open_after_load.take()
                            && let Some(person) =
                                book.people.iter().find(|p| p.ids.contains(&id)).cloned()
                        {
                            this.open_contact(person, cx);
                        } else if let Some(open) = &this.contacts.open {
                            let first = open.person.ids.first().copied();
                            let found = book
                                .people
                                .iter()
                                .find(|p| first.is_some_and(|f| p.ids.contains(&f)))
                                .cloned();
                            match found {
                                Some(person) => this.open_contact(person, cx),
                                None => this.contacts.open = None,
                            }
                        }
                        this.contacts.book = Some(Ok(Rc::new(book)));
                    }
                    // Before the daemon made the tables, the page waits.
                    Err(err) => {
                        if this.contacts.book.is_none() {
                            this.contacts.book = Some(Err(err));
                        }
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// The saved picture of the person at `email`, if they have one;
    /// asks for it while it is not read yet.
    pub(super) fn saved_photo(&self, email: &str) -> Option<Arc<RenderImage>> {
        let key = *self.contacts.by_email.get(&email.trim().to_lowercase())?;
        self.saved_photo_of(key)
    }

    /// Whether `email` is saved as a contact.
    pub(super) fn is_saved_contact(&self, email: &str) -> bool {
        self.contacts
            .by_email
            .contains_key(&email.trim().to_lowercase())
    }

    fn saved_photo_of(&self, key: i64) -> Option<Arc<RenderImage>> {
        if !self.contacts.has_photo.contains(&key) {
            return None;
        }
        match self.contacts.photos.get(&key) {
            Some(photo) => photo.clone(),
            None => {
                self.contacts.wanted.borrow_mut().insert(key);
                None
            }
        }
    }

    /// Reads the pictures asked for while drawing.
    pub(super) fn fetch_saved_photos(&mut self, cx: &mut Context<Self>) {
        let wanted = std::mem::take(&mut *self.contacts.wanted.borrow_mut());
        let Some(Ok(book)) = &self.contacts.book else {
            return;
        };
        for key in wanted {
            if self.contacts.photos.contains_key(&key) {
                continue;
            }
            self.contacts.photos.insert(key, None);
            let ids = book
                .people
                .iter()
                .find(|p| p.ids.first() == Some(&key))
                .map(|p| p.ids.clone())
                .unwrap_or_else(|| vec![key]);
            let paths = self.paths.clone();
            cx.spawn(async move |this, cx| {
                let photo = cx
                    .background_executor()
                    .spawn(async move {
                        crate::data::contact_photo(&paths, &ids)
                            .ok()
                            .flatten()
                            .and_then(super::remote::sender_logo)
                    })
                    .await;
                this.update(cx, |this, cx| {
                    if photo.is_some() {
                        this.contacts.photos.insert(key, photo);
                        cx.notify();
                    }
                })
                .ok();
            })
            .detach();
        }
    }

    fn contact_avatar(&self, person: &SavedContact, size: f32) -> AnyElement {
        let email = person.emails.first().cloned().unwrap_or_default();
        if let Some(photo) = person.ids.first().and_then(|&k| self.saved_photo_of(k)) {
            return super::remote::logo(photo, size);
        }
        self.person_avatar(&person.name, &email, size)
    }

    pub(super) fn open_saved_contact(&mut self, person: SavedContact, cx: &mut Context<Self>) {
        self.open_contact(person, cx);
    }

    fn open_contact(&mut self, person: SavedContact, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        let ids = person.ids.clone();
        let task = cx.spawn(async move |this, cx| {
            let cards = cx
                .background_executor()
                .spawn(async move { crate::data::saved_cards(&paths, &ids) })
                .await;
            this.update(cx, |this, cx| {
                if let Some(open) = &mut this.contacts.open {
                    open.cards = Some(cards);
                }
                cx.notify();
            })
            .ok();
        });
        let cards = self
            .contacts
            .open
            .take()
            .filter(|o| o.person.ids == person.ids)
            .and_then(|o| o.cards);
        self.contacts.open = Some(Open {
            person,
            cards,
            _task: task,
        });
        cx.notify();
    }

    fn close_contact(&mut self, cx: &mut Context<Self>) {
        self.contacts.open = None;
        self.contacts.edit = None;
        cx.notify();
    }

    pub(super) fn set_contacts_view(&mut self, view: View, cx: &mut Context<Self>) {
        self.contacts.view = view;
        self.contacts.open = None;
        self.contacts.edit = None;
        if self.contacts.view == View::Frequent
            && !matches!(self.people, Some(super::apps::People::Loaded(_)))
        {
            self.load_people(cx);
        }
        cx.notify();
    }

    /// Turns the top bar's search box to contacts while the page shows,
    /// and back to mail after, each keeping its own words.
    pub(super) fn swap_contacts_search(&mut self, entering: bool, cx: &mut Context<Self>) {
        let (placeholder, text) = if entering {
            self.contacts.mail_query = Some(self.search.read(cx).text().to_owned());
            (tr!("contacts-search"), self.contacts.query.clone())
        } else {
            let text = self.contacts.mail_query.take().unwrap_or_default();
            (tr!("search-mail"), text)
        };
        self.search.update(cx, |search, cx| {
            search.set_placeholder(placeholder);
            search.set_text(text, cx);
        });
    }

    /// The top bar's search box changed while the page shows.
    pub(super) fn on_contacts_search(
        &mut self,
        search: &Entity<TextInput>,
        event: &InputEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Changed => {
                self.contacts.query = search.read(cx).text().to_owned();
                self.contacts.open = None;
            }
            InputEvent::Cancel => {
                self.contacts.query.clear();
                search.update(cx, |search, cx| search.set_text("", cx));
            }
            InputEvent::Submit => {}
        }
        cx.notify();
    }

    /// Asks the account's provider again, allowing its contacts.
    fn allow_contacts(
        &mut self,
        account: Account,
        provider: OAuthProvider,
        cx: &mut Context<Self>,
    ) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                daemon::sign_in(&connection, provider, Some(account.id.0), &account.address)
                    .await
                    .map_err(|err| match err {
                        daemon::AddError::Password(e) | daemon::AddError::Other(e) => e,
                    })
            }
            .await;
            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    this.show_snackbar(err, None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// The Contacts page.
    pub(super) fn render_contacts_page(
        &self,
        th: &Theme,
        window: &gpui::Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let query = self.contacts.query.trim().to_lowercase();
        let book = match &self.contacts.book {
            Some(Ok(book)) => Some(book.clone()),
            _ => None,
        };
        let nav = self.contacts_nav(book.as_deref(), th, cx);
        let body = if let Some(editor) = self.render_contact_editor(th, window, cx) {
            editor
        } else if self.contacts.view == View::Frequent && query.is_empty() {
            self.render_contacts(th, cx)
        } else if let (View::Other, None, Some(book)) =
            (&self.contacts.view, &self.contacts.open, &book)
        {
            self.render_other_contacts(book, &query, th, cx)
        } else if let (View::Merge, None, Some(book)) =
            (&self.contacts.view, &self.contacts.open, &book)
        {
            self.render_merge_page(book, th, cx)
        } else if let Some(open) = &self.contacts.open {
            let person = open.person.clone();
            let cards = open.cards.clone();
            self.render_contact(&person, cards, book.as_deref(), th, cx)
        } else {
            match &self.contacts.book {
                None => placeholder(&tr!("contacts-loading"), th),
                Some(Err(_)) => placeholder(&tr!("contacts-loading"), th),
                Some(Ok(book)) => {
                    let book = book.clone();
                    self.contacts_list(&book, &query, th, cx)
                }
            }
        };
        let nav = self.page_side(nav, NAV_WIDTH, true, th, cx);
        div()
            .relative()
            .size_full()
            .flex()
            .flex_row()
            .children(nav.docked)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .pt(px(8.0))
                    .children(self.allow_banner(book.as_deref(), th, cx))
                    .child(div().flex_1().min_h_0().child(body)),
            )
            .children(nav.drawer)
            .children(self.render_label_menu(th, cx))
            .into_any_element()
    }

    /// Contacts, Frequent and the labels.
    fn contacts_nav(
        &self,
        book: Option<&SavedBook>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let count = book.map_or(0, |b| visible(&b.people, &self.contacts.hidden).len());
        let item = |id: (&'static str, usize),
                    glyph: &str,
                    label: String,
                    count: Option<usize>,
                    view: View,
                    cx: &mut Context<Self>| {
            let on = self.contacts.view == view;
            super::nav::side_row(id, glyph, label, on, th)
                .children(count.map(|n| {
                    div()
                        .flex_none()
                        .text_size(px(12.0))
                        .text_color(rgba(if on {
                            th.row_selected_text
                        } else {
                            th.text_faint
                        }))
                        .child(katna_i18n::format::number(n as u64))
                }))
                .on_click(
                    cx.listener(move |this, _, _, cx| this.set_contacts_view(view.clone(), cx)),
                )
        };
        // Shown for Google accounts: ones with other contacts, or asked to
        // allow them.
        let others = book
            .filter(|b| !b.others.is_empty() || !b.others_blocked.is_empty())
            .map(|b| self.other_count(b));
        // Counted in people, as the list shows them, not saved cards.
        let labels: Vec<(String, usize)> = book
            .map(|b| {
                b.labels
                    .iter()
                    .map(|l| {
                        let people = visible(&b.people, &self.contacts.hidden)
                            .into_iter()
                            .filter(|&ix| {
                                let p = &b.people[ix];
                                self.person_labels(p.ids.first().copied(), &p.labels)
                                    .contains(&l.name)
                            });
                        (l.name.clone(), people.count())
                    })
                    .collect()
            })
            .unwrap_or_default();
        div()
            .id("contacts-nav-column")
            .flex_none()
            .w(px(NAV_WIDTH))
            .h_full()
            .overflow_y_scroll()
            .pb(px(16.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(self.create_contact_button(th, cx))
            .child(item(
                ("contacts-nav", 0),
                "contacts",
                tr!("contacts-all"),
                Some(count),
                View::Contacts,
                cx,
            ))
            .child(item(
                ("contacts-nav", 1),
                "refresh",
                tr!("contacts-frequent"),
                None,
                View::Frequent,
                cx,
            ))
            .when(others.is_some(), |d| {
                d.child(item(
                    ("contacts-nav", 2),
                    "person-add",
                    tr!("contacts-other"),
                    others.filter(|&n| n > 0),
                    View::Other,
                    cx,
                ))
            })
            .when(!labels.is_empty(), |d| {
                d.child(
                    div()
                        .pt(px(18.0))
                        .pb(px(6.0))
                        .pl(px(20.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(th.text))
                        .child(tr!("contacts-labels")),
                )
            })
            .children(labels.into_iter().enumerate().map(|(ix, (name, n))| {
                let menu = name.clone();
                let more = name.clone();
                item(
                    ("contacts-label", ix),
                    "label",
                    name.clone(),
                    Some(n),
                    View::Label(name),
                    cx,
                )
                .group("contacts-label-item")
                .on_mouse_down(
                    gpui::MouseButton::Right,
                    cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                        this.open_label_menu(
                            LabelMenu::Label {
                                name: menu.clone(),
                                at: event.position,
                            },
                            cx,
                        )
                    }),
                )
                .child(
                    div()
                        .flex_none()
                        .invisible()
                        .group_hover("contacts-label-item", |s| s.visible())
                        .child(
                            crate::widgets::icon_button(
                                ("contacts-label-more", ix),
                                "more",
                                18.0,
                                th,
                            )
                            .size(px(28.0))
                            .tooltip(tip(tr!("contacts-label-options"), th))
                            .on_click(cx.listener(
                                move |this, event: &gpui::ClickEvent, _, cx| {
                                    cx.stop_propagation();
                                    this.open_label_menu(
                                        LabelMenu::Label {
                                            name: more.clone(),
                                            at: event.position(),
                                        },
                                        cx,
                                    )
                                },
                            )),
                        ),
                )
            }))
            .child(self.contacts_accounts_nav(book, th, cx))
            .child(self.contacts_manage_nav(book, th, cx))
            .into_any_element()
    }

    /// "Accounts" in the column: every mail account with how many people
    /// are saved in it, and why one shows none.
    pub(super) fn contacts_accounts_nav(
        &self,
        book: Option<&SavedBook>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // A mail archive on this computer has no address book.
        let accounts: Vec<(i64, String)> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail() && a.kind != AccountKind::Local)
            .map(|a| (a.id.0, a.address.clone()))
            .collect();
        let rows = accounts.into_iter().enumerate().map(|(ix, (id, address))| {
            let on = self.contacts.view == View::Account(id);
            let (people, books) = book.map_or((0, 0), |b| {
                let people = visible(&b.people, &self.contacts.hidden)
                    .into_iter()
                    .filter(|&i| {
                        b.people[i]
                            .accounts
                            .iter()
                            .any(|a| a.is_some_and(|a| a.0 == id))
                    })
                    .count();
                let books = b
                    .books
                    .iter()
                    .filter(|x| x.account.is_some_and(|a| a.0 == id))
                    .count();
                (people, books)
            });
            let row =
                div()
                    .id(("contacts-account", ix))
                    .relative()
                    .overflow_hidden()
                    .h(px(36.0))
                    .mr(px(12.0))
                    .pl(px(20.0))
                    .pr(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(14.0))
                    .rounded_r_full()
                    .cursor_pointer()
                    .when(on, |d| d.bg(rgba(th.row_selected)))
                    .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .child(Ripple::new(("contacts-account", ix), rgba(th.ripple)))
                    .child(icon(
                        "cloud",
                        if on {
                            th.row_selected_text
                        } else {
                            th.text_dim
                        },
                        20.0,
                    ))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(14.0))
                            .when(on, |d| d.font_weight(FontWeight::SEMIBOLD))
                            .text_color(rgba(if on { th.row_selected_text } else { th.text }))
                            .child(address),
                    )
                    .when(people > 0, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .text_size(px(12.0))
                                .text_color(rgba(if on {
                                    th.row_selected_text
                                } else {
                                    th.text_faint
                                }))
                                .child(katna_i18n::format::number(people as u64)),
                        )
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.set_contacts_view(View::Account(id), cx)
                    }));
            div()
                .flex()
                .flex_col()
                .child(row)
                // Nothing yet, or old people that no longer sync: say why.
                .when(books == 0 || self.contacts.accounts.failing(id), |d| {
                    d.child(self.render_account_status(Of::Contacts, id, th, cx))
                })
        });
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(
                div()
                    .pt(px(18.0))
                    .pb(px(6.0))
                    .pl(px(20.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text))
                    .child(tr!("contacts-accounts")),
            )
            .children(rows)
            .into_any_element()
    }

    /// "Create contact", at the top of the column like Compose in Mail.
    fn create_contact_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .child(
                super::nav::side_create_button(
                    "contact-create",
                    "person-add",
                    tr!("contacts-create"),
                    th,
                )
                .on_click(cx.listener(|this, _, window, cx| {
                    this.contacts.open = None;
                    this.start_contact_edit(None, window, cx)
                })),
            )
            .into_any_element()
    }

    /// Asks to allow contacts for accounts signed in before Katna asked.
    fn allow_banner(
        &self,
        book: Option<&SavedBook>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let books: Vec<&AddressBook> = book?
            .books
            .iter()
            .filter(|b| b.state == BookState::NeedsPermission)
            .collect();
        let first = books.first()?;
        let account = self
            .accounts
            .iter()
            .find(|a| Some(a.id) == first.account)?
            .clone();
        let provider = match first.source {
            BookSource::Google => OAuthProvider::Google,
            BookSource::Microsoft => OAuthProvider::Microsoft,
            _ => return None,
        };
        let text = if books.len() > 1 {
            tr!(
                "contacts-allow-many",
                address = account.address.clone(),
                more = books.len() - 1
            )
        } else {
            tr!("contacts-allow", address = account.address.clone())
        };
        Some(
            div()
                .flex_none()
                .mx(px(16.0))
                .mb(px(8.0))
                .px(px(16.0))
                .py(px(10.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .rounded(px(14.0))
                .bg(rgba(mix(th.surface, th.accent | 0xff, 0.10)))
                .child(icon("info", th.accent, 20.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_size(px(14.0))
                        .text_color(rgba(th.text))
                        .child(text),
                )
                .child(
                    div()
                        .id("contacts-allow")
                        .relative()
                        .overflow_hidden()
                        .flex_none()
                        .px(px(16.0))
                        .h(px(36.0))
                        .flex()
                        .items_center()
                        .rounded_full()
                        .cursor_pointer()
                        .bg(rgba(th.accent))
                        .text_color(rgba(th.on_accent))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(Ripple::new(("contacts-allow", 0usize), rgba(th.ripple)))
                        .child(tr!("contacts-allow-button"))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.allow_contacts(account.clone(), provider, cx)
                        })),
                )
                .into_any_element(),
        )
    }

    /// Whether `person` is listed in the view: on its label, or saved in
    /// its account.
    pub(super) fn in_view(&self, person: &SavedContact) -> bool {
        match &self.contacts.view {
            View::Label(name) => self
                .person_labels(person.ids.first().copied(), &person.labels)
                .contains(name),
            View::Account(id) => person
                .accounts
                .iter()
                .any(|a| a.is_some_and(|a| a.0 == *id)),
            _ => true,
        }
    }

    /// The people of the view that match `query`, starred first.
    fn contacts_list(
        &self,
        book: &Rc<SavedBook>,
        query: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let shown: Vec<usize> = visible(&book.people, &self.contacts.hidden)
            .into_iter()
            .filter(|&ix| {
                let p = &book.people[ix];
                self.in_view(p) && (query.is_empty() || matches(p, query))
            })
            .collect();
        if shown.is_empty() {
            let text = if !query.is_empty() {
                tr!("contacts-none-found")
            } else if book.books.is_empty() {
                tr!("contacts-empty-no-books")
            } else {
                tr!("contacts-empty")
            };
            return placeholder(&text, th);
        }
        let starred: Vec<usize> = shown
            .iter()
            .copied()
            .filter(|&ix| book.people[ix].starred)
            .collect();
        let mut lines = Vec::new();
        if !starred.is_empty() && query.is_empty() {
            lines.push(Line::Heading(Heading::Starred, starred.len()));
            lines.extend(starred.iter().map(|&ix| Line::Person(ix)));
            lines.push(Line::Heading(Heading::Contacts, shown.len()));
        } else {
            lines.push(Line::Heading(Heading::Contacts, shown.len()));
        }
        lines.extend(shown.iter().map(|&ix| Line::Person(ix)));
        let lines = Rc::new(lines);
        let book = book.clone();
        let columns = self.contacts_columns();
        let header = columns.then(|| {
            div()
                .flex_none()
                .h(px(36.0))
                .mx(px(16.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(div().w(px(36.0)).flex_none())
                .child(column(2.0).child(tr!("contacts-col-name")))
                .child(column(2.4).child(tr!("contacts-col-email")))
                .child(column(1.6).child(tr!("contacts-col-phone")))
                .child(column(2.0).child(tr!("contacts-col-job")))
                .child(column(1.6).child(tr!("contacts-col-labels")))
        });
        let count = lines.len();
        let list = uniform_list(
            "contacts",
            count,
            cx.processor(move |this, range: Range<usize>, window, cx| {
                let th = this.theme(window);
                let rows = range
                    .map(|ix| match lines[ix] {
                        Line::Heading(heading, n) => heading_row(ix, heading, n, &th),
                        Line::Person(p) => this.contact_row(ix, &book.people[p], &th, cx),
                    })
                    .collect::<Vec<_>>();
                this.fetch_saved_photos(cx);
                this.fetch_pictures(cx);
                rows
            }),
        )
        .size_full();
        let label = match &self.contacts.view {
            View::Label(name) => Some(name.clone()),
            _ => None,
        };
        // An account's people are headed by its address.
        let account = match &self.contacts.view {
            View::Account(id) => self
                .accounts
                .iter()
                .find(|a| a.id.0 == *id)
                .map(|a| a.address.clone()),
            _ => None,
        };
        let account_title = account.map(|address| {
            div()
                .flex_none()
                .h(px(48.0))
                .mx(px(16.0))
                .px(px(8.0))
                .flex()
                .items_center()
                .text_size(px(20.0))
                .text_color(rgba(th.text))
                .truncate()
                .child(address)
        });
        let title = label.map(|name| {
            let email = name.clone();
            div()
                .flex_none()
                .h(px(48.0))
                .mx(px(16.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_size(px(20.0))
                        .text_color(rgba(th.text))
                        .child(name.clone()),
                )
                .child(
                    crate::widgets::icon_button("contacts-label-email", "mail", 20.0, th)
                        .tooltip(tip(tr!("contacts-label-email"), th))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.email_label(&email, window, cx)
                        })),
                )
                .child(
                    crate::widgets::icon_button("contacts-label-title-more", "more", 20.0, th)
                        .tooltip(tip(tr!("contacts-label-options"), th))
                        .on_click(cx.listener(move |this, event: &gpui::ClickEvent, _, cx| {
                            this.open_label_menu(
                                LabelMenu::Label {
                                    name: name.clone(),
                                    at: event.position(),
                                },
                                cx,
                            )
                        })),
                )
        });
        div()
            .size_full()
            .flex()
            .flex_col()
            .children(title)
            .children(account_title)
            .children(header)
            .child(div().flex_1().min_h_0().child(list))
            .into_any_element()
    }

    /// Whether the list has room for its columns beside the names.
    fn contacts_columns(&self) -> bool {
        let shape = self.layout.shape;
        let side = self.page_side_width(NAV_WIDTH);
        shape.width - shape.rail() - side >= COLUMNS_FROM
    }

    fn contact_row(
        &self,
        ix: usize,
        person: &SavedContact,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let open = person.clone();
        let dim = |d: gpui::Div| d.text_color(rgba(th.text_dim));
        let row = div()
            .id(("contact", ix))
            .w_full()
            .h(px(ROW))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .rounded(px(8.0))
            .cursor_pointer()
            .text_size(px(14.0))
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| this.open_contact(open.clone(), cx)))
            .child(
                div()
                    .w(px(36.0))
                    .flex_none()
                    .child(self.contact_avatar(person, 36.0)),
            )
            .when(!self.contacts_columns(), |row| {
                let under = person
                    .emails
                    .first()
                    .filter(|e| !e.is_empty())
                    .cloned()
                    .unwrap_or_else(|| person.phone.clone());
                row.child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .truncate()
                                .text_color(rgba(th.text))
                                .font_weight(FontWeight::MEDIUM)
                                .child(person.name.clone()),
                        )
                        .when(!under.is_empty(), |d| {
                            d.child(
                                div()
                                    .truncate()
                                    .text_size(px(12.0))
                                    .text_color(rgba(th.text_dim))
                                    .child(under),
                            )
                        }),
                )
            })
            .when(self.contacts_columns(), |row| {
                row.child(
                    column(2.0)
                        .text_color(rgba(th.text))
                        .font_weight(FontWeight::MEDIUM)
                        .child(person.name.clone()),
                )
                .child(dim(column(2.4)).child(person.emails.first().cloned().unwrap_or_default()))
                .child(dim(column(1.6)).child(person.phone.clone()))
                .child(dim(column(2.0)).child(person.job.clone()))
                .child(
                    column(1.6).flex().flex_row().gap(px(4.0)).children(
                        self.person_labels(person.ids.first().copied(), &person.labels)
                            .iter()
                            .take(2)
                            .map(|l| chip(l, th)),
                    ),
                )
            });
        div().w_full().px(px(16.0)).child(row).into_any_element()
    }

    /// A person's own page.
    fn render_contact(
        &self,
        person: &SavedContact,
        cards: Option<Result<Vec<StoredCard>, String>>,
        book: Option<&SavedBook>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let cards = match cards {
            None => return placeholder(&tr!("contacts-loading"), th),
            Some(Err(err)) => return placeholder(&err, th),
            Some(Ok(cards)) => cards,
        };
        let merged = merge(&cards);
        let email = person.emails.first().cloned();
        let phone = merged.phones.first().map(|p| p.value.clone());
        let back = div()
            .id("contact-back")
            .relative()
            .overflow_hidden()
            .size(px(40.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(tip(tr!("contacts-back"), th))
            .child(Ripple::new(("contact-back", 0usize), rgba(th.ripple)).centered())
            .child(icon("back", th.text_dim, 20.0))
            .on_click(cx.listener(|this, _, _, cx| this.close_contact(cx)));
        let job = merged.job();
        let head = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(28.0))
            .pb(px(20.0))
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(self.contact_avatar(person, 128.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(28.0))
                            .text_color(rgba(th.text))
                            .child(person.name.clone()),
                    )
                    .when(!job.is_empty(), |d| {
                        d.child(
                            div()
                                .text_size(px(15.0))
                                .text_color(rgba(th.text_dim))
                                .child(job),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap(px(6.0))
                            .children(
                                self.person_labels(person.ids.first().copied(), &person.labels)
                                    .iter()
                                    .map(|l| chip(l, th)),
                            )
                            .child(label_button(th, cx)),
                    )
                    .child(self.contact_buttons(email, phone, th, cx)),
            );
        let mut details: Vec<AnyElement> = Vec::new();
        for e in &merged.emails {
            details.push(fact("mail", e.value.clone(), kind_label(&e.kind), th));
        }
        for p in &merged.phones {
            details.push(fact("phone", p.value.clone(), kind_label(&p.kind), th));
        }
        for a in &merged.addresses {
            details.push(fact(
                "location",
                a.lines().join("\n"),
                kind_label(&a.kind),
                th,
            ));
        }
        if let Some(day) = birthday(&merged.birthday) {
            details.push(fact("cake", day, tr!("contacts-birthday"), th));
        }
        for u in &merged.urls {
            details.push(fact("link", u.value.clone(), kind_label(&u.kind), th));
        }
        if !merged.nickname.is_empty() {
            details.push(fact(
                "contacts",
                merged.nickname.clone(),
                tr!("contacts-nickname"),
                th,
            ));
        }
        let accounts: Vec<AnyElement> = cards
            .iter()
            .map(|c| {
                let account = c
                    .account
                    .and_then(|id| self.accounts.iter().find(|a| a.id == id));
                let (address, service) = match account {
                    Some(a) => (a.address.clone(), source_name(c.source)),
                    None => (tr!("contacts-this-computer"), String::new()),
                };
                let book_name = book
                    .and_then(|b| b.books.iter().find(|x| x.id == c.book))
                    .map(|b| b.name.clone())
                    .filter(|n| !n.is_empty());
                let under = [Some(service), book_name]
                    .into_iter()
                    .flatten()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" · ");
                fact("cloud", address, under, th)
            })
            .collect();
        let mut sections = vec![section(tr!("contacts-details"), details, 0, th)];
        sections.push(section(tr!("contacts-saved-in"), accounts, 1, th));
        if !merged.note.is_empty() {
            sections.push(section(
                tr!("contacts-notes"),
                vec![
                    div()
                        .text_size(px(14.0))
                        .line_height(px(21.0))
                        .text_color(rgba(th.text))
                        .child(merged.note.clone())
                        .into_any_element(),
                ],
                2,
                th,
            ));
        }
        div()
            .id("contact-page")
            .size_full()
            .overflow_y_scroll()
            .px(px(24.0))
            .pb(px(24.0))
            .child(
                div()
                    .pb(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(back)
                    .child(div().flex_1())
                    .child(
                        crate::widgets::icon_button("contact-qr", "qr-code", 20.0, th)
                            .tooltip(tip(tr!("contacts-qr"), th))
                            .on_click(cx.listener({
                                let (name, card) = (person.name.clone(), merged.clone());
                                move |this, _, window, cx| {
                                    window.focus(&this.dialog_focus, cx);
                                    this.open_contact_qr(name.clone(), &card, cx)
                                }
                            })),
                    )
                    .child(
                        crate::widgets::icon_button("contact-print", "print", 20.0, th)
                            .tooltip(tip(tr!("contacts-print"), th))
                            .on_click(cx.listener({
                                let (name, ids) = (person.name.clone(), person.ids.clone());
                                move |this, _, window, cx| {
                                    this.print_contacts(name.clone(), vec![ids.clone()], window, cx)
                                }
                            })),
                    )
                    .child(
                        crate::widgets::icon_button("contact-delete", "trash", 20.0, th)
                            .tooltip(tip(tr!("contacts-delete"), th))
                            .on_click(cx.listener({
                                let person = person.clone();
                                let cards = cards.clone();
                                move |this, _, _, cx| this.delete_contact(&person, &cards, cx)
                            })),
                    )
                    .child(
                        crate::widgets::outlined_button("contact-edit", tr!("contacts-edit"), th)
                            .on_click(cx.listener({
                                let card = cards.first().cloned();
                                move |this, _, window, cx| {
                                    this.start_contact_edit(card.clone(), window, cx)
                                }
                            })),
                    ),
            )
            .child(head)
            .child(
                div()
                    .pt(px(20.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(16.0))
                    .children(sections),
            )
            .with_animation(
                (
                    "contact-page",
                    person.ids.first().copied().unwrap_or(0) as usize,
                ),
                gpui::Animation::new(std::time::Duration::from_millis(220))
                    .with_easing(gpui::ease_out_quint()),
                |el, t| el.opacity(t).mt(px(10.0 * (1.0 - t))),
            )
            .into_any_element()
    }

    /// Write to them, find their mail, call them.
    fn contact_buttons(
        &self,
        email: Option<String>,
        phone: Option<String>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let tint = mix(
            th.surface,
            th.accent | 0xff,
            if th.dark { 0.16 } else { 0.17 },
        );
        let tint_hover = mix(
            th.surface,
            th.accent | 0xff,
            if th.dark { 0.24 } else { 0.25 },
        );
        let button = |id: &'static str, glyph: &str, label: String, click: OnClick| {
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .id(id)
                        .relative()
                        .overflow_hidden()
                        .size(px(40.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .cursor_pointer()
                        .bg(rgba(tint))
                        .hover(move |s| s.bg(rgba(tint_hover)))
                        .on_click(click)
                        .child(Ripple::new((id, 0usize), rgba(th.ripple)).centered())
                        .child(icon(glyph, th.accent, 20.0)),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
        };
        let mut row = div().pt(px(8.0)).flex().flex_row().gap(px(20.0));
        if let Some(email) = email {
            let to = email.clone();
            row = row.child(button(
                "contact-page-email",
                "mail",
                tr!("contact-email"),
                Box::new(cx.listener(move |this, _, window, cx| {
                    this.open_app(App::Mail, cx);
                    let mail = crate::mailto::Mailto {
                        to: vec![to.clone()],
                        ..Default::default()
                    };
                    this.open_mailto(mail, window, cx);
                })),
            ));
            let query = format!("from:{email} OR to:{email}");
            row = row.child(button(
                "contact-page-search",
                "search",
                tr!("contacts-find-mail"),
                Box::new(cx.listener(move |this, _, window, cx| {
                    this.open_app(App::Mail, cx);
                    this.search_for(query.clone(), window, cx);
                })),
            ));
        }
        if let Some(number) = phone {
            let dial = format!(
                "tel:{}",
                number
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '+')
                    .collect::<String>()
            );
            row = row.child(button(
                "contact-page-call",
                "phone",
                tr!("contact-call"),
                Box::new(move |_, _, cx| cx.open_url(&dial)),
            ));
        }
        row.into_any_element()
    }
}

/// Whether `person` matches the lower-case `query`: any word of the name,
/// an address, the phone or the job.
/// What the line under an account in the column says.
pub(super) fn say(say: Say<'_>) -> String {
    match say {
        Say::SignIn => tr!("contacts-account-sign-in"),
        Say::SignInRefused { provider } => {
            tr!("contacts-account-sign-in-refused", provider = provider)
        }
        Say::SignedIn { address } => tr!("contacts-account-signed-in", address = address),
        Say::Refused => tr!("contacts-account-password"),
        Say::ChangePassword => tr!("contacts-account-change-password"),
        Say::ChangePasswordTooltip => tr!("contacts-account-change-password-tooltip"),
        // The contacts sync never reports an API switched off.
        Say::NotEnabled | Say::Failed => tr!("contacts-account-failed"),
        Say::Error { reason } => tr!("contacts-account-error", reason = reason),
        Say::None => tr!("contacts-account-none"),
        Say::NoneWhy { reason } => tr!("contacts-account-none-why", reason = reason),
        Say::UseSignIn { provider } => tr!("contacts-account-use-sign-in", provider = provider),
        Say::SignInWith { provider } => tr!("contacts-account-sign-in-with", provider = provider),
        Say::Looking => tr!("contacts-account-looking"),
        Say::TryAgain => tr!("contacts-account-try-again"),
        Say::TryAgainTooltip => tr!("contacts-account-try-again-tooltip"),
        Say::Fixing => tr!("contacts-account-fixing"),
    }
}

fn matches(person: &SavedContact, query: &str) -> bool {
    let name = person.name.to_lowercase();
    name.starts_with(query)
        || name.split_whitespace().any(|w| w.starts_with(query))
        || person.emails.iter().any(|e| e.contains(query))
        || (!person.phone.is_empty()
            && query.chars().any(|c| c.is_ascii_digit())
            && digits(&person.phone).contains(&digits(query)))
        || person.job.to_lowercase().contains(query)
}

fn digits(text: &str) -> String {
    text.chars().filter(char::is_ascii_digit).collect()
}

/// A column of the list, `weight` wide.
fn column(weight: f32) -> gpui::Div {
    div()
        .flex_basis(px(0.0))
        .flex_grow(weight)
        .min_w_0()
        .truncate()
}

fn heading_row(ix: usize, heading: Heading, count: usize, th: &Theme) -> AnyElement {
    let text = match heading {
        Heading::Starred => tr!("contacts-starred", count = count),
        Heading::Contacts => tr!("contacts-count", count = count),
    };
    div()
        .id(("contact-heading", ix))
        .w_full()
        .h(px(ROW))
        .px(px(24.0))
        .pt(px(16.0))
        .flex()
        .items_center()
        .text_size(px(12.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgba(th.text_dim))
        .child(text)
        .into_any_element()
}

fn chip(label: &str, th: &Theme) -> AnyElement {
    div()
        .flex_none()
        .max_w(px(140.0))
        .truncate()
        .px(px(8.0))
        .py(px(1.0))
        .rounded_full()
        .bg(rgba(th.chip))
        .text_size(px(12.0))
        .text_color(rgba(th.text_dim))
        .child(label.to_owned())
        .into_any_element()
}

/// "Label", after a person's labels: opens the menu that ticks them.
fn label_button(th: &Theme, cx: &mut Context<MailWindow>) -> AnyElement {
    div()
        .id("contact-labels")
        .flex_none()
        .h(px(24.0))
        .pl(px(6.0))
        .pr(px(10.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(4.0))
        .rounded_full()
        .border_1()
        .border_color(rgba(th.divider))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .text_size(px(12.0))
        .text_color(rgba(th.text_dim))
        .child(icon("add", th.text_dim, 16.0))
        .child(tr!("contacts-label-button"))
        .on_click(cx.listener(|this, event: &gpui::ClickEvent, _, cx| {
            this.open_label_menu(
                LabelMenu::Person {
                    at: event.position(),
                },
                cx,
            )
        }))
        .into_any_element()
}

/// One detail: its icon, the value, and a quiet line under it.
fn fact(glyph: &str, value: String, under: String, th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(14.0))
        .child(div().pt(px(1.0)).child(icon(glyph, th.text_faint, 20.0)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(px(14.0))
                        .line_height(px(20.0))
                        .text_color(rgba(th.text))
                        .children(value.lines().map(|l| div().child(l.to_owned()))),
                )
                .when(!under.is_empty(), |d| {
                    d.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(under),
                    )
                }),
        )
        .into_any_element()
}

/// A faint tinted card with a heading, like the contact panel's.
fn section(title: String, rows: Vec<AnyElement>, tint: usize, th: &Theme) -> AnyElement {
    let hues = [th.accent | 0xff, 0x188038ff, 0xe37400ff];
    let hue = hues[tint % hues.len()];
    let top = mix(th.surface, hue, if th.dark { 0.10 } else { 0.07 });
    div()
        .flex_basis(px(360.0))
        .flex_grow(1.0)
        .min_w(px(280.0))
        .p(px(16.0))
        .flex()
        .flex_col()
        .gap(px(14.0))
        .rounded(px(14.0))
        .border_1()
        .border_color(rgba(th.divider))
        .bg(gpui::linear_gradient(
            180.0,
            gpui::linear_color_stop(rgba(top), 0.0),
            gpui::linear_color_stop(rgba(th.surface), 0.35),
        ))
        .child(
            div()
                .text_size(px(15.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(th.text))
                .child(title),
        )
        .children(rows)
        .into_any_element()
}

/// One card from several: the first name, every address and number once.
pub(super) fn merge(cards: &[StoredCard]) -> Card {
    let mut out = cards.first().map(|c| c.card.clone()).unwrap_or_default();
    for other in cards.iter().skip(1).map(|c| &c.card) {
        for e in &other.emails {
            if !out
                .emails
                .iter()
                .any(|x| x.value.eq_ignore_ascii_case(&e.value))
            {
                out.emails.push(e.clone());
            }
        }
        for p in &other.phones {
            if !out
                .phones
                .iter()
                .any(|x| digits(&x.value) == digits(&p.value))
            {
                out.phones.push(p.clone());
            }
        }
        for a in &other.addresses {
            if !out.addresses.contains(a) {
                out.addresses.push(a.clone());
            }
        }
        for u in &other.urls {
            if !out.urls.iter().any(|x| x.value == u.value) {
                out.urls.push(u.clone());
            }
        }
        let fill = |mine: &mut String, theirs: &String| {
            if mine.is_empty() {
                mine.clone_from(theirs);
            }
        };
        fill(&mut out.title, &other.title);
        fill(&mut out.organization, &other.organization);
        fill(&mut out.birthday, &other.birthday);
        fill(&mut out.nickname, &other.nickname);
        fill(&mut out.note, &other.note);
    }
    out
}

pub(super) fn kind_label(kind: &str) -> String {
    match kind {
        "home" => tr!("contacts-kind-home"),
        "work" => tr!("contacts-kind-work"),
        "mobile" => tr!("contacts-kind-mobile"),
        "other" => tr!("contacts-kind-other"),
        "" => String::new(),
        other => {
            let mut c = other.chars();
            c.next()
                .map(|f| f.to_uppercase().chain(c).collect())
                .unwrap_or_default()
        }
    }
}

fn source_name(source: BookSource) -> String {
    match source {
        BookSource::Google => tr!("contacts-source-google"),
        BookSource::Microsoft => tr!("contacts-source-microsoft"),
        BookSource::CardDav => tr!("contacts-source-carddav"),
        BookSource::Local => String::new(),
    }
}

/// `YYYY-MM-DD` or `--MM-DD` in the user's language.
pub(super) fn birthday(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    let parse = |y: &str, m: &str, d: &str| -> Option<jiff::civil::DateTime> {
        jiff::civil::Date::new(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
            .ok()
            .map(|d| d.at(0, 0, 0, 0))
    };
    if let Some(rest) = value.strip_prefix("--") {
        let (m, d) = rest.split_once('-')?;
        return parse("2000", m, d).map(katna_i18n::format::day_month);
    }
    let mut parts = value.splitn(3, '-');
    let date = match (parts.next(), parts.next(), parts.next()) {
        (Some(y), Some(m), Some(d)) => parse(y, m, d),
        _ => None,
    };
    Some(date.map_or_else(|| value.to_owned(), katna_i18n::format::day_month_year))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn person(name: &str, email: &str, phone: &str) -> SavedContact {
        SavedContact {
            ids: vec![1],
            name: name.into(),
            emails: vec![email.into()],
            phone: phone.into(),
            ..SavedContact::default()
        }
    }

    #[test]
    fn search_finds_words_addresses_and_numbers() {
        let arjun = person("Arjun Mehta", "arjun@acme.co", "+91 99000 55120");
        assert!(matches(&arjun, "meh"));
        assert!(matches(&arjun, "acme"));
        assert!(matches(&arjun, "99000"));
        assert!(matches(&arjun, "9900055"));
        assert!(!matches(&arjun, "rjun m"));
        assert!(!matches(&arjun, "zed"));
    }

    #[test]
    fn birthdays_read_with_and_without_a_year() {
        assert!(birthday("").is_none());
        assert!(birthday("--03-14").is_some());
        assert!(birthday("1985-03-14").is_some());
        assert_eq!(birthday("sometime").as_deref(), Some("sometime"));
    }
}
