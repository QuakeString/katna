// SPDX-License-Identifier: GPL-3.0-or-later

//! The sounds Katna plays (Settings > Notifications > Sounds): sets of
//! sounds, one per event, a sound picked for an event, a file of the
//! user's own, and playing them.
//!
//! The System set is the desktop's own sounds. On Linux they come from the
//! desktop's sound theme (KDE's Ocean, or the freedesktop one), found by
//! their freedesktop names and played by the desktop's own player; Katna
//! plays them itself because notification servers such as Plasma's leave
//! a notification's `sound-name` unplayed. On Windows they are the Windows
//! notification sounds, which a toast plays itself (so Do not disturb
//! silences it). The other sets are made by Katna ([`synth`], the Katna
//! chime in [`chime`]), written once as WAV files to Katna's cache folder
//! and played like a file of the user's own. Birds is the usual set.

use std::path::{Path, PathBuf};

use katna_core::config::SoundEvent;

mod chime;
mod synth;

/// Katna's own new-mail chime, the Katna set's first sound.
pub const CHIME: &str = "katna-chime";
/// The set of the desktop's own sounds.
pub const SYSTEM: &str = "system";
/// The set used when none is picked.
pub const USUAL_SET: &str = "birds";
/// What starts the name of a sound that is a file of the user's own,
/// followed by its path.
pub const FILE: &str = "file:";
/// Files of the user's own play for at most this long.
pub const FILE_SECONDS: u64 = 5;

/// A set of sounds: one for each event, in [`SoundEvent::ALL`]'s order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Set {
    pub id: &'static str,
    pub sounds: [&'static str; 5],
}

/// The sets, as Settings shows them.
pub const SETS: &[Set] = &[
    Set {
        id: SYSTEM,
        sounds: SYSTEM_SOUNDS,
    },
    Set {
        id: "katna",
        sounds: [
            CHIME,
            "katna.twin-bells",
            "katna.soft-bell",
            "katna.rising",
            "katna.falling",
        ],
    },
    Set {
        id: "nature",
        sounds: [
            "nature.water-drop",
            "nature.wind-chimes",
            "nature.ripple",
            "nature.breeze",
            "nature.pebble",
        ],
    },
    Set {
        id: "birds",
        sounds: [
            "birds.robin",
            "birds.wren-trill",
            "birds.finch",
            "birds.swallow",
            "birds.cuckoo",
        ],
    },
    Set {
        id: "animals",
        sounds: [
            "animals.tree-frog",
            "animals.owl",
            "animals.cat",
            "animals.dolphin",
            "animals.bullfrog",
        ],
    },
    Set {
        id: "insects",
        sounds: [
            "insects.cricket",
            "insects.cicada",
            "insects.katydid",
            "insects.bee",
            "insects.fly",
        ],
    },
    Set {
        id: "electronic",
        sounds: [
            "electronic.blip",
            "electronic.beacon",
            "electronic.arcade",
            "electronic.whoosh",
            "electronic.buzz",
        ],
    },
    Set {
        id: "morning",
        sounds: [
            "morning.kalimba",
            "morning.sunrise",
            "morning.marimba",
            "morning.glockenspiel",
            "morning.low-marimba",
        ],
    },
];

/// The System set: each event's usual desktop sound.
#[cfg(not(windows))]
const SYSTEM_SOUNDS: [&str; 5] = [
    "message-new-email",
    "alarm-clock-elapsed",
    "bell",
    "message-sent-email",
    "dialog-error",
];
#[cfg(windows)]
const SYSTEM_SOUNDS: [&str; 5] = ["Mail", "Reminder", "Default", "IM", "Error"];

/// One of the desktop's own sounds: its name, stored in the settings, and
/// on Linux the theme sounds tried for it in turn.
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

/// The set `id`, or the usual one for an empty or unknown name.
pub fn set(id: &str) -> &'static Set {
    SETS.iter()
        .find(|s| s.id == id)
        .or_else(|| SETS.iter().find(|s| s.id == USUAL_SET))
        .unwrap_or(&SETS[0])
}

/// The sound `event` plays in set `set` (a [`Set::id`]; the usual set for
/// an empty or unknown name).
pub fn usual(event: SoundEvent, set_id: &str) -> &'static str {
    let at = SoundEvent::ALL
        .iter()
        .position(|e| *e == event)
        .unwrap_or_default();
    set(set_id).sounds[at]
}

