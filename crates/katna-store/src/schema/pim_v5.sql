-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v5: calendars and their events (docs/ARCHITECTURE.md §18),
-- synced by the daemon through each service's own API (Google Calendar,
-- Microsoft Graph, CalDAV) or kept on this computer.

-- One calendar of an account, or a local one.
CREATE TABLE calendar (
    id          INTEGER PRIMARY KEY,
    account_id  INTEGER,                 -- mail.db account.id; NULL: local
    source      TEXT    NOT NULL
                CHECK (source IN ('google', 'microsoft', 'caldav', 'local')),
    remote_id   TEXT    NOT NULL DEFAULT '',  -- the service's ID, or the CalDAV href
    name        TEXT    NOT NULL,
    color       TEXT    NOT NULL DEFAULT '',  -- #rrggbb
    access      TEXT    NOT NULL DEFAULT 'owner'
                CHECK (access IN ('owner', 'writer', 'reader', 'freebusy')),
    is_primary  INTEGER NOT NULL DEFAULT 0 CHECK (is_primary IN (0, 1)),
    -- Unticked in the calendar list: its events are not shown.
    hidden      INTEGER NOT NULL DEFAULT 0 CHECK (hidden IN (0, 1)),
    time_zone   TEXT    NOT NULL DEFAULT '',  -- IANA name, or empty
    sync_token  TEXT,                     -- for the next incremental sync
    position    INTEGER NOT NULL DEFAULT 0,
    UNIQUE (account_id, remote_id)
);

-- One event: a single one, the first of a repeating series (with `rrule`)
-- or a changed occurrence of a series (with `recurrence_id`).
CREATE TABLE event (
    id            INTEGER PRIMARY KEY,
    calendar_id   INTEGER NOT NULL REFERENCES calendar (id) ON DELETE CASCADE,
    remote_id     TEXT    NOT NULL DEFAULT '',
    uid           TEXT    NOT NULL DEFAULT '',  -- iCalendar UID
    etag          TEXT,
    -- A changed occurrence: the start it had in the series (Unix seconds).
    recurrence_id INTEGER,
    status        TEXT    NOT NULL DEFAULT 'confirmed'
                  CHECK (status IN ('confirmed', 'tentative', 'cancelled')),
    title         TEXT    NOT NULL DEFAULT '',
    location      TEXT    NOT NULL DEFAULT '',
    description   TEXT    NOT NULL DEFAULT '',
    -- Unix seconds, the end after the event. A whole-day event starts and
    -- ends at UTC midnights of its days.
    start         INTEGER NOT NULL,
    end           INTEGER NOT NULL,
    all_day       INTEGER NOT NULL DEFAULT 0 CHECK (all_day IN (0, 1)),
    time_zone     TEXT    NOT NULL DEFAULT '',  -- IANA name of the start
    rrule         TEXT    NOT NULL DEFAULT '',  -- RRULE value, without "RRULE:"
    exdates       TEXT    NOT NULL DEFAULT '',  -- skipped starts, Unix seconds, spaces
    rdates        TEXT    NOT NULL DEFAULT '',  -- extra starts, Unix seconds, spaces
    -- Where the series ends (Unix seconds); NULL when it never does. The
    -- end of the event when it doesn't repeat.
    range_end     INTEGER,
    busy          INTEGER NOT NULL DEFAULT 1 CHECK (busy IN (0, 1)),
    kind          TEXT    NOT NULL DEFAULT 'default'
                  CHECK (kind IN ('default', 'focus', 'out_of_office', 'working_location', 'birthday')),
    color         TEXT    NOT NULL DEFAULT '',  -- #rrggbb, over the calendar's
    organizer     TEXT    NOT NULL DEFAULT '',  -- address
    organizer_name TEXT   NOT NULL DEFAULT '',
    attendees_json TEXT   NOT NULL DEFAULT '[]',
    -- The user's answer: '', 'accepted', 'tentative', 'declined', 'needs_action'.
    self_status   TEXT    NOT NULL DEFAULT '',
    join_url      TEXT    NOT NULL DEFAULT '',  -- https:// video call
    reminders     TEXT    NOT NULL DEFAULT '',  -- minutes before, spaces
    web_link      TEXT    NOT NULL DEFAULT '',
    updated_at    INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX event_by_remote ON event (calendar_id, remote_id);
CREATE INDEX event_by_uid ON event (calendar_id, uid, recurrence_id);
CREATE INDEX event_by_time ON event (start, range_end);
