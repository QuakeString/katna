# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Panel de carpetas
accounts-folder-pane-detail = De qué cuentas muestra las carpetas el panel de la izquierda.
accounts-shown-one = Una cuenta cada vez; se cambia en la tarjeta de la cuenta
accounts-shown-all = Todas las cuentas, una tras otra
accounts-row = Cuentas
accounts-row-detail = Al quitar una cuenta se elimina la copia de su correo que Katna guarda en este ordenador. El correo sigue en el servidor.
accounts-none = Aún no hay cuentas.
accounts-kind-imported = Importada
accounts-picture-reset = Usar la imagen del escritorio
accounts-picture-change = Cambiar imagen
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
