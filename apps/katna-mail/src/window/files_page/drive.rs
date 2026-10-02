// SPDX-License-Identifier: GPL-3.0-or-later

//! The accounts' cloud drives on the Files page (`docs/ARCHITECTURE.md`
//! §13.8): a Drives group in the side column under Accounts, and, once
//! one is clicked, that drive in place of the mail files: a folder path
//! to click back through, folder tiles, then the files as the page's
//! cards. The top bar's search box searches the drive on its server.
//!
//! The daemon asks the drive (`CloudList`, `CloudFetch`,
//! `CloudThumbnail`); nothing is synced. Listings are kept for a few
//! minutes, so going back through folders is instant.

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, ClipboardItem, Context, FontWeight, ListAlignment, ListState, MouseButton,
    MouseDownEvent, Pixels, Point, SharedString, Task, Window, div, list, prelude::*, rgba,
};
use katna_core::{AccountId, OAuthProvider};
use katna_dbus::{CloudEntry, cloud_place, cloud_state};
use katna_i18n::tr;
use katna_preview::image::{RgbaImage, imageops};
use katna_preview::{Kind, Picture};
use katna_ui::{ScrollBar, px};

use super::super::MailWindow;
use super::super::attachments::{
    CARD_RADIUS, Item, THUMB_PIXELS, Thumb, card_top, hover_panel, kind_badge, panel_button,
    picture_thumb,
};
use super::super::compose::attach::MAX_TOTAL;
use super::picker::{pick_eye, pick_ring, pick_tick};
use super::{
    GAP, HEADING_HEIGHT, Menu, NAME_HEIGHT, ROW_HEIGHT, Sort, THUMBS_KEPT, Time, Types, chip_arrow,
    filter_chip, thumb_height,
};
use crate::format;
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, icon_button, outlined_button, placeholder, tip};
use katna_core::config::OpenIn;

mod manage;
mod tray;

/// How long a folder's listing is used again without asking the drive.
const FRESH: Duration = Duration::from_secs(3 * 60);

/// How long typing rests before the drive is searched.
const SEARCH_WAIT: Duration = Duration::from_millis(400);

/// Thumbnails asked for at once.
const THUMB_BATCH: usize = 8;

/// A drive file bigger than this opens in the desktop's app rather than
/// Katna's viewer, which reads it whole.
const VIEWER_MAX: u64 = 64 * 1024 * 1024;

/// The width of a folder tile in a phone's sideways row.
const STRIP_TILE: f32 = 160.0;

/// The height of a folder tile.
const FOLDER_HEIGHT: f32 = 44.0;

/// The height of a drive card's foot: its name and one line under it.
const DRIVE_FOOT: f32 = NAME_HEIGHT + 30.0;

/// The drives part of the Files page.
#[derive(Default)]
pub(in crate::window) struct Cloud {
    /// The drive the page shows, or `None` for the mail files.
    pub(in crate::window) view: Option<DriveView>,
    /// The accounts whose drive Files offers: Google accounts not turned
    /// off in Settings, with their addresses. Read as the page opens.
    pub(in crate::window) drives: Vec<(AccountId, String)>,
    /// Listings read lately, by account and place.
    cache: HashMap<(AccountId, String), Cached>,
    thumbs: HashMap<String, (Thumb, u64)>,
    asked: HashSet<String>,
    /// Thumbnails to fetch: the account, the link, and whether it is of
    /// a picture (which fills the card) rather than a page (whose top
    /// shows).
    wanted: Vec<(AccountId, String, bool)>,
    _thumbs: Option<Task<()>>,
    /// Files being fetched, by id: their cards say so.
    fetching: HashSet<String>,
    /// Uploads under way, by the daemon's id.
    /// Uploads since the tray was last closed, oldest first, by the
    /// daemon's id.
    uploads: Vec<(i64, Upload)>,
    _uploads: Option<Task<()>>,
    /// The uploads tray shows only its head.
    tray_folded: bool,
    /// The item whose name is being typed over.
    renaming: Option<manage::Renaming>,
}

/// A file or folder going up into a drive, shown in the uploads tray.
struct Upload {
    name: String,
    /// The listing it lands in, read again once it is there.
    key: (AccountId, String),
    /// The folders from the top to where it goes, id and name, to show it
    /// there.
    crumbs: Vec<(String, String)>,
    /// Bytes the drive has, and the size.
    sent: u64,
    size: u64,
    state: Going,
    started: Instant,
}

/// Where an upload stands.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Going {
    Uploading,
    Done,
    /// Why it failed.
    Failed(String),
    /// The sign-in does not allow uploads yet.
    NeedsPermission,
    Cancelled,
}

struct Cached {
    entries: Rc<Vec<CloudEntry>>,
    next: String,
    at: Instant,
}

/// What the drive said.
enum Listing {
    Loading,
    Ready,
    NeedsPermission,
    Failed(String),
}

/// A line of the drive's list.
#[derive(Debug, Clone)]
enum DriveLine {
    /// "Folders" or "Files".
    Heading(bool),
    /// A row of folder tiles or file cards: places in `order`.
    Tiles(Range<usize>),
    Cards(Range<usize>),
    /// A folder or file in the list view: its place in `order`.
    Row(usize),
}

/// One drive open on the page.
pub(in crate::window) struct DriveView {
    pub(in crate::window) account: AccountId,
    /// Its "Shared with me" rather than the drive itself.
    shared: bool,
    /// The folders opened from the top: id and name.
    crumbs: Vec<(String, String)>,
    /// The words searched for; the search box's text while it waits.
    pub(in crate::window) search: String,
    searched: String,
    listing: Listing,
    entries: Rc<Vec<CloudEntry>>,
    next: String,
    /// The folders, then the files that pass the filters: places in
    /// `entries`.
    order: Rc<Vec<usize>>,
    folders: usize,
    lines: Rc<Vec<DriveLine>>,
    types: Types,
    stale: bool,
    columns: usize,
    /// The folders are one row that scrolls sideways, on a phone.
    strip: bool,
    state: ListState,
    bar: ScrollBar,
    /// The item the arrow keys are on: its place in `order`.
    cursor: Option<usize>,
    _load: Option<Task<()>>,
    _more: Option<Task<()>>,
    _wait: Option<Task<()>>,
}

impl DriveView {
    fn new(account: AccountId, shared: bool) -> Self {
        Self {
            account,
            shared,
            crumbs: Vec::new(),
            search: String::new(),
            searched: String::new(),
            listing: Listing::Loading,
            entries: Rc::default(),
            next: String::new(),
            order: Rc::default(),
            folders: 0,
            lines: Rc::default(),
            types: Types::All,
            stale: true,
            columns: 0,
            strip: false,
            state: ListState::new(0, ListAlignment::Top, px(600.0)),
            bar: ScrollBar::default(),
            cursor: None,
            _load: None,
            _more: None,
            _wait: None,
        }
    }

    /// The filters or the layout changed: the lines are made again.
    pub(super) fn stale_now(&mut self) {
        self.stale = true;
    }

    /// What `CloudList` is asked for: a place and its folder or words.
    fn place(&self) -> (&'static str, String) {
        if !self.searched.is_empty() {
            (cloud_place::SEARCH, self.searched.clone())
        } else if let Some((id, _)) = self.crumbs.last() {
            (cloud_place::FOLDER, id.clone())
        } else if self.shared {
            (cloud_place::SHARED, String::new())
        } else {
            (cloud_place::FOLDER, String::new())
        }
    }

    fn key(&self) -> (AccountId, String) {
        let (place, what) = self.place();
        (self.account, format!("{place}:{what}"))
    }

