// SPDX-License-Identifier: GPL-3.0-or-later

//! Changing a drive's files from Files: New folder and new documents,
//! Move to bin and Rename, each with Undo (`docs/ARCHITECTURE.md` §13.8).

use std::rc::Rc;

use gpui::{AppContext, Context, Entity, Focusable, Subscription, Window, rgba};
use katna_core::AccountId;
use katna_dbus::{CloudEntry, cloud_kind};
use katna_i18n::tr;
use katna_ui::{InputEvent, TextInput};

use super::super::super::MailWindow;
use crate::daemon::Command;

/// A drive item's name being typed over in place, or a new one's being
/// typed.
pub(in crate::window) struct Renaming {
    pub(super) account: AccountId,
    pub(super) id: String,
    old: String,
    pub(super) input: Entity<TextInput>,
    /// A new item, not made yet: what it is (one of `cloud_kind`) and the
    /// folder it goes in.
    new: Option<(&'static str, String)>,
    _subscription: Subscription,
}

/// The id of a new item's card while its name is typed.
pub(super) const NEW_ID: &str = "katna:new";

/// The type a new item's card wears while its name is typed, on Google
/// Drive (`onedrive` false) or OneDrive.
fn new_mime(kind: &str, onedrive: bool) -> &'static str {
    match (kind, onedrive) {
        (cloud_kind::DOCUMENT, false) => "application/vnd.google-apps.document",
        (cloud_kind::SPREADSHEET, false) => "application/vnd.google-apps.spreadsheet",
        (cloud_kind::PRESENTATION, false) => "application/vnd.google-apps.presentation",
        (cloud_kind::DOCUMENT, true) => {
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        }
        (cloud_kind::SPREADSHEET, true) => {
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        }
        (cloud_kind::PRESENTATION, true) => {
            "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        }
        _ => "",
    }
}

/// The name a new item starts with.
fn new_name(kind: &str) -> String {
    match kind {
        cloud_kind::DOCUMENT => tr!("files-drive-untitled-document"),
        cloud_kind::SPREADSHEET => tr!("files-drive-untitled-spreadsheet"),
        cloud_kind::PRESENTATION => tr!("files-drive-untitled-presentation"),
        _ => tr!("files-drive-untitled-folder"),
    }
}

impl MailWindow {
    /// Whether the drive on show is the account's own, whose items Katna
    /// may bin and rename (not what others shared with it).
    pub(in crate::window) fn drive_owned(&self) -> bool {
        self.picker.is_none() && self.library.cloud.view.as_ref().is_some_and(|v| !v.shared)
    }

    /// Moves drive item `entry` to the drive's bin at once, with Undo.
    pub(super) fn trash_drive_item(&mut self, entry: &CloudEntry, cx: &mut Context<Self>) {
        self.library.menu = None;
        if !self.drive_owned() {
            return;
        }
        let Some(view) = self.library.cloud.view.as_mut() else {
            return;
        };
        let account = view.account;
        // It leaves the page now; the listing is read again once the
        // drive has done it.
        let entries: Vec<CloudEntry> = view
            .entries
            .iter()
            .filter(|e| e.id != entry.id)
            .cloned()
            .collect();
        view.entries = Rc::new(entries);
        view.stale = true;
        view.cursor = view
            .cursor
            .map(|c| c.min(view.order.len().saturating_sub(2)));
        let drive = self.drive_name(account);
        let done = tr!(
            "files-drive-trashed",
            name = entry.name.as_str(),
            drive = drive.as_str()
        );
        let ids = vec![entry.id.clone()];
        self.send(
            Command::CloudTrash(account, ids.clone(), true),
            Some(done),
            Some(Command::CloudTrash(account, ids, false)),
            false,
            cx,
        );
        cx.notify();
    }

