# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Mensaje nuevo
compose-restore = Restaurar
compose-minimize = Minimizar
compose-exit-full-screen = Salir de pantalla completa
compose-open-window = Abrir en una ventana nueva
compose-save-close = Guardar y cerrar
compose-back-to-mail = Volver a la ventana del correo
compose-pop-out-reply = Abrir la respuesta en otra ventana
compose-show-trimmed = Mostrar contenido recortado

## Recipients and subject

compose-to = Para
compose-cc = Cc
compose-bcc = Cco
compose-from = De
compose-from-choose = Enviar desde otra cuenta
compose-recipients = Destinatarios
compose-subject = Asunto

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Primero envía o descarta el mensaje abierto.
compose-bad-address = «{ $address }» no es una dirección de correo electrónico.
compose-no-recipients = Añade al menos un destinatario.
compose-attachments-too-large = Los archivos adjuntos ocupan { $size }; los servidores de correo admiten hasta { $limit }.
compose-no-account = Añade una cuenta desde la que enviar correo.
compose-past-time = Elige una hora futura.
compose-scheduling = Programando…
compose-sending = Enviando…
compose-scheduled = Envío programado para { $when }
compose-sent-archived = Enviado y archivado
compose-sent = Mensaje enviado
compose-discarded = Borrador descartado
compose-draft-saved = Borrador guardado
compose-draft-failed = No se ha podido guardar el borrador: { $error }
compose-draft-not-opened = No se ha podido abrir el borrador.

## Attachments

compose-picker-insert = Insertar
compose-picker-attach = Adjuntar
compose-file-too-large = { $name } es demasiado grande: un mensaje puede llevar hasta { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Quitar archivo adjunto
compose-attachments-total = { $count ->
    [one] { $count } archivo, { $size }
    [many] { $count } de archivos, { $size }
   *[other] { $count } archivos, { $size }
}
compose-drop-files = Suelta los archivos aquí
compose-drop-here = Suelta aquí
compose-paste-keep-formatting = Mantener formato
compose-paste-table = Tabla
compose-paste-picture = Imagen
compose-paste-plain-text = Texto sin formato
compose-paste-inline = En el texto
compose-paste-attachment = Adjunto

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Cifrar
compose-encrypted = Cifrado: solo los destinatarios pueden leerlo
compose-sign = Firmar
compose-signed = Firmado: los destinatarios pueden comprobar que es tuyo

## Spelling

spell-no-dictionary = No hay ningún diccionario ortográfico instalado para { $language } (por ejemplo, hunspell-en_us).
spell-dictionary-error = Diccionario ortográfico: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = «{ $words }»
grammar-add = Añadir «{ $words }»
grammar-remove = Quitar «{ $words }»
grammar-ignore = Ignorar

## Send checks (asked before a message goes out)

send-check-attachment-title = ¿Querías adjuntar archivos?
send-check-attachment-text = Mencionas un archivo adjunto, pero no hay nada adjunto.
send-check-attach = Adjuntar un archivo
send-check-subject-title = ¿Enviar sin asunto?
send-check-subject-text = Este mensaje no tiene asunto.
send-check-add-subject = Añadir asunto
send-check-send-anyway = Enviar de todos modos
recipient-not-valid = No es una dirección de correo válida
recipient-show-address = Mostrar dirección
recipient-bad-title = Revisa la dirección
recipient-bad-text = «{ $address }» no es una dirección de correo válida. Corrígela o quítala antes de enviar.
recipient-bad-fix = Corregir
