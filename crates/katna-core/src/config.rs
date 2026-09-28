// SPDX-License-Identifier: GPL-3.0-or-later

//! User settings, stored as TOML in `$XDG_CONFIG_HOME/katna/config.toml`.
//!
//! Every field has a default, so a missing file or a missing key is not an
//! error. Unknown keys are ignored, so an older Katna can read a file written
//! by a newer one.

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Allowed undo-send delays in seconds (`docs/ARCHITECTURE.md` §10).
pub const UNDO_SEND_CHOICES: [u32; 5] = [0, 5, 10, 20, 30];

/// [`Sending::send_from`] when new mail goes out from the account whose mail
/// is open.
pub const SEND_FROM_CURRENT: &str = "current";

/// All user settings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: General,
    pub logging: Logging,
    pub sending: Sending,
    pub mail: MailView,
    pub shortcuts: Shortcuts,
    pub sync: SyncConfig,
    pub notifications: Notifications,
    pub onboarding: Onboarding,
    pub experimental: Experimental,
    pub feedback: Feedback,
}

/// Settings > User feedback: crash reports (`docs/ARCHITECTURE.md` §19.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Feedback {
    /// Write a report on this computer when a Katna program crashes.
    pub save_crash_reports: bool,
    /// Send new crash reports to Katna's crash tracker. `None` until the
    /// user has answered "Help improve Katna"; nothing is sent unless it
    /// is `Some(true)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_crash_reports: Option<bool>,
    /// Where crash reports go instead of Katna's own Sentry project
    /// ([`crate::ids::SENTRY_DSN`]): a self-hosted Sentry or GlitchTip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dsn: Option<String>,
}

impl Feedback {
    /// Whether crash reports are sent.
    pub fn sending(&self) -> bool {
        self.save_crash_reports && self.send_crash_reports == Some(true)
    }

    /// The Sentry DSN reports go to; empty means nowhere.
    pub fn dsn(&self) -> &str {
        self.dsn.as_deref().unwrap_or(crate::ids::SENTRY_DSN).trim()
    }
}

impl Default for Feedback {
    fn default() -> Self {
        Self {
            save_crash_reports: true,
            send_crash_reports: None,
            dsn: None,
        }
    }
}

/// Settings > Experimental: features still being tried out.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Experimental {
    /// Who draws the window frame.
    pub window_frame: WindowFrame,
    /// A translucent window background that the compositor blurs.
    pub blur: bool,
}

/// [`Experimental::window_frame`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowFrame {
    /// The desktop's choice: KDE's own frame on KDE, Katna's on GNOME.
    #[default]
    Native,
    /// Katna's own title bar, buttons, rounded corners and shadow.
    Katna,
}

/// First-run help in Katna Mail, and What's new after an update.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Onboarding {
    /// The welcome and the offer of a tour of the window have been seen.
    pub done: bool,
    /// The What's new highlights shown, or offered by the first start, by
    /// name (`2026-09-27-0444-about-katna`), so one merged after newer ones
    /// still shows.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub whats_new_shown: Vec<String>,
    /// The number of the newest highlight shown, from the versions that
    /// numbered them. Read once, then replaced by `whats_new_shown`.
    pub whats_new_seen: Option<u32>,
    /// The version of Katna Mail that started last, as its package names
    /// it, for the link to the changes since.
    pub last_version: Option<String>,
}

/// Desktop notifications from `katna-daemon` (`docs/ARCHITECTURE.md` §15.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Notifications {
    /// Notify about new mail in the inbox (Primary tab).
    pub new_mail: bool,
    /// New-mail notifications play the desktop's new-mail sound.
    pub sound: bool,
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            new_mail: true,
            sound: true,
        }
    }
}

/// How `katna-daemon` syncs (`docs/ARCHITECTURE.md` §6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SyncConfig {
    /// Whether to save data as on a metered network: no bodies downloaded
    /// ahead of time.
    pub metered: Metered,
    /// Mail of the last this many days is downloaded whole ahead of time,
    /// for reading offline; 0 for all mail. Older mail downloads when it
    /// is opened. Lowering it keeps what is already downloaded.
    pub offline_days: u32,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            metered: Metered::default(),
            offline_days: 30,
        }
    }
}

impl SyncConfig {
    /// [`Self::offline_days`] as a window: `None` for all mail.
    pub fn offline_window(&self) -> Option<u32> {
        (self.offline_days > 0).then_some(self.offline_days)
    }
}

/// [`SyncConfig::metered`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Metered {
    /// Ask NetworkManager.
    #[default]
    Auto,
    /// Always, for example on a phone hotspot NetworkManager does not know.
    Always,
    /// Never, whatever NetworkManager says.
    Never,
}

impl Metered {
    /// Whether to act metered when NetworkManager says `network`.
    pub fn decide(self, network: bool) -> bool {
        match self {
            Self::Auto => network,
            Self::Always => true,
            Self::Never => false,
        }
    }
}

