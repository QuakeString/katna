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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Откройте это письмо, чтобы прочитать вложения.
text-copy = Копировать
text-select-all = Выделить всё

## Settings page: its tabs

settings-tab-general = Общие
settings-tab-inbox = Входящие
settings-tab-accounts = Аккаунты
settings-tab-subscriptions = Подписки
settings-tab-appearance = Внешний вид
settings-tab-shortcuts = Быстрые клавиши
settings-tab-default-apps = Приложения по умолчанию
settings-tab-folders-rules = Папки и правила
settings-tab-compose = Написание писем
settings-tab-mcp-server = Сервер MCP
settings-tab-feedback = Отзывы пользователей
settings-tab-experimental = Экспериментальные

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Смотрите, какие рассылки и списки рассылки вы получаете, и отписывайтесь в один клик.
settings-tab-folders-rules-coming = Создавайте, переименовывайте, перемещайте и скрывайте папки и ярлыки, выбирайте, какие из них синхронизировать. Правила сами сортируют, помечают ярлыками, пересылают или удаляют новые письма — по отправителю, теме или словам.
settings-tab-mcp-server-coming = Разрешите ИИ-помощникам на этом компьютере искать, читать и составлять черновики ваших писем — с вашего согласия.

## Settings > General

settings-general-conversations = Цепочки писем
settings-general-conversations-group = Группировать ответы на одно письмо
settings-general-conversations-group-detail = Одна строка на цепочку в списке
settings-general-reading = Чтение
settings-general-newest-first = Сначала новые письма
settings-general-newest-first-detail = Цепочка начинается с последнего ответа
settings-general-full-headers = Показывать заголовки полностью
settings-general-full-headers-detail = От кого, кому, копия, дата и тема открыты в каждом письме
settings-general-full-names = Полные имена получателей
settings-general-full-names-detail = «мне, Ada Lovelace», а не «мне, Ada»
settings-general-mark-read = Отмечать как прочитанное
settings-general-mark-read-now = Сразу при открытии
settings-general-mark-read-1s = Через 1 секунду после открытия
settings-general-mark-read-3s = Через 3 секунды после открытия
settings-general-mark-read-never = Только когда я отмечу сам
settings-general-reply-button = Кнопка «Ответить»
settings-general-reply-all = Отвечать всем
settings-general-reply-all-detail = Кнопка ответа рядом с каждым письмом отвечает всем, а не только отправителю
settings-general-remote-images = Изображения из интернета
settings-general-remote-images-detail = Загрузка изображений письма сообщает отправителю, что вы его открыли, когда и примерно где. Если выключено, каждое письмо сначала спрашивает, и вы всегда можете показать изображения отправителя.
settings-general-remote-images-always = Всегда показывать изображения
settings-general-remote-images-always-detail = В каждом письме, а не только от надёжных отправителей
settings-general-sending = Отправка
settings-general-sending-detail = Сколько отправленное письмо ждёт, чтобы его можно было отменить.
settings-general-offline = Почта офлайн
settings-general-offline-detail = Недавние письма загружаются целиком, чтобы читать их без подключения. Более старые загружаются, когда вы их открываете.
settings-general-offline-days = { $count ->
    [one] { $count } день
    [few] { $count } дня
    [many] { $count } дней
   *[other] { $count } дня
}
settings-general-offline-years = { $count ->
    [one] { $count } год
    [few] { $count } года
    [many] { $count } лет
   *[other] { $count } года
}
settings-general-offline-all = Вся почта
settings-general-offline-note = Если выбрать меньше дней, уже загруженные письма останутся. На сервере ничего не меняется.
settings-general-notifications = Уведомления
settings-general-notifications-detail = О новых письмах во «Входящих», даже когда Katna Mail закрыта.
settings-general-new-mail = Уведомлять о новых письмах
settings-general-new-mail-detail = С кнопками «Ответить всем», «Отметить как прочитанное» и «Архивировать»
settings-general-new-mail-sound = Воспроизводить звук
settings-general-new-mail-sound-detail = Звук новой почты рабочего стола
settings-general-desktop = Рабочий стол
settings-general-open-at-login = Открывать Katna Mail при входе в систему
settings-general-open-at-login-detail = Почта синхронизируется при входе в любом случае, пока работает служба
settings-general-tray = Показывать Katna в системном лотке
settings-general-tray-detail = Со счётчиком непрочитанных и меню
settings-general-unread-badge = Счётчик непрочитанных на значке в панели задач
settings-general-unread-badge-detail = Сколько писем во «Входящих» не прочитано

