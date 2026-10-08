# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = बन्द गर्नुहोस्
reader-back = पछाडि
reader-mark-unread = नपढिएको भनी चिन्ह लगाउनुहोस्
reader-move-to = यहाँ सार्नुहोस्
reader-snooze = स्नुज गर्नुहोस्
reader-remind = मलाई सम्झाउनुहोस्
reader-more = थप
reader-original-colors = मौलिक रङहरू देखाउनुहोस्
reader-dark-colors = गाढा रङहरूमा देखाउनुहोस्
reader-print-all = सबै प्रिन्ट गर्नुहोस्
reader-new-window = नयाँ विन्डोमा
reader-position = { $total } मध्ये { $position }
reader-newer = नयाँ
reader-older = पुरानो

## Reading pane: the conversation

reader-removed = यो वार्तालाप हटाइयो।
reader-no-subject = (विषय छैन)
reader-collapse-all = सबै खुम्च्याउनुहोस्
reader-expand-all = सबै विस्तार गर्नुहोस्
reader-unknown-sender = (अज्ञात प्रेषक)
reader-date-ago = { $date } ({ $ago })
reader-sending = पठाउँदै…
reader-me = म
reader-to = { $names } लाई
reader-to-label = प्रापक:
reader-tick-delivered = पुग्यो: { $when }
reader-tick-no-bounce = पठाइयो: { $when }; बाउन्स फर्केन, त्यसैले सम्भवतः पुग्यो
reader-tick-bounced = पुगेन: { $when } मा बाउन्स भयो
reader-tick-read = पढियो: { $when } (पढेको रसिद)
reader-tick-opened = खोलियो, पछिल्लो पटक { $when } (खोलेको ट्र्याकिङ)
reader-starred = तारा लगाइएको
reader-chip-remove = { $label } हटाउनुहोस्
reader-not-starred = तारा नलगाइएको
reader-too-long = सन्देश पूरै देखाउन धेरै लामो छ।
reader-encrypted-images = इन्क्रिप्ट गरिएको मेलमा वेबका तस्बिरहरू कहिल्यै लोड गरिँदैनन्।
reader-window-failed = नयाँ विन्डो खोल्न सकिएन।

## Reading pane: message details (opened from "to me")

reader-details-from = प्रेषक:
reader-details-to = प्रापक:
reader-details-cc = cc:
reader-details-date = मिति:
reader-details-subject = विषय:

## Reading pane: downloading a message

reader-downloading = सर्भरबाट यो सन्देश डाउनलोड गर्दै…
reader-download-failed = यो सन्देश डाउनलोड गर्न सकिएन।
reader-download-failed-reason = यो सन्देश डाउनलोड गर्न सकिएन। { $reason }
reader-download-offline = यो खाता अफलाइन छ। यो सन्देश डाउनलोड गर्न अनलाइन हुनुहोस्।
reader-try-again = फेरि प्रयास गर्नुहोस्

## Reply row

reply-reply = जवाफ दिनुहोस्
reply-reply-all = सबैलाई जवाफ दिनुहोस्
reply-forward = फर्वार्ड गर्नुहोस्

## Encrypted and signed mail

