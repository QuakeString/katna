# Katna Mail, English: search options.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Search options (the panel from the button at the right of the search box)

search-options = Search options
# Closes the panel.
search-options-close = Close
# The fields: mail from, to, with a subject, with or without some words.
search-from = From
search-to = To
search-subject = Subject
search-has-words = Has the words
search-without = Doesn't have
# Followed by choices of how recent the mail is.
search-date-within = Date within
search-has-attachment = Has attachment
# Empties every field of the panel.
search-clear-filter = Clear filter

## Search options: "Date within" choices

search-within-any = Any time
# $count: how many days back.
search-within-days = { $count ->
    [one] { $count } day
   *[other] { $count } days
}
# $count: how many weeks back.
search-within-weeks = { $count ->
    [one] { $count } week
   *[other] { $count } weeks
}
# $count: how many months back.
search-within-months = { $count ->
    [one] { $count } month
   *[other] { $count } months
}
# $count: how many years back.
search-within-years = { $count ->
    [one] { $count } year
   *[other] { $count } years
}
# Opens a calendar to pick the dates.
search-within-custom = Custom

## Search options: custom dates (the calendar popover)

# Mail on, before, since or between the picked dates.
search-dates-on = On
search-dates-before = Before
search-dates-since = Since
search-dates-between = Between
# The start and end of the span, for "Between".
search-dates-from = From
search-dates-to = To
# In an empty date field: dates are typed as year-month-day in digits.
# Translate the letters only; the order and dashes stay.
search-dates-placeholder = YYYY-MM-DD
search-dates-missing = Pick a date
# Dates are typed as year-month-day; keep the example date as it is.
search-dates-unreadable = Use a date like 2026-09-01
search-dates-out-of-range = That date is out of range
# The chip text after picking dates. $date: a date, such as May 14, 2002.
search-dates-chip-before = Before { $date }
# $date: a date, such as May 14, 2002.
search-dates-chip-since = Since { $date }
# $first and $last: the first and last dates, such as May 1 and May 31, 2002.
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Cancel
# Keeps the picked dates and closes the calendar.
search-dates-done = Done
# Tooltips on the arrows beside the month and the year.
search-dates-month-back = Previous month
search-dates-month-on = Next month
search-dates-year-back = Previous year
search-dates-year-on = Next year
