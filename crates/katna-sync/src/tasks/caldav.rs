// SPDX-License-Identifier: GPL-3.0-or-later

//! To-dos (`VTODO`) on the CalDAV server of an account with a password,
//! found the way its calendars are ([`CalDav`]). Each calendar collection
//! that holds to-dos is a task list; each to-do is a task, and one with a
//! parent (`RELATED-TO;RELTYPE=PARENT`) is a step.
//!
//! A pull asks the list for its `getctag` (or `sync-token`) and, when it
//! changed, for the etags of its to-dos, then downloads only those whose
//! etag changed. What the last pull saw (each to-do's etag and `UID`) is
//! the list's sync state, so a to-do gone from the listing is a deletion.
//! A change is written over the server's own text of the to-do
//! ([`katna_dav::todo::write`]), so what Katna doesn't show stays.

use std::collections::{BTreeMap, HashMap};

use jiff::tz::TimeZone;
use katna_dav::todo::{self, Todo};
use katna_store::tasks::{RemoteTask, RemoteTaskList, Task, TaskExtras};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};

use super::Pull;
use crate::{
    Error, Result,
    calendar::{
        CalendarError,
        caldav::{CALENDARS, CalDav, DavResponse, path, xml_escape},
    },
};

/// To-dos downloaded per `calendar-multiget`.
const MULTIGET: usize = 50;

const MARKER: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<d:propfind xmlns:d="DAV:" xmlns:cs="http://calendarserver.org/ns/"><d:prop><cs:getctag/><d:sync-token/></d:prop></d:propfind>"#;

const ETAGS: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<c:calendar-query xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
<d:prop><d:getetag/></d:prop>
<c:filter><c:comp-filter name="VCALENDAR"><c:comp-filter name="VTODO"/></c:comp-filter></c:filter>
</c:calendar-query>"#;

const ICAL: &str = "text/calendar; charset=utf-8";

/// One account's to-dos on its CalDAV server.
pub struct DavTasks {
    dav: CalDav,
}

/// What the last pull of a list saw.
#[derive(Serialize, Deserialize, Default)]
struct State {
    marker: Option<String>,
    /// Each to-do's href: its etag and `UID`.
    items: BTreeMap<String, (String, String)>,
}

/// A calendar error as a task sync error.
fn failed(err: CalendarError) -> Error {
    match err {
        CalendarError::NeedsSignIn(message) => Error::Auth(message),
        CalendarError::NotEnabled(message) => Error::NotEnabled(message),
        CalendarError::NotOffered => Error::Rejected("the server offers no to-dos".into()),
        CalendarError::Failed(err) => err,
    }
}

