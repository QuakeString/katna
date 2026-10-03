// SPDX-License-Identifier: GPL-3.0-or-later

//! Browsing a whole OneDrive for Files (`docs/ARCHITECTURE.md` §13.8),
//! the counterpart of [`crate::drive::browse`] through Microsoft Graph:
//! folders, what was shared with the account, search, thumbnails,
//! downloads, new folders and uploads into any folder. [`MICROSOFT_FILES`]
//! covers it all.
//!
//! An item shared from another person's drive lives in that drive: its id
//! here is `drive/item`, and Graph is asked under `/drives/{drive}`. The
//! account's own ids never hold a `/`.

use std::path::Path;

use katna_core::wildcard;
use serde::Deserialize;

use super::{OneDrive, TIMEOUT, check, parse};
use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    cloud::{CloudItem, CloudPage, Fetched, MAX_FETCH, MAX_THUMBNAIL, Place, ROOT},
    drive::DriveFile,
    oauth::MICROSOFT_FILES,
};

/// What one page lists.
const PAGE: usize = 200;

/// The biggest listing page read, at about 1 KB an item.
const MAX_LISTING: usize = 2 * 1024 * 1024;

/// What Graph is asked to tell about each item.
const SELECT: &str = "id,name,size,lastModifiedDateTime,webUrl,file,folder,remoteItem";

