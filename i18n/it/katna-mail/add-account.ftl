# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Aggiungi un account di posta
add-account-looking = Ricerca dei server di posta di { $address }…
add-account-address-intro = Inserisci il tuo indirizzo email. Katna trova i server per te.
add-account-servers-title = Impostazioni del server
add-account-servers-intro = Dove Katna legge e invia la posta di { $address }.
add-account-password-title = Inserisci la password
add-account-signing-in = Accesso in corso…

## Add a mail account: fields

add-account-field-address = Indirizzo email
add-account-incoming = Posta in arrivo ({ $protocol })
add-account-outgoing = Posta in uscita ({ $protocol })
add-account-field-server = Server
add-account-field-port = Porta
add-account-security-none = Nessuna
add-account-field-username = Nome utente
add-account-field-password = Password
add-account-show-password = Mostra la password
add-account-app-password-hint = Qui { $provider } richiede una password per le app, non quella che usi sul web. Creane una nelle impostazioni di sicurezza del tuo account { $provider }.
add-account-field-name = Il tuo nome (facoltativo)
add-account-name-hint = Visibile alle persone a cui scrivi.
add-account-servers-pair = { $imap } e { $smtp }
add-account-servers-found = { $source ->
    [built-in] Server: { $servers }, trovati nell’elenco dei provider di Katna.
    [provider] Server: { $servers }, trovati nelle impostazioni del tuo provider.
    [ispdb] Server: { $servers }, trovati nell’elenco dei provider di Thunderbird.
    [dns] Server: { $servers }, trovati nei record DNS del tuo dominio.
   *[other] Server: { $servers }, ipotizzati; controllali se l’accesso non riesce.
}
add-account-servers-entered = Server: { $servers }, come inseriti.

## Add a mail account: buttons

add-account-servers-button = Impostazioni del server
add-account-back = Indietro
add-account-add = Aggiungi account
add-account-next = Avanti
add-account-cancel = Annulla

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Inserisci il server della posta in arrivo.
   *[outgoing] Inserisci il server della posta in uscita.
}
add-account-server-space = { $kind ->
    [incoming] Il nome del server della posta in arrivo contiene uno spazio.
   *[outgoing] Il nome del server della posta in uscita contiene uno spazio.
}
add-account-port-invalid = { $kind ->
    [incoming] La porta della posta in arrivo deve essere un numero da { $min } a { $max }.
   *[outgoing] La porta della posta in uscita deve essere un numero da { $min } a { $max }.
}
add-account-address-empty = Inserisci un indirizzo email.
add-account-address-invalid = Inserisci un indirizzo email come { $example }.
add-account-not-found = Katna non ha trovato i server di { $address }, quindi ha inserito i nomi più comuni. Verificali con il tuo provider.
add-account-password-empty = Inserisci la password.
add-account-added = { $address } aggiunto. Scaricamento della posta…
add-account-app-password-refused = { $provider } ha rifiutato la password. Serve una password per le app, non quella che usi sul web.
add-account-password-refused = Il server ha rifiutato la password. Controllala e riprova.

## The account menu (from the account button on the top bar)

add-account-menu-another = Aggiungi un altro account
add-account-menu-manage = Gestisci gli account
