# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = नयाँ सन्देश
compose-restore = पुनर्स्थापित गर्नुहोस्
compose-minimize = सानो बनाउनुहोस्
compose-exit-full-screen = पूरा स्क्रिनबाट बाहिरिनुहोस्
compose-open-window = नयाँ विन्डोमा खोल्नुहोस्
compose-save-close = सेभ गरेर बन्द गर्नुहोस्
compose-back-to-mail = मेल विन्डोमा फर्कनुहोस्
compose-pop-out-reply = जवाफ छुट्टै विन्डोमा खोल्नुहोस्
compose-edit-recipients = प्रापकहरू सम्पादन गर्नुहोस्
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = थप { $count }
compose-show-trimmed = काटिएको सामग्री देखाउनुहोस्
compose-hide-trimmed = काटिएको सामग्री लुकाउनुहोस्
compose-remove-trimmed = उद्धृत पाठ हटाउनुहोस्
compose-trimmed-removed = उद्धृत पाठ हटाइयो

## Recipients and subject

compose-to = प्रापक
compose-cc = Cc
compose-bcc = Bcc
compose-from = प्रेषक
compose-from-choose = अर्को खाताबाट पठाउनुहोस्
compose-recipients = प्रापकहरू
compose-subject = विषय

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = पहिले खुला सन्देश पठाउनुहोस् वा खारेज गर्नुहोस्।
compose-bad-address = “{ $address }” इमेल ठेगाना होइन।
compose-no-recipients = कम्तीमा एक जना प्रापक थप्नुहोस्।
compose-attachments-too-large = संलग्नकहरू { $size } छन्; मेल सर्भरहरूले { $limit } सम्म मात्र लिन्छन्।
compose-no-account = मेल पठाउनका लागि एउटा खाता थप्नुहोस्।
compose-past-time = भविष्यको समय छान्नुहोस्।
compose-scheduling = तालिका बनाउँदै…
compose-sending = पठाउँदै…
compose-scheduled = { $when } मा पठाउने तालिका बनाइयो
compose-sent-archived = पठाइयो र संग्रह गरियो
compose-sent = सन्देश पठाइयो
compose-discarded = ड्राफ्ट खारेज गरियो
compose-draft-saved = ड्राफ्ट सेभ गरियो
compose-draft-failed = ड्राफ्ट सेभ गर्न सकिएन: { $error }
compose-draft-not-opened = ड्राफ्ट खोल्न सकिएन।

## Attachments

compose-picker-insert = घुसाउनुहोस्
compose-picker-attach = संलग्न गर्नुहोस्
compose-file-too-large = { $name } धेरै ठूलो छ: एउटा सन्देशमा { $limit } सम्म मात्र पठाउन सकिन्छ।
compose-attachment-size = ({ $size })
compose-remove-attachment = संलग्नक हटाउनुहोस्
compose-attachments-total = { $count ->
    [one] { $count } फाइल, { $size }
   *[other] { $count } फाइलहरू, { $size }
}
compose-drive-note = { $name } { $limit } भन्दा ठूलो छ, त्यसैले यो तपाईंको Google Drive मा जान्छ र सन्देशमा यसको लिङ्क हुन्छ।
compose-drive-tip = तपाईंको Google Drive मा; सन्देशमा लिङ्क हुन्छ
compose-drive-uploading = अपलोड गर्दै { $percent }%
compose-drive-allow = Drive लाई अनुमति दिनुहोस्
compose-drive-allow-tip = ठूला फाइलहरू तपाईंको Drive मा राख्न Katna लाई अनुमति दिन Google बाट फेरि साइन इन गर्नुहोस्
compose-drive-retry = फेरि प्रयास गर्नुहोस्
compose-drive-sends-when-uploaded = { $name } अपलोड भएपछि पठाइनेछ
compose-drive-not-uploaded = { $name } अझै Google Drive मा छैन
compose-drive-share-failed = Google Drive मा फाइलहरू सेयर गर्न सकिएन: { $error }
compose-drive-share-title = फाइलहरू सबैसँग सेयर गर्ने?
compose-drive-share-text = { $count ->
    [one] Google Drive ले { $addresses } सँग फाइलहरू सेयर गर्न सक्दैन, जसको Google खाता छैन। बरु लिङ्क भएको जोसुकैले तिनलाई खोल्न सक्छ।
   *[other] Google Drive ले { $addresses } सँग फाइलहरू सेयर गर्न सक्दैन, जसको Google खाता छैन। बरु लिङ्क भएको जोसुकैले तिनलाई खोल्न सक्छ।
}
compose-drive-share-link = लिङ्कमार्फत सेयर गर्नुहोस्
compose-drive-send-without = सेयर नगरी पठाउनुहोस्
compose-drive-share-cancel = रद्द गर्नुहोस्
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = फाइलहरू यहाँ छोड्नुहोस्
compose-drop-here = यहाँ छोड्नुहोस्
compose-paste-keep-formatting = ढाँचा राख्नुहोस्
compose-paste-table = तालिका
compose-paste-picture = तस्बिर
compose-paste-plain-text = सादा पाठ
compose-paste-inline = पाठभित्र
compose-paste-attachment = संलग्नक

