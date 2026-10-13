# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Fungua _Kikasha
tray-new-message = _Ujumbe Mpya
tray-new-task = _Jukumu Jipya
tray-new-note = _Dokezo Jipya
tray-preferences = _Mipangilio
tray-quit = _Ondoka

## The tray icon's tooltip

tray-unread = { $count ->
    [0] Hakuna barua ambayo haijasomwa
    [one] Ujumbe { $count } ambao haujasomwa
   *[other] Jumbe { $count } ambazo hazijasomwa
}
tray-password-refused = Nenosiri jipya linahitajika kwa { $address }
tray-signed-out = Ingia tena kwenye { $address }
tray-accounts-need-you = Akaunti { $count } zinakuhitaji
tray-not-sent = { $count ->
    [one] Ujumbe { $count } haukutumwa
   *[other] Jumbe { $count } hazikutumwa
}
