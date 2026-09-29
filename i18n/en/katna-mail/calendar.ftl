# SPDX-License-Identifier: GPL-3.0-or-later
# The Calendar page of the Katna window.

calendar-today = Today
# Tooltip of Today: the date it goes to.
calendar-today-tip = Go to today
calendar-view-day = Day
calendar-view-week = Week
calendar-view-month = Month
calendar-view-schedule = Schedule
# The gear at the right of the calendar bar, and its menu.
calendar-options = Options
calendar-density = Density
calendar-density-responsive = Responsive to your screen
calendar-density-comfortable = Comfortable
calendar-density-compact = Compact
calendar-second-zone = Second time zone
calendar-zone-none = None
# A time zone to choose. $zone: its city ("New York"), $offset: "GMT+5:30".
calendar-zone = { $zone } ({ $offset })
calendar-previous-day = Previous day
calendar-next-day = Next day
calendar-previous-week = Previous week
calendar-next-week = Next week
calendar-previous-month = Previous month
calendar-next-month = Next month
calendar-previous-period = Earlier
calendar-next-period = Later
# The title when the days shown cross two months: "September – October 2026".
calendar-title-months = { $first } – { $last }
calendar-loading = Loading…
calendar-read-failed = The calendar could not be read: { $error }
# Calendars kept on this computer, without an account.
calendar-local = On this computer
calendar-account-gone = Removed account
calendar-empty-title = No calendars yet
calendar-empty-text = Katna shows the calendars of your Google and Microsoft accounts here once they are synced, and those of other servers that offer CalDAV.
calendar-schedule-empty = Nothing planned for the next two months.
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
calendar-repeats = Repeats
calendar-join = Join
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
# Tabs above a new event's times, as Google Calendar's.
calendar-kind-event = Event
calendar-kind-focus = Focus time
calendar-kind-out-of-office = Out of office
calendar-kind-working-location = Working location
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
