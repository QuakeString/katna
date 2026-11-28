# Katna Mail, English: Settings > Subscription (the Katna account).
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# A Katna account is an account on Katna's own server (Katna Server),
# needed for its features. It is not a mail account.

## Settings > Subscription: the Katna account

# At the top of the page.
katna-intro = A Katna account turns on Katna's online features: read receipts, link tracking, Activity and automatic translation. Its password is its own, not a mail password, and your mail logins never leave this computer.
# While the page asks the background service.
katna-checking = Checking…
katna-email = Email
katna-password = Password
katna-password-detail = Changing it signs your other computers out.
katna-sign-in = Sign in
katna-sign-in-detail = Sign in on each computer where you want the online features.
katna-create = Create account
katna-create-detail = Use an address you can read: we mail you a code to confirm it.
# First start: the form's heading after "Create account" or "I have an account".
katna-onboarding-create-title = Create your Katna account
katna-onboarding-sign-in-title = Sign in to Katna
# Link from "Create account" back to signing in.
katna-have-account = I have an account
katna-forgot = Forgot password?
katna-forgot-detail = We mail you a code to choose a new password.
katna-forgot-code-detail = Type the code we mailed you and a new password. Your other computers are signed out.
katna-send-code = Send code
katna-back-to-sign-in = Back to sign in
katna-code = Code
katna-new-password = New password
katna-current-password = Current password
katna-set-password = Set password
katna-save-password = Save password
katna-change-password = Change password
katna-cancel = Cancel
# After signing up, before the address is confirmed.
katna-confirm-title = Confirm your address
# $email: the Katna account's address.
katna-confirm-detail = We mailed a 6-digit code to { $email }. It works for 30 minutes.
katna-confirm = Confirm
katna-resend = Send a new code
katna-code-resent = A new code is on its way.
katna-reset-code-sent = If that address has a Katna account, a code is on its way.
katna-password-changed = Password changed. Your other computers are signed out.
katna-signed-in = Signed in
katna-signed-in-detail = The online features work on this computer.
katna-sign-out = Sign out
# Settings > Subscription, beside Computers: signs out every computer but
# this one.
katna-sign-out-others = Sign out the others
katna-devices = Computers
katna-devices-detail = The computers signed in to this account. Sign out any you don't use.
# A computer that sent no name.
katna-device-unnamed = Unnamed computer
# $date: when it signed in.
katna-device-this = This computer · signed in { $date }
katna-device-since = Signed in { $date }
# $date: when another computer last reached Katna's server.
katna-device-last-seen = Last seen { $date }
# Beside this computer's name in the list of computers.
katna-device-this-tag = This computer
katna-delete = Delete account
katna-delete-detail = Deletes the account and everything the server keeps for it, such as read receipts. Mail on this computer stays.
katna-delete-confirm = Delete for good
# The short button beside "Delete account".
katna-delete-short = Delete
# The short button beside "Password".
katna-change = Change
# Settings > Subscription: the card with the password and deleting the
# account.
katna-security = Password and account

## Settings > Subscription: the plan
## Katna AI is writing help on Katna's server: free for a month from the
## first use, then paid. The Katna account itself is free.

katna-plan = Plan
katna-plan-detail = The Katna account is free. Katna AI is the only part that costs money.
katna-plan-account = Katna account
katna-plan-free = Free
# The account's price, shown large.
katna-plan-account-price = $0
katna-plan-account-always = always
katna-plan-account-tracking = Read receipts and link tracking
katna-plan-account-activity = Activity
katna-plan-account-translation = Automatic translation
katna-plan-ai = Katna AI
# Katna AI's price, shown large, then "a month" or "a month after the
# free month" beside it.
katna-plan-ai-price = $5
katna-plan-ai-per-month = a month
katna-plan-ai-after-trial = a month after the free month
# Chips beside "Katna AI".
katna-plan-ai-trial = Free month
katna-plan-ai-active = Active
katna-plan-ai-over = Free month over
katna-plan-ai-own = Your own key
katna-plan-ai-off = Off
# $days: how long the free month is.
katna-plan-ai-new = { $days } days free
katna-plan-ai-new-detail = Free for { $days } days from the first time you use it.
# $days: whole days of the free month left.
katna-plan-ai-days-left = { $days ->
    [one] { $days } day left
   *[other] { $days } days left
}
# $date: the free month's last day.
katna-plan-ai-ends = Ends { $date }
katna-plan-ai-paid = Paid for this month.
# $date: when the paid time ends.
katna-plan-ai-paid-until = Paid until { $date }
katna-plan-ai-own-detail = Writing help uses your own AI service, so Katna AI is not needed.
katna-plan-ai-off-detail = Writing help with AI is off.
katna-plan-ai-writing = Rephrase, subject ideas and reply drafts
katna-plan-ai-summaries = Conversation summaries
katna-plan-ai-no-key = No key or setup needed
katna-plan-nothing-charged = Nothing is charged in the free month.
# Buttons in the Katna AI tile; each opens Settings > AI.
katna-plan-ai-settings = AI settings
katna-plan-use-own-key = Use my own key
katna-plan-turn-on = Turn on
# A note above the plan once Katna AI's free month is over.
katna-plan-over-title = Your free month of Katna AI is over
katna-plan-over-detail = The AI buttons rest until you use your own key in Settings > AI.

## Settings > Subscription: what the account turns on

katna-features = What your account turns on
katna-features-detail = These work while this computer is signed in.
katna-feature-tracking = Read receipts and link tracking
katna-feature-tracking-detail = Turn them on for each mail as you write it.
katna-feature-activity = Activity
katna-feature-activity-detail = Who opened your tracked mail, from the top bar.
katna-feature-translation = Automatic translation
katna-feature-translation-detail = Mail in other languages, in yours.
katna-feature-ai-detail = Writing help, summaries and reply ideas.
# The state at the end of a feature's row.
katna-feature-on = On
katna-feature-off = Off
katna-feature-own-key = Your own key
# Beside an online feature while signed out.
katna-sign-in-needed = Sign in to a Katna account to use this.

## Errors

katna-error-wrong-password = Wrong email or password.
katna-error-exists = This address already has a Katna account. Sign in instead.
katna-error-bad-email = That doesn't look like an email address.
katna-error-short-password = The password needs at least 8 characters.
katna-error-long-password = The password is too long.
katna-error-wrong-code = That code is not right.
katna-error-code-expired = That code has expired. Ask for a new one.
katna-error-too-many = Too many tries. Wait a few minutes and try again.
katna-error-mail-failed = The mail with the code could not be sent. Try again later.
katna-error-signed-out = This computer was signed out. Sign in again.
katna-error-offline = Katna's server could not be reached. Check your connection and try again.
katna-error-server = Katna's server had a problem. Try again later.
