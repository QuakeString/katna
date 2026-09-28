// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna's own session bus on Windows, end to end: the first client starts
//! `dbus-daemon.exe`, the next one finds it, and a call to the daemon's bus
//! name starts `katna-daemon.exe`, which answers.
//!
//! Ignored by default: it needs `dbus-daemon.exe` (with its DLLs) and
//! `katna-daemon.exe` in the folder of this test program, and `APPDATA` and
//! `LOCALAPPDATA` pointing at empty folders, as CI's Windows job sets up.

#![cfg(windows)]

use katna_dbus::PimProxy;

#[test]
#[ignore = "needs dbus-daemon.exe and katna-daemon.exe beside the test"]
fn clients_share_one_bus_that_starts_the_daemon() {
    let local = std::path::PathBuf::from(std::env::var_os("LOCALAPPDATA").unwrap());
    futures_lite::future::block_on(async {
        let first = katna_dbus::session().await.expect("starting the bus");
        let second = katna_dbus::session().await.expect("finding the bus");
        let address = std::fs::read_to_string(local.join(r"Katna\State\bus\address")).unwrap();
        assert!(address.starts_with("nonce-tcp:"), "{address}");
        assert_ne!(first.unique_name(), second.unique_name());
        let pim = PimProxy::new(&second).await.unwrap();
        let accounts = pim.accounts().await.expect("the daemon answers");
        assert!(accounts.is_empty());
    });
}
