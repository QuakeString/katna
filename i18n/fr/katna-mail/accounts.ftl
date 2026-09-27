# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Volet des dossiers
accounts-folder-pane-detail = Les comptes dont le volet de gauche affiche les dossiers.
accounts-shown-one = Un compte à la fois ; changez-en depuis la fiche du compte
accounts-shown-all = Tous les comptes, les uns après les autres
accounts-row = Comptes
accounts-row-detail = Le volet des dossiers et le menu du compte affichent les comptes dans cet ordre ; le premier est celui par défaut. Supprimer un compte efface la copie de ses messages que Katna conserve sur cet ordinateur. Les messages restent sur le serveur.
accounts-none = Aucun compte pour l’instant.
accounts-kind-imported = Importé
accounts-picture-reset = Utiliser la photo du bureau
accounts-picture-change = Changer de photo
accounts-picture-remove = Retirer la photo
accounts-rename = Renommer
accounts-name-save = Enregistrer
accounts-name-cancel = Annuler
accounts-name-placeholder = Votre nom
accounts-rename-failed = Impossible de renommer le compte : { $error }
accounts-move-up = Monter
accounts-move-down = Descendre
accounts-drag = Faites glisser pour changer l’ordre
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
reset-cache-about = Supprime les messages et pièces jointes téléchargés par Katna, les photos des expéditeurs et l’index de recherche, puis télécharge à nouveau les messages récents. Les comptes, les paramètres et les messages qui ne se trouvent que sur cet ordinateur sont conservés.
reset-cache-button = Réinitialiser le cache
reset-cache-title = Réinitialiser le cache ?
reset-cache-deleted = Supprimé, puis téléchargé à nouveau :
reset-cache-mail = Messages et pièces jointes téléchargés depuis vos serveurs IMAP : les messages récents sont téléchargés à nouveau dès maintenant, les plus anciens quand vous les ouvrez
reset-cache-index = L’index de recherche, reconstruit immédiatement
reset-cache-pictures = Photos des expéditeurs
reset-cache-kept = Conservé : vos comptes, mots de passe et paramètres ; les suivis, libellés, marques de lecture et épingles ; les brouillons, la boîte d’envoi et les modifications pas encore sur le serveur ; et les messages des comptes POP3 ou des fichiers importés, qui n’ont peut-être aucune autre copie. Rien ne change sur vos serveurs de messagerie.
reset-cache-confirm = Réinitialiser le cache
reset-cache-busy = Réinitialisation…
reset-cache-done = Le cache a été réinitialisé. Les messages récents sont de nouveau en cours de téléchargement.
reset-cache-done-freed = Le cache a été réinitialisé et { $size } ont été libérés. Les messages récents sont de nouveau en cours de téléchargement.
