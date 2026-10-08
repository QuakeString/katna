# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > User feedback (crash reports)

feedback-intro-sending = Los informes de fallos nuevos se envían para ayudar a corregir el problema. Nada más sale de este ordenador.
feedback-intro-local = Katna no envía nada a ningún sitio. Los informes de fallos se quedan en este ordenador, para que los consultes o los adjuntes a un informe de error.
feedback-crash-reports = Informes de fallos
feedback-crash-reports-detail = Se crean cuando Katna Mail o su servicio en segundo plano falla.
feedback-save = Guardar informes de fallos en este ordenador
feedback-save-detail = Se omiten tu carpeta personal, los nombres de usuario y de ordenador, y las direcciones de correo
feedback-saved = Informes de fallos guardados
feedback-saved-detail = { $count ->
    [one] Se conserva el más reciente.
    [many] Se conservan los { $count } de más recientes.
   *[other] Se conservan los { $count } más recientes.
}
feedback-help-improve = Ayuda a mejorar Katna
feedback-help-improve-detail = Desactivado salvo que lo actives, y puedes desactivarlo aquí en cualquier momento.
feedback-send = Enviar informes de fallos
feedback-send-detail = El informe guardado, tal como puedes verlo aquí, se envía al sistema de seguimiento de fallos de Katna (Sentry, en la UE). Sin dirección IP, mensajes ni direcciones de correo
feedback-none-saved = No hay informes de fallos guardados.
feedback-delete-all = Eliminar todo
feedback-app-daemon = Servicio en segundo plano
feedback-report-sent = { $date } · Enviado
feedback-view = Ver
feedback-view-tooltip = Abrir el informe
feedback-copy-tooltip = Copiarlo para pegarlo en un informe de error
feedback-copied = Informe de fallo copiado.
feedback-deleted-all = Informes de fallos eliminados.
feedback-read-failed = No se ha podido leer el informe de fallo: { $error }
feedback-delete-failed = No se ha podido eliminar el informe de fallo: { $error }
feedback-delete-all-failed = No se han podido eliminar los informes de fallos: { $error }

## Settings > User feedback (usage statistics)

feedback-usage = Enviar estadísticas de uso anónimas
feedback-usage-detail = Una vez a la semana: qué funciones usaste, sí o no. Nunca cantidades, direcciones, nombres ni palabras de búsqueda
feedback-intro-sending-usage = Se envían informes de fallos y estadísticas de uso semanales. Nada más sale de este ordenador.
feedback-intro-usage-only = Se envían estadísticas de uso semanales. Los informes de fallos se quedan en este ordenador.
feedback-counted = Qué se cuenta
feedback-counted-detail = Cada una es sí o no para la semana.
feedback-counted-also = Además: la versión de Katna, la familia de Linux, el escritorio, la escala de pantalla y cuántas cuentas (1, 2–3, 4+)
feedback-see-report = Ver el informe de esta semana
feedback-hide-report = Ocultar el informe de esta semana
feedback-report-goes = Se envía al terminar la semana, el { $date }, si las estadísticas de uso siguen activadas.
feedback-install-id = ID de instalación { $id }
feedback-install-id-tooltip = Aleatorio, para que un ordenador no se cuente dos veces en una semana. Cambia cada 90 días y nunca se envía con los informes de fallos ni con los comentarios
feedback-install-id-reset = Restablecer
feedback-install-id-new = Se ha creado un nuevo ID de instalación.
feedback-report-copied = Informe copiado.
feedback-send-feedback = Comentarios
feedback-send-feedback-detail = Un problema, una idea, lo que sea.
feedback-send-feedback-button = Enviar comentarios…
usage-feature-search-options = Opciones de búsqueda
usage-feature-pins = Correo fijado
usage-feature-labels = Etiquetas
usage-feature-scheduled-send = Envío programado
usage-feature-snooze = Posponer y recordatorios
usage-feature-encrypted = Correo cifrado
usage-feature-viewers = Visores integrados
usage-feature-calendar = Calendario
usage-feature-contacts = Contactos
usage-feature-tasks-notes = Tareas y Notas
usage-feature-phone-layout = Diseño para ancho de teléfono
usage-feature-own-frame = Marco de ventana propio de Katna

## Help > Send feedback

send-feedback-title = Enviar comentarios
send-feedback-about = Tema
send-feedback-problem = Problema
send-feedback-idea = Idea
send-feedback-other = Otra cosa
send-feedback-message = Tu mensaje
send-feedback-message-placeholder = ¿Qué ha pasado, o qué te gustaría?
send-feedback-reply = Correo para una respuesta (opcional)
send-feedback-reply-placeholder = tu@example.org
send-feedback-system = Incluir la versión de Katna y tu sistema
send-feedback-what-is-sent = Qué se envía
send-feedback-show = Mostrar
send-feedback-hide = Ocultar
send-feedback-where = Se envía al buzón de comentarios de Katna en Sentry (UE). Sin dirección IP, cuentas, mensajes ni ID de instalación.
send-feedback-cancel = Cancelar
send-feedback-send = Enviar
send-feedback-sending = Enviando…
send-feedback-sent = Comentarios enviados. Gracias
send-feedback-failed = No se han podido enviar los comentarios: { $error }
