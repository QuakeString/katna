# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = General
settings-tab-inbox = Recibidos
settings-tab-accounts = Cuentas
settings-tab-katna-account = Cuenta de Katna
settings-tab-subscriptions = Suscripción
settings-tab-appearance = Apariencia
settings-tab-shortcuts = Combinaciones de teclas
settings-tab-default-apps = Aplicaciones predeterminadas
settings-tab-folders-rules = Carpetas y reglas
settings-tab-compose = Redacción
settings-tab-mcp-server = Servidor MCP
settings-tab-feedback = Comentarios de usuarios
settings-tab-experimental = Experimental

## Settings page: tabs still to come

settings-tab-folders-rules-coming = Crea, renombra, mueve y oculta carpetas y etiquetas, y elige cuáles se sincronizan. Las reglas ordenan, etiquetan, reenvían o eliminan el correo nuevo automáticamente, según el remitente, el asunto o las palabras que contenga.
settings-tab-mcp-server-coming = Permite que los asistentes de IA de este ordenador busquen, lean y redacten borradores de tu correo, con tu permiso.

## Settings > General

settings-general-conversations = Vista de conversación
settings-general-conversations-group = Agrupar las respuestas al mismo correo
settings-general-conversations-group-detail = Una línea por conversación en la lista
settings-general-reading = Lectura
settings-general-newest-first = Mensaje más reciente primero
settings-general-newest-first-detail = Una conversación empieza por su última respuesta
settings-general-full-headers = Mostrar encabezados completos
settings-general-full-headers-detail = De, para, cc, fecha y asunto visibles en cada mensaje
settings-general-full-names = Nombres completos de los destinatarios
settings-general-full-names-detail = «para mí, Ada Lovelace» en lugar de «para mí, Ada»
settings-translation = Traducción
settings-translation-detail = El correo en otro idioma se puede leer en el tuyo.
settings-translation-offer = Ofrecer traducción
settings-translation-offer-detail = El texto de un mensaje va al servidor de Katna para traducirse solo cuando lo pides o cuando siempre traduces su idioma. Los adjuntos nunca se envían.
settings-translation-reading = Traducir a
settings-translation-always = Traducir siempre
settings-translation-never = No ofrecer nunca para
settings-translation-none = Ninguno aún. Elige desde la barra Traducir de un mensaje.
settings-general-mark-read = Marcar como leído
settings-general-mark-read-now = En cuanto se abre
settings-general-mark-read-1s = Tras 1 segundo abierta
settings-general-mark-read-3s = Tras 3 segundos abierta
settings-general-mark-read-never = Solo cuando la marque como leída
settings-general-auto-advance = Avance automático
settings-general-auto-advance-detail = Después de eliminar, archivar o mover la conversación abierta
settings-general-auto-advance-next = Abrir la conversación siguiente
settings-general-auto-advance-previous = Abrir la conversación anterior
settings-general-auto-advance-list = Volver a la lista
settings-general-confirm-delete = Eliminación
settings-general-confirm-delete-ask = Preguntar antes de eliminar varias conversaciones
settings-general-confirm-delete-ask-detail = La eliminación definitiva pregunta siempre
settings-general-reply-button = Botón de respuesta
settings-general-reply-all = Responder a todos
settings-general-reply-all-detail = El botón de respuesta junto a cada mensaje responde a todos, no solo al remitente
settings-general-remote-images = Imágenes de la Web
settings-general-remote-images-detail = Al cargar las imágenes de un mensaje, su remitente sabe que lo has abierto, cuándo y aproximadamente dónde. Si está desactivado, cada mensaje pregunta antes, y siempre puedes mostrar las imágenes de un remitente.
settings-general-remote-images-always = Mostrar siempre las imágenes
settings-general-remote-images-always-detail = En todos los mensajes, no solo de remitentes de confianza
settings-general-sending = Envío
settings-general-sending-detail = Cuánto tiempo espera un mensaje enviado, para poder recuperarlo.
settings-general-offline = Correo sin conexión
settings-general-offline-detail = El correo reciente se descarga completo para leerlo sin conexión. El correo más antiguo se descarga al abrirlo.
settings-general-offline-days = { $count ->
    [one] { $count } día
    [many] { $count } de días
   *[other] { $count } días
}
settings-general-offline-years = { $count ->
    [one] { $count } año
    [many] { $count } de años
   *[other] { $count } años
}
settings-general-offline-all = Todo el correo
settings-general-offline-note = Si eliges menos días, se conserva el correo ya descargado. No cambia nada en el servidor.
settings-general-notifications = Notificaciones
settings-general-notifications-detail = Del correo nuevo en Recibidos, incluso con Katna Mail cerrado.
settings-general-new-mail = Notificarme del correo nuevo
settings-general-new-mail-detail = Con Responder a todos, Marcar como leído y Archivar
settings-general-new-mail-sound = Reproducir un sonido
settings-general-new-mail-sound-detail = El sonido de correo nuevo del escritorio
settings-general-reset-cache = Restablecer caché
settings-general-reset-cache-detail = Cuando el correo se ve mal o desactualizado, o para liberar espacio en disco. No cambia nada en tus servidores de correo.
settings-general-desktop = Escritorio
settings-general-start-at-login = Iniciar Katna al iniciar sesión
settings-general-start-at-login-detail = Sincroniza el correo y muestra las notificaciones de correo nuevo y el icono de la bandeja, sin abrir la ventana
settings-general-login-window = Abrir también la ventana de Katna Mail
settings-general-login-window-detail = La ventana también se abre al iniciar sesión
settings-general-tray = Mostrar Katna en la bandeja del sistema
settings-general-tray-detail = Con el número de no leídos y un menú
settings-general-unread-badge = Número de no leídos en el icono de la barra de tareas
settings-general-unread-badge-detail = Cuántos mensajes de Recibidos están sin leer
settings-general-search-triggers = Buscar desde el escritorio
settings-general-search-triggers-detail = Escribe una de estas palabras y un espacio en KRunner o en la búsqueda de GNOME, y luego lo que quieras encontrar, para buscar en tu correo como lo hace aquí el cuadro de búsqueda. Separa las palabras con comas.
settings-general-search-triggers-none = Sin palabras; solo funciona «mail:»

