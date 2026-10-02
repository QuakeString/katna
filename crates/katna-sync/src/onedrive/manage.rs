// SPDX-License-Identifier: GPL-3.0-or-later

//! Changing items in a OneDrive from Files (`docs/ARCHITECTURE.md`
//! §13.8): moving them to the recycle bin and back, and renaming them.

use super::{OneDrive, check};
use crate::Result;

impl OneDrive {
    /// Moves item `id` to the recycle bin, or (`trashed` false) restores
    /// it to where it was. Graph restores only on personal OneDrives; a
    /// work or school one stays in its recycle bin and this fails.
    pub async fn trash(&self, id: &str, trashed: bool) -> Result<()> {
        let url = self.item_url(id);
        if trashed {
            let reply = self.call("DELETE", &url, None).await?;
            return check(&reply, "moving to the recycle bin");
        }
        let url = format!("{url}/restore");
        let reply = self.call("POST", &url, Some(b"{}")).await?;
        check(&reply, "restoring from the recycle bin")
    }

    /// Gives item `id` the name `name`.
    pub async fn rename(&self, id: &str, name: &str) -> Result<()> {
        let body = serde_json::json!({ "name": name }).to_string();
        let reply = self
            .call("PATCH", &self.item_url(id), Some(body.as_bytes()))
            .await?;
        check(&reply, "renaming")
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
        oauth::{MICROSOFT_FILES, Provider, TokenSource},
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

    #[test]
    fn bins_and_restores() {
        let (api, seen) = serve(|request, _| {
            let status = if request.method == "DELETE" { 204 } else { 200 };
            (status, Vec::new(), "{}".into())
        });
        let drive = onedrive(&api);
        smol::block_on(drive.trash("p1", true)).unwrap();
        smol::block_on(drive.trash("d9/p2", false)).unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].method, "DELETE");
        assert_eq!(seen[0].path, "/me/drive/items/p1");
        assert_eq!(seen[1].method, "POST");
        assert_eq!(seen[1].path, "/drives/d9/items/p2/restore");
    }

    #[test]
    fn renames() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), r#"{"id": "p1"}"#.into()));
        smol::block_on(onedrive(&api).rename("p1", "Trip.pdf")).unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].method, "PATCH");
        assert!(seen[0].text().contains(r#""name":"Trip.pdf""#));
    }
}
