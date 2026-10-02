// SPDX-License-Identifier: GPL-3.0-or-later

//! What the cloud drives have in common for Files (`docs/ARCHITECTURE.md`
//! §13.8): a folder's items, a page at a time, the same for Google Drive
//! ([`crate::drive`]) and OneDrive ([`crate::onedrive`]).

/// Where in a drive to look.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Place {
    /// A folder by its id; [`ROOT`] is the top of the drive.
    Folder(String),
    /// What other people shared with the account.
    Shared,
    /// Files and folders whose name or text has these words, anywhere in
    /// the drive.
    Search(String),
}

/// The top folder of a drive.
pub const ROOT: &str = "root";

/// A file or folder in a drive.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CloudItem {
    pub id: String,
    pub name: String,
    /// The file's type; empty for a folder.
    pub mime: String,
    /// Bytes; 0 for folders and for a drive's own documents, which have
    /// no size of their own.
    pub size: u64,
    /// When it last changed, as Unix seconds.
    pub modified: Option<i64>,
    pub folder: bool,
    /// One of the drive's own documents (a Google Doc, Sheet or Slides),
    /// which only opens as a copy in another format.
    pub native: bool,
    /// Where people open it in the browser.
    pub link: String,
    /// A small picture of it, fetched with the account's token; empty
    /// when the drive has none.
    pub thumbnail: String,
}

/// One page of a listing; `next` asks for the page after it and is empty
/// on the last.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CloudPage {
    pub items: Vec<CloudItem>,
    pub next: String,
}

/// A downloaded file: its name and type, which differ from the item's for
/// a drive's own documents (they come as PDF).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fetched {
    pub name: String,
    pub mime: String,
    pub size: u64,
}

/// The biggest file Katna fetches for the viewer or a download.
pub const MAX_FETCH: u64 = 2 * 1024 * 1024 * 1024;

/// The biggest thumbnail it reads.
pub const MAX_THUMBNAIL: usize = 4 * 1024 * 1024;

/// What someone may do with a shared item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Owner,
    Editor,
    /// Views and comments; Google Drive only.
    Commenter,
    Viewer,
}

impl Role {
    /// Its name over D-Bus.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Editor => "editor",
            Self::Commenter => "commenter",
            Self::Viewer => "viewer",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "owner" => Self::Owner,
            "editor" => Self::Editor,
            "commenter" => Self::Commenter,
            "viewer" => Self::Viewer,
            _ => return None,
        })
    }
}

/// Who a grant of access is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Who {
    Person,
    Group,
    /// Everyone in an organisation.
    Domain,
    /// Anyone with the link.
    Anyone,
}

impl Who {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Person => "person",
            Self::Group => "group",
            Self::Domain => "domain",
            Self::Anyone => "anyone",
        }
    }
}

/// One grant of access to an item: who, and what they may do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Access {
    /// The drive's id for the grant, to change or take it away.
    pub id: String,
    pub who: Who,
    pub role: Role,
    /// The person's or group's address, or the organisation's domain.
    pub address: String,
    pub name: String,
    /// Given on a folder above; changed there, not here.
    pub inherited: bool,
    /// The link it opens with, for anyone with the link.
    pub link: String,
}
