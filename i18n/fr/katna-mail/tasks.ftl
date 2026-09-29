# Katna Mail, French (Français): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Créer
tasks-all = Toutes les tâches
tasks-today = Aujourd’hui
tasks-starred = Suivies
tasks-new-list = Créer une liste
tasks-on-this-computer = Sur cet ordinateur
tasks-my-tasks = Mes tâches
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Reconnectez-vous pour afficher les tâches
tasks-account-signed-in = Reconnecté à { $address }. Récupération de vos tâches…
tasks-account-sign-in-refused = { $provider } n’a pas laissé entrer Katna. Réessayez, et autorisez l’accès à vos tâches.
tasks-account-refused = Le serveur n’a pas accepté le mot de passe. Yahoo, iCloud, Zoho et d’autres demandent un mot de passe d’application.
tasks-account-change-password = Changer le mot de passe
tasks-account-change-password-tooltip = Ouvrir Paramètres > Comptes
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
tasks-today-empty = Aucune échéance aujourd’hui.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = En retard
tasks-completed = { $count ->
    [one] Terminées ({ $count })
    [many] Terminées ({ $count })
   *[other] Terminées ({ $count })
}
tasks-list-options = Options de la liste
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
tasks-toast-added = { $count ->
    [one] Ajoutée aux tâches
    [many] { $count } tâches ajoutées
   *[other] { $count } tâches ajoutées
}
tasks-mail-gone = Ce message n’existe plus.
tasks-toast-list-deleted = Liste supprimée
tasks-toast-moved = Tâche déplacée vers { $list }
tasks-toast-rescheduled = Tâche reprogrammée
