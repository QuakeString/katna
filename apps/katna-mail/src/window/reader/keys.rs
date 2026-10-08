// SPDX-License-Identifier: GPL-3.0-or-later

//! Keys in the reading view (plan E.3): a popover from the banner with the
//! details of the key a message was signed with, and one that offers to
//! import a key, found in the sender's Web Key Directory ("Look up key")
//! or attached to the message. Nothing is looked up or imported without
//! the user's click: a lookup would tell the sender the mail was opened.
//! Imported keys go to the user's own GnuPG keyring; Undo takes a key
//! that was new back out.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{
    AnimationExt, AnyElement, Bounds, ClipboardItem, Context, Div, ElementId, FontWeight, Pixels,
    Point, Window, canvas, deferred, div, point, prelude::*, rgba,
};
use katna_crypto::{Gnupg, KeyInfo, Signature, SignatureState, Standard, Validity};
use katna_i18n::tr;
use katna_render::Attachment;
use katna_store::MessageId;
use katna_ui::anchored;
use katna_ui::tokens::{space, text};
use katna_ui::{px, unpx};

use super::security::{Secured, green};
use crate::daemon::{self, Command};
use crate::format;
use crate::theme::Theme;
use crate::widgets::{ButtonStyle, button, filled_button, icon, spinner, text_button};
use crate::window::MailWindow;
use crate::window::notched::{self, notch};

const WIDTH: f32 = 360.0;
/// The height assumed until the popover is measured.
const FIRST_HEIGHT: f32 = 320.0;
/// The width of the labels before the values.
const LABEL: f32 = 92.0;
/// The colour of a key attachment's badge.
const KEY_COLOR: u32 = 0x00897bff;

/// The open key popover.
pub(in crate::window) struct KeyCard {
    message: MessageId,
    /// What it points at.
    at: Bounds<Pixels>,
    shows: Shows,
    /// Its height as last laid out, and whether it has been measured: it
    /// shows from then on.
    height: Rc<Cell<f32>>,
    measured: Rc<Cell<bool>>,
    /// Import was pressed and GnuPG is at work.
    importing: bool,
    /// When it closed and began to fade out.
    closing: Option<Instant>,
}

enum Shows {
    /// The key a signature was made with.
    Signature {
        signature: Box<Signature>,
        standard: Standard,
    },
    /// Asking the sender's domain for a key.
    LookingUp { domain: String },
    /// The domain has none.
    NotFound { domain: String },
    /// A key to import.
    Import {
        key: Box<KeyInfo>,
        data: Arc<Vec<u8>>,
        from: Source,
    },
    /// Looking up or importing failed, in GnuPG's or the daemon's words.
    Failed(String),
}

/// Where a key to import came from.
enum Source {
    /// The Web Key Directory of this domain.
    Directory(String),
    /// The attachment of this name.
    Attachment(String),
}

/// Whether `attachment` looks like an OpenPGP public key: its type, or a
/// key file's name. What it holds is checked when it is clicked.
pub(in crate::window) fn is_key_attachment(attachment: &Attachment) -> bool {
    is_key_file(&attachment.name, &attachment.mime)
}

/// [`is_key_attachment`] for a file known by its name and type, like a
/// list row's attachment chip.
pub(in crate::window) fn is_key_file(name: &str, mime: &str) -> bool {
    let mime = mime.trim().to_ascii_lowercase();
    if mime == "application/pgp-keys" {
        return true;
    }
    let name = name.to_ascii_lowercase();
    let key_file = [".asc", ".key", ".pub", ".gpg", ".pgp"]
        .iter()
        .any(|ext| name.ends_with(ext));
    // The parts of signed and encrypted mail are named like key files.
    let protection = ["signature.asc", "encrypted.asc", "msg.asc", "smime.p7s"]
        .iter()
        .any(|part| name == *part);
    key_file
        && !protection
        && matches!(
            mime.as_str(),
            "application/octet-stream" | "text/plain" | "application/pgp" | ""
        )
}

/// The top of a key attachment's card: its badge on the card's fill.
pub(in crate::window) fn key_card_top(badge: f32, th: &Theme) -> Div {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .bg(rgba(th.read_row))
        .child(key_badge(badge))
}

