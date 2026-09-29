// SPDX-License-Identifier: GPL-3.0-or-later

//! Contacts from each account's own address book (`docs/ARCHITECTURE.md`
//! §8.6): Google's People API for Gmail, Microsoft Graph for Outlook, and
//! CardDAV ([`crate::carddav`]) for the rest. Each read ends in a
//! [`katna_store::BookSync`] the daemon saves.

use std::sync::Arc;
use std::time::Duration;

use katna_core::contact::{Card, Name, PostalAddress, Typed};
use katna_store::{BookSync, SyncedContact, SyncedGroup};
use serde::Deserialize;

use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    net::Tls,
    oauth::{GOOGLE_CONTACTS, GOOGLE_OTHER_CONTACTS, MICROSOFT_CONTACTS, TokenSource},
};

/// Google's People API host.
pub const PEOPLE_API: &str = "https://people.googleapis.com";

/// Microsoft Graph.
pub const GRAPH_API: &str = "https://graph.microsoft.com/v1.0";

/// How long one request may take.
const TIMEOUT: Duration = Duration::from_secs(60);

/// Largest answer read: a page of contacts, or a picture.
const MAX_ANSWER: usize = 16 * 1024 * 1024;

/// Contacts on one page of Google's listing (up to 1000 allowed); pages
/// stay a few hundred kilobytes.
const PAGE: u32 = 500;

/// The fields of a person Katna reads.
const PERSON_FIELDS: &str = "names,nicknames,emailAddresses,phoneNumbers,addresses,\
organizations,birthdays,urls,biographies,photos,memberships,metadata";

/// The fields of an "other contact" Katna reads (all Google gives).
const OTHER_FIELDS: &str = "names,emailAddresses,phoneNumbers,photos,metadata";

/// What Google copies when an other contact is saved.
const COPY_FIELDS: &str = "names,emailAddresses,phoneNumbers";

/// The fields Katna writes; memberships (labels, the star) are left alone.
const UPDATE_FIELDS: &str = "names,nicknames,emailAddresses,phoneNumbers,addresses,\
organizations,birthdays,urls,biographies";

/// Google's group of starred contacts.
const STARRED: &str = "contactGroups/starred";

/// Google's own groups, which Google Contacts does not show as labels.
const SYSTEM_GROUPS: [&str; 8] = [
    "contactGroups/myContacts",
    "contactGroups/starred",
    "contactGroups/friends",
    "contactGroups/family",
    "contactGroups/coworkers",
    "contactGroups/chatBuddies",
    "contactGroups/all",
    "contactGroups/blocked",
];

/// One Google account's contacts.
#[derive(Clone)]
pub struct GoogleContacts {
    tokens: Arc<TokenSource>,
    tls: Tls,
    api: String,
}

