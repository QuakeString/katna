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
