// SPDX-License-Identifier: GPL-3.0-or-later

//! Attachments too large for mail, through the sender's Google Drive or
//! OneDrive (`docs/ARCHITECTURE.md` §6.6): Katna Mail asks for a file to go up as
//! soon as it is attached, shows the upload's progress, and at Send asks
//! to share the files with the recipients before the message with their
//! links goes out.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicI64, Ordering},
    },
};

use katna_core::{AccountId, OAuthProvider};
use katna_dbus::{CloudEntry, DriveUpload, drive_state};
use katna_sync::{
    Error, Result as SyncResult,
    drive::{Drive, DriveFile},
    net::Tls,
    onedrive::OneDrive,
};

use super::{CommandError, Daemon, Notice, cloud::Cloud};

/// The uploads of this run of the daemon.
#[derive(Default)]
pub(crate) struct Uploads {
    next: AtomicI64,
    all: Mutex<HashMap<i64, Upload>>,
}

struct Upload {
    status: DriveUpload,
    /// The file in Drive, once uploaded.
    file: Option<String>,
    /// Whether Katna put `file` there, so cancelling bins it; a file
    /// someone picked from their drive is only linked and stays.
    owned: bool,
    /// The upload under way; dropping it stops it.
    task: Option<smol::Task<()>>,
}

/// Where an account's large files go: its provider's own storage.
enum Storage {
    Google(Drive),
    Microsoft(OneDrive),
}

impl Storage {
    async fn allowed(&self) -> SyncResult<bool> {
        match self {
            Self::Google(drive) => drive.allowed().await,
            Self::Microsoft(onedrive) => onedrive.allowed().await,
        }
    }

    async fn upload(
        &self,
        path: &std::path::Path,
        name: &str,
        progress: &(dyn Fn(u64, u64) + Sync),
    ) -> SyncResult<DriveFile> {
        match self {
            Self::Google(drive) => drive.upload(path, name, mime_of(name), progress).await,
            Self::Microsoft(onedrive) => onedrive.upload(path, name, progress).await,
        }
    }

    async fn share(&self, id: &str, addresses: &[String]) -> SyncResult<Vec<String>> {
        match self {
            Self::Google(drive) => drive.share(id, addresses).await,
            Self::Microsoft(onedrive) => onedrive.share(id, addresses).await,
        }
    }

    /// Shares file `id` with anyone who has its link; returns a new link
    /// for it, when the provider gives one.
    async fn share_with_link(&self, id: &str) -> SyncResult<Option<String>> {
        match self {
            Self::Google(drive) => drive.share_with_link(id).await.map(|()| None),
            Self::Microsoft(onedrive) => onedrive.share_with_link(id).await.map(Some),
        }
    }

    async fn remove(&self, id: &str) -> SyncResult<()> {
        match self {
            Self::Google(drive) => drive.remove(id).await,
            Self::Microsoft(onedrive) => onedrive.remove(id).await,
        }
    }
}

impl Daemon {
    /// The Google Drive or OneDrive of `account`, which must sign in with
    /// Google or Microsoft.
    async fn drive(&self, account: AccountId) -> Result<Storage, CommandError> {
        let settings = self.store().account_settings(account)?.unwrap_or_default();
        let Some(provider) = settings.oauth else {
            return Err(CommandError::InvalidArgs(
                "only accounts signed in with Google or Microsoft keep large files".into(),
            ));
        };
        let tokens = self
            .oauth_tokens(account, provider)
            .await
            .map_err(CommandError::AuthFailed)?;
        let tls = Tls::system().map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        Ok(match provider {
            OAuthProvider::Google => Storage::Google(Drive::new(tokens, tls)),
            OAuthProvider::Microsoft => Storage::Microsoft(OneDrive::new(tokens, tls)),
            OAuthProvider::Zoho => {
                return Err(CommandError::InvalidArgs(
                    "Zoho accounts keep no large files".into(),
                ));
            }
        })
    }

