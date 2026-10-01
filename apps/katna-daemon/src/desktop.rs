// SPDX-License-Identifier: GPL-3.0-or-later

//! The daemon's place on the desktop (`docs/ARCHITECTURE.md` §15.2): the
//! unread count (§15.1.1) on Katna Mail's taskbar or dock icon, and the tray
//! icon with its badge and menu. Both follow `[general]` `unread_badge` and
//! `show_in_tray`, and stay up while the app is closed.

use std::time::Duration;

use async_channel::{Receiver, Sender};
use katna_core::{Paths, config::General, ids};
use katna_dbus::app_action;
use katna_i18n::tr;
use katna_platform::dbusmenu::MenuItem;
use katna_platform::launcher::LauncherEntry;
use katna_platform::tray::{self, Tray};
use katna_store::{Mode, Store};

use crate::mail_app;

/// Mail changes closer together than this are counted once.
const SETTLE: Duration = Duration::from_millis(500);

/// What the desktop presence reacts to.
#[derive(Debug)]
pub(crate) enum Event {
    MailChanged,
    Settings(General),
    /// A click on the tray icon or its menu, with an activation token.
    Tray(String, Option<String>),
}

/// Sends events to a running [`run`].
#[derive(Debug, Clone)]
pub(crate) struct Handle(Sender<Event>);

impl Handle {
    pub(crate) fn mail_changed(&self) {
        let _ = self.0.try_send(Event::MailChanged);
    }

    pub(crate) fn settings(&self, general: General) {
        let _ = self.0.try_send(Event::Settings(general));
    }
}

/// A channel for [`run`] and the handle that feeds it.
pub(crate) fn channel() -> (Handle, Receiver<Event>) {
    let (sender, receiver) = async_channel::unbounded();
    (Handle(sender), receiver)
}

/// Unread messages that count on the taskbar and tray, in every account:
/// by default those in an Inbox's Primary tab, less anything muted
/// (`docs/ARCHITECTURE.md` §15.1.1).
pub(crate) fn counted_unread(store: &Store) -> katna_store::Result<u64> {
    store.counted_unread(unix_now())
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

/// The tray's right-click menu, in the current language.
fn tray_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::action(tr!("tray-open-inbox"), app_action::OPEN_INBOX).icon("mail-folder-inbox"),
        MenuItem::action(tr!("tray-new-message"), app_action::COMPOSE).icon("mail-message-new"),
        MenuItem::Separator,
        MenuItem::action(tr!("tray-preferences"), app_action::PREFERENCES)
            .icon("preferences-system-symbolic"),
        MenuItem::Separator,
        MenuItem::action(tr!("tray-quit"), app_action::QUIT).icon("application-exit"),
    ]
}

/// The tooltip's second line.
fn status_line(count: u64) -> String {
    tr!("tray-unread", count = count)
}

/// Keeps the badge and the tray up to date until `events` closes. The
/// tray's Quit sends on `quit` after closing the app.
pub(crate) async fn run(
    connection: zbus::Connection,
    paths: Paths,
    mut general: General,
    handle: Handle,
    events: Receiver<Event>,
    quit: Sender<()>,
) {
    let launcher =
        match LauncherEntry::serve(&connection, ids::LAUNCHER_ENTRY_PATH, ids::MAIL_APP_ID).await {
            Ok(launcher) => Some(launcher),
            Err(err) => {
                tracing::warn!(%err, "no unread count on the taskbar icon");
                None
            }
        };
    let mut tray: Option<Tray> = None;
    let mut count = None;
    let mut dirty = true;
    // The language the tray's menu and tooltip are in.
    let mut language = general.language.clone();
    loop {
        // A new tray icon shows no count yet; after a new language, the
        // tooltip is in the old one.
        let appeared = follow_setting(&connection, &handle, &mut tray, &general).await;
        let translated = follow_language(tray.as_ref(), &general, &mut language).await;
        if appeared || translated {
            count = None;
            dirty = true;
        }
        if dirty {
            // Let a burst of changes settle, then count once.
            smol::Timer::after(SETTLE).await;
            while let Ok(event) = events.try_recv() {
                if !handle_now(&connection, &mut general, event, &quit).await {
                    return hide(tray).await;
                }
            }
            let appeared = follow_setting(&connection, &handle, &mut tray, &general).await;
            let translated = follow_language(tray.as_ref(), &general, &mut language).await;
            if appeared || translated {
                count = None;
            }
            let paths = paths.clone();
            match smol::unblock(move || counted_unread(&Store::open(&paths, Mode::ReadOnly)?)).await
            {
                Ok(unread) => {
                    show_count(launcher.as_ref(), tray.as_ref(), &general, unread, count).await;
                    count = Some(unread);
                }
                Err(err) => tracing::warn!(%err, "counting unread mail"),
            }
            dirty = false;
        }
        let Ok(event) = events.recv().await else {
            return;
        };
        match event {
            Event::MailChanged => dirty = true,
            Event::Settings(new) => {
                dirty = new.unread_badge != general.unread_badge;
                general = new;
            }
            event => {
                if !handle_now(&connection, &mut general, event, &quit).await {
                    return hide(tray).await;
                }
            }
        }
    }
}

