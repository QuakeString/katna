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
    Keyring(keyring::Keyring),
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

#[cfg(unix)]
impl From<zbus::Error> for Error {
    fn from(err: zbus::Error) -> Self {
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
        Ok(Self::Keyring(keyring::Keyring::new().await?))
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

    /// Whether the keyring is locked and the user turned down unlocking
    /// it: reading passwords then fails until they unlock it themselves
    /// (in KWallet, GNOME Keyring's Passwords, or a Sync now in Katna).
    pub fn declined(&self) -> bool {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => keyring.declined(),
            #[cfg(windows)]
            Self::Keyring(_) => false,
            Self::Memory(_) => false,
        }
    }

    /// After [`Self::declined`]: whether the keyring is open now, looked up
    /// without asking the user.
    pub async fn unlocked_since(&self) -> bool {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => keyring.unlocked_since().await,
            #[cfg(windows)]
            Self::Keyring(_) => true,
            Self::Memory(_) => true,
        }
    }

    /// Lets the next read ask the user to unlock the keyring again: they
    /// asked for something that needs it, such as Sync now.
    pub fn ask_again(&self) {
        #[cfg(unix)]
        if let Self::Keyring(keyring) = self {
            keyring.ask_again();
        }
    }

    /// The password of `account`, if one is saved.
    pub async fn password(&self, account: AccountId) -> Result<Option<String>, Error> {
        match self {
            #[cfg(unix)]
            Self::Keyring(keyring) => {
                // Asks the user to unlock the keyring if it is locked,
                // unless they turned that down (keyring::Keyring::unlock).
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
                keyring.unlock_asking().await?;
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
                keyring.unlock_asking().await?;
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
                keyring.unlock_asking().await?;
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
                keyring.unlock_asking().await?;
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
                keyring.unlock_asking().await?;
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
                keyring.unlock_asking().await?;
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

/// The Secret Service at login: the keyring may still be locked when the
/// daemon starts, and its unlock prompt may be dismissed or never answered.
/// Katna then asks once, not once per account, and waits for the user to
/// unlock it rather than asking again at every read.
#[cfg(unix)]
mod keyring {
    use std::sync::atomic::{AtomicBool, Ordering};

    use smol::lock::Mutex;
    use zbus::proxy::CacheProperties;

    use super::Error;

    pub struct Keyring {
        inner: oo7::Keyring,
        /// Our own connection, for reading `Locked` without a cache: the
        /// keyring may not say when it changes.
        connection: zbus::Connection,
        /// One unlock prompt at a time; the others wait for its answer.
        prompt: Mutex<()>,
        /// The user dismissed the prompt and has not unlocked it since.
        declined: AtomicBool,
    }

    impl std::ops::Deref for Keyring {
        type Target = oo7::Keyring;

        fn deref(&self) -> &oo7::Keyring {
            &self.inner
        }
    }

    impl Keyring {
        pub async fn new() -> Result<Self, Error> {
            Ok(Self {
                inner: match oo7::Keyring::new().await {
                    Ok(keyring) => keyring,
                    // In a Snap (or a Flatpak) oo7 asks the secret portal
                    // first and gives up if no portal service runs at all;
                    // the Secret Service itself (the Snap's
                    // password-manager-service plug) still does, as oo7
                    // does when the portal is merely missing its Secret part.
                    Err(oo7::Error::File(err)) => {
                        tracing::info!("no secret portal ({err}); using the Secret Service");
                        let service = oo7::dbus::Service::new().await.map_err(oo7::Error::from)?;
                        oo7::Keyring::DBus(
                            service
                                .default_collection()
                                .await
                                .map_err(oo7::Error::from)?,
                        )
                    }
                    Err(err) => return Err(err.into()),
                },
                connection: zbus::Connection::session().await?,
                prompt: Mutex::new(()),
                declined: AtomicBool::new(false),
            })
        }

        pub fn declined(&self) -> bool {
            self.declined.load(Ordering::SeqCst)
        }

        pub fn ask_again(&self) {
            self.declined.store(false, Ordering::SeqCst);
        }

        /// Unlocks the keyring for a read: asks the user unless they
        /// turned that down, then only checks whether they unlocked it.
        pub async fn unlock(&self) -> Result<(), Error> {
            let _one = self.prompt.lock().await;
            if self.declined() {
                return if self.unlocked_since().await {
                    Ok(())
                } else {
                    Err(Error(LOCKED.into()))
                };
            }
            self.ask().await
        }

        /// Unlocks the keyring for a change the user made: asks even if
        /// they turned it down before.
        pub async fn unlock_asking(&self) -> Result<(), Error> {
            let _one = self.prompt.lock().await;
            self.ask().await
        }

        async fn ask(&self) -> Result<(), Error> {
            match self.inner.unlock().await {
                Ok(()) => {
                    self.ask_again();
                    Ok(())
                }
                Err(err) if still_locked(&err) => {
                    tracing::warn!(%err, "the keyring stays locked; waiting for it");
                    self.declined.store(true, Ordering::SeqCst);
                    Err(Error(LOCKED.into()))
                }
                Err(err) => Err(err.into()),
            }
        }

        /// Whether the keyring is unlocked now, without asking; clears
        /// `declined` when it is.
        pub async fn unlocked_since(&self) -> bool {
            let locked = match &self.inner {
                oo7::Keyring::DBus(collection) => {
                    self.locked(collection.path().to_owned().into()).await
                }
                other => other.is_locked().await.map_err(Error::from),
            };
            match locked {
                Ok(false) => {
                    self.ask_again();
                    true
                }
                Ok(true) => false,
                Err(err) => {
                    tracing::debug!(%err, "is the keyring locked?");
                    false
                }
            }
        }

        async fn locked(&self, path: zbus::zvariant::OwnedObjectPath) -> Result<bool, Error> {
            let collection = zbus::proxy::Builder::<zbus::Proxy<'_>>::new(&self.connection)
                .destination("org.freedesktop.secrets")?
                .path(path)?
                .interface("org.freedesktop.Secret.Collection")?
                .cache_properties(CacheProperties::No)
                .build()
                .await?;
            Ok(collection.get_property::<bool>("Locked").await?)
        }
    }

    const LOCKED: &str = "the keyring is locked; unlock it to sync";

    /// The prompt was dismissed or the keyring said it is locked.
    fn still_locked(err: &oo7::Error) -> bool {
        use oo7::dbus::{Error, ServiceError};
        matches!(
            err,
            oo7::Error::DBus(Error::Dismissed | Error::Service(ServiceError::IsLocked(_)))
        )
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