    /// Starts uploading `path` to the Drive of `account`.
    pub async fn drive_upload(
        self: &Arc<Self>,
        account: AccountId,
        path: &str,
    ) -> Result<i64, CommandError> {
        let path = PathBuf::from(path);
        let size = std::fs::metadata(&path)
            .ok()
            .filter(|m| m.is_file())
            .ok_or_else(|| CommandError::InvalidArgs(format!("no file {}", path.display())))?
            .len();
        let drive = self.drive(account).await?;
        let name = path
            .file_name()
            .map_or_else(|| "file".to_owned(), |n| n.to_string_lossy().into_owned());
        let id = self.uploads.next.fetch_add(1, Ordering::Relaxed) + 1;
        let status = DriveUpload {
            id,
            account: account.0,
            name: name.clone(),
            sent: 0,
            size,
            state: drive_state::UPLOADING.into(),
            link: String::new(),
            error: String::new(),
        };
        let task = smol::spawn(upload(Arc::downgrade(self), drive, id, path, name));
        self.uploads.all.lock().unwrap().insert(
            id,
            Upload {
                status,
                file: None,
                owned: true,
                task: Some(task),
            },
        );
        tracing::info!(upload = id, %account, size, "uploading a large attachment");
        Ok(id)
    }

    /// Starts uploading file or folder `path` into folder `folder` (empty
    /// for the top of My Drive) of the Google Drive of `account`, for
    /// Files. A folder goes up with everything in it. Its progress comes
    /// as `DriveChanged`, as an attachment's does.
    pub async fn cloud_upload(
        self: &Arc<Self>,
        account: AccountId,
        folder: &str,
        path: &str,
    ) -> Result<i64, CommandError> {
        let path = PathBuf::from(path);
        let plan = smol::unblock({
            let path = path.clone();
            move || plan_upload(&path)
        })
        .await
        .ok_or_else(|| {
            CommandError::InvalidArgs(format!("nothing to upload at {}", path.display()))
        })?;
        let Some(drive) = self.cloud(account).await? else {
            return Err(CommandError::InvalidArgs(
                "only accounts signed in with Google or Microsoft have a drive in Files".into(),
            ));
        };
        let name = path
            .file_name()
            .map_or_else(|| "file".to_owned(), |n| n.to_string_lossy().into_owned());
        let id = self.uploads.next.fetch_add(1, Ordering::Relaxed) + 1;
        let status = DriveUpload {
            id,
            account: account.0,
            name,
            sent: 0,
            size: plan.size,
            state: drive_state::UPLOADING.into(),
            link: String::new(),
            error: String::new(),
        };
        let task = smol::spawn(upload_tree(
            Arc::downgrade(self),
            drive,
            id,
            folder.to_owned(),
            plan,
        ));
        self.uploads.all.lock().unwrap().insert(
            id,
            Upload {
                status,
                file: None,
                owned: true,
                task: Some(task),
            },
        );
        tracing::info!(upload = id, %account, "uploading into the drive");
        Ok(id)
    }

    /// Links file `entry`, already in the drive of `account`, to a
    /// message as a finished upload, so it is shared at Send as an
    /// uploaded file is. Cancelling it leaves the file where it is.
    pub fn cloud_link(&self, account: AccountId, entry: &CloudEntry) -> Result<i64, CommandError> {
        if entry.id.is_empty() || entry.folder {
            return Err(CommandError::InvalidArgs(
                "only a file can be linked".into(),
            ));
        }
        let id = self.uploads.next.fetch_add(1, Ordering::Relaxed) + 1;
        let status = DriveUpload {
            id,
            account: account.0,
            name: entry.name.clone(),
            sent: entry.size,
            size: entry.size,
            state: drive_state::DONE.into(),
            link: entry.link.clone(),
            error: String::new(),
        };
        self.uploads.all.lock().unwrap().insert(
            id,
            Upload {
                status,
                file: Some(entry.id.clone()),
                owned: false,
                task: None,
            },
        );
        Ok(id)
    }

