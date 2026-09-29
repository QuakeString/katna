# Katna Mail, Ukrainian (Українська): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Контакти
contacts-frequent = Часто використовувані
contacts-other = Інші контакти
contacts-other-about = Люди, яким ви писали з Gmail, але яких не зберегли
contacts-other-email = Написати листа
contacts-other-empty = Інших контактів немає. Люди, яким ви пишете з Gmail, але яких не зберігаєте, з’являться тут.
contacts-other-allow = Щоб побачити інші контакти, знову увійдіть в обліковий запис Gmail і дозвольте Katna їх переглядати.
contacts-labels = Мітки
contacts-label-options = Параметри мітки
contacts-label-rename = Перейменувати мітку
contacts-label-email = Написати всім
contacts-label-delete = Видалити мітку
contacts-label-new = Нова мітка
contacts-label-name = Назва мітки
contacts-label-button = Мітка
contacts-label-menu = Позначити міткою:
contacts-label-added = Додано до мітки «{ $name }»
contacts-label-removed = Вилучено з мітки «{ $name }»
contacts-label-renamed = Мітку перейменовано на «{ $name }»
contacts-label-deleted = Мітку «{ $name }» видалено
contacts-label-no-email = Ніхто з цією міткою не має адреси електронної пошти
contacts-manage = Виправлення й керування
contacts-merge = Об’єднання й виправлення
contacts-merge-about = { $count ->
    [one] { $count } пропозиція: контакти, схожі на одну людину
    [few] { $count } пропозиції: контакти, схожі на одну людину
    [many] { $count } пропозицій: контакти, схожі на одну людину
   *[other] { $count } пропозиції: контакти, схожі на одну людину
}
contacts-merge-none = Дублікатів немає. Тут з’являться контакти з однаковим іменем або номером телефону.
contacts-merge-count = { $count ->
    [one] { $count } контакт
    [few] { $count } контакти
    [many] { $count } контактів
   *[other] { $count } контакту
}
contacts-merge-all = Об’єднати всі
contacts-merge-button = Об’єднати
contacts-merge-dismiss = Відхилити
contacts-merged = { $count ->
    [1] Контакти об’єднано
    [one] Виконано об’єднань: { $count }
    [few] Виконано об’єднань: { $count }
    [many] Виконано об’єднань: { $count }
   *[other] Виконано об’єднань: { $count }
}
contacts-import = Імпортувати
contacts-export = Експортувати
contacts-import-title = Імпорт контактів із файлу vCard
contacts-imported = { $count ->
    [one] Імпортовано { $count } контакт до { $place }
    [few] Імпортовано { $count } контакти до { $place }
    [many] Імпортовано { $count } контактів до { $place }
   *[other] Імпортовано { $count } контакту до { $place }
}
contacts-imported-some = { $count ->
    [one] Імпортовано { $count } контакт до { $place }; пропущено вже збережених: { $skipped }
    [few] Імпортовано { $count } контакти до { $place }; пропущено вже збережених: { $skipped }
    [many] Імпортовано { $count } контактів до { $place }; пропущено вже збережених: { $skipped }
   *[other] Імпортовано { $count } контакту до { $place }; пропущено вже збережених: { $skipped }
}
contacts-import-none = У файлі { $name } контактів не знайдено
contacts-import-all-saved = Усі з файлу { $name } уже збережені
contacts-import-failed = Не вдалося прочитати { $name }: { $error }
contacts-exported = { $count ->
    [one] Експортовано { $count } контакт до { $path }
    [few] Експортовано { $count } контакти до { $path }
    [many] Експортовано { $count } контактів до { $path }
   *[other] Експортовано { $count } контакту до { $path }
}
contacts-export-none = Немає контактів для експорту
contacts-export-failed = Не вдалося експортувати контакти: { $error }
contacts-create = Створити контакт

## Search and the list

contacts-search = Пошук контактів
contacts-loading = Завантаження контактів…
contacts-empty = Збережених контактів поки немає. Контакти, які ви зберігаєте в Gmail, Outlook або поштовому сервісі, з’являться тут.
contacts-empty-no-books = Контакти з ваших облікових записів з’являться тут після синхронізації.
contacts-none-found = Жоден контакт не відповідає запиту.
contacts-starred = { $count ->
    [one] Контакт із зірочкою ({ $count })
    [few] Контакти із зірочкою ({ $count })
    [many] Контактів із зірочкою ({ $count })
   *[other] Контакту із зірочкою ({ $count })
}
contacts-count = Контакти ({ $count })
contacts-col-name = Ім’я
contacts-col-email = Електронна пошта
contacts-col-phone = Номер телефону
contacts-col-job = Посада й компанія
contacts-col-labels = Мітки

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Дозволити Katna читати контакти облікового запису { $address }.
contacts-allow-many = { $more ->
    [one] Дозволити Katna читати контакти облікового запису { $address } і ще { $more } облікового запису.
    [few] Дозволити Katna читати контакти облікового запису { $address } і ще { $more } облікових записів.
    [many] Дозволити Katna читати контакти облікового запису { $address } і ще { $more } облікових записів.
   *[other] Дозволити Katna читати контакти облікового запису { $address } і ще { $more } облікового запису.
}
contacts-allow-button = Дозволити

## A contact's page

contacts-back = Назад до контактів
contacts-edit = Змінити
contacts-delete = Видалити
contacts-deleted = Видалено: { $name }
contacts-added = Контакт { $name } додано
contacts-find-mail = Пошта
contacts-details = Контактні дані
contacts-saved-in = Збережено в
contacts-notes = Нотатки
contacts-birthday = День народження
contacts-nickname = Псевдонім
contacts-this-computer = Цей комп’ютер
contacts-kind-home = Дім
contacts-kind-work = Робота
contacts-kind-mobile = Мобільний
contacts-kind-other = Інше
contacts-source-google = Google Контакти
contacts-source-microsoft = Контакти Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Створити контакт
contacts-edit-title = Змінити контакт
contacts-edit-save = Зберегти
contacts-edit-saving = Збереження…
contacts-edit-cancel = Скасувати
contacts-saved = Контакт збережено
contacts-edit-save-to = Зберегти в
contacts-edit-changes-go-to = Зміни зберігаються в { $place }.
contacts-edit-given = Ім’я
contacts-edit-family = Прізвище
contacts-edit-company = Компанія
contacts-edit-job = Посада
contacts-edit-email = Електронна пошта
contacts-edit-phone = Телефон
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Додати електронну адресу
contacts-edit-add-phone = Додати номер телефону
contacts-edit-street = Вулиця
contacts-edit-city = Місто
contacts-edit-postcode = Поштовий індекс
contacts-edit-country = Країна
contacts-edit-birthday = День народження (YYYY-MM-DD)
contacts-edit-empty = Спершу додайте ім’я, електронну адресу або номер телефону.
