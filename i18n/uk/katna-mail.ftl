# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Мова: { $language }
language-tooltip-system = Мова: { $language }, як у системі
language-search = Пошук мови
language-system-default = Як у системі
language-system-now = Зараз: { $language }
language-no-match = Не знайдено мови за запитом «{ $query }»
language-machine = Перекладено машинно. Допоможіть покращити
language-setting = Мова
language-setting-detail = Мова меню, кнопок і повідомлень, а також формат дат і чисел. Варіант «Як у системі» використовує налаштування стільниці.

## Dates and sizes

ago-just-now = щойно
ago-minutes = { $count ->
    [one] { $count } хвилину тому
    [few] { $count } хвилини тому
    [many] { $count } хвилин тому
   *[other] { $count } хвилини тому
}
ago-hours = { $count ->
    [one] { $count } годину тому
    [few] { $count } години тому
    [many] { $count } годин тому
   *[other] { $count } години тому
}
ago-days = { $count ->
    [one] { $count } день тому
    [few] { $count } дні тому
    [many] { $count } днів тому
   *[other] { $count } дня тому
}
size-bytes = { $count ->
    [one] { $count } байт
    [few] { $count } байти
    [many] { $count } байтів
   *[other] { $count } байта
}
size-kb = { $size } КБ
size-mb = { $size } МБ
size-gb = { $size } ГБ
size-tb = { $size } ТБ

## Top bar

folders-hide = Сховати папки
folders-show = Показати папки
compose = Написати
search = Пошук
search-mail = Пошук у пошті
search-settings = Пошук у налаштуваннях
search-clear = Очистити пошук
search-options-show = Показати параметри пошуку
settings = Налаштування
account-add = Додати обліковий запис

## App rail (and the bottom bar on a phone)

rail-mail = Пошта
rail-calendar = Календар
rail-contacts = Контакти
rail-tasks = Завдання
rail-notes = Нотатки
rail-feeds = Стрічки

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Незабаром
app-calendar-promise = Ваші календарі CalDAV, запрошення на зустрічі з пошти й нагадування — поруч із вхідними.
app-tasks-promise = Списки справ, що синхронізуються через CalDAV, і завдання, створені з листів.
app-notes-promise = Швидкі нотатки й нотатки до листа чи ланцюжка на потім.
app-feeds-promise = Читайте стрічки RSS і Atom поруч із поштою.

## Contacts page

app-contacts-loading = Збираємо людей із вашої пошти…
app-contacts-empty = Тут з’являться люди, з якими ви листуєтеся.
app-contacts-count = { $count ->
    [one] { $count } людина з вашої пошти, спершу найчастіші співрозмовники
    [few] { $count } людини з вашої пошти, спершу найчастіші співрозмовники
    [many] { $count } людей з вашої пошти, спершу найчастіші співрозмовники
   *[other] { $count } людини з вашої пошти, спершу найчастіші співрозмовники
}
app-contacts-top = { $count ->
    [one] { $count } найчастіший співрозмовник із вашої пошти
    [few] { $count } найчастіші співрозмовники з вашої пошти
    [many] { $count } найчастіших співрозмовників із вашої пошти
   *[other] { $count } найчастішого співрозмовника з вашої пошти
}
app-contacts-messages = { $count ->
    [one] { $count } лист
    [few] { $count } листи
    [many] { $count } листів
   *[other] { $count } листа
}
app-contacts-last = останній: { $date }

## Navigation (the folders pane)

nav-labels = Мітки
nav-folders = Папки
nav-label-new = Створити мітку
nav-folder-new = Створити папку
nav-account-unnamed = Обліковий запис { $number }
nav-tab-new = { $count ->
    [one] { $count } новий
    [few] { $count } нові
    [many] { $count } нових
   *[other] { $count } нового
}

## Special folders (the user's own folders keep their names)

folder-inbox = Вхідні
folder-starred = Із зірочкою
folder-drafts = Чернетки
folder-sent = Надіслані
folder-archive = Архів
folder-spam = Спам
folder-trash = Кошик
folder-all-mail = Уся пошта
folder-scheduled = Заплановані

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Нова мітка
label-folder-new-title = Нова папка
label-prompt = Введіть назву нової мітки:
label-folder-prompt = Введіть назву нової папки:
label-name-hint = Назва мітки
label-folder-name-hint = Назва папки
label-nest = Вкласти мітку в:
label-folder-nest = Вкласти папку в:
label-cancel = Скасувати
label-create = Створити
label-creating = Створення…
label-created = Мітку «{ $name }» створено.
label-folder-created = Папку «{ $name }» створено.

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

## Reading pane: toolbar

reader-close = Закрити
reader-back = Назад
reader-mark-unread = Позначити як непрочитане
reader-move-to = Перемістити в
reader-more = Більше
reader-print-all = Надрукувати все
reader-new-window = У новому вікні
reader-position = { $position } з { $total }
reader-newer = Новіший
reader-older = Старіший

## Reading pane: the conversation

