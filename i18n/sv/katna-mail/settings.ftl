# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Allmänt
settings-tab-inbox = Inkorg
settings-tab-accounts = Konton
settings-tab-subscriptions = Prenumerationer
settings-tab-appearance = Utseende
settings-tab-shortcuts = Kortkommandon
settings-tab-default-apps = Standardappar
settings-tab-folders-rules = Mappar och regler
settings-tab-compose = Skriva
settings-tab-mcp-server = MCP-server
settings-tab-feedback = Feedback
settings-tab-experimental = Experimentellt

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Se vilka nyhetsbrev och e-postlistor du får, och avprenumerera med ett klick.
settings-tab-folders-rules-coming = Skapa, byt namn på, flytta och dölj mappar och etiketter, och välj vilka som synkroniseras. Regler sorterar, etiketterar, vidarebefordrar eller raderar ny e-post automatiskt, efter avsändare, ämne eller ord.
settings-tab-mcp-server-coming = Låt AI-assistenter på den här datorn söka i, läsa och skriva utkast till din e-post, med ditt godkännande.

## Settings > General

settings-general-conversations = Konversationsvy
settings-general-conversations-group = Gruppera svar på samma e-post
settings-general-conversations-group-detail = En rad per konversation i listan
settings-general-reading = Läsa
settings-general-newest-first = Nyaste meddelandet först
settings-general-newest-first-detail = En konversation börjar med det senaste svaret
settings-general-full-headers = Visa fullständiga rubriker
settings-general-full-headers-detail = Från, till, kopia, datum och ämne visas i varje meddelande
settings-general-full-names = Mottagarnas fullständiga namn
settings-general-full-names-detail = ”till mig, Ada Lovelace” i stället för ”till mig, Ada”
settings-general-mark-read = Markera som läst
settings-general-mark-read-now = Så fort det öppnas
settings-general-mark-read-1s = När det har varit öppet i 1 sekund
settings-general-mark-read-3s = När det har varit öppet i 3 sekunder
settings-general-mark-read-never = Bara när jag markerar det som läst
settings-general-reply-button = Svarsknapp
settings-general-reply-all = Svara alla
settings-general-reply-all-detail = Svarsknappen bredvid varje meddelande svarar alla, inte bara avsändaren
settings-general-remote-images = Bilder från webben
settings-general-remote-images-detail = När ett meddelandes bilder läses in får avsändaren veta att du har öppnat det, när och ungefär var. Om det är av frågar varje meddelande först, och du kan alltid visa en avsändares bilder.
settings-general-remote-images-always = Visa alltid bilder
settings-general-remote-images-always-detail = I alla meddelanden, inte bara från avsändare du litar på
settings-general-sending = Skicka
settings-general-sending-detail = Hur länge ett skickat meddelande väntar, så att det går att ångra.
settings-general-offline = E-post offline
settings-general-offline-detail = Ny e-post hämtas i sin helhet, så att du kan läsa den utan anslutning. Äldre e-post hämtas när du öppnar den.
settings-general-offline-days = { $count ->
    [one] { $count } dag
   *[other] { $count } dagar
}
settings-general-offline-years = { $count ->
    [one] { $count } år
   *[other] { $count } år
}
settings-general-offline-all = All e-post
settings-general-offline-note = Om du väljer färre dagar behålls e-post som redan har hämtats. Ingenting ändras på servern.
settings-general-notifications = Aviseringar
settings-general-notifications-detail = För ny e-post i inkorgen, även när Katna Mail är stängt.
settings-general-new-mail = Avisera mig om ny e-post
settings-general-new-mail-detail = Med Svara alla, Markera som läst och Arkivera
settings-general-new-mail-sound = Spela upp ett ljud
settings-general-new-mail-sound-detail = Skrivbordets ljud för ny e-post
settings-general-desktop = Skrivbord
settings-general-open-at-login = Öppna Katna Mail vid inloggning
settings-general-open-at-login-detail = E-posten synkroniseras vid inloggning ändå, så länge tjänsten körs
settings-general-tray = Visa Katna i systemfältet
settings-general-tray-detail = Med antalet olästa och en meny
settings-general-unread-badge = Antal olästa på aktivitetsfältets ikon
settings-general-unread-badge-detail = Hur många meddelanden i inkorgen som är olästa

