# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Почтовый сервер
problems-signed-out = { $provider } завершил сеанс Katna в { $address }. Синхронизация почты остановлена.
problems-password-refused = { $provider } не принял пароль для { $address }. Возможно, он изменился.
problems-no-answer = { $provider } не отвечает для { $address }. Katna продолжает попытки.
problems-offline = Вы не в сети. Ваша почта по-прежнему здесь, а отправленные письма подождут, пока вы снова не подключитесь.
problems-accounts-need-you = { $count ->
    [one] { $count } аккаунт требует внимания
    [few] { $count } аккаунта требуют внимания
    [many] { $count } аккаунтов требуют внимания
   *[other] { $count } аккаунта требуют внимания
}
problems-show = Показать
problems-later = Позже
problems-new-password = Новый пароль
problems-try-again = Повторить

## The New password card

problems-password-title = Новый пароль
problems-password-detail = { $provider } не принял сохранённый пароль для { $address }. Введите новый; Katna проверит его, прежде чем сохранить.
problems-password-placeholder = Пароль
problems-password-show = Показать пароль
problems-password-hide = Скрыть пароль
problems-password-cancel = Отмена
problems-password-save = Сохранить
problems-password-checking = Проверка…
problems-password-refused-again = { $provider } не принял и этот пароль. Проверьте его и повторите попытку.
problems-password-saved = Пароль для { $address } сохранён. Получаем вашу почту…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Почтовый сервер { $address } не позволил переместить { $count ->
    [one] письмо, поэтому оно вернулось на место.
    [few] { $count } письма, поэтому они вернулись на место.
    [many] { $count } писем, поэтому они вернулись на место.
   *[other] { $count } письма, поэтому они вернулись на место.
}
problems-refused-flags = Почтовый сервер { $address } не позволил отметить { $count ->
    [one] письмо (прочитанное, помеченное…), поэтому оно осталось как было.
    [few] { $count } письма (прочитанные, помеченные…), поэтому они остались как были.
    [many] { $count } писем (прочитанные, помеченные…), поэтому они остались как были.
   *[other] { $count } письма (прочитанные, помеченные…), поэтому они остались как были.
}
problems-refused-label = Почтовый сервер { $address } не позволил изменить ярлыки { $count ->
    [one] письма, поэтому оно осталось как было.
    [few] { $count } писем, поэтому они остались как были.
    [many] { $count } писем, поэтому они остались как были.
   *[other] { $count } письма, поэтому они остались как были.
}
problems-refused-delete = Почтовый сервер { $address } не позволил удалить { $count ->
    [one] письмо, поэтому оно вернулось.
    [few] { $count } письма, поэтому они вернулись.
    [many] { $count } писем, поэтому они вернулись.
   *[other] { $count } письма, поэтому они вернулись.
}
problems-refused-other = Почтовый сервер { $address } не принял { $count ->
    [one] изменение, поэтому Katna вернула всё как было.
    [few] { $count } изменения, поэтому Katna вернула всё как было.
    [many] { $count } изменений, поэтому Katna вернула всё как было.
   *[other] { $count } изменения, поэтому Katna вернула всё как было.
}
problems-details = Подробности

## Katna's background service (katna-daemon) isn't running

service-starting = Запуск фоновой службы Katna…
service-failed = Фоновая служба Katna не запускается, поэтому почта не синхронизируется.
service-start-again = Запустить снова
service-started-again = Фоновая служба Katna остановилась и была запущена снова.
service-details-title = Почему служба не запускается
service-details-body = Скопируйте это и отправьте вместе с отчётом. Здесь нет ни писем, ни паролей.
service-details-copy = Копировать
service-details-close = Закрыть
service-not-running = Фоновая служба Katna не запущена.
service-no-answer = Фоновая служба Katna не ответила: { $error }
service-no-session = Нет сеанса D-Bus: { $error }
safe-line = Katna в безопасном режиме из-за проблемы с обновлением, поэтому почта не синхронизируется.
safe-try-again = Повторить
safe-restore = Восстановить
safe-restoring = Восстановление данных на { $when }…
safe-restored = Данные восстановлены на { $when }. Прежние данные сохранены в папке.
safe-show-folder = Показать папку
safe-restore-failed = Не удалось восстановить данные: { $error }
safe-restore-title = Восстановить данные, какими они были до обновления?
safe-restore-body = Katna вернётся к выбранной копии. Письма, пришедшие после неё, снова загрузятся из ваших аккаунтов.
safe-restore-none = Копий пока нет. Katna создаёт копию перед каждым обновлением, которое меняет ваши данные.
safe-restore-keep = Всё, что есть сейчас, включая неотправленные письма, черновики и ещё не синхронизированные изменения, сначала сохраняется в папке, поэтому ничего не потеряется.
safe-restore-cancel = Отмена
safe-restore-mail = Почта
safe-restore-pim = Аккаунты и контакты
safe-restore-blobs = Вложения
safe-report-title = Отчёт для отладки
safe-report-body = Скопируйте его и приложите к отчёту об ошибке. В нём нет писем, адресов и паролей.
safe-report-restore = Восстановить…
safe-report-copied = Отчёт для отладки скопирован
