// SPDX-License-Identifier: GPL-3.0-or-later

//! Changing a drive's files from Files: Move to bin and Rename, each with
//! Undo (`docs/ARCHITECTURE.md` §13.8).

use std::rc::Rc;

use gpui::{AppContext, Context, Entity, Focusable, Subscription, Window, rgba};
use katna_core::AccountId;
use katna_dbus::CloudEntry;
use katna_i18n::tr;
use katna_ui::{InputEvent, TextInput};

use super::super::super::MailWindow;
use crate::daemon::Command;

/// A drive item's name being typed over in place.
pub(in crate::window) struct Renaming {
    pub(super) account: AccountId,
    pub(super) id: String,
    old: String,
    pub(super) input: Entity<TextInput>,
    _subscription: Subscription,
}

impl MailWindow {
    /// Whether the drive on show is the account's own, whose items Katna
    /// may bin and rename (not what others shared with it).
    pub(super) fn drive_owned(&self) -> bool {
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