/// Whether `id` names a sound Katna has: the desktop's or a set's.
fn known(id: &str) -> bool {
    CHOICES.iter().any(|c| c.id == id) || SETS.iter().any(|s| s.sounds.contains(&id))
}

/// The sound `event` plays when the settings name set `set_id` and, for
/// the event, `chosen`: that sound, a file of the user's own that is still
/// there, or the set's sound for an empty or unknown name.
pub fn resolve(event: SoundEvent, set_id: &str, chosen: &str) -> String {
    let fits = match file_of(chosen) {
        Some(path) => path.is_file(),
        None => known(chosen),
    };
    if fits {
        chosen.to_owned()
    } else {
        usual(event, set_id).to_owned()
    }
}

/// The path of a file of the user's own that `id` names.
pub fn file_of(id: &str) -> Option<&Path> {
    id.strip_prefix(FILE).map(Path::new)
}

/// The name of the sound that is the user's file `path`.
pub fn file_sound(path: &Path) -> String {
    format!("{FILE}{}", path.display())
}

/// The desktop's own sounds to pick from: those this computer has, or all
/// of them when it has none of them.
pub fn system_choices() -> &'static [Choice] {
    #[cfg(target_os = "linux")]
    {
        use std::sync::OnceLock;
        static FOUND: OnceLock<Vec<Choice>> = OnceLock::new();
        let found = FOUND.get_or_init(|| {
            CHOICES
                .iter()
                .filter(|c| linux::find(c.names).is_some())
                .copied()
                .collect()
        });
        if !found.is_empty() {
            return found;
        }
    }
    CHOICES
}

/// Whether a notification's server plays sound `id` itself: on Windows a
/// toast plays the Windows sounds, and only those. Katna plays the others.
pub fn toast_plays(id: &str) -> bool {
    cfg!(windows) && CHOICES.iter().any(|c| c.id == id)
}

/// Plays sound `id` (a [`Choice::id`], a set's sound, or a file of the
/// user's own), without waiting for it. Nothing plays where it is not
/// found.
pub fn play(id: &str) {
    if let Some(path) = file_of(id) {
        play_path(path.to_owned(), FILE_SECONDS);
        return;
    }
    if let Some(file) = made_file(id) {
        play_path(file, 3);
        return;
    }
    let Some(choice) = CHOICES.iter().find(|c| c.id == id) else {
        tracing::debug!(id, "no such sound");
        return;
    };
    #[cfg(target_os = "linux")]
    linux::play(choice.names);
    #[cfg(windows)]
    win::play_media(choice.file);
    #[cfg(not(any(target_os = "linux", windows)))]
    let _ = choice;
}

/// Plays the file at `path` for at most `seconds`.
fn play_path(path: PathBuf, seconds: u64) {
    #[cfg(target_os = "linux")]
    linux::play_file(Some(path), seconds);
    #[cfg(windows)]
    win::play_path(path, std::time::Duration::from_secs(seconds));
    #[cfg(not(any(target_os = "linux", windows)))]
    let _ = (path, seconds);
}

/// Changes with the made sounds, so an old file is never played.
const MADE_VERSION: u32 = 2;

/// The file of Katna's own sound `id` in its cache folder, written there
/// the first time; `None` for a sound Katna does not make.
fn made_file(id: &str) -> Option<PathBuf> {
    if !SETS
        .iter()
        .any(|s| s.id != SYSTEM && s.sounds.contains(&id))
    {
        return None;
    }
    let dir = katna_core::Paths::from_env()
        .map(|p| p.cache_dir().join("sounds"))
        .unwrap_or_else(|_| std::env::temp_dir().join("katna-sounds"));
    let file = dir.join(format!("{id}-{MADE_VERSION}.wav"));
    if file.is_file() {
        return Some(file);
    }
    let wav = synth::wav(&synth::samples(id)?);
    let written = std::fs::create_dir_all(&dir).and_then(|()| {
        // Written aside first: the daemon and the app may both write it.
        let part = dir.join(format!("{id}-{MADE_VERSION}.{}.part", std::process::id()));
        std::fs::write(&part, &wav)?;
        std::fs::rename(&part, &file)
    });
    match written {
        Ok(()) => Some(file),
        Err(err) => {
            tracing::debug!(%err, id, "cannot write a sound");
            None
        }
    }
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
pub fn play_unless_quiet(id: String) {
    std::thread::spawn(move || {
        let quiet = futures_lite::future::block_on(async {
            match zbus::Connection::session().await {
                Ok(connection) => quiet(&connection).await,
                Err(_) => false,
            }
        });
        if !quiet {
            play(&id);
        }
    });
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "linux")]
mod linux {
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    /// Sound themes looked in after the desktop's own.
    const THEMES: [&str; 2] = ["ocean", "freedesktop"];
    const EXTENSIONS: [&str; 3] = ["oga", "ogg", "wav"];

