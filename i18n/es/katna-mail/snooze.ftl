# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Posponer hasta…
snooze-later-today = Más tarde hoy
snooze-tomorrow = Mañana
snooze-this-weekend = Este fin de semana
snooze-next-week = La próxima semana
snooze-pick = Elegir fecha y hora
snooze-back = Volver a las horas
snooze-type-placeholder = Escribe una hora
snooze-type-hint = Como «mar 15:00», «mañana» o «en 2 horas»
snooze-type-hint-unclear = Katna no entiende eso como una hora
snooze-type-unclear = «{ $text }» no es una hora que Katna conozca

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Posponer
remind-tab = Recordármelo
snooze-says = Lo oculta hasta entonces
remind-says = Lo deja donde está y te avisa
remind-before-due = Antes del vencimiento
remind-note = Nota (opcional)
remind-note-placeholder = El asunto, si se deja vacía
toast-remind-set = Recordatorio para el { $date }
remind-chat-line = Recordatorio { $date } · { $title }
remind-done = Hecho
toast-remind-done = Recordatorio completado
snooze-chat-line = Pospuesto hasta { $date }
snooze-chat-change = Cambiar

## The date and time picker

snooze-cancel = Cancelar
snooze-save = Guardar
snooze-in-the-past = Elige una hora posterior a la actual.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Hacer seguimiento si no responden…
follow-up-title = Hacer seguimiento si no responden
follow-up-off = Desactivado
follow-up-days = { $days ->
    [one] { $days } día
    [many] { $days } de días
   *[other] { $days } días
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } semana
    [many] { $weeks } de semanas
   *[other] { $weeks } semanas
}
follow-up-pick = Elegir…
follow-up-pick-title = Hacer seguimiento si no responden antes del
follow-up-remind = Recordármelo
follow-up-remind-note = La conversación vuelve al principio de tus Recibidos
follow-up-send = Enviar un seguimiento por mí
follow-up-send-note = A las mismas personas, en la misma conversación
follow-up-send-encrypted = No para correo cifrado
follow-up-text-placeholder = Qué escribir
follow-up-text-named = Hola, { $name }: solo quería confirmar que viste mi mensaje de abajo.
follow-up-text = Hola: solo quería confirmar que viste mi mensaje de abajo.
follow-up-template = Usar una plantilla
follow-up-signature = Se añade tu firma
follow-up-again = Si sigue sin haber respuesta, volver a hacer seguimiento tras
follow-up-note = Se detiene en cuanto alguien de la conversación responde. Las respuestas automáticas no cuentan.
follow-up-note-send = Se detiene en cuanto alguien de la conversación responde. Se envía los días laborables de { $start } a { $end }, y nunca con más de un día de retraso.
follow-up-cancel = Cancelar
follow-up-done = Hecho
follow-up-chip-send = Seguimiento en { $time }
follow-up-chip-remind = Recordatorio en { $time }
follow-up-chip-send-on = Seguimiento { $date }
follow-up-chip-remind-on = Recordatorio { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Aún sin respuesta
follow-up-card-title-waiting = Tu seguimiento está en espera
follow-up-card-send = Katna envía tu seguimiento el { $date }. Se detiene cuando alguien responde.
follow-up-card-send-twice = Katna envía tu seguimiento el { $date } y una vez más después. Se detiene cuando alguien responde.
follow-up-card-remind = Si nadie responde, esta conversación vuelve a tus Recibidos el { $date }.
follow-up-card-waiting = Venció mientras tu ordenador estaba apagado, así que no se envió con retraso. Envíalo ahora, elige otra hora o detenlo.
follow-up-card-edit = Editar
follow-up-card-edit-title = Hacer seguimiento el
follow-up-card-send-now = Enviar ahora
follow-up-card-stop = Detener
follow-up-chat-send = Seguimiento · { $date } si nadie responde
follow-up-chat-step = Seguimiento { $step } de { $steps } · { $date } si nadie responde
follow-up-chat-waiting = Seguimiento en espera · venció mientras tu ordenador estaba apagado
follow-up-chat-remind = De vuelta en Recibidos { $date } si no hay respuesta
toast-follow-up-sent = Seguimiento enviado
toast-follow-up-stopped = Seguimiento detenido
toast-follow-up-moved = Seguimiento movido al { $date }

nudge-row = Enviado { $days ->
    [one] hace 1 día
    [many] hace { $days } de días
   *[other] hace { $days } días
}. ¿Hacer seguimiento?
nudge-row-tip = Escribir un seguimiento a todos los participantes
nudge-follow-up = Hacer seguimiento
nudge-dismiss = Descartar
nudge-card-title = Aún sin respuesta
nudge-card-text = Preguntaste algo { $days ->
    [one] hace 1 día
    [many] hace { $days } de días
   *[other] hace { $days } días
} y nadie respondió.
nudge-chat-line = Enviado { $days ->
    [one] hace 1 día
    [many] hace { $days } de días
   *[other] hace { $days } días
}, aún sin respuesta
toast-nudge-dismissed = Aviso descartado
