# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Несортированные
tab-promotions = Промоакции
tab-social = Соцсети
tab-updates = Оповещения
tab-forums = Форумы
tab-focused = Отсортированные
tab-other = Другие
tab-inbox = Входящие
tab-newsletters = Рассылки
tab-notifications = Уведомления
tab-provider-other = сортирует Katna

## Mail list: toolbar

list-select = Выбрать
list-refresh = Обновить
list-back-to-top = Наверх
list-checking = Проверяем новую почту…
list-more = Ещё
list-mark-read = Отметить как прочитанное
list-mark-unread = Отметить как непрочитанное
list-move-to = Переместить в
list-archive = Архивировать
list-spam = В спам
list-delete = Удалить
list-snooze = Отложить
list-unsnooze = Вернуть сейчас
list-newer = Более новые
list-older = Более старые
list-range = { $first }–{ $last } из { $total }
list-range-about = { $first }–{ $last } из примерно { $total }
list-results = Результаты по запросу «{ $query }»
list-results-corrected = Показаны результаты по запросу «{ $query }»
list-search-instead = Искать вместо этого «{ $query }»
list-search-no-index = Поиск не готов: индекс ещё не построен.
list-search-not-ready = Поиск не готов: { $error }
list-files-more = +{ $count }
list-replied = Вы ответили

## Mail list: Select menu (which lines to tick)

