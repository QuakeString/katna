// SPDX-License-Identifier: GPL-3.0-or-later

//! Menus served over D-Bus with `com.canonical.dbusmenu`
//! (`docs/ARCHITECTURE.md` §15.2): the tray icon's right-click menu and the
//! KDE global menu both use it. Plasma, the GNOME AppIndicator extension and
//! other panels read the menu from the bus and send clicks back.
//!
//! Each clickable item carries an action name; a click calls the handler
//! with it. No GPUI types here.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Serialize, Serializer};
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{OwnedValue, Signature, Type, Value};
use zbus::{Connection, fdo};

/// One entry of a menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuItem {
    /// A clickable item.
    Action {
        /// The text; `_` marks the mnemonic, as in `_New Message`.
        label: String,
        /// Passed to the click handler.
        action: String,
        /// Shown next to the label: modifiers then the key, such as
        /// `["Control", "N"]`. Empty for none.
        shortcut: Vec<String>,
        /// A freedesktop icon name.
        icon: Option<String>,
        enabled: bool,
    },
    /// An item that opens a submenu.
    Submenu {
        label: String,
        items: Vec<MenuItem>,
    },
    Separator,
}

impl MenuItem {
    /// A clickable item that calls the handler with `action`.
    pub fn action(label: impl Into<String>, action: impl Into<String>) -> Self {
        Self::Action {
            label: label.into(),
            action: action.into(),
            shortcut: Vec::new(),
            icon: None,
            enabled: true,
        }
    }

    pub fn submenu(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self::Submenu {
            label: label.into(),
            items,
        }
    }

    /// Shows `keys` (modifiers, then the key) as the item's shortcut.
    pub fn shortcut(mut self, keys: Vec<String>) -> Self {
        if let Self::Action { shortcut, .. } = &mut self {
            *shortcut = keys;
        }
        self
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        if let Self::Action { icon, .. } = &mut self {
            *icon = Some(name.into());
        }
        self
    }
}

/// One item as the protocol numbers it. Item 0 is the root.
#[derive(Debug, Clone)]
struct Node {
    item: MenuItem,
    children: Vec<i32>,
}

/// The menu and its revision, which grows with every change.
struct State {
    revision: u32,
    nodes: Vec<Node>,
}

impl State {
    fn new(items: Vec<MenuItem>) -> Self {
        let mut state = Self {
            revision: 1,
            nodes: Vec::new(),
        };
        state.replace(items);
        state
    }

    fn replace(&mut self, items: Vec<MenuItem>) {
        self.nodes.clear();
        let root = MenuItem::submenu("", items);
        add(&mut self.nodes, root);
    }

    fn node(&self, id: i32) -> Option<&Node> {
        usize::try_from(id).ok().and_then(|i| self.nodes.get(i))
    }

    fn layout(&self, id: i32, depth: i32, names: &[String]) -> Option<Layout> {
        let node = self.node(id)?;
        let children = if depth == 0 {
            Vec::new()
        } else {
            node.children
                .iter()
                .filter_map(|&child| self.layout(child, depth - 1, names))
                .map(Child)
                .collect()
        };
        Some(Layout {
            id,
            properties: properties(node, names),
            children,
        })
    }
}

/// Numbers `item` and its children depth first; returns its ID.
fn add(nodes: &mut Vec<Node>, item: MenuItem) -> i32 {
    let id = nodes.len();
    let children = match &item {
        MenuItem::Submenu { items, .. } => items.clone(),
        _ => Vec::new(),
    };
    nodes.push(Node {
        item,
        children: Vec::new(),
    });
    let ids = children
        .into_iter()
        .map(|child| add(nodes, child))
        .collect();
    nodes[id].children = ids;
    i32::try_from(id).unwrap_or(i32::MAX)
}

/// The item's properties that differ from the protocol's defaults, only
/// those in `names` unless it is empty.
fn properties(node: &Node, names: &[String]) -> HashMap<&'static str, OwnedValue> {
    let mut props: Vec<(&'static str, Value<'static>)> = Vec::new();
    match &node.item {
        MenuItem::Action {
            label,
            shortcut,
            icon,
            enabled,
            ..
        } => {
            props.push(("label", Value::from(label.clone())));
            if !shortcut.is_empty() {
                props.push(("shortcut", Value::from(vec![shortcut.clone()])));
            }
            if let Some(icon) = icon {
                props.push(("icon-name", Value::from(icon.clone())));
            }
            if !enabled {
                props.push(("enabled", Value::from(false)));
            }
        }
        MenuItem::Submenu { label, .. } => {
            if !label.is_empty() {
                props.push(("label", Value::from(label.clone())));
            }
            props.push(("children-display", Value::from("submenu")));
        }
        MenuItem::Separator => props.push(("type", Value::from("separator"))),
    }
    props
        .into_iter()
        .filter(|(name, _)| names.is_empty() || names.iter().any(|n| n == name))
        .filter_map(|(name, value)| Some((name, OwnedValue::try_from(value).ok()?)))
        .collect()
}

