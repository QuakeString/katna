# Katna Mail, English: adding a mail account.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Add a mail account: titles and steps

add-account-title = Add a mail account
# $address: the email address being added.
add-account-looking = Looking for the mail servers of { $address }…
add-account-address-intro = Enter your email address. Katna finds the servers for you.
add-account-servers-title = Server settings
# $address: the email address being added.
add-account-servers-intro = Where Katna reads and sends mail for { $address }.
add-account-password-title = Enter your password
add-account-signing-in = Signing in…

## Add a mail account: fields

add-account-field-address = Email address
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

# Opens the server settings step.
add-account-servers-button = Server settings
add-account-back = Back
# Adds the account, on the last step.
add-account-add = Add account
add-account-next = Next
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
# Shown briefly after the account is added. $address: its email address.
add-account-added = Added { $address }. Getting your mail…
# $provider: the mail provider's name, such as Gmail. An app password is a
# separate password a provider makes for mail apps.
add-account-app-password-refused = { $provider } refused the password. It needs an app password, not the one you use on the web.
add-account-password-refused = The server refused the password. Check it and try again.

## The account menu (from the account button on the top bar)

add-account-menu-another = Add another account
# Opens Settings > Accounts.
add-account-menu-manage = Manage accounts
