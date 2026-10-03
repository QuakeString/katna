# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Soek lêers

## Left side (and chips on a phone)

files-all = Alle lêers
files-pictures = Prente
files-pdfs = PDF's
files-documents = Dokumente
files-sheets = Sigblaaie
files-slides = Skyfies
files-other = Ander
files-accounts = Rekeninge
files-drives = Skywe
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Met my gedeel
files-shown = Gewys
files-received = Ontvang
files-sent = Deur my gestuur

## Over the files

files-count = { $count ->
    [one] { $count } lêer · { $size }
   *[other] { $count } lêers · { $size }
}
files-anyone = Enigiemand
files-from-person = Van { $name }
files-time-any = Enige tyd
files-time-today = Vandag
files-time-yesterday = Gister
files-time-this-week = Hierdie week
files-time-last-week = Verlede week
files-time-this-month = Hierdie maand
files-time-last-month = Verlede maand
files-time-between = { $first } – { $last }
files-time-hint = Klik op 'n dag, of sleep oor dae
files-time-summary = { $count ->
    [one] { $days } · { $count } lêer
   *[other] { $days } · { $count } lêers
}
files-time-clear = Maak skoon
files-time-month-back = Vorige maand
files-time-month-on = Volgende maand
files-time-wheel = Rol om hierdie datums te skuif, met dieselfde lengte
files-sort-newest = Nuutste eerste
files-sort-oldest = Oudste eerste
files-sort-largest = Grootste eerste
files-sort-name = Volgens naam
files-grid = Kaarte
files-list = Lys
files-this-week = Hierdie week
files-undated = Geen datum
files-me = Ek
files-no-subject = (geen onderwerp)
files-loading = Versamel tans lêers uit jou e-pos…
files-empty = Lêers uit jou e-pos verskyn hier.
files-none-match = Geen lêers pas nie.
files-load-failed = Kon nie die lêers lees nie: { $error }

## A file's menu and buttons

files-open = Maak oop
files-open-with = Maak oop met…
files-save = Stoor…
files-show-mail = Wys die e-pos
files-mail-window = Maak die e-pos in 'n nuwe venster oop
files-forward = Stuur die lêer aan
files-from-them = Lêers van { $name }
files-copy-name = Kopieer lêernaam
files-name-copied = Lêernaam gekopieer
files-downloading = Laai tans die e-pos af…
files-download-failed = Kon nie hierdie e-pos aflaai nie.

## A cloud drive in place of the mail files

files-drive-mine = My Drive
files-drive-mine-onedrive = My lêers
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 lêer
       *[other] { $files } lêers
    }
    [one] 1 vouer · { $files ->
        [one] 1 lêer
       *[other] { $files } lêers
    }
   *[other] { $folders } vouers · { $files ->
        [one] 1 lêer
       *[other] { $files } lêers
    }
}
files-drive-folders = Vouers
files-drive-files = Lêers
files-drive-folder = Vouer
files-drive-meta = { $what } · Gewysig { $date }
files-drive-as-link = { $what } · as 'n skakel
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Haal dit tans…
files-drive-loading = Maak tans die skyf oop…
files-drive-empty = Hierdie vouer is leeg.
files-drive-unreachable = Kan { $drive } nie bereik nie.
files-drive-try-again = Probeer weer
files-drive-needs-permission = Katna het een keer jou toestemming nodig om hierdie skyf te wys. Meld weer aan en laat Katna toe om jou lêers te sien.
files-drive-allow = Laat toe
files-drive-allow-failed = Die aanmelding is nie voltooi nie, dus bly die skyf toe.
files-drive-attach = Heg aan
files-drive-more = Meer
files-drive-download = Laai af…
files-drive-open-web = Maak oop in { $drive }
files-drive-copy-link = Kopieer skakel
files-drive-link-copied = Skakel gekopieer
files-drive-share = Deel…
files-drive-rename = Hernoem
files-drive-trash = Skuif na asblik
files-drive-trashed = “{ $name }” is in die { $drive }-asblik
files-drive-renamed = Hernoem na “{ $name }”
files-drive-getting = Haal tans { $name } van { $drive } af…
files-drive-get-failed = Kon nie { $name } kry nie: { $error }
files-drive-upload = Laai op
files-drive-upload-files = Laai lêers op
files-drive-upload-folder = Laai vouer op
files-drive-upload-failed = Kon nie { $name } oplaai nie: { $error }
files-drive-upload-needs = Om op te laai, het Katna een keer jou toestemming nodig: druk Laat toe in Instellings › Verstekprogramme › Lêers-bladsy.

## The Share dialog of a drive file or folder

files-share-title = Deel “{ $name }”
files-share-add = Voeg mense by volgens naam of adres
files-share-not-address = “{ $text }” is nie 'n e-posadres nie
files-share-notify = Laat { $drive } hulle ook e-pos
files-share-people = Mense met toegang
files-share-general = Algemene toegang
files-share-loading = Lees tans wie toegang het…
files-share-restricted = Beperk
files-share-restricted-about = Net mense met toegang kan dit met die skakel oopmaak
files-share-anyone = Enigiemand met die skakel
files-share-anyone-can = { $role ->
    [editor] Enigiemand met die skakel kan wysig
    [commenter] Enigiemand met die skakel kan kommentaar lewer
   *[viewer] Enigiemand met die skakel kan kyk
}
files-share-anyone-about = { $role ->
    [editor] Enigiemand op die internet met die skakel kan wysig
    [commenter] Enigiemand op die internet met die skakel kan kommentaar lewer
   *[viewer] Enigiemand op die internet met die skakel kan kyk
}
files-share-role-owner = Eienaar
files-share-role-editor = Redigeerder
files-share-role-commenter = Kommentator
files-share-role-viewer = Kyker
files-share-you = { $name } (jy)
files-share-domain = Almal by { $domain }
files-share-inherited = Toegang van 'n vouer waarin dit is
files-share-remove = Verwyder toegang
files-share-copy-link = Kopieer skakel
files-share-share = Deel
files-share-done = Klaar
files-share-close = Maak toe
files-share-sharing = Deel tans…
files-share-shared = { $count ->
    [one] Met 1 persoon gedeel
   *[other] Met { $count } mense gedeel
}
files-share-refused = { $drive } kon nie met { $addresses } deel nie
files-share-failed = Kon nie deling verander nie: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Laai 1 item op
   *[other] Laai { $count } items op
}
files-tray-done = { $count ->
    [one] 1 oplaai klaar
   *[other] { $count } oplaaie klaar
}
files-tray-some-failed = { $done } opgelaai, { $failed } het misluk
files-tray-minutes-left = { $minutes ->
    [one] Omtrent 'n minuut oor
   *[other] Omtrent { $minutes } minute oor
}
files-tray-seconds-left = Minder as 'n minuut oor
files-tray-starting = Begin tans…
files-tray-cancel-all = Kanselleer alles
files-tray-cancel = Kanselleer
files-tray-fold = Versteek die lys
files-tray-unfold = Wys die lys
files-tray-close = Maak toe
files-tray-progress = { $place } · { $sent } van { $size }
files-tray-in = In { $place }
files-tray-cancelled = Gekanselleer
