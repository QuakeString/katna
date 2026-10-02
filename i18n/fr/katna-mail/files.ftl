# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Rechercher des fichiers

## Left side (and chips on a phone)

files-all = Tous les fichiers
files-pictures = Images
files-pdfs = PDF
files-documents = Documents
files-sheets = Feuilles de calcul
files-slides = Présentations
files-other = Autres
files-accounts = Comptes
files-drives = Drives
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Partagés avec moi
files-shown = Affichés
files-received = Reçus
files-sent = Envoyés par moi

## Over the files

files-count = { $count ->
    [one] { $count } fichier · { $size }
    [many] { $count } de fichiers · { $size }
   *[other] { $count } fichiers · { $size }
}
files-anyone = Tout le monde
files-from-person = De { $name }
files-time-any = Toutes les dates
files-time-today = Aujourd’hui
files-time-yesterday = Hier
files-time-this-week = Cette semaine
files-time-last-week = La semaine dernière
files-time-this-month = Ce mois-ci
files-time-last-month = Le mois dernier
files-time-between = { $first } – { $last }
files-time-hint = Cliquez sur un jour, ou faites glisser sur plusieurs jours
files-time-summary = { $count ->
    [one] { $days } · { $count } fichier
    [many] { $days } · { $count } de fichiers
   *[other] { $days } · { $count } fichiers
}
files-time-clear = Effacer
files-time-month-back = Mois précédent
files-time-month-on = Mois suivant
files-time-wheel = Faites défiler pour décaler ces dates en gardant leur durée
files-sort-newest = Plus récents d’abord
files-sort-oldest = Plus anciens d’abord
files-sort-largest = Plus volumineux d’abord
files-sort-name = Par nom
files-grid = Fiches
files-list = Liste
files-this-week = Cette semaine
files-undated = Sans date
files-me = Moi
files-no-subject = (aucun objet)
files-loading = Rassemblement des fichiers de vos messages…
files-empty = Les fichiers de vos messages s’affichent ici.
files-none-match = Aucun fichier ne correspond.
files-load-failed = Échec de la lecture des fichiers : { $error }

## A file's menu and buttons

files-open = Ouvrir
files-open-with = Ouvrir avec…
files-save = Enregistrer…
files-show-mail = Afficher le message
files-mail-window = Ouvrir le message dans une nouvelle fenêtre
files-forward = Transférer le fichier
files-from-them = Fichiers de { $name }
files-copy-name = Copier le nom du fichier
files-name-copied = Nom du fichier copié
files-downloading = Téléchargement du message…
files-download-failed = Impossible de télécharger ce message.

## A cloud drive in place of the mail files

files-drive-mine = Mon Drive
files-drive-mine-onedrive = Mes fichiers
files-drive-results = « { $words } »
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 fichier
        [many] { $files } de fichiers
       *[other] { $files } fichiers
    }
    [one] 1 dossier · { $files ->
        [one] 1 fichier
        [many] { $files } de fichiers
       *[other] { $files } fichiers
    }
    [many] { $folders } de dossiers · { $files ->
        [one] 1 fichier
        [many] { $files } de fichiers
       *[other] { $files } fichiers
    }
   *[other] { $folders } dossiers · { $files ->
        [one] 1 fichier
        [many] { $files } de fichiers
       *[other] { $files } fichiers
    }
}
files-drive-folders = Dossiers
files-drive-files = Fichiers
files-drive-folder = Dossier
files-drive-meta = { $what } · Modifié le { $date }
files-drive-as-link = { $what } · en lien
files-drive-google-doc = Google Docs
files-drive-google-sheet = Google Sheets
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawings
files-drive-fetching = Récupération…
files-drive-loading = Ouverture du Drive…
files-drive-empty = Ce dossier est vide.
files-drive-unreachable = Impossible de joindre { $drive }.
files-drive-try-again = Réessayer
files-drive-needs-permission = Katna a besoin de votre autorisation une seule fois pour afficher ce Drive. Reconnectez-vous et autorisez Katna à voir vos fichiers.
files-drive-allow = Autoriser
files-drive-allow-failed = La connexion n’a pas abouti, le Drive reste donc fermé.
files-drive-attach = Joindre
files-drive-more = Plus
files-drive-download = Télécharger…
files-drive-open-web = Ouvrir dans { $drive }
files-drive-copy-link = Copier le lien
files-drive-link-copied = Lien copié
files-drive-share = Partager…
files-drive-rename = Renommer
files-drive-trash = Mettre à la corbeille
files-drive-trashed = « { $name } » est dans la corbeille de { $drive }
files-drive-renamed = Renommé en « { $name } »
files-drive-getting = Récupération de { $name } depuis { $drive }…
files-drive-get-failed = Impossible de récupérer { $name } : { $error }
files-drive-upload = Importer
files-drive-upload-files = Importer des fichiers
files-drive-upload-folder = Importer un dossier
files-drive-upload-failed = Impossible d’importer { $name } : { $error }
files-drive-upload-needs = Pour importer, Katna a besoin de votre autorisation une seule fois : appuyez sur Autoriser dans Paramètres › Applications par défaut › Page Fichiers.

