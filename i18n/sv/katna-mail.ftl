# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Språk: { $language }
language-tooltip-system = Språk: { $language }, följer systemet
language-search = Sök språk
language-system-default = Systemstandard
language-system-now = Nu { $language }
language-no-match = Inget språk matchar ”{ $query }”
language-machine = Maskinöversatt. Hjälp till att förbättra
language-setting = Språk
language-setting-detail = Språket i menyer, knappar och meddelanden samt formatet för datum och tal. Systemstandard följer skrivbordet.

## Dates and sizes

ago-just-now = nyss
ago-minutes = { $count ->
    [one] för { $count } minut sedan
   *[other] för { $count } minuter sedan
}
ago-hours = { $count ->
    [one] för { $count } timme sedan
   *[other] för { $count } timmar sedan
}
ago-days = { $count ->
    [one] för { $count } dag sedan
   *[other] för { $count } dagar sedan
}
size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } byte
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Dölj mappar
folders-show = Visa mappar
compose = Skriv
search = Sök
search-mail = Sök i e-post
search-settings = Sök i inställningar
search-clear = Rensa sökning
search-options-show = Visa sökalternativ
settings = Inställningar
account-add = Lägg till ett konto

## App rail (and the bottom bar on a phone)

rail-mail = E-post
rail-calendar = Kalender
rail-contacts = Kontakter
rail-tasks = Uppgifter
rail-notes = Anteckningar
rail-feeds = Flöden

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Kommer snart
app-calendar-promise = Dina CalDAV-kalendrar, mötesinbjudningar från din e-post och påminnelser, bredvid inkorgen.
app-tasks-promise = Att göra-listor som synkroniseras med CalDAV, och uppgifter som skapats från e-post.
app-notes-promise = Snabba anteckningar, och anteckningar om ett meddelande eller en konversation till senare.
app-feeds-promise = Läs RSS- och Atom-flöden bredvid din e-post.

## Contacts page

app-contacts-loading = Samlar personer från din e-post…
app-contacts-empty = Personer du skriver med visas här.
app-contacts-count = { $count ->
    [one] { $count } person från din e-post, de du skriver mest med först
   *[other] { $count } personer från din e-post, de du skriver mest med först
}
app-contacts-top = { $count ->
    [one] Personen du skriver mest med
   *[other] De { $count } personer du skriver mest med, flest först
}
app-contacts-messages = { $count ->
    [one] { $count } meddelande
   *[other] { $count } meddelanden
}
app-contacts-last = senast { $date }

## Navigation (the folders pane)

nav-labels = Etiketter
nav-folders = Mappar
nav-label-new = Skapa ny etikett
nav-folder-new = Skapa ny mapp
nav-account-unnamed = Konto { $number }
nav-tab-new = { $count ->
    [one] { $count } nytt
   *[other] { $count } nya
}

## Special folders (the user's own folders keep their names)

folder-inbox = Inkorgen
folder-starred = Stjärnmärkt
folder-drafts = Utkast
folder-sent = Skickat
folder-archive = Arkiv
folder-spam = Skräppost
folder-trash = Papperskorgen
folder-all-mail = Alla mail
folder-scheduled = Schemalagt

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Ny etikett
label-folder-new-title = Ny mapp
label-prompt = Ange ett nytt etikettnamn:
label-folder-prompt = Ange ett nytt mappnamn:
label-name-hint = Etikettnamn
label-folder-name-hint = Mappnamn
label-nest = Kapsla etikett under:
label-folder-nest = Kapsla mapp under:
label-cancel = Avbryt
label-create = Skapa
label-creating = Skapar…
label-created = Etiketten ”{ $name }” har skapats.
label-folder-created = Mappen ”{ $name }” har skapats.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Primär
tab-promotions = Kampanjer
tab-social = Socialt
tab-updates = Uppdateringar
tab-forums = Forum
tab-focused = Prioriterat
tab-other = Övrigt
tab-inbox = Inkorgen
tab-newsletters = Nyhetsbrev
tab-notifications = Aviseringar
tab-new = { $count ->
    [one] { $count } nytt
   *[other] { $count } nya
}
tab-provider-other = sorteras av Katna

## Mail list: toolbar

