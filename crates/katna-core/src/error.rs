// SPDX-License-Identifier: GPL-3.0-or-later

//! Error type shared by the `katna-core` modules.

use std::io;
use std::path::PathBuf;

/// Errors from configuration, paths and logging setup.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Neither the XDG variable nor `$HOME` gives a usable base directory.
    #[error("cannot find the home directory: $HOME is unset or not absolute")]
    NoHomeDir,

    /// A file or directory operation failed.
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// The configuration file is not valid TOML or has wrongly typed values.
    #[error("{path}: invalid configuration: {source}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    /// The configuration could not be written as TOML.
    #[error("cannot serialize configuration: {0}")]
    ConfigSerialize(#[from] toml::ser::Error),

    /// The configuration parsed, but a value is out of range.
    #[error("invalid configuration value for `{key}`: {message}")]
    ConfigValue { key: &'static str, message: String },

    /// The logging filter is invalid or a global logger is already installed.
    #[error("cannot set up logging: {0}")]
    Logging(String),
}

impl Error {
    /// Wraps an [`io::Error`] with the path it happened on.
    pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

/// Result type of `katna-core`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
