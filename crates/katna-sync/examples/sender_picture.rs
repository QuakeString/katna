// SPDX-License-Identifier: GPL-3.0-or-later

//! Looks up sender pictures as the daemon does, with an empty cache, and
//! prints where each came from:
//! `cargo run -p katna-sync --example sender_picture -- info@example.org …`.
//! Touches no mail account; only DNS and the senders' websites.

use katna_core::image::ImageKind;
use katna_sync::pictures::Pictures;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("katna_sync=debug")
        .with_target(false)
        .without_time()
        .init();
    let addresses: Vec<String> = std::env::args().skip(1).collect();
    if addresses.is_empty() {
        eprintln!("usage: sender_picture ADDRESS...");
        std::process::exit(2);
    }
    let cache = tempfile::tempdir().expect("temporary directory");
    let pictures = Pictures::system(cache.path()).expect("TLS setup");
    for address in addresses {
        let started = std::time::Instant::now();
        let picture = smol::block_on(pictures.sender(&address));
        let kind = ImageKind::sniff(&picture);
        println!(
            "{address}: {} bytes, {kind:?}, {:.1} s",
            picture.len(),
            started.elapsed().as_secs_f32()
        );
    }
}
