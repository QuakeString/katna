# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Sabon saƙo
compose-restore = Maido da girma
compose-minimize = Rage
compose-exit-full-screen = Fita daga cikakken allo
compose-open-window = Buɗe a sabuwar taga
compose-save-close = Ajiye kuma rufe
compose-back-to-mail = Koma taga wasiƙu
compose-pop-out-reply = Buɗe amsar a taga ta daban
compose-edit-recipients = Gyara masu karɓa
compose-summary-cc = Kwafi: { $names }
compose-summary-bcc = Kwafi a ɓoye: { $names }
compose-more-recipients = { $count } ƙari
compose-show-trimmed = Nuna abin da aka taƙaita
compose-hide-trimmed = Ɓoye abin da aka taƙaita
compose-remove-trimmed = Cire rubutun da aka ambato
compose-trimmed-removed = An cire rubutun da aka ambato

## Recipients and subject

compose-to = Zuwa
compose-cc = Kwafi
compose-bcc = Kwafi a ɓoye
compose-from = Daga
compose-from-choose = Aika daga wani asusu
compose-recipients = Masu karɓa
compose-subject = Jigo

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Da farko ku aika ko ku yi watsi da saƙon da ke buɗe.
compose-bad-address = “{ $address }” ba adireshin imel ba ne.
compose-no-recipients = Ƙara aƙalla mai karɓa ɗaya.
compose-attachments-too-large = Abubuwan haɗawa sun kai { $size }; sabobin wasiƙa suna karɓar har zuwa { $limit }.
compose-no-account = Ƙara asusun da za a aika wasiƙa daga gare shi.
compose-past-time = Zaɓi lokaci a nan gaba.
compose-scheduling = Ana tsara lokaci…
compose-sending = Ana aikawa…
compose-scheduled = An tsara aikawa a { $when }
compose-sent-archived = An aika kuma an adana a ma'ajiya
compose-sent = An aika saƙo
compose-discarded = An yi watsi da zayyana
compose-draft-saved = An adana zayyana
compose-draft-saving = Ana ajiyewa…
compose-draft-failed = Ba a iya adana zayyanar ba: { $error }
compose-draft-not-opened = Ba a iya buɗe zayyanar ba.

## Attachments

compose-picker-insert = Saka
compose-picker-attach = Haɗa
compose-file-too-large = { $name } ya yi girma sosai: saƙo zai iya ɗaukar har zuwa { $limit }.
compose-forward-files-missing = Ba a sauke fayilolin saƙon da aka tura ba, don haka ba a haɗa su ba.
compose-attachment-size = ({ $size })
compose-remove-attachment = Cire abin haɗawa
compose-attachments-total = { $count ->
    [one] fayil { $count }, { $size }
   *[other] fayiloli { $count }, { $size }
}
compose-drive-note = { $name } ya wuce { $limit }, don haka ana ajiye shi a Google Drive ɗinka kuma saƙon yana ɗauke da mahaɗi.
compose-drive-tip = A Google Drive ɗinka; saƙon yana ɗauke da mahaɗi
compose-drive-uploading = Ana ɗorawa { $percent }%
compose-drive-allow = Ba da izinin Drive
compose-drive-allow-tip = Sake shiga da Google don Katna ta iya sanya manyan fayiloli a Drive ɗinka
compose-drive-retry = Sake gwadawa
compose-drive-sends-when-uploaded = Za a aika bayan an ɗora { $name }
compose-drive-not-uploaded = { $name } bai riga ya shiga Google Drive ba
compose-drive-share-failed = Ba a iya raba fayilolin a Google Drive ba: { $error }
compose-drive-share-title = A raba fayilolin da kowa?
compose-drive-share-text = { $count ->
    [one] Google Drive ba zai iya raba fayilolin da { $addresses }, wanda ba shi da asusun Google ba. Maimakon haka, duk wanda yake da mahaɗin zai iya buɗe su.
   *[other] Google Drive ba zai iya raba fayilolin da { $addresses }, waɗanda ba su da asusun Google ba. Maimakon haka, duk wanda yake da mahaɗin zai iya buɗe su.
}
compose-drive-share-link = Raba ta hanyar mahaɗi
compose-drive-send-without = Aika ba tare da rabawa ba
compose-drive-share-cancel = Soke
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } ya wuce { $limit }, don haka ana ajiye shi a OneDrive ɗinka kuma saƙon yana ɗauke da mahaɗi.
compose-onedrive-tip = A OneDrive ɗinka; saƙon yana ɗauke da mahaɗi
compose-onedrive-allow = Ba da izinin OneDrive
compose-onedrive-allow-tip = Sake shiga da Microsoft don Katna ta iya sanya manyan fayiloli a OneDrive ɗinka
compose-onedrive-not-uploaded = { $name } bai riga ya shiga OneDrive ba
compose-onedrive-share-failed = Ba a iya raba fayilolin a OneDrive ba: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive ba zai iya raba fayilolin da { $addresses } ba. Maimakon haka, duk wanda yake da mahaɗin zai iya buɗe su.
   *[other] OneDrive ba zai iya raba fayilolin da { $addresses } ba. Maimakon haka, duk wanda yake da mahaɗin zai iya buɗe su.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = Saki fayiloli a nan
