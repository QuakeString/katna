# Katna Mail, Spanish (Spain) (Español): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notas
notes-view-reminders = Recordatorios
notes-view-archive = Archivo
notes-view-trash = Papelera
notes-edit-labels = Editar etiquetas
notes-search = Buscar notas
notes-loading = Abriendo tus notas…

## Board

notes-take-a-note = Crear una nota…
notes-new-list = Nueva lista
notes-new-note = Nueva nota
notes-pinned = Fijadas
notes-others = Otras
notes-empty = Las notas que añadas aparecerán aquí
notes-archive-empty = Las notas archivadas aparecerán aquí
notes-trash-empty = No hay notas en la papelera
notes-none-found = No hay notas que coincidan
notes-label-empty = Aún no hay notas con esta etiqueta
notes-reminders-empty = Las notas con recordatorios próximos aparecerán aquí
notes-trash-note = Las notas de la papelera se eliminan después de 7 días.
notes-empty-trash = Vaciar papelera
notes-ticked = { $count ->
    [one] + { $count } elemento marcado
    [many] + { $count } elementos marcados
   *[other] + { $count } elementos marcados
}
notes-select = Seleccionar nota
notes-selected = { $count ->
    [one] { $count } seleccionada
    [many] { $count } seleccionadas
   *[other] { $count } seleccionadas
}
notes-select-clear = Borrar selección

## A note's buttons

notes-pin = Fijar nota
notes-unpin = Dejar de fijar nota
notes-archive = Archivar
notes-unarchive = Desarchivar
notes-delete = Eliminar nota
notes-restore = Restaurar
notes-delete-forever = Eliminar permanentemente
notes-color = Color de fondo
notes-checkboxes = Mostrar u ocultar casillas
notes-labels = Etiquetas
notes-close = Cerrar
notes-more = Más
notes-make-copy = Hacer una copia
notes-remind = Recordármelo
notes-add-picture = Añadir imagen
notes-history = Historial de versiones
notes-ai = Ayúdame a escribir
notes-send-as-mail = Enviar como correo
notes-save-markdown = Guardar como Markdown
notes-save-pdf = Guardar como PDF

## The open note

notes-title = Título
notes-edited = Editada: { $date }
notes-on-this-computer = En este ordenador
notes-where = Dónde se guarda esta nota
notes-untitled = Nota sin título

## Pictures

notes-picture-choose = Añadir imágenes
notes-picture-remove = Quitar imagen
notes-picture-too-big = Una nota admite imágenes de hasta { $size }
notes-picture-kind = Ese archivo no es una imagen que Katna pueda mostrar
notes-picture-unreadable = No se ha podido leer { $name }: { $error }

## Reminders

notes-remind-me = Recordármelo
notes-remind-off = Quitar recordatorio
notes-remind-in-the-past = Elige una hora que aún no haya pasado
notes-remind-today = Hoy, { $time }
notes-remind-tomorrow = Mañana, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Recordatorio para el { $when }
notes-reminder-off = Recordatorio quitado

## Links between notes

notes-link-note = Enlazar una nota
notes-link-new = Nueva nota «{ $title }»
notes-linked-from = Enlazada desde
notes-link-gone = Esa nota ya no está aquí
notes-new-note-gone = La nota nueva ya no existe.

## Version history

notes-versions = Versiones
notes-version-now = Ahora
notes-version-here = Tú, en este ordenador
notes-version-yesterday = Ayer, { $time }
notes-version-changes = { $count ->
    [one] { $count } cambio
    [many] { $count } de cambios
   *[other] { $count } cambios
}
notes-version-from = Desde { $device }
notes-version-elsewhere = Desde otro dispositivo
notes-version-created = Creada
notes-version-restore = Restaurar esta versión
notes-version-restored = Versión restaurada
notes-history-none = Aún no hay versiones anteriores

## AI help

