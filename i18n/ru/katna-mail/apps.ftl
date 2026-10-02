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