#[derive(Deserialize)]
struct Listing {
    #[serde(default)]
    value: Vec<Item>,
    #[serde(rename = "@odata.nextLink", default)]
    next: String,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Item {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    size: u64,
    #[serde(default)]
    last_modified_date_time: Option<String>,
    #[serde(default)]
    web_url: String,
    #[serde(default)]
    file: Option<FileFacet>,
    #[serde(default)]
    folder: Option<serde_json::Value>,
    /// Shared from another drive: the item itself, there.
    #[serde(default)]
    remote_item: Option<Box<Item>>,
    #[serde(default)]
    parent_reference: Option<Parent>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct FileFacet {
    #[serde(default)]
    mime_type: String,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Parent {
    #[serde(default)]
    drive_id: String,
}

#[derive(Deserialize)]
struct Download {
    #[serde(rename = "@microsoft.graph.downloadUrl", default)]
    url: String,
}

impl Item {
    /// The item, or `None` for what Files can't show (a notebook, a
    /// package).
    fn item(self) -> Option<CloudItem> {
        if let Some(remote) = self.remote_item {
            let drive = remote
                .parent_reference
                .as_ref()
                .map(|p| p.drive_id.clone())
                .unwrap_or_default();
            let mut item = Item {
                name: if remote.name.is_empty() {
                    self.name
                } else {
                    remote.name.clone()
                },
                last_modified_date_time: remote
                    .last_modified_date_time
                    .clone()
                    .or(self.last_modified_date_time),
                ..*remote
            }
            .item()?;
            if !drive.is_empty() && !item.id.contains('/') {
                item.id = format!("{drive}/{}", item.id);
            }
            return Some(item);
        }
        let folder = self.folder.is_some();
        if !folder && self.file.is_none() {
            return None;
        }
        let mime = match self.file {
            Some(file) if !file.mime_type.is_empty() => file.mime_type,
            Some(_) => "application/octet-stream".into(),
            None => String::new(),
        };
        // Graph draws pictures of pictures, PDFs and Office files.
        let pictured = mime.starts_with("image/")
            || mime == "application/pdf"
            || mime.starts_with("application/vnd.openxmlformats-officedocument.")
            || mime.starts_with("application/vnd.ms-")
            || mime == "application/msword";
        Some(CloudItem {
            thumbnail: if pictured && !self.id.is_empty() {
                self.id.clone()
            } else {
                String::new()
            },
            size: if folder { 0 } else { self.size },
            modified: self
                .last_modified_date_time
                .and_then(|t| t.parse::<jiff::Timestamp>().ok())
                .map(|t| t.as_second()),
            id: self.id,
            name: self.name,
            mime,
            folder,
            native: false,
            link: self.web_url,
        })
    }
}

impl OneDrive {
    /// Whether the account's sign-in lets Katna browse OneDrive.
    pub async fn readable(&self) -> Result<bool> {
        self.allowed().await
    }

    /// Whether it lets Katna put files in any folder: the same grant.
    pub async fn writable(&self) -> Result<bool> {
        self.allowed().await
    }

    /// Where item `id` is in Graph: the account's drive, or the drive of
    /// an item shared with it (`drive/item`).
    pub(super) fn item_url(&self, id: &str) -> String {
        match id.split_once('/') {
            Some((drive, item)) => format!(
                "{}/drives/{}/items/{}",
                self.api,
                http::escape(drive),
                http::escape(item)
            ),
            None if id.is_empty() || id == ROOT => format!("{}/me/drive/root", self.api),
            None => format!("{}/me/drive/items/{}", self.api, http::escape(id)),
        }
    }

    /// A Graph `GET` with the token, its answer at most `max_body` bytes.
    async fn get(&self, url: &str, max_body: usize) -> Result<Reply> {
        loop {
            let token = format!(
                "Bearer {}",
                self.tokens.access_token_for(MICROSOFT_FILES).await?
            );
            let reply = http::exchange_limited(
                "GET",
                url,
                &[("Authorization", token.as_str())],
                None,
                None,
                &self.tls,
                TIMEOUT,
                max_body,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token_for(MICROSOFT_FILES) {
                continue;
            }
            return Ok(reply);
        }
    }

    /// One page of `place`; `page` is the last page's `next`, or empty
    /// for the first. Folders come first, then the newest files.
    pub async fn list(&self, place: &Place, page: &str) -> Result<CloudPage> {
        let url = if page.is_empty() {
            let base = match place {
                Place::Folder(id) => format!("{}/children", self.item_url(id)),
                Place::Shared => format!("{}/me/drive/sharedWithMe", self.api),
                Place::Search(words) => {
                    // Graph can't take wildcards: a pattern asks for its
                    // plain pieces, and only what fits is kept below.
                    let words = words
                        .split_whitespace()
                        .flat_map(|w| {
                            if wildcard::is_pattern(w) {
                                wildcard::pieces(w)
                            } else {
                                vec![w]
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                        .replace('\'', "''");
                    format!(
                        "{}/me/drive/root/search(q='{}')",
                        self.api,
                        http::escape(&words)
                    )
                }
            };
            format!("{base}?$top={PAGE}&$select={SELECT}")
        } else if page.starts_with(&format!("{}/", self.api)) {
            page.to_owned()
        } else {
            // The token only ever goes to Graph.
            return Err(Error::Protocol("OneDrive's next page is elsewhere".into()));
        };
        let reply = self.get(&url, MAX_LISTING).await?;
        check(&reply, "listing files")?;
        let listing: Listing = parse(&reply.body)?;
        let mut items: Vec<CloudItem> = listing.value.into_iter().filter_map(Item::item).collect();
        if let Place::Search(words) = place {
            crate::cloud::keep_matching(&mut items, words);
        }
        // Graph keeps its own order; a search keeps its ranking.
        if !matches!(place, Place::Search(_)) {
            items.sort_by_key(|i| (!i.folder, std::cmp::Reverse(i.modified)));
        }
        Ok(CloudPage {
            items,
            next: listing.next,
        })
    }

    /// Downloads `item` into `to`. Graph gives a short-lived address that
    /// needs no token. Returns what was saved.
    pub async fn fetch(&self, item: &CloudItem, to: &Path) -> Result<Fetched> {
        let url = format!(
            "{}?$select={}",
            self.item_url(&item.id),
            http::escape("id,@microsoft.graph.downloadUrl")
        );
        let reply = self.get(&url, 64 * 1024).await?;
        check(&reply, "downloading")?;
        let download: Download = parse(&reply.body)?;
        if !download.url.starts_with("https://") && !download.url.starts_with(&self.api) {
            return Err(Error::Protocol("OneDrive gave no download address".into()));
        }
        let mut file = std::fs::File::create(to)?;
        let mut sink = |piece: &[u8], _total: u64| std::io::Write::write_all(&mut file, piece);
        match http::download_with(&download.url, &[], &self.tls, MAX_FETCH, &mut sink).await? {
            Ok(size) => Ok(Fetched {
                name: item.name.clone(),
                mime: item.mime.clone(),
                size,
            }),
            Err(status) => {
                let _ = std::fs::remove_file(to);
                Err(Error::Rejected(format!(
                    "OneDrive, downloading: status {status}"
                )))
            }
        }
    }

    /// A picture of item `id` (its `thumbnail`), about `width` pixels
    /// wide. The token goes to Graph only, not where it sends on to.
    pub async fn thumbnail(&self, id: &str, width: u32) -> Result<Vec<u8>> {
        let size = match width {
            0..=96 => "small",
            97..=176 => "medium",
            _ => "large",
        };
        let url = format!("{}/thumbnails/0/{size}/content", self.item_url(id));
        let token = format!(
            "Bearer {}",
            self.tokens.access_token_for(MICROSOFT_FILES).await?
        );
        let mut picture = Vec::new();
        let mut sink = |piece: &[u8], _total: u64| {
            picture.extend_from_slice(piece);
            Ok(())
        };
        match http::download_with(
            &url,
            &[("Authorization", &token)],
            &self.tls,
            MAX_THUMBNAIL as u64,
            &mut sink,
        )
        .await?
        {
            Ok(_) => Ok(picture),
            Err(status) => Err(Error::Rejected(format!(
                "OneDrive, getting a thumbnail: status {status}"
            ))),
        }
    }

    /// Makes folder `name` in folder `parent` (empty for the top) and
    /// returns its id; a name taken gets a number.
    pub async fn create_folder(&self, name: &str, parent: &str) -> Result<String> {
        let url = format!("{}/children", self.item_url(parent));
        let body = serde_json::json!({
            "name": name,
            "folder": {},
            "@microsoft.graph.conflictBehavior": "rename",
        })
        .to_string();
        let reply = self.call("POST", &url, Some(body.as_bytes())).await?;
        check(&reply, "making a folder")?;
        let item: super::Item = parse(&reply.body)?;
        Ok(item.id)
    }

    /// Uploads the file at `path` as `name` into folder `parent` (empty
    /// for the top), as [`OneDrive::upload`] does into Katna's folder.
    pub async fn upload_into(
        &self,
        path: &Path,
        name: &str,
        parent: &str,
        progress: &(dyn Fn(u64, u64) + Sync),
    ) -> Result<DriveFile> {
        let url = format!(
            "{}:/{}:/createUploadSession",
            self.item_url(parent),
            http::escape(name)
        );
        self.upload_at(&url, path, progress).await
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
        oauth::{Provider, TokenSource},
    };

    fn onedrive(api: &str) -> OneDrive {
        let provider = Provider {
            kind: OAuthProvider::Microsoft,
            auth_url: "https://login.test/authorize".into(),
            token_url: "http://127.0.0.1:1/token".into(),
            client_id: "katna-test".into(),
            client_secret: String::new(),
            scope: "offline_access".into(),
            consent: MICROSOFT_FILES.into(),
            redirect_host: "localhost",
            tls: Tls::insecure_for_local_tests(),
        };
        let tokens = TokenSource::new(provider, "rt".into(), None).with_access_token_for(
            MICROSOFT_FILES,
            "at-files".into(),
            Duration::from_secs(3600),
        );
        OneDrive::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
    }

    const LISTING: &str = r#"{
        "@odata.nextLink": "NEXT",
        "value": [
            {"id": "f-old", "name": "Personal", "folder": {"childCount": 3},
             "lastModifiedDateTime": "2026-08-01T10:00:00Z", "webUrl": "https://onedrive.test/f-old"},
            {"id": "p1", "name": "report.pdf", "size": 2400000, "file": {"mimeType": "application/pdf"},
             "lastModifiedDateTime": "2026-09-28T12:30:00Z", "webUrl": "https://onedrive.test/p1"},
            {"id": "n1", "name": "Notebook", "package": {"type": "oneNote"}},
            {"id": "f-new", "name": "Northwind", "folder": {"childCount": 4},
             "lastModifiedDateTime": "2026-09-30T10:00:00Z"},
            {"id": "z1", "name": "old.zip", "size": 10, "file": {"mimeType": "application/zip"},
             "lastModifiedDateTime": "2026-09-30T11:00:00Z"}
        ]
    }"#;

    #[test]
    fn lists_a_folder_with_folders_first() {
        let (api, seen) = serve(|request, base| {
            let body = LISTING.replace("NEXT", &format!("{base}/me/drive/root/children?page=2"));
            let _ = request;
            (200, Vec::new(), body)
        });
        let drive = onedrive(&api);
        let page = smol::block_on(drive.list(&Place::Folder(ROOT.into()), "")).unwrap();
        let names: Vec<_> = page.items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, ["Northwind", "Personal", "old.zip", "report.pdf"]);
        assert!(page.items[0].folder && page.items[0].mime.is_empty());
        assert_eq!(page.items[3].size, 2_400_000);
        assert_eq!(page.items[3].thumbnail, "p1");
        assert!(page.items[2].thumbnail.is_empty());
        assert!(page.next.ends_with("/me/drive/root/children?page=2"));
        // The next page comes from Graph only.
        smol::block_on(drive.list(&Place::Folder(ROOT.into()), &page.next)).unwrap();
        assert!(smol::block_on(drive.list(&Place::Shared, "https://elsewhere.test/x")).is_err());
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].header("Authorization"), Some("Bearer at-files"));
        assert!(
            seen[0].path.starts_with("/me/drive/root/children?$top=200"),
            "{}",
            seen[0].path
        );
        assert_eq!(seen[1].path, "/me/drive/root/children?page=2");
        assert_eq!(seen.len(), 2);
    }

    #[test]
    fn shared_items_live_in_their_own_drive() {
        let (api, seen) = serve(|_, _| {
            (
                200,
                Vec::new(),
                r#"{"value": [{"id": "local", "name": "Agenda.pdf",
                    "remoteItem": {"id": "R1", "name": "Agenda.pdf", "size": 5,
                        "file": {"mimeType": "application/pdf"},
                        "parentReference": {"driveId": "D9"}}}]}"#
                    .into(),
            )
        });
        let drive = onedrive(&api);
        let page = smol::block_on(drive.list(&Place::Shared, "")).unwrap();
        assert_eq!(page.items[0].id, "D9/R1");
        assert_eq!(page.items[0].size, 5);
        smol::block_on(drive.list(&Place::Folder("D9/R1".into()), "")).unwrap();
        smol::block_on(drive.list(&Place::Search("it's".into()), "")).unwrap();
        smol::block_on(drive.share("D9/R1", &["ana@example.org".into()])).unwrap();
        let seen = seen.lock().unwrap();
        assert!(seen[0].path.starts_with("/me/drive/sharedWithMe?"));
        assert!(seen[1].path.starts_with("/drives/D9/items/R1/children?"));
        assert!(
            seen[2]
                .path
                .starts_with("/me/drive/root/search(q='it%27%27s')?"),
            "{}",
            seen[2].path
        );
        assert_eq!(seen[3].path, "/drives/D9/items/R1/invite");
    }

