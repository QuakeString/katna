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

## Remind me if no reply, in compose's send menu

# The heading of the choices; the daemon brings the conversation back to
# the Inbox when nobody has answered by then.
follow-up-title = Remind me if no reply
follow-up-off = Don't remind me
follow-up-days = { $days ->
    [one] After { $days } day
   *[other] After { $days } days
}