    /// Puts a card for a new folder or document (`kind`, one of
    /// `cloud_kind`) first on the page, its name selected to type over;
    /// Enter makes it in the folder on show, Escape drops it.
    pub(in crate::window) fn start_drive_new(
        &mut self,
        kind: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.library.menu = None;
        if !self.drive_owned() {
            return;
        }
        if self.library.cloud.renaming.is_some() {
            self.finish_drive_rename(true, window, cx);
        }
        let Some(view) = self.library.cloud.view.as_ref() else {
            return;
        };
        let account = view.account;
        let folder = view
            .crumbs
            .last()
            .map(|(id, _)| id.clone())
            .unwrap_or_default();
        let onedrive = self.drive_is_onedrive(account);
        let name = new_name(kind);
        let card = CloudEntry {
            id: NEW_ID.into(),
            name: name.clone(),
            mime: new_mime(kind, onedrive).into(),
            folder: kind == cloud_kind::FOLDER,
            native: !onedrive && kind != cloud_kind::FOLDER,
            modified: jiff::Timestamp::now().as_second(),
            ..CloudEntry::default()
        };
        if let Some(view) = self.library.cloud.view.as_mut() {
            let mut entries: Vec<CloudEntry> = view
                .entries
                .iter()
                .filter(|e| e.id != NEW_ID)
                .cloned()
                .collect();
            entries.insert(0, card);
            view.entries = Rc::new(entries);
            // The new card shows whatever the filters, at the top.
            view.types = super::Types::All;
            view.cursor = None;
            view.stale = true;
            view.state.scroll_to_reveal_item(0);
        }
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(name.clone(), cx);
            input.set_accent(accent);
            input.set_text(name.clone(), cx);
            input.select_range(0..name.len(), cx);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.finish_drive_rename(true, window, cx),
                InputEvent::Cancel => this.finish_drive_rename(false, window, cx),
                InputEvent::Changed => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.library.cloud.renaming = Some(Renaming {
            account,
            id: NEW_ID.into(),
            old: String::new(),
            input,
            new: Some((kind, folder)),
            _subscription: subscription,
        });
        cx.notify();
    }

