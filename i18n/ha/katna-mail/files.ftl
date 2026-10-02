# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Bincika fayiloli

## Left side (and chips on a phone)

files-all = Duk fayiloli
files-pictures = Hotuna
files-pdfs = PDF
files-documents = Takardu
files-sheets = Maƙunsar bayanai
files-slides = Silaidi
files-other = Wasu
files-accounts = Asusu
files-drives = Drive
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = An raba da ni
files-shown = Ana nunawa
files-received = An karɓa
files-sent = Ni na aika

## Over the files

files-count = { $count ->
    [one] fayil { $count } · { $size }
   *[other] fayiloli { $count } · { $size }
}
files-anyone = Kowa
files-from-person = Daga { $name }
files-time-any = Kowane lokaci
files-time-today = Yau
files-time-yesterday = Jiya
files-time-this-week = Wannan mako
files-time-last-week = Makon da ya wuce
files-time-this-month = Wannan wata
files-time-last-month = Watan da ya wuce
files-time-between = { $first } – { $last }
files-time-hint = Danna rana, ko ja a kan kwanaki
files-time-summary = { $count ->
    [one] { $days } · fayil { $count }
   *[other] { $days } · fayiloli { $count }
}
files-time-clear = Share zaɓi
files-time-month-back = Watan baya
files-time-month-on = Wata mai zuwa
files-time-wheel = Juya don matsar da waɗannan kwanaki, tsawonsu yana nan
files-sort-newest = Sababbi farko
files-sort-oldest = Tsofaffi farko
files-sort-largest = Mafi girma farko
files-sort-name = Bisa suna
files-grid = Katuna
files-list = Jeri
files-this-week = Wannan mako
files-undated = Babu kwanan wata
files-me = Ni
files-no-subject = (babu jigo)
files-loading = Ana tattara fayiloli daga wasiƙunku…
files-empty = Fayiloli daga wasiƙunku suna bayyana a nan.
files-none-match = Babu fayilolin da suka dace.
files-load-failed = Karanta fayilolin ya kasa: { $error }

## A file's menu and buttons

files-open = Buɗe
files-open-with = Buɗe da…
files-save = Ajiye…
files-show-mail = Nuna wasiƙar
files-mail-window = Buɗe wasiƙar a sabuwar taga
files-forward = Tura fayil ɗin
files-from-them = Fayiloli daga { $name }
files-copy-name = Kwafa sunan fayil
files-name-copied = An kwafa sunan fayil
files-downloading = Ana sauke wasiƙar…
files-download-failed = Ba a iya sauke wannan wasiƙa ba.

## A cloud drive in place of the mail files

files-drive-mine = Drive ɗina
files-drive-mine-onedrive = Fayiloli na
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] fayil 1
       *[other] fayiloli { $files }
    }
    [one] folda 1 · { $files ->
        [one] fayil 1
       *[other] fayiloli { $files }
    }
   *[other] folda { $folders } · { $files ->
        [one] fayil 1
       *[other] fayiloli { $files }
    }
}
files-drive-folders = Folda
files-drive-files = Fayiloli
files-drive-folder = Folda
files-drive-meta = { $what } · An gyara { $date }
files-drive-as-link = { $what } · a matsayin mahaɗi
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Ana samo shi…
files-drive-loading = Ana buɗe Drive ɗin…
files-drive-empty = Wannan folda babu komai.
files-drive-unreachable = Ba a iya isa ga { $drive } ba.
files-drive-try-again = Sake gwadawa
files-drive-needs-permission = Katna tana buƙatar izininku sau ɗaya don nuna wannan Drive. Sake shiga kuma ku bar Katna ta ga fayilolinku.
files-drive-allow = Ba da izini
files-drive-allow-failed = Shigar ba ta kammala ba, don haka Drive ɗin yana zama a rufe.
files-drive-attach = Haɗa
files-drive-more = Ƙari
files-drive-download = Sauke…
files-drive-open-web = Buɗe a { $drive }
files-drive-copy-link = Kwafa mahaɗi
files-drive-link-copied = An kwafa mahaɗi
files-drive-share = Raba…
files-drive-rename = Sake suna
files-drive-trash = Matsar zuwa kwandon shara
files-drive-trashed = “{ $name }” yana cikin kwandon shara na { $drive }
files-drive-renamed = An sake suna zuwa “{ $name }”
files-drive-getting = Ana samo { $name } daga { $drive }…
files-drive-get-failed = Ba a iya samo { $name } ba: { $error }
files-drive-upload = Ɗora
files-drive-upload-files = Ɗora fayiloli
files-drive-upload-folder = Ɗora folda
files-drive-upload-failed = Ba a iya ɗora { $name } ba: { $error }
files-drive-upload-needs = Don ɗorawa, Katna tana buƙatar izininku sau ɗaya: danna Ba da izini a Saituna › Manhajojin asali › Shafin Fayiloli.

## The Share dialog of a drive file or folder

files-share-title = Raba “{ $name }”
files-share-add = Ƙara mutane da suna ko adireshi
files-share-not-address = “{ $text }” ba adireshin imel ba ne
files-share-notify = Bari { $drive } ta aika musu imel kuma
files-share-people = Mutanen da ke da dama
files-share-general = Dama gabaɗaya
files-share-loading = Ana karanta waɗanda ke da dama…
files-share-restricted = An taƙaita
files-share-restricted-about = Mutanen da ke da dama kaɗai za su iya buɗe shi da mahaɗin
files-share-anyone = Duk wanda ke da mahaɗin
files-share-anyone-can = { $role ->
    [editor] Duk wanda ke da mahaɗin zai iya gyarawa
    [commenter] Duk wanda ke da mahaɗin zai iya yin sharhi
   *[viewer] Duk wanda ke da mahaɗin zai iya duba
}
files-share-anyone-about = { $role ->
    [editor] Duk wanda ke intanet da mahaɗin zai iya gyarawa
    [commenter] Duk wanda ke intanet da mahaɗin zai iya yin sharhi
   *[viewer] Duk wanda ke intanet da mahaɗin zai iya duba
}
files-share-role-owner = Mai shi
files-share-role-editor = Mai gyarawa
files-share-role-commenter = Mai sharhi
files-share-role-viewer = Mai dubawa
files-share-you = { $name } (ku)
files-share-domain = Kowa a { $domain }
files-share-inherited = Dama daga folda da yake ciki
files-share-remove = Cire dama
files-share-copy-link = Kwafa mahaɗi
files-share-share = Raba
files-share-done = An gama
files-share-sharing = Ana rabawa…
files-share-shared = { $count ->
    [one] An raba da mutum 1
   *[other] An raba da mutane { $count }
}
files-share-refused = { $drive } ba ta iya raba da { $addresses } ba
files-share-failed = Ba a iya canza rabawa ba: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Ana ɗora abu 1
   *[other] Ana ɗora abubuwa { $count }
}
files-tray-done = { $count ->
    [one] An gama ɗora 1
   *[other] An gama ɗora { $count }
}
files-tray-some-failed = An ɗora { $done }, { $failed } sun kasa
files-tray-minutes-left = { $minutes ->
    [one] Saura kusan minti ɗaya
   *[other] Saura kusan mintuna { $minutes }
}
files-tray-seconds-left = Saura ƙasa da minti ɗaya
files-tray-starting = Ana farawa…
files-tray-cancel-all = Soke duka
files-tray-cancel = Soke
files-tray-fold = Ɓoye jerin
files-tray-unfold = Nuna jerin
files-tray-close = Rufe
files-tray-progress = { $place } · { $sent } daga { $size }
files-tray-in = A cikin { $place }
files-tray-cancelled = An soke
