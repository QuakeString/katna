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
compose-show-trimmed = Afficher le contenu masqué

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
recipient-bad-title = Vérifiez l’adresse
recipient-bad-text = « { $address } » n’est pas une adresse e-mail valide. Corrigez-la ou supprimez-la avant l’envoi.
recipient-bad-fix = Corriger
