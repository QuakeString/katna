-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v9: events changed on this computer that their service
-- does not have yet (docs/ARCHITECTURE.md §18). The daemon writes a change
-- at once, marks its rows, and clears the mark once the service took it;
-- a sync leaves marked rows alone until then.

-- 0: as the service has it; 1: changed here, to send; 2: deleted here, to
-- delete on the service (kept, cancelled, until then).
ALTER TABLE event ADD COLUMN pending INTEGER NOT NULL DEFAULT 0 CHECK (pending IN (0, 1, 2));
CREATE INDEX event_pending ON event (calendar_id, remote_id) WHERE pending != 0;
