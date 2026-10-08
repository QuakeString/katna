# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Reglas
settings-rules-summary = Ordenar, etiquetar, reenviar o silenciar el correo nuevo automáticamente
settings-rules-intro = Las reglas ordenan el correo nuevo automáticamente, en este orden. Arrastra para reordenar.
settings-rules-all-accounts = Todas las cuentas
settings-rules-new = Nueva regla
settings-rules-none = Aún no hay reglas. Una regla ordena el correo nuevo automáticamente: por remitente, asunto o palabras.
settings-rules-none-account = Aún no hay reglas para esta cuenta.
settings-rules-drag = Arrastra para reordenar
settings-rules-edit = Editar regla
settings-rules-turn-off = Desactivar esta regla
settings-rules-turn-on = Activar esta regla

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Reglas iniciales
settings-rules-starters-intro = Desactivadas hasta que actives una. Funcionan en todas tus cuentas; edita una para cambiarla.
settings-rules-starter-turning-on = Activando «{ $name }»…
settings-rules-starter-failed = No se ha podido activar «{ $name }»: { $error }
rules-starter-promotions = Silenciar promociones
rules-starter-newsletters = Boletines a Lectura
rules-starter-receipts = Recibos y facturas
rules-starter-deliveries = Envíos
rules-starter-train = Billetes de tren
rules-starter-flight = Billetes de avión
rules-starter-codes = Códigos de un solo uso
rules-starter-security = Alertas de seguridad
rules-starter-social = Correo social
rules-starter-invites = Invitaciones de calendario
rules-starter-folder-reading = Lectura
rules-starter-folder-receipts = Recibos
rules-starter-folder-deliveries = Envíos
rules-starter-folder-travel = Viajes
rules-starter-folder-social = Social
rules-runs-katna = Se ejecuta en Katna
rules-runs-gmail = Se ejecuta en Gmail
rules-runs-sieve = Se ejecuta en el servidor
rules-stopped = Detenida
rules-error-folder-gone = La carpeta que usa esta regla ya no existe. Edita la regla para elegir otra.
rules-error-no-archive = Esta cuenta no tiene carpeta de archivo. Edita la regla para que haga otra cosa.
rules-error-no-trash = Esta cuenta no tiene carpeta Papelera. Edita la regla para que haga otra cosa.
rules-error-cannot-send = Esta cuenta no puede enviar correo, así que la regla no puede reenviarlo.
rules-error-other = { $error }. Edita la regla y vuelve a activarla.

settings-folders = Carpetas
settings-folders-summary = Contadores de no leídos en el panel de carpetas
settings-folders-unread-counts = Contador de no leídos en cada carpeta
settings-folders-unread-counts-detail = Desactivado: solo Recibidos muestra cuántos hay sin leer

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } y { $next }
rules-summary-or = { $first } o { $next }
rules-summary-more = { $count } más
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Tiene un adjunto
rules-summary-no-attachment = No tiene adjuntos
rules-summary-mailing-list = De una lista de correo
rules-summary-not-mailing-list = No es de una lista de correo
rules-summary-tab = En la pestaña { $tab }
rules-summary-not-tab = Fuera de la pestaña { $tab }
rules-summary-move = mover a { $folder }
rules-summary-archive = saltar Recibidos
rules-summary-trash = mover a la papelera
rules-summary-mark-read = marcar como leído
rules-summary-star = destacar
rules-summary-important = marcar como importante
rules-summary-label = etiquetar como { $label }
rules-summary-forward = reenviar a { $address }
rules-summary-dont-notify = no notificar
rules-summary-read-after = { $count ->
    [one] marcar como leído tras { $count } día
    [many] marcar como leído tras { $count } de días
   *[other] marcar como leído tras { $count } días
}
rules-summary-folder-gone = una carpeta que ya no existe

## The rule editor

