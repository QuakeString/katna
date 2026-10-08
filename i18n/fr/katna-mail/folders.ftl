# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Libellés
nav-folders = Dossiers
nav-label-new = Créer un libellé
nav-folder-new = Créer un dossier
nav-menu-check-mail = Rechercher de nouveaux messages
nav-menu-check-inbox = Relever cette boîte de réception
nav-unified-leave-out = Exclure de la boîte de réception unifiée
nav-unified-bring-back = Réintégrer dans la boîte de réception unifiée
nav-menu-sign-in-again = Se reconnecter
nav-menu-new-mail = Nouveau message depuis ce compte
nav-menu-account-settings = Paramètres du compte
nav-account-checked = Synchronisé · vérifié { $ago }
nav-account-in-sync = Synchronisé
nav-account-connecting = Connexion…
nav-account-offline = Hors ligne, nouvelle tentative
nav-account-signed-out = Connexion { $provider } expirée
nav-account-password-refused = Mot de passe refusé
nav-account-storage = { $used } utilisés sur { $total }
nav-menu-new-subfolder = Nouveau dossier à l’intérieur
nav-menu-new-sublabel = Nouveau libellé à l’intérieur
nav-menu-rename = Renommer
nav-menu-delete = Supprimer
nav-menu-empty-trash = Vider la corbeille
nav-account-unnamed = Compte { $number }
nav-all-accounts = Tous les comptes
nav-expand = Afficher les dossiers
nav-collapse = Masquer les dossiers
storage-used = { $percent } % utilisés sur { $total }
storage-used-detail = { $address } : { $used } utilisés sur { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Boîte de réception
folder-starred = Messages suivis
folder-snoozed = En attente
folder-unread = Non lus
folder-important = Importants
folder-drafts = Brouillons
folder-sent = Messages envoyés
folder-archive = Archives
folder-spam = Spam
folder-trash = Corbeille
folder-all-mail = Tous les messages
folder-scheduled = Messages programmés
folder-waiting = En attente de réponse
folder-waiting-short = Sans réponse
folder-reminders = Rappels
folder-outbox = Boîte d’envoi
folder-activity = Activité
folder-not-on-account = Ce compte n’a pas ce dossier.

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
label-rename-title = Renommer le libellé
label-folder-rename-title = Renommer le dossier
label-rename = Renommer
label-renaming = Renommage…
label-renamed = Libellé renommé en « { $name } ».
label-folder-renamed = Dossier renommé en « { $name } ».

## Deleting a folder or label (asked first)

folder-delete-title = Supprimer « { $name } » ?
folder-delete-body = { $count ->
    [0] Il ne contient aucun message. Le dossier est supprimé du serveur, il disparaît donc aussi du webmail et de votre téléphone.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Sa conversation va dans la corbeille, vous pouvez donc encore la récupérer.
            [many] Ses { $count } de conversations vont dans la corbeille, vous pouvez donc encore les récupérer.
           *[other] Ses { $count } conversations vont dans la corbeille, vous pouvez donc encore les récupérer.
        }
       *[message] { $count ->
            [one] Son message va dans la corbeille, vous pouvez donc encore le récupérer.
            [many] Ses { $count } de messages vont dans la corbeille, vous pouvez donc encore les récupérer.
           *[other] Ses { $count } messages vont dans la corbeille, vous pouvez donc encore les récupérer.
        }
    } Le dossier est supprimé du serveur, il disparaît donc aussi du webmail et de votre téléphone.
}
folder-delete-forever-body = { $count ->
    [0] Il ne contient aucun message. Le dossier est supprimé du serveur, il disparaît donc aussi du webmail et de votre téléphone.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Sa conversation est supprimée définitivement ; ce compte n’a pas de corbeille.
            [many] Ses { $count } de conversations sont supprimées définitivement ; ce compte n’a pas de corbeille.
           *[other] Ses { $count } conversations sont supprimées définitivement ; ce compte n’a pas de corbeille.
        }
       *[message] { $count ->
            [one] Son message est supprimé définitivement ; ce compte n’a pas de corbeille.
            [many] Ses { $count } de messages sont supprimés définitivement ; ce compte n’a pas de corbeille.
           *[other] Ses { $count } messages sont supprimés définitivement ; ce compte n’a pas de corbeille.
        }
    } Le dossier est supprimé du serveur, il disparaît donc aussi du webmail et de votre téléphone.
}
folder-delete-label-body = Le libellé est supprimé. Ses messages restent dans Tous les messages et dans leurs autres libellés.
folder-delete-confirm = Supprimer le dossier
folder-delete-label-confirm = Supprimer le libellé
folder-deleted = Dossier « { $name } » supprimé
label-deleted = Libellé « { $name } » supprimé
