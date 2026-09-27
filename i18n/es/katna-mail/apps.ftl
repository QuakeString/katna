# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = Correo
rail-calendar = Calendario
rail-contacts = Contactos
rail-tasks = Tareas
rail-notes = Notas
rail-feeds = Feeds

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Próximamente
app-calendar-promise = Tus calendarios CalDAV, las invitaciones a reuniones de tu correo y los recordatorios, junto a tu bandeja de entrada.
app-tasks-promise = Listas de tareas que se sincronizan con CalDAV y tareas creadas a partir de correos.
app-notes-promise = Notas rápidas y notas sobre un correo o una conversación para más tarde.
app-feeds-promise = Lee feeds RSS y Atom junto a tu correo.

## Contacts page

app-contacts-loading = Reuniendo a las personas de tu correo…
app-contacts-empty = Aquí aparecen las personas con las que te escribes.
app-contacts-count = { $count ->
    [one] { $count } persona de tu correo, primero con quien más te escribes
    [many] { $count } de personas de tu correo, primero con quienes más te escribes
   *[other] { $count } personas de tu correo, primero con quienes más te escribes
}
app-contacts-top = { $count ->
    [one] La persona de tu correo con quien más te escribes
    [many] Las { $count } de personas de tu correo con quienes más te escribes
   *[other] Las { $count } personas de tu correo con quienes más te escribes
}
app-contacts-messages = { $count ->
    [one] { $count } mensaje
    [many] { $count } de mensajes
   *[other] { $count } mensajes
}
app-contacts-last = último: { $date }
