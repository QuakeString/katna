# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _Открыть «Входящие»
tray-new-message = _Новое письмо
tray-new-task = Новая _задача
tray-new-note = Новая за_метка
tray-preferences = На_стройка
tray-quit = _Выход

## The tray icon's tooltip

tray-unread = { $count ->
    [0] Нет непрочитанных писем
    [one] { $count } непрочитанное письмо
    [few] { $count } непрочитанных письма
    [many] { $count } непрочитанных писем
   *[other] { $count } непрочитанного письма
}
tray-password-refused = Нужен новый пароль для { $address }
tray-signed-out = Войдите снова в { $address }
tray-accounts-need-you = Аккаунтов, требующих внимания: { $count }
tray-not-sent = { $count ->
    [one] { $count } письмо не отправлено
    [few] { $count } письма не отправлены
    [many] { $count } писем не отправлено
   *[other] { $count } письма не отправлено
}
