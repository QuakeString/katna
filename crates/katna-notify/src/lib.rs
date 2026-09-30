// SPDX-License-Identifier: GPL-3.0-or-later

//! Desktop notifications with actions and inline reply.
//! See `docs/ARCHITECTURE.md` §15.1.
//!
//! Talks to the desktop's `org.freedesktop.Notifications` server directly
//! (Plasma, GNOME Shell, mako, dunst, …). So far: new-mail notifications
//! with Open, Reply all, Mark as read and Archive, reminders (snooze,
//! follow-up) with Open, Mark as read and Archive, and event reminders
//! with Join and Snooze, and a note with Undo after Archive.

use std::collections::HashMap;

use katna_core::ids;
use katna_i18n::tr;
use zbus::zvariant::Value;

/// Actions on a new-mail notification, as `ActionInvoked` names them.
pub mod action {
    /// A click on the notification itself.
    pub const OPEN: &str = "default";
    /// Only on a notification about one message.
    pub const REPLY_ALL: &str = "reply-all";
    pub const MARK_READ: &str = "mark-read";
    pub const ARCHIVE: &str = "archive";
    /// On the notification that an update is ready: install it.
    pub const UPDATE: &str = "update";
    /// On an event's reminder: open its video call.
    pub const JOIN: &str = "join";
    /// On an event's or a task's reminder: remind again in a few minutes.
    pub const SNOOZE: &str = "snooze";
    /// On a task's reminder: tick the task off.
    pub const DONE: &str = "done";
    /// On the note that mail was archived from a notification: put it
    /// back in the inbox.
    pub const UNDO: &str = "undo";
}

/// At most this many messages are listed in a grouped notification.
const LISTED: usize = 4;
/// Longest preview of a single message's text, in characters.
const PREVIEW_CHARS: usize = 160;
/// How long the note that mail was archived stays, in milliseconds.
const ARCHIVED_SHOWN_MS: i32 = 8000;

#[zbus::proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;

    fn close_notification(&self, id: u32) -> zbus::Result<()>;

    fn get_capabilities(&self) -> zbus::Result<Vec<String>>;

    #[zbus(signal)]
    fn action_invoked(&self, id: u32, action_key: &str) -> zbus::Result<()>;

    /// Sent before `ActionInvoked` by servers that support it; the token
    /// lets the opened window take focus on Wayland.
    #[zbus(signal)]
    fn activation_token(&self, id: u32, activation_token: &str) -> zbus::Result<()>;

    #[zbus(signal)]
    fn notification_closed(&self, id: u32, reason: u32) -> zbus::Result<()>;
}

/// One new message, as a notification shows it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NewMail {
    /// Display name, or the address when there is none.
    pub sender: String,
    pub subject: String,
    /// The start of the text, when the body is downloaded.
    pub preview: Option<String>,
}

/// Summary and body of a notification for `mails` (not empty), in the
/// current language (`i18n/<language>/katna-daemon/notifications.ftl`).
pub fn new_mail_text(mails: &[NewMail]) -> (String, String) {
    let subject = |mail: &NewMail| {
        if mail.subject.trim().is_empty() {
            tr!("notify-no-subject")
        } else {
            mail.subject.clone()
        }
    };
    if let [mail] = mails {
        let mut body = escape(&subject(mail));
        if let Some(preview) = mail.preview.as_deref().filter(|p| !p.trim().is_empty()) {
            body.push('\n');
            body.push_str(&escape(&shorten(preview, PREVIEW_CHARS)));
        }
        return (mail.sender.clone(), body);
    }
    let summary = tr!("notify-new-emails", count = mails.len());
    let mut lines: Vec<String> = mails
        .iter()
        .take(LISTED)
        .map(|mail| escape(&format!("{}: {}", mail.sender, subject(mail))))
        .collect();
    if mails.len() > LISTED {
        lines.push(escape(&tr!(
            "notify-and-more",
            count = mails.len() - LISTED
        )));
    }
    (summary, lines.join("\n"))
}

/// The body may hold markup on some servers: `&`, `<` and `>` must not
/// be read as such.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn shorten(text: &str, max: usize) -> String {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    match text.char_indices().nth(max) {
        Some((at, _)) => format!("{}…", text[..at].trim_end()),
        None => text,
    }
}

/// Shows new-mail notifications on the session bus.
#[derive(Clone)]
pub struct Notifier {
    proxy: NotificationsProxy<'static>,
}

impl Notifier {
    pub async fn new(connection: &zbus::Connection) -> zbus::Result<Self> {
        Ok(Self {
            proxy: NotificationsProxy::new(connection).await?,
        })
    }