notes-ai-tidy = Pulir el texto
notes-ai-checklist = Convertir en lista de comprobación
notes-ai-summarise = Resumir
notes-ai-empty = Escribe algo primero
notes-ai-tidied = Texto pulido. Ctrl+Z lo deshace.
notes-ai-listed = Convertida en lista de comprobación. Ctrl+Z lo deshace.
notes-ai-summarised = Resumen añadido arriba

## Labels

notes-label-note = Etiquetar nota
notes-label-name = Introduce el nombre de la etiqueta
notes-label-create = Crear “{ $name }”
notes-label-remove = Quitar etiqueta
notes-label-delete = Eliminar etiqueta
notes-labels-none = Aún no hay etiquetas. Añade una desde el botón de etiqueta de una nota.
notes-labels-done = Hecho
notes-label-renamed = Etiqueta renombrada como “{ $name }”
notes-label-deleted = Etiqueta “{ $name }” eliminada

## A note about a mail

notes-mail = Correo
notes-open-mail = Abrir el correo
notes-open-note = Abrir la nota

## Meeting notes

notes-meeting-take = Tomar notas de la reunión
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Asistentes: { $names }
notes-meeting-notes = Notas
notes-meeting-actions = Tareas pendientes
notes-event = Evento
notes-open-event = Abrir el evento

## Formatting

notes-format = Formato
notes-format-heading-1 = Título 1
notes-format-heading-2 = Título 2
notes-format-normal = Texto normal
notes-format-bold = Negrita
notes-format-italic = Cursiva
notes-format-underline = Subrayado
notes-format-quote = Cita
notes-format-code = Código
notes-format-divider = Separador
notes-format-clear = Borrar formato

## Tasks

notes-make-task = Convertir en tarea

## Colors (tooltips)

notes-color-none = Sin color
notes-color-coral = Coral
notes-color-peach = Melocotón
notes-color-sand = Arena
notes-color-mint = Menta
notes-color-sage = Salvia
notes-color-fog = Niebla
notes-color-storm = Tormenta
notes-color-dusk = Anochecer
notes-color-blossom = Flor
notes-color-clay = Arcilla
notes-color-chalk = Tiza

## Messages at the foot of the window

notes-archived = Nota archivada
notes-unarchived = Nota desarchivada
notes-trashed = Nota movida a la papelera
notes-restored = Nota restaurada
notes-saved = Nota guardada
notes-pinned-count = { $count ->
    [one] Nota fijada
    [many] { $count } de notas fijadas
   *[other] { $count } notas fijadas
}
notes-unpinned-count = { $count ->
    [one] Nota desfijada
    [many] { $count } de notas desfijadas
   *[other] { $count } notas desfijadas
}
notes-colored-count = { $count ->
    [one] Color cambiado
    [many] Color cambiado en { $count } de notas
   *[other] Color cambiado en { $count } notas
}
notes-archived-count = { $count ->
    [one] Nota archivada
    [many] { $count } de notas archivadas
   *[other] { $count } notas archivadas
}
notes-unarchived-count = { $count ->
    [one] Nota desarchivada
    [many] { $count } de notas desarchivadas
   *[other] { $count } notas desarchivadas
}
notes-trashed-count = { $count ->
    [one] Nota movida a la papelera
    [many] { $count } de notas movidas a la papelera
   *[other] { $count } notas movidas a la papelera
}
notes-restored-count = { $count ->
    [one] Nota restaurada
    [many] { $count } de notas restauradas
   *[other] { $count } notas restauradas
}
notes-copied-count = { $count ->
    [one] Copia creada
    [many] { $count } de copias creadas
   *[other] { $count } copias creadas
}
notes-empty-discarded = Nota vacía descartada
notes-mail-gone = Ese correo ya no está aquí
notes-deleted-forever = { $count ->
    [one] Nota eliminada permanentemente
    [many] { $count } notas eliminadas permanentemente
   *[other] { $count } notas eliminadas permanentemente
}