reader-removed = Цей ланцюжок видалено.
reader-no-subject = (без теми)
reader-collapse-all = Згорнути все
reader-expand-all = Розгорнути все
reader-unknown-sender = (невідомий відправник)
reader-date-ago = { $date } ({ $ago })
reader-me = мені
reader-to = кому: { $names }
reader-starred = Із зірочкою
reader-not-starred = Без зірочки
reader-too-long = Лист задовгий, щоб показати його повністю.
reader-encrypted-images = У зашифрованих листах зображення з інтернету ніколи не завантажуються.
reader-window-failed = Не вдалося відкрити нове вікно.

## Reading pane: message details (opened from "to me")

reader-details-from = від:
reader-details-to = кому:
reader-details-cc = копія:
reader-details-date = дата:
reader-details-subject = тема:

## Reading pane: downloading a message

reader-downloading = Завантаження цього листа із сервера…
reader-download-failed = Не вдалося завантажити цей лист.
reader-try-again = Повторити спробу

## Reply row

reply-reply = Відповісти
reply-reply-all = Відповісти всім
reply-forward = Переслати

## Encrypted and signed mail

security-decrypting = Розшифрування…
security-checking = Перевірка підпису…
security-partly-encrypted = Зашифровано лише частину цього листа. Решту додано поза захистом, і її міг надіслати будь-хто.
security-partly-signed = Підписано лише частину цього листа. Решту додано поза захистом, і її міг надіслати будь-хто.
security-encrypted = Зашифрований лист
security-encrypted-smime = Зашифрований лист (S/MIME)
security-no-key = Не вдається розшифрувати лист: його зашифровано для ключа, якого у вас немає.
security-cancelled = Розшифрування скасовано.
security-damaged = Не вдається розшифрувати лист: зашифровані дані пошкоджено або змінено.
security-decrypt-unavailable = Не вдається розшифрувати лист: установіть { $tool }, щоб читати зашифровану пошту.
security-decrypt-failed = Не вдається розшифрувати лист: { $reason }
security-unknown-signer = невідомий автор підпису
security-signed-verified = Підписано: { $signer } · підпис перевірено
security-signed-not-sender = Підписано: { $signer } — це не відправник
security-signed-untrusted = Підписано: { $signer }, ключем, який ви позначили як ненадійний
security-signed-unverified = Підписано: { $signer } · ключ не перевірено
security-bad-signature = Недійсний підпис: лист змінено після підписання, або підпис підроблено.
security-signature-expired = Підписано: { $signer } · термін дії підпису минув
security-key-expired = Підписано: { $signer } · термін дії ключа відтоді минув
security-key-revoked = Підписано: { $signer }, ключем, який було відкликано
security-missing-key = Підписано ключем, якого у вас немає, тому підпис не можна перевірити
security-missing-key-id = Підписано ключем, якого у вас немає ({ $key }), тому підпис не можна перевірити
security-signature-unavailable = Підписано; установіть { $tool }, щоб перевірити підпис
security-signature-error = Не вдалося перевірити підпис.

## Remote images and pictures

remote-hidden = Зображення в цьому листі приховано.
remote-show = Показати зображення
remote-always-show = Завжди показувати від цього відправника
remote-picture-use = Вибрати
remote-picture-too-big = Виберіть зображення розміром не більше 8 МБ.
remote-picture-type = Виберіть зображення PNG, JPEG, GIF, WebP або SVG.
remote-picture-read-failed = Не вдається прочитати зображення: { $error }
remote-picture-keep-failed = Не вдається зберегти зображення: { $error }
remote-picture-remove-failed = Не вдається видалити зображення: { $error }

## Attachments

attachment-count = { $count ->
    [one] { $count } вкладення
    [few] { $count } вкладення
    [many] { $count } вкладень
   *[other] { $count } вкладення
}
attachment-save = Зберегти
attachment-save-all = Зберегти все
attachment-save-all-tooltip = Зберегти всі вкладення в папку
attachment-save-here = Зберегти тут
attachment-not-downloaded = Цей лист не завантажено.
attachment-not-found = Це вкладення не знайдено в листі.
attachment-read-failed = Не вдалося прочитати { $name }
attachment-numbered = вкладення { $number }
attachment-saved-all = { $count ->
    [one] { $count } файл збережено в папку «{ $place }»
    [few] { $count } файли збережено в папку «{ $place }»
    [many] { $count } файлів збережено в папку «{ $place }»
   *[other] { $count } файлу збережено в папку «{ $place }»
}
attachment-saved-some = { $total ->
    [one] Збережено { $saved } з { $total } файлу в папку «{ $place }». Не вдалося зберегти { $failed }
    [few] Збережено { $saved } з { $total } файлів у папку «{ $place }». Не вдалося зберегти { $failed }
    [many] Збережено { $saved } з { $total } файлів у папку «{ $place }». Не вдалося зберегти { $failed }
   *[other] Збережено { $saved } з { $total } файлу в папку «{ $place }». Не вдалося зберегти { $failed }
}
attachment-saved-to = Збережено в { $path }
attachment-save-failed = Не вдалося зберегти { $name }: { $error }
attachment-open-failed = Не вдалося відкрити { $name }: { $error }
attachment-risky = Цей файл може запустити програму, тому Katna його не відкриває. Натомість збережіть його.
attachment-encrypted-open = Цей файл надійшов зашифрованим. Збережіть його, щоб відкрити в іншій програмі.

