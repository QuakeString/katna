# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Informazioni su Katna
about-tagline = Posta e calendario per il desktop Linux
about-version = Katna Mail { $version }
about-copy-version = Copia i dettagli della versione
about-version-copied = Copiato
about-version-built = Compilato: { $date }
about-version-system = Sistema: { $system }
about-whats-new = Novità

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = Gli aggiornamenti non sono ancora stati controllati
about-update-checking = Controllo aggiornamenti…
about-update-up-to-date = Katna Mail è aggiornato
about-update-check-failed = Impossibile controllare gli aggiornamenti
about-update-available = La versione { $version } è disponibile
about-update-downloading = Scaricamento della versione { $version }… { $percent }%
about-update-download-failed = Lo scaricamento della versione { $version } non è stato completato
about-update-ready = La versione { $version } è pronta per l’installazione
about-update-ready-detail = Katna Mail si riavvia per completare l’aggiornamento.
about-update-confirm = Installare la versione { $version }?
about-update-confirm-detail = Katna Mail si chiuderà, installerà l’aggiornamento e si aprirà di nuovo da dove avevi interrotto. Il computer chiederà la tua password.
about-update-confirm-detail-windows = Katna Mail si chiuderà, installerà l’aggiornamento e si riaprirà tra un momento.
about-update-installing = Installazione della versione { $version }…
about-update-installing-detail = Inserisci la password nella finestra che si è aperta.
about-update-installing-detail-windows = Katna Mail si chiude ora e si riapre una volta installato l’aggiornamento.
about-update-cancelled = L’aggiornamento non è stato installato, perché la password non è stata fornita.
about-update-failed = Non è stato possibile installare l’aggiornamento: { $error }
about-update-not-self-updating = Questa copia di Katna Mail non si aggiorna da sola. Aggiornala nello stesso modo in cui l’hai installata.
about-update-restart-failed = L’aggiornamento è installato, ma Katna Mail non è riuscito ad aprirsi di nuovo ({ $error }). Aprilo tu stesso.
about-update-check = Controlla aggiornamenti
about-update-download = Scarica
about-update-retry = Riprova
about-update-button = Aggiorna
about-update-restart = Aggiorna e riavvia
about-update-cancel = Non ora
about-changelog = Registro delle modifiche
about-source = Codice sorgente
about-coffee = Offrimi un caffè
about-coffee-coffee = Un caffè?
about-coffee-tea = Un tè?
about-coffee-pizza = Una pizza?
about-coffee-nothing = Niente? Proprio niente?
about-coffee-water = Sopravvivo con l’acqua!!
about-coffee-thanks = Grazie per usare Katna
about-coming-soon = Prossimamente
about-follow-me = Seguimi su
about-love-title = Fatto con amore per Rust, KDE e Linux
about-love-text = Con Rust scrivere un’app di posta veloce e sicura è un piacere: Katna non contiene codice unsafe. Il desktop Plasma di KDE e la sua suite PIM hanno ispirato Katna, e Linux e la comunità del software libero sono le fondamenta su cui poggia. Grazie, e grazie alle librerie qui sotto.
about-kde-text = KDE crea il desktop su cui Katna si sente più a casa, ed è fatto da volontari e finanziato da persone come te. Se ti piacciono Plasma o le app di KDE, valuta una donazione a KDE.
about-donate-kde = Fai una donazione a KDE
about-gpui-title = Costruito su GPUI, dal progetto Zed
about-gpui-text = Tutta l’interfaccia di Katna Mail è costruita su GPUI, il framework per interfacce veloce e accelerato dalla GPU che Zed Industries ha creato per l’editor Zed. Ogni pixel, animazione e finestra che vedi è disegnato da GPUI. Grazie, team di Zed, per averlo sviluppato alla luce del sole. Apache-2.0.
about-gpui-github = GPUI su GitHub
about-personal-title = Un progetto personale
about-personal-text = Katna Mail non cerca di essere nuova o rivoluzionaria. È l’app di posta che il suo autore desiderava, e funzioni e aspetto sono presi in prestito da Gmail, Mailspring e Thunderbird. È stata possibile solo grazie ai progressi fatti dagli LLM.
about-built-on = COSTRUITO CON SOFTWARE LIBERO
about-credit-pimalaya = IMAP, SMTP e accesso (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Lettura e scrittura di IMAP
about-credit-tantivy = Ricerca
about-credit-sqlite = L’archivio della posta
about-credit-rustls = Connessioni sicure
about-credit-mail-parser = Lettura della posta, da Stalwart Labs
about-credit-html5ever = Posta HTML, dal progetto Servo
about-credit-zbus = Comunicazione con il desktop tramite D-Bus e portali
about-credit-oo7 = Password nel portachiavi del desktop
about-credit-hayro = Visualizzazione e stampa di PDF
about-credit-calamine = Anteprime dei fogli di calcolo
about-credit-resvg = Immagini SVG
about-credit-jiff = Date e fusi orari
about-credit-spellbook = Controllo ortografico, dall’editor Helix
about-credit-smol = Fare tante cose insieme
about-credit-color-schemes = Le tavolozze delle combinazioni di colori integrate
about-all-libraries = Tutte le librerie usate da Katna ({ $count })
about-library-authors = di { $authors }
about-license = Katna è software libero rilasciato sotto la GNU GPL, versione 3 o successiva.
about-close = Chiudi

## What’s new (shown after an update)

whats-new-title = Novità di Katna Mail
whats-new-updated = Aggiornato alla versione { $version }
whats-new-version = Versione { $version }
whats-new-more = { $count ->
    [one] E un’altra novità nel registro completo delle modifiche.
    [many] E altre { $count } di novità nel registro completo delle modifiche.
   *[other] E altre { $count } novità nel registro completo delle modifiche.
}
whats-new-changelog = Registro completo delle modifiche
whats-new-got-it = Ho capito

## First run: welcome page

onboarding-welcome-title = Benvenuto in Katna Mail
onboarding-welcome-lead = La tua posta sul tuo computer: veloce da cercare, leggibile offline e privata.
onboarding-fast-title = Veloce, anche offline
onboarding-fast-text = Katna conserva qui una copia della tua posta, così aprirla e cercarla è immediato, con o senza connessione.
onboarding-providers-title = Funziona con la tua posta
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud e qualsiasi altro account IMAP o POP.
onboarding-private-title = Privata
onboarding-private-text = La tua posta arriva direttamente dal tuo provider a questo computer. Nessun server di Katna la vede.
onboarding-get-started = Inizia

## First run: adding an account

onboarding-service-checking = Controllo del servizio in background di Katna…
onboarding-service-running = Il servizio in background di Katna è in esecuzione.
onboarding-service-missing = Il servizio in background di Katna non è in esecuzione
onboarding-service-start = Scarica e invia la tua posta. Avvialo da un terminale, poi controlla di nuovo:
onboarding-check-again = Controlla di nuovo
onboarding-account-title = Aggiungi il tuo account di posta
onboarding-account-lead = Scrivi il tuo indirizzo email e la password, e Katna trova le impostazioni del server. Gmail, Yahoo e iCloud richiedono una password per le app, da creare nelle impostazioni di sicurezza del tuo account.
onboarding-add-account = Aggiungi un account
onboarding-back = Indietro

## First run: choosing the look

onboarding-look-title = Rendila tua
onboarding-look-lead = Scegli come si apre la posta e l’aspetto di Katna. Puoi cambiarli quando vuoi nelle impostazioni rapide.
onboarding-reading-pane = Riquadro di lettura
onboarding-pane-right = A destra dell’elenco
onboarding-pane-none = Nessuna suddivisione
onboarding-theme = Tema
onboarding-theme-system = Sistema
onboarding-theme-light = Chiaro
onboarding-theme-dark = Scuro
onboarding-density = Densità
onboarding-density-default = Predefinita
onboarding-density-compact = Compatta
onboarding-continue = Continua

## First start: the Katna account page. A Katna account is an account on
## Katna's own server, not a mail account; see katna-account.ftl.

onboarding-katna-title = Ottieni di più con un account Katna
onboarding-katna-lead = È facoltativo. Attiva le funzioni online di Katna e puoi crearlo più tardi in Impostazioni > Abbonamento.
onboarding-katna-receipts-title = Conferme di lettura
onboarding-katna-receipts-text = Scopri quando le persone aprono la posta che invii.
onboarding-katna-links-title = Tracciamento dei link
onboarding-katna-links-text = Scopri quali link nella tua posta vengono cliccati.
onboarding-katna-activity-title = Attività
onboarding-katna-activity-text = Aperture e clic di tutto ciò che hai inviato, in un unico posto.
onboarding-katna-translate-title = Traduzione automatica
onboarding-katna-translate-text = Leggi nella tua lingua la posta scritta in altre lingue.
onboarding-katna-private = Ha una propria password. Le credenziali della tua posta non lasciano mai questo computer.

## First run: done

onboarding-ready-title = Tutto pronto
onboarding-ready-lead = Katna sta scaricando la tua posta. I messaggi compaiono man mano che arrivano, e la nuova posta appare da sola.
onboarding-ready-lead-address = Katna sta scaricando la posta di { $address }. I messaggi compaiono man mano che arrivano, e la nuova posta appare da sola.
onboarding-apps = App che userai
onboarding-ready-tour = Vuoi fare un tour di un minuto per vedere dove si trova tutto?
onboarding-skip = Salta per ora
onboarding-take-tour = Fai il tour

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Aiuta a migliorare Katna
share-lead = Quando Katna si arresta in modo anomalo, salva un rapporto su questo computer. Inviare questi rapporti aiuta a correggere ciò che non ha funzionato. Puoi cambiare questa scelta quando vuoi in Impostazioni > Feedback degli utenti.
share-sent = Cosa viene inviato
share-sent-detail = Il rapporto sull’arresto anomalo così come puoi vederlo nelle Impostazioni: cosa si è arrestato e in quale punto di Katna, la versione, il tuo sistema Linux e il desktop, e le ultime righe del registro di Katna, che possono contenere nomi di cartelle di posta.
share-never-sent = Cosa non viene mai inviato
share-never-sent-detail = I tuoi messaggi, contatti, password, indirizzo IP, nome utente o nome del computer. Gli indirizzi email vengono rimossi dal rapporto.
share-where = Dove finisce
share-where-detail = Nel sistema di tracciamento degli arresti anomali di Katna su Sentry, conservato nell’UE. Nessun ID collega i rapporti a te.
share-dont-send = Non inviare
share-send = Invia i rapporti sugli arresti anomali
share-sending = I rapporti sugli arresti anomali verranno inviati. Grazie.
share-local = I rapporti sugli arresti anomali restano su questo computer.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Benvenuto in Katna Mail
tour-welcome-text = Un tour di un minuto mostra dove si trova tutto.
tour-not-now = Non ora
tour-start = Fai il tour
tour-close = Chiudi
tour-skip = Salta il tour
tour-back = Indietro
tour-done = Fine
tour-next = Avanti
tour-step = { $step } di { $total }
tour-compose-title = Scrivi un messaggio
tour-compose-text = Scrivi apre un nuovo messaggio in basso a destra, così puoi continuare a leggere mentre scrivi.
tour-search-title = Cerca in tutta la tua posta
tour-search-text = La ricerca funziona anche offline. Il pulsante all’estremità destra aggiunge filtri: mittente, destinatario, oggetto, date e allegati.
tour-menu-title = Mostra o nascondi le cartelle
tour-menu-text = Questo pulsante ripiega l’elenco delle cartelle. Mentre è nascosto, appoggia il puntatore su Posta a sinistra per vedere le cartelle.
tour-apps-title = Le tue app
tour-apps-text = La posta si trova qui, accanto a Calendario, Contatti, Attività, Note e File.
tour-tabs-title = Schede della Posta in arrivo
tour-tabs-text = La nuova posta viene suddivisa in Principale, Promozioni, Social, Aggiornamenti e Forum. Puoi disattivare le schede nelle impostazioni rapide.
tour-list-title = I tuoi messaggi
tour-list-text = Fai clic su un messaggio per leggerlo. Passaci sopra con il puntatore per le azioni rapide, fai clic con il tasto destro per altre opzioni, oppure selezionane diversi per agire su tutti insieme.
tour-settings-title = Impostazioni rapide
tour-settings-text = Qui puoi cambiare il riquadro di lettura, la densità e il tema. Da qui puoi anche riavviare il tour.
tour-account-title = Il tuo account
tour-account-text = Guarda in quale account ti trovi e aggiungine un altro.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Il servizio in background di Katna si è arrestato in modo imprevisto.
    [one] Il servizio in background di Katna si è arrestato in modo imprevisto. È salvato un altro rapporto sugli arresti anomali.
    [many] Il servizio in background di Katna si è arrestato in modo imprevisto. Sono salvati altri { $more } di rapporti sugli arresti anomali.
   *[other] Il servizio in background di Katna si è arrestato in modo imprevisto. Sono salvati altri { $more } rapporti sugli arresti anomali.
}
crash-mail = { $more ->
    [0] L’ultima volta Katna Mail si è chiusa in modo imprevisto.
    [one] L’ultima volta Katna Mail si è chiusa in modo imprevisto. È salvato un altro rapporto sugli arresti anomali.
    [many] L’ultima volta Katna Mail si è chiusa in modo imprevisto. Sono salvati altri { $more } di rapporti sugli arresti anomali.
   *[other] L’ultima volta Katna Mail si è chiusa in modo imprevisto. Sono salvati altri { $more } rapporti sugli arresti anomali.
}
crash-view = Visualizza il rapporto
crash-view-tooltip = Apri il rapporto, salvato su questo computer
crash-copy = Copia il rapporto
crash-close = Chiudi

