# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Lectura
chat-view = Conversaciones como chats
chat-view-detail = El correo entre personas se lee como un chat de grupo: una burbuja por correo con solo lo que se escribió, y los tuyos a la derecha. Los boletines mantienen la vista habitual.
chat-view-switch = Mostrar las conversaciones como chats
chat-view-switch-detail = El correo citado y las firmas esperan tras ··· en cada burbuja

chat-switch-chat = Chat
chat-switch-mail = Correo
chat-people = { $names } y tú · { $count ->
    [one] { $count } correo
    [many] { $count } de correos
   *[other] { $count } correos
}
chat-people-heading = { $count ->
    [one] En este chat · { $count } persona
    [many] En este chat · { $count } de personas
   *[other] En este chat · { $count } personas
}
chat-member-mails = { $count ->
    [0] Ningún correo
    [one] { $count } correo
    [many] { $count } de correos
   *[other] { $count } correos
}
chat-today = Hoy
chat-yesterday = Ayer
chat-added = { $who } ha añadido a { $names }
chat-renamed = { $who } ha cambiado el asunto a «{ $subject }»
chat-you = Tú
chat-not-downloaded = Aún no descargado
chat-forwarded = Reenviado
chat-show-quoted = Mostrar el correo citado y la firma
chat-hide-quoted = Ocultar el correo citado y la firma
chat-hide-dots = Ocultar ···
chat-show-card = Mostrar su ficha
chat-reply-all = Responder a todos
chat-more = Más
chat-reply-only = Responder solo a { $name }
chat-forward = Reenviar
chat-copy-text = Copiar texto
chat-show-as-mail = Mostrar como correo
chat-go-down = Ir al correo más reciente
chat-pin = Fijar arriba
chat-pin-file = Fijar archivo arriba
chat-unpin = Dejar de fijar
chat-unpin-file = Dejar de fijar el archivo
chat-pinned-of = Fijado { $at } de { $count }
chat-pins-all = Todos los fijados
chat-pins-heading = Fijados · { $count } de { $most }
chat-pins-drag = Arrastra para reordenar
chat-pin-from-mail = Correo de { $name } · { $when }
chat-pin-from-file = Archivo de { $name } · { $when }
chat-pin-from-text = Texto de { $name } · { $when }
chat-pins-full = Este chat ya tiene 5 elementos fijados
chat-pins-replace-title = Reemplazar un fijado
chat-pins-replace-hint = Un chat admite hasta 5 elementos fijados. Elige cuál quitar.
chat-pins-replace = Reemplazar
chat-pins-cancel = Cancelar
chat-undo = Deshacer

chat-reply-to = Responder a { $names }
chat-send = Enviar (Ctrl+Enter). Clic derecho o mantén pulsado para más opciones
chat-send-now = Enviar ahora
chat-attach = Adjuntar
chat-attach-photo = Foto
chat-attach-file = Archivo
chat-attach-library = Desde Archivos
chat-attach-template = Plantilla
chat-attach-signature = Firma
chat-replying-to = Respondiendo a { $name }
chat-reply-newest = Responder al correo más reciente

## The attach picker (paperclip > From Files)

picker-title = Adjuntar desde Archivos
picker-search = Buscar nombres, personas, asuntos
picker-search-drive = Buscar en esta unidad
picker-mail-files = Archivos del correo
picker-this-chat = Esta conversación
picker-this-computer = Este ordenador…
picker-in-chat = EN ESTA CONVERSACIÓN
picker-recent = RECIENTES
picker-preview = Vista previa
picker-cancel = Cancelar
picker-attach = Adjuntar
picker-attach-count = Adjuntar { $count }
picker-selected = { $count } seleccionados
picker-of-limit = de { $limit }
picker-in-mail = { $size } en el correo
picker-drive-links = { $count ->
    [one] 1 como enlace de Google Drive
    [many] { $count } de archivos como enlaces de Google Drive
   *[other] { $count } como enlaces de Google Drive
}
picker-onedrive-links = { $count ->
    [one] 1 como enlace de OneDrive
    [many] { $count } de archivos como enlaces de OneDrive
   *[other] { $count } como enlaces de OneDrive
}
picker-over = { $size }, más de los { $limit } que admite un correo
picker-getting = { $count ->
    [one] Obteniendo el archivo de la unidad…
    [many] Obteniendo { $count } de archivos de la unidad…
   *[other] Obteniendo { $count } archivos de la unidad…
}
picker-some-failed = { $count ->
    [one] No se ha podido leer un archivo
    [many] No se han podido leer { $count } de archivos
   *[other] No se han podido leer { $count } archivos
}
