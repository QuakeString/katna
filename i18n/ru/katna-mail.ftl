# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Язык: { $language }
language-tooltip-system = Язык: { $language }, как в системе
language-search = Поиск языка
language-system-default = Как в системе
language-system-now = Сейчас: { $language }
language-no-match = Нет языков по запросу «{ $query }»
language-machine = Машинный перевод. Помогите улучшить
language-setting = Язык
language-setting-detail = Язык меню, кнопок и сообщений, а также формат дат и чисел. «Как в системе» — по настройкам рабочего стола.

## Dates and sizes

ago-just-now = только что
ago-minutes = { $count ->
    [one] { $count } минуту назад
    [few] { $count } минуты назад
    [many] { $count } минут назад
   *[other] { $count } минуты назад
}
ago-hours = { $count ->
    [one] { $count } час назад
    [few] { $count } часа назад
    [many] { $count } часов назад
   *[other] { $count } часа назад
}
ago-days = { $count ->
    [one] { $count } день назад
    [few] { $count } дня назад
    [many] { $count } дней назад
   *[other] { $count } дня назад
}
size-bytes = { $count ->
    [one] { $count } байт
    [few] { $count } байта
    [many] { $count } байт
   *[other] { $count } байта
}
size-kb = { $size } КБ
size-mb = { $size } МБ
size-gb = { $size } ГБ
size-tb = { $size } ТБ

## Top bar

folders-hide = Скрыть папки
folders-show = Показать папки
compose = Написать
search = Поиск
search-mail = Поиск в почте
search-settings = Поиск в настройках
search-clear = Очистить поиск
search-options-show = Показать параметры поиска
settings = Настройки
account-add = Добавить аккаунт

## App rail (and the bottom bar on a phone)

rail-mail = Почта
rail-calendar = Календарь
rail-contacts = Контакты
rail-tasks = Задачи
rail-notes = Заметки
rail-feeds = Ленты

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Скоро
app-calendar-promise = Ваши календари CalDAV, приглашения на встречи из почты и напоминания — рядом с входящими.
app-tasks-promise = Списки задач с синхронизацией по CalDAV и задачи, созданные из писем.
app-notes-promise = Быстрые заметки и заметки к письму или цепочке на потом.
app-feeds-promise = Читайте ленты RSS и Atom рядом с почтой.

## Contacts page

app-contacts-loading = Собираем людей из вашей почты…
app-contacts-empty = Здесь появятся люди, с которыми вы переписываетесь.
app-contacts-count = { $count ->
    [one] { $count } человек из вашей почты, сначала самые частые собеседники
    [few] { $count } человека из вашей почты, сначала самые частые собеседники
    [many] { $count } человек из вашей почты, сначала самые частые собеседники
   *[other] { $count } человека из вашей почты, сначала самые частые собеседники
}
app-contacts-top = { $count ->
    [one] { $count } самый частый собеседник из вашей почты
    [few] { $count } самых частых собеседника из вашей почты
    [many] { $count } самых частых собеседников из вашей почты
   *[other] { $count } самого частого собеседника из вашей почты
}
app-contacts-messages = { $count ->
    [one] { $count } письмо
    [few] { $count } письма
    [many] { $count } писем
   *[other] { $count } письма
}
app-contacts-last = последнее: { $date }

## Navigation (the folders pane)

nav-labels = Ярлыки
nav-folders = Папки
nav-label-new = Создать ярлык
nav-folder-new = Создать папку
nav-account-unnamed = Аккаунт { $number }
nav-tab-new = { $count ->
    [one] { $count } новое
    [few] { $count } новых
    [many] { $count } новых
   *[other] { $count } новых
}

## Special folders (the user's own folders keep their names)

folder-inbox = Входящие
folder-starred = Помеченные
folder-drafts = Черновики
folder-sent = Отправленные
folder-archive = Архив
folder-spam = Спам
folder-trash = Корзина
folder-all-mail = Вся почта
folder-scheduled = Запланированные

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Новый ярлык
label-folder-new-title = Новая папка
label-prompt = Введите название нового ярлыка:
label-folder-prompt = Введите название новой папки:
label-name-hint = Название ярлыка
label-folder-name-hint = Название папки
label-nest = Вложить ярлык в:
label-folder-nest = Вложить папку в:
label-cancel = Отмена
label-create = Создать
label-creating = Создание…
label-created = Ярлык «{ $name }» создан.
label-folder-created = Папка «{ $name }» создана.

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
tab-new = { $count ->
    [one] { $count } новое
    [few] { $count } новых
    [many] { $count } новых
   *[other] { $count } новых
}
tab-provider-other = сортирует Katna

