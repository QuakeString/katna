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
    /// The newest What's new highlight shown, or offered by the first
    /// start. `None` in files written before What's new existed.
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
}

impl Default for Notifications {
    fn default() -> Self {
        Self { new_mail: true }
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
}

impl Default for General {
    fn default() -> Self {
        Self {
            run_in_background: true,
            show_in_tray: true,
            unread_badge: true,
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
    pub theme: Theme,
    /// Use the desktop's color scheme and accent color instead of Katna's
    /// own colors.
    pub desktop_colors: bool,
    /// Show the names under the icons of the app bar (Mail, Calendar, ...).
    pub app_labels: bool,
    /// Show the logo of each sender's organization (its BIMI logo or
    /// website icon) in place of their initial.
    pub sender_pictures: bool,
    /// Where each kind of attachment opens.
    pub open: OpenAttachments,
    /// With several accounts: the folder pane shows one account, picked in
    /// the account card, or all of them one after another.
    pub accounts_shown: AccountsShown,
    /// The account on show with [`AccountsShown::One`], by lower-case
    /// address; empty for the first.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub current_account: String,
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
            theme: Theme::System,
            desktop_colors: true,
            app_labels: true,
            sender_pictures: true,
            open: OpenAttachments::default(),
            accounts_shown: AccountsShown::One,
            current_account: String::new(),
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
    /// Keys changed from the defaults, by shortcut name (`archive`,
    /// `reply`, ...): each a list of keystrokes such as `ctrl-shift-a` or
    /// `g i`. An empty list turns the shortcut off.
    pub keys: BTreeMap<String, Vec<String>>,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            single_keys: true,
            keys: BTreeMap::new(),
        }
    }
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
    use super::*;

    #[test]
    fn missing_file_gives_defaults() {
        let tmp = tempfile::tempdir().unwrap();
        let config = Config::load(&tmp.path().join("config.toml")).unwrap();
        assert_eq!(config, Config::default());
        assert!(config.general.run_in_background);
        assert_eq!(config.sending.undo_send_seconds, 10);
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
        config.onboarding.last_version = Some("0.0.0.r90.gabc1234".to_owned());
        let text = toml::to_string_pretty(&config).unwrap();
        assert_eq!(Config::parse(&text).unwrap(), config);
        // Unset values stay out of the file.
        let text = toml::to_string_pretty(&Config::default()).unwrap();
        assert!(!text.contains("whats_new_seen"));
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
