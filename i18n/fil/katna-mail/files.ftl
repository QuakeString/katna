# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Maghanap ng mga file

## Left side (and chips on a phone)

files-all = Lahat ng file
files-pictures = Mga larawan
files-pdfs = Mga PDF
files-documents = Mga dokumento
files-sheets = Mga spreadsheet
files-slides = Mga slide
files-other = Iba pa
files-accounts = Mga Account
files-drives = Mga drive
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Ibinahagi sa akin
files-shown = Ipinapakita
files-received = Natanggap
files-sent = Ipinadala ko

## Over the files

files-count = { $count ->
    [one] { $count } file · { $size }
   *[other] { $count } file · { $size }
}
files-anyone = Kahit sino
files-from-person = Mula kay { $name }
files-time-any = Anumang oras
files-time-today = Ngayon
files-time-yesterday = Kahapon
files-time-this-week = Ngayong linggo
files-time-last-week = Noong nakaraang linggo
files-time-this-month = Ngayong buwan
files-time-last-month = Noong nakaraang buwan
files-time-between = { $first } – { $last }
files-time-hint = Mag-click ng isang araw, o mag-drag sa mga araw
files-time-summary = { $count ->
    [one] { $days } · { $count } file
   *[other] { $days } · { $count } file
}
files-time-clear = I-clear
files-time-month-back = Nakaraang buwan
files-time-month-on = Susunod na buwan
files-time-wheel = I-scroll para ilipat ang mga petsang ito, nang hindi binabago ang haba
files-sort-newest = Pinakabago muna
files-sort-oldest = Pinakaluma muna
files-sort-largest = Pinakamalaki muna
files-sort-name = Ayon sa pangalan
files-grid = Mga card
files-list = Listahan
files-this-week = Ngayong linggo
files-undated = Walang petsa
files-me = Ako
files-no-subject = (walang subject)
files-loading = Tinitipon ang mga file mula sa iyong mail…
files-empty = Lumalabas dito ang mga file mula sa iyong mail.
files-none-match = Walang file na tumutugma.
files-load-failed = Pumalya ang pagbasa ng mga file: { $error }

## A file's menu and buttons

files-open = Buksan
files-open-with = Buksan gamit ang…
files-save = I-save…
files-show-mail = Ipakita ang mail
files-mail-window = Buksan ang mail sa bagong window
files-forward = Ipasa ang file
files-from-them = Mga file mula kay { $name }
files-copy-name = Kopyahin ang pangalan ng file
files-name-copied = Nakopya ang pangalan ng file
files-downloading = Dina-download ang mail…
files-download-failed = Hindi ma-download ang mail na ito.

## A cloud drive in place of the mail files

files-drive-mine = My Drive
files-drive-mine-onedrive = Aking mga file
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] { $files } file
       *[other] { $files } file
    }
    [one] { $folders } folder · { $files ->
        [one] { $files } file
       *[other] { $files } file
    }
   *[other] { $folders } folder · { $files ->
        [one] { $files } file
       *[other] { $files } file
    }
}
files-drive-folders = Mga Folder
files-drive-files = Mga File
files-drive-folder = Folder
files-drive-meta = { $what } · In-edit noong { $date }
files-drive-as-link = { $what } · bilang link
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Kinukuha…
files-drive-loading = Binubuksan ang drive…
files-drive-empty = Walang laman ang folder na ito.
files-drive-unreachable = Hindi maabot ang { $drive }.
files-drive-try-again = Subukang muli
files-drive-needs-permission = Kailangan ng Katna ng pahintulot mo nang isang beses para ipakita ang drive na ito. Mag-sign in ulit at payagan ang Katna na makita ang iyong mga file.
files-drive-allow = Payagan
files-drive-allow-failed = Hindi natapos ang pag-sign in, kaya nananatiling sarado ang drive.
files-drive-attach = I-attach
files-drive-more = Higit pa
files-drive-download = I-download…
files-drive-open-web = Buksan sa { $drive }
files-drive-copy-link = Kopyahin ang link
files-drive-link-copied = Nakopya ang link
files-drive-share = Ibahagi…
files-drive-rename = I-rename
files-drive-trash = Ilipat sa basurahan
files-drive-trashed = Nasa basurahan ng { $drive } ang “{ $name }”
files-drive-renamed = Na-rename bilang “{ $name }”
files-drive-getting = Kinukuha ang { $name } mula sa { $drive }…
files-drive-get-failed = Hindi makuha ang { $name }: { $error }
files-drive-upload = Mag-upload
files-drive-upload-files = Mag-upload ng mga file
files-drive-upload-folder = Mag-upload ng folder
files-drive-upload-failed = Hindi ma-upload ang { $name }: { $error }
files-drive-upload-needs = Para makapag-upload, kailangan ng Katna ng pahintulot mo nang isang beses: pindutin ang Payagan sa Mga setting › Mga default na app › Pahina ng Mga File.

