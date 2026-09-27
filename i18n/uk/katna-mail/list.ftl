# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Основні
tab-promotions = Реклама
tab-social = Соцмережі
tab-updates = Оновлення
tab-forums = Форуми
tab-focused = Пріоритетні
tab-other = Інші
tab-inbox = Вхідні
tab-newsletters = Розсилки
tab-notifications = Сповіщення
tab-new = { $count ->
    [one] { $count } новий
    [few] { $count } нові
    [many] { $count } нових
   *[other] { $count } нового
}
tab-provider-other = сортує Katna

## Mail list: toolbar

list-select = Вибрати
list-refresh = Оновити
list-more = Більше
list-mark-read = Позначити як прочитане
list-mark-unread = Позначити як непрочитане
list-move-to = Перемістити в
list-archive = Архівувати
list-spam = Повідомити про спам
list-delete = Видалити
list-newer = Новіші
list-older = Старіші
list-range = { $first }–{ $last } з { $total }
list-range-about = { $first }–{ $last } з приблизно { $total }
list-results = Результати за запитом «{ $query }»
list-results-corrected = Показано результати за запитом «{ $query }»
list-search-instead = Натомість шукати «{ $query }»
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Усі
list-pick-none = Жодного
list-pick-read = Прочитані
list-pick-unread = Непрочитані
list-pick-starred = Із зірочкою
list-pick-unstarred = Без зірочки

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Вибрано { $count } ланцюжок.
        [few] Вибрано всі { $count } ланцюжки.
        [many] Вибрано всі { $count } ланцюжків.
       *[other] Вибрано всі { $count } ланцюжка.
    }
   *[message] { $count ->
        [one] Вибрано { $count } лист.
        [few] Вибрано всі { $count } листи.
        [many] Вибрано всі { $count } листів.
       *[other] Вибрано всі { $count } листа.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Вибрано { $count } ланцюжок у папці «{ $folder }».
        [few] Вибрано всі { $count } ланцюжки в папці «{ $folder }».
        [many] Вибрано всі { $count } ланцюжків у папці «{ $folder }».
       *[other] Вибрано всі { $count } ланцюжка в папці «{ $folder }».
    }
   *[message] { $count ->
        [one] Вибрано { $count } лист у папці «{ $folder }».
        [few] Вибрано всі { $count } листи в папці «{ $folder }».
        [many] Вибрано всі { $count } листів у папці «{ $folder }».
       *[other] Вибрано всі { $count } листа в папці «{ $folder }».
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Вибрано { $count } ланцюжок на цій сторінці.
        [few] Вибрано всі { $count } ланцюжки на цій сторінці.
        [many] Вибрано всі { $count } ланцюжків на цій сторінці.
       *[other] Вибрано всі { $count } ланцюжка на цій сторінці.
    }
   *[message] { $count ->
        [one] Вибрано { $count } лист на цій сторінці.
        [few] Вибрано всі { $count } листи на цій сторінці.
        [many] Вибрано всі { $count } листів на цій сторінці.
       *[other] Вибрано всі { $count } листа на цій сторінці.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Вибрати { $count } ланцюжок
        [few] Вибрати всі { $count } ланцюжки
        [many] Вибрати всі { $count } ланцюжків
       *[other] Вибрати всі { $count } ланцюжка
    }
   *[message] { $count ->
        [one] Вибрати { $count } лист
        [few] Вибрати всі { $count } листи
        [many] Вибрати всі { $count } листів
       *[other] Вибрати всі { $count } листа
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Вибрати { $count } ланцюжок у папці «{ $folder }»
        [few] Вибрати всі { $count } ланцюжки в папці «{ $folder }»
        [many] Вибрати всі { $count } ланцюжків у папці «{ $folder }»
       *[other] Вибрати всі { $count } ланцюжка в папці «{ $folder }»
    }
   *[message] { $count ->
        [one] Вибрати { $count } лист у папці «{ $folder }»
        [few] Вибрати всі { $count } листи в папці «{ $folder }»
        [many] Вибрати всі { $count } листів у папці «{ $folder }»
       *[other] Вибрати всі { $count } листа в папці «{ $folder }»
    }
}
list-clear-selection = Скасувати вибір

## Mail list: empty states

list-empty-search = Немає листів, що відповідають запиту.
list-empty-tab = На вкладці «{ $tab }» немає листів.
list-empty-tab-unknown = На цій вкладці немає листів.
list-empty-folder = У папці «{ $folder }» немає листів.
list-empty-folder-unknown = У цій папці немає листів.
list-first-sync = Отримуємо вашу пошту…
list-first-sync-detail = Листи з’являтимуться тут у міру надходження.

## Mail list: lines

row-removed = Цей лист видалено.
row-starred = Із зірочкою
row-not-starred = Без зірочки
row-important = Важливе. Натисніть, щоб позначити як неважливе.
row-mark-important = Позначити як важливе
row-pinned = Закріплено вгорі
row-pin = Закріпити вгорі
row-unpin = Відкріпити

## Mail list: More menu and right-click menu

