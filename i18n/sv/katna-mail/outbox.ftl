# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Skickades inte eftersom { $reason }.
outbox-retrying = Har inte skickats än eftersom { $reason }. Katna försöker igen av sig själv.
outbox-waiting-sign-in = Väntar på att du loggar in på { $address } igen. Då skickas det.
outbox-waiting-password = Väntar på det nya lösenordet för { $address }. Då skickas det.
outbox-waiting-connection = Väntar på en anslutning. Det skickas när du är online igen.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = det saknar mottagare
outbox-reason-address = en adress det skickas till finns inte
outbox-reason-too-large = det är för stort för e-postservern
outbox-reason-blocked = e-postservern blockerade det
outbox-reason-gone = dess kopia på den här datorn är borta
outbox-reason-refused = e-postservern nekade det

## Buttons and notes

outbox-try-again = Försök igen
outbox-edit = Redigera
outbox-delete = Radera
outbox-deleted = Raderat från Utkorgen
outbox-sending-again = Skickar igen…

outbox-snackbar-not-sent = ”{ $subject }” skickades inte eftersom { $reason }.
outbox-open = Utkorgen
