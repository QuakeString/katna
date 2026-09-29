# Katna Mail, Italian (Italiano): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Oggi
calendar-today-tip = Vai a oggi
calendar-view-day = Giorno
calendar-view-week = Settimana
calendar-view-month = Mese
calendar-view-schedule = Agenda
calendar-previous-day = Giorno precedente
calendar-next-day = Giorno successivo
calendar-previous-week = Settimana precedente
calendar-next-week = Settimana successiva
calendar-previous-month = Mese precedente
calendar-next-month = Mese successivo
calendar-previous-period = Prima
calendar-next-period = Dopo
calendar-title-months = { $first } – { $last }
calendar-loading = Caricamento in corso…
calendar-read-failed = Impossibile leggere il calendario: { $error }
calendar-local = Questo computer
calendar-account-gone = Account rimosso
calendar-empty-title = Ancora nessun calendario
calendar-empty-text = Katna mostra qui i calendari dei tuoi account Google e Microsoft non appena sono sincronizzati, e quelli di altri server che offrono CalDAV.
calendar-schedule-empty = Niente in programma per i prossimi due mesi.
calendar-no-title = (Senza titolo)
calendar-all-day = Tutto il giorno
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = Altri { $count }
calendar-repeats = Si ripete
calendar-join = Partecipa
calendar-guests =
    { $count ->
        [one] { $count } ospite
        [many] { $count } di ospiti
       *[other] { $count } ospiti
    }
calendar-guest-answers = { $yes } sì, { $maybe } forse, { $no } no, { $waiting } in attesa
calendar-organizer = Organizzatore
calendar-optional = Facoltativo
calendar-open-web = Apri nel browser
calendar-close = Chiudi

## Adding, changing and deleting events.

calendar-add-title = Aggiungi titolo
calendar-add-location = Aggiungi luogo
calendar-add-notes = Aggiungi descrizione
calendar-add-guests = Aggiungi ospiti
calendar-remove-guest = Rimuovi
calendar-add-meet = Aggiungi videoconferenza Google Meet
calendar-add-teams = Aggiungi riunione Teams
calendar-has-call = Videoconferenza aggiunta
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Tutto il giorno
calendar-more-options = Altre opzioni
calendar-save = Salva
calendar-saved = Evento salvato
calendar-deleted = Evento eliminato
calendar-discard = Ignora modifiche
calendar-edit = Modifica evento
calendar-delete = Elimina evento
calendar-event-details = Dettagli evento
calendar-kind-event = Evento
calendar-kind-focus = Tempo di concentrazione
calendar-kind-out-of-office = Fuori sede
calendar-kind-working-location = Luogo di lavoro
calendar-working-home = Casa
calendar-busy = Occupato
calendar-free = Libero
calendar-cancel = Annulla
calendar-ok = OK
calendar-read-only = Non puoi modificare gli eventi di questo calendario
calendar-none-editable = Ancora nessun calendario a cui puoi aggiungere eventi
calendar-no-such-time = Quell’ora non esiste nel tuo fuso orario
calendar-end-before-start = L’evento termina prima di iniziare
calendar-repeat-never = Non si ripete
calendar-repeat-daily = Ogni giorno
calendar-repeat-weekly = Ogni settimana il { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Ogni mese: { $weekday }, prima settimana
        [2] Ogni mese: { $weekday }, seconda settimana
        [3] Ogni mese: { $weekday }, terza settimana
        [4] Ogni mese: { $weekday }, quarta settimana
       *[other] Ogni mese: { $weekday }, ultima settimana
    }
calendar-repeat-yearly = Ogni anno il { $day }
calendar-repeat-weekdays = Ogni giorno feriale (dal lunedì al venerdì)
calendar-repeat-custom = Personalizzato
calendar-reminder-none = Nessuna notifica
calendar-reminder-at-start = All’inizio
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minuto prima
        [many] { $count } di minuti prima
       *[other] { $count } minuti prima
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } ora prima
        [many] { $count } di ore prima
       *[other] { $count } ore prima
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } giorno prima
        [many] { $count } di giorni prima
       *[other] { $count } giorni prima
    }
calendar-scope-edit-title = Modifica evento ricorrente
calendar-scope-delete-title = Elimina evento ricorrente
calendar-scope-this = Questo evento
calendar-scope-following = Questo evento e quelli successivi
calendar-scope-all = Tutti gli eventi
calendar-scope-respond-title = Risposta per un evento ricorrente
calendar-going = Partecipi?
calendar-answer-yes = Sì
calendar-answer-no = No
calendar-answer-maybe = Forse
calendar-answered-yes = Partecipi
calendar-answered-no = Non partecipi
calendar-answered-maybe = Forse partecipi

## The card at the top of a mail with an invitation.

calendar-invite = Invito
calendar-invite-cancelled = Evento annullato
calendar-invite-reply = { $name } ha risposto
calendar-invite-reply-yes = { $name } ha accettato
calendar-invite-reply-no = { $name } ha rifiutato
calendar-invite-reply-maybe = { $name } forse partecipa
calendar-invite-organizer = Organizzato da { $name }
calendar-invite-open = Apri in Calendario
calendar-invite-not-yet = Non ancora nel tuo calendario. Potrai rispondere dopo la sincronizzazione.
calendar-invite-by-mail = Non è nel tuo calendario: la tua risposta arriva all’organizzatore via e-mail.
calendar-mail-yes = Accettato: { $title }
calendar-mail-yes-body = { $name } ha accettato questo invito.
calendar-mail-no = Rifiutato: { $title }
calendar-mail-no-body = { $name } ha rifiutato questo invito.
calendar-mail-maybe = Con riserva: { $title }
calendar-mail-maybe-body = { $name } ha accettato questo invito con riserva.
calendar-invite-your-day = La tua giornata
calendar-invite-clashes =
    { $count ->
        [one] In conflitto con { $count } evento
        [many] In conflitto con { $count } di eventi
       *[other] In conflitto con { $count } eventi
    }

## The day's agenda beside the mail.

agenda-show = Mostra l’agenda del giorno
agenda-hide = Nascondi l’agenda
agenda-today = Oggi, { $date }
agenda-day = { $weekday } { $date }
agenda-empty = Niente in programma per questo giorno.
