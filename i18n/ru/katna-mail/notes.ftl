# Katna Mail, Russian (Русский): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Заметки
notes-view-archive = Архив
notes-view-trash = Корзина
notes-search = Поиск по заметкам
notes-loading = Открываем ваши заметки…

## Board

notes-take-a-note = Создать заметку…
notes-new-list = Новый список
notes-pinned = Закреплённые
notes-others = Другие
notes-empty = Здесь будут ваши заметки
notes-archive-empty = Здесь будут ваши заметки из архива
notes-trash-empty = В корзине нет заметок
notes-none-found = Подходящих заметок нет
notes-trash-note = Заметки из корзины удаляются через 7 дней.
notes-empty-trash = Очистить корзину
notes-ticked = { $count ->
    [one] + { $count } отмеченный пункт
    [few] + { $count } отмеченных пункта
    [many] + { $count } отмеченных пунктов
   *[other] + { $count } отмеченного пункта
}

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
notes-close = Закрыть

## The open note

notes-title = Название
notes-edited = Изменено: { $date }
notes-on-this-computer = На этом компьютере
notes-where = Где хранится эта заметка

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
notes-empty-discarded = Пустая заметка удалена
notes-deleted-forever = { $count ->
    [one] { $count } заметка удалена навсегда
    [few] { $count } заметки удалены навсегда
    [many] { $count } заметок удалено навсегда
   *[other] { $count } заметки удалено навсегда
}
