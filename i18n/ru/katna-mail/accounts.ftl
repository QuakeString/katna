# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Панель папок
accounts-folder-pane-detail = Папки каких аккаунтов показывает панель слева.
accounts-shown-one = Один аккаунт за раз; переключение в карточке аккаунта
accounts-shown-all = Все аккаунты, один за другим
accounts-unified = Общие входящие
accounts-unified-switch = Показывать почту всех аккаунтов вместе
accounts-unified-switch-detail = «Все аккаунты» стоят в начале панели папок: входящие, отправленные и другие папки каждого аккаунта в одном списке. Аккаунты ниже сначала свёрнуты.
accounts-row = Аккаунты
accounts-row-detail = Панель папок и меню аккаунтов показывают аккаунты в этом порядке; первый используется по умолчанию. При удалении аккаунта удаляется копия его почты, которую Katna хранит на этом компьютере. На сервере почта остаётся.
accounts-none = Аккаунтов пока нет.
accounts-pop3-row = Почта на сервере
accounts-pop3-row-detail = Аккаунты POP3 загружают почту на этот компьютер. Выберите, что затем происходит с копией на сервере.
accounts-pop3-with-katna = Хранить, пока я не удалю письмо в Katna
accounts-pop3-at-once = Удалять сразу после загрузки
accounts-pop3-after-days = { $count ->
    [one] Удалять через { $count } день
    [few] Удалять через { $count } дня
    [many] Удалять через { $count } дней
   *[other] Удалять через { $count } дня
}
accounts-pop3-never = Никогда не удалять
accounts-pop3-days-less = Меньше дней
accounts-pop3-days-more = Больше дней
accounts-kind-imported = Импортирован
accounts-picture-reset = Взять изображение из системы
accounts-picture-change = Сменить изображение
accounts-picture-remove = Удалить изображение
accounts-rename = Переименовать
accounts-name-save = Сохранить
accounts-name-cancel = Отмена
accounts-name-placeholder = Ваше имя
accounts-rename-failed = Не удалось переименовать аккаунт: { $error }
accounts-move-up = Переместить вверх
accounts-move-down = Переместить вниз
accounts-drag = Перетащите, чтобы изменить порядок
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
reset-cache-about = Удаляет почту и вложения, загруженные Katna, изображения отправителей и поисковый индекс, а затем снова загружает недавние письма. Аккаунты, настройки и почта, которая есть только на этом компьютере, остаются.
reset-cache-button = Сбросить кэш
reset-cache-title = Сбросить кэш?
reset-cache-deleted = Удаляется, а затем загружается снова:
reset-cache-mail = Почта и вложения, загруженные с ваших серверов IMAP: недавние письма загружаются снова сразу, более старые — когда вы их откроете
reset-cache-index = Поисковый индекс, который сразу перестраивается
reset-cache-pictures = Изображения отправителей
reset-cache-kept = Остаются: ваши аккаунты, пароли и настройки; пометки, ярлыки, отметки о прочтении и закрепления; черновики, исходящие и изменения, которых ещё нет на сервере; а также почта из аккаунтов POP3 или импортированных файлов, у которой может не быть другой копии. На почтовых серверах ничего не меняется.
reset-cache-confirm = Сбросить кэш
reset-cache-busy = Сброс…
reset-cache-done = Кэш сброшен. Недавние письма загружаются снова.
reset-cache-done-freed = Кэш сброшен, освобождено { $size }. Недавние письма загружаются снова.
