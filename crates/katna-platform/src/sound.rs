// SPDX-License-Identifier: GPL-3.0-or-later

//! The sounds Katna plays (Settings > Notifications > Sounds): a short list
//! of the desktop's own sounds to pick from, each event's usual one, and
//! playing them.
//!
//! On Linux the sounds come from the desktop's sound theme (KDE's Ocean,
//! or the freedesktop one), found by their freedesktop names and played by
//! the desktop's own player; Katna plays them itself because notification
//! servers such as Plasma's leave a notification's `sound-name` unplayed.
//! Katna's own chime ([`CHIME`], made in [`chime`]) is new mail's usual
//! sound there: the themes' new-mail sounds can be too faint to hear.
//! On Windows they are the Windows notification sounds: a toast plays its
//! own (so Do not disturb silences it), and [`play`] plays the same file
//! for everything else.

use katna_core::config::SoundEvent;

#[cfg(not(windows))]
mod chime;

/// Katna's own new-mail chime, which no sound theme has.
#[cfg(not(windows))]
pub const CHIME: &str = "katna-chime";

/// One sound to pick: its name, stored in the settings, and on Linux the
/// theme sounds tried for it in turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Choice {
    pub id: &'static str,
    #[cfg(not(windows))]
    names: &'static [&'static str],
    #[cfg(windows)]
    file: &'static str,
}

#[cfg(not(windows))]
const CHOICES: &[Choice] = &[
    Choice {
        id: CHIME,
        names: &[],
    },
    Choice {
        id: "message-new-email",
        names: &["message-new-email", "message-new-instant", "message"],
    },
    Choice {
        id: "message-new-instant",
        names: &["message-new-instant", "message"],
    },
    Choice {
        id: "message-sent-email",
        names: &["message-sent-email", "message-sent-instant", "complete"],
    },
    Choice {
        id: "alarm-clock-elapsed",
        names: &["alarm-clock-elapsed", "bell"],
    },
    Choice {
        id: "bell",
        names: &["bell", "bell-window-system"],
    },
    Choice {
        id: "complete",
        names: &["complete", "outcome-success"],
    },
    Choice {
        id: "dialog-information",
        names: &["dialog-information"],
    },
    Choice {
        id: "dialog-warning",
        names: &["dialog-warning"],
    },
    Choice {
        id: "dialog-error",
        names: &["dialog-error", "outcome-failure", "dialog-warning"],
    },
];

/// The names of the toast sounds, which a notification's `sound-name`
/// carries to the toast, and their files in `%SystemRoot%\Media`.
#[cfg(windows)]
const CHOICES: &[Choice] = &[
    Choice {
        id: "Mail",
        file: "Windows Notify Email.wav",
    },
    Choice {
        id: "IM",
        file: "Windows Notify Messaging.wav",
    },
    Choice {
        id: "Default",
        file: "Windows Notify System Generic.wav",
    },
    Choice {
        id: "Reminder",
        file: "Windows Notify Calendar.wav",
    },
    Choice {
        id: "Alarm",
        file: "Alarm01.wav",
    },
    Choice {
        id: "Error",
        file: "Windows Background.wav",
    },
];

/// The usual sound of `event`.
pub fn usual(event: SoundEvent) -> &'static str {
    #[cfg(not(windows))]
    let id = match event {
        SoundEvent::NewMail => CHIME,
        SoundEvent::Reminders => "alarm-clock-elapsed",
        SoundEvent::MailBack => "bell",
        SoundEvent::Sent => "message-sent-email",
        SoundEvent::NotSent => "dialog-error",
    };
    #[cfg(windows)]
    let id = match event {
        SoundEvent::NewMail => "Mail",
        SoundEvent::Reminders => "Reminder",
        SoundEvent::MailBack => "Default",
        SoundEvent::Sent => "IM",
        SoundEvent::NotSent => "Error",
    };
    id
}

/// The sound `event` plays when its setting names `chosen`: that one, or
/// the usual one for an empty or unknown name.
pub fn resolve(event: SoundEvent, chosen: &str) -> &'static str {
    CHOICES
        .iter()
        .find(|c| c.id == chosen)
        .map_or_else(|| usual(event), |c| c.id)
}

/// The sounds to pick from: those this computer has, or all of them when
/// it has none of them.
pub fn choices() -> &'static [Choice] {
    #[cfg(target_os = "linux")]
    {
        use std::sync::OnceLock;
        static FOUND: OnceLock<Vec<Choice>> = OnceLock::new();
        let found = FOUND.get_or_init(|| {
            CHOICES
                .iter()
                .filter(|c| c.id == CHIME || linux::find(c.names).is_some())
                .copied()
                .collect()
        });
        if !found.is_empty() {
            return found;
        }
    }
    CHOICES
}

/// Plays sound `id` (a [`Choice::id`]), without waiting for it. Nothing
/// plays where it is not found.
pub fn play(id: &str) {
    let Some(choice) = CHOICES.iter().find(|c| c.id == id) else {
        tracing::debug!(id, "no such sound");
        return;
    };
    #[cfg(target_os = "linux")]
    if choice.id == CHIME {
        linux::play_file(chime_file());
    } else {
        linux::play(choice.names);
    }
    #[cfg(windows)]
    win::play(choice.file);
    #[cfg(not(any(target_os = "linux", windows)))]
    let _ = choice;
}

