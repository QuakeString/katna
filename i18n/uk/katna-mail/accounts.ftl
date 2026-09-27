# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Панель папок
accounts-folder-pane-detail = Папки яких облікових записів показує панель ліворуч.
accounts-shown-one = Один обліковий запис за раз; перемикання в картці облікового запису
accounts-shown-all = Усі облікові записи, один за одним
accounts-row = Облікові записи
accounts-row-detail = Панель папок і меню облікових записів показують облікові записи в цьому порядку; перший — типовий. Вилучення облікового запису видаляє копію його пошти, яку Katna зберігає на цьому комп’ютері. Пошта залишається на сервері.
accounts-none = Облікових записів ще немає.
accounts-kind-imported = Імпортовано
accounts-picture-reset = Використати зображення системи
accounts-picture-change = Змінити зображення
accounts-picture-remove = Вилучити зображення
accounts-rename = Перейменувати
accounts-name-save = Зберегти
accounts-name-cancel = Скасувати
accounts-name-placeholder = Ваше ім’я
accounts-rename-failed = Не вдалося перейменувати обліковий запис: { $error }
accounts-move-up = Перемістити вгору
accounts-move-down = Перемістити вниз
accounts-drag = Перетягніть, щоб змінити порядок
accounts-remove = Вилучити
accounts-delete-all-row = Видалити всі дані
accounts-delete-all-row-detail = Почати заново, як після нового встановлення.
accounts-delete-all-about = Видаляє з цього комп’ютера всі облікові записи, усю збережену пошту, контакти й календарі, пошуковий індекс, ваші налаштування та збережені паролі. На ваших поштових серверах нічого не змінюється.
accounts-delete-all-open = Видалити всі дані Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } вилучено з Katna.
accounts-removed = { $address } вилучено з Katna. Його пошта й далі на сервері.
accounts-all-deleted = Усі дані Katna видалено з цього комп’ютера.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Вилучити { $address }?
accounts-remove-confirm = Вилучити обліковий запис
accounts-removing = Вилучення…
accounts-remove-local-mail = { $folders ->
    [0] Уся пошта, імпортована в цей обліковий запис
    [1] Уся пошта, імпортована в цей обліковий запис, у його папці
    [one] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папці
    [few] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папках
    [many] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папках
   *[other] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папках
}
accounts-remove-local-settings = Його налаштування в Katna
accounts-remove-mail = { $folders ->
    [0] Уся пошта цього облікового запису, збережена Katna
    [1] Уся пошта цього облікового запису, збережена Katna в його папці
    [one] Уся пошта цього облікового запису, збережена Katna в його { $folders } папці
    [few] Уся пошта цього облікового запису, збережена Katna в його { $folders } папках
    [many] Уся пошта цього облікового запису, збережена Katna в його { $folders } папках
   *[other] Уся пошта цього облікового запису, збережена Katna в його { $folders } папках
}
accounts-remove-outbox = Його листи, що чекають у вихідних
accounts-remove-settings = Його збережений пароль і налаштування в Katna
accounts-delete-all-title = Видалити всі дані Katna?
accounts-delete-all-confirm = Видалити все
accounts-deleting = Видалення…
accounts-delete-all-accounts = Усі облікові записи, а також уся пошта й вкладення, збережені Katna
accounts-delete-all-contacts = Контакти, календарі й пошуковий індекс
accounts-delete-all-settings = Усі налаштування, підписи й комбінації клавіш
accounts-delete-all-passwords = Усі збережені паролі
accounts-deleted-heading = Видаляється з цього комп’ютера:
accounts-cannot-undo = Цю дію не можна скасувати.
accounts-server-delete-all = На ваших поштових серверах нічого не змінюється: пошта залишається там, і якщо знову додати обліковий запис, вона завантажиться знову. Пошта, імпортована з файлів, є лише в Katna; самі файли не змінюються.
accounts-server-local = Цю пошту імпортовано з файлів, тож єдина її копія — у Katna. Файли, з яких її імпортовано, не змінюються; імпортуйте їх знову, щоб повернути пошту.
accounts-server-remove = На поштовому сервері нічого не змінюється: пошта залишається там, і якщо знову додати обліковий запис, вона завантажиться знову.
accounts-confirm-word = видалити
accounts-confirm-placeholder = Введіть «{ accounts-confirm-word }»
accounts-confirm-prompt = Щоб підтвердити, введіть «{ accounts-confirm-word }»:
accounts-cancel = Скасувати
