# Katna Mail, Ukrainian (Українська): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Нотатки
notes-view-reminders = Нагадування
notes-view-archive = Архів
notes-view-trash = Кошик
notes-edit-labels = Редагувати мітки
notes-search = Пошук нотаток
notes-loading = Відкриваємо ваші нотатки…

## Board

notes-take-a-note = Створити нотатку…
notes-new-list = Новий список
notes-new-note = Нова нотатка
notes-pinned = Закріплені
notes-others = Інші
notes-empty = Тут з’являтимуться ваші нотатки
notes-archive-empty = Тут з’являтимуться ваші архівовані нотатки
notes-trash-empty = У кошику немає нотаток
notes-none-found = Немає відповідних нотаток
notes-label-empty = Нотаток із цією міткою ще немає
notes-reminders-empty = Тут з’являться нотатки з майбутніми нагадуваннями
notes-trash-note = Нотатки в кошику видаляються через 7 днів.
notes-empty-trash = Очистити кошик
notes-ticked = { $count ->
    [one] + { $count } позначений пункт
    [few] + { $count } позначені пункти
    [many] + { $count } позначених пунктів
   *[other] + { $count } позначеного пункту
}
notes-select = Вибрати нотатку
notes-selected = { $count ->
    [one] Вибрано { $count }
    [few] Вибрано { $count }
    [many] Вибрано { $count }
   *[other] Вибрано { $count }
}
notes-select-clear = Скасувати вибір

## A note's buttons

notes-pin = Закріпити нотатку
notes-unpin = Відкріпити нотатку
notes-archive = Архівувати
notes-unarchive = Розархівувати
notes-delete = Видалити нотатку
notes-restore = Відновити
notes-delete-forever = Видалити назавжди
notes-color = Колір фону
notes-checkboxes = Показати або сховати прапорці
notes-labels = Мітки
notes-close = Закрити
notes-more = Більше
notes-make-copy = Створити копію
notes-remind = Нагадати мені
notes-add-picture = Додати зображення
notes-history = Історія версій
notes-ai = Допомогти написати
notes-send-as-mail = Надіслати листом
notes-save-markdown = Зберегти як Markdown
notes-save-pdf = Зберегти як PDF

## The open note

notes-title = Назва
notes-edited = Змінено: { $date }
notes-on-this-computer = На цьому комп’ютері
notes-where = Де зберігається ця нотатка
notes-untitled = Нотатка без назви
notes-picture-choose = Додати зображення
notes-picture-remove = Прибрати зображення
notes-picture-too-big = До нотатки можна додати зображення розміром до { $size }
notes-picture-kind = Цей файл не є зображенням, яке Katna може показати
notes-picture-unreadable = Не вдалося прочитати { $name }: { $error }
notes-remind-me = Нагадати мені
notes-remind-off = Прибрати нагадування
notes-remind-in-the-past = Виберіть час, який ще не минув
notes-remind-today = Сьогодні, { $time }
notes-remind-tomorrow = Завтра, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Нагадування встановлено на { $when }
notes-reminder-off = Нагадування прибрано
notes-link-note = Пов’язати нотатку
notes-link-new = Нова нотатка «{ $title }»
notes-linked-from = Посилання з
notes-link-gone = Цієї нотатки тут більше немає
notes-new-note-gone = Нової нотатки вже немає.
notes-versions = Версії
notes-version-now = Зараз
notes-version-here = Ви, на цьому комп’ютері
notes-version-yesterday = Учора, { $time }
notes-version-changes = { $count ->
    [one] { $count } зміна
    [few] { $count } зміни
    [many] { $count } змін
   *[other] { $count } зміни
}
notes-version-from = З пристрою { $device }
notes-version-elsewhere = З іншого пристрою
notes-version-created = Створено
notes-version-restore = Відновити цю версію
notes-version-restored = Версію відновлено
notes-history-none = Попередніх версій ще немає
notes-ai-tidy = Упорядкувати текст
notes-ai-checklist = Перетворити на список із прапорцями
notes-ai-summarise = Підсумувати
notes-ai-empty = Спершу щось напишіть
notes-ai-tidied = Текст упорядковано. Ctrl+Z поверне як було.
notes-ai-listed = Перетворено на список із прапорцями. Ctrl+Z поверне як було.
notes-ai-summarised = Підсумок додано вгорі

