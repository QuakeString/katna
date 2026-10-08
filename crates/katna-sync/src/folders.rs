// SPDX-License-Identifier: GPL-3.0-or-later

//! Renaming and deleting folders (labels, on Gmail) on the server, then in
//! the store. Unlike the changes in [`crate::ops`] these are not queued:
//! they need the server, and fail while it cannot be reached.
//!
//! - [`rename_folder`] changes only the last part of a folder's path; the
//!   folders inside it move along (IMAP RENAME does that on the server).
//! - [`delete_folder`] deletes a folder and the folders inside it, deepest
//!   first. Elsewhere than on Gmail their mail goes to the Trash first, so
//!   it can still be recovered; on Gmail deleting a label only takes the
//!   label off, and the mail stays in All Mail and its other labels.
//!
//! Special folders (Inbox, Sent, Drafts, Trash, Junk, Archive, All Mail,
//! Gmail's system labels, Notes) keep their names and cannot be deleted.

use std::cmp::Reverse;

use katna_core::AccountId;
use katna_store::{FolderId, FolderRole, Store, StoredFolder};

use crate::notes::NOTES_FOLDER;
use crate::ops::{self, ChangeError};
use crate::{Folder, MailBackend};

/// Where Gmail keeps its system labels.
const GMAIL_ROOTS: [&str; 2] = ["[Gmail]", "[Google Mail]"];

/// UIDs moved to the Trash per command.
const MOVE_CHUNK: usize = 1000;

/// Why a folder could not be renamed or deleted.
#[derive(Debug, thiserror::Error)]
pub enum FolderError {
    /// The request itself is wrong: a special folder, a taken name, ….
    #[error("{0}")]
    Invalid(String),
    #[error("no folder {0}")]
    UnknownFolder(i64),
    /// The server refused or could not be reached, or a queued change
    /// still needs the folder.
    #[error("{0}")]
    Failed(String),
    #[error(transparent)]
    Store(#[from] katna_store::Error),
}

impl From<ChangeError> for FolderError {
    fn from(err: ChangeError) -> Self {
        match err {
            ChangeError::Store(err) => Self::Store(err),
            ChangeError::UnknownFolder(id) => Self::UnknownFolder(id),
            ChangeError::Invalid(reason) => Self::Invalid(reason),
            other => Self::Failed(other.to_string()),
        }
    }
}

/// Whether the account of `folders` is a Gmail account, whose folders are
/// labels.
pub fn is_gmail(folders: &[StoredFolder]) -> bool {
    folders.iter().any(|f| is_gmail_system(&f.path))
}

/// Whether `path` is Gmail's system label root or a label under it.
fn is_gmail_system(path: &str) -> bool {
    GMAIL_ROOTS.iter().any(|root| {
        path.strip_prefix(root)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
    })
}

/// Whether `folder` is one of its account's special folders, which keep
/// their names: one with a role, the inbox, Gmail's system labels, the
/// notes folder, and, on servers that do not mark them, a top-level folder
/// (or one right inside INBOX) named like a special one ("Sent Items").
/// `delimiter` is the server's hierarchy separator, if known.
pub fn is_special(folder: &StoredFolder, delimiter: Option<char>) -> bool {
    let path = folder.path.as_str();
    if folder.role.is_some()
        || path.eq_ignore_ascii_case("INBOX")
        || is_gmail_system(path)
        || path == NOTES_FOLDER
    {
        return true;
    }
    let name = match delimiter.and_then(|d| path.rsplit_once(d)) {
        None => path,
        Some((parent, name)) if parent.eq_ignore_ascii_case("INBOX") => name,
        Some(_) => return false,
    };
    FolderRole::from_name(name).is_some()
}

/// The account of `folder` and the folder itself, refusing special ones
/// before the server is asked anything.
pub fn editable(store: &Store, folder: FolderId) -> Result<(AccountId, StoredFolder), FolderError> {
    let account = store
        .folder_account(folder)?
        .ok_or(FolderError::UnknownFolder(folder.0))?;
    let found = store
        .folders(account)?
        .into_iter()
        .find(|f| f.id == folder)
        .ok_or(FolderError::UnknownFolder(folder.0))?;
    if is_special(&found, None) {
        return Err(special(&found));
    }
    Ok((account, found))
}

fn special(folder: &StoredFolder) -> FolderError {
    FolderError::Invalid(format!(
        "\u{201c}{}\u{201d} is a special folder; it cannot be renamed or deleted",
        folder.path
    ))
}

fn failed(what: &str) -> impl Fn(crate::Error) -> FolderError + '_ {
    move |err| FolderError::Failed(format!("the mail server did not {what}: {err}"))
}

