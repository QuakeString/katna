// SPDX-License-Identifier: GPL-3.0-or-later

//! Sends new crash reports and finished weeks of usage statistics to
//! Katna's crash tracker, each only while the user has chosen to
//! (Settings > User feedback, or "Help improve Katna" on the first start;
//! `docs/ARCHITECTURE.md` §19.2), and feedback written in Katna Mail when
//! it is sent. Nothing but feedback is sent on a metered connection. What
//! goes is the saved report, scrubbed once more, the week's report as
//! Settings shows it, or the feedback as the form showed it, each as a
//! Sentry envelope ([`katna_core::sentry`]).

use std::sync::Weak;
use std::time::{Duration, SystemTime};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::crash::{self, Scrubber};
use katna_core::sentry::{self, Dsn};
use katna_core::usage::{self, WeeklyReport};
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
        send_usage(&daemon).await;
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

/// The outcome of one post.
enum Posted {
    /// Taken, or refused for good: sending again would get the same answer.
    Done,
    /// Try again later: no network, a server error or "slow down".
    Later(String),
}

async fn post(dsn: &Dsn, body: &[u8]) -> Posted {
    let tls = match Tls::system() {
        Ok(tls) => tls,
        Err(err) => return Posted::Later(err.to_string()),
    };
    let auth = dsn.auth_header();
    let headers = [
        ("Content-Type", sentry::CONTENT_TYPE),
        ("X-Sentry-Auth", auth.as_str()),
    ];
    match http::post(dsn.envelope_url(), &headers, body, &tls, TIMEOUT).await {
        Ok(200..=299) => Posted::Done,
        Ok(429) => Posted::Later("the server asks to wait".into()),
        Ok(status @ 400..=499) => {
            tracing::warn!(status, "the crash tracker refused an envelope");
            Posted::Done
        }
        Ok(status) => Posted::Later(format!("HTTP {status}")),
        Err(err) => Posted::Later(err.to_string()),
    }
}

/// Sends the last finished week of usage statistics, once.
async fn send_usage(daemon: &Daemon) {
    let paths = daemon.paths().clone();
    let feedback = settings(&paths).feedback;
    if !feedback.send_usage_statistics || daemon.metered() {
        return;
    }
    let Some(dsn) = Dsn::parse(feedback.dsn()) else {
        return;
    };
    let now = SystemTime::now();
    let Some(week) = usage::due(&paths, now) else {
        return;
    };
    let accounts = daemon.accounts().map(|a| a.len()).unwrap_or(0);
    let (install_id, _) = usage::install_id(&paths, now);
    let report = WeeklyReport::build(&paths, week, accounts, install_id);
    match post(&dsn, &sentry::usage_envelope(&dsn, &report, now)).await {
        Posted::Done => {
            tracing::info!(week, "usage statistics sent");
            if let Err(err) = usage::mark_sent(&paths, week) {
                tracing::warn!(%err, "could not remember usage statistics as sent");
            }
        }
        Posted::Later(why) => tracing::info!(%why, "usage statistics wait"),
    }
}

/// Sends feedback from Katna Mail's form: `text` exactly as the form
/// showed it. Returns why it could not be sent.
pub(crate) async fn send_feedback(
    daemon: &Daemon,
    text: &str,
    kind: &str,
    reply_to: &str,
) -> Result<(), String> {
    let feedback = settings(daemon.paths()).feedback;
    let Some(dsn) = Dsn::parse(feedback.dsn()) else {
        return Err("no feedback address is set".into());
    };
    let body = sentry::feedback_envelope(&dsn, text, kind, reply_to, SystemTime::now());
    match post(&dsn, &body).await {
        Posted::Done => {
            tracing::info!("feedback sent");
            Ok(())
        }
        Posted::Later(why) => Err(why),
    }
}
