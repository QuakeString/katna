# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Langue : { $language }
language-tooltip-system = Langue : { $language }, selon le système
language-search = Rechercher une langue
language-system-default = Langue du système
language-system-now = Actuellement : { $language }
language-no-match = Aucune langue ne correspond à « { $query } »
language-machine = Traduction automatique. Aidez-nous à l’améliorer
language-setting = Langue
language-setting-detail = Langue des menus, des boutons et des messages, et format des dates et des nombres. « Langue du système » suit le bureau.

## Dates and sizes

ago-just-now = à l’instant
ago-minutes = { $count ->
    [one] il y a { $count } minute
    [many] il y a { $count } de minutes
   *[other] il y a { $count } minutes
}
ago-hours = { $count ->
    [one] il y a { $count } heure
    [many] il y a { $count } d’heures
   *[other] il y a { $count } heures
}
ago-days = { $count ->
    [one] il y a { $count } jour
    [many] il y a { $count } de jours
   *[other] il y a { $count } jours
}
size-bytes = { $count ->
    [one] { $count } octet
    [many] { $count } d’octets
   *[other] { $count } octets
}
size-kb = { $size } Ko
size-mb = { $size } Mo
size-gb = { $size } Go
size-tb = { $size } To

## Top bar

folders-hide = Masquer les dossiers
folders-show = Afficher les dossiers
compose = Nouveau message
search = Rechercher
search-mail = Rechercher dans les messages
search-settings = Rechercher dans les paramètres
search-clear = Effacer la recherche
search-options-show = Afficher les options de recherche
settings = Paramètres
account-add = Ajouter un compte

## App rail (and the bottom bar on a phone)

rail-mail = Courrier
rail-calendar = Calendrier
rail-contacts = Contacts
rail-tasks = Tâches
rail-notes = Notes
rail-feeds = Flux

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Bientôt disponible
app-calendar-promise = Vos agendas CalDAV, les invitations à des réunions reçues par e-mail et vos rappels, à côté de votre boîte de réception.
app-tasks-promise = Des listes de tâches synchronisées avec CalDAV, et des tâches créées à partir de messages.
app-notes-promise = Des notes rapides, et des notes sur un message ou une conversation pour plus tard.
app-feeds-promise = Lisez vos flux RSS et Atom à côté de vos messages.

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

## Navigation (the folders pane)

nav-labels = Libellés
nav-folders = Dossiers
nav-label-new = Créer un libellé
nav-folder-new = Créer un dossier
nav-account-unnamed = Compte { $number }
nav-tab-new = { $count ->
    [one] { $count } nouveau
    [many] { $count } de nouveaux
   *[other] { $count } nouveaux
}

## Special folders (the user's own folders keep their names)

folder-inbox = Boîte de réception
folder-starred = Messages suivis
folder-drafts = Brouillons
folder-sent = Messages envoyés
folder-archive = Archives
folder-spam = Spam
folder-trash = Corbeille
folder-all-mail = Tous les messages
folder-scheduled = Messages programmés

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nouveau libellé
label-folder-new-title = Nouveau dossier
label-prompt = Veuillez saisir le nom du nouveau libellé :
label-folder-prompt = Veuillez saisir le nom du nouveau dossier :
label-name-hint = Nom du libellé
label-folder-name-hint = Nom du dossier
label-nest = Imbriquer le libellé sous :
label-folder-nest = Imbriquer le dossier sous :
label-cancel = Annuler
label-create = Créer
label-creating = Création…
label-created = Libellé « { $name } » créé.
label-folder-created = Dossier « { $name } » créé.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Principale
tab-promotions = Promotions
tab-social = Réseaux sociaux
tab-updates = Notifications
tab-forums = Forums
tab-focused = Prioritaire
tab-other = Autres
tab-inbox = Boîte de réception
tab-newsletters = Newsletters
tab-notifications = Notifications
tab-new = { $count ->
    [one] { $count } nouveau
    [many] { $count } de nouveaux
   *[other] { $count } nouveaux
}
tab-provider-other = tri par Katna

## Mail list: toolbar