/// `(ia{sv}av)`: an item, its properties and its children, each child a
/// variant holding the same structure.
#[derive(Debug, Serialize, Type)]
struct Layout {
    id: i32,
    properties: HashMap<&'static str, OwnedValue>,
    children: Vec<Child>,
}

/// A child layout, sent as a variant.
#[derive(Debug)]
struct Child(Layout);

impl Type for Child {
    const SIGNATURE: &'static Signature = &Signature::Variant;
}

impl Serialize for Child {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        zbus::zvariant::as_value::serialize(&self.0, serializer)
    }
}

type Handler = Arc<dyn Fn(&str) + Send + Sync>;

/// The `com.canonical.dbusmenu` object.
struct MenuObject {
    state: Arc<Mutex<State>>,
    on_click: Handler,
}

#[zbus::interface(name = "com.canonical.dbusmenu")]
impl MenuObject {
    #[zbus(out_args("revision", "layout"))]
    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: Vec<String>,
    ) -> fdo::Result<(u32, Layout)> {
        let state = self.state.lock().unwrap();
        let layout = state
            .layout(parent_id, recursion_depth, &property_names)
            .ok_or_else(|| fdo::Error::InvalidArgs(format!("no menu item {parent_id}")))?;
        Ok((state.revision, layout))
    }

    fn get_group_properties(
        &self,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> Vec<(i32, HashMap<&'static str, OwnedValue>)> {
        let state = self.state.lock().unwrap();
        let every: Vec<i32> = (0..state.nodes.len())
            .filter_map(|i| i32::try_from(i).ok())
            .collect();
        let ids = if ids.is_empty() { every } else { ids };
        ids.into_iter()
            .filter_map(|id| Some((id, properties(state.node(id)?, &property_names))))
            .collect()
    }

    fn get_property(&self, id: i32, name: String) -> fdo::Result<OwnedValue> {
        let state = self.state.lock().unwrap();
        let node = state
            .node(id)
            .ok_or_else(|| fdo::Error::InvalidArgs(format!("no menu item {id}")))?;
        properties(node, std::slice::from_ref(&name))
            .remove(name.as_str())
            .ok_or_else(|| fdo::Error::InvalidArgs(format!("no property {name}")))
    }

    fn event(&self, id: i32, event_id: String, _data: OwnedValue, _timestamp: u32) {
        self.handle(id, &event_id);
    }

    fn event_group(&self, events: Vec<(i32, String, OwnedValue, u32)>) -> Vec<i32> {
        let mut unknown = Vec::new();
        for (id, event_id, _, _) in events {
            if !self.handle(id, &event_id) {
                unknown.push(id);
            }
        }
        unknown
    }

    fn about_to_show(&self, _id: i32) -> bool {
        false
    }

    #[zbus(out_args("updates_needed", "id_errors"))]
    fn about_to_show_group(&self, _ids: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        (Vec::new(), Vec::new())
    }

    #[zbus(signal)]
    async fn layout_updated(
        emitter: &SignalEmitter<'_>,
        revision: u32,
        parent: i32,
    ) -> zbus::Result<()>;

    #[zbus(property)]
    fn version(&self) -> u32 {
        3
    }

    #[zbus(property)]
    fn text_direction(&self) -> &str {
        "ltr"
    }

    #[zbus(property)]
    fn status(&self) -> &str {
        "normal"
    }

    #[zbus(property)]
    fn icon_theme_path(&self) -> Vec<String> {
        Vec::new()
    }
}

impl MenuObject {
    /// Runs the handler for a click on `id`; `false` if there is no such item.
    fn handle(&self, id: i32, event: &str) -> bool {
        let action = {
            let state = self.state.lock().unwrap();
            match state.node(id).map(|node| &node.item) {
                Some(MenuItem::Action {
                    action, enabled, ..
                }) => enabled.then(|| action.clone()),
                Some(_) => None,
                None => return false,
            }
        };
        if event == "clicked"
            && let Some(action) = action
        {
            (self.on_click)(&action);
        }
        true
    }
}

/// A menu served on a connection.
#[derive(Clone)]
pub struct Menu {
    connection: Connection,
    path: String,
    state: Arc<Mutex<State>>,
}

