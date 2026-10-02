// SPDX-License-Identifier: GPL-3.0-or-later

//! Google Drive, for attachments too large to send by mail
//! (`docs/ARCHITECTURE.md` §6.6): a file goes up in a resumable upload,
//! is shared with the recipients, and the mail carries its link, as Gmail
//! does. Only the few Drive v3 calls this needs, over our own HTTPS
//! client, with the account's OAuth2 access tokens (scope
//! [`GOOGLE_DRIVE_FILE`]: only files Katna put there).

use std::{
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use serde::Deserialize;

use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    net::Tls,
    oauth::{GOOGLE_DRIVE_FILE, TokenSource},
};

/// Google's API host.
pub const GOOGLE_API: &str = "https://www.googleapis.com";

/// How much goes up in one request: a multiple of 256 KiB, as Drive asks.
pub const CHUNK: usize = 8 * 1024 * 1024;

/// How long one request, a whole chunk included, may take.
const TIMEOUT: Duration = Duration::from_secs(5 * 60);

/// How often a failed chunk is tried again before the upload fails.
const RETRIES: u32 = 5;

/// A file in Drive.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DriveFile {
    pub id: String,
    /// Where people open it in the browser.
    #[serde(rename = "webViewLink", default)]
    pub link: String,
}

/// One Google account's Drive.
#[derive(Clone)]
pub struct Drive {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`GOOGLE_API`], or a server under test.
    api: String,
}

