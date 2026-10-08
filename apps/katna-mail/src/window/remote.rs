// SPDX-License-Identifier: GPL-3.0-or-later

//! Remote images in messages, and pictures of people.
//!
//! Loading an image of a message from the web tells its server that the
//! message was opened, and when, and from where. So none is loaded until
//! the user says so: "Show images" for one message, or "Always show from"
//! a sender (kept one per line in `$XDG_CONFIG_HOME/katna/trusted-senders`),
//! or for all mail with Settings > General > Images from the web. Anyone
//! can write any `From`, so a trusted sender's images load only when the
//! user's provider vouched for the address (DMARC, or aligned DKIM; see
//! [`katna_render::sender_authenticated`]). A message loads at most
//! [`rich::MAX_REMOTE_IMAGES`] images, [`MAX_FETCHES`] at a time.
//!
//! SVG pictures, carried in a message or on the web, are drawn to bitmaps
//! by [`katna_preview::svg`] before GPUI sees them: GPUI's own SVG support
//! reads any local file an SVG links to.
//!
//! A sender's picture is their organization's BIMI logo or website icon.
//! Looking it up does reach the network: a DNS query for the
//! organization's BIMI record and HTTPS requests to its website, from this
//! computer. The organization can see those, and when one arrives soon
//! after it sent mail it can guess that the mail was shown. To keep that
//! small, the daemon looks up only the organizational domain (never the
//! sender's own subdomain, which could be unique to one recipient), keeps
//! the answer for a week, follows no URL outside that domain, and only
//! looks up senders whose mail the user's provider authenticated (DMARC
//! or aligned DKIM). It is shown for every such sender unless the "Sender
//! pictures" setting is off (then only for trusted senders). The user's own accounts show the picture picked for
//! the account in Settings (which can be the desktop user's picture),
//! else its picture from Google, for an account that signed in with Google
//! (the daemon saves it under `account-pictures/provider/`), else a
//! coloured letter, so each account looks different.
//! The daemon does all fetching; the app never uses the network.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gpui::{
    AnyElement, Context, ObjectFit, PathPromptOptions, RenderImage, SharedString, div, img,
    prelude::*, rgba,
};
use katna_core::image::ImageKind;
use katna_core::{AccountId, Paths};
use katna_i18n::tr;
use katna_store::MessageId;
use katna_ui::px;

use super::MailWindow;
use super::attachments::bitmap;
use super::rich;
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::{avatar, avatar_filled, icon};

/// Where a remote image or picture is.
pub(crate) enum Fetch {
    Loading,
    Ready(MailImage),
    /// Failed, or (for a picture) there is none.
    Missing,
}

/// A picture from a message, ready for GPUI: never SVG (see
/// [`mail_image`]).
#[derive(Clone)]
pub(crate) struct MailImage {
    pub image: Arc<gpui::Image>,
    /// The size to draw it at, in CSS pixels, for an SVG drawn at twice
    /// its size; `None` for a picture drawn at its own size.
    pub size: Option<(f32, f32)>,
}

/// Remote images fetched at once.
const MAX_FETCHES: usize = 6;
/// An SVG from mail is drawn at twice its size (sharp on a high-density
/// screen), at most this many pixels on a side.
const SVG_SIDE: u32 = 2048;