## Settings > Inbox

settings-inbox-tabs = Pestañas de Recibidos
settings-inbox-tabs-detail = Ordena Recibidos en pestañas, como hace la web de tu proveedor de correo.
settings-inbox-tabs-show = Mostrar pestañas de Recibidos
settings-inbox-tabs-show-detail = Si está desactivado, una sola lista para todas las cuentas
settings-inbox-no-accounts = Añade una cuenta para elegir sus pestañas.
settings-inbox-tabs-automatic = Automático: { $tabs } ({ $provider })
settings-inbox-tabs-off = Sin pestañas
settings-inbox-tabs-gmail = Principal, Promociones, Social, Notificaciones, Foros
settings-inbox-tabs-focused = Prioritarios y Otros
settings-inbox-tabs-zoho = Bandeja de entrada, Boletines y Notificaciones
settings-inbox-tabs-shown = Pestañas visibles. El correo de una pestaña que desactives se queda en { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Panel de lectura
settings-appearance-reading-pane-detail = Dónde se muestra una conversación abierta.
settings-appearance-pane-right = A la derecha de la lista
settings-appearance-pane-none = Sin división
settings-appearance-density = Densidad
settings-appearance-density-default = Predeterminada
settings-appearance-density-compact = Compacta
settings-appearance-scaling = Escala
settings-appearance-scaling-detail = Hace todo más grande o más pequeño en Katna Mail, además de la escala del propio escritorio: texto, iconos, espaciado y separadores. El correo que envías mantiene su propio tamaño de letra. Con tamaños muy pequeños puede costar hacer clic en los iconos.
settings-appearance-theme = Tema
settings-appearance-theme-system = Sistema
settings-appearance-theme-light = Claro
settings-appearance-theme-dark = Oscuro
settings-appearance-desktop-colors = Colores del escritorio
settings-appearance-desktop-colors-use = Usar los colores del escritorio
settings-appearance-desktop-colors-use-detail = El esquema de color y el color de acento del escritorio
settings-appearance-app-names = Nombres de las aplicaciones
settings-appearance-app-names-show = Mostrar los nombres de las aplicaciones
settings-appearance-app-names-show-detail = Nombres bajo los iconos de las aplicaciones, en el extremo izquierdo
settings-appearance-sender-pictures = Imágenes de los remitentes
settings-appearance-sender-pictures-show = Mostrar logotipos de empresas
settings-appearance-sender-pictures-show-detail = Se buscan por el dominio del remitente, nunca por mensaje, y se guardan una semana
settings-appearance-important = Marcadores de importancia
settings-appearance-important-show = Mostrar marcadores de importancia
settings-appearance-important-show-detail = Junto a cada mensaje de la lista
settings-appearance-message-width = Ancho de los mensajes
settings-appearance-message-width-limit = Limitar el ancho de los mensajes
settings-appearance-message-width-limit-detail = Las líneas largas se leen mejor en una ventana ancha
settings-appearance-mail-colors = Colores del correo
settings-appearance-mail-colors-detail = La mayoría del correo está diseñado para una página blanca. Con un tema oscuro, sus colores se cambian por otros oscuros que se leen bien; si está desactivado, mantiene los colores del remitente sobre una página clara.
settings-appearance-dark-mail = Colores oscuros también para el correo
settings-appearance-dark-mail-detail = Solo con el tema oscuro
settings-appearance-attachment-previews = Vistas previas de adjuntos
settings-appearance-attachment-previews-show = Mostrar vistas previas de los archivos adjuntos
settings-appearance-attachment-previews-show-detail = Una pequeña imagen del contenido de cada archivo en su tarjeta

## Settings > Default apps

settings-default-apps-intro = Dónde se abren los archivos adjuntos al hacer clic en ellos. El visor siempre puede abrir un archivo también en otra aplicación. Las aplicaciones predeterminadas del escritorio se configuran en sus propios ajustes.
settings-default-apps-pdf = Archivos PDF
settings-default-apps-pdf-detail = Páginas, con zoom.
settings-default-apps-pictures = Imágenes
settings-default-apps-pictures-detail = Fotos (enderezadas), PNG, GIF, WebP, BMP, TIFF y SVG.
settings-default-apps-text = Archivos de texto
settings-default-apps-text-detail = Texto sin formato, registros, código y otros textos.
settings-default-apps-sheets = Hojas de cálculo
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) y CSV.
settings-default-apps-documents = Documentos
settings-default-apps-documents-detail = Word (docx, doc), texto de OpenDocument (odt) y presentaciones (pptx, ppt, odp).
settings-default-apps-katna = Visor de Katna Mail
settings-default-apps-system = Aplicación predeterminada del escritorio
settings-default-apps-ask = Preguntar qué aplicación usar cada vez
settings-default-apps-after-saving = Después de guardar
settings-default-apps-show-folder = Mostrar los archivos guardados en su carpeta
settings-default-apps-show-folder-detail = Abre el gestor de archivos con los adjuntos guardados seleccionados

