# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Buscar archivos

## Left side (and chips on a phone)

files-all = Todos los archivos
files-pictures = Imágenes
files-pdfs = PDF
files-documents = Documentos
files-sheets = Hojas de cálculo
files-slides = Presentaciones
files-other = Otros
files-accounts = Cuentas
files-drives = Unidades
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Compartido conmigo
files-shown = Mostrados
files-received = Recibidos
files-sent = Enviados por mí

## Over the files

files-count = { $count ->
    [one] { $count } archivo · { $size }
    [many] { $count } de archivos · { $size }
   *[other] { $count } archivos · { $size }
}
files-anyone = Cualquiera
files-from-person = De { $name }
files-time-any = Cualquier fecha
files-time-today = Hoy
files-time-yesterday = Ayer
files-time-this-week = Esta semana
files-time-last-week = La semana pasada
files-time-this-month = Este mes
files-time-last-month = El mes pasado
files-time-between = { $first } – { $last }
files-time-hint = Haz clic en un día o arrastra sobre varios días
files-time-summary = { $count ->
    [one] { $days } · { $count } archivo
    [many] { $days } · { $count } de archivos
   *[other] { $days } · { $count } archivos
}
files-time-clear = Borrar
files-time-month-back = Mes anterior
files-time-month-on = Mes siguiente
files-time-wheel = Desplázate para mover estas fechas manteniendo su duración
files-sort-newest = Más recientes primero
files-sort-oldest = Más antiguos primero
files-sort-largest = Más grandes primero
files-sort-name = Por nombre
files-grid = Tarjetas
files-list = Lista
files-this-week = Esta semana
files-undated = Sin fecha
files-me = Yo
files-no-subject = (sin asunto)
files-loading = Reuniendo los archivos de tu correo…
files-empty = Los archivos de tu correo aparecen aquí.
files-none-match = Ningún archivo coincide.
files-load-failed = No se han podido leer los archivos: { $error }

## A file's menu and buttons

files-open = Abrir
files-open-with = Abrir con…
files-save = Guardar…
files-show-mail = Mostrar el correo
files-mail-window = Abrir el correo en una ventana nueva
files-forward = Reenviar el archivo
files-from-them = Archivos de { $name }
files-copy-name = Copiar el nombre del archivo
files-name-copied = Nombre del archivo copiado
files-downloading = Descargando el correo…
files-download-failed = No se ha podido descargar este correo.

## A cloud drive in place of the mail files

files-drive-mine = Mi unidad
files-drive-mine-onedrive = Mis archivos
files-drive-results = «{ $words }»
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 archivo
        [many] { $files } de archivos
       *[other] { $files } archivos
    }
    [one] 1 carpeta · { $files ->
        [one] 1 archivo
        [many] { $files } de archivos
       *[other] { $files } archivos
    }
    [many] { $folders } de carpetas · { $files ->
        [one] 1 archivo
        [many] { $files } de archivos
       *[other] { $files } archivos
    }
   *[other] { $folders } carpetas · { $files ->
        [one] 1 archivo
        [many] { $files } de archivos
       *[other] { $files } archivos
    }
}
files-drive-folders = Carpetas
files-drive-files = Archivos
files-drive-folder = Carpeta
files-drive-meta = { $what } · Editado { $date }
files-drive-as-link = { $what } · como enlace
files-drive-google-doc = Documento de Google
files-drive-google-sheet = Hoja de cálculo de Google
files-drive-google-slides = Presentación de Google
files-drive-google-drawing = Dibujo de Google
files-drive-fetching = Obteniéndolo…
files-drive-loading = Abriendo la unidad…
files-drive-empty = Esta carpeta está vacía.
files-drive-unreachable = No se puede conectar con { $drive }.
files-drive-try-again = Reintentar
files-drive-needs-permission = Katna necesita tu permiso una vez para mostrar esta unidad. Vuelve a iniciar sesión y permite que Katna vea tus archivos.
files-drive-allow = Permitir
files-drive-allow-failed = El inicio de sesión no ha terminado, así que la unidad sigue cerrada.
files-drive-attach = Adjuntar
files-drive-more = Más
files-drive-download = Descargar…
files-drive-open-web = Abrir en { $drive }
files-drive-copy-link = Copiar enlace
files-drive-link-copied = Enlace copiado
files-drive-share = Compartir…
files-drive-rename = Renombrar
files-drive-trash = Mover a la papelera
files-drive-trashed = «{ $name }» está en la papelera de { $drive }
files-drive-renamed = Renombrado como «{ $name }»
files-drive-getting = Obteniendo { $name } de { $drive }…
files-drive-get-failed = No se ha podido obtener { $name }: { $error }
files-drive-upload = Subir
files-drive-upload-files = Subir archivos
files-drive-upload-folder = Subir carpeta
files-drive-upload-failed = No se ha podido subir { $name }: { $error }
files-drive-upload-needs = Para subir archivos, Katna necesita tu permiso una vez: pulsa Permitir en Ajustes › Aplicaciones predeterminadas › Página Archivos.

