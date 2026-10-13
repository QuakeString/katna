# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Buka Kotak _Masuk
tray-new-message = Pesan _Baru
tray-new-task = _Tugas baru
tray-new-note = _Catatan baru
tray-preferences = _Setelan
tray-quit = _Keluar

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Tidak ada email yang belum dibaca
   *[other] { $count } pesan belum dibaca
}
tray-password-refused = Sandi baru diperlukan untuk { $address }
tray-signed-out = Masuk lagi ke { $address }
tray-accounts-need-you = { $count } akun memerlukan tindakan Anda
tray-not-sent = { $count ->
   *[other] { $count } pesan tidak terkirim
}