security-decrypting = डिक्रिप्ट गर्दै…
security-checking = हस्ताक्षर जाँच गर्दै…
security-partly-encrypted = यो सन्देशको केही भाग मात्र इन्क्रिप्ट गरिएको छ। बाँकी भाग सुरक्षाबाहिर थपिएको हो र जोसुकैबाट आएको हुन सक्छ।
security-partly-signed = यो सन्देशको केही भागमा मात्र हस्ताक्षर गरिएको छ। बाँकी भाग सुरक्षाबाहिर थपिएको हो र जोसुकैबाट आएको हुन सक्छ।
security-encrypted = इन्क्रिप्ट गरिएको सन्देश
security-encrypted-smime = इन्क्रिप्ट गरिएको सन्देश (S/MIME)
security-no-key = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: यो तपाईंसँग नभएको कुञ्जीका लागि इन्क्रिप्ट गरिएको थियो।
security-cancelled = डिक्रिप्ट गर्ने कार्य रद्द गरियो।
security-damaged = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: इन्क्रिप्ट गरिएको डेटा बिग्रिएको छ वा परिवर्तन गरिएको छ।
security-decrypt-unavailable = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: इन्क्रिप्ट गरिएको मेल पढ्न { $tool } स्थापना गर्नुहोस्।
security-decrypt-failed = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: { $reason }
security-unknown-signer = एक अज्ञात हस्ताक्षरकर्ता
security-signed-verified = { $signer } द्वारा हस्ताक्षरित · प्रमाणित
security-signed-not-sender = { $signer } द्वारा हस्ताक्षरित, जो प्रेषक होइनन्
security-signed-untrusted = तपाईंले अविश्वसनीय भनी चिन्ह लगाएको कुञ्जीद्वारा { $signer } ले हस्ताक्षर गरेको
security-signed-unverified = { $signer } द्वारा हस्ताक्षरित · कुञ्जी प्रमाणित छैन
security-bad-signature = गलत हस्ताक्षर: यो सन्देश हस्ताक्षरपछि परिवर्तन गरिएको छ, वा हस्ताक्षर नक्कली हो।
security-signature-expired = { $signer } द्वारा हस्ताक्षरित · हस्ताक्षरको म्याद सकिएको छ
security-key-expired = { $signer } द्वारा हस्ताक्षरित · त्यसपछि कुञ्जीको म्याद सकिएको छ
security-key-revoked = रद्द गरिएको कुञ्जीद्वारा { $signer } ले हस्ताक्षर गरेको
security-missing-key = तपाईंसँग नभएको कुञ्जीद्वारा हस्ताक्षर गरिएकाले जाँच गर्न सकिँदैन
security-missing-key-id = तपाईंसँग नभएको कुञ्जी ({ $key }) द्वारा हस्ताक्षर गरिएकाले जाँच गर्न सकिँदैन
security-signature-unavailable = हस्ताक्षरित; हस्ताक्षर जाँच गर्न { $tool } स्थापना गर्नुहोस्
security-signature-error = हस्ताक्षर जाँच गर्न सकिएन।
security-look-up-key = कुञ्जी खोज्नुहोस्

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = प्रमाणित हस्ताक्षर
key-card-verified-detail = हस्ताक्षर ठीक छ र तपाईं यो कुञ्जीमा विश्वास गर्नुहुन्छ।
key-card-unverified = हस्ताक्षर प्रमाणित छैन
key-card-unverified-detail = हस्ताक्षर ठीक छ, तर कुञ्जी उहाँकै हो भनी कुनै कुराले पुष्टि गर्दैन। उहाँसँग फिंगरप्रिन्ट मिलाउनुहोस्, त्यसपछि GnuPG मा कुञ्जीमा विश्वास गर्नुहोस् (Kleopatra वा gpg --edit-key)।
key-card-not-sender = अरू कसैले हस्ताक्षर गरेको
key-card-not-sender-detail = हस्ताक्षर ठीक छ, तर कुञ्जी प्रेषकको होइन।
key-card-untrusted = कुञ्जी विश्वसनीय छैन
key-card-untrusted-detail = तपाईंले GnuPG मा यो कुञ्जीलाई अविश्वसनीय भनी चिन्ह लगाउनुभएको छ।
key-card-signature-expired = हस्ताक्षरको म्याद सकियो
key-card-signature-expired-detail = हस्ताक्षर ठीक थियो, तर यसको म्याद सकिएको छ।
key-card-key-expired = कुञ्जीको म्याद सकियो
key-card-key-expired-detail = हस्ताक्षर ठीक छ, तर त्यसपछि कुञ्जीको म्याद सकिएको छ।
key-card-key-revoked = कुञ्जी रद्द गरिएको
key-card-key-revoked-detail = यसको मालिकले यो कुञ्जी रद्द गर्नुभएको छ, त्यसैले हस्ताक्षरमा विश्वास गर्न सकिँदैन।
key-card-bad = गलत हस्ताक्षर
key-card-bad-detail = यो सन्देश हस्ताक्षरपछि परिवर्तन गरिएको छ, वा हस्ताक्षर नक्कली हो।
key-card-signed-by = हस्ताक्षरकर्ता
key-card-belongs-to = मालिक
key-card-fingerprint = फिंगरप्रिन्ट
key-card-signed = हस्ताक्षर गरिएको
key-card-key = कुञ्जी
key-card-kind = { $standard }, { $algorithm }
key-card-created = बनाइएको
key-card-expires = म्याद सकिने
key-card-never = कहिल्यै होइन
key-card-issued-by = जारीकर्ता
key-card-found-in = भेटिएको ठाउँ
key-card-keyring = तपाईंको GnuPG कुञ्जीसङ्ग्रह
key-card-copy = फिंगरप्रिन्ट कपी गर्नुहोस्
key-card-import-title = यो कुञ्जी आयात गर्ने?
key-card-from-directory = { $domain } को कुञ्जी निर्देशिकामा भेटियो।
key-card-from-attachment = संलग्नक { $name } बाट।
key-card-import-note = त्यसपछि Katna ले यो व्यक्तिका हस्ताक्षरहरू जाँच्न र उहाँलाई इन्क्रिप्ट गरिएको मेल पठाउन सक्छ। कुञ्जीमा पूर्ण विश्वास गर्न, उहाँसँग फिंगरप्रिन्ट मिलाउनुहोस्।
key-card-cancel = रद्द गर्नुहोस्
key-card-import = कुञ्जी आयात गर्नुहोस्
key-card-looking-up = कुञ्जी खोज्दै…
key-card-looking-up-detail = { $domain } को कुञ्जी निर्देशिकालाई सोध्दै।
key-card-not-found = कुनै कुञ्जी भेटिएन
key-card-not-found-detail = { $domain } ले यो ठेगानाका लागि कुञ्जी प्रकाशित गर्दैन। प्रेषकलाई आफ्नो कुञ्जी पठाउन भन्नुहोस्।
key-card-not-kept = भेटिएको कुञ्जी प्रयोग गर्न सकिँदैन।
key-card-failed = कुञ्जी ल्याउन सकिएन

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = यो { $domain } बाट नआएको हुन सक्छ
sender-failed-body = यो { $provider } को प्रेषक जाँचमा असफल भयो। लिङ्क, संलग्नक र जवाफहरूमा होसियार हुनुहोस्।
sender-provider-unknown = तपाईंको मेल प्रदायक
sender-details = विवरण
sender-details-hide = विवरण लुकाउनुहोस्
sender-looks-safe = सुरक्षित देखिन्छ
sender-move-to-spam = स्प्याममा सार्नुहोस्
sender-checked-by = { $provider } द्वारा जाँच गरिएको
sender-checked-by-server = { $provider } ({ $server }) द्वारा जाँच गरिएको
sender-dmarc = प्रेषकको डोमेन (DMARC)
sender-dkim = हस्ताक्षर (DKIM)
sender-spf = पठाउने सर्भर (SPF)
sender-result-pass = पास भयो
sender-result-fail = असफल
sender-result-unsure = निश्चित छैन
sender-result-none = छैन
sender-result-missing = जाँच गरिएन
sender-dmarc-pass = { $domain } ले यो प्रेषकलाई पुष्टि गर्छ।
sender-dmarc-fail = { $domain } ले आफ्नो मेल जसरी पठाइन्छ भन्छ, यो मेल त्यससँग मेल खाँदैन।
sender-dmarc-none = { $domain } ले आफ्नो मेलका लागि कुनै नियम प्रकाशित गर्दैन।
sender-dkim-pass = { $domain } द्वारा हस्ताक्षरित।
sender-dkim-fail = { $domain } को हस्ताक्षर मेलसँग मेल खाँदैन।
sender-dkim-none = सन्देशमा हस्ताक्षर गरिएको थिएन।
sender-spf-pass = { $domain } ले सूचीमा राखेको सर्भरबाट पठाइएको।
sender-spf-fail = { $domain } ले सूचीमा नराखेको सर्भरबाट पठाइएको।
sender-spf-none = { $domain } ले आफ्ना सर्भरहरूको सूची दिँदैन।
sender-check-unsure = जाँचले स्पष्ट उत्तर दिन सकेन।
sender-unconfirmed = यो { $domain } बाट आएको हो भनी { $provider } ले पुष्टि गर्न सकेन। जोसुकैले जुनसुकै प्रेषक लेख्न सक्छ।
sender-link-title = यो लिङ्क खोल्ने?
sender-link-body = यो मेल आफ्नो प्रेषक जाँचमा असफल भयो। लिङ्क { $host } मा जान्छ:
sender-link-cancel = रद्द गर्नुहोस्
sender-link-open = खोल्नुहोस्
tracking-opened = { $who } ले यो { $count ->
    [one] एक पटक
   *[other] { $count } पटक
} खोल्नुभयो, पछिल्लो पटक { $when }
tracking-opens-clicks = { $who } ले यो { $opens ->
    [one] एक पटक
   *[other] { $opens } पटक
} खोल्नुभयो र लिङ्क { $clicks ->
    [one] एक पटक
   *[other] { $clicks } पटक
} खोल्नुभयो, पछिल्लो पटक { $when }
tracking-clicked = { $who } ले लिङ्क { $clicks ->
    [one] एक पटक
   *[other] { $clicks } पटक
} खोल्नुभयो, पछिल्लो पटक { $when }
tracking-maybe-opened = { $who } ले यो खोल्नुभएको हुन सक्छ (Apple Mail ले गोपनीयताका लागि तस्बिरहरू लोड गर्छ)
tracking-seen-none = अहिलेसम्म कसैले यो खोल्नुभएको छैन वा कुनै लिङ्क खोल्नुभएको छैन
tracking-receipt = { $who } ले पढेको रसिद पठाउनुभयो
tracking-receipt-read = { $who } ले यो पढ्नुभयो (पठन रसिद), { $when }
tracking-receipt-displayed = पढेको रसिद: { $who } ले तपाईंको सन्देश खोल्नुभयो
tracking-receipt-other = पढेको रसिद: { $who } ले तपाईंको सन्देश नखोली मेटाउनुभयो वा व्यवस्थापन गर्नुभयो

