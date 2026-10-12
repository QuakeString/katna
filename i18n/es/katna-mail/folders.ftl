# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Etiquetas
nav-folders = Carpetas
nav-label-new = Crear etiqueta
nav-folder-new = Crear carpeta
nav-menu-check-mail = Buscar correo nuevo
nav-menu-check-inbox = Comprobar estos Recibidos
nav-unified-leave-out = Excluir de Recibidos unificados
nav-unified-bring-back = Volver a incluir en Recibidos unificados
nav-menu-sign-in-again = Volver a iniciar sesión
nav-menu-new-mail = Nuevo correo desde esta cuenta
nav-menu-account-settings = Ajustes de la cuenta
nav-account-checked = Sincronizada · comprobada { $ago }
nav-account-in-sync = Sincronizada
nav-account-connecting = Conectando…
nav-account-offline = Sin conexión, reintentando
nav-account-signed-out = La sesión de { $provider } ha caducado
nav-account-password-refused = Contraseña rechazada
nav-account-storage = { $used } de { $total } usados
nav-menu-new-subfolder = Nueva carpeta dentro
nav-menu-new-sublabel = Nueva etiqueta dentro
nav-menu-rename = Cambiar nombre
nav-menu-delete = Eliminar
nav-menu-empty-trash = Vaciar la papelera
nav-account-unnamed = Cuenta { $number }
nav-all-accounts = Todas las cuentas
nav-expand = Mostrar carpetas
nav-collapse = Ocultar carpetas
storage-used = { $percent } % de { $total } usado
storage-used-detail = { $address }: { $used } de { $total } usados

## Special folders (the user's own folders keep their names)

folder-inbox = Recibidos
folder-starred = Destacados
folder-snoozed = Pospuestos
folder-unread = No leídos
folder-important = Importantes
folder-drafts = Borradores
folder-sent = Enviados
folder-archive = Archivo
folder-spam = Spam
folder-trash = Papelera
folder-all-mail = Todos
folder-scheduled = Programados
folder-waiting = Esperando respuesta
folder-waiting-short = En espera
folder-reminders = Recordatorios
folder-outbox = Bandeja de salida
folder-activity = Actividad
folder-not-on-account = Esta cuenta no tiene esa carpeta.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nueva etiqueta
label-folder-new-title = Nueva carpeta
label-prompt = Introduce el nombre de la nueva etiqueta:
label-folder-prompt = Introduce el nombre de la nueva carpeta:
label-name-hint = Nombre de la etiqueta
label-folder-name-hint = Nombre de la carpeta
label-nest = Anidar etiqueta en:
label-folder-nest = Anidar carpeta en:
label-cancel = Cancelar
label-create = Crear
label-creating = Creando…
label-created = Se ha creado la etiqueta «{ $name }».
label-folder-created = Se ha creado la carpeta «{ $name }».
label-rename-title = Cambiar nombre de la etiqueta
label-folder-rename-title = Cambiar nombre de la carpeta
label-rename = Cambiar nombre
label-renaming = Cambiando nombre…
label-renamed = Etiqueta renombrada como «{ $name }».
label-folder-renamed = Carpeta renombrada como «{ $name }».

## Deleting a folder or label (asked first)

folder-delete-title = ¿Eliminar «{ $name }»?
folder-delete-body = { $count ->
    [0] No contiene correo. La carpeta se elimina del servidor, así que también desaparece del correo web y de tu teléfono.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Su { $count } conversación va a la papelera, así que aún puedes recuperarla.
            [many] Sus { $count } de conversaciones van a la papelera, así que aún puedes recuperarlas.
           *[other] Sus { $count } conversaciones van a la papelera, así que aún puedes recuperarlas.
        }
       *[message] { $count ->
            [one] Su { $count } mensaje va a la papelera, así que aún puedes recuperarlo.
            [many] Sus { $count } de mensajes van a la papelera, así que aún puedes recuperarlos.
           *[other] Sus { $count } mensajes van a la papelera, así que aún puedes recuperarlos.
        }
    } La carpeta se elimina del servidor, así que también desaparece del correo web y de tu teléfono.
}
folder-delete-forever-body = { $count ->
    [0] No contiene correo. La carpeta se elimina del servidor, así que también desaparece del correo web y de tu teléfono.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Su { $count } conversación se elimina definitivamente; esta cuenta no tiene papelera.
            [many] Sus { $count } de conversaciones se eliminan definitivamente; esta cuenta no tiene papelera.
           *[other] Sus { $count } conversaciones se eliminan definitivamente; esta cuenta no tiene papelera.
        }
       *[message] { $count ->
            [one] Su { $count } mensaje se elimina definitivamente; esta cuenta no tiene papelera.
            [many] Sus { $count } de mensajes se eliminan definitivamente; esta cuenta no tiene papelera.
           *[other] Sus { $count } mensajes se eliminan definitivamente; esta cuenta no tiene papelera.
        }
    } La carpeta se elimina del servidor, así que también desaparece del correo web y de tu teléfono.
}
folder-delete-label-body = Se quita la etiqueta. Su correo se queda en Todos y en sus otras etiquetas.
folder-delete-confirm = Eliminar carpeta
folder-delete-label-confirm = Eliminar etiqueta
folder-deleted = Se ha eliminado la carpeta «{ $name }»
label-deleted = Se ha eliminado la etiqueta «{ $name }»
