# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } nuova email
    [many] { $count } di nuove email
   *[other] { $count } nuove email
}
notify-and-more = { $count ->
    [one] e { $count } altra
   *[other] e altre { $count }
}
notify-no-subject = (nessun oggetto)
notify-unknown-sender = Mittente sconosciuto

## Its buttons

notify-open = Apri
notify-reply-all = Rispondi a tutti
notify-mark-read = Segna come già letto
notify-mark-all-read = Segna tutti come già letti
notify-archive = Archivia
