# Katna Mail, Igbo (Igbo).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Chọọ faịlụ

## Left side (and chips on a phone)

files-all = Faịlụ niile
files-pictures = Foto
files-pdfs = PDF
files-documents = Akwụkwọ
files-sheets = Mpempe mgbakọ
files-slides = Slaịdị
files-other = Ndị ọzọ
files-accounts = Akaụntụ
files-drives = Draịv
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = E kekọrịtara m
files-shown = Egosiri
files-received = Anatara
files-sent = M zitere

## Over the files

files-count = { $count ->
   *[other] faịlụ { $count } · { $size }
}
files-anyone = Onye ọ bụla
files-from-person = Site n'aka { $name }
files-time-any = Oge ọ bụla
files-time-today = Taa
files-time-yesterday = Ụnyaahụ
files-time-this-week = Izu a
files-time-last-week = Izu gara aga
files-time-this-month = Ọnwa a
files-time-last-month = Ọnwa gara aga
files-time-between = { $first } – { $last }
files-time-hint = Pịa otu ụbọchị, ma ọ bụ dọrọ gafee ụbọchị
files-time-summary = { $count ->
   *[other] { $days } · faịlụ { $count }
}
files-time-clear = Kpochapụ
files-time-month-back = Ọnwa gara aga
files-time-month-on = Ọnwa na-abịa
files-time-wheel = Pịgharịa ka ị kwaga ụbọchị ndị a, na-edobe ogologo ha
files-sort-newest = Nke kacha ọhụrụ na mbụ
files-sort-oldest = Nke kacha ochie na mbụ
files-sort-largest = Nke kacha ibu na mbụ
files-sort-name = Site n'aha
files-grid = Kaadị
files-list = Ndepụta
files-this-week = Izu a
files-undated = Enweghị ụbọchị
files-me = Mụ
files-no-subject = (enweghị isiokwu)
files-loading = Na-achịkọta faịlụ site n'ozi gị…
files-empty = Faịlụ si n'ozi gị na-apụta ebe a.
files-none-match = Ọ dịghị faịlụ dabara.
files-load-failed = Ịgụ faịlụ ahụ dara: { $error }

## A file's menu and buttons

files-open = Mepee
files-open-with = Mepee site na…
files-save = Chekwaa…
files-show-mail = Gosi ozi ahụ
files-mail-window = Mepee ozi ahụ na windo ọhụrụ
files-forward = Zigaa faịlụ ahụ
files-from-them = Faịlụ sitere n'aka { $name }
files-copy-name = Detuo aha faịlụ
files-name-copied = Edepụtala aha faịlụ
files-downloading = Na-ebudata ozi ahụ…
files-download-failed = Enweghị ike ibudata ozi a.

## A cloud drive in place of the mail files

files-drive-mine = Draịv m
files-drive-mine-onedrive = Faịlụ m
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
       *[other] faịlụ { $files }
    }
   *[other] folda { $folders } · { $files ->
       *[other] faịlụ { $files }
    }
}
files-drive-folders = Folda
files-drive-files = Faịlụ
files-drive-folder = Folda
files-drive-meta = { $what } · Edeziri { $date }
files-drive-as-link = { $what } · dị ka njikọ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Na-enweta ya…
files-drive-loading = Na-emepe draịv…
files-drive-empty = Folda a tọgbọrọ chakoo.
files-drive-unreachable = Enweghị ike iru { $drive }.
files-drive-try-again = Nwaa ọzọ
files-drive-needs-permission = Katna chọrọ ikike gị otu ugboro iji gosi draịv a. Banye ọzọ ma kwe ka Katna hụ faịlụ gị.
files-drive-allow = Kwe
files-drive-allow-failed = Ịbanye ahụ emezughị, ya mere draịv ahụ ka mechiri emechi.
files-drive-attach = Gbakwunye
files-drive-more = Ọzọ
files-drive-download = Budata…
files-drive-open-web = Mepee na { $drive }
files-drive-copy-link = Detuo njikọ
files-drive-link-copied = Edepụtala njikọ
files-drive-share = Kekọrịta…
files-drive-rename = Gbanwee aha
files-drive-trash = Buga na Ihe mkpofu
files-drive-trashed = “{ $name }” nọ n'Ihe mkpofu { $drive }
files-drive-renamed = Agbanwere aha ka ọ bụrụ “{ $name }”
files-drive-getting = Na-enweta { $name } site na { $drive }…
files-drive-get-failed = Enweghị ike inweta { $name }: { $error }
files-drive-upload = Bugote
files-drive-upload-files = Bugote faịlụ
files-drive-upload-folder = Bugote folda
files-drive-upload-failed = Enweghị ike ibugote { $name }: { $error }
files-drive-upload-needs = Iji bugote, Katna chọrọ ikike gị otu ugboro: pịa Kwe na Ntọala › Ngwa ndabara › Peeji Faịlụ.

