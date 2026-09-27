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

## Settings page: its tabs

settings-tab-general = General
settings-tab-inbox = Inbox
settings-tab-accounts = Accounts
# The tab listing newsletters and mailing lists, to unsubscribe from.
settings-tab-subscriptions = Subscription
settings-tab-appearance = Appearance
# The tab of keyboard shortcuts.
settings-tab-shortcuts = Shortcuts
# The tab choosing which app opens each kind of attachment.
settings-tab-default-apps = Default apps
settings-tab-folders-rules = Folders & rules
# The tab with signatures and other settings for writing mail.
settings-tab-compose = Compose
# MCP: the Model Context Protocol, which AI assistants use.
settings-tab-mcp-server = MCP server
settings-tab-feedback = User feedback
settings-tab-experimental = Experimental

## Settings page: tabs still to come

settings-tab-subscriptions-coming = See the newsletters and mailing lists you get, and unsubscribe in one click.
settings-tab-folders-rules-coming = Create, rename, move and hide folders and labels, and choose which ones sync. Rules sort, label, forward or delete new mail by itself, by sender, subject or words.
settings-tab-mcp-server-coming = Let AI assistants on this computer search, read and draft your mail, with your say.

## Settings > General
# The Language row uses language-setting and language-setting-detail.

settings-general-conversations = Conversation view
settings-general-conversations-group = Group replies to the same mail
settings-general-conversations-group-detail = One line per conversation in the list
# The heading of the switches for how an opened conversation shows.
settings-general-reading = Reading
settings-general-newest-first = Newest message first
settings-general-newest-first-detail = A conversation starts with its latest reply
settings-general-full-headers = Show full headers
settings-general-full-headers-detail = From, to, cc, date and subject open on every message
settings-general-full-names = Full names of recipients
# An example of the recipients line with and without full names.
settings-general-full-names-detail = “to me, Ada Lovelace” rather than “to me, Ada”
settings-general-mark-read = Mark as read
settings-general-mark-read-now = As soon as it opens
settings-general-mark-read-1s = After it is open for 1 second
settings-general-mark-read-3s = After it is open for 3 seconds
settings-general-mark-read-never = Only when I mark it read
settings-general-reply-button = Reply button
settings-general-reply-all = Reply to everyone
settings-general-reply-all-detail = The reply button beside each message replies to all, not only the sender
settings-general-remote-images = Images from the web
settings-general-remote-images-detail = Loading a message's images tells its sender that you opened it, when, and roughly where. Off, each message asks first, and you can always show a sender's images.
settings-general-remote-images-always = Always show images
settings-general-remote-images-always-detail = In every message, not only from senders you trust
# The row of the undo-send delay.
settings-general-sending = Sending
settings-general-sending-detail = How long a sent message waits, so it can be taken back.
settings-general-offline = Offline mail
settings-general-offline-detail = Recent mail is downloaded whole, to read without a connection. Older mail downloads when you open it.
# A choice of how much mail is kept for offline reading.
settings-general-offline-days = { $count ->
    [one] { $count } day
   *[other] { $count } days
}
settings-general-offline-years = { $count ->
    [one] { $count } year
   *[other] { $count } years
}
settings-general-offline-all = All mail
settings-general-offline-note = Choosing fewer days keeps mail already downloaded. Nothing changes on the server.
settings-general-notifications = Notifications
settings-general-notifications-detail = For new mail in the Inbox, even while Katna Mail is closed.
settings-general-new-mail = Notify me about new mail
# The buttons a new-mail notification has.
settings-general-new-mail-detail = With Reply all, Mark as read and Archive
settings-general-new-mail-sound = Play a sound
settings-general-new-mail-sound-detail = The desktop's new-mail sound
# The row of settings about the desktop: login, tray and taskbar.
settings-general-desktop = Desktop
settings-general-open-at-login = Open Katna Mail at login
settings-general-open-at-login-detail = Mail syncs at login either way, while the service runs
settings-general-tray = Show Katna in the system tray
settings-general-tray-detail = With the unread count and a menu
settings-general-unread-badge = Unread count on the taskbar icon
settings-general-unread-badge-detail = How many Inbox messages are unread

## Settings > Inbox

