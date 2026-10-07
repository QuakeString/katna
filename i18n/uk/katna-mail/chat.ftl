# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
chat-heading = Читання
chat-view = Ланцюжки як чати
chat-view-detail = Листування між людьми читається як груповий чат: бульбашка на кожен лист лише з тим, що написано, ваші — праворуч. Розсилки показуються як звичайно.
chat-view-switch = Показувати ланцюжки як чати
chat-view-switch-detail = Цитований лист і підписи сховано за ··· у кожній бульбашці
chat-switch-chat = Чат
chat-switch-mail = Лист
chat-people = { $names } і ви · { $count ->
    [one] { $count } лист
    [few] { $count } листи
    [many] { $count } листів
   *[other] { $count } листа
}
chat-people-heading = { $count ->
    [one] У цьому чаті · { $count } людина
    [few] У цьому чаті · { $count } людини
    [many] У цьому чаті · { $count } людей
   *[other] У цьому чаті · { $count } людини
}
chat-member-mails = { $count ->
    [0] Немає листів
    [one] { $count } лист
    [few] { $count } листи
    [many] { $count } листів
   *[other] { $count } листа
}
chat-today = Сьогодні
chat-yesterday = Учора
chat-added = { $who } додає { $names }
chat-renamed = { $who } змінює тему на «{ $subject }»
chat-you = Ви
chat-not-downloaded = Ще не завантажено
chat-forwarded = Переслано
chat-show-quoted = Показати цитований лист і підпис
chat-hide-quoted = Сховати цитований лист і підпис
chat-hide-dots = Сховати ···
chat-show-card = Показати картку
chat-reply-all = Відповісти всім
chat-more = Більше
chat-reply-only = Відповісти лише { $name }
chat-forward = Переслати
chat-copy-text = Копіювати текст
chat-show-as-mail = Показати як лист
chat-go-down = До найновішого листа
chat-pin = Закріпити вгорі
chat-pin-file = Закріпити файл угорі
chat-unpin = Відкріпити
chat-unpin-file = Відкріпити файл
chat-pinned-of = Закріплено: { $at } із { $count }
chat-pins-all = Усі закріплені
chat-pins-heading = Закріплені · { $count } із { $most }
chat-pins-drag = Перетягніть, щоб змінити порядок
chat-pin-from-mail = Лист від { $name } · { $when }
chat-pin-from-file = Файл від { $name } · { $when }
chat-pin-from-text = Текст від { $name } · { $when }
chat-pins-full = У цьому чаті вже 5 закріплених
chat-pins-replace-title = Замінити закріплене
chat-pins-replace-hint = У чаті може бути до 5 закріплених. Виберіть, яке відкріпити.
chat-pins-replace = Замінити
chat-pins-cancel = Скасувати
chat-undo = Скасувати
chat-reply-to = Відповісти: { $names }
chat-send = Надіслати (Ctrl+Enter). Клацніть правою кнопкою або утримуйте, щоб побачити більше
chat-send-now = Надіслати зараз
chat-attach = Вкласти
chat-attach-photo = Фото
chat-attach-file = Файл
chat-attach-library = З «Файлів»
chat-attach-template = Шаблон
chat-attach-signature = Підпис
chat-replying-to = Відповідь для { $name }
chat-reply-newest = Відповісти на найновіший лист

## The attach picker (paperclip > From Files)

picker-title = Вкласти з «Файлів»
picker-search = Пошук за назвою, людьми, темою
picker-search-drive = Пошук на цьому диску
picker-mail-files = Файли з пошти
picker-this-chat = Цей ланцюжок
picker-this-computer = Цей комп’ютер…
picker-in-chat = У ЦЬОМУ ЛАНЦЮЖКУ
picker-recent = НЕЩОДАВНІ
picker-preview = Попередній перегляд
picker-cancel = Скасувати
picker-attach = Вкласти
picker-attach-count = Вкласти { $count }
picker-selected = Вибрано: { $count }
picker-of-limit = із { $limit }
picker-in-mail = { $size } у листі
picker-drive-links = { $count ->
    [one] { $count } як посилання Google Drive
    [few] { $count } як посилання Google Drive
    [many] { $count } як посилання Google Drive
   *[other] { $count } як посилання Google Drive
}
picker-onedrive-links = { $count ->
    [one] { $count } як посилання OneDrive
    [few] { $count } як посилання OneDrive
    [many] { $count } як посилання OneDrive
   *[other] { $count } як посилання OneDrive
}
picker-over = { $size } — більше за { $limit }, які може вмістити лист
picker-getting = { $count ->
    [one] Отримання файлу з диска…
    [few] Отримання { $count } файлів із диска…
    [many] Отримання { $count } файлів із диска…
   *[other] Отримання { $count } файлу з диска…
}
picker-some-failed = { $count ->
    [one] Не вдалося прочитати { $count } файл
    [few] Не вдалося прочитати { $count } файли
    [many] Не вдалося прочитати { $count } файлів
   *[other] Не вдалося прочитати { $count } файлу
}
