# Katna Mail, Spanish (Spain) (Español): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Nueva tarea
tasks-all = Todas las tareas
tasks-today = Hoy
tasks-upcoming = Próximas
tasks-starred = Destacadas
tasks-completed-view = Completadas
tasks-new-list = Crear lista nueva
tasks-labels-heading = Etiquetas
tasks-on-this-computer = En este ordenador
tasks-my-tasks = Mis tareas
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Vuelve a iniciar sesión para mostrar las tareas
tasks-account-signed-in = Has vuelto a iniciar sesión en { $address }. Obteniendo tus tareas…
tasks-account-sign-in-refused = { $provider } no ha dejado entrar a Katna. Vuelve a intentarlo y permite el acceso a tus tareas.
tasks-account-refused = El servidor no ha aceptado la contraseña. Yahoo, iCloud, Zoho y otros necesitan una contraseña de aplicación.
tasks-account-change-password = Cambiar contraseña
tasks-account-change-password-tooltip = Escribe la nueva contraseña; Katna la comprueba con el servidor
tasks-account-not-enabled = El acceso a las tareas para Katna aún no está activado.
tasks-account-failed = No se han podido leer las listas de tareas.
# $reason is the server's own words, in English.
tasks-account-error = No se han podido leer las listas de tareas: { $reason }
tasks-account-none = No se han encontrado listas de tareas
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = No se han encontrado listas de tareas: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } solo muestra las tareas a Katna si ha iniciado sesión con { $provider }.
tasks-account-sign-in-with = Iniciar sesión con { $provider }
tasks-account-looking = Buscando listas de tareas…
tasks-account-try-again = Reintentar
tasks-account-try-again-tooltip = Volver a comprobar ahora las tareas de esta cuenta
tasks-account-fixing = En ello…
tasks-list-name-placeholder = Nombre de la lista

## Lists and tasks

tasks-loading = Leyendo tus tareas…
tasks-no-lists = Tus listas de tareas aparecen aquí.
tasks-search = Buscar tareas
tasks-search-none = Ninguna tarea coincide con tu búsqueda.
tasks-add = Añadir una tarea
tasks-title-placeholder = Título
tasks-add-step = Añadir una subtarea
tasks-empty = Aún no hay tareas. Añade una arriba.
tasks-starred-empty = Destaca una tarea para verla aquí.
tasks-label-empty = No hay tareas abiertas con esta etiqueta.
tasks-today-empty = Nada vence hoy.
tasks-completed-empty = Las tareas que completes aparecen aquí.
tasks-upcoming-add = Añadir una tarea para el { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Desde un correo
tasks-from-note-quiet = Desde una nota
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Vencidas
tasks-completed = { $count ->
    [one] Completadas ({ $count })
    [many] Completadas ({ $count })
   *[other] Completadas ({ $count })
}
tasks-list-options = Opciones de la lista
tasks-sort-by = Ordenar por
tasks-sort-my-order = Mi orden
tasks-sort-date = Fecha
tasks-sort-starred = Destacadas recientemente
tasks-sort-title = Título
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
tasks-from-note = Nota
tasks-open-note = Abrir la nota
tasks-note-gone = Esa nota ya no está aquí.
tasks-no-subject = (sin asunto)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } seleccionada
    [many] { $count } de seleccionadas
   *[other] { $count } seleccionadas
}
tasks-select-clear = Borrar selección
tasks-select-move = Mover a lista
tasks-select-date = Establecer fecha
tasks-next-week = La próxima semana

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
tasks-remind = Recordármelo
tasks-remind-off = No recordar
tasks-remind-on-time = A la hora de la tarea
tasks-remind-morning = El mismo día, { $time }
tasks-remind-hour-before = Una hora antes
tasks-remind-day-before = El día anterior
tasks-label-add = Añadir etiqueta
tasks-label-task = Etiquetar tarea
tasks-files-attach = Adjuntar archivos
tasks-files-pick = Adjuntar
tasks-file-open = Abrir
tasks-file-remove = Quitar archivo
tasks-file-here = Solo en este ordenador
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
tasks-toast-next = Hecho. La siguiente es el { $date }
tasks-toast-deleted = Tarea eliminada
tasks-files-added = { $count ->
    [one] Archivo adjuntado
    [many] { $count } de archivos adjuntados
   *[other] { $count } archivos adjuntados
}
tasks-file-removed = Se ha quitado «{ $name }»
tasks-files-left-out = No adjuntados: { $names }. Una tarea admite archivos de hasta { $limit }, no carpetas.
tasks-file-missing = Ese archivo ya no está aquí.
tasks-toast-added = { $count ->
    [one] Añadida a Tareas
    [many] { $count } tareas añadidas
   *[other] { $count } tareas añadidas
}
tasks-mail-gone = Ese correo ya no está aquí.
tasks-toast-list-deleted = Lista eliminada
tasks-toast-moved = Tarea movida a { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Tarea movida
tasks-toast-rescheduled = Tarea reprogramada
tasks-toast-rescheduled-several = { $count ->
    [one] Tarea reprogramada
    [many] { $count } de tareas reprogramadas
   *[other] { $count } tareas reprogramadas
}
tasks-toast-done-several = { $count ->
    [one] Tarea completada
    [many] { $count } de tareas completadas
   *[other] { $count } tareas completadas
}
tasks-toast-open-several = { $count ->
    [one] Tarea marcada como no completada
    [many] { $count } de tareas marcadas como no completadas
   *[other] { $count } tareas marcadas como no completadas
}
tasks-toast-starred = { $count ->
    [one] Tarea destacada
    [many] { $count } de tareas destacadas
   *[other] { $count } tareas destacadas
}
tasks-toast-unstarred = { $count ->
    [one] Estrella quitada
    [many] Estrellas quitadas de { $count } de tareas
   *[other] Estrellas quitadas de { $count } tareas
}
tasks-toast-deleted-several = { $count ->
    [one] Tarea eliminada
    [many] { $count } de tareas eliminadas
   *[other] { $count } tareas eliminadas
}
