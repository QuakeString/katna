# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Een e-mailaccount toevoegen
add-account-providers-intro = Kies je e-mailprovider. Katna regelt de rest.
add-account-provider-other = Andere e-mail
add-account-provider-other-detail = Elk IMAP- of POP3-account
add-account-provider-google-detail = Gmail en Google Workspace
add-account-provider-microsoft-detail = Outlook en Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Aanmelden bij { $provider }
add-account-form-title-other = Je e-mailaccount
add-account-form-intro = Katna bewaart je wachtwoord in de sleutelbos van je systeem.
add-account-looking = De e-mailservers van { $address } zoeken…
add-account-address-intro = Voer je e-mailadres in. Katna zoekt de servers voor je.
add-account-servers-title = Serverinstellingen
add-account-servers-intro = Waar Katna e-mail leest en verstuurt voor { $address }.
add-account-signing-in = Aanmelden…
add-account-browser-title = Ga verder in je browser
add-account-browser-intro = Katna heeft de aanmeldpagina van { $provider } geopend in je browser. Meld je daar aan en sta Katna toe je e-mail te lezen en te versturen, en kom dan hier terug.
add-account-browser-hint = Geen pagina geopend? Kijk bij de vensters van je browser, of ga terug en probeer het opnieuw.
add-account-stage-browser = Wachten tot je je aanmeldt in je browser…
add-account-stage-signing-in-at = Aanmelden bij { $server }…
add-account-help-app-password-link = Zo maak je een app-wachtwoord
add-account-help-turn-on-imap = { $provider } laat e-mailapps pas toe als IMAP- en POP3-toegang aan staat in de instellingen van de webmail.
add-account-help-turn-on-imap-link = Zo zet je het aan

## Add a mail account: fields

add-account-field-address = E-mailadres
add-account-receive-with = E-mail ontvangen met
add-account-imap-about = IMAP bewaart je e-mail en mappen op de server, op elk apparaat hetzelfde. Kies dit als het kan.
add-account-pop3-about = POP3 downloadt je e-mail naar deze computer. E-mail die je hier leest of verplaatst, blijft op de server en je andere apparaten zoals hij was.
add-account-incoming = Inkomende e-mail ({ $protocol })
add-account-outgoing = Uitgaande e-mail ({ $protocol })
add-account-field-server = Server
add-account-field-port = Poort
add-account-security-none = Geen
add-account-security-none-warning = Niet versleuteld: je wachtwoord en e-mail kunnen onderweg worden meegelezen.
add-account-field-username = Gebruikersnaam
add-account-field-password = Wachtwoord
add-account-show-password = Wachtwoord tonen
add-account-app-password-hint = { $provider } heeft hier een app-wachtwoord nodig, niet het wachtwoord dat je op het web gebruikt. Maak er een aan in de beveiligingsinstellingen van je { $provider }-account.
add-account-field-name = Je naam (optioneel)
add-account-name-hint = Zichtbaar voor de mensen aan wie je schrijft.
add-account-servers-pair = { $imap } en { $smtp }
add-account-servers-found = { $source ->
    [built-in] Servers: { $servers }, gevonden in de providerlijst van Katna.
    [provider] Servers: { $servers }, gevonden in de instellingen van je provider.
    [ispdb] Servers: { $servers }, gevonden in de providerlijst van Thunderbird.
    [dns] Servers: { $servers }, gevonden in de DNS-records van je domein.
   *[other] Servers: { $servers }, geraden; controleer ze als aanmelden mislukt.
}
add-account-servers-entered = Servers: { $servers }, zoals ingevoerd.

## Add a mail account: buttons

add-account-sign-in-with = Aanmelden met { $provider }
add-account-sign-in-instead = In plaats daarvan aanmelden met { $provider }

add-account-servers-button = Serverinstellingen
add-account-back = Terug
add-account-add = Account toevoegen
add-account-done = Klaar
add-account-another = Nog een account toevoegen
add-account-cancel = Annuleren

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Voer de inkomende server in.
   *[outgoing] Voer de uitgaande server in.
}
add-account-server-space = { $kind ->
    [incoming] De naam van de inkomende server bevat een spatie.
   *[outgoing] De naam van de uitgaande server bevat een spatie.
}
add-account-port-invalid = { $kind ->
    [incoming] De inkomende poort moet een getal van { $min } tot { $max } zijn.
   *[outgoing] De uitgaande poort moet een getal van { $min } tot { $max } zijn.
}
add-account-address-empty = Voer een e-mailadres in.
add-account-address-invalid = Voer een e-mailadres in zoals { $example }.
add-account-not-found = Katna kon de servers voor { $address } niet vinden en heeft daarom de gebruikelijke namen ingevuld. Controleer ze bij je provider.
add-account-password-empty = Voer het wachtwoord in.
add-account-name-is-password = De naam is hetzelfde als het wachtwoord. Typ daar liever je naam, zoals anderen die moeten zien.
add-account-app-password-refused = { $provider } heeft het wachtwoord geweigerd. Er is een app-wachtwoord nodig, niet het wachtwoord dat je op het web gebruikt.
add-account-password-refused = De server heeft het wachtwoord geweigerd. Controleer het en probeer het opnieuw.
add-account-sign-in-refused = { $provider } heeft Katna niet binnengelaten. Probeer het opnieuw en geef toegang tot je e-mail.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Deze versie van Katna kan zich nog niet aanmelden bij Microsoft-accounts.
    [Google] Deze versie van Katna kan zich nog niet aanmelden bij Google-accounts.
   *[other] Deze provider staat aanmelden alleen toe op zijn eigen pagina, en dat kan Katna er nog niet voor doen.
}
add-account-smtp-not-found = Katna heeft gevonden waar je e-mail gelezen wordt, maar niet waar hij verstuurd wordt. Vul de uitgaande server in.

## Add a mail account: the last step

add-account-done-title = Je account is klaar
add-account-done-intro = Katna haalt je e-mail nu op. Nieuwe e-mail verschijnt zodra hij binnenkomt.
add-account-done-sign-in = Aanmelden
add-account-done-signed-in-with = Met { $provider }, in je browser
add-account-done-receiving = E-mail ontvangen
add-account-done-sending = E-mail versturen
add-account-done-on-server = E-mail op de server
add-account-done-kept = Bewaard tot je het in Katna verwijdert
add-account-done-pop3-hint = Wat er met e-mail op de server gebeurt, wijzig je in Instellingen > Accounts.
add-account-done-zoho-title = Taken en agenda’s
add-account-done-zoho-about = Zoho houdt deze los van e-mail. Meld je één keer aan met Zoho om ze in Katna te halen.
add-account-done-linked = Taken en agenda’s gekoppeld

## The account menu (from the account button on the top bar)

add-account-menu-another = Nog een account toevoegen
app-menu = Hoofdmenu
app-menu-back = Terug
