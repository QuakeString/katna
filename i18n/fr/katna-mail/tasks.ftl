# Katna Mail, French (Français): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Créer
tasks-all = Toutes les tâches
tasks-starred = Suivies
tasks-new-list = Créer une liste
tasks-on-this-computer = Sur cet ordinateur
tasks-my-tasks = Mes tâches
tasks-list-name-placeholder = Nom de la liste

## Lists and tasks

tasks-loading = Lecture de vos tâches…
tasks-no-lists = Vos listes de tâches s’affichent ici.
tasks-add = Ajouter une tâche
tasks-title-placeholder = Titre
tasks-add-step = Ajouter une sous-tâche
tasks-empty = Aucune tâche pour le moment. Ajoutez-en une ci-dessus.
tasks-starred-empty = Suivez une tâche pour la voir ici.
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
tasks-toast-deleted = Tâche supprimée
tasks-toast-added = { $count ->
    [one] Ajoutée aux tâches
    [many] { $count } tâches ajoutées
   *[other] { $count } tâches ajoutées
}
tasks-mail-gone = Ce message n’existe plus.
tasks-toast-list-deleted = Liste supprimée
tasks-toast-moved = Tâche déplacée vers { $list }
