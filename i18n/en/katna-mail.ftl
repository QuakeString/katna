# Katna Mail, English (the source of every translation).
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Language picker (top bar and Settings > General)

# The top bar button's tooltip. $language: the language's own name.
language-tooltip = Language: { $language }
language-tooltip-system = Language: { $language }, following the system
language-search = Search language
language-system-default = System default
# Under "System default": the language it is using now.
language-system-now = Now { $language }
language-no-match = No language matches “{ $query }”
language-machine = Translated by machine. Help improve it
language-setting = Language
language-setting-detail = The language of menus, buttons and messages, and the format of dates and numbers. System default follows the desktop.

## Dates and sizes

ago-just-now = just now
ago-minutes = { $count ->
    [one] { $count } minute ago
   *[other] { $count } minutes ago
}
ago-hours = { $count ->
    [one] { $count } hour ago
   *[other] { $count } hours ago
}
ago-days = { $count ->
    [one] { $count } day ago
   *[other] { $count } days ago
}
size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } bytes
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Hide folders
folders-show = Show folders
compose = Compose
search = Search
search-mail = Search mail
search-settings = Search settings
search-clear = Clear search
search-options-show = Show search options
settings = Settings
account-add = Add an account

## App rail (and the bottom bar on a phone)

rail-mail = Mail
rail-calendar = Calendar
# Also the heading of the Contacts page.
rail-contacts = Contacts
rail-tasks = Tasks
rail-notes = Notes
# RSS and Atom news feeds.
rail-feeds = Feeds

## Pages of apps still to come

# The page's heading. $app: the app's name from the rail, such as "Calendar".
app-page-title = Katna { $app }
app-coming-soon = Coming soon
app-calendar-promise = Your CalDAV calendars, meeting invitations from your mail and reminders, next to your inbox.
app-tasks-promise = To-do lists that sync with CalDAV, and tasks made from mail.
app-notes-promise = Quick notes, and notes on a mail or conversation for later.
app-feeds-promise = Read RSS and Atom feeds beside your mail.

## Contacts page

app-contacts-loading = Gathering people from your mail…
app-contacts-empty = People you write with show up here.
# Under the heading, when every person is listed.
app-contacts-count = { $count ->
    [one] { $count } person from your mail, most written with first
   *[other] { $count } people from your mail, most written with first
}
# Under the heading, when the list stops at its limit.
app-contacts-top = { $count ->
    [one] The top { $count } person from your mail, most written with first
   *[other] The top { $count } people from your mail, most written with first
}
# How many messages there are with one person.
app-contacts-messages = { $count ->
    [one] { $count } message
   *[other] { $count } messages
}
# When you last wrote with a person. $date: a time or date, such as "14:05" or "Sep 3".
app-contacts-last = last { $date }

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
# The badge of an inbox tab in a phone's drawer: how many new messages it has.
nav-tab-new = { $count ->
    [one] { $count } new
   *[other] { $count } new
}

## Special folders (the user's own folders keep their names)

folder-inbox = Inbox
# Starred (flagged) messages.
folder-starred = Starred
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

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Primary
tab-promotions = Promotions
tab-social = Social
tab-updates = Updates
tab-forums = Forums
tab-focused = Focused
tab-other = Other
tab-inbox = Inbox
tab-newsletters = Newsletters
tab-notifications = Notifications
# Badge under a tab's name: how many unread messages it has.
tab-new = { $count } new
# Settings > Inbox tabs, "Automatic: Gmail (sorted by Katna)": for accounts
# that are not Gmail, Outlook or Zoho, Katna sorts mail into tabs itself.
tab-provider-other = sorted by Katna

## Mail list: toolbar

# Tooltip of the checkbox that ticks every line.
list-select = Select
list-refresh = Refresh
# Tooltip of the "more actions" button (three dots).
list-more = More
list-mark-read = Mark as read
list-mark-unread = Mark as unread
list-move-to = Move to
list-archive = Archive
list-spam = Report spam
list-delete = Delete
# Tooltips of the page arrows: newer mail is on the previous page.
list-newer = Newer
list-older = Older
# Which lines show: $first and $last are line numbers, $total all lines.
list-range = { $first }–{ $last } of { $total }
# As list-range, when the total is an estimate from the search engine.
list-range-about = { $first }–{ $last } of about { $total }
# $query: what the user typed in the search box.
list-results = Results for “{ $query }”
# $query: the spelling-corrected search that ran instead.
list-results-corrected = Showing results for “{ $query }”
# A link that runs the search as typed. $query: what the user typed.
list-search-instead = Search instead for “{ $query }”
# The "+3" button after a line's attachment chips: $count more files.
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = All
list-pick-none = None
list-pick-read = Read
list-pick-unread = Unread
list-pick-starred = Starred
list-pick-unstarred = Unstarred

