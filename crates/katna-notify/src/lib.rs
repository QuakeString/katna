// SPDX-License-Identifier: GPL-3.0-or-later

//! Desktop notifications with actions and inline reply.
//! See `docs/ARCHITECTURE.md` §15.1.
//!
//! Talks to the desktop's `org.freedesktop.Notifications` server directly
//! (Plasma, GNOME Shell, mako, dunst, …). So far: new-mail notifications
//! with Peek, Reply (typed into the notification where the server can,
//! §15.1.2), Mark as read and Archive, a button that copies a one-time
//! code or opens a verify link (§15.1.3), the note that a reply typed there
//! is on its way, with Undo, reminders (snooze,
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
    /// Only on a notification about one message: show more of it, in the
    /// same notification.
    pub const PEEK: &str = "peek";
    /// Only on a notification about one message, where the server takes
    /// replies typed into it: Plasma's key for its reply field. The text
    /// comes back in `NotificationReplied`.
    pub const INLINE_REPLY: &str = "inline-reply";
    /// Only on a notification about one message, where the server takes no
    /// typed replies: Katna Mail's reply window.
    pub const REPLY: &str = "reply";
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
    /// back in the inbox; on the note that a reply is on its way: keep it
    /// from going, and write it on in Katna Mail.
    pub const UNDO: &str = "undo";
    /// On the note that a reply is on its way: show the conversation in
    /// Katna Mail.
    pub const SHOW: &str = "show";
    /// Only on a notification about one message with a one-time code:
    /// copy the code.
    pub const COPY_CODE: &str = "copy-code";
    /// Only on a notification about one message with a verify, confirm
    /// or activate link: open it in the browser.
    pub const OPEN_LINK: &str = "open-link";
}

/// At most this many messages are listed in a grouped notification.
const LISTED: usize = 4;
/// Longest preview of a single message's text, in characters.
const PREVIEW_CHARS: usize = 160;
/// Longest text of a peeked message, in characters.
const PEEK_CHARS: usize = 1200;
/// How long the note that mail was archived stays, in milliseconds.
const ARCHIVED_SHOWN_MS: i32 = 8000;
/// How long the note that a code was copied stays, in milliseconds.
const COPIED_SHOWN_MS: i32 = 4000;
/// The server capability of replies typed into a notification.
const INLINE_REPLY_CAPABILITY: &str = "inline-reply";

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

    /// A reply typed into notification `id` (Plasma's inline reply).
    #[zbus(signal)]
    fn notification_replied(&self, id: u32, text: &str) -> zbus::Result<()>;
}

/// One new message, as a notification shows it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NewMail {
    /// Display name, or the address when there is none.
    pub sender: String,
    pub subject: String,
    /// The start of the text, when the body is downloaded.
    pub preview: Option<String>,
    /// Its button for a one-time code or a verify link, if any.
    pub shortcut: Option<Shortcut>,
}

/// What a notification about one message offers besides reading and
/// answering it (`docs/ARCHITECTURE.md` §15.1.3). A link's button names
/// where it goes, so a link that pretends to be someone else shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shortcut {
    /// Copy this one-time code.
    Code(String),
    /// Open a link to verify an address, on `domain`.
    Verify { domain: String },
    /// Open a link to confirm something, on `domain`.
    Confirm { domain: String },
    /// Open a link to activate an account, on `domain`.
    Activate { domain: String },
}

impl Shortcut {
    /// Its button: action key and label.
    fn action(&self) -> (&'static str, String) {
        match self {
            Shortcut::Code(code) => (
                action::COPY_CODE,
                tr!("notify-copy-code", code = code.clone()),
            ),
            Shortcut::Verify { domain } => (
                action::OPEN_LINK,
                tr!("notify-link-verify", domain = domain.clone()),
            ),
            Shortcut::Confirm { domain } => (
                action::OPEN_LINK,
                tr!("notify-link-confirm", domain = domain.clone()),
            ),
            Shortcut::Activate { domain } => (
                action::OPEN_LINK,
                tr!("notify-link-activate", domain = domain.clone()),
            ),
        }
    }
}

