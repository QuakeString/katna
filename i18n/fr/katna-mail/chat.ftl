# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Lecture
chat-view = Conversations en mode discussion
chat-view-detail = Les messages entre personnes se lisent comme une discussion de groupe : une bulle par message avec seulement ce qui a été écrit, les vôtres à droite. Les newsletters gardent l’affichage habituel.
chat-view-switch = Afficher les conversations comme des discussions
chat-view-switch-detail = Le message cité et les signatures attendent derrière ··· dans chaque bulle

chat-switch-chat = Discussion
chat-switch-mail = Message
chat-people = { $names } et vous · { $count ->
    [one] { $count } message
    [many] { $count } de messages
   *[other] { $count } messages
}
chat-people-heading = { $count ->
    [one] Dans cette discussion · { $count } personne
    [many] Dans cette discussion · { $count } de personnes
   *[other] Dans cette discussion · { $count } personnes
}
chat-member-mails = { $count ->
    [0] Aucun message
    [one] { $count } message
    [many] { $count } de messages
   *[other] { $count } messages
}
chat-today = Aujourd’hui
chat-yesterday = Hier
chat-added = { $who } a ajouté { $names }
chat-renamed = { $who } a changé l’objet en « { $subject } »
chat-you = Vous
chat-not-downloaded = Pas encore téléchargé
chat-forwarded = Transféré
chat-show-quoted = Afficher le message cité et la signature
chat-hide-quoted = Masquer le message cité et la signature
chat-hide-dots = Masquer ···
chat-show-card = Afficher sa fiche
chat-reply-all = Répondre à tous
chat-more = Plus
chat-reply-only = Répondre uniquement à { $name }
chat-forward = Transférer
chat-copy-text = Copier le texte
chat-show-as-mail = Afficher comme message
chat-go-down = Aller au message le plus récent
chat-pin = Épingler en haut
chat-pin-file = Épingler le fichier en haut
chat-unpin = Désépingler
chat-unpin-file = Désépingler le fichier
chat-pinned-of = Épinglé { $at } sur { $count }
chat-pins-all = Tous les éléments épinglés
chat-pins-heading = Épinglés · { $count } sur { $most }
chat-pins-drag = Faites glisser pour réorganiser
chat-pin-from-mail = Message de { $name } · { $when }
chat-pin-from-file = Fichier de { $name } · { $when }
chat-pin-from-text = Texte de { $name } · { $when }
chat-pins-full = Cette discussion contient déjà 5 éléments épinglés
chat-pins-replace-title = Remplacer un élément épinglé
chat-pins-replace-hint = Une discussion contient jusqu’à 5 éléments épinglés. Choisissez celui à retirer.
chat-pins-replace = Remplacer
chat-pins-cancel = Annuler
chat-undo = Annuler

chat-reply-to = Répondre à { $names }
chat-send = Envoyer (Ctrl+Entrée). Clic droit ou appui long pour plus d’options
chat-send-now = Envoyer maintenant
chat-attach = Joindre
chat-attach-photo = Photo
chat-attach-file = Fichier
chat-attach-library = Depuis Fichiers
chat-attach-template = Modèle
chat-attach-signature = Signature
chat-replying-to = Réponse à { $name }
chat-reply-newest = Répondre au message le plus récent

## The attach picker (paperclip > From Files)

picker-title = Joindre depuis Fichiers
picker-search = Rechercher des noms, des personnes, des objets
picker-search-drive = Rechercher dans ce Drive
picker-mail-files = Fichiers des messages
picker-this-chat = Cette conversation
picker-this-computer = Cet ordinateur…
picker-in-chat = DANS CETTE CONVERSATION
picker-recent = RÉCENTS
picker-preview = Aperçu
picker-cancel = Annuler
picker-attach = Joindre
picker-attach-count = Joindre { $count }
picker-selected = { $count } sélectionné(s)
picker-of-limit = sur { $limit }
picker-in-mail = { $size } dans le message
picker-drive-links = { $count ->
    [one] 1 en lien Google Drive
    [many] { $count } de fichiers en liens Google Drive
   *[other] { $count } en liens Google Drive
}
picker-onedrive-links = { $count ->
    [one] 1 en lien OneDrive
    [many] { $count } de fichiers en liens OneDrive
   *[other] { $count } en liens OneDrive
}
picker-over = { $size }, plus que les { $limit } qu’un message peut contenir
picker-getting = { $count ->
    [one] Récupération du fichier depuis le Drive…
    [many] Récupération de { $count } de fichiers depuis le Drive…
   *[other] Récupération de { $count } fichiers depuis le Drive…
}
picker-some-failed = { $count ->
    [one] Un fichier n’a pas pu être lu
    [many] { $count } de fichiers n’ont pas pu être lus
   *[other] { $count } fichiers n’ont pas pu être lus
}
