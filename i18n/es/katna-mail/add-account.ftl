# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Añadir una cuenta de correo
add-account-looking = Buscando los servidores de correo de { $address }…
add-account-address-intro = Escribe tu dirección de correo. Katna encuentra los servidores por ti.
add-account-servers-title = Configuración del servidor
add-account-servers-intro = Dónde lee y envía Katna el correo de { $address }.
add-account-password-title = Escribe tu contraseña
add-account-signing-in = Iniciando sesión…

## Add a mail account: fields

add-account-field-address = Dirección de correo
add-account-incoming = Correo entrante ({ $protocol })
add-account-outgoing = Correo saliente ({ $protocol })
add-account-field-server = Servidor
add-account-field-port = Puerto
add-account-security-none = Ninguna
add-account-field-username = Nombre de usuario
add-account-field-password = Contraseña
add-account-show-password = Mostrar contraseña
add-account-app-password-hint = { $provider } necesita aquí una contraseña de aplicación, no la que usas en la web. Crea una en los ajustes de seguridad de tu cuenta de { $provider }.
add-account-field-name = Tu nombre (opcional)
add-account-name-hint = Lo ven las personas a las que escribes.
add-account-servers-pair = { $imap } y { $smtp }
add-account-servers-found = { $source ->
    [built-in] Servidores: { $servers }, encontrados en la lista de proveedores de Katna.
    [provider] Servidores: { $servers }, encontrados en la configuración de tu proveedor.
    [ispdb] Servidores: { $servers }, encontrados en la lista de proveedores de Thunderbird.
    [dns] Servidores: { $servers }, encontrados en los registros DNS de tu dominio.
   *[other] Servidores: { $servers }, deducidos; compruébalos si falla el inicio de sesión.
}
add-account-servers-entered = Servidores: { $servers }, tal como se escribieron.

## Add a mail account: buttons

add-account-servers-button = Configuración del servidor
add-account-back = Atrás
add-account-add = Añadir cuenta
add-account-next = Siguiente
add-account-cancel = Cancelar

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Escribe el servidor entrante.
   *[outgoing] Escribe el servidor saliente.
}
add-account-server-space = { $kind ->
    [incoming] El nombre del servidor entrante tiene un espacio.
   *[outgoing] El nombre del servidor saliente tiene un espacio.
}
add-account-port-invalid = { $kind ->
    [incoming] El puerto entrante debe ser un número del { $min } al { $max }.
   *[outgoing] El puerto saliente debe ser un número del { $min } al { $max }.
}
add-account-address-empty = Escribe una dirección de correo.
add-account-address-invalid = Escribe una dirección de correo como { $example }.
add-account-not-found = Katna no ha encontrado los servidores de { $address }, así que ha rellenado los nombres habituales. Compruébalos con tu proveedor.
add-account-password-empty = Escribe la contraseña.
add-account-name-is-password = El nombre es igual que la contraseña. Escribe ahí tu nombre, tal como deben verlo los demás.
add-account-added = Se ha añadido { $address }. Descargando tu correo…
add-account-app-password-refused = { $provider } ha rechazado la contraseña. Necesita una contraseña de aplicación, no la que usas en la web.
add-account-password-refused = El servidor ha rechazado la contraseña. Compruébala y vuelve a intentarlo.

## The account menu (from the account button on the top bar)

add-account-menu-another = Añadir otra cuenta
add-account-menu-manage = Gestionar cuentas