## Settings > Inbox

settings-inbox-tabs = Вкладки «Входящих»
settings-inbox-tabs-detail = Сортировать входящие по вкладкам, как это делает сайт вашего почтового сервиса.
settings-inbox-tabs-show = Показывать вкладки «Входящих»
settings-inbox-tabs-show-detail = Если выключено, для каждого аккаунта один список
settings-inbox-no-accounts = Добавьте аккаунт, чтобы выбрать его вкладки.
settings-inbox-tabs-automatic = Автоматически: { $tabs } ({ $provider })
settings-inbox-tabs-off = Без вкладок
settings-inbox-tabs-gmail = Несортированные, Промоакции, Соцсети, Оповещения, Форумы
settings-inbox-tabs-focused = Отсортированные и Другие
settings-inbox-tabs-zoho = Входящие, Рассылки и Уведомления
settings-inbox-tabs-shown = Показанные вкладки. Письма с выключенной вкладки остаются во вкладке «{ $tab }».

## Settings > Appearance

settings-appearance-reading-pane = Область просмотра
settings-appearance-reading-pane-detail = Где показывается открытая цепочка.
settings-appearance-pane-right = Справа от списка
settings-appearance-pane-none = Без разделения
settings-appearance-density = Плотность
settings-appearance-density-default = Обычная
settings-appearance-density-compact = Компактная
settings-appearance-scaling = Масштаб
settings-appearance-scaling-detail = Делает всё в Katna Mail крупнее или мельче поверх масштаба рабочего стола: текст, значки, отступы и разделители. Отправляемые письма сохраняют свой размер шрифта. При очень малом масштабе по значкам трудно попасть.
settings-appearance-theme = Тема
settings-appearance-theme-system = Как на рабочем столе
settings-appearance-theme-light = Светлая
settings-appearance-theme-dark = Тёмная
settings-appearance-desktop-colors = Цвета рабочего стола
settings-appearance-desktop-colors-use = Использовать цвета рабочего стола
settings-appearance-desktop-colors-use-detail = Цветовая схема и акцентный цвет рабочего стола
settings-appearance-app-names = Названия приложений
settings-appearance-app-names-show = Показывать названия приложений
settings-appearance-app-names-show-detail = Подписи под значками приложений слева
settings-appearance-sender-pictures = Изображения отправителей
settings-appearance-sender-pictures-show = Показывать логотипы компаний
settings-appearance-sender-pictures-show-detail = Ищутся по домену отправителя, никогда по письму, и хранятся неделю
settings-appearance-important = Маркеры важности
settings-appearance-important-show = Показывать маркеры важности
settings-appearance-important-show-detail = Рядом с каждым письмом в списке
settings-appearance-message-width = Ширина письма
settings-appearance-message-width-limit = Ограничить ширину писем
settings-appearance-message-width-limit-detail = Так в широком окне легче читать длинные строки
settings-appearance-mail-colors = Цвета писем
settings-appearance-mail-colors-detail = Большинство писем рассчитаны на белый фон. В тёмной теме их цвета заменяются тёмными, удобными для чтения; если выключено, письмо сохраняет цвета отправителя на светлом фоне.
settings-appearance-dark-mail = Тёмные цвета и для писем
settings-appearance-dark-mail-detail = Только когда тема тёмная
settings-appearance-attachment-previews = Предпросмотр вложений
settings-appearance-attachment-previews-show = Показывать предпросмотр вложений
settings-appearance-attachment-previews-show-detail = Миниатюра содержимого каждого файла на его карточке

## Settings > Default apps

settings-default-apps-intro = Где открываются вложения, когда вы на них нажимаете. Из окна просмотра файл всегда можно открыть и в другом приложении. Приложения по умолчанию для рабочего стола задаются в его собственных настройках.
settings-default-apps-pdf = Файлы PDF
settings-default-apps-pdf-detail = Страницы с масштабированием.
settings-default-apps-pictures = Изображения
settings-default-apps-pictures-detail = Фотографии (с правильной ориентацией), PNG, GIF, WebP, BMP, TIFF и SVG.
settings-default-apps-text = Текстовые файлы
settings-default-apps-text-detail = Обычный текст, журналы, код и другой текст.
settings-default-apps-sheets = Таблицы
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) и CSV.
settings-default-apps-documents = Документы
settings-default-apps-documents-detail = Word (docx) и текст OpenDocument (odt).
settings-default-apps-katna = Просмотрщик Katna Mail
settings-default-apps-system = Приложение по умолчанию на рабочем столе
settings-default-apps-ask = Каждый раз спрашивать, каким приложением
settings-default-apps-after-saving = После сохранения
settings-default-apps-show-folder = Показывать сохранённые файлы в их папке
settings-default-apps-show-folder-detail = Открывает файловый менеджер с выделенными сохранёнными вложениями

