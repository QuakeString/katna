// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna from the Microsoft Store, an MSIX package
//! (`packaging/windows/store`, [`katna_core::update::Package::MsStore`]).
//! Windows keeps a packaged program's registry writes to itself, so the
//! `Run` key cannot start it at sign-in: the package's startup task does,
//! which Settings > Apps > Startup also shows and turns off.

use windows::ApplicationModel::Activation::ActivationKind;
use windows::ApplicationModel::{AppInstance, StartupTask, StartupTaskState};
use windows::core::HSTRING;

/// The startup task's `TaskId` in the package's manifest
/// (`packaging/windows/store/AppxManifest.xml`).
const STARTUP_TASK: &str = "KatnaStartup";

fn task() -> windows::core::Result<StartupTask> {
    StartupTask::GetAsync(&HSTRING::from(STARTUP_TASK))?.join()
}

/// Whether Windows starts Katna Mail at sign-in.
pub fn starts_at_sign_in() -> bool {
    task().and_then(|task| task.State()).is_ok_and(|state| {
        state == StartupTaskState::Enabled || state == StartupTaskState::EnabledByPolicy
    })
}

/// What turning the startup task on or off did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// It is as asked.
    Done,
    /// The user turned it off in Windows' Settings, and only they can turn
    /// it on again there.
    OffInWindows,
}

/// Makes Windows start Katna Mail at sign-in, or not.
pub fn set_starts_at_sign_in(on: bool) -> windows::core::Result<Outcome> {
    let task = task()?;
    if !on {
        task.Disable()?;
        return Ok(Outcome::Done);
    }
    let state = task.RequestEnableAsync()?.join()?;
    Ok(match state {
        StartupTaskState::Enabled | StartupTaskState::EnabledByPolicy => Outcome::Done,
        _ => Outcome::OffInWindows,
    })
}

/// Whether Windows started this Katna Mail at sign-in, through the startup
/// task (which cannot pass `--background`).
pub fn started_at_sign_in() -> bool {
    AppInstance::GetActivatedEventArgs()
        .and_then(|args| args.Kind())
        .is_ok_and(|kind| kind == ActivationKind::StartupTask)
}
