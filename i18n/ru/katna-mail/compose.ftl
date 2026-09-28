# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Новое письмо
compose-restore = Восстановить
compose-minimize = Свернуть
compose-exit-full-screen = Выйти из полноэкранного режима
compose-open-window = Открыть в новом окне
compose-save-close = Сохранить и закрыть
compose-back-to-mail = Вернуться в окно почты
compose-pop-out-reply = Открыть ответ в отдельном окне
compose-edit-recipients = Изменить получателей
compose-summary-cc = Копия: { $names }
compose-summary-bcc = Скрытая копия: { $names }
compose-more-recipients = ещё { $count }
compose-show-trimmed = Показать скрытую часть
compose-hide-trimmed = Свернуть скрытую часть
compose-remove-trimmed = Удалить цитируемый текст
compose-trimmed-removed = Цитируемый текст удалён

## Recipients and subject

compose-to = Кому
compose-cc = Копия
compose-bcc = Скрытая копия
compose-from = От
compose-from-choose = Отправить с другого аккаунта
compose-recipients = Получатели
compose-subject = Тема

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Сначала отправьте или удалите открытое письмо.
compose-bad-address = «{ $address }» не является адресом электронной почты.
compose-no-recipients = Добавьте хотя бы одного получателя.
compose-attachments-too-large = Размер вложений { $size }; почтовые серверы принимают не больше { $limit }.
compose-no-account = Добавьте аккаунт, с которого будет отправляться почта.
compose-past-time = Выберите время в будущем.
compose-scheduling = Планирование…
compose-sending = Отправка…
compose-scheduled = Отправка запланирована на { $when }
compose-sent-archived = Отправлено и перемещено в архив
compose-sent = Письмо отправлено
compose-discarded = Черновик удалён
compose-draft-saved = Черновик сохранён
compose-draft-failed = Не удалось сохранить черновик: { $error }
compose-draft-not-opened = Не удалось открыть черновик.

## Attachments

compose-picker-insert = Вставить
compose-picker-attach = Прикрепить
compose-file-too-large = Файл { $name } слишком большой: письмо может содержать не больше { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Удалить вложение
compose-attachments-total = { $count ->
    [one] { $count } файл, { $size }
    [few] { $count } файла, { $size }
    [many] { $count } файлов, { $size }
   *[other] { $count } файла, { $size }
}
compose-drive-note = { $name } больше { $limit }, поэтому файл попадёт на ваш Google Drive, а в письме будет ссылка.
compose-drive-tip = На вашем Google Drive; в письме будет ссылка
compose-drive-uploading = Загрузка { $percent }%
compose-drive-allow = Разрешить Drive
compose-drive-allow-tip = Войдите через Google ещё раз, чтобы Katna могла помещать большие файлы на ваш Drive
compose-drive-retry = Повторить попытку
compose-drive-sends-when-uploaded = Письмо будет отправлено после загрузки { $name }
compose-drive-not-uploaded = { $name } ещё не на Google Drive
compose-drive-share-failed = Не удалось предоставить доступ к файлам на Google Drive: { $error }
compose-drive-share-title = Предоставить доступ к файлам всем?
compose-drive-share-text = { $count ->
    [one] Google Drive не может предоставить доступ к файлам адресату { $addresses }, у которого нет аккаунта Google. Вместо этого файлы сможет открыть любой, у кого есть ссылка.
    [few] Google Drive не может предоставить доступ к файлам адресатам { $addresses }, у которых нет аккаунта Google. Вместо этого файлы сможет открыть любой, у кого есть ссылка.
    [many] Google Drive не может предоставить доступ к файлам адресатам { $addresses }, у которых нет аккаунта Google. Вместо этого файлы сможет открыть любой, у кого есть ссылка.
   *[other] Google Drive не может предоставить доступ к файлам адресатам { $addresses }, у которых нет аккаунта Google. Вместо этого файлы сможет открыть любой, у кого есть ссылка.
}
compose-drive-share-link = Поделиться по ссылке
compose-drive-send-without = Отправить без доступа
compose-drive-share-cancel = Отмена
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = Перетащите файлы сюда
compose-drop-here = Перетащите сюда
compose-paste-keep-formatting = Сохранить форматирование
compose-paste-table = Таблица
compose-paste-picture = Изображение
compose-paste-plain-text = Обычный текст
compose-paste-inline = В тексте
compose-paste-attachment = Вложение

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Зашифровать
compose-encrypted = Зашифровано: прочитать его могут только получатели
compose-sign = Подписать
compose-signed = Подписано: получатели могут проверить, что письмо от вас
compose-track = Отслеживать открытия и переходы
compose-tracked = Отслеживается: вы увидите, когда каждый получатель откроет письмо или перейдёт по ссылке
compose-track-clicks = Отслеживать переходы по ссылкам (обычный текст не показывает открытия)
compose-tracked-clicks = Отслеживается: вы увидите, когда каждый получатель перейдёт по ссылке
compose-track-sign-in = Войдите в аккаунт Katna, чтобы отслеживать открытия и переходы
compose-receipt = Запросить уведомление о прочтении
compose-receipt-on = Уведомление о прочтении запрошено: приложение получателя может предложить его отправить
compose-delivery = Запросить уведомление о доставке
compose-delivery-on = Уведомление о доставке запрошено: ваш почтовый сервер пришлёт вам письмо, когда сервер каждого получателя примет сообщение
compose-delivery-unavailable = Ваш почтовый сервер не отправляет уведомления о доставке

## Spelling

spell-no-dictionary = Словарь для проверки орфографии ({ $language }) не установлен (например, hunspell-en_us).
spell-dictionary-error = Словарь орфографии: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = «{ $words }»
grammar-add = Добавить «{ $words }»
grammar-remove = Удалить «{ $words }»
grammar-ignore = Пропустить

## Send checks (asked before a message goes out)

send-check-attachment-title = Хотели прикрепить файлы?
send-check-attachment-text = В письме упоминается вложение, но ничего не прикреплено.
send-check-attach = Прикрепить файл
send-check-subject-title = Отправить без темы?
send-check-subject-text = У этого письма нет темы.
send-check-add-subject = Добавить тему
send-check-send-anyway = Всё равно отправить
recipient-not-valid = Недопустимый адрес электронной почты
recipient-show-address = Показать адрес
recipient-remove = Удалить
recipient-bad-title = Проверьте адрес
recipient-bad-text = «{ $address }» — недопустимый адрес электронной почты. Исправьте или удалите его перед отправкой.
recipient-bad-fix = Исправить
