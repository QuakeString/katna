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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Ouvrez ce message pour lire ses pièces jointes.
text-copy = Copier
text-select-all = Tout sélectionner

## Settings page: its tabs

settings-tab-general = Général
settings-tab-inbox = Boîte de réception
settings-tab-accounts = Comptes
settings-tab-subscriptions = Abonnements
settings-tab-appearance = Apparence
settings-tab-shortcuts = Raccourcis
settings-tab-default-apps = Applications par défaut
settings-tab-folders-rules = Dossiers et règles
settings-tab-compose = Rédaction
settings-tab-mcp-server = Serveur MCP
settings-tab-feedback = Retours des utilisateurs
settings-tab-experimental = Expérimental

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Consultez les newsletters et les listes de diffusion que vous recevez, et désabonnez-vous en un clic.
settings-tab-folders-rules-coming = Créez, renommez, déplacez et masquez des dossiers et des libellés, et choisissez ceux qui sont synchronisés. Les règles trient, libellent, transfèrent ou suppriment automatiquement les nouveaux messages, selon l’expéditeur, l’objet ou des mots.
settings-tab-mcp-server-coming = Permettez aux assistants IA de cet ordinateur de rechercher, lire et rédiger vos messages, avec votre accord.

## Settings > General

settings-general-conversations = Mode Conversation
settings-general-conversations-group = Regrouper les réponses à un même message
settings-general-conversations-group-detail = Une ligne par conversation dans la liste
settings-general-reading = Lecture
settings-general-newest-first = Message le plus récent en premier
settings-general-newest-first-detail = Une conversation commence par sa dernière réponse
settings-general-full-headers = Afficher les en-têtes complets
settings-general-full-headers-detail = De, à, cc, date et objet affichés pour chaque message
settings-general-full-names = Noms complets des destinataires
settings-general-full-names-detail = « à moi, Ada Lovelace » plutôt que « à moi, Ada »
settings-general-mark-read = Marquer comme lu
settings-general-mark-read-now = Dès l’ouverture
settings-general-mark-read-1s = Après 1 seconde d’ouverture
settings-general-mark-read-3s = Après 3 secondes d’ouverture
settings-general-mark-read-never = Uniquement manuellement
settings-general-reply-button = Bouton Répondre
settings-general-reply-all = Répondre à tous
settings-general-reply-all-detail = Le bouton de réponse à côté de chaque message répond à tous, pas seulement à l’expéditeur
settings-general-remote-images = Images provenant du Web
settings-general-remote-images-detail = Charger les images d’un message indique à son expéditeur que vous l’avez ouvert, quand et à peu près où. Si l’option est désactivée, chaque message vous demande d’abord, et vous pouvez toujours afficher les images d’un expéditeur.
settings-general-remote-images-always = Toujours afficher les images
settings-general-remote-images-always-detail = Dans tous les messages, pas seulement ceux des expéditeurs de confiance
settings-general-sending = Envoi
settings-general-sending-detail = Délai pendant lequel un message envoyé patiente, pour pouvoir être annulé.
settings-general-offline = Messages hors connexion
settings-general-offline-detail = Les messages récents sont téléchargés en entier pour être lus sans connexion. Les plus anciens sont téléchargés à l’ouverture.
settings-general-offline-days = { $count ->
    [one] { $count } jour
    [many] { $count } de jours
   *[other] { $count } jours
}
settings-general-offline-years = { $count ->
    [one] { $count } an
    [many] { $count } d’années
   *[other] { $count } ans
}
settings-general-offline-all = Tous les messages
settings-general-offline-note = Choisir moins de jours conserve les messages déjà téléchargés. Rien ne change sur le serveur.
settings-general-notifications = Notifications
settings-general-notifications-detail = Pour les nouveaux messages de la boîte de réception, même lorsque Katna Mail est fermé.
settings-general-new-mail = M’avertir des nouveaux messages
settings-general-new-mail-detail = Avec Répondre à tous, Marquer comme lu et Archiver
settings-general-new-mail-sound = Émettre un son
settings-general-new-mail-sound-detail = Le son de nouveau message du bureau
settings-general-desktop = Bureau
settings-general-open-at-login = Ouvrir Katna Mail à la connexion
settings-general-open-at-login-detail = Les messages sont de toute façon synchronisés à la connexion, tant que le service fonctionne
settings-general-tray = Afficher Katna dans la zone de notification
settings-general-tray-detail = Avec le nombre de messages non lus et un menu
settings-general-unread-badge = Nombre de non-lus sur l’icône de la barre des tâches
settings-general-unread-badge-detail = Nombre de messages non lus dans la boîte de réception

