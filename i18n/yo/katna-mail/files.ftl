# Katna Mail, Yoruba (Yorùbá).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Wá fáìlì

## Left side (and chips on a phone)

files-all = Gbogbo fáìlì
files-pictures = Àwòrán
files-pdfs = PDF
files-documents = Ìwé
files-sheets = Ìwé ìṣirò
files-slides = Síláìdì
files-other = Òmíràn
files-accounts = Àwọn àkáǹtì
files-drives = Àwọn drive
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Tí a pín pẹ̀lú mi
files-shown = Ohun tí a fi hàn
files-received = Tí a gbà
files-sent = Tí mo fi ránṣẹ́

## Over the files

files-count = { $count ->
   *[other] Fáìlì { $count } · { $size }
}
files-anyone = Ẹnikẹ́ni
files-from-person = Láti ọ̀dọ̀ { $name }
files-time-any = Nígbàkígbà
files-time-today = Òní
files-time-yesterday = Àná
files-time-this-week = Ọ̀sẹ̀ yìí
files-time-last-week = Ọ̀sẹ̀ tó kọjá
files-time-this-month = Oṣù yìí
files-time-last-month = Oṣù tó kọjá
files-time-between = { $first } – { $last }
files-time-hint = Tẹ ọjọ́ kan, tàbí fà á kọjá àwọn ọjọ́
files-time-summary = { $count ->
   *[other] { $days } · Fáìlì { $count }
}
files-time-clear = Pa á rẹ́
files-time-month-back = Oṣù tó kọjá
files-time-month-on = Oṣù tó ń bọ̀
files-time-wheel = Yí i láti gbé àwọn ọjọ́ wọ̀nyí, láìyí gígùn wọn padà
files-sort-newest = Tuntun jùlọ ní àkọ́kọ́
files-sort-oldest = Àtijọ́ jùlọ ní àkọ́kọ́
files-sort-largest = Títóbi jùlọ ní àkọ́kọ́
files-sort-name = Nípa orúkọ
files-grid = Káàdì
files-list = Àkójọ
files-this-week = Ọ̀sẹ̀ yìí
files-undated = Kò ní ọjọ́
files-me = Èmi
files-no-subject = (kò sí àkọlé)
files-loading = Ń kó àwọn fáìlì jọ láti inú lẹ́tà rẹ…
files-empty = Àwọn fáìlì láti inú lẹ́tà rẹ yóò hàn níbí.
files-none-match = Kò sí fáìlì tó bá a mu.
files-load-failed = Kíka àwọn fáìlì kùnà: { $error }

## A file's menu and buttons

files-open = Ṣí i
files-open-with = Ṣí i pẹ̀lú…
files-save = Fi pamọ́…
files-show-mail = Fi lẹ́tà náà hàn
files-mail-window = Ṣí lẹ́tà náà ní fèrèsé tuntun
files-forward = Fi fáìlì náà ránṣẹ́ síwájú
files-from-them = Fáìlì láti ọ̀dọ̀ { $name }
files-copy-name = Ṣẹ̀dà orúkọ fáìlì
files-name-copied = A ti ṣẹ̀dà orúkọ fáìlì
files-downloading = Ń gba lẹ́tà náà sílẹ̀…
files-download-failed = A kò lè gba lẹ́tà yìí sílẹ̀.

## A cloud drive in place of the mail files

files-drive-mine = Drive mi
files-drive-mine-onedrive = Fáìlì mi
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
       *[other] Fáìlì { $files }
    }
   *[other] Fódà { $folders } · { $files ->
       *[other] Fáìlì { $files }
    }
}
files-drive-folders = Àwọn fódà
files-drive-files = Àwọn fáìlì
files-drive-folder = Fódà
files-drive-meta = { $what } · A ṣàtúnṣe rẹ̀ { $date }
files-drive-as-link = { $what } · bí ìjápọ̀
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Ń gbà á…
files-drive-loading = Ń ṣí drive…
files-drive-empty = Fódà yìí ṣófo.
files-drive-unreachable = A kò lè dé { $drive }.
files-drive-try-again = Gbìyànjú lẹ́ẹ̀kan sí i
files-drive-needs-permission = Katna nílò àṣẹ rẹ lẹ́ẹ̀kan láti fi drive yìí hàn. Wọlé lẹ́ẹ̀kan sí i kí o sì gba Katna láàyè láti rí àwọn fáìlì rẹ.
files-drive-allow = Gbà láàyè
files-drive-allow-failed = Wíwọlé kò parí, nítorí náà drive náà wà ní títì.
files-drive-attach = So mọ́ ọn
files-drive-more = Síwájú sí i
files-drive-download = Gbà á sílẹ̀…
files-drive-open-web = Ṣí i nínú { $drive }
files-drive-copy-link = Ṣẹ̀dà ìjápọ̀
files-drive-link-copied = A ti ṣẹ̀dà ìjápọ̀
files-drive-share = Pín…
files-drive-rename = Yí orúkọ padà
files-drive-trash = Gbé lọ sí àpótí ìdọ̀tí
files-drive-trashed = “{ $name }” wà nínú àpótí ìdọ̀tí { $drive }
files-drive-renamed = A ti yí orúkọ rẹ̀ padà sí “{ $name }”
files-drive-getting = Ń gba { $name } láti { $drive }…
files-drive-get-failed = A kò lè gba { $name }: { $error }
files-drive-upload = Gbé sókè
files-drive-upload-files = Gbé fáìlì sókè
files-drive-upload-folder = Gbé fódà sókè
files-drive-upload-failed = A kò lè gbé { $name } sókè: { $error }
files-drive-upload-needs = Láti gbé sókè, Katna nílò àṣẹ rẹ lẹ́ẹ̀kan: tẹ Gbà láàyè nínú Ètò › Àwọn áàpù àtilẹ̀wá › Ojú-ìwé Fáìlì.

