# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _Posteingang öffnen
tray-new-message = _Neue Nachricht
tray-new-task = Neue _Aufgabe
tray-new-note = Neue No_tiz
tray-preferences = _Einstellungen
tray-quit = _Beenden

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Keine ungelesenen E-Mails
    [one] { $count } ungelesene Nachricht
   *[other] { $count } ungelesene Nachrichten
}

tray-password-refused = Neues Passwort für { $address } nötig
tray-signed-out = Erneut bei { $address } anmelden
tray-accounts-need-you = { $count } Konten brauchen Sie
tray-not-sent = { $count ->
    [one] { $count } Nachricht wurde nicht gesendet
   *[other] { $count } Nachrichten wurden nicht gesendet
}