## Settings > Inbox

settings-inbox-tabs = Onglets de la boîte de réception
settings-inbox-tabs-detail = Triez la boîte de réception en onglets, comme le fait le site Web de votre fournisseur de messagerie.
settings-inbox-tabs-show = Afficher les onglets de la boîte de réception
settings-inbox-tabs-show-detail = Désactivé, une seule liste pour tous les comptes
settings-inbox-no-accounts = Ajoutez un compte pour choisir ses onglets.
settings-inbox-tabs-automatic = Automatique : { $tabs } ({ $provider })
settings-inbox-tabs-off = Aucun onglet
settings-inbox-tabs-gmail = Principale, Promotions, Réseaux sociaux, Notifications, Forums
settings-inbox-tabs-focused = Prioritaire et Autres
settings-inbox-tabs-zoho = Boîte de réception, Newsletters et Notifications
settings-inbox-tabs-shown = Onglets affichés. Les messages d’un onglet désactivé restent dans { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Volet de lecture
settings-appearance-reading-pane-detail = Emplacement d’une conversation ouverte.
settings-appearance-pane-right = À droite de la liste
settings-appearance-pane-none = Aucune séparation
settings-appearance-density = Densité
settings-appearance-density-default = Par défaut
settings-appearance-density-compact = Compacte
settings-appearance-scaling = Mise à l’échelle
settings-appearance-scaling-detail = Agrandit ou réduit tout dans Katna Mail, en plus de l’échelle du bureau : texte, icônes, espacements et séparateurs. Les messages que vous envoyez gardent leur propre taille de police. Avec de très petites tailles, il peut être difficile de cliquer sur les icônes.
settings-appearance-theme = Thème
settings-appearance-theme-system = Comme le bureau
settings-appearance-theme-light = Clair
settings-appearance-theme-dark = Sombre
settings-appearance-desktop-colors = Couleurs du bureau
settings-appearance-desktop-colors-use = Utiliser les couleurs du bureau
settings-appearance-desktop-colors-use-detail = Le jeu de couleurs et la couleur d’accentuation du bureau
settings-appearance-app-names = Noms des applications
settings-appearance-app-names-show = Afficher le nom des applications
settings-appearance-app-names-show-detail = Noms sous les icônes des applications, tout à gauche
settings-appearance-sender-pictures = Photos des expéditeurs
settings-appearance-sender-pictures-show = Afficher les logos des entreprises
settings-appearance-sender-pictures-show-detail = Recherchés d’après le domaine de l’expéditeur, jamais d’après le message, et conservés une semaine
settings-appearance-important = Marqueurs d’importance
settings-appearance-important-show = Afficher les marqueurs d’importance
settings-appearance-important-show-detail = À côté de chaque message de la liste
settings-appearance-message-width = Largeur des messages
settings-appearance-message-width-limit = Limiter la largeur des messages
settings-appearance-message-width-limit-detail = Les lignes longues sont plus faciles à lire dans une fenêtre large
settings-appearance-mail-colors = Couleurs des messages
settings-appearance-mail-colors-detail = La plupart des messages sont conçus pour une page blanche. Avec un thème sombre, leurs couleurs sont remplacées par des couleurs sombres bien lisibles ; si l’option est désactivée, ils gardent les couleurs de l’expéditeur sur une page claire.
settings-appearance-dark-mail = Couleurs sombres aussi pour les messages
settings-appearance-dark-mail-detail = Uniquement avec le thème sombre
settings-appearance-attachment-previews = Aperçus des pièces jointes
settings-appearance-attachment-previews-show = Afficher un aperçu des pièces jointes
settings-appearance-attachment-previews-show-detail = Une miniature du contenu de chaque fichier sur sa fiche

## Settings > Default apps

settings-default-apps-intro = Où s’ouvrent les pièces jointes quand vous cliquez dessus. La visionneuse peut aussi toujours ouvrir un fichier dans une autre application. Les applications par défaut du bureau se règlent dans ses propres paramètres.
settings-default-apps-pdf = Fichiers PDF
settings-default-apps-pdf-detail = Pages, avec zoom.
settings-default-apps-pictures = Images
settings-default-apps-pictures-detail = Photos (redressées), PNG, GIF, WebP, BMP, TIFF et SVG.
settings-default-apps-text = Fichiers texte
settings-default-apps-text-detail = Texte brut, journaux, code et autres textes.
settings-default-apps-sheets = Feuilles de calcul
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) et CSV.
settings-default-apps-documents = Documents
settings-default-apps-documents-detail = Word (docx) et texte OpenDocument (odt).
settings-default-apps-katna = Visionneuse de Katna Mail
settings-default-apps-system = Application par défaut du bureau
settings-default-apps-ask = Demander l’application à chaque fois
settings-default-apps-after-saving = Après l’enregistrement
settings-default-apps-show-folder = Afficher les fichiers enregistrés dans leur dossier
settings-default-apps-show-folder-detail = Ouvre le gestionnaire de fichiers avec les pièces jointes enregistrées sélectionnées