## Settings > Compose

settings-compose-send-from = Enviar mensajes nuevos desde
settings-compose-send-from-detail = Los mensajes nuevos empiezan desde esta cuenta; la fila De elige otra. Las respuestas y los reenvíos siempre salen de la cuenta a la que llegó el mensaje original.
settings-compose-send-from-current = La cuenta en la que estás
settings-compose-send-on-replies = Enviar en respuestas
settings-compose-send-on-replies-detail = Qué hace Enviar en una respuesta o un reenvío. El menú junto a Enviar ofrece la otra opción.
settings-compose-send-plain = Enviar
settings-compose-send-archive = Enviar y archivar
settings-compose-signatures = Firmas
settings-compose-signatures-detail = Se añade debajo de tu mensaje, tras una línea «--». Elige otra en la ventana de redacción.
settings-compose-untitled = Sin título
settings-compose-signature-name = Nombre, como Trabajo
settings-compose-signature-first = Mi firma
settings-compose-signature-numbered = Firma { $number }
settings-compose-signature-delete = Eliminar
settings-compose-signature-deleted = Firma eliminada
settings-compose-signature-new = Crear nueva
settings-compose-no-signatures = Aún no hay firmas.
settings-compose-no-signature = Sin firma
settings-compose-for-new-mail = Para correos nuevos
settings-compose-for-replies = Para respuestas y reenvíos
settings-compose-for-replies-detail = En una conversación en la que firmaste un mensaje, la respuesta empieza con esa firma.
settings-compose-format = Formato
settings-compose-plain-text = Escribir en texto sin formato
settings-compose-plain-text-detail = El correo nuevo empieza sin formato; la ventana de redacción puede cambiarlo
settings-compose-spelling = Ortografía
settings-compose-spell-check = Revisar la ortografía mientras escribo
settings-compose-spell-check-detail = Las palabras mal escritas se subrayan, con sugerencias al hacer clic derecho
settings-compose-spell-desktop = Idioma del escritorio ({ $language })
settings-compose-templates = Plantillas
settings-compose-templates-detail = Guarda los correos que escribes a menudo y úsalos para empezar un correo nuevo o una respuesta.
settings-compose-no-templates = Aún no hay plantillas. En un mensaje, elige Plantillas y luego Guardar como plantilla.
settings-compose-template-new = Crear nueva
settings-compose-template-new-name = Nueva plantilla
settings-compose-template-subject = Asunto
settings-compose-template-text = Texto de la plantilla
settings-compose-template-fields = {"{"}first name{"}"}, {"{"}name{"}"} y {"{"}my name{"}"} se rellenan con el nombre del destinatario y el tuyo.
settings-compose-template-remove-file = Quitar archivo adjunto
settings-compose-template-save = Guardar
settings-compose-template-saved = Plantilla guardada
settings-compose-template-needs-name = Pon un nombre a la plantilla
settings-compose-template-delete = Eliminar plantilla
settings-compose-template-deleted = Plantilla eliminada
settings-compose-template-delete-failed = No se ha podido eliminar la plantilla: { $error }

