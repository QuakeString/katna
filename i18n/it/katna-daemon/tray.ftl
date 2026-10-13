# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Apri Posta in _arrivo
tray-new-message = _Nuovo messaggio
tray-new-task = Nuova _attività
tray-new-note = Nuova n_ota
tray-preferences = Imp_ostazioni
tray-quit = _Esci

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Nessun messaggio da leggere
    [one] { $count } messaggio da leggere
    [many] { $count } di messaggi da leggere
   *[other] { $count } messaggi da leggere
}

tray-password-refused = Serve una nuova password per { $address }
tray-signed-out = Accedi di nuovo a { $address }
tray-accounts-need-you = { $count } account richiedono la tua attenzione
tray-not-sent = { $count ->
    [one] { $count } messaggio non è stato inviato
    [many] { $count } di messaggi non sono stati inviati
   *[other] { $count } messaggi non sono stati inviati
}
