# Katna Mail, Ukrainian (Українська): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Нотатки
notes-view-archive = Архів
notes-view-trash = Кошик
notes-edit-labels = Редагувати мітки
notes-search = Пошук нотаток
notes-loading = Відкриваємо ваші нотатки…

## Board

notes-take-a-note = Створити нотатку…
notes-new-list = Новий список
notes-pinned = Закріплені
notes-others = Інші
notes-empty = Тут з’являтимуться ваші нотатки
notes-archive-empty = Тут з’являтимуться ваші архівовані нотатки
notes-trash-empty = У кошику немає нотаток
notes-none-found = Немає відповідних нотаток
notes-label-empty = Нотаток із цією міткою ще немає
notes-trash-note = Нотатки в кошику видаляються через 7 днів.
notes-empty-trash = Очистити кошик
notes-ticked = { $count ->
    [one] + { $count } позначений пункт
    [few] + { $count } позначені пункти
    [many] + { $count } позначених пунктів
   *[other] + { $count } позначеного пункту
}

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

## The open note

notes-title = Назва
notes-edited = Змінено: { $date }
notes-on-this-computer = На цьому комп’ютері
notes-where = Де зберігається ця нотатка

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
notes-empty-discarded = Порожню нотатку відхилено
notes-mail-gone = Цього листа тут більше немає
notes-deleted-forever = { $count ->
    [one] { $count } нотатку видалено назавжди
    [few] { $count } нотатки видалено назавжди
    [many] { $count } нотаток видалено назавжди
   *[other] { $count } нотатки видалено назавжди
}
