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
# The folder pane's right-click menu. Checks the account the folder is in,
# or every account from All Accounts.
nav-menu-check-mail = Check for new mail
# The same, on an account's inbox under All Accounts.
nav-menu-check-inbox = Check this inbox
# On an account's inbox under All Accounts: its mail stops showing in the
# unified Inbox; the row stays, dimmed, and still opens that inbox.
nav-unified-leave-out = Leave out of unified Inbox
# On such a left-out inbox, in the same menu and on its eye: undoes it.
nav-unified-bring-back = Bring back into unified Inbox
# On an account's heading or its row under All Accounts: shown when the
# account's sign-in stopped working; opens the provider's sign-in page.
nav-menu-sign-in-again = Sign in again
# Opens a new message sent from this account.
nav-menu-new-mail = New mail from this account
# Opens Settings > Accounts.
nav-menu-account-settings = Account settings
# The account on top of that menu, under its address: how its sync stands.
# $ago: how long ago it last checked, such as “2 minutes ago”.
nav-account-checked = In sync · checked { $ago }
nav-account-in-sync = In sync
nav-account-connecting = Connecting…
nav-account-offline = Offline, trying again
# $provider: Google or Microsoft.
nav-account-signed-out = { $provider } sign-in expired
nav-account-password-refused = Password refused
# Under the account's storage bar. $used and $total: sizes such as “1.2 GB”
# and “5 GB”.
nav-account-storage = { $used } of { $total } used
# Makes a folder inside the one right-clicked.
nav-menu-new-subfolder = New folder inside
# Gmail: makes a label nested under the one right-clicked.
nav-menu-new-sublabel = New label inside
# On a folder (or Gmail label) the user made: gives it another name.
nav-menu-rename = Rename
# On a folder (or Gmail label) the user made: deletes it, after asking.
nav-menu-delete = Delete
# Deletes everything in Trash for good, after asking.
nav-menu-empty-trash = Empty Trash
# An account whose name is unknown. $number: its number.
nav-account-unnamed = Account { $number }
# The heading of the unified inbox, over the accounts in the folder pane:
# each special folder of every account in one list.
nav-all-accounts = All Accounts
# Tooltips of the arrow beside an account's name and beside "All Accounts".
nav-expand = Show folders
nav-collapse = Hide folders
# Under the bar at the foot of the folder pane: how full the account's
# mail storage is. $percent: a whole number such as “34”; $total: the
# account's storage, such as “15 GB”.
storage-used = { $percent }% of { $total } used
# Tooltip of that bar. $address: the account; $used and $total: sizes
# such as “5.1 GB” and “15 GB”.
storage-used-detail = { $address }: { $used } of { $total } used

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
# Mail that has not gone out yet: waiting for a connection or a sign-in,
# or refused by the mail server. Shows only while there is some.
folder-outbox = Outbox
# How mail sent with open and click tracking did.
folder-activity = Activity

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
# The same dialog, renaming a label or folder the user made.
label-rename-title = Rename label
label-folder-rename-title = Rename folder
label-rename = Rename
label-renaming = Renaming…
# $name: the new name.
label-renamed = Label renamed to “{ $name }”.
label-folder-renamed = Folder renamed to “{ $name }”.

## Deleting a folder or label (asked first)

# $name: the folder's name.
folder-delete-title = Delete “{ $name }”?
# $count: the mail in it and in the folders inside it; $kind:
# "conversation" or "message", as the list groups mail.
folder-delete-body = { $count ->
    [0] It holds no mail. The folder is removed from the server, so webmail and your phone lose it too.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Its { $count } conversation goes to Trash, so you can still get it back.
           *[other] Its { $count } conversations go to Trash, so you can still get them back.
        }
       *[message] { $count ->
            [one] Its { $count } message goes to Trash, so you can still get it back.
           *[other] Its { $count } messages go to Trash, so you can still get them back.
        }
    } The folder is removed from the server, so webmail and your phone lose it too.
}
# The same, on an account without a Trash folder.
folder-delete-forever-body = { $count ->
    [0] It holds no mail. The folder is removed from the server, so webmail and your phone lose it too.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Its { $count } conversation is deleted for good; this account has no Trash.
           *[other] Its { $count } conversations are deleted for good; this account has no Trash.
        }
       *[message] { $count ->
            [one] Its { $count } message is deleted for good; this account has no Trash.
           *[other] Its { $count } messages are deleted for good; this account has no Trash.
        }
    } The folder is removed from the server, so webmail and your phone lose it too.
}
# Gmail: a label goes, its mail stays.
folder-delete-label-body = The label is removed. Its mail stays in All mail and in its other labels.
folder-delete-confirm = Delete folder
folder-delete-label-confirm = Delete label
# The snackbar once it is gone. $name: its name.
folder-deleted = Folder “{ $name }” deleted
label-deleted = Label “{ $name }” deleted
