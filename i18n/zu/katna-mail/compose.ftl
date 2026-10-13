# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Umlayezo Omusha
compose-restore = Buyisela
compose-minimize = Nciphisa
compose-exit-full-screen = Phuma kusikrini esigcwele
compose-open-window = Vula ewindini elisha
compose-save-close = Londoloza bese uvala
compose-back-to-mail = Buyela ewindini le-imeyili
compose-pop-out-reply = Vula impendulo ngokwehlukile
compose-edit-recipients = Hlela abamukeli
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = { $count } ngaphezulu
compose-show-trimmed = Bonisa okuqukethwe okufinyeziwe
compose-hide-trimmed = Fihla okuqukethwe okufinyeziwe
compose-remove-trimmed = Susa umbhalo ocashuniwe
compose-trimmed-removed = Umbhalo ocashuniwe ususiwe

## The quoted or forwarded message, in the mail itself

compose-quote-header = Mhla { $date }, { $from } wabhala:
compose-forward-header = ---------- Umlayezo odluliselwe ---------
compose-forward-from = Kusuka ku: { $from }
compose-forward-date = Usuku: { $date }
compose-forward-subject = Isihloko: { $subject }
compose-forward-to = Ku: { $to }
compose-forward-cc = Cc: { $cc }

## Recipients and subject

compose-to = Ku
compose-cc = Cc
compose-bcc = Bcc
compose-from = Kusuka ku
compose-from-choose = Thumela kusuka kwenye i-akhawunti
compose-recipients = Abamukeli
compose-subject = Isihloko

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Thumela noma ulahle umlayezo ovuliwe kuqala.
compose-bad-address = “{ $address }” akulona ikheli le-imeyili.
compose-no-recipients = Engeza okungenani umamukeli oyedwa.
compose-attachments-too-large = Okunamathiselwe kungu-{ $size }; amaseva e-imeyili amukela kufika ku-{ $limit }.
compose-no-account = Engeza i-akhawunti ozothumela ngayo imeyili.
compose-past-time = Khetha isikhathi esizayo.
compose-scheduling = Iyahlela…
compose-sending = Iyathumela…
compose-scheduled = Ukuthumela kuhlelelwe { $when }
compose-sent-archived = Kuthunyelwe futhi kwafakwa kungobo yomlando
compose-sent = Umlayezo uthunyelwe
compose-discarded = Okusalungiswa kulahliwe
compose-draft-saved = Okusalungiswa kulondoloziwe
compose-draft-saving = Iyalondoloza…
compose-draft-failed = Okusalungiswa akukwazanga ukulondolozwa: { $error }
compose-draft-not-opened = Okusalungiswa akukwazanga ukuvulwa.

## Attachments

compose-picker-insert = Faka
compose-picker-attach = Namathisela
compose-file-too-large = { $name } likhulu kakhulu: umlayezo ungathwala kufika ku-{ $limit }.
compose-forward-files-missing = Amafayela omlayezo odluliselwayo awakalandwa, ngakho awanamathiselwanga.
compose-attachment-size = ({ $size })
compose-remove-attachment = Susa okunamathiselwe
compose-attachment-open-tip = Vula ukuze ulihlole
compose-attachments-total = { $count ->
    [one] Ifayela elingu-{ $count }, { $size }
   *[other] Amafayela angu-{ $count }, { $size }
}
compose-drive-note = { $name } idlula { $limit }, ngakho iya ku-Google Drive yakho futhi umlayezo uphethe isixhumanisi.
compose-drive-tip = Ku-Google Drive yakho; umlayezo uphethe isixhumanisi
compose-drive-uploading = Iyalayisha { $percent }%
compose-drive-allow = Vumela i-Drive
compose-drive-allow-tip = Ngena ngeGoogle futhi ukuze i-Katna ikwazi ukufaka amafayela amakhulu ku-Drive yakho
compose-drive-retry = Zama futhi
compose-drive-sends-when-uploaded = Kuzothunyelwa uma { $name } isilayishiwe
compose-drive-not-uploaded = { $name } ayikho ku-Google Drive okwamanje
compose-drive-share-failed = Ayikwazanga ukwabelana ngamafayela ku-Google Drive: { $error }
compose-drive-share-title = Wabelane amafayela nabo bonke?
compose-drive-share-text = { $count ->
    [one] I-Google Drive ayikwazi ukwabelana amafayela no-{ $addresses }, ongenayo i-akhawunti ye-Google. Noma ubani onesixhumanisi angawavula esikhundleni salokho.
   *[other] I-Google Drive ayikwazi ukwabelana amafayela nabo-{ $addresses }, abangenayo i-akhawunti ye-Google. Noma ubani onesixhumanisi angawavula esikhundleni salokho.
}
compose-drive-share-link = Wabelane ngesixhumanisi
compose-drive-send-without = Thumela ngaphandle kokwabelana
compose-drive-share-cancel = Khansela
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } idlula { $limit }, ngakho iya ku-OneDrive yakho futhi umlayezo uphethe isixhumanisi.
compose-onedrive-tip = Ku-OneDrive yakho; umlayezo uphethe isixhumanisi
compose-onedrive-allow = Vumela i-OneDrive
compose-onedrive-allow-tip = Ngena ngeMicrosoft futhi ukuze i-Katna ikwazi ukufaka amafayela amakhulu ku-OneDrive yakho
compose-onedrive-not-uploaded = { $name } ayikho ku-OneDrive okwamanje
compose-onedrive-share-failed = Ayikwazanga ukwabelana ngamafayela ku-OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] I-OneDrive ayikwazi ukwabelana amafayela no-{ $addresses }. Noma ubani onesixhumanisi angawavula esikhundleni salokho.
   *[other] I-OneDrive ayikwazi ukwabelana amafayela nabo-{ $addresses }. Noma ubani onesixhumanisi angawavula esikhundleni salokho.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = Yehlisela amafayela lapha
