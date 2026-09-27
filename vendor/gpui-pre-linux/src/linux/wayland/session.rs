//! xdg-session-management-v1 (added for Katna; not in upstream GPUI): the
//! compositor remembers where a window was and puts it back there when the
//! app opens it again (`crate::linux::placement`).
//!
//! `wayland-protocols` ships the protocol file without Rust bindings, so
//! they are generated here from a copy of it
//! (`protocols/xdg-session-management-v1.xml`).

use wayland_client::{Connection, Dispatch, QueueHandle};

use super::client::WaylandClientStatePtr;
use crate::linux::placement::set_placement_session;

#[allow(
    dead_code,
    non_camel_case_types,
    unused_unsafe,
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_imports,
    missing_docs,
    clippy::all
)]
pub(crate) mod protocol {
    use wayland_client;
    use wayland_client::protocol::*;
    use wayland_protocols::xdg::shell::client::*;

    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;
        use wayland_protocols::xdg::shell::client::__interfaces::*;
        wayland_scanner::generate_interfaces!("protocols/xdg-session-management-v1.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_client_code!("protocols/xdg-session-management-v1.xml");
}

pub(crate) use protocol::{xdg_session_manager_v1, xdg_session_v1, xdg_toplevel_session_v1};

/// A window's place in a session: dropped with the window, which leaves
/// the session's record of it with the compositor.
pub(crate) struct WindowSession {
    session: xdg_session_v1::XdgSessionV1,
    toplevel: xdg_toplevel_session_v1::XdgToplevelSessionV1,
}

impl WindowSession {
    /// Adds `toplevel` to a session before its first commit, as
    /// `placement` asks.
    pub(crate) fn start(
        manager: &xdg_session_manager_v1::XdgSessionManagerV1,
        toplevel: &wayland_protocols::xdg::shell::client::xdg_toplevel::XdgToplevel,
        placement: crate::linux::placement::Placement,
        qh: &QueueHandle<WaylandClientStatePtr>,
    ) -> Self {
        let reason = xdg_session_manager_v1::Reason::Launch;
        let session_id = if placement.restore {
            placement.session
        } else {
            // Starting afresh: forget the old session, then open a new one.
            if let Some(old) = placement.session {
                manager.get_session(reason, Some(old), qh, ()).remove();
            }
            None
        };
        // A restored session keeps its id; a new one gets its own in
        // `created`.
        if let Some(id) = &session_id {
            set_placement_session(id.clone());
        }
        let session = manager.get_session(reason, session_id, qh, ());
        let toplevel = if placement.restore {
            session.restore_toplevel(toplevel, placement.name, qh, ())
        } else {
            session.add_toplevel(toplevel, placement.name, qh, ())
        };
        Self { session, toplevel }
    }

    /// Ends the window's part in the session, before its toplevel goes.
    pub(crate) fn destroy(&self) {
        self.toplevel.destroy();
        self.session.destroy();
    }
}

impl Dispatch<xdg_session_manager_v1::XdgSessionManagerV1, ()> for WaylandClientStatePtr {
    fn event(
        _: &mut Self,
        _: &xdg_session_manager_v1::XdgSessionManagerV1,
        _: xdg_session_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<xdg_session_v1::XdgSessionV1, ()> for WaylandClientStatePtr {
    fn event(
        _: &mut Self,
        _: &xdg_session_v1::XdgSessionV1,
        event: xdg_session_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            xdg_session_v1::Event::Created { session_id } => {
                log::info!("new window session");
                set_placement_session(session_id);
            }
            xdg_session_v1::Event::Restored => log::info!("window session restored"),
            xdg_session_v1::Event::Replaced => log::info!("window session taken over"),
        }
    }
}

impl Dispatch<xdg_toplevel_session_v1::XdgToplevelSessionV1, ()> for WaylandClientStatePtr {
    fn event(
        _: &mut Self,
        _: &xdg_toplevel_session_v1::XdgToplevelSessionV1,
        _: xdg_toplevel_session_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}