## Printing

print-failed = Не вдалося надрукувати: { $error }
print-no-font = не знайдено шрифту
print-opened-as-pdf = Відкрито як PDF, щоб надрукувати звідти.
print-not-downloaded = (Ще не завантажено.)
print-encrypted = (Зашифровано. Відкрийте лист у Katna Mail, щоб надрукувати його текст.)
print-to = Кому: { $addresses }
print-cc = Копія: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Відкрийте цей лист, щоб переглянути вкладення.
text-copy = Копіювати
text-select-all = Вибрати все

## Settings page: its tabs

settings-tab-general = Загальні
settings-tab-inbox = Вхідні
settings-tab-accounts = Облікові записи
settings-tab-subscriptions = Підписки
settings-tab-appearance = Вигляд
settings-tab-shortcuts = Комбінації клавіш
settings-tab-default-apps = Типові програми
settings-tab-folders-rules = Папки й правила
settings-tab-compose = Написання листів
settings-tab-mcp-server = Сервер MCP
settings-tab-feedback = Відгуки користувачів
settings-tab-experimental = Експериментальні

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Переглядайте розсилки й списки розсилки, які отримуєте, і відписуйтеся одним клацанням.
settings-tab-folders-rules-coming = Створюйте, перейменовуйте, переміщуйте й приховуйте папки та мітки, вибирайте, які з них синхронізувати. Правила самі сортують, позначають мітками, пересилають або видаляють нові листи — за відправником, темою чи словами.
settings-tab-mcp-server-coming = Дозвольте помічникам зі ШІ на цьому комп’ютері шукати, читати й готувати чернетки ваших листів — з вашої згоди.

## Settings > General

settings-general-conversations = Ланцюжки листів
settings-general-conversations-group = Групувати відповіді на той самий лист
settings-general-conversations-group-detail = Один рядок на ланцюжок у списку
settings-general-reading = Читання
settings-general-newest-first = Спочатку найновіший лист
settings-general-newest-first-detail = Ланцюжок починається з останньої відповіді
settings-general-full-headers = Показувати повні заголовки
settings-general-full-headers-detail = Від, кому, копія, дата й тема відкриті в кожному листі
settings-general-full-names = Повні імена одержувачів
settings-general-full-names-detail = «мені, Ada Lovelace», а не «мені, Ada»
settings-general-mark-read = Позначати як прочитане
settings-general-mark-read-now = Одразу після відкриття
settings-general-mark-read-1s = Через 1 секунду після відкриття
settings-general-mark-read-3s = Через 3 секунди після відкриття
settings-general-mark-read-never = Лише коли я позначу сам
settings-general-reply-button = Кнопка «Відповісти»
settings-general-reply-all = Відповідати всім
settings-general-reply-all-detail = Кнопка відповіді біля кожного листа відповідає всім, а не лише відправникові
settings-general-remote-images = Зображення з інтернету
settings-general-remote-images-detail = Завантаження зображень листа повідомляє відправникові, що ви його відкрили, коли і приблизно де. Якщо вимкнено, кожен лист спершу запитує, і ви завжди можете показати зображення відправника.
settings-general-remote-images-always = Завжди показувати зображення
settings-general-remote-images-always-detail = У кожному листі, а не лише від надійних відправників
settings-general-sending = Надсилання
settings-general-sending-detail = Скільки надісланий лист чекає, щоб його можна було скасувати.
settings-general-offline = Пошта офлайн
settings-general-offline-detail = Нещодавні листи завантажуються повністю, щоб читати їх без з’єднання. Старіші завантажуються, коли ви їх відкриваєте.
settings-general-offline-days = { $count ->
    [one] { $count } день
    [few] { $count } дні
    [many] { $count } днів
   *[other] { $count } дня
}
settings-general-offline-years = { $count ->
    [one] { $count } рік
    [few] { $count } роки
    [many] { $count } років
   *[other] { $count } року
}
settings-general-offline-all = Уся пошта
settings-general-offline-note = Якщо вибрати менше днів, уже завантажені листи залишаться. На сервері нічого не змінюється.
settings-general-notifications = Сповіщення
settings-general-notifications-detail = Про нові листи у «Вхідних», навіть коли Katna Mail закрито.
settings-general-new-mail = Сповіщати про нові листи
settings-general-new-mail-detail = З кнопками «Відповісти всім», «Позначити як прочитане» й «Архівувати»
settings-general-new-mail-sound = Відтворювати звук
settings-general-new-mail-sound-detail = Звук нової пошти стільниці
settings-general-desktop = Стільниця
settings-general-open-at-login = Відкривати Katna Mail під час входу
settings-general-open-at-login-detail = Пошта однаково синхронізується під час входу, поки працює служба
settings-general-tray = Показувати Katna в системному лотку
settings-general-tray-detail = Із лічильником непрочитаних і меню
settings-general-unread-badge = Лічильник непрочитаних на значку панелі завдань
settings-general-unread-badge-detail = Скільки листів у «Вхідних» не прочитано

## Settings > Inbox