## Remote images and pictures

remote-hidden = यो सन्देशका तस्बिरहरू लुकाइएका छन्।
remote-hidden-unconfirmed = तस्बिरहरू लुकाइएका छन्: प्रेषकको पुष्टि गर्न सकिएन।
remote-hidden-failed = तस्बिरहरू लुकाइएका छन्: यो मेल आफ्नो प्रेषक जाँचमा असफल भयो।
remote-show = तस्बिरहरू देखाउनुहोस्
remote-always-show = यो प्रेषकबाट सधैँ देखाउनुहोस्
remote-picture-use = प्रयोग गर्नुहोस्
remote-picture-too-big = 8 MB वा सोभन्दा सानो तस्बिर छान्नुहोस्।
remote-picture-type = PNG, JPEG, GIF, WebP वा SVG तस्बिर छान्नुहोस्।
remote-picture-read-failed = तस्बिर पढ्न सकिँदैन: { $error }
remote-picture-keep-failed = तस्बिर राख्न सकिँदैन: { $error }
remote-picture-remove-failed = तस्बिर हटाउन सकिँदैन: { $error }

## Attachments

attachment-count = { $count ->
    [one] एउटा संलग्नक
   *[other] { $count } संलग्नकहरू
}
attachment-save = सेभ गर्नुहोस्
attachment-forward = फर्वार्ड गर्नुहोस्
attachment-save-all = सबै सेभ गर्नुहोस्
attachment-save-all-tooltip = सबै संलग्नकहरू एउटा फोल्डरमा सेभ गर्नुहोस्
attachment-save-here = यहीँ सेभ गर्नुहोस्
attachment-not-downloaded = यो सन्देश डाउनलोड गरिएको छैन।
attachment-not-found = यो संलग्नक सन्देशमा फेला पार्न सकिएन।
attachment-read-failed = { $name } पढ्न सकिएन
attachment-numbered = संलग्नक { $number }
attachment-saved-all = { $count ->
    [one] { $count } फाइल { $place } मा सेभ गरियो
   *[other] { $count } फाइलहरू { $place } मा सेभ गरिए
}
attachment-saved-some = { $total ->
    [one] { $total } मध्ये { $saved } फाइल { $place } मा सेभ गरियो। { $failed } सेभ गर्न सकिएन
   *[other] { $total } मध्ये { $saved } फाइलहरू { $place } मा सेभ गरिए। { $failed } सेभ गर्न सकिएन
}
attachment-saved-to = { $path } मा सेभ गरियो
attachment-save-failed = { $name } सेभ गर्न सकिएन: { $error }
attachment-open-failed = { $name } खोल्न सकिएन: { $error }
attachment-risky = यो फाइलले कुनै प्रोग्राम चलाउन सक्छ, त्यसैले Katna ले यसलाई खोल्दैन। बरु यसलाई सेभ गर्नुहोस्।
attachment-encrypted-open = यो फाइल इन्क्रिप्ट भएर आएको हो। अन्यत्र खोल्न यसलाई सेभ गर्नुहोस्।

