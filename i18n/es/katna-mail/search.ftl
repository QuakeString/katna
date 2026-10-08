# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Opciones de búsqueda
search-options-close = Cerrar
search-from = De
search-to = Para
search-subject = Asunto
search-has-words = Contiene las palabras
search-without = No contiene
search-date-within = Periodo
search-has-attachment = Con archivo adjunto
search-attachment-custom = Personalizado
search-attachment-image = Imagen
search-attachment-custom-hint = Escribe una extensión, como png, y pulsa Espacio
search-attachment-remove = Quitar
search-clear-filter = Borrar filtro

## Search options: "Date within" choices

search-within-any = Cualquier fecha
search-within-days = { $count ->
    [one] { $count } día
    [many] { $count } de días
   *[other] { $count } días
}
search-within-weeks = { $count ->
    [one] { $count } semana
    [many] { $count } de semanas
   *[other] { $count } semanas
}
search-within-months = { $count ->
    [one] { $count } mes
    [many] { $count } de meses
   *[other] { $count } meses
}
search-within-years = { $count ->
    [one] { $count } año
    [many] { $count } de años
   *[other] { $count } años
}
search-within-custom = Personalizado

## Search options: custom dates (the calendar popover)

search-dates-on = El
search-dates-before = Antes
search-dates-since = Desde
search-dates-between = Entre
search-dates-from = Inicio
search-dates-to = Fin
search-dates-placeholder = AAAA-MM-DD
search-dates-missing = Elige una fecha
search-dates-unreadable = Usa una fecha como 2026-09-01
search-dates-out-of-range = Esa fecha está fuera del intervalo
search-dates-chip-before = Antes del { $date }
search-dates-chip-since = Desde el { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Cancelar
search-dates-done = Hecho
search-dates-month-back = Mes anterior
search-dates-month-on = Mes siguiente
search-dates-year-back = Año anterior
search-dates-year-on = Año siguiente
