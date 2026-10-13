# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = Correo
rail-calendar = Calendario
rail-contacts = Contactos
rail-tasks = Tareas
rail-notes = Notas
rail-files = Archivos

## Rail right-click menu

rail-menu-open = Abrir { $app }
rail-menu-settings = Ajustes de { $app }
rail-menu-turn-off = Desactivar { $app }…

## Turning an app off (Settings > Apps)

app-off-title = ¿Desactivar { $app }?
app-off-body = Katna deja de sincronizar { $app } y lo quita de:
app-off-keep = Conservar una copia en este ordenador
app-off-keep-detail = Volver a activarlo es instantáneo
app-off-remove = Quitar la copia de este ordenador
app-off-remove-detail = No cambia nada en tus cuentas, y al volver a activarlo se descarga de nuevo. Lo que solo está en este ordenador, o aún no se ha enviado, se conserva.
app-off-cancel = Cancelar
app-off-confirm = Desactivar
app-off-done = { $app } desactivado
app-off-note = { $app } está desactivado
app-off-turn-on = Activar
app-off-leaves-calendar-rail = La barra de aplicaciones y Ctrl+2
app-off-leaves-calendar-agenda = La agenda junto a tu correo
app-off-leaves-calendar-meeting = Programar una reunión, y Abrir en Calendario en las invitaciones
app-off-leaves-calendar-reminders = Recordatorios de eventos
app-off-leaves-calendar-desktop = Eventos en KRunner y en el reloj del escritorio
app-off-leaves-contacts-rail = La barra de aplicaciones y Ctrl+3
app-off-leaves-contacts-card = Añadir a contactos en la tarjeta de un remitente
app-off-leaves-contacts-birthdays = Cumpleaños en Calendario
app-off-leaves-tasks-rail = La barra de aplicaciones y Ctrl+4
app-off-leaves-tasks-mail = Añadir a Tareas en el correo, y Mayús+T
app-off-leaves-tasks-calendar = Tareas en Calendario
app-off-leaves-tasks-tray = Nueva tarea en la bandeja del sistema, y Meta+Alt+T
app-off-leaves-tasks-reminders = Recordatorios de tareas
app-off-leaves-notes-rail = La barra de aplicaciones y Ctrl+5
app-off-leaves-notes-mail = Añadir una nota en el correo
app-off-leaves-notes-meetings = Notas de reunión en los eventos
app-off-leaves-notes-tray = Nueva nota en la bandeja del sistema, y Meta+Alt+N
app-off-leaves-notes-reminders = Recordatorios de notas
app-off-leaves-files-rail = La barra de aplicaciones y Ctrl+7
app-off-leaves-files-compose = Archivos al adjuntar en Redactar

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Próximamente
app-calendar-promise = Tus calendarios CalDAV, las invitaciones a reuniones de tu correo y los recordatorios, junto a tu bandeja de entrada.
app-tasks-promise = Listas de tareas que se sincronizan con CalDAV y tareas creadas a partir de correos.
app-notes-promise = Notas rápidas y notas sobre un correo o una conversación para más tarde.

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
