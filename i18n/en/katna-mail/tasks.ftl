# Katna Mail, English: the Tasks page.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Left side

# The big button at the top of the left bar on Tasks.
tasks-create = New task
tasks-all = All tasks
# Tasks due today and overdue, from every list.
tasks-today = Today
tasks-starred = Starred
tasks-new-list = Create new list
# Heading over the labels on tasks (the same labels as on notes); each
# shows every task with it.
tasks-labels-heading = Labels
# Heading over the lists kept on this computer, not in an account.
tasks-on-this-computer = On this computer
# The name of the list kept on this computer.
tasks-my-tasks = My Tasks
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Sign in again to show tasks
tasks-account-signed-in = Signed in to { $address } again. Getting your tasks…
tasks-account-sign-in-refused = { $provider } did not let Katna in. Try again, and allow access to your tasks.
tasks-account-refused = The server did not accept the password. Yahoo, iCloud, Zoho and others need an app password.
tasks-account-change-password = Change password
tasks-account-change-password-tooltip = Open Settings > Accounts
tasks-account-not-enabled = Task access for Katna is not switched on yet.
tasks-account-failed = The task lists could not be read.
# $reason is the server's own words, in English.
tasks-account-error = The task lists could not be read: { $reason }
tasks-account-none = No task lists found
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = No task lists found: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } shows tasks only to Katna signed in with { $provider }.
tasks-account-sign-in-with = Sign in with { $provider }
tasks-account-looking = Looking for task lists…
tasks-account-try-again = Try again
tasks-account-try-again-tooltip = Check this account's tasks again now
tasks-account-fixing = Working on it…
tasks-list-name-placeholder = List name

## Lists and tasks

tasks-loading = Reading your tasks…
tasks-no-lists = Your task lists show up here.
# The top bar's search box on the Tasks page, and when nothing matches.
tasks-search = Search tasks
tasks-search-none = No tasks match your search.
tasks-add = Add a task
tasks-title-placeholder = Title
tasks-add-step = Add a subtask
tasks-empty = No tasks yet. Add one above.
tasks-starred-empty = Star a task to see it here.
tasks-label-empty = No open tasks with this label.
tasks-today-empty = Nothing due today.
# Under the Today heading: $weekday is the day's name, $day the day and month.
tasks-today-date = { $weekday }, { $day }
# The section of Today with tasks whose day has passed.
tasks-overdue = Overdue
# The folded section at the bottom of a list.
tasks-completed = { $count ->
    [one] Completed ({ $count })
   *[other] Completed ({ $count })
}
tasks-list-options = List options
tasks-rename-list = Rename list
tasks-delete-list = Delete list
tasks-mark-done = Mark completed
tasks-mark-open = Mark uncompleted
tasks-star = Star
tasks-unstar = Remove star
tasks-edit-title = Edit title
# Opens the dialog with a task's details, date and repeat.
tasks-details = Details
tasks-delete = Delete
# $list: the name of another list.
tasks-move-to = Move to { $list }
# A chip on a task made from a mail.
tasks-from-mail = Mail
tasks-open-mail = Open the mail
# A chip on a task made from a note's checklist line.
tasks-from-note = Note
tasks-open-note = Open the note
# The note a task was made from is no longer here (deleted).
tasks-note-gone = That note isn't here any more.
# The title of a task made from a mail with no subject.
tasks-no-subject = (no subject)

## The details dialog

tasks-notes-placeholder = Add details
tasks-date = Date
tasks-no-date = No date
tasks-time-placeholder = Add time
tasks-repeat = Repeat
tasks-repeat-never = Doesn't repeat
tasks-repeat-daily = Daily
tasks-repeat-weekly = Weekly
tasks-repeat-monthly = Monthly
tasks-repeat-yearly = Yearly
# A repeat set elsewhere that has no button here, like every second Tuesday.
tasks-repeat-other = Custom
# When a task reminds, in its details. The "morning" one is for a task
# without a time: on its day at { $time } ("9:00 AM").
tasks-remind = Remind me
tasks-remind-off = Don't remind
tasks-remind-on-time = At the time
tasks-remind-morning = On the day, { $time }
tasks-remind-hour-before = An hour before
tasks-remind-day-before = The day before
# The chip that opens the label picker in a task's details, and the
# picker's heading.
tasks-label-add = Add label
tasks-label-task = Label task
# The paper clip in a task's details, and the file chooser's button.
tasks-files-attach = Attach files
tasks-files-pick = Attach
tasks-file-open = Open
tasks-file-remove = Remove file
# Beside a file To Do or the CalDAV server would not take (too large, or
# the service keeps no files): it stays with the task on this computer.
tasks-file-here = Only on this computer
tasks-cancel = Cancel
tasks-save = Save
# $text: what was typed; $example: a time written the usual way, like "4:00 PM".
tasks-not-a-time = “{ $text }” is not a time, for example { $example }.

## Due days

tasks-due-today = Today
tasks-due-tomorrow = Tomorrow
tasks-due-yesterday = Yesterday
# $day: "Today", "Fri" or "2 Oct"; $time: "4:00 PM".
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Task completed
# A repeating task was ticked off and comes back on its next day ("5 Oct").
tasks-toast-next = Done. Next one on { $date }
tasks-toast-deleted = Task deleted
tasks-files-added = { $count ->
    [one] File attached
   *[other] { $count } files attached
}
# $name: the file's name.
tasks-file-removed = Removed “{ $name }”
# $names: the files' names; $limit: the largest file a task takes ("25 MB").
tasks-files-left-out = Not attached: { $names }. A task takes files up to { $limit }, not folders.
# A task's file could not be read (removed meanwhile).
tasks-file-missing = That file isn't here any more.
tasks-toast-added = { $count ->
    [one] Added to Tasks
   *[other] { $count } tasks added
}
# The mail a task was made from is no longer here (deleted, or not synced).
tasks-mail-gone = That mail isn't here any more.
tasks-toast-list-deleted = List deleted
# $list: the list's name.
tasks-toast-moved = Moved to { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Task moved
# A task dragged to another day or time on the Calendar.
tasks-toast-rescheduled = Task rescheduled
