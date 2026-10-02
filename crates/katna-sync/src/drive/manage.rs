// SPDX-License-Identifier: GPL-3.0-or-later

//! Changing files in a Google Drive from Files (`docs/ARCHITECTURE.md`
//! §13.8): moving them to the bin and back, and renaming them.

use super::{Drive, check};
use crate::{Result, autoconfig::http};

impl Drive {
    /// Sends `body` as a change to the metadata of file `id`.
    async fn change(&self, id: &str, body: &serde_json::Value, doing: &str) -> Result<()> {
        let url = format!(
            "{}/drive/v3/files/{}?fields=id&supportsAllDrives=true",
            self.api,
            http::escape(id)
        );
        let body = body.to_string();
        let reply = self
            .call(
                "PATCH",
                &url,
                &[],
                Some(("application/json; charset=UTF-8", body.as_bytes())),
                None,
            )
            .await?;
        check(&reply, doing)
    }

    /// Moves file or folder `id` to the Drive's bin, or (`trashed` false)
    /// back out of it to where it was.
    pub async fn trash(&self, id: &str, trashed: bool) -> Result<()> {
        let doing = if trashed {
            "moving to the bin"
        } else {
            "taking out of the bin"
        };
        self.change(id, &serde_json::json!({ "trashed": trashed }), doing)
            .await
    }

    /// Gives file or folder `id` the name `name`.
    pub async fn rename(&self, id: &str, name: &str) -> Result<()> {
        self.change(id, &serde_json::json!({ "name": name }), "renaming")
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use katna_core::OAuthProvider;

    use super::*;
    use crate::{
        Error,
        fake_http::serve,
        net::Tls,
        oauth::{GOOGLE_DRIVE, Provider, TokenSource},
    };

    fn drive(api: &str) -> Drive {
        let provider = Provider {
            kind: OAuthProvider::Google,
            auth_url: "https://accounts.test/auth".into(),
            token_url: "http://127.0.0.1:1/token".into(),
            client_id: "katna-test".into(),
            client_secret: "not-secret".into(),
            scope: format!("https://mail.test/ {GOOGLE_DRIVE}"),
            consent: String::new(),
            redirect_host: "127.0.0.1",
            tls: Tls::insecure_for_local_tests(),
        };
        let tokens = TokenSource::new(provider, "rt".into(), None)
            .with_access_token("at-1".into(), Duration::from_secs(3600))
            .with_scope(Some(GOOGLE_DRIVE.to_owned()));
        Drive::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
    }

    #[test]
    fn bins_and_brings_back() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), r#"{"id": "p1"}"#.into()));
        let drive = drive(&api);
        smol::block_on(drive.trash("p1", true)).unwrap();
        smol::block_on(drive.trash("p1", false)).unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].method, "PATCH");
        assert_eq!(
            seen[0].path,
            "/drive/v3/files/p1?fields=id&supportsAllDrives=true"
        );
        let first: serde_json::Value = serde_json::from_slice(&seen[0].body).unwrap();
        let second: serde_json::Value = serde_json::from_slice(&seen[1].body).unwrap();
        assert_eq!(first, serde_json::json!({ "trashed": true }));
        assert_eq!(second, serde_json::json!({ "trashed": false }));
    }

    #[test]
    fn renames() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), r#"{"id": "p1"}"#.into()));
        smol::block_on(drive(&api).rename("p1", "Lease “final”.pdf")).unwrap();
        let seen = seen.lock().unwrap();
        let body: serde_json::Value = serde_json::from_slice(&seen[0].body).unwrap();
        assert_eq!(body, serde_json::json!({ "name": "Lease “final”.pdf" }));
    }

    #[test]
    fn a_file_of_someone_else_cannot_be_binned() {
        let (api, _) = serve(|_, _| {
            (
                403,
                Vec::new(),
                r#"{"error":{"code":403,"message":"The user does not have sufficient permissions for this file.","errors":[{"reason":"insufficientFilePermissions"}]}}"#.into(),
            )
        });
        let err = smol::block_on(drive(&api).trash("p1", true)).unwrap_err();
        assert!(!matches!(err, Error::Auth(_)), "{err:?}");
    }
}
