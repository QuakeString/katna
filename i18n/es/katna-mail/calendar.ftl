# Katna Mail, Spanish (Spain) (Español): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Hoy
calendar-today-tip = Ir a hoy
calendar-view-day = Día
calendar-view-week = Semana
calendar-view-month = Mes
calendar-view-year = Año
calendar-view-schedule = Agenda
calendar-view-days =
    { $count ->
        [one] { $count } día
        [many] { $count } de días
       *[other] { $count } días
    }
calendar-options = Opciones
calendar-density = Densidad
calendar-density-responsive = Adaptable a tu pantalla
calendar-density-comfortable = Cómoda
calendar-density-compact = Compacta
calendar-custom-days = Vista personalizada
calendar-second-zone = Segunda zona horaria
calendar-zone-none = Ninguna
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Compartir horas libres
calendar-free-subject = Horas en las que estoy libre
calendar-free-intro = Estas son algunas horas en las que estoy libre ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = No tengo tiempo libre en los próximos días laborables.
calendar-previous-day = Día anterior
calendar-next-day = Día siguiente
calendar-previous-week = Semana anterior
calendar-next-week = Semana siguiente
calendar-previous-month = Mes anterior
calendar-next-month = Mes siguiente
calendar-previous-year = Año anterior
calendar-next-year = Año siguiente
calendar-previous-period = Antes
calendar-next-period = Después
calendar-title-months = { $first } – { $last }
calendar-loading = Cargando…
calendar-read-failed = No se ha podido leer el calendario: { $error }
calendar-sets = Grupos de calendarios
calendar-set-add = Guardar los calendarios visibles como grupo
calendar-set-name = Nombre del grupo
calendar-set-remove = Quitar grupo
calendar-local = Este ordenador
calendar-account-gone = Cuenta eliminada
calendar-account-sign-in = Vuelve a iniciar sesión para mostrar los calendarios
calendar-account-signed-in = Has vuelto a iniciar sesión en { $address }. Obteniendo tus calendarios…
calendar-account-sign-in-refused = { $provider } no ha dejado entrar a Katna. Vuelve a intentarlo y permite el acceso a tus calendarios.
calendar-account-refused = El servidor no ha aceptado la contraseña. Yahoo, iCloud, Zoho y otros necesitan una contraseña de aplicación.
calendar-account-change-password = Cambiar contraseña
calendar-account-change-password-tooltip = Escribe la nueva contraseña; Katna la comprueba con el servidor
calendar-account-not-enabled = El acceso al calendario para Katna aún no está activado.
calendar-account-failed = No se han podido leer los calendarios.
calendar-account-error = No se han podido leer los calendarios: { $reason }
calendar-account-none = No se han encontrado calendarios
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = No se han encontrado calendarios: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } solo muestra los calendarios a Katna si ha iniciado sesión con { $provider }.
calendar-account-sign-in-with = Iniciar sesión con { $provider }
calendar-account-looking = Buscando calendarios…
calendar-account-try-again = Reintentar
calendar-account-try-again-tooltip = Volver a comprobar ahora los calendarios de esta cuenta
calendar-account-fixing = En ello…
calendar-birthdays = Cumpleaños
calendar-tasks = Tareas
calendar-birthday-of = Cumpleaños de { $name }
calendar-empty-title = Aún no hay calendarios
calendar-empty-text = Katna muestra aquí los calendarios de tus cuentas de Google y Microsoft cuando estén sincronizados, y los de otros servidores compatibles con CalDAV.
calendar-schedule-empty = No hay nada planeado para los próximos dos meses.
calendar-search = Buscar eventos
calendar-search-past = Eventos pasados
calendar-search-none = Ningún evento coincide con tu búsqueda.
calendar-no-title = (Sin título)
calendar-all-day = Todo el día
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } más
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Se repite
calendar-join = Unirse
calendar-join-with = Unirse con { $service }
calendar-email-guests = Enviar correo a los invitados
calendar-running-late = Llegaré tarde
calendar-late-subject = Llegaré tarde: { $title }
calendar-late-body = Disculpen, llegaré unos minutos tarde a { $title }. Estaré ahí pronto.
calendar-guests =
    { $count ->
        [one] { $count } invitado
        [many] { $count } de invitados
       *[other] { $count } invitados
    }