## Settings > Compose

settings-compose-send-from = Envoyer les nouveaux messages depuis
settings-compose-send-from-detail = Les réponses et les transferts partent toujours du compte dans lequel vous vous trouvez.
settings-compose-send-from-current = Le compte dans lequel vous êtes
settings-compose-send-on-replies = Envoi des réponses
settings-compose-send-on-replies-detail = Ce que fait Envoyer pour une réponse ou un transfert. Le menu à côté d’Envoyer propose l’autre option.
settings-compose-send-plain = Envoyer
settings-compose-send-archive = Envoyer et archiver
settings-compose-signatures = Signatures
settings-compose-signatures-detail = Ajoutée sous votre message, après une ligne « -- ». Choisissez-en une autre dans la fenêtre de rédaction.
settings-compose-untitled = Sans titre
settings-compose-signature-name = Nom, par exemple Travail
settings-compose-signature-first = Ma signature
settings-compose-signature-numbered = Signature { $number }
settings-compose-signature-delete = Supprimer
settings-compose-signature-deleted = Signature supprimée
settings-compose-signature-new = Créer
settings-compose-no-signatures = Aucune signature pour l’instant.
settings-compose-no-signature = Aucune signature
settings-compose-for-new-mail = Pour les nouveaux messages
settings-compose-for-replies = Pour les réponses et les transferts
settings-compose-for-replies-detail = Dans une conversation où vous avez signé un message, une réponse commence plutôt par cette signature.
settings-compose-format = Format
settings-compose-plain-text = Écrire en texte brut
settings-compose-plain-text-detail = Les nouveaux messages commencent sans mise en forme ; la fenêtre de rédaction permet d’en changer
settings-compose-spelling = Orthographe
settings-compose-spell-check = Vérifier l’orthographe pendant la saisie
settings-compose-spell-check-detail = Les mots mal orthographiés sont soulignés, avec des suggestions par clic droit
settings-compose-spell-desktop = Langue du bureau ({ $language })
settings-compose-templates = Modèles
settings-compose-templates-detail = Enregistrez les messages que vous écrivez souvent, et partez-en pour un nouveau message ou une réponse.

## Settings > Shortcuts

