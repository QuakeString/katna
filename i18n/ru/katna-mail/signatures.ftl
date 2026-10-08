# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Ваше имя и всё, что хотите добавить под ним

## Its formatting bar

signature-bold = Полужирный
signature-italic = Курсив
signature-underline = Подчёркнутый
signature-link = Ссылка
signature-link-apply = Применить
signature-picture = Вставить изображение
signature-align-left = По левому краю
signature-align-center = По центру
signature-align-right = По правому краю
signature-numbered-list = Нумерованный список
signature-bulleted-list = Маркированный список
signature-remove-formatting = Очистить форматирование

## Adding a picture

signature-picture-choose = Вставить
signature-picture-too-big = Изображения в подписи могут быть размером до { $size }.
signature-picture-kind = Выберите изображение PNG, JPEG, GIF или WebP.
signature-picture-unreadable = { $name }: { $error }
signature-layout = Макет
signature-layout-own = Свой
signature-layout-classic = Классический
signature-layout-logo-left = Логотип слева
signature-layout-photo = Фото
signature-layout-band = Цветная полоса
signature-layout-one-line = В одну строку
signature-layout-centred = По центру
signature-layout-banner = С баннером
signature-layout-underline = С подчёркиванием
signature-layout-side-bar = Боковая полоса
signature-layout-card = Карточка
signature-layout-monogram = Монограмма
signature-layout-plain = Обычный текст
signature-layout-mobile-label = М:
signature-layout-office-label = Р:
signature-layout-email-label = E:
signature-layout-name = Имя
signature-layout-job = Должность
signature-layout-company = Компания
signature-layout-mobile = Мобильный
signature-layout-office = Рабочий
signature-layout-email = Эл. почта
signature-layout-website = Сайт
signature-layout-address = Адрес
signature-layout-pictures = Изображения
signature-layout-logo = Логотип
signature-layout-photo-picture = Фото
signature-layout-banner-picture = Баннер
signature-layout-remove-picture = Удалить
signature-layout-pages = Страницы
signature-layout-page-placeholder = Добавьте адрес страницы
signature-layout-colour = Цвет
signature-layout-picture-failed = Не удалось использовать { $name } как изображение.
signature-layout-preview = Как это увидит получатель
signature-layout-light = Светлая
signature-layout-dark = Тёмная
signature-layout-text = Обычный текст
signature-layout-inside = Изображения отправляются внутри письма, поэтому видны даже там, где внешние изображения отключены. Это добавляет { $size } к каждому письму.
signature-layout-free = Хотите что-то другое?
signature-layout-edit = Изменить вручную
signature-layout-edit-confirm = Изменить вручную? Поля и макет исчезнут, а внешний вид сохранится настолько, насколько позволяет редактор.
signature-layout-use-confirm = Использовать макет «{ $layout }»? Он заменит эту подпись и будет заполнен её данными.
signature-layout-use = Использовать макет
signature-layout-cancel = Отмена
signature-html-title = Вставить HTML
signature-html-subtitle = Для подписи, созданной в другом месте
signature-html-placeholder = Вставьте сюда HTML подписи
signature-html-name = Вставленная
signature-html-new = Сохраняется как новая подпись «{ $name }»
signature-html-replaces = Заменяет «{ $name }»
signature-html-cancel = Отмена
signature-html-save = Сохранить
signature-html-fetching = Загружаем изображения…
signature-html-pictures-inside = { $count ->
    [one] { $count } изображение загружено и вложено в письмо ({ $size })
    [few] { $count } изображения загружены и вложены в письмо ({ $size })
    [many] { $count } изображений загружено и вложено в письмо ({ $size })
   *[other] { $count } изображения загружено и вложено в письмо ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } изображение не удалось загрузить, поэтому получатели загрузят его из интернета
    [few] { $count } изображения не удалось загрузить, поэтому получатели загрузят их из интернета
    [many] { $count } изображений не удалось загрузить, поэтому получатели загрузят их из интернета
   *[other] { $count } изображения не удалось загрузить, поэтому получатели загрузят их из интернета
}
signature-html-removed = Удалены скрипты, формы и пиксели отслеживания — почтовые программы всё равно их блокируют
signature-html-style-sheet = Таблица стилей не включена: в письме сохраняются только стили, заданные у каждого элемента
signature-html-links = Удалены ссылки, которые вели не на сайт, адрес почты или телефон
signature-html-plain-text = Из неё создана текстовая версия для почтовых программ, которые показывают только текст
signature-import-title = Импорт
signature-import-subtitle = Из Gmail, Thunderbird, Evolution и KMail
signature-import-looking = Ищем подписи…
signature-import-none = Подписи не найдены. Для другой программы скопируйте HTML её подписи и используйте «Вставить HTML».
signature-import-from = Из { $app }
signature-import-already = уже есть в Katna
signature-import-gmail-sign-in = { $address }: войдите снова в «Настройки» > «Аккаунты», чтобы Katna могла прочитать подписи Gmail.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Отмена
signature-import-do = { $count ->
    [one] Импортировать { $count } подпись
    [few] Импортировать { $count } подписи
    [many] Импортировать { $count } подписей
   *[other] Импортировать { $count } подписи
}
signature-import-name = { $name } ({ $app })
