# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Додати поштовий обліковий запис
add-account-looking = Пошук поштових серверів для { $address }…
add-account-address-intro = Введіть свою адресу електронної пошти. Katna знайде сервери за вас.
add-account-servers-title = Налаштування сервера
add-account-servers-intro = Де Katna читає й надсилає пошту для { $address }.
add-account-password-title = Введіть пароль
add-account-signing-in = Вхід…

## Add a mail account: fields

add-account-field-address = Адреса електронної пошти
add-account-incoming = Вхідна пошта ({ $protocol })
add-account-outgoing = Вихідна пошта ({ $protocol })
add-account-field-server = Сервер
add-account-field-port = Порт
add-account-security-none = Немає
add-account-field-username = Ім’я користувача
add-account-field-password = Пароль
add-account-show-password = Показати пароль
add-account-app-password-hint = Тут { $provider } потребує пароля застосунку, а не того, яким ви користуєтеся в браузері. Створіть його в налаштуваннях безпеки свого облікового запису { $provider }.
add-account-field-name = Ваше ім’я (необов’язково)
add-account-name-hint = Показується людям, яким ви пишете.
add-account-servers-pair = { $imap } і { $smtp }
add-account-servers-found = { $source ->
    [built-in] Сервери: { $servers }, знайдено в списку постачальників Katna.
    [provider] Сервери: { $servers }, знайдено в налаштуваннях вашого постачальника.
    [ispdb] Сервери: { $servers }, знайдено в списку постачальників Thunderbird.
    [dns] Сервери: { $servers }, знайдено в DNS-записах вашого домену.
   *[other] Сервери: { $servers }, вгадано; перевірте їх, якщо вхід не вдасться.
}
add-account-servers-entered = Сервери: { $servers }, як введено.

## Add a mail account: buttons

add-account-servers-button = Налаштування сервера
add-account-back = Назад
add-account-add = Додати обліковий запис
add-account-next = Далі
add-account-cancel = Скасувати

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Введіть сервер вхідної пошти.
   *[outgoing] Введіть сервер вихідної пошти.
}
add-account-server-space = { $kind ->
    [incoming] У назві сервера вхідної пошти є пробіл.
   *[outgoing] У назві сервера вихідної пошти є пробіл.
}
add-account-port-invalid = { $kind ->
    [incoming] Порт вхідної пошти має бути числом від { $min } до { $max }.
   *[outgoing] Порт вихідної пошти має бути числом від { $min } до { $max }.
}
add-account-address-empty = Введіть адресу електронної пошти.
add-account-address-invalid = Введіть адресу електронної пошти, наприклад { $example }.
add-account-not-found = Katna не вдалося знайти сервери для { $address }, тож вона заповнила звичні назви. Уточніть їх у свого постачальника.
add-account-password-empty = Введіть пароль.
add-account-name-is-password = Ім’я збігається з паролем. Натомість введіть там своє ім’я так, як його мають бачити люди.
add-account-added = { $address } додано. Отримання пошти…
add-account-app-password-refused = { $provider } відхилив пароль. Потрібен пароль застосунку, а не той, яким ви користуєтеся в браузері.
add-account-password-refused = Сервер відхилив пароль. Перевірте його й спробуйте ще раз.

## The account menu (from the account button on the top bar)

add-account-menu-another = Додати ще обліковий запис
add-account-menu-manage = Керувати обліковими записами
