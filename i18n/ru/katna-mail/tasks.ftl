# Katna Mail, Russian (Русский): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Создать
tasks-all = Все задачи
tasks-starred = Помеченные
tasks-new-list = Создать новый список
tasks-on-this-computer = На этом компьютере
tasks-my-tasks = Мои задачи
tasks-list-name-placeholder = Название списка

## Lists and tasks

tasks-loading = Чтение ваших задач…
tasks-no-lists = Здесь появятся ваши списки задач.
tasks-add = Добавить задачу
tasks-title-placeholder = Название
tasks-add-step = Добавить подзадачу
tasks-empty = Задач пока нет. Добавьте одну выше.
tasks-starred-empty = Пометьте задачу, чтобы увидеть её здесь.
tasks-completed = { $count ->
    [one] Выполненные ({ $count })
    [few] Выполненные ({ $count })
    [many] Выполненные ({ $count })
   *[other] Выполненные ({ $count })
}
tasks-list-options = Параметры списка
tasks-rename-list = Переименовать список
tasks-delete-list = Удалить список
tasks-mark-done = Отметить как выполненную
tasks-mark-open = Отметить как невыполненную
tasks-star = Пометить
tasks-unstar = Снять пометку
tasks-edit-title = Изменить название
tasks-details = Подробности
tasks-delete = Удалить
tasks-move-to = Переместить в { $list }
tasks-from-mail = Почта
tasks-open-mail = Открыть письмо
tasks-no-subject = (без темы)

## The details dialog

tasks-notes-placeholder = Добавить подробности
tasks-date = Дата
tasks-no-date = Без даты
tasks-time-placeholder = Добавить время
tasks-repeat = Повтор
tasks-repeat-never = Не повторяется
tasks-repeat-daily = Каждый день
tasks-repeat-weekly = Каждую неделю
tasks-repeat-monthly = Каждый месяц
tasks-repeat-yearly = Каждый год
tasks-repeat-other = Другое
tasks-cancel = Отмена
tasks-save = Сохранить
tasks-not-a-time = «{ $text }» – не время, например { $example }.

## Due days

tasks-due-today = Сегодня
tasks-due-tomorrow = Завтра
tasks-due-yesterday = Вчера
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Задача выполнена
tasks-toast-deleted = Задача удалена
tasks-toast-added = { $count ->
    [one] { $count } задача добавлена
    [few] { $count } задачи добавлены
    [many] { $count } задач добавлено
   *[other] { $count } задачи добавлено
}
tasks-mail-gone = Этого письма больше нет.
tasks-toast-list-deleted = Список удалён
tasks-toast-moved = Перемещено в { $list }
