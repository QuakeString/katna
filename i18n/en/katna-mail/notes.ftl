# Katna Mail, English: the Notes page.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Side list and search

notes-view-notes = Notes
notes-view-reminders = Reminders
notes-view-archive = Archive
notes-view-trash = Trash
# Opens the dialog that renames and deletes labels.
notes-edit-labels = Edit labels
notes-search = Search notes
notes-loading = Opening your notes…

## Board

# The bar at the top of the board that opens a new note, and the empty
# text of a new note.
notes-take-a-note = Take a note…
notes-new-list = New list
# The big button at the top of the left bar on Notes.
notes-new-note = New note
# Headings over the pinned notes and the rest.
notes-pinned = Pinned
notes-others = Others
notes-empty = Notes you add appear here
notes-archive-empty = Your archived notes appear here
notes-trash-empty = No notes in Trash
notes-none-found = No matching notes
notes-label-empty = No notes with this label yet
notes-reminders-empty = Notes with upcoming reminders appear here
notes-trash-note = Notes in Trash are deleted after 7 days.
notes-empty-trash = Empty Trash
# Under a checklist on a card: how many of its items are ticked.
notes-ticked = { $count ->
    [one] + { $count } ticked item
   *[other] + { $count } ticked items
}
# Tooltip of the tick at a card's top left.
notes-select = Select note
# Over the board while cards are ticked: how many, and the × that unticks them.
notes-selected = { $count ->
    [one] { $count } selected
   *[other] { $count } selected
}
notes-select-clear = Clear selection

## A note's buttons

notes-pin = Pin note
notes-unpin = Unpin note
notes-archive = Archive
notes-unarchive = Unarchive
notes-delete = Delete note
notes-restore = Restore
notes-delete-forever = Delete forever
notes-color = Background color
notes-checkboxes = Show or hide checkboxes
notes-labels = Labels
notes-close = Close
notes-more = More
notes-make-copy = Make a copy
# The bell: when the note reminds.
notes-remind = Remind me
notes-add-picture = Add picture
# The clock button: earlier versions of the note.
notes-history = Version history
# The sparkle: AI help with the note.
notes-ai = Help me write
notes-send-as-mail = Send as mail
notes-save-markdown = Save as Markdown
notes-save-pdf = Save as PDF

## The open note

notes-title = Title
# At the foot of an open note. $date: when it was last changed, such as
# "10:42" or "Sep 28".
notes-edited = Edited { $date }
# Where the note is kept: a mail account's name, or this.
notes-on-this-computer = On this computer
# Tooltip of where the note is kept, which opens the choice of accounts.
notes-where = Where this note is kept
# A note with neither a title nor text, where its name shows.
notes-untitled = Untitled note

## Pictures

notes-picture-choose = Add pictures
notes-picture-remove = Remove picture
# $size: the largest picture taken, such as "10 MB".
notes-picture-too-big = Pictures up to { $size } can go in a note
notes-picture-kind = That file is not a picture Katna can show
# $name: the file; $error: why it could not be read.
notes-picture-unreadable = Could not read { $name }: { $error }

## Reminders

# Over the reminder times, as Snooze's menu has them.
notes-remind-me = Remind me
notes-remind-off = Remove reminder
notes-remind-in-the-past = Pick a time that hasn't passed yet
# A reminder's chip. $time: such as "18:00"; $day: a weekday, such as "Mon".
notes-remind-today = Today, { $time }
notes-remind-tomorrow = Tomorrow, { $time }
notes-remind-weekday = { $day }, { $time }
# Snackbars. $when: the day and time, such as "Sun, Sep 27, 2026, 8:00 AM".
notes-reminder-set = Reminder set for { $when }
notes-reminder-off = Reminder removed

## Links between notes

# Over the notes suggested after typing [[, and in the ⋮ menu.
notes-link-note = Link a note
# The last suggestion: a new note named as typed. $title: what was typed.
notes-link-new = New note "{ $title }"
# At the foot of a note: the notes that link to it.
notes-linked-from = Linked from
notes-link-gone = That note is no longer here
# A note at the bottom when a note made from the [[ suggestions vanished
# before it could be linked.
notes-new-note-gone = The new note is gone.

## Version history

