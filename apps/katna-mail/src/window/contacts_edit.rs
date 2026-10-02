// SPDX-License-Identifier: GPL-3.0-or-later

//! Creating, changing and deleting contacts on the Contacts page, as in
//! Google Contacts: a form with the name, work, addresses and numbers; a
//! "Save to" choice for a new contact; and Delete with Undo.
//!
//! The daemon writes each change to the account's own service first
//! (Google, Microsoft, CardDAV), then to `pim.db`. A delete waits until
//! its Undo is gone, so Undo only has to show the person again; once sent,
//! Undo saves the card again.

use std::collections::BTreeSet;
use std::time::Duration;

use gpui::{
    AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, Subscription, Task, Window,
    div, prelude::*, rgba,
};
use katna_core::contact::{Card, PostalAddress, Typed};
use katna_i18n::tr;
use katna_store::{BookSource, BookState, SavedContact, StoredCard};
use katna_ui::{InputEvent, Ripple, TextInput, px};

use super::MailWindow;
use crate::daemon::{self, Command};
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, icon_button, outlined_button, tip};

/// How long a delete waits for Undo before it goes to the account.
const DELETE_AFTER: Duration = Duration::from_secs(8);

/// The form of a contact being made or changed.
pub(super) struct Editor {
    /// The card changed, 0 for a new contact.
    card: i64,
    /// Where a new contact goes: an address book, 0 for this computer.
    book: i64,
    /// The books a new contact can go to, with how each is named.
    books: Vec<(i64, String)>,
    /// The card as it was: what the form does not show is kept.
    base: Card,
    given: Entity<TextInput>,
    family: Entity<TextInput>,
    company: Entity<TextInput>,
    job: Entity<TextInput>,
    emails: Vec<Row>,
    phones: Vec<Row>,
    street: Entity<TextInput>,
    city: Entity<TextInput>,
    postcode: Entity<TextInput>,
    country: Entity<TextInput>,
    birthday: Entity<TextInput>,
    note: Entity<TextInput>,
    saving: bool,
    error: Option<String>,
    subscriptions: Vec<Subscription>,
    _task: Option<Task<()>>,
}

/// An address or number with its kind (home, work, mobile).
struct Row {
    kind: String,
    input: Entity<TextInput>,
}

/// People deleted on the page, waiting for their Undo to go.
pub(super) struct PendingDelete {
    /// The people, by their first card.
    keys: Vec<i64>,
    /// Every card to delete.
    cards: Vec<i64>,
    _task: Task<()>,
}

/// A deleted person's cards, to save again if Undo comes after the
/// delete went out.
#[derive(Clone)]
pub(super) struct Deleted {
    key: i64,
    cards: Vec<(i64, Card)>,
}

