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
notify-follow-up-sent = Повторное письмо отправлено
notify-follow-up-sent-to = Никто не ответил на «{ $subject }», поэтому Katna отправила повторное письмо.
notify-follow-up-waiting = Повторное письмо не отправлено
notify-follow-up-waiting-to = Срок наступил, пока компьютер был выключен. «{ $subject }» снова во «Входящих».
notify-tracking-opened = Письмо «{ $subject }» открыто: { $who }
notify-tracking-clicked = Переход по ссылке в «{ $subject }»: { $who }

## Its buttons

notify-update-ready = Katna Mail можно обновить
notify-update-ready-body = Версия { $version } загружена. Обновление установит её и перезапустит Katna Mail.
notify-update = Обновить
notify-signed-out = Войдите снова
notify-signed-out-body = { $provider } завершил сеанс Katna в { $address }. Синхронизация почты остановлена.
notify-sign-in = Войти
notify-password-refused = Пароль не принят
notify-password-refused-body = Почтовый сервер не принял пароль для { $address }. Возможно, он изменился.
notify-new-password = Новый пароль
notify-not-sent = Письмо «{ $subject }» не отправлено
notify-not-sent-no-subject = Письмо не отправлено
notify-not-sent-body = Оно в «Исходящих», там указана причина.
notify-open-outbox = Открыть «Исходящие»
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
notify-peek = Просмотр
notify-reply = Ответить
notify-reply-placeholder = Ответ для { $name }…
notify-send = Отправить
notify-reply-all = Ответить всем
notify-mark-read = Отметить как прочитанное
notify-mark-all-read = Отметить все как прочитанные
notify-archive = Архивировать
notify-snooze-hour = Отложить на 1 час
notify-snooze-tomorrow = Завтра
notify-copy-code = Копировать { $code }
notify-link-verify = Подтвердить на { $domain }
notify-link-confirm = Подтвердить на { $domain }
notify-link-activate = Активировать на { $domain }
notify-archived = В архиве
notify-archived-count = { $count ->
    [one] { $count } письмо убрано из «Входящих»
    [few] { $count } письма убраны из «Входящих»
    [many] { $count } писем убрано из «Входящих»
   *[other] { $count } письма убрано из «Входящих»
}
notify-undo = Отменить
notify-code-copied = Код скопирован
notify-code-not-copied = Не удалось скопировать код
notify-reply-sent = Ответ для { $name } отправлен
notify-open-in-katna = Открыть в Katna