settings-shortcuts-set = Jeu de raccourcis
settings-shortcuts-set-detail = Partez des touches d’une application de messagerie que vous connaissez. Ici, Cmd correspond à Ctrl. Vos propres modifications s’appliquent par-dessus le jeu, et Rétablir les valeurs par défaut revient aux touches du jeu.
settings-shortcuts-single = Raccourcis à une touche
settings-shortcuts-single-detail = Des touches sans Ctrl ni Alt, comme dans un webmail : e archive, j et k déplacent, / recherche. Ils fonctionnent dans la liste et dans la conversation ouverte, jamais pendant la saisie.
settings-shortcuts-single-use = Utiliser les raccourcis à une touche
settings-shortcuts-single-use-detail = Les raccourcis avec Ctrl fonctionnent toujours
settings-shortcuts-how = Cliquez sur une touche pour la modifier, ou sur + pour en ajouter une, puis appuyez sur les nouvelles touches. Échap annule.
settings-shortcuts-restore = Rétablir les valeurs par défaut
settings-shortcuts-no-key = Aucune touche
settings-shortcuts-press = Appuyez sur des touches…
settings-shortcuts-then = { $keys } puis…
settings-shortcuts-moved = { $keys } effectue désormais « { $action } » au lieu de « { $previous } ».
settings-shortcuts-single-off = Les raccourcis à une touche sont désactivés ; cette touche fonctionnera une fois qu’ils seront activés.
settings-shortcuts-restored = Tous les raccourcis ont retrouvé les touches de leur jeu.

## Settings search: the line under a result

settings-general-language-summary = Langue de l’application, des dates et des nombres
settings-general-reading-summary = Message le plus récent en premier, en-têtes complets, noms complets des destinataires
settings-general-mark-read-summary = Quand une conversation ouverte est marquée comme lue : immédiatement, après 1 ou 3 secondes, ou manuellement
settings-general-reply-button-summary = Le bouton de réponse à côté de chaque message répond à tous
settings-general-remote-images-summary = Toujours afficher les images de tous les messages
settings-general-sending-summary = Annuler l’envoi : délai pendant lequel un message envoyé patiente, pour pouvoir être annulé
settings-general-offline-summary = Nombre de jours de messages récents téléchargés en entier, pour les lire sans connexion
settings-general-notifications-summary = Notifications de nouveaux messages et leur son
settings-general-desktop-summary = Ouvrir Katna Mail à la connexion, l’icône de la zone de notification et le nombre de non-lus sur l’icône de la barre des tâches
settings-accounts-accounts-summary = Ajouter ou supprimer un compte, ou changer sa photo
settings-appearance-density-summary = Lignes par défaut ou compactes dans la liste
settings-appearance-scaling-summary = Tout agrandir ou réduire : texte, icônes, espacements et séparateurs
settings-appearance-theme-summary = Comme le bureau, clair ou sombre
settings-appearance-sender-pictures-summary = Logos des entreprises, recherchés d’après le domaine de l’expéditeur
settings-appearance-important-summary = Le marqueur d’importance à côté de chaque message de la liste
settings-appearance-mail-colors-summary = Couleurs sombres pour les messages HTML avec un thème sombre, ou couleurs de l’expéditeur
settings-appearance-attachment-previews-summary = Une miniature du contenu de chaque pièce jointe
settings-shortcuts-set-summary = Partir des touches de Gmail, Inbox by Gmail, Apple Mail, Outlook ou Thunderbird
settings-shortcuts-single-summary = Des touches sans Ctrl ni Alt, comme dans un webmail
settings-default-apps-pdf-summary = Où s’ouvrent les pièces jointes PDF
settings-default-apps-pictures-summary = Où s’ouvrent les photos et les images
settings-default-apps-text-summary = Où s’ouvrent le texte brut, les journaux et le code
settings-default-apps-sheets-summary = Où s’ouvrent les fichiers Excel, OpenDocument et CSV
settings-default-apps-documents-summary = Où s’ouvrent les textes Word et OpenDocument
settings-default-apps-after-saving-summary = Afficher les pièces jointes enregistrées dans leur dossier
settings-compose-send-from-summary = Le compte d’où partent les nouveaux messages : celui dans lequel vous êtes, ou toujours le même
settings-compose-send-on-replies-summary = Envoyer, ou Envoyer et archiver la conversation, pour les réponses et les transferts
settings-compose-signatures-summary = Ajoutée sous votre message, après une ligne « -- »
settings-compose-for-new-mail-summary = La signature par laquelle commencent les nouveaux messages
settings-compose-for-replies-summary = La signature par laquelle commencent les réponses et les transferts
settings-compose-format-summary = Écrire les nouveaux messages en texte brut
settings-compose-spelling-summary = Vérifier l’orthographe pendant la saisie, et la langue du dictionnaire
settings-compose-templates-summary = Bientôt : enregistrer les messages que vous écrivez souvent, et en partir pour un nouveau message ou une réponse
settings-feedback-crash-reports-summary = Enregistrer des rapports de plantage sur cet ordinateur quand Katna Mail ou son service d’arrière-plan plante
settings-feedback-saved-summary = Afficher, copier ou supprimer les rapports de plantage enregistrés sur cet ordinateur
settings-feedback-help-improve-summary = Envoyer les rapports de plantage pour aider à corriger le problème ; désactivé sauf si vous l’activez
settings-experimental-blur-summary = Le bureau transparaît, flouté, à travers la barre supérieure, et les menus sont en verre dépoli
settings-search-shortcut = Raccourci clavier
settings-search-tab = Onglet des paramètres
settings-search-none = Aucun paramètre ne correspond à « { $query } ».
settings-search-results = Paramètres correspondant à « { $query } »

