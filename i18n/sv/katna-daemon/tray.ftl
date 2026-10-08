# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _Öppna Inkorgen
tray-new-message = _Nytt meddelande
tray-new-task = Ny _uppgift
tray-new-note = Ny _anteckning
tray-preferences = _Inställningar
tray-quit = A_vsluta

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Inga olästa mejl
    [one] { $count } oläst meddelande
   *[other] { $count } olästa meddelanden
}

tray-password-refused = Nytt lösenord behövs för { $address }
tray-signed-out = Logga in igen på { $address }
tray-accounts-need-you = { $count } konton behöver dig
tray-not-sent = { $count ->
    [one] { $count } meddelande skickades inte
   *[other] { $count } meddelanden skickades inte
}