## The Share dialog of a drive file or folder

files-share-title = Pín “{ $name }”
files-share-add = Fi ènìyàn kún un nípa orúkọ tàbí àdírẹ́sì
files-share-not-address = “{ $text }” kì í ṣe àdírẹ́sì ímeèlì
files-share-notify = Jẹ́ kí { $drive } fi ímeèlì ránṣẹ́ sí wọn pẹ̀lú
files-share-people = Àwọn ènìyàn tí ó ní ààyè
files-share-general = Ààyè gbogbogbò
files-share-loading = Ń ka àwọn tí ó ní ààyè…
files-share-restricted = Ní ìhámọ́
files-share-restricted-about = Àwọn tí ó ní ààyè nìkan ló lè fi ìjápọ̀ náà ṣí i
files-share-anyone = Ẹnikẹ́ni tí ó ní ìjápọ̀ náà
files-share-anyone-can = { $role ->
    [editor] Ẹnikẹ́ni tí ó ní ìjápọ̀ náà lè ṣàtúnṣe
    [commenter] Ẹnikẹ́ni tí ó ní ìjápọ̀ náà lè sọ àsọyé
   *[viewer] Ẹnikẹ́ni tí ó ní ìjápọ̀ náà lè wò ó
}
files-share-anyone-about = { $role ->
    [editor] Ẹnikẹ́ni lórí ayélujára tí ó ní ìjápọ̀ náà lè ṣàtúnṣe
    [commenter] Ẹnikẹ́ni lórí ayélujára tí ó ní ìjápọ̀ náà lè sọ àsọyé
   *[viewer] Ẹnikẹ́ni lórí ayélujára tí ó ní ìjápọ̀ náà lè wò ó
}
files-share-role-owner = Olúwa
files-share-role-editor = Olóòtú
files-share-role-commenter = Asọ̀rọ̀sí
files-share-role-viewer = Olùwò
files-share-you = { $name } (ìwọ)
files-share-domain = Gbogbo ènìyàn ní { $domain }
files-share-inherited = Ààyè láti inú fódà tí ó wà nínú rẹ̀
files-share-remove = Yọ ààyè kúrò
files-share-copy-link = Ṣẹ̀dà ìjápọ̀
files-share-share = Pín
files-share-done = Ti parí
files-share-close = Pa á dé
files-share-sharing = Ń pín in…
files-share-shared = { $count ->
   *[other] A ti pín in pẹ̀lú ènìyàn { $count }
}
files-share-refused = { $drive } kò lè pín in pẹ̀lú { $addresses }
files-share-failed = A kò lè yí pínpín padà: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] Ń gbé ohun { $count } sókè
}
files-tray-done = { $count ->
   *[other] A ti gbé { $count } sókè
}
files-tray-some-failed = A gbé { $done } sókè, { $failed } kùnà
files-tray-minutes-left = { $minutes ->
   *[other] Ó ku bí ìṣẹ́jú { $minutes }
}
files-tray-seconds-left = Ó ku kéré sí ìṣẹ́jú kan
files-tray-starting = Ń bẹ̀rẹ̀…
files-tray-cancel-all = Fagilé gbogbo rẹ̀
files-tray-cancel = Fagilé
files-tray-fold = Fi àkójọ pamọ́
files-tray-unfold = Fi àkójọ hàn
files-tray-close = Pa á dé
files-tray-progress = { $place } · { $sent } nínú { $size }
files-tray-in = Nínú { $place }
files-tray-cancelled = A ti fagilé e
