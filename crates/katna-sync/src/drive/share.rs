// SPDX-License-Identifier: GPL-3.0-or-later

//! Who may open a file in a Google Drive, from Files' Share dialog
//! (`docs/ARCHITECTURE.md` §13.8): the grants, adding people, changing
//! or taking away what they may do, and the link for anyone.

use serde::Deserialize;

use super::{Drive, check, refused_sharing};
use crate::{
    Error, Result,
    autoconfig::http,
    cloud::{Access, Role, Who},
};

#[derive(Deserialize)]
struct Permissions {
    #[serde(default)]
    permissions: Vec<Permission>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Permission {
    id: String,
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    role: String,
    #[serde(default)]
    email_address: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    permission_details: Vec<Detail>,
}

#[derive(Deserialize)]
struct Detail {
    #[serde(default)]
    inherited: bool,
}

impl Permission {
    fn access(self) -> Option<Access> {
        let who = match self.kind.as_str() {
            "user" => Who::Person,
            "group" => Who::Group,
            "domain" => Who::Domain,
            "anyone" => Who::Anyone,
            _ => return None,
        };
        let role = match self.role.as_str() {
            "owner" | "organizer" => Role::Owner,
            "fileOrganizer" | "writer" => Role::Editor,
            "commenter" => Role::Commenter,
            "reader" => Role::Viewer,
            _ => return None,
        };
        Some(Access {
            id: self.id,
            who,
            role,
            address: if who == Who::Domain {
                self.domain
            } else {
                self.email_address
            },
            name: self.display_name,
            inherited: !self.permission_details.is_empty()
                && self.permission_details.iter().all(|d| d.inherited),
            link: String::new(),
        })
    }
}

/// Drive's name for `role`.
fn role_name(role: Role) -> &'static str {
    match role {
        Role::Owner => "owner",
        Role::Editor => "writer",
        Role::Commenter => "commenter",
        Role::Viewer => "reader",
    }
}

impl Drive {
    fn permissions_url(&self, id: &str) -> String {
        format!(
            "{}/drive/v3/files/{}/permissions",
            self.api,
            http::escape(id)
        )
    }

    /// Who may open file or folder `id`, the owner first.
    pub async fn access(&self, id: &str) -> Result<Vec<Access>> {
        let url = format!(
            "{}?supportsAllDrives=true&fields=permissions(id,type,role,emailAddress,\
             displayName,domain,permissionDetails(inherited))",
            self.permissions_url(id)
        );
        let reply = self.call("GET", &url, &[], None, None).await?;
        check(&reply, "reading who has access")?;
        let listed: Permissions = serde_json::from_slice(&reply.body)
            .map_err(|err| Error::Protocol(format!("Drive answer: {err}")))?;
        let mut all: Vec<Access> = listed
            .permissions
            .into_iter()
            .filter_map(Permission::access)
            .collect();
        all.sort_by_key(|a| (a.role != Role::Owner, a.who == Who::Anyone));
        Ok(all)
    }

