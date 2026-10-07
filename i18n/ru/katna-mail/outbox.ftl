# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Не отправлено: { $reason }.
outbox-retrying = Пока не отправлено: { $reason }. Katna попробует снова сама.
outbox-waiting-sign-in = Ждёт, пока вы снова войдёте в { $address }. Тогда и отправится.
outbox-waiting-password = Ждёт новый пароль для { $address }. Тогда и отправится.
outbox-waiting-connection = Ждёт подключения. Отправится, когда вы снова будете в сети.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = нет получателей
outbox-reason-address = один из адресов получателей не существует
outbox-reason-too-large = письмо слишком велико для почтового сервера
outbox-reason-blocked = почтовый сервер его заблокировал
outbox-reason-gone = его копии на этом компьютере больше нет
outbox-reason-refused = почтовый сервер его отклонил

## Buttons and notes

outbox-try-again = Повторить
outbox-edit = Изменить
outbox-delete = Удалить
outbox-deleted = Удалено из «Исходящих»
outbox-sending-again = Повторная отправка…
outbox-snackbar-not-sent = Письмо «{ $subject }» не отправлено: { $reason }.
outbox-open = Исходящие
