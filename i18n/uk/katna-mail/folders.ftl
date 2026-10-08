# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Мітки
nav-folders = Папки
nav-label-new = Створити мітку
nav-folder-new = Створити папку
nav-menu-check-mail = Перевірити нову пошту
nav-menu-check-inbox = Перевірити ці «Вхідні»
nav-unified-leave-out = Не включати до єдиних «Вхідних»
nav-unified-bring-back = Повернути до єдиних «Вхідних»
nav-menu-sign-in-again = Увійти знову
nav-menu-new-mail = Новий лист із цього облікового запису
nav-menu-account-settings = Налаштування облікового запису
nav-account-checked = Синхронізовано · перевірено { $ago }
nav-account-in-sync = Синхронізовано
nav-account-connecting = З’єднання…
nav-account-offline = Немає з’єднання, повторна спроба
nav-account-signed-out = Термін входу в { $provider } минув
nav-account-password-refused = Пароль не прийнято
nav-account-storage = Використано { $used } із { $total }
nav-menu-new-subfolder = Нова папка всередині
nav-menu-new-sublabel = Нова мітка всередині
nav-menu-rename = Перейменувати
nav-menu-delete = Видалити
nav-menu-empty-trash = Очистити кошик
nav-account-unnamed = Обліковий запис { $number }
nav-all-accounts = Усі облікові записи
nav-expand = Показати папки
nav-collapse = Сховати папки
storage-used = Використано { $percent }% із { $total }
storage-used-detail = { $address }: використано { $used } із { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Вхідні
folder-starred = Із зірочкою
folder-snoozed = Відкладені
folder-unread = Непрочитані
folder-important = Важливі
folder-drafts = Чернетки
folder-sent = Надіслані
folder-archive = Архів
folder-spam = Спам
folder-trash = Кошик
folder-all-mail = Уся пошта
folder-scheduled = Заплановані
folder-waiting = Очікують відповіді
folder-waiting-short = Очікують
folder-reminders = Нагадування
folder-outbox = Вихідні
folder-activity = Активність

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
label-rename-title = Перейменувати мітку
label-folder-rename-title = Перейменувати папку
label-rename = Перейменувати
label-renaming = Перейменування…
label-renamed = Мітку перейменовано на «{ $name }».
label-folder-renamed = Папку перейменовано на «{ $name }».
folder-delete-title = Видалити «{ $name }»?
folder-delete-body = { $count ->
    [0] Вона порожня. Папку буде видалено із сервера, тож вона зникне й у вебпошті та на телефоні.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Її { $count } ланцюжок переміститься в кошик, тож його ще можна буде повернути.
            [few] Її { $count } ланцюжки перемістяться в кошик, тож їх ще можна буде повернути.
            [many] Її { $count } ланцюжків переміститься в кошик, тож їх ще можна буде повернути.
           *[other] Її { $count } ланцюжка переміститься в кошик, тож їх ще можна буде повернути.
        }
       *[message] { $count ->
            [one] Її { $count } лист переміститься в кошик, тож його ще можна буде повернути.
            [few] Її { $count } листи перемістяться в кошик, тож їх ще можна буде повернути.
            [many] Її { $count } листів переміститься в кошик, тож їх ще можна буде повернути.
           *[other] Її { $count } листа переміститься в кошик, тож їх ще можна буде повернути.
        }
    } Папку буде видалено із сервера, тож вона зникне й у вебпошті та на телефоні.
}
folder-delete-forever-body = { $count ->
    [0] Вона порожня. Папку буде видалено із сервера, тож вона зникне й у вебпошті та на телефоні.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Її { $count } ланцюжок буде видалено назавжди: у цьому обліковому записі немає кошика.
            [few] Її { $count } ланцюжки буде видалено назавжди: у цьому обліковому записі немає кошика.
            [many] Її { $count } ланцюжків буде видалено назавжди: у цьому обліковому записі немає кошика.
           *[other] Її { $count } ланцюжка буде видалено назавжди: у цьому обліковому записі немає кошика.
        }
       *[message] { $count ->
            [one] Її { $count } лист буде видалено назавжди: у цьому обліковому записі немає кошика.
            [few] Її { $count } листи буде видалено назавжди: у цьому обліковому записі немає кошика.
            [many] Її { $count } листів буде видалено назавжди: у цьому обліковому записі немає кошика.
           *[other] Її { $count } листа буде видалено назавжди: у цьому обліковому записі немає кошика.
        }
    } Папку буде видалено із сервера, тож вона зникне й у вебпошті та на телефоні.
}
folder-delete-label-body = Мітку буде видалено. Її листи залишаться в «Усій пошті» та в інших мітках.
folder-delete-confirm = Видалити папку
folder-delete-label-confirm = Видалити мітку
folder-deleted = Папку «{ $name }» видалено
label-deleted = Мітку «{ $name }» видалено