impl GoogleContacts {
    /// Google's People API, or the server under test in
    /// `KATNA_PEOPLE_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_PEOPLE_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(PEOPLE_API))
    }

    /// Talks to `api` instead of Google, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna into its contacts.
    /// Accounts signed in before Katna asked have to allow it.
    pub async fn allowed(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_CONTACTS).await
    }

    async fn get(&self, url: &str) -> Result<Reply> {
        self.send("GET", url, None).await
    }

    async fn send(&self, method: &str, url: &str, json: Option<&[u8]>) -> Result<Reply> {
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let reply = http::exchange_limited(
                method,
                url,
                &[("Authorization", token.as_str())],
                json.map(|body| ("application/json", body)),
                None,
                &self.tls,
                TIMEOUT,
                MAX_ANSWER,
            )
            .await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    /// Reads the contacts changed since `sync_token`, or all of them
    /// without one or when Google no longer knows it.
    pub async fn sync(&self, sync_token: Option<&str>) -> Result<BookSync> {
        let groups = self.groups().await?;
        match self.connections(sync_token).await {
            Err(Error::Rejected(e)) if sync_token.is_some() && e.contains("410") => {
                tracing::info!("contacts: Google's sync token ran out; reading all again");
                self.connections(None).await
            }
            other => other,
        }
        .map(|mut sync| {
            sync.groups = Some(groups);
            sync.groups_complete = true;
            sync
        })
    }

    /// Saves `card` as a new contact, or over `remote_id` last read with
    /// `etag`; returns the contact as Google keeps it. Labels and the star
    /// stay as they are.
    pub async fn save(
        &self,
        remote_id: Option<&str>,
        etag: Option<&str>,
        card: &Card,
    ) -> Result<SyncedContact> {
        let mut person = person_json(card);
        let (method, url) = match remote_id {
            Some(id) => {
                person["etag"] = serde_json::Value::from(etag.unwrap_or_default());
                (
                    "PATCH",
                    format!(
                        "{}/v1/{id}:updateContact?updatePersonFields={UPDATE_FIELDS}\
                         &personFields={PERSON_FIELDS}",
                        self.api
                    ),
                )
            }
            None => (
                "POST",
                format!(
                    "{}/v1/people:createContact?personFields={PERSON_FIELDS}",
                    self.api
                ),
            ),
        };
        let body = person.to_string();
        let reply = self.send(method, &url, Some(body.as_bytes())).await?;
        check(&reply, "saving a contact")?;
        let person: Person = parse(&reply.body)?;
        Ok(person.into_synced())
    }

    /// Deletes contact `remote_id`; Google keeps it in its Trash for 30
    /// days. One already gone is fine.
    pub async fn delete(&self, remote_id: &str) -> Result<()> {
        let url = format!("{}/v1/{remote_id}:deleteContact", self.api);
        let reply = self.send("DELETE", &url, None).await?;
        if reply.status == 404 {
            return Ok(());
        }
        check(&reply, "deleting a contact")
    }

    /// Contact `remote_id` as Google keeps it now.
    pub async fn person(&self, remote_id: &str) -> Result<SyncedContact> {
        let url = format!("{}/v1/{remote_id}?personFields={PERSON_FIELDS}", self.api);
        let reply = self.get(&url).await?;
        check(&reply, "reading a contact")?;
        let person: Person = parse(&reply.body)?;
        Ok(person.into_synced())
    }

    /// Makes a label called `name`.
    pub async fn create_group(&self, name: &str) -> Result<SyncedGroup> {
        let url = format!("{}/v1/contactGroups", self.api);
        let body = serde_json::json!({ "contactGroup": { "name": name } }).to_string();
        let reply = self.send("POST", &url, Some(body.as_bytes())).await?;
        check(&reply, "making a label")?;
        let group: GroupReply = parse(&reply.body)?;
        Ok(group.into_synced())
    }

    /// Renames label `group`.
    pub async fn rename_group(&self, group: &str, name: &str) -> Result<SyncedGroup> {
        let url = format!("{}/v1/{group}", self.api);
        // Google wants the label's current etag with the change.
        let reply = self.get(&url).await?;
        check(&reply, "reading a label")?;
        let current: GroupReply = parse(&reply.body)?;
        let body = serde_json::json!({
            "contactGroup": { "name": name, "etag": current.etag },
            "updateGroupFields": "name",
        })
        .to_string();
        let reply = self.send("PUT", &url, Some(body.as_bytes())).await?;
        check(&reply, "renaming a label")?;
        let group: GroupReply = parse(&reply.body)?;
        Ok(group.into_synced())
    }

    /// Deletes label `group`; its people stay. One already gone is fine.
    pub async fn delete_group(&self, group: &str) -> Result<()> {
        let url = format!("{}/v1/{group}?deleteContacts=false", self.api);
        let reply = self.send("DELETE", &url, None).await?;
        if reply.status == 404 {
            return Ok(());
        }
        check(&reply, "deleting a label")
    }

    /// Adds people to label `group` and takes others off it.
    pub async fn modify_group(&self, group: &str, add: &[&str], remove: &[&str]) -> Result<()> {
        let url = format!("{}/v1/{group}/members:modify", self.api);
        let body = serde_json::json!({
            "resourceNamesToAdd": add,
            "resourceNamesToRemove": remove,
        })
        .to_string();
        let reply = self.send("POST", &url, Some(body.as_bytes())).await?;
        check(&reply, "changing a label")
    }

    /// Whether the account's sign-in allowed Katna to read its other
    /// contacts.
    pub async fn other_allowed(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_OTHER_CONTACTS).await
    }

    /// Reads the other contacts (people mailed but not saved) changed
    /// since `sync_token`, or all of them without one or when Google no
    /// longer knows it.
    pub async fn other_contacts(&self, sync_token: Option<&str>) -> Result<BookSync> {
        match self.other_pages(sync_token).await {
            Err(Error::Rejected(e)) if sync_token.is_some() && e.contains("410") => {
                tracing::info!("contacts: Google's other contacts token ran out; reading all");
                self.other_pages(None).await
            }
            other => other,
        }
    }

    /// Saves other contact `remote_id` in the user's contacts; returns the
    /// new contact.
    pub async fn copy_other(&self, remote_id: &str) -> Result<SyncedContact> {
        let url = format!(
            "{}/v1/{remote_id}:copyOtherContactToMyContactsGroup",
            self.api
        );
        let body = serde_json::json!({ "copyMask": COPY_FIELDS }).to_string();
        let reply = self.send("POST", &url, Some(body.as_bytes())).await?;
        check(&reply, "saving an other contact")?;
        let person: Person = parse(&reply.body)?;
        // The copy's answer leaves out labels and the like: read it whole.
        self.person(&person.resource_name).await
    }

    async fn other_pages(&self, sync_token: Option<&str>) -> Result<BookSync> {
        #[derive(Deserialize)]
        struct Page {
            #[serde(default, rename = "otherContacts")]
            people: Vec<Person>,
            #[serde(rename = "nextPageToken")]
            next: Option<String>,
            #[serde(rename = "nextSyncToken")]
            sync: Option<String>,
        }
        let mut out = BookSync {
            full: sync_token.is_none(),
            ..BookSync::default()
        };
        let mut page_token: Option<String> = None;
        loop {
            let mut url = format!(
                "{}/v1/otherContacts?pageSize=1000&readMask={OTHER_FIELDS}&requestSyncToken=true",
                self.api
            );
            if let Some(token) = sync_token {
                url.push_str(&format!("&syncToken={}", http::escape(token)));
            }
            if let Some(token) = &page_token {
                url.push_str(&format!("&pageToken={}", http::escape(token)));
            }
            let reply = self.get(&url).await?;
            check(&reply, "reading other contacts")?;
            let page: Page = parse(&reply.body)?;
            for person in page.people {
                if person.metadata.deleted {
                    out.deleted.push(person.resource_name);
                } else {
                    out.contacts.push(person.into_synced());
                }
            }
            if page.sync.is_some() {
                out.sync_token = page.sync;
            }
            match page.next {
                Some(next) if !next.is_empty() => page_token = Some(next),
                _ => return Ok(out),
            }
        }
    }

    /// The user's own labels (not Google's system groups).
    async fn groups(&self) -> Result<Vec<SyncedGroup>> {
        #[derive(Deserialize)]
        struct Page {
            #[serde(default, rename = "contactGroups")]
            groups: Vec<Group>,
            #[serde(rename = "nextPageToken")]
            next: Option<String>,
        }
        #[derive(Deserialize)]
        struct Group {
            #[serde(rename = "resourceName")]
            resource: String,
            #[serde(default)]
            name: String,
            #[serde(default, rename = "formattedName")]
            formatted: String,
            #[serde(default, rename = "groupType")]
            kind: String,
        }
        let mut out = Vec::new();
        let mut page_token: Option<String> = None;
        loop {
            let mut url = format!(
                "{}/v1/contactGroups?pageSize=1000&groupFields=name,groupType",
                self.api
            );
            if let Some(token) = &page_token {
                url.push_str(&format!("&pageToken={}", http::escape(token)));
            }
            let reply = self.get(&url).await?;
            check(&reply, "reading labels")?;
            let page: Page = parse(&reply.body)?;
            out.extend(
                page.groups
                    .into_iter()
                    .filter(|g| g.kind == "USER_CONTACT_GROUP")
                    .map(|g| SyncedGroup {
                        remote_id: g.resource,
                        name: if g.formatted.is_empty() {
                            g.name
                        } else {
                            g.formatted
                        },
                    }),
            );
            match page.next {
                Some(next) if !next.is_empty() => page_token = Some(next),
                _ => return Ok(out),
            }
        }
    }

    async fn connections(&self, sync_token: Option<&str>) -> Result<BookSync> {
        #[derive(Deserialize)]
        struct Page {
            #[serde(default)]
            connections: Vec<Person>,
            #[serde(rename = "nextPageToken")]
            next: Option<String>,
            #[serde(rename = "nextSyncToken")]
            sync: Option<String>,
        }
        let mut out = BookSync {
            full: sync_token.is_none(),
            ..BookSync::default()
        };
        let mut page_token: Option<String> = None;
        loop {
            let mut url = format!(
                "{}/v1/people/me/connections?pageSize={PAGE}&personFields={PERSON_FIELDS}\
                 &requestSyncToken=true",
                self.api
            );
            if let Some(token) = sync_token {
                url.push_str(&format!("&syncToken={}", http::escape(token)));
            }
            if let Some(token) = &page_token {
                url.push_str(&format!("&pageToken={}", http::escape(token)));
            }
            let reply = self.get(&url).await?;
            check(&reply, "reading contacts")?;
            let page: Page = parse(&reply.body)?;
            for person in page.connections {
                if person.metadata.deleted {
                    out.deleted.push(person.resource_name);
                } else {
                    out.contacts.push(person.into_synced());
                }
            }
            if page.sync.is_some() {
                out.sync_token = page.sync;
            }
            match page.next {
                Some(next) if !next.is_empty() => page_token = Some(next),
                _ => return Ok(out),
            }
        }
    }
}