## Labels

notes-label-note = Додати мітку до нотатки
notes-label-name = Введіть назву мітки
notes-label-create = Створити «{ $name }»
notes-label-remove = Вилучити мітку
notes-label-delete = Видалити мітку
notes-labels-none = Міток ще немає. Додайте мітку кнопкою міток у нотатці.
notes-labels-done = Готово
notes-label-renamed = Мітку перейменовано на «{ $name }»
notes-label-deleted = Мітку «{ $name }» видалено

## A note about a mail

notes-mail = Пошта
notes-open-mail = Відкрити лист
notes-open-note = Відкрити нотатку

## Meeting notes

notes-meeting-take = Вести нотатки зустрічі
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Учасники: { $names }
notes-meeting-notes = Нотатки
notes-meeting-actions = Завдання
notes-event = Подія
notes-open-event = Відкрити подію

## Formatting

notes-format = Форматування
notes-format-heading-1 = Заголовок 1
notes-format-heading-2 = Заголовок 2
notes-format-normal = Звичайний текст
notes-format-bold = Жирний
notes-format-italic = Курсив
notes-format-underline = Підкреслений
notes-format-quote = Цитата
notes-format-code = Код
notes-format-divider = Роздільник
notes-format-clear = Очистити форматування

## Tasks

notes-make-task = Зробити завданням

## Colors (tooltips)

notes-color-none = Без кольору
notes-color-coral = Кораловий
notes-color-peach = Персиковий
notes-color-sand = Пісочний
notes-color-mint = М’ятний
notes-color-sage = Шавлія
notes-color-fog = Туман
notes-color-storm = Гроза
notes-color-dusk = Присмерк
notes-color-blossom = Квітка
notes-color-clay = Глина
notes-color-chalk = Крейда

## Messages at the foot of the window

notes-archived = Нотатку архівовано
notes-unarchived = Нотатку розархівовано
notes-trashed = Нотатку переміщено в кошик
notes-restored = Нотатку відновлено
notes-saved = Нотатку збережено
notes-pinned-count = { $count ->
    [one] { $count } нотатку закріплено
    [few] { $count } нотатки закріплено
    [many] { $count } нотаток закріплено
   *[other] { $count } нотатки закріплено
}
notes-unpinned-count = { $count ->
    [one] { $count } нотатку відкріплено
    [few] { $count } нотатки відкріплено
    [many] { $count } нотаток відкріплено
   *[other] { $count } нотатки відкріплено
}
notes-colored-count = { $count ->
    [one] Колір змінено в { $count } нотатці
   *[other] Колір змінено в нотатках: { $count }
}
notes-archived-count = { $count ->
    [one] { $count } нотатку архівовано
    [few] { $count } нотатки архівовано
    [many] { $count } нотаток архівовано
   *[other] { $count } нотатки архівовано
}
notes-unarchived-count = { $count ->
    [one] { $count } нотатку повернуто з архіву
    [few] { $count } нотатки повернуто з архіву
    [many] { $count } нотаток повернуто з архіву
   *[other] { $count } нотатки повернуто з архіву
}
notes-trashed-count = { $count ->
    [one] { $count } нотатку переміщено в кошик
    [few] { $count } нотатки переміщено в кошик
    [many] { $count } нотаток переміщено в кошик
   *[other] { $count } нотатки переміщено в кошик
}
notes-restored-count = { $count ->
    [one] { $count } нотатку відновлено
    [few] { $count } нотатки відновлено
    [many] { $count } нотаток відновлено
   *[other] { $count } нотатки відновлено
}
notes-copied-count = { $count ->
    [one] Створено { $count } копію
    [few] Створено { $count } копії
    [many] Створено { $count } копій
   *[other] Створено { $count } копії
}
notes-empty-discarded = Порожню нотатку відхилено
notes-mail-gone = Цього листа тут більше немає
notes-deleted-forever = { $count ->
    [one] { $count } нотатку видалено назавжди
    [few] { $count } нотатки видалено назавжди
    [many] { $count } нотаток видалено назавжди
   *[other] { $count } нотатки видалено назавжди
}