    pub fn drive_upload_status(&self, id: i64) -> Result<DriveUpload, CommandError> {
        self.uploads
            .all
            .lock()
            .unwrap()
            .get(&id)
            .map(|u| u.status.clone())
            .ok_or_else(|| CommandError::InvalidArgs(format!("no upload {id}")))
    }

    /// Stops upload `id` and moves the file it put in the drive to the
    /// bin; a linked file stays.
    pub async fn drive_cancel(&self, id: i64) -> Result<bool, CommandError> {
        let Some(upload) = self.uploads.all.lock().unwrap().remove(&id) else {
            return Ok(false);
        };
        drop(upload.task);
        if let Some(file) = upload.file.filter(|_| upload.owned) {
            let drive = self.drive(AccountId(upload.status.account)).await?;
            if let Err(err) = drive.remove(&file).await {
                tracing::warn!(upload = id, %err, "could not remove the uploaded file");
            }
        }
        Ok(true)
    }

    /// The account and Drive file of each finished upload in `ids`.
    fn uploaded(&self, ids: &[i64]) -> Result<Vec<(AccountId, String)>, CommandError> {
        let all = self.uploads.all.lock().unwrap();
        ids.iter()
            .map(|id| {
                let upload = all
                    .get(id)
                    .ok_or_else(|| CommandError::InvalidArgs(format!("no upload {id}")))?;
                let file = upload
                    .file
                    .clone()
                    .ok_or_else(|| CommandError::InvalidArgs(format!("upload {id} is not done")))?;
                Ok((AccountId(upload.status.account), file))
            })
            .collect()
    }

    /// Shares the files of `ids` with `addresses`; returns the addresses
    /// Drive would not share with.
    pub async fn drive_share(
        &self,
        ids: &[i64],
        addresses: &[String],
    ) -> Result<Vec<String>, CommandError> {
        let mut refused: Vec<String> = Vec::new();
        for (account, file) in self.uploaded(ids)? {
            let drive = self.drive(account).await?;
            for address in drive.share(&file, addresses).await.map_err(failed)? {
                if !refused.iter().any(|a| a.eq_ignore_ascii_case(&address)) {
                    refused.push(address);
                }
            }
        }
        Ok(refused)
    }

    /// Lets anyone with the link view the files of `ids`; returns the
    /// links for the message, in order.
    pub async fn drive_share_with_link(&self, ids: &[i64]) -> Result<Vec<String>, CommandError> {
        let mut links = Vec::new();
        for (id, (account, file)) in ids.iter().zip(self.uploaded(ids)?) {
            let drive = self.drive(account).await?;
            let new = drive.share_with_link(&file).await.map_err(failed)?;
            let mut all = self.uploads.all.lock().unwrap();
            let upload = all
                .get_mut(id)
                .ok_or_else(|| CommandError::InvalidArgs(format!("no upload {id}")))?;
            if let Some(new) = new {
                upload.status.link = new;
            }
            links.push(upload.status.link.clone());
        }
        Ok(links)
    }

    /// Changes upload `id` with `change` and tells Katna Mail.
    fn update_upload(&self, id: i64, change: impl FnOnce(&mut Upload)) {
        if let Some(upload) = self.uploads.all.lock().unwrap().get_mut(&id) {
            change(upload);
        } else {
            return;
        }
        let _ = self.notices.try_send(Notice::DriveChanged(id));
    }
}

fn failed(err: Error) -> CommandError {
    match err {
        Error::Auth(message) => CommandError::AuthFailed(message),
        err => CommandError::Failed(err.to_string()),
    }
}

