// SPDX-License-Identifier: GPL-3.0-or-later

//! Who may open an item in a OneDrive, from Files' Share dialog
//! (`docs/ARCHITECTURE.md` §13.8): the grants, inviting people, changing
//! or taking away what they may do, and the link for anyone. OneDrive
//! knows viewers and editors; a commenter is asked for as a viewer.

use serde::Deserialize;

use super::{OneDrive, check, is_sharing, parse};
use crate::{
    Result,
    cloud::{Access, Role, Who},
};

#[derive(Deserialize)]
struct Permissions {
    #[serde(default)]
    value: Vec<Permission>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Permission {
    id: String,
    #[serde(default)]
    roles: Vec<String>,
    #[serde(default)]
    granted_to_v2: Option<Granted>,
    #[serde(default)]
    granted_to_identities_v2: Vec<Granted>,
    #[serde(default)]
    invitation: Option<Invitation>,
    #[serde(default)]
    link: Option<LinkInfo>,
    #[serde(default)]
    inherited_from: Option<serde_json::Value>,
}

#[derive(Deserialize, Default)]
struct Granted {
    #[serde(default)]
    user: Option<Identity>,
    #[serde(default)]
    group: Option<Identity>,
    #[serde(default, rename = "siteUser")]
    site_user: Option<Identity>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct Identity {
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    login_name: String,
}

#[derive(Deserialize)]
struct Invitation {
    #[serde(default)]
    email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkInfo {
    #[serde(default)]
    scope: String,
    #[serde(default, rename = "type")]
    kind: String,
    #[serde(default)]
    web_url: String,
}

impl Permission {
    fn role(&self) -> Role {
        if self.roles.iter().any(|r| r == "owner") {
            Role::Owner
        } else if self.roles.iter().any(|r| r == "write")
            || self.link.as_ref().is_some_and(|l| l.kind == "edit")
        {
            Role::Editor
        } else {
            Role::Viewer
        }
    }

    /// Its grants: one for a link to anyone, one per person otherwise.
    fn access(self) -> Vec<Access> {
        let role = self.role();
        let inherited = self.inherited_from.is_some();
        if let Some(link) = &self.link {
            if link.scope != "anonymous" {
                // A link for the organisation or for named people only.
                return Vec::new();
            }
            return vec![Access {
                id: self.id,
                who: Who::Anyone,
                role,
                address: String::new(),
                name: String::new(),
                inherited,
                link: link.web_url.clone(),
            }];
        }
        let mut people: Vec<(Who, Identity)> = Vec::new();
        for granted in self
            .granted_to_v2
            .into_iter()
            .chain(self.granted_to_identities_v2)
        {
            if let Some(user) = granted.user.or(granted.site_user) {
                people.push((Who::Person, user));
            } else if let Some(group) = granted.group {
                people.push((Who::Group, group));
            }
        }
        if people.is_empty()
            && let Some(invitation) = self.invitation
        {
            people.push((
                Who::Person,
                Identity {
                    email: invitation.email,
                    ..Identity::default()
                },
            ));
        }
        people.dedup_by(|a, b| a.1.email == b.1.email && a.1.display_name == b.1.display_name);
        people
            .into_iter()
            .map(|(who, identity)| Access {
                id: self.id.clone(),
                who,
                role,
                address: if identity.email.is_empty() {
                    identity.login_name
                } else {
                    identity.email
                },
                name: identity.display_name,
                inherited,
                link: String::new(),
            })
            .collect()
    }
}

/// Graph's role for `role` in an invitation.
fn role_name(role: Role) -> &'static str {
    match role {
        Role::Owner | Role::Editor => "write",
        Role::Commenter | Role::Viewer => "read",
    }
}

impl OneDrive {
    /// Who may open item `id`, the owner first.
    pub async fn access(&self, id: &str) -> Result<Vec<Access>> {
        let url = format!("{}/permissions", self.item_url(id));
        let reply = self.call("GET", &url, None).await?;
        check(&reply, "reading who has access")?;
        let listed: Permissions = parse(&reply.body)?;
        let mut all: Vec<Access> = listed
            .value
            .into_iter()
            .flat_map(Permission::access)
            .collect();
        all.sort_by_key(|a| (a.role != Role::Owner, a.who == Who::Anyone));
        Ok(all)
    }

