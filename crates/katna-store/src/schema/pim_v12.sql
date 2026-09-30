-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v12: calendars read through Zoho Calendar's own API
-- (docs/ARCHITECTURE.md §18) take source 'zoho'. SQLite can't change a
-- CHECK constraint in place, and rebuilding `calendar` would delete its
-- events through their foreign key, so the constraint's text is widened
-- in the schema itself: allowing one more value changes nothing stored
-- (sqlite.org/lang_altertable.html, "Making Other Kinds Of Table Schema
-- Changes"). The migration then bumps the schema cookie (db.rs).

PRAGMA writable_schema = ON;
UPDATE sqlite_schema
   SET sql = replace(sql, '''caldav'', ''local''', '''caldav'', ''zoho'', ''local''')
 WHERE type = 'table' AND name = 'calendar';
PRAGMA writable_schema = OFF;