## Settings > Inbox

settings-inbox-tabs = Inkorgsflikar
settings-inbox-tabs-detail = Sortera inkorgen i flikar, som din e-postleverantörs webbplats gör.
settings-inbox-tabs-show = Visa inkorgsflikar
settings-inbox-tabs-show-detail = Av visar en lista för varje konto
settings-inbox-no-accounts = Lägg till ett konto för att välja dess flikar.
settings-inbox-tabs-automatic = Automatiskt: { $tabs } ({ $provider })
settings-inbox-tabs-off = Inga flikar
settings-inbox-tabs-gmail = Primär, Kampanjer, Socialt, Uppdateringar, Forum
settings-inbox-tabs-focused = Prioriterat och Övrigt
settings-inbox-tabs-zoho = Inkorgen, Nyhetsbrev och Aviseringar
settings-inbox-tabs-shown = Flikar som visas. E-post i en flik som du stänger av stannar i { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Läsruta
settings-appearance-reading-pane-detail = Var en öppnad konversation visas.
settings-appearance-pane-right = Till höger om listan
settings-appearance-pane-none = Ingen delning
settings-appearance-density = Täthet
settings-appearance-density-default = Standard
settings-appearance-density-compact = Kompakt
settings-appearance-scaling = Skalning
settings-appearance-scaling-detail = Gör allt i Katna Mail större eller mindre, utöver skrivbordets egen skalning: text, ikoner, avstånd och avdelare. E-post du skickar behåller sin egen teckenstorlek. Mycket små storlekar kan göra ikonerna svåra att klicka på.
settings-appearance-theme = Tema
settings-appearance-theme-system = Samma som skrivbordet
settings-appearance-theme-light = Ljust
settings-appearance-theme-dark = Mörkt
settings-appearance-desktop-colors = Skrivbordets färger
settings-appearance-desktop-colors-use = Använd skrivbordets färger
settings-appearance-desktop-colors-use-detail = Skrivbordets färgschema och accentfärg
settings-appearance-app-names = Appnamn
settings-appearance-app-names-show = Visa appnamn
settings-appearance-app-names-show-detail = Namn under appikonerna längst till vänster
settings-appearance-sender-pictures = Avsändarbilder
settings-appearance-sender-pictures-show = Visa företagslogotyper
settings-appearance-sender-pictures-show-detail = Hämtas utifrån avsändarens domän, aldrig per meddelande, och sparas i en vecka
settings-appearance-important = Viktigt-markeringar
settings-appearance-important-show = Visa Viktigt-markeringar
settings-appearance-important-show-detail = Bredvid varje meddelande i listan
settings-appearance-message-width = Meddelandebredd
settings-appearance-message-width-limit = Begränsa meddelandenas bredd
settings-appearance-message-width-limit-detail = Långa rader är lättare att läsa i ett brett fönster
settings-appearance-mail-colors = E-postfärger
settings-appearance-mail-colors-detail = Det mesta av all e-post är utformat för en vit sida. Med ett mörkt tema byts färgerna mot mörka som är lätta att läsa; om det är av behåller e-posten avsändarens färger på en ljus sida.
settings-appearance-dark-mail = Mörka färger även för e-post
settings-appearance-dark-mail-detail = Bara när temat är mörkt
settings-appearance-attachment-previews = Förhandsvisning av bilagor
settings-appearance-attachment-previews-show = Visa förhandsvisningar av bilagor
settings-appearance-attachment-previews-show-detail = En liten bild av varje fils innehåll på dess kort

## Settings > Default apps

settings-default-apps-intro = Var bilagor öppnas när du klickar på dem. Visningsprogrammet kan alltid öppna en fil i en annan app också. Skrivbordets standardappar ställs in i dess egna inställningar.
settings-default-apps-pdf = PDF-filer
settings-default-apps-pdf-detail = Sidor, med zoom.
settings-default-apps-pictures = Bilder
settings-default-apps-pictures-detail = Foton (rättvända), PNG, GIF, WebP, BMP, TIFF och SVG.
settings-default-apps-text = Textfiler
settings-default-apps-text-detail = Oformaterad text, loggar, kod och annan text.
settings-default-apps-sheets = Kalkylblad
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) och CSV.
settings-default-apps-documents = Dokument
settings-default-apps-documents-detail = Word (docx) och OpenDocument-text (odt).
settings-default-apps-katna = Katna Mails visningsprogram
settings-default-apps-system = Skrivbordets standardapp
settings-default-apps-ask = Fråga efter app varje gång
settings-default-apps-after-saving = Efter sparande
settings-default-apps-show-folder = Visa sparade filer i deras mapp
settings-default-apps-show-folder-detail = Öppnar filhanteraren med de sparade bilagorna markerade

