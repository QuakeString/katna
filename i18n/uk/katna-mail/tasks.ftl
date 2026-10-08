# Katna Mail, Ukrainian (Українська): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Нове завдання
tasks-all = Усі завдання
tasks-today = Сьогодні
tasks-upcoming = Найближчі
tasks-starred = Із зірочкою
tasks-completed-view = Виконані
tasks-new-list = Створити новий список
tasks-labels-heading = Мітки
tasks-on-this-computer = На цьому комп’ютері
tasks-my-tasks = Мої завдання
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Увійдіть знову, щоб показати завдання
tasks-account-signed-in = Знову виконано вхід в { $address }. Отримання завдань…
tasks-account-sign-in-refused = { $provider } не впустив Katna. Спробуйте ще раз і дозвольте доступ до завдань.
tasks-account-refused = Сервер не прийняв пароль. Для Yahoo, iCloud, Zoho та інших потрібен пароль застосунку.
tasks-account-change-password = Змінити пароль
tasks-account-change-password-tooltip = Введіть новий пароль; Katna перевірить його на сервері
tasks-account-not-enabled = Доступ Katna до завдань ще не ввімкнено.
tasks-account-failed = Не вдалося прочитати списки завдань.
# $reason is the server's own words, in English.
tasks-account-error = Не вдалося прочитати списки завдань: { $reason }
tasks-account-none = Списків завдань не знайдено
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Списків завдань не знайдено: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } показує завдання лише Katna, що увійшла через { $provider }.
tasks-account-sign-in-with = Увійти через { $provider }
tasks-account-looking = Пошук списків завдань…
tasks-account-try-again = Повторити спробу
tasks-account-try-again-tooltip = Перевірити завдання цього облікового запису ще раз зараз
tasks-account-fixing = Виправляємо…
tasks-list-name-placeholder = Назва списку

## Lists and tasks

tasks-loading = Читання ваших завдань…
tasks-no-lists = Тут з’являться ваші списки завдань.
tasks-search = Пошук завдань
tasks-search-none = Жодне завдання не відповідає запиту.
tasks-add = Додати завдання
tasks-title-placeholder = Назва
tasks-add-step = Додати підзавдання
tasks-empty = Завдань ще немає. Додайте одне вище.
tasks-starred-empty = Позначте завдання зірочкою, щоб побачити його тут.
tasks-label-empty = Немає відкритих завдань із цією міткою.
tasks-today-empty = На сьогодні нічого немає.
tasks-completed-empty = Тут з’являться виконані завдання.
tasks-upcoming-add = Додати завдання на { $day }
tasks-upcoming-overdue-day = { $weekday }, { $day }
tasks-from-mail-quiet = З листа
tasks-from-note-quiet = З нотатки
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Прострочені
tasks-completed = { $count ->
    [one] Виконані ({ $count })
    [few] Виконані ({ $count })
    [many] Виконані ({ $count })
   *[other] Виконані ({ $count })
}
tasks-list-options = Параметри списку
tasks-sort-by = Сортувати за
tasks-sort-my-order = Мій порядок
tasks-sort-date = Дата
tasks-sort-starred = Нещодавно позначені зірочкою
tasks-sort-title = Назва
tasks-rename-list = Перейменувати список
tasks-delete-list = Видалити список
tasks-mark-done = Позначити як виконане
tasks-mark-open = Позначити як невиконане
tasks-star = Позначити зірочкою
tasks-unstar = Зняти зірочку
tasks-edit-title = Змінити назву
tasks-details = Подробиці
tasks-delete = Видалити
tasks-move-to = Перемістити до { $list }
tasks-from-mail = Пошта
tasks-open-mail = Відкрити лист
tasks-from-note = Нотатка
tasks-open-note = Відкрити нотатку
tasks-note-gone = Цієї нотатки тут більше немає.
tasks-no-subject = (без теми)
tasks-selected = { $count ->
    [one] Вибрано { $count }
    [few] Вибрано { $count }
    [many] Вибрано { $count }
   *[other] Вибрано { $count }
}
tasks-select-clear = Скасувати вибір
tasks-select-move = Перемістити до списку
tasks-select-date = Установити дату
tasks-next-week = Наступного тижня