/// A teal square with a key, for key attachments.
pub(in crate::window) fn key_badge(size: f32) -> AnyElement {
    div()
        .size(px(size))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(size * 0.2))
        .bg(rgba(KEY_COLOR))
        .child(icon("key", 0xffffffff, size * 0.72))
        .into_any_element()
}

impl MailWindow {
    /// Opens the details of signature `ix` of message `id`, pointing at
    /// the banner line `spot` clicked at `click`.
    pub(super) fn open_key_details(
        &mut self,
        id: MessageId,
        ix: usize,
        spot: ElementId,
        click: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let Some((signature, standard)) = self.signature_of(id, ix) else {
            return;
        };
        let at = self.person_at(spot, click);
        self.show_key_card(
            id,
            at,
            Shows::Signature {
                signature: Box::new(signature),
                standard,
            },
        );
        cx.notify();
    }

    fn signature_of(&self, id: MessageId, ix: usize) -> Option<(Signature, Standard)> {
        let part = self.reader.as_ref()?.parts.iter().find(|p| p.id == id)?;
        match part.body.as_ref()?.security.as_ref()? {
            Secured::Opened(security) => {
                Some((security.signatures.get(ix)?.clone(), security.standard))
            }
            Secured::Opening(_) => None,
        }
    }

    fn show_key_card(&mut self, message: MessageId, at: Bounds<Pixels>, shows: Shows) {
        self.key_card = Some(KeyCard {
            message,
            at,
            shows,
            height: Rc::new(Cell::new(FIRST_HEIGHT)),
            measured: Rc::default(),
            importing: false,
            closing: None,
        });
    }

    /// Looks up a key for the sender of message `id` in their domain's
    /// Web Key Directory, through the daemon, and offers to import it.
    pub(super) fn look_up_sender_key(
        &mut self,
        id: MessageId,
        spot: ElementId,
        click: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let Some(address) = self
            .reader
            .as_ref()
            .and_then(|r| r.view(Some(id)))
            .and_then(|view| view.from.first())
            .map(|from| from.email.trim().to_lowercase())
        else {
            return;
        };
        let domain = address
            .rsplit_once('@')
            .map(|(_, domain)| domain.to_owned())
            .unwrap_or_default();
        let at = self.person_at(spot, click);
        self.show_key_card(
            id,
            at,
            Shows::LookingUp {
                domain: domain.clone(),
            },
        );
        cx.notify();
        let connection = self.daemon.clone();
        let peers = self.paths.peer_keys_dir();
        cx.spawn(async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    let fingerprint = daemon::look_up_key(&connection, &address).await?;
                    if fingerprint.is_empty() {
                        return Ok(None);
                    }
                    // The daemon kept it for encrypting; read it from there.
                    let (_, file) = katna_crypto::PeerKeys::new(peers)
                        .get(&address)
                        .ok_or_else(|| tr!("key-card-not-kept"))?;
                    let data = std::fs::read(file).map_err(|err| err.to_string())?;
                    let key = katna_crypto::show_keys(&Gnupg::new(), &data)
                        .into_iter()
                        .find(|key| key.fingerprint == fingerprint)
                        .ok_or_else(|| tr!("key-card-not-kept"))?;
                    Ok::<_, String>(Some((key, data)))
                })
                .await;
            this.update(cx, |this, cx| {
                let Some(card) = this
                    .key_card
                    .as_mut()
                    .filter(|c| c.message == id && c.closing.is_none())
                else {
                    return;
                };
                if !matches!(card.shows, Shows::LookingUp { .. }) {
                    return;
                }
                card.shows = match found {
                    Ok(Some((key, data))) => Shows::Import {
                        key: Box::new(key),
                        data: Arc::new(data),
                        from: Source::Directory(domain),
                    },
                    Ok(None) => Shows::NotFound { domain },
                    Err(err) => Shows::Failed(err),
                };
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// A click on the key attachment `index` of message `id`: offers to
    /// import the key in it, or opens it as usual when it holds none.
    pub(in crate::window) fn open_key_attachment(
        &mut self,
        id: MessageId,
        index: usize,
        spot: ElementId,
        click: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((raw, _)) = self.attachment_raw(id) else {
            self.open_attachment(id, index, window, cx);
            return;
        };
        let at = self.person_at(spot, click);
        cx.spawn_in(window, async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move {
                    let file = katna_render::attachment_file(&raw, index)?;
                    let key = katna_crypto::show_keys(&Gnupg::new(), &file.bytes)
                        .into_iter()
                        .next()?;
                    Some((key, file.bytes, file.name))
                })
                .await;
            this.update_in(cx, |this, window, cx| match found {
                Some((key, data, name)) => {
                    this.show_key_card(
                        id,
                        at,
                        Shows::Import {
                            key: Box::new(key),
                            data: Arc::new(data),
                            from: Source::Attachment(name),
                        },
                    );
                    cx.notify();
                }
                None => this.open_attachment(id, index, window, cx),
            })
            .ok();
        })
        .detach();
    }

