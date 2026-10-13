# Katna Mail, French (Français): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Nouvelle tâche
tasks-all = Toutes les tâches
tasks-today = Aujourd’hui
tasks-upcoming = À venir
tasks-starred = Suivies
tasks-completed-view = Terminées
tasks-new-list = Créer une liste
tasks-labels-heading = Libellés
tasks-on-this-computer = Sur cet ordinateur
tasks-my-tasks = Mes tâches
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Reconnectez-vous pour afficher les tâches
tasks-account-signed-in = Reconnecté à { $address }. Récupération de vos tâches…
tasks-account-sign-in-refused = { $provider } n’a pas laissé entrer Katna. Réessayez, et autorisez l’accès à vos tâches.
tasks-account-refused = Le serveur n’a pas accepté le mot de passe. Yahoo, iCloud, Zoho et d’autres demandent un mot de passe d’application.
tasks-account-change-password = Changer le mot de passe
tasks-account-change-password-tooltip = Saisissez le nouveau mot de passe ; Katna le vérifie auprès du serveur
tasks-account-not-enabled = L’accès aux tâches pour Katna n’est pas encore activé.
tasks-account-failed = Impossible de lire les listes de tâches.
# $reason is the server's own words, in English.
tasks-account-error = Impossible de lire les listes de tâches : { $reason }
tasks-account-none = Aucune liste de tâches trouvée
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Aucune liste de tâches trouvée: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } ne montre les tâches qu’à Katna connecté avec { $provider }.
tasks-account-sign-in-with = Se connecter avec { $provider }
tasks-account-looking = Recherche des listes de tâches…
tasks-account-try-again = Réessayer
tasks-account-try-again-tooltip = Vérifier à nouveau les tâches de ce compte maintenant
tasks-account-fixing = En cours…
tasks-list-name-placeholder = Nom de la liste

## Lists and tasks

tasks-loading = Lecture de vos tâches…
tasks-no-lists = Vos listes de tâches s’affichent ici.
tasks-search = Rechercher dans les tâches
tasks-search-none = Aucune tâche ne correspond à votre recherche.
tasks-add = Ajouter une tâche
tasks-title-placeholder = Titre
tasks-add-step = Ajouter une sous-tâche
tasks-empty = Aucune tâche pour le moment. Ajoutez-en une ci-dessus.
tasks-starred-empty = Suivez une tâche pour la voir ici.
tasks-label-empty = Aucune tâche en cours avec ce libellé.
tasks-today-empty = Aucune échéance aujourd’hui.
tasks-completed-empty = Les tâches que vous terminez s’affichent ici.
tasks-upcoming-add = Ajouter une tâche pour { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Depuis un message
tasks-from-note-quiet = Depuis une note
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = En retard
tasks-completed = { $count ->
    [one] Terminées ({ $count })
    [many] Terminées ({ $count })
   *[other] Terminées ({ $count })
}
tasks-list-options = Options de la liste
tasks-sort-by = Trier par
tasks-sort-my-order = Mon ordre
tasks-sort-date = Date
tasks-sort-starred = Suivies récemment
tasks-sort-title = Titre
tasks-rename-list = Renommer la liste
tasks-delete-list = Supprimer la liste
tasks-mark-done = Marquer comme terminée
tasks-mark-open = Marquer comme non terminée
tasks-star = Suivre
tasks-unstar = Ne plus suivre
tasks-edit-title = Modifier le titre
tasks-details = Détails
tasks-delete = Supprimer
tasks-move-to = Déplacer vers { $list }
tasks-from-mail = Courrier
tasks-open-mail = Ouvrir le message
tasks-from-note = Note
tasks-open-note = Ouvrir la note
tasks-note-gone = Cette note n’existe plus.
tasks-no-subject = (aucun objet)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } sélectionnée
    [many] { $count } de sélectionnées
   *[other] { $count } sélectionnées
}
tasks-select-clear = Effacer la sélection
tasks-select-move = Déplacer vers une liste
tasks-select-date = Définir la date
tasks-next-week = La semaine prochaine

## The details dialog

tasks-notes-placeholder = Ajouter des détails
tasks-date = Date
tasks-no-date = Aucune date
tasks-time-placeholder = Ajouter une heure
tasks-repeat = Répéter
tasks-repeat-never = Ne se répète pas
tasks-repeat-daily = Tous les jours
tasks-repeat-weekly = Toutes les semaines
tasks-repeat-monthly = Tous les mois
tasks-repeat-yearly = Tous les ans
tasks-repeat-other = Personnalisée
tasks-remind = Me le rappeler
tasks-remind-off = Ne pas rappeler
tasks-remind-on-time = À l'heure prévue
tasks-remind-morning = Le jour même, { $time }
tasks-remind-hour-before = Une heure avant
tasks-remind-day-before = La veille
tasks-label-add = Ajouter un libellé
tasks-label-task = Libeller la tâche
tasks-files-attach = Joindre des fichiers
tasks-files-pick = Joindre
tasks-file-open = Ouvrir
tasks-file-remove = Retirer le fichier
tasks-file-here = Uniquement sur cet ordinateur
tasks-cancel = Annuler
tasks-save = Enregistrer
tasks-not-a-time = « { $text } » n’est pas une heure, par exemple { $example }.

## Due days

tasks-due-today = Aujourd’hui
tasks-due-tomorrow = Demain
tasks-due-yesterday = Hier
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Tâche terminée
tasks-toast-next = Terminée. Prochaine le { $date }
tasks-toast-deleted = Tâche supprimée
tasks-files-added = { $count ->
    [one] Fichier joint
    [many] { $count } de fichiers joints
   *[other] { $count } fichiers joints
}
tasks-file-removed = « { $name } » retiré
tasks-files-left-out = Non joints : { $names }. Une tâche accepte des fichiers jusqu’à { $limit }, mais pas de dossiers.
tasks-file-missing = Ce fichier n’est plus là.
tasks-toast-added = { $count ->
    [one] Ajoutée aux tâches
    [many] { $count } tâches ajoutées
   *[other] { $count } tâches ajoutées
}
tasks-mail-gone = Ce message n’existe plus.
tasks-toast-list-deleted = Liste supprimée
tasks-toast-moved = Tâche déplacée vers { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Tâche déplacée
tasks-toast-rescheduled = Tâche reprogrammée
tasks-toast-rescheduled-several = { $count ->
    [one] Tâche replanifiée
    [many] { $count } de tâches replanifiées
   *[other] { $count } tâches replanifiées
}
tasks-toast-done-several = { $count ->
    [one] Tâche terminée
    [many] { $count } de tâches terminées
   *[other] { $count } tâches terminées
}
tasks-toast-open-several = { $count ->
    [one] Tâche marquée comme non terminée
    [many] { $count } de tâches marquées comme non terminées
   *[other] { $count } tâches marquées comme non terminées
}
tasks-toast-starred = { $count ->
    [one] Tâche suivie
    [many] { $count } de tâches suivies
   *[other] { $count } tâches suivies
}
tasks-toast-unstarred = { $count ->
    [one] Suivi retiré
    [many] Suivi retiré de { $count } de tâches
   *[other] Suivi retiré de { $count } tâches
}
tasks-toast-deleted-several = { $count ->
    [one] Tâche supprimée
    [many] { $count } de tâches supprimées
   *[other] { $count } tâches supprimées
}