## The Share dialog of a drive file or folder

files-share-title = Kekọrịta “{ $name }”
files-share-add = Tinye mmadụ site n'aha ma ọ bụ adreesị
files-share-not-address = “{ $text }” abụghị adreesị email
files-share-notify = Kwe ka { $drive } zigakwara ha email
files-share-people = Ndị nwere ohere
files-share-general = Ohere izugbe
files-share-loading = Na-agụ ndị nwere ohere…
files-share-restricted = Amachibidoro
files-share-restricted-about = Naanị ndị nwere ohere nwere ike imeghe ya site na njikọ ahụ
files-share-anyone = Onye ọ bụla nwere njikọ ahụ
files-share-anyone-can = { $role ->
    [editor] Onye ọ bụla nwere njikọ ahụ nwere ike idezi
    [commenter] Onye ọ bụla nwere njikọ ahụ nwere ike ikwu okwu
   *[viewer] Onye ọ bụla nwere njikọ ahụ nwere ike ilele
}
files-share-anyone-about = { $role ->
    [editor] Onye ọ bụla nọ n'ịntanetị nwere njikọ ahụ nwere ike idezi
    [commenter] Onye ọ bụla nọ n'ịntanetị nwere njikọ ahụ nwere ike ikwu okwu
   *[viewer] Onye ọ bụla nọ n'ịntanetị nwere njikọ ahụ nwere ike ilele
}
files-share-role-owner = Onye nwe ya
files-share-role-editor = Onye ndezi
files-share-role-commenter = Onye na-ekwu okwu
files-share-role-viewer = Onye na-ele
files-share-you = { $name } (gị)
files-share-domain = Onye ọ bụla nọ na { $domain }
files-share-inherited = Ohere sitere na folda ọ nọ n'ime ya
files-share-remove = Wepụ ohere
files-share-copy-link = Detuo njikọ
files-share-share = Kekọrịta
files-share-done = O mechara
files-share-close = Mechie
files-share-sharing = Na-ekekọrịta…
files-share-shared = { $count ->
   *[other] E kekọrịtara ya na mmadụ { $count }
}
files-share-refused = { $drive } enweghị ike ịkekọrịta ya na { $addresses }
files-share-failed = Enweghị ike ịgbanwe ịkekọrịta: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] Na-ebugote ihe { $count }
}
files-tray-done = { $count ->
   *[other] Mbugote { $count } emechala
}
files-tray-some-failed = Ebugotere { $done }, { $failed } dara
files-tray-minutes-left = { $minutes ->
   *[other] Ihe dị ka nkeji { $minutes } fọdụrụ
}
files-tray-seconds-left = Ihe na-erughị otu nkeji fọdụrụ
files-tray-starting = Na-amalite…
files-tray-cancel-all = Kagbuo niile
files-tray-cancel = Kagbuo
files-tray-fold = Zoo ndepụta
files-tray-unfold = Gosi ndepụta
files-tray-close = Mechie
files-tray-progress = { $place } · { $sent } n'ime { $size }
files-tray-in = Na { $place }
files-tray-cancelled = Akagburu