    /// Imports the key the popover shows into the user's keyring, then
    /// checks the open messages' signatures again.
    fn import_shown_key(&mut self, cx: &mut Context<Self>) {
        let Some(card) = self.key_card.as_mut().filter(|c| !c.importing) else {
            return;
        };
        let Shows::Import { data, .. } = &card.shows else {
            return;
        };
        let data = data.clone();
        card.importing = true;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let imported = cx
                .background_executor()
                .spawn(async move { katna_crypto::import_keys(&Gnupg::new(), &data) })
                .await;
            this.update(cx, |this, cx| match imported {
                Ok(imported) => {
                    this.close_key_card(cx);
                    this.recheck_signatures(cx);
                    if imported.new.is_empty() {
                        this.show_snackbar(tr!("toast-key-updated"), None, cx);
                    } else {
                        this.show_snackbar(
                            tr!("toast-key-imported"),
                            Some(Command::RemoveKeys(imported.new)),
                            cx,
                        );
                    }
                }
                Err(err) => {
                    if let Some(card) = this.key_card.as_mut() {
                        card.importing = false;
                        card.shows = Shows::Failed(err);
                    }
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    /// Undo of an import: takes the keys that were new back out of the
    /// keyring.
    pub(in crate::window) fn remove_imported_keys(
        &mut self,
        fingerprints: Vec<String>,
        cx: &mut Context<Self>,
    ) {
        cx.spawn(async move |this, cx| {
            let removed = cx
                .background_executor()
                .spawn(async move {
                    let gnupg = Gnupg::new();
                    fingerprints
                        .iter()
                        .all(|fingerprint| katna_crypto::delete_key(&gnupg, fingerprint))
                })
                .await;
            this.update(cx, |this, cx| {
                this.recheck_signatures(cx);
                let note = if removed {
                    tr!("toast-key-removed")
                } else {
                    tr!("toast-key-not-removed")
                };
                this.show_snackbar(note, None, cx);
            })
            .ok();
        })
        .detach();
    }

    /// Checks again every open message whose signature was checked, after
    /// the keyring changed.
    fn recheck_signatures(&mut self, cx: &mut Context<Self>) {
        let Some(reader) = &self.reader else {
            return;
        };
        let signed: Vec<MessageId> = reader
            .parts
            .iter()
            .filter(|p| {
                p.body.as_ref().is_some_and(
                    |b| matches!(&b.security, Some(Secured::Opened(s)) if !s.signatures.is_empty()),
                )
            })
            .map(|p| p.id)
            .collect();
        for id in signed {
            self.retry_sealed(id, cx);
        }
    }

    /// Closes the popover, if open.
    pub(in crate::window) fn close_key_card(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(card) = self.key_card.as_mut().filter(|c| c.closing.is_none()) else {
            return false;
        };
        card.closing = notched::fade_out(cx);
        if card.closing.is_none() {
            self.key_card = None;
        }
        cx.notify();
        true
    }

    /// The popover, over everything, its notch on what opened it.
    pub(in crate::window) fn render_key_card(
        &mut self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if let Some(since) = self.key_card.as_ref().and_then(|c| c.closing)
            && notched::faded(since, cx)
        {
            self.key_card = None;
        }
        // It goes with its message.
        let message = self.key_card.as_ref()?.message;
        let open = self.reading
            && self
                .reader
                .as_ref()
                .is_some_and(|r| r.parts.iter().any(|p| p.id == message));
        if !open {
            self.key_card = None;
            return None;
        }
        let card = self.key_card.as_ref()?;
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let height = card.height.get().min(vh - 2.0 * notched::MARGIN);
        let (x, y, side, along) =
            notched::place(card.at, (WIDTH, height), (vw, vh), notched::RADIUS);
        let (measured, seen, closing) = (card.height.clone(), card.measured.clone(), card.closing);
        let ready = seen.get();
        let body = self.key_card_body(card, th, cx);
        let view = cx.entity_id();
        let measure = canvas(
            move |bounds, window, _| {
                // With the border round it.
                let h = unpx(bounds.size.height) + 2.0;
                if (measured.get() - h).abs() > 0.5 || !seen.get() {
                    measured.set(h);
                    seen.set(true);
                    window.refresh();
                    window.on_next_frame(move |_, cx| cx.notify(view));
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();
        let popover = div()
            .id("key-card")
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(WIDTH))
            .map(|d| notched::popover(d, th))
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.close_key_card(cx);
            }))
            .child(div().relative().child(body).child(measure))
            .children(notch(side, along, th));
        let popover = match closing {
            Some(_) => notched::fading(popover, "key-card-out"),
            // Laid out unseen once, to learn its height.
            None if !ready => popover.opacity(0.0).into_any_element(),
            None => popover
                .with_animation(
                    "key-card-in",
                    gpui::Animation::new(katna_ui::motion::time(Duration::from_millis(160)))
                        .with_easing(gpui::ease_out_quint()),
                    |el, t| el.opacity(t),
                )
                .into_any_element(),
        };
        let layer = div().relative().w(px(vw)).h(px(vh)).child(popover);
        Some(
            deferred(anchored().position(point(px(0.0), px(0.0))).child(layer))
                .with_priority(3)
                .into_any_element(),
        )
    }

    fn key_card_body(&self, card: &KeyCard, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let column = div().flex().flex_col().text_size(px(text::BODY));
        match &card.shows {
            Shows::Signature {
                signature,
                standard,
            } => {
                let (name, color, title, detail) = signature_summary(signature, th);
                let mut rows = Vec::new();
                if let Some(key) = &signature.details {
                    rows.push((tr!("key-card-signed-by"), owner(key, th)));
                    rows.push((
                        tr!("key-card-fingerprint"),
                        fingerprint(&key.fingerprint, th),
                    ));
                }
                if let Some(when) = signature.created.and_then(|at| format::local(at, &self.tz)) {
                    rows.push((tr!("key-card-signed"), value(format::long_date(when), th)));
                }
                if let Some(key) = &signature.details {
                    self.key_rows(key, *standard, &mut rows, th);
                    if let Some(issuer) = &key.issuer {
                        rows.push((tr!("key-card-issued-by"), value(issuer.clone(), th)));
                    }
                }
                rows.push((tr!("key-card-found-in"), value(tr!("key-card-keyring"), th)));
                let copy = signature.details.as_ref().map(|key| {
                    let fingerprint = key.fingerprint.clone();
                    div()
                        .px(px(space::S3))
                        .pb(px(space::S3))
                        .flex()
                        .flex_row()
                        .child(
                            text_button("key-card-copy", "copy", tr!("key-card-copy"), th)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        fingerprint.clone(),
                                    ));
                                    this.show_snackbar(tr!("toast-fingerprint-copied"), None, cx);
                                })),
                        )
                });
                column
                    .child(header(icon(name, color, 18.0), title, detail, th))
                    .child(rule(th))
                    .child(table(rows, th))
                    .children(copy)
                    .into_any_element()
            }
            Shows::LookingUp { domain } => column
                .child(header(
                    spinner("key-card-spinner", th.text_dim, 18.0),
                    tr!("key-card-looking-up"),
                    tr!("key-card-looking-up-detail", domain = domain.as_str()),
                    th,
                ))
                .into_any_element(),
            Shows::NotFound { domain } => column
                .child(header(
                    icon("key", th.text_dim, 18.0),
                    tr!("key-card-not-found"),
                    tr!("key-card-not-found-detail", domain = domain.as_str()),
                    th,
                ))
                .into_any_element(),
            Shows::Failed(reason) => column
                .child(header(
                    icon("info", th.error, 18.0),
                    tr!("key-card-failed"),
                    format::sentence(reason),
                    th,
                ))
                .into_any_element(),
            Shows::Import { key, from, .. } => {
                let from = match from {
                    Source::Directory(domain) => {
                        tr!("key-card-from-directory", domain = domain.as_str())
                    }
                    Source::Attachment(name) => {
                        tr!("key-card-from-attachment", name = name.as_str())
                    }
                };
                let mut rows = vec![
                    (tr!("key-card-belongs-to"), owner(key, th)),
                    (
                        tr!("key-card-fingerprint"),
                        fingerprint(&key.fingerprint, th),
                    ),
                ];
                self.key_rows(key, Standard::OpenPgp, &mut rows, th);
                let importing = card.importing;
                let cancel = button("key-card-cancel", ButtonStyle::Text, th)
                    .child(tr!("key-card-cancel"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.close_key_card(cx);
                    }));
                let import = filled_button("key-card-import", tr!("key-card-import"), th)
                    .when(importing, |b| b.opacity(0.6))
                    .on_click(cx.listener(|this, _, _, cx| this.import_shown_key(cx)));
                column
                    .child(header(
                        icon("key", th.text, 18.0),
                        tr!("key-card-import-title"),
                        from,
                        th,
                    ))
                    .child(rule(th))
                    .child(table(rows, th))
                    .child(rule(th))
                    .child(
                        div()
                            .px(px(space::S5))
                            .pt(px(space::S4))
                            .flex()
                            .flex_row()
                            .items_start()
                            .gap(px(space::S3))
                            .text_size(px(text::SMALL))
                            .text_color(rgba(th.text_dim))
                            .child(icon("info", th.text_faint, 18.0))
                            .child(div().flex_1().min_w_0().child(tr!("key-card-import-note"))),
                    )
                    .child(
                        div()
                            .p(px(space::S5))
                            .flex()
                            .flex_row()
                            .justify_end()
                            .gap(px(space::S3))
                            .child(cancel)
                            .child(import),
                    )
                    .into_any_element()
            }
        }
    }

    /// The Key, Created and Expires rows of `key`.
    fn key_rows(
        &self,
        key: &KeyInfo,
        standard: Standard,
        rows: &mut Vec<(String, AnyElement)>,
        th: &Theme,
    ) {
        let standard = match standard {
            Standard::OpenPgp => "OpenPGP",
            Standard::Smime => "S/MIME",
        };
        let kind = if key.algorithm.is_empty() {
            standard.to_owned()
        } else {
            tr!(
                "key-card-kind",
                standard = standard,
                algorithm = key.algorithm.as_str()
            )
        };
        rows.push((tr!("key-card-key"), value(kind, th)));
        let day = |at: i64| format::local(at, &self.tz).map(katna_i18n::format::day_month_year);
        if let Some(created) = key.created.and_then(day) {
            rows.push((tr!("key-card-created"), value(created, th)));
        }
        let expires = match key.expires {
            None => Some(tr!("key-card-never")),
            Some(at) => day(at),
        };
        if let Some(expires) = expires {
            rows.push((tr!("key-card-expires"), value(expires, th)));
        }
    }
}