    /// Lets each of `addresses` open file `id` as `role`, with Google's
    /// own email when `notify`. Returns the addresses Drive would not
    /// share with.
    pub async fn grant(
        &self,
        id: &str,
        addresses: &[String],
        role: Role,
        notify: bool,
    ) -> Result<Vec<String>> {
        let url = format!(
            "{}?sendNotificationEmail={notify}&supportsAllDrives=true&fields=id",
            self.permissions_url(id)
        );
        let mut refused = Vec::new();
        for address in addresses {
            let permission = serde_json::json!({
                "type": "user",
                "role": role_name(role),
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

    /// Changes grant `permission` of file `id` to `role`, or takes it
    /// away (`None`).
    pub async fn set_access(&self, id: &str, permission: &str, role: Option<Role>) -> Result<()> {
        let url = format!(
            "{}/{}?supportsAllDrives=true",
            self.permissions_url(id),
            http::escape(permission)
        );
        let reply = match role {
            Some(role) => {
                let body = serde_json::json!({ "role": role_name(role) }).to_string();
                self.call(
                    "PATCH",
                    &url,
                    &[],
                    Some(("application/json", body.as_bytes())),
                    None,
                )
                .await?
            }
            None => self.call("DELETE", &url, &[], None, None).await?,
        };
        check(&reply, "changing access")
    }

    /// Lets anyone with the link open file `id` as `role`, or (`None`)
    /// only the people it is shared with.
    pub async fn set_link(&self, id: &str, role: Option<Role>) -> Result<()> {
        let anyone: Vec<Access> = self
            .access(id)
            .await?
            .into_iter()
            .filter(|a| a.who == Who::Anyone && !a.inherited)
            .collect();
        match (role, anyone.first()) {
            (None, _) => {
                for grant in &anyone {
                    self.set_access(id, &grant.id, None).await?;
                }
                Ok(())
            }
            (Some(role), Some(grant)) => self.set_access(id, &grant.id, Some(role)).await,
            (Some(role), None) => {
                let url = format!(
                    "{}?supportsAllDrives=true&fields=id",
                    self.permissions_url(id)
                );
                let body =
                    serde_json::json!({ "type": "anyone", "role": role_name(role) }).to_string();
                let reply = self
                    .call(
                        "POST",
                        &url,
                        &[],
                        Some(("application/json", body.as_bytes())),
                        None,
                    )
                    .await?;
                check(&reply, "sharing the link")
            }
        }
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

    const LISTED: &str = r#"{"permissions": [
        {"id": "anyoneWithLink", "type": "anyone", "role": "reader"},
        {"id": "p2", "type": "user", "role": "writer", "emailAddress": "marco@northwind.example",
         "displayName": "Marco Ruiz", "permissionDetails": [{"inherited": true}]},
        {"id": "p1", "type": "user", "role": "owner", "emailAddress": "priya.demo@gmail.com",
         "displayName": "Priya Demo"},
        {"id": "p3", "type": "domain", "role": "commenter", "domain": "northwind.example"}
    ]}"#;

    #[test]
    fn lists_who_has_access_owner_first() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), LISTED.into()));
        let all = smol::block_on(drive(&api).access("f1")).unwrap();
        assert_eq!(all[0].role, Role::Owner);
        assert_eq!(all[0].address, "priya.demo@gmail.com");
        assert_eq!(all[1].role, Role::Editor);
        assert!(all[1].inherited);
        assert_eq!(all[2].who, Who::Domain);
        assert_eq!(all[2].address, "northwind.example");
        assert_eq!(all[3].who, Who::Anyone);
        assert!(
            seen.lock().unwrap()[0]
                .path
                .starts_with("/drive/v3/files/f1/permissions?")
        );
    }

    #[test]
    fn grants_a_role_with_or_without_googles_email() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), r#"{"id": "p9"}"#.into()));
        let refused = smol::block_on(drive(&api).grant(
            "f1",
            &["ana@northwind.example".into()],
            Role::Editor,
            true,
        ))
        .unwrap();
        assert!(refused.is_empty());
        let seen = seen.lock().unwrap();
        assert!(seen[0].path.contains("sendNotificationEmail=true"));
        let body: serde_json::Value = serde_json::from_slice(&seen[0].body).unwrap();
        assert_eq!(body["role"], "writer");
        assert_eq!(body["emailAddress"], "ana@northwind.example");
    }

    #[test]
    fn changes_and_takes_away_a_grant() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), "{}".into()));
        let drive = drive(&api);
        smol::block_on(drive.set_access("f1", "p2", Some(Role::Viewer))).unwrap();
        smol::block_on(drive.set_access("f1", "p2", None)).unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].method, "PATCH");
        assert!(seen[0].text().contains(r#""role":"reader""#));
        assert_eq!(seen[1].method, "DELETE");
        assert_eq!(
            seen[1].path,
            "/drive/v3/files/f1/permissions/p2?supportsAllDrives=true"
        );
    }

    #[test]
    fn the_link_opens_and_closes() {
        let (api, seen) = serve(|request, _| {
            if request.method == "GET" {
                (200, Vec::new(), LISTED.into())
            } else {
                (200, Vec::new(), "{}".into())
            }
        });
        let drive = drive(&api);
        smol::block_on(drive.set_link("f1", None)).unwrap();
        smol::block_on(drive.set_link("f1", Some(Role::Editor))).unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen[1].method, "DELETE");
        assert!(seen[1].path.contains("/permissions/anyoneWithLink"));
        assert_eq!(seen[3].method, "PATCH");
        assert!(seen[3].text().contains("writer"));
    }
}