## Settings > Compose

settings-compose-send-from = Skicka nya meddelanden från
settings-compose-send-from-detail = Svar och vidarebefordringar skickas alltid från kontot du befinner dig i.
settings-compose-send-from-current = Kontot du befinner dig i
settings-compose-send-on-replies = Skicka vid svar
settings-compose-send-on-replies-detail = Vad Skicka gör vid ett svar eller en vidarebefordran. Menyn bredvid Skicka erbjuder det andra.
settings-compose-send-plain = Skicka
settings-compose-send-archive = Skicka och arkivera
settings-compose-signatures = Signaturer
settings-compose-signatures-detail = Läggs till under ditt meddelande, efter en rad med ”--”. Välj en annan i skrivfönstret.
settings-compose-untitled = Namnlös
settings-compose-signature-name = Namn, till exempel Jobb
settings-compose-signature-first = Min signatur
settings-compose-signature-numbered = Signatur { $number }
settings-compose-signature-delete = Radera
settings-compose-signature-deleted = Signaturen har raderats
settings-compose-signature-new = Skapa ny
settings-compose-no-signatures = Inga signaturer än.
settings-compose-no-signature = Ingen signatur
settings-compose-for-new-mail = För ny e-post
settings-compose-for-replies = För svar och vidarebefordringar
settings-compose-for-replies-detail = I en konversation där du har signerat ett meddelande börjar ett svar i stället med den signaturen.
settings-compose-format = Format
settings-compose-plain-text = Skriv med oformaterad text
settings-compose-plain-text-detail = Ny e-post börjar utan formatering; skrivfönstret kan byta
settings-compose-spelling = Stavning
settings-compose-spell-check = Kontrollera stavningen medan jag skriver
settings-compose-spell-check-detail = Felstavade ord stryks under, med förslag vid högerklick
settings-compose-spell-desktop = Skrivbordets språk ({ $language })
settings-compose-templates = Mallar
settings-compose-templates-detail = Spara e-post du ofta skriver, och börja nya meddelanden eller svar utifrån den.

## Settings > Shortcuts

settings-shortcuts-set = Uppsättning kortkommandon
settings-shortcuts-set-detail = Börja med tangenterna från en e-postapp du känner till. Cmd är Ctrl här. Dina egna ändringar ligger kvar ovanpå uppsättningen, och Återställ standardvärden går tillbaka till uppsättningens tangenter.
settings-shortcuts-single = Kortkommandon med en tangent
settings-shortcuts-single-detail = Tangenter utan Ctrl eller Alt, som i webbmejl: e arkiverar, j och k flyttar, / söker. De fungerar i listan och i den öppna konversationen, aldrig medan du skriver.
settings-shortcuts-single-use = Använd kortkommandon med en tangent
settings-shortcuts-single-use-detail = Kortkommandon med Ctrl fungerar alltid
settings-shortcuts-how = Klicka på en tangent för att ändra den, eller på + för att lägga till en, och tryck sedan på de nya tangenterna. Esc avbryter.
settings-shortcuts-restore = Återställ standardvärden
settings-shortcuts-no-key = Ingen tangent
settings-shortcuts-press = Tryck på tangenter…
settings-shortcuts-then = { $keys } sedan…
settings-shortcuts-moved = { $keys } gör nu ”{ $action }” i stället för ”{ $previous }”.
settings-shortcuts-single-off = Kortkommandon med en tangent är av, så den här tangenten fungerar när de slås på.
settings-shortcuts-restored = Alla kortkommandon har uppsättningens tangenter igen.

