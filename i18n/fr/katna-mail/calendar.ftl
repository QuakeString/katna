# Katna Mail, French (Français): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Aujourd’hui
calendar-today-tip = Aller à aujourd’hui
calendar-view-day = Jour
calendar-view-week = Semaine
calendar-view-month = Mois
calendar-view-year = Année
calendar-view-schedule = Planning
calendar-view-days =
    { $count ->
        [one] { $count } jour
        [many] { $count } de jours
       *[other] { $count } jours
    }
calendar-options = Options
calendar-density = Densité
calendar-density-responsive = Adaptée à votre écran
calendar-density-comfortable = Confortable
calendar-density-compact = Compacte
calendar-custom-days = Vue personnalisée
calendar-second-zone = Deuxième fuseau horaire
calendar-zone-none = Aucun
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Partager mes disponibilités
calendar-free-subject = Mes disponibilités
calendar-free-intro = Voici quelques moments où je suis disponible ({ $zone }) :
calendar-free-day = { $weekday } { $date } : { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Je n’ai aucun créneau libre dans les prochains jours ouvrés.
calendar-previous-day = Jour précédent
calendar-next-day = Jour suivant
calendar-previous-week = Semaine précédente
calendar-next-week = Semaine suivante
calendar-previous-month = Mois précédent
calendar-next-month = Mois suivant
calendar-previous-year = Année précédente
calendar-next-year = Année suivante
calendar-previous-period = Plus tôt
calendar-next-period = Plus tard
calendar-title-months = { $first } – { $last }
calendar-loading = Chargement…
calendar-read-failed = Impossible de lire le calendrier : { $error }
calendar-sets = Groupes de calendriers
calendar-set-add = Enregistrer les calendriers affichés comme groupe
calendar-set-name = Nom du groupe
calendar-set-remove = Supprimer le groupe
calendar-local = Cet ordinateur
calendar-account-gone = Compte supprimé
calendar-birthdays = Anniversaires
calendar-birthday-of = Anniversaire de { $name }
calendar-empty-title = Aucun calendrier pour le moment
calendar-empty-text = Katna affiche ici les calendriers de vos comptes Google et Microsoft dès qu’ils sont synchronisés, ainsi que ceux des autres serveurs compatibles CalDAV.
calendar-schedule-empty = Rien de prévu pour les deux prochains mois.
calendar-search = Rechercher des événements
calendar-search-past = Événements passés
calendar-search-none = Aucun événement ne correspond à votre recherche.
calendar-no-title = (Sans titre)
calendar-all-day = Toute la journée
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } autres
calendar-repeats = Se répète
calendar-join = Participer
calendar-email-guests = Envoyer un e-mail aux invités
calendar-running-late = En retard
calendar-late-subject = En retard : { $title }
calendar-late-body = Toutes mes excuses, j’ai quelques minutes de retard pour { $title }. J’arrive bientôt.
calendar-guests =
    { $count ->
        [one] { $count } invité
        [many] { $count } d’invités
       *[other] { $count } invités
    }
calendar-guest-answers = { $yes } oui, { $maybe } peut-être, { $no } non, { $waiting } en attente
calendar-organizer = Organisateur
calendar-optional = Facultatif
calendar-open-web = Ouvrir dans le navigateur
calendar-open-contact = Ouvrir le contact
calendar-close = Fermer

## Adding, changing and deleting events.

