// SPDX-License-Identifier: GPL-3.0-or-later

//! A tray icon (`docs/ARCHITECTURE.md` §15.2): a StatusNotifierItem with a
//! [`dbusmenu`](crate::dbusmenu) right-click menu. Plasma shows it in its
//! system tray; GNOME shows it with the AppIndicator extension (on by
//! default on Ubuntu). The icon carries a badge with the unread count.
//!
//! The item takes a name of its own, `org.kde.StatusNotifierItem-PID-N`, and
//! registers it with `org.kde.StatusNotifierWatcher`, again whenever the
//! watcher restarts. Hiding releases the name, which removes the icon.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use futures_lite::StreamExt;
use zbus::fdo::{DBusProxy, RequestNameFlags};
use zbus::names::BusName;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::OwnedObjectPath;
use zbus::{Connection, Task};

use crate::dbusmenu::{Menu, MenuItem};
use crate::icon::{self, Style};

const ITEM_PATH: &str = "/StatusNotifierItem";
const MENU_PATH: &str = "/StatusNotifierItem/Menu";
const WATCHER: &str = "org.kde.StatusNotifierWatcher";

/// Sizes the icon is drawn at; panels pick the closest.
const SIZES: [u32; 6] = [16, 22, 24, 32, 48, 64];

/// `a(iiay)`: width, height and ARGB32 pixels in network byte order.
type Pixmaps = Vec<(i32, i32, Vec<u8>)>;

/// What the tray calls its handler with, besides menu actions.
pub const ACTIVATE: &str = "activate";
/// A middle click.
pub const SECONDARY_ACTIVATE: &str = "secondary-activate";

/// A click on the icon or its menu: [`ACTIVATE`], [`SECONDARY_ACTIVATE`]
/// or a menu item's action, with the activation token the panel gave for
/// raising a window, if any.
pub type Handler = Arc<dyn Fn(&str, Option<String>) + Send + Sync>;

/// What the icon shows.
struct Look {
    title: String,
    /// The app's icon name; `-symbolic` names its one-color icon.
    id: String,
    style: Style,
    /// The tooltip's second line.
    status: String,
    badge: Option<String>,
    pixmaps: Pixmaps,
}

impl Look {
    fn new(title: String, id: String, style: Style) -> Self {
        let mut look = Self {
            title,
            id,
            style,
            status: String::new(),
            badge: None,
            pixmaps: Vec::new(),
        };
        look.draw();
        look
    }

    /// The icon from the icon theme, which the panel draws at its size and,
    /// for the one-color icon, in its own text color.
    fn icon_name(&self) -> String {
        match self.style {
            Style::Color => self.id.clone(),
            Style::Mono(_) => format!("{}-symbolic", self.id),
        }
    }

    fn draw(&mut self) {
        self.pixmaps = SIZES
            .iter()
            .map(|&size| {
                let side = i32::try_from(size).unwrap_or(i32::MAX);
                let argb = icon::tray_icon_argb(size, self.style, self.badge.as_deref());
                (side, side, argb)
            })
            .collect();
    }
}

struct ItemObject {
    id: String,
    look: Arc<Mutex<Look>>,
    token: Mutex<Option<String>>,
    handler: Handler,
}

impl ItemObject {
    fn run(&self, action: &str) {
        let token = self.token.lock().unwrap().take();
        (self.handler)(action, token);
    }
}

#[zbus::interface(name = "org.kde.StatusNotifierItem")]
impl ItemObject {
    fn activate(&self, _x: i32, _y: i32) {
        self.run(ACTIVATE);
    }

    fn secondary_activate(&self, _x: i32, _y: i32) {
        self.run(SECONDARY_ACTIVATE);
    }

    /// Panels without menu support ask for one here; they get the window.
    fn context_menu(&self, _x: i32, _y: i32) {
        self.run(ACTIVATE);
    }

    fn scroll(&self, _delta: i32, _orientation: &str) {}

    /// Plasma sends a token before `Activate`, so the window may take focus
    /// on Wayland.
    fn provide_xdg_activation_token(&self, token: String) {
        *self.token.lock().unwrap() = Some(token);
    }