    #[test]
    fn downloads_through_graphs_own_address() {
        let (api, seen) = serve(|request, base| {
            if request.path.starts_with("/me/drive/items/p1?") {
                let body = format!(r#"{{"@microsoft.graph.downloadUrl": "{base}/dl/p1"}}"#);
                (200, Vec::new(), body)
            } else {
                (200, Vec::new(), "%PDF-1.7".into())
            }
        });
        let drive = onedrive(&api);
        let item = CloudItem {
            id: "p1".into(),
            name: "report.pdf".into(),
            mime: "application/pdf".into(),
            ..CloudItem::default()
        };
        let dir = tempfile::tempdir().unwrap();
        let to = dir.path().join("f");
        let fetched = smol::block_on(drive.fetch(&item, &to)).unwrap();
        assert_eq!(fetched.size, 8);
        assert_eq!(std::fs::read(&to).unwrap(), b"%PDF-1.7");
        let seen = seen.lock().unwrap();
        assert_eq!(seen[1].path, "/dl/p1");
        // The download address carries its own permission.
        assert_eq!(seen[1].header("Authorization"), None);
    }

    #[test]
    fn makes_a_folder_in_a_folder() {
        let (api, seen) = serve(|_, _| (201, Vec::new(), r#"{"id": "new-1"}"#.into()));
        let drive = onedrive(&api);
        let id = smol::block_on(drive.create_folder("Trip", "f-new")).unwrap();
        assert_eq!(id, "new-1");
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].method, "POST");
        assert_eq!(seen[0].path, "/me/drive/items/f-new/children");
        assert!(
            seen[0].text().contains(r#""folder":{}"#),
            "{}",
            seen[0].text()
        );
    }
}
