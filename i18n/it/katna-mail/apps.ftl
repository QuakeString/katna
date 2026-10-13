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

## Rail right-click menu

rail-menu-open = Apri { $app }
rail-menu-settings = Impostazioni di { $app }
rail-menu-turn-off = Disattiva { $app }…

## Turning an app off (Settings > Apps)

app-off-title = Disattivare { $app }?
app-off-body = Katna smette di sincronizzare { $app } e lo rimuove da:
app-off-keep = Mantieni una copia su questo computer
app-off-keep-detail = Riattivarlo è immediato
app-off-remove = Rimuovi la copia su questo computer
app-off-remove-detail = Sui tuoi account non cambia nulla e, riattivandolo, viene scaricato di nuovo. Ciò che è solo su questo computer, o non ancora inviato, resta.
app-off-cancel = Annulla
app-off-confirm = Disattiva
app-off-done = { $app } disattivato
app-off-note = { $app } è disattivato
app-off-turn-on = Attiva
app-off-leaves-calendar-rail = La barra laterale e Ctrl+2
app-off-leaves-calendar-agenda = L’agenda accanto alla posta
app-off-leaves-calendar-meeting = Pianifica riunione e Apri in Calendario sugli inviti
app-off-leaves-calendar-reminders = Promemoria degli eventi
app-off-leaves-calendar-desktop = Eventi in KRunner e nell’orologio del desktop
app-off-leaves-contacts-rail = La barra laterale e Ctrl+3
app-off-leaves-contacts-card = Aggiungi ai contatti sulla scheda di un mittente
app-off-leaves-contacts-birthdays = Compleanni in Calendario
app-off-leaves-tasks-rail = La barra laterale e Ctrl+4
app-off-leaves-tasks-mail = Aggiungi ad Attività sulla posta e Shift+T
app-off-leaves-tasks-calendar = Attività in Calendario
app-off-leaves-tasks-tray = Nuova attività nell’area di notifica e Meta+Alt+T
app-off-leaves-tasks-reminders = Promemoria delle attività
app-off-leaves-notes-rail = La barra laterale e Ctrl+5
app-off-leaves-notes-mail = Aggiungi una nota alla posta
app-off-leaves-notes-meetings = Appunti delle riunioni sugli eventi
app-off-leaves-notes-tray = Nuova nota nell’area di notifica e Meta+Alt+N
app-off-leaves-notes-reminders = Promemoria delle note
app-off-leaves-files-rail = La barra laterale e Ctrl+7
app-off-leaves-files-compose = File quando alleghi in Scrittura

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
