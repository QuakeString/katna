# Katna Mail, English: adding a mail account.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Add a mail account: titles and steps

add-account-title = Add a mail account
# Under the title, above the tiles of mail providers.
add-account-providers-intro = Pick your mail provider. Katna finds the rest.
# The last tile: any provider without a tile of its own.
add-account-provider-other = Other mail
add-account-provider-other-detail = Any IMAP or POP3 account
# Under Google on its tile: Google's mail services.
add-account-provider-google-detail = Gmail and Google Workspace
# Under Microsoft on its tile: Microsoft's mail services.
add-account-provider-microsoft-detail = Outlook and Microsoft 365
# Under a provider's name on its tile. $provider: its name, such as Yahoo.
add-account-provider-mail = { $provider } Mail
# The title of the step asking for the address and password. $provider:
# the name of the provider's mail, such as Yahoo Mail.
add-account-form-title = Sign in to { $provider }
# The same title after Other mail.
add-account-form-title-other = Your mail account
add-account-form-intro = Katna keeps your password in your system's keyring.
# $address: the email address being added.
add-account-looking = Looking for the mail servers of { $address }…
add-account-address-intro = Enter your email address. Katna finds the servers for you.
add-account-servers-title = Server settings
# $address: the email address being added.
add-account-servers-intro = Where Katna reads and sends mail for { $address }.
add-account-signing-in = Signing in…
# The step while the provider's sign-in page is open in the web browser.
add-account-browser-title = Continue in your browser
# $provider: Google or Microsoft.
add-account-browser-intro = Katna opened the { $provider } sign-in page in your browser. Sign in there and allow Katna to read and send your mail, then come back here.
add-account-browser-hint = No page opened? Check your browser's windows, or go back and try again.
# A stage of adding, with a turning arrow while it lasts: the provider's
# page is open in the browser.
add-account-stage-browser = Waiting for you to sign in in your browser…
# A stage of adding. $server: the incoming server's name, such as
# imap.example.org.
add-account-stage-signing-in-at = Signing in at { $server }…
# The link after add-account-app-password-hint: the provider's own page on
# app passwords.
add-account-help-app-password-link = How to make an app password
# $provider: the mail provider's name, such as GMX.
add-account-help-turn-on-imap = { $provider } lets mail apps in only once IMAP and POP3 access is turned on in the settings of its web mail.
# The link after add-account-help-turn-on-imap: the provider's own page.
add-account-help-turn-on-imap-link = How to turn it on

## Add a mail account: fields

add-account-field-address = Email address
# A section heading on the server step, above the IMAP and POP3 choice.
add-account-receive-with = Receive mail with
# Under the IMAP and POP3 choice while IMAP is picked.
add-account-imap-about = IMAP keeps your mail and folders on the server, the same on every device. Pick it when you can.
# Under the IMAP and POP3 choice while POP3 is picked.
add-account-pop3-about = POP3 downloads your mail to this computer. Mail you read or move here stays as it is on the server and your other devices.
# A section heading. $protocol: IMAP, the protocol for reading mail.
add-account-incoming = Incoming mail ({ $protocol })
# A section heading. $protocol: SMTP, the protocol for sending mail.
add-account-outgoing = Outgoing mail ({ $protocol })
# The server's host name, such as imap.example.org.
add-account-field-server = Server
# The server's network port number, such as 993.
add-account-field-port = Port
# The connection's encryption, in the list beside SSL/TLS and STARTTLS
# (which stay as they are): no encryption.
add-account-security-none = None
# Shown under a server's settings while None is picked.
add-account-security-none-warning = Not encrypted: your password and mail can be read on the way.
# The name to sign in to the server with.
add-account-field-username = Username
add-account-field-password = Password
add-account-show-password = Show password
# $provider: the mail provider's name, such as Gmail. An app password is a
# separate password a provider makes for mail apps.
add-account-app-password-hint = { $provider } needs an app password here, not the one you use on the web. Make one in the security settings of your { $provider } account.
add-account-field-name = Your name (optional)
add-account-name-hint = Shown to the people you write to.
# $imap and $smtp: the incoming and outgoing server names, when they differ.
add-account-servers-pair = { $imap } and { $smtp }
# Where the server settings came from. $servers: the server names;
# $source: where they were found. Thunderbird is another mail app; DNS
# records are a domain's public settings.
add-account-servers-found = { $source ->
    [built-in] Servers: { $servers }, found in Katna's list of providers.
    [provider] Servers: { $servers }, found in your provider's settings.
    [ispdb] Servers: { $servers }, found in Thunderbird's list of providers.
    [dns] Servers: { $servers }, found in your domain's DNS records.
   *[other] Servers: { $servers }, found in a guess; check them if signing in fails.
}
# $servers: the server names, typed by hand.
add-account-servers-entered = Servers: { $servers }, as entered.