list-select = Markera
list-refresh = Uppdatera
list-more = Mer
list-mark-read = Markera som läst
list-mark-unread = Markera som oläst
list-move-to = Flytta till
list-archive = Arkivera
list-spam = Rapportera som skräppost
list-delete = Radera
list-newer = Nyare
list-older = Äldre
list-range = { $first }–{ $last } av { $total }
list-range-about = { $first }–{ $last } av cirka { $total }
list-results = Resultat för ”{ $query }”
list-results-corrected = Visar resultat för ”{ $query }”
list-search-instead = Sök i stället efter ”{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Alla
list-pick-none = Inga
list-pick-read = Lästa
list-pick-unread = Olästa
list-pick-starred = Stjärnmärkta
list-pick-unstarred = Utan stjärna

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } konversation har markerats.
       *[other] Alla { $count } konversationer har markerats.
    }
   *[message] { $count ->
        [one] { $count } meddelande har markerats.
       *[other] Alla { $count } meddelanden har markerats.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } konversation i { $folder } har markerats.
       *[other] Alla { $count } konversationer i { $folder } har markerats.
    }
   *[message] { $count ->
        [one] { $count } meddelande i { $folder } har markerats.
       *[other] Alla { $count } meddelanden i { $folder } har markerats.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } konversation på den här sidan har markerats.
       *[other] Alla { $count } konversationer på den här sidan har markerats.
    }
   *[message] { $count ->
        [one] { $count } meddelande på den här sidan har markerats.
       *[other] Alla { $count } meddelanden på den här sidan har markerats.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Markera { $count } konversation
       *[other] Markera alla { $count } konversationer
    }
   *[message] { $count ->
        [one] Markera { $count } meddelande
       *[other] Markera alla { $count } meddelanden
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Markera { $count } konversation i { $folder }
       *[other] Markera alla { $count } konversationer i { $folder }
    }
   *[message] { $count ->
        [one] Markera { $count } meddelande i { $folder }
       *[other] Markera alla { $count } meddelanden i { $folder }
    }
}
list-clear-selection = Rensa markering

## Mail list: empty states

list-empty-search = Inga meddelanden matchade sökningen.
list-empty-tab = Ingen e-post i { $tab }.
list-empty-tab-unknown = Ingen e-post på den här fliken.
list-empty-folder = Inga meddelanden i { $folder }.
list-empty-folder-unknown = Inga meddelanden i den här mappen.
list-first-sync = Hämtar din e-post…
list-first-sync-detail = Den visas här allt eftersom den kommer in.

## Mail list: lines

row-removed = Meddelandet har tagits bort.
row-starred = Stjärnmärkt
row-not-starred = Inte stjärnmärkt
row-important = Viktigt. Klicka för att markera som inte viktigt.
row-mark-important = Markera som viktigt
row-pinned = Fäst högst upp
row-pin = Fäst högst upp
row-unpin = Lossa

## Mail list: More menu and right-click menu

