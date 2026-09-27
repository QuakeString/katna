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
