# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Taking an account offline and back

offline-go-offline = Passa offline
offline-for-hour = Per 1 ora
offline-until-tomorrow = Fino a domani
offline-until-online = Finché non lo riattivo
offline-go-online = Torna online
offline-work-offline = Lavora offline

## How an offline account shows

offline-state = Offline
offline-until = Offline fino alle { $time }
offline-until-day = Offline fino a { $day } { $time }
offline-waiting = { $state } · { $count ->
    [one] 1 in attesa
    [many] { $count } di elementi in attesa
   *[other] { $count } in attesa
}
offline-click-online = Clic per tornare online
offline-account-tip = { $account } è offline
offline-tag = Offline
offline-compose = { $account } è offline. Questo messaggio attende in Posta in uscita e parte quando l’account torna online.

## Settings > Accounts

offline-settings-row = Connesso
offline-settings-detail = Disattiva un account per impedire a Katna di connettersi. La sua posta resta qui da leggere, e ciò che fai nel frattempo parte quando lo riattivi.