pub(crate) struct Remote {
    path: PathBuf,
    /// Lower-case addresses whose images and picture are always shown.
    trusted: BTreeSet<String>,
    /// Messages whose images the user chose to show, this session.
    shown: HashSet<MessageId>,
    /// Every message's images load (`mail.remote_images`).
    pub(super) always: bool,
    pub(super) images: HashMap<String, Fetch>,
    /// Remote images waiting for one of the [`MAX_FETCHES`], in order.
    queue: VecDeque<String>,
    fetching: usize,
    /// SVG pictures carried in the open messages, drawn to bitmaps, by the
    /// address of their bytes ([`svg_key`]); the bytes are kept here so
    /// the address stays theirs.
    pub(super) drawn: HashMap<usize, (Arc<[u8]>, Fetch)>,
    /// Sender pictures made to fill a circle, by lower-case domain;
    /// `None` while loading or when there is none.
    pictures: HashMap<String, Option<Arc<RenderImage>>>,
    /// Domains whose picture was asked for while drawing, each with an
    /// address there; fetched once the frame is built.
    wanted: RefCell<BTreeMap<String, String>>,
    /// The desktop user's picture, offered for an account in Settings.
    desktop: Option<Arc<gpui::Image>>,
    /// Pictures picked for accounts, by account ID.
    own: HashMap<i64, Arc<gpui::Image>>,
    own_dir: PathBuf,
    /// Pictures from the accounts' providers, by account ID, and when
    /// their folder last changed.
    provider: HashMap<i64, Arc<gpui::Image>>,
    provider_seen: Option<std::time::SystemTime>,
    /// An installed monospace font, found on first use.
    mono: Option<Option<SharedString>>,
}

impl Remote {
    pub(super) fn load(paths: &Paths) -> Self {
        let path = paths.trusted_senders_file();
        let trusted = std::fs::read_to_string(&path)
            .map(|text| {
                text.lines()
                    .map(|l| l.trim().to_ascii_lowercase())
                    .filter(|l| l.contains('@') && !l.starts_with('#'))
                    .collect()
            })
            .unwrap_or_default();
        let own_dir = paths.account_pictures_dir();
        let own = std::fs::read_dir(&own_dir)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let id = entry.file_name().to_str()?.parse::<i64>().ok()?;
                Some((id, read_picture(&entry.path())?))
            })
            .collect();
        let mut remote = Self {
            path,
            trusted,
            shown: HashSet::new(),
            always: false,
            images: HashMap::new(),
            queue: VecDeque::new(),
            fetching: 0,
            drawn: HashMap::new(),
            pictures: HashMap::new(),
            wanted: RefCell::default(),
            desktop: desktop_picture().map(|(_, picture)| picture),
            own,
            provider: HashMap::new(),
            provider_seen: None,
            own_dir,
            mono: None,
        };
        remote.load_provider_pictures();
        remote
    }

    /// Reads the providers' account pictures again if the daemon saved a
    /// new one.
    pub(super) fn load_provider_pictures(&mut self) {
        let dir = self.own_dir.join("provider");
        let modified = std::fs::metadata(&dir).and_then(|m| m.modified()).ok();
        if modified == self.provider_seen {
            return;
        }
        self.provider_seen = modified;
        self.provider = std::fs::read_dir(&dir)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let id = entry.file_name().to_str()?.parse::<i64>().ok()?;
                Some((id, read_picture(&entry.path())?))
            })
            .collect();
    }

    pub(super) fn trusts(&self, sender: &str) -> bool {
        !sender.is_empty() && self.trusted.contains(&sender.to_ascii_lowercase())
    }

    /// Whether remote content of `message` from `sender` may load. A
    /// trusted sender counts only when the provider vouched for the
    /// address (`authenticated`): anyone can write any `From`.
    pub(super) fn allowed(&self, message: MessageId, sender: &str, authenticated: bool) -> bool {
        self.always || self.shown.contains(&message) || (authenticated && self.trusts(sender))
    }

    fn trust(&mut self, sender: &str) {
        if !self.trusted.insert(sender.to_ascii_lowercase()) {
            return;
        }
        let mut text = String::from("# Senders whose remote images Katna Mail shows.\n");
        for sender in &self.trusted {
            text.push_str(sender);
            text.push('\n');
        }
        if let Err(err) = std::fs::create_dir_all(self.path.parent().unwrap_or(&self.path))
            .and_then(|()| std::fs::write(&self.path, text))
        {
            tracing::warn!(path = %self.path.display(), %err, "cannot save trusted senders");
        }
    }

    /// Whether a picture was picked for `account` (else it shows a
    /// letter).
    pub(super) fn has_own_picture(&self, account: AccountId) -> bool {
        self.own.contains_key(&account.0)
    }

    /// Whether the desktop's user has a picture to use for an account.
    pub(super) fn has_desktop_picture(&self) -> bool {
        self.desktop.is_some()
    }

    /// The monospace font found by [`Self::find_mono`].
    pub(super) fn mono(&self) -> Option<SharedString> {
        self.mono.clone().flatten()
    }

    pub(super) fn find_mono(&mut self, cx: &gpui::App) {
        self.mono.get_or_insert_with(|| {
            let installed = cx.text_system().all_font_names();
            [
                "DejaVu Sans Mono",
                "Noto Sans Mono",
                "Liberation Mono",
                "Hack",
                "Ubuntu Mono",
                "Source Code Pro",
                "Cascadia Mono",
                "Monospace",
            ]
            .into_iter()
            .find(|f| installed.iter().any(|i| i == f))
            .map(SharedString::from)
        });
    }
}

