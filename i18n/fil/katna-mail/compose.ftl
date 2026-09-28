# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Bagong Mensahe
compose-restore = Ibalik ang laki
compose-minimize = I-minimize
compose-exit-full-screen = Lumabas sa full screen
compose-open-window = Buksan sa bagong window
compose-save-close = I-save at isara
compose-back-to-mail = Bumalik sa mail window
compose-pop-out-reply = Buksan nang hiwalay ang sagot
compose-edit-recipients = I-edit ang mga tatanggap
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = { $count } pa
compose-show-trimmed = Ipakita ang tinabas na nilalaman
compose-hide-trimmed = Itago ang tinabas na nilalaman
compose-remove-trimmed = Alisin ang siniping teksto
compose-trimmed-removed = Inalis ang siniping teksto

## Recipients and subject

compose-to = Para kay
compose-cc = Cc
compose-bcc = Bcc
compose-from = Mula kay
compose-from-choose = Ipadala mula sa ibang account
compose-recipients = Mga tatanggap
compose-subject = Paksa

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Ipadala o itapon muna ang nakabukas na mensahe.
compose-bad-address = Hindi email address ang “{ $address }”.
compose-no-recipients = Magdagdag ng kahit isang tatanggap.
compose-attachments-too-large = { $size } ang mga attachment; hanggang { $limit } lang ang tinatanggap ng mga mail server.
compose-no-account = Magdagdag ng account na pagpapadalhan ng mail.
compose-past-time = Pumili ng oras sa hinaharap.
compose-scheduling = Nag-iiskedyul…
compose-sending = Ipinapadala…
compose-scheduled = Naka-iskedyul ipadala sa { $when }
compose-sent-archived = Naipadala at na-archive
compose-sent = Naipadala ang mensahe
compose-discarded = Itinapon ang draft
compose-draft-saved = Na-save ang draft
compose-draft-failed = Hindi ma-save ang draft: { $error }
compose-draft-not-opened = Hindi mabuksan ang draft.

## Attachments

compose-picker-insert = Ilagay
compose-picker-attach = I-attach
compose-file-too-large = Masyadong malaki ang { $name }: hanggang { $limit } lang ang kaya ng isang mensahe.
compose-attachment-size = ({ $size })
compose-remove-attachment = Alisin ang attachment
compose-attachments-total = { $count ->
    [one] { $count } file, { $size }
   *[other] { $count } file, { $size }
}
compose-drive-note = Lampas sa { $limit } ang { $name }, kaya mapupunta ito sa iyong Google Drive at may link ang mensahe.
compose-drive-tip = Nasa iyong Google Drive; may link ang mensahe
compose-drive-uploading = Ina-upload { $percent }%
compose-drive-allow = Payagan ang Drive
compose-drive-allow-tip = Mag-sign in ulit gamit ang Google para mailagay ng Katna ang malalaking file sa iyong Drive
compose-drive-retry = Subukang muli
compose-drive-sends-when-uploaded = Ipapadala kapag na-upload na ang { $name }
compose-drive-not-uploaded = Wala pa sa Google Drive ang { $name }
compose-drive-share-failed = Hindi maibahagi ang mga file sa Google Drive: { $error }
compose-drive-share-title = Ibahagi ang mga file sa lahat?
compose-drive-share-text = { $count ->
    [one] Hindi maibabahagi ng Google Drive ang mga file kay { $addresses }, na walang Google account. Sa halip, mabubuksan ito ng sinumang may link.
   *[other] Hindi maibabahagi ng Google Drive ang mga file kina { $addresses }, na walang Google account. Sa halip, mabubuksan ito ng sinumang may link.
}
compose-drive-share-link = Ibahagi gamit ang link
compose-drive-send-without = Ipadala nang hindi nagbabahagi
compose-drive-share-cancel = Kanselahin
compose-drive-card-detail = { $size } · Google Drive
compose-onedrive-note = Lampas sa { $limit } ang { $name }, kaya mapupunta ito sa iyong OneDrive at may link ang mensahe.
compose-onedrive-tip = Nasa iyong OneDrive; may link ang mensahe
compose-onedrive-allow = Payagan ang OneDrive
compose-onedrive-allow-tip = Mag-sign in ulit gamit ang Microsoft para mailagay ng Katna ang malalaking file sa iyong OneDrive
compose-onedrive-not-uploaded = Wala pa sa OneDrive ang { $name }
compose-onedrive-share-failed = Hindi maibahagi ang mga file sa OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] Hindi maibabahagi ng OneDrive ang mga file kay { $addresses }. Sa halip, mabubuksan ito ng sinumang may link.
   *[other] Hindi maibabahagi ng OneDrive ang mga file kay { $addresses }. Sa halip, mabubuksan ito ng sinumang may link.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-drop-files = I-drop dito ang mga file
compose-drop-here = I-drop dito
compose-paste-keep-formatting = Panatilihin ang format
compose-paste-table = Talahanayan
compose-paste-picture = Larawan
compose-paste-plain-text = Plain text
compose-paste-inline = Sa text
compose-paste-attachment = Attachment

## Encryption and signing (the toggles by the recipients)

compose-encrypt = I-encrypt
compose-encrypted = Naka-encrypt: ang mga tatanggap lang ang makakabasa nito
compose-sign = Lagdaan
compose-signed = Nilagdaan: masusuri ng mga tatanggap na galing ito sa iyo
compose-track = I-track ang mga pagbukas at pag-click
compose-tracked = Naka-track: makikita mo kung kailan ito binubuksan ng bawat tatanggap o sinusundan ang isang link
compose-track-clicks = I-track ang mga pag-click sa link (hindi maipapakita ng plain text ang mga pagbukas)
compose-tracked-clicks = Naka-track: makikita mo kung kailan sinusundan ng bawat tatanggap ang isang link
compose-track-sign-in = Mag-sign in sa isang Katna account para ma-track ang mga pagbukas at pag-click
compose-receipt = Humingi ng read receipt
compose-receipt-on = Humingi ng read receipt: maaaring hilingin ng app ng tatanggap na magpadala sila nito
compose-delivery = Humingi ng delivery receipt
compose-delivery-on = Humingi ng delivery receipt: mag-e-email sa iyo ang mail server mo kapag tinanggap ito ng server ng bawat tatanggap
compose-delivery-unavailable = Hindi nagpapadala ng mga delivery receipt ang mail server mo

## Spelling

spell-no-dictionary = Walang naka-install na diksyunaryo ng pagbaybay para sa { $language } (halimbawa hunspell-en_us).
spell-dictionary-error = Diksyunaryo ng pagbaybay: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Idagdag ang “{ $words }”
grammar-remove = Alisin ang “{ $words }”
grammar-ignore = Balewalain

## Send checks (asked before a message goes out)

send-check-attachment-title = Balak mo bang mag-attach ng mga file?
send-check-attachment-text = Nabanggit mo ang isang attachment, pero walang naka-attach.
send-check-attach = Mag-attach ng file
send-check-subject-title = Ipadala nang walang paksa?
send-check-subject-text = Walang paksa ang mensaheng ito.
send-check-add-subject = Magdagdag ng paksa
send-check-send-anyway = Ipadala pa rin
recipient-not-valid = Hindi valid na email address
recipient-show-address = Ipakita ang address
recipient-remove = Alisin
recipient-bad-title = Suriin ang address
recipient-bad-text = Hindi valid na email address ang “{ $address }”. Ayusin o alisin ito bago ipadala.
recipient-bad-fix = Ayusin
