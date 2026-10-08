# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = El servidor de correo

problems-signed-out = { $provider } ha cerrado la sesión de Katna en { $address }. El correo ha dejado de sincronizarse.
problems-password-refused = { $provider } ha rechazado la contraseña de { $address }. Puede que haya cambiado.
problems-no-answer = { $provider } no responde para { $address }. Katna sigue intentándolo.
problems-offline = Estás sin conexión. Tu correo sigue aquí, y el que envíes espera hasta que vuelvas a estar conectado.
problems-accounts-need-you = { $count ->
    [one] 1 cuenta te necesita
    [many] { $count } de cuentas te necesitan
   *[other] { $count } cuentas te necesitan
}
problems-show = Mostrar
problems-later = Más tarde
problems-new-password = Nueva contraseña
problems-try-again = Reintentar

## The New password card

problems-password-title = Nueva contraseña
problems-password-detail = { $provider } ha rechazado la contraseña guardada de { $address }. Escribe la nueva; Katna la comprueba antes de guardarla.
problems-password-placeholder = Contraseña
problems-password-show = Mostrar contraseña
problems-password-hide = Ocultar contraseña
problems-password-cancel = Cancelar
problems-password-save = Guardar
problems-password-checking = Comprobando…
problems-password-refused-again = { $provider } también ha rechazado esta contraseña. Revísala y vuelve a intentarlo.
problems-password-saved = Contraseña guardada para { $address }. Obteniendo tu correo…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = El servidor de correo de { $address } no ha aceptado mover { $count ->
    [one] un mensaje, así que ha vuelto a donde estaba.
    [many] { $count } de mensajes, así que han vuelto a donde estaban.
   *[other] { $count } mensajes, así que han vuelto a donde estaban.
}
problems-refused-flags = El servidor de correo de { $address } no ha aceptado marcar { $count ->
    [one] un mensaje (leído, destacado…), así que ha vuelto a como estaba.
    [many] { $count } de mensajes (leídos, destacados…), así que han vuelto a como estaban.
   *[other] { $count } mensajes (leídos, destacados…), así que han vuelto a como estaban.
}
problems-refused-label = El servidor de correo de { $address } no ha aceptado cambiar las etiquetas de { $count ->
    [one] un mensaje, así que ha vuelto a como estaba.
    [many] { $count } de mensajes, así que han vuelto a como estaban.
   *[other] { $count } mensajes, así que han vuelto a como estaban.
}
problems-refused-delete = El servidor de correo de { $address } no ha aceptado eliminar { $count ->
    [one] un mensaje, así que ha vuelto.
    [many] { $count } de mensajes, así que han vuelto.
   *[other] { $count } mensajes, así que han vuelto.
}
problems-refused-other = El servidor de correo de { $address } no ha aceptado { $count ->
    [one] un cambio, así que Katna lo ha dejado como estaba.
    [many] { $count } de cambios, así que Katna los ha dejado como estaban.
   *[other] { $count } cambios, así que Katna los ha dejado como estaban.
}
problems-details = Detalles

## Katna's background service (katna-daemon) isn't running

service-starting = Iniciando el servicio en segundo plano de Katna…
service-failed = El servicio en segundo plano de Katna no se inicia, así que el correo no se sincroniza.
service-start-again = Volver a iniciar
service-started-again = El servicio en segundo plano de Katna se había detenido y se ha vuelto a iniciar.
service-details-title = Por qué no se inicia el servicio
service-details-body = Copia esto y envíalo con tu informe. No contiene correo ni contraseñas.
service-details-copy = Copiar
service-details-close = Cerrar
service-not-running = El servicio en segundo plano de Katna no se está ejecutando.
service-no-answer = El servicio en segundo plano de Katna no ha respondido: { $error }
service-no-session = No hay sesión de D-Bus: { $error }
