# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = Пошта
rail-calendar = Календар
rail-contacts = Контакти
rail-tasks = Завдання
rail-notes = Нотатки
rail-files = Файли
rail-menu-open = Відкрити { $app }
rail-menu-settings = Налаштування: { $app }
rail-menu-turn-off = Вимкнути { $app }…
app-off-title = Вимкнути { $app }?
app-off-body = Katna припинить синхронізувати { $app } і прибере цю програму з:
app-off-keep = Зберегти копію на цьому комп’ютері
app-off-keep-detail = Увімкнути знову можна миттєво
app-off-remove = Видалити копію з цього комп’ютера
app-off-remove-detail = В облікових записах нічого не зміниться, а після повторного ввімкнення все завантажиться знову. Те, що є лише на цьому комп’ютері або ще не надіслане, залишиться.
app-off-cancel = Скасувати
app-off-confirm = Вимкнути
app-off-done = { $app } вимкнено
app-off-note = { $app } вимкнено
app-off-turn-on = Увімкнути
app-off-leaves-calendar-rail = Панелі програм і Ctrl+2
app-off-leaves-calendar-agenda = Розкладу поруч із поштою
app-off-leaves-calendar-meeting = «Запланувати зустріч» і «Відкрити в Календарі» в запрошеннях
app-off-leaves-calendar-reminders = Нагадувань про події
app-off-leaves-calendar-desktop = Подій у KRunner і годиннику стільниці
app-off-leaves-contacts-rail = Панелі програм і Ctrl+3
app-off-leaves-contacts-card = «Додати до контактів» у картці відправника
app-off-leaves-contacts-birthdays = Днів народження в Календарі
app-off-leaves-tasks-rail = Панелі програм і Ctrl+4
app-off-leaves-tasks-mail = «Додати до Завдань» у листах і Shift+T
app-off-leaves-tasks-calendar = Завдань у Календарі
app-off-leaves-tasks-tray = «Нове завдання» в лотку і Meta+Alt+T
app-off-leaves-tasks-reminders = Нагадувань про завдання
app-off-leaves-notes-rail = Панелі програм і Ctrl+5
app-off-leaves-notes-mail = «Додати нотатку» в листах
app-off-leaves-notes-meetings = Нотаток до зустрічей у подіях
app-off-leaves-notes-tray = «Нова нотатка» в лотку і Meta+Alt+N
app-off-leaves-notes-reminders = Нагадувань про нотатки
app-off-leaves-files-rail = Панелі програм і Ctrl+7
app-off-leaves-files-compose = «Файлів» під час додавання вкладень у листі

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Незабаром
app-calendar-promise = Ваші календарі CalDAV, запрошення на зустрічі з пошти й нагадування — поруч із вхідними.
app-tasks-promise = Списки справ, що синхронізуються через CalDAV, і завдання, створені з листів.
app-notes-promise = Швидкі нотатки й нотатки до листа чи ланцюжка на потім.

## Contacts page

app-contacts-loading = Збираємо людей із вашої пошти…
app-contacts-empty = Тут з’являться люди, з якими ви листуєтеся.
app-contacts-count = { $count ->
    [one] { $count } людина з вашої пошти, спершу найчастіші співрозмовники
    [few] { $count } людини з вашої пошти, спершу найчастіші співрозмовники
    [many] { $count } людей з вашої пошти, спершу найчастіші співрозмовники
   *[other] { $count } людини з вашої пошти, спершу найчастіші співрозмовники
}
app-contacts-top = { $count ->
    [one] { $count } найчастіший співрозмовник із вашої пошти
    [few] { $count } найчастіші співрозмовники з вашої пошти
    [many] { $count } найчастіших співрозмовників із вашої пошти
   *[other] { $count } найчастішого співрозмовника з вашої пошти
}
app-contacts-messages = { $count ->
    [one] { $count } лист
    [few] { $count } листи
    [many] { $count } листів
   *[other] { $count } листа
}
app-contacts-last = останній: { $date }
top-brand = Katna
