# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = बंद करा
reader-back = मागे
reader-mark-unread = न वाचलेले म्हणून खूण करा
reader-move-to = येथे हलवा
reader-snooze = स्नूझ करा
reader-remind = मला आठवण करून द्या
reader-more = आणखी
reader-original-colors = मूळ रंग दाखवा
reader-dark-colors = गडद रंगांत दाखवा
reader-print-all = सर्व प्रिंट करा
reader-new-window = नवीन विंडोमध्ये
reader-position = { $total } पैकी { $position }
reader-newer = नवीन
reader-older = जुने

## Reading pane: the conversation

reader-removed = हे संभाषण काढून टाकले.
reader-no-subject = (विषय नाही)
reader-collapse-all = सर्व संकुचित करा
reader-expand-all = सर्व विस्तृत करा
reader-unknown-sender = (अज्ञात प्रेषक)
reader-date-ago = { $date } ({ $ago })
reader-sending = पाठवत आहे…
reader-me = मला
reader-to = प्रति { $names }
reader-to-label = प्रति
reader-tick-delivered = पोहोचला: { $when }
reader-tick-no-bounce = पाठवला: { $when }; बाउन्स परत आला नाही, त्यामुळे तो बहुधा पोहोचला
reader-tick-bounced = पोहोचला नाही: बाउन्स झाला, { $when }
reader-tick-read = वाचला: { $when } (वाचल्याची पावती)
reader-tick-opened = उघडला, शेवटी { $when } (उघडणे ट्रॅकिंग)
reader-starred = तारांकित
reader-chip-remove = { $label } काढा
reader-not-starred = तारांकित नाही
reader-too-long = मेसेज खूप मोठा असल्यामुळे पूर्ण दाखवता येत नाही.
reader-encrypted-images = एन्क्रिप्ट केलेल्या मेलमध्ये वेबवरील इमेज कधीही लोड केल्या जात नाहीत.
reader-window-failed = नवीन विंडो उघडता आली नाही.

## Reading pane: message details (opened from "to me")

reader-details-from = प्रेषक:
reader-details-to = प्रति:
reader-details-cc = cc:
reader-details-date = तारीख:
reader-details-subject = विषय:

## Reading pane: downloading a message

reader-downloading = सर्व्हरवरून हा मेसेज डाउनलोड करत आहे…
reader-download-failed = हा मेसेज डाउनलोड करता आला नाही.
reader-download-offline = हे खाते ऑफलाइन आहे. हा मेसेज डाउनलोड करण्यासाठी ऑनलाइन जा.
reader-try-again = पुन्हा प्रयत्न करा

## Reply row

reply-reply = उत्तर द्या
reply-reply-all = सर्वांना उत्तर द्या
reply-forward = फॉरवर्ड करा

## Encrypted and signed mail

security-decrypting = डिक्रिप्ट करत आहे…
security-checking = स्वाक्षरी तपासत आहे…
security-partly-encrypted = या मेसेजचा फक्त काही भाग एन्क्रिप्ट केलेला आहे. उरलेला भाग संरक्षणाच्या बाहेर जोडला गेला होता आणि तो कोणाकडूनही आलेला असू शकतो.
security-partly-signed = या मेसेजच्या फक्त काही भागावर स्वाक्षरी आहे. उरलेला भाग संरक्षणाच्या बाहेर जोडला गेला होता आणि तो कोणाकडूनही आलेला असू शकतो.
security-encrypted = एन्क्रिप्ट केलेला मेसेज
security-encrypted-smime = एन्क्रिप्ट केलेला मेसेज (S/MIME)
security-no-key = हा मेसेज डिक्रिप्ट करता येत नाही: तो तुमच्याकडे नसलेल्या कीसाठी एन्क्रिप्ट केला होता.
security-cancelled = डिक्रिप्ट करणे रद्द केले.
security-damaged = हा मेसेज डिक्रिप्ट करता येत नाही: एन्क्रिप्ट केलेला डेटा खराब झाला आहे किंवा बदलला गेला आहे.
security-decrypt-unavailable = हा मेसेज डिक्रिप्ट करता येत नाही: एन्क्रिप्ट केलेला मेल वाचण्यासाठी { $tool } इंस्टॉल करा.
security-decrypt-failed = हा मेसेज डिक्रिप्ट करता येत नाही: { $reason }
security-unknown-signer = अज्ञात स्वाक्षरीकर्ता
security-signed-verified = { $signer } यांची स्वाक्षरी · पडताळलेली
security-signed-not-sender = { $signer } यांची स्वाक्षरी, जे प्रेषक नाहीत
security-signed-untrusted = { $signer } यांची स्वाक्षरी, तुम्ही विश्वसनीय नाही म्हणून खूण केलेल्या कीसह
security-signed-unverified = { $signer } यांची स्वाक्षरी · की पडताळलेली नाही
security-bad-signature = चुकीची स्वाक्षरी: स्वाक्षरी केल्यानंतर हा मेसेज बदलला गेला, किंवा स्वाक्षरी बनावट आहे.
security-signature-expired = { $signer } यांची स्वाक्षरी · स्वाक्षरीची मुदत संपली आहे
security-key-expired = { $signer } यांची स्वाक्षरी · त्यानंतर कीची मुदत संपली आहे
security-key-revoked = { $signer } यांची स्वाक्षरी, रद्द केलेल्या कीसह
security-missing-key = तुमच्याकडे नसलेल्या कीने स्वाक्षरी केली आहे, त्यामुळे ती तपासता येत नाही
security-missing-key-id = तुमच्याकडे नसलेल्या कीने ({ $key }) स्वाक्षरी केली आहे, त्यामुळे ती तपासता येत नाही
security-signature-unavailable = स्वाक्षरी केलेले; स्वाक्षरी तपासण्यासाठी { $tool } इंस्टॉल करा
security-signature-error = स्वाक्षरी तपासता आली नाही.
tracking-opened = { $who } यांनी तो { $count ->
    [one] एकदा
   *[other] { $count } वेळा
} उघडला, शेवटी { $when }
tracking-opens-clicks = { $who } यांनी तो { $opens ->
    [one] एकदा
   *[other] { $opens } वेळा
} उघडला आणि लिंक { $clicks ->
    [one] एकदा
   *[other] { $clicks } वेळा
} उघडली, शेवटी { $when }
tracking-clicked = { $who } यांनी लिंक { $clicks ->
    [one] एकदा
   *[other] { $clicks } वेळा
} उघडली, शेवटी { $when }
tracking-maybe-opened = { $who } यांनी तो कदाचित उघडला असेल (Apple Mail गोपनीयतेसाठी चित्रे लोड करते)
tracking-seen-none = अद्याप कोणीही तो उघडलेला नाही किंवा लिंक उघडलेली नाही
tracking-receipt = { $who } यांनी वाचल्याची पावती पाठवली
tracking-receipt-read = { $who } ने तो वाचला (वाचल्याची पावती), { $when }
tracking-receipt-displayed = वाचल्याची पावती: { $who } यांनी तुमचा मेसेज उघडला
tracking-receipt-other = वाचल्याची पावती: { $who } यांनी तुमचा मेसेज न उघडता हटवला किंवा हाताळला