/// Whether `path` is `parent` or inside it.
fn inside(path: &str, parent: &str, delimiter: Option<char>) -> bool {
    path == parent
        || delimiter.is_some_and(|d| {
            path.strip_prefix(parent)
                .is_some_and(|rest| rest.starts_with(d))
        })
}

/// The server's hierarchy separator for `path`: its own, else any.
fn delimiter_of(listed: &[Folder], path: &str) -> Option<char> {
    listed
        .iter()
        .find(|f| f.name == path)
        .or_else(|| listed.iter().find(|f| f.delimiter.is_some()))
        .and_then(|f| f.delimiter)
}

/// The stored folder `folder` and those inside it, refused when any is
/// special or a queued change still needs one.
fn tree(
    store: &Store,
    account: AccountId,
    target: &StoredFolder,
    delimiter: Option<char>,
) -> Result<Vec<StoredFolder>, FolderError> {
    let tree: Vec<StoredFolder> = store
        .folders(account)?
        .into_iter()
        .filter(|f| inside(&f.path, &target.path, delimiter))
        .collect();
    if let Some(special_one) = tree.iter().find(|f| is_special(f, delimiter)) {
        return Err(if special_one.id == target.id {
            special(target)
        } else {
            FolderError::Invalid(format!(
                "\u{201c}{}\u{201d} holds the special folder \u{201c}{}\u{201d}",
                target.path, special_one.path
            ))
        });
    }
    let ids: Vec<FolderId> = tree.iter().map(|f| f.id).collect();
    let paths: Vec<String> = tree.iter().map(|f| f.path.clone()).collect();
    if ops::waits_on_folders(store, account, &ids, &paths)? {
        return Err(FolderError::Failed(
            "changes to this folder are still on their way to the server; try again in a \
             moment"
                .to_owned(),
        ));
    }
    Ok(tree)
}

/// Renames `folder` to `name` on the server, keeping it where it is, then
/// in the store, folders inside it included. Returns its new path.
pub async fn rename_folder<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    folder: FolderId,
    name: &str,
) -> Result<String, FolderError> {
    let (account, target) = editable(store, folder)?;
    let listed = backend
        .list_folders()
        .await
        .map_err(failed("list folders"))?;
    let delimiter = delimiter_of(&listed, &target.path);
    if let Some(d) = delimiter
        && name.contains(d)
    {
        return Err(FolderError::Invalid(format!(
            "a name cannot contain \u{201c}{d}\u{201d}"
        )));
    }
    let new_path = match delimiter.and_then(|d| target.path.rfind(d).map(|at| at + d.len_utf8())) {
        Some(end) => format!("{}{name}", &target.path[..end]),
        None => name.to_owned(),
    };
    if new_path == target.path {
        return Ok(new_path);
    }
    tree(store, account, &target, delimiter)?;
    if !listed.iter().any(|f| f.name == target.path) {
        return Err(FolderError::Failed(format!(
            "\u{201c}{}\u{201d} is not on the mail server any more",
            target.path
        )));
    }
    // Gmail and most servers ignore case in names.
    if let Some(taken) = listed
        .iter()
        .find(|f| f.name != target.path && f.name.to_lowercase() == new_path.to_lowercase())
    {
        return Err(FolderError::Invalid(format!(
            "\u{201c}{}\u{201d} already exists",
            taken.name
        )));
    }
    backend
        .rename_folder(&target.path, &new_path)
        .await
        .map_err(failed("rename it"))?;
    let mut batch = store.mail_batch()?;
    batch.rename_folder_tree(account, &target.path, &new_path, delimiter)?;
    batch.commit()?;
    tracing::info!(%account, from = target.path, to = new_path, "folder renamed");
    Ok(new_path)
}