## Add a mail account: buttons

# Signs in on the provider's own page in the browser. $provider: Google or
# Microsoft.
add-account-sign-in-with = Sign in with { $provider }
# On the password step of a Google address, instead of an app password.
# $provider: Google.
add-account-sign-in-instead = Sign in with { $provider } instead

# Opens the server settings step.
add-account-servers-button = Server settings
add-account-back = Back
# Adds the account, on the last step.
add-account-add = Add account
# On the last step, closes the dialog.
add-account-done = Done
# On the last step, starts again at the providers.
add-account-another = Add another account
add-account-cancel = Cancel

## Add a mail account: problems

# $kind: which server, incoming (reading mail) or outgoing (sending mail).
add-account-server-missing = { $kind ->
    [incoming] Enter the incoming server.
   *[outgoing] Enter the outgoing server.
}
# $kind: which server, incoming (reading mail) or outgoing (sending mail).
add-account-server-space = { $kind ->
    [incoming] The incoming server name has a space in it.
   *[outgoing] The outgoing server name has a space in it.
}
# $kind: which server, incoming or outgoing. $min and $max: the lowest and
# highest port numbers (1 and 65535).
add-account-port-invalid = { $kind ->
    [incoming] The incoming port must be a number from { $min } to { $max }.
   *[outgoing] The outgoing port must be a number from { $min } to { $max }.
}
add-account-address-empty = Enter an email address.
# $example: a made-up email address.
add-account-address-invalid = Enter an email address such as { $example }.
# $address: the email address being added.
add-account-not-found = Katna could not find the servers for { $address }, so it filled in the usual names. Check them with your provider.
add-account-password-empty = Enter the password.
# Shown when the name field holds the same text as the password field.
add-account-name-is-password = The name is the same as the password. Type your name there instead, as people should see it.
# $provider: the mail provider's name, such as Gmail. An app password is a
# separate password a provider makes for mail apps.
add-account-app-password-refused = { $provider } refused the password. It needs an app password, not the one you use on the web.
add-account-password-refused = The server refused the password. Check it and try again.
# $provider: Google or Microsoft. The user closed the page or did not allow
# access.
add-account-sign-in-refused = { $provider } did not let Katna in. Try again, and allow access to your mail.
# The address belongs to a provider that only allows signing in on its own
# page, which this copy of Katna cannot do yet. $provider: Microsoft or
# Google, or empty when unknown.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] This copy of Katna cannot sign in to Microsoft accounts yet.
    [Google] This copy of Katna cannot sign in to Google accounts yet.
   *[other] This provider only allows signing in on its own page, which Katna cannot do for it yet.
}
# Shown on the server step when the provider names no outgoing server.
add-account-smtp-not-found = Katna found where to read your mail but not where to send it. Enter the outgoing server.

## Add a mail account: the last step

add-account-done-title = Your account is ready
add-account-done-intro = Katna is getting your mail now. New mail shows as it arrives.
# Labels of the facts about the account, beside their values.
add-account-done-sign-in = Sign-in
# $provider: Google or Microsoft.
add-account-done-signed-in-with = With { $provider }, in your browser
add-account-done-receiving = Receiving mail
add-account-done-sending = Sending mail
add-account-done-on-server = Mail on the server
add-account-done-kept = Kept until you delete it in Katna
add-account-done-pop3-hint = Change what happens to mail on the server in Settings > Accounts.
# A panel on the last step for a Zoho Mail account.
add-account-done-zoho-title = Tasks and calendars
add-account-done-zoho-about = Zoho keeps these apart from mail. Sign in with Zoho once to bring them into Katna.
# Shown once the Zoho sign-in succeeded.
add-account-done-linked = Tasks and calendars connected

## The account menu (from the account button on the top bar)

add-account-menu-another = Add another account
# Opens Settings > Accounts.
# The ☰ button beside it: opens the application menu (File, Edit, View…),
# for desktops without a global menu.
app-menu = Main menu
app-menu-back = Back