    /// Makes `order` and the lines again for `columns` across; on a
    /// phone (`strip`) the folders go in one row that scrolls sideways.
    #[allow(clippy::too_many_arguments)]
    fn rebuild(
        &mut self,
        columns: usize,
        grid: bool,
        strip: bool,
        time: Time,
        sort: Sort,
        line: f32,
        tz: &jiff::tz::TimeZone,
    ) {
        self.stale = false;
        self.columns = columns;
        self.strip = strip;
        let entries = self.entries.clone();
        let mut folders: Vec<usize> = Vec::new();
        let mut files: Vec<usize> = Vec::new();
        for (ix, entry) in entries.iter().enumerate() {
            if entry.folder {
                folders.push(ix);
                continue;
            }
            let day = (entry.modified != 0)
                .then(|| format::local(entry.modified, tz).map(|d| d.date()))
                .flatten();
            if (self.types == Types::All || self.types == Types::of(entry_kind(entry)))
                && time.keeps(day)
            {
                files.push(ix);
            }
        }
        let name = |ix: &usize| entries[*ix].name.to_lowercase();
        match sort {
            Sort::Newest => {}
            Sort::Oldest => files.sort_by_key(|&ix| entries[ix].modified),
            Sort::Largest => files.sort_by_key(|&ix| std::cmp::Reverse(entries[ix].size)),
            Sort::Name => {
                folders.sort_by_cached_key(name);
                files.sort_by_cached_key(name);
            }
        }
        self.folders = folders.len();
        let mut order = folders;
        order.extend(files);
        let mut lines = Vec::new();
        let columns = columns.max(1);
        let add = |range: Range<usize>, folders: bool, lines: &mut Vec<DriveLine>| {
            if range.is_empty() {
                return;
            }
            if grid {
                lines.push(DriveLine::Heading(folders));
                if folders && strip {
                    lines.push(DriveLine::Tiles(range));
                    return;
                }
                let mut at = range.start;
                while at < range.end {
                    let to = (at + columns).min(range.end);
                    lines.push(if folders {
                        DriveLine::Tiles(at..to)
                    } else {
                        DriveLine::Cards(at..to)
                    });
                    at = to;
                }
            } else {
                lines.extend(range.map(DriveLine::Row));
            }
        };
        add(0..self.folders, true, &mut lines);
        add(self.folders..order.len(), false, &mut lines);
        self.order = Rc::new(order);
        self.lines = Rc::new(lines);
        let line = if grid { line } else { ROW_HEIGHT };
        self.state
            .reset_with_uniform_height(self.lines.len(), px(line));
    }
}

/// What a drive item is, for its badge and the side column's kinds.
fn entry_kind(entry: &CloudEntry) -> Kind {
    if entry.native {
        // Google's documents open as PDFs; their badge says what they are.
        return match entry.mime.rsplit('.').next() {
            Some("spreadsheet") => Kind::Sheet { csv: false },
            Some("presentation") => Kind::Slides,
            Some("drawing") => Kind::Picture(Picture::Png),
            _ => Kind::Document,
        };
    }
    katna_preview::kind(&entry.mime, &entry.name)
}

/// The format of a thumbnail, by its first bytes.
fn picture_format(bytes: &[u8]) -> Option<Picture> {
    if bytes.starts_with(b"\x89PNG") {
        Some(Picture::Png)
    } else if bytes.starts_with(&[0xff, 0xd8]) {
        Some(Picture::Jpeg)
    } else if bytes.starts_with(b"GIF8") {
        Some(Picture::Gif)
    } else if bytes.len() > 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        Some(Picture::Webp)
    } else {
        None
    }
}

/// A drive's thumbnail made the card's top, `w` by `h`: a `photo` fills
/// it, centred; a page shows its top, as the mail files' cards do.
fn card_picture(bytes: &[u8], photo: bool, w: u32, h: u32) -> Option<RgbaImage> {
    let format = picture_format(bytes)?;
    if photo {
        return katna_preview::picture::thumbnail(bytes, format, w, h).ok();
    }
    let page = katna_preview::image::load_from_memory(bytes)
        .ok()?
        .into_rgba8();
    let (pw, ph) = page.dimensions();
    if pw == 0 || ph == 0 {
        return None;
    }
    let tall = (u64::from(ph) * u64::from(w) / u64::from(pw)).max(1) as u32;
    let scaled = imageops::resize(&page, w, tall, imageops::FilterType::Triangle);
    let mut card = RgbaImage::from_pixel(w, h, katna_preview::image::Rgba([255, 255, 255, 255]));
    imageops::overlay(&mut card, &scaled, 0, 0);
    Some(card)
}

/// What to do with a drive file once it is fetched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Act {
    Open,
    Attach,
    Save,
}

/// The drive's own menus.
pub(super) enum DriveMenu {
    /// An item's menu: its place in `order`.
    Item { place: usize, at: Point<Pixels> },
    /// The type chip's menu.
    Kind(Point<Pixels>),
    /// The Upload button's arrow: files or a folder.
    Upload(Point<Pixels>),
}

impl MailWindow {
    /// Whether the drive of `account` is OneDrive (it signs in with
    /// Microsoft) rather than Google Drive.
    pub(in crate::window) fn drive_is_onedrive(&self, account: AccountId) -> bool {
        self.mail
            .as_ref()
            .is_ok_and(|mail| mail.sign_in_provider(account) == Some(OAuthProvider::Microsoft))
    }

    /// "Google Drive" or "OneDrive", for the drive of `account`.
    pub(in crate::window) fn drive_name(&self, account: AccountId) -> String {
        if self.drive_is_onedrive(account) {
            tr!("files-drive-onedrive")
        } else {
            tr!("files-drive-google")
        }
    }

    /// The mark of the drive of `account`.
    pub(in crate::window) fn drive_mark_of(&self, account: AccountId, size: f32) -> AnyElement {
        drive_mark(self.drive_is_onedrive(account), size)
    }

    /// Reads which accounts have a drive Files can show, as the page opens.
    pub(in crate::window) fn load_drives(&mut self) {
        let off = &self.config.mail.files.drives_off;
        let drives: Vec<(AccountId, String)> = match self.mail.as_ref() {
            Ok(mail) => self
                .accounts
                .iter()
                .filter(|a| !off.contains(&a.id.0))
                .filter(|a| {
                    matches!(
                        mail.sign_in_provider(a.id),
                        Some(OAuthProvider::Google | OAuthProvider::Microsoft)
                    )
                })
                .map(|a| (a.id, a.address.clone()))
                .collect(),
            Err(_) => Vec::new(),
        };
        let cloud = &mut self.library.cloud;
        if cloud
            .view
            .as_ref()
            .is_some_and(|v| !drives.iter().any(|(a, _)| *a == v.account))
        {
            cloud.view = None;
        }
        cloud.drives = drives;
    }

    /// Shows the drive of `account`, or what was shared with it.
    fn open_drive(&mut self, account: AccountId, shared: bool, cx: &mut Context<Self>) {
        let same = self
            .library
            .cloud
            .view
            .as_ref()
            .is_some_and(|v| v.account == account && v.shared == shared && v.crumbs.is_empty());
        self.library.menu = None;
        if same {
            return;
        }
        let mut view = DriveView::new(account, shared);
        // The search box's text stays for the mail files.
        view.types = Types::All;
        self.library.cloud.view = Some(view);
        if !self.library.query.is_empty() {
            self.search.update(cx, |search, cx| search.set_text("", cx));
        }
        self.load_drive(false, cx);
        cx.notify();
    }

    /// Shows the top of the drive of `account` in the attach picker.
    pub(super) fn open_picker_drive(&mut self, account: AccountId, cx: &mut Context<Self>) {
        self.library.menu = None;
        self.library.cloud.view = Some(DriveView::new(account, false));
        self.load_drive(false, cx);
        cx.notify();
    }

    /// Empties the box a drive is searched from: the picker's while it is
    /// open, else the top bar's.
    fn clear_drive_search_box(&mut self, cx: &mut Context<Self>) {
        let search = match &self.picker {
            Some(picker) => picker.search_box(),
            None => self.search.clone(),
        };
        search.update(cx, |search, cx| search.set_text("", cx));
    }

    /// Back to the mail files.
    pub(super) fn close_drive(&mut self, cx: &mut Context<Self>) {
        if self.library.cloud.view.take().is_some() {
            self.search.update(cx, |search, cx| {
                search.set_text(self.library.query.clone(), cx)
            });
        }
    }

    /// Lists the drive's place, from the cache when it is fresh, unless
    /// `again`.
    fn load_drive(&mut self, again: bool, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let cloud = &mut self.library.cloud;
        let Some(view) = cloud.view.as_mut() else {
            return;
        };
        let key = view.key();
        cloud.renaming = None;
        view._more = None;
        view.cursor = None;
        view.stale = true;
        if !again && let Some(cached) = cloud.cache.get(&key).filter(|c| c.at.elapsed() < FRESH) {
            view.entries = cached.entries.clone();
            view.next = cached.next.clone();
            view.listing = Listing::Ready;
            view._load = None;
            return;
        }
        view.listing = Listing::Loading;
        view.entries = Rc::default();
        view.next.clear();
        let account = view.account.0;
        let (place, what) = view.place();
        view._load = Some(cx.spawn(async move |this, cx| {
            let got = crate::daemon::cloud_list(&connection, account, place, &what, "").await;
            this.update(cx, |this, cx| {
                this.drive_listed(key, got, false);
                cx.notify();
            })
            .ok();
        }));
    }