/// A random name for a new to-do or list.
fn random_name() -> Result<String> {
    let mut bytes = [0u8; 16];
    SystemRandom::new()
        .fill(&mut bytes)
        .map_err(|_| Error::Protocol("no random bytes".into()))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

fn now() -> i64 {
    jiff::Timestamp::now().as_second()
}

/// `task` as Katna's part of a to-do with `uid`.
fn todo_of(task: &Task, uid: String, parent: Option<String>) -> Todo {
    Todo {
        uid,
        title: task.title.clone(),
        notes: task.notes.clone(),
        due: task.due.clone(),
        due_time: task.due_time,
        done_at: task.done_at,
        parent,
        remind_at: task.remind_at,
        repeat: task.repeat.clone(),
        starred: task.starred,
    }
}

impl DavTasks {
    pub fn new(dav: CalDav) -> Self {
        Self { dav }
    }

    /// The calendar home.
    async fn home(&self) -> Result<String> {
        self.dav
            .home()
            .await
            .map_err(failed)?
            .ok_or_else(|| Error::Rejected("the server has no CalDAV".into()))
    }

    /// The whole URL of `href` on the server.
    async fn url(&self, href: &str) -> Result<String> {
        let home = self.home().await?;
        self.dav
            .absolute(&home, href)
            .ok_or_else(|| Error::Protocol(format!("CalDAV led elsewhere: {href}")))
    }

    /// Why no CalDAV was found, for people; empty when it was.
    pub fn missing_why(&self) -> String {
        self.dav.missing_why()
    }

    /// Whether the server offers CalDAV at all.
    pub async fn allowed(&self) -> Result<bool> {
        match self.dav.home().await {
            Ok(home) => Ok(home.is_some()),
            Err(CalendarError::NotOffered) => Ok(false),
            Err(err) => Err(failed(err)),
        }
    }

    /// The collections that hold to-dos. The first one that holds only
    /// to-dos is the default (a server's own "Tasks"), else the first.
    pub async fn lists(&self) -> Result<Vec<RemoteTaskList>> {
        let home = self.home().await?;
        let Some((_, responses)) = self
            .dav
            .dav("PROPFIND", &home, "1", CALENDARS)
            .await
            .map_err(failed)?
        else {
            return Err(Error::Protocol("the CalDAV calendar home is gone".into()));
        };
        let found: Vec<(RemoteTaskList, bool)> = responses
            .iter()
            .filter(|r| {
                r.prop("resourcetype")
                    .is_some_and(|p| p.inside.iter().any(|n| n == "calendar"))
            })
            .filter_map(|r| {
                let comps = r
                    .prop("supported-calendar-component-set")
                    .map(|p| p.comps.clone())
                    .unwrap_or_default();
                if !comps.is_empty() && !comps.iter().any(|c| c == "VTODO") {
                    return None;
                }
                let remote_id = path(&r.href).to_owned();
                let title = r.text("displayname").map_or_else(
                    || {
                        remote_id
                            .trim_end_matches('/')
                            .rsplit('/')
                            .next()
                            .unwrap_or_default()
                            .to_owned()
                    },
                    str::to_owned,
                );
                let only_todos = comps.iter().all(|c| c == "VTODO") && !comps.is_empty();
                Some((
                    RemoteTaskList {
                        remote_id,
                        title,
                        is_default: false,
                    },
                    only_todos,
                ))
            })
            .collect();
        let default = found.iter().position(|(_, only)| *only).unwrap_or(0);
        Ok(found
            .into_iter()
            .enumerate()
            .map(|(index, (mut list, _))| {
                list.is_default = index == default;
                list
            })
            .collect())
    }

    pub async fn add_list(&self, title: &str) -> Result<String> {
        let home = self.home().await?;
        let url = format!("{}/{}/", home.trim_end_matches('/'), random_name()?);
        let body = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<c:mkcalendar xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav"><d:set><d:prop>
<d:displayname>{}</d:displayname>
<c:supported-calendar-component-set><c:comp name="VTODO"/></c:supported-calendar-component-set>
</d:prop></d:set></c:mkcalendar>"#,
            xml_escape(title)
        );
        let reply = self
            .dav
            .request(
                "MKCALENDAR",
                &url,
                &[],
                Some(("application/xml; charset=utf-8", body.as_bytes())),
            )
            .await?;
        check(reply.status, "adding a task list")?;
        Ok(path(&url).to_owned())
    }

    pub async fn rename_list(&self, id: &str, title: &str) -> Result<()> {
        let url = self.url(id).await?;
        let body = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<d:propertyupdate xmlns:d="DAV:"><d:set><d:prop><d:displayname>{}</d:displayname></d:prop></d:set></d:propertyupdate>"#,
            xml_escape(title)
        );
        let reply = self
            .dav
            .request(
                "PROPPATCH",
                &url,
                &[],
                Some(("application/xml; charset=utf-8", body.as_bytes())),
            )
            .await?;
        check(reply.status, "renaming a task list")
    }

    pub async fn delete_list(&self, id: &str) -> Result<()> {
        let url = self.url(id).await?;
        let reply = self.dav.request("DELETE", &url, &[], None).await?;
        if matches!(reply.status, 404 | 410) {
            return Ok(());
        }
        check(reply.status, "deleting a task list")
    }

    /// The etags and text of the to-dos at `hrefs`.
    async fn fetch(
        &self,
        list_url: &str,
        hrefs: &[String],
    ) -> Result<Vec<(String, String, String)>> {
        let mut out = Vec::new();
        for batch in hrefs.chunks(MULTIGET) {
            let hrefs: String = batch
                .iter()
                .map(|h| format!("<d:href>{}</d:href>", xml_escape(h)))
                .collect();
            let body = format!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<c:calendar-multiget xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
<d:prop><d:getetag/><c:calendar-data/></d:prop>{hrefs}</c:calendar-multiget>"#
            );
            let Some((_, responses)) = self
                .dav
                .dav("REPORT", list_url, "1", &body)
                .await
                .map_err(failed)?
            else {
                return Err(Error::Rejected(
                    "the CalDAV server can't send to-dos".into(),
                ));
            };
            out.extend(responses.iter().filter_map(|r: &DavResponse| {
                let data = r.text("calendar-data")?;
                Some((
                    path(&r.href).to_owned(),
                    r.text("getetag").unwrap_or_default().to_owned(),
                    data.to_owned(),
                ))
            }));
        }
        Ok(out)
    }

    /// The changes of list `list` since `state`, or all of its to-dos.
    pub async fn pull(&self, list: &str, state: Option<&str>) -> Result<Pull> {
        let url = self.url(list).await?;
        let before: Option<State> = state.and_then(|s| serde_json::from_str(s).ok());
        let all = before.is_none();
        let before = before.unwrap_or_default();
        let unchanged = |state: &State| Pull {
            tasks: Vec::new(),
            all: false,
            state: serde_json::to_string(state).ok(),
            steps_of: Vec::new(),
        };

        let marker = match self.dav.dav("PROPFIND", &url, "0", MARKER).await {
            Ok(Some((_, responses))) => responses
                .iter()
                .find_map(|r| r.text("getctag").or_else(|| r.text("sync-token")))
                .map(str::to_owned),
            Ok(None) => {
                // The list went meanwhile; the next round drops it.
                return Ok(unchanged(&before));
            }
            Err(err) => return Err(failed(err)),
        };
        if !all && marker.is_some() && marker == before.marker {
            return Ok(unchanged(&before));
        }

        let Some((_, listed)) = self
            .dav
            .dav("REPORT", &url, "1", ETAGS)
            .await
            .map_err(failed)?
        else {
            return Err(Error::Rejected(
                "the CalDAV server can't list to-dos".into(),
            ));
        };
        let own = path(&url).trim_end_matches('/').to_owned();
        let remote: HashMap<String, String> = listed
            .iter()
            .filter(|r| path(&r.href).trim_end_matches('/') != own)
            .filter_map(|r| Some((path(&r.href).to_owned(), r.text("getetag")?.to_owned())))
            .collect();
        let wanted: Vec<String> = remote
            .iter()
            .filter(|(href, etag)| before.items.get(*href).map(|(e, _)| e) != Some(*etag))
            .map(|(href, _)| href.clone())
            .collect();

        let zone = TimeZone::system();
        let mut after = State {
            marker,
            items: before
                .items
                .iter()
                .filter(|(href, _)| remote.contains_key(*href))
                .map(|(href, item)| (href.clone(), item.clone()))
                .collect(),
        };
        let mut read = Vec::new();
        for (href, etag, text) in self.fetch(&url, &wanted).await? {
            let Some(todo) = todo::parse(&text, &zone) else {
                continue;
            };
            let etag = if etag.is_empty() {
                remote.get(&href).cloned().unwrap_or_default()
            } else {
                etag
            };
            after
                .items
                .insert(href.clone(), (etag.clone(), todo.uid.clone()));
            read.push((href, etag, todo));
        }
        let by_uid: HashMap<&str, &str> = after
            .items
            .iter()
            .map(|(href, (_, uid))| (uid.as_str(), href.as_str()))
            .collect();
        let mut tasks: Vec<RemoteTask> = read
            .iter()
            .map(|(href, etag, todo)| RemoteTask {
                remote_id: href.clone(),
                parent: todo
                    .parent
                    .as_deref()
                    .and_then(|uid| by_uid.get(uid))
                    .map(|href| (*href).to_owned()),
                deleted: false,
                title: todo.title.clone(),
                notes: todo.notes.clone(),
                due: todo.due.clone(),
                done_at: todo.done_at,
                position: String::new(),
                etag: etag.clone(),
                extras: Some(TaskExtras {
                    remind_at: todo.remind_at,
                    repeat: todo.repeat.clone(),
                    starred: todo.starred,
                }),
            })
            .collect();
        tasks.extend(
            before
                .items
                .keys()
                .filter(|href| !remote.contains_key(*href))
                .map(|href| RemoteTask {
                    remote_id: href.clone(),
                    deleted: true,
                    ..RemoteTask::default()
                }),
        );
        Ok(Pull {
            tasks,
            all,
            state: serde_json::to_string(&after).ok(),
            steps_of: Vec::new(),
        })
    }

    /// The server's etag and text of the to-do at `href`; `None` when it
    /// is gone.
    async fn current(&self, list: &str, href: &str) -> Result<Option<(String, String)>> {
        let url = self.url(list).await?;
        let found = self.fetch(&url, &[href.to_owned()]).await?;
        Ok(found
            .into_iter()
            .find(|(h, _, _)| h == href)
            .map(|(_, etag, text)| (etag, text)))
    }

    /// Adds `task` to `list`, as a step of the to-do at `parent` if given.
    pub async fn insert(
        &self,
        list: &str,
        task: &Task,
        parent: Option<&str>,
    ) -> Result<RemoteTask> {
        let parent_uid = match parent {
            Some(parent) => match self.current(list, parent).await? {
                Some((_, text)) => todo::parse(&text, &TimeZone::system()).map(|t| t.uid),
                None => return Err(Error::Rejected("the step's task is gone".into())),
            },
            None => None,
        };
        let name = random_name()?;
        let href = format!("{}/{name}.ics", list.trim_end_matches('/'));
        let text = todo::write(
            &todo_of(task, name, parent_uid),
            None,
            now(),
            &TimeZone::system(),
        );
        let url = self.url(&href).await?;
        let reply = self
            .dav
            .request(
                "PUT",
                &url,
                &[("If-None-Match", "*")],
                Some((ICAL, text.as_bytes())),
            )
            .await?;
        check(reply.status, "adding a to-do")?;
        Ok(RemoteTask {
            remote_id: href,
            parent: parent.map(str::to_owned),
            ..RemoteTask::default()
        })
    }

    /// Changes the to-do at `id`; `None` when the server no longer has it.
    pub async fn update(&self, list: &str, id: &str, task: &Task) -> Result<Option<RemoteTask>> {
        let url = self.url(id).await?;
        // Katna's change wins: once more if the to-do changed meanwhile.
        for _ in 0..2 {
            let Some((etag, old)) = self.current(list, id).await? else {
                return Ok(None);
            };
            let zone = TimeZone::system();
            let before = todo::parse(&old, &zone).unwrap_or_default();
            let text = todo::write(
                &todo_of(task, before.uid, before.parent),
                Some(&old),
                now(),
                &zone,
            );
            let quoted;
            let mut headers = Vec::new();
            if !etag.is_empty() {
                quoted = etag;
                headers.push(("If-Match", quoted.as_str()));
            }
            let reply = self
                .dav
                .request("PUT", &url, &headers, Some((ICAL, text.as_bytes())))
                .await?;
            match reply.status {
                404 | 410 => return Ok(None),
                412 => continue,
                status => check(status, "changing a to-do")?,
            }
            return Ok(Some(RemoteTask {
                remote_id: id.to_owned(),
                ..RemoteTask::default()
            }));
        }
        Err(Error::Closed(
            "the to-do keeps changing on the server".into(),
        ))
    }

    pub async fn delete(&self, _list: &str, id: &str) -> Result<()> {
        let url = self.url(id).await?;
        let reply = self.dav.request("DELETE", &url, &[], None).await?;
        if matches!(reply.status, 404 | 410) {
            return Ok(());
        }
        check(reply.status, "deleting a to-do")
    }
}

/// A write's status as a result: a refusal is logged and left for the
/// next round, a busy server waits.
fn check(status: u16, doing: &str) -> Result<()> {
    match status {
        200..=299 => Ok(()),
        401 | 403 => Err(Error::Rejected(format!(
            "the CalDAV server refused {doing}"
        ))),
        429 | 500.. => Err(Error::Closed(format!(
            "CalDAV server, {doing}: status {status}"
        ))),
        _ => Err(Error::Rejected(format!(
            "CalDAV server, {doing}: status {status}"
        ))),
    }
}

#[cfg(test)]
mod tests;
