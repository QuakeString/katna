# Katna Mail, Ukrainian (Українська): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Контакти
contacts-frequent = Часто використовувані
contacts-labels = Мітки

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
