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