/// Uploads `path`, keeping upload `id` up to date.
async fn upload(daemon: Weak<Daemon>, drive: Storage, id: i64, path: PathBuf, name: String) {
    let update = |change: &dyn Fn(&mut Upload)| {
        if let Some(daemon) = daemon.upgrade() {
            daemon.update_upload(id, change);
        }
    };
    let allowed = drive.allowed().await;
    if let Ok(false) | Err(Error::Auth(_)) = allowed {
        tracing::info!(upload = id, "the sign-in did not allow Drive or OneDrive");
        update(&|u| u.status.state = drive_state::NEEDS_PERMISSION.into());
        return;
    }
    // Katna Mail hears of every percent, not of every piece.
    let shown = Mutex::new(u64::MAX);
    let progress = |sent: u64, size: u64| {
        let percent = (sent * 100).checked_div(size).unwrap_or(100);
        if std::mem::replace(&mut *shown.lock().unwrap(), percent) != percent {
            update(&|u| u.status.sent = sent);
        }
    };
    match drive.upload(&path, &name, &progress).await {
        Ok(file) => {
            tracing::info!(upload = id, "uploaded");
            update(&|u| {
                u.status.sent = u.status.size;
                u.status.state = drive_state::DONE.into();
                u.status.link = file.link.clone();
                u.file = Some(file.id.clone());
            });
        }
        Err(Error::Auth(message)) => {
            tracing::info!(upload = id, %message, "Drive or OneDrive refused access");
            update(&|u| {
                u.status.state = drive_state::NEEDS_PERMISSION.into();
            });
        }
        Err(err) => {
            tracing::warn!(upload = id, %err, "upload failed");
            // Switched off in Katna's Google Cloud project: the app names
            // the API and offers the page that turns it on.
            let message = match err {
                Error::NotEnabled(detail) => detail,
                err => err.to_string(),
            };
            update(&|u| {
                u.status.state = drive_state::FAILED.into();
                u.status.error = message.clone();
            });
        }
    }
}

/// The most files one folder upload takes.
const MAX_TREE_FILES: usize = 10_000;

/// What goes up for a file or folder: the folders to make, parents
/// first, and the files, each with the place of its folder in `folders`
/// (`None` for the folder the upload goes into).
#[derive(Debug, Default)]
struct Plan {
    /// Each folder's name and the place of its own folder.
    folders: Vec<(String, Option<usize>)>,
    files: Vec<(PathBuf, String, Option<usize>)>,
    size: u64,
}

/// Lists what uploading `path` sends: a file, or a folder with its
/// files and folders (links and other odd entries left out). `None`
/// when there is nothing there or too much.
fn plan_upload(path: &std::path::Path) -> Option<Plan> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    let name = |p: &std::path::Path| {
        p.file_name()
            .map_or_else(|| "file".to_owned(), |n| n.to_string_lossy().into_owned())
    };
    let mut plan = Plan::default();
    if meta.is_file() {
        plan.size = meta.len();
        plan.files.push((path.to_owned(), name(path), None));
        return Some(plan);
    }
    if !meta.is_dir() {
        return None;
    }
    plan.folders.push((name(path), None));
    let mut pending = vec![(path.to_owned(), 0)];
    while let Some((dir, at)) = pending.pop() {
        let mut entries: Vec<_> = std::fs::read_dir(&dir).ok()?.flatten().collect();
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            let path = entry.path();
            if kind.is_dir() {
                plan.folders.push((name(&path), Some(at)));
                pending.push((path, plan.folders.len() - 1));
            } else if kind.is_file() {
                plan.size += entry.metadata().map_or(0, |m| m.len());
                plan.files.push((path.clone(), name(&path), Some(at)));
                if plan.files.len() > MAX_TREE_FILES {
                    return None;
                }
            }
        }
    }
    Some(plan)
}

