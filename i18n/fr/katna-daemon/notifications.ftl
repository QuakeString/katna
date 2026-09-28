# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } nouvel e-mail
    [many] { $count } de nouveaux e-mails
   *[other] { $count } nouveaux e-mails
}
notify-and-more = { $count ->
    [one] et { $count } autre
    [many] et { $count } d’autres
   *[other] et { $count } autres
}
notify-no-subject = (aucun objet)
notify-unknown-sender = Expéditeur inconnu
notify-snooze-back = Retour de la mise en attente
notify-no-reply = Pas encore de réponse
notify-no-reply-to = Personne n’a répondu à « { $subject } ».
notify-tracking-opened = { $who } a ouvert { $subject }
notify-tracking-clicked = { $who } a cliqué sur un lien dans { $subject }

notify-update-ready = Katna Mail peut être mis à jour
notify-update-ready-body = La version { $version } est téléchargée. Mettre à jour l’installe et redémarre Katna Mail.
notify-update = Mettre à jour

## Its buttons

notify-open = Ouvrir
notify-reply-all = Répondre à tous
notify-mark-read = Marquer comme lu
notify-mark-all-read = Tout marquer comme lu
notify-archive = Archiver
