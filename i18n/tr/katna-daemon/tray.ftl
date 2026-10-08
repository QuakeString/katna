# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _Gelen Kutusunu Aç
tray-new-message = _Yeni İleti
tray-new-task = Yeni _görev
tray-new-note = Yeni _not
tray-preferences = _Ayarlar
tray-quit = _Çık

## The tray icon's tooltip

tray-unread = { $count ->
    [0] Okunmamış e-posta yok
   *[other] { $count } okunmamış ileti
}
tray-password-refused = { $address } için yeni parola gerekiyor
tray-signed-out = { $address } hesabında yeniden oturum açın
tray-accounts-need-you = { $count } hesap sizi bekliyor
tray-not-sent = { $count ->
   *[other] { $count } ileti gönderilmedi
}
