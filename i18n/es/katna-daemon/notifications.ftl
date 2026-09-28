# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } correo nuevo
    [many] { $count } de correos nuevos
   *[other] { $count } correos nuevos
}
notify-and-more = y { $count } más
notify-no-subject = (sin asunto)
notify-unknown-sender = Remitente desconocido
notify-snooze-back = Vuelve el correo pospuesto
notify-no-reply = Aún sin respuesta
notify-no-reply-to = Nadie ha respondido a «{ $subject }».
notify-tracking-opened = { $who } abrió { $subject }
notify-tracking-clicked = { $who } hizo clic en un enlace de { $subject }

notify-update-ready = Katna Mail se puede actualizar
notify-update-ready-body = La versión { $version } se ha descargado. Actualizar la instala y reinicia Katna Mail.
notify-update = Actualizar

## Its buttons

notify-open = Abrir
notify-reply-all = Responder a todos
notify-mark-read = Marcar como leído
notify-mark-all-read = Marcar todo como leído
notify-archive = Archivar