menu-reply = Svara
menu-reply-all = Svara alla
menu-forward = Vidarebefordra
menu-archive = Arkivera
menu-delete = Radera
menu-spam = Rapportera som skräppost
menu-mark-read = Markera som läst
menu-mark-unread = Markera som oläst
menu-mark-all-read = Markera alla som lästa
menu-star = Lägg till stjärna
menu-unstar = Ta bort stjärna
menu-important = Markera som viktigt
menu-not-important = Markera som inte viktigt
menu-pin = Fäst högst upp
menu-unpin = Lossa
menu-print-all = Skriv ut alla
menu-new-window = Öppna i nytt fönster
menu-move-to = Flytta till
menu-move-to-heading = Flytta till:
menu-find-from = Hitta e-post från { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har arkiverats.
       *[other] { $count } konversationer har arkiverats.
    }
   *[message] { $count ->
        [one] Meddelandet har arkiverats.
       *[other] { $count } meddelanden har arkiverats.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har flyttats till papperskorgen.
       *[other] { $count } konversationer har flyttats till papperskorgen.
    }
   *[message] { $count ->
        [one] Meddelandet har flyttats till papperskorgen.
       *[other] { $count } meddelanden har flyttats till papperskorgen.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har flyttats.
       *[other] { $count } konversationer har flyttats.
    }
   *[message] { $count ->
        [one] Meddelandet har flyttats.
       *[other] { $count } meddelanden har flyttats.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har stjärnmärkts.
       *[other] { $count } konversationer har stjärnmärkts.
    }
   *[message] { $count ->
        [one] Meddelandet har stjärnmärkts.
       *[other] { $count } meddelanden har stjärnmärkts.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Stjärnan har tagits bort från konversationen.
       *[other] Stjärnan har tagits bort från { $count } konversationer.
    }
   *[message] { $count ->
        [one] Stjärnan har tagits bort från meddelandet.
       *[other] Stjärnan har tagits bort från { $count } meddelanden.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har markerats som viktig.
       *[other] { $count } konversationer har markerats som viktiga.
    }
   *[message] { $count ->
        [one] Meddelandet har markerats som viktigt.
       *[other] { $count } meddelanden har markerats som viktiga.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har markerats som inte viktig.
       *[other] { $count } konversationer har markerats som inte viktiga.
    }
   *[message] { $count ->
        [one] Meddelandet har markerats som inte viktigt.
       *[other] { $count } meddelanden har markerats som inte viktiga.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har fästs högst upp.
       *[other] { $count } konversationer har fästs högst upp.
    }
   *[message] { $count ->
        [one] Meddelandet har fästs högst upp.
       *[other] { $count } meddelanden har fästs högst upp.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har lossats.
       *[other] { $count } konversationer har lossats.
    }
   *[message] { $count ->
        [one] Meddelandet har lossats.
       *[other] { $count } meddelanden har lossats.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har rapporterats som skräppost.
       *[other] { $count } konversationer har rapporterats som skräppost.
    }
   *[message] { $count ->
        [one] Meddelandet har rapporterats som skräppost.
       *[other] { $count } meddelanden har rapporterats som skräppost.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har raderats permanent.
       *[other] { $count } konversationer har raderats permanent.
    }
   *[message] { $count ->
        [one] Meddelandet har raderats permanent.
       *[other] { $count } meddelanden har raderats permanent.
    }
}
toast-undone = Åtgärden har ångrats.
toast-undo = Ångra
toast-no-spam-folder = Det här kontot har ingen skräppostmapp.

## Reading pane: toolbar

reader-close = Stäng
reader-back = Tillbaka
reader-mark-unread = Markera som oläst
reader-move-to = Flytta till
reader-more = Mer
reader-print-all = Skriv ut alla
reader-new-window = I nytt fönster
reader-position = { $position } av { $total }
reader-newer = Nyare
reader-older = Äldre

## Reading pane: the conversation

reader-removed = Konversationen har tagits bort.
reader-no-subject = (inget ämne)
reader-collapse-all = Komprimera alla
reader-expand-all = Expandera alla
reader-unknown-sender = (okänd avsändare)
reader-date-ago = { $date } ({ $ago })
reader-me = mig
reader-to = till { $names }
reader-starred = Stjärnmärkt
reader-not-starred = Inte stjärnmärkt
reader-too-long = Meddelandet är för långt för att visas i sin helhet.
reader-encrypted-images = Bilder från webben läses aldrig in i krypterad e-post.
reader-window-failed = Det gick inte att öppna ett nytt fönster.

## Reading pane: message details (opened from "to me")

reader-details-from = från:
reader-details-to = till:
reader-details-cc = kopia:
reader-details-date = datum:
reader-details-subject = ämne:

## Reading pane: downloading a message

reader-downloading = Hämtar meddelandet från servern…
reader-download-failed = Det gick inte att hämta meddelandet.
reader-try-again = Försök igen

## Reply row

reply-reply = Svara
reply-reply-all = Svara alla
reply-forward = Vidarebefordra

## Encrypted and signed mail