## The Share dialog of a drive file or folder

files-share-title = Partager « { $name } »
files-share-add = Ajouter des personnes par nom ou adresse
files-share-not-address = « { $text } » n’est pas une adresse e-mail
files-share-notify = Laisser { $drive } leur envoyer aussi un e-mail
files-share-people = Personnes ayant accès
files-share-general = Accès général
files-share-loading = Lecture des personnes ayant accès…
files-share-restricted = Limité
files-share-restricted-about = Seules les personnes ayant accès peuvent l’ouvrir avec le lien
files-share-anyone = Tous les utilisateurs qui ont le lien
files-share-anyone-can = { $role ->
    [editor] Tous ceux qui ont le lien peuvent modifier
    [commenter] Tous ceux qui ont le lien peuvent commenter
   *[viewer] Tous ceux qui ont le lien peuvent consulter
}
files-share-anyone-about = { $role ->
    [editor] Tout internaute disposant du lien peut modifier
    [commenter] Tout internaute disposant du lien peut commenter
   *[viewer] Tout internaute disposant du lien peut consulter
}
files-share-role-owner = Propriétaire
files-share-role-editor = Éditeur
files-share-role-commenter = Commentateur
files-share-role-viewer = Lecteur
files-share-you = { $name } (vous)
files-share-domain = Tout le monde chez { $domain }
files-share-inherited = Accès hérité d’un dossier parent
files-share-remove = Retirer l’accès
files-share-copy-link = Copier le lien
files-share-share = Partager
files-share-done = Terminé
files-share-sharing = Partage…
files-share-shared = { $count ->
    [one] Partagé avec 1 personne
    [many] Partagé avec { $count } de personnes
   *[other] Partagé avec { $count } personnes
}
files-share-refused = { $drive } n’a pas pu partager avec { $addresses }
files-share-failed = Impossible de modifier le partage : { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Importation d’1 élément
    [many] Importation de { $count } d’éléments
   *[other] Importation de { $count } éléments
}
files-tray-done = { $count ->
    [one] 1 importation terminée
    [many] { $count } d’importations terminées
   *[other] { $count } importations terminées
}
files-tray-some-failed = { $done } importés, { $failed } en échec
files-tray-minutes-left = { $minutes ->
    [one] Environ une minute restante
    [many] Environ { $minutes } de minutes restantes
   *[other] Environ { $minutes } minutes restantes
}
files-tray-seconds-left = Moins d’une minute restante
files-tray-starting = Démarrage…
files-tray-cancel-all = Tout annuler
files-tray-cancel = Annuler
files-tray-fold = Masquer la liste
files-tray-unfold = Afficher la liste
files-tray-close = Fermer
files-tray-progress = { $place } · { $sent } sur { $size }
files-tray-in = Dans { $place }
files-tray-cancelled = Annulé
