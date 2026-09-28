# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Opções de pesquisa
search-options-close = Fechar
search-from = De
search-to = Para
search-subject = Assunto
search-has-words = Contém as palavras
search-without = Não contém
search-date-within = Data nos últimos
search-has-attachment = Com anexo
search-attachment-custom = Personalizado
search-attachment-custom-hint = Digite uma extensão, como png, e depois Espaço
search-attachment-remove = Remover
search-clear-filter = Limpar filtro

## Search options: "Date within" choices

search-within-any = Qualquer data
search-within-days = { $count ->
    [one] { $count } dia
    [many] { $count } de dias
   *[other] { $count } dias
}
search-within-weeks = { $count ->
    [one] { $count } semana
    [many] { $count } de semanas
   *[other] { $count } semanas
}
search-within-months = { $count ->
    [one] { $count } mês
    [many] { $count } de meses
   *[other] { $count } meses
}
search-within-years = { $count ->
    [one] { $count } ano
    [many] { $count } de anos
   *[other] { $count } anos
}
search-within-custom = Personalizado

## Search options: custom dates (the calendar popover)

search-dates-on = Em
search-dates-before = Antes de
search-dates-since = Desde
search-dates-between = Entre
search-dates-from = De
search-dates-to = Até
search-dates-placeholder = AAAA-MM-DD
search-dates-missing = Escolha uma data
search-dates-unreadable = Use uma data como 2026-09-01
search-dates-out-of-range = Essa data está fora do intervalo
search-dates-chip-before = Antes de { $date }
search-dates-chip-since = Desde { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Cancelar
search-dates-done = Concluir
search-dates-month-back = Mês anterior
search-dates-month-on = Próximo mês
search-dates-year-back = Ano anterior
search-dates-year-on = Próximo ano
