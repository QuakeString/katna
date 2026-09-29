-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v11: formatted notes (docs/ARCHITECTURE.md §13.11). A note
-- with bold, italic, underline or headings keeps its text as HTML too, the
-- way its Notes folder has it; `body` stays its plain text for the board,
-- search and checklists. Empty: no formatting.

ALTER TABLE note ADD COLUMN html TEXT NOT NULL DEFAULT '';
