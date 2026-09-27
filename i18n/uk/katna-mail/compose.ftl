# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Новий лист
compose-restore = Відновити
compose-minimize = Згорнути
compose-exit-full-screen = Вийти з повноекранного режиму
compose-open-window = Відкрити в новому вікні
compose-save-close = Зберегти й закрити
compose-back-to-mail = Повернутися до вікна пошти
compose-pop-out-reply = Відкрити відповідь окремо
compose-show-trimmed = Показати приховану частину

## Recipients and subject

compose-to = Кому
compose-cc = Копія
compose-bcc = Прихована копія
compose-from = Від
compose-from-choose = Надіслати з іншого облікового запису
compose-recipients = Одержувачі
compose-subject = Тема

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Спершу надішліть або відкиньте відкритий лист.
compose-bad-address = «{ $address }» — не адреса електронної пошти.
compose-no-recipients = Додайте принаймні одного одержувача.
compose-attachments-too-large = Вкладення займають { $size }; поштові сервери приймають до { $limit }.
compose-no-account = Додайте обліковий запис, з якого надсилати пошту.
compose-past-time = Виберіть час у майбутньому.
compose-scheduling = Планування…
compose-sending = Надсилання…
compose-scheduled = Надсилання заплановано на { $when }
compose-sent-archived = Надіслано й заархівовано
compose-sent = Лист надіслано
compose-discarded = Чернетку відкинуто
compose-draft-saved = Чернетку збережено
compose-draft-failed = Не вдалося зберегти чернетку: { $error }
compose-draft-not-opened = Не вдалося відкрити чернетку.

## Attachments

compose-picker-insert = Вставити
compose-picker-attach = Вкласти
compose-file-too-large = { $name } завеликий: лист може містити до { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Вилучити вкладення
compose-attachments-total = { $count ->
    [one] { $count } файл, { $size }
    [few] { $count } файли, { $size }
    [many] { $count } файлів, { $size }
   *[other] { $count } файлу, { $size }
}
compose-drop-files = Перетягніть файли сюди
compose-drop-here = Перетягніть сюди
compose-paste-keep-formatting = Зберегти форматування
compose-paste-table = Таблиця
compose-paste-picture = Зображення
compose-paste-plain-text = Звичайний текст
compose-paste-inline = У тексті
compose-paste-attachment = Вкладення

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Зашифрувати
compose-encrypted = Зашифровано: прочитати можуть лише одержувачі
compose-sign = Підписати
compose-signed = Підписано: одержувачі можуть перевірити, що лист від вас

## Spelling

spell-no-dictionary = Словник правопису для { $language } не встановлено (наприклад, hunspell-en_us).
spell-dictionary-error = Словник правопису: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = «{ $words }»
grammar-add = Додати «{ $words }»
grammar-remove = Вилучити «{ $words }»
grammar-ignore = Пропустити

## Send checks (asked before a message goes out)

send-check-attachment-title = Ви хотіли вкласти файли?
send-check-attachment-text = Ви згадали про вкладення, але нічого не вкладено.
send-check-attach = Вкласти файл
send-check-subject-title = Надіслати без теми?
send-check-subject-text = Цей лист не має теми.
send-check-add-subject = Додати тему
send-check-send-anyway = Усе одно надіслати
recipient-not-valid = Недійсна адреса електронної пошти
recipient-show-address = Показати адресу
recipient-remove = Вилучити
recipient-bad-title = Перевірте адресу
recipient-bad-text = «{ $address }» — недійсна адреса електронної пошти. Виправте або вилучіть її перед надсиланням.
recipient-bad-fix = Виправити
