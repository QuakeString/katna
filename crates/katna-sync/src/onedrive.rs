// SPDX-License-Identifier: GPL-3.0-or-later

//! OneDrive, for attachments too large to send by mail from Microsoft
//! accounts (Outlook.com, Hotmail, Microsoft 365), as Outlook does
//! (`docs/ARCHITECTURE.md` §6.6): the counterpart of [`crate::drive`]
//! through Microsoft Graph. Files go to Katna's own folder (`Apps/Katna`)
//! in an upload session, are shared with the recipients, and the mail
//! carries their links. Graph's tokens come from the account's refresh
//! token ([`TokenSource::access_token_for`], scope [`MICROSOFT_FILES`]).

use std::{path::Path, sync::Arc, time::Duration};

use serde::Deserialize;

use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    drive::{DriveFile, read_chunk},
    net::Tls,
    oauth::{MICROSOFT_FILES, TokenSource},
};

/// Microsoft Graph.
pub const GRAPH_API: &str = "https://graph.microsoft.com/v1.0";

/// How much goes up in one request: a multiple of 320 KiB, as OneDrive
/// asks.
pub const CHUNK: usize = 32 * 320 * 1024;

const TIMEOUT: Duration = Duration::from_secs(5 * 60);

const RETRIES: u32 = 5;

/// One Microsoft account's OneDrive.
#[derive(Clone)]
pub struct OneDrive {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`GRAPH_API`], or a server under test.
    api: String,
}

#[derive(Deserialize)]
struct Item {
    id: String,
    #[serde(rename = "webUrl", default)]
    web_url: String,
}

#[derive(Deserialize)]
struct Session {
    #[serde(rename = "uploadUrl")]
    upload_url: String,
}

#[derive(Deserialize, Default)]
struct Ranges {
    #[serde(rename = "nextExpectedRanges", default)]
    next: Vec<String>,
}

#[derive(Deserialize)]
struct Link {
    link: LinkUrl,
}

#[derive(Deserialize)]
struct LinkUrl {
    #[serde(rename = "webUrl")]
    web_url: String,
}

