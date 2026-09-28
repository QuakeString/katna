# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Generali
settings-tab-inbox = Posta in arrivo
settings-tab-accounts = Account
settings-tab-katna-account = Account Katna
settings-tab-subscriptions = Abbonamento
settings-tab-appearance = Aspetto
settings-tab-shortcuts = Scorciatoie
settings-tab-default-apps = App predefinite
settings-tab-folders-rules = Cartelle e regole
settings-tab-compose = Scrittura
settings-tab-mcp-server = Server MCP
settings-tab-feedback = Feedback degli utenti
settings-tab-experimental = Sperimentali

## Settings page: tabs still to come

settings-tab-folders-rules-coming = Crea, rinomina, sposta e nascondi cartelle ed etichette e scegli quali sincronizzare. Le regole ordinano, etichettano, inoltrano o eliminano automaticamente la nuova posta, in base a mittente, oggetto o parole.
settings-tab-mcp-server-coming = Consenti agli assistenti IA su questo computer di cercare, leggere e scrivere bozze della tua posta, con il tuo consenso.

## Settings > General

settings-general-conversations = Visualizzazione per conversazione
settings-general-conversations-group = Raggruppa le risposte alla stessa email
settings-general-conversations-group-detail = Una riga per conversazione nell’elenco
settings-general-reading = Lettura
settings-general-newest-first = Prima il messaggio più recente
settings-general-newest-first-detail = Una conversazione inizia con l’ultima risposta
settings-general-full-headers = Mostra intestazioni complete
settings-general-full-headers-detail = Da, a, cc, data e oggetto visibili in ogni messaggio
settings-general-full-names = Nomi completi dei destinatari
settings-general-full-names-detail = «a me, Ada Lovelace» anziché «a me, Ada»
settings-translation = Traduzione
settings-translation-detail = Le email in un’altra lingua si possono leggere nella tua.
settings-translation-offer = Offri la traduzione
settings-translation-offer-detail = Il testo di un messaggio va al server di Katna per essere tradotto solo quando lo chiedi o se traduci sempre la sua lingua. Gli allegati non vengono mai inviati.
settings-translation-reading = Traduci in
settings-translation-always = Traduci sempre
settings-translation-never = Non offrire mai per
settings-translation-none = Ancora nessuna. Scegli dalla barra Traduci di un messaggio.
settings-general-mark-read = Segna come già letto
settings-general-mark-read-now = Appena si apre
settings-general-mark-read-1s = Dopo 1 secondo di apertura
settings-general-mark-read-3s = Dopo 3 secondi di apertura
settings-general-mark-read-never = Solo quando la segno come già letta
settings-general-auto-advance = Avanzamento automatico
settings-general-auto-advance-detail = Dopo aver eliminato, archiviato o spostato la conversazione aperta
settings-general-auto-advance-next = Apri la conversazione successiva
settings-general-auto-advance-previous = Apri la conversazione precedente
settings-general-auto-advance-list = Torna all’elenco
settings-general-reply-button = Pulsante Rispondi
settings-general-reply-all = Rispondi a tutti
settings-general-reply-all-detail = Il pulsante di risposta accanto a ogni messaggio risponde a tutti, non solo al mittente
settings-general-remote-images = Immagini dal web
settings-general-remote-images-detail = Caricare le immagini di un messaggio fa sapere al mittente che l’hai aperto, quando e più o meno dove. Se disattivato, ogni messaggio chiede prima, e puoi sempre mostrare le immagini di un mittente.
settings-general-remote-images-always = Mostra sempre le immagini
settings-general-remote-images-always-detail = In ogni messaggio, non solo da mittenti attendibili
settings-general-sending = Invio
settings-general-sending-detail = Quanto attende un messaggio inviato, per poterlo annullare.
settings-general-offline = Posta offline
settings-general-offline-detail = La posta recente viene scaricata per intero, per leggerla senza connessione. La posta meno recente viene scaricata quando la apri.
settings-general-offline-days = { $count ->
    [one] { $count } giorno
    [many] { $count } di giorni
   *[other] { $count } giorni
}
settings-general-offline-years = { $count ->
    [one] { $count } anno
    [many] { $count } di anni
   *[other] { $count } anni
}
settings-general-offline-all = Tutta la posta
settings-general-offline-note = Scegliendo meno giorni, la posta già scaricata viene mantenuta. Sul server non cambia nulla.
settings-general-notifications = Notifiche
settings-general-notifications-detail = Per la nuova posta in Posta in arrivo, anche quando Katna Mail è chiuso.
settings-general-new-mail = Avvisami quando arriva nuova posta
settings-general-new-mail-detail = Con Rispondi a tutti, Segna come già letto e Archivia
settings-general-new-mail-sound = Riproduci un suono
settings-general-new-mail-sound-detail = Il suono di nuova posta del desktop
settings-general-reset-cache = Reimposta la cache
settings-general-reset-cache-detail = Quando la posta sembra sbagliata o non aggiornata, o per liberare spazio su disco. Sui tuoi server di posta non cambia nulla.
settings-general-desktop = Desktop
settings-general-start-at-login = Avvia Katna all’accesso
settings-general-start-at-login-detail = Sincronizza la posta e mostra le notifiche di nuova posta e l’icona nell’area di notifica, senza aprire la finestra
settings-general-login-window = Apri anche la finestra di Katna Mail
settings-general-login-window-detail = All’accesso si apre anche la finestra
settings-general-tray = Mostra Katna nell’area di notifica
settings-general-tray-detail = Con il numero di messaggi da leggere e un menu
settings-general-unread-badge = Numero di messaggi da leggere sull’icona nella barra delle applicazioni
settings-general-unread-badge-detail = Quanti messaggi in Posta in arrivo sono da leggere
settings-general-search-triggers = Cerca dal desktop
settings-general-search-triggers-detail = Digita una di queste parole e uno spazio in KRunner o nella ricerca di GNOME, poi ciò che vuoi trovare, per cercare nella tua posta come fa qui la casella di ricerca. Separa le parole con virgole.
settings-general-search-triggers-none = Nessuna parola; funziona solo «mail:»