list-pick-all = Все
list-pick-none = Ни одного
list-pick-read = Прочитанные
list-pick-unread = Непрочитанные
list-pick-starred = Помеченные
list-pick-unstarred = Без пометки

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Выбрана { $count } цепочка.
        [few] Выбраны все { $count } цепочки.
        [many] Выбраны все { $count } цепочек.
       *[other] Выбраны все { $count } цепочки.
    }
   *[message] { $count ->
        [one] Выбрано { $count } письмо.
        [few] Выбраны все { $count } письма.
        [many] Выбраны все { $count } писем.
       *[other] Выбраны все { $count } письма.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Выбрана { $count } цепочка в папке «{ $folder }».
        [few] Выбраны все { $count } цепочки в папке «{ $folder }».
        [many] Выбраны все { $count } цепочек в папке «{ $folder }».
       *[other] Выбраны все { $count } цепочки в папке «{ $folder }».
    }
   *[message] { $count ->
        [one] Выбрано { $count } письмо в папке «{ $folder }».
        [few] Выбраны все { $count } письма в папке «{ $folder }».
        [many] Выбраны все { $count } писем в папке «{ $folder }».
       *[other] Выбраны все { $count } письма в папке «{ $folder }».
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Выбрана { $count } цепочка на этой странице.
        [few] Выбраны все { $count } цепочки на этой странице.
        [many] Выбраны все { $count } цепочек на этой странице.
       *[other] Выбраны все { $count } цепочки на этой странице.
    }
   *[message] { $count ->
        [one] Выбрано { $count } письмо на этой странице.
        [few] Выбраны все { $count } письма на этой странице.
        [many] Выбраны все { $count } писем на этой странице.
       *[other] Выбраны все { $count } письма на этой странице.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Выбрать { $count } цепочку
        [few] Выбрать все { $count } цепочки
        [many] Выбрать все { $count } цепочек
       *[other] Выбрать все { $count } цепочки
    }
   *[message] { $count ->
        [one] Выбрать { $count } письмо
        [few] Выбрать все { $count } письма
        [many] Выбрать все { $count } писем
       *[other] Выбрать все { $count } письма
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Выбрать { $count } цепочку в папке «{ $folder }»
        [few] Выбрать все { $count } цепочки в папке «{ $folder }»
        [many] Выбрать все { $count } цепочек в папке «{ $folder }»
       *[other] Выбрать все { $count } цепочки в папке «{ $folder }»
    }
   *[message] { $count ->
        [one] Выбрать { $count } письмо в папке «{ $folder }»
        [few] Выбрать все { $count } письма в папке «{ $folder }»
        [many] Выбрать все { $count } писем в папке «{ $folder }»
       *[other] Выбрать все { $count } письма в папке «{ $folder }»
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } прочитанная цепочка на этой странице.
            [few] Выбраны все { $count } прочитанные цепочки на этой странице.
            [many] Выбраны все { $count } прочитанных цепочек на этой странице.
           *[other] Выбраны все { $count } прочитанные цепочки на этой странице.
        }
       *[message] { $count ->
            [one] Выбрано { $count } прочитанное письмо на этой странице.
            [few] Выбраны все { $count } прочитанных письма на этой странице.
            [many] Выбраны все { $count } прочитанных писем на этой странице.
           *[other] Выбраны все { $count } прочитанных письма на этой странице.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } непрочитанная цепочка на этой странице.
            [few] Выбраны все { $count } непрочитанные цепочки на этой странице.
            [many] Выбраны все { $count } непрочитанных цепочек на этой странице.
           *[other] Выбраны все { $count } непрочитанные цепочки на этой странице.
        }
       *[message] { $count ->
            [one] Выбрано { $count } непрочитанное письмо на этой странице.
            [few] Выбраны все { $count } непрочитанных письма на этой странице.
            [many] Выбраны все { $count } непрочитанных писем на этой странице.
           *[other] Выбраны все { $count } непрочитанных письма на этой странице.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } помеченная цепочка на этой странице.
            [few] Выбраны все { $count } помеченные цепочки на этой странице.
            [many] Выбраны все { $count } помеченных цепочек на этой странице.
           *[other] Выбраны все { $count } помеченные цепочки на этой странице.
        }
       *[message] { $count ->
            [one] Выбрано { $count } помеченное письмо на этой странице.
            [few] Выбраны все { $count } помеченных письма на этой странице.
            [many] Выбраны все { $count } помеченных писем на этой странице.
           *[other] Выбраны все { $count } помеченных письма на этой странице.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } цепочка без пометки на этой странице.
            [few] Выбраны все { $count } цепочки без пометки на этой странице.
            [many] Выбраны все { $count } цепочек без пометки на этой странице.
           *[other] Выбраны все { $count } цепочки без пометки на этой странице.
        }
       *[message] { $count ->
            [one] Выбрано { $count } письмо без пометки на этой странице.
            [few] Выбраны все { $count } письма без пометки на этой странице.
            [many] Выбраны все { $count } писем без пометки на этой странице.
           *[other] Выбраны все { $count } письма без пометки на этой странице.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } прочитанную цепочку
            [few] Выбрать все { $count } прочитанные цепочки
            [many] Выбрать все { $count } прочитанных цепочек
           *[other] Выбрать все { $count } прочитанные цепочки
        }
       *[message] { $count ->
            [one] Выбрать { $count } прочитанное письмо
            [few] Выбрать все { $count } прочитанных письма
            [many] Выбрать все { $count } прочитанных писем
           *[other] Выбрать все { $count } прочитанных письма
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } непрочитанную цепочку
            [few] Выбрать все { $count } непрочитанные цепочки
            [many] Выбрать все { $count } непрочитанных цепочек
           *[other] Выбрать все { $count } непрочитанные цепочки
        }
       *[message] { $count ->
            [one] Выбрать { $count } непрочитанное письмо
            [few] Выбрать все { $count } непрочитанных письма
            [many] Выбрать все { $count } непрочитанных писем
           *[other] Выбрать все { $count } непрочитанных письма
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } помеченную цепочку
            [few] Выбрать все { $count } помеченные цепочки
            [many] Выбрать все { $count } помеченных цепочек
           *[other] Выбрать все { $count } помеченные цепочки
        }
       *[message] { $count ->
            [one] Выбрать { $count } помеченное письмо
            [few] Выбрать все { $count } помеченных письма
            [many] Выбрать все { $count } помеченных писем
           *[other] Выбрать все { $count } помеченных письма
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } цепочку без пометки
            [few] Выбрать все { $count } цепочки без пометки
            [many] Выбрать все { $count } цепочек без пометки
           *[other] Выбрать все { $count } цепочки без пометки
        }
       *[message] { $count ->
            [one] Выбрать { $count } письмо без пометки
            [few] Выбрать все { $count } письма без пометки
            [many] Выбрать все { $count } писем без пометки
           *[other] Выбрать все { $count } письма без пометки
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } прочитанную цепочку в папке «{ $folder }»
            [few] Выбрать все { $count } прочитанные цепочки в папке «{ $folder }»
            [many] Выбрать все { $count } прочитанных цепочек в папке «{ $folder }»
           *[other] Выбрать все { $count } прочитанные цепочки в папке «{ $folder }»
        }
       *[message] { $count ->
            [one] Выбрать { $count } прочитанное письмо в папке «{ $folder }»
            [few] Выбрать все { $count } прочитанных письма в папке «{ $folder }»
            [many] Выбрать все { $count } прочитанных писем в папке «{ $folder }»
           *[other] Выбрать все { $count } прочитанных письма в папке «{ $folder }»
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } непрочитанную цепочку в папке «{ $folder }»
            [few] Выбрать все { $count } непрочитанные цепочки в папке «{ $folder }»
            [many] Выбрать все { $count } непрочитанных цепочек в папке «{ $folder }»
           *[other] Выбрать все { $count } непрочитанные цепочки в папке «{ $folder }»
        }
       *[message] { $count ->
            [one] Выбрать { $count } непрочитанное письмо в папке «{ $folder }»
            [few] Выбрать все { $count } непрочитанных письма в папке «{ $folder }»
            [many] Выбрать все { $count } непрочитанных писем в папке «{ $folder }»
           *[other] Выбрать все { $count } непрочитанных письма в папке «{ $folder }»
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } помеченную цепочку в папке «{ $folder }»
            [few] Выбрать все { $count } помеченные цепочки в папке «{ $folder }»
            [many] Выбрать все { $count } помеченных цепочек в папке «{ $folder }»
           *[other] Выбрать все { $count } помеченные цепочки в папке «{ $folder }»
        }
       *[message] { $count ->
            [one] Выбрать { $count } помеченное письмо в папке «{ $folder }»
            [few] Выбрать все { $count } помеченных письма в папке «{ $folder }»
            [many] Выбрать все { $count } помеченных писем в папке «{ $folder }»
           *[other] Выбрать все { $count } помеченных письма в папке «{ $folder }»
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Выбрать { $count } цепочку без пометки в папке «{ $folder }»
            [few] Выбрать все { $count } цепочки без пометки в папке «{ $folder }»
            [many] Выбрать все { $count } цепочек без пометки в папке «{ $folder }»
           *[other] Выбрать все { $count } цепочки без пометки в папке «{ $folder }»
        }
       *[message] { $count ->
            [one] Выбрать { $count } письмо без пометки в папке «{ $folder }»
            [few] Выбрать все { $count } письма без пометки в папке «{ $folder }»
            [many] Выбрать все { $count } писем без пометки в папке «{ $folder }»
           *[other] Выбрать все { $count } письма без пометки в папке «{ $folder }»
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } прочитанная цепочка.
            [few] Выбраны все { $count } прочитанные цепочки.
            [many] Выбраны все { $count } прочитанных цепочек.
           *[other] Выбраны все { $count } прочитанные цепочки.
        }
       *[message] { $count ->
            [one] Выбрано { $count } прочитанное письмо.
            [few] Выбраны все { $count } прочитанных письма.
            [many] Выбраны все { $count } прочитанных писем.
           *[other] Выбраны все { $count } прочитанных письма.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } непрочитанная цепочка.
            [few] Выбраны все { $count } непрочитанные цепочки.
            [many] Выбраны все { $count } непрочитанных цепочек.
           *[other] Выбраны все { $count } непрочитанные цепочки.
        }
       *[message] { $count ->
            [one] Выбрано { $count } непрочитанное письмо.
            [few] Выбраны все { $count } непрочитанных письма.
            [many] Выбраны все { $count } непрочитанных писем.
           *[other] Выбраны все { $count } непрочитанных письма.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } помеченная цепочка.
            [few] Выбраны все { $count } помеченные цепочки.
            [many] Выбраны все { $count } помеченных цепочек.
           *[other] Выбраны все { $count } помеченные цепочки.
        }
       *[message] { $count ->
            [one] Выбрано { $count } помеченное письмо.
            [few] Выбраны все { $count } помеченных письма.
            [many] Выбраны все { $count } помеченных писем.
           *[other] Выбраны все { $count } помеченных письма.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } цепочка без пометки.
            [few] Выбраны все { $count } цепочки без пометки.
            [many] Выбраны все { $count } цепочек без пометки.
           *[other] Выбраны все { $count } цепочки без пометки.
        }
       *[message] { $count ->
            [one] Выбрано { $count } письмо без пометки.
            [few] Выбраны все { $count } письма без пометки.
            [many] Выбраны все { $count } писем без пометки.
           *[other] Выбраны все { $count } письма без пометки.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } прочитанная цепочка в папке «{ $folder }».
            [few] Выбраны все { $count } прочитанные цепочки в папке «{ $folder }».
            [many] Выбраны все { $count } прочитанных цепочек в папке «{ $folder }».
           *[other] Выбраны все { $count } прочитанные цепочки в папке «{ $folder }».
        }
       *[message] { $count ->
            [one] Выбрано { $count } прочитанное письмо в папке «{ $folder }».
            [few] Выбраны все { $count } прочитанных письма в папке «{ $folder }».
            [many] Выбраны все { $count } прочитанных писем в папке «{ $folder }».
           *[other] Выбраны все { $count } прочитанных письма в папке «{ $folder }».
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } непрочитанная цепочка в папке «{ $folder }».
            [few] Выбраны все { $count } непрочитанные цепочки в папке «{ $folder }».
            [many] Выбраны все { $count } непрочитанных цепочек в папке «{ $folder }».
           *[other] Выбраны все { $count } непрочитанные цепочки в папке «{ $folder }».
        }
       *[message] { $count ->
            [one] Выбрано { $count } непрочитанное письмо в папке «{ $folder }».
            [few] Выбраны все { $count } непрочитанных письма в папке «{ $folder }».
            [many] Выбраны все { $count } непрочитанных писем в папке «{ $folder }».
           *[other] Выбраны все { $count } непрочитанных письма в папке «{ $folder }».
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } помеченная цепочка в папке «{ $folder }».
            [few] Выбраны все { $count } помеченные цепочки в папке «{ $folder }».
            [many] Выбраны все { $count } помеченных цепочек в папке «{ $folder }».
           *[other] Выбраны все { $count } помеченные цепочки в папке «{ $folder }».
        }
       *[message] { $count ->
            [one] Выбрано { $count } помеченное письмо в папке «{ $folder }».
            [few] Выбраны все { $count } помеченных письма в папке «{ $folder }».
            [many] Выбраны все { $count } помеченных писем в папке «{ $folder }».
           *[other] Выбраны все { $count } помеченных письма в папке «{ $folder }».
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Выбрана { $count } цепочка без пометки в папке «{ $folder }».
            [few] Выбраны все { $count } цепочки без пометки в папке «{ $folder }».
            [many] Выбраны все { $count } цепочек без пометки в папке «{ $folder }».
           *[other] Выбраны все { $count } цепочки без пометки в папке «{ $folder }».
        }
       *[message] { $count ->
            [one] Выбрано { $count } письмо без пометки в папке «{ $folder }».
            [few] Выбраны все { $count } письма без пометки в папке «{ $folder }».
            [many] Выбраны все { $count } писем без пометки в папке «{ $folder }».
           *[other] Выбраны все { $count } письма без пометки в папке «{ $folder }».
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Здесь нет прочитанных цепочек.
       *[message] Здесь нет прочитанных писем.
    }
   *[unread] { $kind ->
        [conversation] Здесь нет непрочитанных цепочек.
       *[message] Здесь нет непрочитанных писем.
    }
    [starred] { $kind ->
        [conversation] Здесь нет помеченных цепочек.
       *[message] Здесь нет помеченных писем.
    }
    [unstarred] { $kind ->
        [conversation] Здесь нет цепочек без пометки.
       *[message] Здесь нет писем без пометки.
    }
}
list-clear-selection = Отменить выбор

