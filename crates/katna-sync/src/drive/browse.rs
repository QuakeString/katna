// SPDX-License-Identifier: GPL-3.0-or-later

//! Browsing a whole Google Drive for Files (`docs/ARCHITECTURE.md`
//! §13.8): folders, what was shared with the account, search, thumbnails
//! downloads, new folders and uploads into any folder. Needs
//! [`GOOGLE_DRIVE`] (or, to browse only, [`GOOGLE_DRIVE_READ`]), which
//! accounts signed in before Katna asked for it don't have yet.

use std::path::Path;

use katna_core::wildcard;
use serde::Deserialize;

use super::{Drive, check, failure};
use crate::{
    Error, Result,
    autoconfig::http,
    cloud::{CloudItem, CloudPage, Fetched, MAX_FETCH, MAX_THUMBNAIL, Place},
    oauth::{GOOGLE_DRIVE, GOOGLE_DRIVE_READ},
};

const FOLDER: &str = "application/vnd.google-apps.folder";

/// Google's own documents, which download as PDF.
const NATIVE: [&str; 4] = [
    "application/vnd.google-apps.document",
    "application/vnd.google-apps.spreadsheet",
    "application/vnd.google-apps.presentation",
    "application/vnd.google-apps.drawing",
];

/// What one page lists; a folder rarely has more.
const PAGE: usize = 200;

/// The biggest listing page read, at about 1 KB an item.
const MAX_LISTING: usize = 2 * 1024 * 1024;

/// The biggest PDF Google makes of one of its documents.
const MAX_EXPORT: usize = 10 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Listing {
    #[serde(default)]
    files: Vec<File>,
    #[serde(default)]
    next_page_token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct File {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    mime_type: String,
    /// A string of digits, absent for folders and Google's documents.
    #[serde(default)]
    size: Option<String>,
    #[serde(default)]
    modified_time: Option<String>,
    #[serde(default)]
    thumbnail_link: String,
    #[serde(default)]
    web_view_link: String,
}

impl File {
    /// The item, or `None` for what Files can't show: shortcuts, forms,
    /// sites and the like.
    fn item(self) -> Option<CloudItem> {
        let folder = self.mime_type == FOLDER;
        let native = NATIVE.contains(&self.mime_type.as_str());
        if self.mime_type.starts_with("application/vnd.google-apps.") && !folder && !native {
            return None;
        }
        Some(CloudItem {
            size: self.size.and_then(|s| s.parse().ok()).unwrap_or(0),
            modified: self
                .modified_time
                .and_then(|t| t.parse::<jiff::Timestamp>().ok())
                .map(|t| t.as_second()),
            mime: if folder {
                String::new()
            } else {
                self.mime_type
            },
            id: self.id,
            name: self.name,
            folder,
            native,
            link: self.web_view_link,
            thumbnail: self.thumbnail_link,
        })
    }
}

/// `text` inside single quotes in a Drive query.
fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\\', "\\\\").replace('\'', "\\'"))
}

/// The query of a search for `words`. Drive can't take wildcards, so a
/// pattern (`*.pdf`, `invoice*2026*`) asks for its plain pieces, or a
/// common extension's type, and [`crate::cloud::keep_matching`] keeps what fits.
fn search_query(words: &str) -> String {
    let (patterns, plain): (Vec<&str>, Vec<&str>) = words
        .split_whitespace()
        .partition(|w| wildcard::is_pattern(w));
    let mut clauses = Vec::new();
    if !plain.is_empty() {
        clauses.push(format!(
            "(name contains {0} or fullText contains {0})",
            quoted(&plain.join(" "))
        ));
    }
    for pattern in patterns {
        if let Some(mime) = wildcard::extension(pattern).and_then(extension_mime) {
            clauses.push(format!("mimeType = {}", quoted(mime)));
            continue;
        }
        for piece in wildcard::pieces(pattern) {
            clauses.push(format!(
                "(name contains {0} or fullText contains {0})",
                quoted(piece)
            ));
        }
    }
    clauses.push("trashed = false".into());
    clauses.join(" and ")
}

/// The type Drive gives files of a common extension.
fn extension_mime(ext: &str) -> Option<&'static str> {
    Some(match ext.to_ascii_lowercase().as_str() {
        "pdf" => "application/pdf",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "zip" => "application/zip",
        "mp4" => "video/mp4",
        "doc" => "application/msword",
        "xls" => "application/vnd.ms-excel",
        "ppt" => "application/vnd.ms-powerpoint",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        _ => return None,
    })
}

impl Drive {
    /// Whether the account's sign-in lets Katna browse the whole Drive.
    pub async fn readable(&self) -> Result<bool> {
        Ok(self.writable().await? || self.tokens.has_scope(GOOGLE_DRIVE_READ).await?)
    }

