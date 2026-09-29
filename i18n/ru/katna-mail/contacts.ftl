# Katna Mail, Russian (Русский): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Контакты
contacts-frequent = Часто используемые
contacts-other = Другие контакты
contacts-other-about = Люди, которым вы писали из Gmail, но которых не сохранили
contacts-other-email = Написать письмо
contacts-other-empty = Других контактов нет. Люди, которым вы пишете из Gmail, но которых не сохраняете, появятся здесь.
contacts-other-allow = Чтобы увидеть другие контакты, снова войдите в аккаунт Gmail и разрешите Katna их просматривать.
contacts-labels = Ярлыки
contacts-label-options = Параметры ярлыка
contacts-label-rename = Переименовать ярлык
contacts-label-email = Написать всем
contacts-label-delete = Удалить ярлык
contacts-label-new = Новый ярлык
contacts-label-name = Название ярлыка
contacts-label-button = Ярлык
contacts-label-menu = Назначить ярлык:
contacts-label-added = Добавлено в ярлык «{ $name }»
contacts-label-removed = Удалено из ярлыка «{ $name }»
contacts-label-renamed = Ярлык переименован в «{ $name }»
contacts-label-deleted = Ярлык «{ $name }» удалён
contacts-label-no-email = Ни у кого с этим ярлыком нет адреса электронной почты
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Аккаунты
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Войдите снова, чтобы показать контакты
contacts-account-signed-in = Вход в { $address } выполнен снова. Загрузка контактов…
contacts-account-sign-in-refused = { $provider } не впустил Katna. Попробуйте снова и разрешите доступ к контактам.
contacts-account-password = Сервер не принял пароль. Для Yahoo, iCloud, Zoho и других нужен пароль приложения.
contacts-account-change-password = Сменить пароль
contacts-account-change-password-tooltip = Открыть Настройки > Аккаунты
contacts-account-failed = Не удалось прочитать контакты.
# $reason is the server's own words, in English.
contacts-account-error = Не удалось прочитать контакты: { $reason }
contacts-account-none = Адресная книга не найдена
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Адресная книга не найдена: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } показывает контакты только Katna, вошедшей через { $provider }.
contacts-account-sign-in-with = Войти через { $provider }
contacts-account-looking = Поиск контактов…
contacts-account-try-again = Повторить попытку
contacts-account-try-again-tooltip = Сейчас снова проверить контакты этого аккаунта
contacts-account-fixing = Исправляем…
contacts-manage = Исправление и управление
contacts-merge = Объединение и исправление
contacts-merge-about = { $count ->
    [one] { $count } предложение: контакты, которые похожи на одного человека
    [few] { $count } предложения: контакты, которые похожи на одного человека
    [many] { $count } предложений: контакты, которые похожи на одного человека
   *[other] { $count } предложения: контакты, которые похожи на одного человека
}
contacts-merge-none = Дубликатов нет. Здесь появятся контакты с одинаковым именем или номером телефона.
contacts-merge-count = { $count ->
    [one] { $count } контакт
    [few] { $count } контакта
    [many] { $count } контактов
   *[other] { $count } контакта
}
contacts-merge-all = Объединить все
contacts-merge-button = Объединить
contacts-merge-dismiss = Отклонить
contacts-merged = { $count ->
    [1] Контакты объединены
    [one] Выполнено объединений: { $count }
    [few] Выполнено объединений: { $count }
    [many] Выполнено объединений: { $count }
   *[other] Выполнено объединений: { $count }
}
contacts-import = Импортировать
contacts-export = Экспортировать
contacts-import-file = Импорт контактов из файла vCard или CSV
contacts-imported = { $count ->
    [one] Импортирован { $count } контакт в { $place }
    [few] Импортировано { $count } контакта в { $place }
    [many] Импортировано { $count } контактов в { $place }
   *[other] Импортировано { $count } контакта в { $place }
}
contacts-imported-some = { $count ->
    [one] Импортирован { $count } контакт в { $place }; пропущено уже сохранённых: { $skipped }
    [few] Импортировано { $count } контакта в { $place }; пропущено уже сохранённых: { $skipped }
    [many] Импортировано { $count } контактов в { $place }; пропущено уже сохранённых: { $skipped }
   *[other] Импортировано { $count } контакта в { $place }; пропущено уже сохранённых: { $skipped }
}
contacts-import-none = В файле { $name } контакты не найдены
contacts-import-all-saved = Все, кто указан в файле { $name }, уже сохранены
contacts-import-failed = Не удалось прочитать { $name }: { $error }
contacts-exported = { $count ->
    [one] Экспортирован { $count } контакт в { $path }
    [few] Экспортировано { $count } контакта в { $path }
    [many] Экспортировано { $count } контактов в { $path }
   *[other] Экспортировано { $count } контакта в { $path }
}
contacts-export-none = Нет контактов для экспорта
contacts-export-failed = Не удалось экспортировать контакты: { $error }
contacts-print = Печать
contacts-print-title = Контакты
contacts-print-none = Нет контактов для печати
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = День рождения: { $day }
contacts-print-nickname = Псевдоним: { $name }
contacts-create = Создать контакт

