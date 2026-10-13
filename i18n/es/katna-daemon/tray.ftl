# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Abrir _Recibidos
tray-new-message = _Nuevo mensaje
tray-new-task = Nueva _tarea
tray-new-note = Nueva n_ota
tray-preferences = _Preferencias
tray-quit = _Salir

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] No hay correo sin leer
    [one] { $count } mensaje sin leer
    [many] { $count } de mensajes sin leer
   *[other] { $count } mensajes sin leer
}

tray-password-refused = Se necesita una nueva contraseña para { $address }
tray-signed-out = Vuelve a iniciar sesión en { $address }
tray-accounts-need-you = { $count } cuentas te necesitan
tray-not-sent = { $count ->
    [one] { $count } mensaje no se ha enviado
    [many] { $count } de mensajes no se han enviado
   *[other] { $count } mensajes no se han enviado
}
