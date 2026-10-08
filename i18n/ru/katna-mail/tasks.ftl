# Katna Mail, Russian (Русский): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Новая задача
tasks-all = Все задачи
tasks-today = Сегодня
tasks-upcoming = Предстоящие
tasks-starred = Помеченные
tasks-completed-view = Выполненные
tasks-new-list = Создать новый список
tasks-labels-heading = Ярлыки
tasks-on-this-computer = На этом компьютере
tasks-my-tasks = Мои задачи
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Войдите снова, чтобы показать задачи
tasks-account-signed-in = Вход в { $address } выполнен снова. Загрузка задач…
tasks-account-sign-in-refused = { $provider } не впустил Katna. Попробуйте снова и разрешите доступ к задачам.
tasks-account-refused = Сервер не принял пароль. Для Yahoo, iCloud, Zoho и других нужен пароль приложения.
tasks-account-change-password = Сменить пароль
tasks-account-change-password-tooltip = Введите новый пароль; Katna проверит его на сервере
tasks-account-not-enabled = Доступ Katna к задачам ещё не включён.
tasks-account-failed = Не удалось прочитать списки задач.
# $reason is the server's own words, in English.
tasks-account-error = Не удалось прочитать списки задач: { $reason }
tasks-account-none = Списки задач не найдены
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Списки задач не найдены: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } показывает задачи только Katna, вошедшей через { $provider }.
tasks-account-sign-in-with = Войти через { $provider }
tasks-account-looking = Поиск списков задач…
tasks-account-try-again = Повторить попытку
tasks-account-try-again-tooltip = Сейчас снова проверить задачи этого аккаунта
tasks-account-fixing = Исправляем…
tasks-list-name-placeholder = Название списка

## Lists and tasks

tasks-loading = Чтение ваших задач…
tasks-no-lists = Здесь появятся ваши списки задач.
tasks-search = Поиск задач
tasks-search-none = Нет задач, подходящих под запрос.
tasks-add = Добавить задачу
tasks-title-placeholder = Название
tasks-add-step = Добавить подзадачу
tasks-empty = Задач пока нет. Добавьте одну выше.
tasks-starred-empty = Пометьте задачу, чтобы увидеть её здесь.
tasks-label-empty = Нет открытых задач с этим ярлыком.
tasks-today-empty = Сегодня нет задач со сроком.
tasks-completed-empty = Здесь появятся выполненные задачи.
tasks-upcoming-add = Добавить задачу на { $day }
tasks-upcoming-overdue-day = { $weekday }, { $day }
tasks-from-mail-quiet = Из письма
tasks-from-note-quiet = Из заметки
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Просроченные
tasks-completed = { $count ->
    [one] Выполненные ({ $count })
    [few] Выполненные ({ $count })
    [many] Выполненные ({ $count })
   *[other] Выполненные ({ $count })
}
tasks-list-options = Параметры списка
tasks-sort-by = Сортировка
tasks-sort-my-order = Мой порядок
tasks-sort-date = Дата
tasks-sort-starred = Недавно помеченные
tasks-sort-title = Название
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
tasks-from-note = Заметка
tasks-open-note = Открыть заметку
tasks-note-gone = Этой заметки здесь больше нет.
tasks-no-subject = (без темы)
tasks-selected = { $count ->
    [one] Выбрана { $count }
    [few] Выбрано { $count }
    [many] Выбрано { $count }
   *[other] Выбрано { $count }
}
tasks-select-clear = Снять выделение
tasks-select-move = Переместить в список
tasks-select-date = Задать дату
tasks-next-week = На следующей неделе

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
tasks-remind = Напомнить
tasks-remind-off = Не напоминать
tasks-remind-on-time = В момент задачи
tasks-remind-morning = В этот день, { $time }
tasks-remind-hour-before = За час
tasks-remind-day-before = За день
tasks-label-add = Добавить ярлык
tasks-label-task = Добавить ярлык к задаче
tasks-files-attach = Прикрепить файлы
tasks-files-pick = Прикрепить
tasks-file-open = Открыть
tasks-file-remove = Удалить файл
tasks-file-here = Только на этом компьютере
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
tasks-toast-next = Готово. Следующая: { $date }
tasks-toast-deleted = Задача удалена
tasks-files-added = { $count ->
    [one] Файл прикреплён
    [few] Прикреплено { $count } файла
    [many] Прикреплено { $count } файлов
   *[other] Прикреплено { $count } файла
}
tasks-file-removed = «{ $name }» удалён
tasks-files-left-out = Не прикреплено: { $names }. К задаче можно прикрепить файлы размером до { $limit }, но не папки.
tasks-file-missing = Этого файла здесь больше нет.
tasks-toast-added = { $count ->
    [one] { $count } задача добавлена
    [few] { $count } задачи добавлены
    [many] { $count } задач добавлено
   *[other] { $count } задачи добавлено
}
tasks-mail-gone = Этого письма больше нет.
tasks-toast-list-deleted = Список удалён
tasks-toast-moved = Перемещено в { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Задача перемещена
tasks-toast-rescheduled = Задача перенесена
tasks-toast-rescheduled-several = { $count ->
    [one] Срок задачи перенесён
    [few] Срок { $count } задач перенесён
    [many] Срок { $count } задач перенесён
   *[other] Срок { $count } задачи перенесён
}
tasks-toast-done-several = { $count ->
    [one] Задача выполнена
    [few] { $count } задачи выполнены
    [many] { $count } задач выполнено
   *[other] { $count } задачи выполнено
}
tasks-toast-open-several = { $count ->
    [one] Задача отмечена как невыполненная
    [few] { $count } задачи отмечены как невыполненные
    [many] { $count } задач отмечено как невыполненные
   *[other] { $count } задачи отмечено как невыполненные
}
tasks-toast-starred = { $count ->
    [one] Задача помечена
    [few] { $count } задачи помечены
    [many] { $count } задач помечено
   *[other] { $count } задачи помечено
}
tasks-toast-unstarred = { $count ->
    [one] Пометка снята
    [few] Пометка снята с { $count } задач
    [many] Пометка снята с { $count } задач
   *[other] Пометка снята с { $count } задачи
}
tasks-toast-deleted-several = { $count ->
    [one] Задача удалена
    [few] { $count } задачи удалены
    [many] { $count } задач удалено
   *[other] { $count } задачи удалено
}