/// Uploads what `plan` lists into folder `into`, keeping upload `id` up
/// to date with the bytes sent of all its files.
async fn upload_tree(daemon: Weak<Daemon>, drive: Cloud, id: i64, into: String, plan: Plan) {
    let update = |change: &dyn Fn(&mut Upload)| {
        if let Some(daemon) = daemon.upgrade() {
            daemon.update_upload(id, change);
        }
    };
    let fail = |err: Error| {
        if let Error::Auth(message) = &err {
            tracing::info!(upload = id, %message, "Drive refused access");
            update(&|u| u.status.state = drive_state::NEEDS_PERMISSION.into());
            return;
        }
        tracing::warn!(upload = id, %err, "upload into the drive failed");
        let message = match err {
            Error::NotEnabled(detail) => detail,
            err => err.to_string(),
        };
        update(&|u| {
            u.status.state = drive_state::FAILED.into();
            u.status.error = message.clone();
        });
    };
    match drive.writable().await {
        Ok(true) => {}
        Ok(false) | Err(Error::Auth(_)) => {
            tracing::info!(
                upload = id,
                "the sign-in did not allow uploads into the drive"
            );
            update(&|u| u.status.state = drive_state::NEEDS_PERMISSION.into());
            return;
        }
        Err(err) => return fail(err),
    }
    let mut made: Vec<String> = Vec::with_capacity(plan.folders.len());
    for (name, parent) in &plan.folders {
        let parent = parent.map_or(into.as_str(), |at| made[at].as_str());
        match drive.create_folder(name, parent).await {
            Ok(folder) => made.push(folder),
            Err(err) => return fail(err),
        }
    }
    let shown = Mutex::new(u64::MAX);
    let total = plan.size;
    let mut done = 0;
    for (path, name, parent) in &plan.files {
        let parent = parent.map_or(into.as_str(), |at| made[at].as_str());
        let base = done;
        let progress = |sent: u64, _size: u64| {
            let sent = (base + sent).min(total);
            let percent = (sent * 100).checked_div(total).unwrap_or(100);
            if std::mem::replace(&mut *shown.lock().unwrap(), percent) != percent {
                update(&|u| u.status.sent = sent);
            }
        };
        match drive.upload_into(path, name, parent, &progress).await {
            Ok(file) => {
                done += std::fs::metadata(path).map_or(0, |m| m.len());
                if plan.folders.is_empty() {
                    update(&|u| {
                        u.status.link = file.link.clone();
                        u.file = Some(file.id.clone());
                    });
                }
            }
            // A file that went away since it was chosen is left out.
            Err(Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return fail(err),
        }
    }
    tracing::info!(
        upload = id,
        files = plan.files.len(),
        "uploaded into the drive"
    );
    update(&|u| {
        u.status.sent = u.status.size;
        u.status.state = drive_state::DONE.into();
    });
}

/// The type Drive files a file under, from its name; Drive works out
/// the rest itself.
pub(super) fn mime_of(name: &str) -> &'static str {
    let ext = name
        .rsplit_once('.')
        .map(|(_, e)| e.to_ascii_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "mp4" => "video/mp4",
        "mov" => "video/quicktime",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_plans_its_folders_before_their_files() {
        let dir = tempfile::tempdir().unwrap();
        let top = dir.path().join("Trip");
        std::fs::create_dir_all(top.join("Day 1")).unwrap();
        std::fs::write(top.join("plan.txt"), b"abc").unwrap();
        std::fs::write(top.join("Day 1").join("a.jpg"), b"12345").unwrap();
        let plan = plan_upload(&top).unwrap();
        assert_eq!(
            plan.folders,
            [("Trip".to_owned(), None), ("Day 1".to_owned(), Some(0))]
        );
        let files: Vec<_> = plan
            .files
            .iter()
            .map(|(_, name, at)| (name.as_str(), *at))
            .collect();
        assert_eq!(files, [("plan.txt", Some(0)), ("a.jpg", Some(1))]);
        assert_eq!(plan.size, 8);
        let one = plan_upload(&top.join("plan.txt")).unwrap();
        assert!(one.folders.is_empty());
        assert_eq!(one.files[0].2, None);
        assert!(plan_upload(&top.join("missing")).is_none());
    }
}
