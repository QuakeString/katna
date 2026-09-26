// SPDX-License-Identifier: GPL-3.0-or-later

//! User settings, stored as TOML in `$XDG_CONFIG_HOME/katna/config.toml`.
//!
//! Every field has a default, so a missing file or a missing key is not an
//! error. Unknown keys are ignored, so an older Katna can read a file written
//! by a newer one.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Allowed undo-send delays in seconds (`docs/ARCHITECTURE.md` §10).
pub const UNDO_SEND_CHOICES: [u32; 5] = [0, 5, 10, 20, 30];

/// All user settings.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: General,
    pub logging: Logging,
    pub sending: Sending,
}

/// Background-service behavior (`docs/ARCHITECTURE.md` §9.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct General {
    /// Keep `katna-daemon` running when no window is open.
    pub run_in_background: bool,
    /// Show a tray icon.
    pub tray_icon: bool,
}

impl Default for General {
    fn default() -> Self {
        Self {
            run_in_background: true,
            tray_icon: false,
        }
    }
}

/// Logging settings. `$KATNA_LOG` overrides [`Logging::filter`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Logging {
    /// A `tracing` filter such as `info` or `warn,katna_sync=debug`.
    pub filter: String,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            filter: "info".to_owned(),
        }
    }
}

/// Sending settings (`docs/ARCHITECTURE.md` §11).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sending {
    /// Undo-send delay in seconds; one of [`UNDO_SEND_CHOICES`].
    pub undo_send_seconds: u32,
}

impl Default for Sending {
    fn default() -> Self {
        Self {
            undo_send_seconds: 10,
        }
    }
}

impl Config {
    /// Reads and validates the configuration at `path`. A missing file gives
    /// the defaults.
    pub fn load(path: &Path) -> Result<Self> {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(err) => return Err(Error::io(path, err)),
        };
        Self::parse(&text).map_err(|err| match err {
            ParseError::Toml(source) => Error::ConfigParse {
                path: path.to_owned(),
                source,
            },
            ParseError::Invalid(err) => err,
        })
    }

    /// Parses and validates TOML text.
    fn parse(text: &str) -> Result<Self, ParseError> {
        let config: Self = toml::from_str(text).map_err(ParseError::Toml)?;
        config.validate().map_err(ParseError::Invalid)?;
        Ok(config)
    }

    /// Checks that every value is in range.
    pub fn validate(&self) -> Result<()> {
        if !UNDO_SEND_CHOICES.contains(&self.sending.undo_send_seconds) {
            return Err(Error::ConfigValue {
                key: "sending.undo_send_seconds",
                message: format!(
                    "{} is not one of {UNDO_SEND_CHOICES:?}",
                    self.sending.undo_send_seconds
                ),
            });
        }
        if self.logging.filter.trim().is_empty() {
            return Err(Error::ConfigValue {
                key: "logging.filter",
                message: "must not be empty".to_owned(),
            });
        }
        Ok(())
    }

    /// Validates and writes the configuration to `path`, creating the parent
    /// directory if needed.
    ///
    /// The file is written to a temporary file next to `path` and renamed
    /// over it, so readers never see a half-written file.
    pub fn save(&self, path: &Path) -> Result<()> {
        self.validate()?;
        let text = toml::to_string_pretty(self)?;
        let dir = path
            .parent()
            .filter(|dir| !dir.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        fs::create_dir_all(dir).map_err(|err| Error::io(dir, err))?;
        let mut tmp = tempfile_in(dir)?;
        tmp.1
            .write_all(text.as_bytes())
            .and_then(|()| tmp.1.sync_all())
            .map_err(|err| Error::io(&tmp.0, err))?;
        fs::rename(&tmp.0, path).map_err(|err| {
            let _ = fs::remove_file(&tmp.0);
            Error::io(path, err)
        })
    }
}

#[derive(Debug)]
enum ParseError {
    Toml(toml::de::Error),
    Invalid(Error),
}

/// Creates a new, uniquely named file in `dir`.
fn tempfile_in(dir: &Path) -> Result<(std::path::PathBuf, fs::File)> {
    let pid = std::process::id();
    for attempt in 0u32.. {
        let path = dir.join(format!(".config.toml.{pid}.{attempt}.tmp"));
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => return Ok((path, file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(Error::io(path, err)),
        }
    }
    unreachable!("u32 range exhausted while creating a temporary file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_gives_defaults() {
        let tmp = tempfile::tempdir().unwrap();
        let config = Config::load(&tmp.path().join("config.toml")).unwrap();
        assert_eq!(config, Config::default());
        assert!(config.general.run_in_background);
        assert_eq!(config.sending.undo_send_seconds, 10);
    }

    #[test]
    fn partial_file_keeps_other_defaults() {
        let config = Config::parse("[sending]\nundo_send_seconds = 30\n").unwrap();
        assert_eq!(config.sending.undo_send_seconds, 30);
        assert_eq!(config.general, General::default());
        assert_eq!(config.logging, Logging::default());
    }

    #[test]
    fn unknown_keys_are_ignored() {
        let config = Config::parse("future = 1\n[general]\nnew_option = \"x\"\n").unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn rejects_bad_toml_with_path() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("config.toml");
        fs::write(&path, "[general\n").unwrap();
        let err = Config::load(&path).unwrap_err();
        assert!(matches!(err, Error::ConfigParse { .. }), "{err:?}");
        assert!(err.to_string().contains("config.toml"), "{err}");
    }

    #[test]
    fn rejects_wrong_types() {
        assert!(matches!(
            Config::parse("[general]\nrun_in_background = \"yes\"\n"),
            Err(ParseError::Toml(_))
        ));
    }

    #[test]
    fn rejects_out_of_range_values() {
        let Err(ParseError::Invalid(err)) = Config::parse("[sending]\nundo_send_seconds = 7\n")
        else {
            panic!("undo_send_seconds = 7 was accepted");
        };
        assert!(matches!(
            err,
            Error::ConfigValue {
                key: "sending.undo_send_seconds",
                ..
            }
        ));
        assert!(matches!(
            Config::parse("[logging]\nfilter = \" \"\n"),
            Err(ParseError::Invalid(_))
        ));
    }

    #[test]
    fn save_and_load_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("nested/config.toml");
        let mut config = Config::default();
        config.general.tray_icon = true;
        config.logging.filter = "debug".to_owned();
        config.sending.undo_send_seconds = 0;
        config.save(&path).unwrap();
        assert_eq!(Config::load(&path).unwrap(), config);
        // No temporary files are left behind.
        let entries: Vec<_> = fs::read_dir(path.parent().unwrap()).unwrap().collect();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn save_refuses_invalid_config() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("config.toml");
        let mut config = Config::default();
        config.sending.undo_send_seconds = 3;
        assert!(config.save(&path).is_err());
        assert!(!path.exists());
    }
}