    /// Plays the first of `names` found.
    pub(super) fn play(names: &[&str]) {
        if play_file(find(names), 10) {
            return;
        }
        // libcanberra finds the sound in the desktop's theme itself.
        if let Some(name) = names.first() {
            spawn(Command::new("canberra-gtk-play").args(["-i", name]), 10);
        }
    }

    /// Plays `file`, if any, with the desktop's player, for at most
    /// `seconds`. Tells whether one started.
    pub(super) fn play_file(file: Option<PathBuf>, seconds: u64) -> bool {
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
                spawn(command.arg(&file), seconds)
            })
    }

    /// Starts `command` quietly, stops it after `seconds` and reaps it.
    /// Tells whether it started.
    fn spawn(command: &mut Command, seconds: u64) -> bool {
        let child = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        match child {
            Ok(mut child) => {
                std::thread::spawn(move || {
                    let until = std::time::Instant::now() + std::time::Duration::from_secs(seconds);
                    while std::time::Instant::now() < until {
                        if !matches!(child.try_wait(), Ok(None)) {
                            return;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    let _ = child.kill();
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

    /// Plays `file` of `%SystemRoot%\Media`.
    pub(super) fn play_media(file: &'static str) {
        let root = std::env::var_os("SystemRoot")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
        play_path(root.join("Media").join(file), LONGEST);
    }

    /// Plays the file at `path` for at most `limit` on a thread of its
    /// own, which keeps the player until then.
    pub(super) fn play_path(path: PathBuf, limit: Duration) {
        std::thread::spawn(move || {
            if let Err(err) = play_now(&path, limit) {
                tracing::debug!(%err, path = %path.display(), "cannot play a sound");
            }
        });
    }

    fn play_now(path: &std::path::Path, limit: Duration) -> windows::core::Result<()> {
        let mut url = String::from("file:///");
        for c in path.to_string_lossy().replace('\\', "/").chars() {
            match c {
                ' ' | '%' | '#' | '?' => url.push_str(&format!("%{:02X}", c as u32)),
                c => url.push(c),
            }
        }
        let source = MediaSource::CreateFromUri(&Uri::CreateUri(&HSTRING::from(url))?)?;
        let player = MediaPlayer::new()?;
        player.SetSource(&source)?;
        player.Play()?;
        std::thread::sleep(limit);
        player.Close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_set_has_a_sound_for_every_event() {
        assert_eq!(set("").id, USUAL_SET);
        assert_eq!(set("no-such-set").id, USUAL_SET);
        for s in SETS {
            assert_eq!(set(s.id), s);
            for (n, event) in SoundEvent::ALL.into_iter().enumerate() {
                let id = usual(event, s.id);
                assert_eq!(id, s.sounds[n]);
                assert!(known(id), "{id}");
                assert_eq!(resolve(event, s.id, ""), id);
                assert_eq!(resolve(event, s.id, "no-such-sound"), id);
            }
        }
        // A sound picked for an event wins over the set's.
        assert_eq!(resolve(SoundEvent::NewMail, "birds", CHIME), CHIME);
        assert_eq!(
            resolve(SoundEvent::NewMail, "birds", "nature.pebble"),
            "nature.pebble"
        );
    }

    #[test]
    fn a_file_plays_while_it_is_there() {
        let dir = std::env::temp_dir().join(format!("katna-sound-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("my sound.wav");
        let id = file_sound(&path);
        assert_eq!(file_of(&id), Some(path.as_path()));
        assert_eq!(resolve(SoundEvent::Sent, "katna", &id), "katna.rising");
        std::fs::write(&path, b"RIFF").unwrap();
        assert_eq!(resolve(SoundEvent::Sent, "katna", &id), id);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn only_windows_toasts_play_their_own_sounds() {
        assert!(!toast_plays("birds.robin"));
        assert_eq!(toast_plays(SYSTEM_SOUNDS[0]), cfg!(windows));
    }
}
