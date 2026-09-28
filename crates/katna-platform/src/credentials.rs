// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna's entries in the Windows Credential Manager (`docs/ARCHITECTURE.md`
//! §27.1): generic credentials named `<user>.in.invenia.katna`, kept on this
//! computer only. The daemon keeps passwords here; Setup deletes them when
//! asked to remove the user's data.

use std::collections::HashMap;
use std::sync::Arc;

use katna_core::ids;
use keyring_core::{Entry, api::CredentialStoreApi};
use windows_native_keyring_store::Store as WinStore;

fn error(err: keyring_core::Error) -> String {
    format!("Credential Manager: {err}")
}

/// The Credential Manager, for Katna's entries.
pub struct Credentials(Arc<WinStore>);

impl Credentials {
    pub fn open() -> Result<Self, String> {
        WinStore::new().map(Self).map_err(error)
    }

    fn entry(&self, user: &str) -> Result<Entry, String> {
        let local = HashMap::from([("persistence", "Local")]);
        self.0.build(ids::PREFIX, user, Some(&local)).map_err(error)
    }

    /// The secret saved for `user`, if any.
    pub fn get(&self, user: &str) -> Result<Option<String>, String> {
        match self.entry(user)?.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring_core::Error::NoEntry) => Ok(None),
            Err(err) => Err(error(err)),
        }
    }

    /// Saves `secret` for `user`, replacing an old one.
    pub fn set(&self, user: &str, secret: &str) -> Result<(), String> {
        self.entry(user)?.set_password(secret).map_err(error)
    }

    /// Deletes the secret of `user`, if any.
    pub fn delete(&self, user: &str) -> Result<(), String> {
        match self.entry(user)?.delete_credential() {
            Ok(()) | Err(keyring_core::Error::NoEntry) => Ok(()),
            Err(err) => Err(error(err)),
        }
    }

    /// Deletes every secret Katna saved.
    pub fn delete_all(&self) -> Result<(), String> {
        let pattern = format!(r"^.+\.{}$", ids::PREFIX.replace('.', r"\."));
        let spec = HashMap::from([("pattern", pattern.as_str())]);
        for entry in self.0.search(&spec).map_err(error)? {
            match entry.delete_credential() {
                Ok(()) | Err(keyring_core::Error::NoEntry) => {}
                Err(err) => return Err(error(err)),
            }
        }
        Ok(())
    }
}