## Mail list: banner when every line is ticked
# $kind: "conversation" or "message", as the list groups mail.
# $count: how many; $folder: the folder's name.

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] All { $count } conversations are selected.
       *[other] All { $count } conversations are selected.
    }
   *[message] { $count ->
        [one] All { $count } messages are selected.
       *[other] All { $count } messages are selected.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] All { $count } conversations in { $folder } are selected.
       *[other] All { $count } conversations in { $folder } are selected.
    }
   *[message] { $count ->
        [one] All { $count } messages in { $folder } are selected.
       *[other] All { $count } messages in { $folder } are selected.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] All { $count } conversations on screen are selected.
       *[other] All { $count } conversations on screen are selected.
    }
   *[message] { $count ->
        [one] All { $count } messages on screen are selected.
       *[other] All { $count } messages on screen are selected.
    }
}
# A link that ticks every line, not only those on screen.
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Select all { $count } conversations
       *[other] Select all { $count } conversations
    }
   *[message] { $count ->
        [one] Select all { $count } messages
       *[other] Select all { $count } messages
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Select all { $count } conversations in { $folder }
       *[other] Select all { $count } conversations in { $folder }
    }
   *[message] { $count ->
        [one] Select all { $count } messages in { $folder }
       *[other] Select all { $count } messages in { $folder }
    }
}
list-clear-selection = Clear selection

## Mail list: empty states

list-empty-search = No messages matched your search.
# $tab: the inbox tab's name, such as Promotions.
list-empty-tab = No mail in { $tab }.
list-empty-tab-unknown = No mail in this tab.
# $folder: the folder's name.
list-empty-folder = No messages in { $folder }.
list-empty-folder-unknown = No messages in this folder.
# While the first sync of a new account downloads its mail.
list-first-sync = Getting your mail…
list-first-sync-detail = It shows up here as it arrives.

## Mail list: lines

# A line whose message was deleted elsewhere while the list showed it.
row-removed = This message was removed.
# Tooltips of a line's star.
row-starred = Starred
row-not-starred = Not starred
# Tooltips of a line's importance marker.
row-important = Important. Click to mark as not important.
row-mark-important = Mark as important
# Tooltip of the pin icon on a pinned line.
row-pinned = Pinned to the top
# Tooltips of the pin button shown on a line under the pointer.
row-pin = Pin to top
row-unpin = Unpin

## Mail list: More menu and right-click menu

menu-reply = Reply
menu-reply-all = Reply all
menu-forward = Forward
menu-archive = Archive
menu-delete = Delete
menu-spam = Report spam
menu-mark-read = Mark as read
menu-mark-unread = Mark as unread
# Marks every line in the list as read.
menu-mark-all-read = Mark all as read
menu-star = Add star
menu-unstar = Remove star
menu-important = Mark as important
menu-not-important = Mark as not important
menu-pin = Pin to top
menu-unpin = Unpin
# Prints every message of the open conversation.
menu-print-all = Print all
menu-new-window = Open in new window
# Opens a submenu of folders.
menu-move-to = Move to
# Heading over the list of folders to move the ticked mail to.
menu-move-to-heading = Move to:
# Searches for mail from the sender. $name: the sender's name or address.
menu-find-from = Find emails from { $name }

