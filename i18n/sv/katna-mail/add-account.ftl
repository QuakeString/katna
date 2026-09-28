# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Lägg till ett e-postkonto
add-account-looking = Letar efter e-postservrarna för { $address }…
add-account-address-intro = Ange din e-postadress. Katna hittar servrarna åt dig.
add-account-servers-title = Serverinställningar
add-account-servers-intro = Var Katna läser och skickar e-post för { $address }.
add-account-password-title = Ange ditt lösenord
add-account-signing-in = Loggar in…
add-account-browser-title = Fortsätt i webbläsaren
add-account-browser-intro = Katna har öppnat inloggningssidan för { $provider } i din webbläsare. Logga in där och låt Katna läsa och skicka din e-post, och kom sedan tillbaka hit.
add-account-browser-hint = Öppnades ingen sida? Titta bland webbläsarens fönster, eller gå tillbaka och försök igen.

## Add a mail account: fields

add-account-field-address = E-postadress
add-account-incoming = Inkommande e-post ({ $protocol })
add-account-outgoing = Utgående e-post ({ $protocol })
add-account-field-server = Server
add-account-field-port = Port
add-account-security-none = Ingen
add-account-security-none-warning = Inte krypterad: ditt lösenord och din e-post kan läsas på vägen.
add-account-field-username = Användarnamn
add-account-field-password = Lösenord
add-account-show-password = Visa lösenord
add-account-app-password-hint = { $provider } kräver ett applösenord här, inte det du använder på webben. Skapa ett i säkerhetsinställningarna för ditt { $provider }-konto.
add-account-field-name = Ditt namn (valfritt)
add-account-name-hint = Visas för dem du skriver till.
add-account-servers-pair = { $imap } och { $smtp }
add-account-servers-found = { $source ->
    [built-in] Servrar: { $servers }, hittade i Katnas lista över leverantörer.
    [provider] Servrar: { $servers }, hittade i din leverantörs inställningar.
    [ispdb] Servrar: { $servers }, hittade i Thunderbirds lista över leverantörer.
    [dns] Servrar: { $servers }, hittade i din domäns DNS-poster.
   *[other] Servrar: { $servers }, gissade; kontrollera dem om inloggningen misslyckas.
}
add-account-servers-entered = Servrar: { $servers }, som angivna.
add-account-or = eller
add-account-sign-in-with = Logga in med { $provider }
add-account-sign-in-instead = Logga in med { $provider } i stället

## Add a mail account: buttons

add-account-servers-button = Serverinställningar
add-account-back = Tillbaka
add-account-add = Lägg till konto
add-account-next = Nästa
add-account-cancel = Avbryt

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Ange den inkommande servern.
   *[outgoing] Ange den utgående servern.
}
add-account-server-space = { $kind ->
    [incoming] Namnet på den inkommande servern innehåller ett mellanslag.
   *[outgoing] Namnet på den utgående servern innehåller ett mellanslag.
}
add-account-port-invalid = { $kind ->
    [incoming] Den inkommande porten måste vara ett tal från { $min } till { $max }.
   *[outgoing] Den utgående porten måste vara ett tal från { $min } till { $max }.
}
add-account-address-empty = Ange en e-postadress.
add-account-address-invalid = Ange en e-postadress som { $example }.
add-account-not-found = Katna kunde inte hitta servrarna för { $address }, så de vanliga namnen fylldes i. Kontrollera dem med din leverantör.
add-account-password-empty = Ange lösenordet.
add-account-name-is-password = Namnet är detsamma som lösenordet. Skriv ditt namn där i stället, så som andra ska se det.
add-account-added = { $address } har lagts till. Hämtar din e-post…
add-account-app-password-refused = { $provider } avvisade lösenordet. Det krävs ett applösenord, inte det du använder på webben.
add-account-password-refused = Servern avvisade lösenordet. Kontrollera det och försök igen.
add-account-sign-in-refused = { $provider } släppte inte in Katna. Försök igen och ge åtkomst till din e-post.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Den här versionen av Katna kan inte logga in på Microsoft-konton än.
    [Google] Den här versionen av Katna kan inte logga in på Google-konton än.
   *[other] Den här leverantören tillåter bara inloggning på sin egen sida, vilket Katna inte kan göra för den än.
}
add-account-signed-in = Inloggad med { $provider }. Hämtar din e-post…

## The account menu (from the account button on the top bar)

add-account-menu-another = Lägg till ett konto till
add-account-menu-manage = Hantera konton
app-menu = Huvudmeny
