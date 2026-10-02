# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = Courrier
rail-calendar = Calendrier
rail-contacts = Contacts
rail-tasks = Tâches
rail-notes = Notes

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Bientôt disponible
app-calendar-promise = Vos agendas CalDAV, les invitations à des réunions reçues par e-mail et vos rappels, à côté de votre boîte de réception.
app-tasks-promise = Des listes de tâches synchronisées avec CalDAV, et des tâches créées à partir de messages.
app-notes-promise = Des notes rapides, et des notes sur un message ou une conversation pour plus tard.

## Contacts page

app-contacts-loading = Recherche des personnes dans vos messages…
app-contacts-empty = Les personnes avec qui vous échangez s’affichent ici.
app-contacts-count = { $count ->
    [one] { $count } personne issue de vos messages, par nombre d’échanges
    [many] { $count } de personnes issues de vos messages, par nombre d’échanges
   *[other] { $count } personnes issues de vos messages, par nombre d’échanges
}
app-contacts-top = { $count ->
    [one] La personne avec qui vous échangez le plus
    [many] Les { $count } de personnes avec qui vous échangez le plus
   *[other] Les { $count } personnes avec qui vous échangez le plus
}
app-contacts-messages = { $count ->
    [one] { $count } message
    [many] { $count } de messages
   *[other] { $count } messages
}
app-contacts-last = dernier échange : { $date }
top-brand = Katna