list-select = Sélectionner
list-refresh = Actualiser
list-more = Plus
list-mark-read = Marquer comme lu
list-mark-unread = Marquer comme non lu
list-move-to = Déplacer vers
list-archive = Archiver
list-spam = Signaler comme spam
list-delete = Supprimer
list-newer = Plus récents
list-older = Plus anciens
list-range = { $first }–{ $last } sur { $total }
list-range-about = { $first }–{ $last } sur environ { $total }
list-results = Résultats pour « { $query } »
list-results-corrected = Affichage des résultats pour « { $query } »
list-search-instead = Rechercher plutôt « { $query } »
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Tous
list-pick-none = Aucun
list-pick-read = Lus
list-pick-unread = Non lus
list-pick-starred = Suivis
list-pick-unstarred = Non suivis

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversation est sélectionnée.
        [many] Les { $count } de conversations sont sélectionnées.
       *[other] Les { $count } conversations sont sélectionnées.
    }
   *[message] { $count ->
        [one] { $count } message est sélectionné.
        [many] Les { $count } de messages sont sélectionnés.
       *[other] Les { $count } messages sont sélectionnés.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversation est sélectionnée dans { $folder }.
        [many] Les { $count } de conversations dans { $folder } sont sélectionnées.
       *[other] Les { $count } conversations dans { $folder } sont sélectionnées.
    }
   *[message] { $count ->
        [one] { $count } message est sélectionné dans { $folder }.
        [many] Les { $count } de messages dans { $folder } sont sélectionnés.
       *[other] Les { $count } messages dans { $folder } sont sélectionnés.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversation est sélectionnée sur cette page.
        [many] Les { $count } de conversations de cette page sont sélectionnées.
       *[other] Les { $count } conversations de cette page sont sélectionnées.
    }
   *[message] { $count ->
        [one] { $count } message est sélectionné sur cette page.
        [many] Les { $count } de messages de cette page sont sélectionnés.
       *[other] Les { $count } messages de cette page sont sélectionnés.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Sélectionner { $count } conversation
        [many] Sélectionner les { $count } de conversations
       *[other] Sélectionner les { $count } conversations
    }
   *[message] { $count ->
        [one] Sélectionner { $count } message
        [many] Sélectionner les { $count } de messages
       *[other] Sélectionner les { $count } messages
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Sélectionner { $count } conversation dans { $folder }
        [many] Sélectionner les { $count } de conversations dans { $folder }
       *[other] Sélectionner les { $count } conversations dans { $folder }
    }
   *[message] { $count ->
        [one] Sélectionner { $count } message dans { $folder }
        [many] Sélectionner les { $count } de messages dans { $folder }
       *[other] Sélectionner les { $count } messages dans { $folder }
    }
}
list-clear-selection = Effacer la sélection

## Mail list: empty states

list-empty-search = Aucun message ne correspond à votre recherche.
list-empty-tab = Aucun message dans { $tab }.
list-empty-tab-unknown = Aucun message dans cet onglet.
list-empty-folder = Aucun message dans { $folder }.
list-empty-folder-unknown = Aucun message dans ce dossier.
list-first-sync = Récupération de vos messages…
list-first-sync-detail = Ils s’affichent ici au fur et à mesure de leur arrivée.

## Mail list: lines

row-removed = Ce message a été supprimé.
row-starred = Suivi
row-not-starred = Non suivi
row-important = Important. Cliquez pour le marquer comme non important.
row-mark-important = Marquer comme important
row-pinned = Épinglé en haut
row-pin = Épingler en haut
row-unpin = Désépingler

## Mail list: More menu and right-click menu

