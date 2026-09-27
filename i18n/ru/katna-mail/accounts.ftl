# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
