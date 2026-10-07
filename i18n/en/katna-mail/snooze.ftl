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