impl MailWindow {
    /// Opens the form: for `card`, or for a new contact.
    pub(super) fn start_contact_edit(
        &mut self,
        card: Option<StoredCard>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let accent = rgba(self.theme(window).accent).into();
        let mut subscriptions = Vec::new();
        let mut field = |text: String, placeholder: String, cx: &mut Context<Self>| {
            let input = cx.new(|cx| {
                let mut input = TextInput::new(placeholder, cx);
                input.set_accent(accent);
                input.set_text(text, cx);
                input
            });
            subscriptions.push(cx.subscribe(
                &input,
                |this, _, event: &InputEvent, cx| match event {
                    InputEvent::Submit => this.save_contact_edit(cx),
                    InputEvent::Cancel => this.cancel_contact_edit(cx),
                    InputEvent::Changed => {
                        if let Some(edit) = &mut this.contacts.edit {
                            edit.error = None;
                        }
                    }
                },
            ));
            input
        };
        let base = card.as_ref().map(|c| c.card.clone()).unwrap_or_default();
        let (given, family) = if base.name.given.is_empty() && base.name.family.is_empty() {
            split_name(&base.name.full)
        } else {
            (base.name.given.clone(), base.name.family.clone())
        };
        let address = base.addresses.first().cloned().unwrap_or_default();
        let mut rows = |list: &[Typed], kind: &str, cx: &mut Context<Self>| -> Vec<Row> {
            let mut rows: Vec<Row> = list
                .iter()
                .map(|t| Row {
                    kind: t.kind.clone(),
                    input: field(t.value.clone(), String::new(), cx),
                })
                .collect();
            if rows.is_empty() {
                rows.push(Row {
                    kind: kind.into(),
                    input: field(String::new(), String::new(), cx),
                });
            }
            rows
        };
        let emails = rows(&base.emails, "home", cx);
        let phones = rows(&base.phones, "mobile", cx);
        let given = field(given, String::new(), cx);
        let books = self.writable_books();
        let book = card
            .as_ref()
            .map_or_else(|| books.first().map_or(0, |(id, _)| *id), |c| c.book);
        let editor = Editor {
            card: card.as_ref().map_or(0, |c| c.id),
            book,
            books,
            given: given.clone(),
            family: field(family, String::new(), cx),
            company: field(base.organization.clone(), String::new(), cx),
            job: field(base.title.clone(), String::new(), cx),
            emails,
            phones,
            street: field(address.street.replace('\n', ", "), String::new(), cx),
            city: field(address.city.clone(), String::new(), cx),
            postcode: field(address.postcode.clone(), String::new(), cx),
            country: field(address.country.clone(), String::new(), cx),
            // A day without a year shows as MM-DD rather than vCard's --MM-DD.
            birthday: field(
                base.birthday
                    .strip_prefix("--")
                    .unwrap_or(&base.birthday)
                    .to_owned(),
                String::new(),
                cx,
            ),
            note: field(base.note.clone(), String::new(), cx),
            base,
            saving: false,
            error: None,
            subscriptions,
            _task: None,
        };
        self.contacts.edit = Some(editor);
        window.focus(&given.focus_handle(cx), cx);
        cx.notify();
    }

    /// The books a new contact can be saved to: each account's that syncs,
    /// then this computer.
    pub(super) fn writable_books(&self) -> Vec<(i64, String)> {
        let mut out = Vec::new();
        if let Some(Ok(book)) = &self.contacts.book {
            for b in &book.books {
                if b.source == BookSource::Local || b.state != BookState::Ok {
                    continue;
                }
                let Some(account) = b
                    .account
                    .and_then(|id| self.accounts.iter().find(|a| a.id == id))
                else {
                    continue;
                };
                let name = if b.name.is_empty() {
                    account.address.clone()
                } else {
                    format!("{} · {}", account.address, b.name)
                };
                out.push((b.id, name));
            }
        }
        out.push((0, tr!("contacts-this-computer")));
        out
    }

    pub(super) fn cancel_contact_edit(&mut self, cx: &mut Context<Self>) {
        self.contacts.edit = None;
        cx.notify();
    }

    /// Adds an empty address or number to the form.
    fn add_contact_row(&mut self, phone: bool, window: &mut Window, cx: &mut Context<Self>) {
        let accent = rgba(self.theme(window).accent).into();
        let Some(edit) = &mut self.contacts.edit else {
            return;
        };
        let input = cx.new(|cx| {
            let mut input = TextInput::new("", cx);
            input.set_accent(accent);
            input
        });
        edit.subscriptions.push(cx.subscribe(
            &input,
            |this, _, event: &InputEvent, cx| match event {
                InputEvent::Submit => this.save_contact_edit(cx),
                InputEvent::Cancel => this.cancel_contact_edit(cx),
                InputEvent::Changed => {}
            },
        ));
        let row = Row {
            kind: if phone { "mobile" } else { "home" }.into(),
            input: input.clone(),
        };
        if phone {
            edit.phones.push(row);
        } else {
            edit.emails.push(row);
        }
        window.focus(&input.focus_handle(cx), cx);
        cx.notify();
    }