    /// The drive answered for `key`; `more` adds a page to the one shown.
    fn drive_listed(
        &mut self,
        key: (AccountId, String),
        got: Result<katna_dbus::CloudListing, String>,
        more: bool,
    ) {
        let cloud = &mut self.library.cloud;
        let Some(view) = cloud.view.as_mut().filter(|v| v.key() == key) else {
            return;
        };
        view._load = None;
        view._more = None;
        view.stale = true;
        let listing = match got {
            Ok(listing) => listing,
            Err(err) => {
                if !more {
                    view.listing = Listing::Failed(err);
                }
                return;
            }
        };
        match listing.state.as_str() {
            cloud_state::OK => {
                let mut entries = if more {
                    view.entries.as_ref().clone()
                } else {
                    Vec::new()
                };
                entries.extend(listing.items);
                view.entries = Rc::new(entries);
                view.next = listing.next;
                view.listing = Listing::Ready;
                cloud.cache.insert(
                    key,
                    Cached {
                        entries: view.entries.clone(),
                        next: view.next.clone(),
                        at: Instant::now(),
                    },
                );
            }
            cloud_state::NEEDS_PERMISSION => view.listing = Listing::NeedsPermission,
            _ if more => {}
            _ => view.listing = Listing::Failed(listing.error),
        }
    }

    /// Reads the drive's next page, when there is one and none is coming.
    fn drive_more(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(view) = self.library.cloud.view.as_mut() else {
            return;
        };
        if view.next.is_empty() || view._more.is_some() || view._load.is_some() {
            return;
        }
        let key = view.key();
        let account = view.account.0;
        let (place, what) = view.place();
        let page = view.next.clone();
        view._more = Some(cx.spawn(async move |this, cx| {
            let got = crate::daemon::cloud_list(&connection, account, place, &what, &page).await;
            this.update(cx, |this, cx| {
                this.drive_listed(key, got, true);
                cx.notify();
            })
            .ok();
        }));
    }

    /// Opens folder `entry`, from the folder shown or a search.
    fn enter_folder(&mut self, entry: &CloudEntry, cx: &mut Context<Self>) {
        let Some(view) = self.library.cloud.view.as_mut() else {
            return;
        };
        if !view.searched.is_empty() {
            // A folder found by a search opens on its own, from the top.
            view.crumbs.clear();
            view.searched.clear();
            view.search.clear();
            self.clear_drive_search_box(cx);
        }
        if let Some(view) = self.library.cloud.view.as_mut() {
            view.crumbs.push((entry.id.clone(), entry.name.clone()));
        }
        self.library.menu = None;
        self.load_drive(false, cx);
        cx.notify();
    }

    /// Goes back to crumb `depth`: 0 is the top of the drive.
    fn drive_crumb(&mut self, depth: usize, cx: &mut Context<Self>) {
        let Some(view) = self.library.cloud.view.as_mut() else {
            return;
        };
        let searching = !view.searched.is_empty();
        if depth >= view.crumbs.len() && !searching {
            return;
        }
        view.crumbs.truncate(depth);
        view.searched.clear();
        view.search.clear();
        if searching {
            self.clear_drive_search_box(cx);
        }
        self.load_drive(false, cx);
        cx.notify();
    }