menu-reply = Відповісти
menu-reply-all = Відповісти всім
menu-forward = Переслати
menu-archive = Архівувати
menu-delete = Видалити
menu-spam = Повідомити про спам
menu-mark-read = Позначити як прочитане
menu-mark-unread = Позначити як непрочитане
menu-mark-all-read = Позначити все як прочитане
menu-star = Позначити зірочкою
menu-unstar = Зняти зірочку
menu-important = Позначити як важливе
menu-not-important = Позначити як неважливе
menu-pin = Закріпити вгорі
menu-unpin = Відкріпити
menu-print-all = Надрукувати все
menu-new-window = Відкрити в новому вікні
menu-move-to = Перемістити в
menu-move-to-heading = Перемістити в:
menu-find-from = Знайти листи від { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок заархівовано.
        [few] { $count } ланцюжки заархівовано.
        [many] { $count } ланцюжків заархівовано.
       *[other] { $count } ланцюжка заархівовано.
    }
   *[message] { $count ->
        [one] { $count } лист заархівовано.
        [few] { $count } листи заархівовано.
        [many] { $count } листів заархівовано.
       *[other] { $count } листа заархівовано.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок переміщено в кошик.
        [few] { $count } ланцюжки переміщено в кошик.
        [many] { $count } ланцюжків переміщено в кошик.
       *[other] { $count } ланцюжка переміщено в кошик.
    }
   *[message] { $count ->
        [one] { $count } лист переміщено в кошик.
        [few] { $count } листи переміщено в кошик.
        [many] { $count } листів переміщено в кошик.
       *[other] { $count } листа переміщено в кошик.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок переміщено.
        [few] { $count } ланцюжки переміщено.
        [many] { $count } ланцюжків переміщено.
       *[other] { $count } ланцюжка переміщено.
    }
   *[message] { $count ->
        [one] { $count } лист переміщено.
        [few] { $count } листи переміщено.
        [many] { $count } листів переміщено.
       *[other] { $count } листа переміщено.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок позначено зірочкою.
        [few] { $count } ланцюжки позначено зірочкою.
        [many] { $count } ланцюжків позначено зірочкою.
       *[other] { $count } ланцюжка позначено зірочкою.
    }
   *[message] { $count ->
        [one] { $count } лист позначено зірочкою.
        [few] { $count } листи позначено зірочкою.
        [many] { $count } листів позначено зірочкою.
       *[other] { $count } листа позначено зірочкою.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Зірочку знято з { $count } ланцюжка.
        [few] Зірочку знято з { $count } ланцюжків.
        [many] Зірочку знято з { $count } ланцюжків.
       *[other] Зірочку знято з { $count } ланцюжка.
    }
   *[message] { $count ->
        [one] Зірочку знято з { $count } листа.
        [few] Зірочку знято з { $count } листів.
        [many] Зірочку знято з { $count } листів.
       *[other] Зірочку знято з { $count } листа.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок позначено як важливий.
        [few] { $count } ланцюжки позначено як важливі.
        [many] { $count } ланцюжків позначено як важливі.
       *[other] { $count } ланцюжка позначено як важливі.
    }
   *[message] { $count ->
        [one] { $count } лист позначено як важливий.
        [few] { $count } листи позначено як важливі.
        [many] { $count } листів позначено як важливі.
       *[other] { $count } листа позначено як важливі.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок позначено як неважливий.
        [few] { $count } ланцюжки позначено як неважливі.
        [many] { $count } ланцюжків позначено як неважливі.
       *[other] { $count } ланцюжка позначено як неважливі.
    }
   *[message] { $count ->
        [one] { $count } лист позначено як неважливий.
        [few] { $count } листи позначено як неважливі.
        [many] { $count } листів позначено як неважливі.
       *[other] { $count } листа позначено як неважливі.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок закріплено вгорі.
        [few] { $count } ланцюжки закріплено вгорі.
        [many] { $count } ланцюжків закріплено вгорі.
       *[other] { $count } ланцюжка закріплено вгорі.
    }
   *[message] { $count ->
        [one] { $count } лист закріплено вгорі.
        [few] { $count } листи закріплено вгорі.
        [many] { $count } листів закріплено вгорі.
       *[other] { $count } листа закріплено вгорі.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок відкріплено.
        [few] { $count } ланцюжки відкріплено.
        [many] { $count } ланцюжків відкріплено.
       *[other] { $count } ланцюжка відкріплено.
    }
   *[message] { $count ->
        [one] { $count } лист відкріплено.
        [few] { $count } листи відкріплено.
        [many] { $count } листів відкріплено.
       *[other] { $count } листа відкріплено.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок позначено як спам.
        [few] { $count } ланцюжки позначено як спам.
        [many] { $count } ланцюжків позначено як спам.
       *[other] { $count } ланцюжка позначено як спам.
    }
   *[message] { $count ->
        [one] { $count } лист позначено як спам.
        [few] { $count } листи позначено як спам.
        [many] { $count } листів позначено як спам.
       *[other] { $count } листа позначено як спам.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] { $count } ланцюжок видалено назавжди.
        [few] { $count } ланцюжки видалено назавжди.
        [many] { $count } ланцюжків видалено назавжди.
       *[other] { $count } ланцюжка видалено назавжди.
    }
   *[message] { $count ->
        [one] { $count } лист видалено назавжди.
        [few] { $count } листи видалено назавжди.
        [many] { $count } листів видалено назавжди.
       *[other] { $count } листа видалено назавжди.
    }
}
toast-undone = Дію скасовано.
toast-undo = Скасувати
toast-no-spam-folder = У цьому обліковому записі немає папки «Спам».
