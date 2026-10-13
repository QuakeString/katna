# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Ярлыки
nav-folders = Папки
nav-label-new = Создать ярлык
nav-folder-new = Создать папку
nav-menu-check-mail = Проверить новую почту
nav-menu-check-inbox = Проверить эти «Входящие»
nav-unified-leave-out = Не включать в общие входящие
nav-unified-bring-back = Вернуть в общие входящие
nav-menu-sign-in-again = Войти снова
nav-menu-new-mail = Новое письмо с этого аккаунта
nav-menu-account-settings = Настройки аккаунта
nav-account-checked = Синхронизировано · проверено { $ago }
nav-account-in-sync = Синхронизировано
nav-account-connecting = Подключение…
nav-account-offline = Нет сети, повторная попытка
nav-account-signed-out = Срок входа в { $provider } истёк
nav-account-password-refused = Пароль не принят
nav-account-storage = Занято { $used } из { $total }
nav-menu-new-subfolder = Новая папка внутри
nav-menu-new-sublabel = Новый ярлык внутри
nav-menu-rename = Переименовать
nav-menu-delete = Удалить
nav-menu-empty-trash = Очистить корзину
nav-account-unnamed = Аккаунт { $number }
nav-all-accounts = Все аккаунты
nav-expand = Показать папки
nav-collapse = Скрыть папки
storage-used = Занято { $percent } % из { $total }
storage-used-detail = { $address }: занято { $used } из { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Входящие
folder-starred = Помеченные
folder-snoozed = Отложенные
folder-unread = Непрочитанные
folder-important = Важные
folder-drafts = Черновики
folder-sent = Отправленные
folder-archive = Архив
folder-spam = Спам
folder-trash = Корзина
folder-all-mail = Вся почта
folder-scheduled = Запланированные
folder-waiting = Ждут ответа
folder-waiting-short = Ждут ответа
folder-reminders = Напоминания
folder-outbox = Исходящие
folder-activity = Активность
folder-not-on-account = В этом аккаунте нет такой папки.

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
label-rename-title = Переименовать ярлык
label-folder-rename-title = Переименовать папку
label-rename = Переименовать
label-renaming = Переименование…
label-renamed = Ярлык переименован в «{ $name }».
label-folder-renamed = Папка переименована в «{ $name }».
folder-delete-title = Удалить «{ $name }»?
folder-delete-body = { $count ->
    [0] В ней нет писем. Папка удаляется с сервера, поэтому пропадёт и в веб-почте, и на телефоне.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Её { $count } цепочка попадёт в корзину, так что её ещё можно будет вернуть.
            [few] Её { $count } цепочки попадут в корзину, так что их ещё можно будет вернуть.
            [many] Её { $count } цепочек попадут в корзину, так что их ещё можно будет вернуть.
           *[other] Её { $count } цепочки попадут в корзину, так что их ещё можно будет вернуть.
        }
       *[message] { $count ->
            [one] Её { $count } письмо попадёт в корзину, так что его ещё можно будет вернуть.
            [few] Её { $count } письма попадут в корзину, так что их ещё можно будет вернуть.
            [many] Её { $count } писем попадут в корзину, так что их ещё можно будет вернуть.
           *[other] Её { $count } письма попадут в корзину, так что их ещё можно будет вернуть.
        }
    } Папка удаляется с сервера, поэтому пропадёт и в веб-почте, и на телефоне.
}
folder-delete-forever-body = { $count ->
    [0] В ней нет писем. Папка удаляется с сервера, поэтому пропадёт и в веб-почте, и на телефоне.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Её { $count } цепочка будет удалена навсегда: в этом аккаунте нет корзины.
            [few] Её { $count } цепочки будут удалены навсегда: в этом аккаунте нет корзины.
            [many] Её { $count } цепочек будут удалены навсегда: в этом аккаунте нет корзины.
           *[other] Её { $count } цепочки будут удалены навсегда: в этом аккаунте нет корзины.
        }
       *[message] { $count ->
            [one] Её { $count } письмо будет удалено навсегда: в этом аккаунте нет корзины.
            [few] Её { $count } письма будут удалены навсегда: в этом аккаунте нет корзины.
            [many] Её { $count } писем будут удалены навсегда: в этом аккаунте нет корзины.
           *[other] Её { $count } письма будут удалены навсегда: в этом аккаунте нет корзины.
        }
    } Папка удаляется с сервера, поэтому пропадёт и в веб-почте, и на телефоне.
}
folder-delete-label-body = Ярлык будет удалён. Его письма останутся во «Всей почте» и в других ярлыках.
folder-delete-confirm = Удалить папку
folder-delete-label-confirm = Удалить ярлык
folder-deleted = Папка «{ $name }» удалена
label-deleted = Ярлык «{ $name }» удалён
