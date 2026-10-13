# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Non envoyé, car { $reason }.
outbox-retrying = Pas encore envoyé, car { $reason }. Katna réessaie de lui-même.
outbox-waiting-sign-in = En attente de votre reconnexion à { $address }. Il partira alors.
outbox-waiting-password = En attente du nouveau mot de passe de { $address }. Il partira alors.
outbox-waiting-connection = En attente d’une connexion. Il partira quand vous serez de nouveau en ligne.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = il n’a aucun destinataire
outbox-reason-address = une adresse à laquelle il est envoyé n’existe pas
outbox-reason-too-large = il est trop volumineux pour le serveur de messagerie
outbox-reason-blocked = le serveur de messagerie l’a bloqué
outbox-reason-gone = sa copie sur cet ordinateur a disparu
outbox-reason-refused = le serveur de messagerie l’a refusé

## Buttons and notes

outbox-try-again = Réessayer
outbox-edit = Modifier
outbox-delete = Supprimer
outbox-deleted = Supprimé de la boîte d’envoi
outbox-sending-again = Nouvel envoi…

outbox-snackbar-not-sent = « { $subject } » n’a pas été envoyé, car { $reason }.
outbox-open = Boîte d’envoi