/// Background-service behavior (`docs/ARCHITECTURE.md` §9.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct General {
    /// Keep `katna-daemon` running when no window is open.
    pub run_in_background: bool,
    /// Show a tray icon with the unread count and a menu (§15.2).
    ///
    /// Not `tray_icon`: versions before the tray existed saved
    /// `tray_icon = false` as their default into every config file the app
    /// wrote, which would keep the icon hidden. That key is ignored.
    pub show_in_tray: bool,
    /// Show the Inbox unread count on Katna Mail's taskbar or dock icon.
    pub unread_badge: bool,
    /// The language of the interface, a tag from `i18n/languages.toml`
    /// (`bn`, `en-IN`); empty follows the desktop (§13.10).
    pub language: String,
    /// 12- or 24-hour times, or as the language writes them.
    pub clock: Clock,
    /// Katna Mail has set up starting at login once (on by default). The
    /// autostart entry is the setting itself; this only stops the default
    /// from coming back after it was turned off.
    pub start_at_login_set: bool,
    /// Words that, typed first in KRunner or GNOME's search with a space
    /// after them, search the mail as Katna Mail's search box does
    /// (`k budget`); `mail:` always does (§15.3).
    pub search_triggers: Vec<String>,
}

impl General {
    /// [`General::search_triggers`] from what was typed in Settings: words
    /// apart by commas or spaces, each once, in lowercase.
    pub fn parse_search_triggers(text: &str) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        for word in text.split(|c: char| c == ',' || c.is_whitespace()) {
            let word = word.trim_end_matches(':').to_lowercase();
            if !word.is_empty() && !words.contains(&word) {
                words.push(word);
            }
        }
        words
    }
}

/// How times show ([`General::clock`]).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Clock {
    /// As the language (its formats) writes them.
    #[default]
    #[serde(rename = "language")]
    Language,
    /// `2:05 PM`.
    #[serde(rename = "12-hour")]
    TwelveHour,
    /// `14:05`.
    #[serde(rename = "24-hour")]
    TwentyFourHour,
}

impl Default for General {
    fn default() -> Self {
        Self {
            run_in_background: true,
            show_in_tray: true,
            unread_badge: true,
            language: String::new(),
            clock: Clock::Language,
            start_at_login_set: false,
            search_triggers: vec!["k".to_owned(), "m".to_owned()],
        }
    }
}

/// Logging settings. `$KATNA_LOG` overrides [`Logging::filter`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Logging {
    /// A `tracing` filter such as `info` or `warn,katna_sync=debug`.
    pub filter: String,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            filter: "info".to_owned(),
        }
    }
}

/// Sending settings (`docs/ARCHITECTURE.md` §11).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sending {
    /// Undo-send delay in seconds; one of [`UNDO_SEND_CHOICES`].
    pub undo_send_seconds: u32,
    /// The one signature of older versions. Read once and moved into
    /// [`Sending::signatures`]; never written.
    #[serde(skip_serializing)]
    pub signature: String,
    /// Signatures, added below the message after a "-- " line.
    pub signatures: Vec<Signature>,
    /// The [`Signature::id`] new mail starts with; `None` for none.
    pub new_mail_signature: Option<u32>,
    /// The [`Signature::id`] replies and forwards start with, unless the
    /// conversation shows which one the user signed with before.
    pub reply_signature: Option<u32>,
    /// New messages open full screen.
    pub compose_full_screen: bool,
    /// Messages are written and sent as plain text, without formatting.
    pub plain_text: bool,
    /// Misspelled words are underlined while writing.
    pub spell_check: bool,
    /// The dictionary, as `en_US`; empty for the desktop's language.
    pub spell_language: String,
    /// Grammar mistakes are underlined while writing, in English.
    pub grammar_check: bool,
    /// The likely rest of a phrase shows grey ahead of the cursor while
    /// writing, learned on this computer from the user's sent mail.
    pub writing_suggestions: bool,
    /// The address new mail is sent from by default; empty for the first
    /// account, [`SEND_FROM_CURRENT`] for the account whose mail is open.
    /// Replies go out from the account they answer.
    pub send_from: String,
    /// Send on replies and forwards also archives the conversation; the
    /// Send menu offers the other way.
    pub send_and_archive: bool,
    /// A short sound plays when a message has gone out.
    pub sent_sound: bool,
}

impl Default for Sending {
    fn default() -> Self {
        Self {
            undo_send_seconds: 10,
            signature: String::new(),
            signatures: Vec::new(),
            new_mail_signature: None,
            reply_signature: None,
            compose_full_screen: false,
            plain_text: false,
            spell_check: true,
            spell_language: String::new(),
            grammar_check: true,
            writing_suggestions: true,
            send_from: String::new(),
            send_and_archive: false,
            sent_sound: true,
        }
    }
}

impl Sending {
    pub fn signature(&self, id: Option<u32>) -> Option<&Signature> {
        id.and_then(|id| self.signatures.iter().find(|s| s.id == id))
    }

    /// Adds a signature and returns its ID. The first one becomes the
    /// default for new mail and replies.
    pub fn add_signature(&mut self, name: String, text: String) -> u32 {
        let id = self.signatures.iter().map(|s| s.id).max().unwrap_or(0) + 1;
        self.signatures.push(Signature {
            id,
            name,
            text,
            html: String::new(),
        });
        if self.signatures.len() == 1 {
            self.new_mail_signature = Some(id);
            self.reply_signature = Some(id);
        }
        id
    }

