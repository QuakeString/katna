# Katna Mail, Russian (Русский): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Заметки
notes-view-reminders = Напоминания
notes-view-archive = Архив
notes-view-trash = Корзина
notes-edit-labels = Изменить ярлыки
notes-search = Поиск по заметкам
notes-loading = Открываем ваши заметки…

## Board

notes-take-a-note = Создать заметку…
notes-new-list = Новый список
notes-new-note = Новая заметка
notes-pinned = Закреплённые
notes-others = Другие
notes-empty = Здесь будут ваши заметки
notes-archive-empty = Здесь будут ваши заметки из архива
notes-trash-empty = В корзине нет заметок
notes-none-found = Подходящих заметок нет
notes-label-empty = Заметок с этим ярлыком пока нет
notes-reminders-empty = Здесь появятся заметки с предстоящими напоминаниями
notes-trash-note = Заметки из корзины удаляются через 7 дней.
notes-empty-trash = Очистить корзину
notes-ticked = { $count ->
    [one] + { $count } отмеченный пункт
    [few] + { $count } отмеченных пункта
    [many] + { $count } отмеченных пунктов
   *[other] + { $count } отмеченного пункта
}
notes-select = Выбрать заметку
notes-selected = { $count ->
    [one] Выбрана { $count }
    [few] Выбрано { $count }
    [many] Выбрано { $count }
   *[other] Выбрано { $count }
}
notes-select-clear = Снять выделение

## A note's buttons

notes-pin = Закрепить заметку
notes-unpin = Открепить заметку
notes-archive = Архивировать
notes-unarchive = Вернуть из архива
notes-delete = Удалить заметку
notes-restore = Восстановить
notes-delete-forever = Удалить навсегда
notes-color = Цвет фона
notes-checkboxes = Показать или скрыть флажки
notes-labels = Ярлыки
notes-close = Закрыть
notes-more = Ещё
notes-make-copy = Создать копию
notes-remind = Напомнить
notes-add-picture = Добавить изображение
notes-history = История версий
notes-ai = Помочь написать
notes-send-as-mail = Отправить письмом
notes-save-markdown = Сохранить как Markdown
notes-save-pdf = Сохранить как PDF

## The open note

notes-title = Название
notes-edited = Изменено: { $date }
notes-on-this-computer = На этом компьютере
notes-where = Где хранится эта заметка
notes-untitled = Заметка без названия
notes-picture-choose = Добавить изображения
notes-picture-remove = Удалить изображение
notes-picture-too-big = В заметку можно добавить изображения размером до { $size }
notes-picture-kind = Этот файл — не изображение, которое Katna может показать
notes-picture-unreadable = Не удалось прочитать { $name }: { $error }
notes-remind-me = Напомнить
notes-remind-off = Удалить напоминание
notes-remind-in-the-past = Выберите время, которое ещё не прошло
notes-remind-today = Сегодня, { $time }
notes-remind-tomorrow = Завтра, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Напоминание: { $when }
notes-reminder-off = Напоминание удалено
notes-link-note = Связать с заметкой
notes-link-new = Новая заметка «{ $title }»
notes-linked-from = Ссылаются сюда
notes-link-gone = Этой заметки больше нет
notes-versions = Версии
notes-version-now = Сейчас
notes-version-here = Вы, на этом компьютере
notes-version-yesterday = Вчера, { $time }
notes-version-changes = { $count ->
    [one] { $count } изменение
    [few] { $count } изменения
    [many] { $count } изменений
   *[other] { $count } изменения
}
notes-version-from = С устройства { $device }
notes-version-elsewhere = С другого устройства
notes-version-created = Создана
notes-version-restore = Восстановить эту версию
notes-version-restored = Версия восстановлена
notes-history-none = Более ранних версий пока нет
notes-ai-tidy = Привести текст в порядок
notes-ai-checklist = Превратить в список дел
notes-ai-summarise = Кратко изложить
notes-ai-empty = Сначала напишите что-нибудь
notes-ai-tidied = Текст приведён в порядок. Ctrl+Z вернёт как было.
notes-ai-listed = Превращено в список дел. Ctrl+Z вернёт как было.
notes-ai-summarised = Краткое изложение добавлено в начало

