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

/// The attributes of the refresh token of a sign-in linked to `account`
/// (`AccountSettings::linked`). Its own attribute names, not `account`:
/// the Secret Service finds items by a subset of their attributes, so a
/// password lookup would otherwise find this token too.
#[cfg_attr(windows, allow(dead_code))]
fn linked_attributes(account: AccountId) -> [(&'static str, String); 2] {
    [
        ("application", ids::PREFIX.to_owned()),
        ("linked-account", account.to_string()),
    ]
}

/// Where the linked sign-in of `account` sits in the in-memory store,
/// apart from the accounts' passwords (ids are positive).
fn linked_key(account: AccountId) -> AccountId {
    AccountId(-1 - account.0)
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

    /// The refresh token of the sign-in linked to `account`, if saved.
    pub async fn linked_token(&self, account: AccountId) -> Result<Option<String>, Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                let Some(item) = keyring
                    .search_items(&linked_attributes(account))
                    .await?
                    .into_iter()
                    .next()
                else {
                    return Ok(None);
                };
                let secret = item.secret().await?;
                String::from_utf8(secret.to_vec())
                    .map(Some)
                    .map_err(|_| Error("the saved sign-in is not UTF-8".into()))
            }
            #[cfg(windows)]
            Self::Keyring(store) => store.get(&windows::linked(account)).await,
            Self::Memory(map) => Ok(map.lock().unwrap().get(&linked_key(account)).cloned()),
        }
    }

    /// Saves the refresh token of the sign-in linked to `account`.
    pub async fn set_linked_token(
        &self,
        account: AccountId,
        label: &str,
        token: &str,
    ) -> Result<(), Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                keyring
                    .create_item(
                        &format!("Katna: {label}"),
                        &linked_attributes(account),
                        token,
                        true,
                    )
                    .await?;
            }
            #[cfg(windows)]
            Self::Keyring(store) => {
                let _ = label;
                store.set(&windows::linked(account), token).await?
            }
            Self::Memory(map) => {
                map.lock()
                    .unwrap()
                    .insert(linked_key(account), token.to_owned());
            }
        }
        Ok(())
    }

    /// Deletes the refresh token of the sign-in linked to `account`, if
    /// any.
    pub async fn delete_linked_token(&self, account: AccountId) -> Result<(), Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                keyring.unlock().await?;
                keyring.delete(&linked_attributes(account)).await?;
            }
            #[cfg(windows)]
            Self::Keyring(store) => store.delete(&windows::linked(account)).await?,
            Self::Memory(map) => {
                map.lock().unwrap().remove(&linked_key(account));
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

/// Credential Manager keeps at most 2560 bytes of UTF-16 per entry:
/// longer secrets, such as Microsoft's refresh tokens, are split over
/// several entries (`<user>`, `<user>~1`, …).
#[cfg_attr(not(windows), allow(dead_code))]
const PART_UNITS: usize = 1280;

#[cfg_attr(not(windows), allow(dead_code))]
fn split(secret: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let (mut start, mut units) = (0, 0);
    for (at, c) in secret.char_indices() {
        if units + c.len_utf16() > PART_UNITS {
            parts.push(&secret[start..at]);
            (start, units) = (at, 0);
        }
        units += c.len_utf16();
    }
    parts.push(&secret[start..]);
    parts
}

/// The daemon's view of the Windows Credential Manager
/// ([`katna_platform::credentials`]): each entry a generic credential
/// named `<user>.in.invenia.katna`, kept on this computer only.
#[cfg(windows)]
mod windows {
    use katna_core::AccountId;
    use katna_platform::credentials::Credentials;

    use super::{Error, split};

    /// The user name of the Katna Server token.
    pub const SERVER: &str = "katna-server";

    /// The user name of an account's password.
    pub fn account(account: AccountId) -> String {
        format!("account-{account}")
    }

    /// The user name of the refresh token of a sign-in linked to an
    /// account.
    pub fn linked(account: AccountId) -> String {
        format!("linked-{account}")
    }

    /// The user name of part `index` of a long secret; part 0 is `user`.
    fn part(user: &str, index: usize) -> String {
        match index {
            0 => user.to_owned(),
            _ => format!("{user}~{index}"),
        }
    }

    pub struct Store(Credentials);

    impl Store {
        pub fn new() -> Result<Self, Error> {
            Credentials::open().map(Self).map_err(Error)
        }

        /// Deletes the parts from `from` on.
        fn delete_parts(&self, user: &str, from: usize) -> Result<(), Error> {
            for index in from.. {
                let name = part(user, index);
                if self.0.get(&name).map_err(Error)?.is_none() {
                    break;
                }
                self.0.delete(&name).map_err(Error)?;
            }
            Ok(())
        }

        pub async fn get(&self, user: &str) -> Result<Option<String>, Error> {
            let Some(mut secret) = self.0.get(user).map_err(Error)? else {
                return Ok(None);
            };
            for index in 1.. {
                match self.0.get(&part(user, index)).map_err(Error)? {
                    Some(more) => secret.push_str(&more),
                    None => break,
                }
            }
            Ok(Some(secret))
        }

        pub async fn set(&self, user: &str, secret: &str) -> Result<(), Error> {
            let parts = split(secret);
            for (index, text) in parts.iter().enumerate() {
                self.0.set(&part(user, index), text).map_err(Error)?;
            }
            self.delete_parts(user, parts.len())
        }

        pub async fn delete(&self, user: &str) -> Result<(), Error> {
            self.delete_parts(user, 0)
        }

        pub async fn delete_all(&self) -> Result<(), Error> {
            self.0.delete_all().map_err(Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use katna_core::AccountId;

    use super::{PART_UNITS, Secrets, split};

    #[test]
    fn linked_sign_ins_are_kept_apart_from_passwords() {
        smol::block_on(async {
            let secrets = Secrets::memory();
            let id = AccountId(3);
            secrets.set_password(id, "a@zoho.in", "pw").await.unwrap();
            secrets.set_linked_token(id, "Zoho", "rt").await.unwrap();
            assert_eq!(secrets.password(id).await.unwrap().as_deref(), Some("pw"));
            assert_eq!(
                secrets.linked_token(id).await.unwrap().as_deref(),
                Some("rt")
            );
            assert_eq!(secrets.linked_token(AccountId(4)).await.unwrap(), None);
            secrets.delete_linked_token(id).await.unwrap();
            assert_eq!(secrets.linked_token(id).await.unwrap(), None);
            assert_eq!(secrets.password(id).await.unwrap().as_deref(), Some("pw"));
        });
    }

    #[test]
    fn long_secrets_are_split() {
        assert_eq!(split(""), [""]);
        assert_eq!(split("short"), ["short"]);
        let long = "a".repeat(PART_UNITS * 2 + 5);
        let parts = split(&long);
        assert_eq!(
            parts.iter().map(|p| p.len()).collect::<Vec<_>>(),
            [PART_UNITS, PART_UNITS, 5]
        );
        // A character never straddles two parts.
        let wide = "😀".repeat(PART_UNITS);
        let parts = split(&wide);
        assert!(parts.iter().all(|p| p.encode_utf16().count() <= PART_UNITS));
        assert_eq!(parts.concat(), wide);
    }
}