## Encryption and signing (the toggles by the recipients)

compose-encrypt = इन्क्रिप्ट गर्नुहोस्
compose-encrypted = इन्क्रिप्ट गरिएको: प्रापकहरूले मात्र पढ्न सक्छन्
compose-sign = हस्ताक्षर गर्नुहोस्
compose-signed = हस्ताक्षर गरिएको: यो तपाईंबाटै आएको हो भनी प्रापकहरूले जाँच्न सक्छन्
compose-track = खोलेको र क्लिक ट्र्याक गर्नुहोस्
compose-tracked = ट्र्याक गरिँदै: हरेक प्रापकले यो कहिले खोल्छन् वा लिङ्क खोल्छन् भनी तपाईं देख्नुहुन्छ
compose-track-clicks = लिङ्क क्लिक ट्र्याक गर्नुहोस् (सादा पाठमा खोलेको देखाउन सकिँदैन)
compose-tracked-clicks = ट्र्याक गरिँदै: हरेक प्रापकले कहिले लिङ्क खोल्छन् भनी तपाईं देख्नुहुन्छ
compose-track-sign-in = खोलेको र क्लिक ट्र्याक गर्न Katna खातामा साइन इन गर्नुहोस्
compose-receipt = पढेको रसिद माग्नुहोस्
compose-receipt-on = पढेको रसिद मागिएको छ: प्रापकको एपले उनीहरूलाई रसिद पठाउन सोध्न सक्छ
compose-delivery = डेलिभरी रसिद माग्नुहोस्
compose-delivery-on = डेलिभरी रसिद मागिएको छ: हरेक प्रापकको सर्भरले सन्देश स्वीकार गर्दा तपाईंको मेल सर्भरले तपाईंलाई इमेल पठाउनेछ
compose-delivery-unavailable = तपाईंको मेल सर्भरले डेलिभरी रसिद पठाउँदैन

## Spelling

spell-no-dictionary = { $language } का लागि कुनै हिज्जे शब्दकोश स्थापना गरिएको छैन (उदाहरणका लागि hunspell-en_us)।
spell-dictionary-error = हिज्जे शब्दकोश: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” थप्नुहोस्
grammar-remove = “{ $words }” हटाउनुहोस्
grammar-ignore = बेवास्ता गर्नुहोस्

## Send checks (asked before a message goes out)

send-check-attachment-title = के तपाईं फाइलहरू संलग्न गर्न चाहनुहुन्थ्यो?
send-check-attachment-text = तपाईंले संलग्नकको कुरा लेख्नुभयो, तर केही पनि संलग्न गरिएको छैन।
send-check-attach = फाइल संलग्न गर्नुहोस्
send-check-subject-title = विषयबिना पठाउने?
send-check-subject-text = यो सन्देशको कुनै विषय छैन।
send-check-add-subject = विषय थप्नुहोस्
send-check-send-anyway = जे भए पनि पठाउनुहोस्
recipient-not-valid = मान्य इमेल ठेगाना होइन
recipient-show-address = ठेगाना देखाउनुहोस्
recipient-remove = हटाउनुहोस्
recipient-bad-title = ठेगाना जाँच गर्नुहोस्
recipient-bad-text = “{ $address }” मान्य इमेल ठेगाना होइन। पठाउनुअघि यसलाई सच्याउनुहोस् वा हटाउनुहोस्।
recipient-bad-fix = सच्याउनुहोस्
