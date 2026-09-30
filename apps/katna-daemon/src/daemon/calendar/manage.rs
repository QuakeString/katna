// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendars themselves, from the side panel of the Calendar page: a new
//! calendar, a new name or colour, and deleting one or taking one shared
//! with the person off their list ([`katna_sync::calendar::manage`]).
//! Each goes to the calendar's service at once, the way its calendars
//! came, and reaches the store only once the service took it; a calendar
//! on this computer changes in the store alone.

use katna_core::AccountId;
use katna_store::calendar::{Calendar, CalendarSource};
use katna_sync::{
    calendar::{CalendarError, manage},
    methods::{self, Data, Method},
};

use super::{Service, unix_now};
use crate::daemon::{CommandError, Daemon, Notice};

/// What a service's refusal says to people.
fn failed(err: CalendarError) -> CommandError {
    match err {
        CalendarError::NotOffered => {
            CommandError::Failed("the account's server offers no calendars".into())
        }
        err => CommandError::Failed(err.to_string()),
    }
}

/// `#rrggbb`, or empty.
fn color(text: &str) -> Result<String, CommandError> {
    let text = text.trim();
    let valid = text.is_empty()
        || (text.len() == 7
            && text.starts_with('#')
            && text[1..].bytes().all(|b| b.is_ascii_hexdigit()));
    if !valid {
        return Err(CommandError::InvalidArgs(format!("not a colour: {text}")));
    }
    Ok(text.to_ascii_lowercase())
}

fn name(text: &str) -> Result<String, CommandError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(CommandError::InvalidArgs("a calendar needs a name".into()));
    }
    Ok(text.chars().take(200).collect())
}

/// The way calendars of `source` came.
fn method_of(source: CalendarSource) -> Option<Method> {
    match source {
        CalendarSource::CalDav => Some(Method::Dav),
        CalendarSource::Google | CalendarSource::Microsoft | CalendarSource::Zoho => {
            Some(Method::Api)
        }
        CalendarSource::Local => None,
    }
}

impl Service {
    fn source(&self) -> CalendarSource {
        match self {
            Self::Google(_) => CalendarSource::Google,
            Self::Microsoft(_) => CalendarSource::Microsoft,
            Self::CalDav(_) => CalendarSource::CalDav,
            Self::Zoho(_) => CalendarSource::Zoho,
        }
    }
}

impl Daemon {
    fn known_calendar(&self, id: i64) -> Result<Calendar, CommandError> {
        self.store()
            .calendar(id)?
            .ok_or_else(|| CommandError::InvalidArgs(format!("no calendar {id}")))
    }

    /// The way `account`'s calendars come: that of those it has, else the
    /// one that worked last.
    fn calendar_method(&self, account: AccountId) -> Result<Method, CommandError> {
        let store = self.store();
        let listed = store
            .calendars()?
            .into_iter()
            .filter(|c| c.account == Some(account))
            .find_map(|c| method_of(c.source));
        listed
            .or_else(|| methods::remembered(&store, account, Data::Calendar, unix_now()))
            .ok_or_else(|| {
                CommandError::Failed("the account has no calendars Katna can reach".into())
            })
    }

    /// Has calendars changed here and on their services show.
    fn calendars_changed(&self) {
        let _ = self.notices.try_send(Notice::CalendarChanged);
        self.wake_calendars();
    }

    /// Adds a calendar named `name` in `color` to `account` (0: this
    /// computer). Returns its ID.
    pub async fn add_calendar(
        &self,
        account: i64,
        name: &str,
        color: &str,
    ) -> Result<i64, CommandError> {
        let (name, color) = (self::name(name)?, self::color(color)?);
        if account == 0 {
            let id = self.store().add_local_calendar(&name, &color)?;
            tracing::info!(calendar = id, "calendar added on this computer");
            let _ = self.notices.try_send(Notice::CalendarChanged);
            return Ok(id);
        }
        let account = self.account(AccountId(account))?;
        let method = self.calendar_method(account.id)?;
        let (source, made) = {
            let mut services = self.calendars.services.lock().await;
            let service = self
                .service(&mut services, &account, method)
                .await
                .map_err(CommandError::Failed)?
                .ok_or_else(|| {
                    CommandError::Failed("the account has no calendars Katna can reach".into())
                })?;
            let made = manage::add(&service.remote(), &name, &color)
                .await
                .map_err(failed)?;
            (service.source(), made)
        };
        let id = {
            let mut store = self.store();
            let position = store.next_calendar_position(account.id)?;
            store.upsert_calendar(Some(account.id), source, &made, position)?
        };
        tracing::info!(calendar = id, account = %account.id, "calendar added");
        self.calendars_changed();
        Ok(id)
    }

    /// The service of `calendar`, for `doing` something to it there;
    /// `None` for a calendar on this computer.
    async fn on_service<T>(
        &self,
        calendar: &Calendar,
        doing: impl AsyncFnOnce(&Service) -> Result<T, CalendarError>,
    ) -> Result<Option<T>, CommandError> {
        let (Some(account), Some(method)) = (calendar.account, method_of(calendar.source)) else {
            return Ok(None);
        };
        let account = self.account(account)?;
        let mut services = self.calendars.services.lock().await;
        let service = self
            .service(&mut services, &account, method)
            .await
            .map_err(CommandError::Failed)?
            .ok_or_else(|| {
                CommandError::Failed("the account has no calendars Katna can reach".into())
            })?;
        doing(service).await.map(Some).map_err(failed)
    }

    /// Renames calendar `id`.
    pub async fn rename_calendar(&self, id: i64, name: &str) -> Result<(), CommandError> {
        let name = self::name(name)?;
        let calendar = self.known_calendar(id)?;
        self.on_service(&calendar, async |service| {
            manage::rename(&service.remote(), &calendar, &name).await
        })
        .await?;
        self.store().rename_calendar(id, &name)?;
        tracing::info!(calendar = id, "calendar renamed");
        self.calendars_changed();
        Ok(())
    }

    /// Gives calendar `id` colour `color`; returns the colour it got (an
    /// Outlook calendar takes the nearest of Outlook's).
    pub async fn set_calendar_color(&self, id: i64, color: &str) -> Result<String, CommandError> {
        let color = self::color(color)?;
        let calendar = self.known_calendar(id)?;
        let kept = self
            .on_service(&calendar, async |service| {
                manage::recolor(&service.remote(), &calendar, &color).await
            })
            .await?
            .unwrap_or(color);
        self.store().set_calendar_color(id, &kept)?;
        tracing::info!(calendar = id, "calendar recoloured");
        self.calendars_changed();
        Ok(kept)
    }

    /// Deletes calendar `id` with its events (`delete`), or takes it off
    /// the person's list, on its service and here.
    pub async fn delete_calendar(&self, id: i64, delete: bool) -> Result<(), CommandError> {
        let calendar = self.known_calendar(id)?;
        if calendar.source == CalendarSource::Local {
            let locals = self
                .store()
                .calendars()?
                .into_iter()
                .filter(|c| c.source == CalendarSource::Local)
                .count();
            if locals <= 1 {
                return Err(CommandError::Failed(
                    "Katna keeps one calendar on this computer".into(),
                ));
            }
        }
        self.on_service(&calendar, async |service| {
            manage::remove(&service.remote(), &calendar, delete).await
        })
        .await?;
        self.store().delete_calendar(id)?;
        tracing::info!(calendar = id, delete, "calendar removed");
        self.calendars_changed();
        Ok(())
    }
}
