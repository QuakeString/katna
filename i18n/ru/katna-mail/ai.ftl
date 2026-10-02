# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
compose-ai-rephrase-tip = Перефразировать (Ctrl+J)
compose-ai-tone-clearer = Яснее
compose-ai-tone-shorter = Короче
compose-ai-tone-friendlier = Дружелюбнее
compose-ai-tone-formal = Официально
compose-ai-tone-grammar = Исправить грамматику
compose-ai-tone-longer = Длиннее
compose-ai-custom = Сказать, как…
compose-ai-more = Другие варианты
compose-ai-replace = Заменить
compose-ai-again = Ещё раз
compose-ai-below = Добавить ниже
compose-ai-copy = Копировать
compose-ai-cancel = Отмена
compose-ai-rephrase = Перефразировать
compose-ai-replaced = Перефразировано
compose-ai-added = Добавлено ниже
compose-ai-copied = Скопировано
compose-ai-katna = Katna AI
compose-ai-own = Ваш ИИ-сервис
compose-ai-trial-left = { $service } · { $days ->
    [one] остался { $days } бесплатный день
    [few] осталось { $days } бесплатных дня
    [many] осталось { $days } бесплатных дней
   *[other] осталось { $days } бесплатного дня
}
compose-ai-encrypted = Это письмо будет зашифровано. При перефразировании выделенный текст отправляется в { $service } без шифрования. Всё равно перефразировать?
compose-ai-sign-in = Для Katna AI нужен аккаунт Katna. Войдите, чтобы пользоваться им, или используйте свой ключ.
compose-ai-pay = Бесплатный месяц Katna AI закончился. Дальше — $5 в месяц, или используйте свой ключ.
compose-ai-too-many = Сейчас слишком много запросов. Попробуйте чуть позже.
compose-ai-no-key = Чтобы перефразировать, добавьте ключ { $service } в настройках.
compose-ai-bad-key = { $service } не принял ваш ключ. Проверьте его в настройках.
compose-ai-off = Помощь ИИ при написании выключена в настройках.
compose-ai-failed = Не удалось связаться с { $service }. Попробуйте ещё раз.
compose-ai-try-again = Повторить
compose-ai-open-settings = Открыть настройки
compose-ai-write-reply-tip = Написать ответ (Ctrl+J)
compose-ai-write-note-tip = Написать заметку (Ctrl+J)
compose-ai-rephrase-empty-tip = Введите текст, чтобы перефразировать
compose-ai-write-reply = Написать ответ
compose-ai-write-note = Написать заметку
compose-ai-write-from = { $count ->
    [one] по { $count } письму
    [few] по { $count } письмам
    [many] по { $count } письмам
   *[other] по { $count } письмам
}
compose-ai-write-ideas = Идеи из переписки
compose-ai-write-own = Или скажите, о чём написать…
compose-ai-write-short = Коротко
compose-ai-write-longer = Подробнее
compose-ai-write-friendly = Дружелюбно
compose-ai-write-formal = Официально
compose-ai-write-insert = Вставить
compose-ai-write-back = Другие идеи
compose-ai-written = Черновик добавлен
compose-ai-write-encrypted = Эта цепочка зашифрована. Чтобы написать ответ, её письма отправляются в { $service } без шифрования. Всё равно написать?
compose-ai-write-anyway = Написать
compose-ai-write-encrypted-off = Эта цепочка зашифрована, а в настройках помощь при написании для зашифрованной почты отключена.
compose-ai-subject-tip = Перефразировать тему
compose-ai-subject-title = Другие формулировки
compose-ai-subject-done = Тема изменена

## Summing up a conversation: the list's right-click menu, the reading

## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = Кратко изложить
summary-hide = Скрыть сводку
summary-close = Закрыть
summary-fold = Свернуть
summary-title = Сводка
summary-mails = { $count ->
    [one] { $count } письмо
    [few] { $count } письма
    [many] { $count } писем
   *[other] { $count } письма
}
summary-of-mails = { $count } из { $total } писем
summary-peek-count = { $mails ->
    [one] { $mails } письмо
    [few] { $mails } письма
    [many] { $mails } писем
   *[other] { $mails } письма
} · { $people ->
    [one] { $people } человек
    [few] { $people } человека
    [many] { $people } человек
   *[other] { $people } человека
}
summary-catch-up = { $count ->
    [one] { $count } новое с прошлого прочтения
    [few] { $count } новых с прошлого прочтения
    [many] { $count } новых с прошлого прочтения
   *[other] { $count } новых с прошлого прочтения
}
summary-strip-newer = { $count ->
    [one] { $count } новое с тех пор · { $gist }
    [few] { $count } новых с тех пор · { $gist }
    [many] { $count } новых с тех пор · { $gist }
   *[other] { $count } новых с тех пор · { $gist }
}
summary-add-new = { $count ->
    [one] Добавить { $count } новое
    [few] Добавить { $count } новых
    [many] Добавить { $count } новых
   *[other] Добавить { $count } новых
}
summary-point-settled = Решено
summary-point-money = Деньги
summary-point-dates = Даты
summary-point-next = Дальше
summary-point-open = Открытые вопросы
summary-files = Файлы
summary-for-you = Для вас
summary-from-mail = { $name }, { $date }
summary-you = Вы
summary-made = { $service } · { $time }
summary-not-read = { $service } · не отмечено как прочитанное
summary-copy = Копировать
summary-copied = Сводка скопирована
summary-again = Изложить заново
summary-open = Открыть
summary-open-tip = Открыть цепочку
summary-reply = Ответить
summary-reply-tip = Написать ответ с помощью ИИ
summary-reply-to = Ответ для { $name }
summary-reply-summary = Сводка
summary-reply-send = Отправить
summary-reply-open = Открыть
summary-asking = Запрос к { $service }…
summary-stop = Остановить
summary-cancel = Отмена
summary-send = Отправить и изложить
summary-ask-short = Ждём вашего согласия
summary-encrypted = Эта цепочка зашифрована. Для краткого изложения её текст отправляется в { $service } без шифрования.
summary-encrypted-off = Эта цепочка зашифрована, а в настройках помощь при написании для зашифрованной почты отключена.
summary-try-again = Повторить
summary-open-settings = Открыть настройки
