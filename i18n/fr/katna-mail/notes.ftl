# Katna Mail, French (Français): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notes
notes-view-archive = Archives
notes-view-trash = Corbeille
notes-search = Rechercher dans les notes
notes-loading = Ouverture de vos notes…

## Board

notes-take-a-note = Créer une note…
notes-new-list = Nouvelle liste
notes-pinned = Épinglées
notes-others = Autres
notes-empty = Les notes que vous ajoutez s’affichent ici
notes-archive-empty = Vos notes archivées s’affichent ici
notes-trash-empty = Aucune note dans la corbeille
notes-none-found = Aucune note correspondante
notes-trash-note = Les notes placées dans la corbeille sont supprimées au bout de 7 jours.
notes-empty-trash = Vider la corbeille
notes-ticked = { $count ->
    [one] + { $count } élément coché
    [many] + { $count } éléments cochés
   *[other] + { $count } éléments cochés
}

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
notes-close = Fermer

## The open note

notes-title = Titre
notes-edited = Modifiée : { $date }
notes-on-this-computer = Sur cet ordinateur
notes-where = Emplacement de cette note

## A note about a mail

notes-mail = Courrier
notes-open-mail = Ouvrir le message
notes-open-note = Ouvrir la note

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
notes-empty-discarded = Note vide supprimée
notes-mail-gone = Ce message n’existe plus
notes-deleted-forever = { $count ->
    [one] Note supprimée définitivement
    [many] { $count } notes supprimées définitivement
   *[other] { $count } notes supprimées définitivement
}
