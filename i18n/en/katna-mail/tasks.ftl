# Katna Mail, English: the Tasks page.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Left side

# The button at the top left that adds a task, like Google Tasks' "Create".
tasks-create = Create
tasks-all = All tasks
# Tasks due today and overdue, from every list.
tasks-today = Today
tasks-starred = Starred
tasks-new-list = Create new list
# Heading over the lists kept on this computer, not in an account.
tasks-on-this-computer = On this computer
# The name of the list kept on this computer.
tasks-my-tasks = My Tasks
tasks-list-name-placeholder = List name

## Lists and tasks

tasks-loading = Reading your tasks…
tasks-no-lists = Your task lists show up here.
tasks-add = Add a task
tasks-title-placeholder = Title
tasks-add-step = Add a subtask
tasks-empty = No tasks yet. Add one above.
tasks-starred-empty = Star a task to see it here.
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
tasks-toast-deleted = Task deleted
tasks-toast-added = { $count ->
    [one] Added to Tasks
   *[other] { $count } tasks added
}
# The mail a task was made from is no longer here (deleted, or not synced).
tasks-mail-gone = That mail isn't here any more.
tasks-toast-list-deleted = List deleted
# $list: the list's name.
tasks-toast-moved = Moved to { $list }
# A task dragged to another day or time on the Calendar.
tasks-toast-rescheduled = Task rescheduled