## Mail list: toolbar

list-select = Выбрать
list-refresh = Обновить
list-more = Ещё
list-mark-read = Отметить как прочитанное
list-mark-unread = Отметить как непрочитанное
list-move-to = Переместить в
list-archive = Архивировать
list-spam = В спам
list-delete = Удалить
list-newer = Более новые
list-older = Более старые
list-range = { $first }–{ $last } из { $total }
list-range-about = { $first }–{ $last } из примерно { $total }
list-results = Результаты по запросу «{ $query }»
list-results-corrected = Показаны результаты по запросу «{ $query }»
list-search-instead = Искать вместо этого «{ $query }»
list-files-more = +{ $count }

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
list-clear-selection = Отменить выбор

## Mail list: empty states

list-empty-search = Нет писем, соответствующих запросу.
list-empty-tab = На вкладке «{ $tab }» нет писем.
list-empty-tab-unknown = На этой вкладке нет писем.
list-empty-folder = В папке «{ $folder }» нет писем.
list-empty-folder-unknown = В этой папке нет писем.
list-first-sync = Загружаем вашу почту…
list-first-sync-detail = Письма будут появляться здесь по мере получения.

## Mail list: lines

row-removed = Это письмо удалено.
row-starred = Помечено
row-not-starred = Без пометки
row-important = Важное. Нажмите, чтобы отметить как неважное.
row-mark-important = Отметить как важное
row-pinned = Закреплено вверху
row-pin = Закрепить вверху
row-unpin = Открепить

## Mail list: More menu and right-click menu

menu-reply = Ответить
menu-reply-all = Ответить всем
menu-forward = Переслать
menu-archive = Архивировать
menu-delete = Удалить
menu-spam = В спам
menu-mark-read = Отметить как прочитанное
menu-mark-unread = Отметить как непрочитанное
menu-mark-all-read = Отметить все как прочитанные
menu-star = Пометить
menu-unstar = Снять пометку
menu-important = Отметить как важное
menu-not-important = Отметить как неважное
menu-pin = Закрепить вверху
menu-unpin = Открепить
menu-print-all = Распечатать все
menu-new-window = Открыть в новом окне
menu-move-to = Переместить в
menu-move-to-heading = Переместить в:
menu-find-from = Найти письма от { $name }

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
toast-undone = Действие отменено.
toast-undo = Отменить
toast-no-spam-folder = В этом аккаунте нет папки «Спам».

## Reading pane: toolbar

reader-close = Закрыть
reader-back = Назад
reader-mark-unread = Отметить как непрочитанное
reader-move-to = Переместить в
reader-more = Ещё
reader-print-all = Распечатать все
reader-new-window = В новом окне
reader-position = { $position } из { $total }
reader-newer = Более новая
reader-older = Более старая

## Reading pane: the conversation

reader-removed = Эта цепочка удалена.
reader-no-subject = (без темы)
reader-collapse-all = Свернуть все
reader-expand-all = Развернуть все
reader-unknown-sender = (неизвестный отправитель)
reader-date-ago = { $date } ({ $ago })
reader-me = мне
reader-to = кому: { $names }
reader-starred = Помечено
reader-not-starred = Без пометки
reader-too-long = Письмо слишком длинное, чтобы показать его полностью.
reader-encrypted-images = В зашифрованных письмах изображения из интернета никогда не загружаются.
reader-window-failed = Не удалось открыть новое окно.

## Reading pane: message details (opened from "to me")

reader-details-from = от:
reader-details-to = кому:
reader-details-cc = копия:
reader-details-date = дата:
reader-details-subject = тема:

## Reading pane: downloading a message

reader-downloading = Загрузка письма с сервера…
reader-download-failed = Не удалось загрузить это письмо.
reader-try-again = Повторить попытку

## Reply row

reply-reply = Ответить
reply-reply-all = Ответить всем
reply-forward = Переслать

## Encrypted and signed mail

