# Katna Mail, Ukrainian (Українська): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Нове завдання
tasks-all = Усі завдання
tasks-today = Сьогодні
tasks-starred = Із зірочкою
tasks-new-list = Створити новий список
tasks-on-this-computer = На цьому комп’ютері
tasks-my-tasks = Мої завдання
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Увійдіть знову, щоб показати завдання
tasks-account-signed-in = Знову виконано вхід в { $address }. Отримання завдань…
tasks-account-sign-in-refused = { $provider } не впустив Katna. Спробуйте ще раз і дозвольте доступ до завдань.
tasks-account-refused = Сервер не прийняв пароль. Для Yahoo, iCloud, Zoho та інших потрібен пароль застосунку.
tasks-account-change-password = Змінити пароль
tasks-account-change-password-tooltip = Відкрити Налаштування > Облікові записи
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
tasks-today-empty = На сьогодні нічого немає.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Прострочені
tasks-completed = { $count ->
    [one] Виконані ({ $count })
    [few] Виконані ({ $count })
    [many] Виконані ({ $count })
   *[other] Виконані ({ $count })
}
tasks-list-options = Параметри списку
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
