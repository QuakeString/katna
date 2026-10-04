# Katna service, English: Katna's results in the desktop's search (KRunner
# on KDE Plasma, the Activities search on GNOME).
# Guide: i18n/README.md. Keep ids stable; change the text freely.

## Results

# The heading over messages in KRunner's results.
search-category-mail = Mail
# The heading over people (addresses from the mail) in KRunner's results.
search-category-people = People
# The heading over tasks in KRunner's results.
search-category-tasks = Tasks
# The heading over a note to add ("note: …") in KRunner's results.
search-category-notes = Notes
# The heading over events in KRunner's results.
search-category-events = Events
# The line under a message's subject: who sent it.
search-mail-from = From { $sender }
# A message without a subject.
search-no-subject = (no subject)
# The line under a task's title: the list it is in.
search-task-in = In { $list }
# The line under an event's title: when its next time is, and where or in
# which calendar. $when is one of the four below.
search-event-at = { $when } · { $place }
search-event-in = { $when } · { $calendar }
search-event-now = Now
search-event-today = Today
search-event-tomorrow = Tomorrow
search-event-in-days = In { $count } days

## Quick capture: "task: …" or "note: …" typed in KRunner

# The one result for "task: Call the plumber fri 6pm": Enter adds the
# task. $title is the task's title, without the day and time typed.
search-add-task = Add task “{ $title }”
# Under it: the list the task goes to, and when it is due if a day was
# typed ($when is one of search-event-today, -tomorrow and -in-days).
search-add-task-to = To { $list }
search-add-task-when = { $when } · { $list }
# The one result for "note: Ideas for the garden": Enter adds the note.
search-add-note = Add note “{ $title }”
# Under it: where the note is kept, an account's name or "this computer".
search-add-note-to = To Notes in { $place }
search-add-note-here = To Notes on this computer
# The result for "task:" or "note:" with nothing after it yet: opens the
# quick capture card.
search-new-task = New task
search-new-note = New note

## Buttons on a result in KRunner

# On a message: opens it and starts a reply to everyone on it.
search-reply-all = Reply all
# On a person: copies their email address.
search-copy-address = Copy address
# On a person: searches Katna Mail for mail from or to them.
search-find-mail = Find mail
# On a task or note to add: opens the quick capture card with the text
# typed, to change it before it is saved.
search-edit-capture = Change before adding