## Snackbar after an action on mail in the list
# $kind: "conversation" or "message", as the list groups mail.
# $count: how many were acted on.

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Conversation archived.
       *[other] { $count } conversations archived.
    }
   *[message] { $count ->
        [one] Message archived.
       *[other] { $count } messages archived.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Conversation moved to Trash.
       *[other] { $count } conversations moved to Trash.
    }
   *[message] { $count ->
        [one] Message moved to Trash.
       *[other] { $count } messages moved to Trash.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Conversation moved.
       *[other] { $count } conversations moved.
    }
   *[message] { $count ->
        [one] Message moved.
       *[other] { $count } messages moved.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Conversation starred.
       *[other] { $count } conversations starred.
    }
   *[message] { $count ->
        [one] Message starred.
       *[other] { $count } messages starred.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Conversation unstarred.
       *[other] { $count } conversations unstarred.
    }
   *[message] { $count ->
        [one] Message unstarred.
       *[other] { $count } messages unstarred.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Conversation marked as important.
       *[other] { $count } conversations marked as important.
    }
   *[message] { $count ->
        [one] Message marked as important.
       *[other] { $count } messages marked as important.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Conversation marked as not important.
       *[other] { $count } conversations marked as not important.
    }
   *[message] { $count ->
        [one] Message marked as not important.
       *[other] { $count } messages marked as not important.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Conversation pinned to the top.
       *[other] { $count } conversations pinned to the top.
    }
   *[message] { $count ->
        [one] Message pinned to the top.
       *[other] { $count } messages pinned to the top.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Conversation unpinned.
       *[other] { $count } conversations unpinned.
    }
   *[message] { $count ->
        [one] Message unpinned.
       *[other] { $count } messages unpinned.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversation reported as spam.
       *[other] { $count } conversations reported as spam.
    }
   *[message] { $count ->
        [one] Message reported as spam.
       *[other] { $count } messages reported as spam.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Conversation deleted forever.
       *[other] { $count } conversations deleted forever.
    }
   *[message] { $count ->
        [one] Message deleted forever.
       *[other] { $count } messages deleted forever.
    }
}
# After Undo on the snackbar took an action back.
toast-undone = Action undone.
# The snackbar's button that takes the action back.
toast-undo = Undo
toast-no-spam-folder = This account has no spam folder.

## Reading pane: toolbar

reader-close = Close
reader-back = Back
reader-mark-unread = Mark as unread
reader-move-to = Move to
# The ⋮ button that opens more actions.
reader-more = More
# Prints every message of the open conversation.
reader-print-all = Print all
# Opens the conversation in a window of its own.
reader-new-window = In new window
# Where the open conversation is in the list: "3 of 120".
reader-position = { $position } of { $total }
# Go to the newer conversation in the list.
reader-newer = Newer
# Go to the older conversation in the list.
reader-older = Older

## Reading pane: the conversation

# Shown in place of a conversation that was deleted or moved meanwhile.
reader-removed = This conversation was removed.
# The title of a conversation whose messages have no subject.
reader-no-subject = (no subject)
reader-collapse-all = Collapse all
reader-expand-all = Expand all
reader-unknown-sender = (unknown sender)
# A message's date with how long ago it was. $date: the full date; $ago: "2 hours ago".
reader-date-ago = { $date } ({ $ago })
# Stands for the user's own address among the recipients: "to me, Bob".
reader-me = me
# Under the sender's name. $names: the recipients, separated by commas ("me, Bob").
reader-to = to { $names }
# Tooltip of the star button on a starred message.
reader-starred = Starred
# Tooltip of the star button on a message that is not starred.
reader-not-starred = Not starred
reader-too-long = The message is too long to show in full.
reader-encrypted-images = Images from the web are never loaded in encrypted mail.
reader-window-failed = Could not open a new window.

## Reading pane: message details (opened from "to me")

reader-details-from = from:
reader-details-to = to:
reader-details-cc = cc:
reader-details-date = date:
reader-details-subject = subject:

## Reading pane: downloading a message

reader-downloading = Downloading this message from the server…
reader-download-failed = Could not download this message.
reader-try-again = Try again

## Reply row

reply-reply = Reply
reply-reply-all = Reply all
reply-forward = Forward

## Encrypted and signed mail

