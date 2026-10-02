# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Sesha amafayela

## Left side (and chips on a phone)

files-all = Wonke amafayela
files-pictures = Izithombe
files-pdfs = Ama-PDF
files-documents = Amadokhumenti
files-sheets = Amaspredishithi
files-slides = Amaslayidi
files-other = Okunye
files-accounts = Ama-akhawunti
files-drives = Ama-drive
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Okwabelwe nami
files-shown = Okuboniswayo
files-received = Okutholiwe
files-sent = Engikuthumele

## Over the files

files-count = { $count ->
    [one] ifayela elingu-{ $count } · { $size }
   *[other] amafayela angu-{ $count } · { $size }
}
files-anyone = Noma ubani
files-from-person = Kusuka ku-{ $name }
files-time-any = Noma inini
files-time-today = Namuhla
files-time-yesterday = Izolo
files-time-this-week = Leli viki
files-time-last-week = Iviki eledlule
files-time-this-month = Le nyanga
files-time-last-month = Inyanga edlule
files-time-between = { $first } – { $last }
files-time-hint = Chofoza usuku, noma uhudule phezu kwezinsuku
files-time-summary = { $count ->
    [one] { $days } · ifayela elingu-{ $count }
   *[other] { $days } · amafayela angu-{ $count }
}
files-time-clear = Sula
files-time-month-back = Inyanga edlule
files-time-month-on = Inyanga elandelayo
files-time-wheel = Skrola ukuze uhambise lezi zinsuku, ubude bazo bungashintshi
files-sort-newest = Okusha kuqala
files-sort-oldest = Okudala kuqala
files-sort-largest = Okukhulu kuqala
files-sort-name = Ngegama
files-grid = Amakhadi
files-list = Uhlu
files-this-week = Leli viki
files-undated = Alikho usuku
files-me = Mina
files-no-subject = (asikho isihloko)
files-loading = Kuqoqwa amafayela emeyilini yakho…
files-empty = Amafayela asemeyilini yakho avela lapha.
files-none-match = Awekho amafayela afanayo.
files-load-failed = Ukufunda amafayela kuhlulekile: { $error }

## A file's menu and buttons

files-open = Vula
files-open-with = Vula nge-…
files-save = Londoloza…
files-show-mail = Bonisa imeyili
files-mail-window = Vula imeyili ewindini elisha
files-forward = Dlulisela ifayela
files-from-them = Amafayela avela ku-{ $name }
files-copy-name = Kopisha igama lefayela
files-name-copied = Igama lefayela likopishiwe
files-downloading = Kulandwa imeyili…
files-download-failed = Akukwazekanga ukulanda le meyili.

## A cloud drive in place of the mail files

files-drive-mine = I-Drive yami
files-drive-mine-onedrive = Amafayela ami
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] ifayela elingu-{ $files }
       *[other] amafayela angu-{ $files }
    }
    [one] ifolda elingu-{ $folders } · { $files ->
        [one] ifayela elingu-{ $files }
       *[other] amafayela angu-{ $files }
    }
   *[other] amafolda angu-{ $folders } · { $files ->
        [one] ifayela elingu-{ $files }
       *[other] amafayela angu-{ $files }
    }
}
files-drive-folders = Amafolda
files-drive-files = Amafayela
files-drive-folder = Ifolda
files-drive-meta = { $what } · Kuhlelwe { $date }
files-drive-as-link = { $what } · njengesixhumanisi
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Kuyalandwa…
files-drive-loading = Kuvulwa i-drive…
files-drive-empty = Le folda ayinalutho.
files-drive-unreachable = Ayifinyeleleki i-{ $drive }.
files-drive-try-again = Zama futhi
files-drive-needs-permission = I-Katna idinga imvume yakho kanye ukuze ibonise le drive. Ngena futhi bese uvumela i-Katna ibone amafayela akho.
files-drive-allow = Vumela
files-drive-allow-failed = Ukungena akuqedanga, ngakho i-drive ihlala ivaliwe.
files-drive-attach = Namathisela
files-drive-more = Okwengeziwe
files-drive-download = Landa…
files-drive-open-web = Vula ku-{ $drive }
files-drive-copy-link = Kopisha isixhumanisi
files-drive-link-copied = Isixhumanisi sikopishiwe
files-drive-share = Yabelana…
files-drive-rename = Qamba kabusha
files-drive-trash = Hambisa kudoti
files-drive-trashed = “{ $name }” likudoti we-{ $drive }
files-drive-renamed = Kuqanjwe kabusha kwaba ngu-“{ $name }”
files-drive-getting = Kulandwa i-{ $name } ku-{ $drive }…
files-drive-get-failed = Akukwazekanga ukuthola i-{ $name }: { $error }
files-drive-upload = Layisha
files-drive-upload-files = Layisha amafayela
files-drive-upload-folder = Layisha ifolda
files-drive-upload-failed = Akukwazekanga ukulayisha i-{ $name }: { $error }
files-drive-upload-needs = Ukuze ulayishe, i-Katna idinga imvume yakho kanye: cindezela u-Vumela ku-Izilungiselelo › Ama-app azenzakalelayo › Ikhasi lamafayela.

