# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Параметри пошуку
search-options-close = Закрити
search-from = Від
search-to = Кому
search-subject = Тема
search-has-words = Містить слова
search-without = Не містить
search-date-within = Дата в межах
search-has-attachment = Має вкладення
search-attachment-custom = Власний
search-attachment-custom-hint = Введіть розширення, наприклад png, і натисніть Пробіл
search-attachment-remove = Вилучити
search-clear-filter = Очистити фільтр

## Search options: "Date within" choices

search-within-any = Будь-коли
search-within-days = { $count ->
    [one] { $count } день
    [few] { $count } дні
    [many] { $count } днів
   *[other] { $count } дня
}
search-within-weeks = { $count ->
    [one] { $count } тиждень
    [few] { $count } тижні
    [many] { $count } тижнів
   *[other] { $count } тижня
}
search-within-months = { $count ->
    [one] { $count } місяць
    [few] { $count } місяці
    [many] { $count } місяців
   *[other] { $count } місяця
}
search-within-years = { $count ->
    [one] { $count } рік
    [few] { $count } роки
    [many] { $count } років
   *[other] { $count } року
}
search-within-custom = Власний

## Search options: custom dates (the calendar popover)

search-dates-on = Дата
search-dates-before = До
search-dates-since = Від
search-dates-between = Між
search-dates-from = Від
search-dates-to = До
search-dates-placeholder = РРРР-ММ-ДД
search-dates-missing = Виберіть дату
search-dates-unreadable = Вкажіть дату на зразок 2026-09-01
search-dates-out-of-range = Ця дата поза допустимим діапазоном
search-dates-chip-before = До { $date }
search-dates-chip-since = Від { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Скасувати
search-dates-done = Готово
search-dates-month-back = Попередній місяць
search-dates-month-on = Наступний місяць
search-dates-year-back = Попередній рік
search-dates-year-on = Наступний рік
