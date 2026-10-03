# Katna Mail, English: mail rules (Settings > Folders & rules, the rule
# editor, and Make a rule… in the mail list's right-click menu).
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Settings > Folders & rules

# The Rules row's name.
settings-rules = Rules
# Under it in search results.
settings-rules-summary = Sort, label, forward or quiet new mail by itself
# The faint line over the list of rules.
settings-rules-intro = Rules sort new mail by themselves, in this order. Drag to reorder.
# The account filter over the list: every account's rules.
settings-rules-all-accounts = All accounts
# The button that opens the editor on a new rule.
settings-rules-new = New rule
settings-rules-none = No rules yet. A rule sorts new mail by itself: by sender, subject or words.
# With one account picked in the filter.
settings-rules-none-account = No rules for this account yet.
# Tooltips of a rule's row.
settings-rules-drag = Drag to reorder
settings-rules-edit = Edit rule
settings-rules-turn-off = Turn this rule off
settings-rules-turn-on = Turn this rule on

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Starter rules
settings-rules-starters-intro = Off until you turn one on. They work for all your accounts; edit one to change it.
# Snackbar while a starter rule's folders are made.
settings-rules-starter-turning-on = Turning on “{ $name }”…
# $name: the starter rule's name; $error: what went wrong.
settings-rules-starter-failed = Couldn't turn on “{ $name }”: { $error }
# The starter rules' names.
rules-starter-promotions = Quiet promotions
rules-starter-newsletters = Newsletters to Reading
rules-starter-receipts = Receipts and invoices
rules-starter-deliveries = Deliveries
rules-starter-train = Train tickets
rules-starter-flight = Flight tickets
rules-starter-codes = One-time codes
rules-starter-security = Security alerts
rules-starter-social = Social mail
rules-starter-invites = Calendar invites
# The folders (labels, on Gmail) starter rules make when turned on.
rules-starter-folder-reading = Reading
rules-starter-folder-receipts = Receipts
rules-starter-folder-deliveries = Deliveries
rules-starter-folder-travel = Travel
rules-starter-folder-social = Social
# The small tag on a rule's row: where it runs.
rules-runs-katna = Runs in Katna
rules-runs-gmail = Runs on Gmail
rules-runs-sieve = Runs on the server
# The tag of a rule switched off because it failed.
rules-stopped = Stopped
# Why a rule was switched off, in red under its summary.
rules-error-folder-gone = The folder this rule uses no longer exists. Edit the rule to pick another.
rules-error-no-archive = This account has no archive folder. Edit the rule to do something else.
rules-error-no-trash = This account has no Trash folder. Edit the rule to do something else.
rules-error-cannot-send = This account can't send mail, so the rule can't forward it.
# Any other reason. $error: the service's own words.
rules-error-other = { $error }. Edit the rule and turn it back on.

# The Folders row's name.
settings-folders = Folders
settings-folders-summary = Unread counts in the folder pane
settings-folders-unread-counts = Unread count on every folder
settings-folders-unread-counts-detail = Off: only Inbox shows how many are unread

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

# $when: the conditions; $then: the actions.
rules-summary = { $when } → { $then }
# Joining the conditions of a rule that needs all of them.
rules-summary-and = { $first } and { $next }
# Joining the conditions of a rule that needs only one.
rules-summary-or = { $first } or { $next }
# The last of a long list of words: "ticket, pnr, booking or 6 more".
rules-summary-more = { $count } more
# Joining the actions.
rules-summary-list = { $first }, { $next }
# $field: "From", "Subject"…; $comparator: "contains"…; $value: what the
# user typed.
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Has an attachment
rules-summary-no-attachment = Has no attachment
rules-summary-mailing-list = From a mailing list
rules-summary-not-mailing-list = Not from a mailing list
# $tab: an inbox tab's name, such as Promotions.
rules-summary-tab = In the { $tab } tab
rules-summary-not-tab = Not in the { $tab } tab
# $folder: a folder's name.
rules-summary-move = move to { $folder }
rules-summary-archive = skip the inbox
rules-summary-trash = move to the trash
rules-summary-mark-read = mark read
rules-summary-star = star
rules-summary-important = mark important
# $label: a Gmail label's name.
rules-summary-label = label { $label }
# $address: an email address.
rules-summary-forward = forward to { $address }
rules-summary-dont-notify = don't notify
rules-summary-read-after = { $count ->
    [one] mark read after { $count } day
   *[other] mark read after { $count } days
}
# In place of a folder that was deleted.
rules-summary-folder-gone = a folder that's gone

## The rule editor