impl OneDrive {
    /// Microsoft's OneDrive, or the server under test in
    /// `KATNA_GRAPH_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GRAPH_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(GRAPH_API))
    }

    /// Talks to `api` instead of Microsoft, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna into OneDrive. Accounts
    /// signed in before Katna asked for it have to sign in again.
    pub async fn allowed(&self) -> Result<bool> {
        match self.tokens.access_token_for(MICROSOFT_FILES).await {
            Ok(_) => Ok(true),
            Err(Error::Auth(_)) => Ok(false),
            Err(err) => Err(err),
        }
    }

    /// Sends a request to Graph with the account's access token, trying
    /// once more with a fresh token when Graph refuses the one it had.
    async fn call(&self, method: &str, url: &str, body: Option<&[u8]>) -> Result<Reply> {
        loop {
            let token = format!(
                "Bearer {}",
                self.tokens.access_token_for(MICROSOFT_FILES).await?
            );
            let headers = [("Authorization", token.as_str())];
            let body = body.map(|b| ("application/json", b));
            let reply =
                http::exchange(method, url, &headers, body, None, &self.tls, TIMEOUT).await?;
            if reply.status == 401 && self.tokens.forget_access_token_for(MICROSOFT_FILES) {
                continue;
            }
            return Ok(reply);
        }
    }

    /// Uploads the file at `path` as `name` to Katna's folder, calling
    /// `progress` with the bytes OneDrive has and the size as they go up.
    /// A dropped connection resumes where OneDrive stopped.
    pub async fn upload(
        &self,
        path: &Path,
        name: &str,
        progress: &(dyn Fn(u64, u64) + Sync),
    ) -> Result<DriveFile> {
        let size = std::fs::metadata(path)?.len();
        let url = format!(
            "{}/me/drive/special/approot:/{}:/createUploadSession",
            self.api,
            http::escape(name)
        );
        let body = serde_json::json!({
            "item": { "@microsoft.graph.conflictBehavior": "rename" }
        })
        .to_string();
        let started = self.call("POST", &url, Some(body.as_bytes())).await?;
        check(&started, "starting the upload")?;
        let session: Session = parse(&started.body)?;
        // The session's address carries its own authorisation; Graph
        // refuses pieces that also bring a token.
        let session = session.upload_url;

        let mut offset = 0;
        let mut failures = 0;
        progress(0, size);
        loop {
            let chunk = read_chunk(path.to_owned(), offset, size, CHUNK).await?;
            let end = offset + chunk.len() as u64;
            let range = format!("bytes {offset}-{}/{size}", end.saturating_sub(1));
            let base = offset;
            let sent = move |n: usize| progress(base + n as u64, size);
            let reply = http::exchange(
                "PUT",
                &session,
                &[("Content-Range", &range)],
                Some(("application/octet-stream", &chunk)),
                Some(&sent),
                &self.tls,
                TIMEOUT,
            )
            .await;
            let reply = match reply {
                Ok(reply) if reply.status < 500 => reply,
                failed => {
                    failures += 1;
                    if failures > RETRIES {
                        return match failed {
                            Err(err) => Err(err),
                            Ok(reply) => Err(failure(&reply, "uploading")),
                        };
                    }
                    async_io::Timer::after(Duration::from_secs(1 << failures)).await;
                    match self.received(&session).await {
                        Ok(have) => {
                            offset = have;
                            progress(offset, size);
                            continue;
                        }
                        Err(err) if err.is_transient() => continue,
                        Err(err) => return Err(err),
                    }
                }
            };
            match reply.status {
                200 | 201 => {
                    progress(size, size);
                    let item: Item = parse(&reply.body)?;
                    return Ok(DriveFile {
                        id: item.id,
                        link: item.web_url,
                    });
                }
                202 => {
                    let ranges: Ranges = serde_json::from_slice(&reply.body).unwrap_or_default();
                    offset = next_offset(&ranges.next).unwrap_or(end);
                    failures = 0;
                    progress(offset, size);
                }
                404 | 410 => {
                    return Err(Error::Protocol("OneDrive forgot the upload".into()));
                }
                _ => return Err(failure(&reply, "uploading")),
            }
        }
    }

    /// How much of an interrupted upload OneDrive has.
    async fn received(&self, session: &str) -> Result<u64> {
        let reply = http::exchange("GET", session, &[], None, None, &self.tls, TIMEOUT).await?;
        check(&reply, "resuming the upload")?;
        let ranges: Ranges = parse(&reply.body)?;
        next_offset(&ranges.next)
            .ok_or_else(|| Error::Protocol("OneDrive said nothing is missing".into()))
    }

    /// Lets each of `addresses` view item `id`, without Microsoft's own
    /// sharing mail. Returns the addresses OneDrive would not share with.
    pub async fn share(&self, id: &str, addresses: &[String]) -> Result<Vec<String>> {
        let url = format!("{}/me/drive/items/{}/invite", self.api, http::escape(id));
        let mut refused = Vec::new();
        for address in addresses {
            let body = serde_json::json!({
                "recipients": [{ "email": address }],
                "requireSignIn": true,
                "sendInvitation": false,
                "roles": ["read"],
            })
            .to_string();
            let reply = self.call("POST", &url, Some(body.as_bytes())).await?;
            match reply.status {
                200 | 201 => {}
                // Some recipients failed, and there was only one.
                207 | 400 => refused.push(address.clone()),
                // The organisation does not share outside.
                403 if is_sharing(&reply.body) => refused.push(address.clone()),
                _ => check(&reply, "sharing")?,
            }
        }
        Ok(refused)
    }

    /// Lets anyone with the link view item `id`; returns that link, which
    /// replaces the item's own in the message.
    pub async fn share_with_link(&self, id: &str) -> Result<String> {
        let url = format!(
            "{}/me/drive/items/{}/createLink",
            self.api,
            http::escape(id)
        );
        let body = br#"{"type":"view","scope":"anonymous"}"#;
        let reply = self.call("POST", &url, Some(body.as_slice())).await?;
        check(&reply, "sharing the link")?;
        let link: Link = parse(&reply.body)?;
        Ok(link.link.web_url)
    }

    /// Moves item `id` to the recycle bin (it was taken off a message
    /// before it went).
    pub async fn remove(&self, id: &str) -> Result<()> {
        let url = format!("{}/me/drive/items/{}", self.api, http::escape(id));
        let reply = self.call("DELETE", &url, None).await?;
        match reply.status {
            404 => Ok(()),
            _ => check(&reply, "removing the file"),
        }
    }
}

/// Where to go on: the start of the first range OneDrive still expects
/// (`"26-"` or `"26-99"`).
fn next_offset(ranges: &[String]) -> Option<u64> {
    ranges
        .iter()
        .filter_map(|r| r.split('-').next()?.trim().parse::<u64>().ok())
        .min()
}

fn parse<T: for<'a> Deserialize<'a>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(body).map_err(|err| Error::Protocol(format!("OneDrive answer: {err}")))
}

#[derive(Deserialize)]
struct Failure {
    error: FailureBody,
}

#[derive(Deserialize)]
struct FailureBody {
    #[serde(default)]
    code: String,
    #[serde(default)]
    message: String,
}

fn reason(body: &[u8]) -> (String, String) {
    serde_json::from_slice::<Failure>(body)
        .map_or_else(|_| Default::default(), |f| (f.error.code, f.error.message))
}

/// Whether Graph's message is about sharing with someone.
fn is_sharing(body: &[u8]) -> bool {
    let (_, message) = reason(body);
    let message = message.to_ascii_lowercase();
    message.contains("shar") || message.contains("recipient") || message.contains("invit")
}

fn check(reply: &Reply, doing: &str) -> Result<()> {
    if (200..300).contains(&reply.status) {
        Ok(())
    } else {
        Err(failure(reply, doing))
    }
}

/// What a failed answer means: a refused token or permission asks to sign
/// in again ([`Error::Auth`]); anything else is OneDrive's own message.
fn failure(reply: &Reply, doing: &str) -> Error {
    let (code, message) = reason(&reply.body);
    let denied = matches!(code.as_str(), "accessDenied" | "unauthenticated");
    if reply.status == 401 || (reply.status == 403 && denied && !is_sharing(&reply.body)) {
        return Error::Auth(format!("OneDrive refused access while {doing}"));
    }
    let detail = if message.is_empty() {
        format!("status {}", reply.status)
    } else {
        message
    };
    Error::Rejected(format!("OneDrive, {doing}: {detail}"))
}

#[cfg(test)]
mod tests;