    /// Sends the form to the daemon, which saves it to the account.
    pub(super) fn save_contact_edit(&mut self, cx: &mut Context<Self>) {
        let Some(edit) = &self.contacts.edit else {
            return;
        };
        if edit.saving {
            return;
        }
        let card = edit.card_from_form(cx);
        if card.is_empty() {
            if let Some(edit) = &mut self.contacts.edit {
                edit.error = Some(tr!("contacts-edit-empty"));
            }
            cx.notify();
            return;
        }
        let (contact, book) = (edit.card, edit.book);
        let connection = self.daemon.clone();
        let task = cx.spawn(async move |this, cx| {
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                daemon::save_contact(&connection, contact, book, &card).await
            }
            .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(id) => {
                        this.contacts.edit = None;
                        this.contacts.open_after_load = Some(id);
                        this.load_contacts(cx);
                        this.show_snackbar(tr!("contacts-saved"), None, cx);
                    }
                    Err(err) => {
                        if let Some(edit) = &mut this.contacts.edit {
                            edit.saving = false;
                            edit.error = Some(err);
                        }
                    }
                }
                cx.notify();
            })
            .ok();
        });
        if let Some(edit) = &mut self.contacts.edit {
            edit.saving = true;
            edit.error = None;
            edit._task = Some(task);
        }
        cx.notify();
    }

    /// Saves someone from Mail at once: to the address book of the account
    /// in view, else the first account's, else this computer. Undo
    /// deletes them again.
    pub(super) fn add_to_contacts(
        &mut self,
        email: &str,
        name: Option<String>,
        signature: Option<crate::profile::Card>,
        cx: &mut Context<Self>,
    ) {
        let email = email.trim().to_lowercase();
        if email.is_empty() || !self.contacts.adding.insert(email.clone()) {
            return;
        }
        let (given, family) = split_name(name.as_deref().unwrap_or_default());
        let signature = signature.unwrap_or_default();
        let card = Card {
            name: katna_core::contact::Name {
                given,
                family,
                ..Default::default()
            },
            emails: vec![Typed::new(&email, "")],
            phones: signature
                .phone
                .map(|p| vec![Typed::new(&p, "")])
                .unwrap_or_default(),
            title: signature.title.unwrap_or_default(),
            organization: signature.company.unwrap_or_default(),
            ..Card::default()
        };
        let book = self.book_for_new_contact();
        let shown = name.unwrap_or_else(|| email.clone());
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                daemon::save_contact(&connection, 0, book, &card).await
            }
            .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(id) => {
                        // Saved: the panel offers the contact until read back.
                        this.contacts.by_email.insert(email.clone(), id);
                        this.load_contacts(cx);
                        this.show_snackbar(
                            tr!("contacts-added", name = shown),
                            Some(Command::DeleteContacts(vec![id])),
                            cx,
                        );
                    }
                    Err(err) => this.show_snackbar(crate::format::sentence(&err), None, cx),
                }
                this.contacts.adding.remove(&email);
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    /// Where Add to contacts saves: the account in view's address book,
    /// the first account's, or this computer (0).
    pub(super) fn book_for_new_contact(&self) -> i64 {
        let books = self.writable_books();
        let Some(Ok(saved)) = &self.contacts.book else {
            return 0;
        };
        let account = self.account();
        let of_account = books.iter().find(|(id, _)| {
            saved
                .books
                .iter()
                .any(|b| b.id == *id && b.account.is_some() && b.account == account)
        });
        of_account
            .or_else(|| books.iter().find(|(id, _)| *id != 0))
            .map_or(0, |(id, _)| *id)
    }

    /// Opens the Contacts page on the person saved at `email`.
    pub(super) fn show_saved_contact(&mut self, email: &str, cx: &mut Context<Self>) {
        let Some(Ok(book)) = &self.contacts.book else {
            return;
        };
        let email = email.trim().to_lowercase();
        let Some(person) = book
            .people
            .iter()
            .find(|p| p.emails.contains(&email))
            .cloned()
        else {
            return;
        };
        self.open_app(super::apps::App::Contacts, cx);
        self.contacts.view = super::contacts_page::View::Contacts;
        self.contacts.edit = None;
        self.open_saved_contact(person, cx);
    }

    /// Deletes `person` from every account that keeps them, once the
    /// snackbar's Undo is gone.
    pub(super) fn delete_contact(
        &mut self,
        person: &SavedContact,
        cards: &[StoredCard],
        cx: &mut Context<Self>,
    ) {
        let Some(&key) = person.ids.first() else {
            return;
        };
        // One waiting before goes now, so it is not lost.
        self.send_pending_delete(cx);
        self.contacts.hidden.insert(key);
        self.contacts.open = None;
        self.contacts.deleted.push(Deleted {
            key,
            cards: cards.iter().map(|c| (c.book, c.card.clone())).collect(),
        });
        // Keep the last few, for a late Undo.
        if self.contacts.deleted.len() > 20 {
            self.contacts.deleted.remove(0);
        }
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(DELETE_AFTER).await;
            this.update(cx, |this, cx| this.send_pending_delete(cx))
                .ok();
        });
        self.contacts.pending_delete = Some(PendingDelete {
            keys: vec![key],
            cards: person.ids.clone(),
            _task: task,
        });
        self.show_snackbar(
            tr!("contacts-deleted", name = person.name.clone()),
            Some(Command::RestoreContacts(vec![key])),
            cx,
        );
        cx.notify();
    }

    /// Sends the delete waiting for its Undo, if one is.
    pub(super) fn send_pending_delete(&mut self, cx: &mut Context<Self>) {
        let Some(pending) = self.contacts.pending_delete.take() else {
            return;
        };
        let connection = self.daemon.clone();
        let keys = pending.keys.clone();
        cx.spawn(async move |this, cx| {
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                daemon::delete_contacts(&connection, &pending.cards).await
            }
            .await;
            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    // Not deleted: they show again, with why.
                    for key in &keys {
                        this.contacts.hidden.remove(key);
                    }
                    this.show_snackbar(err, None, cx);
                }
                this.load_contacts(cx);
            })
            .ok();
        })
        .detach();
    }

    /// Undo of a delete: shows the people again, or saves their cards
    /// again if the delete went out already.
    pub(super) fn restore_contacts(&mut self, keys: &[i64], cx: &mut Context<Self>) {
        let waiting = self
            .contacts
            .pending_delete
            .as_ref()
            .is_some_and(|p| p.keys.iter().any(|k| keys.contains(k)));
        if waiting {
            self.contacts.pending_delete = None;
            for key in keys {
                self.contacts.hidden.remove(key);
            }
            self.show_snackbar(tr!("toast-undone"), None, cx);
            cx.notify();
            return;
        }
        let cards: Vec<(i64, Card)> = self
            .contacts
            .deleted
            .iter()
            .filter(|d| keys.contains(&d.key))
            .flat_map(|d| d.cards.clone())
            .collect();
        let connection = self.daemon.clone();
        let keys = keys.to_vec();
        cx.spawn(async move |this, cx| {
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                for (book, card) in &cards {
                    daemon::save_contact(&connection, 0, *book, card).await?;
                }
                Ok::<_, String>(())
            }
            .await;
            this.update(cx, |this, cx| {
                for key in &keys {
                    this.contacts.hidden.remove(key);
                }
                match result {
                    Ok(()) => this.show_snackbar(tr!("toast-undone"), None, cx),
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                this.load_contacts(cx);
            })
            .ok();
        })
        .detach();
    }

    /// The form, in place of the list.
    pub(super) fn render_contact_editor(
        &self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let edit = self.contacts.edit.as_ref()?;
        let new = edit.card == 0;
        let field = |id: &'static str,
                     ix: usize,
                     label: String,
                     input: &Entity<TextInput>,
                     cx: &mut Context<Self>| {
            div().flex_1().min_w_0().child(self.outlined_field(
                (id, ix),
                label,
                input,
                false,
                th,
                window,
                cx,
            ))
        };
        let line = |glyph: &str, fields: Vec<AnyElement>| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .child(
                    div()
                        .w(px(24.0))
                        .flex_none()
                        .child(icon(glyph, th.text_faint, 20.0)),
                )
                .children(fields)
                .into_any_element()
        };
        let mut lines = vec![
            line(
                "people",
                vec![
                    field(
                        "contact-edit-given",
                        0,
                        tr!("contacts-edit-given"),
                        &edit.given,
                        cx,
                    )
                    .into_any_element(),
                    field(
                        "contact-edit-family",
                        0,
                        tr!("contacts-edit-family"),
                        &edit.family,
                        cx,
                    )
                    .into_any_element(),
                ],
            ),
            line(
                "work",
                vec![
                    field(
                        "contact-edit-company",
                        0,
                        tr!("contacts-edit-company"),
                        &edit.company,
                        cx,
                    )
                    .into_any_element(),
                    field(
                        "contact-edit-job",
                        0,
                        tr!("contacts-edit-job"),
                        &edit.job,
                        cx,
                    )
                    .into_any_element(),
                ],
            ),
        ];
        for (ix, row) in edit.emails.iter().enumerate() {
            let label = with_kind(tr!("contacts-edit-email"), &row.kind);
            lines.push(line(
                if ix == 0 { "mail" } else { "" },
                vec![field("contact-edit-email", ix, label, &row.input, cx).into_any_element()],
            ));
        }
        lines.push(
            add_row("contact-edit-add-email", tr!("contacts-edit-add-email"), th)
                .on_click(
                    cx.listener(|this, _, window, cx| this.add_contact_row(false, window, cx)),
                )
                .into_any_element(),
        );
        for (ix, row) in edit.phones.iter().enumerate() {
            let label = with_kind(tr!("contacts-edit-phone"), &row.kind);
            lines.push(line(
                if ix == 0 { "phone" } else { "" },
                vec![field("contact-edit-phone", ix, label, &row.input, cx).into_any_element()],
            ));
        }
        lines.push(
            add_row("contact-edit-add-phone", tr!("contacts-edit-add-phone"), th)
                .on_click(cx.listener(|this, _, window, cx| this.add_contact_row(true, window, cx)))
                .into_any_element(),
        );
        lines.push(line(
            "location",
            vec![
                field(
                    "contact-edit-street",
                    0,
                    tr!("contacts-edit-street"),
                    &edit.street,
                    cx,
                )
                .into_any_element(),
            ],
        ));
        lines.push(line(
            "",
            vec![
                field(
                    "contact-edit-city",
                    0,
                    tr!("contacts-edit-city"),
                    &edit.city,
                    cx,
                )
                .into_any_element(),
                field(
                    "contact-edit-postcode",
                    0,
                    tr!("contacts-edit-postcode"),
                    &edit.postcode,
                    cx,
                )
                .into_any_element(),
                field(
                    "contact-edit-country",
                    0,
                    tr!("contacts-edit-country"),
                    &edit.country,
                    cx,
                )
                .into_any_element(),
            ],
        ));
        lines.push(line(
            "cake",
            vec![
                field(
                    "contact-edit-birthday",
                    0,
                    tr!("contacts-edit-birthday"),
                    &edit.birthday,
                    cx,
                )
                .into_any_element(),
            ],
        ));
        lines.push(line(
            "notes",
            vec![
                field(
                    "contact-edit-note",
                    0,
                    tr!("contacts-notes"),
                    &edit.note,
                    cx,
                )
                .into_any_element(),
            ],
        ));

        // A new contact picks its account; a saved one says where it is.
        let save_to = if new {
            let chips = edit.books.iter().map(|(id, name)| {
                let on = *id == edit.book;
                let book = *id;
                div()
                    .id(("contact-edit-book", book.max(0) as usize))
                    .relative()
                    .overflow_hidden()
                    .h(px(32.0))
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(rgba(if on { th.nav_selected } else { th.outline }))
                    .when(on, |d| d.bg(rgba(th.nav_selected)))
                    .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .cursor_pointer()
                    .text_size(px(13.0))
                    .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
                    .child(Ripple::new(
                        ("contact-edit-book-ripple", book.max(0) as usize),
                        rgba(th.ripple),
                    ))
                    .when(on, |d| d.child(icon("check", th.nav_selected_text, 16.0)))
                    .child(name.clone())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(edit) = &mut this.contacts.edit {
                            edit.book = book;
                        }
                        cx.notify();
                    }))
            });
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("contacts-edit-save-to")),
                )
                .children(chips)
                .into_any_element()
        } else {
            let book = self
                .contacts
                .book
                .as_ref()
                .and_then(|b| b.as_ref().ok())
                .and_then(|b| b.books.iter().find(|x| x.id == edit.book));
            let place = match book.and_then(|b| b.account) {
                Some(id) => self
                    .accounts
                    .iter()
                    .find(|a| a.id == id)
                    .map(|a| a.address.clone())
                    .unwrap_or_default(),
                None => tr!("contacts-this-computer"),
            };
            div()
                .text_size(px(13.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("contacts-edit-changes-go-to", place = place))
                .into_any_element()
        };

        let top = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .pb(px(16.0))
            .child(
                icon_button("contact-edit-close", "close", 20.0, th)
                    .tooltip(tip(tr!("contacts-edit-cancel"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.cancel_contact_edit(cx))),
            )
            .child(
                div()
                    .flex_1()
                    .text_size(px(22.0))
                    .text_color(rgba(th.text))
                    .child(if new {
                        tr!("contacts-edit-new-title")
                    } else {
                        tr!("contacts-edit-title")
                    }),
            )
            .child(
                outlined_button("contact-edit-cancel", tr!("contacts-edit-cancel"), th)
                    .on_click(cx.listener(|this, _, _, cx| this.cancel_contact_edit(cx))),
            )
            .child(
                filled_button(
                    "contact-edit-save",
                    if edit.saving {
                        tr!("contacts-edit-saving")
                    } else {
                        tr!("contacts-edit-save")
                    },
                    th,
                )
                .when(edit.saving, |d| d.opacity(0.7))
                .on_click(cx.listener(|this, _, _, cx| this.save_contact_edit(cx))),
            );
        Some(
            div()
                .id("contact-editor")
                .size_full()
                .overflow_y_scroll()
                .px(px(24.0))
                .pb(px(24.0))
                .child(top)
                .child(
                    div()
                        .max_w(px(720.0))
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .child(save_to)
                        .children(edit.error.clone().map(|err| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(8.0))
                                .text_size(px(14.0))
                                .text_color(rgba(th.error))
                                .child(icon("info", th.error, 18.0))
                                .child(err)
                        }))
                        .children(lines),
                )
                .with_animation(
                    ("contact-editor", edit.card.max(0) as usize),
                    gpui::Animation::new(Duration::from_millis(220))
                        .with_easing(gpui::ease_out_quint()),
                    |el, t| el.opacity(t).mt(px(10.0 * (1.0 - t))),
                )
                .into_any_element(),
        )
    }
}

