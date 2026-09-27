// SPDX-License-Identifier: GPL-3.0-or-later

//! How an app's main window was when it closed, so it opens the same way
//! next time (`docs/ARCHITECTURE.md` §13.1). The state belongs to one run
//! of the Katna service: after the service quits, the window opens at its
//! default size and place again.

use std::fs;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// The main window's size, place and maximized state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowState {
    /// The window's visible frame in logical pixels, without any shadow
    /// around it; its size before it was maximized, if it was.
    pub width: f32,
    pub height: f32,
    /// Where the window was on screen (X11 only: Wayland does not tell a
    /// window where it is).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<(f32, f32)>,
    #[serde(default)]
    pub maximized: bool,
    /// The Wayland session the compositor remembers the window in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// Which run of the Katna service the state belongs to (see
    /// [`service_run`]); `None` when the service was not running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl WindowState {
    /// The state saved at `path`, if there is a readable one.
    pub fn load(path: &Path) -> Option<Self> {
        let text = fs::read_to_string(path).ok()?;
        let state: Self = toml::from_str(&text).ok()?;
        (state.width.is_finite() && state.height.is_finite() && state.width > 0.0).then_some(state)
    }

    /// Writes the state to `path`, through a temporary file so a reader
    /// never sees half of it.
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let text = toml::to_string(self).map_err(std::io::Error::other)?;
        let dir = path.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(dir)?;
        let tmp = path.with_extension("toml.new");
        let mut file = fs::File::create(&tmp)?;
        file.write_all(text.as_bytes())?;
        file.sync_all()?;
        fs::rename(&tmp, path)
    }

    /// Whether the window goes back to this state now that the service's
    /// run is `current`: only within the run it was saved in.
    pub fn same_run(&self, current: Option<&str>) -> bool {
        self.service.as_deref() == current
    }
}

/// Names one run of the process `pid`: its id and start time, which stay
/// the same when it replaces its program (as the daemon does after an
/// update) and change when it quits and starts again.
pub fn service_run(pid: u32) -> Option<String> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    start_time(&stat).map(|start| format!("{pid}-{start}"))
}

/// Field 22 of `/proc/<pid>/stat`, the start time; the command name before
/// it is in parentheses and may hold spaces.
fn start_time(stat: &str) -> Option<&str> {
    let (_, after_name) = stat.rsplit_once(')')?;
    // Fields 3 onwards follow the name; the start time is the 20th of them.
    after_name.split_whitespace().nth(19)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_loads() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state/mail-window.toml");
        let state = WindowState {
            width: 1280.0,
            height: 800.0,
            position: Some((40.0, 60.0)),
            maximized: true,
            session: Some("abc".into()),
            service: Some("12-345".into()),
        };
        state.save(&path).unwrap();
        assert_eq!(WindowState::load(&path), Some(state));
    }

    #[test]
    fn a_broken_file_is_no_state() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mail-window.toml");
        assert_eq!(WindowState::load(&path), None);
        fs::write(&path, "width = \"wide\"").unwrap();
        assert_eq!(WindowState::load(&path), None);
        fs::write(&path, "width = 0.0\nheight = 10.0").unwrap();
        assert_eq!(WindowState::load(&path), None);
    }

    #[test]
    fn belongs_to_one_run() {
        let mut state = WindowState {
            width: 800.0,
            height: 600.0,
            position: None,
            maximized: false,
            session: None,
            service: Some("12-345".into()),
        };
        assert!(state.same_run(Some("12-345")));
        assert!(!state.same_run(Some("12-999")));
        assert!(!state.same_run(None));
        state.service = None;
        assert!(state.same_run(None));
    }

    #[test]
    fn reads_the_start_time() {
        let stat = "4242 (katna daemon) S 1 4242 4242 0 -1 4194560 1 0 0 0 3 1 0 0 \
                    20 0 5 0 987654 1234 56";
        assert_eq!(start_time(stat), Some("987654"));
        let own = service_run(std::process::id()).unwrap();
        assert!(own.starts_with(&format!("{}-", std::process::id())));
        assert_eq!(service_run(std::process::id()), Some(own));
    }
}
