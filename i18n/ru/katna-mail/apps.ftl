# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = Почта
rail-calendar = Календарь
rail-contacts = Контакты
rail-tasks = Задачи
rail-notes = Заметки
rail-files = Файлы
rail-menu-open = Открыть { $app }
rail-menu-settings = Настройки: { $app }
rail-menu-turn-off = Отключить { $app }…
app-off-title = Отключить { $app }?
app-off-body = Katna перестанет синхронизировать { $app } и уберёт это отовсюду:
app-off-keep = Оставить копию на этом компьютере
app-off-keep-detail = Включить снова можно мгновенно
app-off-remove = Удалить копию с этого компьютера
app-off-remove-detail = В ваших аккаунтах ничего не изменится, а при повторном включении всё загрузится снова. То, что есть только на этом компьютере или ещё не отправлено, останется.
app-off-cancel = Отмена
app-off-confirm = Отключить
app-off-done = { $app }: отключено
app-off-note = { $app }: отключено
app-off-turn-on = Включить
app-off-leaves-calendar-rail = Значок на боковой панели и Ctrl+2
app-off-leaves-calendar-agenda = Повестка рядом с почтой
app-off-leaves-calendar-meeting = «Запланировать встречу» и «Открыть в Календаре» в приглашениях
app-off-leaves-calendar-reminders = Напоминания о мероприятиях
app-off-leaves-calendar-desktop = Мероприятия в KRunner и на часах рабочего стола
app-off-leaves-contacts-rail = Значок на боковой панели и Ctrl+3
app-off-leaves-contacts-card = «Добавить в контакты» в карточке отправителя
app-off-leaves-contacts-birthdays = Дни рождения в Календаре
app-off-leaves-tasks-rail = Значок на боковой панели и Ctrl+4
app-off-leaves-tasks-mail = «Добавить в Задачи» в письмах и Shift+T
app-off-leaves-tasks-calendar = Задачи в Календаре
app-off-leaves-tasks-tray = «Новая задача» в системном трее и Meta+Alt+T
app-off-leaves-tasks-reminders = Напоминания о задачах
app-off-leaves-notes-rail = Значок на боковой панели и Ctrl+5
app-off-leaves-notes-mail = Заметки к письмам
app-off-leaves-notes-meetings = Заметки о встречах в мероприятиях
app-off-leaves-notes-tray = «Новая заметка» в системном трее и Meta+Alt+N
app-off-leaves-notes-reminders = Напоминания о заметках
app-off-leaves-files-rail = Значок на боковой панели и Ctrl+7
app-off-leaves-files-compose = Файлы при прикреплении в новом письме

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Скоро
app-calendar-promise = Ваши календари CalDAV, приглашения на встречи из почты и напоминания — рядом с входящими.
app-tasks-promise = Списки задач с синхронизацией по CalDAV и задачи, созданные из писем.
app-notes-promise = Быстрые заметки и заметки к письму или цепочке на потом.

## Contacts page

app-contacts-loading = Собираем людей из вашей почты…
app-contacts-empty = Здесь появятся люди, с которыми вы переписываетесь.
app-contacts-count = { $count ->
    [one] { $count } человек из вашей почты, сначала самые частые собеседники
    [few] { $count } человека из вашей почты, сначала самые частые собеседники
    [many] { $count } человек из вашей почты, сначала самые частые собеседники
   *[other] { $count } человека из вашей почты, сначала самые частые собеседники
}
app-contacts-top = { $count ->
    [one] { $count } самый частый собеседник из вашей почты
    [few] { $count } самых частых собеседника из вашей почты
    [many] { $count } самых частых собеседников из вашей почты
   *[other] { $count } самого частого собеседника из вашей почты
}
app-contacts-messages = { $count ->
    [one] { $count } письмо
    [few] { $count } письма
    [many] { $count } писем
   *[other] { $count } письма
}
app-contacts-last = последнее: { $date }
top-brand = Katna
