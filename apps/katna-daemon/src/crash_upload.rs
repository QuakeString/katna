// SPDX-License-Identifier: GPL-3.0-or-later

//! Sends new crash reports to Katna's crash tracker, only while the user
//! has chosen to (Settings > User feedback, or "Help improve Katna" on the
//! first start; `docs/ARCHITECTURE.md` §19.2). Nothing is sent on a
//! metered connection. What goes is the saved report, scrubbed once more,
//! as a Sentry envelope ([`katna_core::sentry`]).

use std::sync::Weak;
use std::time::{Duration, SystemTime};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::crash::{self, Scrubber};
use katna_core::sentry::{self, Dsn};
use katna_sync::autoconfig::http;
use katna_sync::net::Tls;

use crate::daemon::{Daemon, settings};

/// The programs whose core dumps become reports before sending.
const APPS: [&str; 2] = ["katna-mail", "katna-daemon"];

/// The first look waits a little, so starting stays quick.
const FIRST_LOOK: Duration = Duration::from_secs(20);
/// How often new reports are looked for, besides when settings change.
const EVERY: Duration = Duration::from_secs(15 * 60);
/// Reports sent at most per look.
const MAX_PER_LOOK: usize = 5;
const TIMEOUT: Duration = Duration::from_secs(30);

/// Looks for reports to send now and then, and when `wake` says the
/// settings changed, until the daemon is gone.
pub(crate) async fn run(daemon: Weak<Daemon>, wake: Receiver<()>) {
    let mut pause = FIRST_LOOK;
    loop {
        let woken = async {
            let _ = wake.recv().await;
        };
        woken
            .or(async {
                async_io::Timer::after(pause).await;
            })
            .await;
        pause = EVERY;
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.closing() {
            return;
        }
        send_new(&daemon).await;
    }
}

async fn send_new(daemon: &Daemon) {
    let paths = daemon.paths().clone();
    let feedback = settings(&paths).feedback;
    if !feedback.sending() {
        return;
    }
    let Some(dsn) = Dsn::parse(feedback.dsn()) else {
        tracing::debug!("no crash tracker set; not sending crash reports");
        return;
    };
    if daemon.metered() {
        tracing::debug!("metered connection; crash reports wait");
        return;
    }
    let dir = paths.crash_dir();
    let unsent = smol::unblock(move || {
        // A crash of this daemon, or of Katna Mail not started since.
        crash::collect_core_dumps(&paths, &APPS);
        crash::unsent(&paths.crash_dir(), SystemTime::now())
    })
    .await;
    if unsent.is_empty() {
        return;
    }
    let tls = match Tls::system() {
        Ok(tls) => tls,
        Err(err) => {
            tracing::warn!(%err, "TLS setup failed; crash reports wait");
            return;
        }
    };
    let scrubber = Scrubber::from_system();
    let auth = dsn.auth_header();
    let headers = [
        ("Content-Type", sentry::CONTENT_TYPE),
        ("X-Sentry-Auth", auth.as_str()),
    ];
    for report in unsent.into_iter().take(MAX_PER_LOOK) {
        let Ok(text) = report.read() else {
            continue;
        };
        // Scrubbed again, in case the file was changed by hand.
        let text = scrubber.scrub(&text);
        let body = sentry::envelope(&dsn, &report, &text, SystemTime::now());
        let sent = match http::post(dsn.envelope_url(), &headers, &body, &tls, TIMEOUT).await {
            Ok(200..=299) => {
                tracing::info!(report = report.name, "crash report sent");
                true
            }
            Ok(429) => {
                tracing::info!("the crash tracker asks to wait; trying later");
                false
            }
            Ok(status @ 400..=499) => {
                // Sending it again would get the same answer.
                tracing::warn!(
                    status,
                    report = report.name,
                    "the crash tracker refused a report"
                );
                true
            }
            Ok(status) => {
                tracing::info!(status, "the crash tracker did not answer; trying later");
                false
            }
            Err(err) => {
                tracing::info!(%err, "could not reach the crash tracker; trying later");
                false
            }
        };
        if !sent {
            return;
        }
        if let Err(err) = crash::mark_sent(&dir, &report) {
            tracing::warn!(%err, "could not remember a crash report as sent");
            return;
        }
    }
}
