# Katna Mail, Russian (Русский): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Контакты
contacts-frequent = Часто используемые
contacts-labels = Ярлыки
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
contacts-deleted = Удалено: { $name }
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