## The Share dialog of a drive file or folder

files-share-title = Yabelana nge-“{ $name }”
files-share-add = Engeza abantu ngegama noma ngekheli
files-share-not-address = “{ $text }” akulona ikheli le-imeyili
files-share-notify = Vumela i-{ $drive } ibathumelele ne-imeyili
files-share-people = Abantu abanokufinyelela
files-share-general = Ukufinyelela okujwayelekile
files-share-loading = Kufundwa ukuthi ubani onokufinyelela…
files-share-restricted = Kunqunyelwe
files-share-restricted-about = Abantu abanokufinyelela kuphela abangakuvula ngesixhumanisi
files-share-anyone = Noma ubani onesixhumanisi
files-share-anyone-can = { $role ->
    [editor] Noma ubani onesixhumanisi angahlela
    [commenter] Noma ubani onesixhumanisi angaphawula
   *[viewer] Noma ubani onesixhumanisi angabuka
}
files-share-anyone-about = { $role ->
    [editor] Noma ubani ku-inthanethi onesixhumanisi angahlela
    [commenter] Noma ubani ku-inthanethi onesixhumanisi angaphawula
   *[viewer] Noma ubani ku-inthanethi onesixhumanisi angabuka
}
files-share-role-owner = Umnikazi
files-share-role-editor = Umhleli
files-share-role-commenter = Ophawulayo
files-share-role-viewer = Obukayo
files-share-you = { $name } (wena)
files-share-domain = Wonke umuntu ku-{ $domain }
files-share-inherited = Ukufinyelela okuvela kufolda ekuyo
files-share-remove = Susa ukufinyelela
files-share-copy-link = Kopisha isixhumanisi
files-share-share = Yabelana
files-share-done = Kwenziwe
files-share-close = Vala
files-share-sharing = Kwabelwana…
files-share-shared = { $count ->
    [one] Kwabelwe nomuntu o-{ $count }
   *[other] Kwabelwe nabantu abangu-{ $count }
}
files-share-refused = I-{ $drive } ayikwazanga ukwabelana no-{ $addresses }
files-share-failed = Akukwazekanga ukushintsha ukwabelana: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Kulayishwa into engu-{ $count }
   *[other] Kulayishwa izinto ezingu-{ $count }
}
files-tray-done = { $count ->
    [one] Ukulayisha okungu-{ $count } kuqediwe
   *[other] Ukulayisha okungu-{ $count } kuqediwe
}
files-tray-some-failed = { $done } kulayishiwe, { $failed } kuhlulekile
files-tray-minutes-left = { $minutes ->
    [one] Kusele cishe umzuzu o-{ $minutes }
   *[other] Kusele cishe imizuzu engu-{ $minutes }
}
files-tray-seconds-left = Kusele ngaphansi komzuzu
files-tray-starting = Kuyaqala…
files-tray-cancel-all = Khansela konke
files-tray-cancel = Khansela
files-tray-fold = Fihla uhlu
files-tray-unfold = Bonisa uhlu
files-tray-close = Vala
files-tray-progress = { $place } · { $sent } kokungu-{ $size }
files-tray-in = Ku-{ $place }
files-tray-cancelled = Kukhanseliwe