/// The icon, its colour, the title and the line under it for the details
/// of `signature`.
fn signature_summary(signature: &Signature, th: &Theme) -> (&'static str, u32, String, String) {
    match signature.state {
        SignatureState::Good if signature.verified() => (
            "shield-check",
            green(th),
            tr!("key-card-verified"),
            tr!("key-card-verified-detail"),
        ),
        SignatureState::Good if !signature.from_sender => (
            "shield-alert",
            th.error,
            tr!("key-card-not-sender"),
            tr!("key-card-not-sender-detail"),
        ),
        SignatureState::Good if signature.validity == Validity::Never => (
            "shield-alert",
            th.error,
            tr!("key-card-untrusted"),
            tr!("key-card-untrusted-detail"),
        ),
        SignatureState::Good => (
            "shield",
            th.text_dim,
            tr!("key-card-unverified"),
            tr!("key-card-unverified-detail"),
        ),
        SignatureState::Expired => (
            "shield",
            th.text_dim,
            tr!("key-card-signature-expired"),
            tr!("key-card-signature-expired-detail"),
        ),
        SignatureState::KeyExpired => (
            "shield",
            th.text_dim,
            tr!("key-card-key-expired"),
            tr!("key-card-key-expired-detail"),
        ),
        SignatureState::KeyRevoked => (
            "shield-alert",
            th.error,
            tr!("key-card-key-revoked"),
            tr!("key-card-key-revoked-detail"),
        ),
        SignatureState::Bad
        | SignatureState::MissingKey
        | SignatureState::Unavailable
        | SignatureState::Error => (
            "shield-alert",
            th.error,
            tr!("key-card-bad"),
            tr!("key-card-bad-detail"),
        ),
    }
}

