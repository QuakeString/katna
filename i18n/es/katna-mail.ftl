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