impl Drive {
    /// Google's Drive, or the server under test in `KATNA_GOOGLE_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GOOGLE_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(GOOGLE_API))
    }

    /// Talks to `api` instead of Google, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna into Drive. Accounts
    /// signed in before Katna asked for it have to sign in again.
    pub async fn allowed(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_DRIVE_FILE).await
    }

    /// Sends a request with the account's access token, trying once more
    /// with a fresh token when Google refuses the one it had.
    async fn call(
        &self,
        method: &str,
        url: &str,
        headers: &[(&str, &str)],
        body: Option<(&str, &[u8])>,
        sent: Option<http::Progress<'_>>,
    ) -> Result<Reply> {
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let mut all = vec![("Authorization", token.as_str())];
            all.extend_from_slice(headers);
            let reply = http::exchange(method, url, &all, body, sent, &self.tls, TIMEOUT).await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    /// A `GET`-like [`Self::call`] without a body, reading answers of up
    /// to `max_body` bytes: listings, thumbnails and PDFs.
    async fn call_limited(&self, method: &str, url: &str, max_body: usize) -> Result<Reply> {
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let reply = http::exchange_limited(
                method,
                url,
                &[("Authorization", token.as_str())],
                None,
                None,
                &self.tls,
                TIMEOUT,
                max_body,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    /// Uploads the file at `path` as `name`, calling `progress` with the
    /// bytes Drive has and the size as they go up. A dropped connection
    /// resumes where Drive stopped rather than starting over.
    pub async fn upload(
        &self,
        path: &Path,
        name: &str,
        mime: &str,
        progress: &(dyn Fn(u64, u64) + Sync),
    ) -> Result<DriveFile> {
        self.upload_into(path, name, mime, "", progress).await
    }

    /// [`Self::upload`] into folder `parent` (empty for the top of My
    /// Drive), which needs [`crate::oauth::GOOGLE_DRIVE`] unless Katna made
    /// the folder.
    pub async fn upload_into(
        &self,
        path: &Path,
        name: &str,
        mime: &str,
        parent: &str,
        progress: &(dyn Fn(u64, u64) + Sync),
    ) -> Result<DriveFile> {
        let size = std::fs::metadata(path)?.len();
        let mut metadata = serde_json::json!({ "name": name, "mimeType": mime });
        if !parent.is_empty() {
            metadata["parents"] = serde_json::json!([parent]);
        }
        let metadata = metadata.to_string();
        let length = size.to_string();
        let url = format!(
            "{}/upload/drive/v3/files?uploadType=resumable&fields=id,webViewLink",
            self.api
        );
        let started = self
            .call(
                "POST",
                &url,
                &[
                    ("X-Upload-Content-Type", mime),
                    ("X-Upload-Content-Length", &length),
                ],
                Some(("application/json; charset=UTF-8", metadata.as_bytes())),
                None,
            )
            .await?;
        check(&started, "starting the upload")?;
        let session = started
            .location
            .ok_or_else(|| Error::Protocol("Drive gave no upload address".into()))?;

        let mut offset = 0;
        let mut failures = 0;
        progress(0, size);
        loop {
            let chunk = read_chunk(path.to_owned(), offset, size, CHUNK).await?;
            let end = offset + chunk.len() as u64;
            let range = if size == 0 {
                "bytes */0".to_owned()
            } else {
                format!("bytes {offset}-{}/{size}", end.saturating_sub(1))
            };
            let base = offset;
            let sent = move |n: usize| progress(base + n as u64, size);
            let reply = self
                .call(
                    "PUT",
                    &session,
                    &[("Content-Range", &range)],
                    Some(("application/octet-stream", &chunk)),
                    Some(&sent),
                )
                .await;
            let reply = match reply {
                Ok(reply) if reply.status < 500 => reply,
                Err(Error::Auth(message)) => return Err(Error::Auth(message)),
                // The network or Google failed: ask how much arrived and go
                // on from there.
                failed => {
                    failures += 1;
                    if failures > RETRIES {
                        return match failed {
                            Err(err) => Err(err),
                            Ok(reply) => Err(failure(&reply, "uploading")),
                        };
                    }
                    async_io::Timer::after(Duration::from_secs(1 << failures)).await;
                    match self.received(&session, size).await {
                        Ok(Some(have)) => {
                            offset = have;
                            progress(offset, size);
                            continue;
                        }
                        Ok(None) => {
                            return Err(Error::Protocol(
                                "Drive finished the upload without saying so".into(),
                            ));
                        }
                        Err(err) if err.is_transient() => continue,
                        Err(err) => return Err(err),
                    }
                }
            };
            match reply.status {
                200 | 201 => {
                    progress(size, size);
                    return parse(&reply.body);
                }
                308 => {
                    offset = next_offset(reply.range.as_deref());
                    failures = 0;
                    progress(offset, size);
                }
                // The upload address ran out (after about a week).
                404 | 410 => {
                    return Err(Error::Protocol("Drive forgot the upload".into()));
                }
                _ => return Err(failure(&reply, "uploading")),
            }
        }
    }

    /// How much of an interrupted upload Drive has, or `None` when it
    /// already has it all.
    async fn received(&self, session: &str, size: u64) -> Result<Option<u64>> {
        let range = format!("bytes */{size}");
        let reply = self
            .call("PUT", session, &[("Content-Range", &range)], None, None)
            .await?;
        match reply.status {
            308 => Ok(Some(next_offset(reply.range.as_deref()))),
            200 | 201 => Ok(None),
            _ => Err(failure(&reply, "resuming the upload")),
        }
    }

    /// Lets each of `addresses` view file `id`, without Google's own
    /// sharing mail. Returns the addresses Drive would not share with
    /// (no Google account, or the organisation forbids it).
    pub async fn share(&self, id: &str, addresses: &[String]) -> Result<Vec<String>> {
        let url = format!(
            "{}/drive/v3/files/{}/permissions?sendNotificationEmail=false&fields=id",
            self.api,
            http::escape(id)
        );
        let mut refused = Vec::new();
        for address in addresses {
            let permission = serde_json::json!({
                "type": "user",
                "role": "reader",
                "emailAddress": address,
            })
            .to_string();
            let reply = self
                .call(
                    "POST",
                    &url,
                    &[],
                    Some(("application/json", permission.as_bytes())),
                    None,
                )
                .await?;
            match reply.status {
                200 => {}
                400 | 403 if refused_sharing(&reply.body) => refused.push(address.clone()),
                _ => check(&reply, "sharing")?,
            }
        }
        Ok(refused)
    }

    /// Lets anyone with the link view file `id`.
    pub async fn share_with_link(&self, id: &str) -> Result<()> {
        let url = format!(
            "{}/drive/v3/files/{}/permissions?fields=id",
            self.api,
            http::escape(id)
        );
        let permission = br#"{"type":"anyone","role":"reader"}"#;
        let reply = self
            .call(
                "POST",
                &url,
                &[],
                Some(("application/json", permission.as_slice())),
                None,
            )
            .await?;
        check(&reply, "sharing the link")
    }

    /// Moves file `id` to the Drive's bin (it was taken off a message
    /// before it went).
    pub async fn remove(&self, id: &str) -> Result<()> {
        let url = format!("{}/drive/v3/files/{}?fields=id", self.api, http::escape(id));
        let reply = self
            .call(
                "PATCH",
                &url,
                &[],
                Some(("application/json", br#"{"trashed":true}"#.as_slice())),
                None,
            )
            .await?;
        match reply.status {
            404 => Ok(()),
            _ => check(&reply, "removing the file"),
        }
    }
}

/// Reads up to `chunk` bytes of `path` from `offset`, off the async
/// threads.
pub(crate) async fn read_chunk(
    path: PathBuf,
    offset: u64,
    size: u64,
    chunk: usize,
) -> Result<Vec<u8>> {
    let length = (size - offset.min(size)).min(chunk as u64) as usize;
    blocking::unblock(move || {
        let mut file = std::fs::File::open(path)?;
        file.seek(SeekFrom::Start(offset))?;
        let mut chunk = vec![0; length];
        file.read_exact(&mut chunk)?;
        Ok(chunk)
    })
    .await
}

/// Where to go on after a `308`: its `Range: bytes=0-N` says Drive has up
/// to byte N; without one it has nothing.
fn next_offset(range: Option<&str>) -> u64 {
    range
        .and_then(|r| r.trim().strip_prefix("bytes=0-"))
        .and_then(|last| last.trim().parse::<u64>().ok())
        .map_or(0, |last| last + 1)
}

fn parse(body: &[u8]) -> Result<DriveFile> {
    serde_json::from_slice(body).map_err(|err| Error::Protocol(format!("Drive answer: {err}")))
}

#[derive(Deserialize)]
struct Failure {
    error: FailureBody,
}

#[derive(Deserialize)]
struct FailureBody {
    #[serde(default)]
    message: String,
    #[serde(default)]
    errors: Vec<Reason>,
}

#[derive(Deserialize)]
struct Reason {
    #[serde(default)]
    reason: String,
}

fn reasons(body: &[u8]) -> (Vec<String>, String) {
    serde_json::from_slice::<Failure>(body).map_or_else(
        |_| (Vec::new(), String::new()),
        |f| {
            (
                f.error.errors.into_iter().map(|r| r.reason).collect(),
                f.error.message,
            )
        },
    )
}

/// Whether Drive refused to share with an address, rather than failing.
fn refused_sharing(body: &[u8]) -> bool {
    reasons(body).0.iter().any(|r| {
        matches!(
            r.as_str(),
            "invalidSharingRequest" | "cannotShareTeamDriveWithNonGoogleAccounts" | "forbidden"
        ) || r.starts_with("sharing")
    })
}

/// A `2xx` answer is fine; anything else is [`failure`].
fn check(reply: &Reply, doing: &str) -> Result<()> {
    if (200..300).contains(&reply.status) {
        Ok(())
    } else {
        Err(failure(reply, doing))
    }
}

/// What a failed answer means: the Drive API switched off for Katna's
/// Google Cloud project ([`Error::NotEnabled`]); a refused grant or scope
/// asks to sign in again ([`Error::Auth`]); anything else is Drive's own
/// message.
fn failure(reply: &Reply, doing: &str) -> Error {
    if let Some(off) = crate::google_api::switched_off(reply.status, &reply.body) {
        return off;
    }
    let (reasons, message) = reasons(&reply.body);
    let scope = reasons.iter().any(|r| {
        matches!(
            r.as_str(),
            "insufficientPermissions" | "authError" | "ACCESS_TOKEN_SCOPE_INSUFFICIENT"
        )
    });
    if reply.status == 401 || (reply.status == 403 && scope) {
        return Error::Auth(format!("Google Drive refused access while {doing}"));
    }
    let detail = if message.is_empty() {
        format!("status {}", reply.status)
    } else {
        message
    };
    Error::Rejected(format!("Google Drive, {doing}: {detail}"))
}

mod browse;
mod manage;

#[cfg(test)]
mod tests;