    /// Removes a signature, and it as a default.
    pub fn remove_signature(&mut self, id: u32) {
        self.signatures.retain(|s| s.id != id);
        for default in [&mut self.new_mail_signature, &mut self.reply_signature] {
            if *default == Some(id) {
                *default = None;
            }
        }
    }

    /// Moves the signature of older versions into the list.
    fn upgrade(&mut self) {
        let old = std::mem::take(&mut self.signature);
        if !old.trim().is_empty() && self.signatures.is_empty() {
            self.add_signature("My signature".to_owned(), old);
        }
    }
}

/// A named signature ([`Sending::signatures`]).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Signature {
    /// Stable within the file; defaults refer to it.
    pub id: u32,
    pub name: String,
    /// The signature as plain text (what plain text mail carries).
    pub text: String,
    /// The signature with its formatting, as HTML with pictures inside as
    /// `data:` URIs; empty for a plain text one.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub html: String,
}

/// How Katna Mail shows mail (its quick settings).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MailView {
    /// Where an opened message shows.
    pub reading_pane: ReadingPane,
    /// Share of the width the reading pane takes beside the list
    /// (0.25 to 0.75).
    pub reading_pane_share: f32,
    /// List conversations instead of single messages.
    pub conversations: bool,
    /// Split the inbox into category tabs (Primary, Promotions, ...).
    /// Off turns them off for every account.
    pub inbox_tabs: bool,
    /// Which tabs each account's inbox has, by lower-case address.
    /// Accounts not listed use [`TabStyle::Auto`].
    pub account_tabs: BTreeMap<String, AccountTabs>,
    pub density: Density,
    /// The size of everything in the windows, in percent, on top of the
    /// desktop's own scale (75 to 200).
    pub scale: u16,
    pub theme: Theme,
    /// Use the desktop's color scheme and accent color instead of Katna's
    /// own colors.
    pub desktop_colors: bool,
    /// Show the names under the icons of the app bar (Mail, Calendar, ...).
    pub app_labels: bool,
    /// Show the logo of each sender's organization (its BIMI logo or
    /// website icon) in place of their initial.
    pub sender_pictures: bool,
    /// Show a conversation with its newest message at the top.
    pub newest_first: bool,
    /// Show the contact panel beside an open conversation, in windows wide
    /// enough for it: the sender's mail, files and signature details.
    pub contact_panel: bool,
    /// Open each message with its full headers (from, to, cc, date and
    /// subject) shown.
    pub full_headers: bool,
    /// Name every recipient in full in the "to" line, instead of by first
    /// name.
    pub full_names: bool,
    /// Where each kind of attachment opens.
    pub open: OpenAttachments,
    /// When an opened conversation is marked read.
    pub mark_read: MarkRead,
    /// What opens after the open conversation is deleted, archived or
    /// moved away.
    pub auto_advance: AutoAdvance,
    /// Ask before deleting two or more conversations at once. Deleting for
    /// good (in Trash, or on an account without one) always asks.
    pub confirm_delete: bool,
    /// Load the images of every message from the web, not only those of
    /// trusted senders. Loading them tells senders that a message was read.
    pub remote_images: bool,
    /// The reply button of each message in a conversation replies to
    /// everyone, not only the sender.
    pub reply_all: bool,
    /// Show the Important marker in the message list.
    pub important_markers: bool,
    /// Keep the lines of a message no wider than is easy to read.
    pub limit_width: bool,
    /// In a dark theme, give HTML mail dark colors too; off, mail keeps the
    /// colors its sender picked, on a light page.
    pub dark_mail: bool,
    /// Show a small picture of each attachment's content on its card.
    pub attachment_previews: bool,
    /// Open the folder in the file manager after saving attachments.
    pub open_saved_folder: bool,
    /// With several accounts: the folder pane shows one account, picked in
    /// the account card, or all of them one after another.
    pub accounts_shown: AccountsShown,
    /// With several accounts: an "All Accounts" section heads the folder
    /// pane, with each special folder (Inbox, Sent, ...) of every account
    /// in one list, and the accounts below it start folded.
    pub unified_inbox: bool,
    /// The account on show with [`AccountsShown::One`], by lower-case
    /// address; empty for the first.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub current_account: String,
    /// The order accounts are listed in (Settings > Accounts), by
    /// lower-case address; accounts not in it follow, oldest first. The
    /// first account is the default where there is one.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub account_order: Vec<String>,
    /// Translating mail into the reading language with Katna Server.
    pub translation: TranslationSettings,
    /// The newest open or click seen in Activity (the server's event
    /// number), so the Activity button can count the ones after it.
    #[serde(skip_serializing_if = "is_zero")]
    pub activity_seen: i64,
    /// Activity's list leaves out opens and clicks up to this event
    /// (Clear all) ...
    #[serde(skip_serializing_if = "is_zero")]
    pub activity_cleared: i64,
    /// ... and these ones, removed one by one after it.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub activity_removed: Vec<i64>,
}

