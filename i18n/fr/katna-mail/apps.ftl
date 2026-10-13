# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = Courrier
rail-calendar = Calendrier
rail-contacts = Contacts
rail-tasks = Tâches
rail-notes = Notes
rail-files = Fichiers

## Rail right-click menu

rail-menu-open = Ouvrir { $app }
rail-menu-settings = Paramètres de { $app }
rail-menu-turn-off = Désactiver { $app }…

## Turning an app off (Settings > Apps)

app-off-title = Désactiver { $app } ?
app-off-body = Katna arrête de synchroniser { $app } et le retire de :
app-off-keep = Garder une copie sur cet ordinateur
app-off-keep-detail = Le réactiver est instantané
app-off-remove = Supprimer la copie sur cet ordinateur
app-off-remove-detail = Rien ne change sur vos comptes, et le réactiver le télécharge à nouveau. Ce qui n’existe que sur cet ordinateur, ou n’a pas encore été envoyé, reste.
app-off-cancel = Annuler
app-off-confirm = Désactiver
app-off-done = { $app } désactivé
app-off-note = { $app } est désactivé
app-off-turn-on = Activer
app-off-leaves-calendar-rail = La barre latérale et Ctrl+2
app-off-leaves-calendar-agenda = L’agenda à côté de votre courrier
app-off-leaves-calendar-meeting = Planifier une réunion, et Ouvrir dans Calendrier sur les invitations
app-off-leaves-calendar-reminders = Les rappels d’événements
app-off-leaves-calendar-desktop = Les événements dans KRunner et l’horloge du bureau
app-off-leaves-contacts-rail = La barre latérale et Ctrl+3
app-off-leaves-contacts-card = Ajouter aux contacts sur la fiche d’un expéditeur
app-off-leaves-contacts-birthdays = Les anniversaires dans Calendrier
app-off-leaves-tasks-rail = La barre latérale et Ctrl+4
app-off-leaves-tasks-mail = Ajouter aux tâches sur les messages, et Maj+T
app-off-leaves-tasks-calendar = Les tâches dans Calendrier
app-off-leaves-tasks-tray = Nouvelle tâche dans la zone de notification, et Meta+Alt+T
app-off-leaves-tasks-reminders = Les rappels de tâches
app-off-leaves-notes-rail = La barre latérale et Ctrl+5
app-off-leaves-notes-mail = Ajouter une note sur les messages
app-off-leaves-notes-meetings = Les notes de réunion sur les événements
app-off-leaves-notes-tray = Nouvelle note dans la zone de notification, et Meta+Alt+N
app-off-leaves-notes-reminders = Les rappels de notes
app-off-leaves-files-rail = La barre latérale et Ctrl+7
app-off-leaves-files-compose = Fichiers lors de l’ajout de pièces jointes dans un nouveau message

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