## Settings > Compose

settings-compose-send-from = Отправлять новые письма с
settings-compose-send-from-detail = Ответы и пересылки всегда уходят с аккаунта, в котором вы находитесь.
settings-compose-send-from-current = Текущего аккаунта
settings-compose-send-on-replies = Отправка ответов
settings-compose-send-on-replies-detail = Что делает кнопка «Отправить» при ответе или пересылке. Другой вариант — в меню рядом с ней.
settings-compose-send-plain = Отправить
settings-compose-send-archive = Отправить и архивировать
settings-compose-signatures = Подписи
settings-compose-signatures-detail = Добавляется под вашим письмом после строки «--». Другую подпись можно выбрать в окне нового письма.
settings-compose-untitled = Без названия
settings-compose-signature-name = Название, например «Работа»
settings-compose-signature-first = Моя подпись
settings-compose-signature-numbered = Подпись { $number }
settings-compose-signature-delete = Удалить
settings-compose-signature-deleted = Подпись удалена
settings-compose-signature-new = Создать
settings-compose-no-signatures = Подписей пока нет.
settings-compose-no-signature = Без подписи
settings-compose-for-new-mail = Для новых писем
settings-compose-for-replies = Для ответов и пересылок
settings-compose-for-replies-detail = В цепочке, где вы уже подписали письмо, ответ начинается с той же подписи.
settings-compose-format = Формат
settings-compose-plain-text = Писать обычным текстом
settings-compose-plain-text-detail = Новые письма начинаются без форматирования; в окне письма это можно переключить
settings-compose-spelling = Орфография
settings-compose-spell-check = Проверять орфографию при вводе
settings-compose-spell-check-detail = Слова с ошибками подчёркиваются, варианты — по правому щелчку
settings-compose-spell-desktop = Язык рабочего стола ({ $language })
settings-compose-templates = Шаблоны
settings-compose-templates-detail = Сохраняйте письма, которые часто пишете, и начинайте с них новое письмо или ответ.

## Settings > Shortcuts

settings-shortcuts-set = Набор сочетаний
settings-shortcuts-set-detail = Начните с клавиш знакомого почтового приложения. Cmd здесь — это Ctrl. Ваши изменения сохраняются поверх набора, а «Восстановить по умолчанию» возвращает клавиши набора.
settings-shortcuts-single = Сочетания из одной клавиши
settings-shortcuts-single-detail = Клавиши без Ctrl и Alt, как в веб-почте: e архивирует, j и k перемещают, / ищет. Работают в списке и в открытой цепочке, но не во время ввода текста.
settings-shortcuts-single-use = Использовать сочетания из одной клавиши
settings-shortcuts-single-use-detail = Сочетания с Ctrl работают всегда
settings-shortcuts-how = Нажмите на клавишу, чтобы изменить её, или на +, чтобы добавить, затем нажмите новые клавиши. Esc — отмена.
settings-shortcuts-restore = Восстановить по умолчанию
settings-shortcuts-no-key = Нет клавиши
settings-shortcuts-press = Нажмите клавиши…
settings-shortcuts-then = { $keys }, затем…
settings-shortcuts-moved = { $keys } теперь выполняет «{ $action }» вместо «{ $previous }».
settings-shortcuts-single-off = Сочетания из одной клавиши выключены, поэтому эта клавиша заработает, когда вы их включите.
settings-shortcuts-restored = Всем сочетаниям снова назначены клавиши набора.

## Settings search: the line under a result