notes-versions = Versions
# The newest version, and where it was written.
notes-version-now = Now
notes-version-here = You, on this computer
# A version written yesterday. $time: such as "21:15".
notes-version-yesterday = Yesterday, { $time }
# How many lines a version changed.
notes-version-changes = { $count ->
    [one] { $count } change
   *[other] { $count } changes
}
# A version written on another device. $device: such as "iPhone".
notes-version-from = From { $device }
notes-version-elsewhere = From another device
notes-version-created = Created
notes-version-restore = Restore this version
notes-version-restored = Version restored
notes-history-none = No earlier versions yet

## AI help

notes-ai-tidy = Tidy the text
notes-ai-checklist = Turn it into a checklist
notes-ai-summarise = Summarise
notes-ai-empty = Write something first
notes-ai-tidied = Text tidied. Ctrl+Z puts it back.
notes-ai-listed = Made into a checklist. Ctrl+Z puts it back.
notes-ai-summarised = Summary added on top

## Labels

# Over the box on an open note that finds or makes a label.
notes-label-note = Label note
notes-label-name = Enter label name
# Makes a label that does not exist yet. $name: what was typed.
notes-label-create = Create "{ $name }"
# Tooltip of the × on a label chip of an open note.
notes-label-remove = Remove label
notes-label-delete = Delete label
# In Edit labels when no note has a label.
notes-labels-none = No labels yet. Add one from a note's label button.
notes-labels-done = Done
# Snackbars, with Undo. $name: the label's (new) name.
notes-label-renamed = Label renamed to "{ $name }"
notes-label-deleted = Label "{ $name }" deleted

## A note about a mail

# The chip on a note made from a mail, which opens that mail.
notes-mail = Mail
notes-open-mail = Open the mail
# Tooltip of a note shown under a mail's subject.
notes-open-note = Open the note

## Meeting notes

# On an event's card: a new note about the event.
notes-meeting-take = Take meeting notes
# A meeting note's title. $title: the event's; $date: its day, such as "Sep 29".
notes-meeting-title = { $title } · { $date }
# The lines a meeting note starts with. $names: who comes, with commas.
notes-meeting-attendees = Attendees: { $names }
notes-meeting-notes = Notes
notes-meeting-actions = Action items
# The chip on a meeting note, which opens the Calendar on its day.
notes-event = Event
notes-open-event = Open the event

## Formatting

# In a note's toolbar: shows the formatting row (Keep's "Formatting options").
notes-format = Formatting
notes-format-heading-1 = Heading 1
notes-format-heading-2 = Heading 2
notes-format-normal = Normal text
notes-format-bold = Bold
notes-format-italic = Italic
notes-format-underline = Underline
notes-format-quote = Quote
notes-format-code = Code
notes-format-divider = Divider
notes-format-clear = Clear formatting

## Tasks

# In a note's toolbar: a task made from the checklist line the cursor is on.
notes-make-task = Make it a task

## Colors (tooltips)

notes-color-none = No color
notes-color-coral = Coral
notes-color-peach = Peach
notes-color-sand = Sand
notes-color-mint = Mint
notes-color-sage = Sage
notes-color-fog = Fog
notes-color-storm = Storm
notes-color-dusk = Dusk
notes-color-blossom = Blossom
notes-color-clay = Clay
notes-color-chalk = Chalk

## Messages at the foot of the window

notes-archived = Note archived
notes-unarchived = Note unarchived
notes-trashed = Note moved to Trash
notes-restored = Note restored
notes-saved = Note saved
notes-pinned-count = { $count ->
    [one] Note pinned
   *[other] { $count } notes pinned
}
notes-unpinned-count = { $count ->
    [one] Note unpinned
   *[other] { $count } notes unpinned
}
notes-colored-count = { $count ->
    [one] Color changed
   *[other] Color changed on { $count } notes
}
notes-archived-count = { $count ->
    [one] Note archived
   *[other] { $count } notes archived
}
notes-unarchived-count = { $count ->
    [one] Note unarchived
   *[other] { $count } notes unarchived
}
notes-trashed-count = { $count ->
    [one] Note moved to Trash
   *[other] { $count } notes moved to Trash
}
notes-restored-count = { $count ->
    [one] Note restored
   *[other] { $count } notes restored
}
notes-copied-count = { $count ->
    [one] Copy made
   *[other] { $count } copies made
}
notes-empty-discarded = Empty note discarded
notes-mail-gone = That mail is no longer here
notes-deleted-forever = { $count ->
    [one] Note deleted forever
   *[other] { $count } notes deleted forever
}
