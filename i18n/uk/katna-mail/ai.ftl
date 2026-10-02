# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
compose-ai-rephrase-tip = Перефразувати (Ctrl+J)
compose-ai-tone-clearer = Зрозуміліше
compose-ai-tone-shorter = Коротше
compose-ai-tone-friendlier = Дружніше
compose-ai-tone-formal = Офіційно
compose-ai-tone-grammar = Виправити граматику
compose-ai-tone-longer = Довше
compose-ai-custom = Сказати як…
compose-ai-more = Інші варіанти
compose-ai-replace = Замінити
compose-ai-again = Ще раз
compose-ai-below = Додати нижче
compose-ai-copy = Копіювати
compose-ai-cancel = Скасувати
compose-ai-rephrase = Перефразувати
compose-ai-replaced = Перефразовано
compose-ai-added = Додано нижче
compose-ai-copied = Скопійовано
compose-ai-katna = Katna AI
compose-ai-own = Ваш сервіс ШІ
compose-ai-trial-left = { $service } · { $days ->
    [one] залишився { $days } безкоштовний день
    [few] залишилося { $days } безкоштовні дні
    [many] залишилося { $days } безкоштовних днів
   *[other] залишилося { $days } безкоштовного дня
}
compose-ai-encrypted = Цей лист буде зашифровано. Перефразування надсилає виділений текст до { $service } без шифрування. Усе одно перефразувати?
compose-ai-sign-in = Для Katna AI потрібен обліковий запис Katna. Увійдіть, щоб скористатися ним, або використайте власний ключ.
compose-ai-pay = Ваш безкоштовний місяць Katna AI закінчився. Далі — $5 на місяць, або можна використати власний ключ.
compose-ai-too-many = Забагато запитів. Спробуйте трохи згодом.
compose-ai-no-key = Додайте ключ { $service } у налаштуваннях, щоб перефразовувати.
compose-ai-bad-key = { $service } не прийняв ваш ключ. Перевірте його в налаштуваннях.
compose-ai-off = Допомогу з письмом за допомогою ШІ вимкнено в налаштуваннях.
compose-ai-failed = Не вдалося зв’язатися з { $service }. Спробуйте ще раз.
compose-ai-try-again = Спробувати ще раз
compose-ai-open-settings = Відкрити налаштування
compose-ai-write-reply-tip = Написати відповідь (Ctrl+J)
compose-ai-write-note-tip = Написати нотатку (Ctrl+J)
compose-ai-rephrase-empty-tip = Введіть текст, щоб перефразувати
compose-ai-write-reply = Написати відповідь
compose-ai-write-note = Написати нотатку
compose-ai-write-from = { $count ->
    [one] з { $count } листа
    [few] з { $count } листів
    [many] з { $count } листів
   *[other] з { $count } листа
}
compose-ai-write-ideas = Ідеї з ланцюжка
compose-ai-write-own = Або скажіть, про що написати…
compose-ai-write-short = Коротко
compose-ai-write-longer = Довше
compose-ai-write-friendly = Дружньо
compose-ai-write-formal = Офіційно
compose-ai-write-insert = Вставити
compose-ai-write-back = Інші ідеї
compose-ai-written = Чернетку додано
compose-ai-write-encrypted = Цей ланцюжок зашифровано. Щоб написати відповідь, його листи буде надіслано до { $service } без шифрування. Усе одно написати?
compose-ai-write-anyway = Написати
compose-ai-write-encrypted-off = Цей ланцюжок зашифровано, а налаштування не дозволяють допомогу з письмом у зашифрованій пошті.
compose-ai-subject-tip = Перефразувати тему
compose-ai-subject-title = Інші варіанти
compose-ai-subject-done = Тему змінено

## Summing up a conversation: the list's right-click menu, the reading

## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = Підсумувати
summary-hide = Сховати підсумок
summary-close = Закрити
summary-fold = Згорнути
summary-title = Підсумок
summary-mails = { $count ->
    [one] { $count } лист
    [few] { $count } листи
    [many] { $count } листів
   *[other] { $count } листа
}
summary-of-mails = Листів: { $count } із { $total }
summary-peek-count = { $mails ->
    [one] { $mails } лист
    [few] { $mails } листи
    [many] { $mails } листів
   *[other] { $mails } листа
} · { $people ->
    [one] { $people } людина
    [few] { $people } людини
    [many] { $people } людей
   *[other] { $people } людини
}
summary-catch-up = { $count ->
    [one] { $count } новий відтоді, як ви читали
    [few] { $count } нові відтоді, як ви читали
    [many] { $count } нових відтоді, як ви читали
   *[other] { $count } нового відтоді, як ви читали
}
summary-strip-newer = { $count ->
    [one] { $count } новий відтоді · { $gist }
    [few] { $count } нові відтоді · { $gist }
    [many] { $count } нових відтоді · { $gist }
   *[other] { $count } нового відтоді · { $gist }
}
summary-add-new = { $count ->
    [one] Додати { $count } новий
    [few] Додати { $count } нові
    [many] Додати { $count } нових
   *[other] Додати { $count } нового
}
summary-point-settled = Вирішено
summary-point-money = Гроші
summary-point-dates = Дати
summary-point-next = Далі
summary-point-open = Відкриті питання
summary-files = Файли
summary-for-you = Для вас
summary-from-mail = { $name }, { $date }
summary-you = Ви
summary-made = { $service } · { $time }
summary-not-read = { $service } · не позначено як прочитане
summary-copy = Копіювати
summary-copied = Підсумок скопійовано
summary-again = Підсумувати ще раз
summary-open = Відкрити
summary-open-tip = Відкрити ланцюжок
summary-reply = Відповісти
summary-reply-tip = Написати відповідь за допомогою ШІ
summary-reply-to = Відповідь для { $name }
summary-reply-summary = Підсумок
summary-reply-send = Надіслати
summary-reply-open = Відкрити
summary-asking = Запит до { $service }…
summary-stop = Зупинити
summary-cancel = Скасувати
summary-send = Надіслати й підсумувати
summary-ask-short = Чекаємо вашої згоди
summary-encrypted = Цей ланцюжок зашифровано. Для підсумку його текст буде надіслано до { $service } без шифрування.
summary-encrypted-off = Цей ланцюжок зашифровано, а налаштування не дозволяють допомогу з письмом у зашифрованій пошті.
summary-try-again = Спробувати ще раз
summary-open-settings = Відкрити налаштування
