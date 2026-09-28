# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Ujumbe Mpya
compose-restore = Rejesha
compose-minimize = Punguza
compose-exit-full-screen = Toka kwenye skrini nzima
compose-open-window = Fungua katika dirisha jipya
compose-save-close = Hifadhi na ufunge
compose-back-to-mail = Rudi kwenye dirisha la barua
compose-pop-out-reply = Fungua jibu nje
compose-edit-recipients = Hariri wapokeaji
compose-summary-cc = Nakala: { $names }
compose-summary-bcc = Nakala fiche: { $names }
compose-more-recipients = wengine { $count }
compose-show-trimmed = Onyesha maudhui yaliyofupishwa
compose-hide-trimmed = Ficha maudhui yaliyofupishwa
compose-remove-trimmed = Ondoa maandishi yaliyonukuliwa
compose-trimmed-removed = Maandishi yaliyonukuliwa yameondolewa

## Recipients and subject

compose-to = Kwa
compose-cc = Nakala
compose-bcc = Nakala fiche
compose-from = Kutoka
compose-from-choose = Tuma kutoka akaunti nyingine
compose-recipients = Wapokeaji
compose-subject = Mada

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Tuma au tupa ujumbe ulio wazi kwanza.
compose-bad-address = “{ $address }” si anwani ya barua pepe.
compose-no-recipients = Ongeza angalau mpokeaji mmoja.
compose-attachments-too-large = Viambatisho ni { $size }; seva za barua hupokea hadi { $limit }.
compose-no-account = Ongeza akaunti ya kutumia kutuma barua.
compose-past-time = Chagua wakati ujao.
compose-scheduling = Inaratibu…
compose-sending = Inatuma…
compose-scheduled = Kutuma kumeratibiwa { $when }
compose-sent-archived = Umetumwa na kuwekwa kwenye kumbukumbu
compose-sent = Ujumbe umetumwa
compose-discarded = Rasimu imetupwa
compose-draft-saved = Rasimu imehifadhiwa
compose-draft-failed = Imeshindwa kuhifadhi rasimu: { $error }
compose-draft-not-opened = Imeshindwa kufungua rasimu.

## Attachments

compose-picker-insert = Weka
compose-picker-attach = Ambatisha
compose-file-too-large = { $name } ni kubwa mno: ujumbe unaweza kubeba hadi { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Ondoa kiambatisho
compose-attachments-total = { $count ->
    [one] Faili { $count }, { $size }
   *[other] Faili { $count }, { $size }
}
compose-drive-note = { $name } ni kubwa kuliko { $limit }, kwa hiyo huenda kwenye Google Drive yako na ujumbe hubeba kiungo.
compose-drive-tip = Kwenye Google Drive yako; ujumbe hubeba kiungo
compose-drive-uploading = Inapakia { $percent }%
compose-drive-allow = Ruhusu Drive
compose-drive-allow-tip = Ingia tena kwa Google ili Katna iweke faili kubwa kwenye Drive yako
compose-drive-retry = Jaribu tena
compose-drive-sends-when-uploaded = Ujumbe utatumwa { $name } ikishapakiwa
compose-drive-not-uploaded = { $name } bado haiko kwenye Google Drive
compose-drive-share-failed = Imeshindwa kushiriki faili kwenye Google Drive: { $error }
compose-drive-share-title = Shiriki faili na kila mtu?
compose-drive-share-text = { $count ->
    [one] Google Drive haiwezi kushiriki faili na { $addresses }, ambaye hana akaunti ya Google. Badala yake, yeyote mwenye kiungo anaweza kuzifungua.
   *[other] Google Drive haiwezi kushiriki faili na { $addresses }, ambao hawana akaunti ya Google. Badala yake, yeyote mwenye kiungo anaweza kuzifungua.
}
compose-drive-share-link = Shiriki kwa kiungo
compose-drive-send-without = Tuma bila kushiriki
compose-drive-share-cancel = Ghairi
compose-drive-card-detail = { $size } · Google Drive
compose-onedrive-note = { $name } ni kubwa kuliko { $limit }, kwa hiyo huenda kwenye OneDrive yako na ujumbe hubeba kiungo.
compose-onedrive-tip = Kwenye OneDrive yako; ujumbe hubeba kiungo
compose-onedrive-allow = Ruhusu OneDrive
compose-onedrive-allow-tip = Ingia tena kwa Microsoft ili Katna iweke faili kubwa kwenye OneDrive yako
compose-onedrive-not-uploaded = { $name } bado haiko kwenye OneDrive
compose-onedrive-share-failed = Imeshindwa kushiriki faili kwenye OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive haiwezi kushiriki faili na { $addresses }. Badala yake, yeyote mwenye kiungo anaweza kuzifungua.
   *[other] OneDrive haiwezi kushiriki faili na { $addresses }. Badala yake, yeyote mwenye kiungo anaweza kuzifungua.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-drop-files = Dondosha faili hapa
compose-drop-here = Dondosha hapa
compose-paste-keep-formatting = Dumisha uumbizaji
compose-paste-table = Jedwali
compose-paste-picture = Picha
compose-paste-plain-text = Maandishi matupu
compose-paste-inline = Ndani ya maandishi
compose-paste-attachment = Kiambatisho

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Simba
compose-encrypted = Umesimbwa: wapokeaji pekee wanaweza kuusoma
compose-sign = Weka sahihi
compose-signed = Una sahihi: wapokeaji wanaweza kuthibitisha kuwa umetoka kwako
compose-track = Fuatilia kufunguliwa na kubofya
compose-tracked = Unafuatiliwa: utaona kila mpokeaji anapoufungua au kufuata kiungo
compose-track-clicks = Fuatilia kubofya viungo (maandishi matupu hayawezi kuonyesha kufunguliwa)
compose-tracked-clicks = Unafuatiliwa: utaona kila mpokeaji anapofuata kiungo
compose-track-sign-in = Ingia kwenye akaunti ya Katna ili kufuatilia kufunguliwa na kubofya
compose-receipt = Omba stakabadhi ya kusoma
compose-receipt-on = Stakabadhi ya kusoma imeombwa: programu ya mpokeaji inaweza kumwomba aitume
compose-delivery = Omba stakabadhi ya kufikishwa
compose-delivery-on = Stakabadhi ya kufikishwa imeombwa: seva yako ya barua itakutumia barua pepe seva ya kila mpokeaji itakapoupokea ujumbe
compose-delivery-unavailable = Seva yako ya barua haitumi stakabadhi za kufikishwa

## Spelling

spell-no-dictionary = Hakuna kamusi ya tahajia ya { $language } iliyosakinishwa (kwa mfano hunspell-en_us).
spell-dictionary-error = Kamusi ya tahajia: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Ongeza “{ $words }”
grammar-remove = Ondoa “{ $words }”
grammar-ignore = Puuza

## Send checks (asked before a message goes out)

send-check-attachment-title = Ulikusudia kuambatisha faili?
send-check-attachment-text = Uliandika kuhusu kiambatisho, lakini hakuna kilichoambatishwa.
send-check-attach = Ambatisha faili
send-check-subject-title = Tuma bila mada?
send-check-subject-text = Ujumbe huu hauna mada.
send-check-add-subject = Ongeza mada
send-check-send-anyway = Tuma hata hivyo
recipient-not-valid = Si anwani sahihi ya barua pepe
recipient-show-address = Onyesha anwani
recipient-remove = Ondoa
recipient-bad-title = Kagua anwani
recipient-bad-text = “{ $address }” si anwani sahihi ya barua pepe. Irekebishe au uiondoe kabla ya kutuma.
recipient-bad-fix = Irekebishe
