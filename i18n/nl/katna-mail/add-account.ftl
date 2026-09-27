# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Een e-mailaccount toevoegen
add-account-looking = De e-mailservers van { $address } zoeken…
add-account-address-intro = Voer je e-mailadres in. Katna zoekt de servers voor je.
add-account-servers-title = Serverinstellingen
add-account-servers-intro = Waar Katna e-mail leest en verstuurt voor { $address }.
add-account-password-title = Voer je wachtwoord in
add-account-signing-in = Aanmelden…

## Add a mail account: fields

add-account-field-address = E-mailadres
add-account-incoming = Inkomende e-mail ({ $protocol })
add-account-outgoing = Uitgaande e-mail ({ $protocol })
add-account-field-server = Server
add-account-field-port = Poort
add-account-security-none = Geen
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

add-account-servers-button = Serverinstellingen
add-account-back = Terug
add-account-add = Account toevoegen
add-account-next = Volgende
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
add-account-added = { $address } toegevoegd. Je e-mail wordt opgehaald…
add-account-app-password-refused = { $provider } heeft het wachtwoord geweigerd. Er is een app-wachtwoord nodig, niet het wachtwoord dat je op het web gebruikt.
add-account-password-refused = De server heeft het wachtwoord geweigerd. Controleer het en probeer het opnieuw.

## The account menu (from the account button on the top bar)

add-account-menu-another = Nog een account toevoegen
add-account-menu-manage = Accounts beheren
