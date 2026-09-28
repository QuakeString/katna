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
compose-show-trimmed = Bonisa okuqukethwe okufinyeziwe
compose-hide-trimmed = Fihla okuqukethwe okufinyeziwe
compose-remove-trimmed = Susa umbhalo ocashuniwe
compose-trimmed-removed = Umbhalo ocashuniwe ususiwe

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
compose-draft-failed = Okusalungiswa akukwazanga ukulondolozwa: { $error }
compose-draft-not-opened = Okusalungiswa akukwazanga ukuvulwa.

## Attachments

compose-picker-insert = Faka
compose-picker-attach = Namathisela
compose-file-too-large = { $name } likhulu kakhulu: umlayezo ungathwala kufika ku-{ $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Susa okunamathiselwe
compose-attachments-total = { $count ->
    [one] Ifayela elingu-{ $count }, { $size }
   *[other] Amafayela angu-{ $count }, { $size }
}
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
