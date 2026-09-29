# Katna Mail, Spanish (Spain) (Español): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Crear
tasks-all = Todas las tareas
tasks-starred = Destacadas
tasks-new-list = Crear lista nueva
tasks-on-this-computer = En este ordenador
tasks-my-tasks = Mis tareas
tasks-list-name-placeholder = Nombre de la lista

## Lists and tasks

tasks-loading = Leyendo tus tareas…
tasks-no-lists = Tus listas de tareas aparecen aquí.
tasks-add = Añadir una tarea
tasks-title-placeholder = Título
tasks-add-step = Añadir una subtarea
tasks-empty = Aún no hay tareas. Añade una arriba.
tasks-starred-empty = Destaca una tarea para verla aquí.
tasks-completed = { $count ->
    [one] Completadas ({ $count })
    [many] Completadas ({ $count })
   *[other] Completadas ({ $count })
}
tasks-list-options = Opciones de la lista
tasks-rename-list = Cambiar el nombre de la lista
tasks-delete-list = Eliminar lista
tasks-mark-done = Marcar como completada
tasks-mark-open = Marcar como no completada
tasks-star = Destacar
tasks-unstar = Quitar destacado
tasks-edit-title = Editar título
tasks-details = Detalles
tasks-delete = Eliminar
tasks-move-to = Mover a { $list }
tasks-from-mail = Correo
tasks-open-mail = Abrir el correo
tasks-no-subject = (sin asunto)

## The details dialog

tasks-notes-placeholder = Añadir detalles
tasks-date = Fecha
tasks-no-date = Sin fecha
tasks-time-placeholder = Añadir hora
tasks-repeat = Repetir
tasks-repeat-never = No se repite
tasks-repeat-daily = Todos los días
tasks-repeat-weekly = Todas las semanas
tasks-repeat-monthly = Todos los meses
tasks-repeat-yearly = Todos los años
tasks-repeat-other = Personalizado
tasks-cancel = Cancelar
tasks-save = Guardar
tasks-not-a-time = “{ $text }” no es una hora, por ejemplo { $example }.

## Due days

tasks-due-today = Hoy
tasks-due-tomorrow = Mañana
tasks-due-yesterday = Ayer
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Tarea completada
tasks-toast-deleted = Tarea eliminada
tasks-toast-added = { $count ->
    [one] Añadida a Tareas
    [many] { $count } tareas añadidas
   *[other] { $count } tareas añadidas
}
tasks-mail-gone = Ese correo ya no está aquí.
tasks-toast-list-deleted = Lista eliminada
tasks-toast-moved = Tarea movida a { $list }
