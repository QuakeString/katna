# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Закрити
reader-back = Назад
reader-mark-unread = Позначити як непрочитане
reader-move-to = Перемістити в
reader-snooze = Відкласти
reader-remind = Нагадати мені
reader-more = Більше
reader-original-colors = Показати оригінальні кольори
reader-dark-colors = Показати в темних кольорах
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
reader-sending = Надсилання…
reader-me = мені
reader-to = кому: { $names }
reader-to-label = кому:
reader-tick-delivered = Доставлено { $when }
reader-tick-no-bounce = Надіслано { $when }; повернення не надійшло, тож лист, найімовірніше, дійшов
reader-tick-bounced = Не доставлено: повернуто { $when }
reader-tick-read = Прочитано { $when } (сповіщення про прочитання)
reader-tick-opened = Відкрито, востаннє { $when } (відстеження відкриттів)
reader-starred = Із зірочкою
reader-chip-remove = Прибрати { $label }
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
reader-download-failed-reason = Не вдалося завантажити цей лист. { $reason }
reader-download-offline = Цей обліковий запис офлайн. Поверніться в онлайн, щоб завантажити цей лист.
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
security-look-up-key = Знайти ключ
key-card-verified = Перевірений підпис
key-card-verified-detail = Підпис дійсний, і ви довіряєте цьому ключу.
key-card-unverified = Підпис не перевірено
key-card-unverified-detail = Підпис дійсний, але ніщо не підтверджує, що ключ належить цій людині. Звірте з нею відбиток, а потім позначте ключ як надійний у GnuPG (Kleopatra або gpg --edit-key).
key-card-not-sender = Підписано кимось іншим
key-card-not-sender-detail = Підпис дійсний, але ключ не належить відправникові.
key-card-untrusted = Ключ ненадійний
key-card-untrusted-detail = Ви позначили цей ключ як ненадійний у GnuPG.
key-card-signature-expired = Термін дії підпису минув
key-card-signature-expired-detail = Підпис був дійсним, але термін його дії минув.
key-card-key-expired = Термін дії ключа минув
key-card-key-expired-detail = Підпис дійсний, але термін дії ключа відтоді минув.
key-card-key-revoked = Ключ відкликано
key-card-key-revoked-detail = Власник відкликав цей ключ, тому підписові не можна довіряти.
key-card-bad = Недійсний підпис
key-card-bad-detail = Цей лист змінено після підписання, або підпис підроблено.
key-card-signed-by = Підписав
key-card-belongs-to = Належить
key-card-fingerprint = Відбиток
key-card-signed = Підписано
key-card-key = Ключ
key-card-kind = { $standard }, { $algorithm }
key-card-created = Створено
key-card-expires = Дійсний до
key-card-never = Безстроково
key-card-issued-by = Видавець
key-card-found-in = Знайдено в
key-card-keyring = Ваша в’язка ключів GnuPG
key-card-copy = Копіювати відбиток
key-card-import-title = Імпортувати цей ключ?
key-card-from-directory = Знайдено в каталозі ключів { $domain }.
key-card-from-attachment = З вкладення { $name }.
key-card-import-note = Тоді Katna зможе перевіряти підписи цієї людини й шифрувати для неї листи. Щоб повністю довіряти ключу, звірте з нею відбиток.
key-card-cancel = Скасувати
key-card-import = Імпортувати ключ
key-card-looking-up = Пошук ключа…
key-card-looking-up-detail = Запит до каталогу ключів { $domain }.
key-card-not-found = Ключ не знайдено
key-card-not-found-detail = { $domain } не публікує ключ для цієї адреси. Попросіть відправника надіслати вам свій.
key-card-not-kept = Знайдений ключ не можна використати.
key-card-failed = Не вдалося отримати ключ
sender-failed-title = Можливо, цей лист не від { $domain }
sender-failed-body = Він не пройшов перевірки відправника, які виконує { $provider }. Будьте обережні з посиланнями, вкладеннями й відповідями.
sender-provider-unknown = ваш поштовий провайдер
sender-details = Докладніше
sender-details-hide = Сховати подробиці
sender-looks-safe = Виглядає безпечно
sender-move-to-spam = Перемістити до спаму
sender-checked-by = Перевірено: { $provider }
sender-checked-by-server = Перевірено: { $provider } ({ $server })
sender-dmarc = Домен відправника (DMARC)
sender-dkim = Підпис (DKIM)
sender-spf = Сервер-відправник (SPF)
sender-result-pass = Пройдено
sender-result-fail = Не пройдено
sender-result-unsure = Невідомо
sender-result-none = Немає
sender-result-missing = Не перевірено
sender-dmarc-pass = { $domain } підтверджує цього відправника.
sender-dmarc-fail = Лист не відповідає тому, як, за словами { $domain }, надсилається його пошта.
sender-dmarc-none = { $domain } не публікує правил для своєї пошти.
sender-dkim-pass = Підписано доменом { $domain }.
sender-dkim-fail = Підпис від { $domain } не відповідає листу.
sender-dkim-none = Лист не підписано.
sender-spf-pass = Надіслано із сервера, який указує { $domain }.
sender-spf-fail = Надіслано із сервера, якого { $domain } не вказує.
sender-spf-none = { $domain } не вказує своїх серверів.
sender-check-unsure = Перевірка не дала чіткої відповіді.
sender-unconfirmed = { $provider } не може підтвердити, що лист надійшов від { $domain }. Будь-хто може вказати будь-якого відправника.
sender-link-title = Відкрити це посилання?
sender-link-body = Цей лист не пройшов перевірку відправника. Посилання веде на { $host }:
sender-link-cancel = Скасувати
sender-link-open = Відкрити
tracking-opened = { $who }: лист відкрито { $count ->
    [one] { $count } раз
    [few] { $count } рази
    [many] { $count } разів
   *[other] { $count } раза
}, востаннє { $when }
tracking-opens-clicks = { $who }: лист відкрито { $opens ->
    [one] { $opens } раз
    [few] { $opens } рази
    [many] { $opens } разів
   *[other] { $opens } раза
} і перейдено за посиланням { $clicks ->
    [one] { $clicks } раз
    [few] { $clicks } рази
    [many] { $clicks } разів
   *[other] { $clicks } раза
}, востаннє { $when }
tracking-clicked = { $who }: перехід за посиланням { $clicks ->
    [one] { $clicks } раз
    [few] { $clicks } рази
    [many] { $clicks } разів
   *[other] { $clicks } раза
}, востаннє { $when }
tracking-maybe-opened = { $who }: лист, можливо, відкрито (Apple Mail завантажує зображення задля приватності)
tracking-seen-none = Ще ніхто не відкрив лист і не перейшов за посиланням
tracking-receipt = { $who }: надійшло сповіщення про прочитання
tracking-receipt-read = { $who } прочитав(-ла) його (сповіщення про прочитання), { $when }
tracking-receipt-displayed = Сповіщення про прочитання: { $who } — ваш лист відкрито
tracking-receipt-other = Сповіщення про прочитання: { $who } — ваш лист видалено чи оброблено без відкриття

