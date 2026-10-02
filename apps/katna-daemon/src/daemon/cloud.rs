// SPDX-License-Identifier: GPL-3.0-or-later

//! The accounts' cloud drives in Files (`docs/ARCHITECTURE.md` §13.8):
//! Katna Mail asks for a folder, a search or a file, and the daemon asks
//! the drive with the account's token. Nothing is synced; a file opened
//! or downloaded is kept in the cache for a day.

use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use katna_core::{AccountId, OAuthProvider};
use katna_dbus::{CloudEntry, CloudListing, cloud_place, cloud_state};
use katna_sync::{
    Error, Result as SyncResult,
    cloud::{CloudItem, CloudPage, Fetched, Place, ROOT},
    drive::{Drive, DriveFile},
    net::Tls,
    onedrive::OneDrive,
};

use super::{CommandError, Daemon};

/// How long a fetched file stays in the cache.
const KEEP: Duration = Duration::from_secs(24 * 60 * 60);

/// The biggest thumbnail asked for.
const MAX_WIDTH: u32 = 1600;

/// An account's drive in Files: Google Drive or OneDrive.
pub(super) enum Cloud {
    Google(Drive),
    Microsoft(OneDrive),
}

impl Cloud {
    async fn readable(&self) -> SyncResult<bool> {
        match self {
            Self::Google(drive) => drive.readable().await,
            Self::Microsoft(onedrive) => onedrive.readable().await,
        }
    }

    pub(super) async fn writable(&self) -> SyncResult<bool> {
        match self {
            Self::Google(drive) => drive.writable().await,
            Self::Microsoft(onedrive) => onedrive.writable().await,
        }
    }

    async fn list(&self, place: &Place, page: &str) -> SyncResult<CloudPage> {
        match self {
            Self::Google(drive) => drive.list(place, page).await,
            Self::Microsoft(onedrive) => onedrive.list(place, page).await,
        }
    }

    async fn fetch(&self, item: &CloudItem, to: &Path) -> SyncResult<Fetched> {
        match self {
            Self::Google(drive) => drive.fetch(item, to).await,
            Self::Microsoft(onedrive) => onedrive.fetch(item, to).await,
        }
    }

    async fn thumbnail(&self, link: &str, width: u32) -> SyncResult<Vec<u8>> {
        match self {
            Self::Google(drive) => drive.thumbnail(link, width).await,
            Self::Microsoft(onedrive) => onedrive.thumbnail(link, width).await,
        }
    }

    pub(super) async fn create_folder(&self, name: &str, parent: &str) -> SyncResult<String> {
        match self {
            Self::Google(drive) => drive.create_folder(name, parent).await,
            Self::Microsoft(onedrive) => onedrive.create_folder(name, parent).await,
        }
    }

    async fn trash(&self, id: &str, trashed: bool) -> SyncResult<()> {
        match self {
            Self::Google(drive) => drive.trash(id, trashed).await,
            Self::Microsoft(onedrive) => onedrive.trash(id, trashed).await,
        }
    }

    async fn rename(&self, id: &str, name: &str) -> SyncResult<()> {
        match self {
            Self::Google(drive) => drive.rename(id, name).await,
            Self::Microsoft(onedrive) => onedrive.rename(id, name).await,
        }
    }

    pub(super) async fn upload_into(
        &self,
        path: &Path,
        name: &str,
        parent: &str,
        progress: &(dyn Fn(u64, u64) + Sync),
    ) -> SyncResult<DriveFile> {
        match self {
            Self::Google(drive) => {
                let mime = super::drive::mime_of(name);
                drive.upload_into(path, name, mime, parent, progress).await
            }
            Self::Microsoft(onedrive) => onedrive.upload_into(path, name, parent, progress).await,
        }
    }
}