## Settings > Shortcuts

settings-shortcuts-set = Conjunto de combinaciones
settings-shortcuts-set-detail = Empieza con las teclas de una aplicación de correo que conozcas. Aquí, Cmd es Ctrl. Tus propios cambios se mantienen sobre el conjunto, y Restablecer valores predeterminados vuelve a las teclas del conjunto.
settings-shortcuts-single = Combinaciones de una tecla
settings-shortcuts-single-detail = Teclas sin Ctrl ni Alt, como en el correo web: e archiva, j y k mueven, / busca. Funcionan en la lista y en la conversación abierta, nunca mientras escribes.
settings-shortcuts-single-use = Usar combinaciones de una tecla
settings-shortcuts-single-use-detail = Las combinaciones con Ctrl siempre funcionan
settings-shortcuts-how = Haz clic en una tecla para cambiarla, o en + para añadir una, y pulsa las teclas nuevas. Esc cancela.
settings-shortcuts-restore = Restablecer valores predeterminados
settings-shortcuts-no-key = Sin tecla
settings-shortcuts-press = Pulsa las teclas…
settings-shortcuts-then = { $keys } y luego…
settings-shortcuts-moved = { $keys } ahora hace «{ $action }» en lugar de «{ $previous }».
settings-shortcuts-single-off = Las combinaciones de una tecla están desactivadas, así que esta tecla funcionará cuando se activen.
settings-shortcuts-restored = Todas las combinaciones vuelven a tener las teclas de su conjunto.

## Settings search: the line under a result