settings-inbox-tabs = Inbox tabs
settings-inbox-tabs-detail = Sort the inbox into tabs, as your mail provider's website does.
settings-inbox-tabs-show = Show inbox tabs
settings-inbox-tabs-show-detail = Off shows one list for every account
settings-inbox-no-accounts = Add an account to choose its tabs.
# $tabs: the tabs of that style, such as "Focused and Other". $provider: the mail provider, such as "Gmail".
settings-inbox-tabs-automatic = Automatic: { $tabs } ({ $provider })
settings-inbox-tabs-off = No tabs
settings-inbox-tabs-gmail = Primary, Promotions, Social, Updates, Forums
settings-inbox-tabs-focused = Focused and Other
settings-inbox-tabs-zoho = Inbox, Newsletters and Notifications
# $tab: the first tab, such as "Primary".
settings-inbox-tabs-shown = Tabs shown. Mail of a tab you turn off stays in { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Reading pane
settings-appearance-reading-pane-detail = Where an opened conversation shows.
# A reading pane choice: the opened conversation shows beside the list.
settings-appearance-pane-right = Right of the list
# A reading pane choice: the opened conversation takes the list's place.
settings-appearance-pane-none = No split
# How close together the lines of the list are.
settings-appearance-density = Density
settings-appearance-density-default = Default
settings-appearance-density-compact = Compact
settings-appearance-scaling = Scaling
settings-appearance-scaling-detail = Makes everything in Katna Mail bigger or smaller, on top of the desktop's own scale: text, icons, spacing and dividers. Mail you send keeps its own font size. Very small sizes can make icons hard to click.
settings-appearance-theme = Theme
settings-appearance-theme-system = Same as the desktop
settings-appearance-theme-light = Light
settings-appearance-theme-dark = Dark
settings-appearance-desktop-colors = Desktop colors
settings-appearance-desktop-colors-use = Use the desktop's colors
settings-appearance-desktop-colors-use-detail = The color scheme and accent color of the desktop
settings-appearance-app-names = App names
settings-appearance-app-names-show = Show app names
settings-appearance-app-names-show-detail = Names under the app icons at the far left
settings-appearance-sender-pictures = Sender pictures
settings-appearance-sender-pictures-show = Show company logos
settings-appearance-sender-pictures-show-detail = Looked up by the sender's domain, never by message, and kept for a week
settings-appearance-important = Important markers
settings-appearance-important-show = Show Important markers
settings-appearance-important-show-detail = Beside each message in the list
settings-appearance-message-width = Message width
settings-appearance-message-width-limit = Limit the width of messages
settings-appearance-message-width-limit-detail = Long lines are easier to read in a wide window
settings-appearance-mail-colors = Mail colors
settings-appearance-mail-colors-detail = Most mail is designed for a white page. With a dark theme its colors are changed to dark ones that read well; off, it keeps its sender's colors on a light page.
settings-appearance-dark-mail = Dark colors for mail too
settings-appearance-dark-mail-detail = Only while the theme is dark
settings-appearance-attachment-previews = Attachment previews
settings-appearance-attachment-previews-show = Show previews of attachments
settings-appearance-attachment-previews-show-detail = A small picture of each file's content on its card

## Settings > Default apps

settings-default-apps-intro = Where attachments open when you click them. The viewer can always open a file in another app too. The desktop's default apps are set in its own settings.
settings-default-apps-pdf = PDF files
settings-default-apps-pdf-detail = Pages, with zoom.
settings-default-apps-pictures = Pictures
settings-default-apps-pictures-detail = Photos (turned upright), PNG, GIF, WebP, BMP, TIFF and SVG.
settings-default-apps-text = Text files
settings-default-apps-text-detail = Plain text, logs, code and other text.
settings-default-apps-sheets = Spreadsheets
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) and CSV.
settings-default-apps-documents = Documents
settings-default-apps-documents-detail = Word (docx) and OpenDocument text (odt).
settings-default-apps-katna = Katna Mail's viewer
settings-default-apps-system = The desktop's default app
settings-default-apps-ask = Ask which app each time
settings-default-apps-after-saving = After saving
settings-default-apps-show-folder = Show saved files in their folder
settings-default-apps-show-folder-detail = Opens the file manager with the saved attachments picked

## Settings > Compose