/// The popover's top: an icon, a title and a line under it.
fn header(icon: AnyElement, title: String, detail: String, th: &Theme) -> Div {
    div()
        .p(px(space::S5))
        .flex()
        .flex_row()
        .items_start()
        .gap(px(space::S3))
        .child(div().pt(px(space::S1)).child(icon))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(space::S1))
                .child(
                    div()
                        .text_size(px(text::SUBTITLE))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(th.text))
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(text::SMALL))
                        .text_color(rgba(th.text_dim))
                        .child(detail),
                ),
        )
}

/// The faint line between the popover's sections.
fn rule(th: &Theme) -> Div {
    div().h(px(1.0)).w_full().bg(rgba(th.divider))
}

/// Labels and values, one pair a row.
fn table(rows: Vec<(String, AnyElement)>, th: &Theme) -> Div {
    div()
        .px(px(space::S5))
        .py(px(space::S4))
        .flex()
        .flex_col()
        .gap(px(space::S3))
        .text_size(px(text::SMALL))
        .children(rows.into_iter().map(|(label, value)| {
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S3))
                .child(
                    div()
                        .w(px(LABEL))
                        .flex_none()
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
                .child(div().flex_1().min_w_0().child(value))
        }))
}

fn value(text: String, th: &Theme) -> AnyElement {
    div()
        .text_color(rgba(th.text))
        .child(text)
        .into_any_element()
}