calendar-guest-answers = { $yes } sí, { $maybe } quizás, { $no } no, { $waiting } pendientes
calendar-organizer = Organizador
calendar-optional = Opcional
calendar-open-web = Abrir en el navegador
calendar-open-mail = Abrir el correo
calendar-open-contact = Abrir contacto
calendar-close = Cerrar

## Adding, changing and deleting events.

calendar-add-title = Añadir título
calendar-add-location = Añadir ubicación
calendar-add-notes = Añadir descripción
calendar-add-guests = Añadir invitados
calendar-remove-guest = Quitar
calendar-add-meet = Añadir videollamada de Google Meet
calendar-add-teams = Añadir reunión de Teams
calendar-has-call = Videollamada añadida
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Todo el día
calendar-more-options = Más opciones
calendar-save = Guardar
calendar-saved = Evento guardado
calendar-deleted = Evento eliminado
calendar-discard = Descartar cambios
calendar-edit = Editar evento
calendar-delete = Eliminar evento
calendar-event-details = Detalles del evento
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Nuevo evento
calendar-event-window-title = Nuevo evento
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Abrir día
calendar-menu-duplicate = Duplicar
calendar-menu-color = Color
# The event takes its calendar's color.
calendar-menu-color-calendar = Color del calendario
# A task's new due day, a week from today.
calendar-menu-in-a-week = En una semana
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Tomate
calendar-color-flamingo = Flamenco
calendar-color-tangerine = Mandarina
calendar-color-banana = Plátano
calendar-color-sage = Salvia
calendar-color-basil = Albahaca
calendar-color-peacock = Pavo real
calendar-color-blueberry = Arándano
calendar-color-lavender = Lavanda
calendar-color-grape = Uva
calendar-color-graphite = Grafito
calendar-menu-only-this = Mostrar solo este
calendar-menu-rename = Renombrar
calendar-menu-remove = Quitar de la lista
calendar-menu-delete = Eliminar
calendar-menu-new-calendar = Nuevo calendario
calendar-menu-show-all = Mostrar todos
calendar-menu-hide-all = Ocultar todos
calendar-menu-account-settings = Ajustes de la cuenta
calendar-why-main = Calendario principal
calendar-why-last = Solo hay uno
calendar-why-owner = Solo el propietario
calendar-why-contacts = De Contactos
calendar-why-unreached = Sin conexión
calendar-name-placeholder = Nombre del calendario
calendar-toast-added = «{ $name }» añadido
calendar-toast-renamed = Calendario renombrado
calendar-toast-recolored = Color del calendario cambiado
calendar-toast-deleted = «{ $name }» eliminado
calendar-toast-removed = «{ $name }» quitado de tu lista
calendar-edit-failed = No se ha cambiado el calendario: { $reason }
calendar-delete-title = ¿Eliminar «{ $name }»?
calendar-delete-confirm = Eliminar
calendar-deleting = Eliminando…
calendar-delete-heading = Se elimina:
calendar-delete-events = El calendario y todos sus eventos
calendar-delete-shared = Para todas las personas con quienes se comparte
calendar-delete-server = Se elimina de { $account } en el servicio de correo, no solo en Katna.
calendar-delete-local = Se elimina de este ordenador.
calendar-remove-title = ¿Quitar «{ $name }» de tu lista?
calendar-remove-confirm = Quitar
calendar-removing = Quitando…
calendar-remove-heading = Qué cambia:
calendar-remove-events = Dejas de ver sus eventos, aquí y en tus otras aplicaciones
calendar-remove-server = El calendario sigue siendo de su propietario, que puede volver a compartirlo contigo.
calendar-kind-event = Evento
calendar-kind-task = Tarea
calendar-kind-focus = Tiempo de concentración
calendar-kind-out-of-office = Fuera de la oficina
calendar-kind-working-location = Ubicación de trabajo
calendar-task-added = Tarea añadida
calendar-task-added-to = Tarea añadida a { $list }
calendar-task-list-local = En este ordenador
calendar-working-home = Casa
calendar-busy = Ocupado
calendar-free = Disponible
calendar-cancel = Cancelar
calendar-ok = Aceptar
calendar-read-only = No puedes cambiar los eventos de este calendario
calendar-none-editable = Todavía no hay ningún calendario al que puedas añadir eventos
calendar-no-such-time = Esa hora no existe en tu zona horaria
calendar-end-before-start = El evento termina antes de empezar
calendar-repeat-never = No se repite
calendar-repeat-daily = Todos los días
calendar-repeat-weekly = Semanalmente el { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Mensualmente el primer { $weekday }
        [2] Mensualmente el segundo { $weekday }
        [3] Mensualmente el tercer { $weekday }
        [4] Mensualmente el cuarto { $weekday }
       *[other] Mensualmente el último { $weekday }
    }