## Labels

notes-label-note = Добавить ярлык к заметке
notes-label-name = Введите название ярлыка
notes-label-create = Создать «{ $name }»
notes-label-remove = Удалить ярлык с заметки
notes-label-delete = Удалить ярлык
notes-labels-none = Ярлыков пока нет. Добавьте ярлык кнопкой ярлыков в заметке.
notes-labels-done = Готово
notes-label-renamed = Ярлык переименован в «{ $name }»
notes-label-deleted = Ярлык «{ $name }» удалён

## A note about a mail

notes-mail = Почта
notes-open-mail = Открыть письмо
notes-open-note = Открыть заметку

## Meeting notes

notes-meeting-take = Вести заметки о встрече
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Участники: { $names }
notes-meeting-notes = Заметки
notes-meeting-actions = Задачи
notes-event = Мероприятие
notes-open-event = Открыть мероприятие

## Formatting

notes-format = Форматирование
notes-format-heading-1 = Заголовок 1
notes-format-heading-2 = Заголовок 2
notes-format-normal = Обычный текст
notes-format-bold = Полужирный
notes-format-italic = Курсив
notes-format-underline = Подчёркнутый
notes-format-quote = Цитата
notes-format-code = Код
notes-format-divider = Разделитель
notes-format-clear = Очистить форматирование

## Tasks

notes-make-task = Сделать задачей

## Colors (tooltips)

notes-color-none = Без цвета
notes-color-coral = Коралловый
notes-color-peach = Персиковый
notes-color-sand = Песочный
notes-color-mint = Мятный
notes-color-sage = Шалфей
notes-color-fog = Туман
notes-color-storm = Гроза
notes-color-dusk = Сумерки
notes-color-blossom = Цветок
notes-color-clay = Глина
notes-color-chalk = Мел

## Messages at the foot of the window

notes-archived = Заметка архивирована
notes-unarchived = Заметка возвращена из архива
notes-trashed = Заметка перемещена в корзину
notes-restored = Заметка восстановлена
notes-saved = Заметка сохранена
notes-pinned-count = { $count ->
    [one] Заметка закреплена
    [few] { $count } заметки закреплены
    [many] { $count } заметок закреплено
   *[other] { $count } заметки закреплено
}
notes-unpinned-count = { $count ->
    [one] Заметка откреплена
    [few] { $count } заметки откреплены
    [many] { $count } заметок откреплено
   *[other] { $count } заметки откреплено
}
notes-colored-count = { $count ->
    [one] Цвет изменён
    [few] Цвет изменён у { $count } заметок
    [many] Цвет изменён у { $count } заметок
   *[other] Цвет изменён у { $count } заметки
}
notes-archived-count = { $count ->
    [one] Заметка перемещена в архив
    [few] { $count } заметки перемещены в архив
    [many] { $count } заметок перемещено в архив
   *[other] { $count } заметки перемещено в архив
}
notes-unarchived-count = { $count ->
    [one] Заметка возвращена из архива
    [few] { $count } заметки возвращены из архива
    [many] { $count } заметок возвращено из архива
   *[other] { $count } заметки возвращено из архива
}
notes-trashed-count = { $count ->
    [one] Заметка перемещена в корзину
    [few] { $count } заметки перемещены в корзину
    [many] { $count } заметок перемещено в корзину
   *[other] { $count } заметки перемещено в корзину
}
notes-restored-count = { $count ->
    [one] Заметка восстановлена
    [few] { $count } заметки восстановлены
    [many] { $count } заметок восстановлено
   *[other] { $count } заметки восстановлено
}
notes-copied-count = { $count ->
    [one] Копия создана
    [few] Создано { $count } копии
    [many] Создано { $count } копий
   *[other] Создано { $count } копии
}
notes-empty-discarded = Пустая заметка удалена
notes-mail-gone = Этого письма больше нет
notes-deleted-forever = { $count ->
    [one] { $count } заметка удалена навсегда
    [few] { $count } заметки удалены навсегда
    [many] { $count } заметок удалено навсегда
   *[other] { $count } заметки удалено навсегда
}
