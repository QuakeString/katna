# Katna Mail, English: folder pane.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Navigation (the folders pane)

# The heading over a Gmail account's own labels.
nav-labels = Labels
# The heading over an account's own folders.
nav-folders = Folders
# Tooltip of the "+" beside "Labels".
nav-label-new = Create new label
# Tooltip of the "+" beside "Folders".
nav-folder-new = Create new folder
# An account whose name is unknown. $number: its number.
nav-account-unnamed = Account { $number }
# The heading of the unified inbox, over the accounts in the folder pane:
# each special folder of every account in one list.
nav-all-accounts = All Accounts
# Tooltips of the arrow beside an account's name and beside "All Accounts".
nav-expand = Show folders
nav-collapse = Hide folders
# The badge of an inbox tab in a phone's drawer: how many new messages it has.
nav-tab-new = { $count ->
    [one] { $count } new
   *[other] { $count } new
}

## Special folders (the user's own folders keep their names)

folder-inbox = Inbox
# Starred (flagged) messages.
folder-starred = Starred
# Katna's folder of snoozed mail, which comes back to the inbox later.
folder-snoozed = Snoozed
# Unread messages of every account (in the unified inbox).
folder-unread = Unread
# Messages marked important (in the unified inbox).
folder-important = Important
folder-drafts = Drafts
folder-sent = Sent
folder-archive = Archive
# Junk mail.
folder-spam = Spam
folder-trash = Trash
# Every message of the account (Gmail's "All Mail").
folder-all-mail = All mail
# Messages scheduled to be sent later.
folder-scheduled = Scheduled

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = New label
label-folder-new-title = New folder
label-prompt = Please enter a new label name:
label-folder-prompt = Please enter a new folder name:
# Placeholder of the name field.
label-name-hint = Label name
label-folder-name-hint = Folder name
# A check box; the folders to choose from are listed under it.
label-nest = Nest label under:
label-folder-nest = Nest folder under:
label-cancel = Cancel
label-create = Create
label-creating = Creating…
# $name: the name the user gave it.
label-created = Label “{ $name }” created.
label-folder-created = Folder “{ $name }” created.