calendar-repeat-yearly = Anualmente el { $day }
calendar-repeat-weekdays = Todos los días laborables (de lunes a viernes)
calendar-repeat-custom = Personalizado
calendar-reminder-none = Sin notificación
calendar-reminder-at-start = Al comenzar
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minuto antes
        [many] { $count } de minutos antes
       *[other] { $count } minutos antes
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } hora antes
        [many] { $count } de horas antes
       *[other] { $count } horas antes
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } día antes
        [many] { $count } de días antes
       *[other] { $count } días antes
    }
calendar-scope-edit-title = Editar evento recurrente
calendar-scope-delete-title = Eliminar evento recurrente
calendar-scope-this = Este evento
calendar-scope-following = Este evento y los siguientes
calendar-scope-all = Todos los eventos
calendar-scope-respond-title = Respuesta para un evento recurrente
calendar-going = ¿Asistirás?
calendar-answer-yes = Sí
calendar-answer-no = No
calendar-answer-maybe = Quizás
calendar-answered-yes = Asistirás
calendar-answered-no = No asistirás
calendar-answered-maybe = Quizás asistas

## The card at the top of a mail with an invitation.

calendar-invite = Invitación
calendar-invite-cancelled = Evento cancelado
calendar-invite-reply = { $name } ha respondido
calendar-invite-reply-yes = { $name } ha aceptado
calendar-invite-reply-no = { $name } ha rechazado
calendar-invite-reply-maybe = { $name } quizás asista
calendar-invite-organizer = Organizado por { $name }
calendar-invite-open = Abrir en Calendario
calendar-invite-not-yet = Aún no está en tu calendario. Podrás responder cuando se sincronice.
calendar-invite-by-mail = No está en tu calendario: tu respuesta se envía al organizador por correo.
calendar-mail-yes = Aceptada: { $title }
calendar-mail-yes-body = { $name } ha aceptado esta invitación.
calendar-mail-no = Rechazada: { $title }
calendar-mail-no-body = { $name } ha rechazado esta invitación.
calendar-mail-maybe = Aceptada provisionalmente: { $title }
calendar-mail-maybe-body = { $name } ha aceptado provisionalmente esta invitación.
calendar-invite-your-day = Tu día
calendar-invite-clashes =
    { $count ->
        [one] Coincide con { $count } evento
        [many] Coincide con { $count } de eventos
       *[other] Coincide con { $count } eventos
    }

## The day's agenda beside the mail.

agenda-show = Mostrar la agenda del día
agenda-hide = Ocultar la agenda
agenda-today = Hoy, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = No hay nada planeado para este día.
