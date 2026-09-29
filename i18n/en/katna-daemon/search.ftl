# Katna service, English: Katna's results in the desktop's search (KRunner
# on KDE Plasma, the Activities search on GNOME).
# Guide: i18n/README.md. Keep ids stable; change the text freely.

## Results

# The heading over messages in KRunner's results.
search-category-mail = Mail
# The heading over people (addresses from the mail) in KRunner's results.
search-category-people = People
# The heading over tasks in KRunner's results.
search-category-tasks = Tasks
# The heading over events in KRunner's results.
search-category-events = Events
# The line under a message's subject: who sent it.
search-mail-from = From { $sender }
# A message without a subject.
search-no-subject = (no subject)
# The line under a task's title: the list it is in.
search-task-in = In { $list }
# The line under an event's title: when its next time is, and where or in
# which calendar. $when is one of the four below.
search-event-at = { $when } · { $place }
search-event-in = { $when } · { $calendar }
search-event-now = Now
search-event-today = Today
search-event-tomorrow = Tomorrow
search-event-in-days = In { $count } days

## Buttons on a result in KRunner

# On a message: opens it and starts a reply to everyone on it.
search-reply-all = Reply all
# On a person: copies their email address.
search-copy-address = Copy address
# On a person: searches Katna Mail for mail from or to them.
search-find-mail = Find mail