## Quick settings (the panel that slides in from the right)

quick-title = Paramètres rapides
quick-see-all = Voir tous les paramètres
quick-reading-pane = Volet de lecture
quick-pane-right = À droite de la liste
quick-pane-none = Aucune séparation
quick-density = Densité
quick-density-default = Par défaut
quick-density-compact = Compacte
quick-theme = Thème
quick-theme-system = Comme le bureau
quick-theme-light = Clair
quick-theme-dark = Sombre
quick-desktop-colors = Couleurs du bureau
quick-desktop-colors-detail = Le jeu de couleurs et la couleur d’accentuation du bureau
quick-app-names = Noms des applications
quick-app-names-detail = Noms sous les icônes des applications, tout à gauche
quick-inbox-tabs = Onglets de la boîte de réception
quick-inbox-tabs-detail = Les onglets du fournisseur de messagerie de chaque compte
quick-choose-tabs = Choisir les onglets
quick-choose-tabs-detail = Par compte, dans les paramètres
quick-sending = Envoi
quick-undo-send = Annuler l’envoi
quick-undo-send-off = Désactivé
quick-undo-send-seconds = { $seconds } s
quick-signatures = Signatures
quick-signatures-none = Aucune pour l’instant
quick-signatures-one = { $name }, utilisée par défaut
quick-signatures-many = { $count ->
    [one] { $count } signature ; { $name } par défaut
    [many] { $count } de signatures ; { $name } par défaut
   *[other] { $count } signatures ; { $name } par défaut
}
quick-signatures-no-default = { $count ->
    [one] { $count }, aucune par défaut
    [many] { $count }, aucune par défaut
   *[other] { $count }, aucune par défaut
}
quick-signature-untitled = Sans titre
quick-threading = Fils de discussion
quick-conversation-view = Mode Conversation
quick-conversation-view-detail = Regrouper les réponses à un même message
quick-help = Aide
quick-tour = Faire la visite guidée
quick-whats-new = Nouveautés
quick-about = À propos de Katna

## Settings: opening at login

settings-open-at-login-failed = Impossible de modifier l’ouverture à la connexion : { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent } %
scale-reset = Revenir à { $percent } %

## Settings > Experimental > Look & Feel

