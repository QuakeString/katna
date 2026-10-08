# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Не надіслано, тому що { $reason }.
outbox-retrying = Ще не надіслано, тому що { $reason }. Katna спробує знову сама.
outbox-waiting-sign-in = Очікує, доки ви знову ввійдете в { $address }. Тоді лист буде надіслано.
outbox-waiting-password = Очікує нового пароля для { $address }. Тоді лист буде надіслано.
outbox-waiting-connection = Очікує з’єднання. Лист буде надіслано, коли ви знову будете онлайн.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = у листа немає одержувачів
outbox-reason-address = адреси, на яку його надіслано, не існує
outbox-reason-too-large = він завеликий для поштового сервера
outbox-reason-blocked = поштовий сервер його заблокував
outbox-reason-gone = його копії на цьому комп’ютері більше немає
outbox-reason-refused = поштовий сервер його відхилив

## Buttons and notes

outbox-try-again = Спробувати знову
outbox-edit = Змінити
outbox-delete = Видалити
outbox-deleted = Видалено з «Вихідних»
outbox-sending-again = Повторне надсилання…
outbox-snackbar-not-sent = «{ $subject }» не надіслано, тому що { $reason }.
outbox-open = Вихідні