settings-general-language-summary = Idioma de la aplicación, las fechas y los números
settings-general-reading-summary = Mensaje más reciente primero, encabezados completos, nombres completos de los destinatarios
settings-translation-summary = Traduce el correo en otros idiomas con el servidor de Katna al idioma que elijas
settings-general-mark-read-summary = Cuándo se marca como leída una conversación abierta: al momento, tras 1 o 3 segundos, o a mano
settings-general-auto-advance-summary = Qué se abre después de eliminar, archivar o mover la conversación abierta: la siguiente, la anterior o la lista
settings-general-confirm-delete-summary = Preguntar antes de mover varias conversaciones a la papelera
settings-general-reply-button-summary = El botón de respuesta junto a cada mensaje responde a todos
settings-general-remote-images-summary = Mostrar siempre las imágenes de todos los mensajes
settings-general-sending-summary = Deshacer el envío: cuánto tiempo espera un mensaje enviado, para poder recuperarlo
settings-general-offline-summary = Cuántos días de correo reciente se descargan completos, para leerlos sin conexión
settings-general-notifications-summary = Notificaciones de correo nuevo y su sonido
settings-general-reset-cache-summary = Eliminar el correo descargado, las imágenes de los remitentes y el índice de búsqueda, y volver a descargarlos
settings-general-desktop-summary = Iniciar Katna al iniciar sesión, el icono de la bandeja del sistema y el número de no leídos en el icono de la barra de tareas
settings-accounts-accounts-summary = Añadir o quitar una cuenta, o cambiar su imagen
settings-appearance-density-summary = Líneas predeterminadas o compactas en la lista
settings-appearance-scaling-summary = Hacer todo más grande o más pequeño: texto, iconos, espaciado y separadores
settings-appearance-theme-summary = Sistema, claro u oscuro
settings-appearance-sender-pictures-summary = Logotipos de empresas, buscados por el dominio del remitente
settings-appearance-important-summary = El marcador de importancia junto a cada mensaje de la lista
settings-appearance-mail-colors-summary = Colores oscuros para el correo HTML con un tema oscuro, o los colores del remitente
settings-appearance-attachment-previews-summary = Una pequeña imagen del contenido de cada archivo adjunto
settings-shortcuts-set-summary = Empezar con las teclas de Gmail, Inbox by Gmail, Apple Mail, Outlook o Thunderbird
settings-shortcuts-single-summary = Teclas sin Ctrl ni Alt, como en el correo web
settings-default-apps-pdf-summary = Dónde se abren los adjuntos PDF
settings-default-apps-pictures-summary = Dónde se abren las fotos y las imágenes
settings-default-apps-text-summary = Dónde se abren el texto sin formato, los registros y el código
settings-default-apps-sheets-summary = Dónde se abren los archivos de Excel, OpenDocument y CSV
settings-default-apps-documents-summary = Dónde se abren los textos de Word y OpenDocument y las presentaciones
settings-default-apps-after-saving-summary = Mostrar los adjuntos guardados en su carpeta
settings-compose-send-from-summary = La cuenta desde la que sale el correo nuevo: la primera, otra o aquella en la que estás
settings-compose-send-on-replies-summary = Enviar, o Enviar y archivar la conversación, en respuestas y reenvíos
settings-compose-signatures-summary = Se añade debajo de tu mensaje, tras una línea «--»
settings-compose-for-new-mail-summary = La firma con la que empieza el correo nuevo
settings-compose-for-replies-summary = La firma con la que empiezan las respuestas y los reenvíos
settings-compose-format-summary = Escribir el correo nuevo en texto sin formato
settings-compose-spelling-summary = Revisar la ortografía al escribir, y el idioma del diccionario
settings-general-search-triggers-summary = Palabras que buscan en tu correo desde KRunner o la búsqueda de GNOME
settings-compose-templates-summary = Guarda los correos que escribes a menudo y úsalos para empezar un correo nuevo o una respuesta
settings-feedback-crash-reports-summary = Guardar informes de fallos en este ordenador cuando Katna Mail o su servicio en segundo plano fallan
settings-feedback-saved-summary = Ver, copiar o eliminar los informes de fallos guardados en este ordenador
settings-feedback-help-improve-summary = Enviar informes de fallos para ayudar a corregir el problema; desactivado salvo que lo actives
settings-experimental-blur-summary = El escritorio se ve desenfocado a través de la barra superior, y los menús parecen de cristal esmerilado
settings-search-shortcut = Combinación de teclas
settings-search-tab = Pestaña de ajustes
settings-search-none = Ningún ajuste coincide con «{ $query }».
settings-search-results = Ajustes que coinciden con «{ $query }»

## Settings: opening at login

settings-open-at-login-failed = No se ha podido cambiar el inicio al iniciar sesión: { $error }

## Settings > General > Time

settings-time = Hora
settings-clock-language = Como se escribe en el idioma
settings-clock-12 = 12 horas, como 2:05 p. m.
settings-clock-24 = 24 horas, como 14:05
settings-time-summary = Reloj de 12 o 24 horas, o como se escribe en el idioma

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = Aplicación de correo predeterminada
settings-general-mail-app-detail = Los enlaces de correo electrónico de otras aplicaciones y de sitios web abren aquí un mensaje nuevo.
mail-app-is-default = Katna Mail es tu aplicación de correo predeterminada.
mail-app-is-other = Los enlaces de correo electrónico se abren en otra aplicación.
mail-app-make-default = Establecer como predeterminada
mail-app-make-default-failed = No se ha podido cambiar la aplicación de correo predeterminada.
settings-general-mail-app-summary = Abrir en Katna Mail los enlaces de correo electrónico de otras aplicaciones y sitios web
settings-compose-grammar = Gramática
settings-compose-grammar-detail = Se revisa en este ordenador con Harper. Por ahora solo en inglés: el texto en otros idiomas no se toca.
settings-compose-grammar-check = Revisar la gramática
settings-compose-grammar-check-detail = Subrayar los errores gramaticales mientras escribes, en inglés
settings-compose-suggestions = Sugerencias de escritura
settings-compose-suggestions-detail = Se aprenden en este ordenador a partir del correo que enviaste y del correo al que respondes; nada sale de él. Pulsa Tab para aceptar una sugerencia o sigue escribiendo.
settings-compose-suggestions-on = Sugerir mientras escribo
settings-compose-suggestions-on-detail = Mostrar en gris el resto probable de una frase mientras escribes
settings-compose-grammar-summary = Subrayar los errores gramaticales mientras escribes, en inglés
settings-compose-suggestions-summary = Mostrar en gris el resto probable de una frase mientras escribes
