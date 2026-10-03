# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Lägg till ett e-postkonto
add-account-providers-intro = Välj din e-postleverantör. Katna hittar resten.
add-account-provider-other = Annan e-post
add-account-provider-other-detail = Alla IMAP- eller POP3-konton
add-account-provider-google-detail = Gmail och Google Workspace
add-account-provider-microsoft-detail = Outlook och Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Logga in på { $provider }
add-account-form-title-other = Ditt e-postkonto
add-account-form-intro = Katna sparar ditt lösenord i systemets nyckelring.
add-account-looking = Letar efter e-postservrarna för { $address }…
add-account-address-intro = Ange din e-postadress. Katna hittar servrarna åt dig.
add-account-servers-title = Serverinställningar
add-account-servers-intro = Var Katna läser och skickar e-post för { $address }.
add-account-signing-in = Loggar in…
add-account-browser-title = Fortsätt i webbläsaren
add-account-browser-intro = Katna har öppnat inloggningssidan för { $provider } i din webbläsare. Logga in där och låt Katna läsa och skicka din e-post, och kom sedan tillbaka hit.
add-account-browser-hint = Öppnades ingen sida? Titta bland webbläsarens fönster, eller gå tillbaka och försök igen.
add-account-stage-browser = Väntar på att du loggar in i webbläsaren…
add-account-stage-signing-in-at = Loggar in på { $server }…
add-account-help-app-password-link = Så skapar du ett applösenord
add-account-help-turn-on-imap = { $provider } släpper bara in e-postappar när IMAP- och POP3-åtkomst är påslagen i inställningarna för webbmejlen.
add-account-help-turn-on-imap-link = Så slår du på det

## Add a mail account: fields

add-account-field-address = E-postadress
add-account-receive-with = Ta emot e-post med
add-account-imap-about = IMAP behåller din e-post och dina mappar på servern, likadana på alla enheter. Välj det om du kan.
add-account-pop3-about = POP3 hämtar din e-post till den här datorn. E-post du läser eller flyttar här förblir som den är på servern och dina andra enheter.
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

## Add a mail account: buttons

add-account-sign-in-with = Logga in med { $provider }
add-account-sign-in-instead = Logga in med { $provider } i stället

add-account-servers-button = Serverinställningar
add-account-back = Tillbaka
add-account-add = Lägg till konto
add-account-done = Klar
add-account-another = Lägg till ett konto till
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
add-account-app-password-refused = { $provider } avvisade lösenordet. Det krävs ett applösenord, inte det du använder på webben.
add-account-password-refused = Servern avvisade lösenordet. Kontrollera det och försök igen.
add-account-sign-in-refused = { $provider } släppte inte in Katna. Försök igen och ge åtkomst till din e-post.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Den här versionen av Katna kan inte logga in på Microsoft-konton än.
    [Google] Den här versionen av Katna kan inte logga in på Google-konton än.
   *[other] Den här leverantören tillåter bara inloggning på sin egen sida, vilket Katna inte kan göra för den än.
}
add-account-smtp-not-found = Katna hittade var din e-post ska läsas men inte var den ska skickas. Ange servern för utgående e-post.

## Add a mail account: the last step

add-account-done-title = Ditt konto är klart
add-account-done-intro = Katna hämtar din e-post nu. Ny e-post visas allt eftersom den kommer in.
add-account-done-sign-in = Inloggning
add-account-done-signed-in-with = Med { $provider }, i din webbläsare
add-account-done-receiving = Ta emot e-post
add-account-done-sending = Skicka e-post
add-account-done-on-server = E-post på servern
add-account-done-kept = Behålls tills du raderar den i Katna
add-account-done-pop3-hint = Ändra vad som händer med e-post på servern i Inställningar > Konton.
add-account-done-zoho-title = Uppgifter och kalendrar
add-account-done-zoho-about = Zoho håller dem åtskilda från e-posten. Logga in med Zoho en gång för att få in dem i Katna.
add-account-done-linked = Uppgifter och kalendrar anslutna

## The account menu (from the account button on the top bar)

add-account-menu-another = Lägg till ett konto till
app-menu = Huvudmeny
app-menu-back = Tillbaka
