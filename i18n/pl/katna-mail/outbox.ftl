# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Nie wysłano, ponieważ { $reason }.
outbox-retrying = Jeszcze nie wysłano, ponieważ { $reason }. Katna sama spróbuje ponownie.
outbox-waiting-sign-in = Czeka, aż ponownie zalogujesz się do { $address }. Wtedy zostanie wysłana.
outbox-waiting-password = Czeka na nowe hasło do { $address }. Wtedy zostanie wysłana.
outbox-waiting-connection = Czeka na połączenie. Zostanie wysłana, gdy znów będziesz online.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = nie ma odbiorców
outbox-reason-address = adres, na który jest wysyłana, nie istnieje
outbox-reason-too-large = jest za duża dla serwera poczty
outbox-reason-blocked = serwer poczty ją zablokował
outbox-reason-gone = jej kopii na tym komputerze już nie ma
outbox-reason-refused = serwer poczty ją odrzucił

## Buttons and notes

outbox-try-again = Spróbuj ponownie
outbox-edit = Edytuj
outbox-delete = Usuń
outbox-deleted = Usunięto ze skrzynki nadawczej
outbox-sending-again = Ponowne wysyłanie…

outbox-snackbar-not-sent = Nie wysłano „{ $subject }”, ponieważ { $reason }.
outbox-open = Skrzynka nadawcza
