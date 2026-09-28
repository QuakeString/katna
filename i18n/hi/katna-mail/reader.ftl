# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = बंद करें
reader-back = वापस जाएं
reader-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
reader-move-to = इसमें ले जाएं
reader-more = ज़्यादा
reader-original-colors = मूल रंग दिखाएं
reader-dark-colors = गहरे रंगों में दिखाएं
reader-print-all = सभी प्रिंट करें
reader-new-window = नई विंडो में
reader-position = { $total } में से { $position }
reader-newer = नई
reader-older = पुरानी

## Reading pane: the conversation

reader-removed = यह बातचीत हटा दी गई।
reader-no-subject = (कोई विषय नहीं)
reader-collapse-all = सभी को छोटा करें
reader-expand-all = सभी को बड़ा करें
reader-unknown-sender = (अज्ञात भेजने वाला)
reader-date-ago = { $date } ({ $ago })
reader-me = मैं
reader-to = पाने वाले: { $names }
reader-to-label = पाने वाले:
reader-tick-delivered = डिलीवर हुआ { $when }
reader-tick-no-bounce = भेजा गया { $when }; डिलीवरी फेल का कोई जवाब नहीं आया, इसलिए यह बहुत संभव है कि पहुंच गया
reader-tick-bounced = डिलीवर नहीं हुआ: { $when } वापस आ गया
reader-tick-read = पढ़ा गया { $when } (पढ़ने की रसीद)
reader-tick-opened = खोला गया, आखिरी बार { $when } (ओपन ट्रैकिंग)
reader-starred = तारांकित
reader-not-starred = तारांकित नहीं
reader-too-long = मैसेज इतना लंबा है कि पूरा नहीं दिखाया जा सकता।
reader-encrypted-images = एन्क्रिप्ट किए गए मेल में वेब से इमेज कभी लोड नहीं की जातीं।
reader-window-failed = नई विंडो नहीं खोली जा सकी।

## Reading pane: message details (opened from "to me")

reader-details-from = भेजने वाला:
reader-details-to = पाने वाला:
reader-details-cc = cc:
reader-details-date = तारीख:
reader-details-subject = विषय:

## Reading pane: downloading a message

reader-downloading = यह मैसेज सर्वर से डाउनलोड किया जा रहा है…
reader-download-failed = यह मैसेज डाउनलोड नहीं किया जा सका।
reader-try-again = फिर से कोशिश करें

## Reply row

reply-reply = जवाब दें
reply-reply-all = सभी को जवाब दें
reply-forward = फ़ॉरवर्ड करें

## Encrypted and signed mail

security-decrypting = डिक्रिप्ट किया जा रहा है…
security-checking = हस्ताक्षर की जांच की जा रही है…
security-partly-encrypted = इस मैसेज का सिर्फ़ एक हिस्सा एन्क्रिप्ट किया गया है। बाकी हिस्सा सुरक्षा के बाहर जोड़ा गया था और किसी ने भी भेजा हो सकता है।
security-partly-signed = इस मैसेज के सिर्फ़ एक हिस्से पर हस्ताक्षर है। बाकी हिस्सा सुरक्षा के बाहर जोड़ा गया था और किसी ने भी भेजा हो सकता है।
security-encrypted = एन्क्रिप्ट किया गया मैसेज
security-encrypted-smime = एन्क्रिप्ट किया गया मैसेज (S/MIME)
security-no-key = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: इसे ऐसी कुंजी के लिए एन्क्रिप्ट किया गया था जो आपके पास नहीं है।
security-cancelled = डिक्रिप्ट करना रद्द कर दिया गया।
security-damaged = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: एन्क्रिप्ट किया गया डेटा खराब है या बदला गया है।
security-decrypt-unavailable = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: एन्क्रिप्ट किया गया मेल पढ़ने के लिए { $tool } इंस्टॉल करें।
security-decrypt-failed = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: { $reason }
security-unknown-signer = अज्ञात हस्ताक्षरकर्ता
security-signed-verified = { $signer } के हस्ताक्षर · सत्यापित
security-signed-not-sender = { $signer } के हस्ताक्षर, जो भेजने वाले नहीं हैं
security-signed-untrusted = { $signer } के हस्ताक्षर, ऐसी कुंजी से जिसे आपने भरोसेमंद नहीं के रूप में मार्क किया है
security-signed-unverified = { $signer } के हस्ताक्षर · कुंजी सत्यापित नहीं है
security-bad-signature = गलत हस्ताक्षर: हस्ताक्षर के बाद इस मैसेज को बदला गया, या हस्ताक्षर जाली है।
security-signature-expired = { $signer } के हस्ताक्षर · हस्ताक्षर की समय-सीमा खत्म हो गई है
security-key-expired = { $signer } के हस्ताक्षर · तब से कुंजी की समय-सीमा खत्म हो गई है
security-key-revoked = { $signer } के हस्ताक्षर, ऐसी कुंजी से जिसे रद्द कर दिया गया है
security-missing-key = ऐसी कुंजी से हस्ताक्षर किया गया जो आपके पास नहीं है, इसलिए इसकी जांच नहीं हो सकती
security-missing-key-id = ऐसी कुंजी ({ $key }) से हस्ताक्षर किया गया जो आपके पास नहीं है, इसलिए इसकी जांच नहीं हो सकती
security-signature-unavailable = हस्ताक्षरित; हस्ताक्षर की जांच के लिए { $tool } इंस्टॉल करें
security-signature-error = हस्ताक्षर की जांच नहीं हो सकी।
tracking-opened = { $who } ने इसे { $count ->
    [one] एक बार
   *[other] { $count } बार
} खोला, आखिरी बार { $when }
tracking-opens-clicks = { $who } ने इसे { $opens ->
    [one] एक बार
   *[other] { $opens } बार
} खोला और { $clicks ->
    [one] एक बार
   *[other] { $clicks } बार
} लिंक खोला, आखिरी बार { $when }
tracking-clicked = { $who } ने { $clicks ->
    [one] एक बार
   *[other] { $clicks } बार
} लिंक खोला, आखिरी बार { $when }
tracking-maybe-opened = { $who } ने शायद इसे खोला है (Apple Mail निजता के लिए इमेज लोड करता है)
tracking-not-opened = { $who } ने अभी तक इसे नहीं खोला है
tracking-receipt = { $who } ने पढ़ने की रसीद भेजी
tracking-receipt-displayed = पढ़ने की रसीद: { $who } ने आपका मैसेज खोला
tracking-receipt-other = पढ़ने की रसीद: { $who } ने आपका मैसेज खोले बिना मिटा दिया या निपटा दिया

