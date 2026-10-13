# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Панель папок
accounts-folder-pane-detail = Папки яких облікових записів показує панель ліворуч.
accounts-shown-one = Один обліковий запис за раз; перемикання в картці облікового запису
accounts-shown-all = Усі облікові записи, один за одним
accounts-unified = Спільні вхідні
accounts-unified-switch = Показувати пошту всіх облікових записів разом
accounts-unified-switch-detail = «Усі облікові записи» стоять на початку панелі папок: вхідні, надіслані та інші папки кожного облікового запису в одному списку. Облікові записи нижче спочатку згорнуті.
accounts-row = Облікові записи
accounts-row-detail = Панель папок і меню облікових записів показують облікові записи в цьому порядку; перший — типовий. Вилучення облікового запису видаляє копію його пошти, яку Katna зберігає на цьому комп’ютері. Пошта залишається на сервері.
accounts-none = Облікових записів ще немає.
accounts-pop3-row = Пошта на сервері
accounts-pop3-row-detail = Облікові записи POP3 завантажують пошту на цей комп’ютер. Виберіть, що потім буде з копією на сервері.
accounts-pop3-with-katna = Зберігати, доки я не видалю її в Katna
accounts-pop3-at-once = Видаляти одразу після завантаження
accounts-pop3-after-days = { $count ->
    [one] Видаляти через { $count } день
    [few] Видаляти через { $count } дні
    [many] Видаляти через { $count } днів
   *[other] Видаляти через { $count } дня
}
accounts-pop3-never = Ніколи не видаляти
accounts-pop3-days-less = Менше днів
accounts-pop3-days-more = Більше днів
accounts-kind-imported = Імпортовано
accounts-picture-reset = Використати зображення системи
accounts-picture-change = Змінити зображення
accounts-picture-remove = Вилучити зображення
account-color-red = Червоний
account-color-pink = Рожевий
account-color-magenta = Пурпуровий
account-color-brown = Коричневий
account-color-olive = Оливковий
account-color-teal = Бірюзовий
account-color-indigo = Індиго
account-color-slate = Сланцевий
account-color-menu = Колір
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
reset-cache-about = Видаляє пошту й вкладення, які завантажила Katna, зображення відправників і пошуковий індекс, а потім знову завантажує нещодавні листи. Облікові записи, налаштування й пошта, що є лише на цьому комп’ютері, залишаються.
reset-cache-button = Скинути кеш
reset-cache-title = Скинути кеш?
reset-cache-deleted = Видаляються, а потім завантажуються знову:
reset-cache-mail = Пошта й вкладення, завантажені з ваших серверів IMAP: нещодавні листи завантажуються знову одразу, старіші — коли ви їх відкриєте
reset-cache-index = Пошуковий індекс, який одразу буде створено заново
reset-cache-pictures = Зображення відправників
reset-cache-kept = Залишаються: ваші облікові записи, паролі й налаштування; зірочки, мітки, позначки прочитаного й закріплення; чернетки, вихідні та зміни, що ще не дійшли до сервера; а також пошта з облікових записів POP3 чи імпортованих файлів, якої може не бути більше ніде. На ваших поштових серверах нічого не змінюється.
reset-cache-confirm = Скинути кеш
reset-cache-busy = Скидання…
reset-cache-done = Кеш скинуто. Нещодавні листи завантажуються знову.
reset-cache-done-freed = Кеш скинуто, звільнено { $size }. Нещодавні листи завантажуються знову.
