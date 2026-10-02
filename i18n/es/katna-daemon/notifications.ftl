# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } correo nuevo
    [many] { $count } de correos nuevos
   *[other] { $count } correos nuevos
}
notify-and-more = y { $count } más
notify-no-subject = (sin asunto)
notify-unknown-sender = Remitente desconocido

## Reminders the user asked for (same buttons)

notify-snooze-back = Vuelve el correo pospuesto
notify-no-reply = Aún sin respuesta
notify-no-reply-to = Nadie ha respondido a «{ $subject }».

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } abrió { $subject }
notify-tracking-clicked = { $who } hizo clic en un enlace de { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail se puede actualizar
notify-update-ready-body = La versión { $version } se ha descargado. Actualizar la instala y reinicia Katna Mail.
notify-update = Actualizar

## Reminders of calendar events

notify-event-now = Ahora
notify-event-in-minutes = { $count ->
    [one] En { $count } minuto
   *[other] En { $count } minutos
}
notify-event-in-hours = { $count ->
    [one] En { $count } hora
   *[other] En { $count } horas
}
notify-event-in-days = { $count ->
    [1] Mañana
    [one] En { $count } día
   *[other] En { $count } días
}
notify-event-all-day = Todo el día
notify-event-join = Unirse
notify-event-snooze = Posponer 5 min
notify-task-done = Marcar como completada

## The buttons of new-mail notifications and reminders

notify-open = Abrir
notify-peek = Vistazo
notify-reply = Responder
notify-reply-placeholder = Responder a { $name }…
notify-send = Enviar
notify-reply-all = Responder a todos
notify-mark-read = Marcar como leído
notify-mark-all-read = Marcar todo como leído
notify-archive = Archivar

## After Archive on a notification: a short note in the same place

notify-archived = Archivado
notify-archived-count = { $count ->
    [one] { $count } mensaje sacado de Recibidos
    [many] { $count } de mensajes sacados de Recibidos
   *[other] { $count } mensajes sacados de Recibidos
}
notify-undo = Deshacer

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Respuesta enviada a { $name }
notify-open-in-katna = Abrir en Katna
