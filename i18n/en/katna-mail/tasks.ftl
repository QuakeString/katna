# Katna Mail, English: the Tasks page.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Left side

# The button at the top left that adds a task, like Google Tasks' "Create".
tasks-create = Create
tasks-all = All tasks
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
tasks-delete = Delete
# $list: the name of another list.
tasks-move-to = Move to { $list }
# A chip on a task made from a mail.
tasks-from-mail = Mail

## Due days

tasks-due-today = Today
tasks-due-tomorrow = Tomorrow
tasks-due-yesterday = Yesterday
# $day: "Today", "Fri" or "2 Oct"; $time: "4:00 PM".
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Task completed
tasks-toast-deleted = Task deleted
tasks-toast-list-deleted = List deleted
# $list: the list's name.
tasks-toast-moved = Moved to { $list }
