# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Die e-posbediener

problems-signed-out = { $provider } het Katna by { $address } afgemeld. E-pos sinkroniseer nie meer nie.
problems-password-refused = { $provider } het die wagwoord vir { $address } geweier. Dit het dalk verander.
problems-no-answer = { $provider } antwoord nie vir { $address } nie. Katna probeer aanhou.
problems-offline = Jy is vanlyn. Jou e-pos is steeds hier, en e-pos wat jy stuur, wag totdat jy terug is.
problems-accounts-need-you = { $count ->
    [one] 1 rekening het jou nodig
   *[other] { $count } rekeninge het jou nodig
}
problems-show = Wys
problems-later = Later
problems-new-password = Nuwe wagwoord
problems-try-again = Probeer weer

## The New password card

problems-password-title = Nuwe wagwoord
problems-password-detail = { $provider } het die gestoorde wagwoord vir { $address } geweier. Tik die nuwe een; Katna gaan dit na voordat dit gehou word.
problems-password-placeholder = Wagwoord
problems-password-show = Wys wagwoord
problems-password-hide = Versteek wagwoord
problems-password-cancel = Kanselleer
problems-password-save = Stoor
problems-password-checking = Gaan tans na…
problems-password-refused-again = { $provider } het hierdie wagwoord ook geweier. Gaan dit na en probeer weer.
problems-password-saved = Wagwoord gestoor vir { $address }. Haal tans jou e-pos…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Die e-posbediener van { $address } het nie aanvaar om { $count ->
    [one] 'n boodskap te skuif nie, so dit is terug waar dit was.
   *[other] { $count } boodskappe te skuif nie, so hulle is terug waar hulle was.
}
problems-refused-flags = Die e-posbediener van { $address } het nie aanvaar om { $count ->
    [one] 'n boodskap te merk nie (gelees, gester…), so dit is terug soos dit was.
   *[other] { $count } boodskappe te merk nie (gelees, gester…), so hulle is terug soos hulle was.
}
problems-refused-label = Die e-posbediener van { $address } het nie aanvaar om die etikette van { $count ->
    [one] 'n boodskap te verander nie, so dit is terug soos dit was.
   *[other] { $count } boodskappe te verander nie, so hulle is terug soos hulle was.
}
problems-refused-delete = Die e-posbediener van { $address } het nie aanvaar om { $count ->
    [one] 'n boodskap uit te vee nie, so dit is terug.
   *[other] { $count } boodskappe uit te vee nie, so hulle is terug.
}
problems-refused-other = Die e-posbediener van { $address } het { $count ->
    [one] 'n verandering nie aanvaar nie, so Katna het dit teruggesit soos dit was.
   *[other] { $count } veranderinge nie aanvaar nie, so Katna het hulle teruggesit soos hulle was.
}
problems-details = Besonderhede

## Katna's background service (katna-daemon) isn't running

service-starting = Begin tans Katna se agtergronddiens…
service-failed = Katna se agtergronddiens wil nie begin nie, so e-pos sinkroniseer nie.
service-start-again = Begin weer
service-started-again = Katna se agtergronddiens het gestop en is weer begin.
service-details-title = Hoekom die diens nie wil begin nie
service-details-body = Kopieer dit en stuur dit saam met jou verslag. Daar is geen e-pos of wagwoorde in nie.
service-details-copy = Kopieer
service-details-close = Maak toe
service-not-running = Die Katna-agtergronddiens loop nie.
service-no-answer = Die Katna-agtergronddiens het nie geantwoord nie: { $error }
service-no-session = Geen D-Bus-sessie nie: { $error }
