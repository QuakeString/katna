# Katna Mail, Spanish (Spain) (Español): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notas
notes-view-archive = Archivo
notes-view-trash = Papelera
notes-edit-labels = Editar etiquetas
notes-search = Buscar notas
notes-loading = Abriendo tus notas…

## Board

notes-take-a-note = Crear una nota…
notes-new-list = Nueva lista
notes-pinned = Fijadas
notes-others = Otras
notes-empty = Las notas que añadas aparecerán aquí
notes-archive-empty = Las notas archivadas aparecerán aquí
notes-trash-empty = No hay notas en la papelera
notes-none-found = No hay notas que coincidan
notes-label-empty = Aún no hay notas con esta etiqueta
notes-trash-note = Las notas de la papelera se eliminan después de 7 días.
notes-empty-trash = Vaciar papelera
notes-ticked = { $count ->
    [one] + { $count } elemento marcado
    [many] + { $count } elementos marcados
   *[other] + { $count } elementos marcados
}

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

## The open note

notes-title = Título
notes-edited = Editada: { $date }
notes-on-this-computer = En este ordenador
notes-where = Dónde se guarda esta nota

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
notes-empty-discarded = Nota vacía descartada
notes-mail-gone = Ese correo ya no está aquí
notes-deleted-forever = { $count ->
    [one] Nota eliminada permanentemente
    [many] { $count } notas eliminadas permanentemente
   *[other] { $count } notas eliminadas permanentemente
}
