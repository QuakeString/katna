// SPDX-License-Identifier: GPL-3.0-or-later

//! Appends many small messages to a fresh folder on the local Dovecot
//! (STARTTLS) to reproduce intermittent APPEND failures.

use std::{env, time::Instant};

use s2_pimalaya_io::{Credentials, Endpoint, MailBackend, Security, imap::ImapBackend, net::Tls};

fn main() {
    env_logger::init();
    let count: u32 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(5000);
    let endpoint = Endpoint {
        host: "127.0.0.1".into(),
        port: env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(20143),
        security: Security::StartTls,
    };
    let creds = Credentials {
        user: "alice@katna.test".into(),
        password: "katna-dev".into(),
    };
    futures_lite::future::block_on(async {
        let mut imap = ImapBackend::connect(&endpoint, &creds, Tls::insecure_for_local_tests())
            .await
            .unwrap();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let folder = format!("Stress {nanos}");
        imap.create_folder(&folder).await.unwrap();
        let t = Instant::now();
        for i in 0..count {
            let message = format!("Subject: stress {i}\r\n\r\nbody {i}\r\n").into_bytes();
            if let Err(e) = imap.append(&folder, message).await {
                println!("append {i} failed after {:?}: {e}", t.elapsed());
                std::process::exit(1);
            }
        }
        println!("{count} appends ok in {:?}", t.elapsed());
    });
}
