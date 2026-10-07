# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = De mailserver

problems-signed-out = { $provider } heeft Katna afgemeld bij { $address }. E-mail wordt niet meer gesynchroniseerd.
problems-password-refused = { $provider } heeft het wachtwoord voor { $address } geweigerd. Misschien is het gewijzigd.
problems-no-answer = { $provider } reageert niet voor { $address }. Katna blijft het proberen.
problems-offline = Je bent offline. Je e-mail is er nog, en e-mail die je verstuurt wacht tot je weer online bent.
problems-accounts-need-you = { $count ->
    [one] 1 account heeft je nodig
   *[other] { $count } accounts hebben je nodig
}
problems-show = Tonen
problems-later = Later
problems-new-password = Nieuw wachtwoord
problems-try-again = Opnieuw proberen

## The New password card

problems-password-title = Nieuw wachtwoord
problems-password-detail = { $provider } heeft het opgeslagen wachtwoord voor { $address } geweigerd. Typ het nieuwe; Katna controleert het voordat het wordt bewaard.
problems-password-placeholder = Wachtwoord
problems-password-show = Wachtwoord tonen
problems-password-hide = Wachtwoord verbergen
problems-password-cancel = Annuleren
problems-password-save = Opslaan
problems-password-checking = Controleren…
problems-password-refused-again = { $provider } heeft ook dit wachtwoord geweigerd. Controleer het en probeer het opnieuw.
problems-password-saved = Wachtwoord opgeslagen voor { $address }. Je e-mail wordt opgehaald…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = De mailserver van { $address } heeft het verplaatsen van { $count ->
    [one] een bericht niet geaccepteerd, dus het staat weer waar het stond.
   *[other] { $count } berichten niet geaccepteerd, dus ze staan weer waar ze stonden.
}
problems-refused-flags = De mailserver van { $address } heeft het markeren van { $count ->
    [one] een bericht (gelezen, ster…) niet geaccepteerd, dus het is weer zoals het was.
   *[other] { $count } berichten (gelezen, ster…) niet geaccepteerd, dus ze zijn weer zoals ze waren.
}
problems-refused-label = De mailserver van { $address } heeft het wijzigen van de labels van { $count ->
    [one] een bericht niet geaccepteerd, dus het is weer zoals het was.
   *[other] { $count } berichten niet geaccepteerd, dus ze zijn weer zoals ze waren.
}
problems-refused-delete = De mailserver van { $address } heeft het verwijderen van { $count ->
    [one] een bericht niet geaccepteerd, dus het is terug.
   *[other] { $count } berichten niet geaccepteerd, dus ze zijn terug.
}
problems-refused-other = De mailserver van { $address } heeft { $count ->
    [one] een wijziging niet geaccepteerd, dus Katna heeft die teruggezet.
   *[other] { $count } wijzigingen niet geaccepteerd, dus Katna heeft ze teruggezet.
}
problems-details = Details

## Katna's background service (katna-daemon) isn't running

service-starting = De achtergrondservice van Katna wordt gestart…
service-failed = De achtergrondservice van Katna start niet, dus e-mail wordt niet gesynchroniseerd.
service-start-again = Opnieuw starten
service-started-again = De achtergrondservice van Katna was gestopt en is opnieuw gestart.
service-details-title = Waarom de service niet start
service-details-body = Kopieer dit en stuur het mee met je melding. Er staat geen e-mail of wachtwoord in.
service-details-copy = Kopiëren
service-details-close = Sluiten
