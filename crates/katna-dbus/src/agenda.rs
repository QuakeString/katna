// SPDX-License-Identifier: GPL-3.0-or-later

//! `in.invenia.katna.Agenda1`: the daemon's events and tasks for the
//! desktop's clock (Katna Digital Clock on Plasma, Katna's GNOME Shell
//! extension; `docs/ARCHITECTURE.md` §15.4).
//!
//! Served by `katna-daemon` at `/in/invenia/katna/Daemon/Agenda` under
//! its bus name, so a call starts the daemon (D-Bus activation). Its
//! clients are QML and GNOME Shell's JavaScript, so every item is a plain
//! `a{sv}` map with the keys below; a client ignores keys it doesn't know,
//! and the daemon may add keys later.
//!
//! - `Events(x from, x to) -> aa{sv}`: events overlapping `from..to`
//!   (Unix seconds). Keys: [`event`].
//! - `Tasks() -> aa{sv}`: open tasks, and those ticked off in the last
//!   day, due ones first. Keys: [`task`].
//! - `AddTask(s title, s due) -> s id`: `due` is `YYYY-MM-DD` or empty.
//! - `SetTaskDone(s id, b done)`, `DeleteTask(s id)`.
//! - `Open(s id) -> b`: shows the event or task in Katna; false when
//!   Katna has nothing to show it in yet.
//!
//! For Katna's Tasks page (lists are row IDs; `0` means none):
//!
//! - `AddTaskTo(x list, s parent, s title) -> s id`: a task on top of
//!   `list` (`0`: the default list), as a step of task `parent` if not
//!   empty.
//! - `EditTask(s id, a{sv} fields)`: sets the keys of [`edit`] present.
//! - `MoveTask(s id, x list)`: to another list, on top, with its steps.
//! - `AddTaskList(x account, s title) -> x list`: `account` `0` is this
//!   computer. `RenameTaskList(x list, s title)`, `DeleteTaskList(x list)`.
//! - signal `Changed()`: read again.

use std::collections::HashMap;

use zbus::zvariant::OwnedValue;

/// One event or task on the wire.
pub type Item = HashMap<String, OwnedValue>;

/// Keys of an event.
pub mod event {
    /// `s`: the ID for `Open`.
    pub const ID: &str = "id";
    /// `s`
    pub const TITLE: &str = "title";
    /// `x`: Unix seconds.
    pub const START: &str = "start";
    /// `x`: Unix seconds, after the event.
    pub const END: &str = "end";
    /// `b`: a whole day (or days); `start` and `end` are local midnights.
    pub const ALL_DAY: &str = "all_day";
    /// `s`, may be empty.
    pub const LOCATION: &str = "location";
    /// `s`: `#rrggbb`, the event's colour or else its calendar's, or
    /// empty.
    pub const COLOR: &str = "color";
    /// `s`: the calendar's name.
    pub const CALENDAR: &str = "calendar";
    /// `s`: an `https://` video-call link, or empty.
    pub const JOIN_URL: &str = "join_url";
}

/// Keys of a task.
pub mod task {
    /// `s`: the ID for `SetTaskDone`, `DeleteTask` and `Open`.
    pub const ID: &str = "id";
    /// `s`
    pub const TITLE: &str = "title";
    /// `s`, may be empty.
    pub const NOTES: &str = "notes";
    /// `s`: `YYYY-MM-DD`, or empty.
    pub const DUE: &str = "due";
    /// `b`
    pub const DONE: &str = "done";
    /// `s`: the task list's name.
    pub const LIST: &str = "list";
    /// `s`: the Message-ID of the mail it was made from, or empty.
    pub const MAIL: &str = "mail";
}

/// Keys of `EditTask`'s fields; each is optional.
pub mod edit {
    /// `s`
    pub const TITLE: &str = "title";
    /// `s`
    pub const NOTES: &str = "notes";
    /// `s`: `YYYY-MM-DD`, or empty.
    pub const DUE: &str = "due";
    /// `i`: minutes after local midnight on the due day; `-1` for none.
    pub const DUE_TIME: &str = "due_time";
    /// `x`: Unix seconds; `0` for none.
    pub const REMIND_AT: &str = "remind_at";
    /// `s`: an RFC 5545 `RRULE` value, or empty.
    pub const REPEAT: &str = "repeat";
    /// `b`
    pub const STARRED: &str = "starred";
    /// `s`: the Message-ID of the mail it was made from, or empty.
    pub const MAIL: &str = "mail";
}

macro_rules! agenda_proxy {
    ($interface:tt, $bus_name:tt, $path:tt) => {
        /// Client side of `in.invenia.katna.Agenda1`.
        #[zbus::proxy(interface = $interface, default_service = $bus_name, default_path = $path)]
        pub trait Agenda {
            /// Events overlapping `from..to` (Unix seconds).
            fn events(&self, from: i64, to: i64) -> zbus::Result<Vec<Item>>;

            /// Open tasks, and those ticked off in the last day.
            fn tasks(&self) -> zbus::Result<Vec<Item>>;

            /// Adds a task; `due` is `YYYY-MM-DD` or empty. Returns its ID.
            fn add_task(&self, title: &str, due: &str) -> zbus::Result<String>;

            /// Ticks a task off, or opens it again.
            fn set_task_done(&self, id: &str, done: bool) -> zbus::Result<()>;

            /// Deletes a task.
            fn delete_task(&self, id: &str) -> zbus::Result<()>;

            /// Adds a task on top of `list` (`0`: the default list), as a
            /// step of `parent` if not empty. Returns its ID.
            fn add_task_to(&self, list: i64, parent: &str, title: &str) -> zbus::Result<String>;

            /// Sets the fields of task `id` given ([`edit`]).
            fn edit_task(&self, id: &str, fields: Item) -> zbus::Result<()>;

            /// Moves task `id`, with its steps, to `list`.
            fn move_task(&self, id: &str, list: i64) -> zbus::Result<()>;

            /// Adds a list to `account` (`0`: this computer). Returns it.
            fn add_task_list(&self, account: i64, title: &str) -> zbus::Result<i64>;

            fn rename_task_list(&self, list: i64, title: &str) -> zbus::Result<()>;

            fn delete_task_list(&self, list: i64) -> zbus::Result<()>;

            /// Shows an event (the Calendar on its day) or a task (the
            /// Tasks page) in Katna. False for an ID it doesn't know.
            fn open(&self, id: &str) -> zbus::Result<bool>;

            /// Starts a new event on `day` (`YYYY-MM-DD`) in Katna's
            /// Calendar.
            fn new_event(&self, day: &str) -> zbus::Result<()>;

            /// Events or tasks changed: read again.
            #[zbus(signal)]
            fn changed(&self) -> zbus::Result<()>;
        }
    };
}

katna_core::with_agenda_names!(agenda_proxy);