/// Fetches a contact's picture from the URL its card gave (Google's
/// pictures need no token).
pub async fn photo_at(url: &str, tls: &Tls) -> Result<Vec<u8>> {
    let reply =
        http::exchange_limited("GET", url, &[], None, None, tls, TIMEOUT, MAX_ANSWER).await?;
    check(&reply, "fetching a picture")?;
    Ok(reply.body)
}

/// A person as the People API gives it.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Person {
    resource_name: String,
    etag: Option<String>,
    metadata: PersonMetadata,
    names: Vec<PersonName>,
    nicknames: Vec<Value>,
    email_addresses: Vec<Value>,
    phone_numbers: Vec<Value>,
    addresses: Vec<Address>,
    organizations: Vec<Organization>,
    birthdays: Vec<Birthday>,
    urls: Vec<Value>,
    biographies: Vec<Value>,
    photos: Vec<Photo>,
    memberships: Vec<Membership>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct PersonMetadata {
    deleted: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct PersonName {
    display_name: String,
    given_name: String,
    middle_name: String,
    family_name: String,
    honorific_prefix: String,
    honorific_suffix: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Value {
    value: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Address {
    #[serde(rename = "type")]
    kind: String,
    po_box: String,
    street_address: String,
    extended_address: String,
    city: String,
    region: String,
    postal_code: String,
    country: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Organization {
    name: String,
    department: String,
    title: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Birthday {
    date: Option<Date>,
    text: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Photo {
    url: String,
    /// Google's drawn letter, not a real picture.
    default: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Membership {
    contact_group_membership: Option<GroupMembership>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct GroupMembership {
    contact_group_resource_name: String,
}

/// Google's kinds, in lower case, `mobile` as Katna says it.
fn kind(google: &str) -> String {
    match google.to_ascii_lowercase().as_str() {
        "" => String::new(),
        "mobile" | "cell" => "mobile".into(),
        other => other.to_owned(),
    }
}

impl Person {
    fn into_synced(self) -> SyncedContact {
        let name = self.names.into_iter().next().unwrap_or_default();
        let organization = self.organizations.into_iter().next().unwrap_or_default();
        let groups: Vec<String> = self
            .memberships
            .into_iter()
            .filter_map(|m| m.contact_group_membership)
            .map(|m| m.contact_group_resource_name)
            .collect();
        let birthday = self
            .birthdays
            .into_iter()
            .next()
            .map(|b| match b.date {
                Some(d) if d.month > 0 && d.day > 0 && d.year > 0 => {
                    format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
                }
                Some(d) if d.month > 0 && d.day > 0 => format!("--{:02}-{:02}", d.month, d.day),
                _ => b.text,
            })
            .unwrap_or_default();
        let typed = |list: Vec<Value>| -> Vec<Typed> {
            list.into_iter()
                .filter(|v| !v.value.trim().is_empty())
                .map(|v| Typed::new(v.value.trim(), kind(&v.kind)))
                .collect()
        };
        let card = Card {
            name: Name {
                full: name.display_name,
                prefix: name.honorific_prefix,
                given: name.given_name,
                middle: name.middle_name,
                family: name.family_name,
                suffix: name.honorific_suffix,
            },
            nickname: self
                .nicknames
                .into_iter()
                .next()
                .map(|n| n.value)
                .unwrap_or_default(),
            emails: typed(self.email_addresses),
            phones: typed(self.phone_numbers),
            addresses: self
                .addresses
                .into_iter()
                .map(|a| PostalAddress {
                    kind: kind(&a.kind),
                    street: [a.po_box, a.street_address, a.extended_address]
                        .into_iter()
                        .filter(|p| !p.trim().is_empty())
                        .collect::<Vec<_>>()
                        .join("\n"),
                    city: a.city,
                    region: a.region,
                    postcode: a.postal_code,
                    country: a.country,
                })
                .filter(|a| !a.is_empty())
                .collect(),
            organization: organization.name,
            department: organization.department,
            title: organization.title,
            birthday,
            urls: typed(self.urls),
            note: self
                .biographies
                .into_iter()
                .next()
                .map(|b| b.value)
                .unwrap_or_default(),
            photo_url: self
                .photos
                .into_iter()
                .find(|p| !p.default && !p.url.is_empty())
                .map(|p| p.url)
                .unwrap_or_default(),
        };
        SyncedContact {
            remote_id: self.resource_name,
            etag: self.etag,
            card,
            raw: None,
            starred: groups.iter().any(|g| g == STARRED),
            groups: groups
                .into_iter()
                .filter(|g| !SYSTEM_GROUPS.contains(&g.as_str()))
                .collect(),
            photo: None,
        }
    }
}

/// A label as Google answers a change to it.
#[derive(Deserialize)]
struct GroupReply {
    #[serde(rename = "resourceName")]
    resource: String,
    #[serde(default)]
    etag: String,
    #[serde(default)]
    name: String,
    #[serde(default, rename = "formattedName")]
    formatted: String,
}

impl GroupReply {
    fn into_synced(self) -> SyncedGroup {
        SyncedGroup {
            remote_id: self.resource,
            name: if self.formatted.is_empty() {
                self.name
            } else {
                self.formatted
            },
        }
    }
}

/// One Microsoft account's contacts (its main contacts folder).
#[derive(Clone)]
pub struct MicrosoftContacts {
    tokens: Arc<TokenSource>,
    tls: Tls,
    api: String,
}

/// The fields of a Graph contact Katna reads.
const GRAPH_FIELDS: &str = "id,changeKey,displayName,givenName,middleName,surname,title,\
generation,nickName,emailAddresses,mobilePhone,homePhones,businessPhones,homeAddress,\
businessAddress,otherAddress,companyName,department,jobTitle,birthday,businessHomePage,\
personalNotes,categories";

impl MicrosoftContacts {
    /// Microsoft Graph, or the server under test in `KATNA_GRAPH_API_URL`.
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

    /// Whether the sign-in allowed Katna into the account's contacts.
    pub async fn allowed(&self) -> Result<bool> {
        match self.tokens.access_token_for(MICROSOFT_CONTACTS).await {
            Ok(_) => Ok(true),
            Err(Error::Auth(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get(&self, url: &str) -> Result<Reply> {
        self.send("GET", url, None).await
    }

    async fn send(&self, method: &str, url: &str, json: Option<&[u8]>) -> Result<Reply> {
        let mut retried = false;
        loop {
            let token = format!(
                "Bearer {}",
                self.tokens.access_token_for(MICROSOFT_CONTACTS).await?
            );
            let reply = http::exchange_limited(
                method,
                url,
                &[("Authorization", token.as_str())],
                json.map(|body| ("application/json", body)),
                None,
                &self.tls,
                TIMEOUT,
                MAX_ANSWER,
            )
            .await?;
            if reply.status == 401 && !retried {
                retried = true;
                self.tokens.forget_access_token_for(MICROSOFT_CONTACTS);
                continue;
            }
            return Ok(reply);
        }
    }

    /// Saves `card` as a new contact, or over `remote_id`; returns the
    /// contact as Microsoft keeps it. Categories stay as they are.
    pub async fn save(&self, remote_id: Option<&str>, card: &Card) -> Result<SyncedContact> {
        let body = graph_json(card).to_string();
        let reply = match remote_id {
            Some(id) => {
                let url = format!("{}/me/contacts/{id}", self.api);
                self.send("PATCH", &url, Some(body.as_bytes())).await?
            }
            None => {
                let url = format!("{}/me/contacts", self.api);
                self.send("POST", &url, Some(body.as_bytes())).await?
            }
        };
        check(&reply, "saving a contact")?;
        let contact: GraphContact = parse(&reply.body)?;
        Ok(contact.into_synced())
    }

    /// Sets the categories (labels) of contact `remote_id`; returns the
    /// contact as Microsoft keeps it.
    pub async fn set_categories(
        &self,
        remote_id: &str,
        categories: &[String],
    ) -> Result<SyncedContact> {
        let url = format!("{}/me/contacts/{remote_id}", self.api);
        let body = serde_json::json!({ "categories": categories }).to_string();
        let reply = self.send("PATCH", &url, Some(body.as_bytes())).await?;
        check(&reply, "changing labels")?;
        let contact: GraphContact = parse(&reply.body)?;
        Ok(contact.into_synced())
    }

    /// Deletes contact `remote_id` (to the account's Deleted Items). One
    /// already gone is fine.
    pub async fn delete(&self, remote_id: &str) -> Result<()> {
        let url = format!("{}/me/contacts/{remote_id}", self.api);
        let reply = self.send("DELETE", &url, None).await?;
        if reply.status == 404 {
            return Ok(());
        }
        check(&reply, "deleting a contact")
    }

    /// Reads every contact of the main folder. Categories are the labels.
    pub async fn sync(&self) -> Result<BookSync> {
        #[derive(Deserialize)]
        struct Page {
            #[serde(default)]
            value: Vec<GraphContact>,
            #[serde(rename = "@odata.nextLink")]
            next: Option<String>,
        }
        let mut out = BookSync {
            full: true,
            ..BookSync::default()
        };
        let mut labels: Vec<String> = Vec::new();
        let mut url = format!("{}/me/contacts?$top=500&$select={GRAPH_FIELDS}", self.api);
        loop {
            let reply = self.get(&url).await?;
            check(&reply, "reading contacts")?;
            let page: Page = parse(&reply.body)?;
            for contact in page.value {
                for category in &contact.categories {
                    if !labels.contains(category) {
                        labels.push(category.clone());
                    }
                }
                out.contacts.push(contact.into_synced());
            }
            match page.next {
                // Only links back to the same service are followed.
                Some(next) if next.starts_with(&self.api) => url = next,
                _ => break,
            }
        }
        out.groups_complete = true;
        out.groups = Some(
            labels
                .into_iter()
                .map(|name| SyncedGroup {
                    remote_id: name.clone(),
                    name,
                })
                .collect(),
        );
        Ok(out)
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct GraphContact {
    id: String,
    change_key: Option<String>,
    display_name: Option<String>,
    given_name: Option<String>,
    middle_name: Option<String>,
    surname: Option<String>,
    title: Option<String>,
    generation: Option<String>,
    nick_name: Option<String>,
    email_addresses: Vec<GraphEmail>,
    mobile_phone: Option<String>,
    home_phones: Vec<String>,
    business_phones: Vec<String>,
    home_address: Option<GraphAddress>,
    business_address: Option<GraphAddress>,
    other_address: Option<GraphAddress>,
    company_name: Option<String>,
    department: Option<String>,
    job_title: Option<String>,
    birthday: Option<String>,
    business_home_page: Option<String>,
    personal_notes: Option<String>,
    categories: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct GraphEmail {
    address: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct GraphAddress {
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country_or_region: Option<String>,
}

impl GraphContact {
    fn into_synced(self) -> SyncedContact {
        let s = |v: Option<String>| v.unwrap_or_default().trim().to_owned();
        let mut phones = Vec::new();
        if let Some(mobile) = self.mobile_phone.filter(|p| !p.trim().is_empty()) {
            phones.push(Typed::new(mobile.trim(), "mobile"));
        }
        phones.extend(
            self.business_phones
                .into_iter()
                .map(|p| Typed::new(p, "work")),
        );
        phones.extend(self.home_phones.into_iter().map(|p| Typed::new(p, "home")));
        phones.retain(|p| !p.value.trim().is_empty());
        let addresses = [
            ("work", self.business_address),
            ("home", self.home_address),
            ("other", self.other_address),
        ]
        .into_iter()
        .filter_map(|(kind, a)| {
            let a = a?;
            let address = PostalAddress {
                kind: kind.into(),
                street: s(a.street),
                city: s(a.city),
                region: s(a.state),
                postcode: s(a.postal_code),
                country: s(a.country_or_region),
            };
            (!address.is_empty()).then_some(address)
        })
        .collect();
        let card = Card {
            name: Name {
                full: s(self.display_name),
                prefix: s(self.title),
                given: s(self.given_name),
                middle: s(self.middle_name),
                family: s(self.surname),
                suffix: s(self.generation),
            },
            nickname: s(self.nick_name),
            emails: self
                .email_addresses
                .into_iter()
                .filter_map(|e| e.address)
                .filter(|a| !a.trim().is_empty())
                .map(|a| Typed::new(a.trim(), ""))
                .collect(),
            phones,
            addresses,
            organization: s(self.company_name),
            department: s(self.department),
            title: s(self.job_title),
            birthday: self
                .birthday
                .map(|b| b.chars().take(10).collect())
                .unwrap_or_default(),
            urls: self
                .business_home_page
                .filter(|u| !u.trim().is_empty())
                .map(|u| vec![Typed::new(u.trim(), "work")])
                .unwrap_or_default(),
            note: s(self.personal_notes),
            photo_url: String::new(),
        };
        SyncedContact {
            remote_id: self.id,
            etag: self.change_key,
            card,
            raw: None,
            starred: false,
            groups: self.categories,
            photo: None,
        }
    }
}

/// Splits `YYYY-MM-DD` or `--MM-DD` into year (0 when unknown), month and
/// day.
fn birthday_parts(text: &str) -> Option<(u32, u32, u32)> {
    let (year, rest) = match text.strip_prefix("--") {
        Some(rest) => (0, rest),
        None => {
            let (year, rest) = text.split_once('-')?;
            (year.parse().ok()?, rest)
        }
    };
    let (month, day) = rest.split_once('-')?;
    let day: String = day.chars().take_while(char::is_ascii_digit).collect();
    let (month, day) = (month.parse().ok()?, day.parse().ok()?);
    (1..=12).contains(&month).then_some((year, month, day))
}

/// `card` as a People API person, with every field Katna writes, empty
/// ones as empty lists so they are cleared.
fn person_json(card: &Card) -> serde_json::Value {
    use serde_json::{Value as J, json};
    let typed = |list: &[Typed]| -> J {
        list.iter()
            .map(|t| json!({"value": t.value, "type": t.kind}))
            .collect()
    };
    let n = &card.name;
    let names = if n.given.is_empty() && n.family.is_empty() && n.middle.is_empty() {
        if n.full.is_empty() {
            json!([])
        } else {
            json!([{"unstructuredName": n.full}])
        }
    } else {
        json!([{
            "givenName": n.given,
            "middleName": n.middle,
            "familyName": n.family,
            "honorificPrefix": n.prefix,
            "honorificSuffix": n.suffix,
        }])
    };
    let organizations =
        if card.organization.is_empty() && card.department.is_empty() && card.title.is_empty() {
            json!([])
        } else {
            json!([{"name": card.organization, "department": card.department, "title": card.title}])
        };
    let birthdays = match birthday_parts(&card.birthday) {
        Some((year, month, day)) if year > 0 => {
            json!([{"date": {"year": year, "month": month, "day": day}}])
        }
        Some((_, month, day)) => json!([{"date": {"month": month, "day": day}}]),
        None if card.birthday.is_empty() => json!([]),
        None => json!([{"text": card.birthday}]),
    };
    json!({
        "names": names,
        "nicknames": if card.nickname.is_empty() { json!([]) } else { json!([{"value": card.nickname}]) },
        "emailAddresses": typed(&card.emails),
        "phoneNumbers": typed(&card.phones),
        "addresses": card.addresses.iter().map(|a| json!({
            "type": a.kind,
            "streetAddress": a.street,
            "city": a.city,
            "region": a.region,
            "postalCode": a.postcode,
            "country": a.country,
        })).collect::<J>(),
        "organizations": organizations,
        "birthdays": birthdays,
        "urls": typed(&card.urls),
        "biographies": if card.note.is_empty() {
            json!([])
        } else {
            json!([{"value": card.note, "contentType": "TEXT_PLAIN"}])
        },
    })
}

/// `card` as a Graph contact. Graph has one mobile phone, business and
/// home phones, and a home, business and other address.
fn graph_json(card: &Card) -> serde_json::Value {
    use serde_json::{Value as J, json};
    let n = &card.name;
    let mut mobile = String::new();
    let mut business: Vec<&str> = Vec::new();
    let mut home: Vec<&str> = Vec::new();
    for phone in &card.phones {
        match phone.kind.as_str() {
            "mobile" if mobile.is_empty() => mobile = phone.value.clone(),
            "home" => home.push(&phone.value),
            _ => business.push(&phone.value),
        }
    }
    let address = |kind: &str| -> J {
        card.addresses
            .iter()
            .find(|a| match kind {
                "home" => a.kind == "home",
                "work" => a.kind == "work",
                _ => a.kind != "home" && a.kind != "work",
            })
            .map_or_else(
                || json!({}),
                |a| {
                    json!({
                        "street": a.street,
                        "city": a.city,
                        "state": a.region,
                        "postalCode": a.postcode,
                        "countryOrRegion": a.country,
                    })
                },
            )
    };
    let birthday = match birthday_parts(&card.birthday) {
        Some((year, month, day)) if year > 0 => {
            J::from(format!("{year:04}-{month:02}-{day:02}T11:59:00Z"))
        }
        _ => J::Null,
    };
    json!({
        "displayName": card.display_name(),
        "givenName": n.given,
        "middleName": n.middle,
        "surname": n.family,
        "title": n.prefix,
        "generation": n.suffix,
        "nickName": card.nickname,
        "emailAddresses": card.emails.iter().map(|e| json!({"address": e.value, "name": card.display_name()})).collect::<J>(),
        "mobilePhone": mobile,
        "businessPhones": business,
        "homePhones": home,
        "homeAddress": address("home"),
        "businessAddress": address("work"),
        "otherAddress": address("other"),
        "companyName": card.organization,
        "department": card.department,
        "jobTitle": card.title,
        "birthday": birthday,
        "businessHomePage": card.urls.first().map_or("", |u| u.value.as_str()),
        "personalNotes": card.note,
    })
}

fn parse<'a, T: Deserialize<'a>>(body: &'a [u8]) -> Result<T> {
    serde_json::from_slice(body).map_err(|e| Error::Protocol(format!("contacts: {e}")))
}

/// A `2xx` answer is fine. A refused token or permission asks the user to
/// allow contacts ([`Error::Auth`]); anything else is the service's own
/// message with its status.
fn check(reply: &Reply, doing: &str) -> Result<()> {
    if (200..300).contains(&reply.status) {
        return Ok(());
    }
    let body = String::from_utf8_lossy(&reply.body);
    let scope = body.contains("ACCESS_TOKEN_SCOPE_INSUFFICIENT")
        || body.contains("insufficientPermissions")
        || body.contains("ErrorAccessDenied");
    if reply.status == 401 || (reply.status == 403 && scope) {
        return Err(Error::Auth(format!("contacts refused while {doing}")));
    }
    let message: String = body.chars().take(300).collect();
    Err(Error::Rejected(format!(
        "contacts, {doing}: status {}: {message}",
        reply.status
    )))
}

#[cfg(test)]
mod tests;