    /// The server's proxy, for its signals.
    pub fn proxy(&self) -> &NotificationsProxy<'static> {
        &self.proxy
    }

    /// Shows `mails` (not empty) of the account `origin` (its address),
    /// replacing notification `replaces` if not 0, with the new-mail sound
    /// or, without `sound`, silently. Returns its ID.
    pub async fn new_mail(
        &self,
        origin: &str,
        mails: &[NewMail],
        replaces: u32,
        sound: bool,
    ) -> zbus::Result<u32> {
        let (summary, body) = new_mail_text(mails);
        let one = mails.len() == 1;
        let mut actions = vec![(action::OPEN, tr!("notify-open"))];
        if one {
            actions.push((action::REPLY_ALL, tr!("notify-reply-all")));
        }
        let mark_read = if one {
            tr!("notify-mark-read")
        } else {
            tr!("notify-mark-all-read")
        };
        actions.extend([
            (action::MARK_READ, mark_read),
            (action::ARCHIVE, tr!("notify-archive")),
        ]);
        // Key, label, key, label, … as the specification has them.
        let actions: Vec<&str> = actions
            .iter()
            .flat_map(|(key, label)| [*key, label.as_str()])
            .collect();
        let mut hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("email.arrived")),
            ("x-kde-origin-name", Value::from(origin)),
            ("urgency", Value::U8(1)),
        ]);
        if sound {
            hints.insert("sound-name", Value::from("message-new-email"));
        } else {
            hints.insert("suppress-sound", Value::Bool(true));
        }
        self.proxy
            .notify(
                "Katna Mail",
                replaces,
                ids::MAIL_APP_ID,
                &summary,
                &body,
                &actions,
                hints,
                -1,
            )
            .await
    }

    /// Shows a quiet notification that a tracked message was opened or a
    /// link in it followed (`docs/ARCHITECTURE.md` §16.1), with an Open
    /// button for Katna Mail. Returns its ID.
    pub async fn tracking(&self, summary: &str, body: &str) -> zbus::Result<u32> {
        let open = tr!("notify-open");
        let actions = [action::OPEN, open.as_str()];
        let hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("email")),
            ("urgency", Value::U8(1)),
            ("suppress-sound", Value::Bool(true)),
        ]);
        self.proxy
            .notify(
                "Katna Mail",
                0,
                ids::MAIL_APP_ID,
                summary,
                body,
                &actions,
                hints,
                -1,
            )
            .await
    }

    /// Shows a reminder the user asked for (mail back from snooze, a
    /// message nobody replied to) about mail of the account `origin`: `summary`
    /// over `lines`, with Open, Mark as read and Archive. Returns its ID.
    pub async fn reminder(
        &self,
        origin: &str,
        summary: &str,
        lines: &[String],
        sound: bool,
    ) -> zbus::Result<u32> {
        let body = lines
            .iter()
            .map(|line| escape(&shorten(line, PREVIEW_CHARS)))
            .collect::<Vec<_>>()
            .join("\n");
        let labels = [
            (action::OPEN, tr!("notify-open")),
            (action::MARK_READ, tr!("notify-mark-read")),
            (action::ARCHIVE, tr!("notify-archive")),
        ];
        let actions: Vec<&str> = labels
            .iter()
            .flat_map(|(key, label)| [*key, label.as_str()])
            .collect();
        let mut hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("email")),
            ("x-kde-origin-name", Value::from(origin)),
            ("urgency", Value::U8(1)),
        ]);
        if sound {
            hints.insert("sound-name", Value::from("message-new-email"));
        } else {
            hints.insert("suppress-sound", Value::Bool(true));
        }
        self.proxy
            .notify(
                "Katna Mail",
                0,
                ids::MAIL_APP_ID,
                summary,
                &body,
                &actions,
                hints,
                -1,
            )
            .await
    }

    /// Reminds of a calendar event: `summary` (its title) over `lines`
    /// (when and where), with Join when it has a video call, and Snooze.
    /// It stays until dismissed, as calendar reminders do. Returns its ID.
    pub async fn event_reminder(
        &self,
        summary: &str,
        lines: &[String],
        join: bool,
        sound: bool,
    ) -> zbus::Result<u32> {
        let body = lines
            .iter()
            .map(|line| escape(&shorten(line, PREVIEW_CHARS)))
            .collect::<Vec<_>>()
            .join("\n");
        let mut labels = vec![(action::OPEN, tr!("notify-open"))];
        if join {
            labels.push((action::JOIN, tr!("notify-event-join")));
        }
        labels.push((action::SNOOZE, tr!("notify-event-snooze")));
        let actions: Vec<&str> = labels
            .iter()
            .flat_map(|(key, label)| [*key, label.as_str()])
            .collect();
        let mut hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("x-katna.event")),
            ("urgency", Value::U8(1)),
            ("resident", Value::Bool(false)),
        ]);
        if sound {
            hints.insert("sound-name", Value::from("alarm-clock-elapsed"));
        } else {
            hints.insert("suppress-sound", Value::Bool(true));
        }
        self.proxy
            .notify(
                "Katna Calendar",
                0,
                ids::MAIL_APP_ID,
                summary,
                &body,
                &actions,
                hints,
                0,
            )
            .await
    }

    /// Reminds of a task: `summary` (its title) over `lines` (the start of
    /// its details), with Mark as done and Snooze. It stays until
    /// dismissed, as event reminders do. Returns its ID.
    pub async fn task_reminder(
        &self,
        summary: &str,
        lines: &[String],
        sound: bool,
    ) -> zbus::Result<u32> {
        let body = lines
            .iter()
            .map(|line| escape(&shorten(line, PREVIEW_CHARS)))
            .collect::<Vec<_>>()
            .join("\n");
        let labels = [
            (action::OPEN, tr!("notify-open")),
            (action::DONE, tr!("notify-task-done")),
            (action::SNOOZE, tr!("notify-event-snooze")),
        ];
        let actions: Vec<&str> = labels
            .iter()
            .flat_map(|(key, label)| [*key, label.as_str()])
            .collect();
        let mut hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("x-katna.task")),
            ("urgency", Value::U8(1)),
            ("resident", Value::Bool(false)),
        ]);
        if sound {
            hints.insert("sound-name", Value::from("alarm-clock-elapsed"));
        } else {
            hints.insert("suppress-sound", Value::Bool(true));
        }
        self.proxy
            .notify(
                "Katna Tasks",
                0,
                ids::MAIL_APP_ID,
                summary,
                &body,
                &actions,
                hints,
                0,
            )
            .await
    }

    /// Says that Katna `version` is downloaded, with an Update button that
    /// installs it. Returns its ID.
    pub async fn update_ready(&self, version: &str) -> zbus::Result<u32> {
        let update = tr!("notify-update");
        let actions = [
            action::OPEN,
            update.as_str(),
            action::UPDATE,
            update.as_str(),
        ];
        let hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("x-katna.update")),
            ("urgency", Value::U8(1)),
            ("suppress-sound", Value::Bool(true)),
        ]);
        self.proxy
            .notify(
                "Katna Mail",
                0,
                ids::MAIL_APP_ID,
                &tr!("notify-update-ready"),
                &escape(&tr!("notify-update-ready-body", version = version)),
                &actions,
                hints,
                0,
            )
            .await
    }

    /// Says, quietly and for a few seconds, that mail was archived from
    /// a notification: `subject` for one message, else how many, with
    /// Undo. Returns its ID.
    pub async fn archived(&self, subject: Option<&str>, count: usize) -> zbus::Result<u32> {
        let body = match subject {
            Some(subject) if count == 1 && !subject.trim().is_empty() => escape(subject),
            Some(_) if count == 1 => tr!("notify-no-subject"),
            _ => escape(&tr!("notify-archived-count", count = count)),
        };
        let undo = tr!("notify-undo");
        let actions = [action::UNDO, undo.as_str()];
        let hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("email")),
            ("urgency", Value::U8(0)),
            ("suppress-sound", Value::Bool(true)),
            // Gone from the history once it closes: it only confirms.
            ("transient", Value::Bool(true)),
        ]);
        self.proxy
            .notify(
                "Katna Mail",
                0,
                ids::MAIL_APP_ID,
                &tr!("notify-archived"),
                &body,
                &actions,
                hints,
                ARCHIVED_SHOWN_MS,
            )
            .await
    }

    pub async fn close(&self, id: u32) -> zbus::Result<()> {
        self.proxy.close_notification(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mail(sender: &str, subject: &str) -> NewMail {
        NewMail {
            sender: sender.into(),
            subject: subject.into(),
            preview: None,
        }
    }

    #[test]
    fn one_message_shows_sender_subject_and_preview() {
        let mut one = mail("Carol", "Q3 <draft> & notes");
        one.preview = Some("Numbers   attached.\nSee you.".into());
        assert_eq!(
            new_mail_text(&[one]),
            (
                "Carol".into(),
                "Q3 &lt;draft&gt; &amp; notes\nNumbers attached. See you.".into()
            )
        );
        assert_eq!(new_mail_text(&[mail("Bob", " ")]).1, "(no subject)");
    }

    #[test]
    fn a_burst_is_one_notification() {
        let mails: Vec<_> = (0..6)
            .map(|i| mail("Acme", &format!("Order {i}")))
            .collect();
        let (summary, body) = new_mail_text(&mails);
        assert_eq!(summary, "6 new emails");
        assert_eq!(
            body,
            "Acme: Order 0\nAcme: Order 1\nAcme: Order 2\nAcme: Order 3\nand 2 more"
        );
    }

    #[test]
    fn long_previews_are_cut() {
        let long = "word ".repeat(100);
        let cut = shorten(&long, 20);
        assert_eq!(cut, "word word word word…");
    }
}