menu-reply = Répondre
menu-reply-all = Répondre à tous
menu-forward = Transférer
menu-archive = Archiver
menu-delete = Supprimer
menu-spam = Signaler comme spam
menu-mark-read = Marquer comme lu
menu-mark-unread = Marquer comme non lu
menu-mark-all-read = Tout marquer comme lu
menu-star = Ajouter le suivi
menu-unstar = Supprimer le suivi
menu-important = Marquer comme important
menu-not-important = Marquer comme non important
menu-pin = Épingler en haut
menu-unpin = Désépingler
menu-print-all = Tout imprimer
menu-new-window = Ouvrir dans une nouvelle fenêtre
menu-move-to = Déplacer vers
menu-move-to-heading = Déplacer vers :
menu-find-from = Rechercher les e-mails de { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Conversation archivée.
        [many] { $count } de conversations archivées.
       *[other] { $count } conversations archivées.
    }
   *[message] { $count ->
        [one] Message archivé.
        [many] { $count } de messages archivés.
       *[other] { $count } messages archivés.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Conversation placée dans la corbeille.
        [many] { $count } de conversations placées dans la corbeille.
       *[other] { $count } conversations placées dans la corbeille.
    }
   *[message] { $count ->
        [one] Message placé dans la corbeille.
        [many] { $count } de messages placés dans la corbeille.
       *[other] { $count } messages placés dans la corbeille.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Conversation déplacée.
        [many] { $count } de conversations déplacées.
       *[other] { $count } conversations déplacées.
    }
   *[message] { $count ->
        [one] Message déplacé.
        [many] { $count } de messages déplacés.
       *[other] { $count } messages déplacés.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Conversation suivie.
        [many] { $count } de conversations suivies.
       *[other] { $count } conversations suivies.
    }
   *[message] { $count ->
        [one] Message suivi.
        [many] { $count } de messages suivis.
       *[other] { $count } messages suivis.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Suivi retiré de la conversation.
        [many] Suivi retiré de { $count } de conversations.
       *[other] Suivi retiré de { $count } conversations.
    }
   *[message] { $count ->
        [one] Suivi retiré du message.
        [many] Suivi retiré de { $count } de messages.
       *[other] Suivi retiré de { $count } messages.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Conversation marquée comme importante.
        [many] { $count } de conversations marquées comme importantes.
       *[other] { $count } conversations marquées comme importantes.
    }
   *[message] { $count ->
        [one] Message marqué comme important.
        [many] { $count } de messages marqués comme importants.
       *[other] { $count } messages marqués comme importants.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Conversation marquée comme non importante.
        [many] { $count } de conversations marquées comme non importantes.
       *[other] { $count } conversations marquées comme non importantes.
    }
   *[message] { $count ->
        [one] Message marqué comme non important.
        [many] { $count } de messages marqués comme non importants.
       *[other] { $count } messages marqués comme non importants.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Conversation épinglée en haut.
        [many] { $count } de conversations épinglées en haut.
       *[other] { $count } conversations épinglées en haut.
    }
   *[message] { $count ->
        [one] Message épinglé en haut.
        [many] { $count } de messages épinglés en haut.
       *[other] { $count } messages épinglés en haut.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Conversation désépinglée.
        [many] { $count } de conversations désépinglées.
       *[other] { $count } conversations désépinglées.
    }
   *[message] { $count ->
        [one] Message désépinglé.
        [many] { $count } de messages désépinglés.
       *[other] { $count } messages désépinglés.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversation signalée comme spam.
        [many] { $count } de conversations signalées comme spam.
       *[other] { $count } conversations signalées comme spam.
    }
   *[message] { $count ->
        [one] Message signalé comme spam.
        [many] { $count } de messages signalés comme spam.
       *[other] { $count } messages signalés comme spam.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Conversation supprimée définitivement.
        [many] { $count } de conversations supprimées définitivement.
       *[other] { $count } conversations supprimées définitivement.
    }
   *[message] { $count ->
        [one] Message supprimé définitivement.
        [many] { $count } de messages supprimés définitivement.
       *[other] { $count } messages supprimés définitivement.
    }
}
toast-undone = Action annulée.
toast-undo = Annuler
toast-no-spam-folder = Ce compte n’a pas de dossier de spam.

## Reading pane: toolbar

reader-close = Fermer
reader-back = Retour
reader-mark-unread = Marquer comme non lu
reader-move-to = Déplacer vers
reader-more = Plus
reader-print-all = Tout imprimer
reader-new-window = Dans une nouvelle fenêtre
reader-position = { $position } sur { $total }
reader-newer = Plus récent
reader-older = Plus ancien

## Reading pane: the conversation

reader-removed = Cette conversation a été supprimée.
reader-no-subject = (aucun objet)
reader-collapse-all = Tout réduire
reader-expand-all = Tout développer
reader-unknown-sender = (expéditeur inconnu)
reader-date-ago = { $date } ({ $ago })
reader-me = moi
reader-to = à { $names }
reader-starred = Suivi
reader-not-starred = Non suivi
reader-too-long = Le message est trop long pour être affiché en entier.
reader-encrypted-images = Les images du Web ne sont jamais chargées dans les messages chiffrés.
reader-window-failed = Impossible d’ouvrir une nouvelle fenêtre.

## Reading pane: message details (opened from "to me")

reader-details-from = de :
reader-details-to = à :
reader-details-cc = cc :
reader-details-date = date :
reader-details-subject = objet :

## Reading pane: downloading a message

reader-downloading = Téléchargement de ce message depuis le serveur…
reader-download-failed = Impossible de télécharger ce message.
reader-try-again = Réessayer

## Reply row

reply-reply = Répondre
reply-reply-all = Répondre à tous
reply-forward = Transférer

## Encrypted and signed mail

