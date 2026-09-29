# Katna Mail, English: the Notes page.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Side list and search

notes-view-notes = Notes
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
# Headings over the pinned notes and the rest.
notes-pinned = Pinned
notes-others = Others
notes-empty = Notes you add appear here
notes-archive-empty = Your archived notes appear here
notes-trash-empty = No notes in Trash
notes-none-found = No matching notes
notes-label-empty = No notes with this label yet
notes-trash-note = Notes in Trash are deleted after 7 days.
notes-empty-trash = Empty Trash
# Under a checklist on a card: how many of its items are ticked.
notes-ticked = { $count ->
    [one] + { $count } ticked item
   *[other] + { $count } ticked items
}

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

## The open note

notes-title = Title
# At the foot of an open note. $date: when it was last changed, such as
# "10:42" or "Sep 28".
notes-edited = Edited { $date }
# Where the note is kept: a mail account's name, or this.
notes-on-this-computer = On this computer
# Tooltip of where the note is kept, which opens the choice of accounts.
notes-where = Where this note is kept

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
notes-empty-discarded = Empty note discarded
notes-mail-gone = That mail is no longer here
notes-deleted-forever = { $count ->
    [one] Note deleted forever
   *[other] { $count } notes deleted forever
}