settings-compose-send-from = Send new messages from
settings-compose-send-from-detail = Replies and forwards always go out from the account you are in.
# The choice to send new mail from whichever account is open.
settings-compose-send-from-current = The account you are in
settings-compose-send-on-replies = Send on replies
settings-compose-send-on-replies-detail = What Send does on a reply or forward. The menu beside Send offers the other.
# The choice of what the Send button does on a reply: only send.
settings-compose-send-plain = Send
# The choice of what the Send button does on a reply: send, then archive the conversation.
settings-compose-send-archive = Send and archive
settings-compose-signatures = Signatures
settings-compose-signatures-detail = Added below your message, after a “--” line. Pick another one in the compose window.
# A signature, or another thing, without a name.
settings-compose-untitled = Untitled
# The placeholder of a signature's name.
settings-compose-signature-name = Name, such as Work
# The name given to the first signature made.
settings-compose-signature-first = My signature
# The name given to a new signature. $number: how many there are with it.
settings-compose-signature-numbered = Signature { $number }
settings-compose-signature-delete = Delete
settings-compose-signature-deleted = Signature deleted
# The button that makes a new signature.
settings-compose-signature-new = Create new
settings-compose-no-signatures = No signatures yet.
settings-compose-no-signature = No signature
settings-compose-for-new-mail = For new mail
settings-compose-for-replies = For replies and forwards
settings-compose-for-replies-detail = In a conversation where you signed a message, a reply starts with that signature instead.
# The row choosing between plain text and formatted mail.
settings-compose-format = Format
settings-compose-plain-text = Write in plain text
settings-compose-plain-text-detail = New mail starts without formatting; the compose window can switch
settings-compose-spelling = Spelling
settings-compose-spell-check = Check spelling while I write
settings-compose-spell-check-detail = Misspelled words are underlined, with suggestions on right-click
# The spelling dictionary of the desktop's language. $language: its code, such as "en_US".
settings-compose-spell-desktop = Desktop's language ({ $language })
settings-compose-templates = Templates
settings-compose-templates-detail = Save mail you write often, and start new mail or a reply from it.

## Settings > Shortcuts

settings-shortcuts-set = Shortcut set
settings-shortcuts-set-detail = Start from the keys of a mail app you know. Cmd is Ctrl here. Your own changes stay on top of the set, and Restore defaults goes back to the set's keys.
settings-shortcuts-single = Single-key shortcuts
settings-shortcuts-single-detail = Keys without Ctrl or Alt, as in webmail: e archives, j and k move, / searches. They work in the list and the open conversation, never while typing.
settings-shortcuts-single-use = Use single-key shortcuts
settings-shortcuts-single-use-detail = Ctrl shortcuts always work
settings-shortcuts-how = Click a key to change it, or + to add one, then press the new keys. Esc cancels.
settings-shortcuts-restore = Restore defaults
# Beside a shortcut that has no key.
settings-shortcuts-no-key = No key
# While recording a shortcut's keys.
settings-shortcuts-press = Press keys…
# While recording a shortcut of two keys in a row, after the first. $keys: the first key.
settings-shortcuts-then = { $keys } then…
# $keys: the keys pressed. $action: the shortcut they now do. $previous: the shortcut they did before.
settings-shortcuts-moved = { $keys } now does “{ $action }” instead of “{ $previous }”.
settings-shortcuts-single-off = Single-key shortcuts are off, so this key works once they are on.
settings-shortcuts-restored = Every shortcut has its set's keys again.

## Settings search: the line under a result