settings-general-language-summary = Язык приложения, дат и чисел
settings-general-reading-summary = Сначала новые письма, полные заголовки, полные имена получателей
settings-general-mark-read-summary = Когда открытая цепочка отмечается как прочитанная: сразу, через 1 или 3 секунды или вручную
settings-general-reply-button-summary = Кнопка ответа рядом с каждым письмом отвечает всем
settings-general-remote-images-summary = Всегда показывать изображения в каждом письме
settings-general-sending-summary = Отмена отправки: сколько отправленное письмо ждёт, чтобы его можно было отменить
settings-general-offline-summary = За сколько дней недавние письма загружаются целиком, чтобы читать их без подключения
settings-general-notifications-summary = Уведомления о новых письмах и их звук
settings-general-desktop-summary = Открытие Katna Mail при входе в систему, значок в системном лотке и счётчик непрочитанных на значке в панели задач
settings-accounts-accounts-summary = Добавить или удалить аккаунт либо сменить его изображение
settings-appearance-density-summary = Обычные или компактные строки в списке
settings-appearance-scaling-summary = Сделать всё крупнее или мельче: текст, значки, отступы и разделители
settings-appearance-theme-summary = Как на рабочем столе, светлая или тёмная
settings-appearance-sender-pictures-summary = Логотипы компаний, найденные по домену отправителя
settings-appearance-important-summary = Маркер важности рядом с каждым письмом в списке
settings-appearance-mail-colors-summary = Тёмные цвета для HTML-писем в тёмной теме или цвета отправителя
settings-appearance-attachment-previews-summary = Миниатюра содержимого каждого вложения
settings-shortcuts-set-summary = Начать с клавиш Gmail, Inbox by Gmail, Apple Mail, Outlook или Thunderbird
settings-shortcuts-single-summary = Клавиши без Ctrl и Alt, как в веб-почте
settings-default-apps-pdf-summary = Где открываются вложения PDF
settings-default-apps-pictures-summary = Где открываются фотографии и изображения
settings-default-apps-text-summary = Где открываются обычный текст, журналы и код
settings-default-apps-sheets-summary = Где открываются файлы Excel, OpenDocument и CSV
settings-default-apps-documents-summary = Где открываются документы Word и OpenDocument
settings-default-apps-after-saving-summary = Показывать сохранённые вложения в их папке
settings-compose-send-from-summary = Аккаунт, с которого уходят новые письма: текущий или всегда один и тот же
settings-compose-send-on-replies-summary = «Отправить» или «Отправить и архивировать» цепочку при ответах и пересылках
settings-compose-signatures-summary = Добавляется под вашим письмом после строки «--»
settings-compose-for-new-mail-summary = Подпись, с которой начинаются новые письма
settings-compose-for-replies-summary = Подпись, с которой начинаются ответы и пересылки
settings-compose-format-summary = Писать новые письма обычным текстом
settings-compose-spelling-summary = Проверка орфографии при вводе и язык словаря
settings-compose-templates-summary = Скоро: сохраняйте письма, которые часто пишете, и начинайте с них новое письмо или ответ
settings-feedback-crash-reports-summary = Сохранять отчёты о сбоях на этом компьютере, когда Katna Mail или её фоновая служба аварийно завершается
settings-feedback-saved-summary = Просмотр, копирование и удаление отчётов о сбоях, сохранённых на этом компьютере
settings-feedback-help-improve-summary = Отправлять отчёты о сбоях, чтобы помочь исправить ошибки; выключено, пока вы не включите
settings-experimental-blur-summary = Рабочий стол размыто просвечивает сквозь верхнюю панель, а меню — из матового стекла
settings-search-shortcut = Сочетание клавиш
settings-search-tab = Вкладка настроек
settings-search-none = Нет настроек по запросу «{ $query }».
settings-search-results = Настройки по запросу «{ $query }»

## Quick settings (the panel that slides in from the right)

