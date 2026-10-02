# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Añadir una cuenta de correo
add-account-providers-intro = Elige tu proveedor de correo. Katna se encarga del resto.
add-account-provider-other = Otro correo
add-account-provider-other-detail = Cualquier cuenta IMAP o POP3
add-account-provider-google-detail = Gmail y Google Workspace
add-account-provider-microsoft-detail = Outlook y Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Iniciar sesión en { $provider }
add-account-form-title-other = Tu cuenta de correo
add-account-form-intro = Katna guarda tu contraseña en el llavero del sistema.
add-account-looking = Buscando los servidores de correo de { $address }…
add-account-address-intro = Escribe tu dirección de correo. Katna encuentra los servidores por ti.
add-account-servers-title = Configuración del servidor
add-account-servers-intro = Dónde lee y envía Katna el correo de { $address }.
add-account-signing-in = Iniciando sesión…
add-account-browser-title = Continúa en tu navegador
add-account-browser-intro = Katna ha abierto la página de inicio de sesión de { $provider } en tu navegador. Inicia sesión allí y permite que Katna lea y envíe tu correo; después, vuelve aquí.
add-account-browser-hint = ¿No se ha abierto ninguna página? Revisa las ventanas de tu navegador, o vuelve atrás y inténtalo de nuevo.
add-account-stage-browser = Esperando a que inicies sesión en el navegador…
add-account-stage-signing-in-at = Iniciando sesión en { $server }…
add-account-help-app-password-link = Cómo crear una contraseña de aplicación
add-account-help-turn-on-imap = { $provider } solo deja entrar a las aplicaciones de correo una vez activado el acceso IMAP y POP3 en los ajustes de su correo web.
add-account-help-turn-on-imap-link = Cómo activarlo

## Add a mail account: fields

add-account-field-address = Dirección de correo
add-account-receive-with = Recibir correo con
add-account-imap-about = IMAP mantiene tu correo y tus carpetas en el servidor, igual en todos tus dispositivos. Elígelo siempre que puedas.
add-account-pop3-about = POP3 descarga tu correo a este ordenador. El correo que leas o muevas aquí se queda como estaba en el servidor y en tus otros dispositivos.
add-account-incoming = Correo entrante ({ $protocol })
add-account-outgoing = Correo saliente ({ $protocol })
add-account-field-server = Servidor
add-account-field-port = Puerto
add-account-security-none = Ninguna
add-account-security-none-warning = Sin cifrar: tu contraseña y tu correo pueden leerse por el camino.
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

add-account-sign-in-with = Iniciar sesión con { $provider }
add-account-sign-in-instead = Iniciar sesión con { $provider } en su lugar

add-account-servers-button = Configuración del servidor
add-account-back = Atrás
add-account-add = Añadir cuenta
add-account-done = Listo
add-account-another = Añadir otra cuenta
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
add-account-app-password-refused = { $provider } ha rechazado la contraseña. Necesita una contraseña de aplicación, no la que usas en la web.
add-account-password-refused = El servidor ha rechazado la contraseña. Compruébala y vuelve a intentarlo.
add-account-sign-in-refused = { $provider } no ha dejado entrar a Katna. Vuelve a intentarlo y permite el acceso a tu correo.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Esta copia de Katna aún no puede iniciar sesión en cuentas de Microsoft.
    [Google] Esta copia de Katna aún no puede iniciar sesión en cuentas de Google.
   *[other] Este proveedor solo permite iniciar sesión en su propia página, algo que Katna aún no puede hacer con él.
}
add-account-smtp-not-found = Katna ha encontrado dónde leer tu correo, pero no dónde enviarlo. Escribe el servidor de salida.

## Add a mail account: the last step

add-account-done-title = Tu cuenta está lista
add-account-done-intro = Katna está obteniendo tu correo. El correo nuevo aparece a medida que llega.
add-account-done-sign-in = Inicio de sesión
add-account-done-signed-in-with = Con { $provider }, en tu navegador
add-account-done-receiving = Recepción de correo
add-account-done-sending = Envío de correo
add-account-done-on-server = Correo en el servidor
add-account-done-kept = Se conserva hasta que lo eliminas en Katna
add-account-done-pop3-hint = Cambia qué pasa con el correo del servidor en Ajustes > Cuentas.
add-account-done-zoho-title = Tareas y calendarios
add-account-done-zoho-about = Zoho los guarda aparte del correo. Inicia sesión con Zoho una vez para traerlos a Katna.
add-account-done-linked = Tareas y calendarios conectados

## The account menu (from the account button on the top bar)

add-account-menu-another = Añadir otra cuenta
app-menu = Menú principal
app-menu-back = Atrás
