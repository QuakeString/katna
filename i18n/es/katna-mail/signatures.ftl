# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Tu nombre, y lo que quieras añadir debajo

## Its formatting bar

signature-bold = Negrita
signature-italic = Cursiva
signature-underline = Subrayado
signature-link = Enlace
signature-link-apply = Aplicar
signature-picture = Insertar imagen
signature-align-left = Alinear a la izquierda
signature-align-center = Centrar
signature-align-right = Alinear a la derecha
signature-numbered-list = Lista numerada
signature-bulleted-list = Lista con viñetas
signature-remove-formatting = Borrar formato

## Adding a picture

signature-picture-choose = Insertar
signature-picture-too-big = Las imágenes de una firma pueden ocupar hasta { $size }.
signature-picture-kind = Elige una imagen PNG, JPEG, GIF o WebP.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = Diseño
signature-layout-own = El tuyo
signature-layout-classic = Clásico
signature-layout-logo-left = Logotipo a la izquierda
signature-layout-photo = Foto
signature-layout-band = Franja de color
signature-layout-one-line = Una línea
signature-layout-centred = Centrado
signature-layout-banner = Con banner
signature-layout-underline = Subrayado
signature-layout-side-bar = Barra lateral
signature-layout-card = Tarjeta
signature-layout-monogram = Monograma
signature-layout-plain = Texto sin formato
signature-layout-mobile-label = M:
signature-layout-office-label = T:
signature-layout-email-label = E:
signature-layout-name = Nombre
signature-layout-job = Cargo
signature-layout-company = Empresa
signature-layout-mobile = Móvil
signature-layout-office = Oficina
signature-layout-email = Correo electrónico
signature-layout-website = Sitio web
signature-layout-address = Dirección
signature-layout-pictures = Imágenes
signature-layout-logo = Logotipo
signature-layout-photo-picture = Foto
signature-layout-banner-picture = Banner
signature-layout-remove-picture = Quitar
signature-layout-pages = Páginas
signature-layout-page-placeholder = Añade la dirección de una página
signature-layout-colour = Color
signature-layout-picture-failed = No se ha podido usar { $name } como imagen.
signature-layout-preview = Cómo lo ve el lector
signature-layout-light = Claro
signature-layout-dark = Oscuro
signature-layout-text = Texto sin formato
signature-layout-inside = Las imágenes se envían dentro del correo, así que se ven incluso donde las imágenes remotas están desactivadas. Esta añade { $size } a cada correo.
signature-layout-free = ¿Quieres otra cosa?
signature-layout-edit = Editar a mano
signature-layout-edit-confirm = ¿Editarla a mano? Sus campos y su diseño desaparecen, y conserva su aspecto en la medida en que el editor pueda mantenerlo.
signature-layout-use-confirm = ¿Usar el diseño { $layout }? Sustituye esta firma y se rellena con sus datos.
signature-layout-use = Usar diseño
signature-layout-cancel = Cancelar

## Paste HTML

signature-html-title = Pegar HTML
signature-html-subtitle = Para una firma que diseñaste en otro sitio
signature-html-placeholder = Pega aquí el HTML de la firma
signature-html-name = Pegada
signature-html-new = Se guarda como una firma nueva, «{ $name }»
signature-html-replaces = Sustituye a «{ $name }»
signature-html-cancel = Cancelar
signature-html-save = Guardar
signature-html-fetching = Descargando sus imágenes…
signature-html-pictures-inside = { $count ->
    [one] { $count } imagen descargada e incluida en el correo ({ $size })
    [many] { $count } de imágenes descargadas e incluidas en el correo ({ $size })
   *[other] { $count } imágenes descargadas e incluidas en el correo ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] No se ha podido descargar { $count } imagen, así que los lectores la cargan desde la web
    [many] No se han podido descargar { $count } de imágenes, así que los lectores las cargan desde la web
   *[other] No se han podido descargar { $count } imágenes, así que los lectores las cargan desde la web
}
signature-html-removed = Se han quitado scripts, formularios y píxeles de seguimiento, que las aplicaciones de correo bloquean de todos modos
signature-html-style-sheet = Se ha omitido una hoja de estilos: el correo solo conserva los estilos escritos en cada elemento
signature-html-links = Se han quitado enlaces que no llevaban a un sitio web, una dirección o un teléfono
signature-html-plain-text = Se ha creado a partir de ella una versión en texto sin formato, para las aplicaciones de correo que solo muestran texto

## Import

signature-import-title = Importar
signature-import-subtitle = Desde Gmail, Thunderbird, Evolution y KMail
signature-import-looking = Buscando firmas…
signature-import-none = No se han encontrado firmas. Para otra aplicación, copia el HTML de su firma y usa Pegar HTML.
signature-import-from = De { $app }
signature-import-already = ya está en Katna
signature-import-gmail-sign-in = { $address }: vuelve a iniciar sesión en Ajustes > Cuentas para que Katna pueda leer las firmas de Gmail.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Cancelar
signature-import-do = { $count ->
    [one] Importar { $count } firma
    [many] Importar { $count } de firmas
   *[other] Importar { $count } firmas
}
signature-import-name = { $name } ({ $app })