## Remote images and pictures

remote-hidden = या मेसेजमधील इमेज लपवल्या आहेत.
remote-hidden-unconfirmed = इमेज लपवल्या आहेत: प्रेषकाची खात्री करता आली नाही.
remote-show = इमेज दाखवा
remote-always-show = या प्रेषकाकडील इमेज नेहमी दाखवा
remote-picture-use = वापरा
remote-picture-too-big = 8 MB किंवा त्यापेक्षा लहान चित्र निवडा.
remote-picture-type = PNG, JPEG, GIF, WebP किंवा SVG चित्र निवडा.
remote-picture-read-failed = चित्र वाचता येत नाही: { $error }
remote-picture-keep-failed = चित्र ठेवता येत नाही: { $error }
remote-picture-remove-failed = चित्र काढता येत नाही: { $error }

## Attachments

attachment-count = { $count ->
    [one] एक अटॅचमेंट
   *[other] { $count } अटॅचमेंट
}
attachment-save = सेव्ह करा
attachment-forward = फॉरवर्ड करा
attachment-save-all = सर्व सेव्ह करा
attachment-save-all-tooltip = सर्व अटॅचमेंट एका फोल्डरमध्ये सेव्ह करा
attachment-save-here = इथे सेव्ह करा
attachment-not-downloaded = हा मेसेज डाउनलोड केलेला नाही.
attachment-not-found = हे अटॅचमेंट मेसेजमध्ये सापडले नाही.
attachment-read-failed = { $name } वाचता आली नाही
attachment-numbered = अटॅचमेंट { $number }
attachment-saved-all = { $count ->
    [one] { $count } फाइल { $place } मध्ये सेव्ह केली
   *[other] { $count } फाइल { $place } मध्ये सेव्ह केल्या
}
attachment-saved-some = { $total ->
    [one] { $total } पैकी { $saved } फाइल { $place } मध्ये सेव्ह केली. { $failed } सेव्ह करता आली नाही
   *[other] { $total } पैकी { $saved } फाइल { $place } मध्ये सेव्ह केल्या. { $failed } सेव्ह करता आली नाही
}
attachment-saved-to = { $path } मध्ये सेव्ह केले
attachment-save-failed = { $name } सेव्ह करता आली नाही: { $error }
attachment-open-failed = { $name } उघडता आली नाही: { $error }
attachment-risky = ही फाइल एखादा प्रोग्राम चालवू शकते, म्हणून Katna ती उघडत नाही. त्याऐवजी ती सेव्ह करा.
attachment-encrypted-open = ही फाइल एन्क्रिप्ट केलेली आली होती. ती इतरत्र उघडण्यासाठी सेव्ह करा.

## Printing

print-failed = प्रिंट करता आले नाही: { $error }
print-no-font = कोणताही फॉन्ट सापडला नाही
print-opened-as-pdf = तिथून प्रिंट करण्यासाठी PDF म्हणून उघडले.
print-preview-title = प्रिंट पूर्वावलोकन
print-preview-laying-out = पाने मांडत आहे…
print-preview-pages = { $count ->
    [one] { $count } पान
   *[other] { $count } पाने
}
print-preview-more = { $count ->
    [one] आणि आणखी { $count } पान
   *[other] आणि आणखी { $count } पाने
}
print-preview-failed = पाने दाखवता आली नाहीत
print-preview-paper = कागद
print-preview-a4 = A4
print-preview-letter = लेटर
print-preview-layout = मांडणी
print-preview-as-shown = दिसते तसे
print-preview-simple = फक्त मजकूर
print-preview-backgrounds = पार्श्वभूमी
print-preview-cancel = रद्द करा
print-preview-print = प्रिंट करा
print-not-downloaded = (अजून डाउनलोड केलेले नाही.)
print-encrypted = (एन्क्रिप्ट केलेले. त्याचा मजकूर प्रिंट करण्यासाठी तो Katna Mail मध्ये उघडा.)
print-to = प्रति: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = वर पिन करा
text-copy-address = पत्ता कॉपी करा

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = याची अटॅचमेंट वाचण्यासाठी हा मेसेज उघडा.
text-copy = कॉपी करा
text-select-all = सर्व निवडा