security-decrypting = Dekrypterar…
security-checking = Kontrollerar signaturen…
security-partly-encrypted = Endast en del av meddelandet är krypterad. Resten lades till utanför skyddet och kan komma från vem som helst.
security-partly-signed = Endast en del av meddelandet är signerad. Resten lades till utanför skyddet och kan komma från vem som helst.
security-encrypted = Krypterat meddelande
security-encrypted-smime = Krypterat meddelande (S/MIME)
security-no-key = Det går inte att dekryptera meddelandet: det krypterades för en nyckel som du inte har.
security-cancelled = Dekrypteringen avbröts.
security-damaged = Det går inte att dekryptera meddelandet: krypterade data är skadade eller har ändrats.
security-decrypt-unavailable = Det går inte att dekryptera meddelandet: installera { $tool } för att läsa krypterad e-post.
security-decrypt-failed = Det går inte att dekryptera meddelandet: { $reason }
security-unknown-signer = en okänd signerare
security-signed-verified = Signerat av { $signer } · verifierat
security-signed-not-sender = Signerat av { $signer }, som inte är avsändaren
security-signed-untrusted = Signerat av { $signer }, med en nyckel som du har markerat som ej betrodd
security-signed-unverified = Signerat av { $signer } · nyckeln är inte verifierad
security-bad-signature = Ogiltig signatur: meddelandet ändrades efter att det signerades, eller så är signaturen förfalskad.
security-signature-expired = Signerat av { $signer } · signaturen har gått ut
security-key-expired = Signerat av { $signer } · nyckeln har gått ut sedan dess
security-key-revoked = Signerat av { $signer } med en nyckel som har återkallats
security-missing-key = Signerat med en nyckel som du inte har, så det kan inte kontrolleras
security-missing-key-id = Signerat med en nyckel som du inte har ({ $key }), så det kan inte kontrolleras
security-signature-unavailable = Signerat; installera { $tool } för att kontrollera signaturen
security-signature-error = Signaturen kunde inte kontrolleras.

## Remote images and pictures

remote-hidden = Bilder i det här meddelandet är dolda.
remote-show = Visa bilder
remote-always-show = Visa alltid från den här avsändaren
remote-picture-use = Använd
remote-picture-too-big = Välj en bild på högst 8 MB.
remote-picture-type = Välj en PNG-, JPEG-, GIF-, WebP- eller SVG-bild.
remote-picture-read-failed = Det går inte att läsa bilden: { $error }
remote-picture-keep-failed = Det går inte att spara bilden: { $error }
remote-picture-remove-failed = Det går inte att ta bort bilden: { $error }

## Attachments

attachment-count = { $count ->
    [one] En bilaga
   *[other] { $count } bilagor
}
attachment-save = Spara
attachment-save-all = Spara alla
attachment-save-all-tooltip = Spara alla bilagor i en mapp
attachment-save-here = Spara här
attachment-not-downloaded = Meddelandet är inte hämtat.
attachment-not-found = Bilagan hittades inte i meddelandet.
attachment-read-failed = Det gick inte att läsa { $name }
attachment-numbered = bilaga { $number }
attachment-saved-all = { $count ->
    [one] { $count } fil sparades i { $place }
   *[other] { $count } filer sparades i { $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } av { $total } fil sparades i { $place }. Det gick inte att spara { $failed }
   *[other] { $saved } av { $total } filer sparades i { $place }. Det gick inte att spara { $failed }
}
attachment-saved-to = Sparad i { $path }
attachment-save-failed = Det gick inte att spara { $name }: { $error }
attachment-open-failed = Det gick inte att öppna { $name }: { $error }
attachment-risky = Filen kan köra ett program, så Katna öppnar den inte. Spara den i stället.
attachment-encrypted-open = Filen kom krypterad. Spara den för att öppna den någon annanstans.

## Printing

print-failed = Det gick inte att skriva ut: { $error }
print-no-font = inget typsnitt hittades
print-opened-as-pdf = Öppnades som PDF för utskrift därifrån.
print-not-downloaded = (Inte hämtat än.)
print-encrypted = (Krypterat. Öppna det i Katna Mail för att skriva ut texten.)
print-to = Till: { $addresses }
print-cc = Kopia: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Öppna meddelandet för att läsa bilagorna.
text-copy = Kopiera
text-select-all = Markera allt

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
## Quick settings (the panel that slides in from the right)

