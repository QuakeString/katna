# Katna Mail, English: mail list.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

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
# Tooltip of the refresh button while it checks for new mail.
list-checking = Checking for new mail…
# Tooltip of the "more actions" button (three dots).
list-more = More
list-mark-read = Mark as read
list-mark-unread = Mark as unread
list-move-to = Move to
list-archive = Archive
list-spam = Report spam
list-delete = Delete
# Tooltip of the clock button on a line under the pointer: opens the
# snooze times.
list-snooze = Snooze
# The same button in the Snoozed folder: brings the mail back now.
list-unsnooze = Unsnooze
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
# After choosing Read, Unread, Starred or Unstarred in the select menu,
# which ticks such lines on screen; the link then ticks every such line.
# $pick: "read", "unread", "starred" or "unstarred".
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] The { $count } read conversation on screen is selected.
           *[other] All { $count } read conversations on screen are selected.
        }
       *[message] { $count ->
            [one] The { $count } read message on screen is selected.
           *[other] All { $count } read messages on screen are selected.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] The { $count } unread conversation on screen is selected.
           *[other] All { $count } unread conversations on screen are selected.
        }
       *[message] { $count ->
            [one] The { $count } unread message on screen is selected.
           *[other] All { $count } unread messages on screen are selected.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] The { $count } starred conversation on screen is selected.
           *[other] All { $count } starred conversations on screen are selected.
        }
       *[message] { $count ->
            [one] The { $count } starred message on screen is selected.
           *[other] All { $count } starred messages on screen are selected.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] The { $count } unstarred conversation on screen is selected.
           *[other] All { $count } unstarred conversations on screen are selected.
        }
       *[message] { $count ->
            [one] The { $count } unstarred message on screen is selected.
           *[other] All { $count } unstarred messages on screen are selected.
        }
    }
}
# A link that ticks every such line, not only those on screen.
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } read conversation
           *[other] Select all { $count } read conversations
        }
       *[message] { $count ->
            [one] Select the { $count } read message
           *[other] Select all { $count } read messages
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } unread conversation
           *[other] Select all { $count } unread conversations
        }
       *[message] { $count ->
            [one] Select the { $count } unread message
           *[other] Select all { $count } unread messages
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } starred conversation
           *[other] Select all { $count } starred conversations
        }
       *[message] { $count ->
            [one] Select the { $count } starred message
           *[other] Select all { $count } starred messages
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } unstarred conversation
           *[other] Select all { $count } unstarred conversations
        }
       *[message] { $count ->
            [one] Select the { $count } unstarred message
           *[other] Select all { $count } unstarred messages
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } read conversation in { $folder }
           *[other] Select all { $count } read conversations in { $folder }
        }
       *[message] { $count ->
            [one] Select the { $count } read message in { $folder }
           *[other] Select all { $count } read messages in { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } unread conversation in { $folder }
           *[other] Select all { $count } unread conversations in { $folder }
        }
       *[message] { $count ->
            [one] Select the { $count } unread message in { $folder }
           *[other] Select all { $count } unread messages in { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } starred conversation in { $folder }
           *[other] Select all { $count } starred conversations in { $folder }
        }
       *[message] { $count ->
            [one] Select the { $count } starred message in { $folder }
           *[other] Select all { $count } starred messages in { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Select the { $count } unstarred conversation in { $folder }
           *[other] Select all { $count } unstarred conversations in { $folder }
        }
       *[message] { $count ->
            [one] Select the { $count } unstarred message in { $folder }
           *[other] Select all { $count } unstarred messages in { $folder }
        }
    }
}
# Once the link ticked every such line of the list.
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } read conversation is selected.
           *[other] All { $count } read conversations are selected.
        }
       *[message] { $count ->
            [one] { $count } read message is selected.
           *[other] All { $count } read messages are selected.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } unread conversation is selected.
           *[other] All { $count } unread conversations are selected.
        }
       *[message] { $count ->
            [one] { $count } unread message is selected.
           *[other] All { $count } unread messages are selected.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } starred conversation is selected.
           *[other] All { $count } starred conversations are selected.
        }
       *[message] { $count ->
            [one] { $count } starred message is selected.
           *[other] All { $count } starred messages are selected.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } unstarred conversation is selected.
           *[other] All { $count } unstarred conversations are selected.
        }
       *[message] { $count ->
            [one] { $count } unstarred message is selected.
           *[other] All { $count } unstarred messages are selected.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } read conversation in { $folder } is selected.
           *[other] All { $count } read conversations in { $folder } are selected.
        }
       *[message] { $count ->
            [one] { $count } read message in { $folder } is selected.
           *[other] All { $count } read messages in { $folder } are selected.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } unread conversation in { $folder } is selected.
           *[other] All { $count } unread conversations in { $folder } are selected.
        }
       *[message] { $count ->
            [one] { $count } unread message in { $folder } is selected.
           *[other] All { $count } unread messages in { $folder } are selected.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } starred conversation in { $folder } is selected.
           *[other] All { $count } starred conversations in { $folder } are selected.
        }
       *[message] { $count ->
            [one] { $count } starred message in { $folder } is selected.
           *[other] All { $count } starred messages in { $folder } are selected.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } unstarred conversation in { $folder } is selected.
           *[other] All { $count } unstarred conversations in { $folder } are selected.
        }
       *[message] { $count ->
            [one] { $count } unstarred message in { $folder } is selected.
           *[other] All { $count } unstarred messages in { $folder } are selected.
        }
    }
}
# A notice when the select menu's choice matches no line.
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] No read conversations here.
       *[message] No read messages here.
    }
   *[unread] { $kind ->
        [conversation] No unread conversations here.
       *[message] No unread messages here.
    }
    [starred] { $kind ->
        [conversation] No starred conversations here.
       *[message] No starred messages here.
    }
    [unstarred] { $kind ->
        [conversation] No unstarred conversations here.
       *[message] No unstarred messages here.
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
# Tooltips of the eye on a line of mail sent with open and click tracking.
# $opened and $clicked: how many of its $recipients opened it or followed
# a link in it.
row-tracking-none = Tracked. Not opened yet
row-tracking-opened = Opened by { $opened } of { $recipients }
row-tracking-clicked = Opened by { $opened } of { $recipients }, a link followed by { $clicked }
# Tooltips of the pin button shown on a line under the pointer.
row-pin = Pin to top
row-unpin = Unpin
# Tooltip of the time on a snoozed line. $when: date and time it comes back.
row-snoozed-until = Snoozed until { $when }

## Mail list: More menu and right-click menu

menu-reply = Reply
menu-reply-all = Reply all
menu-forward = Forward
menu-archive = Archive
menu-delete = Delete
# In Trash: deletes the mail for good.
menu-delete-forever = Delete forever
# In Trash, Archive and All Mail: back to the inbox.
menu-move-to-inbox = Move to Inbox
menu-spam = Report spam
# In the Spam folder, in place of "Report spam": back to the inbox.
menu-not-spam = Not spam
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
# Opens the snooze times.
menu-snooze = Snooze
# In the Snoozed folder: brings the mail back to the inbox now.
menu-unsnooze = Unsnooze
# Makes a task from the mail, as Gmail's "Add to Tasks".
menu-add-to-tasks = Add to Tasks
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
# $when: the date and time the mail comes back.
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Conversation snoozed until { $when }.
       *[other] { $count } conversations snoozed until { $when }.
    }
   *[message] { $count ->
        [one] Message snoozed until { $when }.
       *[other] { $count } messages snoozed until { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Conversation back in the inbox.
       *[other] { $count } conversations back in the inbox.
    }
   *[message] { $count ->
        [one] Message back in the inbox.
       *[other] { $count } messages back in the inbox.
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
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversation marked as not spam and moved to the inbox.
       *[other] { $count } conversations marked as not spam and moved to the inbox.
    }
   *[message] { $count ->
        [one] Message marked as not spam and moved to the inbox.
       *[other] { $count } messages marked as not spam and moved to the inbox.
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
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Conversation marked as read.
       *[other] { $count } conversations marked as read.
    }
   *[message] { $count ->
        [one] Message marked as read.
       *[other] { $count } messages marked as read.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Conversation marked as unread.
       *[other] { $count } conversations marked as unread.
    }
   *[message] { $count ->
        [one] Message marked as unread.
       *[other] { $count } messages marked as unread.
    }
}
# After Undo on the snackbar took an action back.
toast-undone = Action undone.
# Ctrl+Z when nothing done in this window is left to take back.
toast-nothing-to-undo = Nothing to undo.
# Ctrl+Z right after mail was deleted forever (from Trash).
toast-cannot-undo-delete-forever = Mail deleted forever can't be brought back.
# After Undo on "Message sent": the message opens again, not sent.
toast-send-undone = Sending undone.
# Undo on a sent message once it has already gone to the mail server.
toast-too-late-to-undo-send = Too late to undo: the message has already been sent.
# The snackbar's button that takes the action back.
toast-undo = Undo
# The snackbar's x, which closes it.
toast-close = Close
toast-no-spam-folder = This account has no spam folder.
