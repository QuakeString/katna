# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Nicht gesendet, weil { $reason }.
outbox-retrying = Noch nicht gesendet, weil { $reason }. Katna versucht es von selbst erneut.
outbox-waiting-sign-in = Wartet darauf, dass Sie sich erneut bei { $address } anmelden. Dann wird sie gesendet.
outbox-waiting-password = Wartet auf das neue Passwort für { $address }. Dann wird sie gesendet.
outbox-waiting-connection = Wartet auf eine Verbindung. Sie wird gesendet, sobald Sie wieder online sind.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = sie keine Empfänger hat
outbox-reason-address = eine Empfängeradresse nicht existiert
outbox-reason-too-large = sie zu groß für den E-Mail-Server ist
outbox-reason-blocked = der E-Mail-Server sie blockiert hat
outbox-reason-gone = ihre Kopie auf diesem Computer nicht mehr da ist
outbox-reason-refused = der E-Mail-Server sie abgelehnt hat

## Buttons and notes

outbox-try-again = Erneut versuchen
outbox-edit = Bearbeiten
outbox-delete = Löschen
outbox-deleted = Aus dem Postausgang gelöscht
outbox-sending-again = Wird erneut gesendet…

outbox-snackbar-not-sent = „{ $subject }“ wurde nicht gesendet, weil { $reason }.
outbox-open = Postausgang