settings-inbox-tabs = Вкладки «Вхідних»
settings-inbox-tabs-detail = Сортувати вхідні за вкладками, як це робить сайт вашого поштового сервісу.
settings-inbox-tabs-show = Показувати вкладки «Вхідних»
settings-inbox-tabs-show-detail = Якщо вимкнено, для кожного облікового запису один список
settings-inbox-no-accounts = Додайте обліковий запис, щоб вибрати його вкладки.
settings-inbox-tabs-automatic = Автоматично: { $tabs } ({ $provider })
settings-inbox-tabs-off = Без вкладок
settings-inbox-tabs-gmail = Основні, Реклама, Соцмережі, Оновлення, Форуми
settings-inbox-tabs-focused = Пріоритетні та Інші
settings-inbox-tabs-zoho = Вхідні, Розсилки та Сповіщення
settings-inbox-tabs-shown = Показані вкладки. Листи з вимкненої вкладки залишаються у вкладці «{ $tab }».

## Settings > Appearance

settings-appearance-reading-pane = Панель читання
settings-appearance-reading-pane-detail = Де показується відкритий ланцюжок.
settings-appearance-pane-right = Праворуч від списку
settings-appearance-pane-none = Без поділу
settings-appearance-density = Щільність
settings-appearance-density-default = Типова
settings-appearance-density-compact = Компактна
settings-appearance-scaling = Масштаб
settings-appearance-scaling-detail = Робить усе в Katna Mail більшим або меншим, понад власний масштаб стільниці: текст, значки, відступи й роздільники. Листи, які ви надсилаєте, зберігають свій розмір шрифту. За дуже малого масштабу по значках важко влучити.
settings-appearance-theme = Тема
settings-appearance-theme-system = Як на стільниці
settings-appearance-theme-light = Світла
settings-appearance-theme-dark = Темна
settings-appearance-desktop-colors = Кольори стільниці
settings-appearance-desktop-colors-use = Використовувати кольори стільниці
settings-appearance-desktop-colors-use-detail = Колірна схема й колір акценту стільниці
settings-appearance-app-names = Назви програм
settings-appearance-app-names-show = Показувати назви програм
settings-appearance-app-names-show-detail = Підписи під значками програм ліворуч
settings-appearance-sender-pictures = Зображення відправників
settings-appearance-sender-pictures-show = Показувати логотипи компаній
settings-appearance-sender-pictures-show-detail = Шукаються за доменом відправника, ніколи за листом, і зберігаються тиждень
settings-appearance-important = Позначки важливості
settings-appearance-important-show = Показувати позначки важливості
settings-appearance-important-show-detail = Біля кожного листа в списку
settings-appearance-message-width = Ширина листа
settings-appearance-message-width-limit = Обмежити ширину листів
settings-appearance-message-width-limit-detail = Так у широкому вікні легше читати довгі рядки
settings-appearance-mail-colors = Кольори листів
settings-appearance-mail-colors-detail = Більшість листів розраховано на білу сторінку. У темній темі їхні кольори замінюються темними, які добре читаються; якщо вимкнено, лист зберігає кольори відправника на світлій сторінці.
settings-appearance-dark-mail = Темні кольори й для листів
settings-appearance-dark-mail-detail = Лише коли тема темна
settings-appearance-attachment-previews = Попередній перегляд вкладень
settings-appearance-attachment-previews-show = Показувати попередній перегляд вкладень
settings-appearance-attachment-previews-show-detail = Мініатюра вмісту кожного файлу на його картці

## Settings > Default apps

settings-default-apps-intro = Де відкриваються вкладення, коли ви їх клацаєте. Із переглядача файл завжди можна відкрити й в іншій програмі. Типові програми стільниці задаються в її власних налаштуваннях.
settings-default-apps-pdf = Файли PDF
settings-default-apps-pdf-detail = Сторінки з масштабуванням.
settings-default-apps-pictures = Зображення
settings-default-apps-pictures-detail = Фотографії (з правильною орієнтацією), PNG, GIF, WebP, BMP, TIFF і SVG.
settings-default-apps-text = Текстові файли
settings-default-apps-text-detail = Звичайний текст, журнали, код та інший текст.
settings-default-apps-sheets = Електронні таблиці
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) і CSV.
settings-default-apps-documents = Документи
settings-default-apps-documents-detail = Word (docx) і текст OpenDocument (odt).
settings-default-apps-katna = Переглядач Katna Mail
settings-default-apps-system = Типова програма стільниці
settings-default-apps-ask = Щоразу запитувати, якою програмою
settings-default-apps-after-saving = Після збереження
settings-default-apps-show-folder = Показувати збережені файли в їхній папці
settings-default-apps-show-folder-detail = Відкриває файловий менеджер із позначеними збереженими вкладеннями

## Settings > Compose

