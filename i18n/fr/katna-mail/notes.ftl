# Katna Mail, French (Français): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notes
notes-view-reminders = Rappels
notes-view-archive = Archives
notes-view-trash = Corbeille
notes-edit-labels = Modifier les libellés
notes-search = Rechercher dans les notes
notes-loading = Ouverture de vos notes…

## Board

notes-take-a-note = Créer une note…
notes-new-list = Nouvelle liste
notes-new-note = Nouvelle note
notes-pinned = Épinglées
notes-others = Autres
notes-empty = Les notes que vous ajoutez s’affichent ici
notes-archive-empty = Vos notes archivées s’affichent ici
notes-trash-empty = Aucune note dans la corbeille
notes-none-found = Aucune note correspondante
notes-label-empty = Aucune note avec ce libellé pour le moment
notes-reminders-empty = Les notes avec des rappels à venir s’affichent ici
notes-trash-note = Les notes placées dans la corbeille sont supprimées au bout de 7 jours.
notes-empty-trash = Vider la corbeille
notes-ticked = { $count ->
    [one] + { $count } élément coché
    [many] + { $count } éléments cochés
   *[other] + { $count } éléments cochés
}
notes-select = Sélectionner la note
notes-selected = { $count ->
    [one] { $count } sélectionnée
    [many] { $count } de notes sélectionnées
   *[other] { $count } sélectionnées
}
notes-select-clear = Effacer la sélection

## A note's buttons

notes-pin = Épingler la note
notes-unpin = Désépingler la note
notes-archive = Archiver
notes-unarchive = Désarchiver
notes-delete = Supprimer la note
notes-restore = Restaurer
notes-delete-forever = Supprimer définitivement
notes-color = Couleur d’arrière-plan
notes-checkboxes = Afficher ou masquer les cases à cocher
notes-labels = Libellés
notes-close = Fermer
notes-more = Plus
notes-make-copy = Faire une copie
notes-remind = Me le rappeler
notes-add-picture = Ajouter une image
notes-history = Historique des versions
notes-ai = Aidez-moi à écrire
notes-send-as-mail = Envoyer par e-mail
notes-save-markdown = Enregistrer en Markdown
notes-save-pdf = Enregistrer en PDF

## The open note

notes-title = Titre
notes-edited = Modifiée : { $date }
notes-on-this-computer = Sur cet ordinateur
notes-where = Emplacement de cette note
notes-untitled = Note sans titre

## Pictures

notes-picture-choose = Ajouter des images
notes-picture-remove = Supprimer l’image
notes-picture-too-big = Une note accepte des images jusqu’à { $size }
notes-picture-kind = Ce fichier n’est pas une image que Katna peut afficher
notes-picture-unreadable = Impossible de lire { $name } : { $error }

## Reminders

notes-remind-me = Me le rappeler
notes-remind-off = Supprimer le rappel
notes-remind-in-the-past = Choisissez une heure qui n’est pas encore passée
notes-remind-today = Aujourd’hui, { $time }
notes-remind-tomorrow = Demain, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Rappel défini pour { $when }
notes-reminder-off = Rappel supprimé

## Links between notes

notes-link-note = Lier une note
notes-link-new = Nouvelle note « { $title } »
notes-linked-from = Liée depuis
notes-link-gone = Cette note n’existe plus

## Version history

notes-versions = Versions
notes-version-now = Maintenant
notes-version-here = Vous, sur cet ordinateur
notes-version-yesterday = Hier, { $time }
notes-version-changes = { $count ->
    [one] { $count } modification
    [many] { $count } de modifications
   *[other] { $count } modifications
}
notes-version-from = Depuis { $device }
notes-version-elsewhere = Depuis un autre appareil
notes-version-created = Création
notes-version-restore = Restaurer cette version
notes-version-restored = Version restaurée
notes-history-none = Aucune version antérieure pour l’instant

## AI help

