# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Règles
settings-rules-summary = Trier, libeller, transférer ou rendre silencieux les nouveaux messages automatiquement
settings-rules-intro = Les règles trient automatiquement les nouveaux messages, dans cet ordre. Faites glisser pour réorganiser.
settings-rules-all-accounts = Tous les comptes
settings-rules-new = Nouvelle règle
settings-rules-none = Aucune règle pour le moment. Une règle trie automatiquement les nouveaux messages : par expéditeur, objet ou mots.
settings-rules-none-account = Aucune règle pour ce compte pour le moment.
settings-rules-drag = Faire glisser pour réorganiser
settings-rules-edit = Modifier la règle
settings-rules-turn-off = Désactiver cette règle
settings-rules-turn-on = Activer cette règle

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Règles prêtes à l’emploi
settings-rules-starters-intro = Désactivées jusqu’à ce que vous en activiez une. Elles s’appliquent à tous vos comptes ; modifiez-en une pour la changer.
settings-rules-starter-turning-on = Activation de « { $name } »…
settings-rules-starter-failed = Impossible d’activer « { $name } » : { $error }
rules-starter-promotions = Promotions en silence
rules-starter-newsletters = Newsletters vers À lire
rules-starter-receipts = Reçus et factures
rules-starter-deliveries = Livraisons
rules-starter-train = Billets de train
rules-starter-flight = Billets d’avion
rules-starter-codes = Codes à usage unique
rules-starter-security = Alertes de sécurité
rules-starter-social = Réseaux sociaux
rules-starter-invites = Invitations de calendrier
rules-starter-folder-reading = À lire
rules-starter-folder-receipts = Reçus
rules-starter-folder-deliveries = Livraisons
rules-starter-folder-travel = Voyages
rules-starter-folder-social = Réseaux sociaux
rules-runs-katna = S’exécute dans Katna
rules-runs-gmail = S’exécute sur Gmail
rules-runs-sieve = S’exécute sur le serveur
rules-stopped = Arrêtée
rules-error-folder-gone = Le dossier utilisé par cette règle n’existe plus. Modifiez la règle pour en choisir un autre.
rules-error-no-archive = Ce compte n’a pas de dossier d’archives. Modifiez la règle pour faire autre chose.
rules-error-no-trash = Ce compte n’a pas de dossier Corbeille. Modifiez la règle pour faire autre chose.
rules-error-cannot-send = Ce compte ne peut pas envoyer de messages, la règle ne peut donc pas les transférer.
rules-error-other = { $error }. Modifiez la règle et réactivez-la.

settings-folders = Dossiers
settings-folders-summary = Nombre de messages non lus dans le volet des dossiers
settings-folders-unread-counts = Nombre de non-lus sur chaque dossier
settings-folders-unread-counts-detail = Désactivé : seule la boîte de réception affiche le nombre de non-lus

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } et { $next }
rules-summary-or = { $first } ou { $next }
rules-summary-more = { $count } de plus
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Contient une pièce jointe
rules-summary-no-attachment = Ne contient pas de pièce jointe
rules-summary-mailing-list = Provient d’une liste de diffusion
rules-summary-not-mailing-list = Ne provient pas d’une liste de diffusion
rules-summary-tab = Dans l’onglet { $tab }
rules-summary-not-tab = Pas dans l’onglet { $tab }
rules-summary-move = déplacer vers { $folder }
rules-summary-archive = ne pas afficher dans la boîte de réception
rules-summary-trash = placer dans la corbeille
rules-summary-mark-read = marquer comme lu
rules-summary-star = suivre
rules-summary-important = marquer comme important
rules-summary-label = libeller { $label }
rules-summary-forward = transférer à { $address }
rules-summary-dont-notify = ne pas notifier
rules-summary-read-after = { $count ->
    [one] marquer comme lu après { $count } jour
    [many] marquer comme lu après { $count } de jours
   *[other] marquer comme lu après { $count } jours
}
rules-summary-folder-gone = un dossier qui n’existe plus

## The rule editor