/// The key's owner: the name, and each address on a line of its own.
fn owner(key: &KeyInfo, th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .text_color(rgba(th.text))
        .children(key.name.clone())
        .children(key.emails.iter().map(|email| div().child(email.clone())))
        .into_any_element()
}

/// A fingerprint in groups of four, five groups a line, as GnuPG and
/// Kleopatra show it.
fn fingerprint(fingerprint: &str, th: &Theme) -> AnyElement {
    let groups: Vec<&str> = fingerprint
        .as_bytes()
        .chunks(4)
        .map(|g| std::str::from_utf8(g).unwrap_or_default())
        .collect();
    div()
        .flex()
        .flex_col()
        .font_family("monospace")
        .text_color(rgba(th.text))
        .children(groups.chunks(5).map(|line| div().child(line.join(" "))))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attachment(name: &str, mime: &str) -> Attachment {
        Attachment {
            name: name.into(),
            size: 1,
            mime: mime.into(),
            content_id: None,
        }
    }

    #[test]
    fn key_attachments() {
        assert!(is_key_attachment(&attachment(
            "OpenPGP_0x044D0F0EBC2DE42E.asc",
            "application/pgp-keys"
        )));
        assert!(is_key_attachment(&attachment(
            "ada-public-key.asc",
            "application/octet-stream"
        )));
        assert!(is_key_attachment(&attachment("ada.pub", "text/plain")));
        assert!(!is_key_attachment(&attachment(
            "signature.asc",
            "application/octet-stream"
        )));
        assert!(!is_key_attachment(&attachment("notes.txt", "text/plain")));
        assert!(!is_key_attachment(&attachment("photo.key", "image/png")));
    }
}