/// How a new-mail notification shows its mail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View<'a> {
    /// Sender, subject and the start of the text, with Peek.
    Short,
    /// One message peeked at: `text` is its body, as much as fits.
    Peek { text: &'a str },
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

/// Body of a peeked message: its subject, then `text` with its line
/// breaks, blank lines run together and cut at [`PEEK_CHARS`].
pub fn peek_text(mail: &NewMail, text: &str) -> String {
    let subject = if mail.subject.trim().is_empty() {
        tr!("notify-no-subject")
    } else {
        mail.subject.clone()
    };
    let mut lines: Vec<&str> = Vec::new();
    for line in text.lines().map(str::trim_end) {
        if line.trim().is_empty() && lines.last().is_none_or(|l| l.is_empty()) {
            continue;
        }
        lines.push(if line.trim().is_empty() { "" } else { line });
    }
    let text = lines.join("\n");
    let text = match text.char_indices().nth(PEEK_CHARS) {
        Some((at, _)) => format!("{}…", text[..at].trim_end()),
        None => text,
    };
    let text = text.trim();
    if text.is_empty() {
        return escape(&subject);
    }
    format!("{}\n\n{}", escape(&subject), escape(text))
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

/// What a new-mail notification's actions read as: key, then label.
type Actions = Vec<(&'static str, String)>;

impl Notifier {
    pub async fn new(connection: &zbus::Connection) -> zbus::Result<Self> {
        Ok(Self {
            proxy: NotificationsProxy::new(connection).await?,
        })
    }

    /// Whether the server takes replies typed into a notification. Asked
    /// each time: the desktop's server can be restarted, or replaced.
    pub async fn takes_replies(&self) -> bool {
        self.proxy
            .get_capabilities()
            .await
            .is_ok_and(|caps| caps.iter().any(|c| c == INLINE_REPLY_CAPABILITY))
    }

    /// The buttons of a notification about `mails` shown as `view`.
    /// Peek needs a notification that can grow: not a Windows toast.
    fn new_mail_actions(mails: &[NewMail], view: View<'_>, replies: bool) -> Actions {
        let mut actions = vec![(action::OPEN, tr!("notify-open"))];
        let reply = if replies {
            (action::INLINE_REPLY, tr!("notify-reply"))
        } else {
            (action::REPLY, tr!("notify-reply"))
        };
        // A code or a link takes Reply all's place in a peek, and Reply's
        // where Peek offers it: such mail is rarely answered, and four
        // buttons are as many as fit.
        let shortcut = match mails {
            [mail] => mail.shortcut.as_ref().map(Shortcut::action),
            _ => None,
        };
        match (mails.len(), view) {
            (1, View::Peek { .. }) => {
                actions.extend(shortcut);
                actions.push(reply);
                if actions.len() < 3 {
                    actions.push((action::REPLY_ALL, tr!("notify-reply-all")));
                }
                actions.push((action::ARCHIVE, tr!("notify-archive")));
            }
            (1, View::Short) => {
                let peek = !cfg!(windows);
                if peek {
                    actions.push((action::PEEK, tr!("notify-peek")));
                }
                match shortcut {
                    Some(shortcut) if peek => actions.push(shortcut),
                    Some(shortcut) => actions.extend([shortcut, reply]),
                    None => actions.push(reply),
                }
                actions.extend([
                    (action::MARK_READ, tr!("notify-mark-read")),
                    (action::ARCHIVE, tr!("notify-archive")),
                ]);
            }
            _ => actions.extend([
                (action::MARK_READ, tr!("notify-mark-all-read")),
                (action::ARCHIVE, tr!("notify-archive")),
            ]),
        }
        actions
    }

    /// The server's proxy, for its signals.
    pub fn proxy(&self) -> &NotificationsProxy<'static> {
        &self.proxy
    }

    /// Shows `mails` (not empty) of the account `origin` as `view`,
    /// replacing notification `replaces` if not 0, with `sound` (a
    /// `katna_platform::sound` name the server plays) or silently. Reply
    /// is typed into the notification when `replies` (see
    /// [`Self::takes_replies`]). Returns its ID.
    pub async fn new_mail(
        &self,
        origin: &str,
        mails: &[NewMail],
        view: View<'_>,
        replies: bool,
        replaces: u32,
        sound: Option<&str>,
    ) -> zbus::Result<u32> {
        let (summary, body) = match (mails, view) {
            ([mail], View::Peek { text }) => (mail.sender.clone(), peek_text(mail, text)),
            _ => new_mail_text(mails),
        };
        let labels = Self::new_mail_actions(mails, view, replies);
        // Key, label, key, label, … as the specification has them.
        let actions: Vec<&str> = labels
            .iter()
            .flat_map(|(key, label)| [*key, label.as_str()])
            .collect();
        let mut hints = HashMap::from([
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
            ("category", Value::from("email.arrived")),
            ("x-kde-origin-name", Value::from(origin)),
            ("urgency", Value::U8(1)),
            // Peek replaces the notification in place: it must still be
            // there once its button is pressed. Katna closes it after the
            // other buttons.
            ("resident", Value::Bool(true)),
        ]);
        if let ([mail], true) = (mails, replies) {
            hints.extend([
                (
                    "x-kde-reply-placeholder-text",
                    Value::from(tr!("notify-reply-placeholder", name = mail.sender.clone())),
                ),
                (
                    "x-kde-reply-submit-button-text",
                    Value::from(tr!("notify-send")),
                ),
                (
                    "x-kde-reply-submit-button-icon-name",
                    Value::from("document-send"),
                ),
            ]);
        }
        if let Some(sound) = sound {
            hints.insert("sound-name", Value::from(sound));
        } else {
            hints.insert("suppress-sound", Value::Bool(true));
        }
        // A peek stays until closed: it was asked for, to be read.
        let timeout = if matches!(view, View::Peek { .. }) {
            0
        } else {
            -1
        };
        self.proxy
            .notify(
                "Katna Mail",
                replaces,
                ids::MAIL_APP_ID,
                &summary,
                &body,
                &actions,
                hints,
                timeout,
            )
            .await
    }

    /// Says, in place of notification `replaces` if not 0, that the reply `text`
    /// to `name` is on its way: for `undo_seconds` with Undo (none for
    /// 0), and Open in Katna. Returns its ID.
    pub async fn reply_sent(
        &self,
        name: &str,
        text: &str,
        undo_seconds: u32,
        replaces: u32,
    ) -> zbus::Result<u32> {
        let mut labels = Vec::new();
        if undo_seconds > 0 {
            labels.push((action::UNDO, tr!("notify-undo")));
        }
        labels.push((action::SHOW, tr!("notify-open-in-katna")));
        let actions: Vec<&str> = labels
            .iter()
            .flat_map(|(key, label)| [*key, label.as_str()])
            .collect();
        let hints = HashMap::from([
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
            ("category", Value::from("email")),
            ("urgency", Value::U8(0)),
            ("suppress-sound", Value::Bool(true)),
            ("transient", Value::Bool(true)),
        ]);
        let shown_ms = i32::try_from(undo_seconds.max(5).saturating_mul(1000)).unwrap_or(i32::MAX);
        self.proxy
            .notify(
                "Katna Mail",
                replaces,
                ids::MAIL_APP_ID,
                &tr!("notify-reply-sent", name = name),
                &escape(&shorten(text, PREVIEW_CHARS)),
                &actions,
                hints,
                shown_ms,
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
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
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
        sound: Option<&str>,
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
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
            ("category", Value::from("email")),
            ("x-kde-origin-name", Value::from(origin)),
            ("urgency", Value::U8(1)),
        ]);
        if let Some(sound) = sound {
            hints.insert("sound-name", Value::from(sound));
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
        sound: Option<&str>,
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
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
            ("category", Value::from("x-katna.event")),
            ("urgency", Value::U8(1)),
            ("resident", Value::Bool(false)),
        ]);
        if let Some(sound) = sound {
            hints.insert("sound-name", Value::from(sound));
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
        sound: Option<&str>,
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
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
            ("category", Value::from("x-katna.task")),
            ("urgency", Value::U8(1)),
            ("resident", Value::Bool(false)),
        ]);
        if let Some(sound) = sound {
            hints.insert("sound-name", Value::from(sound));
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
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
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
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
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

    /// Says, quietly and for a few seconds, that `code` was copied, or
    /// shows it to copy by hand when it could not be. Returns its ID.
    pub async fn code_copied(&self, code: &str, copied: bool) -> zbus::Result<u32> {
        let summary = if copied {
            tr!("notify-code-copied")
        } else {
            tr!("notify-code-not-copied")
        };
        let hints = HashMap::from([
            ("desktop-entry", Value::from(ids::NOTIFICATIONS_DESKTOP_ID)),
            ("category", Value::from("email")),
            ("urgency", Value::U8(0)),
            ("suppress-sound", Value::Bool(true)),
            ("transient", Value::Bool(true)),
        ]);
        self.proxy
            .notify(
                "Katna Mail",
                0,
                ids::MAIL_APP_ID,
                &summary,
                &escape(code),
                &[],
                hints,
                if copied { COPIED_SHOWN_MS } else { 0 },
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
            shortcut: None,
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
    fn a_peek_keeps_paragraphs() {
        let one = mail("Alex", "Q3 & plans");
        assert_eq!(
            peek_text(&one, "Hi Kay,\n\n\n\nCan we <meet>?  \nAt 3\n\n"),
            "Q3 &amp; plans\n\nHi Kay,\n\nCan we &lt;meet&gt;?\nAt 3"
        );
        assert_eq!(peek_text(&one, "  "), "Q3 &amp; plans");
        let long = peek_text(&one, &"word ".repeat(1000));
        assert!(long.ends_with('…') && long.chars().count() < PEEK_CHARS + 20);
    }

    #[test]
    fn one_message_has_peek_and_reply() {
        let keys = |actions: Actions| actions.into_iter().map(|(k, _)| k).collect::<Vec<_>>();
        let one = [mail("Alex", "Hi")];
        let short = keys(Notifier::new_mail_actions(&one, View::Short, true));
        let peek = if cfg!(windows) {
            None
        } else {
            Some(action::PEEK)
        };
        let expected: Vec<&str> = [Some(action::OPEN), peek, Some(action::INLINE_REPLY)]
            .into_iter()
            .flatten()
            .chain([action::MARK_READ, action::ARCHIVE])
            .collect();
        assert_eq!(short, expected);
        assert_eq!(
            keys(Notifier::new_mail_actions(
                &one,
                View::Peek { text: "" },
                false
            )),
            [
                action::OPEN,
                action::REPLY,
                action::REPLY_ALL,
                action::ARCHIVE
            ]
        );
        let two = [mail("Alex", "Hi"), mail("Bo", "Yo")];
        assert_eq!(
            keys(Notifier::new_mail_actions(&two, View::Short, true)),
            [action::OPEN, action::MARK_READ, action::ARCHIVE]
        );
    }

    #[test]
    fn a_code_or_link_has_its_own_button() {
        let keys = |actions: Actions| actions.into_iter().map(|(k, _)| k).collect::<Vec<_>>();
        let mut one = mail("Acme", "Your code");
        one.shortcut = Some(Shortcut::Code("123456".into()));
        let short = Notifier::new_mail_actions(std::slice::from_ref(&one), View::Short, true);
        assert!(short.iter().any(|(_, label)| label.contains("123456")));
        let expected: &[&str] = if cfg!(windows) {
            &[
                action::OPEN,
                action::COPY_CODE,
                action::INLINE_REPLY,
                action::MARK_READ,
                action::ARCHIVE,
            ]
        } else {
            &[
                action::OPEN,
                action::PEEK,
                action::COPY_CODE,
                action::MARK_READ,
                action::ARCHIVE,
            ]
        };
        assert_eq!(keys(short), expected);
        one.shortcut = Some(Shortcut::Verify {
            domain: "acme.example".into(),
        });
        let peek = Notifier::new_mail_actions(&[one], View::Peek { text: "" }, false);
        assert!(peek.iter().any(|(_, label)| label.contains("acme.example")));
        assert_eq!(
            keys(peek),
            [
                action::OPEN,
                action::OPEN_LINK,
                action::REPLY,
                action::ARCHIVE
            ]
        );
    }

    #[test]
    fn long_previews_are_cut() {
        let long = "word ".repeat(100);
        let cut = shorten(&long, 20);
        assert_eq!(cut, "word word word word…");
    }
}
