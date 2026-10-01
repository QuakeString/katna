# Katna Mail, English: the Files page, every attachment of every account
# in one place.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Top bar

# Placeholder of the search box while the Files page shows.
files-search = Search files

## Left side (and chips on a phone)

files-all = All files
files-pictures = Pictures
files-pdfs = PDFs
# Word, OpenDocument and plain text files.
files-documents = Documents
files-sheets = Spreadsheets
files-slides = Slides
files-other = Other
# Heading over the accounts, each of which narrows the page to its files.
files-accounts = Accounts
# Heading over "Received" and "Sent by me".
files-shown = Shown
files-received = Received
files-sent = Sent by me

## Over the files

# $count files, $size in all, such as "1,284 files · 2.3 GB".
files-count = { $count ->
    [one] { $count } file · { $size }
   *[other] { $count } files · { $size }
}
# The chip that picks whose files show.
files-anyone = Anyone
# The same chip once a person is picked. $name: their name.
files-from-person = From { $name }
# The chip that picks the days the files are from, and its calendar.
files-time-any = Any time
files-time-today = Today
files-time-yesterday = Yesterday
files-time-this-week = This week
files-time-last-week = Last week
files-time-this-month = This month
files-time-last-month = Last month
# Days picked on the calendar, such as "12 Sep – 28 Sep".
files-time-between = { $first } – { $last }
# Under the calendar before any day is picked.
files-time-hint = Click a day, or drag across days
# Under the calendar: $days picked (as on the chip) and the files they hold.
files-time-summary = { $count ->
    [one] { $days } · { $count } file
   *[other] { $days } · { $count } files
}
files-time-clear = Clear
files-time-month-back = Previous month
files-time-month-on = Next month
# Tooltip of the chip once days are picked.
files-time-wheel = Scroll to move these dates, keeping their length
files-sort-newest = Newest first
files-sort-oldest = Oldest first
files-sort-largest = Largest first
files-sort-name = By name
# Buttons that show the files as cards or as a list.
files-grid = Cards
files-list = List
# Heading over the files of the past seven days.
files-this-week = This week
# Heading over files whose mail has no date.
files-undated = No date
# Sender of mail from one of your own accounts.
files-me = Me
# In place of a missing subject.
files-no-subject = (no subject)
files-loading = Gathering files from your mail…
files-empty = Files from your mail show up here.
files-none-match = No files match.
# $error: why reading the files failed.
files-load-failed = Reading the files failed: { $error }

## A file's menu and buttons

files-open = Open
files-open-with = Open with…
files-save = Save…
# Opens the mail the file came with. Also a button on the file's card and
# in the viewer.
files-show-mail = Show the mail
files-mail-window = Open the mail in a new window
# Starts a new mail with the file attached.
files-forward = Forward the file
# $name: who sent the file.
files-from-them = Files from { $name }
files-copy-name = Copy file name
files-name-copied = File name copied
# The file's mail is being downloaded before the file is saved or sent.
files-downloading = Downloading the mail…
files-download-failed = Could not download this mail.
