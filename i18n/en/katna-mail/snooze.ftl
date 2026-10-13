# Katna Mail, English: snooze (the menu of times and the date and time picker).
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

# The heading over the suggested times.
snooze-until = Snooze until…
# The suggested times; the date and time are shown beside each.
snooze-later-today = Later today
snooze-tomorrow = Tomorrow
snooze-this-weekend = This weekend
snooze-next-week = Next week
# Opens the date and time picker; also its title.
snooze-pick = Pick date & time
snooze-back = Back to the times
snooze-type-placeholder = Type a time
snooze-type-hint = Like “tue 3pm”, “tomorrow” or “in 2 hours”
snooze-type-hint-unclear = Katna can’t read that as a time
snooze-type-unclear = “{ $text }” isn't a time Katna knows

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Snooze
remind-tab = Remind me
# Under the switch: what each does.
snooze-says = Hides it until then
remind-says = Keeps it where it is and notifies you
# The first time offered when the mail says something is due by a day.
remind-before-due = Before it's due
# The note field: what the reminder (a task in Tasks) says.
remind-note = Note (optional)
remind-note-placeholder = The subject, if left empty
# After a reminder is set. $date: "Thu, Oct 8, 2026, 8:00 AM".
toast-remind-set = Reminder set for { $date }
# A reminder at the end of a chat, as a small line.
# $date: "Oct 9, 8:00 AM"; $title: what the reminder says.
remind-chat-line = Reminder { $date } · { $title }
# Ticks the reminder off (its task is done).
remind-done = Done
toast-remind-done = Reminder done
# A snoozed conversation open in a chat: a small line at its end.
# $date: "Oct 9, 8:00 AM".
snooze-chat-line = Snoozed until { $date }
# Picks another time for the snooze.
snooze-chat-change = Change

## The date and time picker

snooze-cancel = Cancel
# Snoozes until the chosen date and time.
snooze-save = Save
# The chosen date and time has passed.
snooze-in-the-past = Pick a time later than now.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

# The send menu's item that opens the popover.
follow-up-menu = Follow up if no reply…
# The popover's title. If nobody answers in time, the daemon brings the
# conversation back to the Inbox, or sends a follow-up for the user.
follow-up-title = Follow up if no reply
# The chips for when.
follow-up-off = Off
follow-up-days = { $days ->
    [one] { $days } day
   *[other] { $days } days
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } week
   *[other] { $weeks } weeks
}
# Opens the date and time picker.
follow-up-pick = Pick…
# The picker's title when it chooses when to follow up.
follow-up-pick-title = Follow up if no reply by
# The two ways, each with a line under it.
follow-up-remind = Remind me
follow-up-remind-note = The conversation comes back to the top of your Inbox
follow-up-send = Send a follow-up for me
follow-up-send-note = To the same people, in the same conversation
# Under "Send a follow-up for me" when the message is encrypted: a
# follow-up would quote it unencrypted.
follow-up-send-encrypted = Not for encrypted mail
# The follow-up's text box: when empty, and the text it starts with.
# $name: the first name of the first recipient.
follow-up-text-placeholder = What to write
follow-up-text-named = Hi { $name }, just checking you saw my message below.
follow-up-text = Hi, just checking you saw my message below.
# Puts a saved template's text in the box.
follow-up-template = Use a template
follow-up-signature = Your signature is added
# Beside a switch and a duration (3 days, 1 week, 2 weeks).
follow-up-again = If still no reply, follow up again after
# The note at the bottom, for each way. $start and $end: times of day.
follow-up-note = Stops as soon as anyone in the conversation replies. Auto-replies don't count.
follow-up-note-send = Stops as soon as anyone in the conversation replies. Goes out on weekdays from { $start } to { $end }, and never more than a day late.
follow-up-cancel = Cancel
follow-up-done = Done
# The chip beside Send. $time: "3 days", "1 week"; $date: a date and time.
follow-up-chip-send = Follow-up in { $time }
follow-up-chip-remind = Reminder in { $time }
follow-up-chip-send-on = Follow-up { $date }
follow-up-chip-remind-on = Reminder { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = No reply yet
follow-up-card-title-waiting = Your follow-up is waiting
follow-up-card-send = Katna sends your follow-up on { $date }. It stops when anyone replies.
follow-up-card-send-twice = Katna sends your follow-up on { $date }, then once more later. It stops when anyone replies.
follow-up-card-remind = If nobody replies, this conversation comes back to your Inbox on { $date }.
follow-up-card-waiting = It fell due while your computer was off, so it wasn't sent late. Send it now, pick a new time, or stop it.
follow-up-card-edit = Edit
# Over the times Edit offers.
follow-up-card-edit-title = Follow up on
follow-up-card-send-now = Send now
follow-up-card-stop = Stop
# The follow-up waiting at the end of a chat, over its text.
follow-up-chat-send = Follow-up · { $date } if nobody replies
follow-up-chat-step = Follow-up { $step } of { $steps } · { $date } if nobody replies
follow-up-chat-waiting = Follow-up waiting · it fell due while your computer was off
# A reminder follow-up at the end of a chat, as a small line.
follow-up-chat-remind = Back in Inbox { $date } if no reply
toast-follow-up-sent = Follow-up sent
toast-follow-up-stopped = Follow-up stopped
toast-follow-up-moved = Follow-up moved to { $date }

# Nudges: a question the user sent that nobody answered in 3 days comes
# back to the top of the Inbox. $days: whole days since it was sent.
nudge-row = Sent { $days ->
    [one] 1 day ago
   *[other] { $days } days ago
}. Follow up?
nudge-row-tip = Write a follow-up to everyone in it
nudge-follow-up = Follow up
nudge-dismiss = Dismiss
nudge-card-title = No reply yet
nudge-card-text = You asked something { $days ->
    [one] 1 day ago
   *[other] { $days } days ago
} and nobody answered.
nudge-chat-line = Sent { $days ->
    [one] 1 day ago
   *[other] { $days } days ago
}, no reply yet
toast-nudge-dismissed = Nudge dismissed