fn is_zero(n: &i64) -> bool {
    *n == 0
}

/// Automatic translation (Settings > General > Translation;
/// `docs/ARCHITECTURE.md` §16.4). Languages are LibreTranslate codes
/// (`es`, `zh`, `zt`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationSettings {
    /// Offer to translate mail in other languages. Nothing is sent until
    /// the user asks, or chose to always translate a language.
    pub offer: bool,
    /// The language mail is translated into; empty for the interface's.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reading_language: String,
    /// Languages translated as soon as a message opens.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub always: Vec<String>,
    /// Languages never offered for translation.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub never: Vec<String>,
}

impl Default for TranslationSettings {
    fn default() -> Self {
        Self {
            offer: true,
            reading_language: String::new(),
            always: Vec::new(),
            never: Vec::new(),
        }
    }
}

impl Default for MailView {
    fn default() -> Self {
        Self {
            reading_pane: ReadingPane::Right,
            reading_pane_share: 0.5,
            conversations: true,
            inbox_tabs: true,
            account_tabs: BTreeMap::new(),
            density: Density::Default,
            scale: 100,
            theme: Theme::System,
            desktop_colors: true,
            app_labels: true,
            sender_pictures: true,
            newest_first: false,
            contact_panel: true,
            full_headers: false,
            full_names: false,
            open: OpenAttachments::default(),
            mark_read: MarkRead::Instantly,
            auto_advance: AutoAdvance::Next,
            confirm_delete: true,
            remote_images: false,
            reply_all: false,
            important_markers: true,
            limit_width: false,
            dark_mail: true,
            attachment_previews: true,
            open_saved_folder: false,
            accounts_shown: AccountsShown::One,
            unified_inbox: false,
            current_account: String::new(),
            account_order: Vec::new(),
            translation: TranslationSettings::default(),
            activity_seen: 0,
            activity_cleared: 0,
            activity_removed: Vec::new(),
        }
    }
}

impl MailView {
    /// The tab settings of the account with `address`.
    pub fn tabs_of(&self, address: &str) -> AccountTabs {
        self.account_tabs
            .get(&address.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    /// Sorts `accounts` into [`Self::account_order`]; the rest keep their
    /// order after them.
    pub fn order_accounts(&self, accounts: &mut [crate::Account]) {
        let place = |account: &crate::Account| {
            let address = account.address.trim().to_lowercase();
            self.account_order
                .iter()
                .position(|a| *a == address)
                .unwrap_or(usize::MAX)
        };
        accounts.sort_by_key(place);
    }

    /// Moves the account at `from` in `accounts` (as ordered by
    /// [`Self::order_accounts`]) to `to`, and keeps the whole order.
    pub fn move_account(&mut self, accounts: &[crate::Account], from: usize, to: usize) {
        let mut order: Vec<String> = accounts
            .iter()
            .map(|a| a.address.trim().to_lowercase())
            .collect();
        if from >= order.len() {
            return;
        }
        let moved = order.remove(from);
        order.insert(to.min(order.len()), moved);
        self.account_order = order;
    }
}

/// What opens after the open conversation leaves the list
/// ([`MailView::auto_advance`]), as Gmail's Auto-advance.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AutoAdvance {
    /// The line below it (the one above when it was the last).
    #[default]
    Next,
    /// The line above it (the one below when it was the first).
    Previous,
    /// No conversation: back to the list.
    List,
}

impl AutoAdvance {
    pub const ALL: [Self; 3] = [Self::Next, Self::Previous, Self::List];

    /// The line to open in a list of `len` lines, where `at` is the line
    /// that took the removed one's place (`len` when it was the last).
    pub fn pick(self, at: usize, len: usize) -> Option<usize> {
        let below = (at < len).then_some(at);
        let above = at.min(len).checked_sub(1);
        match self {
            Self::Next => below.or(above),
            Self::Previous => above.or(below),
            Self::List => None,
        }
    }
}

/// When an opened conversation is marked read ([`MailView::mark_read`]).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MarkRead {
    #[default]
    Instantly,
    /// After it has been open for a second.
    AfterOneSecond,
    /// After it has been open for three seconds.
    AfterThreeSeconds,
    /// Only with Mark as read.
    Manually,
}

impl MarkRead {
    pub const ALL: [Self; 4] = [
        Self::Instantly,
        Self::AfterOneSecond,
        Self::AfterThreeSeconds,
        Self::Manually,
    ];

    /// How long a conversation stays open before it is marked read;
    /// `None` when it never is.
    pub fn delay(self) -> Option<std::time::Duration> {
        match self {
            Self::Instantly => Some(std::time::Duration::ZERO),
            Self::AfterOneSecond => Some(std::time::Duration::from_secs(1)),
            Self::AfterThreeSeconds => Some(std::time::Duration::from_secs(3)),
            Self::Manually => None,
        }
    }
}

/// The inbox tabs of one account ([`MailView::account_tabs`]).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountTabs {
    pub style: TabStyle,
    /// Tabs turned off, by name (`promotions`, `newsletters`, ...). Their
    /// mail shows in the first tab.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hidden: Vec<String>,
}