## Mail list: empty states

list-empty-search = Нет писем, соответствующих запросу.
list-empty-tab = На вкладке «{ $tab }» нет писем.
list-empty-tab-unknown = На этой вкладке нет писем.
list-empty-folder = В папке «{ $folder }» нет писем.
list-empty-folder-unknown = В этой папке нет писем.
list-empty-waiting = Ответа ничего не ждёт.
list-empty-reminders = Напоминаний нет. Нажмите H на письме, чтобы добавить.
list-first-sync = Загружаем вашу почту…
list-first-sync-detail = Письма будут появляться здесь по мере получения.
list-store-unreadable = Не удалось открыть хранилище почты
row-no-subject = (без темы)
row-unknown-sender = (неизвестный отправитель)
row-to = Кому:
row-no-recipients = (нет получателей)

## Mail list: lines

row-removed = Это письмо удалено.
row-starred = Помечено
row-not-starred = Без пометки
row-important = Важное. Нажмите, чтобы отметить как неважное.
row-mark-important = Отметить как важное
row-pinned = Закреплено вверху
row-task = Задача
row-task-open = Открыть задачу: { $title }
row-tracking-none = Отслеживается. Ещё не открыто
row-tracking-opened = Открыли { $opened } из { $recipients }
row-tracking-clicked = Открыли { $opened } из { $recipients }, перешли по ссылке { $clicked }
row-pin = Закрепить вверху
row-unpin = Открепить
row-snoozed-until = Отложено до { $when }
row-snoozed-day-time = { $day }, { $time }
snoozed-group-today = Сегодня
snoozed-group-tomorrow = Завтра
snoozed-group-this-week = На этой неделе
snoozed-group-later = Позже
row-follow-up-step = Повторное письмо { $step } из { $steps } · { $date }
row-follow-up-waiting = Повторное письмо ждёт
row-reminder = Напоминание { $date }