rules-editor-new-title = Nouvelle règle
rules-editor-edit-title = Modifier la règle
rules-editor-name-hint = Nom de la règle
rules-editor-when = Quand un nouveau message correspond à
rules-editor-of-these = ces conditions :
rules-mode-all = toutes
rules-mode-any = l’une de
rules-field-from = De
rules-field-to = À
rules-field-cc = Cc
rules-field-any-recipient = À ou Cc
rules-field-reply-to = Répondre à
rules-field-subject = Objet
rules-field-body = Texte
rules-field-attachment-name = Nom de la pièce jointe
rules-field-has-attachment = Contient une pièce jointe
rules-field-mailing-list = Provient d’une liste de diffusion
rules-field-tab = Onglet de la boîte de réception
rules-comparator-contains = contient
rules-comparator-not-contains = ne contient pas
rules-comparator-begins-with = commence par
rules-comparator-ends-with = se termine par
rules-comparator-equals = est exactement
rules-comparator-matches = correspond au modèle
rules-has-yes = oui
rules-has-no = non
rules-editor-value-hint = Mots ou adresse
rules-editor-add-condition = Ajouter une condition
rules-editor-remove = Retirer
rules-editor-then = Alors :
rules-action-move = Déplacer vers
rules-action-archive = Ne pas afficher dans la boîte de réception (archiver)
rules-action-trash = Placer dans la corbeille
rules-action-mark-read = Marquer comme lu
rules-action-star = Suivre
rules-action-important = Marquer comme important
rules-action-label = Ajouter un libellé
rules-action-forward = Transférer à
rules-action-dont-notify = Ne pas notifier
rules-action-read-after = Marquer comme lu après
rules-editor-choose-folder = Choisir un dossier
rules-editor-choose-label = Choisir un libellé
rules-editor-new-folder = Nouveau : { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Adresse e-mail
rules-editor-days = jours
rules-editor-add-action = Ajouter une action
rules-editor-stop = S’arrêter ici : les règles suivantes ne s’appliquent pas à ce message
rules-editor-accounts = Comptes :
rules-editor-accounts-none = Choisir des comptes
rules-editor-accounts-many = { $count ->
    [one] { $count } compte
    [many] { $count } de comptes
   *[other] { $count } comptes
}
rules-editor-matches = Correspond à { $mails } des { $days } derniers jours
rules-editor-mails = { $count ->
    [one] { $count } message
    [many] { $count } de messages
   *[other] { $count } messages
}
rules-editor-counting = Décompte des messages correspondants…
rules-editor-show = Les afficher
rules-editor-also-apply = Appliquer aussi à ces { $count }
rules-editor-runs-katna = S’exécute dans Katna, tant que cet ordinateur est allumé.
rules-editor-runs-gmail = S’exécute sur Gmail : fonctionne aussi sur votre téléphone et quand cet ordinateur est éteint.
rules-editor-runs-sieve = S’exécute sur votre serveur de messagerie : fonctionne aussi sur votre téléphone et quand cet ordinateur est éteint.
rules-note-gmail-action = S’exécute dans Katna : les filtres Gmail ne savent pas faire « { $action } ».
rules-note-sieve-action = S’exécute dans Katna : les règles de votre serveur de messagerie ne savent pas faire « { $action } ».
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = S’exécute dans Katna : les filtres Gmail ne peuvent pas tester « { $test } » comme le fait Katna.
rules-note-sieve-condition = S’exécute dans Katna : les règles de votre serveur de messagerie ne peuvent pas tester « { $test } » comme le fait Katna.
rules-note-order = S’exécute dans Katna, comme une règle précédente du compte : les règles s’appliquent dans l’ordre de la liste.
rules-note-gmail-stop = S’exécute dans Katna : les filtres Gmail ne peuvent pas empêcher l’exécution des règles suivantes.
rules-note-gmail-forward = S’exécute dans Katna : Gmail ne transfère qu’aux adresses validées dans ses paramètres, et { $address } n’en fait pas partie.
rules-note-gmail-folder = S’exécute dans Katna : Gmail n’a pas de libellé pour un dossier utilisé par cette règle.
rules-note-sieve-folder = S’exécute dans Katna : votre serveur de messagerie n’a pas de dossier utilisé par cette règle.
rules-note-gmail-sign-in = S’exécute dans Katna jusqu’à ce que vous vous reconnectiez à Google et autorisiez Katna à créer des filtres Gmail.
rules-note-sieve-other-script = S’exécute dans Katna : un autre script de règles (« { $name } ») est actif sur votre serveur de messagerie.
rules-note-gmail-failed = S’exécute dans Katna : Gmail ne l’a pas accepté ({ $error }).
rules-note-sieve-failed = S’exécute dans Katna : votre serveur de messagerie ne l’a pas accepté ({ $error }).
rules-editor-cancel = Annuler
rules-editor-save = Enregistrer
rules-editor-saving = Enregistrement…
rules-editor-delete = Supprimer la règle
rules-editor-delete-ask = Supprimer cette règle ?
rules-editor-delete-keep = La conserver
rules-editor-delete-confirm = Supprimer
rules-editor-needs-folder = Choisissez un dossier pour chaque « Déplacer vers » et un libellé pour chaque « Ajouter un libellé ».
rules-editor-needs-days = « Marquer comme lu après » demande un nombre de jours, de 1 à 3650.
rules-saved = Règle enregistrée
rules-saved-applied = { $count ->
    [one] Règle enregistrée et appliquée à { $count } message
    [many] Règle enregistrée et appliquée à { $count } de messages
   *[other] Règle enregistrée et appliquée à { $count } messages
}
rules-apply-failed = Règle enregistrée, mais son application a échoué : { $error }
rules-deleted = Règle supprimée
rules-delete-failed = Impossible de supprimer la règle : { $error }
rules-change-failed = Impossible de modifier les règles : { $error }
