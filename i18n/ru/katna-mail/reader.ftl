# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Закрыть
reader-back = Назад
reader-mark-unread = Отметить как непрочитанное
reader-move-to = Переместить в
reader-more = Ещё
reader-original-colors = Показать исходные цвета
reader-dark-colors = Показать в тёмных цветах
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
reader-sending = Отправка…
reader-me = мне
reader-to = кому: { $names }
reader-to-label = кому:
reader-tick-delivered = Доставлено { $when }
reader-tick-no-bounce = Отправлено { $when }; возврат не пришёл, так что письмо, скорее всего, дошло
reader-tick-bounced = Не доставлено: возврат { $when }
reader-tick-read = Прочитано { $when } (уведомление о прочтении)
reader-tick-opened = Открыто, последний раз { $when } (отслеживание открытий)
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
tracking-opened = { $who }: открыто { $count ->
    [one] { $count } раз
    [few] { $count } раза
    [many] { $count } раз
   *[other] { $count } раза
}, последний раз { $when }
tracking-opens-clicks = { $who }: открыто { $opens ->
    [one] { $opens } раз
    [few] { $opens } раза
    [many] { $opens } раз
   *[other] { $opens } раза
}, по ссылке перешли { $clicks ->
    [one] { $clicks } раз
    [few] { $clicks } раза
    [many] { $clicks } раз
   *[other] { $clicks } раза
}, последний раз { $when }
tracking-clicked = { $who }: по ссылке перешли { $clicks ->
    [one] { $clicks } раз
    [few] { $clicks } раза
    [many] { $clicks } раз
   *[other] { $clicks } раза
}, последний раз { $when }
tracking-maybe-opened = { $who }: возможно, открыто (Apple Mail загружает изображения ради конфиденциальности)
tracking-seen-none = Пока никто не открыл письмо и не перешёл по ссылке
tracking-receipt = Уведомление о прочтении от { $who }
tracking-receipt-displayed = Уведомление о прочтении: ваше письмо открыто получателем { $who }
tracking-receipt-other = Уведомление о прочтении: ваше письмо удалено или обработано получателем { $who } без открытия

## Remote images and pictures

remote-hidden = Изображения в этом письме скрыты.
remote-hidden-unconfirmed = Изображения скрыты: не удалось подтвердить отправителя.
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
attachment-forward = Переслать
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
print-preview-title = Предварительный просмотр печати
print-preview-laying-out = Размещение страниц…
print-preview-pages = { $count ->
    [one] { $count } страница
    [few] { $count } страницы
    [many] { $count } страниц
   *[other] { $count } страницы
}
print-preview-more = { $count ->
    [one] и ещё { $count } страница
    [few] и ещё { $count } страницы
    [many] и ещё { $count } страниц
   *[other] и ещё { $count } страницы
}
print-preview-failed = не удалось показать страницы
print-preview-paper = Бумага
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Оформление
print-preview-as-shown = Как на экране
print-preview-simple = Только текст
print-preview-backgrounds = Фон
print-preview-cancel = Отмена
print-preview-print = Печать
print-not-downloaded = (Ещё не загружено.)
print-encrypted = (Зашифровано. Откройте письмо в Katna Mail, чтобы распечатать его текст.)
print-to = Кому: { $addresses }
print-cc = Копия: { $addresses }
text-pin = Закрепить сверху
text-copy-address = Копировать адрес

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Откройте это письмо, чтобы прочитать вложения.
text-copy = Копировать
text-select-all = Выделить всё