## The details dialog

tasks-notes-placeholder = Додати подробиці
tasks-date = Дата
tasks-no-date = Без дати
tasks-time-placeholder = Додати час
tasks-repeat = Повторення
tasks-repeat-never = Не повторюється
tasks-repeat-daily = Щодня
tasks-repeat-weekly = Щотижня
tasks-repeat-monthly = Щомісяця
tasks-repeat-yearly = Щороку
tasks-repeat-other = Інше
tasks-remind = Нагадати
tasks-remind-off = Не нагадувати
tasks-remind-on-time = У момент завдання
tasks-remind-morning = Цього дня, { $time }
tasks-remind-hour-before = За годину
tasks-remind-day-before = За день
tasks-label-add = Додати мітку
tasks-label-task = Позначити завдання міткою
tasks-files-attach = Прикріпити файли
tasks-files-pick = Прикріпити
tasks-file-open = Відкрити
tasks-file-remove = Прибрати файл
tasks-file-here = Лише на цьому комп’ютері
tasks-cancel = Скасувати
tasks-save = Зберегти
tasks-not-a-time = «{ $text }» – це не час, наприклад { $example }.

## Due days

tasks-due-today = Сьогодні
tasks-due-tomorrow = Завтра
tasks-due-yesterday = Учора
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Завдання виконано
tasks-toast-next = Готово. Наступне: { $date }
tasks-toast-deleted = Завдання видалено
tasks-files-added = { $count ->
    [one] { $count } файл прикріплено
    [few] { $count } файли прикріплено
    [many] { $count } файлів прикріплено
   *[other] { $count } файлу прикріплено
}
tasks-file-removed = «{ $name }» прибрано
tasks-files-left-out = Не прикріплено: { $names }. До завдання можна прикріпити файли розміром до { $limit }, але не папки.
tasks-file-missing = Цього файлу тут більше немає.
tasks-toast-added = { $count ->
    [one] { $count } завдання додано
    [few] { $count } завдання додано
    [many] { $count } завдань додано
   *[other] { $count } завдання додано
}
tasks-mail-gone = Цього листа тут більше немає.
tasks-toast-list-deleted = Список видалено
tasks-toast-moved = Переміщено до { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Завдання переміщено
tasks-toast-rescheduled = Задачу перенесено
tasks-toast-rescheduled-several = { $count ->
    [one] { $count } завдання перенесено
    [few] { $count } завдання перенесено
    [many] { $count } завдань перенесено
   *[other] { $count } завдання перенесено
}
tasks-toast-done-several = { $count ->
    [one] { $count } завдання виконано
    [few] { $count } завдання виконано
    [many] { $count } завдань виконано
   *[other] { $count } завдання виконано
}
tasks-toast-open-several = { $count ->
    [one] { $count } завдання позначено як невиконане
    [few] { $count } завдання позначено як невиконані
    [many] { $count } завдань позначено як невиконані
   *[other] { $count } завдання позначено як невиконані
}
tasks-toast-starred = { $count ->
    [one] { $count } завдання позначено зірочкою
    [few] { $count } завдання позначено зірочкою
    [many] { $count } завдань позначено зірочкою
   *[other] { $count } завдання позначено зірочкою
}
tasks-toast-unstarred = { $count ->
    [one] Зірочку знято з { $count } завдання
    [few] Зірочки знято з { $count } завдань
    [many] Зірочки знято з { $count } завдань
   *[other] Зірочки знято з { $count } завдання
}
tasks-toast-deleted-several = { $count ->
    [one] { $count } завдання видалено
    [few] { $count } завдання видалено
    [many] { $count } завдань видалено
   *[other] { $count } завдання видалено
}
