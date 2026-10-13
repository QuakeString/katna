# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Fermer
reader-back = Retour
reader-mark-unread = Marquer comme non lu
reader-move-to = Déplacer vers
reader-snooze = Mettre en attente
reader-remind = Me le rappeler
reader-more = Plus
reader-original-colors = Afficher les couleurs d’origine
reader-dark-colors = Afficher en couleurs sombres
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
reader-sending = Envoi…
reader-me = moi
reader-to = à { $names }
reader-to-label = à
reader-tick-delivered = Remis { $when }
reader-tick-no-bounce = Envoyé { $when } ; aucun avis de non-remise n’est revenu, il est donc très probablement arrivé
reader-tick-bounced = Non remis : rejeté { $when }
reader-tick-read = Lu { $when } (accusé de lecture)
reader-tick-opened = Ouvert, la dernière fois { $when } (suivi des ouvertures)
reader-starred = Suivi
reader-chip-remove = Retirer { $label }
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
reader-download-failed-reason = Impossible de télécharger ce message. { $reason }
reader-download-offline = Ce compte est hors ligne. Repassez en ligne pour télécharger ce message.
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
security-look-up-key = Rechercher la clé

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Signature vérifiée
key-card-verified-detail = La signature est valide et vous faites confiance à cette clé.
key-card-unverified = Signature non vérifiée
key-card-unverified-detail = La signature est valide, mais rien ne confirme que la clé appartient à cette personne. Comparez l’empreinte avec elle, puis accordez votre confiance à la clé dans GnuPG (Kleopatra ou gpg --edit-key).
key-card-not-sender = Signé par quelqu’un d’autre
key-card-not-sender-detail = La signature est valide, mais la clé n’est pas celle de l’expéditeur.
key-card-untrusted = Clé non fiable
key-card-untrusted-detail = Vous avez marqué cette clé comme non fiable dans GnuPG.
key-card-signature-expired = Signature expirée
key-card-signature-expired-detail = La signature était valide, mais elle a expiré.
key-card-key-expired = Clé expirée
key-card-key-expired-detail = La signature est valide, mais la clé a expiré depuis.
key-card-key-revoked = Clé révoquée
key-card-key-revoked-detail = Son propriétaire a révoqué cette clé ; la signature n’est donc pas digne de confiance.
key-card-bad = Signature non valide
key-card-bad-detail = Ce message a été modifié après avoir été signé, ou la signature est falsifiée.
key-card-signed-by = Signé par
key-card-belongs-to = Appartient à
key-card-fingerprint = Empreinte
key-card-signed = Signé le
key-card-key = Clé
key-card-kind = { $standard }, { $algorithm }
key-card-created = Créée le
key-card-expires = Expire le
key-card-never = Jamais
key-card-issued-by = Émis par
key-card-found-in = Trouvée dans
key-card-keyring = Votre trousseau GnuPG
key-card-copy = Copier l’empreinte
key-card-import-title = Importer cette clé ?
key-card-from-directory = Trouvée dans l’annuaire de clés de { $domain }.
key-card-from-attachment = Provenant de la pièce jointe { $name }.
key-card-import-note = Katna pourra alors vérifier les signatures de cette personne et lui envoyer des messages chiffrés. Pour faire pleinement confiance à la clé, comparez l’empreinte avec elle.
key-card-cancel = Annuler
key-card-import = Importer la clé
key-card-looking-up = Recherche de la clé…
key-card-looking-up-detail = Interrogation de l’annuaire de clés de { $domain }.
key-card-not-found = Aucune clé trouvée
key-card-not-found-detail = { $domain } ne publie aucune clé pour cette adresse. Demandez à l’expéditeur de vous envoyer la sienne.
key-card-not-kept = La clé trouvée ne peut pas être utilisée.
key-card-failed = Impossible d’obtenir la clé

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Ce message ne vient peut-être pas de { $domain }
sender-failed-body = Il n’a pas passé les vérifications d’expéditeur de { $provider }. Méfiez-vous des liens, des pièces jointes et des réponses.
sender-provider-unknown = votre fournisseur de messagerie
sender-details = Détails
sender-details-hide = Masquer les détails
sender-looks-safe = Semble sûr
sender-move-to-spam = Déplacer dans les spams
sender-checked-by = Vérifié par { $provider }
sender-checked-by-server = Vérifié par { $provider } ({ $server })
sender-dmarc = Domaine de l’expéditeur (DMARC)
sender-dkim = Signature (DKIM)
sender-spf = Serveur d’envoi (SPF)
sender-result-pass = Réussi
sender-result-fail = Échoué
sender-result-unsure = Incertain
sender-result-none = Aucun
sender-result-missing = Non vérifié
sender-dmarc-pass = { $domain } confirme cet expéditeur.
sender-dmarc-fail = Le message ne correspond pas à la façon dont { $domain } déclare envoyer ses messages.
sender-dmarc-none = { $domain } ne publie aucune règle pour ses messages.
sender-dkim-pass = Signé par { $domain }.
sender-dkim-fail = La signature de { $domain } ne correspond pas au message.
sender-dkim-none = Le message n’était pas signé.
sender-spf-pass = Envoyé depuis un serveur listé par { $domain }.
sender-spf-fail = Envoyé depuis un serveur que { $domain } ne liste pas.
sender-spf-none = { $domain } ne liste pas ses serveurs.
sender-check-unsure = La vérification n’a pas pu donner de réponse claire.
sender-unconfirmed = { $provider } n’a pas pu confirmer que ce message vient de { $domain }. N’importe qui peut indiquer n’importe quel expéditeur.
sender-link-title = Ouvrir ce lien ?
sender-link-body = Ce message n’a pas passé les vérifications d’expéditeur. Le lien mène à { $host } :
sender-link-cancel = Annuler
sender-link-open = Ouvrir

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } l’a ouvert { $count ->
    [one] une fois
    [many] { $count } de fois
   *[other] { $count } fois
}, la dernière fois { $when }
tracking-opens-clicks = { $who } l’a ouvert { $opens ->
    [one] une fois
    [many] { $opens } de fois
   *[other] { $opens } fois
} et a suivi un lien { $clicks ->
    [one] une fois
    [many] { $clicks } de fois
   *[other] { $clicks } fois
}, la dernière fois { $when }
tracking-clicked = { $who } a suivi un lien { $clicks ->
    [one] une fois
    [many] { $clicks } de fois
   *[other] { $clicks } fois
}, la dernière fois { $when }
tracking-maybe-opened = { $who } l’a peut-être ouvert (Apple Mail charge les images pour protéger la vie privée)
tracking-seen-none = Personne ne l’a encore ouvert ni n’a suivi de lien
tracking-receipt = { $who } a envoyé un accusé de lecture
tracking-receipt-read = { $who } l’a lu (accusé de lecture), { $when }
tracking-receipt-displayed = Accusé de lecture : { $who } a ouvert votre message
tracking-receipt-other = Accusé de lecture : { $who } a supprimé ou traité votre message sans l’ouvrir

