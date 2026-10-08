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
notify-follow-up-sent = Seguimiento enviado
notify-follow-up-sent-to = Nadie había respondido a «{ $subject }», así que Katna ha enviado un seguimiento.
notify-follow-up-waiting = Seguimiento no enviado
notify-follow-up-waiting-to = Debía enviarse mientras este ordenador estaba apagado. «{ $subject }» vuelve a estar en Recibidos.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } abrió { $subject }
notify-tracking-clicked = { $who } hizo clic en un enlace de { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail se puede actualizar
notify-update-ready-body = La versión { $version } se ha descargado. Actualizar la instala y reinicia Katna Mail.
notify-update = Actualizar

## Something needs the user, shown once per problem

notify-signed-out = Vuelve a iniciar sesión
notify-signed-out-body = { $provider } ha cerrado la sesión de Katna en { $address }. El correo ha dejado de sincronizarse.
notify-sign-in = Iniciar sesión
notify-password-refused = Contraseña rechazada
notify-password-refused-body = El servidor de correo ha rechazado la contraseña de { $address }. Puede que haya cambiado.
notify-new-password = Nueva contraseña
notify-not-sent = «{ $subject }» no se ha enviado
notify-not-sent-no-subject = Un mensaje no se ha enviado
notify-not-sent-body = Está en la bandeja de salida, que explica por qué.
notify-open-outbox = Abrir bandeja de salida

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
notify-reply-quote-header = El { $date }, { $from } escribió:
notify-reply-quote-header-no-date = { $from } escribió:
notify-reply-all = Responder a todos
notify-mark-read = Marcar como leído
notify-mark-all-read = Marcar todo como leído
notify-archive = Archivar
notify-snooze-hour = Posponer 1 hora
notify-snooze-tomorrow = Mañana
notify-copy-code = Copiar { $code }
notify-link-verify = Verificar en { $domain }
notify-link-confirm = Confirmar en { $domain }
notify-link-activate = Activar en { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Archivado
notify-archived-count = { $count ->
    [one] { $count } mensaje sacado de Recibidos
    [many] { $count } de mensajes sacados de Recibidos
   *[other] { $count } mensajes sacados de Recibidos
}
notify-undo = Deshacer

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Código copiado
notify-code-not-copied = No se ha podido copiar el código

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Respuesta enviada a { $name }
notify-open-in-katna = Abrir en Katna
