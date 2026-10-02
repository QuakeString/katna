# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

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

## Reminders the user asked for (same buttons)

notify-snooze-back = Retour de la mise en attente
notify-no-reply = Pas encore de réponse
notify-no-reply-to = Personne n’a répondu à « { $subject } ».

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } a ouvert { $subject }
notify-tracking-clicked = { $who } a cliqué sur un lien dans { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail peut être mis à jour
notify-update-ready-body = La version { $version } est téléchargée. Mettre à jour l’installe et redémarre Katna Mail.
notify-update = Mettre à jour

## Reminders of calendar events

notify-event-now = Maintenant
notify-event-in-minutes = { $count ->
    [one] Dans { $count } minute
   *[other] Dans { $count } minutes
}
notify-event-in-hours = { $count ->
    [one] Dans { $count } heure
   *[other] Dans { $count } heures
}
notify-event-in-days = { $count ->
    [1] Demain
    [one] Dans { $count } jour
   *[other] Dans { $count } jours
}
notify-event-all-day = Toute la journée
notify-event-join = Participer
notify-event-snooze = Répéter 5 min
notify-task-done = Marquer comme terminée

## The buttons of new-mail notifications and reminders

notify-open = Ouvrir
notify-peek = Aperçu
notify-reply = Répondre
notify-reply-placeholder = Répondre à { $name }…
notify-send = Envoyer
notify-reply-all = Répondre à tous
notify-mark-read = Marquer comme lu
notify-mark-all-read = Tout marquer comme lu
notify-archive = Archiver

## After Archive on a notification: a short note in the same place

notify-archived = Archivé
notify-archived-count = { $count ->
    [one] { $count } message sorti de la boîte de réception
    [many] { $count } de messages sortis de la boîte de réception
   *[other] { $count } messages sortis de la boîte de réception
}
notify-undo = Annuler

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Réponse envoyée à { $name }
notify-open-in-katna = Ouvrir dans Katna