notes-ai-tidy = Mettre le texte au propre
notes-ai-checklist = En faire une liste de contrôle
notes-ai-summarise = Résumer
notes-ai-empty = Écrivez d’abord quelque chose
notes-ai-tidied = Texte mis au propre. Ctrl+Z le rétablit.
notes-ai-listed = Transformé en liste de contrôle. Ctrl+Z le rétablit.
notes-ai-summarised = Résumé ajouté en haut

## Labels

notes-label-note = Ajouter un libellé à la note
notes-label-name = Saisir le nom du libellé
notes-label-create = Créer « { $name } »
notes-label-remove = Retirer le libellé
notes-label-delete = Supprimer le libellé
notes-labels-none = Aucun libellé pour le moment. Ajoutez-en un depuis le bouton de libellé d’une note.
notes-labels-done = OK
notes-label-renamed = Libellé renommé en « { $name } »
notes-label-deleted = Libellé « { $name } » supprimé

## A note about a mail

notes-mail = Courrier
notes-open-mail = Ouvrir le message
notes-open-note = Ouvrir la note

## Meeting notes

notes-meeting-take = Prendre des notes de réunion
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Participants : { $names }
notes-meeting-notes = Notes
notes-meeting-actions = Actions à mener
notes-event = Événement
notes-open-event = Ouvrir l’événement

## Formatting

notes-format = Mise en forme
notes-format-heading-1 = Titre 1
notes-format-heading-2 = Titre 2
notes-format-normal = Texte normal
notes-format-bold = Gras
notes-format-italic = Italique
notes-format-underline = Souligné
notes-format-quote = Citation
notes-format-code = Code
notes-format-divider = Séparateur
notes-format-clear = Effacer la mise en forme

## Tasks

notes-make-task = En faire une tâche

## Colors (tooltips)

notes-color-none = Aucune couleur
notes-color-coral = Corail
notes-color-peach = Pêche
notes-color-sand = Sable
notes-color-mint = Menthe
notes-color-sage = Sauge
notes-color-fog = Brume
notes-color-storm = Orage
notes-color-dusk = Crépuscule
notes-color-blossom = Fleur
notes-color-clay = Argile
notes-color-chalk = Craie

## Messages at the foot of the window

notes-archived = Note archivée
notes-unarchived = Note désarchivée
notes-trashed = Note déplacée vers la corbeille
notes-restored = Note restaurée
notes-saved = Note enregistrée
notes-pinned-count = { $count ->
    [one] Note épinglée
    [many] { $count } de notes épinglées
   *[other] { $count } notes épinglées
}
notes-unpinned-count = { $count ->
    [one] Note désépinglée
    [many] { $count } de notes désépinglées
   *[other] { $count } notes désépinglées
}
notes-colored-count = { $count ->
    [one] Couleur modifiée
    [many] Couleur modifiée sur { $count } de notes
   *[other] Couleur modifiée sur { $count } notes
}
notes-archived-count = { $count ->
    [one] Note archivée
    [many] { $count } de notes archivées
   *[other] { $count } notes archivées
}
notes-unarchived-count = { $count ->
    [one] Note désarchivée
    [many] { $count } de notes désarchivées
   *[other] { $count } notes désarchivées
}
notes-trashed-count = { $count ->
    [one] Note placée dans la corbeille
    [many] { $count } de notes placées dans la corbeille
   *[other] { $count } notes placées dans la corbeille
}
notes-restored-count = { $count ->
    [one] Note restaurée
    [many] { $count } de notes restaurées
   *[other] { $count } notes restaurées
}
notes-copied-count = { $count ->
    [one] Copie créée
    [many] { $count } de copies créées
   *[other] { $count } copies créées
}
notes-empty-discarded = Note vide supprimée
notes-mail-gone = Ce message n’existe plus
notes-deleted-forever = { $count ->
    [one] Note supprimée définitivement
    [many] { $count } notes supprimées définitivement
   *[other] { $count } notes supprimées définitivement
}
