# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Додати поштовий обліковий запис
add-account-providers-intro = Виберіть свого поштового провайдера. Решту Katna знайде сама.
add-account-provider-other = Інша пошта
add-account-provider-other-detail = Будь-який обліковий запис IMAP або POP3
add-account-provider-google-detail = Gmail і Google Workspace
add-account-provider-microsoft-detail = Outlook і Microsoft 365
add-account-provider-mail = Пошта { $provider }
add-account-form-title = Вхід у { $provider }
add-account-form-title-other = Ваш поштовий обліковий запис
add-account-form-intro = Katna зберігає ваш пароль у системному сховищі ключів.
add-account-looking = Пошук поштових серверів для { $address }…
add-account-address-intro = Введіть свою адресу електронної пошти. Katna знайде сервери за вас.
add-account-servers-title = Налаштування сервера
add-account-servers-intro = Де Katna читає й надсилає пошту для { $address }.
add-account-signing-in = Вхід…
add-account-browser-title = Продовжте в браузері
add-account-browser-intro = У вашому браузері відкрито сторінку входу { $provider }. Увійдіть там і дозвольте Katna читати й надсилати вашу пошту, а потім поверніться сюди.
add-account-browser-hint = Сторінка не відкрилася? Перегляньте вікна браузера або поверніться назад і спробуйте ще раз.
add-account-stage-browser = Очікуємо, поки ви ввійдете в браузері…
add-account-stage-signing-in-at = Вхід на { $server }…
add-account-help-app-password-link = Як створити пароль застосунку
add-account-help-turn-on-imap = { $provider } впускає поштові програми, лише коли в налаштуваннях вебпошти ввімкнено доступ IMAP і POP3.
add-account-help-turn-on-imap-link = Як це ввімкнути

## Add a mail account: fields

add-account-field-address = Адреса електронної пошти
add-account-receive-with = Отримувати пошту через
add-account-imap-about = IMAP зберігає ваші листи й папки на сервері, однаково на кожному пристрої. Вибирайте його, коли можете.
add-account-pop3-about = POP3 завантажує вашу пошту на цей комп’ютер. Листи, які ви тут читаєте чи переміщуєте, залишаються як були на сервері та інших ваших пристроях.
add-account-incoming = Вхідна пошта ({ $protocol })
add-account-outgoing = Вихідна пошта ({ $protocol })
add-account-field-server = Сервер
add-account-field-port = Порт
add-account-security-none = Немає
add-account-security-none-warning = Без шифрування: ваш пароль і листи можуть бути прочитані дорогою.
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
add-account-sign-in-with = Увійти через { $provider }
add-account-sign-in-instead = Натомість увійти через { $provider }

## Add a mail account: buttons

add-account-servers-button = Налаштування сервера
add-account-back = Назад
add-account-add = Додати обліковий запис
add-account-done = Готово
add-account-another = Додати ще обліковий запис
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
add-account-app-password-refused = { $provider } відхилив пароль. Потрібен пароль застосунку, а не той, яким ви користуєтеся в браузері.
add-account-password-refused = Сервер відхилив пароль. Перевірте його й спробуйте ще раз.
add-account-sign-in-refused = { $provider } не впустив Katna. Спробуйте ще раз і дозвольте доступ до своєї пошти.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Ця копія Katna поки не вміє входити в облікові записи Microsoft.
    [Google] Ця копія Katna поки не вміє входити в облікові записи Google.
   *[other] Цей постачальник дозволяє входити лише на власній сторінці, а Katna поки не вміє цього для нього.
}
add-account-smtp-not-found = Katna знайшла, звідки читати пошту, але не знайшла, куди її надсилати. Введіть сервер вихідної пошти.
add-account-done-title = Ваш обліковий запис готовий
add-account-done-intro = Katna вже отримує вашу пошту. Нові листи з’являтимуться, щойно надійдуть.
add-account-done-sign-in = Вхід
add-account-done-signed-in-with = Через { $provider }, у браузері
add-account-done-receiving = Отримання пошти
add-account-done-sending = Надсилання пошти
add-account-done-on-server = Пошта на сервері
add-account-done-kept = Зберігається, доки ви не видалите її в Katna
add-account-done-pop3-hint = Що робити з поштою на сервері, можна змінити в «Налаштування» > «Облікові записи».
add-account-done-zoho-title = Завдання й календарі
add-account-done-zoho-about = Zoho зберігає їх окремо від пошти. Увійдіть через Zoho один раз, щоб перенести їх у Katna.
add-account-done-linked = Завдання й календарі підключено

## The account menu (from the account button on the top bar)

add-account-menu-another = Додати ще обліковий запис
app-menu = Головне меню
app-menu-back = Назад
