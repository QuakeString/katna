# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Bestanden zoeken

## Left side (and chips on a phone)

files-all = Alle bestanden
files-pictures = Afbeeldingen
files-pdfs = PDF’s
files-documents = Documenten
files-sheets = Spreadsheets
files-slides = Presentaties
files-other = Overige
files-accounts = Accounts
files-drives = Drives
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Gedeeld met mij
files-shown = Getoond
files-received = Ontvangen
files-sent = Door mij verstuurd

## Over the files

files-count = { $count ->
    [one] { $count } bestand · { $size }
   *[other] { $count } bestanden · { $size }
}
files-anyone = Iedereen
files-from-person = Van { $name }
files-time-any = Altijd
files-time-today = Vandaag
files-time-yesterday = Gisteren
files-time-this-week = Deze week
files-time-last-week = Vorige week
files-time-this-month = Deze maand
files-time-last-month = Vorige maand
files-time-between = { $first } – { $last }
files-time-hint = Klik op een dag, of sleep over dagen
files-time-summary = { $count ->
    [one] { $days } · { $count } bestand
   *[other] { $days } · { $count } bestanden
}
files-time-clear = Wissen
files-time-month-back = Vorige maand
files-time-month-on = Volgende maand
files-time-wheel = Scroll om deze datums te verschuiven, met dezelfde lengte
files-sort-newest = Nieuwste eerst
files-sort-oldest = Oudste eerst
files-sort-largest = Grootste eerst
files-sort-name = Op naam
files-grid = Kaarten
files-list = Lijst
files-this-week = Deze week
files-undated = Geen datum
files-me = Ik
files-no-subject = (geen onderwerp)
files-loading = Bestanden uit je e-mail verzamelen…
files-empty = Bestanden uit je e-mail verschijnen hier.
files-none-match = Geen bestanden gevonden.
files-load-failed = Het lezen van de bestanden is mislukt: { $error }

## A file's menu and buttons

files-open = Openen
files-open-with = Openen met…
files-save = Opslaan…
files-show-mail = De e-mail tonen
files-mail-window = De e-mail openen in een nieuw venster
files-forward = Het bestand doorsturen
files-from-them = Bestanden van { $name }
files-copy-name = Bestandsnaam kopiëren
files-name-copied = Bestandsnaam gekopieerd
files-downloading = De e-mail downloaden…
files-download-failed = Kan deze e-mail niet downloaden.

## A cloud drive in place of the mail files

files-drive-mine = Mijn Drive
files-drive-mine-onedrive = Mijn bestanden
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 bestand
       *[other] { $files } bestanden
    }
    [one] 1 map · { $files ->
        [one] 1 bestand
       *[other] { $files } bestanden
    }
   *[other] { $folders } mappen · { $files ->
        [one] 1 bestand
       *[other] { $files } bestanden
    }
}
files-drive-folders = Mappen
files-drive-files = Bestanden
files-drive-folder = Map
files-drive-meta = { $what } · Bewerkt { $date }
files-drive-as-link = { $what } · als link
files-drive-google-doc = Google-document
files-drive-google-sheet = Google-spreadsheet
files-drive-google-slides = Google-presentatie
files-drive-google-drawing = Google-tekening
files-drive-fetching = Ophalen…
files-drive-loading = De drive openen…
files-drive-empty = Deze map is leeg.
files-drive-unreachable = Kan { $drive } niet bereiken.
files-drive-try-again = Opnieuw proberen
files-drive-needs-permission = Katna heeft eenmalig je toestemming nodig om deze drive te tonen. Meld je opnieuw aan en sta Katna toe je bestanden te zien.
files-drive-allow = Toestaan
files-drive-allow-failed = Het aanmelden is niet afgerond, dus de drive blijft dicht.
files-drive-attach = Bijvoegen
files-drive-more = Meer
files-drive-download = Downloaden…
files-drive-open-web = Openen in { $drive }
files-drive-copy-link = Link kopiëren
files-drive-link-copied = Link gekopieerd
files-drive-share = Delen…
files-drive-rename = Hernoemen
files-drive-trash = Naar prullenbak verplaatsen
files-drive-trashed = “{ $name }” staat in de prullenbak van { $drive }
files-drive-renamed = Hernoemd naar “{ $name }”
files-drive-getting = { $name } ophalen uit { $drive }…
files-drive-get-failed = Kan { $name } niet ophalen: { $error }
files-drive-upload = Uploaden
files-drive-upload-files = Bestanden uploaden
files-drive-upload-folder = Map uploaden
files-drive-upload-failed = Kan { $name } niet uploaden: { $error }
files-drive-upload-needs = Om te uploaden heeft Katna eenmalig je toestemming nodig: druk op Toestaan in Instellingen › Standaardapps › Bestandenpagina.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” delen
files-share-add = Mensen toevoegen op naam of adres
files-share-not-address = “{ $text }” is geen e-mailadres
files-share-notify = Laat { $drive } hen ook een e-mail sturen
files-share-people = Mensen met toegang
files-share-general = Algemene toegang
files-share-loading = Lezen wie toegang heeft…
files-share-restricted = Beperkt
files-share-restricted-about = Alleen mensen met toegang kunnen het openen met de link
files-share-anyone = Iedereen met de link
files-share-anyone-can = { $role ->
    [editor] Iedereen met de link kan bewerken
    [commenter] Iedereen met de link kan reageren
   *[viewer] Iedereen met de link kan bekijken
}
files-share-anyone-about = { $role ->
    [editor] Iedereen op internet met de link kan bewerken
    [commenter] Iedereen op internet met de link kan reageren
   *[viewer] Iedereen op internet met de link kan bekijken
}
files-share-role-owner = Eigenaar
files-share-role-editor = Bewerker
files-share-role-commenter = Reageerder
files-share-role-viewer = Kijker
files-share-you = { $name } (jij)
files-share-domain = Iedereen bij { $domain }
files-share-inherited = Toegang via een map waarin het staat
files-share-remove = Toegang intrekken
files-share-copy-link = Link kopiëren
files-share-share = Delen
files-share-done = Klaar
files-share-close = Sluiten
files-share-sharing = Delen…
files-share-shared = { $count ->
    [one] Gedeeld met 1 persoon
   *[other] Gedeeld met { $count } personen
}
files-share-refused = { $drive } kon niet delen met { $addresses }
files-share-failed = Kan het delen niet wijzigen: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 item uploaden
   *[other] { $count } items uploaden
}
files-tray-done = { $count ->
    [one] 1 upload klaar
   *[other] { $count } uploads klaar
}
files-tray-some-failed = { $done } geüpload, { $failed } mislukt
files-tray-minutes-left = { $minutes ->
    [one] Nog ongeveer een minuut
   *[other] Nog ongeveer { $minutes } minuten
}
files-tray-seconds-left = Nog minder dan een minuut
files-tray-starting = Starten…
files-tray-cancel-all = Alles annuleren
files-tray-cancel = Annuleren
files-tray-fold = De lijst verbergen
files-tray-unfold = De lijst tonen
files-tray-close = Sluiten
files-tray-progress = { $place } · { $sent } van { $size }
files-tray-in = In { $place }
files-tray-cancelled = Geannuleerd