    /// Invites each of `addresses` to open item `id` as `role`, with
    /// Microsoft's own email when `notify`. Returns the addresses
    /// OneDrive would not share with.
    pub async fn grant(
        &self,
        id: &str,
        addresses: &[String],
        role: Role,
        notify: bool,
    ) -> Result<Vec<String>> {
        let url = format!("{}/invite", self.item_url(id));
        let mut refused = Vec::new();
        for address in addresses {
            let body = serde_json::json!({
                "recipients": [{ "email": address }],
                "requireSignIn": true,
                "sendInvitation": notify,
                "roles": [role_name(role)],
            })
            .to_string();
            let reply = self.call("POST", &url, Some(body.as_bytes())).await?;
            match reply.status {
                200 | 201 => {}
                207 | 400 => refused.push(address.clone()),
                403 if is_sharing(&reply.body) => refused.push(address.clone()),
                _ => check(&reply, "sharing")?,
            }
        }
        Ok(refused)
    }

    /// Changes grant `permission` of item `id` to `role`, or takes it
    /// away (`None`).
    pub async fn set_access(&self, id: &str, permission: &str, role: Option<Role>) -> Result<()> {
        let url = format!(
            "{}/permissions/{}",
            self.item_url(id),
            crate::autoconfig::http::escape(permission)
        );
        let reply = match role {
            Some(role) => {
                let body = serde_json::json!({ "roles": [role_name(role)] }).to_string();
                self.call("PATCH", &url, Some(body.as_bytes())).await?
            }
            None => self.call("DELETE", &url, None).await?,
        };
        check(&reply, "changing access")
    }

    /// Lets anyone with the link open item `id` as `role`, or (`None`)
    /// only the people it is shared with.
    pub async fn set_link(&self, id: &str, role: Option<Role>) -> Result<()> {
        let kind = role.map(|r| if r == Role::Editor { "edit" } else { "view" });
        // Links of the other kind go; one of the kind asked for stays.
        let links: Vec<Access> = self
            .access(id)
            .await?
            .into_iter()
            .filter(|a| a.who == Who::Anyone && !a.inherited)
            .collect();
        let mut kept = false;
        for link in &links {
            let same = kind
                == Some(if link.role == Role::Editor {
                    "edit"
                } else {
                    "view"
                });
            if same && !kept {
                kept = true;
            } else {
                self.set_access(id, &link.id, None).await?;
            }
        }
        let Some(kind) = kind.filter(|_| !kept) else {
            return Ok(());
        };
        let url = format!("{}/createLink", self.item_url(id));
        let body = serde_json::json!({ "type": kind, "scope": "anonymous" }).to_string();
        let reply = self.call("POST", &url, Some(body.as_bytes())).await?;
        check(&reply, "sharing the link")
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

    const LISTED: &str = r#"{"value": [
        {"id": "l1", "roles": ["read"], "link": {"scope": "anonymous", "type": "view",
         "webUrl": "https://1drv.test/abc"}},
        {"id": "p2", "roles": ["write"], "grantedToIdentitiesV2": [{"user":
         {"displayName": "Marco Ruiz", "email": "marco@northwind.example"}}]},
        {"id": "p1", "roles": ["owner"], "grantedToV2": {"user":
         {"displayName": "Priya Demo", "email": "priya@northwind.example"}}},
        {"id": "l2", "roles": ["read"], "link": {"scope": "organization", "type": "view"}}
    ]}"#;

    #[test]
    fn lists_who_has_access_owner_first() {
        let (api, _) = serve(|_, _| (200, Vec::new(), LISTED.into()));
        let all = smol::block_on(onedrive(&api).access("p9")).unwrap();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].role, Role::Owner);
        assert_eq!(all[1].name, "Marco Ruiz");
        assert_eq!(all[1].role, Role::Editor);
        assert_eq!(all[2].who, Who::Anyone);
        assert_eq!(all[2].link, "https://1drv.test/abc");
    }

    #[test]
    fn invites_as_a_role() {
        let (api, seen) = serve(|_, _| (200, Vec::new(), r#"{"value": []}"#.into()));
        smol::block_on(onedrive(&api).grant(
            "p9",
            &["ana@northwind.example".into()],
            Role::Editor,
            false,
        ))
        .unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen[0].path, "/me/drive/items/p9/invite");
        let body: serde_json::Value = serde_json::from_slice(&seen[0].body).unwrap();
        assert_eq!(body["roles"], serde_json::json!(["write"]));
        assert_eq!(body["sendInvitation"], false);
    }

    #[test]
    fn an_edit_link_replaces_a_view_link() {
        let (api, seen) = serve(|request, _| {
            if request.method == "GET" {
                (200, Vec::new(), LISTED.into())
            } else {
                (200, Vec::new(), "{}".into())
            }
        });
        smol::block_on(onedrive(&api).set_link("p9", Some(Role::Editor))).unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen[1].method, "DELETE");
        assert_eq!(seen[1].path, "/me/drive/items/p9/permissions/l1");
        assert_eq!(seen[2].path, "/me/drive/items/p9/createLink");
        assert!(seen[2].text().contains(r#""type":"edit""#));
    }
}