impl Daemon {
    /// The Google Drive or OneDrive of `account` to browse, or `None` when
    /// it has no drive Katna can browse.
    pub(super) async fn cloud(&self, account: AccountId) -> Result<Option<Cloud>, CommandError> {
        let settings = self.store().account_settings(account)?.unwrap_or_default();
        let provider = match settings.oauth {
            Some(provider @ (OAuthProvider::Google | OAuthProvider::Microsoft)) => provider,
            _ => return Ok(None),
        };
        let tokens = self
            .oauth_tokens(account, provider)
            .await
            .map_err(CommandError::AuthFailed)?;
        let tls = Tls::system().map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        Ok(Some(match provider {
            OAuthProvider::Microsoft => Cloud::Microsoft(OneDrive::new(tokens, tls)),
            _ => Cloud::Google(Drive::new(tokens, tls)),
        }))
    }

    pub async fn cloud_readable(&self, account: AccountId) -> Result<bool, CommandError> {
        let Some(drive) = self.cloud(account).await? else {
            return Ok(false);
        };
        Ok(drive.readable().await.unwrap_or(false))
    }

    pub async fn cloud_writable(&self, account: AccountId) -> Result<bool, CommandError> {
        let Some(drive) = self.cloud(account).await? else {
            return Ok(false);
        };
        Ok(drive.writable().await.unwrap_or(false))
    }

    pub async fn cloud_list(
        &self,
        account: AccountId,
        place: &str,
        what: &str,
        page: &str,
    ) -> Result<CloudListing, CommandError> {
        let place = match place {
            cloud_place::FOLDER if what.is_empty() => Place::Folder(ROOT.into()),
            cloud_place::FOLDER => Place::Folder(what.into()),
            cloud_place::SHARED => Place::Shared,
            cloud_place::SEARCH if !what.trim().is_empty() => Place::Search(what.into()),
            _ => {
                return Err(CommandError::InvalidArgs(format!(
                    "no place {place} {what}"
                )));
            }
        };
        let listing = |state: &str, error: String| CloudListing {
            state: state.into(),
            error,
            ..CloudListing::default()
        };
        let Some(drive) = self.cloud(account).await? else {
            return Ok(listing(cloud_state::UNSUPPORTED, String::new()));
        };
        if !drive.readable().await.unwrap_or(false) {
            return Ok(listing(cloud_state::NEEDS_PERMISSION, String::new()));
        }
        match drive.list(&place, page).await {
            Ok(page) => Ok(CloudListing {
                state: cloud_state::OK.into(),
                error: String::new(),
                items: page.items.into_iter().map(entry).collect(),
                next: page.next,
            }),
            Err(Error::Auth(_)) => Ok(listing(cloud_state::NEEDS_PERMISSION, String::new())),
            Err(Error::NotEnabled(detail)) => Ok(listing(cloud_state::FAILED, detail)),
            Err(err) => {
                tracing::warn!(%account, %err, "listing a drive");
                Ok(listing(cloud_state::FAILED, err.to_string()))
            }
        }
    }

    pub async fn cloud_fetch(
        &self,
        account: AccountId,
        entry: CloudEntry,
    ) -> Result<String, CommandError> {
        if entry.folder {
            return Err(CommandError::InvalidArgs(
                "a folder can't be fetched".into(),
            ));
        }
        let drive = self
            .cloud(account)
            .await?
            .ok_or_else(|| CommandError::InvalidArgs("no drive to fetch from".into()))?;
        let root = self.paths.cache_dir().join("drives");
        prune(&root);
        let dir = root.join(format!("{}-{:016x}", account.0, fingerprint(&entry.id)));
        std::fs::create_dir_all(&dir).map_err(|err| CommandError::Failed(err.to_string()))?;
        let part = dir.join(".part");
        let item = item(entry);
        let fetched = drive.fetch(&item, &part).await.map_err(failed)?;
        let path = dir.join(file_name(&fetched.name));
        std::fs::rename(&part, &path).map_err(|err| CommandError::Failed(err.to_string()))?;
        tracing::info!(%account, size = fetched.size, "fetched a drive file");
        Ok(path.to_string_lossy().into_owned())
    }