look-intro = Des fonctionnalités encore à l’essai. Elles peuvent changer ou disparaître.
look-heading = Apparence et style
look-window-frame = Cadre de la fenêtre
look-window-frame-detail = Qui dessine la barre de titre, les boutons de la fenêtre, les coins et l’ombre.
look-frame-native-kde = Natif : le cadre de KDE, dans votre thème Plasma
look-frame-native = Natif : le cadre du bureau
look-frame-katna = Katna : la barre supérieure devient la barre de titre
look-frame-katna-note-named = Katna dessine des coins arrondis et sa propre ombre. Le cadre ne suit plus le thème de { $desktop } ; les règles de fenêtre s’appliquent toujours.
look-frame-katna-note = Katna dessine des coins arrondis et sa propre ombre. Le cadre ne suit plus le thème du bureau ; les règles de fenêtre s’appliquent toujours.
look-frame-client-side = Votre bureau laisse chaque application dessiner son cadre, Katna dessine donc déjà le sien.
look-blurred-background = Arrière-plan flouté
look-blurred-background-detail = Le bureau transparaît, flouté, à travers la barre supérieure et les dossiers, et les menus et fenêtres contextuelles sont en verre dépoli.
look-blur = Flouter ce qui se trouve derrière la fenêtre
look-blur-detail = Les messages restent sur des fiches opaques, pour que le texte garde son contraste
look-blur-off-kde = L’effet de flou de KDE est désactivé. Activez Flou dans Configuration du système, Gestion des fenêtres, Effets de bureau, puis rouvrez Katna Mail.
look-blur-none-gnome = GNOME ne floute pas ce qui se trouve derrière les fenêtres.
look-blur-none-x11 = Votre gestionnaire de fenêtres ne floute pas ce qui se trouve derrière les fenêtres.
look-blur-none-wayland = Votre compositeur ne floute pas ce qui se trouve derrière les fenêtres.

## Settings > User feedback (crash reports)

