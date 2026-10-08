# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
chat-heading = Чтение
chat-view = Цепочки как чаты
chat-view-detail = Переписка между людьми читается как групповой чат: по пузырю на каждое письмо, только то, что написано, а ваши — справа. Рассылки показываются как обычно.
chat-view-switch = Показывать цепочки как чаты
chat-view-switch-detail = Цитаты и подписи скрыты за ··· в каждом пузыре
chat-switch-chat = Чат
chat-switch-mail = Почта
chat-people = { $names } и вы · { $count ->
    [one] { $count } письмо
    [few] { $count } письма
    [many] { $count } писем
   *[other] { $count } письма
}
chat-people-heading = { $count ->
    [one] В этом чате · { $count } человек
    [few] В этом чате · { $count } человека
    [many] В этом чате · { $count } человек
   *[other] В этом чате · { $count } человека
}
chat-member-mails = { $count ->
    [0] Нет писем
    [one] { $count } письмо
    [few] { $count } письма
    [many] { $count } писем
   *[other] { $count } письма
}
chat-today = Сегодня
chat-yesterday = Вчера
chat-added = { $who } добавляет: { $names }
chat-renamed = { $who } меняет тему на «{ $subject }»
chat-you = Вы
chat-not-downloaded = Ещё не загружено
chat-forwarded = Переслано
chat-show-quoted = Показать цитату и подпись
chat-hide-quoted = Скрыть цитату и подпись
chat-hide-dots = Скрыть ···
chat-show-card = Показать карточку
chat-reply-all = Ответить всем
chat-more = Ещё
chat-reply-only = Ответить только { $name }
chat-forward = Переслать
chat-copy-text = Копировать текст
chat-show-as-mail = Показать как письмо
chat-go-down = К самому новому письму
chat-pin = Закрепить сверху
chat-pin-file = Закрепить файл сверху
chat-unpin = Открепить
chat-unpin-file = Открепить файл
chat-pinned-of = Закреплено: { $at } из { $count }
chat-pins-all = Все закреплённые
chat-pins-heading = Закреплено · { $count } из { $most }
chat-pins-drag = Перетащите, чтобы изменить порядок
chat-pin-from-mail = Письмо от { $name } · { $when }
chat-pin-from-file = Файл от { $name } · { $when }
chat-pin-from-text = Текст от { $name } · { $when }
chat-pins-full = В этом чате уже 5 закреплённых
chat-pins-replace-title = Заменить закреплённое
chat-pins-replace-hint = В чате можно закрепить до 5 элементов. Выберите, что открепить.
chat-pins-replace = Заменить
chat-pins-cancel = Отмена
chat-undo = Отменить
chat-reply-to = Ответ для { $names }
chat-send = Отправить (Ctrl+Enter). Щёлкните правой кнопкой или удерживайте для других вариантов
chat-send-now = Отправить сейчас
chat-attach = Прикрепить
chat-attach-photo = Фото
chat-attach-file = Файл
chat-attach-library = Из «Файлов»
chat-attach-template = Шаблон
chat-attach-signature = Подпись
chat-replying-to = Ответ для { $name }
chat-reply-newest = Ответить на последнее письмо

## The attach picker (paperclip > From Files)

picker-title = Прикрепить из «Файлов»
picker-search = Поиск по названиям, людям, темам
picker-search-drive = Поиск на этом диске
picker-mail-files = Файлы из почты
picker-this-chat = Эта цепочка
picker-this-computer = Этот компьютер…
picker-in-chat = В ЭТОЙ ЦЕПОЧКЕ
picker-recent = НЕДАВНИЕ
picker-preview = Предпросмотр
picker-cancel = Отмена
picker-attach = Прикрепить
picker-attach-count = Прикрепить { $count }
picker-selected = Выбрано: { $count }
picker-of-limit = из { $limit }
picker-in-mail = { $size } в самом письме
picker-drive-links = { $count ->
    [one] { $count } как ссылка Google Drive
    [few] { $count } как ссылки Google Drive
    [many] { $count } как ссылки Google Drive
   *[other] { $count } как ссылки Google Drive
}
picker-onedrive-links = { $count ->
    [one] { $count } как ссылка OneDrive
    [few] { $count } как ссылки OneDrive
    [many] { $count } как ссылки OneDrive
   *[other] { $count } как ссылки OneDrive
}
picker-over = { $size } — больше { $limit }, которые вмещает письмо
picker-getting = { $count ->
    [one] Получение файла с диска…
    [few] Получение { $count } файлов с диска…
    [many] Получение { $count } файлов с диска…
   *[other] Получение { $count } файла с диска…
}
picker-some-failed = { $count ->
    [one] Не удалось прочитать { $count } файл
    [few] Не удалось прочитать { $count } файла
    [many] Не удалось прочитать { $count } файлов
   *[other] Не удалось прочитать { $count } файла
}