/// Largest picture read for an account.
const MAX_OWN_PICTURE: u64 = 8 * 1024 * 1024;

/// A picture for GPUI, known by its first bytes. An SVG is drawn to a
/// bitmap first, with nothing it links to loaded, and kept to a sane size.
pub(super) fn mail_image(bytes: Vec<u8>) -> Option<MailImage> {
    let kind = ImageKind::sniff(&bytes)?;
    let Some(format) = rich::format(kind) else {
        let drawing = katna_preview::svg::draw(&bytes, 2.0, SVG_SIDE)
            .map_err(|err| tracing::debug!(err = err.0, "SVG not drawn"))
            .ok()?;
        let png = katna_preview::svg::png(&drawing.image).ok()?;
        return Some(MailImage {
            image: Arc::new(gpui::Image::from_bytes(gpui::ImageFormat::Png, png)),
            size: Some(drawing.size),
        });
    };
    Some(MailImage {
        image: Arc::new(gpui::Image::from_bytes(format, bytes)),
        size: None,
    })
}

fn image(bytes: Vec<u8>) -> Option<Arc<gpui::Image>> {
    mail_image(bytes).map(|picture| picture.image)
}

/// The key of SVG bytes carried in a message in [`Remote::drawn`].
pub(super) fn svg_key(bytes: &Arc<[u8]>) -> usize {
    Arc::as_ptr(bytes).cast::<u8>().addr()
}

fn read_picture(path: &Path) -> Option<Arc<gpui::Image>> {
    let meta = std::fs::metadata(path).ok()?;
    if !meta.is_file() || meta.len() > MAX_OWN_PICTURE {
        return None;
    }
    image(std::fs::read(path).ok()?)
}

/// The picture of the desktop's user: `~/.face.icon` (KDE), the one the
/// system's user settings keep (AccountsService), or `~/.face`.
fn desktop_picture() -> Option<(PathBuf, Arc<gpui::Image>)> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .ok()
        .filter(|u| !u.is_empty() && !u.contains('/'));
    let candidates = [
        home.as_ref().map(|h| h.join(".face.icon")),
        user.map(|u| Path::new("/var/lib/AccountsService/icons").join(u)),
        home.as_ref().map(|h| h.join(".face")),
    ];
    candidates
        .into_iter()
        .flatten()
        .find_map(|p| Some((p.clone(), read_picture(&p)?)))
}

/// The domain of `email`, when it looks like one.
impl MailWindow {
    /// The logo of the organization at `email`, once fetched.
    pub(super) fn domain_logo(&self, email: &str) -> Option<Arc<RenderImage>> {
        self.remote.pictures.get(&domain_of(email)?)?.clone()
    }
}

fn domain_of(email: &str) -> Option<String> {
    let (_, domain) = email.trim().rsplit_once('@')?;
    let domain = domain.trim().trim_end_matches('.').to_ascii_lowercase();
    domain.contains('.').then_some(domain)
}

