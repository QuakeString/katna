# SPDX-License-Identifier: GPL-3.0-or-later
# The Calendar page of the Katna window.

calendar-today = Today
# Tooltip of Today: the date it goes to.
calendar-today-tip = Go to today
calendar-view-day = Day
calendar-view-week = Week
calendar-view-month = Month
calendar-view-year = Year
calendar-view-schedule = Schedule
# The custom view, as many days as chosen in Options.
calendar-view-days = { $count } days
# The gear at the right of the calendar bar, and its menu.
calendar-options = Options
calendar-density = Density
calendar-density-responsive = Responsive to your screen
calendar-density-comfortable = Comfortable
calendar-density-compact = Compact
# How many days the custom view shows: a row of numbers.
calendar-custom-days = Custom view
calendar-second-zone = Second time zone
calendar-zone-none = None
# A time zone to choose. $zone: its city ("New York"), $offset: "GMT+5:30".
calendar-zone = { $zone } ({ $offset })
# Opens a new message listing the free times of the next working days.
calendar-share-free = Share free times
calendar-free-subject = Times I'm free
# $zone: "GMT+5:30".
calendar-free-intro = Here are some times I'm free ({ $zone }):
# $weekday: "Wednesday", $date: "30 Sep", $times: ranges, separated by commas.
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = I have no free time in the next few working days.
calendar-previous-day = Previous day
calendar-next-day = Next day
calendar-previous-week = Previous week
calendar-next-week = Next week
calendar-previous-month = Previous month
calendar-next-month = Next month
calendar-previous-year = Previous year
calendar-next-year = Next year
calendar-previous-period = Earlier
calendar-next-period = Later
# The title when the days shown cross two months: "September – October 2026".
calendar-title-months = { $first } – { $last }
calendar-loading = Loading…
calendar-read-failed = The calendar could not be read: { $error }
# Named groups of calendars in the side column ("Work", "Personal"): a
# click shows only a set's calendars.
calendar-sets = Calendar sets
calendar-set-add = Save the calendars on show as a set
calendar-set-name = Name of the set
calendar-set-remove = Remove set
# Calendars kept on this computer, without an account.
calendar-local = On this computer
calendar-account-gone = Removed account
# The line under an account in the side column that shows no calendars:
# why, and the one click that fixes it.
calendar-account-sign-in = Sign in again to show calendars
calendar-account-signed-in = Signed in to { $address } again. Getting your calendars…
calendar-account-sign-in-refused = { $provider } did not let Katna in. Try again, and allow access to your calendars.
calendar-account-refused = The server did not accept the password. Yahoo, iCloud, Zoho and others need an app password.
calendar-account-change-password = Change password
calendar-account-change-password-tooltip = Type the new password; Katna checks it with the server
calendar-account-not-enabled = Calendar access for Katna is not switched on yet.
calendar-account-failed = The calendars could not be read.
# $reason is the server's own words, in English.
calendar-account-error = The calendars could not be read: { $reason }
calendar-account-none = No calendars found
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = No calendars found: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } shows calendars only to Katna signed in with { $provider }.
calendar-account-sign-in-with = Sign in with { $provider }
calendar-account-looking = Looking for calendars…
calendar-account-try-again = Try again
calendar-account-try-again-tooltip = Check this account's calendars again now
calendar-account-fixing = Working on it…
# The calendar of saved contacts' birthdays, made on this computer.
calendar-birthdays = Birthdays
# The side list's switch that shows or hides tasks on the Calendar.
calendar-tasks = Tasks
# A saved contact's birthday on the Calendar: "Asha Rao's birthday".
calendar-birthday-of = { $name }'s birthday
calendar-empty-title = No calendars yet
calendar-empty-text = Katna shows the calendars of your Google and Microsoft accounts here once they are synced, and those of other servers that offer CalDAV.
calendar-schedule-empty = Nothing planned for the next two months.
# The top bar's search box on the Calendar page, its results' heading for
# events already over, and when nothing matches.
calendar-search = Search events
calendar-search-past = Past events
calendar-search-none = No events match your search.
calendar-no-title = (No title)
calendar-all-day = All day
# "9:00 – 9:30 AM": when an event starts and ends.
calendar-time-range = { $start } – { $end }
# A short event in the grid: "Standup, 9:00 AM".
calendar-short-event = { $title }, { $time }
# "Tuesday, 29 September · 10:30 – 11:30 AM"
calendar-when = { $day } · { $time }
# A whole-day event over several days: "2 Oct – 4 Oct 2026".
calendar-days-range = { $first } – { $last }
# In a month's day when not all its events fit.
calendar-more = { $count } more
# The Year view's day popover heading: "Tuesday, 29 Sept".
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Repeats
calendar-join = Join
# The Join button of a call found in an event: "Join with Microsoft Teams".
calendar-join-with = Join with { $service }
# Opens a new mail to the event's guests.
calendar-email-guests = Email guests
# Opens a new mail telling the event's guests the user is late.
calendar-running-late = Running late
calendar-late-subject = Running late: { $title }
calendar-late-body = Sorry, I'm running a few minutes late for { $title }. I'll be there soon.
calendar-guests =
    { $count ->
        [one] 1 guest
       *[other] { $count } guests
    }
