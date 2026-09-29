# Katna Mail, Spanish (Spain) (Español): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Contactos
contacts-frequent = Frecuentes
contacts-other = Otros contactos
contacts-other-about = Personas a las que has escrito desde Gmail pero no has guardado
contacts-other-email = Enviar correo
contacts-other-empty = No hay otros contactos. Las personas a las que escribes desde Gmail y no guardas aparecen aquí.
contacts-other-allow = Para ver otros contactos, vuelve a iniciar sesión en tu cuenta de Gmail y permite que Katna los vea.
contacts-labels = Etiquetas
contacts-label-options = Opciones de etiqueta
contacts-label-rename = Cambiar nombre de la etiqueta
contacts-label-email = Enviar correo a todos
contacts-label-delete = Eliminar etiqueta
contacts-label-new = Nueva etiqueta
contacts-label-name = Nombre de la etiqueta
contacts-label-button = Etiqueta
contacts-label-menu = Etiquetar como:
contacts-label-added = Añadido a { $name }
contacts-label-removed = Eliminado de { $name }
contacts-label-renamed = Etiqueta renombrada como { $name }
contacts-label-deleted = Se ha eliminado la etiqueta { $name }
contacts-label-no-email = Nadie con esta etiqueta tiene una dirección de correo
contacts-manage = Corregir y administrar
contacts-merge = Combinar y corregir
contacts-merge-about = { $count ->
    [one] { $count } sugerencia: contactos que parecen la misma persona
    [many] { $count } de sugerencias: contactos que parecen la misma persona
   *[other] { $count } sugerencias: contactos que parecen la misma persona
}
contacts-merge-none = No hay duplicados. Aquí aparecen los contactos con el mismo nombre o número de teléfono.
contacts-merge-count = { $count ->
    [one] { $count } contacto
    [many] { $count } de contactos
   *[other] { $count } contactos
}
contacts-merge-all = Combinar todos
contacts-merge-button = Combinar
contacts-merge-dismiss = Descartar
contacts-merged = { $count ->
    [1] Contactos combinados
    [one] { $count } combinación realizada
    [many] { $count } de combinaciones realizadas
   *[other] { $count } combinaciones realizadas
}
contacts-import = Importar
contacts-export = Exportar
contacts-import-file = Importar contactos desde un archivo vCard o CSV
contacts-imported = { $count ->
    [one] { $count } contacto importado a { $place }
    [many] { $count } de contactos importados a { $place }
   *[other] { $count } contactos importados a { $place }
}
contacts-imported-some = { $count ->
    [one] { $count } contacto importado a { $place }; { $skipped } ya guardados, omitidos
    [many] { $count } de contactos importados a { $place }; { $skipped } ya guardados, omitidos
   *[other] { $count } contactos importados a { $place }; { $skipped } ya guardados, omitidos
}
contacts-import-none = No se han encontrado contactos en { $name }
contacts-import-all-saved = Todos los contactos de { $name } ya están guardados
contacts-import-failed = No se ha podido leer { $name }: { $error }
contacts-exported = { $count ->
    [one] { $count } contacto exportado a { $path }
    [many] { $count } de contactos exportados a { $path }
   *[other] { $count } contactos exportados a { $path }
}
contacts-export-none = No hay contactos que exportar
contacts-export-failed = No se han podido exportar los contactos: { $error }
contacts-print = Imprimir
contacts-print-title = Contactos
contacts-print-none = No hay contactos para imprimir
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Cumpleaños: { $day }
contacts-print-nickname = Apodo: { $name }
contacts-create = Crear contacto

## Search and the list

contacts-search = Buscar contactos
contacts-loading = Cargando contactos…
contacts-empty = Aún no hay contactos guardados. Los contactos que guardes en Gmail, Outlook o tu servicio de correo aparecerán aquí.
contacts-empty-no-books = Los contactos de tus cuentas aparecerán aquí cuando se sincronicen.
contacts-none-found = Ningún contacto coincide con tu búsqueda.
contacts-starred = { $count ->
    [one] Contacto destacado ({ $count })
    [many] Contactos destacados ({ $count })
   *[other] Contactos destacados ({ $count })
}
contacts-count = Contactos ({ $count })
contacts-col-name = Nombre
contacts-col-email = Correo electrónico
contacts-col-phone = Número de teléfono
contacts-col-job = Puesto y empresa
contacts-col-labels = Etiquetas

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Permitir que Katna lea los contactos de { $address }.
contacts-allow-many = { $more ->
    [one] Permitir que Katna lea los contactos de { $address } y de { $more } cuenta más.
    [many] Permitir que Katna lea los contactos de { $address } y de { $more } de cuentas más.
   *[other] Permitir que Katna lea los contactos de { $address } y de { $more } cuentas más.
}
contacts-allow-button = Permitir

## A contact's page

contacts-back = Volver a los contactos
contacts-edit = Editar
contacts-delete = Eliminar
contacts-qr = Compartir como código QR
contacts-qr-about = Escanea esto con la cámara de un teléfono para guardar el contacto.
contacts-qr-too-long = Este contacto tiene demasiados datos para caber en un código QR.
contacts-qr-done = Listo
contacts-deleted = Se ha eliminado a { $name }
contacts-added = Se ha añadido a { $name } a los contactos
contacts-find-mail = Correo
contacts-details = Datos de contacto
contacts-saved-in = Guardado en
contacts-notes = Notas
contacts-birthday = Cumpleaños
contacts-nickname = Apodo
contacts-this-computer = Este ordenador
contacts-kind-home = Casa
contacts-kind-work = Trabajo
contacts-kind-mobile = Móvil
contacts-kind-other = Otro
contacts-source-google = Contactos de Google
contacts-source-microsoft = Contactos de Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Crear contacto
contacts-edit-title = Editar contacto
contacts-edit-save = Guardar
contacts-edit-saving = Guardando…
contacts-edit-cancel = Cancelar
contacts-saved = Contacto guardado
contacts-edit-save-to = Guardar en
contacts-edit-changes-go-to = Los cambios se guardan en { $place }.
contacts-edit-given = Nombre
contacts-edit-family = Apellidos
contacts-edit-company = Empresa
contacts-edit-job = Cargo
contacts-edit-email = Correo electrónico
contacts-edit-phone = Teléfono
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Añadir correo electrónico
contacts-edit-add-phone = Añadir teléfono
contacts-edit-street = Dirección
contacts-edit-city = Ciudad
contacts-edit-postcode = Código postal
contacts-edit-country = País
contacts-edit-birthday = Cumpleaños (YYYY-MM-DD)
contacts-edit-empty = Añade primero un nombre, un correo electrónico o un teléfono.