security-decrypting = Расшифровка…
security-checking = Проверка подписи…
security-partly-encrypted = Зашифрована только часть этого письма. Остальное добавлено вне защиты и могло быть отправлено кем угодно.
security-partly-signed = Подписана только часть этого письма. Остальное добавлено вне защиты и могло быть отправлено кем угодно.
security-encrypted = Зашифрованное письмо
security-encrypted-smime = Зашифрованное письмо (S/MIME)
security-no-key = Не удаётся расшифровать письмо: оно зашифровано для ключа, которого у вас нет.
security-cancelled = Расшифровка отменена.
security-damaged = Не удаётся расшифровать письмо: зашифрованные данные повреждены или изменены.
security-decrypt-unavailable = Не удаётся расшифровать письмо: установите { $tool }, чтобы читать зашифрованную почту.
security-decrypt-failed = Не удаётся расшифровать письмо: { $reason }
security-unknown-signer = неизвестный автор подписи
security-signed-verified = Подписано: { $signer } · подпись проверена
security-signed-not-sender = Подписано: { $signer } — это не отправитель
security-signed-untrusted = Подписано: { $signer }, ключом, который вы отметили как ненадёжный
security-signed-unverified = Подписано: { $signer } · ключ не проверен
security-bad-signature = Недействительная подпись: письмо изменено после подписания или подпись подделана.
security-signature-expired = Подписано: { $signer } · срок действия подписи истёк
security-key-expired = Подписано: { $signer } · срок действия ключа с тех пор истёк
security-key-revoked = Подписано: { $signer }, ключом, который был отозван
security-missing-key = Подписано ключом, которого у вас нет, поэтому подпись нельзя проверить
security-missing-key-id = Подписано ключом, которого у вас нет ({ $key }), поэтому подпись нельзя проверить
security-signature-unavailable = Подписано; установите { $tool }, чтобы проверить подпись
security-signature-error = Не удалось проверить подпись.

## Remote images and pictures

remote-hidden = Изображения в этом письме скрыты.
remote-show = Показать изображения
remote-always-show = Всегда показывать от этого отправителя
remote-picture-use = Выбрать
remote-picture-too-big = Выберите изображение размером не более 8 МБ.
remote-picture-type = Выберите изображение PNG, JPEG, GIF, WebP или SVG.
remote-picture-read-failed = Не удаётся прочитать изображение: { $error }
remote-picture-keep-failed = Не удаётся сохранить изображение: { $error }
remote-picture-remove-failed = Не удаётся удалить изображение: { $error }

## Attachments

attachment-count = { $count ->
    [one] { $count } вложение
    [few] { $count } вложения
    [many] { $count } вложений
   *[other] { $count } вложения
}
attachment-save = Сохранить
attachment-save-all = Сохранить все
attachment-save-all-tooltip = Сохранить все вложения в папку
attachment-save-here = Сохранить здесь
attachment-not-downloaded = Это письмо не загружено.
attachment-not-found = Это вложение не найдено в письме.
attachment-read-failed = Не удалось прочитать { $name }
attachment-numbered = вложение { $number }
attachment-saved-all = { $count ->
    [one] { $count } файл сохранён в папку «{ $place }»
    [few] { $count } файла сохранены в папку «{ $place }»
    [many] { $count } файлов сохранены в папку «{ $place }»
   *[other] { $count } файла сохранены в папку «{ $place }»
}
attachment-saved-some = { $total ->
    [one] Сохранено { $saved } из { $total } файла в папку «{ $place }». Не удалось сохранить { $failed }
    [few] Сохранено { $saved } из { $total } файлов в папку «{ $place }». Не удалось сохранить { $failed }
    [many] Сохранено { $saved } из { $total } файлов в папку «{ $place }». Не удалось сохранить { $failed }
   *[other] Сохранено { $saved } из { $total } файла в папку «{ $place }». Не удалось сохранить { $failed }
}
attachment-saved-to = Сохранено в { $path }
attachment-save-failed = Не удалось сохранить { $name }: { $error }
attachment-open-failed = Не удалось открыть { $name }: { $error }
attachment-risky = Этот файл может запустить программу, поэтому Katna его не открывает. Сохраните его.
attachment-encrypted-open = Этот файл пришёл зашифрованным. Сохраните его, чтобы открыть в другом приложении.

## Printing

print-failed = Не удалось распечатать: { $error }
print-no-font = шрифт не найден
print-opened-as-pdf = Открыто как PDF, чтобы распечатать оттуда.
print-not-downloaded = (Ещё не загружено.)
print-encrypted = (Зашифровано. Откройте письмо в Katna Mail, чтобы распечатать его текст.)
print-to = Кому: { $addresses }
print-cc = Копия: { $addresses }
