# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _Inbox openen
tray-new-message = _Nieuw bericht
tray-new-task = Nieuwe _taak
tray-new-note = Nieuwe n_otitie
tray-preferences = In_stellingen
tray-quit = A_fsluiten

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Geen ongelezen e-mail
    [one] { $count } ongelezen bericht
   *[other] { $count } ongelezen berichten
}

tray-password-refused = Nieuw wachtwoord nodig voor { $address }
tray-signed-out = Opnieuw aanmelden bij { $address }
tray-accounts-need-you = { $count } accounts hebben je nodig
tray-not-sent = { $count ->
    [one] { $count } bericht is niet verzonden
   *[other] { $count } berichten zijn niet verzonden
}