impl Editor {
    /// The card the form holds: the old one with the form's fields.
    fn card_from_form(&self, cx: &gpui::App) -> Card {
        let text = |input: &Entity<TextInput>| input.read(cx).text().trim().to_owned();
        let mut card = self.base.clone();
        let (given, family) = (text(&self.given), text(&self.family));
        let (old_given, old_family) =
            if self.base.name.given.is_empty() && self.base.name.family.is_empty() {
                split_name(&self.base.name.full)
            } else {
                (self.base.name.given.clone(), self.base.name.family.clone())
            };
        if given != old_given || family != old_family {
            // The name shown is made from the parts again.
            card.name.full.clear();
            card.name.given = given;
            card.name.family = family;
        }
        card.organization = text(&self.company);
        card.title = text(&self.job);
        let rows = |rows: &[Row]| -> Vec<Typed> {
            rows.iter()
                .map(|r| Typed::new(text(&r.input), r.kind.clone()))
                .filter(|t| !t.value.is_empty())
                .collect()
        };
        card.emails = rows(&self.emails);
        card.phones = rows(&self.phones);
        let first = PostalAddress {
            kind: self
                .base
                .addresses
                .first()
                .map_or_else(|| "home".to_owned(), |a| a.kind.clone()),
            street: text(&self.street),
            city: text(&self.city),
            region: self
                .base
                .addresses
                .first()
                .map(|a| a.region.clone())
                .unwrap_or_default(),
            postcode: text(&self.postcode),
            country: text(&self.country),
        };
        // The street was shown on one line; unchanged, it keeps its lines.
        let first = match self.base.addresses.first() {
            Some(old) if old.street.replace('\n', ", ") == first.street => PostalAddress {
                street: old.street.clone(),
                ..first
            },
            _ => first,
        };
        let mut addresses: Vec<PostalAddress> =
            self.base.addresses.iter().skip(1).cloned().collect();
        if !first.is_empty() {
            addresses.insert(0, first);
        }
        card.addresses = addresses;
        card.birthday = typed_birthday(&text(&self.birthday));
        card.note = text(&self.note);
        card
    }
}