rules-editor-new-title = Nueva regla
rules-editor-edit-title = Editar regla
rules-editor-name-hint = Nombre de la regla
rules-editor-when = Cuando un correo nuevo cumpla
rules-editor-of-these = de estas:
rules-mode-all = todas
rules-mode-any = alguna
rules-field-from = De
rules-field-to = Para
rules-field-cc = Cc
rules-field-any-recipient = Para o Cc
rules-field-reply-to = Responder a
rules-field-subject = Asunto
rules-field-body = Texto
rules-field-attachment-name = Nombre del adjunto
rules-field-has-attachment = Tiene adjunto
rules-field-mailing-list = De una lista de correo
rules-field-tab = Pestaña de Recibidos
rules-comparator-contains = contiene
rules-comparator-not-contains = no contiene
rules-comparator-begins-with = empieza por
rules-comparator-ends-with = termina en
rules-comparator-equals = es exactamente
rules-comparator-matches = coincide con el patrón
rules-has-yes = sí
rules-has-no = no
rules-editor-value-hint = Palabras o una dirección
rules-editor-add-condition = Añadir una condición
rules-editor-remove = Quitar
rules-editor-then = Entonces:
rules-action-move = Mover a
rules-action-archive = Saltar Recibidos (archivar)
rules-action-trash = Mover a la papelera
rules-action-mark-read = Marcar como leído
rules-action-star = Destacar
rules-action-important = Marcar como importante
rules-action-label = Añadir etiqueta
rules-action-forward = Reenviar a
rules-action-dont-notify = No notificar
rules-action-read-after = Marcar como leído tras
rules-editor-choose-folder = Elegir una carpeta
rules-editor-choose-label = Elegir una etiqueta
rules-editor-new-folder = Nueva: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Dirección de correo electrónico
rules-editor-days = días
rules-editor-add-action = Añadir una acción
rules-editor-stop = Parar aquí: las reglas siguientes no se aplican a este correo
rules-editor-accounts = Cuentas:
rules-editor-accounts-none = Elegir cuentas
rules-editor-accounts-many = { $count ->
    [one] { $count } cuenta
    [many] { $count } de cuentas
   *[other] { $count } cuentas
}
rules-editor-matches = Coincide con { $mails } de los últimos { $days } días
rules-editor-mails = { $count ->
    [one] { $count } correo
    [many] { $count } de correos
   *[other] { $count } correos
}
rules-editor-counting = Contando el correo que coincide…
rules-editor-show = Mostrarlos
rules-editor-also-apply = Aplicar también a estos { $count }
rules-editor-runs-katna = Se ejecuta en Katna, mientras este ordenador está encendido.
rules-editor-runs-gmail = Se ejecuta en Gmail, así que también funciona en tu móvil y con este ordenador apagado.
rules-editor-runs-sieve = Se ejecuta en tu servidor de correo, así que también funciona en tu móvil y con este ordenador apagado.
rules-note-gmail-action = Se ejecuta en Katna: los filtros de Gmail no pueden «{ $action }».
rules-note-sieve-action = Se ejecuta en Katna: las reglas de tu servidor de correo no pueden «{ $action }».
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Se ejecuta en Katna: los filtros de Gmail no pueden comprobar «{ $test }» como lo hace Katna.
rules-note-sieve-condition = Se ejecuta en Katna: las reglas de tu servidor de correo no pueden comprobar «{ $test }» como lo hace Katna.
rules-note-order = Se ejecuta en Katna, como una regla anterior de la cuenta: las reglas se aplican en el orden de la lista.
rules-note-gmail-stop = Se ejecuta en Katna: los filtros de Gmail no pueden impedir que se apliquen las reglas siguientes.
rules-note-gmail-forward = Se ejecuta en Katna: Gmail solo reenvía a direcciones verificadas en sus ajustes, y { $address } no lo está.
rules-note-gmail-folder = Se ejecuta en Katna: Gmail no tiene etiqueta para una carpeta que usa esta regla.
rules-note-sieve-folder = Se ejecuta en Katna: tu servidor de correo no tiene una carpeta que usa esta regla.
rules-note-gmail-sign-in = Se ejecuta en Katna hasta que vuelvas a iniciar sesión en Google y permitas a Katna crear filtros de Gmail.
rules-note-sieve-other-script = Se ejecuta en Katna: hay otro script de reglas («{ $name }») activo en tu servidor de correo.
rules-note-gmail-failed = Se ejecuta en Katna: Gmail no la ha aceptado ({ $error }).
rules-note-sieve-failed = Se ejecuta en Katna: tu servidor de correo no la ha aceptado ({ $error }).
rules-editor-cancel = Cancelar
rules-editor-save = Guardar
rules-editor-saving = Guardando…
rules-editor-delete = Eliminar regla
rules-editor-delete-ask = ¿Eliminar esta regla?
rules-editor-delete-keep = Conservarla
rules-editor-delete-confirm = Eliminar
rules-editor-needs-folder = Elige una carpeta para cada «Mover a» y una etiqueta para cada «Añadir etiqueta».
rules-editor-needs-days = «Marcar como leído tras» necesita un número de días, de 1 a 3650.
rules-saved = Regla guardada
rules-saved-applied = { $count ->
    [one] Regla guardada y aplicada a { $count } correo
    [many] Regla guardada y aplicada a { $count } de correos
   *[other] Regla guardada y aplicada a { $count } correos
}
rules-apply-failed = Regla guardada, pero no se ha podido aplicar: { $error }
rules-deleted = Regla eliminada
rules-delete-failed = No se ha podido eliminar la regla: { $error }
rules-change-failed = No se han podido cambiar las reglas: { $error }