    /// The search box changed while a drive shows: its words are looked
    /// for once typing rests, or at once on Enter.
    pub(super) fn drive_search(&mut self, text: String, now: bool, cx: &mut Context<Self>) {
        let Some(view) = self.library.cloud.view.as_mut() else {
            return;
        };
        view.search = text.trim().to_owned();
        if view.search == view.searched {
            view._wait = None;
            return;
        }
        if now {
            view._wait = None;
            view.searched = view.search.clone();
            self.load_drive(false, cx);
            return;
        }
        view._wait = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SEARCH_WAIT).await;
            this.update(cx, |this, cx| {
                if let Some(view) = this.library.cloud.view.as_mut() {
                    view._wait = None;
                    view.searched = view.search.clone();
                }
                this.load_drive(false, cx);
                cx.notify();
            })
            .ok();
        }));
    }

    /// Signs the drive's account in again, now allowing Katna to read
    /// the drive, then lists it.
    fn allow_drive_reading(&mut self, account: AccountId, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let provider = if self.drive_is_onedrive(account) {
            OAuthProvider::Microsoft
        } else {
            OAuthProvider::Google
        };
        let address = self
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.address.clone())
            .unwrap_or_default();
        cx.spawn(async move |this, cx| {
            let signed_in =
                crate::daemon::sign_in(&connection, provider, Some(account.0), &address).await;
            this.update(cx, |this, cx| {
                match signed_in {
                    Ok(_) => this.load_drive(true, cx),
                    Err(_) => this.show_snackbar(tr!("files-drive-allow-failed"), None, cx),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Whether the big button of the side column uploads into the open
    /// drive rather than composing: a drive of the account's own is on
    /// show, not what was shared with it.
    pub(in crate::window) fn drive_upload_here(&self) -> bool {
        self.app == super::super::RailApp::Files
            && self.settings_page.is_none()
            && self.library.cloud.view.as_ref().is_some_and(|v| !v.shared)
    }

    /// The Upload button's arrow, or a phone's button: the menu of what to
    /// upload, at `at`.
    pub(in crate::window) fn open_upload_menu(
        &mut self,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.library.menu = Some(Menu::Drive(DriveMenu::Upload(at)));
        cx.notify();
    }

    /// Asks for files, or with `folders` for folders, and uploads them into
    /// the folder on show.
    pub(in crate::window) fn upload_into_drive(&mut self, folders: bool, cx: &mut Context<Self>) {
        self.library.menu = None;
        cx.notify();
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(view) = self.library.cloud.view.as_ref().filter(|v| !v.shared) else {
            return;
        };
        let account = view.account;
        let crumbs = view.crumbs.clone();
        let folder = view
            .crumbs
            .last()
            .map(|(id, _)| id.clone())
            .unwrap_or_default();
        let key = (account, format!("{}:{folder}", cloud_place::FOLDER));
        let chosen = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: !folders,
            directories: folders,
            multiple: true,
            prompt: Some(tr!("files-drive-upload").into()),
        });
        self.watch_drive_uploads(cx);
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            let mut started = Vec::new();
            let mut failed = Vec::new();
            for path in &paths {
                let name = path.file_name().map_or_else(
                    || path.display().to_string(),
                    |n| n.to_string_lossy().into_owned(),
                );
                let text = path.to_string_lossy();
                let size = std::fs::metadata(path).map_or(0, |m| m.len());
                match crate::daemon::cloud_upload(&connection, account.0, &folder, &text).await {
                    Ok(id) => started.push((id, name, size)),
                    Err(err) => failed.push((name, err)),
                }
            }
            this.update(cx, |this, cx| {
                // The tray shows them going up.
                let cloud = &mut this.library.cloud;
                if !started.is_empty() {
                    cloud.tray_folded = false;
                }
                for (id, name, size) in started {
                    cloud.uploads.push((
                        id,
                        Upload {
                            name,
                            key: key.clone(),
                            crumbs: crumbs.clone(),
                            sent: 0,
                            size,
                            state: Going::Uploading,
                            started: Instant::now(),
                        },
                    ));
                }
                if let Some((name, error)) = failed.into_iter().next() {
                    let text = tr!(
                        "files-drive-upload-failed",
                        name = name.as_str(),
                        error = error
                    );
                    this.show_snackbar(text, None, cx);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Follows the daemon's uploads into drives, once one is asked for.
    fn watch_drive_uploads(&mut self, cx: &mut Context<Self>) {
        if self.library.cloud._uploads.is_some() {
            return;
        }
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self.library.cloud._uploads = Some(cx.spawn(async move |this, cx| {
            use futures_lite::StreamExt;
            let Ok(mut changes) = crate::daemon::drive_changes(&connection).await else {
                return;
            };
            while let Some(id) = changes.next().await {
                let ours = this
                    .read_with(cx, |this, _| {
                        this.library.cloud.uploads.iter().any(|(u, _)| *u == id)
                    })
                    .unwrap_or(false);
                if !ours {
                    continue;
                }
                let Ok(status) = crate::daemon::drive_upload_status(&connection, id).await else {
                    continue;
                };
                if this
                    .update(cx, |this, cx| this.drive_upload_changed(status, cx))
                    .is_err()
                {
                    return;
                }
            }
        }));
    }

    fn drive_upload_changed(&mut self, status: katna_dbus::DriveUpload, cx: &mut Context<Self>) {
        use katna_dbus::drive_state;
        let Some((_, upload)) = self
            .library
            .cloud
            .uploads
            .iter_mut()
            .find(|(id, _)| *id == status.id)
        else {
            return;
        };
        upload.sent = status.sent;
        if status.size > 0 {
            upload.size = status.size;
        }
        upload.state = match status.state.as_str() {
            drive_state::UPLOADING => Going::Uploading,
            drive_state::DONE => Going::Done,
            drive_state::NEEDS_PERMISSION => Going::NeedsPermission,
            _ => Going::Failed(status.error),
        };
        if upload.state != Going::Uploading {
            let key = upload.key.clone();
            let done = upload.state == Going::Done;
            // The folder it went into is read again when next on show.
            self.library.cloud.cache.remove(&key);
            if done
                && self
                    .library
                    .cloud
                    .view
                    .as_ref()
                    .is_some_and(|v| v.key() == key)
            {
                self.drive_listing_changed(key.0, cx);
            }
        }
        cx.notify();
    }

    /// Fetches drive file `entry` and does `act` with it.
    fn drive_act(
        &mut self,
        entry: CloudEntry,
        act: Act,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.library.menu = None;
        // A click elsewhere keeps the name being typed.
        if self.library.cloud.renaming.is_some() {
            self.finish_drive_rename(true, window, cx);
            return;
        }
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(account) = self.library.cloud.view.as_ref().map(|v| v.account) else {
            return;
        };
        if entry.folder {
            self.enter_folder(&entry, cx);
            return;
        }
        let kind = entry_kind(&entry);
        let fetched_kind = if entry.native { Kind::Pdf } else { kind };
        let risky = katna_preview::risky(&entry.mime, &entry.name);
        let item = Item {
            index: 0,
            name: entry.name.clone(),
            size: entry.size,
            kind: fetched_kind,
            risky,
        };
        // A file too big for mail, or one of the drive's own documents,
        // goes as a link from the drive.
        if act == Act::Attach && (entry.native || entry.size > MAX_TOTAL as u64) {
            self.new_mail_with_link(account, entry, window, cx);
            return;
        }
        // Where to save, asked first so the file chooser opens at once.
        let target = (act == Act::Save).then(|| {
            let name = if entry.native {
                format!("{}.pdf", entry.name)
            } else {
                entry.name.clone()
            };
            let dir = super::super::attachments::download_dir();
            (cx.prompt_for_new_path(&dir, Some(&name)), dir, name)
        });
        if !self.library.cloud.fetching.insert(entry.id.clone()) {
            return;
        }
        let provider = self.drive_name(account);
        let getting = SharedString::from(tr!(
            "files-drive-getting",
            name = entry.name.as_str(),
            drive = provider.as_str()
        ));
        // A file for the viewer opens it at once, turning until the file
        // is here; anything else says it is coming.
        let waiting =
            (act == Act::Open && entry.size <= VIEWER_MAX && self.open_in(&item) == OpenIn::Katna)
                .then(|| {
                    self.show_fetching_viewer(item.clone(), window, cx)
                        .downgrade()
                });
        if waiting.is_none() {
            self.show_snackbar(getting.clone(), None, cx);
        }
        cx.notify();
        cx.spawn_in(window, async move |this, cx| {
            let target = match target {
                Some((prompt, dir, name)) => match prompt.await {
                    Ok(Ok(Some(path))) => Some(path),
                    Ok(Ok(None)) => {
                        this.update(cx, |this, cx| {
                            this.library.cloud.fetching.remove(&entry.id);
                            cx.notify();
                        })
                        .ok();
                        return;
                    }
                    _ => Some(super::super::attachments::unique_path(&dir, &name)),
                },
                None => None,
            };
            let fetched = crate::daemon::cloud_fetch(&connection, account.0, &entry).await;
            let read = match &fetched {
                Ok(path)
                    if act != Act::Save && (act == Act::Attach || entry.size <= VIEWER_MAX) =>
                {
                    let path = std::path::PathBuf::from(path);
                    Some(
                        cx.background_executor()
                            .spawn(async move { std::fs::read(path) })
                            .await,
                    )
                }
                _ => None,
            };
            this.update_in(cx, |this, window, cx| {
                this.library.cloud.fetching.remove(&entry.id);
                // The note that it was coming goes once it is here.
                if let Some(snackbar) = &mut this.snackbar
                    && snackbar.text == getting
                {
                    snackbar.shown.set(0.0);
                }
                cx.notify();
                // The viewer waiting for it, unless it was closed meanwhile.
                let viewer = waiting.as_ref().map(|waiting| {
                    waiting
                        .upgrade()
                        .filter(|v| this.files.viewer.as_ref() == Some(v))
                });
                if matches!(viewer, Some(None)) {
                    return;
                }
                let path = match fetched {
                    Ok(path) => std::path::PathBuf::from(path),
                    Err(err) => {
                        if viewer.is_some() {
                            this.close_viewer(window, cx);
                        }
                        let text = tr!(
                            "files-drive-get-failed",
                            name = entry.name.as_str(),
                            error = err
                        );
                        this.show_snackbar(text, None, cx);
                        return;
                    }
                };
                let name = path
                    .file_name()
                    .map_or_else(|| entry.name.clone(), |n| n.to_string_lossy().into_owned());
                let mime = if entry.native {
                    "application/pdf".to_owned()
                } else {
                    entry.mime.clone()
                };
                match (act, read) {
                    (Act::Save, _) => {
                        let Some(target) = target else { return };
                        match std::fs::copy(&path, &target) {
                            Ok(_) => {
                                if this.config.mail.open_saved_folder {
                                    cx.reveal_path(&target);
                                }
                                let text =
                                    tr!("attachment-saved-to", path = target.display().to_string());
                                this.show_snackbar(text, None, cx);
                            }
                            Err(err) => {
                                let text = tr!(
                                    "attachment-save-failed",
                                    name = name.as_str(),
                                    error = err.to_string()
                                );
                                this.show_snackbar(text, None, cx);
                            }
                        }
                    }
                    (Act::Attach, Some(Ok(bytes))) => {
                        let file = katna_render::AttachmentFile { name, mime, bytes };
                        this.new_mail_with_file(&file, window, cx);
                    }
                    (Act::Open, Some(Ok(bytes))) if this.open_in(&item) == OpenIn::Katna => {
                        let raw = crate::outgoing::build(&crate::outgoing::Outgoing {
                            subject: name.clone(),
                            attachments: vec![crate::outgoing::Part {
                                name,
                                mime,
                                data: std::sync::Arc::new(bytes),
                                content_id: None,
                            }],
                            ..Default::default()
                        });
                        let item = Item {
                            name: item.name.clone(),
                            ..item
                        };
                        if let Some(Some(viewer)) = viewer {
                            let raw = std::sync::Arc::new(raw);
                            viewer.update(cx, |viewer, cx| viewer.arrived(raw, vec![item], cx));
                            return;
                        }
                        this.show_viewer(
                            std::sync::Arc::new(raw),
                            false,
                            vec![item],
                            0,
                            None,
                            window,
                            cx,
                        );
                    }
                    (_, Some(Err(err))) => {
                        let text = tr!(
                            "attachment-open-failed",
                            name = name.as_str(),
                            error = err.to_string()
                        );
                        this.show_snackbar(text, None, cx);
                    }
                    (_, _) if risky => this.show_snackbar(tr!("attachment-risky"), None, cx),
                    (_, _) => cx.open_with_system(&path),
                }
            })
            .ok();
        })
        .detach();
    }

    /// The thumbnail behind `entry`'s link, asking for it when it is not
    /// fetched yet.
    fn drive_thumb(&mut self, account: AccountId, entry: &CloudEntry) -> Option<Thumb> {
        if !self.config.mail.attachment_previews || entry.thumbnail.is_empty() {
            return None;
        }
        let frame = self.library.frame;
        let cloud = &mut self.library.cloud;
        if let Some((thumb, drawn)) = cloud.thumbs.get_mut(&entry.thumbnail) {
            *drawn = frame;
            return Some(thumb.clone());
        }
        if !cloud.asked.contains(&entry.thumbnail)
            && !cloud
                .wanted
                .iter()
                .any(|(_, link, _)| *link == entry.thumbnail)
        {
            let photo = matches!(
                katna_preview::kind(&entry.mime, &entry.name),
                Kind::Picture(_)
            );
            cloud.wanted.push((account, entry.thumbnail.clone(), photo));
        }
        None
    }

    /// Fetches the thumbnails the last frame asked for, a few at a time,
    /// and lets go of those not drawn for longest.
    fn request_drive_thumbs(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let cloud = &mut self.library.cloud;
        if cloud._thumbs.is_some() || cloud.wanted.is_empty() {
            return;
        }
        let batch: Vec<(AccountId, String, bool)> =
            cloud.wanted.drain(..).take(THUMB_BATCH).collect();
        cloud.wanted.clear();
        for (_, link, _) in &batch {
            cloud.asked.insert(link.clone());
        }
        cloud._thumbs = Some(cx.spawn(async move |this, cx| {
            let (w, h) = THUMB_PIXELS;
            let mut made = Vec::new();
            for (account, link, photo) in batch {
                let Ok(bytes) =
                    crate::daemon::cloud_thumbnail(&connection, account.0, &link, w).await
                else {
                    continue;
                };
                let thumb = cx
                    .background_executor()
                    .spawn(async move { card_picture(&bytes, photo, w, h).map(picture_thumb) })
                    .await;
                if let Some(thumb) = thumb {
                    made.push((link, thumb));
                }
            }
            this.update(cx, |this, cx| {
                let frame = this.library.frame;
                let cloud = &mut this.library.cloud;
                cloud._thumbs = None;
                for (link, thumb) in made {
                    cloud.thumbs.insert(link, (thumb, frame));
                }
                if cloud.thumbs.len() > THUMBS_KEPT {
                    let mut drawn: Vec<(String, u64)> = cloud
                        .thumbs
                        .iter()
                        .map(|(k, (_, f))| (k.clone(), *f))
                        .collect();
                    drawn.sort_by_key(|(_, f)| *f);
                    let extra = cloud.thumbs.len() - THUMBS_KEPT;
                    for (key, _) in drawn.into_iter().take(extra) {
                        if let Some((thumb, _)) = cloud.thumbs.remove(&key) {
                            this.files.released.extend(thumb.bitmaps());
                        }
                        cloud.asked.remove(&key);
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// The arrow keys over the drive: Left and Right step, Up and Down
    /// go a row, Enter opens, Backspace goes up a folder. Returns whether
    /// the key was the drive's.
    pub(super) fn on_drive_key(
        &mut self,
        key: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.library.cloud.renaming.is_some() {
            return false;
        }
        let at_cursor = self.library.cloud.view.as_ref().and_then(|view| {
            view.cursor
                .and_then(|at| view.order.get(at))
                .and_then(|&ix| view.entries.get(ix))
                .cloned()
        });
        match (key, at_cursor) {
            ("delete", Some(entry)) => {
                self.trash_drive_item(&entry, cx);
                return true;
            }
            ("f2", Some(entry)) => {
                self.start_drive_rename(&entry, window, cx);
                return true;
            }
            _ => {}
        }
        let Some(view) = self.library.cloud.view.as_mut() else {
            return false;
        };
        let count = view.order.len();
        let columns = if self.library.grid {
            view.columns.max(1)
        } else {
            1
        };
        let step = |by: isize| -> Option<usize> {
            let Some(at) = view.cursor else {
                return (count > 0).then_some(0);
            };
            let to = at as isize + by;
            (0..count as isize).contains(&to).then_some(to as usize)
        };
        let to = match key {
            "left" => step(-1),
            "right" => step(1),
            "up" => step(-(columns as isize)),
            "down" => step(columns as isize),
            "home" => (count > 0).then_some(0),
            "end" => count.checked_sub(1),
            "enter" | "space" => {
                let entry = view
                    .cursor
                    .and_then(|at| view.order.get(at))
                    .and_then(|&ix| view.entries.get(ix))
                    .cloned();
                if let Some(entry) = entry {
                    self.drive_act(entry, Act::Open, window, cx);
                }
                return true;
            }
            "backspace" => {
                let depth = view.crumbs.len().saturating_sub(1);
                self.drive_crumb(depth, cx);
                return true;
            }
            _ => return false,
        };
        if let Some(to) = to {
            view.cursor = Some(to);
            let line = view.lines.iter().position(|line| match line {
                DriveLine::Tiles(range) | DriveLine::Cards(range) => range.contains(&to),
                DriveLine::Row(at) => *at == to,
                DriveLine::Heading(_) => false,
            });
            if let Some(line) = line {
                view.state
                    .scroll_to_reveal_item(if line == 1 { 0 } else { line });
            }
            cx.notify();
        }
        true
    }

    // --- Drawing -------------------------------------------------------------

    /// The side column's Drives group: each account's drive and what was
    /// shared with it.
    pub(super) fn render_drives_nav(
        &self,
        nav: gpui::Div,
        heading: impl Fn(String) -> gpui::Div,
        rule: impl Fn() -> gpui::Div,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let cloud = &self.library.cloud;
        if cloud.drives.is_empty() {
            return nav;
        }
        let mut nav = nav.child(rule()).child(heading(tr!("files-drives")));
        let view = cloud.view.as_ref();
        for (n, (account, address)) in cloud.drives.iter().enumerate() {
            let name = self.drive_name(*account);
            for shared in [false, true] {
                let on = view.is_some_and(|v| v.account == *account && v.shared == shared);
                let account = *account;
                let label = div()
                    .flex()
                    .flex_col()
                    .child(div().truncate().child(if shared {
                        tr!("files-drive-shared")
                    } else {
                        name.clone()
                    }))
                    .child(
                        div()
                            .truncate()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::NORMAL)
                            .text_color(rgba(th.text_faint))
                            .child(address.clone()),
                    );
                let mark = if shared {
                    icon(
                        "people",
                        if on {
                            th.row_selected_text
                        } else {
                            th.text_dim
                        },
                        20.0,
                    )
                } else {
                    // The drive itself wears its maker's mark.
                    self.drive_mark_of(account, 20.0)
                };
                let row = super::super::nav::side_row_with(
                    ("files-drive", n * 2 + usize::from(shared)),
                    mark,
                    label,
                    on,
                    th,
                )
                .h(px(44.0));
                nav = nav.child(row.on_click(cx.listener(move |this, _, _, cx| {
                    this.open_drive(account, shared, cx);
                })));
            }
        }
        nav
    }

    /// The drive in place of the mail files: its bar, then its folders
    /// and files.
    pub(super) fn render_drive_body(
        &mut self,
        card_width: f32,
        columns: usize,
        pad: f32,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let grid = self.library.grid;
        let (time, sort) = (self.library.time, self.library.sort);
        let tz = self.tz.clone();
        let Some(view) = self.library.cloud.view.as_mut() else {
            return div().into_any_element();
        };
        let strip = self.layout.shape.is_phone();
        if view.stale || view.columns != columns || view.strip != strip {
            let line = thumb_height(card_width) + DRIVE_FOOT + GAP;
            view.rebuild(columns, grid, strip, time, sort, line, &tz);
        }
        let bar = self.render_drive_bar(pad, th, window, cx);
        let Some(view) = self.library.cloud.view.as_ref() else {
            return div().into_any_element();
        };
        let account = view.account;
        let content = match &view.listing {
            Listing::Loading => placeholder(&tr!("files-drive-loading"), th),
            Listing::NeedsPermission => {
                let allow = filled_button("files-drive-allow", tr!("files-drive-allow"), th)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.allow_drive_reading(account, cx);
                    }));
                drive_notice(
                    tr!("files-drive-needs-permission"),
                    allow,
                    self.drive_mark_of(account, 40.0),
                    th,
                )
            }
            Listing::Failed(err) => {
                let again = outlined_button("files-drive-again", tr!("files-drive-try-again"), th)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.load_drive(true, cx);
                        cx.notify();
                    }));
                let drive = self.drive_name(account);
                let unreachable = tr!("files-drive-unreachable", drive = drive.as_str());
                let text = if err.is_empty() {
                    unreachable
                } else {
                    format!("{unreachable}\n{err}")
                };
                drive_notice(text, again, self.drive_mark_of(account, 40.0), th)
            }
            Listing::Ready if view.order.is_empty() => placeholder(
                &if view.searched.is_empty() {
                    tr!("files-drive-empty")
                } else {
                    tr!("files-none-match")
                },
                th,
            ),
            Listing::Ready => {
                let items = list(
                    view.state.clone(),
                    cx.processor(move |this, ix: usize, window, cx| {
                        let th = this.theme(window);
                        let row = this.render_drive_line(ix, card_width, pad, &th, cx);
                        let near_end = this
                            .library
                            .cloud
                            .view
                            .as_ref()
                            .is_some_and(|v| ix + 3 >= v.lines.len());
                        if near_end {
                            this.drive_more(cx);
                        }
                        this.request_drive_thumbs(cx);
                        row
                    }),
                )
                .size_full();
                let thumb = th.text_dim & 0xffff_ff00 | 0x99;
                view.bar
                    .clone()
                    .wrap("files-drive-scroll", &view.state, items, thumb, window, cx)
                    .into_any_element()
            }
        };
        let focus = self
            .library
            .focus
            .get_or_insert_with(|| cx.focus_handle())
            .clone();
        if window.focused(cx).is_none_or(|f| f == self.list_focus) {
            window.focus(&focus, cx);
        }
        div()
            .id("files-drive")
            .track_focus(&focus)
            .on_key_down(cx.listener(Self::on_files_key))
            .size_full()
            .flex()
            .flex_col()
            .child(bar)
            .child(div().flex_1().min_h_0().child(content))
            .into_any_element()
    }

    /// The bar over a drive: its folder path and what it holds, then the
    /// type, time and order chips.
    fn render_drive_bar(
        &self,
        pad: f32,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(view) = self.library.cloud.view.as_ref() else {
            return div().into_any_element();
        };
        let desktop = self
            .picker
            .as_ref()
            .map_or(self.layout.shape.is_desktop(), |p| p.wide);
        let crumb_size = if self.picker.is_some() { 16.0 } else { 20.0 };
        let root = if view.shared {
            tr!("files-drive-shared")
        } else if self.drive_is_onedrive(view.account) {
            tr!("files-drive-mine-onedrive")
        } else {
            tr!("files-drive-mine")
        };
        let mut crumbs: Vec<(usize, String)> = vec![(0, root)];
        crumbs.extend(
            view.crumbs
                .iter()
                .enumerate()
                .map(|(n, (_, name))| (n + 1, name.clone())),
        );
        let searching = !view.searched.is_empty();
        if searching {
            crumbs.push((
                usize::MAX,
                tr!("files-drive-results", words = view.searched.as_str()),
            ));
        }
        // On a narrow page only the last two show, after an ellipsis.
        let keep = if desktop { 4 } else { 2 };
        let cut = crumbs.len() > keep;
        if cut {
            crumbs.drain(..crumbs.len() - keep);
        }
        let last = crumbs.len() - 1;
        let mut path = div()
            .flex_none()
            .max_w_full()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .text_size(px(crumb_size));
        if cut {
            path = path
                .child(div().text_color(rgba(th.text_faint)).child("…"))
                .child(icon("chevron-right", th.text_faint, 18.0));
        }
        for (n, (depth, name)) in crumbs.into_iter().enumerate() {
            if n > 0 {
                path = path.child(icon("chevron-right", th.text_faint, 18.0));
            }
            let current = n == last;
            path = path.child(
                div()
                    .id(("files-drive-crumb", n))
                    .min_w_0()
                    .truncate()
                    .px(px(4.0))
                    .rounded(px(6.0))
                    .text_color(rgba(if current { th.text } else { th.text_dim }))
                    .when(!current, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.drive_crumb(depth, cx);
                            }))
                    })
                    .child(name),
            );
        }
        let files = view.order.len() - view.folders;
        let meta = tr!("files-drive-count", folders = view.folders, files = files);
        let drop = self.baseline_drop(window, px(crumb_size), px(13.0));
        let title = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(6.0))
            .child(path)
            .when(matches!(view.listing, Listing::Ready), |d| {
                d.child(
                    div()
                        .min_w_0()
                        .relative()
                        .top(drop)
                        .truncate()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_faint))
                        .child(meta),
                )
            });
        let on = view.types != Types::All;
        let kind_chip = filter_chip("files-drive-kind", view.types.label(), on, th)
            .child(chip_arrow(on, th))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::Drive(DriveMenu::Kind(e.position)));
                    cx.notify();
                }),
            );
        let time_chip = self.files_time_chip(th, cx);
        let sort_chip = filter_chip("files-drive-sort", self.library.sort.label(), false, th)
            .child(chip_arrow(false, th))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::Sort(e.position));
                    cx.notify();
                }),
            );
        let chips = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(kind_chip)
            .child(time_chip)
            .child(sort_chip);
        // The picker shows cards only, its path smaller.
        let views = (self.picker.is_none()).then(|| self.files_view_buttons(th, cx));
        let rule = div()
            .flex_none()
            .mx(px(pad))
            .h(px(1.0))
            .bg(rgba(th.divider));
        if desktop {
            div()
                .flex_none()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(56.0))
                        .px(px(pad))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(title)
                        .child(chips)
                        .children(views),
                )
                .child(rule)
                .into_any_element()
        } else {
            div()
                .flex_none()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(52.0))
                        .px(px(pad))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(title)
                        .children(views),
                )
                .child(
                    div()
                        .id("files-drive-chips")
                        .overflow_x_scroll()
                        .px(px(pad))
                        .pb(px(10.0))
                        .flex()
                        .flex_row()
                        .gap(px(8.0))
                        .child(chips),
                )
                .child(rule)
                .into_any_element()
        }
    }

    fn render_drive_line(
        &mut self,
        ix: usize,
        card_width: f32,
        pad: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(view) = self.library.cloud.view.as_ref() else {
            return div().into_any_element();
        };
        let Some(line) = view.lines.get(ix).cloned() else {
            return div().into_any_element();
        };
        let order = view.order.clone();
        let entries = view.entries.clone();
        let columns = view.columns;
        let account = view.account;
        let strip = view.strip;
        let row = |items: Vec<AnyElement>, bottom: f32| {
            let empty = columns.saturating_sub(items.len());
            div()
                .w_full()
                .px(px(pad))
                .pb(px(bottom))
                .flex()
                .flex_row()
                .gap(px(GAP))
                .children(items)
                .children((0..empty).map(|_| div().flex_1().flex_basis(px(0.0)).min_w_0()))
                .into_any_element()
        };
        match line {
            DriveLine::Heading(folders) => div()
                .h(px(HEADING_HEIGHT))
                .px(px(pad))
                .pt(px(18.0))
                .text_size(px(14.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgba(th.text))
                .child(if folders {
                    tr!("files-drive-folders")
                } else {
                    tr!("files-drive-files")
                })
                .into_any_element(),
            DriveLine::Tiles(range) if strip => div()
                .id("files-drive-strip")
                .w_full()
                .overflow_x_scroll()
                .px(px(pad))
                .pb(px(10.0))
                .flex()
                .flex_row()
                .gap(px(10.0))
                .children(range.map(|at| {
                    self.render_folder_tile(at, &entries[order[at]], Some(STRIP_TILE), th, cx)
                }))
                .into_any_element(),
            DriveLine::Tiles(range) => {
                let tiles = range
                    .map(|at| self.render_folder_tile(at, &entries[order[at]], None, th, cx))
                    .collect();
                row(tiles, 10.0)
            }
            DriveLine::Cards(range) => {
                let cards = range
                    .map(|at| {
                        self.render_drive_card(at, account, &entries[order[at]], card_width, th, cx)
                    })
                    .collect();
                row(cards, GAP)
            }
            DriveLine::Row(at) => self.render_drive_row(at, &entries[order[at]], pad, th, cx),
        }
    }

    /// Handlers every drive item shares: open on click, the menu on
    /// right-click.
    fn drive_handlers(
        &self,
        el: gpui::Stateful<gpui::Div>,
        place: usize,
        entry: &CloudEntry,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<gpui::Div> {
        let open = entry.clone();
        // In the attach picker a click ticks a file and opens a folder.
        if let Some(account) = self.picker.as_ref().and(self.library.cloud.view.as_ref()) {
            let account = account.account;
            return el
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    if open.folder {
                        this.enter_folder(&open, cx);
                    } else {
                        this.toggle_drive_pick(account, &open, cx);
                    }
                }));
        }
        el.cursor_pointer()
            .on_click(cx.listener(move |this, _, window, cx| {
                if let Some(view) = this.library.cloud.view.as_mut() {
                    view.cursor = Some(place);
                }
                if let Some(focus) = &this.library.focus {
                    window.focus(focus, cx);
                }
                this.drive_act(open.clone(), Act::Open, window, cx);
            }))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::Drive(DriveMenu::Item {
                        place,
                        at: e.position,
                    }));
                    cx.notify();
                }),
            )
    }

    fn cursor_ring(&self, place: usize, radius: f32, th: &Theme) -> Option<AnyElement> {
        let on = self
            .library
            .cloud
            .view
            .as_ref()
            .is_some_and(|v| v.cursor == Some(place));
        on.then(|| {
            div()
                .absolute()
                .inset_0()
                .rounded(px(radius))
                .border_2()
                .border_color(rgba(th.accent))
                .into_any_element()
        })
    }

    /// A folder's tile: sharing its row's width, or `width` wide.
    fn render_folder_tile(
        &self,
        place: usize,
        entry: &CloudEntry,
        width: Option<f32>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // The padding sits inside, so the tiles share the row's width
        // exactly as the cards under them do.
        let tile = div()
            .id(("files-drive-folder", place))
            .relative()
            .map(|d| match width {
                Some(width) => d.flex_none().w(px(width)),
                None => d.flex_1().flex_basis(px(0.0)).min_w_0(),
            })
            .h(px(FOLDER_HEIGHT))
            .rounded(px(CARD_RADIUS))
            .border_1()
            .border_color(rgba(th.divider))
            .bg(rgba(th.surface))
            .hover(|s| s.bg(rgba(th.hover)))
            .child(
                div()
                    .size_full()
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .text_size(px(13.0))
                    .text_color(rgba(th.text))
                    .child(icon("folder", th.text_dim, 20.0))
                    .child(self.drive_name_el(entry, th)),
            )
            .children(self.cursor_ring(place, CARD_RADIUS, th));
        self.drive_handlers(tile, place, entry, cx)
            .into_any_element()
    }

    /// Item `entry`'s name, or the box its new name is typed in.
    fn drive_name_el(&self, entry: &CloudEntry, th: &Theme) -> AnyElement {
        match self
            .library
            .cloud
            .renaming
            .as_ref()
            .filter(|r| r.id == entry.id)
        {
            Some(renaming) => div()
                .id("files-drive-renaming")
                .flex_1()
                .min_w_0()
                .px(px(4.0))
                .rounded(px(4.0))
                .border_1()
                .border_color(rgba(th.accent))
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(renaming.input.clone())
                .into_any_element(),
            None => div()
                .min_w_0()
                .truncate()
                .child(entry.name.clone())
                .into_any_element(),
        }
    }

    /// The line under a drive file's name: its size or kind, and when it
    /// changed, or that it is being fetched.
    fn drive_meta(&self, entry: &CloudEntry) -> String {
        if self.library.cloud.fetching.contains(&entry.id) {
            return tr!("files-drive-fetching");
        }
        let what = if entry.native {
            match entry_kind(entry) {
                Kind::Sheet { .. } => tr!("files-drive-google-sheet"),
                Kind::Slides => tr!("files-drive-google-slides"),
                Kind::Picture(_) => tr!("files-drive-google-drawing"),
                _ => tr!("files-drive-google-doc"),
            }
        } else {
            format::size(entry.size)
        };
        let date = (entry.modified != 0)
            .then(|| format::local(entry.modified, &self.tz))
            .flatten()
            .zip(format::local(jiff::Timestamp::now().as_second(), &self.tz))
            .map(|(d, now)| format::list_date(d, now));
        // The picker says which files go as a link (Smart attach), in
        // place of the date.
        if self.picker.is_some() && (entry.native || entry.size > MAX_TOTAL as u64) {
            return tr!("files-drive-as-link", what = what.as_str());
        }
        match date {
            Some(date) => tr!(
                "files-drive-meta",
                what = what.as_str(),
                date = date.as_str()
            ),
            None => what,
        }
    }

    fn render_drive_card(
        &mut self,
        place: usize,
        account: AccountId,
        entry: &CloudEntry,
        width: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let thumb = self.drive_thumb(account, entry);
        let thumb_height = thumb_height(width);
        let kind = entry_kind(entry);
        let group = SharedString::from(format!("files-drive-card-{place}"));
        let frost = thumb.as_ref().and_then(Thumb::frosted);
        let picking = self.picker.is_some();
        let mut buttons = Vec::new();
        let e = entry.clone();
        buttons.push(
            panel_button(
                ("files-drive-attach", place),
                "attachment",
                tr!("files-drive-attach"),
                th,
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                cx.stop_propagation();
                this.drive_act(e.clone(), Act::Attach, window, cx);
            }))
            .into_any_element(),
        );
        let e = entry.clone();
        buttons.push(
            panel_button(("files-drive-open", place), "eye", tr!("files-open"), th)
                .on_click(cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();
                    this.drive_act(e.clone(), Act::Open, window, cx);
                }))
                .into_any_element(),
        );
        buttons.push(
            panel_button(
                ("files-drive-more", place),
                "more",
                tr!("files-drive-more"),
                th,
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::Drive(DriveMenu::Item {
                        place,
                        at: e.position,
                    }));
                    cx.notify();
                }),
            )
            .on_click(|_, _, cx| cx.stop_propagation())
            .into_any_element(),
        );
        // In the attach picker a card ticks like a mail file's, its eye
        // looking first.
        let panel = if picking {
            let e = entry.clone();
            pick_eye(("files-drive-eye", place), group.clone(), th)
                .on_click(cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();
                    this.drive_act(e.clone(), Act::Open, window, cx);
                }))
                .into_any_element()
        } else {
            hover_panel(
                group.clone(),
                entry.name.clone(),
                entry.size,
                frost,
                buttons,
                th,
            )
            .into_any_element()
        };
        let ticked = picking && self.drive_picked(account, &entry.id);
        let head = div()
            .relative()
            .flex_none()
            .h(px(thumb_height + NAME_HEIGHT))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(thumb_height))
                    .w_full()
                    .child(card_top(thumb, kind, 44.0, th)),
            )
            .child(
                div()
                    .h(px(NAME_HEIGHT))
                    .px(px(10.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .border_t_1()
                    .border_color(rgba(th.divider))
                    .child(kind_badge(kind, 18.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .text_size(px(13.0))
                            .text_color(rgba(th.text))
                            .child(self.drive_name_el(entry, th)),
                    ),
            )
            .child(panel);
        let card = div()
            .id(("files-drive-card", place))
            .group(group)
            .relative()
            .flex_1()
            .flex_basis(px(0.0))
            .min_w_0()
            .h(px(thumb_height + DRIVE_FOOT))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(CARD_RADIUS))
            .border_1()
            .border_color(rgba(th.divider))
            .bg(rgba(th.surface))
            .hover(|s| s.shadow(crate::widgets::elevation(th, 1.0)))
            .child(head)
            .child(
                div()
                    .h(px(DRIVE_FOOT - NAME_HEIGHT))
                    .px(px(10.0))
                    .pb(px(8.0))
                    .flex()
                    .items_center()
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(div().min_w_0().truncate().child(self.drive_meta(entry))),
            )
            .children(self.cursor_ring(place, CARD_RADIUS, th))
            .when(picking, |d| d.child(pick_tick(ticked, th)))
            .when(ticked, |d| d.child(pick_ring(th)));
        self.drive_handlers(card, place, entry, cx)
            .into_any_element()
    }

    fn render_drive_row(
        &self,
        place: usize,
        entry: &CloudEntry,
        pad: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let on = self
            .library
            .cloud
            .view
            .as_ref()
            .is_some_and(|v| v.cursor == Some(place));
        let badge = if entry.folder {
            div()
                .size(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .child(icon("folder", th.text_dim, 22.0))
                .into_any_element()
        } else {
            kind_badge(entry_kind(entry), 24.0)
        };
        let meta = if entry.folder {
            tr!("files-drive-folder")
        } else {
            self.drive_meta(entry)
        };
        let e = entry.clone();
        let row = div()
            .id(("files-drive-row", place))
            .w_full()
            .h(px(ROW_HEIGHT))
            .px(px(pad))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(14.0))
            .border_b_1()
            .border_color(rgba(th.divider))
            .hover(|s| s.bg(rgba(th.hover)))
            .when(on, |d| d.bg(rgba(th.nav_selected)))
            .child(badge)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text))
                            .child(self.drive_name_el(entry, th)),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(meta),
                    ),
            )
            .child(
                icon_button(("files-drive-row-more", place), "more", 20.0, th)
                    .tooltip(tip(tr!("files-drive-more"), th))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, e: &MouseDownEvent, _, cx| {
                            cx.stop_propagation();
                            this.library.menu = Some(Menu::Drive(DriveMenu::Item {
                                place,
                                at: e.position,
                            }));
                            cx.notify();
                        }),
                    )
                    .on_click(|_, _, cx| cx.stop_propagation()),
            );
        self.drive_handlers(row, place, &e, cx).into_any_element()
    }

    /// The items of a drive menu, and where it opens.
    pub(super) fn drive_menu_items(
        &self,
        menu: &DriveMenu,
        item: impl Fn(SharedString, &'static str, String, bool) -> gpui::Stateful<gpui::Div>,
        separator: impl Fn() -> AnyElement,
        cx: &mut Context<Self>,
    ) -> Option<(Point<Pixels>, Vec<AnyElement>)> {
        let view = self.library.cloud.view.as_ref()?;
        let drive_name = self.drive_name(view.account);
        match *menu {
            DriveMenu::Upload(at) => {
                let items = [
                    (false, "upload", tr!("files-drive-upload-files")),
                    (true, "folder", tr!("files-drive-upload-folder")),
                ]
                .into_iter()
                .map(|(folders, glyph, label)| {
                    item(
                        format!("files-drive-upload-{folders}").into(),
                        glyph,
                        label,
                        false,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.upload_into_drive(folders, cx);
                    }))
                    .into_any_element()
                })
                .collect();
                Some((at, items))
            }
            DriveMenu::Kind(at) => {
                let items = Types::ALL
                    .into_iter()
                    .enumerate()
                    .map(|(n, types)| {
                        item(
                            format!("files-drive-kind-{n}").into(),
                            types.icon(),
                            types.label(),
                            view.types == types,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.library.menu = None;
                            if let Some(view) = this.library.cloud.view.as_mut() {
                                view.types = types;
                                view.stale = true;
                            }
                            cx.notify();
                        }))
                        .into_any_element()
                    })
                    .collect();
                Some((at, items))
            }
            DriveMenu::Item { place, at } => {
                let entry = view.entries.get(*view.order.get(place)?)?.clone();
                let mut items = Vec::new();
                let e = entry.clone();
                items.push(
                    item(
                        "files-drive-menu-open".into(),
                        if entry.folder { "folder" } else { "eye" },
                        tr!("files-open"),
                        false,
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.drive_act(e.clone(), Act::Open, window, cx);
                    }))
                    .into_any_element(),
                );
                if !entry.folder {
                    let e = entry.clone();
                    items.push(
                        item(
                            "files-drive-menu-attach".into(),
                            "attachment",
                            tr!("files-drive-attach"),
                            false,
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.drive_act(e.clone(), Act::Attach, window, cx);
                        }))
                        .into_any_element(),
                    );
                    let e = entry.clone();
                    items.push(
                        item(
                            "files-drive-menu-save".into(),
                            "download",
                            tr!("files-drive-download"),
                            false,
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.drive_act(e.clone(), Act::Save, window, cx);
                        }))
                        .into_any_element(),
                    );
                }
                if !entry.link.is_empty() {
                    items.push(separator());
                    let link = entry.link.clone();
                    items.push(
                        item(
                            "files-drive-menu-web".into(),
                            "open-external",
                            tr!("files-drive-open-web", drive = drive_name.as_str()),
                            false,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.library.menu = None;
                            cx.open_url(&link);
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                    let link = entry.link.clone();
                    items.push(
                        item(
                            "files-drive-menu-link".into(),
                            "link",
                            tr!("files-drive-copy-link"),
                            false,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.library.menu = None;
                            cx.write_to_clipboard(ClipboardItem::new_string(link.clone()));
                            this.show_snackbar(tr!("files-drive-link-copied"), None, cx);
                        }))
                        .into_any_element(),
                    );
                }
                if self.drive_owned() {
                    items.push(separator());
                    let e = entry.clone();
                    items.push(
                        item(
                            "files-drive-menu-rename".into(),
                            "pen",
                            tr!("files-drive-rename"),
                            false,
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.start_drive_rename(&e, window, cx);
                        }))
                        .into_any_element(),
                    );
                    let e = entry.clone();
                    items.push(
                        item(
                            "files-drive-menu-trash".into(),
                            "trash",
                            tr!("files-drive-trash"),
                            false,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.trash_drive_item(&e, cx);
                        }))
                        .into_any_element(),
                    );
                }
                Some((at, items))
            }
        }
    }
}