/// Which set of inbox tabs an account uses.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TabStyle {
    /// The provider's own tabs: Gmail's categories for Gmail, Focused and
    /// Other for Outlook, Newsletters and Notifications for Zoho, and
    /// Gmail's categories by header rules for others.
    #[default]
    Auto,
    /// Primary, Promotions, Social, Updates and Forums.
    Gmail,
    /// Focused and Other.
    Focused,
    /// Inbox, Newsletters and Notifications.
    Zoho,
    /// No tabs.
    Off,
}

/// Where each kind of attachment opens when clicked ([`MailView::open`]):
/// Katna Mail's own viewer, the desktop's default app for the file type,
/// or a choice of apps each time. Files without a preview always open in
/// the viewer, which offers to save them or open them elsewhere.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenAttachments {
    pub pdf: OpenIn,
    pub pictures: OpenIn,
    pub text: OpenIn,
    pub spreadsheets: OpenIn,
    pub documents: OpenIn,
}

/// A kind of attachment with a setting in [`OpenAttachments`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileGroup {
    Pdf,
    Pictures,
    Text,
    Spreadsheets,
    Documents,
}

impl FileGroup {
    pub const ALL: [Self; 5] = [
        Self::Pdf,
        Self::Pictures,
        Self::Text,
        Self::Spreadsheets,
        Self::Documents,
    ];
}

impl OpenAttachments {
    pub fn get(&self, group: FileGroup) -> OpenIn {
        match group {
            FileGroup::Pdf => self.pdf,
            FileGroup::Pictures => self.pictures,
            FileGroup::Text => self.text,
            FileGroup::Spreadsheets => self.spreadsheets,
            FileGroup::Documents => self.documents,
        }
    }

    pub fn set(&mut self, group: FileGroup, open: OpenIn) {
        let slot = match group {
            FileGroup::Pdf => &mut self.pdf,
            FileGroup::Pictures => &mut self.pictures,
            FileGroup::Text => &mut self.text,
            FileGroup::Spreadsheets => &mut self.spreadsheets,
            FileGroup::Documents => &mut self.documents,
        };
        *slot = open;
    }
}

/// [`OpenAttachments`]: where one kind of attachment opens.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenIn {
    /// Katna Mail's viewer.
    #[default]
    Katna,
    /// The desktop's default app for the file type.
    System,
    /// The desktop's "Open with" choice of apps, every time.
    Ask,
}

/// Keyboard shortcuts of Katna Mail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Shortcuts {
    /// Shortcuts without Ctrl or Alt, such as `e` to archive, as in webmail.
    pub single_keys: bool,
    /// Whose keys the shortcuts start from: Katna's own or another mail
    /// app's. [`Shortcuts::keys`] changes them further.
    pub set: ShortcutSet,
    /// Keys changed from the set's, by shortcut name (`archive`,
    /// `reply`, ...): each a list of keystrokes such as `ctrl-shift-a` or
    /// `g i`. An empty list turns the shortcut off.
    pub keys: BTreeMap<String, Vec<String>>,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            single_keys: true,
            set: ShortcutSet::Katna,
            keys: BTreeMap::new(),
        }
    }
}

/// [`Shortcuts::set`]: the keys of a familiar mail app.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShortcutSet {
    /// Gmail's keys, with the usual desktop keys as well.
    #[default]
    Katna,
    Gmail,
    InboxByGmail,
    /// Apple Mail's, with Ctrl for Cmd.
    AppleMail,
    Outlook,
    Thunderbird,
}

/// [`MailView::reading_pane`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadingPane {
    /// Beside the list: three panes with the navigation.
    Right,
    /// In place of the list: two panes.
    None,
}

/// [`MailView::accounts_shown`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountsShown {
    /// One account at a time, as in webmail.
    #[default]
    One,
    /// Every account, one after another.
    All,
}

/// [`MailView::density`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Density {
    Default,
    Compact,
}

/// [`MailView::theme`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Follow the desktop.
    System,
    Light,
    Dark,
}

