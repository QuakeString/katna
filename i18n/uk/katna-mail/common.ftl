# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Мова: { $language }
language-tooltip-system = Мова: { $language }, як у системі
language-search = Пошук мови
language-system-default = Як у системі
language-system-now = Зараз: { $language }
language-no-match = Не знайдено мови за запитом «{ $query }»
language-machine = Перекладено машинно. Допоможіть покращити
language-setting = Мова
language-setting-detail = Мова меню, кнопок і повідомлень, а також формат дат і чисел. Варіант «Як у системі» використовує налаштування стільниці.

## Dates and sizes

ago-just-now = щойно
ago-minutes = { $count ->
    [one] { $count } хвилину тому
    [few] { $count } хвилини тому
    [many] { $count } хвилин тому
   *[other] { $count } хвилини тому
}
ago-hours = { $count ->
    [one] { $count } годину тому
    [few] { $count } години тому
    [many] { $count } годин тому
   *[other] { $count } години тому
}
ago-days = { $count ->
    [one] { $count } день тому
    [few] { $count } дні тому
    [many] { $count } днів тому
   *[other] { $count } дня тому
}
size-bytes = { $count ->
    [one] { $count } байт
    [few] { $count } байти
    [many] { $count } байтів
   *[other] { $count } байта
}
size-kb = { $size } КБ
size-mb = { $size } МБ
size-gb = { $size } ГБ
size-tb = { $size } ТБ

## Top bar

folders-hide = Сховати папки
folders-show = Показати папки
side-pane-hide = Сховати бічну панель
side-pane-show = Показати бічну панель
compose = Написати
search = Пошук
search-mail = Пошук у пошті
search-settings = Пошук у налаштуваннях
search-clear = Очистити пошук
search-options-show = Показати параметри пошуку
settings = Налаштування
account-add = Додати обліковий запис
account-wheel-hint = Прокрутіть, щоб перемкнути обліковий запис
account-menu-all-detail = { $count ->
    [one] { $count } обліковий запис разом
    [few] { $count } облікові записи разом
    [many] { $count } облікових записів разом
   *[other] { $count } облікового запису разом
}