quick-title = Snabbinställningar
quick-see-all = Visa alla inställningar
quick-reading-pane = Läsruta
quick-pane-right = Till höger om listan
quick-pane-none = Ingen delning
quick-density = Täthet
quick-density-default = Standard
quick-density-compact = Kompakt
quick-theme = Tema
quick-theme-system = Samma som skrivbordet
quick-theme-light = Ljust
quick-theme-dark = Mörkt
quick-desktop-colors = Skrivbordets färger
quick-desktop-colors-detail = Skrivbordets färgschema och accentfärg
quick-app-names = Appnamn
quick-app-names-detail = Namn under appikonerna längst till vänster
quick-inbox-tabs = Inkorgsflikar
quick-inbox-tabs-detail = Flikarna från varje kontos e-postleverantör
quick-choose-tabs = Välj flikar
quick-choose-tabs-detail = Per konto, i Inställningar
quick-sending = Skicka
quick-undo-send = Ångra skicka
quick-undo-send-off = Av
quick-undo-send-seconds = { $seconds } s
quick-signatures = Signaturer
quick-signatures-none = Inga än
quick-signatures-one = { $name }, används som standard
quick-signatures-many = { $count ->
    [one] { $count } signatur; { $name } som standard
   *[other] { $count } signaturer; { $name } som standard
}
quick-signatures-no-default = { $count ->
    [one] { $count }, ingen som standard
   *[other] { $count }, ingen som standard
}
quick-signature-untitled = Namnlös
quick-threading = Trådar
quick-conversation-view = Konversationsvy
quick-conversation-view-detail = Gruppera svar på samma e-post
quick-help = Hjälp
quick-tour = Gå igenom rundturen
quick-whats-new = Nyheter
quick-about = Om Katna

## Settings: opening at login

settings-open-at-login-failed = Det gick inte att ändra öppning vid inloggning: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent } %
scale-reset = Tillbaka till { $percent } %

## Settings > Experimental > Look & Feel

look-intro = Funktioner som fortfarande testas. De kan ändras eller försvinna.
look-heading = Utseende och känsla
look-window-frame = Fönsterram
look-window-frame-detail = Vem som ritar namnlisten, fönsterknapparna, hörnen och skuggan.
look-frame-native-kde = Inbyggd: KDE:s ram, i ditt Plasma-tema
look-frame-native = Inbyggd: skrivbordets ram
look-frame-katna = Katna: den övre raden blir namnlisten
look-frame-katna-note-named = Katna ritar rundade hörn och en egen skugga. Ramen följer inte längre { $desktop }-temat; fönsterregler gäller fortfarande.
look-frame-katna-note = Katna ritar rundade hörn och en egen skugga. Ramen följer inte längre skrivbordstemat; fönsterregler gäller fortfarande.
look-frame-client-side = Ditt skrivbord överlåter ramen åt varje app, så Katna ritar redan sin egen.
look-blurred-background = Suddig bakgrund
look-blurred-background-detail = Skrivbordet syns suddigt genom den övre raden och mapparna, och menyer och popup-rutor är av frostat glas.
look-blur = Gör det som finns bakom fönstret suddigt
look-blur-detail = E-posten ligger kvar på heltäckande kort, så att texten behåller sin kontrast
look-blur-off-kde = KDE:s oskärpeeffekt är av. Slå på Oskärpa i Systeminställningar, Fönsterhantering, Skrivbordseffekter och öppna sedan Katna Mail igen.
look-blur-none-gnome = GNOME gör inte det som finns bakom fönster suddigt.
look-blur-none-x11 = Din fönsterhanterare gör inte det som finns bakom fönster suddigt.
look-blur-none-wayland = Din kompositor gör inte det som finns bakom fönster suddigt.

## Settings > User feedback (crash reports)