impl Config {
    /// Reads and validates the configuration at `path`. A missing file gives
    /// the defaults.
    pub fn load(path: &Path) -> Result<Self> {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(err) => return Err(Error::io(path, err)),
        };
        Self::parse(&text).map_err(|err| match err {
            ParseError::Toml(source) => Error::ConfigParse {
                path: path.to_owned(),
                source,
            },
            ParseError::Invalid(err) => err,
        })
    }

    /// Parses and validates TOML text.
    fn parse(text: &str) -> Result<Self, ParseError> {
        let mut config: Self = toml::from_str(text).map_err(ParseError::Toml)?;
        config.sending.upgrade();
        config.validate().map_err(ParseError::Invalid)?;
        Ok(config)
    }

    /// Checks that every value is in range.
    pub fn validate(&self) -> Result<()> {
        if !UNDO_SEND_CHOICES.contains(&self.sending.undo_send_seconds) {
            return Err(Error::ConfigValue {
                key: "sending.undo_send_seconds",
                message: format!(
                    "{} is not one of {UNDO_SEND_CHOICES:?}",
                    self.sending.undo_send_seconds
                ),
            });
        }
        if !(0.25..=0.75).contains(&self.mail.reading_pane_share) {
            return Err(Error::ConfigValue {
                key: "mail.reading_pane_share",
                message: format!(
                    "{} is not between 0.25 and 0.75",
                    self.mail.reading_pane_share
                ),
            });
        }
        let mut ids = std::collections::HashSet::new();
        if let Some(dup) = self.sending.signatures.iter().find(|s| !ids.insert(s.id)) {
            return Err(Error::ConfigValue {
                key: "sending.signatures",
                message: format!("signature id {} is used twice", dup.id),
            });
        }
        if self.logging.filter.trim().is_empty() {
            return Err(Error::ConfigValue {
                key: "logging.filter",
                message: "must not be empty".to_owned(),
            });
        }
        Ok(())
    }

    /// Validates and writes the configuration to `path`, creating the parent
    /// directory if needed.
    ///
    /// The file is written to a temporary file next to `path` and renamed
    /// over it, so readers never see a half-written file.
    pub fn save(&self, path: &Path) -> Result<()> {
        self.validate()?;
        let text = toml::to_string_pretty(self)?;
        let dir = path
            .parent()
            .filter(|dir| !dir.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        fs::create_dir_all(dir).map_err(|err| Error::io(dir, err))?;
        let mut tmp = tempfile_in(dir)?;
        tmp.1
            .write_all(text.as_bytes())
            .and_then(|()| tmp.1.sync_all())
            .map_err(|err| Error::io(&tmp.0, err))?;
        fs::rename(&tmp.0, path).map_err(|err| {
            let _ = fs::remove_file(&tmp.0);
            Error::io(path, err)
        })
    }
}

#[derive(Debug)]
enum ParseError {
    Toml(toml::de::Error),
    Invalid(Error),
}

