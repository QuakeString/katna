# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = No se ha enviado porque { $reason }.
outbox-retrying = Aún no se ha enviado porque { $reason }. Katna vuelve a intentarlo por sí solo.
outbox-waiting-sign-in = Esperando a que vuelvas a iniciar sesión en { $address }. Se enviará entonces.
outbox-waiting-password = Esperando la nueva contraseña de { $address }. Se enviará entonces.
outbox-waiting-connection = Esperando conexión. Se enviará cuando vuelvas a estar conectado.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = no tiene destinatarios
outbox-reason-address = una dirección a la que se envía no existe
outbox-reason-too-large = es demasiado grande para el servidor de correo
outbox-reason-blocked = el servidor de correo lo ha bloqueado
outbox-reason-gone = su copia en este ordenador ha desaparecido
outbox-reason-refused = el servidor de correo lo ha rechazado

## Buttons and notes

outbox-try-again = Reintentar
outbox-edit = Editar
outbox-delete = Eliminar
outbox-deleted = Eliminado de la bandeja de salida
outbox-sending-again = Enviando de nuevo…

outbox-snackbar-not-sent = «{ $subject }» no se ha enviado porque { $reason }.
outbox-open = Bandeja de salida