feedback-intro-sending = Nya kraschrapporter skickas för att hjälpa till att rätta det som gick fel. Inget annat lämnar den här datorn.
feedback-intro-local = Katna skickar ingenting någonstans. Kraschrapporter stannar på den här datorn, så att du kan titta på dem eller bifoga dem till en felrapport.
feedback-crash-reports = Kraschrapporter
feedback-crash-reports-detail = Skrivs när Katna Mail eller dess bakgrundstjänst kraschar.
feedback-save = Spara kraschrapporter på den här datorn
feedback-save-detail = Din hemmapp, användar- och datornamn samt e-postadresser utelämnas
feedback-saved = Sparade kraschrapporter
feedback-saved-detail = { $count ->
    [one] Den nyaste sparas.
   *[other] De { $count } nyaste sparas.
}
feedback-help-improve = Hjälp till att förbättra Katna
feedback-help-improve-detail = Av om du inte slår på det, och du kan när som helst stänga av det här.
feedback-send = Skicka kraschrapporter
feedback-send-detail = Den sparade rapporten, exakt som du kan visa den här, skickas till Katnas kraschspårare (Sentry, i EU). Ingen IP-adress, inga meddelanden eller e-postadresser
feedback-none-saved = Inga kraschrapporter har sparats.
feedback-delete-all = Radera alla
feedback-app-daemon = Bakgrundstjänst
feedback-report-sent = { $date } · Skickad
feedback-view = Visa
feedback-view-tooltip = Öppna rapporten
feedback-copy-tooltip = Kopiera den för att klistra in i en felrapport
feedback-copied = Kraschrapporten har kopierats.
feedback-deleted-all = Kraschrapporterna har raderats.
feedback-read-failed = Det gick inte att läsa kraschrapporten: { $error }
feedback-delete-failed = Det gick inte att radera kraschrapporten: { $error }
feedback-delete-all-failed = Det gick inte att radera kraschrapporterna: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Arkiv
desktop-menu-new-message = _Nytt meddelande
desktop-menu-quit = A_vsluta
desktop-menu-edit = R_edigera
desktop-menu-undo = _Ångra
desktop-menu-select-all = Markera _alla
desktop-menu-select-none = Markera _inga
desktop-menu-find = _Sök…
desktop-menu-view = _Visa
desktop-menu-folder-list = Visa _mapplista
desktop-menu-refresh = _Uppdatera
desktop-menu-go = _Gå
desktop-menu-inbox = _Inkorgen
desktop-menu-starred = _Stjärnmärkt
desktop-menu-sent = S_kickat
desktop-menu-drafts = _Utkast
desktop-menu-all-mail = _Alla mail
desktop-menu-next = _Nästa konversation
desktop-menu-previous = _Föregående konversation
desktop-menu-message = _Meddelande
desktop-menu-open = _Öppna
desktop-menu-reply = _Svara
desktop-menu-reply-all = Svara _alla
desktop-menu-forward = _Vidarebefordra
desktop-menu-archive = Ar_kivera
desktop-menu-delete = _Radera
desktop-menu-spam = Rapportera som s_kräppost
desktop-menu-move-to = _Flytta till…
desktop-menu-mark-read = Markera som _läst
desktop-menu-mark-unread = Markera som _oläst
desktop-menu-star = S_tjärnmärk
desktop-menu-important = Markera som vi_ktigt
desktop-menu-not-important = Markera som _inte viktigt
desktop-menu-settings = _Inställningar
desktop-menu-quick-settings = _Snabbinställningar
desktop-menu-configure = _Anpassa Katna Mail…
desktop-menu-help = _Hjälp
desktop-menu-shortcuts = _Kortkommandon
desktop-menu-whats-new = _Nyheter
desktop-menu-about = _Om Katna
## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Navigera
shortcut-group-actions = Åtgärder
shortcut-group-go-to = Gå till
shortcut-group-app = Program

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Nästa konversation
shortcut-previous = Föregående konversation
shortcut-down = Nedåt i listan
shortcut-up = Uppåt i listan
shortcut-first = Först i listan
shortcut-last = Sist i listan
shortcut-page-down = En sida ned i listan
shortcut-page-up = En sida upp i listan
shortcut-open = Öppna konversation
shortcut-back = Tillbaka till listan
shortcut-scroll-down = Rulla nedåt
shortcut-scroll-up = Rulla uppåt
shortcut-scroll-page-down = Rulla en sida nedåt
shortcut-scroll-page-up = Rulla en sida uppåt
shortcut-compose = Skriv
shortcut-reply = Svara
shortcut-reply-all = Svara alla
shortcut-forward = Vidarebefordra
shortcut-archive = Arkivera
shortcut-delete = Radera
shortcut-spam = Rapportera som skräppost
shortcut-move-to = Flytta till
shortcut-mark-read = Markera som läst
shortcut-mark-unread = Markera som oläst
shortcut-star = Lägg till eller ta bort stjärna
shortcut-important = Markera som viktigt
shortcut-not-important = Markera som inte viktigt
shortcut-check = Bocka för konversationen
shortcut-select-all = Bocka för alla konversationer
shortcut-select-none = Bocka av alla konversationer
shortcut-undo = Ångra senaste åtgärden
shortcut-go-inbox = Inkorgen
shortcut-go-starred = Stjärnmärkt
shortcut-go-sent = Skickat
shortcut-go-drafts = Utkast
shortcut-go-all = Alla mail
shortcut-search = Sök i e-post
shortcut-navigation = Visa eller fäll ihop menyn
shortcut-quick-settings = Snabbinställningar
shortcut-settings = Alla inställningar
shortcut-shortcuts = Kortkommandon
shortcut-reload = Sök efter ny e-post
shortcut-quit = Avsluta

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } sedan { $second }