calendar-add-title = Ajouter un titre
calendar-add-location = Ajouter un lieu
calendar-add-notes = Ajouter une description
calendar-add-guests = Ajouter des invités
calendar-remove-guest = Supprimer
calendar-add-meet = Ajouter une visioconférence Google Meet
calendar-add-teams = Ajouter une réunion Teams
calendar-has-call = Visioconférence ajoutée
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Toute la journée
calendar-more-options = Plus d’options
calendar-save = Enregistrer
calendar-saved = Événement enregistré
calendar-deleted = Événement supprimé
calendar-discard = Ignorer les modifications
calendar-edit = Modifier l’événement
calendar-delete = Supprimer l’événement
calendar-event-details = Détails de l’événement
calendar-kind-event = Événement
calendar-kind-focus = Temps de concentration
calendar-kind-out-of-office = Absent du bureau
calendar-kind-working-location = Lieu de travail
calendar-working-home = Domicile
calendar-busy = Occupé
calendar-free = Disponible
calendar-cancel = Annuler
calendar-ok = OK
calendar-read-only = Vous ne pouvez pas modifier les événements de cet agenda
calendar-none-editable = Aucun agenda auquel vous pouvez ajouter des événements pour le moment
calendar-no-such-time = Cette heure n’existe pas dans votre fuseau horaire
calendar-end-before-start = L’événement se termine avant son début
calendar-repeat-never = Ne se répète pas
calendar-repeat-daily = Tous les jours
calendar-repeat-weekly = Toutes les semaines le { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Tous les mois, le premier { $weekday }
        [2] Tous les mois, le deuxième { $weekday }
        [3] Tous les mois, le troisième { $weekday }
        [4] Tous les mois, le quatrième { $weekday }
       *[other] Tous les mois, le dernier { $weekday }
    }
calendar-repeat-yearly = Tous les ans le { $day }
calendar-repeat-weekdays = Tous les jours ouvrés (du lundi au vendredi)
calendar-repeat-custom = Personnalisé
calendar-reminder-none = Aucune notification
calendar-reminder-at-start = Au début
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minute avant
        [many] { $count } de minutes avant
       *[other] { $count } minutes avant
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } heure avant
        [many] { $count } d’heures avant
       *[other] { $count } heures avant
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } jour avant
        [many] { $count } de jours avant
       *[other] { $count } jours avant
    }
calendar-scope-edit-title = Modifier l’événement récurrent
calendar-scope-delete-title = Supprimer l’événement récurrent
calendar-scope-this = Cet événement
calendar-scope-following = Cet événement et les suivants
calendar-scope-all = Tous les événements
calendar-scope-respond-title = Réponse pour un événement récurrent
calendar-going = Participez-vous ?
calendar-answer-yes = Oui
calendar-answer-no = Non
calendar-answer-maybe = Peut-être
calendar-answered-yes = Vous participez
calendar-answered-no = Vous ne participez pas
calendar-answered-maybe = Vous participez peut-être

## The card at the top of a mail with an invitation.

calendar-invite = Invitation
calendar-invite-cancelled = Événement annulé
calendar-invite-reply = { $name } a répondu
calendar-invite-reply-yes = { $name } a accepté
calendar-invite-reply-no = { $name } a refusé
calendar-invite-reply-maybe = { $name } participera peut-être
calendar-invite-organizer = Organisé par { $name }
calendar-invite-open = Ouvrir dans Calendrier
calendar-invite-not-yet = Pas encore dans votre calendrier. Vous pourrez répondre une fois la synchronisation faite.
calendar-invite-by-mail = Absente de votre agenda : votre réponse est envoyée à l’organisateur par e-mail.
calendar-mail-yes = Accepté : { $title }
calendar-mail-yes-body = { $name } a accepté cette invitation.
calendar-mail-no = Refusé : { $title }
calendar-mail-no-body = { $name } a refusé cette invitation.
calendar-mail-maybe = Peut-être : { $title }
calendar-mail-maybe-body = { $name } a répondu « Peut-être » à cette invitation.
calendar-invite-your-day = Votre journée
calendar-invite-clashes =
    { $count ->
        [one] Chevauche { $count } événement
        [many] Chevauche { $count } d’événements
       *[other] Chevauche { $count } événements
    }

## The day's agenda beside the mail.

agenda-show = Afficher l’agenda du jour
agenda-hide = Masquer l’agenda
agenda-today = Aujourd’hui, { $date }
agenda-day = { $weekday } { $date }
agenda-empty = Rien de prévu ce jour-là.