calendar-guest-answers = { $yes } yes, { $maybe } maybe, { $no } no, { $waiting } waiting
calendar-organizer = Organizer
calendar-optional = Optional
calendar-open-web = Open in the browser
calendar-open-mail = Open the mail
calendar-open-contact = Open contact
calendar-close = Close

## Adding, changing and deleting events.

calendar-add-title = Add title
calendar-add-location = Add location
calendar-add-notes = Add description
calendar-add-guests = Add guests
calendar-remove-guest = Remove
calendar-add-meet = Add Google Meet video call
calendar-add-teams = Add Teams meeting
calendar-has-call = Video call added
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = All day
calendar-more-options = More options
calendar-save = Save
calendar-saved = Event saved
calendar-deleted = Event deleted
calendar-discard = Discard changes
calendar-edit = Edit event
calendar-delete = Delete event
calendar-event-details = Event details
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = New event
# The title of the small window the desktop clock's Add… opens.
calendar-event-window-title = New event
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Open day
calendar-menu-duplicate = Duplicate
calendar-menu-color = Color
# The event takes its calendar's color.
calendar-menu-color-calendar = Calendar color
# A task's new due day, a week from today.
calendar-menu-in-a-week = In a week
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Tomato
calendar-color-flamingo = Flamingo
calendar-color-tangerine = Tangerine
calendar-color-banana = Banana
calendar-color-sage = Sage
calendar-color-basil = Basil
calendar-color-peacock = Peacock
calendar-color-blueberry = Blueberry
calendar-color-lavender = Lavender
calendar-color-grape = Grape
calendar-color-graphite = Graphite
# Right-click menus in the side panel: on a calendar, and on an
# account's heading.
calendar-menu-only-this = Show only this
calendar-menu-rename = Rename
# Takes a calendar shared with you off your list; its owner keeps it.
calendar-menu-remove = Remove from list
calendar-menu-delete = Delete
calendar-menu-new-calendar = New calendar
calendar-menu-show-all = Show all
calendar-menu-hide-all = Hide all
calendar-menu-account-settings = Account settings
# Why a side-panel menu item can't be used, shown dimmed after it. Short.
calendar-why-main = Main calendar
calendar-why-last = Only one here
calendar-why-owner = Owner only
calendar-why-contacts = From Contacts
calendar-why-unreached = Not reached
calendar-name-placeholder = Calendar name
calendar-toast-added = “{ $name }” added
calendar-toast-renamed = Calendar renamed
calendar-toast-recolored = Calendar color changed
calendar-toast-deleted = “{ $name }” deleted
calendar-toast-removed = “{ $name }” removed from your list
# The service refused a change to a calendar; $reason is its answer.
calendar-edit-failed = The calendar wasn't changed: { $reason }
calendar-delete-title = Delete “{ $name }”?
calendar-delete-confirm = Delete
calendar-deleting = Deleting…
calendar-delete-heading = Deleted:
calendar-delete-events = The calendar and all its events
calendar-delete-shared = For everyone it's shared with
# $account is the account's address.
calendar-delete-server = It's deleted from { $account } on the mail service, not only in Katna.
calendar-delete-local = It's deleted from this computer.
calendar-remove-title = Remove “{ $name }” from your list?
calendar-remove-confirm = Remove
calendar-removing = Removing…
calendar-remove-heading = What changes:
calendar-remove-events = You stop seeing its events, here and in your other apps
calendar-remove-server = The calendar stays with its owner, who can share it with you again.
# Tabs above a new event's times, as Google Calendar's.
calendar-kind-event = Event
calendar-kind-task = Task
calendar-kind-focus = Focus time
calendar-kind-out-of-office = Out of office
calendar-kind-working-location = Working location
calendar-task-added = Task added
calendar-task-added-to = Task added to { $list }
calendar-task-list-local = On this computer
# A new working location's title until another place is typed.
calendar-working-home = Home
calendar-busy = Busy
calendar-free = Free
calendar-cancel = Cancel
calendar-ok = OK
calendar-read-only = You can't change events in this calendar
calendar-none-editable = No calendar you can add events to yet
calendar-no-such-time = That time doesn't exist in your time zone
calendar-end-before-start = The event ends before it starts
calendar-repeat-never = Does not repeat
calendar-repeat-daily = Daily
calendar-repeat-weekly = Weekly on { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Monthly on the first { $weekday }
        [2] Monthly on the second { $weekday }
        [3] Monthly on the third { $weekday }
        [4] Monthly on the fourth { $weekday }
       *[other] Monthly on the last { $weekday }
    }
