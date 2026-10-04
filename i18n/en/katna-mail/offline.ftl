# Katna Mail, English: taking an account offline (its right-click menu in
# the folder pane, the account card, Compose's From, Settings > Accounts).
# Guide: i18n/README.md. Keep ids stable; change the text freely.

## Taking an account offline and back

# Right-click menu of an account: Katna stops connecting to it.
offline-go-offline = Go offline
# Under "Go offline": offline for an hour.
offline-for-hour = For 1 hour
# Under "Go offline": offline until 8 tomorrow morning.
offline-until-tomorrow = Until tomorrow
# Settings chip: offline until switched back on.
offline-until-online = Until I turn it on
# Brings an offline account back; what waited goes out.
offline-go-online = Go online
# The account card's switch: every account offline at once.
offline-work-offline = Work offline

## How an offline account shows

# Its line in the account card and Settings.
offline-state = Offline
# Offline until a time today; { $time } is like "9:00 AM".
offline-until = Offline until { $time }
# Offline until a time on another day; { $day } is like "Mon".
offline-until-day = Offline until { $day } { $time }
# { $state } is one of the lines above; { $count } changes and messages
# wait to go out once it is back.
offline-waiting = { $state } · { $count ->
    [one] 1 waiting
   *[other] { $count } waiting
}
# Tooltip of the crossed cloud, under the line above.
offline-click-online = Click to go online
# Tooltip of the account picture, one line per offline account.
offline-account-tip = { $account } is offline
# Beside an offline account in Compose's From.
offline-tag = Offline
# Compose, when it goes out from an offline account.
offline-compose = { $account } is offline. This mail waits in the Outbox and goes out when the account is back online.

## Settings > Accounts

offline-settings-row = Connected
offline-settings-detail = Turn an account off to stop Katna connecting to it. Its mail stays here to read, and what you do meanwhile goes out when you turn it back on.