    /// Makes the new item with the typed name (`save`), or takes its card
    /// away.
    fn finish_drive_new(
        &mut self,
        account: AccountId,
        kind: &'static str,
        folder: String,
        name: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let drop_card = |this: &mut Self| {
            if let Some(view) = this.library.cloud.view.as_mut() {
                let entries: Vec<CloudEntry> = view
                    .entries
                    .iter()
                    .filter(|e| e.id != NEW_ID)
                    .cloned()
                    .collect();
                view.entries = Rc::new(entries);
                view.stale = true;
            }
        };
        let (Some(name), Some(connection)) = (name, self.daemon.clone()) else {
            drop_card(self);
            return;
        };
        // The card keeps the typed name while the drive makes it.
        if let Some(view) = self.library.cloud.view.as_mut() {
            let entries: Vec<CloudEntry> = view
                .entries
                .iter()
                .map(|e| {
                    let mut e = e.clone();
                    if e.id == NEW_ID {
                        e.name = name.clone();
                    }
                    e
                })
                .collect();
            view.entries = Rc::new(entries);
            view.stale = true;
        }
        let drive = self.drive_name(account);
        cx.spawn(async move |this, cx| {
            let made =
                crate::daemon::cloud_create(&connection, account.0, &folder, &name, kind).await;
            this.update(cx, |this, cx| {
                match made {
                    Ok(entry) => {
                        if let Some(view) = this
                            .library
                            .cloud
                            .view
                            .as_mut()
                            .filter(|v| v.account == account)
                        {
                            let entries: Vec<CloudEntry> = view
                                .entries
                                .iter()
                                .map(|e| {
                                    if e.id == NEW_ID {
                                        entry.clone()
                                    } else {
                                        e.clone()
                                    }
                                })
                                .collect();
                            view.entries = Rc::new(entries);
                            view.stale = true;
                        }
                        let text = tr!(
                            "files-drive-made",
                            name = entry.name.as_str(),
                            drive = drive.as_str()
                        );
                        let undo = Command::CloudTrash(account, vec![entry.id.clone()], true);
                        this.show_snackbar(text, Some(undo), cx);
                        // Katna can't edit documents: the drive's own
                        // editor opens in the browser.
                        if !entry.folder && !entry.link.is_empty() {
                            cx.open_url(&entry.link);
                        }
                    }
                    Err(err) => {
                        drop_card(this);
                        let text =
                            tr!("files-drive-make-failed", name = name.as_str(), error = err);
                        this.show_snackbar(text, None, cx);
                    }
                }
                this.drive_listing_changed(account, cx);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Puts drive item `entry`'s name in a box to type over, its name
    /// before the extension selected.
    pub(super) fn start_drive_rename(
        &mut self,
        entry: &CloudEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.library.menu = None;
        if !self.drive_owned() {
            return;
        }
        let Some(account) = self.library.cloud.view.as_ref().map(|v| v.account) else {
            return;
        };
        let accent = rgba(self.theme(window).accent).into();
        let name = entry.name.clone();
        let stem = match name.rfind('.') {
            Some(dot) if dot > 0 && !entry.folder && !entry.native => dot,
            _ => name.len(),
        };
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("files-drive-rename"), cx);
            input.set_accent(accent);
            input.set_text(name.clone(), cx);
            input.select_range(0..stem, cx);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.finish_drive_rename(true, window, cx),
                InputEvent::Cancel => this.finish_drive_rename(false, window, cx),
                InputEvent::Changed => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.library.cloud.renaming = Some(Renaming {
            account,
            id: entry.id.clone(),
            old: name,
            input,
            new: None,
            _subscription: subscription,
        });
        cx.notify();
    }

    /// Gives the item the typed name (`save`), or leaves the old one.
    pub(super) fn finish_drive_rename(
        &mut self,
        save: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(renaming) = self.library.cloud.renaming.take() else {
            return;
        };
        if let Some(focus) = &self.library.focus {
            window.focus(focus, cx);
        }
        cx.notify();
        let name = renaming.input.read(cx).text().trim().to_owned();
        if let Some((kind, folder)) = renaming.new {
            let name = (save && !name.is_empty() && !name.contains('/')).then_some(name);
            self.finish_drive_new(renaming.account, kind, folder, name, cx);
            return;
        }
        if !save || name.is_empty() || name == renaming.old || name.contains('/') {
            return;
        }
        if let Some(view) = self
            .library
            .cloud
            .view
            .as_mut()
            .filter(|v| v.account == renaming.account)
        {
            let entries: Vec<CloudEntry> = view
                .entries
                .iter()
                .map(|e| {
                    let mut e = e.clone();
                    if e.id == renaming.id {
                        e.name = name.clone();
                    }
                    e
                })
                .collect();
            view.entries = Rc::new(entries);
            view.stale = true;
        }
        let done = tr!("files-drive-renamed", name = name.as_str());
        self.send(
            Command::CloudRename(renaming.account, renaming.id.clone(), name),
            Some(done),
            Some(Command::CloudRename(
                renaming.account,
                renaming.id,
                renaming.old,
            )),
            false,
            cx,
        );
    }

    /// A change to the drive of `account` went through (or failed): its
    /// listings are read again, the one on show without blanking it.
    pub(in crate::window) fn drive_listing_changed(
        &mut self,
        account: AccountId,
        cx: &mut Context<Self>,
    ) {
        let cloud = &mut self.library.cloud;
        cloud.cache.retain(|(a, _), _| *a != account);
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(view) = cloud.view.as_mut().filter(|v| v.account == account) else {
            return;
        };
        let key = view.key();
        let (place, what) = view.place();
        view._load = Some(cx.spawn(async move |this, cx| {
            let got = crate::daemon::cloud_list(&connection, account.0, place, &what, "").await;
            this.update(cx, |this, cx| {
                this.drive_listed(key, got, false);
                cx.notify();
            })
            .ok();
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_cards_type_fits_the_drive() {
        assert_eq!(new_mime(cloud_kind::FOLDER, false), "");
        assert_eq!(
            new_mime(cloud_kind::SPREADSHEET, false),
            "application/vnd.google-apps.spreadsheet"
        );
        assert!(new_mime(cloud_kind::PRESENTATION, true).ends_with("presentationml.presentation"));
    }
}