## Remote images and pictures

remote-hidden = Les images de ce message sont masquées.
remote-hidden-unconfirmed = Images masquées : l’expéditeur n’a pas pu être confirmé.
remote-hidden-failed = Images masquées : ce message n’a pas passé les vérifications d’expéditeur.
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
attachment-forward = Transférer
attachment-save-all = Tout enregistrer
attachment-save-all-tooltip = Enregistrer toutes les pièces jointes dans un dossier
attachment-save-here = Enregistrer ici
attachment-not-downloaded = Ce message n’est pas téléchargé.
attachment-open-message = Ouvrez ce message pour lire ses pièces jointes.
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

print-preview-title = Aperçu avant impression
print-preview-laying-out = Mise en page…
print-preview-pages = { $count ->
    [one] { $count } page
    [many] { $count } de pages
   *[other] { $count } pages
}
print-preview-more = { $count ->
    [one] et { $count } page de plus
    [many] et { $count } de pages de plus
   *[other] et { $count } pages de plus
}
print-preview-failed = les pages n’ont pas pu être affichées
print-preview-paper = Papier
print-preview-a4 = A4
print-preview-letter = Lettre US
print-preview-layout = Mise en page
print-preview-as-shown = Tel qu’affiché
print-preview-simple = Texte simple
print-preview-backgrounds = Arrière-plans
print-preview-cancel = Annuler
print-preview-print = Imprimer
print-not-downloaded = (Pas encore téléchargé.)
print-encrypted = (Chiffré. Ouvrez-le dans Katna Mail pour imprimer son texte.)
print-to = À : { $addresses }
print-cc = Cc : { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Épingler en haut
text-copy-address = Copier l’adresse
text-copy = Copier
text-select-all = Tout sélectionner