## Remote images and pictures

remote-hidden = Зображення в цьому листі приховано.
remote-hidden-unconfirmed = Зображення приховано: не вдалося підтвердити відправника.
remote-hidden-failed = Зображення приховано: цей лист не пройшов перевірку відправника.
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
attachment-forward = Переслати
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
print-preview-title = Попередній перегляд друку
print-preview-laying-out = Розташування сторінок…
print-preview-pages = { $count ->
    [one] { $count } сторінка
    [few] { $count } сторінки
    [many] { $count } сторінок
   *[other] { $count } сторінки
}
print-preview-more = { $count ->
    [one] і ще { $count } сторінка
    [few] і ще { $count } сторінки
    [many] і ще { $count } сторінок
   *[other] і ще { $count } сторінки
}
print-preview-failed = не вдалося показати сторінки
print-preview-paper = Папір
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Оформлення
print-preview-as-shown = Як на екрані
print-preview-simple = Лише текст
print-preview-backgrounds = Тло
print-preview-cancel = Скасувати
print-preview-print = Друкувати
print-not-downloaded = (Ще не завантажено.)
print-encrypted = (Зашифровано. Відкрийте лист у Katna Mail, щоб надрукувати його текст.)
print-to = Кому: { $addresses }
print-cc = Копія: { $addresses }
text-pin = Закріпити вгорі
text-copy-address = Копіювати адресу

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Відкрийте цей лист, щоб переглянути вкладення.
text-copy = Копіювати
text-select-all = Вибрати все
