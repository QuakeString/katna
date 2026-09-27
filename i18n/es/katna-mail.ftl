# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Idioma: { $language }
language-tooltip-system = Idioma: { $language }, según el sistema
language-search = Buscar idioma
language-system-default = Predeterminado del sistema
language-system-now = Ahora: { $language }
language-no-match = Ningún idioma coincide con «{ $query }»
language-machine = Traducción automática. Ayúdanos a mejorarla
language-setting = Idioma
language-setting-detail = El idioma de los menús, botones y mensajes, y el formato de fechas y números. «Predeterminado del sistema» sigue la configuración del escritorio.

## Dates and sizes

ago-just-now = ahora mismo
ago-minutes = { $count ->
    [one] hace { $count } minuto
    [many] hace { $count } de minutos
   *[other] hace { $count } minutos
}
ago-hours = { $count ->
    [one] hace { $count } hora
    [many] hace { $count } de horas
   *[other] hace { $count } horas
}
ago-days = { $count ->
    [one] hace { $count } día
    [many] hace { $count } de días
   *[other] hace { $count } días
}
size-bytes = { $count ->
    [one] { $count } byte
    [many] { $count } de bytes
   *[other] { $count } bytes
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Ocultar carpetas
folders-show = Mostrar carpetas
compose = Redactar
search = Buscar
search-mail = Buscar correo
search-settings = Buscar en ajustes
search-clear = Borrar búsqueda
search-options-show = Mostrar opciones de búsqueda
settings = Ajustes
account-add = Añadir una cuenta

## App rail (and the bottom bar on a phone)

rail-mail = Correo
rail-calendar = Calendario
rail-contacts = Contactos
rail-tasks = Tareas
rail-notes = Notas
rail-feeds = Feeds

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Próximamente
app-calendar-promise = Tus calendarios CalDAV, las invitaciones a reuniones de tu correo y los recordatorios, junto a tu bandeja de entrada.
app-tasks-promise = Listas de tareas que se sincronizan con CalDAV y tareas creadas a partir de correos.
app-notes-promise = Notas rápidas y notas sobre un correo o una conversación para más tarde.
app-feeds-promise = Lee feeds RSS y Atom junto a tu correo.

## Contacts page

app-contacts-loading = Reuniendo a las personas de tu correo…
app-contacts-empty = Aquí aparecen las personas con las que te escribes.
app-contacts-count = { $count ->
    [one] { $count } persona de tu correo, primero con quien más te escribes
    [many] { $count } de personas de tu correo, primero con quienes más te escribes
   *[other] { $count } personas de tu correo, primero con quienes más te escribes
}
app-contacts-top = { $count ->
    [one] La persona de tu correo con quien más te escribes
    [many] Las { $count } de personas de tu correo con quienes más te escribes
   *[other] Las { $count } personas de tu correo con quienes más te escribes
}
app-contacts-messages = { $count ->
    [one] { $count } mensaje
    [many] { $count } de mensajes
   *[other] { $count } mensajes
}
app-contacts-last = último: { $date }

## Navigation (the folders pane)

nav-labels = Etiquetas
nav-folders = Carpetas
nav-label-new = Crear etiqueta
nav-folder-new = Crear carpeta
nav-account-unnamed = Cuenta { $number }
nav-tab-new = { $count ->
    [one] { $count } nuevo
    [many] { $count } de nuevos
   *[other] { $count } nuevos
}

## Special folders (the user's own folders keep their names)

folder-inbox = Recibidos
folder-starred = Destacados
folder-drafts = Borradores
folder-sent = Enviados
folder-archive = Archivo
folder-spam = Spam
folder-trash = Papelera
folder-all-mail = Todos
folder-scheduled = Programados

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nueva etiqueta
label-folder-new-title = Nueva carpeta
label-prompt = Introduce el nombre de la nueva etiqueta:
label-folder-prompt = Introduce el nombre de la nueva carpeta:
label-name-hint = Nombre de la etiqueta
label-folder-name-hint = Nombre de la carpeta
label-nest = Anidar etiqueta en:
label-folder-nest = Anidar carpeta en:
label-cancel = Cancelar
label-create = Crear
label-creating = Creando…
label-created = Se ha creado la etiqueta «{ $name }».
label-folder-created = Se ha creado la carpeta «{ $name }».

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Principal
tab-promotions = Promociones
tab-social = Social
tab-updates = Notificaciones
tab-forums = Foros
tab-focused = Prioritarios
tab-other = Otros
tab-inbox = Bandeja de entrada
tab-newsletters = Boletines
tab-notifications = Notificaciones
tab-new = { $count ->
    [one] { $count } nuevo
    [many] { $count } de nuevos
   *[other] { $count } nuevos
}
tab-provider-other = ordenado por Katna

## Mail list: toolbar

list-select = Seleccionar
list-refresh = Actualizar
list-more = Más
list-mark-read = Marcar como leído
list-mark-unread = Marcar como no leído
list-move-to = Mover a
list-archive = Archivar
list-spam = Marcar como spam
list-delete = Eliminar
list-newer = Más recientes
list-older = Más antiguos
list-range = { $first }–{ $last } de { $total }
list-range-about = { $first }–{ $last } de unos { $total }
list-results = Resultados de «{ $query }»
list-results-corrected = Mostrando resultados de «{ $query }»
list-search-instead = Buscar «{ $query }» en su lugar
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Todos
list-pick-none = Ninguno
list-pick-read = Leídos
list-pick-unread = No leídos
list-pick-starred = Destacados
list-pick-unstarred = No destacados

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Se ha seleccionado { $count } conversación.
        [many] Se han seleccionado las { $count } de conversaciones.
       *[other] Se han seleccionado las { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha seleccionado { $count } mensaje.
        [many] Se han seleccionado los { $count } de mensajes.
       *[other] Se han seleccionado los { $count } mensajes.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Se ha seleccionado { $count } conversación de { $folder }.
        [many] Se han seleccionado las { $count } de conversaciones de { $folder }.
       *[other] Se han seleccionado las { $count } conversaciones de { $folder }.
    }
   *[message] { $count ->
        [one] Se ha seleccionado { $count } mensaje de { $folder }.
        [many] Se han seleccionado los { $count } de mensajes de { $folder }.
       *[other] Se han seleccionado los { $count } mensajes de { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Se ha seleccionado { $count } conversación de esta página.
        [many] Se han seleccionado las { $count } de conversaciones de esta página.
       *[other] Se han seleccionado las { $count } conversaciones de esta página.
    }
   *[message] { $count ->
        [one] Se ha seleccionado { $count } mensaje de esta página.
        [many] Se han seleccionado los { $count } de mensajes de esta página.
       *[other] Se han seleccionado los { $count } mensajes de esta página.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Seleccionar { $count } conversación
        [many] Seleccionar las { $count } de conversaciones
       *[other] Seleccionar las { $count } conversaciones
    }
   *[message] { $count ->
        [one] Seleccionar { $count } mensaje
        [many] Seleccionar los { $count } de mensajes
       *[other] Seleccionar los { $count } mensajes
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Seleccionar { $count } conversación de { $folder }
        [many] Seleccionar las { $count } de conversaciones de { $folder }
       *[other] Seleccionar las { $count } conversaciones de { $folder }
    }
   *[message] { $count ->
        [one] Seleccionar { $count } mensaje de { $folder }
        [many] Seleccionar los { $count } de mensajes de { $folder }
       *[other] Seleccionar los { $count } mensajes de { $folder }
    }
}
list-clear-selection = Borrar selección

## Mail list: empty states

list-empty-search = Ningún mensaje coincide con tu búsqueda.
list-empty-tab = No hay correo en { $tab }.
list-empty-tab-unknown = No hay correo en esta pestaña.
list-empty-folder = No hay mensajes en { $folder }.
list-empty-folder-unknown = No hay mensajes en esta carpeta.
list-first-sync = Obteniendo tu correo…
list-first-sync-detail = Aparecerá aquí a medida que llegue.

## Mail list: lines

row-removed = Este mensaje se ha eliminado.
row-starred = Destacado
row-not-starred = No destacado
row-important = Importante. Haz clic para marcarlo como no importante.
row-mark-important = Marcar como importante
row-pinned = Fijado arriba
row-pin = Fijar arriba
row-unpin = No fijar

## Mail list: More menu and right-click menu

menu-reply = Responder
menu-reply-all = Responder a todos
menu-forward = Reenviar
menu-archive = Archivar
menu-delete = Eliminar
menu-spam = Marcar como spam
menu-mark-read = Marcar como leído
menu-mark-unread = Marcar como no leído
menu-mark-all-read = Marcar todo como leído
menu-star = Añadir estrella
menu-unstar = Quitar estrella
menu-important = Marcar como importante
menu-not-important = Marcar como no importante
menu-pin = Fijar arriba
menu-unpin = No fijar
menu-print-all = Imprimir todo
menu-new-window = Abrir en una ventana nueva
menu-move-to = Mover a
menu-move-to-heading = Mover a:
menu-find-from = Buscar correos de { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Se ha archivado la conversación.
        [many] Se han archivado { $count } de conversaciones.
       *[other] Se han archivado { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha archivado el mensaje.
        [many] Se han archivado { $count } de mensajes.
       *[other] Se han archivado { $count } mensajes.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Se ha movido la conversación a la papelera.
        [many] Se han movido { $count } de conversaciones a la papelera.
       *[other] Se han movido { $count } conversaciones a la papelera.
    }
   *[message] { $count ->
        [one] Se ha movido el mensaje a la papelera.
        [many] Se han movido { $count } de mensajes a la papelera.
       *[other] Se han movido { $count } mensajes a la papelera.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Se ha movido la conversación.
        [many] Se han movido { $count } de conversaciones.
       *[other] Se han movido { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha movido el mensaje.
        [many] Se han movido { $count } de mensajes.
       *[other] Se han movido { $count } mensajes.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Se ha destacado la conversación.
        [many] Se han destacado { $count } de conversaciones.
       *[other] Se han destacado { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha destacado el mensaje.
        [many] Se han destacado { $count } de mensajes.
       *[other] Se han destacado { $count } mensajes.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Se ha quitado la estrella de la conversación.
        [many] Se ha quitado la estrella de { $count } de conversaciones.
       *[other] Se ha quitado la estrella de { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha quitado la estrella del mensaje.
        [many] Se ha quitado la estrella de { $count } de mensajes.
       *[other] Se ha quitado la estrella de { $count } mensajes.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como importante.
        [many] Se han marcado { $count } de conversaciones como importantes.
       *[other] Se han marcado { $count } conversaciones como importantes.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como importante.
        [many] Se han marcado { $count } de mensajes como importantes.
       *[other] Se han marcado { $count } mensajes como importantes.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como no importante.
        [many] Se han marcado { $count } de conversaciones como no importantes.
       *[other] Se han marcado { $count } conversaciones como no importantes.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como no importante.
        [many] Se han marcado { $count } de mensajes como no importantes.
       *[other] Se han marcado { $count } mensajes como no importantes.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Se ha fijado la conversación arriba.
        [many] Se han fijado { $count } de conversaciones arriba.
       *[other] Se han fijado { $count } conversaciones arriba.
    }
   *[message] { $count ->
        [one] Se ha fijado el mensaje arriba.
        [many] Se han fijado { $count } de mensajes arriba.
       *[other] Se han fijado { $count } mensajes arriba.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Se ha dejado de fijar la conversación.
        [many] Se han dejado de fijar { $count } de conversaciones.
       *[other] Se han dejado de fijar { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha dejado de fijar el mensaje.
        [many] Se han dejado de fijar { $count } de mensajes.
       *[other] Se han dejado de fijar { $count } mensajes.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como spam.
        [many] Se han marcado { $count } de conversaciones como spam.
       *[other] Se han marcado { $count } conversaciones como spam.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como spam.
        [many] Se han marcado { $count } de mensajes como spam.
       *[other] Se han marcado { $count } mensajes como spam.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Se ha eliminado la conversación definitivamente.
        [many] Se han eliminado { $count } de conversaciones definitivamente.
       *[other] Se han eliminado { $count } conversaciones definitivamente.
    }
   *[message] { $count ->
        [one] Se ha eliminado el mensaje definitivamente.
        [many] Se han eliminado { $count } de mensajes definitivamente.
       *[other] Se han eliminado { $count } mensajes definitivamente.
    }
}
toast-undone = Se ha deshecho la acción.
toast-undo = Deshacer
toast-no-spam-folder = Esta cuenta no tiene carpeta de spam.

## Reading pane: toolbar

reader-close = Cerrar
reader-back = Atrás
reader-mark-unread = Marcar como no leído
reader-move-to = Mover a
reader-more = Más
reader-print-all = Imprimir todo
reader-new-window = En una ventana nueva
reader-position = { $position } de { $total }
reader-newer = Más reciente
reader-older = Más antigua

## Reading pane: the conversation

reader-removed = Esta conversación se ha eliminado.
reader-no-subject = (sin asunto)
reader-collapse-all = Contraer todo
reader-expand-all = Expandir todo
reader-unknown-sender = (remitente desconocido)
reader-date-ago = { $date } ({ $ago })
reader-me = mí
reader-to = para { $names }
reader-starred = Destacado
reader-not-starred = No destacado
reader-too-long = El mensaje es demasiado largo para mostrarlo completo.
reader-encrypted-images = Las imágenes de la Web nunca se cargan en el correo cifrado.
reader-window-failed = No se ha podido abrir una ventana nueva.

## Reading pane: message details (opened from "to me")

reader-details-from = de:
reader-details-to = para:
reader-details-cc = cc:
reader-details-date = fecha:
reader-details-subject = asunto:

## Reading pane: downloading a message

reader-downloading = Descargando este mensaje del servidor…
reader-download-failed = No se ha podido descargar este mensaje.
reader-try-again = Reintentar

## Reply row

reply-reply = Responder
reply-reply-all = Responder a todos
reply-forward = Reenviar

## Encrypted and signed mail

security-decrypting = Descifrando…
security-checking = Comprobando la firma…
security-partly-encrypted = Solo una parte de este mensaje está cifrada. El resto se añadió fuera de la protección y podría venir de cualquiera.
security-partly-signed = Solo una parte de este mensaje está firmada. El resto se añadió fuera de la protección y podría venir de cualquiera.
security-encrypted = Mensaje cifrado
security-encrypted-smime = Mensaje cifrado (S/MIME)
security-no-key = No se puede descifrar este mensaje: se cifró para una clave que no tienes.
security-cancelled = Se ha cancelado el descifrado.
security-damaged = No se puede descifrar este mensaje: los datos cifrados están dañados o se han modificado.
security-decrypt-unavailable = No se puede descifrar este mensaje: instala { $tool } para leer el correo cifrado.
security-decrypt-failed = No se puede descifrar este mensaje: { $reason }
security-unknown-signer = un firmante desconocido
security-signed-verified = Firmado por { $signer } · verificado
security-signed-not-sender = Firmado por { $signer }, que no es el remitente
security-signed-untrusted = Firmado por { $signer } con una clave que marcaste como no fiable
security-signed-unverified = Firmado por { $signer } · la clave no está verificada
security-bad-signature = Firma no válida: este mensaje se modificó después de firmarse o la firma es falsa.
security-signature-expired = Firmado por { $signer } · la firma ha caducado
security-key-expired = Firmado por { $signer } · la clave ha caducado desde entonces
security-key-revoked = Firmado por { $signer } con una clave que se ha revocado
security-missing-key = Firmado con una clave que no tienes, así que no se puede comprobar
security-missing-key-id = Firmado con una clave que no tienes ({ $key }), así que no se puede comprobar
security-signature-unavailable = Firmado; instala { $tool } para comprobar la firma
security-signature-error = No se ha podido comprobar la firma.

## Remote images and pictures

remote-hidden = Las imágenes de este mensaje están ocultas.
remote-show = Mostrar imágenes
remote-always-show = Mostrar siempre las de este remitente
remote-picture-use = Usar
remote-picture-too-big = Elige una imagen de 8 MB como máximo.
remote-picture-type = Elige una imagen PNG, JPEG, GIF, WebP o SVG.
remote-picture-read-failed = No se puede leer la imagen: { $error }
remote-picture-keep-failed = No se puede guardar la imagen: { $error }
remote-picture-remove-failed = No se puede quitar la imagen: { $error }

## Attachments

attachment-count = { $count ->
    [one] Un archivo adjunto
    [many] { $count } de archivos adjuntos
   *[other] { $count } archivos adjuntos
}
attachment-save = Guardar
attachment-save-all = Guardar todos
attachment-save-all-tooltip = Guardar todos los archivos adjuntos en una carpeta
attachment-save-here = Guardar aquí
attachment-not-downloaded = Este mensaje no está descargado.
attachment-not-found = No se ha encontrado este archivo adjunto en el mensaje.
attachment-read-failed = No se ha podido leer { $name }
attachment-numbered = archivo adjunto { $number }
attachment-saved-all = { $count ->
    [one] Se ha guardado { $count } archivo en { $place }
    [many] Se han guardado { $count } de archivos en { $place }
   *[other] Se han guardado { $count } archivos en { $place }
}
attachment-saved-some = { $total ->
    [one] Se han guardado { $saved } de { $total } archivo en { $place }. No se ha podido guardar { $failed }
    [many] Se han guardado { $saved } de { $total } archivos en { $place }. No se ha podido guardar { $failed }
   *[other] Se han guardado { $saved } de { $total } archivos en { $place }. No se ha podido guardar { $failed }
}
attachment-saved-to = Guardado en { $path }
attachment-save-failed = No se ha podido guardar { $name }: { $error }
attachment-open-failed = No se ha podido abrir { $name }: { $error }
attachment-risky = Este archivo podría ejecutar un programa, así que Katna no lo abre. Guárdalo en su lugar.
attachment-encrypted-open = Este archivo llegó cifrado. Guárdalo para abrirlo en otra aplicación.

## Printing

print-failed = No se ha podido imprimir: { $error }
print-no-font = no se ha encontrado ninguna fuente
print-opened-as-pdf = Se ha abierto como PDF para imprimirlo desde allí.
print-not-downloaded = (Aún no se ha descargado).
print-encrypted = (Cifrado. Ábrelo en Katna Mail para imprimir su texto).
print-to = Para: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Abre este mensaje para leer sus archivos adjuntos.
text-copy = Copiar
text-select-all = Seleccionar todo

## Settings page: its tabs

settings-tab-general = General
settings-tab-inbox = Recibidos
settings-tab-accounts = Cuentas
settings-tab-subscriptions = Suscripciones
settings-tab-appearance = Apariencia
settings-tab-shortcuts = Combinaciones de teclas
settings-tab-default-apps = Aplicaciones predeterminadas
settings-tab-folders-rules = Carpetas y reglas
settings-tab-compose = Redacción
settings-tab-mcp-server = Servidor MCP
settings-tab-feedback = Comentarios de usuarios
settings-tab-experimental = Experimental

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Consulta los boletines y las listas de correo que recibes, y date de baja con un clic.
settings-tab-folders-rules-coming = Crea, renombra, mueve y oculta carpetas y etiquetas, y elige cuáles se sincronizan. Las reglas ordenan, etiquetan, reenvían o eliminan el correo nuevo automáticamente, según el remitente, el asunto o las palabras que contenga.
settings-tab-mcp-server-coming = Permite que los asistentes de IA de este ordenador busquen, lean y redacten borradores de tu correo, con tu permiso.

## Settings > General

settings-general-conversations = Vista de conversación
settings-general-conversations-group = Agrupar las respuestas al mismo correo
settings-general-conversations-group-detail = Una línea por conversación en la lista
settings-general-reading = Lectura
settings-general-newest-first = Mensaje más reciente primero
settings-general-newest-first-detail = Una conversación empieza por su última respuesta
settings-general-full-headers = Mostrar encabezados completos
settings-general-full-headers-detail = De, para, cc, fecha y asunto visibles en cada mensaje
settings-general-full-names = Nombres completos de los destinatarios
settings-general-full-names-detail = «para mí, Ada Lovelace» en lugar de «para mí, Ada»
settings-general-mark-read = Marcar como leído
settings-general-mark-read-now = En cuanto se abre
settings-general-mark-read-1s = Tras 1 segundo abierta
settings-general-mark-read-3s = Tras 3 segundos abierta
settings-general-mark-read-never = Solo cuando la marque como leída
settings-general-reply-button = Botón de respuesta
settings-general-reply-all = Responder a todos
settings-general-reply-all-detail = El botón de respuesta junto a cada mensaje responde a todos, no solo al remitente
settings-general-remote-images = Imágenes de la Web
settings-general-remote-images-detail = Al cargar las imágenes de un mensaje, su remitente sabe que lo has abierto, cuándo y aproximadamente dónde. Si está desactivado, cada mensaje pregunta antes, y siempre puedes mostrar las imágenes de un remitente.
settings-general-remote-images-always = Mostrar siempre las imágenes
settings-general-remote-images-always-detail = En todos los mensajes, no solo de remitentes de confianza
settings-general-sending = Envío
settings-general-sending-detail = Cuánto tiempo espera un mensaje enviado, para poder recuperarlo.
settings-general-offline = Correo sin conexión
settings-general-offline-detail = El correo reciente se descarga completo para leerlo sin conexión. El correo más antiguo se descarga al abrirlo.
settings-general-offline-days = { $count ->
    [one] { $count } día
    [many] { $count } de días
   *[other] { $count } días
}
settings-general-offline-years = { $count ->
    [one] { $count } año
    [many] { $count } de años
   *[other] { $count } años
}
settings-general-offline-all = Todo el correo
settings-general-offline-note = Si eliges menos días, se conserva el correo ya descargado. No cambia nada en el servidor.
settings-general-notifications = Notificaciones
settings-general-notifications-detail = Del correo nuevo en Recibidos, incluso con Katna Mail cerrado.
settings-general-new-mail = Notificarme del correo nuevo
settings-general-new-mail-detail = Con Responder a todos, Marcar como leído y Archivar
settings-general-new-mail-sound = Reproducir un sonido
settings-general-new-mail-sound-detail = El sonido de correo nuevo del escritorio
settings-general-desktop = Escritorio
settings-general-open-at-login = Abrir Katna Mail al iniciar sesión
settings-general-open-at-login-detail = El correo se sincroniza al iniciar sesión de todos modos, mientras el servicio esté en marcha
settings-general-tray = Mostrar Katna en la bandeja del sistema
settings-general-tray-detail = Con el número de no leídos y un menú
settings-general-unread-badge = Número de no leídos en el icono de la barra de tareas
settings-general-unread-badge-detail = Cuántos mensajes de Recibidos están sin leer

## Settings > Inbox

settings-inbox-tabs = Pestañas de Recibidos
settings-inbox-tabs-detail = Ordena Recibidos en pestañas, como hace la web de tu proveedor de correo.
settings-inbox-tabs-show = Mostrar pestañas de Recibidos
settings-inbox-tabs-show-detail = Si está desactivado, una sola lista para todas las cuentas
settings-inbox-no-accounts = Añade una cuenta para elegir sus pestañas.
settings-inbox-tabs-automatic = Automático: { $tabs } ({ $provider })
settings-inbox-tabs-off = Sin pestañas
settings-inbox-tabs-gmail = Principal, Promociones, Social, Notificaciones, Foros
settings-inbox-tabs-focused = Prioritarios y Otros
settings-inbox-tabs-zoho = Bandeja de entrada, Boletines y Notificaciones
settings-inbox-tabs-shown = Pestañas visibles. El correo de una pestaña que desactives se queda en { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Panel de lectura
settings-appearance-reading-pane-detail = Dónde se muestra una conversación abierta.
settings-appearance-pane-right = A la derecha de la lista
settings-appearance-pane-none = Sin división
settings-appearance-density = Densidad
settings-appearance-density-default = Predeterminada
settings-appearance-density-compact = Compacta
settings-appearance-scaling = Escala
settings-appearance-scaling-detail = Hace todo más grande o más pequeño en Katna Mail, además de la escala del propio escritorio: texto, iconos, espaciado y separadores. El correo que envías mantiene su propio tamaño de letra. Con tamaños muy pequeños puede costar hacer clic en los iconos.
settings-appearance-theme = Tema
settings-appearance-theme-system = Igual que el escritorio
settings-appearance-theme-light = Claro
settings-appearance-theme-dark = Oscuro
settings-appearance-desktop-colors = Colores del escritorio
settings-appearance-desktop-colors-use = Usar los colores del escritorio
settings-appearance-desktop-colors-use-detail = El esquema de color y el color de acento del escritorio
settings-appearance-app-names = Nombres de las aplicaciones
settings-appearance-app-names-show = Mostrar los nombres de las aplicaciones
settings-appearance-app-names-show-detail = Nombres bajo los iconos de las aplicaciones, en el extremo izquierdo
settings-appearance-sender-pictures = Imágenes de los remitentes
settings-appearance-sender-pictures-show = Mostrar logotipos de empresas
settings-appearance-sender-pictures-show-detail = Se buscan por el dominio del remitente, nunca por mensaje, y se guardan una semana
settings-appearance-important = Marcadores de importancia
settings-appearance-important-show = Mostrar marcadores de importancia
settings-appearance-important-show-detail = Junto a cada mensaje de la lista
settings-appearance-message-width = Ancho de los mensajes
settings-appearance-message-width-limit = Limitar el ancho de los mensajes
settings-appearance-message-width-limit-detail = Las líneas largas se leen mejor en una ventana ancha
settings-appearance-mail-colors = Colores del correo
settings-appearance-mail-colors-detail = La mayoría del correo está diseñado para una página blanca. Con un tema oscuro, sus colores se cambian por otros oscuros que se leen bien; si está desactivado, mantiene los colores del remitente sobre una página clara.
settings-appearance-dark-mail = Colores oscuros también para el correo
settings-appearance-dark-mail-detail = Solo con el tema oscuro
settings-appearance-attachment-previews = Vistas previas de adjuntos
settings-appearance-attachment-previews-show = Mostrar vistas previas de los archivos adjuntos
settings-appearance-attachment-previews-show-detail = Una pequeña imagen del contenido de cada archivo en su tarjeta

## Settings > Default apps

settings-default-apps-intro = Dónde se abren los archivos adjuntos al hacer clic en ellos. El visor siempre puede abrir un archivo también en otra aplicación. Las aplicaciones predeterminadas del escritorio se configuran en sus propios ajustes.
settings-default-apps-pdf = Archivos PDF
settings-default-apps-pdf-detail = Páginas, con zoom.
settings-default-apps-pictures = Imágenes
settings-default-apps-pictures-detail = Fotos (enderezadas), PNG, GIF, WebP, BMP, TIFF y SVG.
settings-default-apps-text = Archivos de texto
settings-default-apps-text-detail = Texto sin formato, registros, código y otros textos.
settings-default-apps-sheets = Hojas de cálculo
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) y CSV.
settings-default-apps-documents = Documentos
settings-default-apps-documents-detail = Word (docx) y texto de OpenDocument (odt).
settings-default-apps-katna = Visor de Katna Mail
settings-default-apps-system = Aplicación predeterminada del escritorio
settings-default-apps-ask = Preguntar qué aplicación usar cada vez
settings-default-apps-after-saving = Después de guardar
settings-default-apps-show-folder = Mostrar los archivos guardados en su carpeta
settings-default-apps-show-folder-detail = Abre el gestor de archivos con los adjuntos guardados seleccionados

## Settings > Compose

settings-compose-send-from = Enviar mensajes nuevos desde
settings-compose-send-from-detail = Las respuestas y los reenvíos siempre salen de la cuenta en la que estás.
settings-compose-send-from-current = La cuenta en la que estás
settings-compose-send-on-replies = Enviar en respuestas
settings-compose-send-on-replies-detail = Qué hace Enviar en una respuesta o un reenvío. El menú junto a Enviar ofrece la otra opción.
settings-compose-send-plain = Enviar
settings-compose-send-archive = Enviar y archivar
settings-compose-signatures = Firmas
settings-compose-signatures-detail = Se añade debajo de tu mensaje, tras una línea «--». Elige otra en la ventana de redacción.
settings-compose-untitled = Sin título
settings-compose-signature-name = Nombre, como Trabajo
settings-compose-signature-first = Mi firma
settings-compose-signature-numbered = Firma { $number }
settings-compose-signature-delete = Eliminar
settings-compose-signature-deleted = Firma eliminada
settings-compose-signature-new = Crear nueva
settings-compose-no-signatures = Aún no hay firmas.
settings-compose-no-signature = Sin firma
settings-compose-for-new-mail = Para correos nuevos
settings-compose-for-replies = Para respuestas y reenvíos
settings-compose-for-replies-detail = En una conversación en la que firmaste un mensaje, la respuesta empieza con esa firma.
settings-compose-format = Formato
settings-compose-plain-text = Escribir en texto sin formato
settings-compose-plain-text-detail = El correo nuevo empieza sin formato; la ventana de redacción puede cambiarlo
settings-compose-spelling = Ortografía
settings-compose-spell-check = Revisar la ortografía mientras escribo
settings-compose-spell-check-detail = Las palabras mal escritas se subrayan, con sugerencias al hacer clic derecho
settings-compose-spell-desktop = Idioma del escritorio ({ $language })
settings-compose-templates = Plantillas
settings-compose-templates-detail = Guarda los correos que escribes a menudo y úsalos para empezar un correo nuevo o una respuesta.

## Settings > Shortcuts

settings-shortcuts-set = Conjunto de combinaciones
settings-shortcuts-set-detail = Empieza con las teclas de una aplicación de correo que conozcas. Aquí, Cmd es Ctrl. Tus propios cambios se mantienen sobre el conjunto, y Restablecer valores predeterminados vuelve a las teclas del conjunto.
settings-shortcuts-single = Combinaciones de una tecla
settings-shortcuts-single-detail = Teclas sin Ctrl ni Alt, como en el correo web: e archiva, j y k mueven, / busca. Funcionan en la lista y en la conversación abierta, nunca mientras escribes.
settings-shortcuts-single-use = Usar combinaciones de una tecla
settings-shortcuts-single-use-detail = Las combinaciones con Ctrl siempre funcionan
settings-shortcuts-how = Haz clic en una tecla para cambiarla, o en + para añadir una, y pulsa las teclas nuevas. Esc cancela.
settings-shortcuts-restore = Restablecer valores predeterminados
settings-shortcuts-no-key = Sin tecla
settings-shortcuts-press = Pulsa las teclas…
settings-shortcuts-then = { $keys } y luego…
settings-shortcuts-moved = { $keys } ahora hace «{ $action }» en lugar de «{ $previous }».
settings-shortcuts-single-off = Las combinaciones de una tecla están desactivadas, así que esta tecla funcionará cuando se activen.
settings-shortcuts-restored = Todas las combinaciones vuelven a tener las teclas de su conjunto.

## Settings search: the line under a result

settings-general-language-summary = Idioma de la aplicación, las fechas y los números
settings-general-reading-summary = Mensaje más reciente primero, encabezados completos, nombres completos de los destinatarios
settings-general-mark-read-summary = Cuándo se marca como leída una conversación abierta: al momento, tras 1 o 3 segundos, o a mano
settings-general-reply-button-summary = El botón de respuesta junto a cada mensaje responde a todos
settings-general-remote-images-summary = Mostrar siempre las imágenes de todos los mensajes
settings-general-sending-summary = Deshacer el envío: cuánto tiempo espera un mensaje enviado, para poder recuperarlo
settings-general-offline-summary = Cuántos días de correo reciente se descargan completos, para leerlos sin conexión
settings-general-notifications-summary = Notificaciones de correo nuevo y su sonido
settings-general-desktop-summary = Abrir Katna Mail al iniciar sesión, el icono de la bandeja del sistema y el número de no leídos en el icono de la barra de tareas
settings-accounts-accounts-summary = Añadir o quitar una cuenta, o cambiar su imagen
settings-appearance-density-summary = Líneas predeterminadas o compactas en la lista
settings-appearance-scaling-summary = Hacer todo más grande o más pequeño: texto, iconos, espaciado y separadores
settings-appearance-theme-summary = Igual que el escritorio, claro u oscuro
settings-appearance-sender-pictures-summary = Logotipos de empresas, buscados por el dominio del remitente
settings-appearance-important-summary = El marcador de importancia junto a cada mensaje de la lista
settings-appearance-mail-colors-summary = Colores oscuros para el correo HTML con un tema oscuro, o los colores del remitente
settings-appearance-attachment-previews-summary = Una pequeña imagen del contenido de cada archivo adjunto
settings-shortcuts-set-summary = Empezar con las teclas de Gmail, Inbox by Gmail, Apple Mail, Outlook o Thunderbird
settings-shortcuts-single-summary = Teclas sin Ctrl ni Alt, como en el correo web
settings-default-apps-pdf-summary = Dónde se abren los adjuntos PDF
settings-default-apps-pictures-summary = Dónde se abren las fotos y las imágenes
settings-default-apps-text-summary = Dónde se abren el texto sin formato, los registros y el código
settings-default-apps-sheets-summary = Dónde se abren los archivos de Excel, OpenDocument y CSV
settings-default-apps-documents-summary = Dónde se abren los textos de Word y OpenDocument
settings-default-apps-after-saving-summary = Mostrar los adjuntos guardados en su carpeta
settings-compose-send-from-summary = La cuenta desde la que sale el correo nuevo: aquella en la que estás o siempre la misma
settings-compose-send-on-replies-summary = Enviar, o Enviar y archivar la conversación, en respuestas y reenvíos
settings-compose-signatures-summary = Se añade debajo de tu mensaje, tras una línea «--»
settings-compose-for-new-mail-summary = La firma con la que empieza el correo nuevo
settings-compose-for-replies-summary = La firma con la que empiezan las respuestas y los reenvíos
settings-compose-format-summary = Escribir el correo nuevo en texto sin formato
settings-compose-spelling-summary = Revisar la ortografía al escribir, y el idioma del diccionario
settings-compose-templates-summary = Próximamente: guarda los correos que escribes a menudo y úsalos para empezar un correo nuevo o una respuesta
settings-feedback-crash-reports-summary = Guardar informes de fallos en este ordenador cuando Katna Mail o su servicio en segundo plano fallan
settings-feedback-saved-summary = Ver, copiar o eliminar los informes de fallos guardados en este ordenador
settings-feedback-help-improve-summary = Enviar informes de fallos para ayudar a corregir el problema; desactivado salvo que lo actives
settings-experimental-blur-summary = El escritorio se ve desenfocado a través de la barra superior, y los menús parecen de cristal esmerilado
settings-search-shortcut = Combinación de teclas
settings-search-tab = Pestaña de ajustes
settings-search-none = Ningún ajuste coincide con «{ $query }».
settings-search-results = Ajustes que coinciden con «{ $query }»

## Quick settings (the panel that slides in from the right)

quick-title = Ajustes rápidos
quick-see-all = Ver todos los ajustes
quick-reading-pane = Panel de lectura
quick-pane-right = A la derecha de la lista
quick-pane-none = Sin división
quick-density = Densidad
quick-density-default = Predeterminada
quick-density-compact = Compacta
quick-theme = Tema
quick-theme-system = Igual que el escritorio
quick-theme-light = Claro
quick-theme-dark = Oscuro
quick-desktop-colors = Colores del escritorio
quick-desktop-colors-detail = El esquema de color y el color de acento del escritorio
quick-app-names = Nombres de las aplicaciones
quick-app-names-detail = Nombres bajo los iconos de las aplicaciones, en el extremo izquierdo
quick-inbox-tabs = Pestañas de Recibidos
quick-inbox-tabs-detail = Las pestañas del proveedor de correo de cada cuenta
quick-choose-tabs = Elegir pestañas
quick-choose-tabs-detail = Por cuenta, en Ajustes
quick-sending = Envío
quick-undo-send = Deshacer el envío
quick-undo-send-off = Desactivado
quick-undo-send-seconds = { $seconds } s
quick-signatures = Firmas
quick-signatures-none = Aún no hay ninguna
quick-signatures-one = { $name }, predeterminada
quick-signatures-many = { $count ->
    [one] { $count } firma; { $name } predeterminada
    [many] { $count } de firmas; { $name } predeterminada
   *[other] { $count } firmas; { $name } predeterminada
}
quick-signatures-no-default = { $count ->
    [one] { $count }, ninguna predeterminada
    [many] { $count }, ninguna predeterminada
   *[other] { $count }, ninguna predeterminada
}
quick-signature-untitled = Sin título
quick-threading = Agrupación en conversaciones
quick-conversation-view = Vista de conversación
quick-conversation-view-detail = Agrupar las respuestas al mismo correo
quick-help = Ayuda
quick-tour = Hacer el recorrido
quick-whats-new = Novedades
quick-about = Acerca de Katna

## Settings: opening at login

settings-open-at-login-failed = No se ha podido cambiar la apertura al iniciar sesión: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent } %
scale-reset = Volver al { $percent } %

## Settings > Experimental > Look & Feel

look-intro = Funciones que aún están en pruebas. Pueden cambiar o desaparecer.
look-heading = Aspecto visual
look-window-frame = Marco de la ventana
look-window-frame-detail = Quién dibuja la barra de título, los botones de la ventana, las esquinas y la sombra.
look-frame-native-kde = Nativo: el marco de KDE, con tu tema de Plasma
look-frame-native = Nativo: el marco del escritorio
look-frame-katna = Katna: la barra superior se convierte en la barra de título
look-frame-katna-note-named = Katna dibuja esquinas redondeadas y su propia sombra. El marco ya no sigue el tema de { $desktop }; las reglas de ventana se siguen aplicando.
look-frame-katna-note = Katna dibuja esquinas redondeadas y su propia sombra. El marco ya no sigue el tema del escritorio; las reglas de ventana se siguen aplicando.
look-frame-client-side = Tu escritorio deja el marco a cada aplicación, así que Katna ya dibuja el suyo.
look-blurred-background = Fondo desenfocado
look-blurred-background-detail = El escritorio se ve desenfocado a través de la barra superior y las carpetas, y los menús y las ventanas emergentes parecen de cristal esmerilado.
look-blur = Desenfocar lo que hay detrás de la ventana
look-blur-detail = El correo se mantiene sobre tarjetas opacas, para que el texto conserve su contraste
look-blur-off-kde = El efecto de desenfoque de KDE está desactivado. Activa Desenfoque en Preferencias del sistema, Gestión de ventanas, Efectos del escritorio y vuelve a abrir Katna Mail.
look-blur-none-gnome = GNOME no desenfoca lo que hay detrás de las ventanas.
look-blur-none-x11 = Tu gestor de ventanas no desenfoca lo que hay detrás de las ventanas.
look-blur-none-wayland = Tu compositor no desenfoca lo que hay detrás de las ventanas.

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

## Menu bar (the KDE global menu)

desktop-menu-file = _Archivo
desktop-menu-new-message = _Nuevo mensaje
desktop-menu-quit = _Salir
desktop-menu-edit = _Editar
desktop-menu-undo = _Deshacer
desktop-menu-select-all = Seleccionar _todo
desktop-menu-select-none = Dese_leccionar
desktop-menu-find = _Buscar…
desktop-menu-view = _Ver
desktop-menu-folder-list = Mostrar lista de _carpetas
desktop-menu-refresh = _Actualizar
desktop-menu-go = _Ir
desktop-menu-inbox = _Recibidos
desktop-menu-starred = _Destacados
desktop-menu-sent = _Enviados
desktop-menu-drafts = _Borradores
desktop-menu-all-mail = _Todos
desktop-menu-next = Conversación _siguiente
desktop-menu-previous = Conversación _anterior
desktop-menu-message = _Mensaje
desktop-menu-open = _Abrir
desktop-menu-reply = _Responder
desktop-menu-reply-all = Responder a _todos
desktop-menu-forward = Reen_viar
desktop-menu-archive = Arc_hivar
desktop-menu-delete = _Eliminar
desktop-menu-spam = Marcar como s_pam
desktop-menu-move-to = _Mover a…
desktop-menu-mark-read = Marcar como _leído
desktop-menu-mark-unread = Marcar como _no leído
desktop-menu-star = Añadir e_strella
desktop-menu-important = Marcar como _importante
desktop-menu-not-important = Marcar como no imp_ortante
desktop-menu-settings = _Preferencias
desktop-menu-quick-settings = Ajustes _rápidos
desktop-menu-configure = _Configurar Katna Mail…
desktop-menu-help = Ay_uda
desktop-menu-shortcuts = _Combinaciones de teclas
desktop-menu-whats-new = _Novedades
desktop-menu-about = _Acerca de Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Navegación
shortcut-group-actions = Acciones
shortcut-group-go-to = Ir a
shortcut-group-app = Aplicación

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Conversación siguiente
shortcut-previous = Conversación anterior
shortcut-down = Bajar en la lista
shortcut-up = Subir en la lista
shortcut-first = Primero de la lista
shortcut-last = Último de la lista
shortcut-page-down = Avanzar una página en la lista
shortcut-page-up = Retroceder una página en la lista
shortcut-open = Abrir conversación
shortcut-back = Volver a la lista
shortcut-scroll-down = Desplazarse hacia abajo
shortcut-scroll-up = Desplazarse hacia arriba
shortcut-scroll-page-down = Bajar una página
shortcut-scroll-page-up = Subir una página
shortcut-compose = Redactar
shortcut-reply = Responder
shortcut-reply-all = Responder a todos
shortcut-forward = Reenviar
shortcut-archive = Archivar
shortcut-delete = Eliminar
shortcut-spam = Marcar como spam
shortcut-move-to = Mover a
shortcut-mark-read = Marcar como leído
shortcut-mark-unread = Marcar como no leído
shortcut-star = Añadir o quitar estrella
shortcut-important = Marcar como importante
shortcut-not-important = Marcar como no importante
shortcut-check = Marcar la conversación
shortcut-select-all = Marcar todas las conversaciones
shortcut-select-none = Desmarcar todas las conversaciones
shortcut-undo = Deshacer la última acción
shortcut-go-inbox = Recibidos
shortcut-go-starred = Destacados
shortcut-go-sent = Enviados
shortcut-go-drafts = Borradores
shortcut-go-all = Todos
shortcut-search = Buscar correo
shortcut-navigation = Mostrar u ocultar el menú
shortcut-quick-settings = Ajustes rápidos
shortcut-settings = Todos los ajustes
shortcut-shortcuts = Combinaciones de teclas
shortcut-reload = Comprobar si hay correo nuevo
shortcut-quit = Salir

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } y luego { $second }

## Settings > Accounts

accounts-folder-pane = Panel de carpetas
accounts-folder-pane-detail = De qué cuentas muestra las carpetas el panel de la izquierda.
accounts-shown-one = Una cuenta cada vez; se cambia en la tarjeta de la cuenta
accounts-shown-all = Todas las cuentas, una tras otra
accounts-row = Cuentas
accounts-row-detail = Al quitar una cuenta se elimina la copia de su correo que Katna guarda en este ordenador. El correo sigue en el servidor.
accounts-none = Aún no hay cuentas.
accounts-kind-imported = Importada
accounts-picture-reset = Usar la imagen del escritorio
accounts-picture-change = Cambiar imagen
accounts-remove = Quitar
accounts-delete-all-row = Eliminar todos los datos
accounts-delete-all-row-detail = Empezar de cero, como tras una instalación nueva.
accounts-delete-all-about = Elimina de este ordenador todas las cuentas, todo el correo guardado, los contactos y calendarios, el índice de búsqueda, tus ajustes y las contraseñas guardadas. No cambia nada en tus servidores de correo.
accounts-delete-all-open = Eliminar todos los datos de Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Se ha quitado { $address } de Katna.
accounts-removed = Se ha quitado { $address } de Katna. Su correo sigue en el servidor.
accounts-all-deleted = Se han eliminado todos los datos de Katna de este ordenador.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = ¿Quitar { $address }?
accounts-remove-confirm = Quitar cuenta
accounts-removing = Quitando…
accounts-remove-local-mail = { $folders ->
    [0] Todo el correo importado en esta cuenta
    [one] Todo el correo importado en esta cuenta, en su carpeta
    [many] Todo el correo importado en esta cuenta, en sus { $folders } de carpetas
   *[other] Todo el correo importado en esta cuenta, en sus { $folders } carpetas
}
accounts-remove-local-settings = Sus ajustes de Katna
accounts-remove-mail = { $folders ->
    [0] Todo el correo de esta cuenta guardado por Katna
    [one] Todo el correo de esta cuenta guardado por Katna en su carpeta
    [many] Todo el correo de esta cuenta guardado por Katna en sus { $folders } de carpetas
   *[other] Todo el correo de esta cuenta guardado por Katna en sus { $folders } carpetas
}
accounts-remove-outbox = Sus mensajes pendientes en la bandeja de salida
accounts-remove-settings = Su contraseña guardada y sus ajustes de Katna
accounts-delete-all-title = ¿Eliminar todos los datos de Katna?
accounts-delete-all-confirm = Eliminar todo
accounts-deleting = Eliminando…
accounts-delete-all-accounts = Todas las cuentas, y todo el correo y los adjuntos guardados por Katna
accounts-delete-all-contacts = Los contactos, los calendarios y el índice de búsqueda
accounts-delete-all-settings = Todos los ajustes, firmas y combinaciones de teclas
accounts-delete-all-passwords = Todas las contraseñas guardadas
accounts-deleted-heading = Se elimina de este ordenador:
accounts-cannot-undo = Esta acción no se puede deshacer.
accounts-server-delete-all = No cambia nada en tus servidores de correo: tu correo sigue allí, y si vuelves a añadir una cuenta, se descarga de nuevo. El correo importado desde archivos solo está en Katna; los archivos no se modifican.
accounts-server-local = Este correo se importó desde archivos, así que Katna tiene la única copia. Los archivos de origen no se modifican; vuelve a importarlos para recuperarlo.
accounts-server-remove = No cambia nada en el servidor de correo: tu correo sigue allí, y si vuelves a añadir la cuenta, se descarga de nuevo.
accounts-confirm-word = eliminar
accounts-confirm-placeholder = Escribe «{ accounts-confirm-word }»
accounts-confirm-prompt = Para confirmar, escribe «{ accounts-confirm-word }»:
accounts-cancel = Cancelar
