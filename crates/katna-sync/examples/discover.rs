// SPDX-License-Identifier: GPL-3.0-or-later

//! Prints the servers found for an address, without the daemon:
//! `cargo run -p katna-sync --example discover -- ada@example.org`.

use katna_sync::autoconfig::Discovery;

fn main() {
    let Some(address) = std::env::args().nth(1) else {
        eprintln!("usage: discover ADDRESS");
        std::process::exit(2);
    };
    let discovery = Discovery::system().expect("TLS setup");
    match smol::block_on(discovery.discover(&address)) {
        Ok(found) => {
            println!("source: {}", found.source.as_str());
            println!("imap: {:?}", found.imap);
            println!("smtp: {:?}", found.smtp);
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
