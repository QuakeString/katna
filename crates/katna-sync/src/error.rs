// SPDX-License-Identifier: GPL-3.0-or-later

//! Errors from the protocol layer.

use std::{io, time::Duration};

/// What went wrong talking to a mail server.
///
/// Protocol details are flattened to text. The engine only needs to decide
/// whether to retry, ask for a new password or give up; [`Error::is_fatal`]
/// says whether the connection can still be used.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The network failed (connection refused, reset, DNS, …).
    #[error("network: {0}")]
    Io(#[from] io::Error),

    /// The TLS handshake failed, for example on an untrusted certificate.
    /// The text names the server and reads as a reason on its own.
    #[error("{0}")]
    Tls(String),

    /// The server refused the user name or password.
    #[error("authentication failed: {0}")]
    Auth(String),

    /// The server answered the command with NO or BAD (a missing folder, a
    /// full mailbox, …). The connection is still usable.
    #[error("server refused the command: {0}")]
    Rejected(String),

    /// The provider has the API switched off for Katna's app (an API not
    /// enabled in Katna's Google Cloud project): signing in again does
    /// not help.
    #[error("not enabled: {0}")]
    NotEnabled(String),

    /// The server said something we could not understand, or broke the
    /// protocol.
    #[error("protocol: {0}")]
    Protocol(String),

    /// The server did not answer in time.
    #[error("no answer after {0:?}")]
    Timeout(Duration),

    /// The server could not be reached in time: no connection, or no
    /// secure handshake. Says which, with the server's name.
    #[error("{0}")]
    Unreachable(String),

    /// The server or the connection's task closed the connection.
    #[error("connection closed: {0}")]
    Closed(String),

    /// Reading or writing the local store failed.
    #[error("store: {0}")]
    Store(#[from] katna_store::Error),
}

impl Error {
    /// Whether the connection is unusable after this error. Only
    /// [`Error::Rejected`], [`Error::NotEnabled`] (and store errors, which never touch it) leave it
    /// ready for the next command.
    pub fn is_fatal(&self) -> bool {
        !matches!(
            self,
            Self::Rejected(_) | Self::NotEnabled(_) | Self::Store(_)
        )
    }

    /// Whether trying again later, on a new connection, may succeed.
    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            Self::Io(_) | Self::Timeout(_) | Self::Unreachable(_) | Self::Closed(_)
        )
    }
}

/// Result type of `katna-sync`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