/// A birthday as typed: `MM-DD` (no year) is kept as vCard's `--MM-DD`.
fn typed_birthday(text: &str) -> String {
    let bytes = text.as_bytes();
    let day_only = bytes.len() == 5
        && bytes[2] == b'-'
        && text.chars().filter(char::is_ascii_digit).count() == 4;
    if day_only {
        format!("--{text}")
    } else {
        text.to_owned()
    }
}

/// "Asha Rao" as given and family name, for a card with a whole name only.
fn split_name(full: &str) -> (String, String) {
    let full = full.trim();
    match full.rsplit_once(' ') {
        Some((given, family)) => (given.trim().to_owned(), family.to_owned()),
        None => (full.to_owned(), String::new()),
    }
}

/// "Email (Work)".
fn with_kind(label: String, kind: &str) -> String {
    let kind = super::contacts_page::kind_label(kind);
    if kind.is_empty() {
        label
    } else {
        tr!("contacts-edit-with-kind", field = label, kind = kind)
    }
}

/// "Add email" under the rows.
fn add_row(id: &'static str, label: String, th: &Theme) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .ml(px(40.0))
        .h(px(32.0))
        .px(px(12.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.0))
        .rounded_full()
        .cursor_pointer()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.accent))
        .hover(|s| s.bg(rgba(th.hover)))
        .child(Ripple::new((id, 0usize), rgba(th.ripple)))
        .child(icon("add", th.accent, 18.0))
        .child(label)
}