## Printing

print-failed = प्रिन्ट गर्न सकिएन: { $error }
print-no-font = कुनै फन्ट फेला परेन
print-opened-as-pdf = त्यहाँबाट प्रिन्ट गर्न PDF को रूपमा खोलियो।
print-preview-title = प्रिन्ट पूर्वावलोकन
print-preview-laying-out = पृष्ठहरू मिलाउँदै…
print-preview-pages = { $count ->
    [one] { $count } पृष्ठ
   *[other] { $count } पृष्ठहरू
}
print-preview-more = { $count ->
    [one] र थप { $count } पृष्ठ
   *[other] र थप { $count } पृष्ठहरू
}
print-preview-failed = पृष्ठहरू देखाउन सकिएन
print-preview-paper = कागज
print-preview-a4 = A4
print-preview-letter = लेटर
print-preview-layout = ले-आउट
print-preview-as-shown = देखिएअनुसार
print-preview-simple = पाठ मात्र
print-preview-backgrounds = पृष्ठभूमि
print-preview-cancel = रद्द गर्नुहोस्
print-preview-print = प्रिन्ट गर्नुहोस्
print-not-downloaded = (अझै डाउनलोड गरिएको छैन।)
print-encrypted = (इन्क्रिप्ट गरिएको। यसको पाठ प्रिन्ट गर्न यसलाई Katna Mail मा खोल्नुहोस्।)
print-to = प्रापक: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = माथि पिन गर्नुहोस्
text-copy-address = ठेगाना कपी गर्नुहोस्

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = यसका संलग्नकहरू पढ्न यो सन्देश खोल्नुहोस्।
text-copy = प्रतिलिपि गर्नुहोस्
text-select-all = सबै चयन गर्नुहोस्