calendar-repeat-yearly = Annually on { $day }
calendar-repeat-weekdays = Every weekday (Monday to Friday)
calendar-repeat-custom = Custom
calendar-reminder-none = No notification
calendar-reminder-at-start = At the start
calendar-reminder-minutes =
    { $count ->
        [one] 1 minute before
       *[other] { $count } minutes before
    }
calendar-reminder-hours =
    { $count ->
        [one] 1 hour before
       *[other] { $count } hours before
    }
calendar-reminder-days =
    { $count ->
        [one] 1 day before
       *[other] { $count } days before
    }
calendar-scope-edit-title = Edit recurring event
calendar-scope-delete-title = Delete recurring event
calendar-scope-this = This event
calendar-scope-following = This and following events
calendar-scope-all = All events
calendar-scope-respond-title = Answer for a recurring event
calendar-going = Going?
calendar-answer-yes = Yes
calendar-answer-no = No
calendar-answer-maybe = Maybe
calendar-answered-yes = You're going
calendar-answered-no = You're not going
calendar-answered-maybe = You might go

## The card at the top of a mail with an invitation.

calendar-invite = Invitation
calendar-invite-cancelled = Event canceled
# An answer to the user's own invitation: "Priya Nair accepted".
calendar-invite-reply = { $name } answered
calendar-invite-reply-yes = { $name } accepted
calendar-invite-reply-no = { $name } declined
calendar-invite-reply-maybe = { $name } might go
calendar-invite-organizer = Organized by { $name }
calendar-invite-open = Open in Calendar
# The invitation is not in the user's calendar yet (it has not synced).
calendar-invite-not-yet = Not in your calendar yet. Answering is possible once it syncs.
# An invitation none of the user's calendars holds: the answer is mailed.
calendar-invite-by-mail = Not in your calendar: your answer goes to the organizer by mail.
# Subjects and text of an answer mailed to an invitation's organizer.
calendar-mail-yes = Accepted: { $title }
calendar-mail-yes-body = { $name } has accepted this invitation.
calendar-mail-no = Declined: { $title }
calendar-mail-no-body = { $name } has declined this invitation.
calendar-mail-maybe = Tentative: { $title }
calendar-mail-maybe-body = { $name } has tentatively accepted this invitation.
calendar-invite-your-day = Your day
calendar-invite-clashes =
    { $count ->
        [one] Clashes with 1 event
       *[other] Clashes with { $count } events
    }

## The day's agenda beside the mail.

agenda-show = Show the day's agenda
agenda-hide = Hide the agenda
# The agenda's title on today: "Today, 29 Sept".
agenda-today = Today, { $date }
# The agenda's title on another day: "Wed, 30 Sept".
agenda-day = { $weekday }, { $date }
agenda-empty = Nothing planned on this day.