## Mail list: More menu and right-click menu

menu-reply = Ответить
menu-reply-all = Ответить всем
menu-forward = Переслать
menu-archive = Архивировать
menu-delete = Удалить
menu-delete-forever = Удалить навсегда
menu-move-to-inbox = Переместить во входящие
menu-spam = В спам
menu-not-spam = Не спам
menu-mark-read = Отметить как прочитанное
menu-mark-unread = Отметить как непрочитанное
menu-mark-all-read = Отметить все как прочитанные
menu-star = Пометить
menu-unstar = Снять пометку
menu-important = Отметить как важное
menu-not-important = Отметить как неважное
menu-pin = Закрепить вверху
menu-unpin = Открепить
menu-snooze = Отложить
menu-remind = Напомнить
menu-unsnooze = Вернуть сейчас
menu-add-to-tasks = Добавить в Задачи
menu-schedule-meeting = Запланировать встречу
menu-start-call = Начать видеозвонок
menu-add-note = Добавить заметку
menu-print-all = Распечатать все
menu-new-window = Открыть в новом окне
menu-move-to = Переместить в
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = Дальнейшие шаги
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = Ещё
menu-move-to-heading = Переместить в:
menu-move-to-search = Переместить в…
menu-label-as = Добавить ярлык
menu-label-as-search = Добавить ярлык…
menu-no-folder = Нет папки «{ $name }»
menu-no-label = Нет ярлыка «{ $name }»
menu-create-folder = Создать «{ $name }»
menu-always-move = Всегда перемещать сюда письма от { $name }
toast-always-move-failed = Письмо перемещено, но правило не создано: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка
        [few] { $count } цепочки
        [many] { $count } цепочек
       *[other] { $count } цепочки
    }
   *[message] { $count ->
        [one] { $count } письмо
        [few] { $count } письма
        [many] { $count } писем
       *[other] { $count } письма
    }
}
menu-find-from = Найти письма от { $name }
menu-make-rule = Создать правило…
toast-key-imported = Ключ импортирован
toast-key-updated = Этот ключ у вас уже был; теперь он обновлён
toast-key-removed = Ключ удалён
toast-key-not-removed = Не удалось удалить ключ
toast-fingerprint-copied = Отпечаток скопирован

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка перенесена в архив.
        [few] { $count } цепочки перенесены в архив.
        [many] { $count } цепочек перенесены в архив.
       *[other] { $count } цепочки перенесены в архив.
    }
   *[message] { $count ->
        [one] { $count } письмо перенесено в архив.
        [few] { $count } письма перенесены в архив.
        [many] { $count } писем перенесены в архив.
       *[other] { $count } письма перенесены в архив.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка перемещена в корзину.
        [few] { $count } цепочки перемещены в корзину.
        [many] { $count } цепочек перемещены в корзину.
       *[other] { $count } цепочки перемещены в корзину.
    }
   *[message] { $count ->
        [one] { $count } письмо перемещено в корзину.
        [few] { $count } письма перемещены в корзину.
        [many] { $count } писем перемещены в корзину.
       *[other] { $count } письма перемещены в корзину.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка перемещена.
        [few] { $count } цепочки перемещены.
        [many] { $count } цепочек перемещены.
       *[other] { $count } цепочки перемещены.
    }
   *[message] { $count ->
        [one] { $count } письмо перемещено.
        [few] { $count } письма перемещены.
        [many] { $count } писем перемещены.
       *[other] { $count } письма перемещены.
    }
}
toast-label-added = Ярлык «{ $label }» добавлен.
toast-label-removed = Ярлык «{ $label }» снят.
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка помечена.
        [few] { $count } цепочки помечены.
        [many] { $count } цепочек помечены.
       *[other] { $count } цепочки помечены.
    }
   *[message] { $count ->
        [one] { $count } письмо помечено.
        [few] { $count } письма помечены.
        [many] { $count } писем помечены.
       *[other] { $count } письма помечены.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] С { $count } цепочки снята пометка.
        [few] С { $count } цепочек снята пометка.
        [many] С { $count } цепочек снята пометка.
       *[other] С { $count } цепочки снята пометка.
    }
   *[message] { $count ->
        [one] С { $count } письма снята пометка.
        [few] С { $count } писем снята пометка.
        [many] С { $count } писем снята пометка.
       *[other] С { $count } письма снята пометка.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка отмечена как важная.
        [few] { $count } цепочки отмечены как важные.
        [many] { $count } цепочек отмечены как важные.
       *[other] { $count } цепочки отмечены как важные.
    }
   *[message] { $count ->
        [one] { $count } письмо отмечено как важное.
        [few] { $count } письма отмечены как важные.
        [many] { $count } писем отмечены как важные.
       *[other] { $count } письма отмечены как важные.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка отмечена как неважная.
        [few] { $count } цепочки отмечены как неважные.
        [many] { $count } цепочек отмечены как неважные.
       *[other] { $count } цепочки отмечены как неважные.
    }
   *[message] { $count ->
        [one] { $count } письмо отмечено как неважное.
        [few] { $count } письма отмечены как неважные.
        [many] { $count } писем отмечены как неважные.
       *[other] { $count } письма отмечены как неважные.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка закреплена вверху.
        [few] { $count } цепочки закреплены вверху.
        [many] { $count } цепочек закреплены вверху.
       *[other] { $count } цепочки закреплены вверху.
    }
   *[message] { $count ->
        [one] { $count } письмо закреплено вверху.
        [few] { $count } письма закреплены вверху.
        [many] { $count } писем закреплены вверху.
       *[other] { $count } письма закреплены вверху.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка откреплена.
        [few] { $count } цепочки откреплены.
        [many] { $count } цепочек откреплены.
       *[other] { $count } цепочки откреплены.
    }
   *[message] { $count ->
        [one] { $count } письмо откреплено.
        [few] { $count } письма откреплены.
        [many] { $count } писем откреплены.
       *[other] { $count } письма откреплены.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка отложена до { $when }.
        [few] { $count } цепочки отложены до { $when }.
        [many] { $count } цепочек отложены до { $when }.
       *[other] { $count } цепочки отложены до { $when }.
    }
   *[message] { $count ->
        [one] { $count } письмо отложено до { $when }.
        [few] { $count } письма отложены до { $when }.
        [many] { $count } писем отложены до { $when }.
       *[other] { $count } письма отложены до { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка снова во входящих.
        [few] { $count } цепочки снова во входящих.
        [many] { $count } цепочек снова во входящих.
       *[other] { $count } цепочки снова во входящих.
    }
   *[message] { $count ->
        [one] { $count } письмо снова во входящих.
        [few] { $count } письма снова во входящих.
        [many] { $count } писем снова во входящих.
       *[other] { $count } письма снова во входящих.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка отмечена как спам.
        [few] { $count } цепочки отмечены как спам.
        [many] { $count } цепочек отмечены как спам.
       *[other] { $count } цепочки отмечены как спам.
    }
   *[message] { $count ->
        [one] { $count } письмо отмечено как спам.
        [few] { $count } письма отмечены как спам.
        [many] { $count } писем отмечены как спам.
       *[other] { $count } письма отмечены как спам.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка помечена как не спам и перемещена во входящие.
        [few] { $count } цепочки помечены как не спам и перемещены во входящие.
        [many] { $count } цепочек помечены как не спам и перемещены во входящие.
       *[other] { $count } цепочки помечены как не спам и перемещены во входящие.
    }
   *[message] { $count ->
        [one] { $count } письмо помечено как не спам и перемещено во входящие.
        [few] { $count } письма помечены как не спам и перемещены во входящие.
        [many] { $count } писем помечены как не спам и перемещены во входящие.
       *[other] { $count } письма помечены как не спам и перемещены во входящие.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка удалена навсегда.
        [few] { $count } цепочки удалены навсегда.
        [many] { $count } цепочек удалены навсегда.
       *[other] { $count } цепочки удалены навсегда.
    }
   *[message] { $count ->
        [one] { $count } письмо удалено навсегда.
        [few] { $count } письма удалены навсегда.
        [many] { $count } писем удалены навсегда.
       *[other] { $count } письма удалены навсегда.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка отмечена как прочитанная.
        [few] { $count } цепочки отмечены как прочитанные.
        [many] { $count } цепочек отмечены как прочитанные.
       *[other] { $count } цепочки отмечены как прочитанные.
    }
   *[message] { $count ->
        [one] { $count } письмо отмечено как прочитанное.
        [few] { $count } письма отмечены как прочитанные.
        [many] { $count } писем отмечены как прочитанные.
       *[other] { $count } письма отмечены как прочитанные.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] { $count } цепочка отмечена как непрочитанная.
        [few] { $count } цепочки отмечены как непрочитанные.
        [many] { $count } цепочек отмечены как непрочитанные.
       *[other] { $count } цепочки отмечены как непрочитанные.
    }
   *[message] { $count ->
        [one] { $count } письмо отмечено как непрочитанное.
        [few] { $count } письма отмечены как непрочитанные.
        [many] { $count } писем отмечены как непрочитанные.
       *[other] { $count } письма отмечены как непрочитанные.
    }
}
toast-undone = Действие отменено.
toast-nothing-to-undo = Нечего отменять.
toast-cannot-undo-delete-forever = Письма, удалённые навсегда, вернуть нельзя.
toast-send-undone = Отправка отменена.
toast-too-late-to-undo-send = Слишком поздно отменять: письмо уже отправлено.
toast-undo = Отменить
toast-close = Закрыть
toast-no-spam-folder = В этом аккаунте нет папки «Спам».
