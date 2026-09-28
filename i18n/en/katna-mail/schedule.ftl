# Katna Mail, English: schedule send and the list of scheduled mail.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## The menu beside Send

# Opens the suggested times; also the button of the date and time picker.
schedule-send = Schedule send
# Opens the list of messages waiting to go out later.
schedule-scheduled-messages = Scheduled messages ({ $count })

## Schedule send's suggested times

# The title of the menu of suggested times.
schedule-title = Schedule send
# Under the title. $zone: the time zone's name, such as Asia/Kolkata.
schedule-zone-note = { $zone }. Katna sends it at that time, even with the app closed.
# Instead of schedule-zone-note when the account's mail server holds mail
# until its time (SMTP FUTURERELEASE).
schedule-zone-note-server = { $zone }. Your mail server will send it at that time, even with this computer off. Once Undo is gone it can't be cancelled.
# Instead of schedule-zone-note when the mail server cannot hold mail.
schedule-zone-note-local = { $zone }. Katna will send it at that time while this computer is on.
# Instead of the zone's name when the system does not tell it.
schedule-local-time = Local time
# The suggested times; the date and time are shown beside each.
schedule-this-morning = This morning
schedule-this-afternoon = This afternoon
schedule-tomorrow-morning = Tomorrow morning
schedule-tomorrow-afternoon = Tomorrow afternoon
schedule-monday-morning = Monday morning
# Opens the date and time picker; also its title.
schedule-pick = Pick date & time

## The date and time picker

# $text: what was typed. $example: a time written as the app writes times.
schedule-not-a-time = “{ $text }” is not a time, for example { $example }.
# The date and time chosen are skipped by a daylight saving change.
schedule-no-such-time = That time does not exist here.

## The list of scheduled mail

# The subject of a scheduled message that has none.
schedule-no-subject = (no subject)
# $when: the date and time it goes out.
schedule-sends-at = Sends { $when }
# A message the mail server holds; it can't be cancelled any more.
# $when: the date and time it goes out.
schedule-server-sends-at = Your mail server sends it { $when }
# Takes the message back and opens it to edit.
schedule-cancel-send = Cancel send
schedule-nothing = Nothing is scheduled.
schedule-close = Close
# Cancel send while another message is being written.
schedule-open-first = Send or discard the open message first.
schedule-cancelled = Send cancelled. The message is open to edit.
schedule-cancelled-not-opened = Send cancelled. The message could not be opened again.
