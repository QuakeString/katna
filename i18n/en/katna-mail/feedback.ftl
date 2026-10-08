# Katna Mail, English: User feedback.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Settings > User feedback (crash reports)

# At the top of the page, when crash reports are sent.
feedback-intro-sending = New crash reports are sent to help fix what went wrong. Nothing else leaves this computer.
# At the top of the page, when crash reports are not sent.
feedback-intro-local = Katna sends nothing anywhere. Crash reports stay on this computer, for you to look at or attach to a bug report.
feedback-crash-reports = Crash reports
feedback-crash-reports-detail = Written when Katna Mail or its background service crashes.
feedback-save = Save crash reports on this computer
feedback-save-detail = Your home folder, user and computer names and email addresses are left out
feedback-saved = Saved crash reports
# $count: how many reports are kept (20).
feedback-saved-detail = { $count ->
    [one] The newest { $count } is kept.
   *[other] The newest { $count } are kept.
}
feedback-help-improve = Help improve Katna
feedback-help-improve-detail = Off unless you turn it on, and you can turn it off here at any time.
feedback-send = Send crash reports
# Sentry is the name of the crash tracker.
feedback-send-detail = The saved report, exactly as you can view it here, goes to Katna's crash tracker (Sentry, in the EU). No IP address, messages or email addresses
feedback-none-saved = No crash reports are saved.
# Deletes every saved crash report.
feedback-delete-all = Delete all
# The program that crashed: katna-daemon, which syncs mail in the background.
feedback-app-daemon = Background service
# Under a crash report's program: when it crashed, and that it was sent. $date: the date and time.
feedback-report-sent = { $date } · Sent
# Opens a crash report.
feedback-view = View
feedback-view-tooltip = Open the report
feedback-copy-tooltip = Copy it to paste into a bug report
feedback-copied = Crash report copied.
feedback-deleted-all = Crash reports deleted.
# $error: the system's error, in English.
feedback-read-failed = Could not read the crash report: { $error }
feedback-delete-failed = Could not delete the crash report: { $error }
feedback-delete-all-failed = Could not delete the crash reports: { $error }

## Settings > User feedback (usage statistics)

feedback-usage = Send anonymous usage statistics
feedback-usage-detail = Once a week: which features you used, yes or no. Never counts, addresses, names or search words
# At the top of the page, when crash reports and usage statistics are sent.
feedback-intro-sending-usage = Crash reports and weekly usage statistics are sent. Nothing else leaves this computer.
# At the top of the page, when only usage statistics are sent.
feedback-intro-usage-only = Weekly usage statistics are sent. Crash reports stay on this computer.
feedback-counted = What is counted
feedback-counted-detail = Each one is yes or no for the week.
feedback-counted-also = Also: Katna's version, the Linux family, the desktop, the screen scale and how many accounts (1, 2–3, 4+)
# Opens the text of this week's usage statistics.
feedback-see-report = See this week's report
feedback-hide-report = Hide this week's report
# Under this week's report. $date: when the week ends and it is sent.
feedback-report-goes = Sent after the week ends, on { $date }, if usage statistics are still on.
# A random number that changes every 90 days. $id: its first and last digits.
feedback-install-id = Install ID { $id }
feedback-install-id-tooltip = Random, so one computer isn't counted twice in a week. It changes every 90 days and is never sent with crash reports or feedback
# Makes a new install ID at once.
feedback-install-id-reset = Reset
feedback-install-id-new = New install ID made.
feedback-report-copied = Report copied.
feedback-send-feedback = Feedback
feedback-send-feedback-detail = A problem, an idea, anything.
feedback-send-feedback-button = Send feedback…
usage-feature-search-options = Search options
usage-feature-pins = Pinned mail
usage-feature-labels = Labels
usage-feature-scheduled-send = Scheduled send
usage-feature-snooze = Snooze and reminders
usage-feature-encrypted = Encrypted mail
usage-feature-viewers = Built-in viewers
usage-feature-calendar = Calendar
usage-feature-contacts = Contacts
usage-feature-tasks-notes = Tasks and Notes
usage-feature-phone-layout = Phone-width layout
usage-feature-own-frame = Katna's own window frame

## Help > Send feedback

send-feedback-title = Send feedback
send-feedback-about = About
send-feedback-problem = Problem
send-feedback-idea = Idea
send-feedback-other = Something else
send-feedback-message = Your message
send-feedback-message-placeholder = What happened, or what would you like?
send-feedback-reply = Email for a reply (optional)
send-feedback-reply-placeholder = you@example.org
send-feedback-system = Include Katna's version and your system
send-feedback-what-is-sent = What is sent
send-feedback-show = Show
send-feedback-hide = Hide
# Under the text that is sent. Sentry is the name of the service that receives it.
send-feedback-where = Sent to Katna's feedback inbox at Sentry (EU). No IP address, accounts, messages or install ID.
send-feedback-cancel = Cancel
send-feedback-send = Send
send-feedback-sending = Sending…
send-feedback-sent = Feedback sent. Thank you
# $error: why it could not be sent, in English.
send-feedback-failed = Could not send feedback: { $error }
