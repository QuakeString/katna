# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
tracking-opened = { $who } lo abrió { $count ->
    [one] una vez
    [many] { $count } de veces
   *[other] { $count } veces
}, la última { $when }
tracking-opened-clicked = { $who } lo abrió y siguió un enlace { $count ->
    [one] una vez
    [many] { $count } de veces
   *[other] { $count } veces
}, la última { $when }
tracking-maybe-opened = Puede que { $who } lo haya abierto (Apple Mail carga las imágenes por privacidad)
tracking-not-opened = { $who } aún no lo ha abierto
tracking-receipt = { $who } envió una confirmación de lectura
tracking-receipt-displayed = Confirmación de lectura: { $who } abrió tu mensaje
tracking-receipt-other = Confirmación de lectura: { $who } eliminó o gestionó tu mensaje sin abrirlo

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
print-preview-title = Vista previa de impresión
print-preview-laying-out = Maquetando las páginas…
print-preview-pages = { $count ->
    [one] { $count } página
    [many] { $count } de páginas
   *[other] { $count } páginas
}
print-preview-more = { $count ->
    [one] y { $count } página más
    [many] y { $count } de páginas más
   *[other] y { $count } páginas más
}
print-preview-failed = no se han podido mostrar las páginas
print-preview-paper = Papel
print-preview-a4 = A4
print-preview-letter = Carta
print-preview-cancel = Cancelar
print-preview-print = Imprimir
print-not-downloaded = (Aún no se ha descargado).
print-encrypted = (Cifrado. Ábrelo en Katna Mail para imprimir su texto).
print-to = Para: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Abre este mensaje para leer sus archivos adjuntos.
text-copy = Copiar
text-select-all = Seleccionar todo