/// Google Drive's mark, `size` px square, in its three colours, or
/// OneDrive's blue cloud.
fn drive_mark(onedrive: bool, size: f32) -> AnyElement {
    if onedrive {
        return super::super::mail_providers::layered(size, &[("onedrive", 0x0a62_c9ff)]);
    }
    super::super::mail_providers::layered(
        size,
        &[
            ("google-drive-green", 0x0f9d58ff),
            ("google-drive-yellow", 0xf4b400ff),
            ("google-drive-blue", 0x4285f4ff),
        ],
    )
}

/// A note in place of the drive's files, with a button.
fn drive_notice(
    text: String,
    button: gpui::Stateful<gpui::Div>,
    mark: AnyElement,
    th: &Theme,
) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(16.0))
        .p(px(24.0))
        .child(mark)
        .child(
            div()
                .max_w(px(420.0))
                .text_center()
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(text),
        )
        .child(button)
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(name: &str, folder: bool, modified: i64, size: u64) -> CloudEntry {
        CloudEntry {
            id: name.into(),
            name: name.into(),
            mime: if folder {
                String::new()
            } else {
                "application/pdf".into()
            },
            size,
            modified,
            folder,
            ..CloudEntry::default()
        }
    }

    #[test]
    fn folders_come_first_and_files_sort() {
        let mut view = DriveView::new(AccountId(1), false);
        view.entries = Rc::new(vec![
            entry("b.pdf", false, 200, 5),
            entry("Zeta", true, 0, 0),
            entry("a.pdf", false, 100, 9),
            entry("Alpha", true, 0, 0),
        ]);
        let tz = jiff::tz::TimeZone::UTC;
        view.rebuild(2, true, false, Time::Any, Sort::Name, 100.0, &tz);
        let names: Vec<_> = view
            .order
            .iter()
            .map(|&ix| view.entries[ix].name.as_str())
            .collect();
        assert_eq!(names, ["Alpha", "Zeta", "a.pdf", "b.pdf"]);
        assert_eq!(view.folders, 2);
        assert!(matches!(view.lines[0], DriveLine::Heading(true)));
        assert!(matches!(view.lines[2], DriveLine::Heading(false)));
        view.rebuild(2, true, false, Time::Any, Sort::Largest, 100.0, &tz);
        let first = &view.entries[view.order[2]];
        assert_eq!(first.name, "a.pdf");
        view.types = Types::Pictures;
        view.rebuild(2, false, false, Time::Any, Sort::Newest, 100.0, &tz);
        assert_eq!(view.order.len(), 2, "folders stay whatever the kind");
        assert!(view.lines.iter().all(|l| matches!(l, DriveLine::Row(_))));
    }

    #[test]
    fn places_ask_the_drive_for_the_right_thing() {
        let mut view = DriveView::new(AccountId(1), true);
        assert_eq!(view.place(), (cloud_place::SHARED, String::new()));
        view.crumbs.push(("f1".into(), "Q3".into()));
        assert_eq!(view.place(), (cloud_place::FOLDER, "f1".into()));
        view.searched = "budget".into();
        assert_eq!(view.place(), (cloud_place::SEARCH, "budget".into()));
        let mine = DriveView::new(AccountId(1), false);
        assert_eq!(mine.place(), (cloud_place::FOLDER, String::new()));
    }

    #[test]
    fn google_documents_wear_their_kind() {
        let mut doc = entry("Plan", false, 0, 0);
        doc.native = true;
        doc.mime = "application/vnd.google-apps.spreadsheet".into();
        assert!(matches!(entry_kind(&doc), Kind::Sheet { .. }));
        doc.mime = "application/vnd.google-apps.document".into();
        assert_eq!(entry_kind(&doc), Kind::Document);
        assert_eq!(picture_format(b"\x89PNG...."), Some(Picture::Png));
        assert_eq!(picture_format(b"RIFF\0\0\0\0WEBPVP8 "), Some(Picture::Webp));
        assert_eq!(picture_format(b"<svg"), None);
    }
}