## The Share dialog of a drive file or folder

files-share-title = Compartir «{ $name }»
files-share-add = Añade personas por nombre o dirección
files-share-not-address = «{ $text }» no es una dirección de correo electrónico
files-share-notify = Que { $drive } les envíe también un correo
files-share-people = Personas con acceso
files-share-general = Acceso general
files-share-loading = Comprobando quién tiene acceso…
files-share-restricted = Restringido
files-share-restricted-about = Solo las personas con acceso pueden abrirlo con el enlace
files-share-anyone = Cualquiera con el enlace
files-share-anyone-can = { $role ->
    [editor] Cualquiera con el enlace puede editar
    [commenter] Cualquiera con el enlace puede comentar
   *[viewer] Cualquiera con el enlace puede ver
}
files-share-anyone-about = { $role ->
    [editor] Cualquiera en Internet que tenga el enlace puede editar
    [commenter] Cualquiera en Internet que tenga el enlace puede comentar
   *[viewer] Cualquiera en Internet que tenga el enlace puede ver
}
files-share-role-owner = Propietario
files-share-role-editor = Editor
files-share-role-commenter = Comentador
files-share-role-viewer = Lector
files-share-you = { $name } (tú)
files-share-domain = Todos en { $domain }
files-share-inherited = Acceso desde una carpeta que lo contiene
files-share-remove = Quitar acceso
files-share-copy-link = Copiar enlace
files-share-share = Compartir
files-share-done = Listo
files-share-close = Cerrar
files-share-sharing = Compartiendo…
files-share-shared = { $count ->
    [one] Compartido con 1 persona
    [many] Compartido con { $count } de personas
   *[other] Compartido con { $count } personas
}
files-share-refused = { $drive } no ha podido compartir con { $addresses }
files-share-failed = No se ha podido cambiar el uso compartido: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Subiendo 1 elemento
    [many] Subiendo { $count } de elementos
   *[other] Subiendo { $count } elementos
}
files-tray-done = { $count ->
    [one] 1 subida completada
    [many] { $count } de subidas completadas
   *[other] { $count } subidas completadas
}
files-tray-some-failed = { $done } subidos, { $failed } con error
files-tray-minutes-left = { $minutes ->
    [one] Queda aproximadamente un minuto
    [many] Quedan aproximadamente { $minutes } de minutos
   *[other] Quedan aproximadamente { $minutes } minutos
}
files-tray-seconds-left = Queda menos de un minuto
files-tray-starting = Iniciando…
files-tray-cancel-all = Cancelar todo
files-tray-cancel = Cancelar
files-tray-fold = Ocultar la lista
files-tray-unfold = Mostrar la lista
files-tray-close = Cerrar
files-tray-progress = { $place } · { $sent } de { $size }
files-tray-in = En { $place }
files-tray-cancelled = Cancelado
