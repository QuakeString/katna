#![cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

pub use linux::{compositor_blur, current_platform, set_client_corner_radius, set_kde_appmenu};