settings-general-language-summary = Language of the app, dates and numbers
settings-general-reading-summary = Newest message first, full headers, full names of recipients
settings-general-mark-read-summary = When an opened conversation is marked read: at once, after 1 or 3 seconds, or by hand
settings-general-reply-button-summary = The reply button beside each message replies to everyone
settings-general-remote-images-summary = Always show the images of every message
settings-general-sending-summary = Undo send: how long a sent message waits, so it can be taken back
settings-general-offline-summary = How many days of recent mail are downloaded whole, to read without a connection
settings-general-notifications-summary = New-mail notifications and their sound
settings-general-desktop-summary = Open Katna Mail at login, the system tray icon and the unread count on the taskbar icon
settings-accounts-accounts-summary = Add or remove an account, or change its picture
settings-appearance-density-summary = Default or compact lines in the list
settings-appearance-scaling-summary = Make everything bigger or smaller: text, icons, spacing and dividers
settings-appearance-theme-summary = Same as the desktop, light or dark
settings-appearance-sender-pictures-summary = Company logos, looked up by the sender's domain
settings-appearance-important-summary = The Important marker beside each message in the list
settings-appearance-mail-colors-summary = Dark colors for HTML mail in a dark theme, or its sender's colors
settings-appearance-attachment-previews-summary = A small picture of each attachment's content
settings-shortcuts-set-summary = Start from the keys of Gmail, Inbox by Gmail, Apple Mail, Outlook or Thunderbird
settings-shortcuts-single-summary = Keys without Ctrl or Alt, as in webmail
settings-default-apps-pdf-summary = Where PDF attachments open
settings-default-apps-pictures-summary = Where photos and pictures open
settings-default-apps-text-summary = Where plain text, logs and code open
settings-default-apps-sheets-summary = Where Excel, OpenDocument and CSV files open
settings-default-apps-documents-summary = Where Word and OpenDocument text open
settings-default-apps-after-saving-summary = Show saved attachments in their folder
settings-compose-send-from-summary = The account new mail goes out from: the one you are in, or always the same one
settings-compose-send-on-replies-summary = Send, or Send and archive the conversation, on replies and forwards
settings-compose-signatures-summary = Added below your message, after a “--” line
settings-compose-for-new-mail-summary = The signature new mail starts with
settings-compose-for-replies-summary = The signature replies and forwards start with
settings-compose-format-summary = Write new mail in plain text
settings-compose-spelling-summary = Check spelling while writing, and the dictionary's language
settings-compose-templates-summary = Coming soon: save mail you write often, and start new mail or a reply from it
settings-feedback-crash-reports-summary = Save crash reports on this computer when Katna Mail or its background service crashes
settings-feedback-saved-summary = View, copy or delete the crash reports saved on this computer
settings-feedback-help-improve-summary = Send crash reports to help fix what went wrong; off unless you turn it on
settings-experimental-blur-summary = The desktop shows through the top bar, blurred, and menus are frosted
# The line under a keyboard shortcut found by the settings search.
settings-search-shortcut = Keyboard shortcut
# The line under a tab of the Settings page found by the search.
settings-search-tab = Settings tab
# $query: what was typed in the search box.
settings-search-none = No settings match “{ $query }”.
settings-search-results = Settings that match “{ $query }”
## Quick settings (the panel that slides in from the right)

quick-title = Quick settings
# A button that opens the full Settings page.
quick-see-all = See all settings
# Section heading (shown in capitals).
quick-reading-pane = Reading pane
# A reading-pane choice: the open mail shows beside the list.
quick-pane-right = Right of the list
# A reading-pane choice: the open mail takes the list's place.
quick-pane-none = No split
# Section heading: how tightly the list's lines are spaced.
quick-density = Density
quick-density-default = Default
quick-density-compact = Compact
# Section heading: light or dark colors.
quick-theme = Theme
quick-theme-system = Same as the desktop
quick-theme-light = Light
quick-theme-dark = Dark
quick-desktop-colors = Desktop colors
quick-desktop-colors-detail = The color scheme and accent color of the desktop
# Names of the apps (Mail, Calendar, …) under their icons in the rail.
quick-app-names = App names
quick-app-names-detail = Names under the app icons at the far left
quick-inbox-tabs = Inbox tabs
quick-inbox-tabs-detail = The tabs of each account's mail provider
# Opens Settings > Inbox, where each account's tabs are chosen.
quick-choose-tabs = Choose tabs
quick-choose-tabs-detail = Per account, in Settings
# Section heading.
quick-sending = Sending
# How long a sent message waits, so it can be taken back.
quick-undo-send = Undo send
# The Undo send choice that turns it off.
quick-undo-send-off = Off
# An Undo send choice: $seconds is a number of seconds; "s" is the unit's symbol.
quick-undo-send-seconds = { $seconds } s
quick-signatures = Signatures
# Under "Signatures" when there are none.
quick-signatures-none = None yet
# Under "Signatures" when there is one. $name: the signature's name.
quick-signatures-one = { $name }, used by default
# Under "Signatures". $count: how many; $name: the one used by default.
quick-signatures-many = { $count ->
    [one] { $count } signatures; { $name } by default
   *[other] { $count } signatures; { $name } by default
}
# Under "Signatures" when none is chosen as the default. $count: how many there are.
quick-signatures-no-default = { $count ->
    [one] { $count }, none by default
   *[other] { $count }, none by default
}
# The name shown for a signature the user left unnamed.
quick-signature-untitled = Untitled
# Section heading: grouping replies into conversations.
quick-threading = Email threading
quick-conversation-view = Conversation view
quick-conversation-view-detail = Group replies to the same mail
# Section heading.
quick-help = Help
# Starts the guided tour of the app.
quick-tour = Take the tour
quick-whats-new = What’s new
quick-about = About Katna