## Settings > Inbox

settings-inbox-tabs = Schede della posta in arrivo
settings-inbox-tabs-detail = Suddividi la posta in arrivo in schede, come fa il sito web del tuo provider di posta.
settings-inbox-tabs-show = Mostra le schede della posta in arrivo
settings-inbox-tabs-show-detail = Se disattivato, un unico elenco per tutti gli account
settings-inbox-no-accounts = Aggiungi un account per sceglierne le schede.
settings-inbox-tabs-automatic = Automatico: { $tabs } ({ $provider })
settings-inbox-tabs-off = Nessuna scheda
settings-inbox-tabs-gmail = Principale, Promozioni, Social, Aggiornamenti, Forum
settings-inbox-tabs-focused = Evidenziata e Altra
settings-inbox-tabs-zoho = Posta in arrivo, Newsletter e Notifiche
settings-inbox-tabs-shown = Schede mostrate. La posta di una scheda disattivata resta in { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Riquadro di lettura
settings-appearance-reading-pane-detail = Dove viene mostrata una conversazione aperta.
settings-appearance-pane-right = A destra dell’elenco
settings-appearance-pane-none = Nessuna suddivisione
settings-appearance-density = Densità
settings-appearance-density-default = Predefinita
settings-appearance-density-compact = Compatta
settings-appearance-scaling = Scala
settings-appearance-scaling-detail = Ingrandisce o rimpicciolisce tutto in Katna Mail, oltre alla scala del desktop: testo, icone, spaziatura e divisori. La posta che invii mantiene la propria dimensione del carattere. Con dimensioni molto piccole può essere difficile fare clic sulle icone.
settings-appearance-theme = Tema
settings-appearance-theme-system = Sistema
settings-appearance-theme-light = Chiaro
settings-appearance-theme-dark = Scuro
settings-appearance-desktop-colors = Colori del desktop
settings-appearance-desktop-colors-use = Usa i colori del desktop
settings-appearance-desktop-colors-use-detail = Lo schema di colori e il colore di risalto del desktop
settings-appearance-app-names = Nomi delle app
settings-appearance-app-names-show = Mostra i nomi delle app
settings-appearance-app-names-show-detail = Nomi sotto le icone delle app all’estrema sinistra
settings-appearance-sender-pictures = Immagini dei mittenti
settings-appearance-sender-pictures-show = Mostra i loghi aziendali
settings-appearance-sender-pictures-show-detail = Cercati in base al dominio del mittente, mai in base al messaggio, e conservati per una settimana
settings-appearance-important = Indicatori di importanza
settings-appearance-important-show = Mostra gli indicatori di importanza
settings-appearance-important-show-detail = Accanto a ogni messaggio nell’elenco
settings-appearance-message-width = Larghezza dei messaggi
settings-appearance-message-width-limit = Limita la larghezza dei messaggi
settings-appearance-message-width-limit-detail = Le righe lunghe sono più facili da leggere in una finestra larga
settings-appearance-mail-colors = Colori della posta
settings-appearance-mail-colors-detail = La maggior parte della posta è pensata per una pagina bianca. Con un tema scuro i suoi colori vengono sostituiti da colori scuri ben leggibili; se disattivato, mantiene i colori del mittente su una pagina chiara.
settings-appearance-dark-mail = Colori scuri anche per la posta
settings-appearance-dark-mail-detail = Solo con il tema scuro
settings-appearance-attachment-previews = Anteprime degli allegati
settings-appearance-attachment-previews-show = Mostra le anteprime degli allegati
settings-appearance-attachment-previews-show-detail = Una piccola immagine del contenuto di ogni file nel suo riquadro

## Settings > Default apps

settings-default-apps-intro = Dove si aprono gli allegati quando fai clic su di essi. Il visualizzatore può sempre aprire un file anche in un’altra app. Le app predefinite del desktop si impostano nelle sue impostazioni.
settings-default-apps-pdf = File PDF
settings-default-apps-pdf-detail = Pagine, con zoom.
settings-default-apps-pictures = Immagini
settings-default-apps-pictures-detail = Foto (raddrizzate), PNG, GIF, WebP, BMP, TIFF e SVG.
settings-default-apps-text = File di testo
settings-default-apps-text-detail = Testo semplice, log, codice e altro testo.
settings-default-apps-sheets = Fogli di calcolo
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) e CSV.
settings-default-apps-documents = Documenti
settings-default-apps-documents-detail = Word (docx, doc), testo OpenDocument (odt) e presentazioni (pptx, ppt, odp).
settings-default-apps-katna = Visualizzatore di Katna Mail
settings-default-apps-system = App predefinita del desktop
settings-default-apps-ask = Chiedi ogni volta quale app usare
settings-default-apps-after-saving = Dopo il salvataggio
settings-default-apps-show-folder = Mostra i file salvati nella loro cartella
settings-default-apps-show-folder-detail = Apre il gestore di file con gli allegati salvati selezionati

