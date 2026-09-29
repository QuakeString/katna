# Katna Mail, English: the Notes page.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Side list and search

notes-view-notes = Notes
notes-view-archive = Archive
notes-view-trash = Trash
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
notes-deleted-forever = { $count ->
    [one] Note deleted forever
   *[other] { $count } notes deleted forever
}