/// The chime's file in Katna's cache folder, written there the first time.
#[cfg(target_os = "linux")]
fn chime_file() -> Option<std::path::PathBuf> {
    use std::sync::OnceLock;
    static FILE: OnceLock<Option<std::path::PathBuf>> = OnceLock::new();
    FILE.get_or_init(|| {
        let dir = katna_core::Paths::from_env()
            .map(|p| p.cache_dir().join("sounds"))
            .unwrap_or_else(|_| std::env::temp_dir().join("katna-sounds"));
        // The name changes with the sound, so an old file is never played.
        let file = dir.join("katna-chime-1.wav");
        if file.is_file() {
            return Some(file);
        }
        let wav = chime::wav();
        let written = std::fs::create_dir_all(&dir).and_then(|()| {
            // Written aside first: the daemon and the app may both write it.
            let part = dir.join(format!("katna-chime-1.{}.part", std::process::id()));
            std::fs::write(&part, &wav)?;
            std::fs::rename(&part, &file)
        });
        match written {
            Ok(()) => Some(file),
            Err(err) => {
                tracing::debug!(%err, "cannot write the chime");
                None
            }
        }
    })
    .clone()
}

/// Whether the desktop's Do not disturb is on, as the notification server
/// on `connection` says; a server that does not say counts as off.
pub async fn quiet(connection: &zbus::Connection) -> bool {
    let reply = connection
        .call_method(
            Some("org.freedesktop.Notifications"),
            "/org/freedesktop/Notifications",
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.freedesktop.Notifications", "Inhibited"),
        )
        .await;
    let Ok(reply) = reply else {
        return false;
    };
    reply
        .body()
        .deserialize::<zbus::zvariant::OwnedValue>()
        .ok()
        .and_then(|v| bool::try_from(v).ok())
        .unwrap_or(false)
}

/// Plays sound `id` unless Do not disturb is on, asking the session bus's
/// notification server first, without waiting for either.
pub fn play_unless_quiet(id: &'static str) {
    std::thread::spawn(move || {
        let quiet = futures_lite::future::block_on(async {
            match zbus::Connection::session().await {
                Ok(connection) => quiet(&connection).await,
                Err(_) => false,
            }
        });
        if !quiet {
            play(id);
        }
    });
}

#[cfg(target_os = "linux")]
mod linux {
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    /// Sound themes looked in after the desktop's own.
    const THEMES: [&str; 2] = ["ocean", "freedesktop"];
    const EXTENSIONS: [&str; 3] = ["oga", "ogg", "wav"];

    /// Plays the first of `names` found.
    pub(super) fn play(names: &[&str]) {
        if play_file(find(names)) {
            return;
        }
        // libcanberra finds the sound in the desktop's theme itself.
        if let Some(name) = names.first() {
            spawn(Command::new("canberra-gtk-play").args(["-i", name]));
        }
    }

    /// Plays `file`, if any, with the desktop's player. Tells whether one
    /// started.
    pub(super) fn play_file(file: Option<PathBuf>) -> bool {
        let Some(file) = file else {
            return false;
        };
        ["pw-play", "paplay", "canberra-gtk-play"]
            .iter()
            .any(|player| {
                let mut command = Command::new(player);
                if *player == "canberra-gtk-play" {
                    command.arg("-f");
                }
                spawn(command.arg(&file))
            })
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

    /// The desktop's sound theme, where it says: KDE's `[Sounds] Theme`.
    fn desktop_theme() -> Option<String> {
        let config = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        let text = std::fs::read_to_string(config.join("kdeglobals")).ok()?;
        let mut in_sounds = false;
        for line in text.lines().map(str::trim) {
            if line.starts_with('[') {
                in_sounds = line == "[Sounds]";
            } else if in_sounds && let Some(theme) = line.strip_prefix("Theme=") {
                let theme = theme.trim();
                return (!theme.is_empty()).then(|| theme.to_owned());
            }
        }
        None
    }

    /// The file of the first of `names` in the sound themes of the XDG
    /// data folders.
    pub(super) fn find(names: &[&str]) -> Option<PathBuf> {
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
        let mut themes: Vec<String> = desktop_theme().into_iter().collect();
        themes.extend(THEMES.iter().map(|t| (*t).to_owned()));
        names.iter().find_map(|name| {
            themes.iter().find_map(|theme| {
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
        })
    }
}

#[cfg(windows)]
mod win {
    use std::path::PathBuf;
    use std::time::Duration;

    use windows::Foundation::Uri;
    use windows::Media::Core::MediaSource;
    use windows::Media::Playback::MediaPlayer;
    use windows::core::HSTRING;

    /// Long enough for any of the Windows notification sounds.
    const LONGEST: Duration = Duration::from_secs(8);

    /// Plays `file` of `%SystemRoot%\Media` on a thread of its own, which
    /// keeps the player until the sound is over.
    pub(super) fn play(file: &'static str) {
        std::thread::spawn(move || {
            if let Err(err) = play_now(file) {
                tracing::debug!(%err, file, "cannot play a sound");
            }
        });
    }

    fn play_now(file: &str) -> windows::core::Result<()> {
        let root = std::env::var_os("SystemRoot")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
        let path = root.join("Media").join(file);
        let url = format!(
            "file:///{}",
            path.to_string_lossy()
                .replace('\\', "/")
                .replace(' ', "%20")
        );
        let source = MediaSource::CreateFromUri(&Uri::CreateUri(&HSTRING::from(url))?)?;
        let player = MediaPlayer::new()?;
        player.SetSource(&source)?;
        player.Play()?;
        std::thread::sleep(LONGEST);
        player.Close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_event_has_a_usual_sound_on_the_list() {
        for event in SoundEvent::ALL {
            let id = usual(event);
            assert!(CHOICES.iter().any(|c| c.id == id), "{event:?}");
            assert_eq!(resolve(event, ""), id);
            assert_eq!(resolve(event, "no-such-sound"), id);
        }
        let other = CHOICES[1].id;
        assert_eq!(resolve(SoundEvent::NewMail, other), other);
    }
}