## Sign in again (a bar at the bottom when Google or Microsoft stopped
## letting an account in; $provider: Google or Microsoft)

sign-in-again-button = Accedi
sign-in-again-tooltip = Apri la pagina di accesso di { $provider } nel browser
sign-in-again-waiting = In attesa del browser…
google-api-off = { $api } è disattivata nel progetto Google Cloud di Katna.
google-api-turn-on = Attiva
google-api-turn-on-tooltip = Apri Google Cloud per attivare { $api }, poi premi Riprova
sign-in-again-done = Accesso a { $address } eseguito di nuovo. Scaricamento della posta…

## Before deleting several conversations, or deleting for good

delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Spostare questa conversazione nel Cestino?
        [many] Spostare { $count } di conversazioni nel Cestino?
       *[other] Spostare { $count } conversazioni nel Cestino?
    }
   *[message] { $count ->
        [one] Spostare questo messaggio nel Cestino?
        [many] Spostare { $count } di messaggi nel Cestino?
       *[other] Spostare { $count } messaggi nel Cestino?
    }
}
delete-ask-body = { $count ->
    [one] Puoi annullare subito dopo, oppure recuperarla dal Cestino più tardi.
    [many] Puoi annullare subito dopo, oppure recuperarle dal Cestino più tardi.
   *[other] Puoi annullare subito dopo, oppure recuperarle dal Cestino più tardi.
}
delete-ask-confirm = Sposta nel Cestino
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Eliminare definitivamente questa conversazione?
        [many] Eliminare definitivamente { $count } di conversazioni?
       *[other] Eliminare definitivamente { $count } conversazioni?
    }
   *[message] { $count ->
        [one] Eliminare definitivamente questo messaggio?
        [many] Eliminare definitivamente { $count } di messaggi?
       *[other] Eliminare definitivamente { $count } messaggi?
    }
}
delete-forever-body = { $count ->
    [one] L’eliminazione avviene anche sul server e non si può annullare.
    [many] L’eliminazione avviene anche sul server e non si può annullare.
   *[other] L’eliminazione avviene anche sul server e non si può annullare.
}
delete-forever-confirm = Elimina definitivamente
delete-ask-dont-ask = Non chiedere più
delete-ask-cancel = Annulla
