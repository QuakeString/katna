# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Ваше ім’я і все, що хочете додати під ним

## Its formatting bar

signature-bold = Жирний
signature-italic = Курсив
signature-underline = Підкреслений
signature-link = Посилання
signature-link-apply = Застосувати
signature-picture = Вставити зображення
signature-align-left = Вирівняти за лівим краєм
signature-align-center = Вирівняти по центру
signature-align-right = Вирівняти за правим краєм
signature-numbered-list = Нумерований список
signature-bulleted-list = Маркований список
signature-remove-formatting = Очистити форматування

## Adding a picture

signature-picture-choose = Вставити
signature-picture-too-big = Зображення в підписі можуть мати розмір до { $size }.
signature-picture-kind = Виберіть зображення PNG, JPEG, GIF або WebP.
signature-picture-unreadable = { $name }: { $error }
signature-layout = Макет
signature-layout-own = Власний
signature-layout-classic = Класичний
signature-layout-logo-left = Логотип ліворуч
signature-layout-photo = Фото
signature-layout-band = Кольорова смуга
signature-layout-one-line = Один рядок
signature-layout-centred = По центру
signature-layout-banner = З банером
signature-layout-underline = Підкреслення
signature-layout-side-bar = Бічна смуга
signature-layout-card = Картка
signature-layout-monogram = Монограма
signature-layout-plain = Звичайний текст
signature-layout-mobile-label = М:
signature-layout-office-label = Р:
signature-layout-email-label = E:
signature-layout-name = Ім’я
signature-layout-job = Посада
signature-layout-company = Компанія
signature-layout-mobile = Мобільний
signature-layout-office = Робочий
signature-layout-email = Електронна пошта
signature-layout-website = Вебсайт
signature-layout-address = Адреса
signature-layout-pictures = Зображення
signature-layout-logo = Логотип
signature-layout-photo-picture = Фото
signature-layout-banner-picture = Банер
signature-layout-remove-picture = Прибрати
signature-layout-pages = Сторінки
signature-layout-page-placeholder = Додайте адресу сторінки
signature-layout-colour = Колір
signature-layout-picture-failed = Не вдалося використати { $name } як зображення.
signature-layout-preview = Як це бачить читач
signature-layout-light = Світлий
signature-layout-dark = Темний
signature-layout-text = Звичайний текст
signature-layout-inside = Зображення надсилаються всередині листа, тож їх видно навіть там, де зображення з інтернету вимкнено. Це додає { $size } до кожного листа.
signature-layout-edit = Змінити вручну
signature-layout-edit-confirm = Змінювати вручну? Поля й макет зникнуть, а вигляд збережеться настільки, наскільки це під силу редактору.
signature-layout-use-confirm = Використати макет «{ $layout }»? Він замінить цей підпис і буде заповнений з нього.
signature-layout-use = Використати макет
signature-layout-cancel = Скасувати
signature-html-title = Вставити HTML
signature-html-subtitle = Для підпису, створеного деінде
signature-html-placeholder = Вставте сюди HTML підпису
signature-html-name = Вставлений
signature-html-new = Збережено як новий підпис «{ $name }»
signature-html-replaces = Замінить «{ $name }»
signature-html-cancel = Скасувати
signature-html-save = Зберегти
signature-html-fetching = Завантаження його зображень…
signature-html-pictures-inside = { $count ->
    [one] { $count } зображення завантажено й вбудовано в лист ({ $size })
    [few] { $count } зображення завантажено й вбудовано в лист ({ $size })
    [many] { $count } зображень завантажено й вбудовано в лист ({ $size })
   *[other] { $count } зображення завантажено й вбудовано в лист ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } зображення не вдалося завантажити, тож читачі завантажать його з інтернету
    [few] { $count } зображення не вдалося завантажити, тож читачі завантажать їх з інтернету
    [many] { $count } зображень не вдалося завантажити, тож читачі завантажать їх з інтернету
   *[other] { $count } зображення не вдалося завантажити, тож читачі завантажать їх з інтернету
}
signature-html-removed = Вилучено скрипти, форми й пікселі відстеження, які поштові програми однаково блокують
signature-html-style-sheet = Таблицю стилів не включено: пошта зберігає лише стилі, записані в кожному елементі
signature-html-links = Вилучено посилання, що вели не на вебсайт, адресу чи телефон
signature-html-plain-text = З нього створено текстову версію для поштових програм, що показують лише текст
signature-import-title = Імпорт
signature-import-subtitle = З Gmail, Thunderbird, Evolution і KMail
signature-import-looking = Пошук підписів…
signature-import-none = Підписів не знайдено. Для іншої програми скопіюйте HTML її підпису й скористайтеся «Вставити HTML».
signature-import-from = З { $app }
signature-import-already = уже є в Katna
signature-import-gmail-sign-in = { $address }: увійдіть знову в «Налаштування > Облікові записи», щоб Katna могла прочитати підписи Gmail.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Скасувати
signature-import-do = { $count ->
    [one] Імпортувати { $count } підпис
    [few] Імпортувати { $count } підписи
    [many] Імпортувати { $count } підписів
   *[other] Імпортувати { $count } підпису
}
signature-import-name = { $name } ({ $app })