## Settings > Compose

settings-compose-send-from = Invia nuovi messaggi da
settings-compose-send-from-detail = I nuovi messaggi partono da questo account; la riga Da ne sceglie un altro. Le risposte e gli inoltri partono sempre dall’account che ha ricevuto il messaggio originale.
settings-compose-send-from-current = L’account in cui ti trovi
settings-compose-send-on-replies = Invio nelle risposte
settings-compose-send-on-replies-detail = Cosa fa Invia in una risposta o un inoltro. Il menu accanto a Invia offre l’altra opzione.
settings-compose-send-plain = Invia
settings-compose-send-archive = Invia e archivia
settings-compose-signatures = Firme
settings-compose-signatures-detail = Aggiunta sotto il tuo messaggio, dopo una riga «--». Scegline un’altra nella finestra di scrittura.
settings-compose-untitled = Senza titolo
settings-compose-signature-name = Nome, ad esempio Lavoro
settings-compose-signature-first = La mia firma
settings-compose-signature-numbered = Firma { $number }
settings-compose-signature-delete = Elimina
settings-compose-signature-deleted = Firma eliminata
settings-compose-signature-new = Crea nuova
settings-compose-no-signatures = Ancora nessuna firma.
settings-compose-no-signature = Nessuna firma
settings-compose-for-new-mail = Per i nuovi messaggi
settings-compose-for-replies = Per risposte e inoltri
settings-compose-for-replies-detail = In una conversazione in cui hai firmato un messaggio, una risposta inizia invece con quella firma.
settings-compose-format = Formato
settings-compose-plain-text = Scrivi in testo semplice
settings-compose-plain-text-detail = La nuova posta inizia senza formattazione; la finestra di scrittura può cambiarlo
settings-compose-spelling = Ortografia
settings-compose-spell-check = Controlla l’ortografia durante la scrittura
settings-compose-spell-check-detail = Le parole errate vengono sottolineate, con suggerimenti facendo clic con il tasto destro
settings-compose-spell-desktop = Lingua del desktop ({ $language })
settings-compose-templates = Modelli
settings-compose-templates-detail = Salva i messaggi che scrivi spesso e usali per iniziare un nuovo messaggio o una risposta.
settings-compose-no-templates = Ancora nessun modello. In un messaggio, scegli Modelli, poi Salva come modello.
settings-compose-template-new = Crea nuovo
settings-compose-template-new-name = Nuovo modello
settings-compose-template-subject = Oggetto
settings-compose-template-text = Testo del modello
settings-compose-template-fields = {"{"}first name{"}"}, {"{"}name{"}"} e {"{"}my name{"}"} vengono sostituiti con il nome del destinatario e con il tuo.
settings-compose-template-remove-file = Rimuovi allegato
settings-compose-template-save = Salva
settings-compose-template-saved = Modello salvato
settings-compose-template-needs-name = Dai un nome al modello
settings-compose-template-delete = Elimina modello
settings-compose-template-deleted = Modello eliminato
settings-compose-template-delete-failed = Impossibile eliminare il modello: { $error }