/// Shows or hides the tray icon as `general.show_in_tray` says; `true` when
/// it just appeared.
async fn follow_setting(
    connection: &zbus::Connection,
    handle: &Handle,
    tray: &mut Option<Tray>,
    general: &General,
) -> bool {
    if general.show_in_tray && tray.is_none() {
        *tray = show_tray(connection, handle).await;
        return tray.is_some();
    }
    if !general.show_in_tray
        && let Some(shown) = tray.take()
        && let Err(err) = shown.hide().await
    {
        tracing::warn!(%err, "could not hide the tray icon");
    }
    false
}

/// Rebuilds the tray's menu when `general.language` is no longer
/// `language`, which the daemon has applied already
/// (`Daemon::reload_config`); `true` when the tooltip must be set again.
async fn follow_language(tray: Option<&Tray>, general: &General, language: &mut String) -> bool {
    if general.language == *language {
        return false;
    }
    language.clone_from(&general.language);
    if let Some(tray) = tray
        && let Err(err) = tray.set_menu(tray_menu()).await
    {
        tracing::warn!(%err, "could not translate the tray menu");
    }
    true
}

/// Takes the tray icon away at once after Quit, while the rest of the
/// daemon shuts down.
async fn hide(tray: Option<Tray>) {
    if let Some(tray) = tray
        && let Err(err) = tray.hide().await
    {
        tracing::debug!(%err, "hiding the tray icon");
    }
}

/// Handles an event while counting is on hold; `false` after Quit.
async fn handle_now(
    connection: &zbus::Connection,
    general: &mut General,
    event: Event,
    quit: &Sender<()>,
) -> bool {
    match event {
        Event::MailChanged => {}
        Event::Settings(new) => *general = new,
        Event::Tray(action, token) => {
            if action == app_action::QUIT {
                mail_app::run(connection, Some(app_action::QUIT), Vec::new(), None).await;
                tracing::info!("quit from the tray");
                let _ = quit.send(()).await;
                return false;
            }
            let action = match action.as_str() {
                tray::ACTIVATE => None,
                tray::SECONDARY_ACTIVATE => Some(app_action::COMPOSE),
                action => Some(action),
            };
            mail_app::run(connection, action, Vec::new(), token).await;
        }
    }
    true
}

async fn show_tray(connection: &zbus::Connection, handle: &Handle) -> Option<Tray> {
    let sender = handle.0.clone();
    let shown = Tray::show(
        connection,
        ids::MAIL_APP_ID,
        "Katna Mail",
        // One colour, which the panel recolours to suit itself.
        &format!("{}-symbolic", ids::MAIL_APP_ID),
        tray_menu(),
        move |action, token| {
            let _ = sender.try_send(Event::Tray(action.to_owned(), token));
        },
    )
    .await;
    match shown {
        Ok(tray) => Some(tray),
        Err(err) => {
            tracing::warn!(%err, "no tray icon");
            None
        }
    }
}

/// Shows `unread` on the taskbar icon and the tray. `before` is what the
/// tray shows now (`None` right after it appeared).
async fn show_count(
    launcher: Option<&LauncherEntry>,
    tray: Option<&Tray>,
    general: &General,
    unread: u64,
    before: Option<u64>,
) {
    if let Some(launcher) = launcher {
        let shown = if general.unread_badge { unread } else { 0 };
        if let Err(err) = launcher.set_count(shown).await {
            tracing::debug!(%err, "taskbar count");
        }
    }
    if let Some(tray) = tray
        && before != Some(unread)
        && let Err(err) = tray.set_unread(unread, &status_line(unread)).await
    {
        tracing::debug!(%err, "tray badge");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_core::AccountKind;
    use katna_store::{FolderRole, MessageFlags, NewMessage};

    #[test]
    fn counts_unread_mail_in_every_inbox_only() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Local, "Test", "test@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let archive = batch
            .upsert_folder(account, "Archive", Some(FolderRole::Archive))
            .unwrap();
        for (folder, flags, raw) in [
            (inbox, MessageFlags::empty(), "a"),
            (inbox, MessageFlags::empty(), "b"),
            (inbox, MessageFlags::SEEN, "c"),
            (archive, MessageFlags::empty(), "d"),
        ] {
            let message = NewMessage {
                raw: raw.as_bytes(),
                message_id_hdr: None,
                subject: Some(raw),
                date: None,
                flags,
                has_attachments: false,
                list_id: None,
                snippet: None,
                participants: &[],
                in_reply_to: None,
                references: &[],
                category: None,
            };
            batch.add_message(account, folder, &message).unwrap();
        }
        batch.commit().unwrap();
        assert_eq!(counted_unread(&store).unwrap(), 2);
    }

    #[test]
    fn tray_menu_has_the_four_actions() {
        let actions: Vec<String> = tray_menu()
            .into_iter()
            .filter_map(|item| match item {
                MenuItem::Action { action, .. } => Some(action),
                _ => None,
            })
            .collect();
        assert_eq!(actions, ["open-inbox", "compose", "preferences", "quit"]);
    }

    #[test]
    fn status_line_counts() {
        assert_eq!(status_line(0), "No unread mail");
        assert_eq!(status_line(1), "1 unread message");
        assert_eq!(status_line(5), "5 unread messages");
    }
}