security-decrypting = Decrypting…
security-checking = Checking the signature…
security-partly-encrypted = Only part of this message is encrypted. The rest was added outside the protection and could come from anyone.
security-partly-signed = Only part of this message is signed. The rest was added outside the protection and could come from anyone.
security-encrypted = Encrypted message
security-encrypted-smime = Encrypted message (S/MIME)
security-no-key = Can't decrypt this message: it was encrypted for a key you don't have.
security-cancelled = Decrypting was cancelled.
security-damaged = Can't decrypt this message: the encrypted data is damaged or was changed.
# $tool: the program to install, such as "GnuPG (gpg)".
security-decrypt-unavailable = Can't decrypt this message: install { $tool } to read encrypted mail.
# $reason: the error GnuPG gave, in English.
security-decrypt-failed = Can't decrypt this message: { $reason }
# Takes the place of $signer below when the signer's name is not known.
security-unknown-signer = an unknown signer
# $signer: the signer's name and address, or "an unknown signer".
security-signed-verified = Signed by { $signer } · verified
security-signed-not-sender = Signed by { $signer }, who is not the sender
security-signed-untrusted = Signed by { $signer }, with a key you marked as not trusted
security-signed-unverified = Signed by { $signer } · the key is not verified
security-bad-signature = Bad signature: this message was changed after it was signed, or the signature is forged.
security-signature-expired = Signed by { $signer } · the signature has expired
security-key-expired = Signed by { $signer } · the key has expired since
security-key-revoked = Signed by { $signer } with a key that has been revoked
security-missing-key = Signed with a key you don't have, so it can't be checked
# $key: the end of the key's fingerprint, such as "658C A70C A20C 0FE0".
security-missing-key-id = Signed with a key you don't have ({ $key }), so it can't be checked
# $tool: the program to install, such as "GnuPG (gpg)".
security-signature-unavailable = Signed; install { $tool } to check the signature
security-signature-error = The signature could not be checked.

## Remote images and pictures

remote-hidden = Images in this message are hidden.
remote-show = Show images
remote-always-show = Always show from this sender
# The button of the file chooser that picks an account's picture.
remote-picture-use = Use
remote-picture-too-big = Pick a picture of 8 MB or less.
remote-picture-type = Pick a PNG, JPEG, GIF, WebP or SVG picture.
# $error: the system's error, in English.
remote-picture-read-failed = Cannot read the picture: { $error }
remote-picture-keep-failed = Cannot keep the picture: { $error }
remote-picture-remove-failed = Cannot remove the picture: { $error }

## Attachments

# Above a message's attachment cards.
attachment-count = { $count ->
    [one] One attachment
   *[other] { $count } attachments
}
# Tooltip of the download button on an attachment card.
attachment-save = Save
attachment-save-all = Save all
attachment-save-all-tooltip = Save every attachment to a folder
# The button of the folder chooser that saves every attachment.
attachment-save-here = Save here
attachment-not-downloaded = This message is not downloaded.
attachment-open-message = Open this message to read its attachments.
attachment-not-found = This attachment could not be found in the message.
# $name: the file's name.
attachment-read-failed = Could not read { $name }
# Names an attachment that has no name, by its place in the message.
attachment-numbered = attachment { $number }
# $place: the folder, such as "Downloads".
attachment-saved-all = { $count ->
    [one] Saved { $count } file to { $place }
   *[other] Saved { $count } files to { $place }
}
# $failed: the first file that could not be saved, with the error.
attachment-saved-some = { $total ->
    [one] Saved { $saved } of { $total } file to { $place }. Could not save { $failed }
   *[other] Saved { $saved } of { $total } files to { $place }. Could not save { $failed }
}
# $path: where the file was saved.
attachment-saved-to = Saved to { $path }
# $error: the system's error, in English.
attachment-save-failed = Could not save { $name }: { $error }
attachment-open-failed = Could not open { $name }: { $error }
attachment-risky = This file could run a program, so Katna does not open it. Save it instead.
attachment-encrypted-open = This file came encrypted. Save it to open it elsewhere.

## Printing

# $error: why, in English.
print-failed = Could not print: { $error }
print-no-font = no font was found
print-opened-as-pdf = Opened as a PDF to print from there.
# In the printed page, in place of a message's text.
print-not-downloaded = (Not downloaded yet.)
print-encrypted = (Encrypted. Open it in Katna Mail to print its text.)
# In the printed page, above a message. $addresses: its recipients.
print-to = To: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-copy = Copy
text-select-all = Select all

## Settings > General > Time

settings-time = Time
settings-clock-language = As the language writes it
settings-clock-12 = 12-hour, like 2:05 PM
settings-clock-24 = 24-hour, like 14:05

## Settings > General > Default mail app

mail-app-is-default = Katna Mail is your default mail app.
mail-app-is-other = Email links open in another app.
mail-app-make-default = Make default
mail-app-make-default-failed = Couldn't change the default mail app.
