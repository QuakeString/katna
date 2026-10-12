# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Cerrar
reader-back = Atrás
reader-mark-unread = Marcar como no leído
reader-move-to = Mover a
reader-snooze = Posponer
reader-remind = Recordármelo
reader-more = Más
reader-original-colors = Mostrar los colores originales
reader-dark-colors = Mostrar en colores oscuros
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
reader-sending = Enviando…
reader-me = mí
reader-to = para { $names }
reader-to-label = para
reader-tick-delivered = Entregado { $when }
reader-tick-no-bounce = Enviado { $when }; no volvió ningún mensaje de error, así que es muy probable que llegara
reader-tick-bounced = No entregado: rebotó { $when }
reader-tick-read = Leído { $when } (confirmación de lectura)
reader-tick-opened = Abierto, la última vez { $when } (seguimiento de aperturas)
reader-starred = Destacado
reader-chip-remove = Quitar { $label }
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
reader-download-failed-reason = No se ha podido descargar este mensaje. { $reason }
reader-download-offline = Esta cuenta está sin conexión. Conéctala para descargar este mensaje.
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
security-look-up-key = Buscar la clave

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Firma verificada
key-card-verified-detail = La firma es válida y confías en esta clave.
key-card-unverified = Firma no verificada
key-card-unverified-detail = La firma es válida, pero nada confirma que la clave sea suya. Compara la huella digital con esa persona y luego marca la clave como de confianza en GnuPG (Kleopatra o gpg --edit-key).
key-card-not-sender = Firmado por otra persona
key-card-not-sender-detail = La firma es válida, pero la clave no es del remitente.
key-card-untrusted = Clave no fiable
key-card-untrusted-detail = Marcaste esta clave como no fiable en GnuPG.
key-card-signature-expired = Firma caducada
key-card-signature-expired-detail = La firma era válida, pero ha caducado.
key-card-key-expired = Clave caducada
key-card-key-expired-detail = La firma es válida, pero la clave ha caducado desde entonces.
key-card-key-revoked = Clave revocada
key-card-key-revoked-detail = Su propietario revocó esta clave, así que no se puede confiar en la firma.
key-card-bad = Firma no válida
key-card-bad-detail = Este mensaje se modificó después de firmarse o la firma es falsa.
key-card-signed-by = Firmado por
key-card-belongs-to = Pertenece a
key-card-fingerprint = Huella digital
key-card-signed = Firmado
key-card-key = Clave
key-card-kind = { $standard }, { $algorithm }
key-card-created = Creada
key-card-expires = Caduca
key-card-never = Nunca
key-card-issued-by = Emitido por
key-card-found-in = Encontrada en
key-card-keyring = Tu anillo de claves de GnuPG
key-card-copy = Copiar huella digital
key-card-import-title = ¿Importar esta clave?
key-card-from-directory = Encontrada en el directorio de claves de { $domain }.
key-card-from-attachment = Del adjunto { $name }.
key-card-import-note = Así Katna podrá comprobar las firmas de esta persona y cifrarle el correo. Para confiar plenamente en la clave, compara la huella digital con esa persona.
key-card-cancel = Cancelar
key-card-import = Importar clave
key-card-looking-up = Buscando la clave…
key-card-looking-up-detail = Consultando el directorio de claves de { $domain }.
key-card-not-found = No se ha encontrado ninguna clave
key-card-not-found-detail = { $domain } no publica ninguna clave para esta dirección. Pide al remitente que te envíe la suya.
key-card-not-kept = La clave encontrada no se puede usar.
key-card-failed = No se ha podido obtener la clave

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Puede que esto no venga de { $domain }
sender-failed-body = No ha superado las comprobaciones de remitente de { $provider }. Ten cuidado con los enlaces, los adjuntos y las respuestas.
sender-provider-unknown = tu proveedor de correo
sender-details = Detalles
sender-details-hide = Ocultar detalles
sender-looks-safe = Parece seguro
sender-move-to-spam = Mover a spam
sender-checked-by = Comprobado por { $provider }
sender-checked-by-server = Comprobado por { $provider } ({ $server })
sender-dmarc = Dominio del remitente (DMARC)
sender-dkim = Firma (DKIM)
sender-spf = Servidor de envío (SPF)
sender-result-pass = Superada
sender-result-fail = No superada
sender-result-unsure = Dudoso
sender-result-none = Ninguno
sender-result-missing = Sin comprobar
sender-dmarc-pass = { $domain } confirma este remitente.
sender-dmarc-fail = El correo no coincide con la forma en que { $domain } dice que envía su correo.
sender-dmarc-none = { $domain } no publica reglas para su correo.
sender-dkim-pass = Firmado por { $domain }.
sender-dkim-fail = La firma de { $domain } no coincide con el correo.
sender-dkim-none = El mensaje no estaba firmado.
sender-spf-pass = Enviado desde un servidor que { $domain } incluye en su lista.
sender-spf-fail = Enviado desde un servidor que { $domain } no incluye en su lista.
sender-spf-none = { $domain } no publica la lista de sus servidores.
sender-check-unsure = La comprobación no ha dado una respuesta clara.
sender-unconfirmed = { $provider } no ha podido confirmar que esto venga de { $domain }. Cualquiera puede poner cualquier remitente.
sender-link-title = ¿Abrir este enlace?
sender-link-body = Este correo no ha superado las comprobaciones de remitente. El enlace lleva a { $host }:
sender-link-cancel = Cancelar
sender-link-open = Abrir

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } lo abrió { $count ->
    [one] una vez
    [many] { $count } de veces
   *[other] { $count } veces
}, la última { $when }
tracking-opens-clicks = { $who } lo abrió { $opens ->
    [one] una vez
    [many] { $opens } de veces
   *[other] { $opens } veces
} y siguió un enlace { $clicks ->
    [one] una vez
    [many] { $clicks } de veces
   *[other] { $clicks } veces
}, la última { $when }
tracking-clicked = { $who } siguió un enlace { $clicks ->
    [one] una vez
    [many] { $clicks } de veces
   *[other] { $clicks } veces
}, la última { $when }
tracking-maybe-opened = Puede que { $who } lo haya abierto (Apple Mail carga las imágenes por privacidad)
tracking-seen-none = Nadie lo ha abierto ni ha seguido un enlace todavía
tracking-receipt = { $who } envió una confirmación de lectura
tracking-receipt-read = { $who } lo ha leído (confirmación de lectura), { $when }
tracking-receipt-displayed = Confirmación de lectura: { $who } abrió tu mensaje
tracking-receipt-other = Confirmación de lectura: { $who } eliminó o gestionó tu mensaje sin abrirlo

## Remote images and pictures

remote-hidden = Las imágenes de este mensaje están ocultas.
remote-hidden-unconfirmed = Imágenes ocultas: no se ha podido confirmar el remitente.
remote-hidden-failed = Imágenes ocultas: este correo no ha superado las comprobaciones de remitente.
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
attachment-forward = Reenviar
attachment-save-all = Guardar todos
attachment-save-all-tooltip = Guardar todos los archivos adjuntos en una carpeta
attachment-save-here = Guardar aquí
attachment-not-downloaded = Este mensaje no está descargado.
attachment-open-message = Abre este mensaje para leer sus archivos adjuntos.
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
print-preview-layout = Diseño
print-preview-as-shown = Como se ve
print-preview-simple = Texto simple
print-preview-backgrounds = Fondos
print-preview-cancel = Cancelar
print-preview-print = Imprimir
print-not-downloaded = (Aún no se ha descargado).
print-encrypted = (Cifrado. Ábrelo en Katna Mail para imprimir su texto).
print-to = Para: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Fijar arriba
text-copy-address = Copiar dirección
text-copy = Copiar
text-select-all = Seleccionar todo
