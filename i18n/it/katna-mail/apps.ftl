# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = Posta
rail-calendar = Calendario
rail-contacts = Contatti
rail-tasks = Attività
rail-notes = Note
rail-files = File

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Prossimamente
app-calendar-promise = I tuoi calendari CalDAV, gli inviti alle riunioni ricevuti per posta e i promemoria, accanto alla Posta in arrivo.
app-tasks-promise = Elenchi di cose da fare sincronizzati con CalDAV e attività create dalla posta.
app-notes-promise = Note veloci e note su un messaggio o una conversazione per dopo.

## Contacts page

app-contacts-loading = Raccolta delle persone dalla tua posta…
app-contacts-empty = Qui compaiono le persone con cui scrivi.
app-contacts-count = { $count ->
    [one] { $count } persona dalla tua posta, prima quelle con cui scrivi di più
    [many] { $count } di persone dalla tua posta, prima quelle con cui scrivi di più
   *[other] { $count } persone dalla tua posta, prima quelle con cui scrivi di più
}
app-contacts-top = { $count ->
    [one] La persona con cui scrivi di più
    [many] Le prime { $count } di persone dalla tua posta, prima quelle con cui scrivi di più
   *[other] Le prime { $count } persone dalla tua posta, prima quelle con cui scrivi di più
}
app-contacts-messages = { $count ->
    [one] { $count } messaggio
    [many] { $count } di messaggi
   *[other] { $count } messaggi
}
app-contacts-last = ultimo: { $date }
