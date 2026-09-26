// SPDX-License-Identifier: GPL-3.0-or-later

//! System events (plan task 1.12, `docs/ARCHITECTURE.md` §6.1): waking
//! from suspend (logind's `PrepareForSleep(false)`), the network coming
//! up (NetworkManager's `StateChanged` to at least "connected, local") and
//! the network becoming metered or not (NetworkManager's `Metered`).
//!
//! After a resume or a new network, a mail connection may be dead without
//! the socket knowing, and an IDLE on it would sit until it times out. The
//! daemon has every worker reconnect at once instead
//! ([`crate::daemon::Daemon::network_changed`]). On a metered network the
//! workers download no bodies ahead of time
//! ([`crate::daemon::Daemon::set_metered`]). Without logind or
//! NetworkManager, that part simply stays quiet.

use futures_lite::{FutureExt, StreamExt};
use zbus::{fdo::PropertiesProxy, proxy::CacheProperties};

#[zbus::proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait Login1Manager {
    #[zbus(signal)]
    fn prepare_for_sleep(&self, start: bool) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    #[zbus(signal)]
    fn state_changed(&self, state: u32) -> zbus::Result<()>;

    #[zbus(property)]
    fn metered(&self) -> zbus::Result<u32>;
}

const NM_SERVICE: &str = "org.freedesktop.NetworkManager";
const NM_PATH: &str = "/org/freedesktop/NetworkManager";

/// NetworkManager's `NM_STATE_CONNECTED_LOCAL`: from here on, servers on
/// the local network may be reachable.
const NM_CONNECTED_LOCAL: u32 = 50;

/// Whether NetworkManager's `NMMetered` value means metered: "yes" (1) or
/// "guess yes" (3), which covers phone hotspots.
fn is_metered(value: u32) -> bool {
    matches!(value, 1 | 3)
}

/// Something that makes existing connections suspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemEvent {
    /// The machine woke up from suspend or hibernation.
    Resumed,
    /// The network is up, or better connected than before.
    NetworkUp,
    /// The network became metered (`true`) or stopped being metered.
    Metered(bool),
}

enum Signal {
    Sleep(PrepareForSleep),
    State(StateChanged),
    Properties(zbus::fdo::PropertiesChanged),
}

/// Watches the system bus `system` and calls `on_event` until the
/// connection closes. Starts with [`SystemEvent::Metered`] if the network
/// already is.
pub async fn watch(system: zbus::Connection, on_event: impl Fn(SystemEvent)) -> zbus::Result<()> {
    let login = Login1ManagerProxy::builder(&system)
        .cache_properties(CacheProperties::No)
        .build()
        .await?;
    let network = NetworkManagerProxy::builder(&system)
        .cache_properties(CacheProperties::No)
        .build()
        .await?;
    let properties = PropertiesProxy::builder(&system)
        .destination(NM_SERVICE)?
        .path(NM_PATH)?
        .cache_properties(CacheProperties::No)
        .build()
        .await?;
    let mut sleep = login.receive_prepare_for_sleep().await?;
    let mut state = network.receive_state_changed().await?;
    let mut changed = properties.receive_properties_changed().await?;
    let mut last_state: Option<u32> = None;
    // Fails without NetworkManager: then nothing is metered.
    let mut metered = network.metered().await.is_ok_and(is_metered);
    if metered {
        on_event(SystemEvent::Metered(true));
    }
    loop {
        let signal = async { sleep.next().await.map(Signal::Sleep) }
            .or(async { state.next().await.map(Signal::State) })
            .or(async { changed.next().await.map(Signal::Properties) })
            .await;
        match signal {
            None => return Ok(()),
            Some(Signal::Sleep(signal)) => {
                if signal.args().is_ok_and(|args| !args.start) {
                    on_event(SystemEvent::Resumed);
                }
            }
            Some(Signal::State(signal)) => {
                let Ok(args) = signal.args() else { continue };
                let new = args.state;
                let up = new >= NM_CONNECTED_LOCAL && last_state.is_none_or(|old| new > old);
                last_state = Some(new);
                if up {
                    on_event(SystemEvent::NetworkUp);
                }
            }
            Some(Signal::Properties(signal)) => {
                let Ok(args) = signal.args() else { continue };
                if args.interface_name.as_str() != NM_SERVICE {
                    continue;
                }
                let Some(Ok(value)) = args
                    .changed_properties
                    .get("Metered")
                    .map(|value| value.downcast_ref::<u32>())
                else {
                    continue;
                };
                if is_metered(value) != metered {
                    metered = !metered;
                    on_event(SystemEvent::Metered(metered));
                }
            }
        }
    }
}
