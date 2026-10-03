# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } новий лист
    [few] { $count } нові листи
    [many] { $count } нових листів
   *[other] { $count } нового листа
}
notify-and-more = і ще { $count }
notify-no-subject = (без теми)
notify-unknown-sender = Невідомий відправник
notify-snooze-back = Повернулося з відкладених
notify-no-reply = Відповіді ще немає
notify-no-reply-to = Ніхто не відповів на «{ $subject }».
notify-tracking-opened = { $who }: лист відкрито { $subject }
notify-tracking-clicked = { $who } переходить за посиланням у листі { $subject }

## Its buttons

notify-update-ready = Katna Mail можна оновити
notify-update-ready-body = Версію { $version } завантажено. Оновлення встановить її й перезапустить Katna Mail.
notify-update = Оновити
notify-event-now = Зараз
notify-event-in-minutes = { $count ->
    [one] Через { $count } хвилину
    [few] Через { $count } хвилини
    [many] Через { $count } хвилин
   *[other] Через { $count } хвилини
}
notify-event-in-hours = { $count ->
    [one] Через { $count } годину
    [few] Через { $count } години
    [many] Через { $count } годин
   *[other] Через { $count } години
}
notify-event-in-days = { $count ->
    [1] Завтра
    [one] Через { $count } день
    [few] Через { $count } дні
    [many] Через { $count } днів
   *[other] Через { $count } дня
}
notify-event-all-day = Увесь день
notify-event-join = Приєднатися
notify-event-snooze = Відкласти на 5 хв
notify-task-done = Позначити як виконане
notify-open = Відкрити
notify-peek = Переглянути
notify-reply = Відповісти
notify-reply-placeholder = Відповідь для { $name }…
notify-send = Надіслати
notify-reply-all = Відповісти всім
notify-mark-read = Позначити як прочитане
notify-mark-all-read = Позначити все як прочитане
notify-archive = Архівувати
notify-archived = Заархівовано
notify-archived-count = { $count ->
    [one] { $count } лист прибрано з «Вхідних»
    [few] { $count } листи прибрано з «Вхідних»
    [many] { $count } листів прибрано з «Вхідних»
   *[other] { $count } листа прибрано з «Вхідних»
}
notify-undo = Скасувати
notify-reply-sent = Відповідь надіслано: { $name }
notify-open-in-katna = Відкрити в Katna