settings-compose-send-from = Надсилати нові листи з
settings-compose-send-from-detail = Відповіді й пересилання завжди надсилаються з облікового запису, у якому ви перебуваєте.
settings-compose-send-from-current = Поточного облікового запису
settings-compose-send-on-replies = Надсилання відповідей
settings-compose-send-on-replies-detail = Що робить «Надіслати» у відповіді чи пересиланні. Інший варіант — у меню біля кнопки «Надіслати».
settings-compose-send-plain = Надіслати
settings-compose-send-archive = Надіслати й архівувати
settings-compose-signatures = Підписи
settings-compose-signatures-detail = Додається під вашим листом після рядка «--». Інший підпис можна вибрати у вікні створення листа.
settings-compose-untitled = Без назви
settings-compose-signature-name = Назва, наприклад «Робота»
settings-compose-signature-first = Мій підпис
settings-compose-signature-numbered = Підпис { $number }
settings-compose-signature-delete = Видалити
settings-compose-signature-deleted = Підпис видалено
settings-compose-signature-new = Створити
settings-compose-no-signatures = Підписів ще немає.
settings-compose-no-signature = Без підпису
settings-compose-for-new-mail = Для нових листів
settings-compose-for-replies = Для відповідей і пересилань
settings-compose-for-replies-detail = У ланцюжку, де ви вже підписали лист, відповідь натомість починається з того самого підпису.
settings-compose-format = Формат
settings-compose-plain-text = Писати звичайним текстом
settings-compose-plain-text-detail = Нові листи починаються без форматування; у вікні листа це можна змінити
settings-compose-spelling = Правопис
settings-compose-spell-check = Перевіряти правопис під час введення
settings-compose-spell-check-detail = Слова з помилками підкреслюються, варіанти — за клацанням правою кнопкою
settings-compose-spell-desktop = Мова стільниці ({ $language })
settings-compose-templates = Шаблони
settings-compose-templates-detail = Зберігайте листи, які часто пишете, і починайте з них новий лист або відповідь.

## Settings > Shortcuts

settings-shortcuts-set = Набір комбінацій
settings-shortcuts-set-detail = Почніть із клавіш знайомої поштової програми. Cmd тут — це Ctrl. Ваші зміни зберігаються поверх набору, а «Відновити типові» повертає клавіші набору.
settings-shortcuts-single = Комбінації з однієї клавіші
settings-shortcuts-single-detail = Клавіші без Ctrl чи Alt, як у вебпошті: e архівує, j і k переміщують, / шукає. Працюють у списку й відкритому ланцюжку, але не під час введення тексту.
settings-shortcuts-single-use = Використовувати комбінації з однієї клавіші
settings-shortcuts-single-use-detail = Комбінації з Ctrl працюють завжди
settings-shortcuts-how = Клацніть клавішу, щоб змінити її, або +, щоб додати, потім натисніть нові клавіші. Esc скасовує.
settings-shortcuts-restore = Відновити типові
settings-shortcuts-no-key = Немає клавіші
settings-shortcuts-press = Натисніть клавіші…
settings-shortcuts-then = { $keys }, потім…
settings-shortcuts-moved = { $keys } тепер виконує «{ $action }» замість «{ $previous }».
settings-shortcuts-single-off = Комбінації з однієї клавіші вимкнено, тож ця клавіша запрацює, коли ви їх увімкнете.
settings-shortcuts-restored = Усім комбінаціям знову призначено клавіші набору.

## Settings search: the line under a result

settings-general-language-summary = Мова програми, дат і чисел
settings-general-reading-summary = Спочатку найновіший лист, повні заголовки, повні імена одержувачів
settings-general-mark-read-summary = Коли відкритий ланцюжок позначається як прочитаний: одразу, через 1 чи 3 секунди або вручну
settings-general-reply-button-summary = Кнопка відповіді біля кожного листа відповідає всім
settings-general-remote-images-summary = Завжди показувати зображення в кожному листі
settings-general-sending-summary = Скасування надсилання: скільки надісланий лист чекає, щоб його можна було скасувати
settings-general-offline-summary = За скільки днів нещодавні листи завантажуються повністю, щоб читати їх без з’єднання
settings-general-notifications-summary = Сповіщення про нові листи та їхній звук
settings-general-desktop-summary = Відкриття Katna Mail під час входу, значок у системному лотку й лічильник непрочитаних на значку панелі завдань
settings-accounts-accounts-summary = Додати чи вилучити обліковий запис або змінити його зображення
settings-appearance-density-summary = Типові чи компактні рядки в списку
settings-appearance-scaling-summary = Зробити все більшим або меншим: текст, значки, відступи й роздільники
settings-appearance-theme-summary = Як на стільниці, світла чи темна
settings-appearance-sender-pictures-summary = Логотипи компаній, знайдені за доменом відправника
settings-appearance-important-summary = Позначка важливості біля кожного листа в списку
settings-appearance-mail-colors-summary = Темні кольори для HTML-листів у темній темі або кольори відправника
settings-appearance-attachment-previews-summary = Мініатюра вмісту кожного вкладення
settings-shortcuts-set-summary = Почати з клавіш Gmail, Inbox by Gmail, Apple Mail, Outlook або Thunderbird
settings-shortcuts-single-summary = Клавіші без Ctrl чи Alt, як у вебпошті
settings-default-apps-pdf-summary = Де відкриваються вкладення PDF
settings-default-apps-pictures-summary = Де відкриваються фотографії та зображення
settings-default-apps-text-summary = Де відкриваються звичайний текст, журнали й код
settings-default-apps-sheets-summary = Де відкриваються файли Excel, OpenDocument і CSV
settings-default-apps-documents-summary = Де відкриваються документи Word і текст OpenDocument
settings-default-apps-after-saving-summary = Показувати збережені вкладення в їхній папці
settings-compose-send-from-summary = Обліковий запис, з якого надсилаються нові листи: поточний або завжди той самий
settings-compose-send-on-replies-summary = «Надіслати» або «Надіслати й архівувати» ланцюжок у відповідях і пересиланнях
settings-compose-signatures-summary = Додається під вашим листом після рядка «--»
settings-compose-for-new-mail-summary = Підпис, з якого починаються нові листи
settings-compose-for-replies-summary = Підпис, з якого починаються відповіді й пересилання
settings-compose-format-summary = Писати нові листи звичайним текстом
settings-compose-spelling-summary = Перевірка правопису під час введення та мова словника
settings-compose-templates-summary = Незабаром: зберігайте листи, які часто пишете, і починайте з них новий лист або відповідь
settings-feedback-crash-reports-summary = Зберігати звіти про збої на цьому комп’ютері, коли Katna Mail або її фонова служба аварійно завершується
settings-feedback-saved-summary = Переглянути, скопіювати або видалити звіти про збої, збережені на цьому комп’ютері
settings-feedback-help-improve-summary = Надсилати звіти про збої, щоб допомогти виправити помилки; вимкнено, доки ви не ввімкнете
settings-experimental-blur-summary = Стільниця розмито просвічує крізь верхню панель, а меню — з матового скла
settings-search-shortcut = Комбінація клавіш
settings-search-tab = Вкладка налаштувань
settings-search-none = Немає налаштувань за запитом «{ $query }».
settings-search-results = Налаштування за запитом «{ $query }»