/// The people not shown while their delete waits.
pub(super) fn visible(people: &[SavedContact], hidden: &BTreeSet<i64>) -> Vec<usize> {
    people
        .iter()
        .enumerate()
        .filter(|(_, p)| p.ids.first().is_none_or(|k| !hidden.contains(k)))
        .map(|(ix, _)| ix)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_names_split_at_the_last_word() {
        assert_eq!(split_name("Asha Rao"), ("Asha".into(), "Rao".into()));
        assert_eq!(
            split_name("Maria de la Cruz"),
            ("Maria de la".into(), "Cruz".into())
        );
        assert_eq!(split_name("Bo"), ("Bo".into(), String::new()));
    }

    #[test]
    fn birthdays_without_a_year_keep_vcard_form() {
        assert_eq!(typed_birthday("03-14"), "--03-14");
        assert_eq!(typed_birthday("1990-03-14"), "1990-03-14");
        assert_eq!(typed_birthday(""), "");
    }

    #[test]
    fn hidden_people_are_left_out() {
        let people = vec![
            SavedContact {
                ids: vec![1, 5],
                ..SavedContact::default()
            },
            SavedContact {
                ids: vec![2],
                ..SavedContact::default()
            },
        ];
        assert_eq!(visible(&people, &BTreeSet::from([1])), [1]);
        assert_eq!(visible(&people, &BTreeSet::new()), [0, 1]);
    }
}