## Settings search: the line under a result

settings-general-language-summary = Språk för appen, datum och tal
settings-general-reading-summary = Nyaste meddelandet först, fullständiga rubriker, mottagarnas fullständiga namn
settings-general-mark-read-summary = När en öppnad konversation markeras som läst: direkt, efter 1 eller 3 sekunder eller manuellt
settings-general-reply-button-summary = Svarsknappen bredvid varje meddelande svarar alla
settings-general-remote-images-summary = Visa alltid bilderna i alla meddelanden
settings-general-sending-summary = Ångra skicka: hur länge ett skickat meddelande väntar, så att det går att ångra
settings-general-offline-summary = Hur många dagars ny e-post som hämtas i sin helhet, för att läsas utan anslutning
settings-general-notifications-summary = Aviseringar om ny e-post och deras ljud
settings-general-desktop-summary = Öppna Katna Mail vid inloggning, ikonen i systemfältet och antalet olästa på aktivitetsfältets ikon
settings-accounts-accounts-summary = Lägg till eller ta bort ett konto, eller byt dess bild
settings-appearance-density-summary = Standardrader eller kompakta rader i listan
settings-appearance-scaling-summary = Gör allt större eller mindre: text, ikoner, avstånd och avdelare
settings-appearance-theme-summary = Samma som skrivbordet, ljust eller mörkt
settings-appearance-sender-pictures-summary = Företagslogotyper, hämtade utifrån avsändarens domän
settings-appearance-important-summary = Viktigt-markeringen bredvid varje meddelande i listan
settings-appearance-mail-colors-summary = Mörka färger för HTML-e-post i ett mörkt tema, eller avsändarens färger
settings-appearance-attachment-previews-summary = En liten bild av varje bilagas innehåll
settings-shortcuts-set-summary = Börja med tangenterna från Gmail, Inbox by Gmail, Apple Mail, Outlook eller Thunderbird
settings-shortcuts-single-summary = Tangenter utan Ctrl eller Alt, som i webbmejl
settings-default-apps-pdf-summary = Var PDF-bilagor öppnas
settings-default-apps-pictures-summary = Var foton och bilder öppnas
settings-default-apps-text-summary = Var oformaterad text, loggar och kod öppnas
settings-default-apps-sheets-summary = Var Excel-, OpenDocument- och CSV-filer öppnas
settings-default-apps-documents-summary = Var Word- och OpenDocument-text öppnas
settings-default-apps-after-saving-summary = Visa sparade bilagor i deras mapp
settings-compose-send-from-summary = Kontot som ny e-post skickas från: det du befinner dig i, eller alltid samma
settings-compose-send-on-replies-summary = Skicka, eller Skicka och arkivera konversationen, vid svar och vidarebefordringar
settings-compose-signatures-summary = Läggs till under ditt meddelande, efter en rad med ”--”
settings-compose-for-new-mail-summary = Signaturen som ny e-post börjar med
settings-compose-for-replies-summary = Signaturen som svar och vidarebefordringar börjar med
settings-compose-format-summary = Skriv ny e-post med oformaterad text
settings-compose-spelling-summary = Kontrollera stavningen medan du skriver, och ordlistans språk
settings-compose-templates-summary = Kommer snart: spara e-post du ofta skriver, och börja nya meddelanden eller svar utifrån den
settings-feedback-crash-reports-summary = Spara kraschrapporter på den här datorn när Katna Mail eller dess bakgrundstjänst kraschar
settings-feedback-saved-summary = Visa, kopiera eller radera kraschrapporterna som har sparats på den här datorn
settings-feedback-help-improve-summary = Skicka kraschrapporter för att hjälpa till att rätta det som gick fel; av om du inte slår på det
settings-experimental-blur-summary = Skrivbordet syns suddigt genom den övre raden, och menyerna är frostade
settings-search-shortcut = Kortkommando
settings-search-tab = Flik i Inställningar
settings-search-none = Inga inställningar matchar ”{ $query }”.
settings-search-results = Inställningar som matchar ”{ $query }”

## Settings: opening at login

settings-open-at-login-failed = Det gick inte att ändra öppning vid inloggning: { $error }