## The Share dialog of a drive file or folder

files-share-title = Ibahagi ang “{ $name }”
files-share-add = Magdagdag ng tao ayon sa pangalan o address
files-share-not-address = Hindi email address ang “{ $text }”
files-share-notify = Hayaang i-email din sila ng { $drive }
files-share-people = Mga taong may access
files-share-general = Pangkalahatang access
files-share-loading = Binabasa kung sino ang may access…
files-share-restricted = Pinaghihigpitan
files-share-restricted-about = Ang mga taong may access lang ang makakapagbukas nito gamit ang link
files-share-anyone = Sinumang may link
files-share-anyone-can = { $role ->
    [editor] Puwedeng mag-edit ang sinumang may link
    [commenter] Puwedeng magkomento ang sinumang may link
   *[viewer] Puwedeng tumingin ang sinumang may link
}
files-share-anyone-about = { $role ->
    [editor] Puwedeng mag-edit ang sinuman sa internet na may link
    [commenter] Puwedeng magkomento ang sinuman sa internet na may link
   *[viewer] Puwedeng tumingin ang sinuman sa internet na may link
}
files-share-role-owner = May-ari
files-share-role-editor = Editor
files-share-role-commenter = Commenter
files-share-role-viewer = Viewer
files-share-you = { $name } (ikaw)
files-share-domain = Lahat sa { $domain }
files-share-inherited = Access mula sa folder na kinalalagyan nito
files-share-remove = Alisin ang access
files-share-copy-link = Kopyahin ang link
files-share-share = Ibahagi
files-share-done = Tapos na
files-share-close = Isara
files-share-sharing = Ibinabahagi…
files-share-shared = { $count ->
    [one] Ibinahagi sa { $count } tao
   *[other] Ibinahagi sa { $count } tao
}
files-share-refused = Hindi maibahagi ng { $drive } sa { $addresses }
files-share-failed = Hindi mabago ang pagbabahagi: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Ina-upload ang { $count } item
   *[other] Ina-upload ang { $count } item
}
files-tray-done = { $count ->
    [one] Tapos ang { $count } upload
   *[other] Tapos ang { $count } upload
}
files-tray-some-failed = { $done } ang na-upload, { $failed } ang pumalya
files-tray-minutes-left = { $minutes ->
    [one] Mga isang minuto pa
   *[other] Mga { $minutes } minuto pa
}
files-tray-seconds-left = Wala nang isang minuto
files-tray-starting = Nagsisimula…
files-tray-cancel-all = Kanselahin lahat
files-tray-cancel = Kanselahin
files-tray-fold = Itago ang listahan
files-tray-unfold = Ipakita ang listahan
files-tray-close = Isara
files-tray-progress = { $place } · { $sent } ng { $size }
files-tray-in = Nasa { $place }
files-tray-cancelled = Nakansela
