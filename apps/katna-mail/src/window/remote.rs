// SPDX-License-Identifier: GPL-3.0-or-later

//! Remote images and sender pictures in the reading pane.
//!
//! Loading anything from the web tells its server that the message was
//! opened, and when, and from where. So nothing is loaded until the user
//! says so: "Show images" for one message, or "Always show from" a sender.
//! A trusted sender's picture (their organization's BIMI logo or website
//! icon) is shown too. The daemon does the fetching; the app never uses
//! the network. Trusted senders are kept one per line in
//! `$XDG_CONFIG_HOME/katna/trusted-senders`.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AnyElement, Context, SharedString, div, img, prelude::*, px, rgba};
use katna_core::Paths;
use katna_core::image::ImageKind;
use katna_store::MessageId;

use super::MailWindow;
use super::rich;
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::{avatar, icon};

/// Where a remote image or picture is.
pub(crate) enum Fetch {
    Loading,
    Ready(Arc<gpui::Image>),
    /// Failed, or (for a picture) there is none.
    Missing,
}

pub(crate) struct Remote {
    path: PathBuf,
    /// Lower-case addresses whose images and picture are always shown.
    trusted: BTreeSet<String>,
    /// Messages whose images the user chose to show, this session.
    shown: HashSet<MessageId>,
    pub(super) images: HashMap<String, Fetch>,
    /// Sender pictures, by lower-case address.
    pictures: HashMap<String, Fetch>,
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
        Self {
            path,
            trusted,
            shown: HashSet::new(),
            images: HashMap::new(),
            pictures: HashMap::new(),
            mono: None,
        }
    }

    pub(super) fn trusts(&self, sender: &str) -> bool {
        !sender.is_empty() && self.trusted.contains(&sender.to_ascii_lowercase())
    }

    /// Whether remote content of `message` from `sender` may load.
    pub(super) fn allowed(&self, message: MessageId, sender: &str) -> bool {
        self.shown.contains(&message) || self.trusts(sender)
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

    fn picture(&self, sender: &str) -> Option<Arc<gpui::Image>> {
        match self.pictures.get(&sender.to_ascii_lowercase()) {
            Some(Fetch::Ready(image)) => Some(image.clone()),
            _ => None,
        }
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

fn image(bytes: Vec<u8>) -> Option<Arc<gpui::Image>> {
    let kind = ImageKind::sniff(&bytes)?;
    Some(Arc::new(gpui::Image::from_bytes(rich::format(kind), bytes)))
}

impl MailWindow {
    /// Starts fetching what the open conversation may show and does not
    /// have yet: remote images of allowed messages, pictures of trusted
    /// senders.
    pub(super) fn fetch_remote(&mut self, cx: &mut Context<Self>) {
        self.remote.find_mono(cx);
        let Some(reader) = &self.reader else {
            return;
        };
        let mut urls = Vec::new();
        let mut senders = Vec::new();
        for (id, sender, remote) in reader.remote_content() {
            if !self.remote.allowed(id, &sender) {
                continue;
            }
            let key = sender.to_ascii_lowercase();
            if !key.is_empty() && !self.remote.pictures.contains_key(&key) {
                senders.push(key);
            }
            urls.extend(
                remote
                    .iter()
                    .filter(|url| !self.remote.images.contains_key(*url))
                    .cloned(),
            );
        }
        for url in urls {
            self.remote.images.insert(url.clone(), Fetch::Loading);
            let connection = self.daemon.clone();
            cx.spawn(async move |this, cx| {
                let fetch_url = url.clone();
                let result = cx
                    .background_executor()
                    .spawn(async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        daemon::fetch_image(&connection, &fetch_url).await
                    })
                    .await;
                this.update(cx, |this, cx| {
                    let fetch = match result.ok().and_then(image) {
                        Some(image) => Fetch::Ready(image),
                        None => Fetch::Missing,
                    };
                    this.remote.images.insert(url, fetch);
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
        for sender in senders {
            self.remote.pictures.insert(sender.clone(), Fetch::Loading);
            let connection = self.daemon.clone();
            cx.spawn(async move |this, cx| {
                let address = sender.clone();
                let result = cx
                    .background_executor()
                    .spawn(async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        daemon::sender_picture(&connection, &address).await
                    })
                    .await;
                this.update(cx, |this, cx| {
                    let fetch = match result.ok().and_then(image) {
                        Some(image) => Fetch::Ready(image),
                        None => Fetch::Missing,
                    };
                    this.remote.pictures.insert(sender, fetch);
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
    }

    /// The sender's picture when there is one, else their initial.
    pub(super) fn sender_avatar(&self, name: &str, email: &str, size: f32) -> AnyElement {
        match self.remote.picture(email) {
            Some(picture) => div()
                .size(px(size))
                .flex_none()
                .rounded_full()
                .overflow_hidden()
                .bg(rgba(0xffffffff))
                .flex()
                .items_center()
                .justify_center()
                .child(img(picture).size(px(size * 0.8)).rounded(px(size * 0.1)))
                .into_any_element(),
            None => avatar(name, email, size),
        }
    }

    /// "Images are hidden" with the two ways to show them.
    pub(super) fn images_banner(
        &self,
        ix: usize,
        id: MessageId,
        sender: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let link = |label: SharedString, id: (&'static str, usize)| {
            div()
                .id(id)
                .px(px(8.0))
                .py(px(2.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .text_color(rgba(th.accent))
                .hover(|s| s.bg(rgba(th.hover)))
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
                    .pl(px(4.0))
                    .pr(px(4.0))
                    .child("Images in this message are hidden."),
            )
            .child(
                link("Show images".into(), ("show-images", ix)).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.remote.shown.insert(id);
                        this.fetch_remote(cx);
                        cx.notify();
                    },
                )),
            )
            .when(!sender.is_empty(), |d| {
                d.child(
                    link("Always show from this sender".into(), ("trust-sender", ix)).on_click(
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
