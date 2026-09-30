# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Добавить почтовый аккаунт
add-account-looking = Поиск почтовых серверов для { $address }…
add-account-address-intro = Введите адрес электронной почты. Katna сама найдёт серверы.
add-account-servers-title = Настройки сервера
add-account-servers-intro = Где Katna получает и отправляет почту для { $address }.
add-account-signing-in = Вход…
add-account-browser-title = Продолжите в браузере
add-account-browser-intro = В браузере открыта страница входа { $provider }. Войдите там и разрешите Katna читать и отправлять вашу почту, затем вернитесь сюда.
add-account-browser-hint = Страница не открылась? Проверьте окна браузера или вернитесь и попробуйте снова.

## Add a mail account: fields

add-account-field-address = Адрес электронной почты
add-account-incoming = Входящая почта ({ $protocol })
add-account-outgoing = Исходящая почта ({ $protocol })
add-account-field-server = Сервер
add-account-field-port = Порт
add-account-security-none = Нет
add-account-security-none-warning = Без шифрования: ваш пароль и письма могут быть прочитаны по пути.
add-account-field-username = Имя пользователя
add-account-field-password = Пароль
add-account-show-password = Показать пароль
add-account-app-password-hint = Здесь { $provider } требует пароль приложения, а не тот, что вы используете на сайте. Создайте его в настройках безопасности аккаунта { $provider }.
add-account-field-name = Ваше имя (необязательно)
add-account-name-hint = Его увидят те, кому вы пишете.
add-account-servers-pair = { $imap } и { $smtp }
add-account-servers-found = { $source ->
    [built-in] Серверы: { $servers }, найдены в списке провайдеров Katna.
    [provider] Серверы: { $servers }, найдены в настройках вашего провайдера.
    [ispdb] Серверы: { $servers }, найдены в списке провайдеров Thunderbird.
    [dns] Серверы: { $servers }, найдены в DNS-записях вашего домена.
   *[other] Серверы: { $servers }, угаданы; проверьте их, если войти не удастся.
}
add-account-servers-entered = Серверы: { $servers }, введены вручную.
add-account-sign-in-with = Войти через { $provider }
add-account-sign-in-instead = Вместо этого войти через { $provider }

## Add a mail account: buttons

add-account-servers-button = Настройки сервера
add-account-back = Назад
add-account-add = Добавить аккаунт
add-account-cancel = Отмена

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Введите сервер входящей почты.
   *[outgoing] Введите сервер исходящей почты.
}
add-account-server-space = { $kind ->
    [incoming] В имени сервера входящей почты есть пробел.
   *[outgoing] В имени сервера исходящей почты есть пробел.
}
add-account-port-invalid = { $kind ->
    [incoming] Порт входящей почты должен быть числом от { $min } до { $max }.
   *[outgoing] Порт исходящей почты должен быть числом от { $min } до { $max }.
}
add-account-address-empty = Введите адрес электронной почты.
add-account-address-invalid = Введите адрес электронной почты, например { $example }.
add-account-not-found = Katna не удалось найти серверы для { $address }, поэтому подставлены обычные имена. Уточните их у своего провайдера.
add-account-password-empty = Введите пароль.
add-account-name-is-password = Имя совпадает с паролем. Введите там своё имя — так, как его должны видеть другие.
add-account-app-password-refused = { $provider } не принял пароль. Нужен пароль приложения, а не тот, что вы используете на сайте.
add-account-password-refused = Сервер не принял пароль. Проверьте его и попробуйте снова.
add-account-sign-in-refused = { $provider } не впустил Katna. Попробуйте снова и разрешите доступ к почте.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Эта копия Katna пока не умеет входить в аккаунты Microsoft.
    [Google] Эта копия Katna пока не умеет входить в аккаунты Google.
   *[other] Этот провайдер разрешает вход только на своей странице, а для него Katna этого пока не умеет.
}

## The account menu (from the account button on the top bar)

add-account-menu-another = Добавить ещё аккаунт
app-menu = Главное меню
app-menu-back = Назад
