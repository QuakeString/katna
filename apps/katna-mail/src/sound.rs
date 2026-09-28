// SPDX-License-Identifier: GPL-3.0-or-later

//! The short sound when a message has gone out: "message-sent-email" of the
//! desktop's sound theme (KDE's Ocean, or the freedesktop one), played by
//! the desktop's own player. Nothing plays where none is found.

/// Plays the message-sent sound, without waiting for it.
pub fn sent() {
    #[cfg(target_os = "linux")]
    linux::play("message-sent-email");
}

#[cfg(target_os = "linux")]
mod linux {
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    /// Sound themes looked in, the desktop's usual one first.
    const THEMES: [&str; 2] = ["ocean", "freedesktop"];
    const EXTENSIONS: [&str; 3] = ["oga", "ogg", "wav"];

    pub(super) fn play(name: &str) {
        // libcanberra finds the sound in the desktop's theme itself.
        if spawn(Command::new("canberra-gtk-play").args(["-i", name])) {
            return;
        }
        let Some(file) = find(name) else {
            tracing::debug!(name, "no sound file found");
            return;
        };
        for player in ["pw-play", "paplay"] {
            if spawn(Command::new(player).arg(&file)) {
                return;
            }
        }
    }

    /// Starts `command` quietly and reaps it once it ends. Tells whether
    /// it started.
    fn spawn(command: &mut Command) -> bool {
        let child = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        match child {
            Ok(mut child) => {
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
                true
            }
            Err(_) => false,
        }
    }

    /// The file of sound `name` in the XDG data folders.
    fn find(name: &str) -> Option<PathBuf> {
        let home = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")));
        let dirs = std::env::var("XDG_DATA_DIRS")
            .ok()
            .filter(|d| !d.is_empty())
            .unwrap_or_else(|| "/usr/local/share:/usr/share".to_owned());
        let roots: Vec<PathBuf> = home
            .into_iter()
            .chain(dirs.split(':').map(PathBuf::from))
            .collect();
        THEMES.iter().find_map(|theme| {
            roots.iter().find_map(|root| {
                EXTENSIONS.iter().find_map(|ext| {
                    let path = root
                        .join("sounds")
                        .join(theme)
                        .join("stereo")
                        .join(format!("{name}.{ext}"));
                    path.is_file().then_some(path)
                })
            })
        })
    }
}
