// SPDX-License-Identifier: GPL-3.0-or-later

//! Attachments too large for mail, through the sender's Google Drive
//! (`docs/ARCHITECTURE.md` §6.6): Katna Mail asks for a file to go up as
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
use katna_dbus::{DriveUpload, drive_state};
use katna_sync::{Error, drive::Drive, net::Tls};

use super::{CommandError, Daemon, Notice};

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
    /// The upload under way; dropping it stops it.
    task: Option<smol::Task<()>>,
}

impl Daemon {
    /// The Drive of `account`, which must sign in with Google.
    async fn drive(&self, account: AccountId) -> Result<Drive, CommandError> {
        let settings = self.store().account_settings(account)?.unwrap_or_default();
        if settings.oauth != Some(OAuthProvider::Google) {
            return Err(CommandError::InvalidArgs(
                "only accounts signed in with Google have a Google Drive".into(),
            ));
        }
        let tokens = self
            .oauth_tokens(account, OAuthProvider::Google)
            .await
            .map_err(CommandError::AuthFailed)?;
        let tls = Tls::system().map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        Ok(Drive::new(tokens, tls))
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
                task: Some(task),
            },
        );
        tracing::info!(upload = id, %account, size, "uploading to Google Drive");
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

    /// Stops upload `id` and moves its file to the bin.
    pub async fn drive_cancel(&self, id: i64) -> Result<bool, CommandError> {
        let Some(upload) = self.uploads.all.lock().unwrap().remove(&id) else {
            return Ok(false);
        };
        drop(upload.task);
        if let Some(file) = upload.file {
            let drive = self.drive(AccountId(upload.status.account)).await?;
            if let Err(err) = drive.remove(&file).await {
                tracing::warn!(upload = id, %err, "could not remove the file from Drive");
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

    /// Lets anyone with the link view the files of `ids`.
    pub async fn drive_share_with_link(&self, ids: &[i64]) -> Result<(), CommandError> {
        for (account, file) in self.uploaded(ids)? {
            let drive = self.drive(account).await?;
            drive.share_with_link(&file).await.map_err(failed)?;
        }
        Ok(())
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
async fn upload(daemon: Weak<Daemon>, drive: Drive, id: i64, path: PathBuf, name: String) {
    let update = |change: &dyn Fn(&mut Upload)| {
        if let Some(daemon) = daemon.upgrade() {
            daemon.update_upload(id, change);
        }
    };
    let allowed = drive.allowed().await;
    if let Ok(false) | Err(Error::Auth(_)) = allowed {
        tracing::info!(upload = id, "the sign-in did not allow Google Drive");
        update(&|u| u.status.state = drive_state::NEEDS_PERMISSION.into());
        return;
    }
    let mime = mime_of(&name);
    // Katna Mail hears of every percent, not of every piece.
    let shown = Mutex::new(u64::MAX);
    let progress = |sent: u64, size: u64| {
        let percent = (sent * 100).checked_div(size).unwrap_or(100);
        if std::mem::replace(&mut *shown.lock().unwrap(), percent) != percent {
            update(&|u| u.status.sent = sent);
        }
    };
    match drive.upload(&path, &name, mime, &progress).await {
        Ok(file) => {
            tracing::info!(upload = id, "uploaded to Google Drive");
            update(&|u| {
                u.status.sent = u.status.size;
                u.status.state = drive_state::DONE.into();
                u.status.link = file.link.clone();
                u.file = Some(file.id.clone());
            });
        }
        Err(Error::Auth(message)) => {
            tracing::info!(upload = id, %message, "Google Drive refused access");
            update(&|u| {
                u.status.state = drive_state::NEEDS_PERMISSION.into();
            });
        }
        Err(err) => {
            tracing::warn!(upload = id, %err, "upload to Google Drive failed");
            let message = err.to_string();
            update(&|u| {
                u.status.state = drive_state::FAILED.into();
                u.status.error = message.clone();
            });
        }
    }
}

/// The type Drive files a file under, from its name; Drive works out
/// the rest itself.
fn mime_of(name: &str) -> &'static str {
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
