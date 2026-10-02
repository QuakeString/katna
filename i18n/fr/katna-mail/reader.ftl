# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Fermer
reader-back = Retour
reader-mark-unread = Marquer comme non lu
reader-move-to = Déplacer vers
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
tracking-receipt-displayed = Accusé de lecture : { $who } a ouvert votre message
tracking-receipt-other = Accusé de lecture : { $who } a supprimé ou traité votre message sans l’ouvrir

## Remote images and pictures

remote-hidden = Les images de ce message sont masquées.
remote-hidden-unconfirmed = Images masquées : l’expéditeur n’a pas pu être confirmé.
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