quick-title = Быстрые настройки
quick-see-all = Все настройки
quick-reading-pane = Область просмотра
quick-pane-right = Справа от списка
quick-pane-none = Без разделения
quick-density = Плотность
quick-density-default = Обычная
quick-density-compact = Компактная
quick-theme = Тема
quick-theme-system = Как на рабочем столе
quick-theme-light = Светлая
quick-theme-dark = Тёмная
quick-desktop-colors = Цвета рабочего стола
quick-desktop-colors-detail = Цветовая схема и акцентный цвет рабочего стола
quick-app-names = Названия приложений
quick-app-names-detail = Подписи под значками приложений слева
quick-inbox-tabs = Вкладки «Входящих»
quick-inbox-tabs-detail = Вкладки почтового сервиса каждого аккаунта
quick-choose-tabs = Выбрать вкладки
quick-choose-tabs-detail = Для каждого аккаунта, в настройках
quick-sending = Отправка
quick-undo-send = Отмена отправки
quick-undo-send-off = Выкл.
quick-undo-send-seconds = { $seconds } с
quick-signatures = Подписи
quick-signatures-none = Пока нет
quick-signatures-one = { $name }, по умолчанию
quick-signatures-many = { $count ->
    [one] { $count } подпись; по умолчанию — { $name }
    [few] { $count } подписи; по умолчанию — { $name }
    [many] { $count } подписей; по умолчанию — { $name }
   *[other] { $count } подписи; по умолчанию — { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, без подписи по умолчанию
    [few] { $count }, без подписи по умолчанию
    [many] { $count }, без подписи по умолчанию
   *[other] { $count }, без подписи по умолчанию
}
quick-signature-untitled = Без названия
quick-threading = Цепочки писем
quick-conversation-view = Группировка в цепочки
quick-conversation-view-detail = Группировать ответы на одно письмо
quick-help = Справка
quick-tour = Пройти обзор
quick-whats-new = Что нового
quick-about = О Katna

## Settings: opening at login

settings-open-at-login-failed = Не удалось изменить запуск при входе в систему: { $error }

## Settings > Appearance > Scaling

scale-letter = А
scale-percent = { $percent } %
scale-reset = Вернуть { $percent } %

## Settings > Experimental > Look & Feel

look-intro = Функции, которые ещё проходят испытания. Они могут измениться или исчезнуть.
look-heading = Внешний вид и поведение
look-window-frame = Рамка окна
look-window-frame-detail = Кто рисует заголовок, кнопки окна, углы и тень.
look-frame-native-kde = Системная: рамка KDE в вашей теме Plasma
look-frame-native = Системная: рамка рабочего стола
look-frame-katna = Katna: верхняя панель становится заголовком окна
look-frame-katna-note-named = Katna рисует скруглённые углы и собственную тень. Рамка больше не следует теме { $desktop }; правила окон по-прежнему действуют.
look-frame-katna-note = Katna рисует скруглённые углы и собственную тень. Рамка больше не следует теме рабочего стола; правила окон по-прежнему действуют.
look-frame-client-side = Ваш рабочий стол оставляет рамку каждому приложению, поэтому Katna уже рисует свою.
look-blurred-background = Размытый фон
look-blurred-background-detail = Рабочий стол размыто просвечивает сквозь верхнюю панель и папки, а меню и всплывающие окна — из матового стекла.
look-blur = Размывать то, что за окном
look-blur-detail = Письма остаются на непрозрачных карточках, поэтому текст сохраняет контраст
look-blur-off-kde = Эффект размытия KDE выключен. Включите «Размытие» в «Параметрах системы» → «Управление окнами» → «Эффекты рабочего стола», затем снова откройте Katna Mail.
look-blur-none-gnome = GNOME не размывает то, что за окнами.
look-blur-none-x11 = Ваш оконный менеджер не размывает то, что за окнами.
look-blur-none-wayland = Ваш композитор не размывает то, что за окнами.

## Settings > User feedback (crash reports)

feedback-intro-sending = Новые отчёты о сбоях отправляются, чтобы помочь исправить ошибки. Больше ничего не покидает этот компьютер.
feedback-intro-local = Katna ничего никуда не отправляет. Отчёты о сбоях остаются на этом компьютере — их можно посмотреть или приложить к сообщению об ошибке.
feedback-crash-reports = Отчёты о сбоях
feedback-crash-reports-detail = Создаются, когда Katna Mail или её фоновая служба аварийно завершается.
feedback-save = Сохранять отчёты о сбоях на этом компьютере
feedback-save-detail = Домашняя папка, имена пользователя и компьютера и адреса электронной почты не включаются
feedback-saved = Сохранённые отчёты о сбоях
feedback-saved-detail = { $count ->
    [one] Хранится { $count } последний отчёт.
    [few] Хранятся { $count } последних отчёта.
    [many] Хранятся { $count } последних отчётов.
   *[other] Хранятся { $count } последнего отчёта.
}
feedback-help-improve = Помочь улучшить Katna
feedback-help-improve-detail = Выключено, пока вы не включите, и здесь это можно выключить в любой момент.
feedback-send = Отправлять отчёты о сбоях
feedback-send-detail = Сохранённый отчёт — ровно такой, каким вы видите его здесь, — отправляется в систему отслеживания сбоев Katna (Sentry, в ЕС). Без IP-адреса, писем и адресов электронной почты
feedback-none-saved = Сохранённых отчётов о сбоях нет.
feedback-delete-all = Удалить все
feedback-app-daemon = Фоновая служба
feedback-report-sent = { $date } · Отправлен
feedback-view = Открыть
feedback-view-tooltip = Открыть отчёт
feedback-copy-tooltip = Скопировать, чтобы вставить в сообщение об ошибке
feedback-copied = Отчёт о сбое скопирован.
feedback-deleted-all = Отчёты о сбоях удалены.
feedback-read-failed = Не удалось прочитать отчёт о сбое: { $error }
feedback-delete-failed = Не удалось удалить отчёт о сбое: { $error }
feedback-delete-all-failed = Не удалось удалить отчёты о сбоях: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Файл
desktop-menu-new-message = _Новое письмо
desktop-menu-quit = _Выход
desktop-menu-edit = _Правка
desktop-menu-undo = _Отменить
desktop-menu-select-all = Выделить _всё
desktop-menu-select-none = _Снять выделение
desktop-menu-find = _Найти…
desktop-menu-view = _Вид
desktop-menu-folder-list = Показывать _список папок
desktop-menu-refresh = _Обновить
desktop-menu-go = Пере_ход
desktop-menu-inbox = _Входящие
desktop-menu-starred = _Помеченные
desktop-menu-sent = _Отправленные
desktop-menu-drafts = _Черновики
desktop-menu-all-mail = Вс_я почта
desktop-menu-next = _Следующая цепочка
desktop-menu-previous = П_редыдущая цепочка
desktop-menu-message = П_исьмо
desktop-menu-open = О_ткрыть
desktop-menu-reply = _Ответить
desktop-menu-reply-all = Ответить _всем
desktop-menu-forward = _Переслать
desktop-menu-archive = _Архивировать
desktop-menu-delete = _Удалить
desktop-menu-spam = В _спам
desktop-menu-move-to = Перемест_ить в…
desktop-menu-mark-read = Отметить как п_рочитанное
desktop-menu-mark-unread = Отметить как _непрочитанное
desktop-menu-star = Пом_етить
desktop-menu-important = Отметить как ва_жное
desktop-menu-not-important = Отметить как не_важное
desktop-menu-settings = _Настройка
desktop-menu-quick-settings = _Быстрые настройки
desktop-menu-configure = _Настроить Katna Mail…
desktop-menu-help = _Справка
desktop-menu-shortcuts = Быстрые _клавиши
desktop-menu-whats-new = _Что нового
desktop-menu-about = _О Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Перемещение
shortcut-group-actions = Действия
shortcut-group-go-to = Переход
shortcut-group-app = Приложение

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Следующая цепочка
shortcut-previous = Предыдущая цепочка
shortcut-down = Вниз по списку
shortcut-up = Вверх по списку
shortcut-first = В начало списка
shortcut-last = В конец списка
shortcut-page-down = На страницу вниз по списку
shortcut-page-up = На страницу вверх по списку
shortcut-open = Открыть цепочку
shortcut-back = Вернуться к списку
shortcut-scroll-down = Прокрутить вниз
shortcut-scroll-up = Прокрутить вверх
shortcut-scroll-page-down = Прокрутить на страницу вниз
shortcut-scroll-page-up = Прокрутить на страницу вверх
shortcut-compose = Написать
shortcut-reply = Ответить
shortcut-reply-all = Ответить всем
shortcut-forward = Переслать
shortcut-archive = Архивировать
shortcut-delete = Удалить
shortcut-spam = В спам
shortcut-move-to = Переместить в
shortcut-mark-read = Отметить как прочитанное
shortcut-mark-unread = Отметить как непрочитанное
shortcut-star = Пометить или снять пометку
shortcut-important = Отметить как важное
shortcut-not-important = Отметить как неважное
shortcut-check = Выбрать цепочку
shortcut-select-all = Выбрать все цепочки
shortcut-select-none = Отменить выбор всех цепочек
shortcut-undo = Отменить последнее действие
shortcut-go-inbox = Входящие
shortcut-go-starred = Помеченные
shortcut-go-sent = Отправленные
shortcut-go-drafts = Черновики
shortcut-go-all = Вся почта
shortcut-search = Поиск в почте
shortcut-navigation = Показать или свернуть меню
shortcut-quick-settings = Быстрые настройки
shortcut-settings = Все настройки
shortcut-shortcuts = Быстрые клавиши
shortcut-reload = Проверить почту
shortcut-quit = Выйти

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first }, затем { $second }