/// A photo filling a circle.
fn photo(picture: Arc<gpui::Image>, size: f32) -> AnyElement {
    img(picture)
        .size(px(size))
        .flex_none()
        .rounded_full()
        .object_fit(ObjectFit::Cover)
        .into_any_element()
}

/// A sender's logo, already made to fill its circle.
pub(super) fn logo(picture: Arc<RenderImage>, size: f32) -> AnyElement {
    img(picture)
        .size(px(size))
        .flex_none()
        .rounded_full()
        .into_any_element()
}

/// A sender picture as the daemon sent it, made to fill a circle: its
/// margin trimmed, and cropped to the circle or put on a disc of its own
/// background (see [`katna_preview::avatar`]).
pub(super) fn sender_logo(bytes: Vec<u8>) -> Option<Arc<RenderImage>> {
    let square = match ImageKind::sniff(&bytes)? {
        ImageKind::Svg => katna_preview::avatar::from_svg(&bytes),
        _ => katna_preview::avatar::from_bytes(&bytes),
    };
    match square {
        Ok(square) => Some(bitmap(square)),
        Err(err) => {
            tracing::debug!(err = err.0, "sender picture not drawn");
            None
        }
    }
}

impl MailWindow {
    /// Starts fetching the remote images of the open conversation that
    /// may show and are not here yet, and drawing the SVG pictures its
    /// messages carry.
    pub(super) fn fetch_remote(&mut self, cx: &mut Context<Self>) {
        self.remote.find_mono(cx);
        let Some(reader) = &self.reader else {
            return;
        };
        let mut urls = Vec::new();
        for content in reader.remote_content() {
            if !self
                .remote
                .allowed(content.id, &content.sender, content.authenticated)
            {
                continue;
            }
            urls.extend(
                content
                    .urls
                    .iter()
                    .filter(|url| !self.remote.images.contains_key(*url))
                    .cloned(),
            );
        }
        let svgs = reader.carried_svgs();
        for url in urls {
            if !self.remote.images.contains_key(&url) {
                self.remote.images.insert(url.clone(), Fetch::Loading);
                self.remote.queue.push_back(url);
            }
        }
        self.fetch_queued(cx);
        self.draw_svgs(svgs, cx);
    }

