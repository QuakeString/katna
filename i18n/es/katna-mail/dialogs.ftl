# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Acerca de Katna
about-tagline = Correo y calendario para el escritorio Linux
about-whats-new = Novedades

about-update-not-checked = Aún no se han comprobado las actualizaciones
about-update-checking = Buscando actualizaciones…
about-update-up-to-date = Katna Mail está actualizado
about-update-check-failed = No se pudieron buscar actualizaciones
about-update-available = La versión { $version } está disponible
about-update-downloading = Descargando la versión { $version }… { $percent } %
about-update-download-failed = La descarga de la versión { $version } no se completó
about-update-ready = La versión { $version } está lista para instalarse
about-update-ready-detail = Katna Mail se reinicia para terminar la actualización.
about-update-confirm = ¿Instalar la versión { $version }?
about-update-confirm-detail = Katna Mail se cerrará, instalará la actualización y se abrirá de nuevo donde lo dejaste. Tu equipo te pedirá la contraseña.
about-update-installing = Instalando la versión { $version }…
about-update-installing-detail = Escribe tu contraseña en la ventana que se ha abierto.
about-update-cancelled = La actualización no se instaló, porque no se dio la contraseña.
about-update-failed = La actualización no se pudo instalar: { $error }
about-update-unsupported = Esta copia de Katna Mail la actualiza tu gestor de paquetes.
about-update-restart-failed = La actualización está instalada, pero Katna Mail no pudo abrirse de nuevo ({ $error }). Ábrelo tú mismo.
about-update-check = Buscar actualizaciones
about-update-download = Descargar
about-update-retry = Reintentar
about-update-button = Actualizar
about-update-restart = Actualizar y reiniciar
about-update-cancel = Ahora no
about-changelog = Registro de cambios
about-source = Código fuente
about-coffee = Invítame a un café
about-coming-soon = Próximamente
about-follow-me = Sígueme en
about-love-title = Hecho con cariño para Rust, KDE y Linux
about-love-text = Con Rust, escribir una aplicación de correo rápida y segura es un placer: Katna no tiene código unsafe. El escritorio Plasma de KDE y su suite PIM inspiraron Katna, y Linux y la comunidad del software libre construyen el suelo que pisa. Gracias, y gracias también a las bibliotecas de abajo.
about-kde-text = KDE crea el escritorio en el que Katna se siente más a gusto, y lo hacen voluntarios y lo financian personas como tú. Si te gustan Plasma o las aplicaciones de KDE, considera hacer una donación a KDE.
about-donate-kde = Donar a KDE
about-gpui-title = Construido sobre GPUI, del proyecto Zed
about-gpui-text = Toda la interfaz de Katna Mail está construida sobre GPUI, el framework de interfaz rápido y acelerado por GPU que Zed Industries creó para el editor Zed. Cada píxel, animación y ventana que ves lo dibuja él. Gracias, equipo de Zed, por desarrollarlo en abierto. Apache-2.0.
about-gpui-github = GPUI en GitHub
about-personal-title = Un proyecto personal
about-personal-text = Katna Mail no pretende ser nuevo ni revolucionario. Es la aplicación de correo que su autor quería, y sus funciones y su aspecto están tomados de Gmail, Mailspring y Thunderbird. Solo ha sido posible gracias a lo lejos que han llegado los LLM.
about-built-on = CONSTRUIDO CON SOFTWARE LIBRE
about-credit-pimalaya = IMAP, SMTP e inicio de sesión (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Lectura y escritura de IMAP
about-credit-tantivy = Búsqueda
about-credit-sqlite = El almacén de correo
about-credit-rustls = Conexiones seguras
about-credit-mail-parser = Lectura del correo, de Stalwart Labs
about-credit-html5ever = Correo HTML, del proyecto Servo
about-credit-zbus = Comunicación con el escritorio mediante D-Bus y portales
about-credit-oo7 = Contraseñas en el llavero del escritorio
about-credit-hayro = Ver e imprimir PDF
about-credit-calamine = Vistas previas de hojas de cálculo
about-credit-resvg = Imágenes SVG
about-credit-jiff = Fechas y zonas horarias
about-credit-spellbook = Corrector ortográfico, del editor Helix
about-credit-smol = Hacer muchas cosas a la vez
about-all-libraries = Todas las bibliotecas que usa Katna ({ $count })
about-library-authors = por { $authors }
about-license = Katna es software libre bajo la GNU GPL, versión 3 o posterior.
about-close = Cerrar

## What’s new (shown after an update)

whats-new-title = Novedades de Katna Mail
whats-new-updated = Actualizado a la versión { $version }
whats-new-version = Versión { $version }
whats-new-more = { $count ->
    [one] Y { $count } más en el registro de cambios completo.
    [many] Y { $count } de cambios más en el registro de cambios completo.
   *[other] Y { $count } más en el registro de cambios completo.
}
whats-new-changelog = Registro de cambios completo
whats-new-got-it = Entendido

## First run: welcome page

onboarding-welcome-title = Te damos la bienvenida a Katna Mail
onboarding-welcome-lead = Tu correo en tu propio ordenador: rápido de buscar, legible sin conexión y privado.
onboarding-fast-title = Rápido, incluso sin conexión
onboarding-fast-text = Katna guarda aquí una copia de tu correo, así que abrirlo y buscar en él es instantáneo, con o sin conexión.
onboarding-providers-title = Funciona con tu correo
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud y cualquier otra cuenta IMAP o POP.
onboarding-private-title = Privado
onboarding-private-text = Tu correo va directamente de tu proveedor a este ordenador. Ningún servidor de Katna lo ve.
onboarding-get-started = Empezar

## First run: adding an account

onboarding-service-checking = Comprobando el servicio en segundo plano de Katna…
onboarding-service-running = El servicio en segundo plano de Katna está en marcha.
onboarding-service-missing = El servicio en segundo plano de Katna no está en marcha
onboarding-service-start = Descarga y envía tu correo. Inícialo desde un terminal y vuelve a comprobarlo:
onboarding-check-again = Volver a comprobar
onboarding-account-title = Añade tu cuenta de correo
onboarding-account-lead = Escribe tu dirección de correo y tu contraseña, y Katna encuentra la configuración del servidor. Gmail, Yahoo e iCloud necesitan una contraseña de aplicación, creada en los ajustes de seguridad de tu cuenta.
onboarding-add-account = Añadir una cuenta
onboarding-back = Atrás

## First run: choosing the look

onboarding-look-title = Hazlo tuyo
onboarding-look-lead = Elige cómo se abre el correo y cómo se ve Katna. Puedes cambiarlo cuando quieras en los ajustes rápidos.
onboarding-reading-pane = Panel de lectura
onboarding-pane-right = A la derecha de la lista
onboarding-pane-none = Sin división
onboarding-theme = Tema
onboarding-theme-system = Sistema
onboarding-theme-light = Claro
onboarding-theme-dark = Oscuro
onboarding-density = Densidad
onboarding-density-default = Predeterminada
onboarding-density-compact = Compacta
onboarding-continue = Continuar

## First run: done

onboarding-ready-title = Todo listo
onboarding-ready-lead = Katna está descargando tu correo. Aparece a medida que llega, y el correo nuevo aparece solo.
onboarding-ready-lead-address = Katna está descargando el correo de { $address }. Aparece a medida que llega, y el correo nuevo aparece solo.
onboarding-ready-tour = ¿Hacer un recorrido de un minuto para ver dónde está todo?
onboarding-skip = Ahora no
onboarding-take-tour = Hacer el recorrido

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Ayuda a mejorar Katna
share-lead = Cuando Katna falla, guarda un informe en este ordenador. Enviar estos informes ayuda a arreglar lo que salió mal. Puedes cambiarlo cuando quieras en Ajustes > Comentarios de usuarios.
share-sent = Qué se envía
share-sent-detail = El informe de fallo tal como puedes verlo en Ajustes: qué falló y en qué parte de Katna, la versión, tu sistema Linux y tu escritorio, y las últimas líneas del registro de Katna, que pueden nombrar carpetas de correo.
share-never-sent = Qué no se envía nunca
share-never-sent-detail = Tus mensajes, contactos, contraseñas, dirección IP, nombre de usuario ni nombre del equipo. Las direcciones de correo se quitan del informe.
share-where = Adónde va
share-where-detail = Al registro de fallos de Katna en Sentry, almacenado en la UE. Ningún identificador vincula los informes contigo.
share-dont-send = No enviar
share-send = Enviar informes de fallos
share-sending = Se enviarán los informes de fallos. Gracias.
share-local = Los informes de fallos se quedan en este ordenador.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Te damos la bienvenida a Katna Mail
tour-welcome-text = Un recorrido de un minuto te enseña dónde está todo.
tour-not-now = Ahora no
tour-start = Hacer el recorrido
tour-close = Cerrar
tour-skip = Saltar el recorrido
tour-back = Atrás
tour-done = Hecho
tour-next = Siguiente
tour-step = { $step } de { $total }
tour-compose-title = Escribe un mensaje
tour-compose-text = Redactar abre un mensaje nuevo abajo a la derecha, para que puedas seguir leyendo mientras escribes.
tour-search-title = Busca en todo tu correo
tour-search-text = La búsqueda también funciona sin conexión. El botón del extremo derecho añade filtros: remitente, destinatario, asunto, fechas y archivos adjuntos.
tour-menu-title = Muestra u oculta las carpetas
tour-menu-text = Este botón pliega la lista de carpetas. Mientras está oculta, deja el puntero sobre Correo a la izquierda para ver las carpetas.
tour-apps-title = Tus aplicaciones
tour-apps-text = El correo vive aquí ahora. Calendario, Contactos, Tareas, Notas y Feeds se le unirán en esta barra.
tour-tabs-title = Pestañas de Recibidos
tour-tabs-text = El correo nuevo se ordena en Principal, Promociones, Social, Notificaciones y Foros. Puedes desactivar las pestañas en los ajustes rápidos.
tour-list-title = Tus mensajes
tour-list-text = Haz clic en un mensaje para leerlo. Pasa el puntero por encima para ver acciones rápidas, haz clic derecho para ver más o marca varios para actuar sobre todos a la vez.
tour-settings-title = Ajustes rápidos
tour-settings-text = Cambia aquí el panel de lectura, la densidad y el tema. Desde ahí también se puede volver a iniciar el recorrido.
tour-account-title = Tu cuenta
tour-account-text = Mira en qué cuenta estás y añade otra.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] El servicio en segundo plano de Katna se ha detenido de forma inesperada.
    [one] El servicio en segundo plano de Katna se ha detenido de forma inesperada. Hay { $more } informe de fallo más guardado.
    [many] El servicio en segundo plano de Katna se ha detenido de forma inesperada. Hay { $more } de informes de fallos más guardados.
   *[other] El servicio en segundo plano de Katna se ha detenido de forma inesperada. Hay { $more } informes de fallos más guardados.
}
crash-mail = { $more ->
    [0] Katna Mail se cerró de forma inesperada la última vez.
    [one] Katna Mail se cerró de forma inesperada la última vez. Hay { $more } informe de fallo más guardado.
    [many] Katna Mail se cerró de forma inesperada la última vez. Hay { $more } de informes de fallos más guardados.
   *[other] Katna Mail se cerró de forma inesperada la última vez. Hay { $more } informes de fallos más guardados.
}
crash-view = Ver informe
crash-view-tooltip = Abrir el informe, guardado en este ordenador
crash-copy = Copiar informe
crash-close = Cerrar
sign-in-again-text = { $provider } te pide que vuelvas a iniciar sesión en { $address }.
sign-in-again-button = Iniciar sesión
sign-in-again-tooltip = Abrir la página de inicio de sesión de { $provider } en tu navegador
sign-in-again-waiting = Esperando a tu navegador…
sign-in-again-close = Cerrar
sign-in-again-done = Has vuelto a iniciar sesión en { $address }. Descargando tu correo…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] ¿Mover esta conversación a la papelera?
        [many] ¿Mover { $count } de conversaciones a la papelera?
       *[other] ¿Mover { $count } conversaciones a la papelera?
    }
   *[message] { $count ->
        [one] ¿Mover este mensaje a la papelera?
        [many] ¿Mover { $count } de mensajes a la papelera?
       *[other] ¿Mover { $count } mensajes a la papelera?
    }
}
delete-ask-body = { $count ->
    [one] Puedes deshacerlo justo después, o recuperarlo de la papelera más tarde.
    [many] Puedes deshacerlo justo después, o recuperarlos de la papelera más tarde.
   *[other] Puedes deshacerlo justo después, o recuperarlos de la papelera más tarde.
}
delete-ask-confirm = Mover a la papelera
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] ¿Eliminar esta conversación definitivamente?
        [many] ¿Eliminar { $count } de conversaciones definitivamente?
       *[other] ¿Eliminar { $count } conversaciones definitivamente?
    }
   *[message] { $count ->
        [one] ¿Eliminar este mensaje definitivamente?
        [many] ¿Eliminar { $count } de mensajes definitivamente?
       *[other] ¿Eliminar { $count } mensajes definitivamente?
    }
}
delete-forever-body = { $count ->
    [one] También se elimina del servidor. Esto no se puede deshacer.
    [many] También se eliminan del servidor. Esto no se puede deshacer.
   *[other] También se eliminan del servidor. Esto no se puede deshacer.
}
delete-forever-confirm = Eliminar definitivamente
delete-ask-dont-ask = No volver a preguntar
delete-ask-cancel = Cancelar