    /// Moves items `ids` of the drive of `account` to its bin, or
    /// (`trashed` false) back out; returns how many it moved.
    pub async fn cloud_trash(
        &self,
        account: AccountId,
        ids: &[String],
        trashed: bool,
    ) -> Result<u32, CommandError> {
        let drive = self
            .cloud(account)
            .await?
            .ok_or_else(|| CommandError::InvalidArgs("no drive".into()))?;
        let mut moved = 0;
        for id in ids {
            drive.trash(id, trashed).await.map_err(failed)?;
            moved += 1;
        }
        tracing::info!(%account, moved, trashed, "moved drive items");
        Ok(moved)
    }

    /// Renames item `id` of the drive of `account` to `name`.
    pub async fn cloud_rename(
        &self,
        account: AccountId,
        id: &str,
        name: &str,
    ) -> Result<(), CommandError> {
        let name = name.trim();
        if name.is_empty() || name.contains('/') {
            return Err(CommandError::InvalidArgs(format!("no name {name:?}")));
        }
        let drive = self
            .cloud(account)
            .await?
            .ok_or_else(|| CommandError::InvalidArgs("no drive".into()))?;
        drive.rename(id, name).await.map_err(failed)
    }

    pub async fn cloud_thumbnail(
        &self,
        account: AccountId,
        link: &str,
        width: u32,
    ) -> Result<Vec<u8>, CommandError> {
        let drive = self
            .cloud(account)
            .await?
            .ok_or_else(|| CommandError::InvalidArgs("no drive".into()))?;
        drive
            .thumbnail(link, width.clamp(64, MAX_WIDTH))
            .await
            .map_err(|err| CommandError::Failed(err.to_string()))
    }
}

fn failed(err: Error) -> CommandError {
    match err {
        Error::Auth(message) => CommandError::AuthFailed(message),
        other => CommandError::Failed(other.to_string()),
    }
}

fn entry(item: CloudItem) -> CloudEntry {
    CloudEntry {
        id: item.id,
        name: item.name,
        mime: item.mime,
        size: item.size,
        modified: item.modified.unwrap_or(0),
        folder: item.folder,
        native: item.native,
        link: item.link,
        thumbnail: item.thumbnail,
    }
}

fn item(entry: CloudEntry) -> CloudItem {
    CloudItem {
        id: entry.id,
        name: entry.name,
        mime: entry.mime,
        size: entry.size,
        modified: (entry.modified != 0).then_some(entry.modified),
        folder: entry.folder,
        native: entry.native,
        link: entry.link,
        thumbnail: entry.thumbnail,
    }
}

/// A short, steady number for a drive's file id, naming its cache folder.
fn fingerprint(id: &str) -> u64 {
    // FNV-1a: steady across runs, unlike the standard hasher.
    id.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// `name` made safe as one file name: no folders, no hidden file.
fn file_name(name: &str) -> String {
    let clean: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    let clean = clean.trim().trim_start_matches('.').trim();
    if clean.is_empty() {
        "file".into()
    } else {
        clean.chars().take(200).collect()
    }
}

/// Deletes the fetched files older than [`KEEP`].
fn prune(root: &Path) {
    let Ok(dirs) = std::fs::read_dir(root) else {
        return;
    };
    let now = SystemTime::now();
    for dir in dirs.flatten() {
        let old = dir
            .metadata()
            .and_then(|m| m.modified())
            .is_ok_and(|at| now.duration_since(at).unwrap_or_default() > KEEP);
        if old {
            let path: PathBuf = dir.path();
            if let Err(err) = std::fs::remove_dir_all(&path) {
                tracing::warn!(path = %path.display(), %err, "deleting a fetched drive file");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_stay_one_plain_file() {
        assert_eq!(file_name("../../etc/passwd"), "_.._etc_passwd");
        assert_eq!(file_name(".hidden"), "hidden");
        assert_eq!(file_name("  "), "file");
        assert_eq!(file_name("Q3 report.pdf"), "Q3 report.pdf");
        assert_eq!(file_name("a:b?.txt"), "a_b_.txt");
    }

    #[test]
    fn fingerprints_are_steady() {
        assert_eq!(fingerprint(""), 0xcbf2_9ce4_8422_2325);
        assert_ne!(fingerprint("a"), fingerprint("b"));
    }
}