## Settings: opening at login

# $error: the system's error, in English.
settings-open-at-login-failed = Could not change opening at login: { $error }

## Settings > Appearance > Scaling

# A sample letter drawn small and large at the two ends of the scale slider.
# Use a common letter of your script.
scale-letter = A
# The interface scale. $percent: a number such as 125.
scale-percent = { $percent }%
# A button that sets the scale back to normal. $percent is 100.
scale-reset = Back to { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Features still being tried out. They may change or go away.
look-heading = Look & Feel
# A row's name; the Settings search finds it by this name too.
look-window-frame = Window frame
look-window-frame-detail = Who draws the title bar, the window buttons, the corners and the shadow.
# The desktop draws the window's frame. KDE and Plasma are names.
look-frame-native-kde = Native: KDE's frame, in your Plasma theme
look-frame-native = Native: the desktop's frame
look-frame-katna = Katna: the top bar becomes the title bar
# Under the Katna frame choice. $desktop: the desktop's name, such as KDE or GNOME.
look-frame-katna-note-named = Katna draws rounded corners and its own shadow. The frame no longer follows the { $desktop } theme; window rules still apply.
# As look-frame-katna-note-named, when the desktop's name is not known.
look-frame-katna-note = Katna draws rounded corners and its own shadow. The frame no longer follows the desktop theme; window rules still apply.
# Shown in place of the frame choices on a desktop where every app draws its own frame.
look-frame-client-side = Your desktop leaves the frame to each app, so Katna already draws its own.
look-blurred-background = Blurred background
# Under "Blurred background".
look-blurred-background-detail = The desktop shows through the top bar and the folders, blurred, and menus and popovers are frosted glass.
look-blur = Blur what is behind the window
look-blur-detail = Mail stays on solid cards, so text keeps its contrast
# Why blur is not available. "Blur", "System Settings", "Window Management" and
# "Desktop Effects" are KDE's own names for its settings; use KDE's translation of them.
look-blur-off-kde = KDE's blur effect is off. Turn on Blur in System Settings, Window Management, Desktop Effects, then open Katna Mail again.
look-blur-none-gnome = GNOME does not blur what is behind windows.
look-blur-none-x11 = Your window manager does not blur what is behind windows.
look-blur-none-wayland = Your compositor does not blur what is behind windows.

## Settings > User feedback (crash reports)

# At the top of the page, when crash reports are sent.
feedback-intro-sending = New crash reports are sent to help fix what went wrong. Nothing else leaves this computer.
# At the top of the page, when crash reports are not sent.
feedback-intro-local = Katna sends nothing anywhere. Crash reports stay on this computer, for you to look at or attach to a bug report.
feedback-crash-reports = Crash reports
feedback-crash-reports-detail = Written when Katna Mail or its background service crashes.
feedback-save = Save crash reports on this computer
feedback-save-detail = Your home folder, user and computer names and email addresses are left out
feedback-saved = Saved crash reports
# $count: how many reports are kept (20).
feedback-saved-detail = { $count ->
    [one] The newest { $count } is kept.
   *[other] The newest { $count } are kept.
}
feedback-help-improve = Help improve Katna
feedback-help-improve-detail = Off unless you turn it on, and you can turn it off here at any time.
feedback-send = Send crash reports
# Sentry is the name of the crash tracker.
feedback-send-detail = The saved report, exactly as you can view it here, goes to Katna's crash tracker (Sentry, in the EU). No IP address, messages or email addresses
feedback-none-saved = No crash reports are saved.
# Deletes every saved crash report.
feedback-delete-all = Delete all
# The program that crashed: katna-daemon, which syncs mail in the background.
feedback-app-daemon = Background service
# Under a crash report's program: when it crashed, and that it was sent. $date: the date and time.
feedback-report-sent = { $date } · Sent
# Opens a crash report.
feedback-view = View
feedback-view-tooltip = Open the report
feedback-copy-tooltip = Copy it to paste into a bug report
feedback-copied = Crash report copied.
feedback-deleted-all = Crash reports deleted.
# $error: the system's error, in English.
feedback-read-failed = Could not read the crash report: { $error }
feedback-delete-failed = Could not delete the crash report: { $error }
feedback-delete-all-failed = Could not delete the crash reports: { $error }

## Menu bar (the KDE global menu)
# "_" marks the mnemonic: the letter after it is underlined and pressed with Alt.
# Keep exactly one "_" in each, before a letter of your translation.

desktop-menu-file = _File
desktop-menu-new-message = _New Message
desktop-menu-quit = _Quit
desktop-menu-edit = _Edit
desktop-menu-undo = _Undo
desktop-menu-select-all = Select _All
desktop-menu-select-none = Select _None
desktop-menu-find = _Find…
desktop-menu-view = _View
# Shows or hides the folders pane.
desktop-menu-folder-list = Show _Folder List
desktop-menu-refresh = _Refresh
desktop-menu-go = _Go
desktop-menu-inbox = _Inbox
desktop-menu-starred = _Starred
desktop-menu-sent = S_ent
desktop-menu-drafts = _Drafts
desktop-menu-all-mail = _All Mail
desktop-menu-next = _Next Conversation
desktop-menu-previous = _Previous Conversation
desktop-menu-message = _Message
desktop-menu-open = _Open
desktop-menu-reply = _Reply
desktop-menu-reply-all = Reply _All
desktop-menu-forward = _Forward
desktop-menu-archive = Arc_hive
desktop-menu-delete = _Delete
desktop-menu-spam = Report _Spam
desktop-menu-move-to = _Move To…
desktop-menu-mark-read = Mark as R_ead
desktop-menu-mark-unread = Mark as _Unread
desktop-menu-star = S_tar
desktop-menu-important = Mark as Im_portant
desktop-menu-not-important = Mark as _Not Important
desktop-menu-settings = _Settings
# Opens the Quick settings panel.
desktop-menu-quick-settings = _Quick Settings
# Opens the full Settings page.
desktop-menu-configure = _Configure Katna Mail…
desktop-menu-help = _Help
desktop-menu-shortcuts = _Keyboard Shortcuts
desktop-menu-whats-new = _What's New
desktop-menu-about = _About Katna
## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Moving around
shortcut-group-actions = Actions
# Shortcuts that open a folder (Inbox, Sent, …).
shortcut-group-go-to = Go to
# Shortcuts of the app itself (search, settings, quit).
shortcut-group-app = Application

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Next conversation
shortcut-previous = Previous conversation
shortcut-down = Move down the list
shortcut-up = Move up the list
shortcut-first = First in the list
shortcut-last = Last in the list
shortcut-page-down = Page down the list
shortcut-page-up = Page up the list
shortcut-open = Open conversation
shortcut-back = Back to the list
# Scrolls the open conversation.
shortcut-scroll-down = Scroll down
shortcut-scroll-up = Scroll up
shortcut-scroll-page-down = Scroll a page down
shortcut-scroll-page-up = Scroll a page up
# Writes a new message.
shortcut-compose = Compose
shortcut-reply = Reply
shortcut-reply-all = Reply all
shortcut-forward = Forward
shortcut-archive = Archive
shortcut-delete = Delete
shortcut-spam = Report spam
# Moves the conversation to a folder or label the user picks.
shortcut-move-to = Move to
shortcut-mark-read = Mark as read
shortcut-mark-unread = Mark as unread
shortcut-star = Star or unstar
shortcut-important = Mark as important
shortcut-not-important = Mark as not important
# Ticks the check box of the selected conversation in the list.
shortcut-check = Tick the conversation
shortcut-select-all = Tick all conversations
shortcut-select-none = Untick all conversations
shortcut-undo = Undo the last action
# Under "Go to": opens the Inbox folder.
shortcut-go-inbox = Inbox
# Under "Go to": opens the Starred folder.
shortcut-go-starred = Starred
# Under "Go to": opens the Sent folder.
shortcut-go-sent = Sent
# Under "Go to": opens the Drafts folder.
shortcut-go-drafts = Drafts
# Under "Go to": opens All mail, every message of the account.
shortcut-go-all = All mail
shortcut-search = Search mail
# Shows or hides the folder pane on the left.
shortcut-navigation = Show or fold the menu
shortcut-quick-settings = Quick settings
# Opens the full Settings page.
shortcut-settings = All settings
shortcut-shortcuts = Keyboard shortcuts
shortcut-reload = Check for new mail
shortcut-quit = Quit

## Keys pressed one after another, as a shortcut shows them ("G then I")

# $first: the key or keys pressed first (for example "G"); $second: the key pressed next (for example "I"). Key names stay as printed on keyboards.
shortcut-sequence = { $first } then { $second }

## Settings > Accounts

# Settings row: which accounts the folder pane lists.
accounts-folder-pane = Folder pane
accounts-folder-pane-detail = Which accounts' folders the pane on the left shows.
# Radio choice under "Folder pane"; the account card is the account's picture and name at the top of the pane.
accounts-shown-one = One account at a time; switch in the account card
accounts-shown-all = All accounts, one after another
# Settings row: the list of accounts.
accounts-row = Accounts
accounts-row-detail = Removing an account deletes Katna's copy of its mail on this computer. The mail stays on the server.
accounts-none = No accounts yet.
# Account type shown after the address, for mail imported from files (mbox, Maildir…).
accounts-kind-imported = Imported
# Button: the account uses the picture of the desktop's user account again.
accounts-picture-reset = Use desktop picture
accounts-picture-change = Change picture
# Button: removes the account from Katna.
accounts-remove = Remove
# Settings row: deletes everything Katna stores.
accounts-delete-all-row = Delete all data
accounts-delete-all-row-detail = Start over, as on a new install.
accounts-delete-all-about = Deletes every account, all stored mail, contacts and calendars, the search index, your settings and saved passwords from this computer. Nothing changes on your mail servers.
accounts-delete-all-open = Delete all Katna data

## Settings > Accounts: snackbars after deleting

# $address: the account's email address.
accounts-removed-local = { $address } was removed from Katna.
# $address: the account's email address.
accounts-removed = { $address } was removed from Katna. Its mail is still on the server.
accounts-all-deleted = All Katna data was deleted from this computer.

## Settings > Accounts: the dialog that asks before deleting

# Title. $address: the account's email address.
accounts-remove-title = Remove { $address }?
# Red button of the dialog.
accounts-remove-confirm = Remove account
accounts-removing = Removing…
# Listed under "Deleted from this computer:", for an account of mail imported from files. $folders: how many folders the account has.
accounts-remove-local-mail = { $folders ->
    [0] All mail imported into this account
    [one] All mail imported into this account in its folder
   *[other] All mail imported into this account in its { $folders } folders
}
accounts-remove-local-settings = Its Katna settings
# Listed under "Deleted from this computer:". $folders: how many folders the account has.
accounts-remove-mail = { $folders ->
    [0] All of this account's mail stored by Katna
    [one] All of this account's mail stored by Katna in its folder
   *[other] All of this account's mail stored by Katna in its { $folders } folders
}
accounts-remove-outbox = Its messages waiting in the outbox
accounts-remove-settings = Its saved password and its Katna settings
accounts-delete-all-title = Delete all Katna data?
# Red button of the dialog.
accounts-delete-all-confirm = Delete everything
accounts-deleting = Deleting…
# Listed under "Deleted from this computer:".
accounts-delete-all-accounts = Every account, and all mail and attachments stored by Katna
accounts-delete-all-contacts = Contacts, calendars and the search index
accounts-delete-all-settings = All settings, signatures and keyboard shortcuts
accounts-delete-all-passwords = Every saved password
# Heading of the red list of what goes.
accounts-deleted-heading = Deleted from this computer:
accounts-cannot-undo = This cannot be undone.
accounts-server-delete-all = Nothing changes on your mail servers: your mail stays there, and adding an account again downloads it again. Mail imported from files is only in Katna; the files are not touched.
accounts-server-local = This mail was imported from files, so Katna has the only copy. The files it came from are not touched; import them again to get it back.
accounts-server-remove = Nothing changes on the mail server: your mail stays there, and adding the account again downloads it again.
# The word to type before everything is deleted; one lowercase word that is easy to type.
accounts-confirm-word = delete
# Placeholder of the field where the word is typed.
accounts-confirm-placeholder = Type “{ accounts-confirm-word }”
accounts-confirm-prompt = To confirm, type “{ accounts-confirm-word }”:
accounts-cancel = Cancel