feedback-intro-sending = Les nouveaux rapports de plantage sont envoyés pour aider à corriger le problème. Rien d’autre ne quitte cet ordinateur.
feedback-intro-local = Katna n’envoie rien nulle part. Les rapports de plantage restent sur cet ordinateur, pour que vous puissiez les consulter ou les joindre à un rapport de bug.
feedback-crash-reports = Rapports de plantage
feedback-crash-reports-detail = Créés quand Katna Mail ou son service d’arrière-plan plante.
feedback-save = Enregistrer les rapports de plantage sur cet ordinateur
feedback-save-detail = Votre dossier personnel, vos noms d’utilisateur et d’ordinateur et les adresses e-mail en sont retirés
feedback-saved = Rapports de plantage enregistrés
feedback-saved-detail = { $count ->
    [one] Seul le plus récent est conservé.
    [many] Les { $count } de plus récents sont conservés.
   *[other] Les { $count } plus récents sont conservés.
}
feedback-help-improve = Aider à améliorer Katna
feedback-help-improve-detail = Désactivé sauf si vous l’activez, et vous pouvez le désactiver ici à tout moment.
feedback-send = Envoyer les rapports de plantage
feedback-send-detail = Le rapport enregistré, tel que vous pouvez le voir ici, est envoyé à l’outil de suivi des plantages de Katna (Sentry, dans l’UE). Aucune adresse IP, aucun message ni aucune adresse e-mail
feedback-none-saved = Aucun rapport de plantage enregistré.
feedback-delete-all = Tout supprimer
feedback-app-daemon = Service d’arrière-plan
feedback-report-sent = { $date } · Envoyé
feedback-view = Afficher
feedback-view-tooltip = Ouvrir le rapport
feedback-copy-tooltip = Le copier pour le coller dans un rapport de bug
feedback-copied = Rapport de plantage copié.
feedback-deleted-all = Rapports de plantage supprimés.
feedback-read-failed = Impossible de lire le rapport de plantage : { $error }
feedback-delete-failed = Impossible de supprimer le rapport de plantage : { $error }
feedback-delete-all-failed = Impossible de supprimer les rapports de plantage : { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Fichier
desktop-menu-new-message = _Nouveau message
desktop-menu-quit = _Quitter
desktop-menu-edit = _Édition
desktop-menu-undo = _Annuler
desktop-menu-select-all = _Tout sélectionner
desktop-menu-select-none = Tout _désélectionner
desktop-menu-find = _Rechercher…
desktop-menu-view = _Affichage
desktop-menu-folder-list = Afficher la liste des _dossiers
desktop-menu-refresh = Actua_liser
desktop-menu-go = A_ller
desktop-menu-inbox = _Boîte de réception
desktop-menu-starred = Messages _suivis
desktop-menu-sent = Messages _envoyés
desktop-menu-drafts = B_rouillons
desktop-menu-all-mail = _Tous les messages
desktop-menu-next = Conversation suiva_nte
desktop-menu-previous = Conversation _précédente
desktop-menu-message = _Message
desktop-menu-open = _Ouvrir
desktop-menu-reply = _Répondre
desktop-menu-reply-all = Répondre à _tous
desktop-menu-forward = Trans_férer
desktop-menu-archive = Arc_hiver
desktop-menu-delete = _Supprimer
desktop-menu-spam = Signaler comme s_pam
desktop-menu-move-to = _Déplacer vers…
desktop-menu-mark-read = Marquer comme _lu
desktop-menu-mark-unread = Marquer comme _non lu
desktop-menu-star = Ajouter le s_uivi
desktop-menu-important = Marquer comme _important
desktop-menu-not-important = Marquer comme non i_mportant
desktop-menu-settings = _Configuration
desktop-menu-quick-settings = _Paramètres rapides
desktop-menu-configure = Con_figurer Katna Mail…
desktop-menu-help = Ai_de
desktop-menu-shortcuts = _Raccourcis clavier
desktop-menu-whats-new = _Nouveautés
desktop-menu-about = À _propos de Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Navigation
shortcut-group-actions = Actions
shortcut-group-go-to = Accéder à
shortcut-group-app = Application

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Conversation suivante
shortcut-previous = Conversation précédente
shortcut-down = Descendre dans la liste
shortcut-up = Monter dans la liste
shortcut-first = Premier élément de la liste
shortcut-last = Dernier élément de la liste
shortcut-page-down = Page suivante de la liste
shortcut-page-up = Page précédente de la liste
shortcut-open = Ouvrir la conversation
shortcut-back = Revenir à la liste
shortcut-scroll-down = Faire défiler vers le bas
shortcut-scroll-up = Faire défiler vers le haut
shortcut-scroll-page-down = Descendre d’une page
shortcut-scroll-page-up = Remonter d’une page
shortcut-compose = Nouveau message
shortcut-reply = Répondre
shortcut-reply-all = Répondre à tous
shortcut-forward = Transférer
shortcut-archive = Archiver
shortcut-delete = Supprimer
shortcut-spam = Signaler comme spam
shortcut-move-to = Déplacer vers
shortcut-mark-read = Marquer comme lu
shortcut-mark-unread = Marquer comme non lu
shortcut-star = Ajouter ou supprimer le suivi
shortcut-important = Marquer comme important
shortcut-not-important = Marquer comme non important
shortcut-check = Cocher la conversation
shortcut-select-all = Cocher toutes les conversations
shortcut-select-none = Décocher toutes les conversations
shortcut-undo = Annuler la dernière action
shortcut-go-inbox = Boîte de réception
shortcut-go-starred = Messages suivis
shortcut-go-sent = Messages envoyés
shortcut-go-drafts = Brouillons
shortcut-go-all = Tous les messages
shortcut-search = Rechercher dans les messages
shortcut-navigation = Afficher ou réduire le menu
shortcut-quick-settings = Paramètres rapides
shortcut-settings = Tous les paramètres
shortcut-shortcuts = Raccourcis clavier
shortcut-reload = Vérifier les nouveaux messages
shortcut-quit = Quitter

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } puis { $second }