## Settings > Shortcuts

settings-shortcuts-set = Set di scorciatoie
settings-shortcuts-set-detail = Parti dai tasti di un’app di posta che conosci. Qui Cmd corrisponde a Ctrl. Le tue modifiche restano applicate sopra il set, e Ripristina predefinite torna ai tasti del set.
settings-shortcuts-single = Scorciatoie a tasto singolo
settings-shortcuts-single-detail = Tasti senza Ctrl o Alt, come nella webmail: e archivia, j e k spostano, / cerca. Funzionano nell’elenco e nella conversazione aperta, mai mentre scrivi.
settings-shortcuts-single-use = Usa le scorciatoie a tasto singolo
settings-shortcuts-single-use-detail = Le scorciatoie con Ctrl funzionano sempre
settings-shortcuts-how = Fai clic su un tasto per cambiarlo, o su + per aggiungerne uno, quindi premi i nuovi tasti. Esc annulla.
settings-shortcuts-restore = Ripristina predefinite
settings-shortcuts-no-key = Nessun tasto
settings-shortcuts-press = Premi i tasti…
settings-shortcuts-then = { $keys } poi…
settings-shortcuts-moved = { $keys } ora esegue «{ $action }» invece di «{ $previous }».
settings-shortcuts-single-off = Le scorciatoie a tasto singolo sono disattivate, quindi questo tasto funzionerà quando saranno attive.
settings-shortcuts-restored = Tutte le scorciatoie hanno di nuovo i tasti del loro set.

## Settings search: the line under a result