## Search and the list

contacts-search = Поиск контактов
contacts-loading = Загрузка контактов…
contacts-empty = Сохранённых контактов пока нет. Контакты, которые вы сохраняете в Gmail, Outlook или почтовом сервисе, появятся здесь.
contacts-empty-no-books = Контакты из ваших аккаунтов появятся здесь после синхронизации.
contacts-none-found = Нет контактов, подходящих под запрос.
contacts-starred = { $count ->
    [one] Контакт с пометкой ({ $count })
    [few] Контакта с пометкой ({ $count })
    [many] Контактов с пометкой ({ $count })
   *[other] Контакта с пометкой ({ $count })
}
contacts-count = Контакты ({ $count })
contacts-col-name = Имя
contacts-col-email = Электронная почта
contacts-col-phone = Номер телефона
contacts-col-job = Должность и компания
contacts-col-labels = Ярлыки

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Разрешить Katna читать контакты аккаунта { $address }.
contacts-allow-many = { $more ->
    [one] Разрешить Katna читать контакты аккаунта { $address } и ещё { $more } аккаунта.
    [few] Разрешить Katna читать контакты аккаунта { $address } и ещё { $more } аккаунтов.
    [many] Разрешить Katna читать контакты аккаунта { $address } и ещё { $more } аккаунтов.
   *[other] Разрешить Katna читать контакты аккаунта { $address } и ещё { $more } аккаунта.
}
contacts-allow-button = Разрешить

## A contact's page

contacts-back = Назад к контактам
contacts-edit = Изменить
contacts-delete = Удалить
contacts-qr = Поделиться в виде QR-кода
contacts-qr-about = Отсканируйте код камерой телефона, чтобы сохранить контакт.
contacts-qr-too-long = У этого контакта слишком много данных для QR-кода.
contacts-qr-done = Готово
contacts-deleted = Удалено: { $name }
contacts-added = Контакт { $name } добавлен
contacts-find-mail = Почта
contacts-details = Контактная информация
contacts-saved-in = Сохранён в
contacts-notes = Заметки
contacts-birthday = День рождения
contacts-nickname = Псевдоним
contacts-this-computer = Этот компьютер
contacts-kind-home = Домашний
contacts-kind-work = Рабочий
contacts-kind-mobile = Мобильный
contacts-kind-other = Другой
contacts-source-google = Google Контакты
contacts-source-microsoft = Контакты Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Создать контакт
contacts-edit-title = Изменить контакт
contacts-edit-save = Сохранить
contacts-edit-saving = Сохранение…
contacts-edit-cancel = Отмена
contacts-saved = Контакт сохранён
contacts-edit-save-to = Сохранить в
contacts-edit-changes-go-to = Изменения сохраняются в { $place }.
contacts-edit-given = Имя
contacts-edit-family = Фамилия
contacts-edit-company = Компания
contacts-edit-job = Должность
contacts-edit-email = Электронная почта
contacts-edit-phone = Телефон
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Добавить адрес электронной почты
contacts-edit-add-phone = Добавить телефон
contacts-edit-street = Улица
contacts-edit-city = Город
contacts-edit-postcode = Почтовый индекс
contacts-edit-country = Страна
contacts-edit-birthday = День рождения (YYYY-MM-DD)
contacts-edit-empty = Сначала добавьте имя, адрес электронной почты или номер телефона.
