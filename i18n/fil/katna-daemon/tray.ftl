# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Buksan ang _Inbox
tray-new-message = _Bagong Mensahe
tray-new-task = Bagong _gawain
tray-new-note = Bagong _tala
tray-preferences = Mga _Setting
tray-quit = _Umalis

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Walang hindi pa nababasang mail
   *[other] { $count } hindi pa nababasang mensahe
}
tray-password-refused = Kailangan ng bagong password para sa { $address }
tray-signed-out = Mag-sign in muli sa { $address }
tray-accounts-need-you = { $count } account ang nangangailangan sa iyo
tray-not-sent = { $count ->
    [one] { $count } mensahe ang hindi naipadala
   *[other] { $count } mensahe ang hindi naipadala
}
