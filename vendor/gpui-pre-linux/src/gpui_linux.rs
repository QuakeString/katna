#![cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

pub use linux::{
    Placement, compositor_blur, current_platform, placement_session, restore_placement,
    set_client_corner_radius, set_kde_appmenu,
};
