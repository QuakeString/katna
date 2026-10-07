# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Buka _Peti Masuk
tray-new-message = Mesej _Baharu
tray-new-task = T_ugasan Baharu
tray-new-note = _Nota Baharu
tray-preferences = _Tetapan
tray-quit = _Keluar

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Tiada mel belum dibaca
   *[other] { $count } mesej belum dibaca
}
tray-password-refused = Kata laluan baharu diperlukan untuk { $address }
tray-signed-out = Log masuk semula ke { $address }
tray-accounts-need-you = { $count } akaun memerlukan perhatian anda
tray-not-sent = { $count ->
   *[other] { $count } mesej tidak dihantar
}
