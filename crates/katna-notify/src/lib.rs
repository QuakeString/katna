// SPDX-License-Identifier: GPL-3.0-or-later

//! Desktop notifications with actions and inline reply.
//! See `docs/ARCHITECTURE.md` §15.1.
//!
//! Talks to the desktop's `org.freedesktop.Notifications` server directly
//! (Plasma, GNOME Shell, mako, dunst, …). So far: new-mail notifications
//! with Open, Reply all, Mark as read and Archive.

use std::collections::HashMap;

use katna_core::ids;
use zbus::zvariant::Value;

/// Actions on a new-mail notification, as `ActionInvoked` names them.
pub mod action {
    /// A click on the notification itself.
    pub const OPEN: &str = "default";
    /// Only on a notification about one message.
    pub const REPLY_ALL: &str = "reply-all";
    pub const MARK_READ: &str = "mark-read";
    pub const ARCHIVE: &str = "archive";
}

/// At most this many messages are listed in a grouped notification.
const LISTED: usize = 4;
/// Longest preview of a single message's text, in characters.
const PREVIEW_CHARS: usize = 160;

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

/// Summary and body of a notification for `mails` (not empty).
pub fn new_mail_text(mails: &[NewMail]) -> (String, String) {
    let subject = |mail: &NewMail| {
        if mail.subject.trim().is_empty() {
            "(no subject)".to_owned()
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
    let summary = format!("{} new emails", mails.len());
    let mut lines: Vec<String> = mails
        .iter()
        .take(LISTED)
        .map(|mail| escape(&format!("{}: {}", mail.sender, subject(mail))))
        .collect();
    if mails.len() > LISTED {
        lines.push(format!("and {} more", mails.len() - LISTED));
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
    /// replacing notification `replaces` if not 0. Returns its ID.
    pub async fn new_mail(
        &self,
        origin: &str,
        mails: &[NewMail],
        replaces: u32,
    ) -> zbus::Result<u32> {
        let (summary, body) = new_mail_text(mails);
        let mark_read = if mails.len() == 1 {
            "Mark as read"
        } else {
            "Mark all as read"
        };
        let mut actions = vec![action::OPEN, "Open"];
        if mails.len() == 1 {
            actions.extend([action::REPLY_ALL, "Reply all"]);
        }
        actions.extend([action::MARK_READ, mark_read, action::ARCHIVE, "Archive"]);
        let hints = HashMap::from([
            ("desktop-entry", Value::from(ids::MAIL_APP_ID)),
            ("category", Value::from("email.arrived")),
            ("sound-name", Value::from("message-new-email")),
            ("x-kde-origin-name", Value::from(origin)),
            ("urgency", Value::U8(1)),
        ]);
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
