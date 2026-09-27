#![cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

pub use linux::{
    DroppedContent, Placement, clipboard_html, compositor_blur, current_platform, dropped_content,
    html_item, placement_session, read_rich, restore_placement, set_client_corner_radius,
    set_kde_appmenu,
};