rules-editor-new-title = New rule
rules-editor-edit-title = Edit rule
rules-editor-name-hint = Rule name
# "When a new mail matches [all ▾] of these:"
rules-editor-when = When a new mail matches
rules-editor-of-these = of these:
rules-mode-all = all
rules-mode-any = any
# What a condition looks at.
rules-field-from = From
rules-field-to = To
rules-field-cc = Cc
rules-field-any-recipient = To or Cc
rules-field-reply-to = Reply-to
rules-field-subject = Subject
rules-field-body = Text
rules-field-attachment-name = Attachment name
rules-field-has-attachment = Has attachment
rules-field-mailing-list = From a mailing list
rules-field-tab = Inbox tab
# How a condition compares.
rules-comparator-contains = contains
rules-comparator-not-contains = doesn't contain
rules-comparator-begins-with = begins with
rules-comparator-ends-with = ends with
rules-comparator-equals = is exactly
rules-comparator-matches = matches the pattern
# The choice of a "Has attachment" condition.
rules-has-yes = yes
rules-has-no = no
rules-editor-value-hint = Words or an address
rules-editor-add-condition = Add a condition
rules-editor-remove = Remove
rules-editor-then = Then:
# What a rule can do.
rules-action-move = Move to
rules-action-archive = Skip the inbox (archive)
rules-action-trash = Move to the trash
rules-action-mark-read = Mark read
rules-action-star = Star
rules-action-important = Mark important
rules-action-label = Add label
rules-action-forward = Forward to
rules-action-dont-notify = Don't notify
rules-action-read-after = Mark read after
rules-editor-choose-folder = Choose a folder
rules-editor-choose-label = Choose a label
# In the folder list of a rule for several accounts. $account: an
# address; $folder: a folder's name.
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Email address
# After the number of "Mark read after".
rules-editor-days = days
rules-editor-add-action = Add an action
rules-editor-stop = Stop here: later rules don't run on this mail
rules-editor-accounts = Accounts:
rules-editor-accounts-none = Choose accounts
rules-editor-accounts-many = { $count ->
    [one] { $count } account
   *[other] { $count } accounts
}
# The light panel under the rule. $mails: rules-editor-mails, shown in
# bold; $days: a number.
rules-editor-matches = Matches { $mails } from the last { $days } days
rules-editor-mails = { $count ->
    [one] { $count } mail
   *[other] { $count } mails
}
rules-editor-counting = Counting the mail it matches…
# Searches the mail for what the rule matches.
rules-editor-show = Show them
rules-editor-also-apply = Also apply to these { $count }
rules-editor-runs-katna = Runs in Katna, while this computer is on.
rules-editor-runs-gmail = Runs on Gmail, so it also works on your phone and with this computer off.
rules-editor-runs-sieve = Runs on your mail server, so it also works on your phone and with this computer off.
# Why a rule runs in Katna although Gmail or the mail server runs rules:
# under the editor's preview, and on the tooltip of the rule's tag.
# $action: what the rule does, as its summary says it ("don't notify").
rules-note-gmail-action = Runs in Katna: Gmail filters can't do “{ $action }”.
rules-note-sieve-action = Runs in Katna: your mail server's rules can't do “{ $action }”.
# $field and $comparator: as the editor shows them ("From", "begins with").
rules-note-test = { $field } { $comparator }
# $test: a condition, as rules-note-test says it, or "Has attachment".
rules-note-gmail-condition = Runs in Katna: Gmail filters can't test “{ $test }” as Katna does.
rules-note-sieve-condition = Runs in Katna: your mail server's rules can't test “{ $test }” as Katna does.
rules-note-order = Runs in Katna, as an earlier rule of the account does: rules run in list order.
rules-note-gmail-stop = Runs in Katna: Gmail filters can't keep later rules from running.
# $address: where the rule forwards to.
rules-note-gmail-forward = Runs in Katna: Gmail forwards only to addresses verified in its settings, and { $address } isn't one.
rules-note-gmail-folder = Runs in Katna: Gmail has no label for a folder this rule uses.
rules-note-sieve-folder = Runs in Katna: your mail server has no folder this rule uses.
rules-note-gmail-sign-in = Runs in Katna until you sign in to Google again and let Katna make Gmail filters.
# $name: the name of the other script.
rules-note-sieve-other-script = Runs in Katna: another rule script (“{ $name }”) is active on your mail server.
# $error: the service's own words.
rules-note-gmail-failed = Runs in Katna: Gmail didn't take it ({ $error }).
rules-note-sieve-failed = Runs in Katna: your mail server didn't take it ({ $error }).
rules-editor-cancel = Cancel
rules-editor-save = Save
rules-editor-saving = Saving…
rules-editor-delete = Delete rule
rules-editor-delete-ask = Delete this rule?
rules-editor-delete-keep = Keep it
rules-editor-delete-confirm = Delete
# Shown in the editor before saving.
rules-editor-needs-folder = Choose a folder for each “Move to” and a label for each “Add label”.
rules-editor-needs-days = “Mark read after” takes a number of days, from 1 to 3650.
# Snackbars.
rules-saved = Rule saved
rules-saved-applied = { $count ->
    [one] Rule saved and applied to { $count } mail
   *[other] Rule saved and applied to { $count } mails
}
# $error: what went wrong.
rules-apply-failed = Rule saved, but applying it failed: { $error }
rules-deleted = Rule deleted
rules-delete-failed = Couldn't delete the rule: { $error }
rules-change-failed = Couldn't change the rules: { $error }
