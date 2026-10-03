# SPDX-License-Identifier: GPL-3.0-or-later

# Katna Mail, English: quick capture, the small card that opens over
# whatever is on screen (Meta+Alt+T, Meta+Alt+N, the tray's New task and
# New note) to write down a task or a note in one line.
# Guide: i18n/README.md. Keep ids stable; change the text freely.

# The card's window title, which the taskbar and window switcher show.
capture-title = Quick capture

## The tabs at the top

capture-task = Task
capture-note = Note
# In the empty line on the Task tab. (The Note tab shows notes-take-a-note.)
capture-task-placeholder = Add a task

## Chips: what was understood from the words typed. A click changes it.

# The due day: $weekday is short ("Fri"), $day the day and month ("9 Oct").
# With a time it becomes tasks-due-at.
capture-day = { $weekday } { $day }
# A chip to pick a day when none was typed.
capture-add-date = Add date
# The list the task goes to, and the account it belongs to.
capture-list = { $list } · { $account }
# A list kept on this computer only.
capture-list-here = { $list }
# Where the note is kept: an account's name, or notes-on-this-computer.
capture-note-place = Notes · { $place }

## What a chip offers when clicked

# The Monday after today.
capture-next-week = Next week
capture-no-date = No date
capture-no-label = No label

## The line at the bottom

capture-hint-task = Enter saves · Tab switches to Note
capture-hint-note = Enter saves · Tab switches to Task
# The key that closes the card.
capture-esc = Esc
# In place of the hint when the daemon did not take it; the card stays open.
capture-not-saved = Not saved. Try again in a moment.
