# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Tafuta faili

## Left side (and chips on a phone)

files-all = Faili zote
files-pictures = Picha
files-pdfs = PDF
files-documents = Hati
files-sheets = Lahajedwali
files-slides = Slaidi
files-other = Nyingine
files-accounts = Akaunti
files-drives = Hifadhi za wingu
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Zilizoshirikiwa nami
files-shown = Zinazoonyeshwa
files-received = Zilizopokelewa
files-sent = Nilizotuma

## Over the files

files-count = { $count ->
    [one] faili { $count } · { $size }
   *[other] faili { $count } · { $size }
}
files-anyone = Yeyote
files-from-person = Kutoka kwa { $name }
files-time-any = Wakati wowote
files-time-today = Leo
files-time-yesterday = Jana
files-time-this-week = Wiki hii
files-time-last-week = Wiki iliyopita
files-time-this-month = Mwezi huu
files-time-last-month = Mwezi uliopita
files-time-between = { $first } – { $last }
files-time-hint = Bofya siku, au buruta juu ya siku kadhaa
files-time-summary = { $count ->
    [one] { $days } · faili { $count }
   *[other] { $days } · faili { $count }
}
files-time-clear = Futa
files-time-month-back = Mwezi uliotangulia
files-time-month-on = Mwezi ujao
files-time-wheel = Sogeza gurudumu ili kuhamisha tarehe hizi, urefu ukibaki uleule
files-sort-newest = Mpya zaidi kwanza
files-sort-oldest = Za zamani zaidi kwanza
files-sort-largest = Kubwa zaidi kwanza
files-sort-name = Kwa jina
files-grid = Kadi
files-list = Orodha
files-this-week = Wiki hii
files-undated = Bila tarehe
files-me = Mimi
files-no-subject = (hakuna mada)
files-loading = Inakusanya faili kutoka kwenye barua zako…
files-empty = Faili kutoka kwenye barua zako huonekana hapa.
files-none-match = Hakuna faili zinazolingana.
files-load-failed = Imeshindwa kusoma faili: { $error }

## A file's menu and buttons

files-open = Fungua
files-open-with = Fungua kwa…
files-save = Hifadhi…
files-show-mail = Onyesha barua
files-mail-window = Fungua barua katika dirisha jipya
files-forward = Sambaza faili
files-from-them = Faili kutoka kwa { $name }
files-copy-name = Nakili jina la faili
files-name-copied = Jina la faili limenakiliwa
files-downloading = Inapakua barua…
files-download-failed = Imeshindwa kupakua barua hii.

## A cloud drive in place of the mail files

files-drive-mine = Hifadhi Yangu
files-drive-mine-onedrive = Faili zangu
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] faili 1
       *[other] faili { $files }
    }
    [one] folda 1 · { $files ->
        [one] faili 1
       *[other] faili { $files }
    }
   *[other] folda { $folders } · { $files ->
        [one] faili 1
       *[other] faili { $files }
    }
}
files-drive-folders = Folda
files-drive-files = Faili
files-drive-folder = Folda
files-drive-meta = { $what } · Ilihaririwa { $date }
files-drive-as-link = { $what } · kama kiungo
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Inaipata…
files-drive-loading = Inafungua hifadhi…
files-drive-empty = Folda hii ni tupu.
files-drive-unreachable = Haiwezi kufikia { $drive }.
files-drive-try-again = Jaribu tena
files-drive-needs-permission = Katna inahitaji ruhusa yako mara moja ili kuonyesha hifadhi hii. Ingia tena na uiruhusu Katna ione faili zako.
files-drive-allow = Ruhusu
files-drive-allow-failed = Kuingia hakukukamilika, kwa hivyo hifadhi inabaki imefungwa.
files-drive-attach = Ambatisha
files-drive-more = Zaidi
files-drive-download = Pakua…
files-drive-open-web = Fungua katika { $drive }
files-drive-copy-link = Nakili kiungo
files-drive-link-copied = Kiungo kimenakiliwa
files-drive-share = Shiriki…
files-drive-rename = Badilisha jina
files-drive-trash = Hamisha kwenye tupio
files-drive-trashed = “{ $name }” iko kwenye tupio la { $drive }
files-drive-renamed = Jina limebadilishwa kuwa “{ $name }”
files-drive-getting = Inapata { $name } kutoka { $drive }…
files-drive-get-failed = Imeshindwa kupata { $name }: { $error }
files-drive-upload = Pakia
files-drive-upload-files = Pakia faili
files-drive-upload-folder = Pakia folda
files-drive-upload-failed = Imeshindwa kupakia { $name }: { $error }
files-drive-upload-needs = Ili kupakia, Katna inahitaji ruhusa yako mara moja: bonyeza Ruhusu katika Mipangilio › Programu chaguomsingi › Ukurasa wa Faili.

