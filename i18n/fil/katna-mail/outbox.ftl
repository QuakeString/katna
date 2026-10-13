# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Hindi naipadala dahil { $reason }.
outbox-retrying = Hindi pa naipapadala dahil { $reason }. Kusang susubukan muli ng Katna.
outbox-waiting-sign-in = Hinihintay na mag-sign in ka muli sa { $address }. Ipapadala ito pagkatapos.
outbox-waiting-password = Hinihintay ang bagong password ng { $address }. Ipapadala ito pagkatapos.
outbox-waiting-connection = Naghihintay ng koneksyon. Ipapadala ito kapag online ka na muli.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = wala itong recipient
outbox-reason-address = hindi umiiral ang isang address na pinadalhan nito
outbox-reason-too-large = masyado itong malaki para sa mail server
outbox-reason-blocked = hinarangan ito ng mail server
outbox-reason-gone = wala na ang kopya nito sa computer na ito
outbox-reason-refused = tinanggihan ito ng mail server

## Buttons and notes

outbox-try-again = Subukang muli
outbox-edit = I-edit
outbox-delete = I-delete
outbox-deleted = Na-delete mula sa Outbox
outbox-sending-again = Ipinapadala muli…
outbox-snackbar-not-sent = Hindi naipadala ang “{ $subject }” dahil { $reason }.
outbox-open = Outbox
