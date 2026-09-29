# Katna Mail, Ukrainian (Українська): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Створити
tasks-all = Усі завдання
tasks-today = Сьогодні
tasks-starred = Із зірочкою
tasks-new-list = Створити новий список
tasks-on-this-computer = На цьому комп’ютері
tasks-my-tasks = Мої завдання
tasks-list-name-placeholder = Назва списку

## Lists and tasks

tasks-loading = Читання ваших завдань…
tasks-no-lists = Тут з’являться ваші списки завдань.
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
tasks-toast-rescheduled = Задачу перенесено