settings-general-language-summary = Lingua dell’app, delle date e dei numeri
settings-general-reading-summary = Prima il messaggio più recente, intestazioni complete, nomi completi dei destinatari
settings-translation-summary = Traduci con il server di Katna le email in altre lingue, nella lingua che scegli
settings-general-mark-read-summary = Quando una conversazione aperta viene segnata come già letta: subito, dopo 1 o 3 secondi, o a mano
settings-general-auto-advance-summary = Cosa si apre dopo aver eliminato, archiviato o spostato la conversazione aperta: la successiva, la precedente o l’elenco
settings-general-reply-button-summary = Il pulsante di risposta accanto a ogni messaggio risponde a tutti
settings-general-remote-images-summary = Mostra sempre le immagini di ogni messaggio
settings-general-sending-summary = Annulla invio: quanto attende un messaggio inviato, per poterlo annullare
settings-general-offline-summary = Quanti giorni di posta recente vengono scaricati per intero, per leggerla senza connessione
settings-general-notifications-summary = Notifiche di nuova posta e il loro suono
settings-general-reset-cache-summary = Elimina la posta scaricata, le immagini dei mittenti e l’indice di ricerca, e scaricali di nuovo
settings-general-desktop-summary = Avvia Katna all’accesso, l’icona nell’area di notifica e il numero di messaggi da leggere sull’icona nella barra delle applicazioni
settings-accounts-accounts-summary = Aggiungi o rimuovi un account, o cambiane l’immagine
settings-appearance-density-summary = Righe predefinite o compatte nell’elenco
settings-appearance-scaling-summary = Ingrandisci o rimpicciolisci tutto: testo, icone, spaziatura e divisori
settings-appearance-theme-summary = Sistema, chiaro o scuro
settings-appearance-sender-pictures-summary = Loghi aziendali, cercati in base al dominio del mittente
settings-appearance-important-summary = L’indicatore di importanza accanto a ogni messaggio nell’elenco
settings-appearance-mail-colors-summary = Colori scuri per la posta HTML con un tema scuro, o i colori del mittente
settings-appearance-attachment-previews-summary = Una piccola immagine del contenuto di ogni allegato
settings-shortcuts-set-summary = Parti dai tasti di Gmail, Inbox by Gmail, Apple Mail, Outlook o Thunderbird
settings-shortcuts-single-summary = Tasti senza Ctrl o Alt, come nella webmail
settings-default-apps-pdf-summary = Dove si aprono gli allegati PDF
settings-default-apps-pictures-summary = Dove si aprono foto e immagini
settings-default-apps-text-summary = Dove si aprono testo semplice, log e codice
settings-default-apps-sheets-summary = Dove si aprono i file Excel, OpenDocument e CSV
settings-default-apps-documents-summary = Dove si aprono i testi Word e OpenDocument e le presentazioni
settings-default-apps-after-saving-summary = Mostra gli allegati salvati nella loro cartella
settings-compose-send-from-summary = L’account da cui parte la nuova posta: il primo, un altro o quello in cui ti trovi
settings-compose-send-on-replies-summary = Invia, o Invia e archivia la conversazione, nelle risposte e negli inoltri
settings-compose-signatures-summary = Aggiunta sotto il tuo messaggio, dopo una riga «--»
settings-compose-for-new-mail-summary = La firma con cui inizia la nuova posta
settings-compose-for-replies-summary = La firma con cui iniziano risposte e inoltri
settings-compose-format-summary = Scrivi la nuova posta in testo semplice
settings-compose-spelling-summary = Controlla l’ortografia durante la scrittura, e la lingua del dizionario
settings-general-search-triggers-summary = Parole che cercano nella tua posta da KRunner o dalla ricerca di GNOME
settings-compose-templates-summary = Salva i messaggi che scrivi spesso e usali per iniziare un nuovo messaggio o una risposta
settings-feedback-crash-reports-summary = Salva i rapporti sugli arresti anomali su questo computer quando Katna Mail o il suo servizio in background si arresta in modo anomalo
settings-feedback-saved-summary = Visualizza, copia o elimina i rapporti sugli arresti anomali salvati su questo computer
settings-feedback-help-improve-summary = Invia i rapporti sugli arresti anomali per aiutare a risolvere il problema; disattivato finché non lo attivi
settings-experimental-blur-summary = Il desktop traspare sfocato attraverso la barra superiore, e i menu sono in vetro smerigliato
settings-search-shortcut = Scorciatoia da tastiera
settings-search-tab = Scheda delle impostazioni
settings-search-none = Nessuna impostazione corrisponde a «{ $query }».
settings-search-results = Impostazioni corrispondenti a «{ $query }»

## Settings: opening at login

settings-open-at-login-failed = Impossibile modificare l’avvio all’accesso: { $error }

## Settings > General > Time

settings-time = Ora
settings-clock-language = Come si scrive nella lingua
settings-clock-12 = 12 ore, ad esempio 02:05 PM
settings-clock-24 = 24 ore, ad esempio 14:05
settings-time-summary = Formato 12 o 24 ore, o come si scrive nella lingua

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = App di posta predefinita
settings-general-mail-app-detail = I link email in altre app e sui siti web aprono qui un nuovo messaggio.
mail-app-is-default = Katna Mail è la tua app di posta predefinita.
mail-app-is-other = I link email si aprono in un’altra app.
mail-app-make-default = Imposta come predefinita
mail-app-make-default-failed = Impossibile cambiare l’app di posta predefinita.
settings-general-mail-app-summary = Apri in Katna Mail i link email di altre app e dei siti web
settings-compose-grammar = Grammatica
settings-compose-grammar-detail = Controllata su questo computer con Harper. Per ora solo in inglese: il testo in altre lingue non viene toccato.
settings-compose-grammar-check = Controlla la grammatica
settings-compose-grammar-check-detail = Sottolinea gli errori grammaticali durante la scrittura, in inglese
settings-compose-suggestions = Suggerimenti di scrittura
settings-compose-suggestions-detail = Appresi su questo computer dalla posta che hai inviato e da quella a cui stai rispondendo; nulla esce dal computer. Premi Tab per accettare un suggerimento, o continua a scrivere.
settings-compose-suggestions-on = Suggerisci durante la scrittura
settings-compose-suggestions-on-detail = Mostra in grigio il probabile seguito della frase mentre scrivi
settings-compose-grammar-summary = Sottolinea gli errori grammaticali durante la scrittura, in inglese
settings-compose-suggestions-summary = Mostra in grigio il probabile seguito della frase mentre scrivi