/// Deletes `folder` and the folders inside it on the server, deepest
/// first, then in the store. Elsewhere than on Gmail their messages are
/// moved to the account's Trash first, when it has one; on Gmail the
/// labels go and the messages stay in All Mail and their other labels.
/// Returns how many messages went to the Trash.
pub async fn delete_folder<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    folder: FolderId,
) -> Result<u32, FolderError> {
    let (account, target) = editable(store, folder)?;
    let listed = backend
        .list_folders()
        .await
        .map_err(failed("list folders"))?;
    let delimiter = delimiter_of(&listed, &target.path);
    let stored = tree(store, account, &target, delimiter)?;
    let mut on_server: Vec<&Folder> = listed
        .iter()
        .filter(|f| inside(&f.name, &target.path, delimiter))
        .collect();
    if let Some(marked) = on_server.iter().find(|f| f.role.is_some()) {
        return Err(FolderError::Invalid(format!(
            "\u{201c}{}\u{201d} holds the special folder \u{201c}{}\u{201d}",
            target.path, marked.name
        )));
    }
    // Inside before outside.
    on_server.sort_by_key(|f| Reverse(f.name.len()));

    let all = store.folders(account)?;
    let gmail = is_gmail(&all) || listed.iter().any(|f| is_gmail_system(&f.name));
    let trash = match gmail {
        true => None,
        false => store
            .trash_folder(account)?
            .and_then(|id| all.into_iter().find(|f| f.id == id))
            .filter(|trash| listed.iter().any(|f| f.name == trash.path)),
    };
    let mut moved = 0usize;
    if let Some(trash) = &trash {
        for folder in on_server.iter().filter(|f| f.selectable) {
            backend
                .select(&folder.name)
                .await
                .map_err(failed("open it"))?;
            let uids = backend.uids().await.map_err(failed("list its mail"))?;
            let known = match stored.iter().find(|f| f.path == folder.name) {
                Some(here) => store.folder_message_uids(here.id)?,
                None => Vec::new(),
            };
            for chunk in uids.chunks(MOVE_CHUNK) {
                let pairs = backend
                    .move_messages(chunk, &trash.path)
                    .await
                    .map_err(failed("move its mail to the Trash"))?;
                moved += chunk.len();
                // What the server reported keeps its body here; the rest
                // comes with the next sync of the Trash.
                let mut batch = store.mail_batch()?;
                for (old, new) in pairs {
                    if let (Some(here), Some((_, message))) = (
                        stored.iter().find(|f| f.path == folder.name),
                        known.iter().find(|(uid, _)| *uid == old),
                    ) {
                        batch.move_location(*message, here.id, trash.id, Some(new))?;
                    }
                }
                batch.commit()?;
            }
        }
        if moved > 0 {
            // Not left inside a folder about to go.
            backend
                .select(&trash.path)
                .await
                .map_err(failed("open the Trash"))?;
        }
    }
    for folder in &on_server {
        backend
            .delete_folder(&folder.name)
            .await
            .map_err(failed("delete it"))?;
    }
    let mut batch = store.mail_batch()?;
    for folder in &stored {
        batch.remove_folder(folder.id)?;
    }
    batch.commit()?;
    tracing::info!(%account, path = target.path, moved, gmail, "folder deleted");
    Ok(u32::try_from(moved).unwrap_or(u32::MAX))
}
