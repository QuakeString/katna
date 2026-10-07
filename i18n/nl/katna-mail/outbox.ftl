# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Niet verzonden omdat { $reason }.
outbox-retrying = Nog niet verzonden omdat { $reason }. Katna probeert het zelf opnieuw.
outbox-waiting-sign-in = Wacht tot je je opnieuw aanmeldt bij { $address }. Dan wordt het verzonden.
outbox-waiting-password = Wacht op het nieuwe wachtwoord van { $address }. Dan wordt het verzonden.
outbox-waiting-connection = Wacht op een verbinding. Het wordt verzonden zodra je weer online bent.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = het geen ontvangers heeft
outbox-reason-address = een adres waarnaar het gaat niet bestaat
outbox-reason-too-large = het te groot is voor de mailserver
outbox-reason-blocked = de mailserver het heeft geblokkeerd
outbox-reason-gone = de kopie op deze computer weg is
outbox-reason-refused = de mailserver het heeft geweigerd

## Buttons and notes

outbox-try-again = Opnieuw proberen
outbox-edit = Bewerken
outbox-delete = Verwijderen
outbox-deleted = Verwijderd uit het postvak UIT
outbox-sending-again = Opnieuw verzenden…

outbox-snackbar-not-sent = ‘{ $subject }’ is niet verzonden omdat { $reason }.
outbox-open = Postvak UIT