## Remote images and pictures

remote-hidden = इस मैसेज की इमेज छिपाई गई हैं।
remote-show = इमेज दिखाएं
remote-always-show = इस भेजने वाले की इमेज हमेशा दिखाएं
remote-picture-use = इस्तेमाल करें
remote-picture-too-big = 8 MB या उससे छोटी तस्वीर चुनें।
remote-picture-type = PNG, JPEG, GIF, WebP या SVG तस्वीर चुनें।
remote-picture-read-failed = तस्वीर पढ़ी नहीं जा सकी: { $error }
remote-picture-keep-failed = तस्वीर सेव नहीं की जा सकी: { $error }
remote-picture-remove-failed = तस्वीर हटाई नहीं जा सकी: { $error }

## Attachments

attachment-count = { $count ->
    [one] एक अटैचमेंट
   *[other] { $count } अटैचमेंट
}
attachment-save = सेव करें
attachment-save-all = सभी सेव करें
attachment-save-all-tooltip = सभी अटैचमेंट किसी फ़ोल्डर में सेव करें
attachment-save-here = यहां सेव करें
attachment-not-downloaded = यह मैसेज डाउनलोड नहीं किया गया है।
attachment-not-found = यह अटैचमेंट मैसेज में नहीं मिला।
attachment-read-failed = { $name } पढ़ी नहीं जा सकी
attachment-numbered = अटैचमेंट { $number }
attachment-saved-all = { $count ->
    [one] { $count } फ़ाइल { $place } में सेव की गई
   *[other] { $count } फ़ाइलें { $place } में सेव की गईं
}
attachment-saved-some = { $total ->
    [one] { $total } में से { $saved } फ़ाइल { $place } में सेव की गई। { $failed } सेव नहीं की जा सकी
   *[other] { $total } में से { $saved } फ़ाइलें { $place } में सेव की गईं। { $failed } सेव नहीं की जा सकी
}
attachment-saved-to = { $path } में सेव किया गया
attachment-save-failed = { $name } सेव नहीं की जा सकी: { $error }
attachment-open-failed = { $name } खोली नहीं जा सकी: { $error }
attachment-risky = यह फ़ाइल कोई प्रोग्राम चला सकती है, इसलिए Katna इसे नहीं खोलता। इसके बजाय इसे सेव करें।
attachment-encrypted-open = यह फ़ाइल एन्क्रिप्ट होकर आई थी। इसे कहीं और खोलने के लिए सेव करें।

## Printing

print-failed = प्रिंट नहीं किया जा सका: { $error }
print-no-font = कोई फ़ॉन्ट नहीं मिला
print-opened-as-pdf = PDF के रूप में खोला गया, ताकि वहां से प्रिंट किया जा सके।
print-preview-title = प्रिंट प्रीव्यू
print-preview-laying-out = पेज तैयार किए जा रहे हैं…
print-preview-pages = { $count ->
    [one] { $count } पेज
   *[other] { $count } पेज
}
print-preview-more = { $count ->
    [one] और { $count } पेज
   *[other] और { $count } पेज
}
print-preview-failed = पेज नहीं दिखाए जा सके
print-preview-paper = कागज़
print-preview-a4 = A4
print-preview-letter = लेटर
print-preview-layout = लेआउट
print-preview-as-shown = जैसा दिख रहा है
print-preview-simple = सादा टेक्स्ट
print-preview-backgrounds = बैकग्राउंड
print-preview-cancel = रद्द करें
print-preview-print = प्रिंट करें
print-not-downloaded = (अभी तक डाउनलोड नहीं किया गया।)
print-encrypted = (एन्क्रिप्ट किया गया। इसका टेक्स्ट प्रिंट करने के लिए इसे Katna Mail में खोलें।)
print-to = पाने वाले: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = इसके अटैचमेंट पढ़ने के लिए यह मैसेज खोलें।
text-copy = कॉपी करें
text-select-all = सभी चुनें