## Settings > Accounts

accounts-folder-pane = Volet des dossiers
accounts-folder-pane-detail = Les comptes dont le volet de gauche affiche les dossiers.
accounts-shown-one = Un compte à la fois ; changez-en depuis la fiche du compte
accounts-shown-all = Tous les comptes, les uns après les autres
accounts-row = Comptes
accounts-row-detail = Supprimer un compte efface la copie de ses messages que Katna conserve sur cet ordinateur. Les messages restent sur le serveur.
accounts-none = Aucun compte pour l’instant.
accounts-kind-imported = Importé
accounts-picture-reset = Utiliser la photo du bureau
accounts-picture-change = Changer de photo
accounts-remove = Supprimer
accounts-delete-all-row = Supprimer toutes les données
accounts-delete-all-row-detail = Repartir de zéro, comme après une nouvelle installation.
accounts-delete-all-about = Supprime de cet ordinateur tous les comptes, tous les messages stockés, les contacts et les agendas, l’index de recherche, vos paramètres et vos mots de passe enregistrés. Rien ne change sur vos serveurs de messagerie.
accounts-delete-all-open = Supprimer toutes les données de Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } a été supprimé de Katna.
accounts-removed = { $address } a été supprimé de Katna. Ses messages restent sur le serveur.
accounts-all-deleted = Toutes les données de Katna ont été supprimées de cet ordinateur.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Supprimer { $address } ?
accounts-remove-confirm = Supprimer le compte
accounts-removing = Suppression…
accounts-remove-local-mail = { $folders ->
    [0] Tous les messages importés dans ce compte
    [one] Tous les messages importés dans ce compte, dans son dossier
    [many] Tous les messages importés dans ce compte, dans ses { $folders } de dossiers
   *[other] Tous les messages importés dans ce compte, dans ses { $folders } dossiers
}
accounts-remove-local-settings = Ses paramètres Katna
accounts-remove-mail = { $folders ->
    [0] Tous les messages de ce compte stockés par Katna
    [one] Tous les messages de ce compte stockés par Katna dans son dossier
    [many] Tous les messages de ce compte stockés par Katna dans ses { $folders } de dossiers
   *[other] Tous les messages de ce compte stockés par Katna dans ses { $folders } dossiers
}
accounts-remove-outbox = Ses messages en attente dans la boîte d’envoi
accounts-remove-settings = Son mot de passe enregistré et ses paramètres Katna
accounts-delete-all-title = Supprimer toutes les données de Katna ?
accounts-delete-all-confirm = Tout supprimer
accounts-deleting = Suppression…
accounts-delete-all-accounts = Tous les comptes, ainsi que tous les messages et pièces jointes stockés par Katna
accounts-delete-all-contacts = Les contacts, les agendas et l’index de recherche
accounts-delete-all-settings = Tous les paramètres, signatures et raccourcis clavier
accounts-delete-all-passwords = Tous les mots de passe enregistrés
accounts-deleted-heading = Supprimé de cet ordinateur :
accounts-cannot-undo = Cette action est irréversible.
accounts-server-delete-all = Rien ne change sur vos serveurs de messagerie : vos messages y restent, et si vous ajoutez à nouveau un compte, ils sont de nouveau téléchargés. Les messages importés depuis des fichiers n’existent que dans Katna ; les fichiers ne sont pas modifiés.
accounts-server-local = Ces messages ont été importés depuis des fichiers, Katna en a donc la seule copie. Les fichiers d’origine ne sont pas modifiés ; importez-les à nouveau pour récupérer les messages.
accounts-server-remove = Rien ne change sur le serveur de messagerie : vos messages y restent, et si vous ajoutez à nouveau le compte, ils sont de nouveau téléchargés.
accounts-confirm-word = supprimer
accounts-confirm-placeholder = Saisissez « { accounts-confirm-word } »
accounts-confirm-prompt = Pour confirmer, saisissez « { accounts-confirm-word } » :
accounts-cancel = Annuler