## The Share dialog of a drive file or folder

files-share-title = Shiriki “{ $name }”
files-share-add = Ongeza watu kwa jina au anwani
files-share-not-address = “{ $text }” si anwani ya barua pepe
files-share-notify = Ruhusu { $drive } iwatumie barua pepe pia
files-share-people = Watu wenye ufikiaji
files-share-general = Ufikiaji wa jumla
files-share-loading = Inasoma walio na ufikiaji…
files-share-restricted = Imezuiwa
files-share-restricted-about = Watu wenye ufikiaji pekee wanaweza kuifungua kwa kiungo
files-share-anyone = Yeyote mwenye kiungo
files-share-anyone-can = { $role ->
    [editor] Yeyote mwenye kiungo anaweza kuhariri
    [commenter] Yeyote mwenye kiungo anaweza kutoa maoni
   *[viewer] Yeyote mwenye kiungo anaweza kutazama
}
files-share-anyone-about = { $role ->
    [editor] Yeyote kwenye intaneti mwenye kiungo anaweza kuhariri
    [commenter] Yeyote kwenye intaneti mwenye kiungo anaweza kutoa maoni
   *[viewer] Yeyote kwenye intaneti mwenye kiungo anaweza kutazama
}
files-share-role-owner = Mmiliki
files-share-role-editor = Mhariri
files-share-role-commenter = Mtoa maoni
files-share-role-viewer = Mtazamaji
files-share-you = { $name } (wewe)
files-share-domain = Kila mtu katika { $domain }
files-share-inherited = Ufikiaji kutoka kwenye folda iliyomo
files-share-remove = Ondoa ufikiaji
files-share-copy-link = Nakili kiungo
files-share-share = Shiriki
files-share-done = Imekamilika
files-share-close = Funga
files-share-sharing = Inashiriki…
files-share-shared = { $count ->
    [one] Imeshirikiwa na mtu 1
   *[other] Imeshirikiwa na watu { $count }
}
files-share-refused = { $drive } haikuweza kushiriki na { $addresses }
files-share-failed = Imeshindwa kubadilisha ushiriki: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Inapakia kipengee 1
   *[other] Inapakia vipengee { $count }
}
files-tray-done = { $count ->
    [one] Upakiaji 1 umekamilika
   *[other] Upakiaji { $count } umekamilika
}
files-tray-some-failed = { $done } zimepakiwa, { $failed } zimeshindwa
files-tray-minutes-left = { $minutes ->
    [one] Imebaki takriban dakika moja
   *[other] Zimebaki takriban dakika { $minutes }
}
files-tray-seconds-left = Imebaki chini ya dakika moja
files-tray-starting = Inaanza…
files-tray-cancel-all = Ghairi zote
files-tray-cancel = Ghairi
files-tray-fold = Ficha orodha
files-tray-unfold = Onyesha orodha
files-tray-close = Funga
files-tray-progress = { $place } · { $sent } kati ya { $size }
files-tray-in = Katika { $place }
files-tray-cancelled = Imeghairiwa