compose-drop-here = Yehlisela lapha
compose-paste-keep-formatting = Gcina ukufometha
compose-paste-table = Ithebula
compose-paste-picture = Isithombe
compose-paste-plain-text = Umbhalo osobala
compose-paste-inline = Embhalweni
compose-paste-attachment = Okunamathiselwe

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Bethela
compose-encrypted = Kubethelwe: abamukeli kuphela abangakufunda
compose-sign = Sayina
compose-signed = Kusayiniwe: abamukeli bangahlola ukuthi kuvela kuwe
compose-track = Landelela ukuvulwa nokuchofozwa
compose-tracked = Kuyalandelelwa: uzobona lapho umamukeli ngamunye ewuvula noma elandela isixhumanisi
compose-track-clicks = Landelela ukuchofozwa kwezixhumanisi (umbhalo osobala awukwazi ukubonisa ukuvulwa)
compose-tracked-clicks = Kuyalandelelwa: uzobona lapho umamukeli ngamunye elandela isixhumanisi
compose-track-sign-in = Ngena ku-akhawunti ye-Katna ukuze ulandelele ukuvulwa nokuchofozwa
compose-receipt = Cela isaziso sokufunda
compose-receipt-on = Isaziso sokufunda siceliwe: uhlelo lokusebenza lomamukeli lungamcela ukuthi asithumele
compose-delivery = Cela isaziso sokulethwa
compose-delivery-on = Isaziso sokulethwa siceliwe: iseva yakho yemeyili izokuthumelela imeyili lapho iseva yomamukeli ngamunye yamukela umlayezo
compose-delivery-unavailable = Iseva yakho yemeyili ayithumeli izaziso zokulethwa

## Spelling

spell-no-dictionary = Asikho isichazamazwi sokupela se-{ $language } esifakiwe (isibonelo hunspell-en_us).
spell-dictionary-error = Isichazamazwi sokupela: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Engeza “{ $words }”
grammar-remove = Susa “{ $words }”
grammar-ignore = Ziba

## Send checks (asked before a message goes out)

send-check-attachment-title = Ingabe ubuhlose ukunamathisela amafayela?
send-check-attachment-text = Ubhale ngokunamathiselwe, kodwa akukho okunamathiselwe.
send-check-attach = Namathisela ifayela
send-check-subject-title = Thumela ngaphandle kwesihloko?
send-check-subject-text = Lo mlayezo awunaso isihloko.
send-check-add-subject = Engeza isihloko
send-check-send-anyway = Thumela noma kunjalo
recipient-not-valid = Akulona ikheli le-imeyili elivumelekile
recipient-show-address = Bonisa ikheli
recipient-remove = Susa
recipient-bad-title = Hlola ikheli
recipient-bad-text = “{ $address }” akulona ikheli le-imeyili elivumelekile. Lilungise noma ulisuse ngaphambi kokuthumela.
recipient-bad-fix = Lilungise