    /// Fetches queued remote images, at most [`MAX_FETCHES`] at once.
    fn fetch_queued(&mut self, cx: &mut Context<Self>) {
        while self.remote.fetching < MAX_FETCHES
            && let Some(url) = self.remote.queue.pop_front()
        {
            self.remote.fetching += 1;
            let connection = self.daemon.clone();
            cx.spawn(async move |this, cx| {
                let fetch_url = url.clone();
                let image = cx
                    .background_executor()
                    .spawn(async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        let bytes = daemon::fetch_image(&connection, &fetch_url).await?;
                        Ok::<_, String>(mail_image(bytes))
                    })
                    .await
                    .ok()
                    .flatten();
                this.update(cx, |this, cx| {
                    this.remote.fetching = this.remote.fetching.saturating_sub(1);
                    let fetch = image.map_or(Fetch::Missing, Fetch::Ready);
                    this.remote.images.insert(url, fetch);
                    this.fetch_queued(cx);
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
    }

    /// Draws the SVG pictures carried in the open messages (`svgs`) that
    /// are not drawn yet, and forgets those of messages no longer open.
    fn draw_svgs(&mut self, svgs: Vec<Arc<[u8]>>, cx: &mut Context<Self>) {
        let open: HashSet<usize> = svgs.iter().map(svg_key).collect();
        self.remote.drawn.retain(|key, _| open.contains(key));
        for bytes in svgs {
            let key = svg_key(&bytes);
            if self.remote.drawn.contains_key(&key) {
                continue;
            }
            self.remote
                .drawn
                .insert(key, (bytes.clone(), Fetch::Loading));
            cx.spawn(async move |this, cx| {
                let image = cx
                    .background_executor()
                    .spawn(async move { mail_image(bytes.to_vec()) })
                    .await;
                this.update(cx, |this, cx| {
                    if let Some((_, fetch)) = this.remote.drawn.get_mut(&key) {
                        *fetch = image.map_or(Fetch::Missing, Fetch::Ready);
                        cx.notify();
                    }
                })
                .ok();
            })
            .detach();
        }
    }

    /// Starts fetching the sender pictures asked for while drawing.
    pub(super) fn fetch_pictures(&mut self, cx: &mut Context<Self>) {
        let wanted = std::mem::take(&mut *self.remote.wanted.borrow_mut());
        for (domain, address) in wanted {
            if self.remote.pictures.contains_key(&domain) {
                continue;
            }
            self.remote.pictures.insert(domain.clone(), None);
            let connection = self.daemon.clone();
            cx.spawn(async move |this, cx| {
                let result = cx
                    .background_executor()
                    .spawn(async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        let bytes = daemon::sender_picture(&connection, &address).await?;
                        Ok::<_, String>(sender_logo(bytes))
                    })
                    .await;
                this.update(cx, |this, cx| {
                    let logo = result.ok().flatten();
                    let seen = logo.is_some();
                    this.remote.pictures.insert(domain, logo);
                    if !seen {
                        return;
                    }
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
    }

    /// The picture of the person at `email`: the user's own picture for
    /// their accounts, else their organization's logo once fetched, else
    /// their initial.
    pub(super) fn person_avatar(&self, name: &str, email: &str, size: f32) -> AnyElement {
        if let Some(picture) = self.own_picture(email) {
            return photo(picture, size);
        }
        // The picture saved with their contact.
        if let Some(picture) = self.saved_photo(email) {
            return logo(picture, size);
        }
        if let Some(domain) = domain_of(email)
            .filter(|_| self.config.mail.sender_pictures || self.remote.trusts(email))
        {
            match self.remote.pictures.get(&domain) {
                Some(Some(picture)) => return logo(picture.clone(), size),
                Some(None) => {}
                None => {
                    self.remote
                        .wanted
                        .borrow_mut()
                        .entry(domain)
                        .or_insert_with(|| email.trim().to_owned());
                }
            }
        }
        // One of the user's accounts wears its own color.
        let own = self
            .accounts
            .iter()
            .find(|a| !email.trim().is_empty() && a.address.eq_ignore_ascii_case(email.trim()));
        match own {
            Some(account) => avatar_filled(name, self.account_fill(&account.address), size),
            None => avatar(name, email, size),
        }
    }

    /// The picture of the user's account at `email`, if it is one.
    fn own_picture(&self, email: &str) -> Option<Arc<gpui::Image>> {
        let email = email.trim();
        let account = self
            .accounts
            .iter()
            .find(|a| !email.is_empty() && a.address.eq_ignore_ascii_case(email))?;
        let id = account.id.0;
        self.remote
            .own
            .get(&id)
            .or_else(|| self.remote.provider.get(&id))
            .cloned()
    }

    /// Asks for a picture file and uses it for `account`.
    pub(super) fn pick_account_picture(&mut self, account: AccountId, cx: &mut Context<Self>) {
        let chosen = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(tr!("remote-picture-use").into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            let read = cx
                .background_executor()
                .spawn(async move {
                    let meta = std::fs::metadata(&path)?;
                    if meta.len() > MAX_OWN_PICTURE {
                        return Ok(None);
                    }
                    std::fs::read(&path).map(Some)
                })
                .await;
            this.update(cx, |this, cx| {
                let bytes = match read {
                    Ok(Some(bytes)) => bytes,
                    Ok(None) => {
                        this.show_snackbar(tr!("remote-picture-too-big"), None, cx);
                        return;
                    }
                    Err(err) => {
                        this.show_snackbar(
                            tr!("remote-picture-read-failed", error = err.to_string()),
                            None,
                            cx,
                        );
                        return;
                    }
                };
                let Some(picture) = image(bytes.clone()) else {
                    this.show_snackbar(tr!("remote-picture-type"), None, cx);
                    return;
                };
                let dir = this.remote.own_dir.clone();
                if let Err(err) = std::fs::create_dir_all(&dir)
                    .and_then(|()| std::fs::write(dir.join(account.0.to_string()), &bytes))
                {
                    this.show_snackbar(
                        tr!("remote-picture-keep-failed", error = err.to_string()),
                        None,
                        cx,
                    );
                    return;
                }
                this.remote.own.insert(account.0, picture);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Uses the desktop user's picture for `account`, as a copy, so the
    /// account keeps it if the desktop's changes.
    pub(super) fn use_desktop_picture(&mut self, account: AccountId, cx: &mut Context<Self>) {
        let Some((path, picture)) = desktop_picture() else {
            return;
        };
        let dir = self.remote.own_dir.clone();
        if let Err(err) = std::fs::create_dir_all(&dir)
            .and_then(|()| std::fs::copy(&path, dir.join(account.0.to_string())).map(drop))
        {
            self.show_snackbar(
                tr!("remote-picture-keep-failed", error = err.to_string()),
                None,
                cx,
            );
            return;
        }
        self.remote.own.insert(account.0, picture);
        cx.notify();
    }

    /// Takes away the picture of `account`, which shows its letter again.
    pub(super) fn reset_account_picture(&mut self, account: AccountId, cx: &mut Context<Self>) {
        let path = self.remote.own_dir.join(account.0.to_string());
        match std::fs::remove_file(&path) {
            Err(err) if err.kind() != std::io::ErrorKind::NotFound => {
                self.show_snackbar(
                    tr!("remote-picture-remove-failed", error = err.to_string()),
                    None,
                    cx,
                );
            }
            _ => {
                self.remote.own.remove(&account.0);
                cx.notify();
            }
        }
    }

    /// "Images are hidden" with the two ways to show them. For a sender
    /// who is trusted already, but whose address the provider did not
    /// vouch for, it says so, and only "Show images" is offered.
    pub(super) fn images_banner(
        &self,
        ix: usize,
        id: MessageId,
        sender: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let unconfirmed = self.remote.trusts(sender);
        let link = |label: SharedString, id: (&'static str, usize)| {
            div()
                .id(id)
                .px(px(8.0))
                .py(px(2.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .text_color(rgba(th.accent))
                .relative()
                .child(crate::widgets::hover_fade("hover-glow", Some(6.0), th))
                .child(label)
        };
        let sender_owned = sender.to_owned();
        div()
            .mb(px(12.0))
            .px(px(12.0))
            .py(px(6.0))
            .rounded(px(8.0))
            .bg(rgba(th.read_row))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(4.0))
            .text_size(px(12.0))
            .text_color(rgba(th.text_dim))
            .child(icon("image", th.text_faint, 16.0))
            .child(
                div()
                    .min_w_0()
                    .pl(px(4.0))
                    .pr(px(4.0))
                    .child(if unconfirmed {
                        tr!("remote-hidden-unconfirmed")
                    } else {
                        tr!("remote-hidden")
                    }),
            )
            .child(
                link(tr!("remote-show").into(), ("show-images", ix)).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.remote.shown.insert(id);
                        this.fetch_remote(cx);
                        cx.notify();
                    },
                )),
            )
            .when(!sender.is_empty() && !unconfirmed, |d| {
                d.child(
                    link(tr!("remote-always-show").into(), ("trust-sender", ix)).on_click(
                        cx.listener(move |this, _, _, cx| {
                            this.remote.trust(&sender_owned);
                            this.fetch_remote(cx);
                            cx.notify();
                        }),
                    ),
                )
            })
            .into_any_element()
    }
}