impl Menu {
    /// Serves `items` at `path` on `connection`. A click on an item calls
    /// `on_click` with its action, on the connection's executor.
    pub async fn serve(
        connection: &Connection,
        path: &str,
        items: Vec<MenuItem>,
        on_click: impl Fn(&str) + Send + Sync + 'static,
    ) -> zbus::Result<Self> {
        let state = Arc::new(Mutex::new(State::new(items)));
        let object = MenuObject {
            state: state.clone(),
            on_click: Arc::new(on_click),
        };
        connection.object_server().at(path, object).await?;
        Ok(Self {
            connection: connection.clone(),
            path: path.to_owned(),
            state,
        })
    }

    /// Where the menu is.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Replaces the items, if they changed, and tells the panels.
    pub async fn set_items(&self, items: Vec<MenuItem>) -> zbus::Result<()> {
        let revision = {
            let mut state = self.state.lock().unwrap();
            let unchanged = state.node(0).is_some_and(
                |root| matches!(&root.item, MenuItem::Submenu { items: old, .. } if *old == items),
            );
            if unchanged {
                return Ok(());
            }
            state.replace(items);
            state.revision += 1;
            state.revision
        };
        let emitter = SignalEmitter::new(&self.connection, self.path.as_str())?;
        MenuObject::layout_updated(&emitter, revision, 0).await
    }

    /// Stops serving the menu.
    pub async fn remove(self) -> zbus::Result<()> {
        self.connection
            .object_server()
            .remove::<MenuObject, _>(self.path.as_str())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::zvariant::{LE, serialized::Context, to_bytes};

    fn menu() -> Vec<MenuItem> {
        vec![
            MenuItem::action("_Open Inbox", "open-inbox"),
            MenuItem::Separator,
            MenuItem::submenu(
                "_File",
                vec![
                    MenuItem::action("_Quit", "quit").shortcut(vec!["Control".into(), "Q".into()]),
                ],
            ),
        ]
    }

    #[test]
    fn numbers_items_depth_first() {
        let state = State::new(menu());
        let labels: Vec<Option<&str>> = state
            .nodes
            .iter()
            .map(|node| match &node.item {
                MenuItem::Action { label, .. } | MenuItem::Submenu { label, .. } => {
                    Some(label.as_str())
                }
                MenuItem::Separator => None,
            })
            .collect();
        assert_eq!(
            labels,
            [
                Some(""),
                Some("_Open Inbox"),
                None,
                Some("_File"),
                Some("_Quit")
            ]
        );
        assert_eq!(state.nodes[0].children, [1, 2, 3]);
        assert_eq!(state.nodes[3].children, [4]);
    }

    #[test]
    fn layout_respects_depth_and_property_names() {
        let state = State::new(menu());
        let all = state.layout(0, -1, &[]).unwrap();
        assert_eq!(all.children.len(), 3);
        assert_eq!(all.children[2].0.children.len(), 1);
        let shallow = state.layout(0, 1, &[]).unwrap();
        assert!(shallow.children[2].0.children.is_empty());
        let labels = state.layout(4, 0, &["label".to_owned()]).unwrap();
        assert_eq!(labels.properties.len(), 1);
        let quit = state.layout(4, 0, &[]).unwrap();
        assert!(quit.properties.contains_key("shortcut"));
        assert!(state.layout(99, 0, &[]).is_none());
    }

    #[test]
    fn layout_has_the_dbusmenu_signature() {
        assert_eq!(Layout::SIGNATURE.to_string(), "(ia{sv}av)");
        let state = State::new(menu());
        let layout = state.layout(0, -1, &[]).unwrap();
        let bytes = to_bytes(Context::new_dbus(LE, 0), &(1u32, layout)).unwrap();
        assert!(bytes.len() > 100);
    }

    #[test]
    fn clicks_run_the_action_of_enabled_items_only() {
        let clicked = Arc::new(Mutex::new(Vec::new()));
        let seen = clicked.clone();
        let mut items = menu();
        items.push(MenuItem::Action {
            label: "Off".into(),
            action: "off".into(),
            shortcut: Vec::new(),
            icon: None,
            enabled: false,
        });
        let object = MenuObject {
            state: Arc::new(Mutex::new(State::new(items))),
            on_click: Arc::new(move |action: &str| seen.lock().unwrap().push(action.to_owned())),
        };
        assert!(object.handle(4, "clicked"));
        assert!(object.handle(1, "hovered"));
        assert!(object.handle(5, "clicked"));
        assert!(!object.handle(42, "clicked"));
        assert_eq!(*clicked.lock().unwrap(), ["quit"]);
    }
}