compose-drop-here = Saki a nan
compose-paste-keep-formatting = Riƙe tsari
compose-paste-table = Tebur
compose-paste-picture = Hoto
compose-paste-plain-text = Rubutu mara ado
compose-paste-inline = A cikin rubutu
compose-paste-attachment = Abin haɗawa

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Ɓoye
compose-encrypted = An ɓoye: masu karɓa kaɗai ne za su iya karanta shi
compose-sign = Sa hannu
compose-signed = An sa hannu: masu karɓa za su iya tabbatar da cewa daga gare ku ne
compose-track = Bibiyi buɗewa da dannawa
compose-tracked = Ana bibiya: za ku ga lokacin da kowane mai karɓa ya buɗe shi ko ya bi mahaɗi
compose-track-clicks = Bibiyi dannawar mahaɗi (rubutu mara ado ba zai iya nuna buɗewa ba)
compose-tracked-clicks = Ana bibiya: za ku ga lokacin da kowane mai karɓa ya bi mahaɗi
compose-track-sign-in = Ku shiga asusun Katna don bibiyar buɗewa da dannawa
compose-receipt = Nemi rasidin karantawa
compose-receipt-on = An nemi rasidin karantawa: manhajar mai karɓa na iya tambayarsa ya aiko da shi
compose-delivery = Nemi rasidin isarwa
compose-delivery-on = An nemi rasidin isarwa: sabar wasiƙunku za ta aiko muku da imel idan sabar kowane mai karɓa ta karɓe shi
compose-delivery-unavailable = Sabar wasiƙunku ba ta aika rasidin isarwa

## Spelling

spell-no-dictionary = Ba a saka ƙamus na rubutun kalmomi na { $language } ba (misali hunspell-en_us).
spell-dictionary-error = Ƙamus na rubutun kalmomi: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Ƙara “{ $words }”
grammar-remove = Cire “{ $words }”
grammar-ignore = Yi watsi

## Send checks (asked before a message goes out)

send-check-attachment-title = Kuna nufin haɗa fayiloli?
send-check-attachment-text = Kun ambaci abin haɗawa, amma ba a haɗa komai ba.
send-check-attach = Haɗa fayil
send-check-subject-title = Aika ba tare da jigo ba?
send-check-subject-text = Wannan saƙon ba shi da jigo.
send-check-add-subject = Ƙara jigo
send-check-send-anyway = Aika duk da haka
recipient-not-valid = Ba adireshin imel mai inganci ba ne
recipient-show-address = Nuna adireshi
recipient-remove = Cire
recipient-bad-title = Duba adireshin
recipient-bad-text = “{ $address }” ba adireshin imel mai inganci ba ne. Gyara shi ko cire shi kafin aikawa.
recipient-bad-fix = Gyara shi