security-decrypting = Déchiffrement…
security-checking = Vérification de la signature…
security-partly-encrypted = Seule une partie de ce message est chiffrée. Le reste a été ajouté hors de cette protection et pourrait provenir de n’importe qui.
security-partly-signed = Seule une partie de ce message est signée. Le reste a été ajouté hors de cette protection et pourrait provenir de n’importe qui.
security-encrypted = Message chiffré
security-encrypted-smime = Message chiffré (S/MIME)
security-no-key = Impossible de déchiffrer ce message : il a été chiffré pour une clé que vous n’avez pas.
security-cancelled = Le déchiffrement a été annulé.
security-damaged = Impossible de déchiffrer ce message : les données chiffrées sont endommagées ou ont été modifiées.
security-decrypt-unavailable = Impossible de déchiffrer ce message : installez { $tool } pour lire les messages chiffrés.
security-decrypt-failed = Impossible de déchiffrer ce message : { $reason }
security-unknown-signer = un signataire inconnu
security-signed-verified = Signé par { $signer } · vérifié
security-signed-not-sender = Signé par { $signer }, qui n’est pas l’expéditeur
security-signed-untrusted = Signé par { $signer }, avec une clé que vous avez marquée comme non fiable
security-signed-unverified = Signé par { $signer } · la clé n’est pas vérifiée
security-bad-signature = Signature non valide : ce message a été modifié après avoir été signé, ou la signature est falsifiée.
security-signature-expired = Signé par { $signer } · la signature a expiré
security-key-expired = Signé par { $signer } · la clé a expiré depuis
security-key-revoked = Signé par { $signer } avec une clé qui a été révoquée
security-missing-key = Signé avec une clé que vous n’avez pas ; la signature ne peut donc pas être vérifiée
security-missing-key-id = Signé avec une clé que vous n’avez pas ({ $key }) ; la signature ne peut donc pas être vérifiée
security-signature-unavailable = Signé ; installez { $tool } pour vérifier la signature
security-signature-error = La signature n’a pas pu être vérifiée.

## Remote images and pictures

remote-hidden = Les images de ce message sont masquées.
remote-show = Afficher les images
remote-always-show = Toujours afficher pour cet expéditeur
remote-picture-use = Utiliser
remote-picture-too-big = Choisissez une image de 8 Mo maximum.
remote-picture-type = Choisissez une image PNG, JPEG, GIF, WebP ou SVG.
remote-picture-read-failed = Impossible de lire l’image : { $error }
remote-picture-keep-failed = Impossible de conserver l’image : { $error }
remote-picture-remove-failed = Impossible de supprimer l’image : { $error }

## Attachments

attachment-count = { $count ->
    [one] Une pièce jointe
    [many] { $count } de pièces jointes
   *[other] { $count } pièces jointes
}
attachment-save = Enregistrer
attachment-save-all = Tout enregistrer
attachment-save-all-tooltip = Enregistrer toutes les pièces jointes dans un dossier
attachment-save-here = Enregistrer ici
attachment-not-downloaded = Ce message n’est pas téléchargé.
attachment-not-found = Cette pièce jointe est introuvable dans le message.
attachment-read-failed = Impossible de lire { $name }
attachment-numbered = pièce jointe { $number }
attachment-saved-all = { $count ->
    [one] { $count } fichier enregistré dans { $place }
    [many] { $count } de fichiers enregistrés dans { $place }
   *[other] { $count } fichiers enregistrés dans { $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } fichier sur { $total } enregistré dans { $place }. Impossible d’enregistrer { $failed }
    [many] { $saved } fichiers sur { $total } enregistrés dans { $place }. Impossible d’enregistrer { $failed }
   *[other] { $saved } fichiers sur { $total } enregistrés dans { $place }. Impossible d’enregistrer { $failed }
}
attachment-saved-to = Enregistré dans { $path }
attachment-save-failed = Impossible d’enregistrer { $name } : { $error }
attachment-open-failed = Impossible d’ouvrir { $name } : { $error }
attachment-risky = Ce fichier pourrait exécuter un programme, Katna ne l’ouvre donc pas. Enregistrez-le plutôt.
attachment-encrypted-open = Ce fichier a été reçu chiffré. Enregistrez-le pour l’ouvrir ailleurs.

## Printing

print-failed = Impossible d’imprimer : { $error }
print-no-font = aucune police trouvée
print-opened-as-pdf = Ouvert au format PDF pour être imprimé depuis celui-ci.
print-not-downloaded = (Pas encore téléchargé.)
print-encrypted = (Chiffré. Ouvrez-le dans Katna Mail pour imprimer son texte.)
print-to = À : { $addresses }
print-cc = Cc : { $addresses }
