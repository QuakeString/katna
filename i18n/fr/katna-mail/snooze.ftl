# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
snooze-until = Mettre en attente jusqu’à…
snooze-later-today = Plus tard aujourd’hui
snooze-tomorrow = Demain
snooze-this-weekend = Ce week-end
snooze-next-week = La semaine prochaine
snooze-pick = Choisir la date et l’heure
snooze-cancel = Annuler
snooze-save = Enregistrer
snooze-in-the-past = Choisissez une heure plus tardive que maintenant.
follow-up-title = Me le rappeler sans réponse
follow-up-off = Ne pas me le rappeler
follow-up-days = { $days ->
    [one] Après { $days } jour
    [many] Après { $days } de jours
   *[other] Après { $days } jours
}

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Mettre en attente jusqu’à…
snooze-later-today = Plus tard aujourd’hui
snooze-tomorrow = Demain
snooze-this-weekend = Ce week-end
snooze-next-week = La semaine prochaine
snooze-pick = Choisir la date et l’heure
snooze-back = Retour aux heures
snooze-type-placeholder = Saisir une heure
snooze-type-hint = Par exemple « mar 15h », « demain » ou « dans 2 heures »
snooze-type-hint-unclear = Katna ne comprend pas cela comme une heure
snooze-type-unclear = « { $text } » n’est pas une heure que Katna connaît

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Mettre en attente
remind-tab = Me le rappeler
snooze-says = Le masque jusque-là
remind-says = Le laisse à sa place et vous prévient
remind-before-due = Avant l’échéance
remind-note = Note (facultatif)
remind-note-placeholder = L’objet, si laissé vide
toast-remind-set = Rappel prévu le { $date }
remind-chat-line = Rappel { $date } · { $title }
remind-done = Terminé
toast-remind-done = Rappel terminé
snooze-chat-line = En attente jusqu’au { $date }
snooze-chat-change = Modifier

## The date and time picker

snooze-cancel = Annuler
snooze-save = Enregistrer
snooze-in-the-past = Choisissez une heure plus tardive que maintenant.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Relancer sans réponse…
follow-up-title = Relancer sans réponse
follow-up-off = Désactivé
follow-up-days = { $days ->
    [one] { $days } jour
    [many] { $days } de jours
   *[other] { $days } jours
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } semaine
    [many] { $weeks } de semaines
   *[other] { $weeks } semaines
}
follow-up-pick = Choisir…
follow-up-pick-title = Relancer sans réponse avant le
follow-up-remind = Me le rappeler
follow-up-remind-note = La conversation revient en haut de votre boîte de réception
follow-up-send = Envoyer une relance à ma place
follow-up-send-note = Aux mêmes personnes, dans la même conversation
follow-up-send-encrypted = Pas pour les messages chiffrés
follow-up-text-placeholder = Ce que vous voulez écrire
follow-up-text-named = Bonjour { $name }, je me permets de vérifier que vous avez bien vu mon message ci-dessous.
follow-up-text = Bonjour, je me permets de vérifier que vous avez bien vu mon message ci-dessous.
follow-up-template = Utiliser un modèle
follow-up-signature = Votre signature est ajoutée
follow-up-again = Toujours sans réponse, relancer à nouveau après
follow-up-note = S’arrête dès que quelqu’un répond dans la conversation. Les réponses automatiques ne comptent pas.
follow-up-note-send = S’arrête dès que quelqu’un répond dans la conversation. Part en semaine de { $start } à { $end }, et jamais avec plus d’un jour de retard.
follow-up-cancel = Annuler
follow-up-done = Terminé
follow-up-chip-send = Relance dans { $time }
follow-up-chip-remind = Rappel dans { $time }
follow-up-chip-send-on = Relance { $date }
follow-up-chip-remind-on = Rappel { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Pas encore de réponse
follow-up-card-title-waiting = Votre relance est en attente
follow-up-card-send = Katna envoie votre relance le { $date }. Elle s’arrête dès que quelqu’un répond.
follow-up-card-send-twice = Katna envoie votre relance le { $date }, puis une fois de plus plus tard. Elle s’arrête dès que quelqu’un répond.
follow-up-card-remind = Si personne ne répond, cette conversation revient dans votre boîte de réception le { $date }.
follow-up-card-waiting = Elle est arrivée à échéance pendant que votre ordinateur était éteint, elle n’a donc pas été envoyée en retard. Envoyez-la maintenant, choisissez une nouvelle heure ou arrêtez-la.
follow-up-card-edit = Modifier
follow-up-card-edit-title = Relancer le
follow-up-card-send-now = Envoyer maintenant
follow-up-card-stop = Arrêter
follow-up-chat-send = Relance · { $date } si personne ne répond
follow-up-chat-step = Relance { $step } sur { $steps } · { $date } si personne ne répond
follow-up-chat-waiting = Relance en attente · arrivée à échéance pendant que votre ordinateur était éteint
follow-up-chat-remind = De retour dans la boîte de réception { $date } sans réponse
toast-follow-up-sent = Relance envoyée
toast-follow-up-stopped = Relance arrêtée
toast-follow-up-moved = Relance déplacée au { $date }

nudge-row = Envoyé { $days ->
    [one] il y a 1 jour
    [many] il y a { $days } de jours
   *[other] il y a { $days } jours
}. Relancer ?
nudge-row-tip = Écrire une relance à tous les participants
nudge-follow-up = Relancer
nudge-dismiss = Ignorer
nudge-card-title = Pas encore de réponse
nudge-card-text = Vous avez posé une question { $days ->
    [one] il y a 1 jour
    [many] il y a { $days } de jours
   *[other] il y a { $days } jours
} et personne n’a répondu.
nudge-chat-line = Envoyé { $days ->
    [one] il y a 1 jour
    [many] il y a { $days } de jours
   *[other] il y a { $days } jours
}, pas encore de réponse
toast-nudge-dismissed = Relance ignorée
