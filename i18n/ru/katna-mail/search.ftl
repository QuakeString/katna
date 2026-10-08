# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Параметры поиска
search-options-close = Закрыть
search-from = От
search-to = Кому
search-subject = Тема
search-has-words = Содержит слова
search-without = Не содержит
search-date-within = Дата в пределах
search-has-attachment = Есть вложение
search-attachment-custom = Другой
search-attachment-image = Изображение
search-attachment-custom-hint = Введите расширение, например png, и нажмите Пробел
search-attachment-remove = Удалить
search-clear-filter = Сбросить фильтр

## Search options: "Date within" choices

search-within-any = За всё время
search-within-days = { $count ->
    [one] { $count } дня
    [few] { $count } дней
    [many] { $count } дней
   *[other] { $count } дня
}
search-within-weeks = { $count ->
    [one] { $count } недели
    [few] { $count } недель
    [many] { $count } недель
   *[other] { $count } недели
}
search-within-months = { $count ->
    [one] { $count } месяца
    [few] { $count } месяцев
    [many] { $count } месяцев
   *[other] { $count } месяца
}
search-within-years = { $count ->
    [one] { $count } года
    [few] { $count } лет
    [many] { $count } лет
   *[other] { $count } года
}
search-within-custom = Другой период

## Search options: custom dates (the calendar popover)

search-dates-on = В день
search-dates-before = До
search-dates-since = С
search-dates-between = Между
search-dates-from = С
search-dates-to = По
search-dates-placeholder = ГГГГ-ММ-ДД
search-dates-missing = Выберите дату
search-dates-unreadable = Введите дату в виде 2026-09-01
search-dates-out-of-range = Дата вне допустимого диапазона
search-dates-chip-before = До { $date }
search-dates-chip-since = С { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Отмена
search-dates-done = Готово
search-dates-month-back = Предыдущий месяц
search-dates-month-on = Следующий месяц
search-dates-year-back = Предыдущий год
search-dates-year-on = Следующий год