    /// Whether the account's sign-in lets Katna put files in any folder.
    pub async fn writable(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_DRIVE).await
    }

    /// Makes folder `name` in folder `parent` (empty for the top of My
    /// Drive) and returns its id.
    pub async fn create_folder(&self, name: &str, parent: &str) -> Result<String> {
        let mut metadata = serde_json::json!({ "name": name, "mimeType": FOLDER });
        if !parent.is_empty() {
            metadata["parents"] = serde_json::json!([parent]);
        }
        let body = metadata.to_string();
        let url = format!("{}/drive/v3/files?fields=id", self.api);
        let reply = self
            .call(
                "POST",
                &url,
                &[],
                Some(("application/json; charset=UTF-8", body.as_bytes())),
                None,
            )
            .await?;
        check(&reply, "making a folder")?;
        Ok(super::parse(&reply.body)?.id)
    }

    /// One page of `place`; `page` is the last page's `next`, or empty
    /// for the first. Folders come first, then the newest files.
    pub async fn list(&self, place: &Place, page: &str) -> Result<CloudPage> {
        let (query, order) = match place {
            Place::Folder(id) => (
                format!("{} in parents and trashed = false", quoted(id)),
                "folder,modifiedTime desc",
            ),
            Place::Shared => (
                "sharedWithMe and trashed = false".to_owned(),
                "folder,modifiedTime desc",
            ),
            // Google won't sort a search of the files' text.
            Place::Search(words) => (search_query(words), ""),
        };
        let mut url = format!(
            "{}/drive/v3/files?q={}&pageSize={PAGE}&fields={}",
            self.api,
            http::escape(&query),
            http::escape(
                "nextPageToken,files(id,name,mimeType,size,modifiedTime,thumbnailLink,webViewLink)"
            ),
        );
        if !order.is_empty() {
            url.push_str(&format!("&orderBy={}", http::escape(order)));
        }
        if !page.is_empty() {
            url.push_str(&format!("&pageToken={}", http::escape(page)));
        }
        let reply = self.call_limited("GET", &url, MAX_LISTING).await?;
        check(&reply, "listing files")?;
        let listing: Listing = serde_json::from_slice(&reply.body)
            .map_err(|err| Error::Protocol(format!("Drive listing: {err}")))?;
        let mut items: Vec<CloudItem> = listing.files.into_iter().filter_map(File::item).collect();
        if let Place::Search(words) = place {
            crate::cloud::keep_matching(&mut items, words);
        }
        Ok(CloudPage {
            items,
            next: listing.next_page_token,
        })
    }

    /// Downloads `item` into `to`: as it is, or as a PDF for one of
    /// Google's own documents. Returns what was saved.
    pub async fn fetch(&self, item: &CloudItem, to: &Path) -> Result<Fetched> {
        let id = http::escape(&item.id);
        if item.native {
            let url = format!(
                "{}/drive/v3/files/{id}/export?mimeType=application%2Fpdf",
                self.api
            );
            let reply = self.call_limited("GET", &url, MAX_EXPORT).await?;
            check(&reply, "making a PDF")?;
            std::fs::write(to, &reply.body)?;
            return Ok(Fetched {
                name: format!("{}.pdf", item.name),
                mime: "application/pdf".into(),
                size: reply.body.len() as u64,
            });
        }
        let url = format!("{}/drive/v3/files/{id}?alt=media", self.api);
        let size = loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let mut file = std::fs::File::create(to)?;
            let mut sink = |piece: &[u8], _total: u64| std::io::Write::write_all(&mut file, piece);
            match http::download_with(
                &url,
                &[("Authorization", &token)],
                &self.tls,
                MAX_FETCH,
                &mut sink,
            )
            .await?
            {
                Ok(size) => break size,
                Err(401) if self.tokens.forget_access_token() => continue,
                Err(status) => {
                    let _ = std::fs::remove_file(to);
                    return Err(failure(
                        &http::Reply {
                            status,
                            location: None,
                            range: None,
                            etag: None,
                            body: Vec::new(),
                        },
                        "downloading",
                    ));
                }
            }
        };
        Ok(Fetched {
            name: item.name.clone(),
            mime: item.mime.clone(),
            size,
        })
    }

    /// The picture behind an item's `thumbnail` link, about `width`
    /// pixels wide. Only Google's own hosts get the token.
    pub async fn thumbnail(&self, link: &str, width: u32) -> Result<Vec<u8>> {
        // Google's links end in a size, `=s220`, that can be changed.
        let url = match link.rsplit_once("=s") {
            Some((base, size)) if size.bytes().all(|b| b.is_ascii_digit()) => {
                format!("{base}=s{width}")
            }
            _ => link.to_owned(),
        };
        let host = url
            .strip_prefix("https://")
            .and_then(|rest| rest.split(['/', ':']).next())
            .unwrap_or_default();
        let google = host.ends_with(".googleusercontent.com")
            || host.ends_with(".google.com")
            || host.ends_with(".googleapis.com")
            || url.starts_with(&self.api);
        if !google {
            return Err(Error::Protocol(format!("thumbnail from {host}")));
        }
        let reply = self.call_limited("GET", &url, MAX_THUMBNAIL).await?;
        check(&reply, "getting a thumbnail")?;
        Ok(reply.body)
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use katna_core::OAuthProvider;

    use super::*;
    use crate::{
        fake_http::serve,
        net::Tls,
        oauth::{GOOGLE_DRIVE_FILE, Provider, TokenSource},
    };

    #[test]
    fn wildcard_searches() {
        assert_eq!(
            search_query("*.pdf"),
            "mimeType = 'application/pdf' and trashed = false"
        );
        assert_eq!(
            search_query("budget invoice*2026*"),
            "(name contains 'budget' or fullText contains 'budget') and \
             (name contains 'invoice' or fullText contains 'invoice') and \
             (name contains '2026' or fullText contains '2026') and trashed = false"
        );
        assert_eq!(search_query("*"), "trashed = false");
    }

    fn drive(api: &str, scope: &str) -> Drive {
        let provider = Provider {
            kind: OAuthProvider::Google,
            auth_url: "https://accounts.test/auth".into(),
            token_url: "http://127.0.0.1:1/token".into(),
            client_id: "katna-test".into(),
            client_secret: "not-secret".into(),
            scope: format!("https://mail.test/ {GOOGLE_DRIVE_READ}"),
            consent: String::new(),
            redirect_host: "127.0.0.1",
            tls: Tls::insecure_for_local_tests(),
        };
        let tokens = TokenSource::new(provider, "rt".into(), None)
            .with_access_token("at-1".into(), Duration::from_secs(3600))
            .with_scope(Some(scope.to_owned()));
        Drive::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
    }

    const LISTING: &str = r#"{
        "nextPageToken": "page-2",
        "files": [
            {"id": "f1", "name": "Northwind", "mimeType": "application/vnd.google-apps.folder",
             "modifiedTime": "2026-09-30T10:00:00.000Z", "webViewLink": "https://drive.test/f1"},
            {"id": "d1", "name": "Q3 summary", "mimeType": "application/vnd.google-apps.document",
             "modifiedTime": "2026-09-29T08:00:00Z", "thumbnailLink": "https://lh3.googleusercontent.com/abc=s220"},
            {"id": "s1", "name": "Old link", "mimeType": "application/vnd.google-apps.shortcut"},
            {"id": "p1", "name": "report.pdf", "mimeType": "application/pdf", "size": "2400000",
             "modifiedTime": "2026-09-28T12:30:00Z", "webViewLink": "https://drive.test/p1"}
        ]
    }"#;

    #[test]
    fn lists_a_folder_with_folders_first() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), LISTING.into()));
        let drive = drive(&api, GOOGLE_DRIVE_READ);
        assert!(smol::block_on(drive.readable()).unwrap());
        let page = smol::block_on(drive.list(&Place::Folder("ab'c".into()), "")).unwrap();
        assert_eq!(page.next, "page-2");
        let names: Vec<_> = page.items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, ["Northwind", "Q3 summary", "report.pdf"]);
        assert!(page.items[0].folder && page.items[0].mime.is_empty());
        assert!(page.items[1].native && page.items[1].size == 0);
        assert_eq!(page.items[2].size, 2_400_000);
        assert_eq!(page.items[2].modified, Some(1_790_598_600));
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].header("Authorization"), Some("Bearer at-1"));
        let path = &seen[0].path;
        assert!(path.starts_with("/drive/v3/files?q="), "{path}");
        assert!(
            path.contains(&http::escape(r"'ab\'c' in parents")),
            "{path}"
        );
        assert!(path.contains("orderBy=folder"), "{path}");
    }

    #[test]
    fn a_search_is_unsorted_and_pages_go_on() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), r#"{"files": []}"#.into()));
        let drive = drive(&api, GOOGLE_DRIVE_READ);
        let page = smol::block_on(drive.list(&Place::Search(" budget ".into()), "p2")).unwrap();
        assert!(page.items.is_empty() && page.next.is_empty());
        let path = seen.lock().unwrap()[0].path.clone();
        assert!(
            path.contains(&http::escape("fullText contains 'budget'")),
            "{path}"
        );
        assert!(!path.contains("orderBy"), "{path}");
        assert!(path.ends_with("&pageToken=p2"), "{path}");
    }

    #[test]
    fn a_sign_in_with_only_katnas_files_cannot_browse() {
        let (api, _) = serve(|_, _| {
            (
                403,
                Vec::new(),
                r#"{"error":{"errors":[{"reason":"insufficientPermissions"}],"message":"no"}}"#
                    .into(),
            )
        });
        let drive = drive(&api, GOOGLE_DRIVE_FILE);
        assert!(!smol::block_on(drive.readable()).unwrap());
        let listed = smol::block_on(drive.list(&Place::Shared, ""));
        assert!(matches!(listed, Err(Error::Auth(_))), "{listed:?}");
    }

    #[test]
    fn downloads_files_and_makes_pdfs_of_documents() {
        let (api, seen) = serve(|request, _| {
            if request.path.contains("/export?") {
                (200, Vec::new(), "%PDF-doc".into())
            } else {
                (200, Vec::new(), "file bytes".into())
            }
        });
        let drive = drive(&api, GOOGLE_DRIVE_READ);
        let dir = tempfile::tempdir().unwrap();
        let to = dir.path().join("one");
        let file = CloudItem {
            id: "p1".into(),
            name: "report.pdf".into(),
            mime: "application/pdf".into(),
            ..Default::default()
        };
        let got = smol::block_on(drive.fetch(&file, &to)).unwrap();
        assert_eq!(got.size, 10);
        assert_eq!(std::fs::read(&to).unwrap(), b"file bytes");
        let doc = CloudItem {
            id: "d1".into(),
            name: "Q3 summary".into(),
            native: true,
            ..Default::default()
        };
        let got = smol::block_on(drive.fetch(&doc, &to)).unwrap();
        assert_eq!(
            (got.name.as_str(), got.mime.as_str()),
            ("Q3 summary.pdf", "application/pdf")
        );
        assert_eq!(std::fs::read(&to).unwrap(), b"%PDF-doc");
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].path, "/drive/v3/files/p1?alt=media");
        assert_eq!(seen[0].header("Authorization"), Some("Bearer at-1"));
        assert_eq!(
            seen[1].path,
            "/drive/v3/files/d1/export?mimeType=application%2Fpdf"
        );
    }

    #[test]
    fn a_missing_file_fails_and_leaves_nothing() {
        let (api, _) = serve(|_, _| (404, Vec::new(), "{}".into()));
        let drive = drive(&api, GOOGLE_DRIVE_READ);
        let dir = tempfile::tempdir().unwrap();
        let to = dir.path().join("one");
        let item = CloudItem {
            id: "gone".into(),
            ..Default::default()
        };
        assert!(smol::block_on(drive.fetch(&item, &to)).is_err());
        assert!(!to.exists());
    }

    #[test]
    fn thumbnails_go_only_to_google() {
        let drive = drive("http://127.0.0.1:1", GOOGLE_DRIVE_READ);
        let elsewhere = smol::block_on(drive.thumbnail("https://evil.test/x=s220", 400));
        assert!(matches!(elsewhere, Err(Error::Protocol(_))));
        let (api, seen) = serve(|_, _| (200, Vec::new(), "png".into()));
        let drive = super::super::Drive::with_api(drive.tokens.clone(), drive.tls.clone(), &api);
        let link = format!("{api}/thumb/abc=s220");
        assert_eq!(smol::block_on(drive.thumbnail(&link, 400)).unwrap(), b"png");
        assert_eq!(seen.lock().unwrap()[0].path, "/thumb/abc=s400");
    }

    #[test]
    fn reading_and_writing_follow_the_grant() {
        let full = drive("http://127.0.0.1:1", GOOGLE_DRIVE);
        assert!(smol::block_on(full.readable()).unwrap());
        assert!(smol::block_on(full.writable()).unwrap());
        let read = drive("http://127.0.0.1:1", GOOGLE_DRIVE_READ);
        assert!(smol::block_on(read.readable()).unwrap());
        assert!(!smol::block_on(read.writable()).unwrap());
        let mine = drive("http://127.0.0.1:1", GOOGLE_DRIVE_FILE);
        assert!(!smol::block_on(mine.readable()).unwrap());
    }

    #[test]
    fn makes_a_folder_in_a_folder() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), r#"{"id": "new-1"}"#.into()));
        let drive = drive(&api, GOOGLE_DRIVE);
        let id = smol::block_on(drive.create_folder("Site photos", "parent-1")).unwrap();
        assert_eq!(id, "new-1");
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].path, "/drive/v3/files?fields=id");
        let body: serde_json::Value = serde_json::from_slice(&seen[0].body).unwrap();
        assert_eq!(body["name"], "Site photos");
        assert_eq!(body["mimeType"], FOLDER);
        assert_eq!(body["parents"], serde_json::json!(["parent-1"]));
    }
}
