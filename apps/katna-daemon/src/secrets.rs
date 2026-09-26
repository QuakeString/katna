// SPDX-License-Identifier: GPL-3.0-or-later

//! Account passwords. They live only in the Secret Service (KWallet, GNOME
//! Keyring, or the secret portal inside Flatpak), never in files
//! (`docs/ARCHITECTURE.md` §5.1).

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use katna_core::{AccountId, ids};

/// Where passwords are kept.
pub enum Secrets {
    /// The desktop's Secret Service.
    Keyring(oo7::Keyring),
    /// In memory, for tests. Clones of the map share it, so a test can
    /// restart a daemon with the same passwords.
    Memory(Arc<Mutex<HashMap<AccountId, String>>>),
}

/// A Secret Service failure, as text for the D-Bus caller.
#[derive(Debug, thiserror::Error)]
#[error("Secret Service: {0}")]
pub struct Error(String);

impl From<oo7::Error> for Error {
    fn from(err: oo7::Error) -> Self {
        Self(err.to_string())
    }
}

fn attributes(account: AccountId) -> [(&'static str, String); 2] {
    [
        ("application", ids::PREFIX.to_owned()),
        ("account", account.to_string()),
    ]
}

impl Secrets {
    /// Connects to the Secret Service.
    pub async fn keyring() -> Result<Self, Error> {
        Ok(Self::Keyring(oo7::Keyring::new().await?))
    }

    /// An empty in-memory store.
    pub fn memory() -> Self {
        Self::Memory(Arc::default())
    }

    /// The password of `account`, if one is saved.
    pub async fn password(&self, account: AccountId) -> Result<Option<String>, Error> {
        match self {
            Self::Keyring(keyring) => {
                // Asks the user to unlock the keyring if it is locked.
                keyring.unlock().await?;
                let Some(item) = keyring
                    .search_items(&attributes(account))
                    .await?
                    .into_iter()
                    .next()
                else {
                    return Ok(None);
                };
                let secret = item.secret().await?;
                String::from_utf8(secret.to_vec())
                    .map(Some)
                    .map_err(|_| Error("the saved password is not UTF-8".into()))
            }
            Self::Memory(map) => Ok(map.lock().unwrap().get(&account).cloned()),
        }
    }

    /// Saves the password of `account`, replacing an old one.
    pub async fn set_password(
        &self,
        account: AccountId,
        address: &str,
        password: &str,
    ) -> Result<(), Error> {
        match self {
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                keyring
                    .create_item(
                        &format!("Katna: {address}"),
                        &attributes(account),
                        password,
                        true,
                    )
                    .await?;
            }
            Self::Memory(map) => {
                map.lock().unwrap().insert(account, password.to_owned());
            }
        }
        Ok(())
    }

    /// Deletes the password of `account`, if any.
    pub async fn delete(&self, account: AccountId) -> Result<(), Error> {
        match self {
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                keyring.delete(&attributes(account)).await?;
            }
            Self::Memory(map) => {
                map.lock().unwrap().remove(&account);
            }
        }
        Ok(())
    }
}
