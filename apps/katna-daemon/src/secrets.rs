// SPDX-License-Identifier: GPL-3.0-or-later

//! Account passwords, and the token of this computer's Katna account on
//! Katna Server. They live only in the Secret Service (KWallet, GNOME
//! Keyring, or the secret portal inside Flatpak) or, on Windows, the
//! Credential Manager, never in files (`docs/ARCHITECTURE.md` §5.1).

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use katna_core::{AccountId, ids};

/// Where passwords are kept.
pub enum Secrets {
    /// The desktop's Secret Service.
    #[cfg(unix)]
    Keyring(oo7::Keyring),
    /// The Windows Credential Manager.
    #[cfg(windows)]
    Keyring(windows::Store),
    /// In memory, for tests. Clones of the map share it, so a test can
    /// restart a daemon with the same passwords.
    Memory(Arc<Mutex<HashMap<AccountId, String>>>),
}

/// A Secret Service failure, as text for the D-Bus caller.
#[derive(Debug, thiserror::Error)]
#[error("Secret Service: {0}")]
pub struct Error(String);

#[cfg(unix)]
impl From<oo7::Error> for Error {
    fn from(err: oo7::Error) -> Self {
        Self(err.to_string())
    }
}

#[cfg_attr(windows, allow(dead_code))]
fn attributes(account: AccountId) -> [(&'static str, String); 2] {
    [
        ("application", ids::PREFIX.to_owned()),
        ("account", account.to_string()),
    ]
}

#[cfg_attr(windows, allow(dead_code))]
fn server_attributes() -> [(&'static str, String); 2] {
    [
        ("application", ids::PREFIX.to_owned()),
        ("katna-server", "device".to_owned()),
    ]
}

/// Where the Katna Server token sits in the in-memory store.
const SERVER_KEY: AccountId = AccountId(i64::MIN);

impl Secrets {
    /// Connects to the Secret Service.
    #[cfg(unix)]
    pub async fn keyring() -> Result<Self, Error> {
        Ok(Self::Keyring(oo7::Keyring::new().await?))
    }

    /// Opens the Windows Credential Manager.
    #[cfg(windows)]
    pub async fn keyring() -> Result<Self, Error> {
        windows::Store::new().map(Self::Keyring)
    }

    /// An empty in-memory store.
    pub fn memory() -> Self {
        Self::Memory(Arc::default())
    }

    /// The password of `account`, if one is saved.
    pub async fn password(&self, account: AccountId) -> Result<Option<String>, Error> {
        match self {
            #[cfg(unix)]
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
            #[cfg(windows)]
            Self::Keyring(store) => store.get(&windows::account(account)).await,
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
            #[cfg(unix)]
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
            #[cfg(windows)]
            Self::Keyring(store) => {
                // Credential Manager shows the target name, not a label.
                let _ = address;
                store.set(&windows::account(account), password).await?
            }
            Self::Memory(map) => {
                map.lock().unwrap().insert(account, password.to_owned());
            }
        }
        Ok(())
    }

    /// This computer's Katna Server token and account, if saved.
    pub async fn server_token(&self) -> Result<Option<String>, Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                let Some(item) = keyring
                    .search_items(&server_attributes())
                    .await?
                    .into_iter()
                    .next()
                else {
                    return Ok(None);
                };
                let secret = item.secret().await?;
                String::from_utf8(secret.to_vec())
                    .map(Some)
                    .map_err(|_| Error("the saved Katna Server token is not UTF-8".into()))
            }
            #[cfg(windows)]
            Self::Keyring(store) => store.get(windows::SERVER).await,
            Self::Memory(map) => Ok(map.lock().unwrap().get(&SERVER_KEY).cloned()),
        }
    }

    /// Saves this computer's Katna Server token and account.
    pub async fn set_server_token(&self, token: &str) -> Result<(), Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                keyring
                    .create_item("Katna account", &server_attributes(), token, true)
                    .await?;
            }
            #[cfg(windows)]
            Self::Keyring(store) => store.set(windows::SERVER, token).await?,
            Self::Memory(map) => {
                map.lock().unwrap().insert(SERVER_KEY, token.to_owned());
            }
        }
        Ok(())
    }

    /// Deletes every password Katna saved, also of accounts that are gone.
    pub async fn delete_all(&self) -> Result<(), Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                keyring
                    .delete(&[("application", ids::PREFIX.to_owned())])
                    .await?;
            }
            #[cfg(windows)]
            Self::Keyring(store) => store.delete_all().await?,
            Self::Memory(map) => map.lock().unwrap().clear(),
        }
        Ok(())
    }

    /// Deletes the password of `account`, if any.
    pub async fn delete(&self, account: AccountId) -> Result<(), Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                keyring.delete(&attributes(account)).await?;
            }
            #[cfg(windows)]
            Self::Keyring(store) => store.delete(&windows::account(account)).await?,
            Self::Memory(map) => {
                map.lock().unwrap().remove(&account);
            }
        }
        Ok(())
    }
}

/// The daemon's view of the Windows Credential Manager
/// ([`katna_platform::credentials`]).
#[cfg(windows)]
mod windows {
    use katna_core::AccountId;
    use katna_platform::credentials::Credentials;

    use super::Error;

    /// The user name of the Katna Server token.
    pub const SERVER: &str = "katna-server";

    /// The user name of an account's password.
    pub fn account(account: AccountId) -> String {
        format!("account-{account}")
    }

    pub struct Store(Credentials);

    impl Store {
        pub fn new() -> Result<Self, Error> {
            Credentials::open().map(Self).map_err(Error)
        }

        pub async fn get(&self, user: &str) -> Result<Option<String>, Error> {
            self.0.get(user).map_err(Error)
        }

        pub async fn set(&self, user: &str, password: &str) -> Result<(), Error> {
            self.0.set(user, password).map_err(Error)
        }

        pub async fn delete(&self, user: &str) -> Result<(), Error> {
            self.0.delete(user).map_err(Error)
        }

        pub async fn delete_all(&self) -> Result<(), Error> {
            self.0.delete_all().map_err(Error)
        }
    }
}