## Settings > Accounts

accounts-folder-pane = Mappfönster
accounts-folder-pane-detail = Vilka kontons mappar fönstret till vänster visar.
accounts-shown-one = Ett konto i taget; byt i kontokortet
accounts-shown-all = Alla konton, efter varandra
accounts-row = Konton
accounts-row-detail = När du tar bort ett konto raderas Katnas kopia av dess e-post på den här datorn. E-posten finns kvar på servern.
accounts-none = Inga konton än.
accounts-kind-imported = Importerat
accounts-picture-reset = Använd skrivbordets bild
accounts-picture-change = Byt bild
accounts-remove = Ta bort
accounts-delete-all-row = Radera all data
accounts-delete-all-row-detail = Börja om, som vid en ny installation.
accounts-delete-all-about = Raderar alla konton, all sparad e-post, kontakter och kalendrar, sökindexet, dina inställningar och sparade lösenord från den här datorn. Ingenting ändras på dina e-postservrar.
accounts-delete-all-open = Radera all Katna-data

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } har tagits bort från Katna.
accounts-removed = { $address } har tagits bort från Katna. E-posten finns kvar på servern.
accounts-all-deleted = All Katna-data har raderats från den här datorn.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Ta bort { $address }?
accounts-remove-confirm = Ta bort konto
accounts-removing = Tar bort…
accounts-remove-local-mail = { $folders ->
    [0] All e-post som har importerats till kontot
    [one] All e-post som har importerats till kontot, i dess mapp
   *[other] All e-post som har importerats till kontot, i dess { $folders } mappar
}
accounts-remove-local-settings = Dess Katna-inställningar
accounts-remove-mail = { $folders ->
    [0] All e-post från kontot som Katna har sparat
    [one] All e-post från kontot som Katna har sparat, i dess mapp
   *[other] All e-post från kontot som Katna har sparat, i dess { $folders } mappar
}
accounts-remove-outbox = Dess meddelanden som väntar i utkorgen
accounts-remove-settings = Dess sparade lösenord och dess Katna-inställningar
accounts-delete-all-title = Radera all Katna-data?
accounts-delete-all-confirm = Radera allt
accounts-deleting = Raderar…
accounts-delete-all-accounts = Alla konton, och all e-post och alla bilagor som Katna har sparat
accounts-delete-all-contacts = Kontakter, kalendrar och sökindexet
accounts-delete-all-settings = Alla inställningar, signaturer och kortkommandon
accounts-delete-all-passwords = Alla sparade lösenord
accounts-deleted-heading = Raderas från den här datorn:
accounts-cannot-undo = Det går inte att ångra.
accounts-server-delete-all = Ingenting ändras på dina e-postservrar: din e-post finns kvar där, och om du lägger till ett konto igen hämtas den igen. E-post som har importerats från filer finns bara i Katna; filerna rörs inte.
accounts-server-local = E-posten har importerats från filer, så Katna har den enda kopian. Filerna den kom från rörs inte; importera dem igen för att få tillbaka den.
accounts-server-remove = Ingenting ändras på e-postservern: din e-post finns kvar där, och om du lägger till kontot igen hämtas den igen.
accounts-confirm-word = radera
accounts-confirm-placeholder = Skriv ”{ accounts-confirm-word }”
accounts-confirm-prompt = Bekräfta genom att skriva ”{ accounts-confirm-word }”:
accounts-cancel = Avbryt