    #[zbus(property)]
    fn category(&self) -> &str {
        "Communications"
    }

    #[zbus(property)]
    fn id(&self) -> &str {
        &self.id
    }

    #[zbus(property)]
    fn title(&self) -> String {
        self.look.lock().unwrap().title.clone()
    }

    #[zbus(property)]
    fn status(&self) -> &str {
        "Active"
    }

    #[zbus(property)]
    fn window_id(&self) -> i32 {
        0
    }

    /// Empty while there is a badge, so panels draw the pixmaps.
    #[zbus(property)]
    fn icon_name(&self) -> String {
        let look = self.look.lock().unwrap();
        if look.badge.is_some() {
            String::new()
        } else {
            look.icon_name()
        }
    }

    #[zbus(property)]
    fn icon_pixmap(&self) -> Pixmaps {
        self.look.lock().unwrap().pixmaps.clone()
    }

    #[zbus(property)]
    fn icon_theme_path(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn overlay_icon_name(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn overlay_icon_pixmap(&self) -> Pixmaps {
        Vec::new()
    }

    #[zbus(property)]
    fn attention_icon_name(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn attention_icon_pixmap(&self) -> Pixmaps {
        Vec::new()
    }

    #[zbus(property)]
    fn attention_movie_name(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn tool_tip(&self) -> (String, Pixmaps, String, String) {
        let look = self.look.lock().unwrap();
        (
            look.icon_name(),
            Vec::new(),
            look.title.clone(),
            look.status.clone(),
        )
    }

    /// `false`: a left click activates, a right click opens the menu.
    #[zbus(property)]
    fn item_is_menu(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn menu(&self) -> OwnedObjectPath {
        OwnedObjectPath::try_from(MENU_PATH).expect("valid object path")
    }

    #[zbus(signal)]
    async fn new_icon(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn new_tool_tip(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn new_title(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.kde.StatusNotifierWatcher",
    default_service = "org.kde.StatusNotifierWatcher",
    default_path = "/StatusNotifierWatcher"
)]
trait Watcher {
    fn register_status_notifier_item(&self, service: &str) -> zbus::Result<()>;
}

/// Numbers the items of this process.
static NEXT_ITEM: AtomicU32 = AtomicU32::new(1);

/// A tray icon on the session bus. Dropping it leaves the icon up until
/// the connection closes; [`Tray::hide`] takes it down.
pub struct Tray {
    connection: Connection,
    name: String,
    look: Arc<Mutex<Look>>,
    menu: Menu,
    _watch: Task<()>,
}

impl Tray {
    /// Shows the app's icon in `style` with `menu` on right click. `id`
    /// names the app to the panel and is its icon's name: the icon theme's
    /// icon while there is no badge, the icon drawn in code with one.
    /// Clicks call `handler`.
    pub async fn show(
        connection: &Connection,
        id: &str,
        title: &str,
        style: Style,
        menu: Vec<MenuItem>,
        handler: impl Fn(&str, Option<String>) + Send + Sync + 'static,
    ) -> zbus::Result<Self> {
        let handler: Handler = Arc::new(handler);
        let look = Arc::new(Mutex::new(Look::new(
            title.to_owned(),
            id.to_owned(),
            style,
        )));
        let object = ItemObject {
            id: id.to_owned(),
            look: look.clone(),
            token: Mutex::new(None),
            handler: handler.clone(),
        };
        let clicks = handler.clone();
        let menu = Menu::serve(connection, MENU_PATH, menu, move |action| {
            clicks(action, None)
        })
        .await?;
        connection.object_server().at(ITEM_PATH, object).await?;
        let name = format!(
            "org.kde.StatusNotifierItem-{}-{}",
            std::process::id(),
            NEXT_ITEM.fetch_add(1, Ordering::Relaxed)
        );
        connection
            .request_name_with_flags(name.as_str(), RequestNameFlags::DoNotQueue.into())
            .await?;
        let watch = connection.executor().spawn(
            keep_registered(connection.clone(), name.clone()),
            "katna tray registration",
        );
        Ok(Self {
            connection: connection.clone(),
            name,
            look,
            menu,
            _watch: watch,
        })
    }

    /// Shows `count` in a badge (none for 0) and `status` in the tooltip.
    pub async fn set_unread(&self, count: u64, status: &str) -> zbus::Result<()> {
        let badge = (count > 0).then(|| icon::badge_text(count));
        let (icon_changed, tooltip_changed) = {
            let mut look = self.look.lock().unwrap();
            let icon_changed = look.badge != badge;
            let tooltip_changed = look.status != status;
            if icon_changed {
                look.badge = badge;
                look.draw();
            }
            look.status = status.to_owned();
            (icon_changed, tooltip_changed)
        };
        let emitter = SignalEmitter::new(&self.connection, ITEM_PATH)?;
        if icon_changed {
            ItemObject::new_icon(&emitter).await?;
        }
        if tooltip_changed {
            ItemObject::new_tool_tip(&emitter).await?;
        }
        Ok(())
    }

    /// Draws the icon in `style` from now on.
    pub async fn set_style(&self, style: Style) -> zbus::Result<()> {
        {
            let mut look = self.look.lock().unwrap();
            if look.style == style {
                return Ok(());
            }
            look.style = style;
            look.draw();
        }
        ItemObject::new_icon(&SignalEmitter::new(&self.connection, ITEM_PATH)?).await
    }

    /// Replaces the right-click menu.
    pub async fn set_menu(&self, items: Vec<MenuItem>) -> zbus::Result<()> {
        self.menu.set_items(items).await
    }

    /// Takes the icon down.
    pub async fn hide(self) -> zbus::Result<()> {
        self.connection.release_name(self.name.as_str()).await?;
        let server = self.connection.object_server();
        server.remove::<ItemObject, _>(ITEM_PATH).await?;
        self.menu.remove().await
    }
}

/// Registers `name` with the watcher now and each time a watcher starts
/// (Plasma's panel restarting, the GNOME extension being turned on).
async fn keep_registered(connection: Connection, name: String) {
    let register = async || {
        let watcher = WatcherProxy::new(&connection).await?;
        watcher.register_status_notifier_item(&name).await
    };
    let owners = match DBusProxy::new(&connection).await {
        Ok(bus) => {
            let watcher = BusName::try_from(WATCHER).expect("valid bus name");
            bus.receive_name_owner_changed_with_args(&[(0, watcher.as_str())])
                .await
                .ok()
        }
        Err(_) => None,
    };
    if let Err(err) = register().await {
        tracing::debug!(%err, "no tray yet; waiting for one");
    }
    let Some(mut owners) = owners else {
        return;
    };
    while let Some(changed) = owners.next().await {
        let has_owner = changed
            .args()
            .map(|args| args.new_owner().is_some())
            .unwrap_or(false);
        if has_owner && let Err(err) = register().await {
            tracing::warn!(%err, "tray registration failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_every_size() {
        let mut look = Look::new(
            "Katna Mail".into(),
            "in.invenia.katna.Mail".into(),
            Style::Color,
        );
        assert_eq!(look.pixmaps.len(), SIZES.len());
        for (w, h, pixels) in &look.pixmaps {
            assert_eq!(w, h);
            assert_eq!(pixels.len(), (w * h * 4) as usize);
        }
        let plain = look.pixmaps.clone();
        look.badge = Some("3".into());
        look.draw();
        assert_ne!(plain, look.pixmaps);
    }

    #[test]
    fn the_icon_name_follows_the_style() {
        let mut look = Look::new(
            "Katna Mail".into(),
            "in.invenia.katna.Mail".into(),
            Style::Color,
        );
        assert_eq!(look.icon_name(), "in.invenia.katna.Mail");
        look.style = Style::Mono(0xffffff);
        assert_eq!(look.icon_name(), "in.invenia.katna.Mail-symbolic");
    }

    #[test]
    fn watcher_names_are_valid() {
        assert!(BusName::try_from(WATCHER).is_ok());
        assert!(OwnedObjectPath::try_from(MENU_PATH).is_ok());
    }
}