## Quick settings (the panel that slides in from the right)

quick-title = Швидкі налаштування
quick-see-all = Переглянути всі налаштування
quick-reading-pane = Панель читання
quick-pane-right = Праворуч від списку
quick-pane-none = Без поділу
quick-density = Щільність
quick-density-default = Типова
quick-density-compact = Компактна
quick-theme = Тема
quick-theme-system = Як на стільниці
quick-theme-light = Світла
quick-theme-dark = Темна
quick-desktop-colors = Кольори стільниці
quick-desktop-colors-detail = Колірна схема й колір акценту стільниці
quick-app-names = Назви програм
quick-app-names-detail = Підписи під значками програм ліворуч
quick-inbox-tabs = Вкладки «Вхідних»
quick-inbox-tabs-detail = Вкладки поштового сервісу кожного облікового запису
quick-choose-tabs = Вибрати вкладки
quick-choose-tabs-detail = Для кожного облікового запису, у налаштуваннях
quick-sending = Надсилання
quick-undo-send = Скасування надсилання
quick-undo-send-off = Вимк.
quick-undo-send-seconds = { $seconds } с
quick-signatures = Підписи
quick-signatures-none = Ще немає
quick-signatures-one = { $name }, типовий
quick-signatures-many = { $count ->
    [one] { $count } підпис; типовий — { $name }
    [few] { $count } підписи; типовий — { $name }
    [many] { $count } підписів; типовий — { $name }
   *[other] { $count } підпису; типовий — { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, типового немає
    [few] { $count }, типового немає
    [many] { $count }, типового немає
   *[other] { $count }, типового немає
}
quick-signature-untitled = Без назви
quick-threading = Ланцюжки листів
quick-conversation-view = Групування в ланцюжки
quick-conversation-view-detail = Групувати відповіді на той самий лист
quick-help = Довідка
quick-tour = Пройти ознайомлення
quick-whats-new = Що нового
quick-about = Про Katna

## Settings: opening at login

settings-open-at-login-failed = Не вдалося змінити відкриття під час входу: { $error }

## Settings > Appearance > Scaling

scale-letter = А
scale-percent = { $percent }%
scale-reset = Повернути { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Функції, які ще випробовуються. Вони можуть змінитися або зникнути.
look-heading = Вигляд і поведінка
look-window-frame = Рамка вікна
look-window-frame-detail = Хто малює смугу заголовка, кнопки вікна, кути й тінь.
look-frame-native-kde = Системна: рамка KDE у вашій темі Plasma
look-frame-native = Системна: рамка стільниці
look-frame-katna = Katna: верхня панель стає смугою заголовка
look-frame-katna-note-named = Katna малює заокруглені кути й власну тінь. Рамка більше не відповідає темі { $desktop }; правила вікон і далі діють.
look-frame-katna-note = Katna малює заокруглені кути й власну тінь. Рамка більше не відповідає темі стільниці; правила вікон і далі діють.
look-frame-client-side = Ваша стільниця залишає рамку кожній програмі, тож Katna вже малює власну.
look-blurred-background = Розмите тло
look-blurred-background-detail = Стільниця розмито просвічує крізь верхню панель і папки, а меню та спливні вікна — з матового скла.
look-blur = Розмивати те, що за вікном
look-blur-detail = Листи залишаються на суцільних картках, тож текст зберігає контраст
look-blur-off-kde = Ефект розмивання KDE вимкнено. Увімкніть «Розмивання» у «Системних параметрах» → «Керування вікнами» → «Ефекти стільниці», потім знову відкрийте Katna Mail.
look-blur-none-gnome = GNOME не розмиває те, що за вікнами.
look-blur-none-x11 = Ваш віконний менеджер не розмиває те, що за вікнами.
look-blur-none-wayland = Ваш композитор не розмиває те, що за вікнами.

## Settings > User feedback (crash reports)

feedback-intro-sending = Нові звіти про збої надсилаються, щоб допомогти виправити помилки. Ніщо інше не залишає цей комп’ютер.
feedback-intro-local = Katna нічого нікуди не надсилає. Звіти про збої залишаються на цьому комп’ютері — їх можна переглянути або долучити до звіту про ваду.
feedback-crash-reports = Звіти про збої
feedback-crash-reports-detail = Створюються, коли Katna Mail або її фонова служба аварійно завершується.
feedback-save = Зберігати звіти про збої на цьому комп’ютері
feedback-save-detail = Домашня папка, імена користувача й комп’ютера та адреси електронної пошти не включаються
feedback-saved = Збережені звіти про збої
feedback-saved-detail = { $count ->
    [one] Зберігається { $count } найновіший звіт.
    [few] Зберігаються { $count } найновіші звіти.
    [many] Зберігаються { $count } найновіших звітів.
   *[other] Зберігається { $count } найновішого звіту.
}
feedback-help-improve = Допомогти покращити Katna
feedback-help-improve-detail = Вимкнено, доки ви не ввімкнете, і тут це можна будь-коли вимкнути.
feedback-send = Надсилати звіти про збої
feedback-send-detail = Збережений звіт — саме такий, яким ви бачите його тут, — надсилається до системи відстеження збоїв Katna (Sentry, у ЄС). Без IP-адреси, листів і адрес електронної пошти
feedback-none-saved = Збережених звітів про збої немає.
feedback-delete-all = Видалити все
feedback-app-daemon = Фонова служба
feedback-report-sent = { $date } · Надіслано
feedback-view = Переглянути
feedback-view-tooltip = Відкрити звіт
feedback-copy-tooltip = Скопіювати, щоб вставити у звіт про ваду
feedback-copied = Звіт про збій скопійовано.
feedback-deleted-all = Звіти про збої видалено.
feedback-read-failed = Не вдалося прочитати звіт про збій: { $error }
feedback-delete-failed = Не вдалося видалити звіт про збій: { $error }
feedback-delete-all-failed = Не вдалося видалити звіти про збої: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Файл
desktop-menu-new-message = _Новий лист
desktop-menu-quit = Ви_йти
desktop-menu-edit = З_міни
desktop-menu-undo = _Скасувати
desktop-menu-select-all = Вибрати _все
desktop-menu-select-none = _Зняти вибір
desktop-menu-find = З_найти…
desktop-menu-view = _Перегляд
desktop-menu-folder-list = Показувати _список папок
desktop-menu-refresh = _Оновити
desktop-menu-go = П_ерейти
desktop-menu-inbox = _Вхідні
desktop-menu-starred = Із з_ірочкою
desktop-menu-sent = _Надіслані
desktop-menu-drafts = _Чернетки
desktop-menu-all-mail = _Уся пошта
desktop-menu-next = Н_аступний ланцюжок
desktop-menu-previous = _Попередній ланцюжок
desktop-menu-message = _Лист
desktop-menu-open = _Відкрити
desktop-menu-reply = Від_повісти
desktop-menu-reply-all = Відповісти в_сім
desktop-menu-forward = Пере_слати
desktop-menu-archive = _Архівувати
desktop-menu-delete = В_идалити
desktop-menu-spam = Повідомити про сп_ам
desktop-menu-move-to = Пере_містити в…
desktop-menu-mark-read = Позначити як п_рочитане
desktop-menu-mark-unread = Позначити як _непрочитане
desktop-menu-star = Позначити з_ірочкою
desktop-menu-important = Позначити як ва_жливе
desktop-menu-not-important = Позначити як не_важливе
desktop-menu-settings = П_араметри
desktop-menu-quick-settings = _Швидкі налаштування
desktop-menu-configure = _Налаштувати Katna Mail…
desktop-menu-help = _Довідка
desktop-menu-shortcuts = _Комбінації клавіш
desktop-menu-whats-new = _Що нового
desktop-menu-about = _Про Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Переміщення
shortcut-group-actions = Дії
shortcut-group-go-to = Перехід
shortcut-group-app = Програма

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Наступний ланцюжок
shortcut-previous = Попередній ланцюжок
shortcut-down = Вниз по списку
shortcut-up = Угору по списку
shortcut-first = На початок списку
shortcut-last = У кінець списку
shortcut-page-down = На сторінку вниз по списку
shortcut-page-up = На сторінку вгору по списку
shortcut-open = Відкрити ланцюжок
shortcut-back = Повернутися до списку
shortcut-scroll-down = Прокрутити вниз
shortcut-scroll-up = Прокрутити вгору
shortcut-scroll-page-down = Прокрутити на сторінку вниз
shortcut-scroll-page-up = Прокрутити на сторінку вгору
shortcut-compose = Написати
shortcut-reply = Відповісти
shortcut-reply-all = Відповісти всім
shortcut-forward = Переслати
shortcut-archive = Архівувати
shortcut-delete = Видалити
shortcut-spam = Повідомити про спам
shortcut-move-to = Перемістити в
shortcut-mark-read = Позначити як прочитане
shortcut-mark-unread = Позначити як непрочитане
shortcut-star = Додати або зняти зірочку
shortcut-important = Позначити як важливе
shortcut-not-important = Позначити як неважливе
shortcut-check = Вибрати ланцюжок
shortcut-select-all = Вибрати всі ланцюжки
shortcut-select-none = Зняти вибір з усіх ланцюжків
shortcut-undo = Скасувати останню дію
shortcut-go-inbox = Вхідні
shortcut-go-starred = Із зірочкою
shortcut-go-sent = Надіслані
shortcut-go-drafts = Чернетки
shortcut-go-all = Уся пошта
shortcut-search = Пошук у пошті
shortcut-navigation = Показати або згорнути меню
shortcut-quick-settings = Швидкі налаштування
shortcut-settings = Усі налаштування
shortcut-shortcuts = Комбінації клавіш
shortcut-reload = Перевірити пошту
shortcut-quit = Вийти

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first }, потім { $second }

## Settings > Accounts

accounts-folder-pane = Панель папок
accounts-folder-pane-detail = Папки яких облікових записів показує панель ліворуч.
accounts-shown-one = Один обліковий запис за раз; перемикання в картці облікового запису
accounts-shown-all = Усі облікові записи, один за одним
accounts-row = Облікові записи
accounts-row-detail = Вилучення облікового запису видаляє копію його пошти, яку Katna зберігає на цьому комп’ютері. Пошта залишається на сервері.
accounts-none = Облікових записів ще немає.
accounts-kind-imported = Імпортовано
accounts-picture-reset = Використати зображення системи
accounts-picture-change = Змінити зображення
accounts-remove = Вилучити
accounts-delete-all-row = Видалити всі дані
accounts-delete-all-row-detail = Почати заново, як після нового встановлення.
accounts-delete-all-about = Видаляє з цього комп’ютера всі облікові записи, усю збережену пошту, контакти й календарі, пошуковий індекс, ваші налаштування та збережені паролі. На ваших поштових серверах нічого не змінюється.
accounts-delete-all-open = Видалити всі дані Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } вилучено з Katna.
accounts-removed = { $address } вилучено з Katna. Його пошта й далі на сервері.
accounts-all-deleted = Усі дані Katna видалено з цього комп’ютера.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Вилучити { $address }?
accounts-remove-confirm = Вилучити обліковий запис
accounts-removing = Вилучення…
accounts-remove-local-mail = { $folders ->
    [0] Уся пошта, імпортована в цей обліковий запис
    [1] Уся пошта, імпортована в цей обліковий запис, у його папці
    [one] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папці
    [few] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папках
    [many] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папках
   *[other] Уся пошта, імпортована в цей обліковий запис, у його { $folders } папках
}
accounts-remove-local-settings = Його налаштування в Katna
accounts-remove-mail = { $folders ->
    [0] Уся пошта цього облікового запису, збережена Katna
    [1] Уся пошта цього облікового запису, збережена Katna в його папці
    [one] Уся пошта цього облікового запису, збережена Katna в його { $folders } папці
    [few] Уся пошта цього облікового запису, збережена Katna в його { $folders } папках
    [many] Уся пошта цього облікового запису, збережена Katna в його { $folders } папках
   *[other] Уся пошта цього облікового запису, збережена Katna в його { $folders } папках
}
accounts-remove-outbox = Його листи, що чекають у вихідних
accounts-remove-settings = Його збережений пароль і налаштування в Katna
accounts-delete-all-title = Видалити всі дані Katna?
accounts-delete-all-confirm = Видалити все
accounts-deleting = Видалення…
accounts-delete-all-accounts = Усі облікові записи, а також уся пошта й вкладення, збережені Katna
accounts-delete-all-contacts = Контакти, календарі й пошуковий індекс
accounts-delete-all-settings = Усі налаштування, підписи й комбінації клавіш
accounts-delete-all-passwords = Усі збережені паролі
accounts-deleted-heading = Видаляється з цього комп’ютера:
accounts-cannot-undo = Цю дію не можна скасувати.
accounts-server-delete-all = На ваших поштових серверах нічого не змінюється: пошта залишається там, і якщо знову додати обліковий запис, вона завантажиться знову. Пошта, імпортована з файлів, є лише в Katna; самі файли не змінюються.
accounts-server-local = Цю пошту імпортовано з файлів, тож єдина її копія — у Katna. Файли, з яких її імпортовано, не змінюються; імпортуйте їх знову, щоб повернути пошту.
accounts-server-remove = На поштовому сервері нічого не змінюється: пошта залишається там, і якщо знову додати обліковий запис, вона завантажиться знову.
accounts-confirm-word = видалити
accounts-confirm-placeholder = Введіть «{ accounts-confirm-word }»
accounts-confirm-prompt = Щоб підтвердити, введіть «{ accounts-confirm-word }»:
accounts-cancel = Скасувати
