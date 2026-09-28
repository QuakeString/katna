# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Nouveau message
compose-restore = Restaurer
compose-minimize = Réduire
compose-exit-full-screen = Quitter le plein écran
compose-open-window = Ouvrir dans une nouvelle fenêtre
compose-save-close = Enregistrer et fermer
compose-back-to-mail = Revenir à la fenêtre de messagerie
compose-pop-out-reply = Ouvrir la réponse dans une fenêtre
compose-edit-recipients = Modifier les destinataires
compose-summary-cc = Cc : { $names }
compose-summary-bcc = Cci : { $names }
compose-more-recipients = { $count } de plus
compose-show-trimmed = Afficher le contenu masqué
compose-hide-trimmed = Masquer le contenu cité
compose-remove-trimmed = Supprimer le texte cité
compose-trimmed-removed = Texte cité supprimé

## Recipients and subject

compose-to = À
compose-cc = Cc
compose-bcc = Cci
compose-from = De
compose-from-choose = Envoyer depuis un autre compte
compose-recipients = Destinataires
compose-subject = Objet

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Envoyez ou supprimez d’abord le message ouvert.
compose-bad-address = « { $address } » n’est pas une adresse e-mail.
compose-no-recipients = Ajoutez au moins un destinataire.
compose-attachments-too-large = Les pièces jointes font { $size } ; les serveurs de messagerie acceptent jusqu’à { $limit }.
compose-no-account = Ajoutez un compte depuis lequel envoyer des e-mails.
compose-past-time = Choisissez une date et une heure à venir.
compose-scheduling = Programmation…
compose-sending = Envoi…
compose-scheduled = Envoi programmé pour { $when }
compose-sent-archived = Envoyé et archivé
compose-sent = Message envoyé
compose-discarded = Brouillon supprimé
compose-draft-saved = Brouillon enregistré
compose-draft-failed = Impossible d’enregistrer le brouillon : { $error }
compose-draft-not-opened = Impossible d’ouvrir le brouillon.

## Attachments

compose-picker-insert = Insérer
compose-picker-attach = Joindre
compose-file-too-large = { $name } est trop volumineux : un message peut contenir jusqu’à { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Retirer la pièce jointe
compose-attachments-total = { $count ->
    [one] { $count } fichier, { $size }
    [many] { $count } de fichiers, { $size }
   *[other] { $count } fichiers, { $size }
}
compose-drive-note = { $name } dépasse { $limit } ; il est donc placé dans votre Google Drive et le message contient un lien.
compose-drive-tip = Dans votre Google Drive ; le message contient un lien
compose-drive-uploading = Envoi en cours : { $percent } %
compose-drive-allow = Autoriser Drive
compose-drive-allow-tip = Connectez-vous de nouveau avec Google pour que Katna puisse placer les gros fichiers dans votre Drive
compose-drive-retry = Réessayer
compose-drive-sends-when-uploaded = Envoi dès que { $name } est téléversé
compose-drive-not-uploaded = { $name } n’est pas encore dans Google Drive
compose-drive-share-failed = Impossible de partager les fichiers dans Google Drive : { $error }
compose-drive-share-title = Partager les fichiers avec tout le monde ?
compose-drive-share-text = { $count ->
    [one] Google Drive ne peut pas partager les fichiers avec { $addresses }, qui n’a pas de compte Google. Toute personne disposant du lien pourra les ouvrir à la place.
    [many] Google Drive ne peut pas partager les fichiers avec { $addresses }, qui n’ont pas de compte Google. Toute personne disposant du lien pourra les ouvrir à la place.
   *[other] Google Drive ne peut pas partager les fichiers avec { $addresses }, qui n’ont pas de compte Google. Toute personne disposant du lien pourra les ouvrir à la place.
}
compose-drive-share-link = Partager par lien
compose-drive-send-without = Envoyer sans partager
compose-drive-share-cancel = Annuler
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = Déposez les fichiers ici
compose-drop-here = Déposez ici
compose-paste-keep-formatting = Conserver la mise en forme
compose-paste-table = Tableau
compose-paste-picture = Image
compose-paste-plain-text = Texte brut
compose-paste-inline = Dans le texte
compose-paste-attachment = Pièce jointe

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Chiffrer
compose-encrypted = Chiffré : seuls les destinataires peuvent le lire
compose-sign = Signer
compose-signed = Signé : les destinataires peuvent vérifier qu’il vient de vous
compose-track = Suivre les ouvertures et les clics
compose-tracked = Suivi : vous voyez quand chaque destinataire l’ouvre ou suit un lien
compose-track-clicks = Suivre les clics sur les liens (le texte brut ne peut pas indiquer les ouvertures)
compose-tracked-clicks = Suivi : vous voyez quand chaque destinataire suit un lien
compose-track-sign-in = Connectez-vous à un compte Katna pour suivre les ouvertures et les clics
compose-receipt = Demander un accusé de lecture
compose-receipt-on = Accusé de lecture demandé : l’application du destinataire peut lui proposer d’en envoyer un
compose-delivery = Demander un accusé de remise
compose-delivery-on = Accusé de remise demandé : votre serveur de messagerie vous enverra un e-mail quand le serveur de chaque destinataire l’acceptera
compose-delivery-unavailable = Votre serveur de messagerie n’envoie pas d’accusés de remise

## Spelling

spell-no-dictionary = Aucun dictionnaire orthographique n’est installé pour { $language } (par exemple hunspell-en_us).
spell-dictionary-error = Dictionnaire orthographique : { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = « { $words } »
grammar-add = Ajouter « { $words } »
grammar-remove = Supprimer « { $words } »
grammar-ignore = Ignorer

## Send checks (asked before a message goes out)

send-check-attachment-title = Vouliez-vous joindre des fichiers ?
send-check-attachment-text = Vous parlez d’une pièce jointe, mais rien n’est joint.
send-check-attach = Joindre un fichier
send-check-subject-title = Envoyer sans objet ?
send-check-subject-text = Ce message n’a pas d’objet.
send-check-add-subject = Ajouter un objet
send-check-send-anyway = Envoyer quand même
recipient-not-valid = Adresse e-mail non valide
recipient-show-address = Afficher l’adresse
recipient-remove = Retirer
recipient-bad-title = Vérifiez l’adresse
recipient-bad-text = « { $address } » n’est pas une adresse e-mail valide. Corrigez-la ou supprimez-la avant l’envoi.
recipient-bad-fix = Corriger
