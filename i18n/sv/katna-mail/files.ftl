# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Sök filer

## Left side (and chips on a phone)

files-all = Alla filer
files-pictures = Bilder
files-pdfs = PDF-filer
files-documents = Dokument
files-sheets = Kalkylblad
files-slides = Presentationer
files-other = Övrigt
files-accounts = Konton
files-drives = Molnlagring
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Delade med mig
files-shown = Visas
files-received = Mottagna
files-sent = Skickade av mig

## Over the files

files-count = { $count ->
    [one] { $count } fil · { $size }
   *[other] { $count } filer · { $size }
}
files-anyone = Vem som helst
files-from-person = Från { $name }
files-time-any = När som helst
files-time-today = Idag
files-time-yesterday = Igår
files-time-this-week = Den här veckan
files-time-last-week = Förra veckan
files-time-this-month = Den här månaden
files-time-last-month = Förra månaden
files-time-between = { $first } – { $last }
files-time-hint = Klicka på en dag, eller dra över flera dagar
files-time-summary = { $count ->
    [one] { $days } · { $count } fil
   *[other] { $days } · { $count } filer
}
files-time-clear = Rensa
files-time-month-back = Föregående månad
files-time-month-on = Nästa månad
files-time-wheel = Rulla för att flytta datumen och behålla deras längd
files-sort-newest = Nyaste först
files-sort-oldest = Äldsta först
files-sort-largest = Största först
files-sort-name = Efter namn
files-grid = Kort
files-list = Lista
files-this-week = Den här veckan
files-undated = Inget datum
files-me = Jag
files-no-subject = (inget ämne)
files-loading = Samlar ihop filer från din e-post…
files-empty = Filer från din e-post visas här.
files-none-match = Inga filer matchar.
files-load-failed = Det gick inte att läsa filerna: { $error }

## A file's menu and buttons

files-open = Öppna
files-open-with = Öppna med…
files-save = Spara…
files-show-mail = Visa e-postmeddelandet
files-mail-window = Öppna e-postmeddelandet i ett nytt fönster
files-forward = Vidarebefordra filen
files-from-them = Filer från { $name }
files-copy-name = Kopiera filnamn
files-name-copied = Filnamnet har kopierats
files-downloading = Hämtar e-postmeddelandet…
files-download-failed = Det gick inte att hämta det här meddelandet.

## A cloud drive in place of the mail files

files-drive-mine = Min enhet
files-drive-mine-onedrive = Mina filer
files-drive-results = ”{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 fil
       *[other] { $files } filer
    }
    [one] 1 mapp · { $files ->
        [one] 1 fil
       *[other] { $files } filer
    }
   *[other] { $folders } mappar · { $files ->
        [one] 1 fil
       *[other] { $files } filer
    }
}
files-drive-folders = Mappar
files-drive-files = Filer
files-drive-folder = Mapp
files-drive-meta = { $what } · Redigerad { $date }
files-drive-as-link = { $what } · som länk
files-drive-google-doc = Google-dokument
files-drive-google-sheet = Google-kalkylark
files-drive-google-slides = Google-presentation
files-drive-google-drawing = Google-teckning
files-drive-fetching = Hämtar…
files-drive-loading = Öppnar lagringen…
files-drive-empty = Den här mappen är tom.
files-drive-unreachable = Det går inte att nå { $drive }.
files-drive-try-again = Försök igen
files-drive-needs-permission = Katna behöver ditt tillstånd en gång för att visa den här lagringen. Logga in igen och låt Katna se dina filer.
files-drive-allow = Tillåt
files-drive-allow-failed = Inloggningen slutfördes inte, så lagringen förblir stängd.
files-drive-attach = Bifoga
files-drive-more = Mer
files-drive-download = Hämta…
files-drive-open-web = Öppna i { $drive }
files-drive-copy-link = Kopiera länk
files-drive-link-copied = Länken har kopierats
files-drive-share = Dela…
files-drive-rename = Byt namn
files-drive-trash = Flytta till papperskorgen
files-drive-trashed = ”{ $name }” ligger i papperskorgen i { $drive }
files-drive-renamed = Bytte namn till ”{ $name }”
files-drive-getting = Hämtar { $name } från { $drive }…
files-drive-get-failed = Det gick inte att hämta { $name }: { $error }
files-drive-upload = Ladda upp
files-drive-upload-files = Ladda upp filer
files-drive-upload-folder = Ladda upp mapp
files-drive-upload-failed = Det gick inte att ladda upp { $name }: { $error }
files-drive-upload-needs = För att ladda upp behöver Katna ditt tillstånd en gång: tryck på Tillåt i Inställningar › Standardappar › Filsidan.

## The Share dialog of a drive file or folder

files-share-title = Dela ”{ $name }”
files-share-add = Lägg till personer med namn eller adress
files-share-not-address = ”{ $text }” är inte en e-postadress
files-share-notify = Låt { $drive } mejla dem också
files-share-people = Personer med åtkomst
files-share-general = Allmän åtkomst
files-share-loading = Läser vem som har åtkomst…
files-share-restricted = Begränsad
files-share-restricted-about = Bara personer med åtkomst kan öppna den med länken
files-share-anyone = Alla som har länken
files-share-anyone-can = { $role ->
    [editor] Alla som har länken kan redigera
    [commenter] Alla som har länken kan kommentera
   *[viewer] Alla som har länken kan visa
}
files-share-anyone-about = { $role ->
    [editor] Alla på internet som har länken kan redigera
    [commenter] Alla på internet som har länken kan kommentera
   *[viewer] Alla på internet som har länken kan visa
}
files-share-role-owner = Ägare
files-share-role-editor = Redigerare
files-share-role-commenter = Kommentator
files-share-role-viewer = Läsare
files-share-you = { $name } (du)
files-share-domain = Alla på { $domain }
files-share-inherited = Åtkomst från en mapp den ligger i
files-share-remove = Ta bort åtkomst
files-share-copy-link = Kopiera länk
files-share-share = Dela
files-share-done = Klar
files-share-sharing = Delar…
files-share-shared = { $count ->
    [one] Delad med 1 person
   *[other] Delad med { $count } personer
}
files-share-refused = { $drive } kunde inte dela med { $addresses }
files-share-failed = Det gick inte att ändra delningen: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Laddar upp 1 objekt
   *[other] Laddar upp { $count } objekt
}
files-tray-done = { $count ->
    [one] 1 uppladdning klar
   *[other] { $count } uppladdningar klara
}
files-tray-some-failed = { $done } uppladdade, { $failed } misslyckades
files-tray-minutes-left = { $minutes ->
    [one] Ungefär en minut kvar
   *[other] Ungefär { $minutes } minuter kvar
}
files-tray-seconds-left = Mindre än en minut kvar
files-tray-starting = Startar…
files-tray-cancel-all = Avbryt alla
files-tray-cancel = Avbryt
files-tray-fold = Dölj listan
files-tray-unfold = Visa listan
files-tray-close = Stäng
files-tray-progress = { $place } · { $sent } av { $size }
files-tray-in = I { $place }
files-tray-cancelled = Avbruten
