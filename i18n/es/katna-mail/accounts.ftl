# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Panel de carpetas
accounts-folder-pane-detail = De qué cuentas muestra las carpetas el panel de la izquierda.
accounts-shown-one = Una cuenta cada vez; se cambia en la tarjeta de la cuenta
accounts-shown-all = Todas las cuentas, una tras otra
accounts-unified = Bandeja de entrada unificada
accounts-unified-switch = Mostrar juntos los correos de todas las cuentas
accounts-unified-switch-detail = «Todas las cuentas» encabeza el panel de carpetas, con los recibidos, los enviados y más de cada cuenta en una sola lista. Las cuentas de debajo empiezan plegadas.
accounts-row = Cuentas
accounts-row-detail = El panel de carpetas y el menú de la cuenta muestran las cuentas en este orden; la primera es la predeterminada. Al quitar una cuenta se elimina la copia de su correo que Katna guarda en este ordenador. El correo sigue en el servidor.
accounts-none = Aún no hay cuentas.
accounts-kind-imported = Importada
accounts-picture-reset = Usar la imagen del escritorio
accounts-picture-change = Cambiar imagen
accounts-picture-remove = Quitar imagen
accounts-rename = Cambiar nombre
accounts-name-save = Guardar
accounts-name-cancel = Cancelar
accounts-name-placeholder = Tu nombre
accounts-rename-failed = No se ha podido cambiar el nombre de la cuenta: { $error }
accounts-move-up = Subir
accounts-move-down = Bajar
accounts-drag = Arrastra para cambiar el orden
accounts-remove = Quitar
accounts-delete-all-row = Eliminar todos los datos
accounts-delete-all-row-detail = Empezar de cero, como tras una instalación nueva.
accounts-delete-all-about = Elimina de este ordenador todas las cuentas, todo el correo guardado, los contactos y calendarios, el índice de búsqueda, tus ajustes y las contraseñas guardadas. No cambia nada en tus servidores de correo.
accounts-delete-all-open = Eliminar todos los datos de Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Se ha quitado { $address } de Katna.
accounts-removed = Se ha quitado { $address } de Katna. Su correo sigue en el servidor.
accounts-all-deleted = Se han eliminado todos los datos de Katna de este ordenador.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = ¿Quitar { $address }?
accounts-remove-confirm = Quitar cuenta
accounts-removing = Quitando…
accounts-remove-local-mail = { $folders ->
    [0] Todo el correo importado en esta cuenta
    [one] Todo el correo importado en esta cuenta, en su carpeta
    [many] Todo el correo importado en esta cuenta, en sus { $folders } de carpetas
   *[other] Todo el correo importado en esta cuenta, en sus { $folders } carpetas
}
accounts-remove-local-settings = Sus ajustes de Katna
accounts-remove-mail = { $folders ->
    [0] Todo el correo de esta cuenta guardado por Katna
    [one] Todo el correo de esta cuenta guardado por Katna en su carpeta
    [many] Todo el correo de esta cuenta guardado por Katna en sus { $folders } de carpetas
   *[other] Todo el correo de esta cuenta guardado por Katna en sus { $folders } carpetas
}
accounts-remove-outbox = Sus mensajes pendientes en la bandeja de salida
accounts-remove-settings = Su contraseña guardada y sus ajustes de Katna
accounts-delete-all-title = ¿Eliminar todos los datos de Katna?
accounts-delete-all-confirm = Eliminar todo
accounts-deleting = Eliminando…
accounts-delete-all-accounts = Todas las cuentas, y todo el correo y los adjuntos guardados por Katna
accounts-delete-all-contacts = Los contactos, los calendarios y el índice de búsqueda
accounts-delete-all-settings = Todos los ajustes, firmas y combinaciones de teclas
accounts-delete-all-passwords = Todas las contraseñas guardadas
accounts-deleted-heading = Se elimina de este ordenador:
accounts-cannot-undo = Esta acción no se puede deshacer.
accounts-server-delete-all = No cambia nada en tus servidores de correo: tu correo sigue allí, y si vuelves a añadir una cuenta, se descarga de nuevo. El correo importado desde archivos solo está en Katna; los archivos no se modifican.
accounts-server-local = Este correo se importó desde archivos, así que Katna tiene la única copia. Los archivos de origen no se modifican; vuelve a importarlos para recuperarlo.
accounts-server-remove = No cambia nada en el servidor de correo: tu correo sigue allí, y si vuelves a añadir la cuenta, se descarga de nuevo.
accounts-confirm-word = eliminar
accounts-confirm-placeholder = Escribe «{ accounts-confirm-word }»
accounts-confirm-prompt = Para confirmar, escribe «{ accounts-confirm-word }»:
accounts-cancel = Cancelar
reset-cache-about = Elimina el correo y los adjuntos que descargó Katna, las imágenes de los remitentes y el índice de búsqueda, y luego vuelve a descargar el correo reciente. Se conservan las cuentas, los ajustes y el correo que solo está en este ordenador.
reset-cache-button = Restablecer caché
reset-cache-title = ¿Restablecer la caché?
reset-cache-deleted = Se elimina y se vuelve a descargar:
reset-cache-mail = El correo y los adjuntos descargados de tus servidores IMAP: el correo reciente se vuelve a descargar ahora, y el más antiguo cuando lo abras
reset-cache-index = El índice de búsqueda, que se reconstruye enseguida
reset-cache-pictures = Imágenes de los remitentes
reset-cache-kept = Se conservan: tus cuentas, contraseñas y ajustes; las estrellas, etiquetas, marcas de leído y elementos fijados; los borradores, la bandeja de salida y los cambios que aún no están en el servidor; y el correo de cuentas POP3 o de archivos importados, que puede no tener otra copia. No cambia nada en tus servidores de correo.
reset-cache-confirm = Restablecer caché
reset-cache-busy = Restableciendo…
reset-cache-done = Se ha restablecido la caché. El correo reciente se está volviendo a descargar.
reset-cache-done-freed = Se ha restablecido la caché y se han liberado { $size }. El correo reciente se está volviendo a descargar.