## Settings > Accounts

accounts-folder-pane = Панель папок
accounts-folder-pane-detail = Папки каких аккаунтов показывает панель слева.
accounts-shown-one = Один аккаунт за раз; переключение в карточке аккаунта
accounts-shown-all = Все аккаунты, один за другим
accounts-row = Аккаунты
accounts-row-detail = При удалении аккаунта удаляется копия его почты, которую Katna хранит на этом компьютере. На сервере почта остаётся.
accounts-none = Аккаунтов пока нет.
accounts-kind-imported = Импортирован
accounts-picture-reset = Взять изображение из системы
accounts-picture-change = Сменить изображение
accounts-remove = Удалить
accounts-delete-all-row = Удалить все данные
accounts-delete-all-row-detail = Начать заново, как после новой установки.
accounts-delete-all-about = Удаляет с этого компьютера все аккаунты, всю сохранённую почту, контакты и календари, поисковый индекс, ваши настройки и сохранённые пароли. На почтовых серверах ничего не меняется.
accounts-delete-all-open = Удалить все данные Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } удалён из Katna.
accounts-removed = { $address } удалён из Katna. Его почта по-прежнему на сервере.
accounts-all-deleted = Все данные Katna удалены с этого компьютера.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Удалить { $address }?
accounts-remove-confirm = Удалить аккаунт
accounts-removing = Удаление…
accounts-remove-local-mail = { $folders ->
    [0] Вся почта, импортированная в этот аккаунт
    [1] Вся почта, импортированная в этот аккаунт, в его папке
    [one] Вся почта, импортированная в этот аккаунт, в его { $folders } папке
    [few] Вся почта, импортированная в этот аккаунт, в его { $folders } папках
    [many] Вся почта, импортированная в этот аккаунт, в его { $folders } папках
   *[other] Вся почта, импортированная в этот аккаунт, в его { $folders } папках
}
accounts-remove-local-settings = Его настройки в Katna
accounts-remove-mail = { $folders ->
    [0] Вся почта этого аккаунта, сохранённая Katna
    [1] Вся почта этого аккаунта, сохранённая Katna в его папке
    [one] Вся почта этого аккаунта, сохранённая Katna в его { $folders } папке
    [few] Вся почта этого аккаунта, сохранённая Katna в его { $folders } папках
    [many] Вся почта этого аккаунта, сохранённая Katna в его { $folders } папках
   *[other] Вся почта этого аккаунта, сохранённая Katna в его { $folders } папках
}
accounts-remove-outbox = Его письма, ожидающие в исходящих
accounts-remove-settings = Его сохранённый пароль и настройки в Katna
accounts-delete-all-title = Удалить все данные Katna?
accounts-delete-all-confirm = Удалить всё
accounts-deleting = Удаление…
accounts-delete-all-accounts = Все аккаунты, а также вся почта и вложения, сохранённые Katna
accounts-delete-all-contacts = Контакты, календари и поисковый индекс
accounts-delete-all-settings = Все настройки, подписи и быстрые клавиши
accounts-delete-all-passwords = Все сохранённые пароли
accounts-deleted-heading = Удаляется с этого компьютера:
accounts-cannot-undo = Это действие нельзя отменить.
accounts-server-delete-all = На ваших почтовых серверах ничего не меняется: почта остаётся там, и если снова добавить аккаунт, она загрузится заново. Почта, импортированная из файлов, есть только в Katna; сами файлы не затрагиваются.
accounts-server-local = Эта почта импортирована из файлов, поэтому единственная её копия — в Katna. Исходные файлы не затрагиваются; импортируйте их снова, чтобы вернуть почту.
accounts-server-remove = На почтовом сервере ничего не меняется: почта остаётся там, и если снова добавить аккаунт, она загрузится заново.
accounts-confirm-word = удалить
accounts-confirm-placeholder = Введите «{ accounts-confirm-word }»
accounts-confirm-prompt = Для подтверждения введите «{ accounts-confirm-word }»:
accounts-cancel = Отмена