/// Creates a new, uniquely named file in `dir`.
fn tempfile_in(dir: &Path) -> Result<(std::path::PathBuf, fs::File)> {
    let pid = std::process::id();
    for attempt in 0u32.. {
        let path = dir.join(format!(".config.toml.{pid}.{attempt}.tmp"));
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => return Ok((path, file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(Error::io(path, err)),
        }
    }
    unreachable!("u32 range exhausted while creating a temporary file")
}

#[cfg(test)]
mod tests {
    #[test]
    fn accounts_follow_the_chosen_order() {
        use crate::{Account, AccountId, AccountKind};
        let account = |id, address: &str| Account {
            id: AccountId(id),
            kind: AccountKind::Imap,
            display_name: String::new(),
            address: address.to_owned(),
        };
        let mut accounts = vec![
            account(1, "a@x.org"),
            account(2, "B@x.org"),
            account(3, "c@x.org"),
        ];
        let mut view = MailView::default();
        view.order_accounts(&mut accounts);
        assert_eq!(accounts[0].id, AccountId(1), "no order keeps the store's");
        view.move_account(&accounts, 2, 0);
        assert_eq!(view.account_order, ["c@x.org", "a@x.org", "b@x.org"]);
        view.order_accounts(&mut accounts);
        let ids: Vec<i64> = accounts.iter().map(|a| a.id.0).collect();
        assert_eq!(ids, [3, 1, 2]);
        // A new account goes last.
        accounts.push(account(4, "d@x.org"));
        accounts.swap(0, 3);
        view.order_accounts(&mut accounts);
        let ids: Vec<i64> = accounts.iter().map(|a| a.id.0).collect();
        assert_eq!(ids, [3, 1, 2, 4]);
    }

    use super::*;

    #[test]
    fn auto_advance_picks_a_neighbor() {
        // Line 2 of 5 removed: 4 left, and line 2 is the one that was below.
        assert_eq!(AutoAdvance::Next.pick(2, 4), Some(2));
        assert_eq!(AutoAdvance::Previous.pick(2, 4), Some(1));
        assert_eq!(AutoAdvance::List.pick(2, 4), None);
        // The last line removed: the one above opens either way.
        assert_eq!(AutoAdvance::Next.pick(4, 4), Some(3));
        // The first line removed: Previous takes the one below.
        assert_eq!(AutoAdvance::Previous.pick(0, 4), Some(0));
        // Nothing left.
        assert_eq!(AutoAdvance::Next.pick(0, 0), None);
        assert_eq!(AutoAdvance::Previous.pick(0, 0), None);
    }

    #[test]
    fn missing_file_gives_defaults() {
        let tmp = tempfile::tempdir().unwrap();
        let config = Config::load(&tmp.path().join("config.toml")).unwrap();
        assert_eq!(config, Config::default());
        assert!(config.general.run_in_background);
        assert_eq!(config.sending.undo_send_seconds, 10);
    }

    #[test]
    fn search_triggers_are_words() {
        assert_eq!(Config::default().general.search_triggers, ["k", "m"]);
        assert_eq!(
            General::parse_search_triggers(" K, mail:  k find,,"),
            ["k", "mail", "find"]
        );
        assert!(General::parse_search_triggers(" , ").is_empty());
        let config = Config::parse("[general]\nsearch_triggers = []\n").unwrap();
        assert!(config.general.search_triggers.is_empty());
    }

    #[test]
    fn partial_file_keeps_other_defaults() {
        let config = Config::parse("[sending]\nundo_send_seconds = 30\n").unwrap();
        assert_eq!(config.sending.undo_send_seconds, 30);
        assert_eq!(config.general, General::default());
        assert_eq!(config.logging, Logging::default());
    }

    #[test]
    fn mail_view() {
        let config = Config::default();
        assert_eq!(config.mail.reading_pane, ReadingPane::Right);
        assert!(config.mail.conversations && config.mail.inbox_tabs);
        let config = Config::parse(
            "[mail]\nreading_pane = \"none\"\ndensity = \"compact\"\ntheme = \"dark\"\n",
        )
        .unwrap();
        assert_eq!(config.mail.reading_pane, ReadingPane::None);
        assert_eq!(config.mail.density, Density::Compact);
        assert_eq!(config.mail.theme, Theme::Dark);
        assert!(config.mail.desktop_colors);
        assert!(Config::parse("[mail]\nreading_pane_share = 0.9\n").is_err());
    }

    #[test]
    fn experimental_look() {
        let config = Config::default();
        assert_eq!(config.experimental.window_frame, WindowFrame::Native);
        assert!(!config.experimental.blur);
        let config =
            Config::parse("[experimental]\nwindow_frame = \"katna\"\nblur = true\n").unwrap();
        assert_eq!(config.experimental.window_frame, WindowFrame::Katna);
        assert!(config.experimental.blur);
    }

    #[test]
    fn where_attachments_open() {
        let config = Config::default();
        for group in FileGroup::ALL {
            assert_eq!(config.mail.open.get(group), OpenIn::Katna);
        }
        let mut config =
            Config::parse("[mail.open]\npdf = \"system\"\nspreadsheets = \"ask\"\n").unwrap();
        assert_eq!(config.mail.open.get(FileGroup::Pdf), OpenIn::System);
        assert_eq!(config.mail.open.get(FileGroup::Spreadsheets), OpenIn::Ask);
        assert_eq!(config.mail.open.get(FileGroup::Pictures), OpenIn::Katna);
        config.mail.open.set(FileGroup::Documents, OpenIn::System);
        let text = toml::to_string(&config).unwrap();
        assert_eq!(Config::parse(&text).unwrap(), config);
    }

    #[test]
    fn the_old_signature_becomes_the_default() {
        let config = Config::parse("[sending]\nsignature = \"Kay\\nEnron\"\n").unwrap();
        let sending = &config.sending;
        assert_eq!(sending.signature, "");
        assert_eq!(sending.signatures.len(), 1);
        assert_eq!(sending.signatures[0].text, "Kay\nEnron");
        assert_eq!(sending.new_mail_signature, Some(sending.signatures[0].id));
        assert_eq!(sending.reply_signature, sending.new_mail_signature);
        // Written back without the old key.
        let text = toml::to_string_pretty(&config).unwrap();
        assert!(
            !text.lines().any(|l| l.starts_with("signature =")),
            "{text}"
        );
        assert_eq!(Config::parse(&text).unwrap(), config);
    }

    #[test]
    fn signatures() {
        let mut sending = Sending::default();
        let work = sending.add_signature("Work".into(), "Kay, Enron".into());
        let home = sending.add_signature("Home".into(), "Kay".into());
        assert_ne!(work, home);
        assert_eq!(sending.new_mail_signature, Some(work));
        sending.reply_signature = Some(home);
        sending.remove_signature(home);
        assert_eq!(sending.reply_signature, None);
        assert_eq!(sending.signature(Some(work)).unwrap().name, "Work");
        let again = sending.add_signature("Again".into(), String::new());
        assert!(again > work);
        let mut config = Config {
            sending,
            ..Config::default()
        };
        config.sending.signatures[1].id = work;
        assert!(config.validate().is_err(), "duplicate IDs");
    }

    #[test]
    fn tabs_and_shortcuts() {
        let config = Config::parse(
            "[mail.account_tabs.\"kay@zoho.example\"]\nstyle = \"zoho\"\nhidden = [\"notifications\"]\n\
             [shortcuts]\nsingle_keys = false\n[shortcuts.keys]\narchive = [\"y\", \"ctrl-e\"]\n",
        )
        .unwrap();
        let tabs = config.mail.tabs_of("Kay@Zoho.example");
        assert_eq!(tabs.style, TabStyle::Zoho);
        assert_eq!(tabs.hidden, ["notifications"]);
        assert_eq!(
            config.mail.tabs_of("other@example.org"),
            AccountTabs::default()
        );
        assert!(!config.shortcuts.single_keys);
        assert_eq!(config.shortcuts.keys["archive"], ["y", "ctrl-e"]);
        let text = toml::to_string_pretty(&config).unwrap();
        assert_eq!(Config::parse(&text).unwrap(), config);
    }

    #[test]
    fn one_account_is_shown_by_default() {
        let config = Config::default();
        assert_eq!(config.mail.accounts_shown, AccountsShown::One);
        assert!(config.mail.current_account.is_empty());
        let config = Config::parse(
            "[mail]\naccounts_shown = \"all\"\ncurrent_account = \"kay@example.org\"\n",
        )
        .unwrap();
        assert_eq!(config.mail.accounts_shown, AccountsShown::All);
        assert_eq!(config.mail.current_account, "kay@example.org");
        let text = toml::to_string_pretty(&config).unwrap();
        assert_eq!(Config::parse(&text).unwrap(), config);
    }

    #[test]
    fn onboarding_is_shown_until_done() {
        assert!(!Config::default().onboarding.done);
        let config = Config::parse("[onboarding]\ndone = true\n").unwrap();
        assert!(config.onboarding.done);
        assert_eq!(config.onboarding.whats_new_seen, None);
    }

    #[test]
    fn whats_new_state_round_trips() {
        let mut config = Config::default();
        config.onboarding.whats_new_seen = Some(3);
        config.onboarding.whats_new_shown = vec!["2026-09-27-0444-about-katna".to_owned()];
        config.onboarding.last_version = Some("0.0.0.r90.gabc1234".to_owned());
        let text = toml::to_string_pretty(&config).unwrap();
        assert_eq!(Config::parse(&text).unwrap(), config);
        // Unset values stay out of the file.
        let text = toml::to_string_pretty(&Config::default()).unwrap();
        assert!(!text.contains("whats_new_seen"));
        assert!(!text.contains("whats_new_shown"));
    }

    #[test]
    fn new_mail_notifications_are_on_by_default() {
        assert!(Config::default().notifications.new_mail);
        let config = Config::parse("[notifications]\nnew_mail = false\n").unwrap();
        assert!(!config.notifications.new_mail);
    }

    #[test]
    fn offline_days_setting() {
        assert_eq!(Config::default().sync.offline_window(), Some(30));
        let config = Config::parse("[sync]\nmetered = \"never\"\n").unwrap();
        assert_eq!(config.sync.offline_window(), Some(30));
        let config = Config::parse("[sync]\noffline_days = 0\n").unwrap();
        assert_eq!(config.sync.offline_window(), None);
        let config = Config::parse("[sync]\noffline_days = 365\n").unwrap();
        assert_eq!(config.sync.offline_window(), Some(365));
    }

    #[test]
    fn metered_setting() {
        assert_eq!(Config::default().sync.metered, Metered::Auto);
        let config = Config::parse("[sync]\nmetered = \"always\"\n").unwrap();
        assert_eq!(config.sync.metered, Metered::Always);
        assert!(Config::parse("[sync]\nmetered = \"sometimes\"\n").is_err());
        assert!(Metered::Auto.decide(true) && !Metered::Auto.decide(false));
        assert!(Metered::Always.decide(false));
        assert!(!Metered::Never.decide(true));
    }

    #[test]
    fn unknown_keys_are_ignored() {
        let config = Config::parse("future = 1\n[general]\nnew_option = \"x\"\n").unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn rejects_bad_toml_with_path() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("config.toml");
        fs::write(&path, "[general\n").unwrap();
        let err = Config::load(&path).unwrap_err();
        assert!(matches!(err, Error::ConfigParse { .. }), "{err:?}");
        assert!(err.to_string().contains("config.toml"), "{err}");
    }

    #[test]
    fn rejects_wrong_types() {
        assert!(matches!(
            Config::parse("[general]\nrun_in_background = \"yes\"\n"),
            Err(ParseError::Toml(_))
        ));
    }

    #[test]
    fn rejects_out_of_range_values() {
        let Err(ParseError::Invalid(err)) = Config::parse("[sending]\nundo_send_seconds = 7\n")
        else {
            panic!("undo_send_seconds = 7 was accepted");
        };
        assert!(matches!(
            err,
            Error::ConfigValue {
                key: "sending.undo_send_seconds",
                ..
            }
        ));
        assert!(matches!(
            Config::parse("[logging]\nfilter = \" \"\n"),
            Err(ParseError::Invalid(_))
        ));
    }

    #[test]
    fn ignores_the_tray_setting_older_versions_saved() {
        let config = Config::parse("[general]\ntray_icon = false\n").unwrap();
        assert!(config.general.show_in_tray);
        let config = Config::parse("[general]\nshow_in_tray = false\n").unwrap();
        assert!(!config.general.show_in_tray);
    }

    #[test]
    fn save_and_load_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("nested/config.toml");
        let mut config = Config::default();
        config.general.show_in_tray = false;
        config.logging.filter = "debug".to_owned();
        config.sending.undo_send_seconds = 0;
        config.save(&path).unwrap();
        assert_eq!(Config::load(&path).unwrap(), config);
        // No temporary files are left behind.
        let entries: Vec<_> = fs::read_dir(path.parent().unwrap()).unwrap().collect();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn save_refuses_invalid_config() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("config.toml");
        let mut config = Config::default();
        config.sending.undo_send_seconds = 3;
        assert!(config.save(&path).is_err());
        assert!(!path.exists());
    }
}
