# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Відкрити _Вхідні
tray-new-message = _Новий лист
tray-new-task = Нове _завдання
tray-new-note = Нова _нотатка
tray-preferences = П_араметри
tray-quit = Ви_йти

## The tray icon's tooltip

tray-unread = { $count ->
    [0] Немає непрочитаних листів
    [one] { $count } непрочитаний лист
    [few] { $count } непрочитані листи
    [many] { $count } непрочитаних листів
   *[other] { $count } непрочитаного листа
}
tray-password-refused = Потрібен новий пароль для { $address }
tray-signed-out = Увійдіть знову в { $address }
tray-accounts-need-you = Облікових записів, що потребують уваги: { $count }
tray-not-sent = { $count ->
    [one] { $count } лист не надіслано
    [few] { $count } листи не надіслано
    [many] { $count } листів не надіслано
   *[other] { $count } листа не надіслано
}
