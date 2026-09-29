# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } новое письмо
    [few] { $count } новых письма
    [many] { $count } новых писем
   *[other] { $count } нового письма
}
notify-and-more = и ещё { $count }
notify-no-subject = (без темы)
notify-unknown-sender = Неизвестный отправитель
notify-snooze-back = Отложенная почта вернулась
notify-no-reply = Ответа пока нет
notify-no-reply-to = Никто не ответил на «{ $subject }».
notify-tracking-opened = Письмо «{ $subject }» открыто: { $who }
notify-tracking-clicked = Переход по ссылке в «{ $subject }»: { $who }

## Its buttons

notify-update-ready = Katna Mail можно обновить
notify-update-ready-body = Версия { $version } загружена. Обновление установит её и перезапустит Katna Mail.
notify-update = Обновить
notify-event-now = Сейчас
notify-event-in-minutes = { $count ->
    [one] Через { $count } минуту
    [few] Через { $count } минуты
    [many] Через { $count } минут
   *[other] Через { $count } минуты
}
notify-event-in-hours = { $count ->
    [one] Через { $count } час
    [few] Через { $count } часа
    [many] Через { $count } часов
   *[other] Через { $count } часа
}
notify-event-in-days = { $count ->
    [1] Завтра
    [one] Через { $count } день
    [few] Через { $count } дня
    [many] Через { $count } дней
   *[other] Через { $count } дня
}
notify-event-all-day = Весь день
notify-event-join = Присоединиться
notify-event-snooze = Отложить на 5 мин
notify-task-done = Отметить как выполненное
notify-open = Открыть
notify-reply-all = Ответить всем
notify-mark-read = Отметить как прочитанное
notify-mark-all-read = Отметить все как прочитанные
notify-archive = Архивировать
